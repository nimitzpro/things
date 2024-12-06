#!/usr/bin/env python

def solution(cutoffScore, scoresLength, scores):
    c = 0

    lessers = []

    for i in scores:
        if i < cutoffScore:
            c += 1

    return c

print(solution(16, 4, [10,5,2,6]))
print(solution(0, 3, [1,2,3]))

'''
MAX m
list l

for max of l
work down


'''
