fn main() {
    let cool_number = 11974834i32;
    // _split_at_half(cool_number);
    check_every_pattern(cool_number);
}

fn _split_at_half(num: i32) {
    let originnum = num;
    let numdigits: f32 = f32::log(originnum as f32, 10.0).ceil();
    //print!("logarithm: {}", {numdigits});
    let tophalfnum = originnum / 10i32.pow(numdigits as u32 / 2u32);
    let bottomhalfnum = originnum % 10i32.pow(numdigits as u32 / 2u32);
    print!("top: {}, bottom: {}", { tophalfnum }, { bottomhalfnum });
}

fn check_every_pattern(num: i32) -> bool {
    let originnum = num;
    let mut is_invalid = false;
    let numdigits: f32 = f32::log(originnum as f32, 10.0).ceil();
    let maximum_cluster = (numdigits / 2.0).floor();

    for cluster_size in 1..maximum_cluster as i32 + 1 {
        println!("Cluster Size: {}", { cluster_size });
        let amount_of_checks = numdigits as i32 / cluster_size;
        println!("amount_of_checks: {}", { amount_of_checks });
        let mut previous_cluster = 0;
        for index in 1..amount_of_checks + 1 {
            println!("Index: {}", { index });
            //cluster_size is the size of the cluster index is the position you wanna take it out of if cluster_size is 2 index needs to move by 2
            let current_cluster =
                originnum / 10i32.pow(numdigits as u32 - (cluster_size * index) as u32); //Now Add something with % index - 1 times cluster_size
            // let current_cluster =
            //     current_cluster_halfway % 10i32.pow(((index - 1) * cluster_size) as u32);
            println!("Current Cluster: {current_cluster}");
            if previous_cluster == current_cluster && current_cluster != 0 {
                is_invalid = true;
                println!("Yipeee");
                break;
            }
            previous_cluster = current_cluster;
        }
    }
    //println!("amount of divisions: {}", { maximum_cluster });

    return is_invalid;
}

// let mut my_vec: Vec<i32> = Vec::new();
// let tophalfnum = originnum / 10i32.pow(numdigits as u32 / 2u32);
