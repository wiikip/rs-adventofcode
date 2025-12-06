use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};

use utils::HashGrid;

#[derive(Debug)]
struct Occupied(bool);


// Return error when not occupied
#[derive(Debug)]
struct ParseOccupiedError;

impl FromStr for Occupied {
    type Err = ParseOccupiedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@" => Ok(Occupied(true)),
            _ => Err(ParseOccupiedError)
        }
    }
}

impl Display for Occupied {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => f.write_str("@"),
            false => f.write_str(".")
        }
    }
}

fn neighbors(x: &i32, y: &i32) -> Vec<(i32,i32)> {
    vec![(x+1, *y), (x - 1,*y), (x+1, y+1), (*x,y+1), (x-1,y+1), (x+1,y-1), (*x,y-1), (x-1,y-1)]
}

fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    let grid: HashGrid<Occupied> = input.parse().unwrap();
    let mut res = 0;
    for (x,v) in &grid.grid {
        for (y, _) in v {
            let n_neigbhbors = neighbors(x, y).into_iter().filter(|(i,j)| grid.get(i, j).is_some()).collect::<Vec<_>>().len();
            if n_neigbhbors < 4 {
                res += 1;
            }
        }
    }
    // Write here code to solve part1 from input
    Ok(res)
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    let mut grid: HashGrid<Occupied> = input.parse().unwrap();
    let mut res = 0;
    loop {
    let mut removable = Vec::new();
    for (x,v) in &grid.grid {
        for (y, _) in v {
            let n_neigbhbors = neighbors(x, y).into_iter().filter(|(i,j)| grid.get(i, j).is_some()).collect::<Vec<_>>().len();
            if n_neigbhbors < 4 {
                res += 1;
                removable.push((*x,*y));
            }
        }
    }
    if removable.len() == 0 {break}
    for (i,j) in removable {
        grid.delete(&i, &j);
    }
    }
    // Write here code to solve part1 from input
    Ok(res)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 4).unwrap();
        assert_eq!(part_one(input).unwrap(), 1435)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 4).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}