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

    i := 0
    for line in strings.split_lines_iterator {
        for i < len(contents); i += 1 {
            if i < 
        }
    }
}

// p2 :: proc(contents: ^string) {
//
// }

main :: proc() {
    contents_raw, err := os.read_entire_file("sample.txt", context.allocator)
    contents := string(contents_raw)
    c2 := contents

    p1(&contents)

    // p2(&c2)

}
