use std::{collections::HashMap, error::Error, str::FromStr, string::ParseError};


fn parse_lists(input: String) -> Result<(Vec<i32>, Vec<i32>), <i32 as FromStr>::Err > {
    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        let splitted_line: Vec<&str> = line.split("   ").collect();
        
        let left_int: i32 = str::parse(splitted_line[0])?;
        let right_int: i32 = str::parse(splitted_line[1])?;

        left_numbers.push(left_int);
        right_numbers.push(right_int);

    };
    Ok((left_numbers, right_numbers))
}
fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part1 from input

    let ( mut left_numbers, mut right_numbers) = parse_lists(input)?;
    left_numbers.sort();
    right_numbers.sort();

    let mut total: i32 = 0;
    for i in 0..left_numbers.len() {
        total = total + i32::abs(left_numbers[i] - right_numbers[i]);
    }
    Ok(total)

}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let ( mut left_numbers, mut right_numbers) = parse_lists(input)?;

    let mut dict: HashMap<i32, i32> = HashMap::new();

    for n in right_numbers {
        let count = dict.entry(n).or_insert(0);
        *count += 1;
    };

    let mut total = 0;
    for m in left_numbers {
        let count = dict.entry(m).or_insert(0);
        total += m * *count;
    };
    Ok(total)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(1).unwrap();
        assert_eq!(part_one(input).unwrap(), 2756096)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(1).unwrap();
        assert_eq!(part_two(input).unwrap(), 23117829)
    }
}