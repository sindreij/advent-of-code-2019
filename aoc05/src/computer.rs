use std::collections::VecDeque;
use std::fmt::{self, Display};

// Opcodes
// 01 ADD op1 op2 addr
// 02 MULTIPLY op1 op2 addr
// 03 INPUT addr
// 04 OUTPUT addr
// 99 HALT

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

#[derive(Debug, Clone)]
pub struct Computer {
    memory: Vec<i32>,
    // instruction pointer, or program counter
    pc: usize,
    output: Vec<i32>,
    input: VecDeque<i32>,
}

impl Computer {
    pub fn from_mem(memory: Vec<i32>) -> Computer {
        Computer {
            memory,
            pc: 0,
            output: vec![],
            input: VecDeque::new(),
        }
    }

    pub fn input(&mut self, data: i32) {
        self.input.push_back(data);
    }

    pub fn output(&self) -> &[i32] {
        &self.output
    }

    pub fn debug(&self) {
        let mut debugger = self.clone();
        while let Ok(instr) = debugger.next_instr() {
            println!("{}", instr);
        }
    }

    fn get(&self, pos: usize) -> Result<i32> {
        Ok(*self
            .memory
            .get(pos)
            .ok_or_else(|| format!("Tried to read past memory, at {}", pos))?)
    }

    fn set(&mut self, pos: usize, value: i32) -> Result<()> {
        *self
            .memory
            .get_mut(pos)
            .ok_or("Tried to write past memory")? = value;
        Ok(())
    }

    fn get_param(&self, param: Param) -> Result<i32> {
        Ok(match param {
            Param::Pos(pos) => self.get(pos)?,
            Param::Immediate(val) => val,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let instr = self.next_instr()?;
            // println!("{}", instr);
            use Instruction::*;
            match instr {
                Add {
                    a,
                    b,
                    result_location,
                } => {
                    self.set(result_location, self.get_param(a)? + self.get_param(b)?)?;
                }
                Multiply {
                    a,
                    b,
                    result_location,
                } => {
                    self.set(result_location, self.get_param(a)? * self.get_param(b)?)?;
                }
                Input { result_location } => {
                    let value = self.input.pop_front().ok_or("Nothing in input queue")?;
                    self.set(result_location, value)?;
                }
                Output { param } => self.output.push(self.get_param(param)?),
                JumpIfTrue { check, jump_to } => {
                    if self.get_param(check)? > 0 {
                        self.pc = self.get_param(jump_to)? as usize;
                    }
                }
                JumpIfFalse { check, jump_to } => {
                    if self.get_param(check)? == 0 {
                        self.pc = self.get_param(jump_to)? as usize;
                    }
                }
                LessThan {
                    a,
                    b,
                    result_location,
                } => {
                    let value = if self.get_param(a)? < self.get_param(b)? {
                        1
                    } else {
                        0
                    };
                    self.set(result_location, value)?;
                }
                Equals {
                    a,
                    b,
                    result_location,
                } => {
                    let value = if self.get_param(a)? == self.get_param(b)? {
                        1
                    } else {
                        0
                    };
                    self.set(result_location, value)?;
                }
                Halt => return Ok(()),
                // unknown => unimplemented!("{:?}", unknown),
            }
        }
    }

    fn next_i32(&mut self) -> Result<i32> {
        let val = self.get(self.pc)?;
        self.pc += 1;
        Ok(val)
    }

    fn next_instr(&mut self) -> Result<Instruction> {
        let mut full_op = self.next_i32()?;
        let op = full_op % 100;
        full_op /= 100;
        let mut modes = Vec::new();
        while full_op > 0 {
            modes.push(full_op % 10);
            full_op /= 10;
        }
        Ok(match op {
            1 => Instruction::Add {
                a: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
                b: Param::from_mode(modes.get(1).copied().unwrap_or(0), self.next_i32()?)?,
                result_location: self.next_i32()? as usize,
            },
            2 => Instruction::Multiply {
                a: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
                b: Param::from_mode(modes.get(1).copied().unwrap_or(0), self.next_i32()?)?,
                result_location: self.next_i32()? as usize,
            },
            3 => Instruction::Input {
                result_location: self.next_i32()? as usize,
            },
            4 => Instruction::Output {
                param: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
            },
            5 => Instruction::JumpIfTrue {
                check: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
                jump_to: Param::from_mode(modes.get(1).copied().unwrap_or(0), self.next_i32()?)?,
            },
            6 => Instruction::JumpIfFalse {
                check: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
                jump_to: Param::from_mode(modes.get(1).copied().unwrap_or(0), self.next_i32()?)?,
            },
            7 => Instruction::LessThan {
                a: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
                b: Param::from_mode(modes.get(1).copied().unwrap_or(0), self.next_i32()?)?,
                result_location: self.next_i32()? as usize,
            },
            8 => Instruction::Equals {
                a: Param::from_mode(modes.get(0).copied().unwrap_or(0), self.next_i32()?)?,
                b: Param::from_mode(modes.get(1).copied().unwrap_or(0), self.next_i32()?)?,
                result_location: self.next_i32()? as usize,
            },
            99 => Instruction::Halt,
            unknown => Err(format!("Unknown instruction {}", unknown))?,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Param {
    Pos(usize),
    Immediate(i32),
}

impl Param {
    fn from_mode(mode: i32, value: i32) -> Result<Param> {
        Ok(match mode {
            0 => Param::Pos(value as usize),
            1 => Param::Immediate(value),
            mode => Err(format!("Unknown mode {}", mode))?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Add {
        a: Param,
        b: Param,
        result_location: usize,
    },
    Multiply {
        a: Param,
        b: Param,
        result_location: usize,
    },
    Input {
        result_location: usize,
    },
    Output {
        param: Param,
    },
    JumpIfTrue {
        check: Param,
        jump_to: Param,
    },
    JumpIfFalse {
        check: Param,
        jump_to: Param,
    },
    LessThan {
        a: Param,
        b: Param,
        result_location: usize,
    },
    Equals {
        a: Param,
        b: Param,
        result_location: usize,
    },
    Halt,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;
        match self {
            Add {
                a,
                b,
                result_location,
            } => write!(f, "ADD {} {} => &{}", a, b, result_location),
            Multiply {
                a,
                b,
                result_location,
            } => write!(f, "MUL {} {} => &{}", a, b, result_location),
            Input { result_location } => write!(f, "INPUT &{}", result_location),
            Output { param } => write!(f, "OUTPUT {}", param),
            JumpIfTrue { check, jump_to } => write!(f, "IF {} JUMP TO {}", check, jump_to),
            JumpIfFalse { check, jump_to } => write!(f, "IF NOT {} JUMP TO {}", check, jump_to),
            LessThan {
                a,
                b,
                result_location,
            } => write!(f, "IF {} < {} => {}", a, b, result_location),
            Equals {
                a,
                b,
                result_location,
            } => write!(f, "IF {} == {} => {}", a, b, result_location),
            Halt => write!(f, "HALT"),
        }
    }
}

impl Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Param::*;
        match self {
            Pos(value) => write!(f, "&{}", value),
            Immediate(value) => write!(f, "{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_state_input_output() -> Result<()> {
        let program = vec![3, 0, 4, 0, 99];
        let mut state = Computer::from_mem(program.clone());
        assert_eq!(state.pc, 0);
        assert_eq!(state.output, vec![]);
        assert_eq!(state.memory, program);
        assert_eq!(state.get(1)?, 0);
        assert_eq!(state.get(2)?, 4);
        state.set(2, 5)?;
        assert_eq!(state.get(2)?, 5);

        Ok(())
    }

    #[test]
    fn test_get_param() -> Result<()> {
        let computer = Computer::from_mem(vec![9, 42, 7]);
        assert_eq!(computer.get_param(Param::Pos(1))?, 42);
        assert_eq!(computer.get_param(Param::Immediate(72))?, 72);
        Ok(())
    }

    #[test]
    fn input_output_program() -> Result<()> {
        let program = vec![3, 0, 4, 0, 99];
        let mut computer = Computer::from_mem(program.clone());
        computer.input(12);

        computer.run()?;

        assert_eq!(computer.output, vec![12]);

        Ok(())
    }

    #[test]
    fn test_next_i32() -> Result<()> {
        let mut program = Computer::from_mem(vec![1, 0, 2]);
        assert_eq!(program.next_i32()?, 1);
        assert_eq!(program.next_i32()?, 0);
        assert_eq!(program.next_i32()?, 2);
        Ok(())
    }

    #[test]
    fn test_next_instr() -> Result<()> {
        let mut program = Computer::from_mem(vec![1, 0, 2, 3, 1, 4, 8, 5, 4, 0, 99]);
        assert_eq!(
            program.next_instr()?,
            Instruction::Add {
                a: Param::Pos(0),
                b: Param::Pos(2),
                result_location: 3
            }
        );
        assert_eq!(
            program.next_instr()?,
            Instruction::Add {
                a: Param::Pos(4),
                b: Param::Pos(8),
                result_location: 5
            }
        );
        assert_eq!(
            program.next_instr()?,
            Instruction::Output {
                param: Param::Pos(0),
            }
        );
        assert_eq!(program.next_instr()?, Instruction::Halt,);
        Ok(())
    }

    #[test]
    fn test_more_instructions() -> Result<()> {
        let mut program = Computer::from_mem(vec![5, 1, 2, 6, 3, 4, 7, 5, 6, 3, 8, 4, 3, 9]);
        assert_eq!(
            program.next_instr()?,
            Instruction::JumpIfTrue {
                check: Param::Pos(1),
                jump_to: Param::Pos(2),
            }
        );
        assert_eq!(
            program.next_instr()?,
            Instruction::JumpIfFalse {
                check: Param::Pos(3),
                jump_to: Param::Pos(4),
            }
        );
        assert_eq!(
            program.next_instr()?,
            Instruction::LessThan {
                a: Param::Pos(5),
                b: Param::Pos(6),
                result_location: 3
            }
        );
        assert_eq!(
            program.next_instr()?,
            Instruction::Equals {
                a: Param::Pos(4),
                b: Param::Pos(3),
                result_location: 9
            }
        );
        Ok(())
    }

    #[test]
    fn test_next_instr_imm() -> Result<()> {
        let mut program = Computer::from_mem(vec![1002, 4, 3, 4, 1102, 10, 8, 7, 99]);
        assert_eq!(
            program.next_instr()?,
            Instruction::Multiply {
                a: Param::Pos(4),
                b: Param::Immediate(3),
                result_location: 4
            }
        );
        assert_eq!(
            program.next_instr()?,
            Instruction::Multiply {
                a: Param::Immediate(10),
                b: Param::Immediate(8),
                result_location: 7
            }
        );
        assert_eq!(program.next_instr()?, Instruction::Halt,);
        Ok(())
    }

    #[test]
    fn just_halt() -> Result<()> {
        let mut comp = Computer::from_mem(vec![99]);
        comp.run()?;
        Ok(())
    }

    #[test]
    fn simple_addition() -> Result<()> {
        let program = vec![1, 0, 0, 0, 4, 0, 99];
        let mut state = Computer::from_mem(program.clone());
        state.run()?;

        assert_eq!(state.output, vec![2]);

        Ok(())
    }

    #[test]
    fn test_multiply() -> Result<()> {
        let program = vec![2, 7, 8, 0, 4, 0, 99, 5, 2];
        let mut state = Computer::from_mem(program.clone());
        state.run()?;

        assert_eq!(state.output, vec![10]);

        Ok(())
    }

    #[test]
    fn multiple_inputs_and_outputs() -> Result<()> {
        let mut computer = Computer::from_mem(vec![3, 0, 3, 4, 99, 4, 0, 0, 4, 0, 4, 4, 99]);
        computer.input(23);
        computer.input(2);
        computer.run()?;
        assert_eq!(computer.output, vec![46, 2]);
        Ok(())
    }

    fn finish(program: Vec<i32>) -> Result<Vec<i32>> {
        let mut computer = Computer::from_mem(program);
        computer.run()?;
        Ok(computer.memory)
    }

    #[test]
    fn test_from_day2() -> Result<()> {
        assert_eq!(finish(vec![1, 0, 0, 0, 99])?, vec![2, 0, 0, 0, 99]);
        assert_eq!(finish(vec![2, 3, 0, 3, 99])?, vec![2, 3, 0, 6, 99]);
        assert_eq!(finish(vec![2, 4, 4, 5, 99, 0])?, vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            finish(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])?,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        Ok(())
    }

    #[test]
    fn programs_with_immediate() -> Result<()> {
        assert_eq!(finish(vec![1101, 5, 6, 0, 99])?, vec![11, 5, 6, 0, 99]);
        assert_eq!(finish(vec![1102, 4, 2, 3, 99])?, vec![1102, 4, 2, 8, 99]);
        Ok(())
    }

    fn get_output(program: Vec<i32>) -> Result<i32> {
        let mut computer = Computer::from_mem(program);
        computer.run()?;
        Ok(computer.output().get(0).copied().ok_or("No output")?)
    }

    fn input_output(program: &[i32], input: i32) -> Result<i32> {
        let mut computer = Computer::from_mem(program.to_vec());
        computer.input(input);
        computer.run()?;
        Ok(computer.output().get(0).copied().ok_or("No output")?)
    }

    fn print_program(program: &[i32]) {
        println!("---");
        Computer::from_mem(program.to_vec()).debug();
        println!("---");
    }

    #[test]
    fn day5_part2_instructions() -> Result<()> {
        // Test EQUALS
        assert_eq!(finish(vec![1108, 8, 2, 0, 99])?, vec![0, 8, 2, 0, 99]);
        assert_eq!(
            finish(vec![8, 5, 6, 0, 99, 2, 2])?,
            vec![1, 5, 6, 0, 99, 2, 2]
        );
        assert_eq!(finish(vec![1108, 8, 8, 0, 99])?, vec![1, 8, 8, 0, 99]);
        assert_eq!(
            finish(vec![8, 5, 6, 0, 99, 2, 3])?,
            vec![0, 5, 6, 0, 99, 2, 3]
        );
        // Test LESS THAN
        assert_eq!(finish(vec![1107, 8, 2, 0, 99])?, vec![0, 8, 2, 0, 99]);
        assert_eq!(get_output(vec![7, 7, 8, 0, 4, 0, 99, 4, 2])?, 0);
        assert_eq!(finish(vec![1107, 8, 9, 0, 99])?, vec![1, 8, 9, 0, 99]);
        assert_eq!(get_output(vec![7, 7, 8, 0, 4, 0, 99, 4, 9])?, 1);

        // Test JUMP IF TRUE
        assert_eq!(get_output(vec![1105, 0, 6, 104, 8, 99, 104, 16, 99])?, 8);
        assert_eq!(get_output(vec![1105, 1, 6, 104, 8, 99, 104, 16, 99])?, 16);

        assert_eq!(
            get_output(vec![5, 9, 10, 104, 8, 99, 104, 16, 99, 0, 6])?,
            8
        );
        assert_eq!(
            get_output(vec![5, 9, 10, 104, 8, 99, 104, 16, 99, 15, 6])?,
            16
        );

        // Test JUMP IF FALSE
        assert_eq!(get_output(vec![1106, 1, 6, 104, 8, 99, 104, 16, 99])?, 8);
        assert_eq!(get_output(vec![1106, 0, 6, 104, 8, 99, 104, 16, 99])?, 16);

        assert_eq!(
            get_output(vec![6, 9, 10, 104, 8, 99, 104, 16, 99, 82, 6])?,
            8
        );
        assert_eq!(
            get_output(vec![6, 9, 10, 104, 8, 99, 104, 16, 99, 0, 6])?,
            16
        );
        Ok(())
    }

    #[test]
    fn day5_part2_test_programs() -> Result<()> {
        let is_eight = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(input_output(&is_eight, 8)?, 1);
        assert_eq!(input_output(&is_eight, 9)?, 0);

        let less_than_eight = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(input_output(&less_than_eight, 8)?, 0);
        assert_eq!(input_output(&less_than_eight, 4)?, 1);

        let is_eight_imm = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(input_output(&is_eight_imm, 8)?, 1);
        assert_eq!(input_output(&is_eight_imm, 9)?, 0);

        let less_than_eight_imm = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(input_output(&less_than_eight_imm, 8)?, 0);
        assert_eq!(input_output(&less_than_eight_imm, 4)?, 1);

        let is_non_zero = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(input_output(&is_non_zero, 0)?, 0);
        assert_eq!(input_output(&is_non_zero, 14)?, 1);
        assert_eq!(input_output(&is_non_zero, 1)?, 1);

        let is_non_zero_imm = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(input_output(&is_non_zero_imm, 0)?, 0);
        assert_eq!(input_output(&is_non_zero_imm, 14)?, 1);
        assert_eq!(input_output(&is_non_zero_imm, 1)?, 1);

        let larger_example = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(input_output(&larger_example, 7)?, 999);
        assert_eq!(input_output(&larger_example, 8)?, 1000);
        assert_eq!(input_output(&larger_example, 9)?, 1001);

        Ok(())
    }
}
