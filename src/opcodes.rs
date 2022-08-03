#![allow(dead_code)]
#![allow(unused_variables)]

use crate::structs::BytecodeWorkspace;
use std::process::exit;


pub fn nop(space: &mut BytecodeWorkspace){
    space.reg_pc += 1;
}

pub fn hlt(space: &mut BytecodeWorkspace){
    println!("");
    exit(0);
}

pub fn ldr(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
    let temp2 = space.pgm_array[space.reg_pc as usize + 2];
    
    let reg_a= temp as u8;

    space.reg_array[reg_a as usize] = temp2;
    space.reg_pc += 3;
}

pub fn mov(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a= temp as u8;
    let reg_b= (temp >> 8) as u8;

    space.reg_array[reg_b as usize] = space.reg_array[reg_a as usize];
    space.reg_pc += 2;
}

pub fn add(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a = temp as u8;
    let reg_b= (temp >> 8) as u8;
    let reg_c= (temp >> 16) as u8;
    //print_state(space);
    space.reg_array[reg_c as usize] = space.reg_array[reg_a as usize] + space.reg_array[reg_b as usize];
    space.reg_pc += 2
}

pub fn sub(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a = temp as u8;
    let reg_b= (temp >> 8) as u8;
    let reg_c= (temp >> 16) as u8;
    space.reg_array[reg_c as usize] = space.reg_array[reg_a as usize] - space.reg_array[reg_b as usize];
    space.reg_pc += 2;
}

pub fn mul(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a = temp as u8;
    let reg_b= (temp >> 8) as u8;
    let reg_c= (temp >> 16) as u8;
    space.reg_array[reg_c as usize] = space.reg_array[reg_a as usize] * space.reg_array[reg_b as usize];
    space.reg_pc += 2;
}

pub fn div(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a = temp as u8;
    let reg_b= (temp >> 8) as u8;
    let reg_c= (temp >> 16) as u8;
    let reg_d= (temp >> 24) as u8;
    space.reg_array[reg_c as usize] = space.reg_array[reg_a as usize] / space.reg_array[reg_b as usize];
    space.reg_array[reg_d as usize] = space.reg_array[reg_a as usize] % space.reg_array[reg_b as usize];
    space.reg_pc += 2;
}

pub fn jmp(space: &mut BytecodeWorkspace){
    /*
    documentation
    space.pgm_array[space.reg_pc as u8 as usize + 1]: gets the 4 byte val in the program 
    *above* as usize lets it be an index in vector
    uses the value in program to find the register to jump to 
    */
    space.reg_pc = space.reg_array[space.pgm_array[space.reg_pc as u8 as usize + 1] as usize];
        
}
pub fn jez(space: &mut BytecodeWorkspace){
    if space.flag_zero {
        space.reg_pc = space.reg_array[space.pgm_array[space.reg_pc as u8 as usize + 1] as usize];
    }
    else {
        space.reg_pc += 2
    }
}
pub fn jnz(space: &mut BytecodeWorkspace){
    if  !space.flag_zero {
        space.reg_pc = space.reg_array[space.pgm_array[space.reg_pc as u8 as usize + 1] as usize];
    }
    else {
        space.reg_pc += 2
    }
}

pub fn blr(space: &mut BytecodeWorkspace){
    space.reg_lk = space.reg_pc + 1;
    space.reg_pc = space.reg_array[space.pgm_array[space.reg_pc as u8 as usize + 1] as usize];       
}

pub fn bez(space: &mut BytecodeWorkspace){
    space.reg_lk = space.reg_pc + 1;
    if space.flag_zero {
        space.reg_pc = space.reg_array[space.pgm_array[space.reg_pc as u8 as usize + 1] as usize];
    }
    else {
        space.reg_pc += 2
    }
}
pub fn bnz(space: &mut BytecodeWorkspace){
    space.reg_lk = space.reg_pc + 1;
    if  !space.flag_zero {
        space.reg_pc = space.reg_array[space.pgm_array[space.reg_pc as u8 as usize + 1] as usize];
    }
    else {
        space.reg_pc += 2
    }
}


// =============================================================================================
pub fn jpc(space: &mut BytecodeWorkspace){
    space.reg_pc = space.pgm_array[space.reg_pc as u8 as usize + 1];
        
}
pub fn jec(space: &mut BytecodeWorkspace){
    if space.flag_zero {
        space.reg_pc = space.pgm_array[space.reg_pc as u8 as usize + 1];
    }
    else {
        space.reg_pc += 2
    }
}
pub fn jnc(space: &mut BytecodeWorkspace){
    if  !space.flag_zero {
        space.reg_pc = space.pgm_array[space.reg_pc as u8 as usize + 1];
    }
    else {
        space.reg_pc += 2
    }
}

pub fn blc(space: &mut BytecodeWorkspace){
    space.reg_lk = space.reg_pc + 1;
    space.reg_pc = space.pgm_array[space.reg_pc as u8 as usize + 1];       
}

pub fn bec(space: &mut BytecodeWorkspace){
    space.reg_lk = space.reg_pc + 1;
    if space.flag_zero {
        space.reg_pc = space.pgm_array[space.reg_pc as u8 as usize + 1]; 
    }
    else {
        space.reg_pc += 2
    }
}
pub fn bnc(space: &mut BytecodeWorkspace){
    space.reg_lk = space.reg_pc + 1;
    if  !space.flag_zero {
        space.reg_pc = space.pgm_array[space.reg_pc as u8 as usize + 1]; 
    }
    else {
        space.reg_pc += 2
    }
}
// =============================================================================================




pub fn ptc(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a= temp as u8;

    print!("{}", space.reg_array[reg_a as usize]as u8  as char);//as u8 as char
    // print_state(space);
    space.reg_pc += 2
}

pub fn pnt(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a= temp as u8;

    print!("{}", space.reg_array[reg_a as usize]);//as u8 as char
    // print_state(space);
    space.reg_pc += 2
}

pub fn pth(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
            
    let reg_a= temp as u8;

    print!("{:#10x}", space.reg_array[reg_a as usize]);
    // print_state(space);
    space.reg_pc += 2
}







pub fn cmp(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];
    
    let reg_a = temp as u8;
    let reg_b = (temp >> 8) as u8;

    space.flag_zero = (space.reg_array[reg_a as usize] - space.reg_array[reg_b as usize]) == 0;
    space.reg_pc += 2;
}

pub fn psh(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];

    for x in 0..12{
        if temp & (0b1 << x) != 0{
            space.mem_array[space.reg_sp as usize] = space.reg_array[x + 1 as usize];
            space.reg_sp += 1;
        }
    }
    space.reg_pc += 2;
}

pub fn pop(space: &mut BytecodeWorkspace){
    let temp = space.pgm_array[space.reg_pc as usize + 1];

    for x in 12..0{
        if temp & (0b1 << x) != 0{
            space.reg_array[x + 1 as usize] = space.mem_array[space.reg_sp as usize];
            if space.reg_sp == 0{
                panic!("SP overflow (but negative)");
            }
            space.reg_sp -= 1;
            
        }
    }
    space.reg_pc += 2;
}

pub fn lrt(space: &mut BytecodeWorkspace){
    space.reg_pc = space.reg_lk;
}





//pub fn (space: &mut BytecodeWorkspace){}