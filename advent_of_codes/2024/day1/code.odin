package day1

import "core:fmt"
import "core:io"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

// insertion_sort :: proc(list: [dynamic]int) -> [dynamic]int {
//     new_list := [dynamic]int{}
//     append(&new_list, list[0])
//     curr := 1
//     for curr < len(list) {
//         for i := 0; i < len(new_list); i += 1 {
//             if list[curr] < new_list[i] {
//                 temp := [dynamic]int{}
//                 append(&temp, ..new_list[:i])
//                 append(&temp, list[curr])
//                 append(&temp, ..new_list[i:])
//                 new_list = temp
//             }
//         }
//         curr += 1
//     }
//     return new_list
// }

p1 :: proc(list1, list2: [dynamic]int) {
    sum := 0
    for i := 0; i < len(list1); i += 1 {
        diff := list1[i] - list2[i]
        if diff < 0 {
            diff = -diff
        }
        sum += diff
    }
    fmt.println(sum)
}

p2 :: proc(list1, list2: [dynamic]int) {
    sim := 0

    b := 0
    quantity := 0
    cached := 0
    for a := 0; a < len(list1); {
        if a > 0 {
            if list1[a] == list1[a - 1] {
                sim += cached
                a += 1
                continue
            }
        }

        quantity = 0
        for b < len(list2) {
            if list2[b] == list1[a] {
                quantity += 1
                b += 1
            } else if (list2[b] < list1[a]) {
                b += 1
            } else {
                break
            }
        }
        cached = list1[a] * quantity
        sim += cached
        a += 1
    }
    fmt.println(sim)
}

main :: proc() {
    contents, err := os.read_entire_file("input.txt", context.allocator)
    contents2 := string(contents)
    // fmt.println(contents2)

    list1 := make([dynamic]int)
    list2 := make([dynamic]int)
    for line in strings.split_lines_iterator(&contents2) {
        number1 := strconv.atoi(line[0:5])
        append(&list1, number1)
        number2 := strconv.atoi(line[8:13])
        append(&list2, number2)
    }
    slice.sort(list1[:])
    slice.sort(list2[:])

    p1(list1, list2)

    p2(list1, list2)

}
