use std::{collections::HashSet, error::Error, i32::MAX, ops::Sub};

use itertools::max;

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let (ranges_lines, ingredients_lines) = input.split_once("\n\n").expect("invalid input");
    let ranges: Vec<(u64, u64)> = ranges_lines.lines()
        .map(|line| {
            let range_str = line.split_once("-").expect("invalid range");
            (range_str.0.parse().unwrap(), range_str.1.parse().unwrap())
            }
        )
        .collect();
    let ingredients: Vec<u64> = ingredients_lines.lines().map(|i| i.parse().unwrap()).collect();

    let fresh_ingredients: Vec<_> = ingredients.iter()
        .filter(|i| {
            ranges.iter().any(| range | **i <= range.1 && **i >= range.0)
        }).collect();
        

    Ok(fresh_ingredients.len())
}

fn part_two(input: String) -> Result<u64, Box<dyn Error>> {
    let (ranges_lines, _) = input.split_once("\n\n").expect("invalid input");
    let mut ranges: Vec<(u64, u64)> = ranges_lines.lines()
        .map(|line| {
            let range_str = line.split_once("-").expect("invalid range");
            (range_str.0.parse().unwrap(), range_str.1.parse().unwrap())
            }
        )
        .collect();

    ranges.sort();

    let mut idx = 0;
    while idx < ranges.len()-1 {
        let (a,b) = ranges[idx];
        let (c,d) = ranges[idx+1];
        // A-------B
        //            C--------D
        if c > b {
            idx += 1;

        // A------B
        //      C---------D
        // or
        // A-------B
        //   C----D
        } else {
            ranges[idx] = (a, b.max(d));
            ranges.remove(idx+1);
        }
    }
    
    let res = ranges.iter().fold(0, |acc, e| acc + e.1 - e.0 + 1);

    
    Ok(res)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 5).unwrap();
        assert_eq!(part_one(input).unwrap(), 798)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 5).unwrap();
        assert_eq!(part_two(input).unwrap(), 366181852921027)
    }
}