package main

import "core:os"
import "core:fmt"
import "core:strings"


read_file_by_lines_in_whole :: proc(filepath: string) -> [][]f32 {
	data, ok := os.read_entire_file(filepath, context.allocator)
	if !ok {
		return
	}
	defer delete(data, context.allocator)

	it := string(data)
    list := [^][^]f32{}
    i := 0
    j := 0
	for line in strings.split_lines_iterator(&it) {
        j = 0
    for var in line.split(',') {
            list[i][j] = var
            j += 1
        }
    i += 1
	}
    return list
}


main :: proc() {
    read_file_by_lines_in_whole("dataset-310405444.csv")
}

