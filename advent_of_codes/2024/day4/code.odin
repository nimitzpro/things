package main
import "core:fmt"
import "core:math"
import "core:os"
import "core:strconv"
import "core:strings"
import "core:unicode/utf8"

check :: proc(str: string) -> int {
    if str == "XMAS" {
        return 1
    }
    return 0
}

p1 :: proc(contents: ^string) {
    safe_line_count := 0
    failed := false
    res := make([dynamic]string)
    init_number, last_number: int

    count := 0

    start := 0
    for i := 0; i < len(contents); i += 1 {
        if contents[i] == '\n' {
            append(&res, contents[start:i])
            start = i + 1
        }
    }

    for i := 0; i < len(res); i += 1 {
        for j := 0; j < len(res[i]); j += 1 {
            if res[i][j] == 'X' {
                // fmt.println("got to here", i, j)
                if j + 4 <= len(res[i]) {
                    count += check(res[i][j:j + 4])
                }
                if i + 4 <= len(res) {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i + 1][j],
                                cast(rune)res[i + 2][j],
                                cast(rune)res[i + 3][j],
                            },
                        ),
                    )
                }
                if i + 4 <= len(res) && j + 4 <= len(res[i]) {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i + 1][j + 1],
                                cast(rune)res[i + 2][j + 2],
                                cast(rune)res[i + 3][j + 3],
                            },
                        ),
                    )
                }
                if i + 4 <= len(res[i]) && j - 4 >= -1 {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i + 1][j - 1],
                                cast(rune)res[i + 2][j - 2],
                                cast(rune)res[i + 3][j - 3],
                            },
                        ),
                    )
                }

                if j - 4 >= -1 {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i][j - 1],
                                cast(rune)res[i][j - 2],
                                cast(rune)res[i][j - 3],
                            },
                        ),
                    )
                }
                if i - 4 >= -1 {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i - 1][j],
                                cast(rune)res[i - 2][j],
                                cast(rune)res[i - 3][j],
                            },
                        ),
                    )
                }
                if i - 4 >= -1 && j - 4 >= -1 {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i - 1][j - 1],
                                cast(rune)res[i - 2][j - 2],
                                cast(rune)res[i - 3][j - 3],
                            },
                        ),
                    )
                }
                if i - 4 >= -1 && j + 4 <= len(res[i]) {
                    count += check(
                        utf8.runes_to_string(
                            {
                                cast(rune)res[i][j],
                                cast(rune)res[i - 1][j + 1],
                                cast(rune)res[i - 2][j + 2],
                                cast(rune)res[i - 3][j + 3],
                            },
                        ),
                    )
                }
            }
        }
    }

    fmt.println(count)
}

check_mas :: proc(str: string) -> bool {
    return str == "MAS" || str == "SAM"
}

p2 :: proc(contents: ^string) {
    safe_line_count := 0
    failed := false
    res := make([dynamic]string)
    init_number, last_number: int

    count := 0

    start := 0
    for i := 0; i < len(contents); i += 1 {
        if contents[i] == '\n' {
            append(&res, contents[start:i])
            start = i + 1
        }
    }

    for i := 0; i < len(res); i += 1 {
        for j := 0; j < len(res[i]); j += 1 {
            if res[i][j] == 'S' || res[i][j] == 'M' {
                if i + 3 <= len(res) && j + 3 <= len(res[i]) {
                    if check_mas(
                           utf8.runes_to_string(
                               {
                                   cast(rune)res[i][j],
                                   cast(rune)res[i + 1][j + 1],
                                   cast(rune)res[i + 2][j + 2],
                               },
                           ),
                       ) &&
                       check_mas(
                           utf8.runes_to_string(
                               {
                                   cast(rune)res[i + 2][j],
                                   cast(rune)res[i + 1][j + 1],
                                   cast(rune)res[i][j + 2],
                               },
                           ),
                       ) {
                        count += 1
                    }
                }
            }
        }
    }

    fmt.println(count)
}

main :: proc() {
    contents_raw, err := os.read_entire_file("input.txt", context.allocator)
    contents := string(contents_raw)
    c2 := contents

    p1(&contents)

    p2(&c2)

}
