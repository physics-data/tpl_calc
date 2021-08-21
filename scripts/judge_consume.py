#!/usr/bin/env python3

import sys, time, os, subprocess, time, shutil, json

with open('./judge') as j:
    level = int(j.readline())
    if os.isatty(1):
        if level == 0:
            print("[Black box] 80/80 pts: All pass")
        elif level == 1:
            print("[Black box] 0/80 pts: Task 1 failed")
        elif level == 2:
            print("[Black box] 40/80 pts: Task 1 passed, task 2 failed")
        elif level == 3:
            print("[Black box] 60/80 pts: Task 1,2 passed, task 3 failed")
    else:
        grade = 0
        if level == 0:
            grade = 80
        elif level == 1:
            grade = 0
        elif level == 2:
            grade = 40
        elif level == 3:
            grade = 60
        print(json.dumps({'grade': grade / 80 * 100}))

