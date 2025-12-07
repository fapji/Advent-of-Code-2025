use std::default;
use std::fmt::Error;
use std::fs::{File, read};
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let mut dialPosition: i32 = 50;
    let mut neutralCounter = 0;
    let mut dialInstructions: Vec<String> = Vec::new();
    let file: File = File::open("DialInstructions.txt")?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line: String = line_result?;
        dialInstructions.push(line);
    }

    for instruction in dialInstructions {
        match instruction.chars().next().unwrap() {
            'R' => {
                let rotationNumber: i32 = instruction[1..].parse().unwrap();
                dialPosition += rotationNumber;
                if (dialPosition > 99) {
                    neutralCounter += dialPosition / 100;
                    dialPosition %= 100;
                }
            }
            'L' => {
                let rotationNumber: i32 = instruction[1..].parse().unwrap();
                if dialPosition == 0 {
                    neutralCounter -= 1;
                }

                dialPosition -= rotationNumber;

                if (dialPosition <= 0) {
                    neutralCounter += dialPosition / -100 + 1;
                    dialPosition %= 100;
                    if dialPosition != 0 {
                        dialPosition += 100;
                    }
                }
            }
            _ => panic!("Oh no!??!?!!?"),
        }
        dbg!(instruction, dialPosition, neutralCounter);
    }

    println!("{neutralCounter}");

    Ok(())
}
