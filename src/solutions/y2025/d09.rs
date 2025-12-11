use std::error::Error;

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    let position: Vec<(usize, usize)> = input.lines().map(|line| {
        let (p1, p2) = line.split_once(",").unwrap();
        (p1.parse().unwrap(), p2.parse().unwrap())
    }).collect();

    let mut max = 0;
    for x in 0..position.len() {
        for y in x+1..position.len() {
            let px = position[x];
            let py = position[y];
            let area = (px.0.abs_diff(py.0) + 1) * (px.1.abs_diff(py.1) + 1);
            if area > max {
                max = area;
            }
        }
    }
    // Write here code to solve part1 from input
    Ok(max)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    let position: Vec<(usize, usize)> = input.lines().map(|line| {
        let (p1, p2) = line.split_once(",").unwrap();
        (p1.parse().unwrap(), p2.parse().unwrap())
    }).collect();
    let mut edges: Vec<(usize, usize, usize, usize)> = Vec::new();
    for i in 0..position.len() {
        edges.push((position[i].0, position[i].1, position[(i+1)%position.len()].0, position[(i+1)%position.len()].1));
    }

    let mut max = 0;
    for i in 0..position.len() {
        'pos: for j in i+1..position.len() {
            let (pxmin, pxmax) = if position[i].0 >= position[j].0 {
                (position[j].0, position[i].0)
            } else {
               (position[i].0, position[j].0) 
            };
            let (pymin, pymax) = if position[i].1 >= position[j].1 {
                (position[j].1, position[i].1)
            } else {
               (position[i].1, position[j].1) 
            };

            for edge in edges.iter() {
                let (exmin, exmax) = if edge.0 < edge.2 {
                    (edge.0, edge.2) 
                }else {
                    (edge.2, edge.0)
                };
                let (eymin, eymax) = if edge.1 < edge.3 {
                    (edge.1, edge.3) 
                }else {
                    (edge.3, edge.1)
                };

                if pxmin < exmax && pxmax > exmin && pymin < eymax && pymax > eymin {
                    continue 'pos;
                }
            }
            let area = (pxmax - pxmin + 1) * (pymax - pymin + 1);
            if area > max {
                max = area;
            } 

        }
    }
    // Write here code to solve part 2 from input
    Ok(max)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2025, 9).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2025, 9).unwrap();
        assert_eq!(part_two(input).unwrap(), 1539809693)
    }
}