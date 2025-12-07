use core::panic;
use std::error::Error;

use utils::Grid;

fn part_one(input: String) -> Result<u64, Box<dyn Error>> {
    let mut lines: Vec<&str> = input.lines().collect();

    let operations: Vec<&str> = lines.pop().expect("input should have at least one element").split_whitespace().collect();

    let numbers: Vec<Vec<u64>> = lines.iter().map(|l| l.split_whitespace().map(| s | s.parse().unwrap()).collect()).collect();

    let mut total = 0;
    for op in 0..numbers[0].len() {
        let mut res = match operations[op] {
            "*" => 1,
            "+" => 0,
            _ => panic!("unexpected operation type")
        };

        for n in 0..numbers.len() {
            match operations[op] {
                "*" => {res *= numbers[n][op]},
                "+" => {res += numbers[n][op]},
                _ => panic!("unexcpeted operation"),
            }
        }
        total += res;
    }
    Ok(total)
}

fn part_two(input: String) -> Result<u64, Box<dyn Error>> {
    let grid: Grid<char> = input.parse().unwrap();
    let mut numbers: Vec<String>= Vec::new();
    let mut op: Vec<String> = Vec::new();
    for y in 0..grid.max_y {
        let mut cur_number_str = String::new();
        for x in 0..grid.max_x {
            let el = &grid.grid[x][y];

            if *el == '*' || *el == '+'{
                op.push(el.to_string());
            }
            else if *el != ' ' {
                cur_number_str.push(*el);
            }
        }
        numbers.push(cur_number_str);
    }
    println!("{op:?}");
    let grouped_numbers: Vec<Vec<String>> = numbers.split(|s| s.is_empty()).map(|s| s.to_vec()).collect();
    println!("{grouped_numbers:?}");
    let mut res = 0;
    for (idx, group) in grouped_numbers.iter().enumerate() {
        res += group.iter().map(|s| s.parse::<u64>().unwrap()).reduce(|acc,e| {
            match op[idx].as_ref() {
                "*" => acc*e,
                "+" => acc+e,
                _ => panic!("unexpected op")
            }
        }).expect("should not be empty")
    }

    Ok(res)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 6).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 6).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}