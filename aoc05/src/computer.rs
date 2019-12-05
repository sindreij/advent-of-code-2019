use std::collections::VecDeque;

// Opcodes
// 01 ADD op1 op2 addr
// 02 MULTIPLY op1 op2 addr
// 03 INPUT addr
// 04 OUTPUT addr
// 99 HALT

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

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

    fn get(&self, pos: usize) -> Result<i32> {
        Ok(*self.memory.get(pos).ok_or("Tried to read past memory")?)
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

            use Instruction::*;
            match instr {
                Add {
                    a,
                    b,
                    result_location,
                } => {
                    self.set(result_location, self.get_param(a)? + self.get_param(b)?)?;
                }
                Output { param } => self.output.push(self.get_param(param)?),
                Halt => return Ok(()),
                value => unimplemented!("{:?}", value),
            }
        }
    }

    fn next_i32(&mut self) -> Result<i32> {
        let val = self.get(self.pc)?;
        self.pc += 1;
        Ok(val)
    }

    fn next_instr(&mut self) -> Result<Instruction> {
        let op = self.next_i32()?;
        Ok(match op {
            1 => Instruction::Add {
                a: Param::Pos(self.next_i32()? as usize),
                b: Param::Pos(self.next_i32()? as usize),
                result_location: self.next_i32()? as usize,
            },
            4 => Instruction::Output {
                param: Param::Pos(self.next_i32()? as usize),
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
    Halt,
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
    #[ignore]
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
}
