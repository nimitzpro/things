package main

import (
	"fmt"
	"sort"
)

func test1751() {
	events := [][]int{{1, 2, 4}, {3, 4, 3}, {2, 3, 10}} // start, end, value -> 10
	k := 2
	println(maxValue(events, k))
	events = [][]int{{1, 2, 4}, {3, 4, 3}, {2, 3, 1}} // start, end, value -> 7
	k = 2
	println(maxValue(events, k))
	events = [][]int{{1, 1, 1}, {2, 2, 2}, {3, 3, 3}, {4, 4, 4}} // start, end, value -> 9
	k = 3
	println(maxValue(events, k))
	events = [][]int{{1, 3, 4}, {2, 4, 1}, {1, 1, 4}, {3, 5, 1}, {2, 5, 5}} // -> 9
	k = 3
	println(maxValue(events, k))
	events = [][]int{{11, 17, 56}, {24, 40, 53}, {5, 62, 67}, {66, 69, 84}, {56, 89, 15}} // -> 151
	k = 2
	println(maxValue(events, k))
	events = [][]int{{1, 1, 10}, {2, 4, 6}, {2, 5, 8}, {5, 8, 4}, {8, 14, 7}, {9, 10, 2}}
	k = 3
	println(maxValue(events, k))
}

//	func addToStack(stack [][]int, event []int) [][]int {
//		for _, a := range stack {
//			if event[1] < a[0] {
//				temp_stack := [][]int{event}
//				fmt.Println("add node greedily, stack:", stack)
//				return append(temp_stack, stack...)
//			} else if event[0] > a[1] {
//				fmt.Println("add node greedily, stack:", stack)
//				return append(stack, event)
//			}
//		}
//		return stack
//	}
func maxValue(events [][]int, k int) int {
	sort.Slice(events, func(j, k int) bool {
		return events[j][1] < events[k][1]
	})
	sort.Slice(events, func(j, k int) bool {
		return events[j][0] < events[k][0]
	})
	events_ascending := events
	sort.Slice(events_ascending, func(j, k int) bool {
		return events_ascending[j][2] > events_ascending[k][2]
	})
	fmt.Println("events:", events)

	// temp_value := 0
	// initial_events_len := len(events)
	// var smallest, smallest_i int

	stack := [][]int{events_ascending[0]}
	temp_stack := [][]int{}
	value := events_ascending[0][2]
	a := 1
	fmt.Println("initial stack:", stack)
	for len(stack) < k && a != len(events_ascending) {
		// temp_stack = [][]int{}
		for b_index, b := range stack {
			if events_ascending[a][1] < b[0] {
				fmt.Printf("stack up to b_index:", stack[:b_index])
				temp_stack := append(stack[:b_index], events_ascending[a])
				fmt.Println("temp_stack:", temp_stack)
				fmt.Printf("stack from b_index:", stack[b_index:])
				stack = append(temp_stack, stack[b_index:]...)
				value += events_ascending[a][2]
				fmt.Println("add node greedily before current node, stack:", stack)
				break
			} else if events_ascending[a][0] > b[1] {
				if b_index == len(stack)-1 {
					stack = append(stack, events_ascending[a])
					value += events_ascending[a][2]
					fmt.Println("add node greedily to the end, stack:", stack)
					break
				} else if events_ascending[a][1] < stack[b_index+1][0] {
					temp_stack = append(stack[:b_index+1], events_ascending[a])
					fmt.Println("temp_stack 2:", temp_stack)
					stack = append(temp_stack, stack[b_index+1:]...)
					value += events_ascending[a][2]
					fmt.Println("add node greedily after current node, stack:", stack)
					break
				}
			}
		}
		a++
	}
	return value
	value_copy := 0
	interim_stack := [][]int{}
	affected_list := []int{}
	affected_sum := 0
	temp_list := [][]int{}
	i := 0
	j := 0
	for value != value_copy && i != len(events) && j != len(stack) {
		if events[i][0] >= stack[j][0] { // current event i starts later or at the same time as event in stack j
			if events[i][0] <= stack[j][1] { // i starts before or at the same time as j finishes
				if events[i][1] <= events[j][1] { // i finishes before or at the same time as j
					if events[i][2] >= events[j][2] { // i more valuable than j
						if len(interim_stack) > 0 { // add all interim events
							interim_stack = append(interim_stack, stack[i])
							temp_list = append(stack[:j], interim_stack...)
							j += len(interim_stack)
							stack = append(temp_list, stack[min(len(stack), j+1):]...)
							temp_list = [][]int{}
							interim_stack = [][]int{}
							continue
						} else {
							temp_list = stack[min(len(stack), j+1):]
							stack = append(stack[:j], events[i])
							stack = append(stack, temp_list...)
							temp_list = [][]int{}
							j += 1
							continue
						}
					} else { // i less valuable the j, add to interim
						interim_stack = append(interim_stack, events[i])
						i += 1
						continue
					}
				}
			}
			affected_list = append(affected_list, j)
			affected_sum += stack[j][2]
		} else if events[i][0] < stack[j][0] { // current event i starts before event in stack j
			if events[i][1] < stack[j][0] { // current event i finishes before event in stack j

			}
		}
		i += 1
		j += 1
	}
	fmt.Println("stack:", stack)
	return value
}

// for _, i := range events {
// 	// split into more if possible and better
// }
// for len(stack) > k {
// 	smallest = 10000000
// 	for index, i := range stack {
// 		if i[2] < smallest {
// 			smallest_i = index
// 			smallest = i[2]
// 		}
// 	}
// 	for index, event := range events {
// 		if eq(event, stack[smallest_i]) {
// 			events = append(events[:index], events[index+1:]...)
// 			break
// 		}
// 	}
// stack = append(stack[:smallest_i], stack[smallest_i+1:]...)
// value -= smallest_i
// fmt.Println("removed the smallest value node from the stack:", stack)
// }
// if value == rev_value {
// 	return value
// } else if value > {
// 	return maxValue(events, k)
// }

func eq(a, b []int) bool {
	return a[0] == b[0] && a[1] == b[1] && a[2] == b[2]
}

func updateSmallest(index, i_value, smallest_i, smallest int) (int, int) {
	if i_value < smallest {
		return index, i_value
	} else {
		return smallest_i, smallest
	}
}
