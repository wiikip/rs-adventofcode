use std::{collections::{HashMap, VecDeque}, error::Error, fmt::Display, hash::Hash, usize, vec};

use utils::Grid;

type Position = (usize,usize);

#[derive(Clone)]
enum Case {
    Empty,
    Corrupted
}



impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Case::Corrupted => write!(f, "#"),
            Case::Empty => write!(f, ".")
        }
    }
}

fn add_positions(pos1: (usize,usize), pos2: &(i64,i64), max_x: usize, max_y: usize) -> Option<(usize,usize)> {
    let ipos1: (i64,i64) = (i64::try_from(pos1.0).unwrap(), i64::try_from(pos1.1).unwrap());
    let (imax_x, imax_y) = (i64::try_from(max_x).unwrap(), i64::try_from(max_y).unwrap());
    let new_pos = (ipos1.0 + pos2.0, ipos1.1 + pos2.1);
    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= imax_x || new_pos.1 >= imax_y{
        return None
    };
    Some((usize::try_from(new_pos.0).unwrap(), usize::try_from(new_pos.1).unwrap()))
}

fn get_neighbors(grid: &utils::Grid<Case>,pos: (usize,usize), dist: usize) -> Vec<(usize,usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let moves: Vec<(i64, i64)> = vec![(0,1), (1,0), (-1,0), (0,-1)];
    let mut to_process: Vec<Position> = vec![pos];
    let mut processing: Vec<Position> = Vec::new();
    for _ in 0..dist {
        processing.append(&mut to_process);
        for node in processing.clone() {
            for m in &moves {
                if let Some(neighbor) = add_positions(node, m, grid.max_x, grid.max_y) {
                    if let Case::Empty = grid.grid[neighbor.0][neighbor.1] {
                        to_process.push(neighbor);
                    }
                }
            };
        }
    }
    to_process
}
fn bfs(grid: &utils::Grid<Case>, start: (usize,usize), end: (usize,usize)) -> Option<usize> {
    let mut queue: VecDeque<(usize,usize)> = VecDeque::new();
    let mut dist: HashMap<(usize,usize), usize> = HashMap::new();
    dist.insert(start, 0);
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        for neighbor in get_neighbors(grid, node, 1) {
            if !dist.contains_key(&neighbor) {
                dist.insert(neighbor, dist.get(&node).unwrap() + 1);
                queue.push_back(neighbor);
            }
        }
    };
    dist.get(&end).copied()
}
fn init_empty_grid(height: usize, width: usize) -> utils::Grid<Case> {
    let mut table:Vec<Vec<Case>> = Vec::new();
    for _ in 0..height {
        table.push(vec![Case::Empty; width]);
    };
    Grid{grid: table, max_x:width, max_y:height}
}

fn read_bytes(s: &str, n: usize, grid: &mut utils::Grid<Case>) {
    s.lines().take(n).for_each(|s| {
        let coords: Vec<usize> = s.split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        grid.grid[coords[0]][coords[1]] = Case::Corrupted
    });
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    let mut grid = init_empty_grid(71, 71);
    read_bytes(input.as_str(), 1024, &mut grid);
    println!("Grid {}", grid);
    let res = bfs(&grid, (0,0), (70,70));
    Ok(res.unwrap())
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let mut failing_idx = 0;
    for bytes in 1024..3450 {
        let mut grid = init_empty_grid(71, 71);
        read_bytes(input.as_str(), bytes, &mut grid);
        let Some(_) = bfs(&grid, (0,0), (70,70)) else {
            failing_idx = bytes;
            break;
        };
    };

    Ok(failing_idx)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 18).unwrap();
        assert_eq!(part_one(input).unwrap(), 506)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 18).unwrap();
        assert_eq!(part_two(input).unwrap(), 2941)
    }
}