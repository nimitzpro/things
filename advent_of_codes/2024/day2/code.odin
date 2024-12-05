package day2

import "core:fmt"
import "core:io"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

p1 :: proc(contents: ^string) {
    safe_line_count := 0
    for line in strings.split_lines_iterator(contents) {
        failed := false
        res, err := strings.split(line, sep = " ")
        init_number := strconv.atoi(res[0])
        last_number := strconv.atoi(res[1])
        increasing := false
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

p2 :: proc(contents: ^string) {
    safe_line_count := 0
    for line in strings.split_lines_iterator(contents) {
        failures := 0
        flips := 0
        res, err := strings.split(line, sep = " ")
        init_number := strconv.atoi(res[0])
        last_number := strconv.atoi(res[1])
        increasing := false
        if last_number == init_number {
            failures += 1
        } else if last_number > init_number {
            increasing = true
            if last_number - init_number > 3 {
                failures += 1
            }
        } else {
            if init_number - last_number > 3 {
                failures += 1
            }
        }

        for i := 2; i < len(res); i += 1 {
            current_number := strconv.atoi(res[i])
            if last_number == current_number {
                failures += 1
            } else if increasing {
                if last_number > current_number {
                    flips += 1
                    increasing = !increasing
                    if last_number - current_number > 3 {
                        failures += 1
                    }
                } else if current_number - last_number > 3 {
                    failures += 1
                }
            } else {
                if last_number < current_number {
                    flips += 1
                    increasing = !increasing
                    if current_number - last_number > 3 {
                        failures += 1
                    }
                } else if last_number - current_number > 3 {
                    failures += 1
                }
            }
            last_number = current_number
        }

        if flips > 2 {
            failures += 1
        }

        if failures > 1 {
            continue
        }
        safe_line_count += 1
    }
    fmt.println(safe_line_count)
}

main :: proc() {
    contents_raw, err := os.read_entire_file("input.txt", context.allocator)
    contents := string(contents_raw)

    p1(&contents)

    p2(&contents)

}
