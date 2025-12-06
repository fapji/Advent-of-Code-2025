use std::fmt::Error;  
use std::fs::{File, read};
use std::io::{self,BufRead};

fn main() -> std::io::Result<()> {
    
    let mut dialPosition:i32 = 50;
    let mut DialInstructions:Vec<String> = Vec::new();
    let file:File = File::open("DialInstructions.txt")?;
    let reader = io::BufReader::new(file);

    for line_result in reader.lines() {
        let line:String = line_result?;
        DialInstructions.push(line);
    }

    for instruction in DialInstructions {
        println!("{instruction}");
    }

    

    Ok(())
}