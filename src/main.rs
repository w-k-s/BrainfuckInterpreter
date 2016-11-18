use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Brainfuck{
	tape 		: Vec<u8>,
	tape_head	: usize,
	stack 		: Vec<u32>,
	stack_head	: usize,
}

impl Brainfuck{
	fn new() -> Brainfuck{
		Brainfuck{
			tape: vec![0u8;100],
			tape_head: 0usize,
			stack: vec![0u32;10],
			stack_head:0usize
		}
	}

	fn interpret(&mut self, source: &str)->Result<String,&str>{
		let mut output : Vec<char> = vec![];
		let tokens : Vec<char>= source.chars().collect();
		let mut i = 0;
		loop{
			if i >= tokens.len(){
				break;
			}

			let c = tokens[i];
			if c == '+'{
				self.tape[self.tape_head] = self.tape[self.tape_head].wrapping_add(1)
			}
			else if c == '-'{
				self.tape[self.tape_head] = self.tape[self.tape_head].wrapping_sub(1)
			}
			else if c == '>' {
				if self.tape_head == self.tape.capacity(){
					return Err("Data pointer beyond range")
				}
				self.tape_head += 1
			}
			else if c == '<'{
				self.tape_head = try!(self.tape_head.checked_sub(1usize).ok_or("Data pointer below range"));
			}
			else if c == '.'{
				output.push(self.tape[self.tape_head] as char)
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
					i = (self.stack[self.stack_head-1]+1) as usize;
					continue;
				}else{
					self.stack_head -= 1;
				}
			}
			i += 1;
		}
		Ok(output.into_iter().collect())
	}

	fn reset(&mut self)->&mut Brainfuck{
		self.tape.clear();
		self.tape_head = 0;
		self.stack.clear();
		self.stack_head = 0;
		return self;
	}
}

fn main() {
    let file_path = env::args().nth(1).expect("Expected path to brainfuck file");
    let path = Path::new(&file_path);
    let mut content = String::new();
    let _ = File::open(&path)
    	.and_then(|mut file| file.read_to_string(&mut content))
    	.map_err(|err| panic!("Failed to read file '{:?}': {}",path.to_str(),err));
	
    let mut brainfuck = Brainfuck::new();
    match brainfuck.interpret(content){
		Ok(text)=>println!("{}",text),
		Err(err)=>println!("Error: {}",err)
	};
}

#[test]
fn test_add(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("+++");
	assert_eq!(brainfuck.tape[0],3);
}

#[test]
fn test_sub(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("++-");
	assert_eq!(brainfuck.tape[0],1u8);
}

#[test]
fn test_signed_sub(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("---");
	assert_eq!(brainfuck.tape[0],253u8);
}

#[test]
fn test_signed_add(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("---++++-");
	assert_eq!(brainfuck.tape[0],0u8);
}

#[test]
fn test_shift_left(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("++>+");
	assert_eq!(brainfuck.tape[0],2u8);
	assert_eq!(brainfuck.tape[1],1u8)
}

#[test]
fn test_shift_right(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("+++>++<--");
	assert_eq!(brainfuck.tape[0],1u8);
	assert_eq!(brainfuck.tape[1],2u8);

	let r = brainfuck.reset().interpret("<");
	assert_eq!(r,Err("Data pointer below range"));
}

#[test]
fn test_loop(){
	let mut brainfuck = Brainfuck::new();
	let _ = brainfuck.interpret("+++[-]+");
	assert_eq!(brainfuck.tape[0],1);
}

#[test]
fn test_mismatched_brackets(){
	let mut brainfuck = Brainfuck::new();
	let r = brainfuck.interpret("]");
	assert_eq!(r,Err("Mismatched brackets"));
}

#[test]
fn test_ascii_out(){
	let mut brainfuck = Brainfuck::new();
	let r = brainfuck.interpret("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
	assert_eq!(r,Ok("Hello World!\n".to_string()));
}