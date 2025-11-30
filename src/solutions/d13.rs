use std::{error::Error, str::FromStr};

#[derive(Debug)]

struct Machine {
    prize: (usize,usize),
    button_a: (usize,usize),
    button_b: (usize,usize),
}

impl Machine {
    fn push_a(&self, n: usize) -> (usize,usize) {
        (n*self.button_a.0, n*self.button_a.1)
    }
    fn push_b(self, n: usize) -> (usize,usize) {
        (n*self.button_b.0, n*self.button_b.1)
    }

    fn find_winning_pushes(&self, max_pushes:usize) -> Vec<(usize,usize)> {
        let mut combination: Vec<(usize,usize)> = Vec::new();

        for a_push in 0..max_pushes {
            let push_a_score = self.push_a(a_push);
            if push_a_score.0 > self.prize.0 || push_a_score.1 > self.prize.1 {
                break;
            }
            let remaining = (self.prize.0 - push_a_score.0, self.prize.1 - push_a_score.1);
            if remaining.0 % self.button_b.0 == 0 && remaining.1 % self.button_b.1 == 0 && remaining.0 / self.button_b.0 == remaining.1 / self.button_b.1 {
                combination.push((a_push,remaining.0 / self.button_b.0));
            }
        }
        combination
    }

    fn solve_equation(&self) -> Option<(usize,usize)> {
    //
    // x_1*a + x_2*b = p_1
    // y_1*a + y_2*b = p_2
    //
    // y_2*x_1*a + y_2*x_2*b = y_2*p_1
    // x_2*y_1*a + x_2*y_2*b = x_2*p_2
    //
    //
    // x_1*a + y_1*b = p_1
    // (x_1*y_2 - x_2*y_1)*a = p_1*y_2 - p_2*x_2
        let x_1 = i64::try_from(self.button_a.0).unwrap();
        let x_2 = i64::try_from(self.button_b.0).unwrap();
        let y_1 = i64::try_from(self.button_a.1).unwrap();
        let y_2 = i64::try_from(self.button_b.1).unwrap();
        let p_1 = i64::try_from(self.prize.0).unwrap();
        let p_2 = i64::try_from(self.prize.1).unwrap();

        let lhs = x_1*y_2 - x_2*y_1;
        let rhs = p_1*y_2 - p_2*x_2;
        if rhs % lhs != 0 {
            return None
        }
        let a = rhs / lhs;

        if (p_1 - x_1*a) % (x_2) != 0 {
            return None
        }
        let b = (p_1 - x_1*a) / x_2;
        if a < 0 || b < 0 {
            return None
        }
        Some((usize::try_from(a).unwrap(), usize::try_from(b).unwrap()))
    }

    fn add_to_prize(&mut self, val: usize) {
        self.prize.0 += val;
        self.prize.1 += val;
    }

}

#[derive(Debug)]
struct ParseMachineError;
impl FromStr for Machine {
    type Err = ParseMachineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut button_a = (0,0);
        let mut button_b = (0,0);
        let mut prize =(0,0);
        for line in s.split("\n") {
            if line.starts_with("Button A") {
                button_a = parse_button_line(line);
            } else if line.starts_with("Button B") {
                button_b = parse_button_line(line);
            } else if line.starts_with("Prize") {
                prize = parse_prize_line(line)
            }
        }
        Ok(Self{prize, button_a, button_b})

    }


}
fn parse_prize_line(s: &str) -> (usize,usize) {
    let score_part = s.split(":").last().unwrap();
    let scores: Vec<usize> = score_part.split(",")
    .map(|p| p.trim().trim_start_matches("X=").trim_start_matches("Y="))
    .map(|s_score| s_score.parse::<usize>().unwrap()).collect();
    (scores[0],scores[1])
}

fn parse_button_line(s: &str) -> (usize,usize) {
    let score_part = s.split(":").last().unwrap();
    let scores: Vec<usize> = score_part.split(",")
    .map(|p| p.trim().trim_start_matches("X+").trim_start_matches("Y+"))
    .map(|s_score| s_score.parse::<usize>().unwrap()).collect();
    (scores[0],scores[1])
}


fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let machines: Vec<Machine> = input.split("\n\n").map(|s| s.parse::<Machine>().unwrap()).collect();
    let mut total = 0;
    for machine in machines {
        let Some((a,b)) = machine.solve_equation() else {
            continue
        };
        total += a*3 + b
    };

    Ok(total)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let machines: Vec<Machine> = input.split("\n\n").map(|s| s.parse::<Machine>().unwrap()).collect();
    let mut total = 0;
    for mut machine in machines {
        machine.add_to_prize(10000000000000);
        let Some((a,b)) = machine.solve_equation() else {
            continue
        };
        total += a*3 + b
    };

    Ok(total)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(13).unwrap();
        assert_eq!(part_one(input).unwrap(), 34393)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(13).unwrap();
        assert_eq!(part_two(input).unwrap(), 83551068361379)
    }
}