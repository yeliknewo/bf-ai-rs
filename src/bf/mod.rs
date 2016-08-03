use std::fmt;

pub fn run<S: ToString>(program: S) -> Result<String, Error> {
    let mut vec = vec!();

    let mut count = 0;

    let mut skip_right = vec!();

    for c in program.to_string().chars() {
        let op = match c {
            '>' => Some(Op::IncPtr),
            '<' => Some(Op::DecPtr),
            '+' => Some(Op::IncVal),
            '-' => Some(Op::DecVal),
            ',' => Some(Op::Input),
            '.' => Some(Op::Output),
            '[' =>  {
                        skip_right.push(count);
                        Some(Op::SkipRight(0))
                    },
            ']' =>  {
                        if let Some(ptr) = skip_right.pop() {
                            vec
                            Some(Op::SkipLeft(ptr))
                        } else {
                            return Err(Error::InvalidSource);
                        }

                    },
            _ => None,
        };
        if let Some(op) = op {
            vec.push(op);
            count += 1;
        }
    }

    Ok(program.to_string())
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    InvalidSource,
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
