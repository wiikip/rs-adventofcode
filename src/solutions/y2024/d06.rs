use std::{collections::{HashMap, HashSet}, error::Error, hash::Hash};

#[derive(Clone)]
enum Case {
    Empty,
    Blocked,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Orientation {
    Up,
    Down,
    Left,
    Right
}

impl Orientation {
    fn rotate(&mut self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
}

#[derive(Clone)]
struct Guard {
    position: (usize, usize),
    orientation: Orientation
}
struct Grid {
    table: Vec<Vec<Case>>,
    max_x: usize,
    max_y: usize,
    guard: Guard
}

#[derive(Debug)]
enum GridError {
    OutOfGrid,
    Deadlock
}


impl Grid {
    //simulate one iteration and return the new guard instance
    fn simulate(&mut self) -> Result<Guard, GridError> {
        let current_guard_position_x: i32 = self.guard.position.0.try_into().unwrap();
        let current_guard_position_y: i32 = self.guard.position.1.try_into().unwrap();
        let next_guard_position: (i32, i32) = match self.guard.orientation{
            Orientation::Down => (current_guard_position_x, current_guard_position_y + 1),
            Orientation::Up => (current_guard_position_x, current_guard_position_y- 1),
            Orientation::Left => (current_guard_position_x - 1, current_guard_position_y),
            Orientation::Right => (current_guard_position_x + 1, current_guard_position_y),
        };
        // Guard goes out of the map
        if next_guard_position.0 < 0 || next_guard_position.1 < 0 {
            return Err(GridError::OutOfGrid)
        }
        let next_guard_position_x: usize = next_guard_position.0.try_into().unwrap();
        let next_guard_position_y: usize = next_guard_position.1.try_into().unwrap();

        if next_guard_position_x >= self.max_x || next_guard_position_y >= self.max_y {
            return Err(GridError::OutOfGrid)
        }

        match self.table[next_guard_position_x][next_guard_position_y] {
            Case::Blocked => self.guard.orientation = self.guard.orientation.rotate(),
            Case::Empty => self.guard.position = (next_guard_position_x, next_guard_position_y)
        }
        Ok(self.guard.clone())
    }
}

fn parse_input(input: &str) -> Grid {
    let lines = input.lines();
    let n_lines = lines.clone().count();
    let l_lines = lines.peekable().peek().unwrap().chars().count();
    let mut table:Vec<Vec<Case>> = Vec::new();
    for _ in 0..l_lines {
        table.push(vec![Case::Empty; n_lines]);
    };
    let mut guard = Guard{
        position: (0,0),
        orientation: Orientation::Up
    };

    for (line_idx, line) in input.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            table[col_idx][line_idx] = match char{
                '#' => Case::Blocked,
                '.' => Case::Empty,
                '^' => {guard.position = (col_idx,line_idx); Case::Empty},
                _ => panic!("Unhandled char found")
            };
        }
    };

    Grid{
        max_x: l_lines,
        max_y: n_lines,
        guard,
        table
    }

}
fn simulate_until_out_or_deadlock(mut grid: Grid, visited: &mut HashMap<(usize, usize), HashSet<Orientation>>) -> Result<usize, GridError> {
    let mut inside = true;
    add_pos_orientation(visited, grid.guard.position.clone(), grid.guard.orientation.clone());
    while inside {
        match grid.simulate() {
            Ok(new_guard) => {
                if !add_pos_orientation(visited, new_guard.position, new_guard.orientation){
                    return Err(GridError::Deadlock)
                }},
            Err(GridError::OutOfGrid) => inside = false,
            Err(err) => return Err(err)
        }
    }
    Ok(visited.len())
}
fn part_one(input: String) -> Result<usize, GridError> {
    let grid = parse_input(input.as_str());
    let mut visited: HashMap<(usize, usize), HashSet<Orientation>> = HashMap::new();
    simulate_until_out_or_deadlock(grid, &mut visited)
}

fn add_pos_orientation<T ,U>(visited: &mut HashMap<T, HashSet<U>>, position: T, value: U) -> bool
where 
  T: Hash + Eq,
  U: Hash + Eq + Clone,
{
    let default:HashSet<U> = HashSet::new();
    let mut visited_for_pos = visited.get(&position).unwrap_or(&default).clone();
    let pos_insert = visited_for_pos.insert(value);
    visited.insert(position, visited_for_pos);
    pos_insert
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    let grid = parse_input(input.as_str());
    let mut visited: HashMap<(usize, usize), HashSet<Orientation>> = HashMap::new();
    simulate_until_out_or_deadlock(grid, &mut visited).unwrap();

    let mut total = 0;

    let possible_block_positions: Vec<(usize,usize)> = visited.clone().iter().map(|(pos, _)| *pos).collect();
    for block_pos in possible_block_positions {
        let mut alternative_grid = parse_input(input.as_str());
        let mut visited: HashMap<(usize, usize), HashSet<Orientation>> = HashMap::new();

        alternative_grid.table[block_pos.0][block_pos.1] = Case::Blocked;
        match simulate_until_out_or_deadlock(alternative_grid, &mut visited){
            Err(GridError::Deadlock) => total += 1,
            _ => ()
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
        let input = crate::load_day_input(2024, 6).unwrap();
        assert_eq!(part_one(input).unwrap(), 5067)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 6).unwrap();
        assert_eq!(part_two(input).unwrap(), 1793)
    }
}