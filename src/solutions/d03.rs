use std::{error::Error, str::FromStr, sync::LazyLock, fmt};

use regex::Regex;

#[derive(Debug)]
enum Token {
    Do,
    Dont,
    Mul(i32, i32)
}

static MULREGEX: LazyLock<Regex> = LazyLock::new( || Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap());

#[derive(Debug, PartialEq, Eq)]
struct ParseTokenError;

impl fmt::Display for ParseTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid token")
    }
}
impl Error for ParseTokenError{}
impl FromStr for Token {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "do()" {
            return Ok(Self::Do);
        }
        if s == "don't()" {
            return Ok(Self::Dont);
        }

        let Some(cap) = MULREGEX.captures(s) else {
            return Err(ParseTokenError);
        };

        let first: i32 = cap.name("first").unwrap().as_str().parse().map_err(|_| ParseTokenError)?;
        let second: i32 = cap.name("second").unwrap().as_str().parse().map_err(|_| ParseTokenError)?;
        return Ok(Self::Mul(first, second))
    }


}

fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
    let captures = re.find_iter(&input);
    for capture in captures {
        let token: Token = capture.as_str().parse::<Token>()?;
        
        match token {
            Token::Do => (),
            Token::Dont => (),
            Token::Mul(x, y) => total += x*y ,
        }
    };
    Ok(total)
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")?;
    let mut enabled = 1;
    let captures = re.find_iter(&input);
    for capture in captures {
        let token: Token = capture.as_str().parse::<Token>()?;
        println!("TOKEN {:?}", token);
        match token {
            Token::Do => enabled = 1,
            Token::Dont => enabled = 0,
            Token::Mul(x, y) => total += x*y*enabled ,
        }
    };
    Ok(total)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(3).unwrap();
        assert_eq!(part_one(input).unwrap(), 187194524)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(3).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}