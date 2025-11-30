use std::{error::Error, str::FromStr};

#[derive(Clone,Debug)]
struct ProgramInput(u8);
#[derive(Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<ProgramInput>,
    instruction_pointer: usize,
    output: Vec<usize>
}


impl PartialEq<ProgramInput> for usize {
    fn eq(&self, other: &ProgramInput) -> bool {
        *self == usize::try_from(other.0).unwrap()
    }

}
#[derive(Debug)]
struct ParseComputerError;

impl FromStr for Computer {
    type Err = ParseComputerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split("\n\n").collect();

        let registers: Vec<&str> = splitted[0].lines().collect();
        
        let reg_a: usize = registers[0].split(":").last().map(|f| usize::from_str_radix(f.trim(), 8).unwrap()).unwrap();
        let reg_b: usize = registers[1].split(":").last().map(|f| f.trim().parse::<usize>().unwrap()).unwrap();
        let reg_c: usize = registers[2].split(":").last().map(|f| f.trim().parse::<usize>().unwrap()).unwrap();

        let program: Vec<ProgramInput> = splitted[1].split(":").last().unwrap().trim().split(",").map(|f| ProgramInput(f.parse::<u8>().unwrap())).collect();

        Ok(Self{
            a: reg_a,
            b: reg_b,
            c: reg_c,
            program,
            instruction_pointer: 0,
            output: Vec::new()
        })

    }
}

impl Computer {
    fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.output = Vec::new();
        self.a = 0;
        self.b = 0;
        self.c = 0;
    }

    fn set_reg_a(&mut self, reg_a: usize) {
        self.a = reg_a
    }
    fn execute(&mut self) {
        let mut hang = false;
        while !hang {

            let program = self.program.clone();

            match program[self.instruction_pointer] {
                ProgramInput(0) => self.adv0(&program[self.instruction_pointer + 1]),
                ProgramInput(1) => self.bxl1(&program[self.instruction_pointer + 1]),
                ProgramInput(2) => self.bst2(&program[self.instruction_pointer + 1]),
                ProgramInput(3) => {
                    if !self.jnz3(&program[self.instruction_pointer + 1]){
                        continue
                    }
                },
                ProgramInput(4) => self.bcx4(&program[self.instruction_pointer + 1]),
                ProgramInput(5) => self.out5(&program[self.instruction_pointer + 1]),
                ProgramInput(6) => self.bdv6(&program[self.instruction_pointer + 1]),
                ProgramInput(7) => self.cdv7(&program[self.instruction_pointer + 1]),
                _ => panic!("Impossible")
            }
            println!("Op: {:?} Reg B: {}, Reg A: {}, Reg C: {}", program[self.instruction_pointer],self.b, self.a, self.c);

            self.instruction_pointer += 2;
            if self.instruction_pointer >= program.len() {
                hang = true;
            }
        }
    }

    fn combo(&self, operand: &ProgramInput) -> usize {
        match operand {
            ProgramInput(4) => self.a,
            ProgramInput(5) => self.b,
            ProgramInput(6) => self.c,
            ProgramInput(7) => panic!("Invalid program"),
            ProgramInput(v) => v.clone().into()
        }
    }
    fn adv0(&mut self, combo: &ProgramInput) {
        self.a = self.a / usize::pow(2, self.combo(combo).try_into().unwrap())
    }

    fn bxl1(&mut self, literal: &ProgramInput) {
        self.b = self.b ^ usize::try_from(literal.0).unwrap()
    }

    fn bst2(&mut self, combo: &ProgramInput) {
        self.b = self.combo(combo) % 8
    }

    fn jnz3(&mut self, literal: &ProgramInput) -> bool {
        if self.a == 0 {
            return true
        }
        self.instruction_pointer = usize::try_from(literal.0).unwrap();
        false
    }

    fn bcx4(&mut self, _: &ProgramInput) {
        self.b = self.b ^ self.c
    }

    fn out5(&mut self, combo: &ProgramInput) {
        self.output.push(self.combo(combo) % 8);
    }

    fn bdv6(&mut self, combo: &ProgramInput) {
        self.b = self.a / usize::pow(2, self.combo(combo).try_into().unwrap())
    }

    fn cdv7(&mut self, combo: &ProgramInput) {
        self.c = self.a / usize::pow(2, self.combo(combo).try_into().unwrap())
    }
}
fn part_one(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part1 from input
    let mut computer: Computer = input.parse().unwrap();
    computer.execute();
    println!("Output: {:?}", computer.output);
    Ok(10)
}

fn part_two(input: String) -> Result<i32, Box<dyn Error>> {
    // Write here code to solve part 2 from input
    // let mut computer: Computer = input.parse().unwrap();
    // for trial in  246290604621824..250688651132928{
    //     computer.reset();
    //     computer.set_reg_a(trial);
    //     computer.execute();
    //     if computer.output == computer.program {
    //         println!("REG A: {}", trial);
    //         break
    //     }
    // }

    // println!("Output: {:?}", computer.output);
    Ok(10)
}
// 4,6,3,5,6,3,5,2,1,0

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn test_part_one(){
        let input = crate::load_day_input(2024, 17).unwrap();
        assert_eq!(part_one(input).unwrap(), 11)
    }

    #[test]
    fn test_part_two(){
        let input = crate::load_day_input(2024, 17).unwrap();
        assert_eq!(part_two(input).unwrap(), 11)
    }
}