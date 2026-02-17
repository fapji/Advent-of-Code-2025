fn main() {
    let cool_number = 2121212119i64;
    // _split_at_half(cool_number);
    check_every_pattern(cool_number);
}

fn _split_at_half(num: i64) {
    let originnum = num;
    let numdigits: f32 = f32::log(originnum as f32, 10.0).ceil();
    //print!("logarithm: {}", {numdigits});
    let tophalfnum = originnum / 10i64.pow(numdigits as u32 / 2u32);
    let bottomhalfnum = originnum % 10i64.pow(numdigits as u32 / 2u32);
    print!("top: {}, bottom: {}", { tophalfnum }, { bottomhalfnum });
}

fn check_every_pattern(originnum: i64) -> bool {
    println!("Given number: {} \n", originnum);

    let mut is_invalid = false;

    //if the number is a power of 10 add 1, was doing .ceil() before but didn't catch this case.
    let numdigits: f32 = (originnum as f32).log10().floor() + 1.0;
    // println!("Number of Digits {} \n", {numdigits});

    let maximum_cluster = (numdigits / 2.0).floor();
    // println!("Maximum Cluster {} \n", {maximum_cluster});

    for cluster_size in 1..maximum_cluster as i32 + 1 {
        println!("Current Cluster Size: {}", { cluster_size });

        //if the number of digits divided by the cluster size is a number without comma's do the checks (This checks if a repeating pattern is possible)
        if numdigits as i32 % cluster_size == 0
        {
            let mut is_invalid_per_clustersize = true;
            let amount_of_checks = numdigits as i32 / cluster_size;
            println!("amount_of_checks: {}", { amount_of_checks });

            let mut previous_cluster = 0;
            for index in 0..amount_of_checks {
                println!("Index: {}", index);

                //cluster_size * the size of the index is the position you wanna take it out of, if cluster_size is 2 index needs to move by 2
                let mut current_cluster = originnum
                    / 10i64.pow(numdigits as u32 - (cluster_size * index + cluster_size) as u32);
                println!("Current Cluster Before modulo: {current_cluster}");

                //Now Add something with % (cluster_size needs to be cut on the right after the first index)
                current_cluster = current_cluster % 10i64.pow((cluster_size) as u32);
                println!("Current Cluster After modulo: {current_cluster}");

                if index == 0 {
                    previous_cluster = current_cluster;
                }

                if previous_cluster != current_cluster {
                    is_invalid_per_clustersize = false;
                    println!("Sadge");
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
    //println!("amount of divisions done: {}", { maximum_cluster });

    if is_invalid {
        println!("Yipee repeated pattern");
    } else {
        println!("sadge no repeating pattern");
    }

    return is_invalid;
}
