package lc1751

import (
	"fmt"
	// "sort"
)

func main() {
	test1751()
}

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

func maxValue(events [][]int, k int) int {
	value := 0

	// current := events[0]
	// stack := []int{events[0]}
	a := 0

	for a < len(events) {
		fmt.Println(events[a])
	}
	return value
}
