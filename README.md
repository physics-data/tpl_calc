# [Calc.](https://www.nicovideo.jp/watch/sm12050471)

## 问题背景

喵喵是猫猫。喵喵正在学算数。准确来说，是大学生都无法准确进行的**四则运算**。

喵喵是懒猫猫。喵喵不想写太多括号。喵喵读到有一种**前缀表达式**，通过按照计算顺序先写符号，再写左右计算数，可以完全不写括号而不造成歧义：

> `(1 + (2 - 3) * 4) / 5`
> 将会被表示为
> `/ + 1 * - 2 3 4 5`
> 读作
> `/ (+ 1 (* (- 2 3) 4)) 5`

喵喵是菜喵喵。它写下了表达式以后，发现自己的两个爪子上只有十个指头，无法进行十以上的运算。

喵喵正在找人帮它算算数，请你救救猫猫。

## 问题描述

在仓库的根目录中有一个文件 `calc.example.sh`，请将其重命名为 `calc.sh` 并仔细阅读其内容。其中包含有你需要开始着手此问题的信息。后续更改请直接在 `calc.sh` 中进行。

你需要从标准输入读入数个 **前缀表达式**，其中包含四种运算符 `+-*/` 和十进制表达的数字。数字及运算符和相邻的数字及运算符之间有一个空白字符分隔（空格 ` ` 或者换行符 `\\n`）。保证输入末尾有一个换行。请注意，新的表达式并不一定在换行处开始。

进行除法得到结果不是整数时，向 0 取整：

> -1 / 3 = 0
> 1 / 3 = 0

你需要输出数行，行数和读入的表达式数量相同；每行一个**整数**，对应表达式的求值结果。

## 子任务

本题目分为三个子任务：

- Task 1: 输入仅包含**一个**前缀表达式，一行保证只有一个运算符或者数字。
- Task 2: 输入仅包含**一个**前缀表达式，但一行内可能有**多个**运算符、数字。运算符及数字之间空格分隔。
- Task 3: 输入包含**多个**前缀表达式，一行内可能有**多个**运算符和数字。

对于所有的输入，保证：
- 表达式合法（形式正确，不包含除0操作）
- 任意子表达式的值不超过 32 位带符号整数 (signed long, i32) 的表示范围

以上三个任务都可以仅使用 Bash 内嵌的功能 (builtins) 实现，但是不强制。

## 样例与评测

本题目评测使用 **随机生成数据** 评测。在 data 目录下你会看到三个子目录，`task1`, `task2` 和 `task3`。这是为了方便同学们在本地测试而预先生成的数据，**并不一定与 Actions 的评测结果相同**。

你可以使用 **./scripts/judge_local.sh** 使用预先生成的数据进行本地评测。

我们推荐直接进行提交以进行随机数据的评测，出现错误的细节将在 Action Build Log 中展示，数据可以使用 Build Artifact 下载。但如果你想在本地测试，或者希望了解在线评测的细节，请参考 `docs/judge.md` 内的说明。

本题分数构成为：

- 黑盒：根据子任务是否通过分别给分。每个子任务会生成多组数据，全部通过将会获得子任务的分数。
  - Task 1: 40 分
  - Task 2: 20 分
  - Task 3: 20 分
- 白盒：代码风格与 Git 使用 20 分（包括恰当注释、合理命名、提交日志等）。

本题不设置时间、空间限制。

助教以 deadline 前 GitHub 上最后一次提交为准进行评测。
