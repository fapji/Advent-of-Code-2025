use std::io::{self, Read};

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

        println!("number 1: {}  number 2: {}", { parsed_id[0] }, {
            parsed_id[1]
        });

        let id1 = parsed_id[0].parse::<i64>().unwrap();
        let id2 = parsed_id[1].parse::<i64>().unwrap();

        for id in id1..=id2 {
            // println!("Current number being checked {id}");
            if check_every_pattern_with_strings(id) {
                sum_invalidids += id;
                // println!("current count: {sum_invalidids} \n");
            }
        }
    }

    print!("\n\n{}", { sum_invalidids });

    Ok(())
}

fn check_every_pattern(originnum: i64) -> bool {
    let mut is_invalid = false;

    //if the number is a power of 10 add 1
    let numdigits: f32 = (originnum as f32).log10().floor() + 1.0;
    let maximum_cluster = (numdigits / 2.0).floor();

    for cluster_size in 1..maximum_cluster as i32 + 1 {
        //if the number of digits divided by the cluster size is a number without comma's do the checks (This checks if a repeating pattern is possible)
        if numdigits as i32 % cluster_size == 0 {
            let mut is_invalid_per_clustersize = true;
            // println!("Cluster Size: {}", { cluster_size });
            let amount_of_checks = numdigits as i32 / cluster_size;
            // println!("amount_of_checks: {}", { amount_of_checks });
            let mut previous_cluster = 0;
            for index in 0..amount_of_checks {
                // println!("Index: {}", index);

                //cluster_size * the size of the index is the position you wanna take it out of, if cluster_size is 2 index needs to move by 2
                let mut current_cluster = originnum
                    / 10i64.pow(numdigits as u32 - (cluster_size * index + cluster_size) as u32);
                //Now Add something with % (cluster_size needs to be cut on the right after the first index)
                current_cluster = current_cluster % 10i64.pow((cluster_size) as u32);

                if index == 0 {
                    previous_cluster = current_cluster;
                }
                // println!("Current Cluster: {current_cluster}");

                if previous_cluster != current_cluster {
                    is_invalid_per_clustersize = false;
                    // println!("Sadge");
                    break;
                }
                previous_cluster = current_cluster;
            }
            if is_invalid_per_clustersize {
                is_invalid = is_invalid_per_clustersize;
                break;
            }
        }
    }
    // println!("amount of divisions: {}", { maximum_cluster });

    // if is_invalid {
    //     println!("repeating pattern \n");
    // } else {
    //     println!("no \n");
    // }

    return is_invalid;
}

fn check_every_pattern_with_strings(originnum: i64) -> bool {
    let num_str = originnum.to_string();
    let max_checks = num_str.len() / 2;
    let mut is_pattern = false;
    for cluster_size in 1..max_checks + 1 {
        if num_str.len() % cluster_size == 0 {
            let mut is_pattern_per_clustersize = true;
            let amount_of_checks = num_str.len() / cluster_size;

            let mut previous_cluster: &str = "";
            for index in 0..amount_of_checks {
                let current_cluster = &num_str[index..index + cluster_size];
                if index == 0 {
                    previous_cluster = current_cluster;
                }

                if previous_cluster != current_cluster {
                    is_pattern_per_clustersize = false;
                    // println!("Sad, not a pattern");
                    break;
                }

                previous_cluster = current_cluster;
            }
            if is_pattern_per_clustersize {
                is_pattern = is_pattern_per_clustersize;
                break;
            }
        }
    }
    return is_pattern;
}
