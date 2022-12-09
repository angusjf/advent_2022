def keep_up(h_x, h_y, t_x, t_y):
    dx = h_x - t_x
    dy = h_y - t_y
    touching = abs(dx) < 2 and abs(dy) < 2

    if not touching:
        same_row = h_x == t_x
        same_column = h_y == t_y

        if same_row:
            if dy > 1:
                return (t_x, t_y + 1)
            elif dy < -1:
                return (t_x, t_y - 1)
        elif same_column:
            if dx > 1:
                return (t_x + 1, t_y)
            elif dx < -1:
                return (t_x - 1, t_y)
        else:
            if dy > 0:
                if dx > 0:
                    return (t_x + 1, t_y + 1)
                if dx < 0:
                    return (t_x - 1, t_y + 1)
            elif dy < 0:
                if dx > 0:
                    return (t_x + 1, t_y - 1)
                if dx < 0:
                    return (t_x - 1, t_y - 1)
    else:
        return (t_x, t_y)

def one(input):
    visited = set()
    h_x = 0
    h_y = 0
    t_x = 0
    t_y = 0
    visited.add((0, 0))
    for line in input:
            c = line[0]
            n = int(line[2])
            for _ in range(0, n):
                if c == 'U':
                    h_y -= 1
                if c == 'L':
                    h_x -= 1
                if c == 'D':
                    h_y += 1
                if c == 'R':
                    h_x += 1

                (new_x, new_y) = keep_up(h_x, h_y, t_x, t_y)
                t_x = new_x
                t_y = new_y
                visited.add((t_x, t_y))
    return len(visited)

# fn print_grid(h_x: i32, h_y: i32, t_x: i32, t_y: i32, visited: &HashSet<(i32, i32)>):
#     let s = 30
#     for y in -s..=s:
#         for x in -s..s:
#             if (x, y) == (t_x, t_y):
#                 print!("T")
#             elif (x, y) == (h_x, h_y):
#                 print!("H")
#             elif (x, y) == (0, 0):
#                 print!("s")
#             elif visited.contains(&(x, y)):
#                 // print!("#")
#                 print!(".")
#             } else:
#                 print!(".")
#             }
#         }
#         println!()
#     }
#     println!()
# }

print(one(open("src/bin/input09.txt")))
