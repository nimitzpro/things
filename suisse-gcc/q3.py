#!/usr/bin/env python

def solution_old(N, M, P): # Doesn't work, but was submitted :/

    if N == 1:
        return 1

    i = 0
    tail = 0
    temp_sum = 0
    while i < N - 1:
        if P[i] > P[i+1]:
            temp_sum += P[i]
            tail = P[i+1]
            i += 1
            continue
        elif tail is not None:
            if P[i+1] == tail:
                temp_sum += P[i]
                i += 1
                continue
            temp_sum += tail
            if temp_sum > M:
                return 0

            temp_sum = 0

        tail = None
        i += 1

    if temp_sum + tail > M:
        return 0

    return 1



def solution(N, M, P):
    i = 0
    while i < N - 1:
        temp = 0
        if P[i] > P[i+1]:
            # print("comparing:", P[i], P[i+1], M)
            if P[i] + P[i+1] > M:
                return 0
            temp = P[i]
            P[i] = P[i+1]
            P[i+1] = temp
            # print(P)
            solution(N, M, P)
        i += 1

    return 1

print(solution(4, 1, [2,1,3,4]))
print(solution(4, 1, [1,2,3,4]))
print(solution(5, 7, [3,2,2,3,3]))

print(solution(6, 200, [5,6,197,2,7,197]))

print(solution(6, 30, [5,15,2,28,1,2]))
