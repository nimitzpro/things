#!/usr/bin/env python
from typing import List

def solution(n: int, l: int, transfers: List[List[int]]) -> bool:
    no_recv = {client:[] for client in range(n)}

    for i in range(len(transfers)-1):
        if transfers[i][0] > transfers[i+1][0]:
            temp = transfers[i]
            transfers[i] = transfers[i+1]
            transfers[i+1] = temp

    for i in transfers:
        no_recv[i[0]].append(i[1])
        if i[1] < i[0]:
            l = []
            for s in range(0, i[0]):
                for r in no_recv[s]:
                    l.append(r)

            if i[0] in l:
                return True

    return False

def sent(sender, no_recv):
    return no_recv[sender], [sent(i, no_recv) for i in no_recv[sender]]

# True is issue, False if no issue

print(solution(4, 5, [[0,1],[1,2],[1,3],[2,0],[3,2]]))
print(solution(3, 2, [[0,1],[1,1]]))
