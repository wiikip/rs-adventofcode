use std::{error::Error, str::FromStr};

#[derive(Debug)]
enum Order {
    Increasing,
    Decreasing,
    Unknown
}
fn check_safety(line: Vec<&str>) -> Result<bool, <i32 as FromStr>::Err> {
    let mut order = Order::Unknown;
    let mut prev: i32 = str::parse(line[0])?;
    for idx in 1..line.len() {
        println!("Order {:?}", order);

        let curr: i32 = str::parse(line[idx])?;

        let delta = curr - prev;
        if delta > 0 {
            match order {
                Order::Unknown => order = Order::Increasing,
                Order::Decreasing => return Ok(false),
                _ => (),
            };
        }
        if delta < 0 {
            match order {
                Order::Unknown => order = Order::Decreasing,
                Order::Increasing => return Ok(false),
                _ => (),
            };
        }
        if !(i32::abs(delta) < 4 &&  i32::abs(delta) > 0 ){
            return Ok(false);
        }

        prev = curr
    };
    Ok(true)
    
}
fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let mut safe_report_count = 0;

    for line in input.lines() {
        let splitted_line: Vec<&str> = line.split(" ").collect();
        if check_safety(splitted_line)? {
            safe_report_count += 1
        }
    }
    Ok(safe_report_count)
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let mut safe_report_count = 0;

    for line in input.lines() {
        let splitted_line: Vec<&str> = line.split(" ").collect();
        for idx in 0..splitted_line.len(){
            let mut less_one_element = splitted_line.clone();
            less_one_element.remove(idx);
            if check_safety(less_one_element)? {
                safe_report_count += 1;
                break
            }
        }
    }
    Ok(safe_report_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 2).unwrap();
        assert_eq!(part_one(input).unwrap(), 486)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 2).unwrap();
        assert_eq!(part_two(input).unwrap(), 540)
    }
}