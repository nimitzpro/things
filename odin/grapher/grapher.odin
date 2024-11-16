package vector_fields


import "core:fmt"
import math "core:math"
import "core:slice"
import time "core:time"
import rl "vendor:raylib"
// --- SIMPLE FUNCTIONS ---
sfx1 :: proc(x: f32) -> f32 {
	return -(x * x)
}
sfx2 :: proc(x: f32) -> f32 {
	return math.pow(math.e, x)
}
sfx3 :: proc(x: f32) -> f32 {
	return -2 * x
}
sfx4 :: proc(x: f32) -> f32 {
	return 0.5 * x
}


// --- VECTOR FUNCTIONS ---
// f := <e^x, -2y>
// fx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return math.pow(math.e, vec[0])
// }
// dfx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return fx(vec)
// }
// fy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return -2 * vec[1]
// }
// dfy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return fy(vec) / vec[1]
// }

// f := <-xy, y>
fx :: proc(vec: ^rl.Vector2) -> f32 {
	return -vec[0] * vec[1]
}
fy :: proc(vec: ^rl.Vector2) -> f32 {
	return vec[1]
}
dfx :: proc(vec: ^rl.Vector2) -> f32 {
	return -vec[1]
}
dfy :: proc(vec: ^rl.Vector2) -> f32 {
	return 1
}

// f := <y, 0>
// fx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return vec[1]
// }
// fy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 0
// }
// dfx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 0
// }
// dfy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 0
// }

// f := <-y, x>
// fx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return -vec[1]
// }
// fy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return vec[0]
// }
// dfx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 0
// }
// dfy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 0
// }

// f:= <x^2 * y, y - x*y^2>
// fx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return vec[0] * vec[0] * vec[1]
// }
// fy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return vec[1] - (vec[0] * vec[1] * vec[1])
// }
// dfx :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 2 * vec[0] * vec[1]
// }
// dfy :: proc(vec: ^rl.Vector2) -> f32 {
// 	return 1 - (2 * vec[0] * vec[1])
// }

div :: proc(vec: ^rl.Vector2) -> f32 {
	return dfx(vec) + dfy(vec)
}

User_Input :: struct {
	left_mouse_clicked:   bool,
	right_mouse_clicked:  bool,
	toggle_pause:         bool,
	mouse_world_position: i32,
	mouse_tile_x:         i32,
	mouse_tile_y:         i32,
}

Window :: struct {
	name:          cstring,
	width:         i32,
	height:        i32,
	fps:           i32,
	control_flags: rl.ConfigFlags,
}

Game :: struct {
	tick_rate: time.Duration,
	last_tick: time.Time,
	pause:     bool,
	colors:    []rl.Color,
	width:     i32,
	height:    i32,
}

World :: struct {
	width:  i32,
	height: i32,
}

HEIGHT: i32 = 1000
HALF_HEIGHT: i32 = HEIGHT / 2
WIDTH: i32 = 1000
HALF_WIDTH: i32 = WIDTH / 2
DELTA: f32 = 1000
N: i32 = 10000
SCALE: f32 = 100
ARROW_SCALE: f32 = 5
DELTA_P: i32 = 10 // (0.1)
calc_func :: proc(func: proc(x: f32) -> f32) -> []f32 {
	output_array := make_slice([]f32, 2 * N)

	for i: i32 = 0; i < 2 * N; i += 1 {
		output_array[i] = func(f32(i - N) / DELTA)
	}
	return output_array
}

draw_line :: proc(output_array: ^[]f32, i: i32, color := rl.WHITE) {
	rl.DrawLine(
		HALF_WIDTH + i32(SCALE * f32(i - N - 1) / DELTA),
		HALF_HEIGHT - i32(SCALE * output_array[i - 1]),
		HALF_WIDTH + i32(SCALE * f32(i - N) / DELTA),
		HALF_HEIGHT - i32(SCALE * output_array[i]),
		color,
	)
}

draw_axes :: proc() {
	color := rl.WHITE
	AXIS_LINE_HEIGHT: i32 = 4
	rl.DrawLine(0, HALF_HEIGHT, WIDTH, HALF_HEIGHT, color) // x-axis
	for i: i32 = 0; i <= WIDTH; i += i32(SCALE) {
		rl.DrawLine(i, HALF_HEIGHT + AXIS_LINE_HEIGHT, i, HALF_HEIGHT - AXIS_LINE_HEIGHT, color)
	}
	rl.DrawLine(HALF_WIDTH, 0, HALF_WIDTH, HEIGHT, color) // y-axis
	for i: i32 = 0; i <= HEIGHT; i += i32(SCALE) {
		rl.DrawLine(HALF_WIDTH + AXIS_LINE_HEIGHT, i, HALF_WIDTH - AXIS_LINE_HEIGHT, i, color)
	}
}

// generate input as 2d array
// input := make([dynamic]([dynamic]f32), 10, 10)
// for i := 0; i < len(input); i += 1 {
// 	input[i] = make([dynamic]f32, 10, 10)
// 	for j := 0; j < 10; j += 1 {
// 		input[i][j] = f32(j) * SCALE
// 	}
// }
// draw_points :: proc(input: [dynamic][dynamic]f32) {
// 	for i := 0; i < len(input); i += 1 {
// 		for j := 0; j < len(input[0]); j += 1 {
// 			rl.DrawCircle(i32(f32(i) * SCALE), i32(f32(j) * SCALE), 2, rl.YELLOW)
// 		}
// 	}
// }
draw_points :: proc(input: ^[]rl.Vector2) {
	for i := 0; i < len(input); i += 1 {
		divergence := div(&[2]f32{input[i][0], input[i][1]})
		// fmt.println("divergence of ", input[i][0], " and ", input[i][1], " = ", divergence)
		color := rl.WHITE
		if divergence > 0 {
			color = rl.BLUE
		} else if divergence < 0 {
			color = rl.RED
		}

		xof := fx(&[2]f32{input[i][0], input[i][1]})
		yof := fy(&[2]f32{input[i][0], input[i][1]})

		vector_scale := math.sqrt(
			math.pow(math.abs(yof - input[i][1]), 2) + math.pow(math.abs(xof - input[i][0]), 2),
		)

		rl.DrawCircle(
			HALF_WIDTH + i32(input[i][0] * SCALE),
			HALF_HEIGHT - i32(input[i][1] * SCALE),
			2,
			color,
		)
		rl.DrawLine(
			HALF_WIDTH + i32(input[i][0] * SCALE),
			HALF_HEIGHT - i32(input[i][1] * SCALE),
			i32((f32(HALF_WIDTH) + (xof * ARROW_SCALE + input[i][0] * SCALE))),
			i32((f32(HALF_HEIGHT) - (yof * ARROW_SCALE + input[i][1] * SCALE))),
			color,
		)

		v1 := rl.Vector2(
			[2]f32 {
				(xof * (ARROW_SCALE + 2) + input[i][0] * SCALE),
				(yof * (ARROW_SCALE + 2) + input[i][1] * SCALE),
			},
		)
		pi_modifier: f32 = 0
		if (xof < input[i][0]) {
			pi_modifier = math.PI
		}
		v2 := rl.Vector2(
			[2]f32 {
				(input[i][0] * SCALE) +
				ARROW_SCALE *
					(xof +
							(math.cos(
										pi_modifier +
										math.atan(-(xof - input[i][0]) / (yof - input[i][1])),
									))),
				((input[i][1] * SCALE) +
					ARROW_SCALE *
						(yof +
								(math.sin(
											pi_modifier +
											math.atan(-(xof - input[i][0]) / (yof - input[i][1])),
										)))),
			},
		)
		v3 := rl.Vector2(
			[2]f32 {
				(input[i][0] * SCALE) +
				ARROW_SCALE *
					(xof -
							(math.cos(
										pi_modifier +
										math.atan(-(xof - input[i][0]) / (yof - input[i][1])),
									))),
				((input[i][1] * SCALE) +
					ARROW_SCALE *
						(yof -
								(math.sin(
											pi_modifier +
											math.atan(-(xof - input[i][0]) / (yof - input[i][1])),
										)))),
			},
		)
		points := []^rl.Vector2{&v1, &v2, &v3} // points counter clockwise for rl triangle
		crossz :=
			(points[1][0] - points[0][0]) * (points[2][1] - points[1][1]) -
			((points[1][1] - points[0][1]) * (points[2][0] - points[1][0]))
		if crossz < 0 { 	// if clockwise, reverse
			points[1], points[2] = points[2], points[1]
		}
		points[0][0] += f32(HALF_WIDTH)
		points[0][1] = f32(HALF_HEIGHT) - points[0][1]
		points[1][0] += f32(HALF_WIDTH)
		points[1][1] = f32(HALF_HEIGHT) - points[1][1]
		points[2][0] += f32(HALF_WIDTH)
		points[2][1] = f32(HALF_HEIGHT) - points[2][1]
		// rl.DrawCircleV(points[0]^, 2, rl.RED)
		// rl.DrawCircleV(points[1]^, 2, rl.GREEN)
		// rl.DrawCircleV(points[2]^, 2, rl.BLUE)
		rl.DrawTriangle(points[0]^, points[1]^, points[2]^, color)

		// fmt.println(input[i] * SCALE, points, sep = " | ")
	}
}

draw_simples :: proc(funcs: ^[](proc(x: f32) -> f32)) {
	colors := []rl.Color {
		rl.RED,
		rl.ORANGE,
		rl.YELLOW,
		rl.GREEN,
		rl.BLUE,
		rl.VIOLET,
		rl.PINK,
		rl.PURPLE,
	}
	for f := 0; f < len(funcs); f += 1 {
		f_array := calc_func(funcs[f])
		for i: i32 = 1; i < 2 * N; i += 1 {
			draw_line(&f_array, i, colors[int(f) % len(colors)])
		}
	}
}

draw_world :: #force_inline proc(world: ^World, colors: []rl.Color) {
	draw_axes()
	//draw_simples(&[](proc(x: f32) -> f32){sfx1, sfx2, sfx3, sfx4})
	input := make_slice([]rl.Vector2, 100)
	j := 0
	for i := 0; i < len(input); i += 1 {
		if i % 10 == 0 {
			j += 1
		}
		input[i] = [2]f32{(f32(i % 10) - 5), (f32(j) - 5)}
	}
	//input := []rl.Vector2{[2]f32{0, 0}, [2]f32{1, 0}, [2]f32{2, 0}, [2]f32{1, 1}, [2]f32{2, 2}}
	// input := []rl.Vector2 { 	// [[-30, 40], [-40, -30], [47.781746, 117.781746], [30, -40], [-110, -20]]
	// 	[2]f32{-30, 40},
	// 	[2]f32{-40, -30},
	// 	[2]f32{9.4974747, -37.071068},
	// 	[2]f32{-89.497475, -22.928932},
	// 	[2]f32{-50, -100},
	// }
	// for i := 0; i < len(input); i += 1 {
	// 	input[i][0] += 500
	// 	input[i][1] = 500 - input[i][1]
	// 	rl.DrawCircleV(input[i], 4, rl.WHITE)
	// }
	// rl.DrawLineV(input[0], input[1], rl.WHITE)
	// rl.DrawTriangle(input[2], input[3], input[4], rl.WHITE)
	draw_points(&input)
}

process_user_input :: proc(user_input: ^User_Input, window: Window, world: World) {
	m_pos := rl.GetMousePosition()
	mouse_x := i32((m_pos[0] / f32(window.width)) * f32(world.width))
	mouse_y := i32((m_pos[1] / f32(window.height)) * f32(world.height))

	if user_input.left_mouse_clicked || user_input.right_mouse_clicked {
		mouse_x %%= world.width
		mouse_y %%= world.height
	}

	user_input^ = User_Input {
		left_mouse_clicked   = rl.IsMouseButtonDown(.LEFT),
		right_mouse_clicked  = rl.IsMouseButtonDown(.RIGHT),
		toggle_pause         = rl.IsKeyPressed(.SPACE),
		mouse_world_position = i32(mouse_y * world.width + mouse_x),
		mouse_tile_x         = mouse_x,
		mouse_tile_y         = mouse_y,
	}
}

main :: proc() {
	window := Window{"Graphing Calculator", HEIGHT, WIDTH, 60, rl.ConfigFlags{.WINDOW_RESIZABLE}}

	game := Game {
		tick_rate = 1000 * time.Millisecond,
		last_tick = time.now(),
		pause     = true,
		colors    = []rl.Color{rl.BLACK, rl.WHITE},
		width     = 64,
		height    = 64,
	}

	world := World{game.width, game.height}
	next_world := World{game.width, game.height}
	user_input: User_Input

	rl.InitWindow(window.width, window.height, window.name)
	rl.SetWindowState(window.control_flags)
	// rl.SetTargetFPS(window.fps)
	rl.SetTargetFPS(60)
	for !rl.WindowShouldClose() {
		if rl.IsWindowResized() {
			window.width = rl.GetScreenWidth()
			window.height = rl.GetScreenHeight()
		}

		process_user_input(&user_input, window, world)

		if user_input.toggle_pause {
			game.pause = !game.pause
		}

		// Step 2: Update the world state
		// There is always a current state of the world that we read from
		// and a future state of the world that we write to
		if !game.pause && time.since(game.last_tick) > game.tick_rate {
			game.last_tick = time.now()
			// update_world(&world, &next_world)
			world, next_world = next_world, world
		}
		rl.BeginDrawing()
		rl.ClearBackground(rl.BLACK)
		draw_world(&world, game.colors)
		rl.EndDrawing()
	}
}
