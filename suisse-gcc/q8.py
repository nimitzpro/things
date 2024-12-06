#!/usr/bin/env python

def solution(n, m, costs):
    strats = []
    for i in range(m-1):
        s = costs[i+1] - costs[i]
        if s > 0:
            if len(strats) < n:
                strats.append(s)
                continue
            for j in range(len(strats)):
                if s > strats[j]:
                    strats = strats[:j] + s + strats[j:]
                    break

    return sum(strats[:n])

print(solution(2, 3, [2,4,1]))
print(solution(2, 6, [3,2,6,5,0,3]))
