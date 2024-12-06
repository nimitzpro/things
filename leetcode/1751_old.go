package main

import (
	"fmt"
	"sort"
)

func test1751_old() {
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
}

func maxValue_old(events [][]int, k int) int {
	sort.Slice(events, func(j, k int) bool {
		return events[j][1] < events[k][1]
	})
	sort.Slice(events, func(j, k int) bool {
		return events[j][0] < events[k][0]
	})
	fmt.Println("events:", events)

	stack := [][]int{events[0]}
	interim_stack := [][]int{}
	value := events[0][2]
	temp_value := 0
	// initial_events_len := len(events)
	// var smallest, smallest_i int
	smallest := events[0][2]
	smallest_i := 0

	for index, i := range events[1:] {
		fmt.Println("interim_stack:", interim_stack)
		if i[0] > stack[len(stack)-1][1] { // greedy
			value += i[2]
			stack = append(stack, i)
			// events = append(events[:index], events[index+1])
			interim_stack = nil
			fmt.Println("add node greedily, stack:", stack)
			smallest_i, smallest = updateSmallest(index, i[2], smallest_i, smallest)
		} else {
			temp_value = 0
			rev_index := len(stack) - 1
			for rev_index >= 0 { // backtrack last few nodes
				if i[0] > stack[rev_index][1] {
					break
				}
				temp_value += stack[rev_index][2]
				rev_index--
			}
			if temp_value < i[2] {
				// stack_len := len(stack)
				stack = stack[:max(0, rev_index+1)]
				value += i[2] - temp_value
				stack = append(stack, i)
				// fmt.Println("events:", events, "stack:", stack, ", rev_index:", rev_index, "stack_len:", stack_len, "events_i:", events_i)
				// events = append(events[0:events_i-max(0, stack_len-max(0, rev_index))], events[events_i:]...)
				// events_i -= stack_len - max(0, rev_index)
				interim_stack = nil
				fmt.Println("replace prior node(s) with new larger value node, stack:", stack)
			} else {
				temp_value = i[2]
				for _, j := range interim_stack {
					temp_value += j[2]
				}
				if temp_value > stack[len(stack)-1][2] {
					value += temp_value - stack[len(stack)-1][2]
					interim_stack = append(interim_stack, i)
					stack = stack[:len(stack)-1]
					stack = append(stack, interim_stack...)
					interim_stack = nil
					fmt.Println("replace large node with a few nodes in the same space for higher total, stack:", stack)
				} else {
					interim_stack = append(interim_stack, i)
				}
			}
		}
		if len(stack) > k {
			stack = append(stack[:smallest_i], stack[smallest_i+1:]...)
			value -= smallest
		}
	}
	fmt.Println("stack:", stack)

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
	// 	stack = append(stack[:smallest_i], stack[smallest_i+1:]...)
	// 	value -= smallest_i
	// 	fmt.Println("removed the smallest value node from the stack:", stack)
	// }
	// if len(events) == initial_events_len {
	// 	return value
	// } else {
	// 	return maxValue(events, k)
	// }

	return value
}

func eq_old(a, b []int) bool {
	return a[0] == b[0] && a[1] == b[1] && a[2] == b[2]
}

func updateSmallest_old(index, i_value, smallest_i, smallest int) (int, int) {
	if i_value < smallest {
		return index, i_value
	} else {
		return smallest_i, smallest
	}
}

// type Key struct {
// 	start, end, value int
// }
//
// func maxValue_old(events [][]int, k int) int {
// 	stack := [][]int{}
// 	return maxValueActual(events, k, stack, -1)
// }
//
// func maxValueActual_old(events [][]int, k int, stack [][]int, pastResult int) int {
// 	// println(strconv.Itoa(events[0][2]), strconv.Itoa(events[1][2]), strconv.Itoa(events[2][2]))
// 	sort.Slice(events, func(j, k int) bool {
// 		return events[j][1] < events[k][1]
// 	})
// 	sort.Slice(events, func(j, k int) bool {
// 		return events[j][0] < events[k][0]
// 	})
// 	// println(strconv.Itoa(events[0][2]), strconv.Itoa(events[1][2]), strconv.Itoa(events[2][2]))
// 	var temp_value int
// 	var smallest int
// 	var smallest_value int
//
// 	// var stack = [][]int{}
// 	value := events[0][2]
// 	if len(stack) == 0 {
// 		stack = append(stack, events[0])
// 		fmt.Println("starting stack:", stack)
// 	}
//
// 	// var swap_history = make(map[Key][][]int)
//
// 	var interim_stack = [][]int{}
//
// 	for _, i := range events[1:] {
// 		fmt.Println("interim_stack:", interim_stack)
// 		if i[0] > stack[len(stack)-1][1] { // greedy
// 			value += i[2]
// 			stack = append(stack, i)
// 			interim_stack = nil
// 			fmt.Println("add node greedily, stack:", stack)
// 		} else {
// 			temp_value = 0
// 			rev_index := len(stack) - 1
// 			for rev_index >= 0 { // backtrack last few nodes
// 				if i[0] > stack[rev_index][1] {
// 					break
// 				}
// 				temp_value += stack[rev_index][2]
// 				rev_index--
// 			}
// 			if temp_value < i[2] {
// 				stack = stack[:max(0, rev_index+1)]
// 				stack = append(stack, i)
// 				value += i[2] - temp_value
// 				interim_stack = nil
// 				fmt.Println("replace prior node(s) with new larger value node, stack:", stack)
// 			} else if pastResult != -1 {
// 				temp_value = i[2]
// 				for _, j := range interim_stack {
// 					temp_value += j[2]
// 				}
// 				if temp_value > stack[len(stack)-1][2] {
// 					value += temp_value - stack[len(stack)-1][2]
// 					interim_stack = append(interim_stack, i)
// 					// new_key := Key{start: stack[len(stack)-1][0], end: stack[len(stack)-1][1], value: stack[len(stack)-1][2]}
// 					// swap_history[new_key] = interim_stack
// 					stack = stack[:len(stack)-1]
// 					stack = append(stack, interim_stack...)
// 					interim_stack = nil
// 					fmt.Println("replace large node with a few nodes in the same space for higher total, stack:", stack)
// 				} else {
// 					interim_stack = append(interim_stack, i)
// 				}
// 			}
// 		}
// 	}
//
// 	for len(stack) > k {
// 		smallest = 10000000
// 		smallest_value = 0
// 		for index, i := range stack {
// 			if i[2] < smallest {
// 				smallest = index
// 				smallest_value = i[2]
// 			}
// 		}
// 		stack = append(stack[:smallest], stack[smallest+1:]...)
// 		value -= smallest_value
// 		fmt.Println("removed the smallest value node from the stack:", stack)
// 	}
// 	fmt.Println(events)
// 	fmt.Println(stack)
// 	// println("pastResult", pastResult)
// 	// println("value", value)
// 	if pastResult == value {
// 		return value
// 	}
//
// 	if value < pastResult {
// 		return pastResult
// 	} else {
// 		return maxValueActual(events, k, stack, value)
// 	}
//
// }
