use std::{collections::HashSet, error::Error};

struct Grid {
    grid: Vec<Vec<usize>>,
    max_x: usize,
    max_y: usize
}

type Position = (usize, usize);
fn parse_input(input: String) -> Grid {
    let lines = input.lines();
    let n_lines = lines.clone().count();
    let l_lines = lines.peekable().peek().unwrap().chars().count();
    let mut table:Vec<Vec<usize>> = Vec::new();
    for _ in 0..l_lines {
        table.push(vec![0;n_lines]);
    };

    for (line_idx, line) in input.lines().enumerate() {
        for (col_idx, char) in line.chars().enumerate() {
            table[col_idx][line_idx] = char.to_digit(10).unwrap().try_into().unwrap();
        }
    };
    return Grid{grid: table, max_x: l_lines, max_y: n_lines}
}

fn bfs_multi_trail(pos: Position, goal: usize, grid: &Grid) -> usize {
    let value = grid.grid[pos.0][pos.1];
    if value == goal {
        return 1
    }
    let mut score = 0;
    if pos.0 > 0 && grid.grid[pos.0-1][pos.1] == value + 1{
        score += bfs_multi_trail((pos.0-1,pos.1), goal, grid);
    };
    if pos.1 > 0 && grid.grid[pos.0][pos.1-1] == value + 1{
        score += bfs_multi_trail((pos.0,pos.1-1), goal, grid);
    };
    if pos.0 < grid.max_x - 1 && grid.grid[pos.0+1][pos.1] == value + 1{
        score += bfs_multi_trail((pos.0+1,pos.1), goal, grid);
    };
    if pos.1 < grid.max_y - 1 && grid.grid[pos.0][pos.1+1] == value + 1{
        score += bfs_multi_trail((pos.0,pos.1+1), goal, grid);
    };
    return score

}

fn bfs(pos: Position, goal: usize, grid: &Grid ,visited: &mut HashSet<Position>) -> usize {
    if visited.contains(&pos) {
        return 0
    }
    visited.insert(pos);
    let value = grid.grid[pos.0][pos.1];
    if value == goal {
        return 1
    }
    let mut score = 0;
    if pos.0 > 0 && grid.grid[pos.0-1][pos.1] == value + 1{
        score += bfs((pos.0-1,pos.1), goal, grid, visited);
    };
    if pos.1 > 0 && grid.grid[pos.0][pos.1-1] == value + 1{
        score += bfs((pos.0,pos.1-1), goal, grid, visited);
    };
    if pos.0 < grid.max_x - 1 && grid.grid[pos.0+1][pos.1] == value + 1{
        score += bfs((pos.0+1,pos.1), goal, grid, visited);
    };
    if pos.1 < grid.max_y - 1 && grid.grid[pos.0][pos.1+1] == value + 1{
        score += bfs((pos.0,pos.1+1), goal, grid, visited);
    };
    return score

}
fn compute_score(begining: Position, grid: &Grid, arrival: usize) -> usize {

    let mut visited:HashSet<Position> = HashSet::new();
    bfs(begining, arrival, grid, &mut visited)
}

fn compute_score_multi(begining: Position, grid: &Grid, arrival: usize) -> usize {
    bfs_multi_trail(begining, arrival, grid)
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let grid = parse_input(input);
    let score = grid.grid.iter().enumerate().map(|(col_idx, col)|
        col.iter().enumerate().map(|(line_idx, v)| {
            if *v == 0 {
                return compute_score((col_idx, line_idx), &grid, 9)
            }
            0
        }).reduce(|acc , e| acc + e).unwrap()).reduce(|acc , e| acc + e).unwrap();
    Ok(score)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let grid = parse_input(input);
    let score = grid.grid.iter().enumerate().map(|(col_idx, col)|
        col.iter().enumerate().map(|(line_idx, v)| {
            if *v == 0 {
                return compute_score_multi((col_idx, line_idx), &grid, 9)
            }
            0
        }).reduce(|acc , e| acc + e).unwrap()).reduce(|acc , e| acc + e).unwrap();
    Ok(score)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 10).unwrap();
        assert_eq!(part_one(input).unwrap(), 461)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 10).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}