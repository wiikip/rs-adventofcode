use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error, fmt::Display, hash::Hash, os::macos::raw::stat, str::FromStr};

type Position = (usize, usize);
type Move = (i64,i64);
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    N,
    S,
    W,
    E
}

impl Direction {
    fn to_move(&self) -> Move {
        match self {
            Self::E => (0,1),
            Self::N => (-1,0),
            Self::S => (1,0),
            Self::W => (0,-1)
        }
    }

    fn rotate_left(self) -> Self {
        match self {
            Self::E => Self::N,
            Self::N => Self::W,
            Self::W => Self::S,
            Self::S => Self::E
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::E => Self::S,
            Self::N => Self::E,
            Self::W => Self::N,
            Self::S => Self::W
        }
    }
}

#[derive(Debug)]
enum Case {
    Empty,
    Wall,
    Exit,
    Start
}

#[derive(Debug)]

struct ParseCaseError;
impl FromStr for Case {
    type Err = ParseCaseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Wall),
            "." => Ok(Self::Empty),
            "E" => Ok(Self::Exit),
            "S" => Ok(Self::Start),
            _ => return Err(ParseCaseError)
        }
    }
}
impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let string_to_print = match self {
            Self::Exit => "E",
            Self::Empty => ".",
            Self::Wall => "#",
            Self::Start => "S"
        };
        write!(f, "{string_to_print}")
    }
}

fn add_positions(pos1: Position, pos2: Move) -> Option<Position>{
    let ipos1: (i64,i64) = (i64::try_from(pos1.0).unwrap(), i64::try_from(pos1.1).unwrap());

    let new_pos = (ipos1.0 + pos2.0, ipos1.1 + pos2.1);
    if new_pos.0 < 0 || new_pos.1 < 0 {
        return None
    };
    Some((usize::try_from(new_pos.0).unwrap(), usize::try_from(new_pos.1).unwrap()))
}

fn get_neighbors(grid: &utils::Grid<Case>, state: State) -> Vec<(usize,State)> {
    // Return all neighbors with their cost


    let mut neighbors: Vec<(usize,State)> = Vec::new();

    if let Some(new_pos) = add_positions(state.position, state.direction.to_move()) {
        match grid.grid[new_pos.0][new_pos.1] {
            Case::Wall => (),
            _ => neighbors.push((1,State{position: new_pos, direction: state.direction})),
        }
    }

    neighbors.push((1000,State{position: state.position, direction: state.direction.rotate_left()}));
    neighbors.push((1000,State{position: state.position, direction: state.direction.rotate_right()}));
    neighbors
}

fn get_dist(state: &State, neighbor: (Position,Direction)) -> usize {
    if state.direction == neighbor.1 {
        1
    } else {
        1001
    }
}
fn count_node(prev: HashMap<State, Vec<State>>, exit: State ) -> usize {
    let mut nodes_visited: HashSet<Position> = HashSet::new();
    let mut states_visited: HashSet<State> = HashSet::new();
    let mut nodes_to_check = vec![exit];

    while let Some(node) = nodes_to_check.pop() {
        if states_visited.contains(&node) {
            continue;
        }
        nodes_visited.insert(node.position);
        states_visited.insert(node);
        if let Some(prevs) = prev.get(&node) {
            for prev in prevs {
                nodes_to_check.push(prev.clone());
            }
        }
    }

    nodes_visited.len()
}
fn dijkstra(grid: utils::Grid<Case>, start: Position) -> Option<(usize, HashMap<State, Vec<State>>)> {
    let mut hqueue: BinaryHeap<StateWithCost> = BinaryHeap::new();
    let mut dist: HashMap<State, usize> = HashMap::new();
    let mut prev: HashMap<State, Vec<State>> = HashMap::new();
    dist.insert(State{position: start, direction: Direction::E}, 0);

    hqueue.push(StateWithCost(0, State{position: start, direction: Direction::E}));

    while let Some(node) = hqueue.pop() {
        let case = &grid.grid[node.1.position.0][node.1.position.1];
        if let Case::Exit = case {
            return Some((node.0, prev))
        }

        let neighbors = get_neighbors(&grid, node.1);
        for neighbor in neighbors {
            let next_cost = node.0 + neighbor.0;
            if let Some(d) = dist.get(&neighbor.1) {
                if *d < next_cost {
                    continue
                }
                if *d == next_cost {
                    let default:Vec<State> = Vec::new();
                    let mut prev_for_pos = prev.get(&neighbor.1).unwrap_or(&default).clone();
                    prev_for_pos.push(node.1);
                    prev.insert(neighbor.1, prev_for_pos); 
                    hqueue.push(StateWithCost(next_cost, neighbor.1));
                    continue
                }
            }
            prev.insert(neighbor.1, vec![node.1]);
            dist.insert(neighbor.1, next_cost);
            hqueue.push(StateWithCost(next_cost, neighbor.1));
        }
    }
    None

}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    position: Position,
    direction: Direction,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct StateWithCost(usize, State);
impl Ord for StateWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.position.0.cmp(&other.1.position.0))
    }
}

impl  PartialOrd for StateWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let grid: utils::Grid<Case> = input.parse().unwrap();
    let (res,prev) = dijkstra(grid, (139,1)).unwrap();
    Ok(res)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let grid: utils::Grid<Case> = input.parse().unwrap();
    let (res,prev) = dijkstra(grid, (139,1)).unwrap();
    Ok(count_node(prev, State{position: (1, 139), direction: Direction::N}))
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(16).unwrap();
        assert_eq!(part_one(input).unwrap(), 83432)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(16).unwrap();
        assert_eq!(part_two(input).unwrap(), 467)
    }
}