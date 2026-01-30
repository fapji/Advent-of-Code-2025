use std::{io::{self, Read, Write, stdout}, vec};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut ids: Vec<String> = input.split(',').map(|s| s.trim().to_string()).collect();
    let s = "hallo";
    let s2 = "hallo".to_string();

    println!("\n\n\n {input} \n\n");

    for id in ids.iter() {
        println!("{}", {id})
    }

    println!(" \n \n ");
    
    for id in ids {
        let parsed_id: Vec<&str> = id.split('-').collect();
        for id in parsed_id {
            println!("{}", {id});
        }
    }

    Ok(())
}
