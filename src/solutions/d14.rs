use std::{error::Error, fmt::{write, Display}, str::FromStr, thread::sleep, time::Duration};

use utils::Grid;

struct Game {
    max_x: i32,
    max_y: i32,
    robots: Vec<Robot>
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create a grid
        let mut grid: Vec<Vec<char>> = Vec::new();
        for _ in 0..self.max_x {
            grid.push(vec![' '; self.max_y.try_into().unwrap()]);
        };
        for robot in self.robots.clone() {
            let robot_pos_usize: (usize,usize) = (robot.pos.0.try_into().unwrap(), robot.pos.1.try_into().unwrap());
            grid[robot_pos_usize.0][robot_pos_usize.1] = 'X';
        }
        write!(f, "{}", Grid{grid, max_x: self.max_x.try_into().unwrap(), max_y: self.max_y.try_into().unwrap()})
    }
}

impl Game {
    fn simulate(&mut self, rounds: usize) {
        let new_robots: Vec<Robot> = self.robots.iter().map(|robot| self.move_robot(robot, rounds)).collect();
        self.robots = new_robots;
    }

    fn move_robot(&self, robot: &Robot, rounds: usize) -> Robot {
        let irounds = i32::try_from(rounds).unwrap();
        let mut robot_next = ((robot.pos.0 + irounds * robot.v.0) % self.max_x, 
                                      (robot.pos.1 + robot.v.1*irounds) % self.max_y);
        if robot_next.0 < 0 {
            robot_next.0 += self.max_x;
        }
        if robot_next.1 < 0 {
            robot_next.1 += self.max_y;
        }
        return Robot{pos: robot_next, v: robot.v}
    }

    fn count_robots_per_quadrants(&self) -> [usize;4] {
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for robot in self.robots.clone() {
            if robot.pos.0 > self.max_x / 2 && robot.pos.1 > self.max_y / 2 {
                q1 += 1
            }
            if robot.pos.0 < self.max_x / 2 && robot.pos.1 > self.max_y / 2 {
                q2 += 1
            }
            if robot.pos.0 < self.max_x / 2 && robot.pos.1 < self.max_y / 2 {
                q3 += 1
            }
            if robot.pos.0 > self.max_x / 2 && robot.pos.1 < self.max_y / 2 {
                q4 += 1
            }
        }
        [q1,q2,q3,q4]
    }
}




#[derive(Debug,Clone)]
struct Robot {
    pos: (i32,i32),
    v: (i32,i32)
}

#[derive(Debug)]
struct ParseRobotError;
impl FromStr for Robot {
    type Err = ParseRobotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // p=9,89 v=-73,-15
        let pos_v: Vec<&str> = s.split(" ").collect();
        let pos: Vec<i32> = pos_v[0].split("=").collect::<Vec<&str>>()[1].split(",").map(|f| f.parse::<i32>().unwrap()).collect();
        let vel: Vec<i32> = pos_v[1].split("=").collect::<Vec<&str>>()[1].split(",").map(|f| f.parse::<i32>().unwrap()).collect();
        Ok(Self{pos: (pos[0], pos[1]), v: (vel[0], vel[1])})
    }
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let robots: Vec<Robot> = input.lines().map(|s| s.parse::<Robot>().unwrap()).collect();
    let mut game = Game{robots, max_x: 101, max_y: 103};
    game.simulate(100);
    let q = game.count_robots_per_quadrants();
    let res: usize = q.iter().map(|f| f.clone()).reduce(|acc,e| acc*e).unwrap();
    Ok(res)
}

pub fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let robots: Vec<Robot> = input.lines().map(|s| s.parse::<Robot>().unwrap()).collect();
    let mut game = Game{robots, max_x: 101, max_y: 103};
    for i in 1..10000 {
        game.simulate(1);
        println!("Round {}\n\n\n\n\n\n\n\n\n\n\n\n", i);
        println!("{}", game);
        // sleep(Duration::new(0, 250000000));
    }
    let q = game.count_robots_per_quadrants();
    let res: usize = q.iter().map(|f| f.clone()).reduce(|acc,e| acc*e).unwrap();
    Ok(res)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(14).unwrap();
        assert_eq!(part_one(input).unwrap(), 228457125)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(14).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}