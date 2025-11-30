use std::{collections::{BTreeSet, HashMap, HashSet}, error::Error};
use itertools::Itertools;
type AdjMap = HashMap<String, HashSet<String>>;

fn parse_input(input: String) -> AdjMap {
    let mut adj_map: AdjMap = HashMap::new();
    for line in input.lines() {
        let computers: Vec<&str> = line.split("-").collect();

        adj_map.entry(computers[0].to_string()).and_modify(|f| {f.insert(computers[1].to_string());}).or_insert(HashSet::from([computers[1].to_string()]));
        adj_map.entry(computers[1].to_string()).and_modify(|f| {f.insert(computers[0].to_string());}).or_insert(HashSet::from([computers[0].to_string()]));
    }
    adj_map
}

fn bronkerbosch(r: BTreeSet<String>, mut p: BTreeSet<String>, mut x: BTreeSet<String>,graph: &HashMap<String, HashSet<String>>) -> HashSet<BTreeSet<String>> {
    let mut cliques: HashSet<BTreeSet<String>> = HashSet::new();
    if p.len() == 0 && x.len() == 0 {
        cliques.insert(r.clone());
    }
    while let Some(node) = p.pop_first() {
        let mut new_r = r.clone();
        new_r.insert(node.clone());
        let new_p: BTreeSet<String> = p.iter().filter(|n_p| {
            if let Some(neigh) = graph.get(&node) {
                return neigh.contains(*n_p)
            }
            return false
        }).map(|s| s.clone()).collect();
        let new_x: BTreeSet<String> = x.iter().filter(|n_x| {
            if let Some(neigh) = graph.get(&node) {
                return neigh.contains(*n_x)
            }
            return false
        }).map(|s| s.clone()).collect();
        let res = bronkerbosch(new_r, new_p, new_x, graph);
        cliques = cliques.union(&res).map(|s| s.clone()).collect();
        x.insert(node);
    }
    cliques
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let adj_map = parse_input(input);
    let vertices: BTreeSet<String> = adj_map.keys().map(|k| k.clone()).collect();
    let all_cliques = bronkerbosch(BTreeSet::new(), vertices, BTreeSet::new(), &adj_map);
    println!("All cliques: {:?}", all_cliques);
    let mut t_3_set: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    for cliques in all_cliques {
        t_3_set = t_3_set.union(&cliques.iter().permutations(3).map(|l| BTreeSet::from_iter(l.iter().map(|s| s.clone()).cloned())).unique().filter(|c| {
            for s in c {
                if s.starts_with("t") {
                    println!("Picking: {:?}", c);
                    return true
                }
            }
            return false
        }).collect()).cloned().collect();
    };
    Ok(t_3_set.len())
}
// t1 t2 t3 t4
// t1 t2 t3 
// t1 t2 t4
// t1 t3 t4
// t2 t3 t4
// t1 t2 3 4 5 6
// 2 parmi 5
// 2 parmi 4
// t1 t2 3
// t1 t2 4
// t1 t2 5
// t1 t2 6
// t1 3 4
// t1 3 5
// t1 3 6
// t1 4 5
// t1 4 6
// t1 5 6
// t2 3 4
// t2 3 5
// t2 3 6
// t2 4 5
// t2 4 6
// t2 5 6
//
//
//
// 1 2 3 4 5
// 1 2 3
// 1 3 4
// 1 4 5
// 1 2 4 
// 1 2 4
// 1 3 5
// 2 3 4
// 2 4 5
// 2 3 5 
//
// 1 2 3 4
// 1 2 3
// 1 2 4
// 1  
//
// aq,cg,yn
// aq,vc,wq x
// co,de,ka
// co,de,ta x
// co,ka,ta
// de,ka,ta x
// kh,qp,ub x
// qp,td,wh  x
// tb,vc,wq x
// tc,td,wh x
// td,wh,yn x
// ub,vc,wq x
//
fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let adj_map = parse_input(input);
    let vertices: BTreeSet<String> = adj_map.keys().map(|k| k.clone()).collect();
    let all_cliques = bronkerbosch(BTreeSet::new(), vertices, BTreeSet::new(), &adj_map);
    let mut max_clique = BTreeSet::new();
    let mut max = 0; 
    for clique in all_cliques {
        if clique.len() > max {
            max_clique = clique.clone();
            max = clique.len();
        }
    }
    println!("Max clique: {:?}", max_clique);
    Ok(max)
}
// bw,dr,du,ha,mm,ov,pj,qh,tz,uv,vq,wq,xw

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(23).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(23).unwrap();
        assert_eq!(part_two(input).unwrap(), 13)
    }
}