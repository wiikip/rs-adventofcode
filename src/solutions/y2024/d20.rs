use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}, error::Error, hash::Hash, str::FromStr};

#[derive(Debug)]
enum Case {
    Empty,
    Wall,
    Exit,
    Start
}

type Position = (usize,usize);
#[derive(Debug)]
struct ParseCaseError;
impl FromStr for Case {
    type Err = ParseCaseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Wall),
            "." => Ok(Self::Empty),
            "E" => Ok(Self::Exit),
            "S" => Ok(Self::Start),
            _ => panic!("Unexpected char")
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

fn get_all_neighbors(grid: &utils::Grid<Case>,pos: (usize,usize), dist: usize) -> Vec<(usize,usize)> {
    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();
    let moves: Vec<(i64, i64)> = vec![(0,1), (1,0), (-1,0), (0,-1)];
    let mut to_process: HashSet<Position> = HashSet::new();
    to_process.insert(pos);
    let mut processing: HashSet<Position> = HashSet::new();
    for _ in 0..dist {
        processing.extend(to_process);
        to_process = HashSet::new();
        for node in processing.clone() {
            for m in &moves {
                if let Some(neighbor) = add_positions(node, m, grid.max_x, grid.max_y) {
                    to_process.insert(neighbor);
                    neighbors.insert(neighbor);
                }
            };
        }
    }
    neighbors.iter().filter(|node| match grid.grid[node.0][node.1] {
        Case::Empty | Case::Exit | Case::Start => true,
        _ => false
    }).map(|f| f.clone()).collect()
}

fn get_neighbors_2(grid: &utils::Grid<Case>,pos: (usize,usize)) -> HashSet<(usize,usize)> {
    let mut neighbors: HashSet<(usize, usize)> = HashSet::new();
    let moves: Vec<(i64, i64)> = vec![(0,2), (2,0), (-2,0), (0,-2),(1,1),(-1,1), (-1,-1),(1,-1)];
    for m in moves {
        if let Some(neighbor) = add_positions(pos, &m, grid.max_x, grid.max_y) {
            match grid.grid[neighbor.0][neighbor.1] {
                Case::Empty | Case::Start | Case::Exit => {neighbors.insert(neighbor);},
                _ => ()
            }
        }
    };
    neighbors
}



// Return all the nodes that are at a distance less than 20
// fn bfs_close_neighbors(grid: &utils::Grid<Case>, start: Position, dist:)

// Return all the shortest from start position
fn bfs(grid: &utils::Grid<Case>, start: Position) -> HashMap<Position, usize> {
    let mut dist: HashMap<Position,usize> = HashMap::new();
    let mut queue: VecDeque<Position> = VecDeque::new();
    dist.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for neighbor in get_all_neighbors(grid, node, 1) {
            if !dist.contains_key(&neighbor) {
                dist.insert(neighbor, dist.get(&node).unwrap() + 1);
                queue.push_back(neighbor);
            }
        }
    }
    dist
}

fn cheat(pos1: Position, pos2: Position, dist_start: &HashMap<Position,usize>, dist_end: &HashMap<Position,usize>) -> usize {
    let dist_x = match pos1.0.cmp(&pos2.0) {
        Ordering::Less => pos2.0 - pos1.0,
        _ => pos1.0 - pos2.0
    };
    let dist_y = match pos1.1.cmp(&pos2.1) {
        Ordering::Less => pos2.1 - pos1.1,
        _ => pos1.1 - pos2.1
    };
    dist_start.get(&pos1).unwrap() + dist_end.get(&pos2).unwrap() + dist_x + dist_y
}

fn find_start(grid: &utils::Grid<Case>) -> Position {
    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if let Case::Start = grid.grid[x][y] {
                return (x,y)
            }
        }
    }
    panic!("Not found start")
}

fn find_exit(grid: &utils::Grid<Case>) -> Position {
    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if let Case::Exit = grid.grid[x][y] {
                return (x,y)
            }
        }
    }
    panic!("Not found start")
}


fn part_one(input: String) -> Result<usize, Box<dyn Error>> {

    let grid:utils::Grid<Case> = input.parse().unwrap();
    let start = find_start(&grid);
    let exit = find_exit(&grid);
    
    let distances_from_start = bfs(&grid, start);
    let distances_from_end = bfs(&grid, exit);
    let no_cheat_path_dist = distances_from_start.get(&exit).unwrap();
    let mut cheated_between: HashSet<(Position,Position)> = HashSet::new();
    let mut res: usize = 0;
    let mut cool_cheat : Vec<(Position,Position)> = Vec::new();
    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if let Case::Empty = grid.grid[x][y] {
                for neighbor in get_all_neighbors(&grid, (x,y), 2) {
                    if cheated_between.contains(&((x,y),neighbor)) {
                        continue
                    }
                    let cheat_dist = cheat((x,y), neighbor, &distances_from_start, &distances_from_end);
                    if cheat_dist < *no_cheat_path_dist && no_cheat_path_dist - cheat_dist >= 100 {
                        res += 1;
                        cheated_between.insert(((x,y),neighbor));
                    }
                }
            }
        }
    }
    Ok(res)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let grid:utils::Grid<Case> = input.parse().unwrap();
    let start = find_start(&grid);
    let exit = find_exit(&grid);
    
    let distances_from_start = bfs(&grid, start);
    let distances_from_end = bfs(&grid, exit);
    let no_cheat_path_dist = distances_from_start.get(&exit).unwrap();
    let mut cheated_between: HashSet<(Position,Position)> = HashSet::new();
    let mut res: usize = 0;
    let mut cool_cheat : Vec<(Position,Position)> = Vec::new();
    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if let Case::Empty | Case::Start | Case::Exit = grid.grid[x][y] {
                for neighbor in get_all_neighbors(&grid, (x,y), 20) {
                    if cheated_between.contains(&((x,y),neighbor)) {
                        continue
                    }
                    cheated_between.insert(((x,y),neighbor));
                    let cheat_dist = cheat((x,y), neighbor, &distances_from_start, &distances_from_end);
                    if cheat_dist < *no_cheat_path_dist && no_cheat_path_dist - cheat_dist >= 100 {
                        res += 1;
                        cool_cheat.push(((x,y), neighbor));
                    }
                }
            }
        }
    }
    Ok(res)
}



#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 20).unwrap();
        assert_eq!(part_one(input).unwrap(), 1422)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 20).unwrap();
        assert_eq!(part_two(input).unwrap(), 1009299)
    }
}