use std::{collections::VecDeque, error::Error, ptr::eq};


type Res = usize;
type Val = usize;
type Vals = VecDeque<usize>;
type Equation = (Res, Vals);
type Equations = Vec<Equation>;

enum Operator {
    Plus,
    Mul,
    Concat,
}

fn compute(op1: usize, op2: usize, operator: &Operator) -> usize {
    match operator {
        Operator::Plus => op1 + op2,
        Operator::Mul => op1 * op2,
        Operator::Concat => op1 * usize::pow(10, usize::ilog10(op2)+1) + op2
    }
}
fn solve_equation(equation: Equation) -> Option<Res> {
    let mut l_op = equation.1.clone();
    let op1 = l_op.pop_front().unwrap();
    let available_operator = vec![Operator::Plus, Operator::Mul];

    if solvable(equation.0, op1, l_op, &available_operator){
        return Some(equation.0)
    }
    return None
}

fn solve_equation_2(equation: Equation) -> Option<Res> {
    let mut l_op = equation.1.clone();
    let op1 = l_op.pop_front().unwrap();
    let available_operator = vec![Operator::Plus, Operator::Mul, Operator::Concat];

    if solvable(equation.0, op1, l_op, &available_operator){
        return Some(equation.0)
    }
    return None
}

fn solvable(res: Res, op1: Val, mut l_op: Vals, available_operators: &Vec<Operator>) -> bool {

    let Some(op2) = l_op.pop_front() else {
        return res == op1
    };

    for operator in available_operators {
        let computed = compute(op1, op2, operator);
        if computed > res {
            continue
        }
        if solvable(res, computed, l_op.clone(), available_operators) {
            return true
        }
    };
    return false
}

fn parse_input(input: String) -> Equations {
    input.lines().map( |line| {
        let splitted_line: Vec<&str> = line.split(": ").collect();
        let res: Res = splitted_line[0].parse().unwrap();
        let vals: Vals = splitted_line[1].split(" ").map(|v| v.parse::<Val>().unwrap()).collect();
        (res, vals)
    }
    ).collect()
}
fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let mut total = 0;
    let equations = parse_input(input);
    total += equations.iter().map(|equation| solve_equation(equation.clone())).map(|sol| sol.unwrap_or(0)).reduce(|acc, e| acc+e).unwrap();
    Ok(total)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    // Write here code to solve part1 from input
    let mut total = 0;
    let equations = parse_input(input);
    total += equations.iter().map(|equation| solve_equation_2(equation.clone())).map(|sol| sol.unwrap_or(0)).reduce(|acc, e| acc+e).unwrap();
    Ok(total)}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 7).unwrap();
        assert_eq!(part_one(input).unwrap(), 2664460013123)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 7).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}