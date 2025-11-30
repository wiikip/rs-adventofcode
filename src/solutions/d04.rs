use std::error::Error;

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32
}

struct Table<T> {
    table: Vec<Vec<T>>,
    max_x: usize,
    max_y: usize
}

impl Position {
    fn move_to(&mut self, direction: &Direction) {
        self.x += match direction {
            Direction::NW => -1,
            Direction::E => 1,
            Direction::NE => 1,
            Direction::SE => 1,
            Direction::SW => -1,
            Direction::W => -1,
            _ => 0
        };
        self.y += match direction {
            Direction::N => -1,
            Direction::NE => -1,
            Direction::NW => -1,
            Direction::S => 1,
            Direction::SE => 1,
            Direction::SW => 1,
            _ => 0
        };
    }

    fn get_x_pos(&self, direction: Direction) -> Option<[Position; 2]> {
        match direction {
            Direction::NE => Some([Position{x: self.x - 1, y: self.y + 1}, Position{x: self.x + 1, y: self.y - 1}]),
            Direction::NW => Some([Position{x: self.x - 1 , y: self.y - 1}, Position{x: self.x + 1, y: self.y + 1}]),
            Direction::SE => Some([Position{x: self.x + 1, y: self.y + 1}, Position{x: self.x - 1, y: self.y - 1}]),
            Direction::SW => Some([Position{x: self.x + 1, y: self.y - 1}, Position{x: self.x - 1, y: self.y + 1}]),
            _ => None,
        }
    }

    fn check_x_mas(&mut self, table: &Table<char>, direction: Direction) -> bool {
        let Some(x_pos) = self.get_x_pos(direction) else {
            return false
        };
        let mas: Vec<char> = x_pos.iter()
            .map(|pos| {
                if pos.x < 0 || pos.y < 0 {
                    return 'O'
                } 
                let x: usize = pos.x.try_into().unwrap();
                let y: usize = pos.y.try_into().unwrap();
                if x >= table.max_x || y >= table.max_y {
                    return 'O'
                }
                table.table[x][y]
            }).collect();
        print!("{:?}", mas);
        mas[0] == 'M' && mas[1] == 'S'


    }
    fn check_xmas(&mut self, table: &utils::Grid<char>, direction: Direction) -> bool {
        let xmas = vec!['M','A','S'];

        match xmas.iter()
        .try_for_each(|c| {
            self.move_to(&direction);

            if self.x < 0 || self.y < 0 {
                return Err(())
            }
            let x: usize = self.x.try_into().unwrap();
            let y: usize = self.y.try_into().unwrap();

            if x >= table.max_x || y >= table.max_y || table.grid[x][y] != *c {
                return Err(())
            };
            Ok(())
        }) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
enum Direction {
    N,
    NW,
    NE,
    E,
    W,
    S,
    SE,
    SW
}

impl Direction {
   const VALUES: [Direction; 8] = [Direction::E, Direction::N, Direction::S, Direction::W, Direction::NE, Direction::NW, Direction::SE, Direction::SW];
}

fn parse_input(input: String) -> Table<char> {
    let lines = input.lines();
    let n_lines = lines.clone().count();
    let l_lines = lines.peekable().peek().unwrap().chars().count();
    let mut table:Vec<Vec<char>> = Vec::new();
    for _ in 0..l_lines {
        table.push(vec!['\0';n_lines]);
    };

    for (line_idx, line) in input.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            table[col_idx][line_idx] = char;
        }
    };
    return Table{table: table, max_x: l_lines, max_y: n_lines}
}

fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part1 from input

    let mut total = 0;
    let table: utils::Grid<char> = input.parse().unwrap();
    for x in 0..table.max_x {
        for y in 0..table.max_y {
            if table.grid[x][y] == 'X' {
                let pos = Position{x: x.try_into().unwrap(), y: y.try_into().unwrap()};
                print!("Position {} {}", pos.x, pos.y);
                for direction in Direction::VALUES {
                    if pos.clone().check_xmas(&table, direction) {
                        total += 1
                    }
                }
            }
            

        }
    }
    Ok(total)
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let mut total = 0;
    let table = parse_input(input);
    for x in 0..table.max_x {
        for y in 0..table.max_y {
            if table.table[x][y] == 'A' {
                let pos = Position{x: x.try_into().unwrap(), y: y.try_into().unwrap()};
                let mut n_ok_diag = 0;
                for direction in Direction::VALUES {
                    if pos.clone().check_x_mas(&table, direction) {
                        n_ok_diag += 1;
                    }

                }
                if n_ok_diag >= 2{
                    total += 1
                }
            }
            

        }
    }
    Ok(total)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(4).unwrap();
        assert_eq!(part_one(input).unwrap(), 2646)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(4).unwrap();
        assert_eq!(part_two(input).unwrap(), 2000)
    }
}