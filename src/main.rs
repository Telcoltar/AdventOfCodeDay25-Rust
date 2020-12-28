mod test_solution;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use std::collections::HashMap;

fn get_input_data(filename: &str) -> (i64, i64) {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let public_keys: Vec<i64> = f.lines().map(|l|l.unwrap().parse().unwrap()).collect();
    return (public_keys[0], public_keys[1]);
}

fn mod_pow(base: i64, exp: i64, modular: i64) -> i64 {
    let mut result = 1;
    for _i in 0..exp {
        result = (result * base) % modular;
    }
    return result;
}

fn discrete_log(order: i64, base: i64, arg: i64) -> i64 {
    let m = (order as f64).sqrt().ceil() as i64;
    let mut table: HashMap<i64, i64> = HashMap::new();
    for i in 0..m {
        table.insert(mod_pow(base, i, order), i);
    }
    let base_inverse = mod_pow(base, order - 2, order);
    let mut gamma = arg;
    for i in 0..m {
        if table.contains_key(&gamma) {
            return i * m + table.get(&gamma).unwrap().to_owned();
        }
        gamma = (gamma * mod_pow(base_inverse, m, order)) % order;
    }
    return -1;
}

fn solution_part_1(filename: &str) -> i64 {
    let (door_public_key, card_public_key) = get_input_data(filename);
    let door_loop_size = discrete_log(20201227, 7, door_public_key);
    debug!("{:?}", door_loop_size);
    return mod_pow(card_public_key, door_loop_size, 20201227);
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
}
