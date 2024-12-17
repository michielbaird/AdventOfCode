use std::{error::Error, i64, io::{stdin, Read}};

use regex::Regex;


struct Program {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    output: Vec<i32>,

    memory: Vec<i32>,
    pc: usize,
}

impl Program {
    // fn from_memory(program: Vec<i32>) -> Self {
    //     Self {
    //         register_a: 0, 
    //         register_b: 0,
    //         register_c: 0,
    //         output: vec![],
    //         memory: program,
    //         pc: 0
    //     }
    // }

    fn from_values(
        register_a: i64,
        register_b: i64,
        register_c: i64,
        memory: Vec<i32>,
    ) -> Self {
        Self {
            register_a: register_a,
            register_b: register_b,
            register_c: register_c,
            memory: memory,
            output: vec![],
            pc: 0,
        }
    }
    fn combo_val(&self, combo: i32) -> i64 {
        match combo {
            c if (0..4).contains(&c) => c as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("you said it"), 
        }
    }

    fn step(&mut self) -> bool { // halts
        //println!("{:?}", self.memory);
        let ins = self.memory[self.pc];
        let combo = self.memory[self.pc+1];
        //println!("{} {} {} {}", self.pc, ins, combo, self.combo_val(combo));
        match ins {
            0 => {
                let n = self.register_a;
                let d: i64 = 1i64.checked_shl(self.combo_val(combo) as u32).unwrap_or(i64::MAX);
                self.register_a = n/d;
                self.pc += 2;
            },
            1 => {
                self.register_b = self.register_b ^ (combo as i64);
                self.pc += 2;
            },
            2 => {
                self.register_b = self.combo_val(combo) % 8;
                self.pc += 2;
            },
            3 => {
                if self.register_a == 0 {
                    self.pc += 2;
                } else {
                    self.pc = combo as usize;
                }
            },
            4 => {
                self.register_b = self.register_b ^ self.register_c;
                self.pc += 2;
            },
            5 => {
                let to_add = (self.combo_val(combo) as i32).rem_euclid(8);
                //println!("to_add: {}, {:0b} {:0b} {:0b}", to_add, self.register_a, self.register_b, self.register_c);
                self.output.push(to_add);
                self.pc += 2;

            },
            6 => {
                let n = self.register_a;
                let d: i64 = 1i64.checked_shl(self.combo_val(combo) as u32).unwrap_or(i64::MAX);
                self.register_b = n/d;
                self.pc += 2;
            }
            _ => {
                let n = self.register_a;
                let d: i64 = 1i64.checked_shl(self.combo_val(combo) as u32).unwrap_or(i64::MAX);
                self.register_c = n/d;
                self.pc += 2;
            }
            
        }
        self.pc < self.memory.len()

    }

}




fn part_one(
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<i32>,
) -> String {
    let mut prog = Program::from_values(reg_a, reg_b, reg_c, program);
    // for i in 0.. 100 {
    //     prog.step();
    // }
    while prog.step() {}
    let output = prog.output.clone();
    output.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
}

fn part_two(
    program: Vec<i32>
) -> i64 {

    let target = program.clone();
    fn backtrack(count: usize, target: &Vec<i32>, current: i64) -> Option<i64> {
        if count == target.len() {
            return Some(current);
        }
        for i in 0..8 {
            let mut prog: Program = Program::from_values(
                current << 3 | i, 
                0, 
                0, 
                target.clone()
            );
            while prog.step() {}
            if prog.output[0] == target[target.len()-count-1] {
                let recurse = backtrack(count + 1, target, current << 3 | i);
                if recurse.is_some() {
                    return recurse;
                }
            }
        }
        None
    }
    let result= backtrack(0, &target, 0);
    println!("{:?}", result);
    result.unwrap()


    
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = vec![];
    stdin().read_to_end(&mut buf)?;
    let _raw_input = String::from_utf8(buf)?;
    let _line_re = Regex::new(r"\r?\n")?;
    let _re_register = Regex::new(r"Register\s.\:\s(\d+)")?;
    let _re_program = Regex::new(r"Program\:\s(.*)")?;
    let mut iter = _line_re.split(&_raw_input);
    let reg_a = _re_register.captures(iter.next().unwrap()).unwrap()[1].parse::<i64>()?;
    let reg_b = _re_register.captures(iter.next().unwrap()).unwrap()[1].parse::<i64>()?;
    let reg_c = _re_register.captures(iter.next().unwrap()).unwrap()[1].parse::<i64>()?;
    iter.next();
    let raw_program = &(_re_program.captures(iter.next().unwrap()).unwrap()[1]);
    println!("{}", raw_program);
    let program = raw_program.split(',').map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("{:?}", program);





    println!("Part 1: {}", part_one(reg_a, reg_b, reg_c, program.clone()));
    println!("Part 2: {}", part_two(program));


    Ok(())
}
