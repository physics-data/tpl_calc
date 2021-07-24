use structopt::StructOpt;
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

#[derive(StructOpt, Debug)]
struct Args {
    /// Number of top-level expressions
    #[structopt(long, short, default_value="1")]
    grpcnt: usize,

    /// Number of operations in each expression
    #[structopt(long, short, default_value="10")]
    opcnt: usize,

    /// Enabled binary operations
    #[structopt(long, default_value="+-*/")]
    ops: String,

    /// Probability of newline after each token
    #[structopt(long, short, default_value="1")]
    nlprob: f64,

    /// Input path
    #[structopt(long, short, default_value="./input.txt")]
    input: PathBuf,

    /// Std result path
    #[structopt(long, short, default_value="./std.txt")]
    std: PathBuf,
}

enum ExprNode {
    Number(u8),
    Expr(
        char,
        Rc<RefCell<ExprNode>>,
        Rc<RefCell<ExprNode>>,
    ),
}

impl ExprNode {
    fn grow<R: Rng>(&mut self, rng: &mut R, ops: &[char]) -> (Rc<RefCell<ExprNode>>, Rc<RefCell<ExprNode>>) {
        if let &mut ExprNode::Number(orig) = self {
            let rhs: u8 = rng.gen();
            let op = ops[rng.gen_range(0..ops.len())];

            let rhs = Rc::new(RefCell::new(ExprNode::Number(rhs)));
            let lhs = Rc::new(RefCell::new(ExprNode::Number(orig)));

            *self = ExprNode::Expr(
                op,
                lhs.clone(),
                rhs.clone(),
            );

            return (lhs, rhs);
        } else {
            panic!("Growing non-leaf node");
        }
    }

    fn flatten<R: Rng>(&self, rng: &mut R, nlprob: f64, builder: &mut String) {
        let nl = rng.gen_bool(nlprob);
        let delim = if nl { "\n" } else { " " };
        match self {
            &ExprNode::Number(n) => {
                builder.push_str(&format!("{}{}", n, delim));
            },
            &ExprNode::Expr(op, ref lhs, ref rhs) => {
                builder.push(op);
                builder.push_str(delim);

                lhs.borrow().flatten(rng, nlprob, builder);
                rhs.borrow().flatten(rng, nlprob, builder);
            },
        }
    }
}

#[paw::main]
fn main(args: Args) -> anyhow::Result<()> {
    let opvec: Vec<_> = args.ops.chars().collect();
    let mut rng = thread_rng();

    let mut flattened = String::new();
    let mut results = Vec::new();

    for _ in 0..args.grpcnt {
        let (expr, val) = guarded_gen_expr(&mut rng, &args, opvec.as_slice());
        expr.borrow().flatten(&mut rng, args.nlprob, &mut flattened);
        results.push(val);
    }

    let trimmed = flattened.trim_end();
    let mut input = File::create(args.input)?;
    write!(input, "{}\n", trimmed)?;

    let mut std = File::create(args.std)?;
    for result in results {
        write!(std, "{}\n", result)?;
    }
    Ok(())
}

fn gen_expr<R: Rng>(rng: &mut R, args: &Args, ops: &[char]) -> Rc<RefCell<ExprNode>> {
    let orig: u8 = rng.gen();
    let root = Rc::new(RefCell::new(ExprNode::Number(orig)));

    let mut leaves = vec![root.clone()];

    for _ in 0..args.opcnt {
        // Sample a node to grow a new expr

        let sel = rng.gen_range(0..leaves.len());
        let picked = leaves.swap_remove(sel);

        let mut borrow = picked.borrow_mut();
        let (lhs, rhs) = borrow.grow(rng, ops);

        leaves.push(lhs);
        leaves.push(rhs);
    }

    root
}

fn eval(expr: &Rc<RefCell<ExprNode>>) -> Option<u32> {
    match expr.borrow().deref() {
        &ExprNode::Number(n) => Some(n as u32),
        &ExprNode::Expr(op, ref l, ref r) => {
            let ln = eval(l)?;
            let rn = eval(r)?;

            match op {
                '+' => ln.checked_add(rn),
                '-' => ln.checked_sub(rn),
                '*' => ln.checked_mul(rn),
                '/' => ln.checked_div(rn),
                _ => panic!("Unsupported op: {}", op),
            }
        }
    }
}

fn guarded_gen_expr<R: Rng>(rng: &mut R, args: &Args, ops: &[char]) -> (Rc<RefCell<ExprNode>>, u32) {
    loop {
        let cur = gen_expr(rng, args, ops);
        if let Some(result) = eval(&cur) {
            break (cur, result)
        }
    }
}
