fn main() {
    let cool_number = 1325i64;
    if check_every_pattern_with_strings(cool_number) {
        println!("Pattern Found");
    } else {
        println!("No Pattern Found");
    }
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
                let current_cluster = &num_str[index..(index * cluster_size)];
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
