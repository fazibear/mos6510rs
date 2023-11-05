pub mod instruction;
pub mod mode;
pub mod opcodes;
pub mod registers;
pub mod status_flags;
pub mod memory;

use instruction::Instruction;
use mode::Mode;
use opcodes::OpCodes;
use registers::Registers;
use status_flags::StatusFlags;
use memory::Memory;

pub struct CPU {
    pub registers: Registers,
    pub status_flags: StatusFlags,
    pub memory: Box<dyn Memory>,
    pub cycles: u64,
    pub opcodes: OpCodes,
}

impl CPU {
    pub fn new(memory: Box<dyn Memory>) -> CPU {
        let memory = memory;
        let cycles = 0;
        let registers = Registers::new();
        let opcodes = OpCodes::new();
        let status_flags = StatusFlags::new();

        CPU {
            registers,
            memory,
            cycles,
            opcodes,
            status_flags,
        }
    }

    pub fn reset(&mut self) {
        let program_counter = self.read_word(0xfffc);
        self.reset_to(program_counter, 0x00);
    }
    
    pub fn reset_to(&mut self, program_counter: u16, accumulator: u8) {
        self.registers = Registers::new();
        self.status_flags = StatusFlags::new();
        
        self.registers.accumulator = accumulator;
        self.registers.program_counter = program_counter;
    }

    pub fn step(&mut self) -> u64 {
        self.cycles = 0;
        let address = self.read_byte_and_increment_pc();

        let (instruction, mode) = self.opcodes.get(address);
                
        // println!("{} {} {} {} {} {} {}", 
        //             self.registers.x, 
        //             self.registers.y, 
        //             self.registers.accumulator, 
        //             self.registers.stack_pointer, 
        //             self.status_flags.to_byte(), 
        //             self.registers.program_counter,
        //             address
        // );
        
        match instruction {
            Instruction::AddWithCarry => {
                let tmp: u16 = self.registers.accumulator as u16
                    + self.get_address(mode) as u16
                    + self.status_flags.carry as u16;
                self.status_flags.carry = tmp & 0x100 != 0;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
                self.status_flags.overflow =
                    (self.status_flags.carry as u16 ^ self.status_flags.negative as u16) != 0;
            }
            Instruction::AndWithAccumulator => {
                let tmp = self.get_address(mode);
                self.registers.accumulator &= tmp;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }
            Instruction::ArithmeticShiftLeft => {
                let mut tmp = self.get_address(mode) as u16;
                tmp <<= 1;
                self.set_address(mode, tmp as u8);
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
                self.status_flags.carry = tmp & 0x100 != 0;
            }
            Instruction::BranchIfCarryClear => {
                self.branch(!self.status_flags.carry);
            }
            Instruction::BranchIfCarrySet => {
                self.branch(self.status_flags.carry);
            }
            Instruction::BranchIfEqual => {
                self.branch(self.status_flags.zero);
            }
            Instruction::BranchIfMinus => {
                self.branch(self.status_flags.negative);
            }
            Instruction::BranchIfNotEqual => {
                self.branch(!self.status_flags.zero);
            }
            Instruction::BranchIfPlus => {
                self.branch(!self.status_flags.negative);
            }
            Instruction::BranchIfOverflowClear => {
                self.branch(!self.status_flags.overflow);
            }
            Instruction::BranchIfOverflowSet => {
                self.branch(self.status_flags.overflow);
            }
            Instruction::BitSet => {
                let tmp = self.get_address(mode);
                self.status_flags.zero = (self.registers.accumulator & tmp) == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
                self.status_flags.overflow = tmp & 0x40 != 0;
            }
            Instruction::Break => {
                self.registers.program_counter = 0;
            }
            Instruction::ClearCarry => {
                self.cycles += 2;
                self.status_flags.carry = false;
            }
            Instruction::ClearDecimal => {
                self.cycles += 2;
                self.status_flags.decimal = false;
            }
            Instruction::ClearInterrupt => {
                self.cycles += 2;
                self.status_flags.interrupt = false;
            }
            Instruction::ClearOverflow => {
                self.cycles += 2;
                self.status_flags.overflow = false;
            }
            Instruction::CompareWithAccumulator => {
                let tmp = self.get_address(mode);
                let mut tmp2 = self.registers.accumulator as i16 - tmp as i16;
                if tmp2 < 0 {
                    tmp2 += 256;
                }

                self.status_flags.zero = tmp2 == 0;
                self.status_flags.negative = tmp2 & 0x80 != 0;
                self.status_flags.carry = self.registers.accumulator >= tmp;
            }
            Instruction::CompareWithX => {
                let tmp = self.get_address(mode);
                let mut tmp2 = self.registers.x as i16 - tmp as i16;
                if tmp2 < 0 {
                    tmp2 += 256;
                }

                self.status_flags.zero = tmp2 == 0;
                self.status_flags.negative = tmp2 & 0x80 != 0;
                self.status_flags.carry = self.registers.x >= tmp;
            }
            Instruction::CompareWithY => {
                let tmp = self.get_address(mode);
                let mut tmp2 = self.registers.y as i16 - tmp as i16;
                if tmp2 < 0 {
                    tmp2 += 256;
                }

                self.status_flags.zero = tmp2 == 0;
                self.status_flags.negative = tmp2 & 0x80 != 0;
                self.status_flags.carry = self.registers.y >= tmp;
            }
            Instruction::Decrement => {
                let mut tmp = self.get_address(mode) as i16 - 1;
                if tmp < 0 {
                    tmp += 256;
                }
                self.set_address(mode, tmp as u8);
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::DecrementX => {
                self.cycles += 2;
                let mut tmp = self.registers.x as i16 - 1;
                if tmp < 0 {
                    tmp += 256
                }
                self.registers.x = tmp as u8;
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::DecrementY => {
                self.cycles += 2;
                let mut tmp = self.registers.y as i16 - 1;
                if tmp < 0 {
                    tmp += 256
                }
                self.registers.y = tmp as u8;
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::ExclusiveOrWithAccumulator => {
                let tmp = self.get_address(mode);
                self.registers.accumulator ^= tmp;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }
            Instruction::Increment => {
                let mut tmp = self.get_address(mode) as i16 + 1;
                if tmp > 255 {
                    tmp -= 256;
                }
                self.set_address(mode, tmp as u8);
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::IncrementX => {
                self.cycles += 2;
                let mut tmp = self.registers.x as i16 + 1;
                if tmp > 255 {
                    tmp -= 256
                }
                tmp &= 0xff;
                self.registers.x = tmp as u8;
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::IncrementY => {
                self.cycles += 2;
                let mut tmp = self.registers.y as i16 + 1;
                if tmp > 255 {
                    tmp -= 256
                }
                tmp &= 0xff;
                self.registers.y = tmp as u8;
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::Jump => {
                self.cycles += 3;
                let mut tmp = self.read_byte_and_increment_pc() as u16;
                tmp |= (self.read_byte_and_increment_pc() as u16) << 8;
                match mode {
                    Mode::Absolute => {
                        self.registers.program_counter = tmp;
                    }
                    Mode::Indirect => {
                        self.registers.program_counter = self.read_byte(tmp) as u16;
                        self.registers.program_counter |=
                            (self.read_byte((tmp + 1) & 0xff) as u16) << 8;
                        self.cycles += 2;
                    }
                    _ => panic!("Unimplemented jump addressing mode!"),
                }
            }
            Instruction::JumpSubroutine => {
                self.cycles += 6;
                self.push((((self.registers.program_counter + 1) & 0xffff) >> 8) as u8);
                self.push(((self.registers.program_counter + 1) & 0xff) as u8);
                let mut tmp = self.read_byte_and_increment_pc() as u16;
                tmp |= (self.read_byte_and_increment_pc() as u16) << 8;
                self.registers.program_counter = tmp;
            }
            Instruction::LoadAccumulator => {
                self.registers.accumulator = self.get_address(mode);
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }
            Instruction::LoadX => {
                self.registers.x = self.get_address(mode);
                self.status_flags.zero = self.registers.x == 0;
                self.status_flags.negative = self.registers.x & 0x80 != 0;
            }
            Instruction::LoadY => {
                self.registers.y = self.get_address(mode);
                self.status_flags.zero = self.registers.y == 0;
                self.status_flags.negative = self.registers.y & 0x80 != 0;
            }
            Instruction::LogicalShiftRight => {
                let tmp = self.get_address(mode) as u16;
                let tmp2 = (tmp >> 1) & 0xff;
                self.set_address(mode, tmp2 as u8);
                self.status_flags.zero = tmp2 == 0;
                self.status_flags.negative = tmp2 & 0x80 != 0;
                self.status_flags.carry = tmp & 0x01 != 0;
            }
            Instruction::NoOperation => {
                self.cycles += 2;
            }
            Instruction::OrWithAccumulator => {
                let tmp = self.get_address(mode);
                self.registers.accumulator |= tmp;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }
            Instruction::PushAccumulator => {
                self.cycles += 3;
                self.push(self.registers.accumulator);
            }
            Instruction::PushProcessorStatus => {
                self.cycles += 3;
                self.push(self.status_flags.to_byte());
            }
            Instruction::PullAccumulator => {
                self.cycles += 4;
                self.registers.accumulator = self.pop();
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }
            Instruction::PullProcessorStatus => {
                self.cycles += 4;
                let tmp = self.pop();
                self.status_flags = self.status_flags.from_byte(tmp);
            }
            Instruction::RotateLeft => {
                let mut tmp = self.get_address(mode) as u16;
                let c = if (self.status_flags.carry as u16) != 0 {
                    128
                } else {
                    0
                };
                self.status_flags.carry = tmp & 1 == 1;
                tmp >>= 1;
                tmp |= c;
                self.set_address(mode, tmp as u8);
                self.status_flags.zero = tmp == 0;
                self.status_flags.negative = tmp & 0x80 != 0;
            }
            Instruction::ReturnFromInterrupt | Instruction::ReturnFromSubroutine => {
                self.cycles += 6;
                let mut tmp = self.pop() as u16;
                tmp |= (self.pop() as u16) << 8;
                self.registers.program_counter = tmp + 1;
            }
            Instruction::SubtractWithCarry => {
                let tmp = self.get_address(mode) as u16 ^ 0xff;
                let tmp2 = self.registers.accumulator as u16 + tmp + self.status_flags.carry as u16;
                self.status_flags.carry = tmp2 & 0x100 != 0;
                self.registers.accumulator = (tmp2 & 0xff) as u8;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator > 127;
                self.status_flags.overflow =
                    ((self.status_flags.carry as u16) ^ (self.registers.accumulator as u16)) & 0x80
                        != 0;
            }
            Instruction::SetCarry => {
                self.cycles += 2;
                self.status_flags.carry = true;
            }
            Instruction::SetDecimal => {
                self.cycles += 2;
                self.status_flags.decimal = true;
            }
            Instruction::SetInterruptDisable => {
                self.cycles += 2;
                self.status_flags.interrupt = true;
            }
            Instruction::StoreAccumulator => {
                self.put_address(mode, self.registers.accumulator);
            }
            Instruction::StoreX => {
                self.put_address(mode, self.registers.x);
            }
            Instruction::StoreY => {
                self.put_address(mode, self.registers.y);
            }
            Instruction::TransferAccumulatorToX => {
                self.cycles += 2;
                self.registers.x = self.registers.accumulator;
                self.status_flags.zero = self.registers.x == 0;
                self.status_flags.negative = self.registers.x & 0x80 != 0;
            }
            Instruction::TransferAccumulatorToY => {
                self.cycles += 2;
                self.registers.y = self.registers.accumulator;
                self.status_flags.zero = self.registers.y == 0;
                self.status_flags.negative = self.registers.y & 0x80 != 0;
            }
            Instruction::TransferStackPointerToX => {
                self.cycles += 2;
                self.registers.x = self.registers.stack_pointer;
                self.status_flags.zero = self.registers.x == 0;
                self.status_flags.negative = self.registers.x & 0x80 != 0;
            }
            Instruction::TransferXToAccumulator => {
                self.cycles += 2;
                self.registers.accumulator = self.registers.x;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }
            Instruction::TransferXToStackPointer => {
                self.cycles += 2;
                self.registers.stack_pointer = self.registers.x;
            }
            Instruction::TransferYToAccumulator => {
                self.cycles += 2;
                self.registers.accumulator = self.registers.y;
                self.status_flags.zero = self.registers.accumulator == 0;
                self.status_flags.negative = self.registers.accumulator & 0x80 != 0;
            }

            _ => panic!("Unknown instruction!"),
        };
        self.cycles
    }

    pub fn push(&mut self, value: u8) {
        self.write_memory(0x100 + self.registers.stack_pointer as u16, value);
        if self.registers.stack_pointer > 0 {
            self.registers.stack_pointer -= 1;
        }
    }

    pub fn pop(&mut self) -> u8 {
        if self.registers.stack_pointer < 0xff {
            self.registers.stack_pointer += 1;
        }
        self.read_byte(0x100 + self.registers.stack_pointer as u16)
    }

    pub fn branch(&mut self, condition: bool) {
        let mut dist = self.get_address(Mode::Immediate) as i32;
        if dist & 0x80 != 0 {
            dist = 0 - ((!dist & 0xff) + 1);
        }
        let mut tmp = self.registers.program_counter as i32 + dist;
        if tmp < 0 {
            tmp += 65536;
        }
        tmp &= 0xffff;

        if condition {
            self.cycles +=
                ((self.registers.program_counter & 0x100) != (tmp as u16 & 0x100)) as u64;
            self.registers.program_counter = tmp as u16;
        }
    }
    
    fn read_word(&self, address: u16) -> u16 {
        let mut val = self.read_byte(address) as u16;
        val |= (self.read_byte(address+1) as u16) << 8;
        val
    }
    
    fn read_word_and_increment_pc(&mut self) -> u16 {
        let val = self.read_word(self.registers.program_counter);
        self.registers.program_counter += 2;
        val
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory.as_ref().read(address)
    }

    fn read_byte_and_increment_pc(&mut self) -> u8 {
        let mem = self.read_byte(self.registers.program_counter);
        self.increment_pc();
        mem
    }

    fn write_memory(&mut self, address: u16, value: u8) {
        self.memory.as_mut().write(address, value);
    }

    fn increment_pc(&mut self) {
        self.registers.program_counter = (self.registers.program_counter + 1) & 0xffff;
    }

    fn get_address(&mut self, mode: Mode) -> u8 {
        match mode {
            Mode::Implied => {
                self.cycles += 2;
                0
            }
            Mode::Immediate => {
                self.cycles += 2;
                self.read_byte_and_increment_pc()
            }
            Mode::Absolute => {
                self.cycles += 4;
                let address = self.read_byte_and_increment_pc() as u16
                    | ((self.read_byte_and_increment_pc() as u16) << 8);
                self.read_byte(address)
            }
            Mode::AbsoluteX => {
                self.cycles += 4;
                let address = self.read_byte_and_increment_pc() as u16
                    | (self.read_byte_and_increment_pc() as u16) << 8;
                let address2 = (address + self.registers.x as u16) & 0xffff;
                if (address2 & 0xff00) != (address & 0xff00) {
                    self.cycles += 1
                };
                self.read_byte(address2)
            }
            Mode::AbsoluteY => {
                self.cycles += 4;
                let address = self.read_byte_and_increment_pc() as u16
                    | (self.read_byte_and_increment_pc() as u16) << 8;
                let address2 = (address + self.registers.y as u16) & 0xffff;
                if (address2 & 0xff00) != (address & 0xff00) {
                    self.cycles += 1
                };
                self.read_byte(address2)
            }
            Mode::ZeroPage => {
                self.cycles += 3;
                let address = self.read_byte_and_increment_pc() as u16;
                self.read_byte(address)
            }
            Mode::ZeroPageX => {
                self.cycles += 4;
                let address =
                    self.read_byte_and_increment_pc() as u16 + self.registers.x as u16;
                self.read_byte(address & 0xff)
            }
            Mode::ZeroPageY => {
                self.cycles += 4;
                let address =
                    self.read_byte_and_increment_pc() as u16 + self.registers.y as u16;
                self.read_byte(address & 0xff)
            }
            Mode::IndirectY => {
                self.cycles += 5;
                let mut address = self.read_byte_and_increment_pc() as u16;
                let address2 = (self.read_byte(address) as u16)
                    | ((self.read_byte((address + 1) & 0xff) as u16) << 8u8);
                address = (address2 + self.registers.y as u16) & 0xffff;
                if (address2 & 0xff00) != (address & 0xff00) {
                    self.cycles += 1
                }
                self.read_byte(address)
            }
            Mode::XIndirect => {
                self.cycles += 6;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address += self.registers.x as u16;
                let mut address2 = self.read_byte(address & 0xff) as u16;
                address += 1;
                address2 |= (self.read_byte(address & 0xff) as u16) << 8;
                self.read_byte(address2)
            }
            Mode::Accumulator => {
                self.cycles += 2;
                self.registers.accumulator
            }
            _ => panic!("Unimplemented get_address addressing mode!"),
        }
    }

    fn set_address(&mut self, mode: Mode, value: u8) {
        match mode {
            Mode::Absolute => {
                self.cycles += 2;
                let mut address = self.read_byte(self.registers.program_counter - 2) as u16;
                address |= (self.read_byte(self.registers.program_counter - 1) as u16) << 8;
                self.write_memory(address, value);
            }
            Mode::AbsoluteX => {
                self.cycles += 3;
                let mut address = self.read_byte(self.registers.program_counter - 2) as u16;
                address |= (self.read_byte(self.registers.program_counter - 1) as u16) << 8;
                let mut address2 = address + self.registers.x as u16;
                address2 &= 0xffff;
                if (address2 & 0xff00) != (address & 0xff00) {
                    self.cycles -= 1;
                }
                self.write_memory(address2, value);
            }
            Mode::ZeroPage => {
                self.cycles += 2;
                let address = self.read_byte(self.registers.program_counter - 1) as u16;
                self.write_memory(address, value);
            }
            Mode::ZeroPageX => {
                self.cycles += 2;
                let mut address = self.read_byte(self.registers.program_counter - 1) as u16;
                address += self.registers.x as u16;
                self.write_memory(address & 0xff, value);
            }
            Mode::Accumulator => {
                self.registers.accumulator = value;
            }
            _ => panic!("Unimplemented set_address addressing mode!"),
        }
    }

    pub fn get_memory_at(&self, address: u16) -> u8 {
        self.memory.as_ref().read(address)
    }

    fn put_address(&mut self, mode: Mode, value: u8) {
        match mode {
            Mode::Absolute => {
                self.cycles += 4;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address |= (self.read_byte_and_increment_pc() as u16) << 8;
                self.write_memory(address, value);
            }
            Mode::AbsoluteX => {
                self.cycles += 4;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address |= (self.read_byte_and_increment_pc() as u16) << 8;
                let mut address2 = address + self.registers.x as u16;
                address2 &= 0xffff;
                self.write_memory(address2, value);
            }
            Mode::AbsoluteY => {
                self.cycles += 4;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address |= (self.read_byte_and_increment_pc() as u16) << 8;
                let mut address2 = address + self.registers.y as u16;
                address2 &= 0xffff;
                if (address2 & 0xff00) != (address & 0xff00) {
                    self.cycles += 1
                };
                self.write_memory(address2, value);
            }
            Mode::ZeroPage => {
                self.cycles += 3;
                let address = self.read_byte_and_increment_pc() as u16;
                self.write_memory(address, value);
            }
            Mode::ZeroPageX => {
                self.cycles += 4;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address += self.registers.x as u16;
                self.write_memory(address & 0xff, value);
            }
            Mode::ZeroPageY => {
                self.cycles += 4;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address += self.registers.y as u16;
                self.write_memory(address & 0xff, value);
            }
            Mode::XIndirect => {
                self.cycles += 6;
                let mut address = self.read_byte_and_increment_pc() as u16;
                address += self.registers.x as u16;
                let mut address2 = self.read_byte(address & 0xff) as u16;
                address += 1;
                address2 |= (self.read_byte(address & 0xff) as u16) << 8;
                self.write_memory(address2, value);
            }
            Mode::IndirectY => {
                self.cycles += 5;
                let mut address = self.read_byte_and_increment_pc() as u16;
                let mut address2 = self.read_byte(address) as u16;
                address2 |= (self.read_byte((address + 1) & 0xff) as u16) << 8;
                address = (address2 + self.registers.y as u16) & 0xffff;
                self.write_memory(address, value);
            }
            Mode::Accumulator => {
                self.registers.accumulator = value;
            }
            _ => panic!("Unimplemented put_address addressing mode!"),
        }
    }

}
