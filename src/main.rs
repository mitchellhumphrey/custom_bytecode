#![allow(dead_code)]
#![allow(unused_variables)]

mod opcodes;
mod structs;
use crate::structs::BytecodeWorkspace;

use std::time::Instant;
use std::env;



fn main() {
    
    let space =  &mut BytecodeWorkspace { 
        reg_array: Vec::new(), 
        mem_array: Vec::new(), 
        pgm_array: Vec::new(),
        reg_pc: 0,
        reg_sp: 0,
        reg_lk: 0,
        flag_zero: false,
    };
    let instruction: u32;
    let params: [u32; 4];

    let mut start = Instant::now();

    find_fib(48);
    let duration = start.elapsed();
    println!("Native Time elapsed is: {:?}", duration);
    let args: Vec<String> = env::args().collect();
    init(space, args[1].to_string());
    //print_pgm(space);

    start = Instant::now();
    loop {
        one_loop(space, start);
    }
}


fn init(space: &mut BytecodeWorkspace, filepath: String){
    space.init();
    let temp = std::fs::read(filepath).unwrap();
    assert!(temp.len() % 4 == 0);
    let mut counter = 0;
    while ! (counter >= temp.len()) {
        space.pgm_array.push(as_u32_be(&temp[counter..counter+4]));
        counter += 4;
    }
    space.reg_array = vec![0; 12];
}





fn one_loop(space: &mut BytecodeWorkspace, start: Instant){

    if space.reg_pc  >= space.pgm_array.len() as u32 {
        println!("{}", space.reg_pc);
        panic!("attempted to read end of file")
    }

    match space.pgm_array[space.reg_pc as usize] {

        //================================= 0x00 ==================================

        //nop
        0x00_00_00_00 => opcodes::nop(space),

        //hlt
        0x00_00_00_01 => {
            let duration = start.elapsed();
            println!("Time elapsed is: {:?}", duration);
            opcodes::hlt(space);
            
        },
        //ldr
        0x00_00_00_02 => {
            opcodes::ldr(space);
        }
        //mov
        0x00_00_00_03 => {
            opcodes::mov(space);
        }
        //================================= 0x10 ==================================
        //add
        0x10_00_00_01 => {
            opcodes::add(space);
        },
        //sub
        0x10_00_00_02 => {
            opcodes::sub(space);
        },
        //================================= 0x20 ==================================
        //jmp
        0x20_00_00_01 => {
            opcodes::jmp(space);
        },
        //jez
        0x20_00_00_02 => {
            opcodes::jez(space);
        },
            
        //jnz
        0x20_00_00_03 => {
            opcodes::jnz(space);
        },
        //jnc
        0x20_10_00_03 => {
            opcodes::jnc(space);
        },
            
        

        //================================= 0x30 ==================================
        //pnt
        0x30_00_00_01 => {
            opcodes::pnt(space);
        },        
        //================================= 0xF0 ==================================
        //cmp
        0xF0_00_00_01 => {
            opcodes::cmp(space);
        }
        _ => {
            println!("{:#08x}", space.pgm_array[space.reg_pc as usize]);
            
            panic!("not an OP code")
        },
    }
}




fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn print_pgm(space: &mut BytecodeWorkspace){
    for n in &space.pgm_array {
        print!("{}\n", n);
    }
}

fn print_state(space: &mut BytecodeWorkspace){
    println!("a: {}", space.reg_array[1]);
    println!("b: {}", space.reg_array[2]);
    println!("c: {}", space.reg_array[3]);
    println!("d: {}", space.reg_array[4]);
    println!("e: {}", space.reg_array[5]);
    println!("f: {}", space.reg_array[6]);
    println!("g: {}", space.reg_array[7]);
    println!("a: {}", space.reg_array[8]);
    println!("b: {}", space.reg_array[9]);
    println!("c: {}", space.reg_array[10]);
    println!("d: {}", space.reg_array[11]);
    println!("pc: {}", space.reg_pc);
    println!("");
}

fn find_fib(count: u32){
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut c: u32 = 0;
    for n in 1..count {
        c = a + b;
        b = a;
        a = c;
    }
    println!("{}", c);
}