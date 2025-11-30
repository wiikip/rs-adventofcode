use core::hash;
use std::{collections::{hash_map::Entry, HashMap}, error::Error, rc::Rc};
use std::borrow::BorrowMut;
use std::cell::RefCell;

use itertools::Itertools;
#[derive(Debug)]

enum Op {
    XOR,
    AND,
    OR,
}

impl Op {
    fn call(&self, b1: bool,b2: bool) -> bool {
        match self {
            Self::AND => b1 && b2,
            Self::OR => b1 || b2,
            Self::XOR => b1 != b2
        }
    }
}
#[derive(Debug)]
enum Variable {
    Value(bool),
    Eval(Rc<RefCell<Variable>>, Op, Rc<RefCell<Variable>>),
    Unknown
}

impl Variable {
    fn compute(&self) -> bool {
        match self {
            Self::Value(b) => *b,
            Self::Eval(v1, o , v2 ) => {
                o.call(v1.borrow().compute(), v2.borrow().compute())
            },
            Self::Unknown => panic!("cannot compute unknown")
        }
    }
}
fn get_variable_map(input: &str) -> HashMap<String, Rc<RefCell<Variable>>> {
    let mut hashmap = HashMap::new();
    let splitted_input: Vec<&str> = input.split("\n\n").collect();
    for line in splitted_input[0].lines() {
        let parts:Vec<&str> = line.split(":").map(|f| f.trim()).collect();
        hashmap.insert(String::from(parts[0]), Rc::new(RefCell::new(Variable::Value(match parts[1] {"1" => true, "0" => false, _ => panic!("should be 0 or 1")}))));
    }
    for line in splitted_input[1].lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let v1 = hashmap.entry(String::from(parts[0])).or_insert(Rc::new(RefCell::new(Variable::Unknown)));
        let rc_v1 = Rc::clone(v1); 
        let v2 = hashmap.entry(String::from(parts[2])).or_insert(Rc::new(RefCell::new(Variable::Unknown)));
        let rc_v2 = Rc::clone(v2);

        match hashmap.entry(String::from(parts[4])) {
            Entry::Occupied(mut entry) => {
                (* entry.get_mut()).replace(Variable::Eval(rc_v1, match parts[1]{
                    "XOR" => Op::XOR,
                    "AND" => Op::AND,
                    "OR" => Op::OR,
                    _ => panic!("impossible")
                }, rc_v2));
            },
            Entry::Vacant(entry) => {entry.insert(Rc::new(RefCell::new(Variable::Eval(rc_v1, match parts[1]{
                "XOR" => Op::XOR,
                "AND" => Op::AND,
                "OR" => Op::OR,
                _ => panic!("impossible")
            }, rc_v2))));}
        };

    }
    hashmap
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let variables = get_variable_map(input.as_str());
 

    let res = compute_value("z", &variables);
    println!("{} + {} = {}", compute_value("x", &variables), compute_value("y", &variables),res);
    Ok(res)
}

fn compute_value(start: &str, hashmap: &HashMap<String, Rc<RefCell<Variable>>>) -> usize {
    let mut res: Vec<(String, bool)> = Vec::new();
    for v in hashmap {
        if v.0.starts_with(start) {
            res.push((v.0.clone(), v.1.borrow().compute()));
        }
    }
    res.sort_by(|a,b| a.0.cmp(&b.0));
    usize::from_str_radix(res.iter().map(|v| match v.1 {
        true => "1",
        false => "0"
    }).rev().join("").as_str(), 2).unwrap() 

}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let mut variables = get_variable_map(input.as_str());
    
    let wires : Vec<&String> = variables.iter().map(|f| f.0).filter(|s| !s.starts_with("z") && !s.starts_with("x") && !s.starts_with("y")).collect();
    println!("Number of wires: {}", wires.len());
    let res = compute_value("z", &variables);
    let x = compute_value("x", &variables);
    let y = compute_value("y", &variables); 
    println!("{} + {} = {} ({})", x, y,res, x+y);
    swap_entry(&mut variables, "sgr", "cfw");
    // swap_entry(&mut variables, "", "z01");

    println!("{} + {} = {} ({})", x,y,compute_value("z", &variables), x+y); 

    Ok(res)
}

fn swap_entry(hashmap: &mut HashMap<String, Rc<RefCell<Variable>>>, a: &str, b: &str) {
    let ka: *mut Rc<RefCell<Variable>> = hashmap.get_mut(a).unwrap();
    let kb = hashmap.get_mut(b).unwrap();
    unsafe {
        std::ptr::swap(ka, kb);
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(24).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(24).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}