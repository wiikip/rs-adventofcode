use std::{collections::{HashMap, HashSet}, error::Error, fmt::Display, str::FromStr};

use utils::Grid;

#[derive(Debug)]
enum Case {
    Empty,
    Splitter,
    Start,
    Beam(usize),
}

#[derive(Debug)]
struct ParseCaseError;

impl FromStr for Case {
    type Err = ParseCaseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "^" => Ok(Self::Splitter),
            "S" => Ok(Self::Start),
            "|" => Ok(Self::Beam(1)),
            _ => Err(ParseCaseError),
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Beam(s) => &format!("{s}"),
            Self::Empty => ".",
            Self::Splitter => "^",
            Self::Start => "S",
        };
        f.write_str(s)
    }
}

fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    let mut grid: Grid<Case> = input.parse().expect("invalid char found in the input");
    let mut to_simulate = vec![find_start(&grid)];
    let mut simulated: HashSet<(usize,usize)> = HashSet::new();
    let mut res = 0;
    while let Some((x,y)) = to_simulate.pop() {
        if simulated.contains(&(x,y)){
            continue;
        }
        if x+1 >= grid.max_x {
            continue
        }
        if let Case::Splitter = grid.grid[x+1][y] {
            res +=1;
            if y >= 1 {
                grid.grid[x+1][y-1] = Case::Beam(1);
                to_simulate.push((x+1,y-1));
            }
            if y < grid.max_y {
                grid.grid[x+1][y+1] = Case::Beam(1);
                to_simulate.push((x+1,y+1))
            }
        } else{
            grid.grid[x+1][y] = Case::Beam(1);
            to_simulate.push((x+1,y));
        }
        simulated.insert((x,y));
        println!("{}", grid)
    }


    Ok(res)
}

fn find_start(grid: &Grid<Case>) -> (usize,usize) {
    for x in 0..grid.max_x {
        for y in 0..grid.max_y {
            if let Case::Start = grid.grid[x][y] {
                return (x,y)
            }
        }
    }
    panic!("Should find start");
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let mut grid: Grid<Case> = input.parse().expect("invalid char found in the input");
    let mut hash_layer: HashMap<(usize,usize), usize> = HashMap::new();
    hash_layer.insert(find_start(&grid), 1);
    let mut to_simulate = vec![hash_layer];
    let mut res = 0;
    while let Some(layer) = to_simulate.pop() {
        let simulating = layer.clone();
        let mut next_layer = HashMap::new();
        for ((x,y), power) in simulating {
            if x+1 >= grid.max_x {
                res += power;
                continue
            }
            if let Case::Splitter = grid.grid[x+1][y] {
                if y >= 1 {
                    next_layer.entry((x+1,y-1)).and_modify(|p| *p += power).or_insert(power);
                    grid.grid[x+1][y-1] = Case::Beam(*next_layer.get(&(x+1,y-1)).unwrap());

                }
                if y < grid.max_y {
                    next_layer.entry((x+1,y+1)).and_modify(|p| *p += power).or_insert(power);
                    grid.grid[x+1][y+1] = Case::Beam(*next_layer.get(&(x+1,y+1)).unwrap());

                }
            } else{
                next_layer.entry((x+1,y)).and_modify(|p| *p += power).or_insert(power);
                grid.grid[x+1][y] = Case::Beam(*next_layer.get(&(x+1,y)).unwrap());

            }

        }
        println!("{grid}");
        if next_layer.len() > 0 {
                to_simulate.push(next_layer.clone());
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
        let input = crate::load_day_input(2025, 7).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 7).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}