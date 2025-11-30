use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};
use cached::proc_macro::cached;

type Position = (usize,usize);
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum DoorKey {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}


#[derive(Debug)]
struct ParseDoorKeyError;
impl FromStr for DoorKey {
    type Err = ParseDoorKeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            _ => panic!("Unknown")

        }
    }
}

#[derive(Clone,Debug, PartialEq, Eq, Hash, Copy)]
enum RobotKey {
    A,
    Up,
    Down,
    Left,
    Right
}

impl Display for RobotKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f,"<"),
            Self::Right => write!(f,">"),
            Self::Up => write!(f,"^")
        }
    }
}
fn get_door_grid() -> HashMap<DoorKey,Position> {
    let mut hashmap: HashMap<DoorKey, Position> = HashMap::new();
    hashmap.insert(DoorKey::A, (3,2));
    hashmap.insert(DoorKey::Zero, (3,1));

    hashmap.insert(DoorKey::Seven, (0,0));
    hashmap.insert(DoorKey::Eight, (0,1));
    hashmap.insert(DoorKey::Nine, (0,2));
    hashmap.insert(DoorKey::Six, (1,2));
    hashmap.insert(DoorKey::Five, (1,1));
    hashmap.insert(DoorKey::Four, (1,0));
    hashmap.insert(DoorKey::Three, (2,2));
    hashmap.insert(DoorKey::Two, (2,1));
    hashmap.insert(DoorKey::One, (2,0));
    hashmap
}

fn get_robot_grid() -> HashMap<RobotKey, Position> {
    let mut hashmap: HashMap<RobotKey, Position> = HashMap::new();

    hashmap.insert(RobotKey::A, (0,2));
    hashmap.insert(RobotKey::Up, (0,1));
    hashmap.insert(RobotKey::Down, (1,1));
    hashmap.insert(RobotKey::Left, (1,0));
    hashmap.insert(RobotKey::Right, (1,2));
    hashmap
}

#[cached]
fn get_door_key_moves(start: DoorKey, end: DoorKey)-> Vec<RobotKey> {
    let door_grid = get_door_grid();
    let start_pos = door_grid.get(&start).unwrap();
    let end_pos = door_grid.get(&end).unwrap();
    let mut keys: Vec<RobotKey> = Vec::new();
    if start_pos.0 == 3 && end_pos.1 == 0 {
        if start_pos.0 > end_pos.0 {
            for _ in end_pos.0..start_pos.0 {
                keys.push(RobotKey::Up);
            }
        }
        if start_pos.1 > end_pos.1 {
            for _ in end_pos.1..start_pos.1 {
                keys.push(RobotKey::Left);
            }
        }
        return keys
    }
    if start_pos.1 == 0 && end_pos.0 == 3 {
        if start_pos.1 < end_pos.1 {
            for _ in start_pos.1..end_pos.1 {
                keys.push(RobotKey::Right);
            }
        }
        if start_pos.0 < end_pos.0 {
            for _ in start_pos.0..end_pos.0 {
                keys.push(RobotKey::Down);
            }
        }
        return keys
    }
    if start_pos.1 > end_pos.1 {
        for _ in end_pos.1..start_pos.1 {
            keys.push(RobotKey::Left);
        }
    }
    if start_pos.0 > end_pos.0 {
        for _ in end_pos.0..start_pos.0 {
            keys.push(RobotKey::Up);
        }
    } else {
        for _ in start_pos.0..end_pos.0 {
            keys.push(RobotKey::Down);
        }
    }
    if start_pos.1 < end_pos.1 {
        for _ in start_pos.1..end_pos.1 {
            keys.push(RobotKey::Right);
        }
    }
    keys
}

#[cached]
fn get_robot_key_moves(start: RobotKey, end: RobotKey) -> Vec<RobotKey> {
    let robot_grid = get_robot_grid();
    let start_pos = robot_grid.get(&start).unwrap();
    let end_pos = robot_grid.get(&end).unwrap();
    let mut keys: Vec<RobotKey> = Vec::new();
    if start_pos.0 == 0 && end_pos.1 == 0 {
        if start_pos.0 < end_pos.0 {
            for _ in start_pos.0..end_pos.0 {
                keys.push(RobotKey::Down)
            }
        }
        if start_pos.1 > end_pos.1 {
            for _ in end_pos.1..start_pos.1 {
                keys.push(RobotKey::Left);
            }
        }
        keys.push(RobotKey::A);
        return keys
    }

    if start_pos.1 == 0 && end_pos.0 == 0 {
        if start_pos.1 < end_pos.1 {
            for _ in start_pos.1..end_pos.1 {
                keys.push(RobotKey::Right);
            }
        }
        if start_pos.0 > end_pos.0 {
            for _ in end_pos.0..start_pos.0 {
                keys.push(RobotKey::Up);
            }
        }
        keys.push(RobotKey::A);
        return keys
    }

    if start_pos.1 > end_pos.1 {
        for _ in end_pos.1..start_pos.1 {
            keys.push(RobotKey::Left);
        }
    }
    if start_pos.0 > end_pos.0 {
        for _ in end_pos.0..start_pos.0 {
            keys.push(RobotKey::Up);
        }
    } else {
        for _ in start_pos.0..end_pos.0 {
            keys.push(RobotKey::Down);
        }
    }
    if start_pos.1 < end_pos.1 {
        for _ in start_pos.1..end_pos.1 {
            keys.push(RobotKey::Right);
        }
    }
    keys.push(RobotKey::A);
    keys
}

#[cached]
fn get_n_moves_after_robots(moves:Vec<RobotKey>, n_iter: usize) -> usize {
    if n_iter == 0 {
        return moves.len()
    }
    let mut total = 0;
    for (idx, m) in moves.iter().enumerate() {
        let mut prev_move = RobotKey::A;
        if idx > 0 {
            prev_move = moves[idx-1]
        }
        total += get_n_moves_after_robots(get_robot_key_moves(prev_move, *m),n_iter-1)
    };
    total
}

fn get_robot_type_for_door(s: &str) -> Vec<RobotKey> {
    let mut curr_pos = DoorKey::A;
    let mut keys: Vec<RobotKey> = Vec::new();

    for char in s.chars() {
        let door_key = char.to_string().parse().unwrap();
        keys.append(&mut get_door_key_moves_prime_cache(curr_pos, door_key));
        keys.push(RobotKey::A);
        curr_pos = door_key;
    }
    keys
}

fn solve(s: &str, robots: usize) -> usize {
    let door_keys = get_robot_type_for_door(s);
    println!("Door: {:?}", door_keys);

    get_n_moves_after_robots(door_keys, robots)
    }

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    let mut total = 0;

    let codes = input.lines().map(|f| {
        (f, f.trim_matches('A').parse::<usize>().unwrap())
    });
    for code in codes {
        let solved = solve(code.0,2);
        println!("Solved: {}", solved);
        total += solved * code.1;
    }
    Ok(total)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let mut total = 0;
    let mut cache: HashMap<(RobotKey,RobotKey), Vec<RobotKey>> = HashMap::new();
    let codes = input.lines().map(|f| {
        (f, f.trim_matches('A').parse::<usize>().unwrap())
    });
    for code in codes {
        total += solve(code.0,25) * code.1;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 21).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 21).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}