use std::{collections::HashMap, error::Error};

type Stones = Vec<i64>;

fn blink(stones: Stones) -> Stones {
    stones.iter().flat_map(|stone| blink_one_stone(stone.clone())).collect()
}

fn blink_one_stone(stone: i64) -> Stones {
    if stone == 0 {
        return vec![1]
    }
    if (stone.ilog10() + 1) % 2 == 0 {
        let ten_pow = i64::pow(10, stone.ilog10()/2 + 1);
        return vec![stone / ten_pow, stone % ten_pow]
    }
    return vec![stone*2024]
}
fn get_n_stone_after_blink(stone: i64, blink: i64, cache: &mut HashMap<(i64,i64,), usize>) -> usize{
    if let Some(length) = cache.get(&(stone,blink)){
        return length.clone()
    };
    if blink == 0 {
        return 1
    }

    let blinked_stones = blink_one_stone(stone);
    blinked_stones.iter().map(|blinked_stone|{
        let blink_length = get_n_stone_after_blink(blinked_stone.clone(), blink-1, cache);
        cache.insert((blinked_stone.clone(), blink-1), blink_length);
        blink_length
    }).reduce(|acc,e| acc+e).unwrap()
}
fn parse_input(input: String) -> Stones {
    input.split(" ").map(|s| {s.parse::<i64>().unwrap()}).collect()
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let mut stones = parse_input(input);
    let mut cache: HashMap<(i64,i64),usize> = HashMap::new();
    Ok(stones.iter().map(|stone| get_n_stone_after_blink(stone.clone(), 25, &mut cache) ).reduce(|acc,e| acc+e).unwrap())
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
 // Write here code to solve part1 from input
 let mut stones = parse_input(input);
 let mut cache: HashMap<(i64,i64),usize> = HashMap::new();
 Ok(stones.iter().map(|stone| get_n_stone_after_blink(stone.clone(), 75, &mut cache) ).reduce(|acc,e| acc+e).unwrap())
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 11).unwrap();
        assert_eq!(part_one(input).unwrap(), 186203)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 11).unwrap();
        assert_eq!(part_two(input).unwrap(), 221291560078593)
    }
}