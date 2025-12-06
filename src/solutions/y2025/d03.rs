use std::error::Error;

fn part_one(input: String) -> Result<u64, Box<dyn Error>> {
    let res = input.lines().map(| line| {
        let batteries: Vec<u64> = line.chars().map(|c| c.to_string().parse::<u64>().expect("failed to parse string")).collect();
        find_joltage(batteries,2)
    }).fold(0, |acc, e| acc + e);
    Ok(res)
}

fn part_two(input: String) -> Result<u64, Box<dyn Error>> {
    let res = input.lines().map(| line| {
        let batteries: Vec<u64> = line.chars().map(|c| c.to_string().parse::<u64>().expect("failed to parse string")).collect();
        find_joltage(batteries,12)
    }).fold(0, |acc, e| acc + e);
    Ok(res)
}

fn find_joltage(j: Vec<u64>, n_bat: usize) -> u64 {
    let mut maxs: Vec<u64> = Vec::new();
    let mut prev_max_idx = 0;
    let mut first = true;
    for b in 0..n_bat {
        let mut max = 0;
        let mut max_idx = 0;
        let begining = if first {
            0
        } else {
            prev_max_idx + 1
        };

        for i in begining..j.len()-(n_bat-b-1) {
            if j[i] > max {
                max = j[i];
                max_idx = i
            }
        }
        prev_max_idx = max_idx;
        first = false;
        maxs.push(max);
    }
    maxs.iter().fold(0, | acc, e| acc*10 + e)
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 3).unwrap();
        assert_eq!(part_one(input).unwrap(), 17324)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 3).unwrap();
        assert_eq!(part_two(input).unwrap(), 171846613143331)
    }

    #[test]
    fn test_find_joltage(){
        let input = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        assert_eq!(find_joltage(input, 2), 98)
    }

    #[test]
    fn test_find_joltage_2(){
        let input = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];
        assert_eq!(find_joltage(input, 12), 987654321111)
    }
}