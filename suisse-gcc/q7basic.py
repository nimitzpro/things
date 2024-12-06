#!/usr/bin/env python


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
            return True

    return False
