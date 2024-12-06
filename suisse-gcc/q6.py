#!/usr/bin/env python

def basic(n, flows):
    hops = []

    for i in range(n):
        j = flows[i] - 1
        i_hops = 1
        while j != i:
            i_hops += 1
            j = flows[j] - 1
        hops.append(str(i_hops))

    print(" ".join(hops))


basic(3, [2,3,1])
basic(5, [1,2,3,4,5])
solution(3, [2,3,1])
solution(5, [1,2,3,4,5])
