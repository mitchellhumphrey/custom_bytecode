#![allow(dead_code)]
#![allow(unused_variables)]

mod structs;
use crate::structs::BytecodeWorkspace;
use std::process::exit;




fn main() {
    let space =  &mut BytecodeWorkspace { 
        reg_array: Vec::new(), 
        mem_array: Vec::new(), 
        pgm_array: Vec::new(),
        reg_pc: 0,
        reg_sp: 0,
        flag_zero: false,
    };
    let instruction: u32;
    let params: [u32; 4];

    init(space, "./fib.bin".to_string());
    //print_pgm(space);
    
    loop {
        one_loop(space);
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





fn one_loop(space: &mut BytecodeWorkspace){

    if space.reg_pc  >= space.pgm_array.len() as u32 {
        println!("{}", space.reg_pc);
        panic!("attempted to read end of file")
    }
    // if space.reg_pc < 0 {
    //     panic!("attempted to execute negative pc value")
    // }

    match space.pgm_array[space.reg_pc as usize] {

        //================================= 0x00 ==================================

        //nop
        0x00_00_00_00 => space.reg_pc += 1,

        //hlt
        0x00_00_00_01 => {
            // println!("");
            // println!("{}", space.reg_array[1]);
            exit(0);
        },
        //ldr
        0x00_00_00_02 => {
            let temp = space.pgm_array[space.reg_pc as usize + 1];
            let temp2 = space.pgm_array[space.reg_pc as usize + 2];
            
            let reg_a= temp as u8;

            space.reg_array[reg_a as usize] = temp2;
            space.reg_pc += 3;
        }
        //mov
        0x00_00_00_03 => {
            let temp = space.pgm_array[space.reg_pc as usize + 1];
            
            let reg_a= temp as u8;
            let reg_b= (temp >> 8) as u8;

            space.reg_array[reg_b as usize] = space.reg_array[reg_a as usize];
            space.reg_pc += 2;
        }
        //================================= 0x10 ==================================
        //add
        0x10_00_00_01 => {
            let temp = space.pgm_array[space.reg_pc as usize + 1];
            
            let reg_a = temp as u8;
            let reg_b= (temp >> 8) as u8;
            let reg_c= (temp >> 16) as u8;
            print_state(space);
            space.reg_array[reg_c as usize] = space.reg_array[reg_a as usize] + space.reg_array[reg_b as usize];
            space.reg_pc += 2
        },
        //sub
        0x10_00_00_02 => {
            let temp = space.pgm_array[space.reg_pc as usize + 1];
            
            let reg_a = temp as u8;
            let reg_b= (temp >> 8) as u8;
            let reg_c= (temp >> 16) as u8;
            println!("a:{} b:{} c:{}", reg_a,reg_b,reg_c);
            println!("{}", space.reg_array[reg_c as usize]);
            space.reg_array[reg_c as usize] = space.reg_array[reg_a as usize] - space.reg_array[reg_b as usize];
            space.reg_pc += 2;
            println!("{}\n", space.reg_array[reg_c as usize]);
        },
        //================================= 0x20 ==================================
        //jmp
        0x20_00_00_01 => {
            space.reg_pc = space.pgm_array[space.reg_pc as usize + 1];
        },
        //jez
        0x20_00_00_02 => {
            if space.flag_zero {
                space.reg_pc = space.pgm_array[space.reg_pc as usize + 1];
            }
            else {
                space.reg_pc += 2
            }
            
        },
        //jnz
        0x20_00_00_03 => {
            if  !space.flag_zero {
                space.reg_pc = space.pgm_array[space.reg_pc as usize + 1];
            }
            else {
                space.reg_pc += 2
            }
            
        }

        //================================= 0x30 ==================================
        //pnt
        0x30_00_00_01 => {
            let temp = space.pgm_array[space.reg_pc as usize + 1];
            
            let reg_a= temp as u8;

            print!("{}", space.reg_array[reg_a as usize] as u8 as char);
            space.reg_pc += 2
        }


        
        //================================= 0xF0 ==================================
        //cmp
        0xF0_00_00_01 => {
            let temp = space.pgm_array[space.reg_pc as usize + 1];
            
            let reg_a = temp as u8;
            let reg_b = (temp >> 8) as u8;

            space.flag_zero = (space.reg_array[reg_a as usize] - space.reg_array[reg_b as usize]) == 0;
            // println!("{}", space.flag_zero);
            space.reg_pc += 2;
        }







        _ => {
            println!("{}", space.pgm_array[space.reg_pc as usize]);
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
    println!("pc: {}", space.reg_pc);
    println!("");
}