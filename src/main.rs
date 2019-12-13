use std::io::prelude::*;
use std::u8;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub enum Instr {
    Forward(usize),
    Backward(usize),
    Increment(u8),
    Decrement(u8),
    Output,
    Input,
    Jump(isize),
}

pub fn opt(instr: Vec<Instr>) -> Vec<Instr> {
    use Instr::*;
    let mut stack = Vec::with_capacity(instr.len() / 2);
    for i in instr {
        match stack.pop() {
            Some(x) => {
                let n = match (x, i) {
                    (Forward(a), Forward(b)) => Forward(a.saturating_add(b)),
                    (Backward(a), Backward(b)) => Backward(a.saturating_add(b)),
                    (Increment(a), Increment(b)) => Increment(a.wrapping_add(b)),
                    (Decrement(a), Decrement(b)) => Decrement(a.wrapping_add(b)),
                    (x, i) => {
                        stack.push(x);
                        i
                    }
                };
                stack.push(n)
            }
            None => stack.push(i),
        }
    }
    stack
}

fn compile(source: &str) -> Vec<Instr> {
    let mut v = source
        .chars()
        .filter_map(|ch| match ch {
            '>' => Some(Instr::Forward(1)),
            '<' => Some(Instr::Backward(1)),
            '+' => Some(Instr::Increment(1)),
            '-' => Some(Instr::Decrement(1)),
            '.' => Some(Instr::Output),
            ',' => Some(Instr::Input),
            '[' => Some(Instr::Jump(1)),
            ']' => Some(Instr::Jump(-1)),
            _ => None,
        })
        .collect::<Vec<Instr>>();
    v = opt(v);

    // patch in jumps
    let mut stack_fwd = Vec::new();
    let mut stack_back = Vec::new();
    for (idx, x) in v.iter_mut().enumerate() {
        match x {
            Instr::Jump(i) if *i > 0 => {
                stack_fwd.push(idx);
            }
            Instr::Jump(i) if *i < 0 => {
                let loc = stack_fwd.pop().expect("] before [!");
                *i = loc as isize - idx as isize;
                stack_back.push((loc, idx as isize));
            }
            _ => {}
        }
    }

    for (idx, jump) in stack_back {
        v[idx] = Instr::Jump(jump - idx as isize);
    }

    v
}

fn input() -> Option<u8> {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
}

fn run(instr: &[Instr]) {
    let mut data: Vec<u8> = (0..1000).map(|_| 0).collect::<Vec<_>>();
    let mut pc = 0;
    let mut ptr = 0;
    loop {
        match instr.get(pc) {
            Some(Instr::Jump(i)) => {
                if *i > 0 {
                    if data[ptr] == 0 {
                        pc = (pc as isize + *i) as usize;
                    }
                } else {
                    if data[ptr] != 0 {
                        pc = (pc as isize + *i) as usize;
                    }
                }
            }
            Some(Instr::Forward(i)) => ptr += i,
            Some(Instr::Backward(i)) => ptr -= i,
            Some(Instr::Increment(i)) => data[ptr] = data[ptr].wrapping_add(*i),
            Some(Instr::Decrement(i)) => data[ptr] = data[ptr].wrapping_sub(*i),
            Some(Instr::Output) => print!("{}", data[ptr] as char),
            Some(Instr::Input) => data[ptr] = input().unwrap(),
            None => break,
        }
        pc += 1;
    }
}

fn main() {
    run(&compile("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."));
    std::io::stdout().flush().unwrap();
}
