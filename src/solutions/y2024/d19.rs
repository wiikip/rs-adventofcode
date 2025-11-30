use std::{collections::HashSet, error::Error, str::FromStr};

struct Game {
    available_str: HashSet<String>,
    created_str: Vec<String>
}

#[derive(Debug)]
struct ParseGameError;
impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted_s: Vec<&str> = s.split("\n\n").collect();
        let available_str: HashSet<String> = splitted_s[0].split(",").map(|ss| String::from(ss.trim())).collect();
        let created_str: Vec<String> = splitted_s[1].split("\n").map(|ss| String::from(ss)).collect();
        Ok(Game{available_str, created_str})
    }
}


impl Game {
    fn solve_all(&self) -> usize {
        let mut res = 0;
        for word in self.created_str.clone() {
            if self.solve(&word) > 0 {
                res += 1;
            }
        }
        res
    }

    fn solve_all_with_combinaison(&self) -> usize {
        let mut res = 0;
        for word in self.created_str.clone() {
            res += self.solve(&word);
        }
        res
    }
    fn solve(&self, word: &str) -> usize {
        let length = word.chars().count();

        let mut possible = vec![0; length+1];
        possible[0] = 1;
        for i in 1..length+1 {
            for j in 0..i {
                if possible[j] > 0  && self.available_str.contains(&word[j..i]) {
                    possible[i] += possible[j];
                }
            }
        }
        possible[length]
    }
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let game: Game = input.parse().unwrap();
    
    Ok(game.solve_all())
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let game: Game = input.parse().unwrap();
    
    Ok(game.solve_all_with_combinaison())
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 19).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 19).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}