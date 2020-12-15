use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
struct Instruction{
    op: String,
    val: isize,
}*/

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);

    let mut program = Vec::new();
    for line in buf.lines() {
        let line = line.unwrap();
        let mut itr = line.split_whitespace();
        let op = String::from(itr.next().unwrap());
        let val = itr.next().unwrap().parse::<isize>()?;
        program.push((op, val));
    }

    for n in 0..program.len() {
        if program.get(n).unwrap().0 == "acc" {
            continue;
        }
        let mut ctr: isize = 0;
        let mut acc = 0;
        let mut seen = HashSet::new();
        let mut looped = false;
        while ctr < program.len() as isize {
            let (op, val) = program.get(ctr as usize).unwrap();
            seen.insert(ctr);
            let last = ctr;
            ctr = match op.as_str() {
                "nop" if ctr != n as isize => ctr + 1,
                "jmp" if ctr == n as isize => ctr + 1,
                "acc" => {
                    acc += val;
                    ctr + 1
                }
                "jmp" if ctr != n as isize => ctr + val,
                "nop" if ctr == n as isize => ctr + val,
                _ => panic!("Unknown operator!"),
            };
            if seen.contains(&ctr) {
                //println!("Loop from {} to {}, acc: {}", last, ctr, acc);
                looped = true;
                break;
            }
        }
        if !looped {
            println!("Program {} terminated. Acc: {}",n, acc);
        }
    }
    Ok(())
}
