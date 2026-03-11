use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

fn skip_loop(buffer: &Vec<u8>, mut current_instruction: usize) -> usize {
    // for situations in which [ command is used but pointer is zero
    // Will skip all instructions until it finds ] command
    loop {
        let Some(token) = buffer.get(current_instruction) else {
            break;
        };
        if *token == b']' {
            current_instruction += 1;
            break;
        } else if *token == b'[' {
            // for nested loops
            current_instruction += 1;
            current_instruction = skip_loop(&buffer, current_instruction);
        }

        current_instruction += 1;
    }

    current_instruction
}

fn interpreter(mut file: File) {
    const CAPACITY: usize = 16;

    let mut memory: [u8; CAPACITY] = [0; CAPACITY]; // memory
    let mut stack = Vec::new(); // to house the instruction to pointer of [ 

    let mut current_instruction = 0;
    let mut current_memory_pointer = 0;

    let mut buffer = Vec::new();

    // read the whole file
    file.read_to_end(&mut buffer)
        .expect("could not read from file");

    let acceptable_symbols = [43, 44, 45, 46, 60, 62, 91, 93];
    // filter everything that is not in the language
    let buffer: Vec<u8> = buffer
        .into_iter()
        .filter(|x| acceptable_symbols.contains(x))
        .collect();

    loop {
        let Some(token) = buffer.get(current_instruction) else {
            println!();
            println!("{memory:?}");
            break;
        };

        //println!("token: {} instruction: {} cell: {}, memory: {}", *token as char, current_instruction, current_memory_pointer, array[current_memory_pointer]);
        match token {
            b'>' => {
                current_memory_pointer += 1;
                current_instruction += 1;
            }
            b'<' => {
                current_memory_pointer -= 1;
                current_instruction += 1;
            }
            b'+' => {
                memory[current_memory_pointer] = memory[current_memory_pointer] + 1;
                current_instruction += 1;
            }
            b'-' => {
                memory[current_memory_pointer] = memory[current_memory_pointer] - 1;
                current_instruction += 1;
            }
            b'[' => {
                if memory[current_memory_pointer] != 0 {
                    stack.push(current_instruction);
                    current_instruction += 1;
                } else {
                    current_instruction += 1;
                    current_instruction = skip_loop(&buffer, current_instruction)
                }
            }
            b']' => {
                if memory[current_memory_pointer] != 0 {
                    current_instruction = stack
                        .pop()
                        .expect("stack error happened. need to add debuging for later");
                } else {
                    current_instruction += 1;
                }
            }
            b'.' => {
                print!("{}", memory[current_memory_pointer] as char); // it is buffered and will only flush when full
                current_instruction += 1;
            }

            b',' => {
                // getting user input
                let mut input = String::new();
                let _ = stdin().read_line(&mut input).expect("faild to read input");

                let number = input
                    .trim()
                    .parse::<u8>()
                    .expect("input cannot be converted to a number");

                memory[current_memory_pointer] = number;
                current_instruction += 1;
            }
            _ => {
                current_instruction += 1;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("please enter brainf*ck file to interpret");
        return;
    }

    match File::open(&args[1]) {
        Ok(file) => interpreter(file),
        Err(_) => println!("Could not open file with name {:?}", &args[1]),
    };
}
