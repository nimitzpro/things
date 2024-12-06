#!/usr/bin/env python

def solution(n, employees, shifts):
    count_ints = 0
    #intervals = [[1,2],[2,3]]
    #interval_employees = [["Alex", "David"],["Josh"]]

    times = {key: [] for key in [h for h in range(1,22)]}
    intervals = []
    interval_employees = []
    for s in range(n):
        for h in range(shifts[s][0], shifts[s][1]):
            times[h] += [employees[s]]

    current = []
    current_employees = []
    for i in times.keys():
        if times[i]: # If there are employees
            if not current and not current_employees:
                current = [i, i+1]
                current_employees = (times[i])
            if current:
                if current_employees == times[i]:
                    current[1] = i+1
                else:
                    intervals.append(current)
                    interval_employees.append(current_employees)
                    current = [i, i+1]
                    current_employees = times[i]
                    count_ints += 1
        else:
            if current_employees:
                intervals.append(current)
                interval_employees.append(current_employees)
                current = [i, i+1]
                current_employees = times[i]
                count_ints += 1

    string = str(count_ints) + "\n"
    for i in range(count_ints):
        string += " ".join([str(j) for j in intervals[i]]) + " "
        interval_employees[i].sort()
        string += str(len(interval_employees[i])) + " "
        string += " ".join(interval_employees[i]) + "\n"
    print(string[:-1])


solution(5, ["Alice", "Bob", "Casey", "Deepak", "Emma"], [[10,14],[11,12],[10,15],[12,16],[14,16]])
solution(3, ["Neil", "Angel", "Alok"], [[1,10],[7,9],[7,10]])
solution(2, ["Alice", "Bob"], [[1,3],[5,7]])
