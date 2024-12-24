package main

import "core:fmt"
import "core:os"
import "core:strings"

Vector2 :: distinct [2]int

step :: proc(operand, operator: ^Vector2) {
    operand.x += operator.x
    operand.y += operator.y
}
step_back :: proc(operand, operator: ^Vector2) {
    operand.x -= operator.x
    operand.y -= operator.y
}

rotate :: proc(shift: ^Vector2) {
    if shift.y != 0 {
        shift.x, shift.y = -shift.y, -shift.x
    } else {
        shift.x, shift.y = shift.y, shift.x
    }
}

out_of_bounds :: proc(current: ^Vector2, height, width: int) -> bool {
    return(
        current.x < 0 ||
        current.x >= width ||
        current.y < 0 ||
        current.y >= height \
    )
}

p1 :: proc(content: ^[]u8) {
    content_str := string(content^)
    c: []string = strings.split(content_str, "\n")
    c = c[:len(c) - 1]

    shift := Vector2{0, -1}
    current := Vector2{1, 1}

    walked: [dynamic][dynamic]rune
    resize(&walked, len(c))
    for i := 0; i < len(c); i += 1 {
        resize(&walked[i], len(c[0]))
        for j := 0; j < len(c[i]); j += 1 {
            if c[i][j] == '^' {
                current = Vector2{j, i}
                walked[i][j] = '^'
            }
        }
    }

    attempted := false
    for !out_of_bounds(&current, len(c), len(c[0])) {
        step(&current, &shift)
        if out_of_bounds(&current, len(c), len(c[0])) {
            break
        }
        if c[current.y][current.x] == '#' {
            if attempted {
                break
            }
            step_back(&current, &shift)
            rotate(&shift)
            attempted = true
            fmt.println("rotated:", shift)
            continue
        }
        fmt.println("stepped:", current)
        attempted = false
        walked[current.y][current.x] = '^'
    }

    distincts := 0
    for i := 0; i < len(walked); i += 1 {
        for j := 0; j < len(walked[0]); j += 1 {
            if walked[i][j] == '^' {
                distincts += 1
            }
        }
    }

    fmt.println(distincts)
}

main :: proc() {
    content, ok := os.read_entire_file_from_filename("input.txt")

    p1(&content)
}
