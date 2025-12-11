use std::{collections::{HashMap, HashSet}, error::Error};


#[derive(Debug)]
struct Position(usize, usize, usize);

impl Position {
    fn from_str(s: &str) -> Self {
        let mut pos = s.split(",").map(|s| s.parse::<usize>().unwrap());
        Position(pos.next().unwrap(), pos.next().unwrap(), pos.next().unwrap())
    }

    fn dist_sq(&self, rhs: &Position) -> usize {
        (self.0 - rhs.0).pow(2) + (self.1 - rhs.1).pow(2) + (self.2 - rhs.2).pow(2)
    }
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    let positions: Vec<Position> = input.lines().map(Position::from_str).collect();
    
    let (mut groups_size, _) = find_groups_for_n_links(&positions, 1000);
    groups_size.sort();
    // Write here code to solve part1 from input
    Ok(groups_size.into_iter().rev().take(3).reduce(|acc, e | acc*e).unwrap())
}


fn find_groups_for_n_links(positions: &Vec<Position>, n: usize) -> (Vec<usize>, (usize,usize)) {
    let mut distances: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            distances.push((positions[i].dist_sq(&positions[j]), i, j));
        }
    }
    distances.sort();
    let linked: Vec<(usize,usize)> = distances.iter().take(n).map(|d| (d.1, d.2)).collect();
    let last = linked.last().unwrap().clone();
    let mut matrix: HashMap<usize, Vec<usize>> = HashMap::new();
    for link in linked {
        matrix.entry(link.0).and_modify(|p| p.push(link.1)).or_insert(vec![link.1]);
        matrix.entry(link.1).and_modify(|p| p.push(link.0)).or_insert(vec![link.0]);
    }


    // Create connex components
    let mut seen: HashSet<usize> = HashSet::new();
    let mut groups_size: Vec<usize> = Vec::new();
    for node in 0..positions.len() {
        let mut curr_group_size = 1;
        if seen.contains(&node) {
            continue;
        };
        seen.insert(node);
        let mut next = vec![node];
        while let Some(next_node) = next.pop() {
            for neighbor in matrix.get(&next_node).unwrap_or(&vec![]) {
                if !seen.contains(neighbor) {
                    curr_group_size += 1;
                    next.push(*neighbor);
                    seen.insert(*neighbor);
                }
            }
        }
        groups_size.push(curr_group_size);
    }
    (groups_size, last)
}
fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let positions: Vec<Position> = input.lines().map(Position::from_str).collect();

    let  (mut i,mut j) = (0, 10000);
    while i<j {
        let mid = (i+j) /2;
        let (groups_size,_) = find_groups_for_n_links(&positions, mid);
        if groups_size[0] == 1000 {
            j = mid;
        }
        else {
            i = mid+1;
        }
    }
    let (_, last) = find_groups_for_n_links(&positions, i);
    println!("{:?}, {:?}", positions[last.0], positions[last.1] );
    // Write here code to solve part 2 from input
    Ok(positions[last.0].0 * positions[last.1].0)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 8).unwrap();
        assert_eq!(part_one(input).unwrap(), 67488)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 8).unwrap();
        assert_eq!(part_two(input).unwrap(), 3767453340)
    }
}