use std::{fmt::{Debug, Display}, str::FromStr};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
