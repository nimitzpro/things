package main

func test2441() {
	nums := []int{-1, 2, -3, 3} // 3
	println(findMaxK(nums[:]))
	nums = []int{-1, 10, 6, 7, -7, 1} // 7
	println(findMaxK(nums[:]))

}

func findMaxK(nums []int) int {
	x := -1
	var positives []int
	var negatives []int
	for _, i := range nums {
		if i < 0 && -i > x {
			for index, j := range positives {
				if -i == j {
					x = -i
					positives = append(positives[:index], positives[index+1:]...)
					break
				}
			}
			if x != -i {
				negatives = append(negatives, -i)
			}
		} else if i > x {
			for index, j := range negatives {
				if i == j {
					x = i
					negatives = append(negatives[:index], negatives[index+1:]...)
					break
				}
			}
			if x != i {
				positives = append(positives, i)
			}
		}
	}
	return x
}
