package main

func test2997() {
	nums := [4]int{2, 1, 3, 4}
	println(minOperations(nums[:], 1))
	nums2 := [4]int{6, 1, 2, 4}
	println(minOperations(nums2[:], 1))
}

func minOperations(nums []int, k int) int {
	ops := 0
	i := nums[0]
	for _, x := range nums[1:] {
		i ^= x
	}
	i ^= k
	for {
		if (i & 1) == 1 {
			ops++
		}
		if i == 0 {
			break
		}
		i = i >> 1
	}
	return ops
}
