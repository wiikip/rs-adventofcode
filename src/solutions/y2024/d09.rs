use std::{borrow::BorrowMut, collections::VecDeque, error::Error, fmt::Display, str::FromStr, vec};

#[derive(Clone, Debug)]
enum Block {
    Partition(usize),
    Empty
}

#[derive(Debug)]
struct DiskBlock(Vec<Block>);

#[derive(Debug)]

struct DiskFile(Vec<SpaceBlock>);

impl Display for DiskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_list: Vec<String> = self.0.iter().map(|f| match f {
            Block::Empty => String::from("."),
            Block::Partition(n) => format!("{n}")
        }).collect();
        write!(f, "{}", string_list.join(" "))
    }
} 

impl Display for DiskFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_list: Vec<String> = self.0.iter().map(|f| match f {
            (Block::Empty, _, l) => String::from(vec!["."; l.clone()].join(" ")),
            (Block::Partition(n), _, l) => String::from(vec![format!("{n}"); l.clone()].join(" ")),
        }).collect();
        write!(f, "{}", string_list.join(" "))
    }
}
#[derive(Debug)]
struct ParseDiskError;
impl Error for ParseDiskError{}

impl Display for ParseDiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse disk string")
    }
}

impl FromStr for DiskFile {
    type Err = ParseDiskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut next_file_idx = 0;
        let vec: Vec<SpaceBlock> = s.chars().enumerate().map(|(idx, char)| {
            let num: usize = char.to_digit(10).unwrap().try_into().unwrap();
            next_file_idx += num;
            if idx % 2 == 0 {
                (Block::Partition(idx/2), next_file_idx-num, num)
            } else {
                (Block::Empty, next_file_idx-num, num)
            }
        }).filter(|(_,_,l)| *l>0).collect();
        Ok(Self(vec))
    }

}

impl FromStr for DiskBlock {
    type Err = ParseDiskError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<Block> = s.chars().enumerate().flat_map(|(idx,char)| {
            let num: usize = char.to_digit(10).unwrap().try_into().unwrap();
            if idx % 2 == 0 {
                vec![Block::Partition(idx/2); num]
            } else {
                vec![Block::Empty; num]
            }
        }).collect();
        Ok(Self(vec))
    }
}
type SpaceBlock = (Block, usize, usize); // Block , first index, length
impl DiskBlock {
    fn reorganize(&mut self){
        let mut next_empty_block_idx = 0;
        let mut last_partition_block_idx = self.0.len() - 1;
        while next_empty_block_idx < last_partition_block_idx {
            match self.0[next_empty_block_idx] {
                Block::Empty => (),
                Block::Partition(_) => {next_empty_block_idx += 1; continue},
            }

            match  self.0[last_partition_block_idx] {
                Block::Empty => { last_partition_block_idx -= 1; continue},
                Block::Partition(_) => ()
            }
            self.0.swap(next_empty_block_idx, last_partition_block_idx);
            next_empty_block_idx += 1;
            last_partition_block_idx -= 1;
        }
    }

    fn checksum(&self) -> usize {
        self.0.iter().enumerate().map(|(idx, block)| match block {
            Block::Empty => 0,
            Block::Partition(n) => *n * idx
        }).reduce(|acc, e| acc + e).unwrap()
    }

}

fn checksum(vec: &Vec<SpaceBlock>) -> usize {
    vec.iter().map(|b| {
        match b {
            (Block::Empty, _, _) => 0,
            (Block::Partition(n), idx, l) => {
                let mut tot = 0;
                let b2 = idx + l;
                for i in *idx..b2 {
                    tot += i*n
                };
                tot
            }
        }
    }).reduce(|acc, e| acc+e).unwrap()
}
impl DiskFile {
    fn reorganize(&self) -> Vec<SpaceBlock>{
       let mut res: Vec<SpaceBlock> = Vec::new();
       let mut partition: Vec<SpaceBlock> = self.0.iter().filter_map(|block| match block {
            (Block::Partition(_),_,_) => Some(block.clone()),
            _ => None
       }).collect();
       let mut empty: Vec<SpaceBlock> = self.0.iter().filter_map(|block| match block {
        (Block::Empty,_,_) => Some(block.clone()),
        _ => None
   }).collect();
   while let Some(file) = partition.pop() {
        let Some((idx,empty_block)) = empty.iter().enumerate().find(|&(_,b)| b.2 >= file.2 && b.1 < file.1) else {
            res.push(file);
            continue;
        };
        let empty_block_cloned = empty_block.clone();
        empty.remove(idx);
        res.push((file.0, empty_block_cloned.1, file.2));
        if empty_block_cloned.2 > file.2 {
            empty.insert(idx, (empty_block_cloned.0, empty_block_cloned.1 + file.2, empty_block_cloned.2 - file.2));
            continue;
        }
   };
   res
}
} 


fn part_one(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let mut disk: DiskBlock  = input.parse()?;
    disk.reorganize();
    Ok(disk.checksum())
}

fn part_two(input: String) -> Result<usize, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    let mut disk: DiskFile  = input.parse()?;
    let res = checksum(&disk.reorganize());
    Ok(res)
}


#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 9).unwrap();
        assert_eq!(part_one(input).unwrap(), 6421128769094)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 9).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}