package main

import "core:fmt"
import "core:os"
import "core:strconv"
import "core:strings"

valid_line :: proc(
    line: ^[dynamic]int,
    rules: ^map[int][dynamic]int,
) -> (
    bool,
    int,
    int,
) {

    for i := 0; i < len(line); i += 1 {
        for j := 0; j < i; j += 1 {
            for k := 0; k < len(rules[line[i]]); k += 1 {
                if line[j] == rules[line[i]][k] {
                    return false, i, j, k
                }
            }
        }
    }
    return true, 0, 0, 0
}

p1 :: proc(contents: ^string) {
    sum := 0

    rules := make(map[int][dynamic]int)

    init := true
    for line in strings.split_iterator(contents, "\n") {
        if init {
            if line == "" {
                init = false
                for rule in rules {
                    for thing in rules[rule] {
                        fmt.print("rule:", rule, thing, " ")
                    }
                    fmt.println()
                }
                continue
            }
            pair := strings.split_n(line, "|", 2)
            fmt.println(pair)
            if len(rules[strconv.atoi(pair[0])]) == 0 {
                rules[strconv.atoi(pair[0])] = make([dynamic]int)
            }
            append(&rules[strconv.atoi(pair[0])], strconv.atoi(pair[1]))
        } else {
            line_array := strings.split(line, ",")
            line_ints := make([dynamic]int)
            resize(&line_ints, len(line_array))
            for i in line_array {
                append(&line_ints, strconv.atoi(i))
            }
            ok, _, _ := valid_line(&line_ints, &rules)
            if ok {
                sum += strconv.atoi(line_array[int(len(line_array) / 2)])
            }
        }
    }


    fmt.println(sum)
}

p2 :: proc(contents: ^string) {
    sum := 0

    rules := make(map[int][dynamic]int)

    init := true
    for line in strings.split_iterator(contents, "\n") {
        if init {
            if line == "" {
                init = false
                for rule in rules {
                    for thing in rules[rule] {
                    }
                    fmt.println()
                }
                continue
            }
            pair := strings.split_n(line, "|", 2)
            if len(rules[strconv.atoi(pair[0])]) == 0 {
                rules[strconv.atoi(pair[0])] = make([dynamic]int)
            }
            append(&rules[strconv.atoi(pair[0])], strconv.atoi(pair[1]))
        } else {
            line_array := strings.split(line, ",")
            line_ints := make([dynamic]int)
            resize(&line_ints, len(line_array))
            for i := 0; i < len(line_ints); i += 1 {
                line_ints[i] = strconv.atoi(line_array[i])
            }

            ok, i, j := valid_line(&line_ints, &rules)
            interim := 0
            for !ok {
                line_ints[i], line_ints[j] = line_ints[j], line_ints[i]
                ok, i, j = valid_line(&line_ints, &rules)
                interim = line_ints[int(len(line_ints) / 2)]
            }
            fmt.println(line_ints)
            sum += interim
        }
    }


    fmt.println(sum)
}

main :: proc() {
    contents_raw, err := os.read_entire_file_from_filename("input.txt")
    contents := string(contents_raw)
    c2 := contents

    p1(&contents)

    p2(&c2)
}
