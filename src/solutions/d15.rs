use std::{collections::HashSet, error::Error, fmt::Display, hash::Hash, str::FromStr, usize};


struct Game {
    robot: (usize,usize),
    moves: Vec<Direction>,
    grid: utils::Grid<Case>
}


fn dist(p1: &(usize,usize), p2: &(usize,usize)) -> usize {
    let dist_x = if p1.0 > p2.0 {
        p1.0 - p2.0
    } else {
        p2.0 - p1.0
    };
    let dist_y = if p1.1 > p2.1 {
        p1.1 - p2.1
    } else {
        p2.1 - p1.1
    };
    dist_x + dist_y
}

impl Game {

    fn try_move_box(&mut self, pos_box: (usize,usize), direction: &Direction) -> Option<()> {
        let mut new_boxes: HashSet<(usize,usize)> = HashSet::new();
        let mut boxes_to_move: Vec<(usize,usize)> = vec![pos_box];
        let mut grid_after_move: utils::Grid<Case> = self.grid.clone();
        while let Some(box_pos) = boxes_to_move.pop() {
            match direction {
                Direction::Up => {
                    match self.grid.grid[box_pos.0][box_pos.1] {
                        Case::BBoxRight => {
                            let next_pos_pushed = (box_pos.0 - 1, box_pos.1);
                            let next_pos_sticked = (box_pos.0 - 1, box_pos.1 - 1);

                            match self.grid.grid[next_pos_pushed.0][next_pos_pushed.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => boxes_to_move.push(next_pos_pushed),
                                Case::BBoxRight => boxes_to_move.push(next_pos_pushed),
                                _ => ()
                            };
                            match self.grid.grid[next_pos_sticked.0][next_pos_sticked.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => (),
                                Case::BBoxRight => boxes_to_move.push(next_pos_sticked),
                                _ => ()
                            };
                            if !new_boxes.contains(&box_pos) {
                                grid_after_move.grid[box_pos.0][box_pos.1] = Case::Empty;
                            }
                            if !new_boxes.contains(&(box_pos.0,box_pos.1 - 1)) {
                                grid_after_move.grid[box_pos.0][box_pos.1 - 1] = Case::Empty;
                            }

                            grid_after_move.grid[next_pos_pushed.0][next_pos_pushed.1] = Case::BBoxRight;
                            grid_after_move.grid[next_pos_sticked.0][next_pos_sticked.1] = Case::BBoxLeft;

                            new_boxes.insert(next_pos_pushed);
                            new_boxes.insert(next_pos_sticked);

                        },
                        Case::BBoxLeft => {
                            let next_pos_pushed = (box_pos.0 - 1, box_pos.1);
                            let next_pos_sticked = (box_pos.0 - 1, box_pos.1 + 1);
                            match self.grid.grid[next_pos_pushed.0][next_pos_pushed.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => boxes_to_move.push(next_pos_pushed),
                                Case::BBoxRight => boxes_to_move.push(next_pos_pushed),
                                _ => ()
                            };
                            match self.grid.grid[next_pos_sticked.0][next_pos_sticked.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => boxes_to_move.push(next_pos_sticked),
                                Case::BBoxRight => (),
                                _ => ()
                            };
                            if !new_boxes.contains(&box_pos) {
                                grid_after_move.grid[box_pos.0][box_pos.1] = Case::Empty;
                            }
                            if !new_boxes.contains(&(box_pos.0,box_pos.1 + 1)) {
                                grid_after_move.grid[box_pos.0][box_pos.1 + 1] = Case::Empty;
                            }
                            grid_after_move.grid[next_pos_pushed.0][next_pos_pushed.1] = Case::BBoxLeft;
                            grid_after_move.grid[next_pos_sticked.0][next_pos_sticked.1] = Case::BBoxRight;

                            new_boxes.insert(next_pos_pushed);
                            new_boxes.insert(next_pos_sticked);
                        },
                        _ => panic!("Trying to move a non box")
                    };
                },  
                Direction::Down => {
                    match self.grid.grid[box_pos.0][box_pos.1] {
                        Case::BBoxRight => {
                            let next_pos_pushed = (box_pos.0 + 1, box_pos.1);
                            let next_pos_sticked = (box_pos.0 + 1, box_pos.1 - 1);

                            match self.grid.grid[next_pos_pushed.0][next_pos_pushed.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => boxes_to_move.push(next_pos_pushed),
                                Case::BBoxRight => boxes_to_move.push(next_pos_pushed),
                                _ => ()
                            };
                            match self.grid.grid[next_pos_sticked.0][next_pos_sticked.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => (),
                                Case::BBoxRight => boxes_to_move.push(next_pos_sticked),
                                _ => ()
                            };
                            if !new_boxes.contains(&box_pos) {
                                grid_after_move.grid[box_pos.0][box_pos.1] = Case::Empty;
                            }
                            if !new_boxes.contains(&(box_pos.0,box_pos.1 - 1)) {
                                grid_after_move.grid[box_pos.0][box_pos.1 - 1] = Case::Empty;
                            }

                            grid_after_move.grid[next_pos_pushed.0][next_pos_pushed.1] = Case::BBoxRight;
                            grid_after_move.grid[next_pos_sticked.0][next_pos_sticked.1] = Case::BBoxLeft;

                            new_boxes.insert(next_pos_pushed);
                            new_boxes.insert(next_pos_sticked);

                        },
                        Case::BBoxLeft => {
                            let next_pos_pushed = (box_pos.0 + 1, box_pos.1);
                            let next_pos_sticked = (box_pos.0 + 1, box_pos.1 + 1);
                            match self.grid.grid[next_pos_pushed.0][next_pos_pushed.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => boxes_to_move.push(next_pos_pushed),
                                Case::BBoxRight => boxes_to_move.push(next_pos_pushed),
                                _ => ()
                            };
                            match self.grid.grid[next_pos_sticked.0][next_pos_sticked.1] {
                                Case::Wall => return None, // Impossible to move
                                Case::BBoxLeft => boxes_to_move.push(next_pos_sticked),
                                Case::BBoxRight => (),
                                _ => ()
                            };
                            if !new_boxes.contains(&box_pos) {
                                grid_after_move.grid[box_pos.0][box_pos.1] = Case::Empty;
                            }
                            if !new_boxes.contains(&(box_pos.0,box_pos.1 + 1)) {
                                grid_after_move.grid[box_pos.0][box_pos.1 + 1] = Case::Empty;
                            }
                            grid_after_move.grid[next_pos_pushed.0][next_pos_pushed.1] = Case::BBoxLeft;
                            grid_after_move.grid[next_pos_sticked.0][next_pos_sticked.1] = Case::BBoxRight;

                            new_boxes.insert(next_pos_pushed);
                            new_boxes.insert(next_pos_sticked);
                        },
                        _ => panic!("Trying to move a non box")
                    };
                },
                _ => panic!("Do not use")
            }
        }
        self.grid = grid_after_move;
        Some(())
    }

    fn scale(self) -> Self {
        let mut new_grid: Vec<Vec<Case>> = Vec::new();
        for _ in 0..(self.grid.max_x) {
            new_grid.push(vec![Case::Empty; 2*self.grid.max_y])
        }

        for x in 0..self.grid.max_x {
            for y in 0..self.grid.max_y {
                match self.grid.grid[x][y] {
                    Case::Empty => {
                        new_grid[x][2*y] = Case::Empty;
                        new_grid[x][2*y+1] = Case::Empty;
                    }
                    Case::Box => {
                        new_grid[x][2*y] = Case::BBoxLeft;
                        new_grid[x][2*y+1] = Case::BBoxRight;
                    }
                    Case::Robot =>{
                        new_grid[x][2*y] = Case::Robot;
                        new_grid[x][2*y+1] = Case::Empty;
                    }
                    Case::Wall => {
                        new_grid[x][2*y] = Case::Wall;
                        new_grid[x][2*y+1] = Case::Wall;
                    }
                    _ => panic!("Unexpected case")
                }
            }
        }
        let new_grid_struct = utils::Grid{grid:new_grid, max_x: self.grid.max_x, max_y:self.grid.max_y*2};
        let Some(new_robot) = find_robot(&new_grid_struct) else {
            panic!("No robot found")
        };

        Self{grid: new_grid_struct, robot:new_robot, moves: self.moves}
    }
    fn compute_gps_coordinates(&self) -> usize {
        let mut total = 0;
        for x in 0..self.grid.max_x {
            for y in 0..self.grid.max_y {
                total += match self.grid.grid[x][y] {
                    Case::Box | Case::BBoxLeft => 100 * x + y,
                    _ => 0
                }
            }
        }
        total
    }

    fn simulate(&mut self) {
        for direction in self.moves.clone() {
            self.move_robot(direction);

        }
    }

    fn simulate_scaled(&mut self) {
        for direction in self.moves.clone() {
            self.move_scaled_robot(direction);

        }
    }

    fn move_robot(&mut self, direction: Direction) {
        let Some(next_empty_case) = self.find_next_empty_case(&direction) else {
            return // Robot cannot move
        };
        self.grid.grid[next_empty_case.0][next_empty_case.1] = Case::Box;
        let next_robot_case = match direction {
            Direction::Down => (self.robot.0 + 1, self.robot.1),
            Direction::Up => (self.robot.0 - 1, self.robot.1),
            Direction::Right => (self.robot.0 , self.robot.1+ 1),
            Direction::Left => (self.robot.0 , self.robot.1 - 1)
        };
        self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
        self.grid.grid[next_robot_case.0][next_robot_case.1] = Case::Robot;
        self.robot = (next_robot_case.0, next_robot_case.1);
    }
    
    fn move_scaled_robot_left_to(&mut self, empty_case: (usize,usize)){
        for case_y in (empty_case.1)..(self.robot.1) {
            self.grid.grid[empty_case.0][case_y] = match (case_y-empty_case.1) % 2 {
                0 => Case::BBoxLeft,
                1 => Case::BBoxRight,
                _ => panic!("Shoud not happen")
            }
        }
        let new_robot = (self.robot.0, self.robot.1 - 1);
        self.grid.grid[new_robot.0][new_robot.1] = Case::Robot;
        self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
        self.robot = new_robot;
    }

    fn move_scaled_robot_right_to(&mut self, empty_case: (usize,usize)){
        for case_y in ((self.robot.1+1)..=empty_case.1).rev() {
            self.grid.grid[empty_case.0][case_y] = match (empty_case.1-case_y) % 2 {
                0 => Case::BBoxRight,
                1 => Case::BBoxLeft,
                _ => panic!("Shoud not happen")
            }
        }
        let new_robot = (self.robot.0, self.robot.1 + 1);
        self.grid.grid[new_robot.0][new_robot.1] = Case::Robot;
        self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
        self.robot = new_robot;
    }

    fn move_scaled_robot(&mut self, direction: Direction) {
        // No changement for move Left and Right, except we need to move all the boxes
        match direction {
            Direction::Left => {
                let Some(next_case) = self.find_next_empty_case(&direction) else {
                    return
                };
                self.move_scaled_robot_left_to(next_case);
            },
            Direction::Right => {
                let Some(next_case) = self.find_next_empty_case(&direction) else {
                    return
                };
                self.move_scaled_robot_right_to(next_case);
            },
            Direction::Up => {
                match self.grid.grid[self.robot.0 - 1][self.robot.1] {
                    Case::BBoxRight | Case::BBoxLeft => {
                        if let Some(_) = self.try_move_box((self.robot.0 - 1, self.robot.1), &direction) {
                            self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
                            self.grid.grid[self.robot.0-1][self.robot.1] = Case::Robot;
                            self.robot = (self.robot.0-1,self.robot.1)
                        }
                    }
                    Case::Empty => {
                        self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
                        self.grid.grid[self.robot.0-1][self.robot.1] = Case::Robot;
                        self.robot = (self.robot.0-1,self.robot.1)
                    },
                    _ => return
                }
            },
            Direction::Down => {
                match self.grid.grid[self.robot.0 + 1][self.robot.1] {
                    Case::BBoxRight | Case::BBoxLeft => {
                        if let Some(_) = self.try_move_box((self.robot.0 + 1, self.robot.1), &direction) {
                            self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
                            self.grid.grid[self.robot.0+1][self.robot.1] = Case::Robot;
                            self.robot = (self.robot.0+1,self.robot.1)
                        }
                    }
                    Case::Empty => {
                        self.grid.grid[self.robot.0][self.robot.1] = Case::Empty;
                        self.grid.grid[self.robot.0+1][self.robot.1] = Case::Robot;
                        self.robot = (self.robot.0+1,self.robot.1)
                    },
                    _ => return
                }
            },
        }
    }

    fn find_next_empty_case(&self, direction: &Direction) -> Option<(usize, usize)> {
        let mut case = self.robot.clone();
        loop {
            case = match direction {
                Direction::Down => (case.0 + 1, case.1 ),
                Direction::Up => (case.0 - 1, case.1 ),
                Direction::Right => (case.0 , case.1 + 1),
                Direction::Left => (case.0 , case.1 - 1),
            };
            
            match self.grid.grid[case.0][case.1] {
                Case::Empty => return Some(case),
                Case::Box => (),
                Case::BBoxLeft => (),
                Case::BBoxRight => (),
                Case::Wall => return None,
                _ => panic!("Found another robot wtf")
            };
        }
    } 
}

#[derive(Debug,Clone)]

enum Direction {
    Up,
    Down,
    Left,
    Right
}
#[derive(Debug)]

struct ParseDirectionError;
impl FromStr for Direction {
    type Err = ParseDirectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Self::Right),
            "<" => Ok(Self::Left),
            "v" => Ok(Self::Down),
            "^" => Ok(Self::Up),
            _ => return Err(ParseDirectionError)
        }
    }
}

#[derive(Debug, Clone)]

enum Case {
    Box,
    Robot,
    Wall,
    Empty,
    BBoxLeft,
    BBoxRight,
}

#[derive(Debug)]

struct ParseCaseError;
impl FromStr for Case {
    type Err = ParseCaseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@" => Ok(Self::Robot),
            "#" => Ok(Self::Wall),
            "." => Ok(Self::Empty),
            "O" => Ok(Self::Box),
            _ => return Err(ParseCaseError)
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let string_to_print = match self {
            Self::Box => "O",
            Self::Empty => ".",
            Self::Robot => "@",
            Self::Wall => "#",
            Self::BBoxLeft => "[",
            Self::BBoxRight => "]"
        };
        write!(f, "{string_to_print}")
    }
}

fn find_robot(grid: &utils::Grid<Case>) -> Option<(usize,usize)> {
    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            match grid.grid[x][y] {
                Case::Robot => return Some((x,y)),
                _ => ()
            }
        }
    }
    None
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let splitted_input: Vec<&str> = input.split("\n\n").collect();

    let grid: utils::Grid<Case> = splitted_input[0].parse().unwrap();
    let moves: Vec<Direction> = splitted_input[1].chars().filter(|c| *c != '\n').map(|c| c.to_string().parse::<Direction>().unwrap()).collect();
    let Some(robot) = find_robot(&grid) else {
        panic!("No robot found");
    };

    let mut game = Game{grid,moves, robot};
    game.simulate();


    Ok(game.compute_gps_coordinates())
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let splitted_input: Vec<&str> = input.split("\n\n").collect();

    let grid: utils::Grid<Case> = splitted_input[0].parse().unwrap();
    let moves: Vec<Direction> = splitted_input[1].chars().filter(|c| *c != '\n').map(|c| c.to_string().parse::<Direction>().unwrap()).collect();
    let Some(robot) = find_robot(&grid) else {
        panic!("No robot found");
    };

    let mut game = Game{grid,moves, robot};
    println!("Grid before simulation:\n {}", game.grid);
    println!("Robot position: {:?}", game.robot);
    let mut new_game = game.scale();

    println!("Grid after scale:\n {}", new_game.grid);
    new_game.simulate_scaled();
    println!("Grid after simulation:\n {}", new_game.grid);


    Ok(new_game.compute_gps_coordinates())
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(15).unwrap();
        assert_eq!(part_one(input).unwrap(), 1294459)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(15).unwrap();
        assert_eq!(part_two(input).unwrap(), 1319212)
    }
}