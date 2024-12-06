#!/usr/bin/env python

def gen_primes(n):
    primes = []

    s = [1,7,11,13,17,19,23,29,31,37,41,43,47,49,53,59]

    return primes

def solution(n):
    options = ["BUY", "SELL", "PASS"]
    even = 0
    odd = 0
    failed = []
#    primes = gen_primes(int(n/3) + 1)
    if not n % 2:
        even += 1
    else:
        odd += 1

    for i in range(1, int(n/2)+1, 1):
        if i in failed:
            continue
        if n % i:
            continue
        j = 1
        while True:
            k = i * j
            if k in failed:
                break
            if k % 2:
                if n % k:
                    failed.append(k)
                    odd += (j-1)
                    break
            j += 1

    if even > odd:
        return options[0]
    elif odd < even:
        return options[1]
    else:
        return options[2]


def basic(n):
    odd = 0
    even = 0
    if n % 2:
        odd += 1
    else:
        even += 1

    for i in range(1, int(n/2)+1):
        if not n % i:
            if i % 2:
                odd += 1
            else:
                even += 1

    if odd > even:
        return "SELL"
    elif even > odd:
        return "BUY"
    else:
        return "PASS"

#print(solution(2))
#print(solution(123456))

print(basic(2))
print(basic(123456))

print(solution(832483274807))
#print(solution(100000000000))
