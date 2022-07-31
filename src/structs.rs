#![allow(dead_code)]
#[derive(Default)]
pub struct BytecodeWorkspace {
    pub reg_array: Vec<u32>,
    pub mem_array: Vec<u32>,
    pub pgm_array: Vec<u32>,
    pub reg_pc: u32,
    pub reg_sp: u32,
    pub flag_zero: bool,
   
}

impl BytecodeWorkspace {
    fn read_reg(&self, reg_num: u32) -> &u32 {
        return &self.reg_array[reg_num as usize];
    }

    fn read_pc(&self) -> &u32 {
        return &self.reg_pc;
    }

    fn read_sp(&self) -> &u32 {
        return &self.reg_sp;
    }

    pub fn init(&mut self) {
        self.reg_array = vec![0, 12];
        self.mem_array = vec![0, 1_000_000];
        self.reg_pc = 0;
        self.reg_sp = 0;
    }

    pub fn read_ram(&self, offset: u32)-> &u32{
        return &self.mem_array[offset as usize];
    }
    
}