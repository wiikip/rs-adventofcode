use std::{collections::{HashMap, HashSet}, error::Error, str::FromStr};
type Position = (i32,i32);
type Frequency = char;

struct Grid {
    antennas: HashMap<Frequency, Vec<Position>>,
    max_x: i32,
    max_y: i32,
    antinodes: HashSet<Position>,
}

#[derive(Debug)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let n_lines = lines.clone().count();
        let l_lines = lines.clone().peekable().peek().unwrap().chars().count();
        let mut antennas:HashMap<Frequency, Vec<Position>> = HashMap::new();
    
        for (line_idx, line) in lines.enumerate() {
            for (col_idx, char) in line.chars().enumerate() {
                if char != '.' {
                    let default : Vec<Position> = Vec::new();
                    let mut positions = antennas.get(&char).unwrap_or(&default).clone();
                    positions.push((col_idx.try_into().unwrap(),line_idx.try_into().unwrap()));
                    antennas.insert(char, positions.to_vec());
                }
            }
        };
        Ok(Grid{antennas, max_x: l_lines.try_into().unwrap(), max_y: n_lines.try_into().unwrap(), antinodes: HashSet::new()})
    }
}

impl Grid{
    fn is_in_grid(&self, pos: Position) -> bool {
        return pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.max_x && pos.1 < self.max_y
    }
    fn count_valid_antinodes(self) -> i32{
        let mut total = 0;
        for antinode in self.antinodes.clone() {
            if self.is_in_grid(antinode) {
                total += 1;
            }
        }
        total
    }

    fn compute_all_antinodes(&mut self){
        for (_,antennas) in self.antennas.clone() {
            let antinodes = self.compute_all_antinodes_for_antennas(antennas);
            antinodes.iter().for_each(|antinode| {self.antinodes.insert(*antinode);});
        }
    }

    fn compute_antinodes(&mut self){
        for (_,antennas) in self.antennas.clone() {
            let antinodes = self.compute_antinodes_for_antennas(antennas);
            antinodes.iter().for_each(|antinode| {self.antinodes.insert(*antinode);});
        }
    }

    fn compute_antinodes_for_antennas(&self, position: Vec<Position>) -> Vec<Position> {
        let mut antinode_positions: Vec<Position> = Vec::new();
        for first_idx in 0..(position.len()-1) {
            for second_idx in (first_idx+1)..position.len() {
                antinode_positions.append(&mut self.compute_antinodes_for_two_antennas(position[first_idx], position[second_idx]));
            };
        };
        antinode_positions
    }

    fn compute_all_antinodes_for_antennas(&self, position: Vec<Position>) -> Vec<Position> {
        let mut antinode_positions: Vec<Position> = Vec::new();
        for first_idx in 0..(position.len()-1) {
            for second_idx in (first_idx+1)..position.len() {
                antinode_positions.append(&mut self.compute_all_antinodes_for_two_antennas(position[first_idx], position[second_idx]));
            };
        };
        antinode_positions
    }

    fn compute_antinodes_for_two_antennas(&self, p1: Position, p2: Position) -> Vec<Position> {
        let delta = (p1.0 - p2.0, p1.1 - p2.1);
        let antinode_1 = (p1.0 + delta.0, p1.1 +delta.1);
        let antinode_2 = (p2.0 - delta.0, p2.1 - delta.1);
        vec![antinode_1,antinode_2]

    }

    fn compute_all_antinodes_for_two_antennas(&self, p1: Position, p2:Position) -> Vec<Position> {
        let delta = (p1.0 - p2.0, p1.1 - p2.1);
        let pgcd = pgcd(delta.0, delta.1);
        let mut antinodes: Vec<Position> = Vec::new();
        let simplified_delta = (delta.0/pgcd, delta.1/pgcd);

        // Antinodes found by adding delta
        let mut antinode = p1;
        while self.is_in_grid(antinode) {
            antinodes.push(antinode);
            antinode = (antinode.0 + simplified_delta.0, antinode.1 + simplified_delta.1);
        };
        antinode = p1;
        while self.is_in_grid(antinode) {
            antinodes.push(antinode);
            antinode = (antinode.0 - simplified_delta.0, antinode.1 - simplified_delta.1)
        };
        antinodes
    }
}

fn pgcd(a:i32, b:i32) -> i32 {
    let op1 = i32::abs(a);
    let op2 = i32::abs(b);
    let mut x = op2;
    let mut y = op1;
    let mut rem = 0;
    if a > b {
        x = op1;
        y = op2;
    }
    rem = x%y;
    while rem > 0 {
        x = y;
        y = rem;
        rem = x % y;
    };
    y
}
fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    let mut grid: Grid = input.parse().unwrap();
    grid.compute_antinodes();

    Ok(grid.count_valid_antinodes())
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    let mut grid: Grid = input.parse().unwrap();
    grid.compute_all_antinodes();

    Ok(grid.count_valid_antinodes())
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_pgcd(){
        assert_eq!(pgcd(2,5), 1);
        assert_eq!(pgcd(21, 14), 7);
        assert_eq!(pgcd(-21,7), 7);
    }
    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(8).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(8).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}