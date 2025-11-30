use std::{collections::HashMap, error::Error, hash::Hash};

fn compute_next_secret_number(number: usize) -> usize {
    let n = prune(mix(number, number*64));
    let n2 = prune(mix(n, n/32));
    let n3 = prune(mix(n2, n2*2048));
    n3
}

fn compute_secret_number_n(number: usize, n: usize) -> usize {
    let mut num = number;
    for _ in 0..n {
        num = compute_next_secret_number(num);
    }
    num
}

fn compute_n_secret_number(number: usize, n: usize) -> Vec<usize> {
    let mut nums = vec![number];
    let mut num = number;
    for _ in 0..n {
        num = compute_next_secret_number(num);
        nums.push(num);
    }
    nums
}
fn mix(n1: usize, n2: usize) -> usize {
    n1 ^ n2
}

fn step(number: usize, m_times: usize) -> usize {
    let m = number*m_times;
    prune(mix(m, number))
}
fn prune(n: usize) -> usize {
    return n % 16777216
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    println!("1: {}", compute_secret_number_n(123, 1));

    let res =input.lines().map(|f| f.parse::<usize>().unwrap()).map(|f| compute_secret_number_n(f, 2000)).reduce(|acc, e| acc + e).unwrap();
    Ok(res)
}

fn compute_price_per_combination(prices: &Vec<usize>, deltas: &Vec<i64>) -> HashMap<(i64,i64,i64,i64), usize> {
    let mut res = HashMap::new();
    for i in 0..prices.len() {
        if i < 4 {
            continue;
        }
        let sequence = (deltas[i-4],deltas[i-3],deltas[i-2],deltas[i-1]);
        if res.contains_key(&sequence) {
            continue;
        }
        res.insert(sequence, prices[i]);
    };
    res
}

fn compute_sum_per_combination(prices_per_combination: Vec<HashMap<(i64,i64,i64,i64), usize>>) -> HashMap<(i64,i64,i64,i64), usize> {
    let mut res = HashMap::new();
    for ppc in prices_per_combination {
        for (seq, price) in ppc {
            res.entry(seq).and_modify(|e| *e += price).or_insert(price);
        }
    }
    res
}
fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let all_secret_numbers: Vec<Vec<usize>> = input.lines().map(|f| f.parse::<usize>().unwrap()).map(|f| compute_n_secret_number(f, 2000)).collect();
    println!("First prices:{:?}", all_secret_numbers[0].len());
    let mut all_prices = Vec::new();
    let mut all_deltas = Vec::new();
    for secrets in all_secret_numbers {
        let mut prices = Vec::new();
        let mut delta = Vec::new();
        for (idx, secret) in secrets.iter().enumerate() {
            prices.push(*secret%10);
            if idx > 0 {
                delta.push(i64::try_from(*secret%10).unwrap() - i64::try_from(secrets[idx-1]%10).unwrap())
            }
        };
        all_prices.push(prices);
        all_deltas.push(delta);
    }
    let mut prices_per_combination = Vec::new();
    for i in 0..all_deltas.len(){
        prices_per_combination.push(compute_price_per_combination(&all_prices[i], &all_deltas[i]));
    }

    let total_price_per_seq = compute_sum_per_combination(prices_per_combination);
    let (mut max, mut max_seq) = (0,0);
    for (seq, price) in total_price_per_seq {
        if price > max {
            max = price;
        }
    }
    Ok(max)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(22).unwrap();
        assert_eq!(part_one(input).unwrap(), 37327623)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(22).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}