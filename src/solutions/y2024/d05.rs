use std::{collections::HashMap, error::Error, vec};

type Page = usize;
type Pages = Vec<Page>;
type PagesUpdates = Vec<Pages>;

type Rule = (Page, Page);
type Rules = Vec<Rule>;

// Graph with adjacent list
type Graph = HashMap<Page, Pages>;

fn create_graph(rules: &Rules, pages: &Pages) -> Graph {
    let mut graph: Graph = HashMap::new();
    for (node, child) in rules {
        if !pages.contains(&node) || !pages.contains(&child) {
            continue
        }
        let default = vec![];
        let mut node_children = graph.get(&node).unwrap_or(&default).clone();
        node_children.push(*child);
        graph.insert(*node, node_children.to_vec());
    }

    for page in pages {
        if !graph.contains_key(&page) {
            graph.insert(*page, vec![]);
        }
    }
    graph
}
struct PagePartialOrder {
    pages_positions: HashMap<Page, usize>

}

trait PartialOrder<T> {
    fn gt(&self, a: &T, b: &T) -> Option<bool>;
    
}
impl PagePartialOrder {
    fn new(pages: &Pages) -> Self {
        let mut p = Self{pages_positions: HashMap::new()};
        pages.iter().enumerate().for_each(|(idx, page)| {
            p.pages_positions.insert(*page, idx);
        });
        return p
    }
}

impl PartialOrder<Page> for PagePartialOrder {
    fn gt(&self, a: &Page, b: &Page) -> Option<bool>{
        if !self.pages_positions.contains_key(&a) || !self.pages_positions.contains_key(&b) {
            return None
        }
        return Some(self.pages_positions[a] > self.pages_positions[b])
    }
}

fn check_rules(rules: &Rules, order: impl PartialOrder<Page>) -> bool {
    match rules.iter().try_for_each(
        |(p1, p2)| {
            let Some(v) = order.gt(p2, p1) else { return Ok(()) };
            if !v {
                return Err(())
            }
            return Ok(())
        }
    ) {
        Ok(()) => true,
        Err(()) => false
    }

}

fn parse_input(input: String) -> (PagesUpdates, Rules) {
    let mut pages: PagesUpdates = Vec::new();
    let mut rules: Rules = Vec::new();

    let mut in_pages = false;
    input.lines().for_each(
        |line| {
            if line.len() == 0 {
                in_pages = true;
                return
            }
            if !in_pages {
                let line_split: Vec<Page> = line.split("|").map(| n | str::parse(n).unwrap()).collect();
                rules.push((line_split[0], line_split[1]));
            }
            else {
                let line_split: Pages = line.split(",").map(| p | str::parse(p).unwrap()).collect();
                pages.push(line_split);
            }
        }
    );
    (pages, rules)
}
fn dfs(graph: &Graph, node: Page, visited: &mut HashMap<Page,bool>, stack: &mut Vec<Page>) -> () {
    visited.insert(node, true);
    
    for child in graph.get(&node).unwrap() {
        if !visited.get(child).unwrap() {
            dfs(graph, *child, visited, stack);
        }
    };
    stack.push(node);
}
fn topological_sort(graph: Graph) -> Pages {
    let mut stack : Vec<Page> = Vec::new();
    let mut visited: HashMap<Page, bool> = HashMap::new();
    // Initialize visited
    graph.iter().for_each(|(node, _)| {
        visited.insert(*node, false);
    });

    for node in graph.clone() {
        if !visited.get(&node.0).unwrap() {
            dfs(&graph, node.0, &mut visited, &mut stack);
        }
    }
    stack.reverse();
    stack
}

fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let (pages, rules) = parse_input(input);
    let mut total = 0;
    for page in pages {
        let order = PagePartialOrder::new(&page);
        if check_rules(&rules, order) {
            let middle_page = page[page.len()/2];
            total += middle_page
        }
    };
    Ok(total)
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let (pages, rules) = parse_input(input);
    let mut total = 0;
    for page in pages {
        let graph = create_graph(&rules,&page);

        let order = PagePartialOrder::new(&page);
        if !check_rules(&rules, order) {
            let sorted_pages = topological_sort(graph);
            total += sorted_pages[sorted_pages.len()/2]
        }
    };
    Ok(total)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 5).unwrap();
        assert_eq!(part_one(input).unwrap(), 4578)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 5).unwrap();
        assert_eq!(part_two(input).unwrap(), 6179)
    }
}