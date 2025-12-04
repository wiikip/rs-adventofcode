use std::{error::Error, fmt::Display};

use thiserror::Error;



struct Range(u128, u128);


#[derive(Debug, Error)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn log10(n: u128) -> usize {
    let mut l = 0;
    let mut n_copy = n;
    while n_copy / 10 > 0 {
        n_copy = n_copy/10;
        l += 1;
    }
    return l + 1
}

fn find_invalid_id_1(range: Range) -> Vec<u128> {
    let r = range.0..range.1;

    r.into_iter().filter_map(|id| {
        if is_invalid_id_part_1(id) {
            Some(id)
        } else {
            None
        }
    }).collect()
}

fn find_invalid_id_2(range: Range) -> Vec<u128> {
    let r = range.0..range.1;

    r.into_iter().filter_map(|id| {
        if is_invalid_id_part_2(id) {
            Some(id)
        } else {
            None
        }
    }).collect()
}

fn is_invalid_id_part_2(id: u128) -> bool {
    let n_digit = log10(id) as u32;
    'outer: for possible_seq_length in 1..(n_digit/2+1) {
        let mut id_copy = id;
        if !(n_digit % possible_seq_length == 0) {
            continue
        }
        let mut prev = id_copy % u128::pow(10, possible_seq_length);
        while id_copy / u128::pow(10, possible_seq_length) > 0 {
            id_copy = id_copy / u128::pow(10, possible_seq_length);
            let next = id_copy % u128::pow(10, possible_seq_length);
            if next != prev {
                continue 'outer
            }
            prev = next
        }
        return true
    }
    false
}
fn is_invalid_id_part_1(id: u128) -> bool {
    let n_digit = log10(id) as u32;
    if n_digit % 2 != 0 {
        return false
    }

    let first = id / u128::pow(10,n_digit/2);
    let second = id % u128::pow(10,n_digit/2);
    if  first == second {
        true
    } else {
        false
    }

}


impl TryFrom<(&str,&str)> for Range {
    type Error = ParseError;
    fn try_from(value: (&str,&str)) -> Result<Self, Self::Error> {
        let a = value.0.parse::<u128>().map_err(|e| ParseError(e.to_string() + value.0))?;
        let b = value.1.parse::<u128>().map_err(|e| ParseError(e.to_string() + value.1))?;
        Ok(Range(a,b))
    }
}

fn part_one(input: String) -> Result<u128, Box<dyn Error>> {
    let ranges: Vec<(&str,&str)> = input.split(",").map(|s| {
        s.split_once("-").expect("invalid range")
    }).collect();

    let invalid_sum = ranges.into_iter().flat_map(|r| {
        let range = Range::try_from(r).expect("Failed parsing range");
        find_invalid_id_1(range)
    }).fold(0, |acc, e| {acc + e});



    // Write here code to solve part1 from input
    Ok(invalid_sum)
}

fn part_two(input: String) -> Result<u128, Box<dyn Error>> {
    let ranges: Vec<(&str,&str)> = input.split(",").map(|s| {
        s.split_once("-").expect("invalid range")
    }).collect();

    let invalid_sum = ranges.into_iter().flat_map(|r| {
        let range = Range::try_from(r).expect("Failed parsing range");
        let invalids = find_invalid_id_2(range);
        return invalids
    }).fold(0, |acc, e| {acc + e});



    // Write here code to solve part1 from input
    Ok(invalid_sum)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 2).unwrap();
        assert_eq!(part_one(input).unwrap(), 40214376723)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 2).unwrap();
        assert_eq!(part_two(input).unwrap(), 50793864718)
    }


    #[test]
    fn test_is_invalid_part_2(){
        let input = 121212;
        assert!(is_invalid_id_part_2(input));
        let input2 = 111111;
        assert!(is_invalid_id_part_2(input2));
        let input3 = 123;
        assert_ne!(is_invalid_id_part_2(input3), true);
        let input4 = 999;
        assert!(is_invalid_id_part_2(input4));
    }
}