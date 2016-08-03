use std::fmt;
use std::io;
use std::io::prelude::*;

pub fn run<S: ToString>(src: S) -> Result<String, Error> {
    let ops = match parse_src(src.to_string()) {
        Ok(ops) => ops,
        Err(err) => return Err(err),
    };

    process(ops)
}

struct Tape {
    cells: Vec<u8>,
    ptr: usize,
}

impl Tape {
    fn new() -> Tape {
        Tape {
            cells: vec!(0),
            ptr: 0,
        }
    }

    fn inc_ptr(&mut self) -> Result<(), Error> {
        match self.ptr.checked_add(1) {
            Some(v) => {
                self.ptr = v;
                if self.cells.len() == self.ptr {
                    self.cells.push(0);
                }
                Ok(())
            },
            None => return Err(Error::PtrOverflow),
        }
    }

    fn dec_ptr(&mut self) -> Result<(), Error> {
        match self.ptr.checked_sub(1) {
            Some(v) => {
                self.ptr = v;
                Ok(())
            },
            None => return Err(Error::PtrUnderflow),
        }
    }

    fn inc_val(&mut self) -> Result<(), Error> {
        match self.cells.get_mut(self.ptr) {
            Some(v) => {
                *v = v.wrapping_add(1);
                Ok(())
            },
            None => return Err(Error::MissingCell),
        }
    }

    fn dec_val(&mut self) -> Result<(), Error> {
        match self.cells.get_mut(self.ptr) {
            Some(v) => {
                *v = v.wrapping_sub(1);
                Ok(())
            },
            None => return Err(Error::MissingCell),
        }
    }

    fn input(&mut self) -> Result<(), Error> {
        let mut input = [0; 1];

        match io::stdin().read(&mut input) {
            Ok(b) => b as u8,
            Err(err) => return Err(Error::IO(err)),
        };

        match self.cells.get_mut(self.ptr) {
            Some(cell) => *cell = *input.get(0).expect("Input was empty"),
            None => return Err(Error::MissingCell),
        }

        Ok(())
    }

    fn output(&self) -> Result<u8, Error> {
        match self.cells.get(self.ptr) {
            Some(v) => Ok(*v),
            None => Err(Error::MissingCell),
        }
    }

    fn skip_right(&mut self, ptr: usize) -> Result<(), Error> {
        if match self.cells.get(self.ptr) {
            Some(v) => *v == 0,
            None => return Err(Error::MissingCell),
        } {
            self.ptr = ptr;
        }
        Ok(())
    }

    fn skip_left(&mut self, ptr: usize) {
        self.ptr = ptr;
    }
}

fn process(ops: Vec<Op>) -> Result<String, Error> {
    let mut tape = Tape::new();

    let mut ops_ptr = 0;

    let mut output = String::new();

    loop {
        if let Some(op) = ops.get(ops_ptr) {
            match match *op {
                Op::IncPtr => tape.inc_ptr(),
                Op::DecPtr => tape.dec_ptr(),
                Op::IncVal => tape.inc_val(),
                Op::DecVal => tape.dec_val(),
                Op::Input => tape.input(),
                Op::Output => {
                    let result = tape.output();
                    match result {
                        Ok(b) => {
                            output.push(b as char);
                            Ok(())
                        },
                        Err(err) => Err(err),
                    }
                },
                Op::SkipRight(ptr) => tape.skip_right(ptr),
                Op::SkipLeft(ptr) => {
                    tape.skip_left(ptr);
                    Ok(())
                },
            } {
                Ok(()) => (),
                Err(err) => return Err(err),
            }
        } else {
            break;
        }
        ops_ptr += 1;
    }

    Ok(output)
}

fn parse_src(src: String) -> Result<Vec<Op>, Error> {
    let mut vec = vec!();

    let mut count = 0;

    let mut skip_right = vec!();

    for c in src.chars() {
        let op = match c {
            '>' =>  Op::IncPtr,
            '<' =>  Op::DecPtr,
            '+' =>  Op::IncVal,
            '-' =>  Op::DecVal,
            ',' =>  Op::Input,
            '.' =>  Op::Output,
            '[' =>  {
                        skip_right.push(count);
                        Op::SkipRight(0)
                    },
            ']' =>  {
                        let ptr = match skip_right.pop() {
                            Some(ptr) => ptr,
                            None => return Err(Error::InvalidSource),
                        };
                        let open = match vec.get_mut(ptr) {
                            Some(open) => open,
                            None => return Err(Error::InvalidPtr),
                        };
                        *open = Op::SkipRight(count);
                        Op::SkipLeft(ptr)
                    },
            _ =>    continue,
        };
        vec.push(op);
        count += 1;
    }
    if !skip_right.is_empty() {
        return Err(Error::InvalidSource);
    }

    Ok(vec)
}

#[derive(Debug)]
pub enum Error {
    InvalidSource,
    InvalidPtr,
    PtrOverflow,
    PtrUnderflow,
    MissingCell,
    IO(io::Error),
}

#[derive(Copy, Clone)]
enum Op {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Input,
    Output,
    SkipRight(usize),
    SkipLeft(usize),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::IncPtr => write!(f, ">"),
            Op::DecPtr => write!(f, "<"),
            Op::IncVal => write!(f, "+"),
            Op::DecVal => write!(f, "-"),
            Op::Input => write!(f, ","),
            Op::Output => write!(f, "."),
            Op::SkipRight(_) => write!(f, "["),
            Op::SkipLeft(_) => write!(f, "]"),
        }
    }
}
