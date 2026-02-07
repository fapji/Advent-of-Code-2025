use std::{io::{self, Read}};

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ids: Vec<String> = input.split(',').map(|s| s.trim().to_string()).collect();

    /*println!("\n\n\n {input} \n\n");
    for id in ids.iter() {
        println!("{}", {id})
    }
    println!("\n\n");*/

    let mut sum_invalidids = 0;

    for id in ids {
        let parsed_id: Vec<&str> = id.split('-').collect();

        println!("number 1: {}  number 2: {}",{parsed_id[0]}, {parsed_id[1]});

        let id1 = parsed_id[0].parse::<i64>().unwrap();
        let id2 = parsed_id[1].parse::<i64>().unwrap();

        for id in id1..id2 {
        let numdigits:f32 = f32::log(id as f32 , 10.0).ceil();
        let tophalfnum = id / 10i64.pow(numdigits as u32 / 2u32);
        let bottomhalfnum = id % 10i64.pow(numdigits as u32 / 2u32);

            if tophalfnum == bottomhalfnum {
                sum_invalidids += id;
            }
            //println!("{}", {id});
        }   
    }

    print!("\n\n{}",{sum_invalidids});

    Ok(())
}
