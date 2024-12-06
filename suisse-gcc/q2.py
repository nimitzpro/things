#!/usr/bin/env python

#import re
#result = re.sub(r"[^a-zA-Z]+", '', files)

def solution(files):
    count = 0
    chars = {}
    longest_odd = 0
    if len(files) == 0:
        return count

    for char in files:
        if char not in chars.keys():
            chars[char] = 1
        else:
            chars[char] += 1

    for c in chars.keys():
        if not chars[c] % 2:
            count += chars[c]
            continue
        if chars[c] > longest_odd:
            longest_odd = chars[c]

    count += longest_odd

    return count
#    return count, longest_odd, chars

print(solution("a"))
print(solution("abccccdd"))

print(solution("aabbccddeeeeeeeeeeeefgGGGGGHHHEEDDJJWWSSSXggggggjjjkkllkiiiIIIIIIIIIIIIIIIIIIHHHHHHHHHHHHHHJJJJJJJJJJJJJjjjjjjjkddddddddddmmmmmmmmmekkcllllllxoaaaappppppzzzzzzzzzzppppppPPQLLLLLLkDIkemDSNEUfhedmsDSWKDKFMDdfkeffLKVNMNCZsdsdDDDWWD"))

print(solution("eva, can i see bees in a cave?"))

print(solution("dddddabbbabbbbbQ"))
