use std::{collections::{HashMap, hash_map::Entry}, fmt::{Debug, Display}, str::FromStr};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Clone)]
pub struct Grid<T>{
    pub grid: Vec<Vec<T>>,
    pub max_x: usize,
    pub max_y: usize
}

#[derive(Debug)]
pub struct ParseGridError;

impl<T> Display for Grid<T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_lines: Vec<String> = self.grid.iter().map(|col| {
            let string_col: Vec<String> = col.iter().map(| el | format!("{}", el)).collect();
            string_col.join("")
        }
        ).collect();
        write!(f, "Grid:\n{}\n Size: {} x {}", string_lines.join("\n"), self.max_x, self.max_y)
    }
}

impl<T> FromStr for Grid<T>
where 
T: FromStr + Debug,
<T as FromStr>::Err: Debug
{
    type Err = ParseGridError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let n_lines = lines.clone().count();
        let l_lines = lines.clone().nth(0).unwrap().chars().count();
        let mut table:Vec<Vec<T>> = Vec::new();
        for l in lines {
            let mut line: Vec<T> = Vec::new();
            for char in l.chars() {
                line.push(char.to_string().parse().unwrap());
            }
            table.push(line);
        };

        Ok(Self{grid: table, max_x: n_lines, max_y:l_lines})
    }
}

// Grid backed by HashMap with m[x][y] = value
pub struct HashGrid<T> {
    pub grid: HashMap<i32, HashMap< i32, T>>,
    pub max_x: usize,
    pub max_y: usize 
}

impl<T> FromStr for HashGrid<T>
where 
T: FromStr + Debug,
<T as FromStr>::Err: Debug
{
    type Err = ParseGridError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let n_lines = lines.clone().count();
        let l_lines = lines.clone().nth(0).unwrap().chars().count();
        let mut table:HashMap<i32, HashMap<i32, T>> = HashMap::new();
        for (idx, l) in lines.enumerate() {
            for (idy,char) in l.chars().enumerate() {
                
                if let Ok(el) = char.to_string().parse::<T>() {
                    match table.entry(idx as i32) {
                        Entry::Occupied(mut h) => {h.get_mut().insert(idy as i32, el);},
                        Entry::Vacant(h) => {
                            let mut new_hashmap = HashMap::new();
                            new_hashmap.insert((idy as i32).clone(), el);
                            h.insert(new_hashmap);
                        }
                    }
                }; 
            };
        };

        Ok(Self{grid: table, max_x: n_lines, max_y:l_lines})
    }
}

impl<T> Display for HashGrid<T>
where T: Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        for i in 0..self.max_x {
            let mut col = vec![];
            for j in 0..self.max_y {
                if let Some(Some(el)) = self.grid.get(&(i as i32)).map(|c| c.get(&(j as i32))) {
                    col.push(format!("{}", el));
                } else {
                    col.push(format!(" "));
                }
            }
            lines.push(col.join(""));
        }
        write!(f, "Grid:\n{}\n Size: {} x {}", lines.join("\n"), self.max_x, self.max_y)
    }
}

impl<T> HashGrid<T> {
    pub fn get(&self, x: &i32, y: &i32) -> Option<&T> {
        let Some(h) = self.grid.get(x) else {
            return None
        };
        h.get(y)
    }

    pub fn delete(&mut self, x: &i32, y: &i32) {
        let Some(h) = self.grid.get_mut(x) else {
            return
        };
        h.remove(y);
    }

    pub fn len(&self) -> usize {
        self.grid.iter().map(|(_, h)| h.len()).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
