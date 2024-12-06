#!/usr/bin/env python

def solution(L, P):
    scanned = 0
    smallest = L

    while scanned < L-1:
        currentTL = scan(L, P)
        currentBR = check_whitespaceY(L, P, currentTL)
        currentBR = check_whitespaceX(L, P, currentBR)

    return currentTL, currentBR

    return smallest

def check_whitespaceY(L, P, currentBR):
    countY = 0
    y = currentBR[0]
    if P[y][currentBR[1]] == "$":
        print("$ found when scanning y:", y, currentBR[1])
        print("recursing on:", y, currentBR[1])
        return check_whitespaceY(L, P, [y+1, currentBR[1]])
    else:
            #countY += 1
            #if countY > 1:
            #currentBR[0] += 1
            #break
        return [currentBR[0]+1, currentBR[1]]

    return currentBR

def check_whitespaceX(L, P, currentBR):
    countX = 0
    x = currentBR[1]
    if P[currentBR[0]][x] == "$":
        print("$ found when scanning x:", currentBR[0], x)
        return check_whitespaceX(L, P, [currentBR[0], x+1])
    else:
            #countX += 1
            #if countX > 1:
            #currentBR[1] += 1
            #break
        return [currentBR[0], currentBR[1]+1]

    return currentBR


def scan(L, P):
    for i in range(L):
        for j in range(L):
            if P[i][j] == "$":
                return [i, j]


print(solution(5, ["$$---","-$$--","$-$--","-----","----$"]))

print(solution(3, ["---","---","---"]))

print(solution(21, ['---------------------', '---------------------', '---$-$---------------', '----$-------------$--', '---$-$-----------$-$-', '------------------$--', '---------------------', '---------------------', '---------------------', '----------$-----$----', '------$--$$$---------', '--------$$$$$--$-----', '-------$$$$$$$-------', '-----$--$$$$$--------', '-------$$$$$$$-------', '------$$$$$$$$$------', '-------$$$$$$$--$----', '------$$$$$$$$$------', '--$--$$$$$$$$$$$-----', '---------$$$---------', '---------$$$---------']))
