use std::{error::Error, fmt::Display, str::FromStr};

use thiserror::Error;

struct Move(isize, Direction);


#[derive(Debug, Error)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}



impl FromStr for Move {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, n_str) = s.split_at(1);
        let direction = match dir_str.chars().nth(0).ok_or(ParseError("No first char".to_string()))? {
            'L' => Direction::Left,
            'R' => Direction::Right,
            s => return Err(ParseError(format!("Unknown direction {s}")))
        };
        let n = n_str.parse::<isize>().map_err(|err| ParseError(err.to_string()))?;
        Ok(Move(n, direction))
    }
}
enum Direction {
    Left,
    Right
}

// rotate return the new position and the number of time we reach 0
fn rotate(pos: isize, n: isize, d: &Direction) -> (isize, isize) {
    let new_pos = pos + n *  match d { Direction::Left => -1, Direction::Right => 1};
    let mut n_turn = new_pos.div_euclid(100).abs();
    println!("n {n_turn}");
    if pos == 0 && new_pos < 0{
        n_turn -= 1 
    };

    if new_pos <= 0 && new_pos.rem_euclid(100) == 0 {
        n_turn += 1 ;
    }

    let ret: (isize, isize) = (new_pos.rem_euclid(100), n_turn);
    println!("{ret:?}");
    return ret
}
fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    let mut res = 0;
    input.lines().map(|line| line.parse::<Move>()).collect::<Result<Vec<Move>,_>>()?.iter().fold(50, | acc, m| {
        let (new_pos, _) = rotate(acc, m.0, &m.1);
        if new_pos == 0 {
            res+=1;
        };
        new_pos
    });
   Ok(res)
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    let mut res = 0;
    input.lines().map(|line| line.parse::<Move>()).collect::<Result<Vec<Move>,_>>()?.iter().fold(50, | acc, m| {
        let (new_pos, n_turn) = rotate(acc, m.0, &m.1);
        res += n_turn;
        new_pos
    });
   Ok(res.try_into()?)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 1).unwrap();
        assert_eq!(part_one(input).unwrap(), 982)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 1).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}