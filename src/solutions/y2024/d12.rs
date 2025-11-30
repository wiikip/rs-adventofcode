use std::{collections::{HashSet, VecDeque}, error::Error};

extern crate utils;

type Position = (usize, usize);

fn is_in_zone(zone: &HashSet<Position>, pos: &Position, side: &(i32,i32)) -> bool {
    let new_pos: (i32,i32) = (i32::try_from(pos.0).unwrap() + side.0, i32::try_from(pos.1).unwrap() + side.1);
    if new_pos.0 < 0 || new_pos.1 < 0 {
        return false
    };
    let new_pos: (usize,usize) = (new_pos.0.try_into().unwrap(), new_pos.1.try_into().unwrap());

    return zone.contains(&new_pos)
}

fn count_side(zone: &HashSet<Position>) -> usize {
    let sides = vec![(1,0), (0,1), (-1,0),(0,-1)];
    let mut n_side = 0;
    for point in zone {
        for side in 0..sides.len() {

            let side_1 = sides[side] ;
            let side_2 = sides[(side+1)%4];
            if !is_in_zone(zone, point, &side_1) && !is_in_zone(zone, point, &side_2) {
                n_side += 1;
            }
            if is_in_zone(zone, point, &side_1) && is_in_zone(zone, point, &side_2)  {
                let corner = (side_1.0 + side_2.0, side_1.1 + side_2.1);
                if !is_in_zone(zone, point, &corner){
                    n_side += 1;
                }
            }

        }
    }
    n_side
}
fn bfs_side_area(grid: &utils::Grid<char>, position: (usize, usize), visited: &mut HashSet<Position>) -> (usize,usize) {
    let mut queue: VecDeque<Position> = vec![position].into();
    let mut zone: HashSet<Position> = HashSet::new();
    let mut area = 0;
    while let Some(pos) = queue.pop_front() {
        if !visited.insert(pos.clone()) {
            continue
        }
        zone.insert(pos);
        area += 1;
        let neighbors = neighbors_fn(grid, &pos);
        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }

    (area, count_side(&zone))

}
fn bfs_perimeter_area(grid: &utils::Grid<char>, position: (usize, usize), visited: &mut HashSet<Position>
) -> (usize, usize) {
    let mut perimeter = 0;
    let mut area = 0;
    let mut queue: VecDeque<Position> = vec![position].into();

    while let Some(pos) = queue.pop_front() {
        if !visited.insert(pos.clone()) {
            continue
        }
        let neighbors = neighbors_fn(grid, &pos);
        area += 1;
        perimeter += 4 - neighbors.len();
        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }
    (perimeter, area)
}

fn  neighbors_fn(grid: &utils::Grid<char>, pos: &Position) -> Vec<Position>{
    let mut neighbors: Vec<Position> = Vec::new();
    if pos.0 > 0 && grid.grid[pos.0-1][pos.1] == grid.grid[pos.0][pos.1]{
        neighbors.push((pos.0-1,pos.1));
    };
    if pos.1 > 0 && grid.grid[pos.0][pos.1-1] == grid.grid[pos.0][pos.1]{
        neighbors.push((pos.0,pos.1-1));
    };
    if pos.0 < grid.max_x - 1 && grid.grid[pos.0+1][pos.1] == grid.grid[pos.0][pos.1]{
        neighbors.push((pos.0+1,pos.1));
    };
    if pos.1 < grid.max_y - 1 && grid.grid[pos.0][pos.1+1] == grid.grid[pos.0][pos.1]{
        neighbors.push((pos.0,pos.1+1));
    };
    neighbors
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let grid: utils::Grid<char> = input.parse().unwrap();




    
    let mut res = 0;
    let mut globally_visited: HashSet<Position> = HashSet::new();

    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if !globally_visited.contains(&(x,y)) {
                let (perimeter, area) = bfs_perimeter_area(&grid, (x,y), &mut globally_visited);
                res += perimeter * area;
            }
        }
    }
    Ok(res)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let grid: utils::Grid<char> = input.parse().unwrap();
    let mut res = 0;
    let mut globally_visited: HashSet<Position> = HashSet::new();

    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if !globally_visited.contains(&(x,y)) {
                let (perimeter, area) = bfs_side_area(&grid, (x,y), &mut globally_visited);
                res += perimeter * area;
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
        let input = crate::load_day_input(2024, 12).unwrap();
        assert_eq!(part_one(input).unwrap(), 1477762)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 12).unwrap();
        assert_eq!(part_two(input).unwrap(), 923480)
    }
}