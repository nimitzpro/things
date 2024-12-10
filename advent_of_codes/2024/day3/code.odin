package main
import "core:fmt"
import "core:mem"
import "core:os"
import "core:strconv"
import "core:strings"

p1 :: proc(contents: ^string) {
    safe_line_count := 0
    failed := false
    res: []string
    ok: mem.Allocator_Error
    init_number, last_number: int

    x1: int
    x2: int
    sum := 0
    temp: int
    token := 0
    for i := 0; i < len(contents); i += 1 {
        if token == 0 {
            if contents[i] == 'm' {
                // fmt.println("got to here:", contents[i:i + 2])
                if contents[i:i + 4] == "mul(" {
                    token = 1
                    fmt.println("parsed mul(...")
                    i += 3
                    continue
                }
            }
        } else {
            if contents[i] == ',' && token == 1 {
                token += 1
                fmt.println("parsed comma...")
                continue
            }
            if contents[i] == ')' && token == 2 {
                sum += (x1 * x2)
                x1, x2, token = 0, 0, 0
                fmt.println(
                    "parsed ), multiplying x1, x2 and adding to sum...",
                )
                continue
            }
            temp = int(contents[i]) - 48
            fmt.println("temp:", temp)
            if temp >= 0 && temp <= 9 {
                if token == 1 {
                    x1 *= 10
                    x1 += temp
                    fmt.println("parsed digit for x1...", temp, "->", x1)
                } else if token == 2 {
                    fmt.println("parsed digit for x2...")
                    x2 *= 10
                    x2 += temp
                }
            } else {
                fmt.println("invalid string, resetting...")
                x1, x2, token = 0, 0, 0
            }
        }
    }
    fmt.println(sum)
}

p2 :: proc(contents: ^string) {
    safe_line_count := 0
    failed := false
    res: []string
    ok: mem.Allocator_Error

    x1: int
    x2: int
    sum := 0
    temp: int
    token := 0
    execute := true
    for i := 0; i < len(contents); i += 1 {
        if i + 3 < len(contents) {
            if contents[i:i + 4] == "do()" {
                execute = true
                i += 3
                continue
            }
        }
        if i + 6 < len(contents) {
            if contents[i:i + 7] == "don't()" {
                execute = false
                x1, x2, token = 0, 0, 0
                i += 6
                continue
            }
        }
        if execute {
            if token == 0 {
                if contents[i] == 'm' {
                    // fmt.println("got to here:", contents[i:i + 2])
                    if contents[i:i + 4] == "mul(" {
                        token = 1
                        fmt.println("parsed mul(...")
                        i += 3
                        continue
                    }
                }
            } else {
                if contents[i] == ',' && token == 1 {
                    token += 1
                    fmt.println("parsed comma...")
                    continue
                }
                if contents[i] == ')' && token == 2 {
                    sum += (x1 * x2)
                    x1, x2, token = 0, 0, 0
                    fmt.println(
                        "parsed ), multiplying x1, x2 and adding to sum...",
                    )
                    continue
                }
                temp = int(contents[i]) - 48
                fmt.println("temp:", temp)
                if temp >= 0 && temp <= 9 {
                    if token == 1 {
                        x1 *= 10
                        x1 += temp
                        fmt.println("parsed digit for x1...", temp, "->", x1)
                    } else if token == 2 {
                        fmt.println("parsed digit for x2...")
                        x2 *= 10
                        x2 += temp
                    }
                } else {
                    fmt.println("invalid string, resetting...")
                    x1, x2, token = 0, 0, 0
                }
            }
        }
    }
    fmt.println(sum)
}

main :: proc() {
    contents_raw, err := os.read_entire_file("input.txt", context.allocator)
    contents := string(contents_raw)
    c2 := contents

    p1(&contents)

    p2(&c2)

}
