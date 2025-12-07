use std::default;
use std::fmt::Error;
use std::fs::{File, read};
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let mut dialPosition: i32 = 50;
    let mut NeutralCounter = 0;
    let mut DialInstructions: Vec<String> = Vec::new();
    let file: File = File::open("DialInstructions.txt")?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line: String = line_result?;
        DialInstructions.push(line);
    }

    for instruction in DialInstructions {
        match instruction.chars().next().unwrap() {
            'R' => {
                let rotationNumber: i32 = instruction[1..].parse().unwrap();
                dialPosition += rotationNumber;
                if (dialPosition > 99) {
                    dialPosition %= 100;
                }

                if (dialPosition == 0) {
                    NeutralCounter += 1;
                }
            }
            'L' => {
                let rotationNumber: i32 = instruction[1..].parse().unwrap();
                dialPosition -= rotationNumber;
                if (dialPosition < 0) {
                    dialPosition %= 100;
                    if dialPosition != 0 {
                        dialPosition += 100;
                    }
                }

                if (dialPosition == 0) {
                    NeutralCounter += 1;
                }
            }
            _ => panic!("Oh no!??!?!!?"),
        }
        dbg!(instruction, dialPosition);
    }

    println!("{NeutralCounter}");

    Ok(())
}