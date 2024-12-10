package day2

import "core:fmt"
import "core:io"
import "core:mem"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

p1 :: proc(contents: ^string) {
    safe_line_count := 0
    failed := false
    res: []string
    ok: mem.Allocator_Error
    init_number, last_number: int
    increasing := false
    for line in strings.split_lines_iterator(contents) {
        failed = false
        res, ok = strings.split(line, sep = " ")
        init_number = strconv.atoi(res[0])
        last_number = strconv.atoi(res[1])
        increasing = false

        if last_number == init_number {
            continue
        } else if last_number > init_number {
            increasing = true
            if last_number - init_number > 3 {
                continue
            }
        } else {
            if init_number - last_number > 3 {
                continue
            }
        }

        for i := 2; i < len(res); i += 1 {
            current_number := strconv.atoi(res[i])
            if last_number == current_number {
                failed = true
                break
            }
            if increasing {
                if last_number > current_number ||
                   current_number - last_number > 3 {
                    failed = true
                    break
                }
            } else {
                if last_number < current_number ||
                   last_number - current_number > 3 {
                    failed = true
                    break
                }
            }
            last_number = current_number
        }

        if failed {
            continue
        }
        safe_line_count += 1
    }
    fmt.println(safe_line_count)
}

validate_list :: proc(res: [dynamic]int, increasing: int) -> bool {
    i := 1
    if increasing > 0 {
        for i < len(res) {
            if res[i - 1] >= res[i] || res[i] - res[i - 1] > 3 {
                return false
            }
            i += 1
        }
    } else {
        for i < len(res) {
            if res[i] >= res[i - 1] || res[i - 1] - res[i] > 3 {
                return false
            }
            i += 1
        }
    }
    return true
}

p2 :: proc(contents: ^string) {
    safe_line_count := 0
    res_ints := make([dynamic]int)
    new_list := make([dynamic]int)
    res: []string
    temp: [dynamic]int
    ok: mem.Allocator_Error
    init_number, last_number, current_number: int
    increasing := 0
    failures := 0
    duplicate := 0
    filtered := 0
    for line in strings.split_lines_iterator(contents) {
        filtered = 0
        res, ok = strings.split(line, sep = " ")
        increasing = 0
        clear(&res_ints)
        last_number = strconv.atoi(res[0])
        current_number = 0
        append(&res_ints, last_number)
        i := 1
        for i < len(res) {
            current_number = strconv.atoi(res[i])
            if last_number > current_number {
                increasing -= 1
            } else if last_number < current_number {
                increasing += 1
            }
            last_number = current_number
            i += 1
            append(&res_ints, current_number)
        }

        if validate_list(res_ints, increasing) {
            safe_line_count += 1
            continue
        }
        for filtered < len(res_ints) {
            clear(&temp)
            append(&temp, ..res_ints[:filtered])
            append(&temp, ..res_ints[filtered + 1:])
            if validate_list(temp, increasing) {
                safe_line_count += 1
                break
            }
            filtered += 1
        }
    }
    fmt.println(safe_line_count)
}

main :: proc() {
    contents_raw, err := os.read_entire_file("input.txt", context.allocator)
    contents := string(contents_raw)
    c2 := contents

    p1(&contents)

    p2(&c2)

}
