use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Brainfuck {
    data: Vec<u8>,
    data_pointer: usize,
    stack: Vec<u32>,
    stack_pointer: usize,
}

impl Brainfuck {
    fn new() -> Brainfuck {
        Brainfuck {
            data: vec![0u8;100],
            data_pointer: 0usize,
            stack: vec![0u32;10],
            stack_pointer: 0usize,
        }
    }

    fn interpret(&mut self, source: &str) -> Result<String, &str> {
        let mut output: Vec<char> = vec![];
        let tokens: Vec<char> = source.chars().collect();
        let mut i = 0;//instruction pointer
        loop {
            if i >= tokens.len() {
                break;
            }

            let c = tokens[i];
            if c == '+' {
                self.data[self.data_pointer] = self.data[self.data_pointer].wrapping_add(1)
            } else if c == '-' {
                self.data[self.data_pointer] = self.data[self.data_pointer].wrapping_sub(1)
            } else if c == '>' {
                if self.data_pointer == self.data.capacity() {
                    return Err("Data pointer beyond range");
                }
                self.data_pointer += 1
            } else if c == '<' {
                self.data_pointer =
                    try!(self.data_pointer.checked_sub(1usize).ok_or("Data pointer below range"));
            } else if c == '.' {
                output.push(self.data[self.data_pointer] as char)
            } else if c == '[' {
                if self.stack_pointer == self.stack.capacity() {
                    return Err("Nested loop limit reached");
                }
                self.stack[self.stack_pointer] = i as u32;
                self.stack_pointer += 1
            } else if c == ']' {
                if self.stack_pointer == 0 {
                    return Err("Mismatched brackets");
                }
                if self.data[self.data_pointer] != 0 {
                    i = (self.stack[self.stack_pointer - 1] + 1) as usize;
                    continue;
                } else {
                    self.stack_pointer -= 1;
                }
            }
            i += 1;
        }
        Ok(output.into_iter().collect())
    }

    fn reset(&mut self) -> &mut Brainfuck {
        self.data.clear();
        self.data_pointer = 0;
        self.stack.clear();
        self.stack_pointer = 0;
        return self;
    }
}

impl fmt::Display for Brainfuck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        for (i, value) in self.data.iter().enumerate() {
            let mut value_string: String;
            if i == self.data_pointer {
                value_string = format!("[{}] ", self.data[i]);
            } else {
                value_string = format!("{} ", self.data[i]);
            }
            display.push_str(&value_string);
        }
        write!(f, "{}", display)
    }
}

fn main() {
    // let file_path = env::args().nth(1).expect("Expected path to brainfuck file");
    // let path = Path::new(&file_path);
    // let mut content = String::new();
    // let _ = File::open(&path)
    // .and_then(|mut file| file.read_to_string(&mut content))
    // .map_err(|err| panic!("Failed to read file '{:?}': {}",path.to_str(),err));

    let source = env::args().nth(1).unwrap_or("".to_string());

    let mut brainfuck = Brainfuck::new();
    match brainfuck.interpret(&source) {
        Ok(text) => println!("{}", text),
        Err(err) => println!("Error: {}", err),
    };
    println!("{}", brainfuck);
}

#[test]
fn test_add() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("+++");
    assert_eq!(brainfuck.data[0], 3);
}

#[test]
fn test_sub() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("++-");
    assert_eq!(brainfuck.data[0], 1u8);
}

#[test]
fn test_signed_sub() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("---");
    assert_eq!(brainfuck.data[0], 253u8);
}

#[test]
fn test_signed_add() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("---++++-");
    assert_eq!(brainfuck.data[0], 0u8);
}

#[test]
fn test_shift_left() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("++>+");
    assert_eq!(brainfuck.data[0], 2u8);
    assert_eq!(brainfuck.data[1], 1u8)
}

#[test]
fn test_shift_right() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("+++>++<--");
    assert_eq!(brainfuck.data[0], 1u8);
    assert_eq!(brainfuck.data[1], 2u8);

    let r = brainfuck.reset().interpret("<");
    assert_eq!(r, Err("Data pointer below range"));
}

#[test]
fn test_loop() {
    let mut brainfuck = Brainfuck::new();
    let _ = brainfuck.interpret("+++[-]+");
    assert_eq!(brainfuck.data[0], 1);
}

#[test]
fn test_mismatched_brackets() {
    let mut brainfuck = Brainfuck::new();
    let r = brainfuck.interpret("]");
    assert_eq!(r, Err("Mismatched brackets"));
}

#[test]
fn test_ascii_out() {
    let mut brainfuck = Brainfuck::new();
    let r = brainfuck.interpret("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.\
                    <<+++++++++++++++.>.+++.------.--------.>+.>.");
    assert_eq!(r, Ok("Hello World!\n".to_string()));
}
