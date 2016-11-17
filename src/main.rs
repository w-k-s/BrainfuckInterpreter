use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Brainfuck{
	tape 		: Vec<u32>,
	tape_head	: usize,
	stack 		: Vec<u32>,
	stack_head	: usize,
}

impl Brainfuck{
	fn new(tape_length: usize,max_stack_level:usize) -> Brainfuck{
		let b = Brainfuck{
			tape: vec![0u32;tape_length],
			tape_head: 0usize,
			stack: vec![0u32;max_stack_level],
			stack_head:0usize
		};

		b
	}

	fn interpret(&mut self, source: &str)->Result<(),&str>{
		let tokens : Vec<char>= source.chars().collect();
		let mut i = 0;
		while true{
					
			if i >= tokens.len(){
				break;
			}

			let c = tokens[i];
			if c == '+'{
				self.tape[self.tape_head] += 1;
			}
			else if c == '-'{
				self.tape[self.tape_head] -= 1;
			}
			else if c == '>' {
				if self.tape_head == self.tape.capacity(){
					return Err("Data pointer out of range")
				}
				self.tape_head += 1
			}
			else if c == '<'{
				self.tape_head = try!(self.tape_head.checked_sub(1usize).ok_or("Data pointer out of range"));
			}
			else if c == '['{
				if self.stack_head == self.stack.capacity(){
					return Err("Nested loop limit reached")
				}
				self.stack[self.stack_head] = i as u32;
				self.stack_head += 1
			}else if c == ']'{
				if self.stack_head == 0{
					return Err("Mismatched brackets")
				}
				if self.tape[self.tape_head] != 0{
					self.stack_head -= 1;
					i = (self.stack[self.stack_head]+1) as usize;
					
					continue;
				}
			}
		}
		return Ok(())
	}
}

fn main() {
    /*let file_path = env::args().nth(1).expect("Expected path to brainfuck file");
    let path = Path::new(&file_path);
    let mut content = String::new();
    let _ = File::open(&path)
    	.and_then(|mut file| file.read_to_string(&mut content))
    	.map_err(|err| panic!("Failed to read file '{:?}': {}",path.to_str(),err));

    print!("Content: {}\n",content)
    */
    let mut brainfuck = Brainfuck::new(2,2);
    brainfuck.interpret("++[-]")
    	
    	.map_err(|e| println!("{}",e));
    println!("{:?}",brainfuck);
}
