use std::env;

fn set_bit(value: i64, bit_index: usize, set_to_one: bool) -> i64 {
    if set_to_one {
        value | (1 << bit_index)
    } else {
        value & !(1 << bit_index)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <value> <bit_index> <set_to_one>", args[0]);
        eprintln!("Example: {} 13 2 1", args[0]);
        return;
    }

    // Parse args
    let value: i64 = args[1].parse().expect("Failed to parse value as i64");
    // Indexing from 0
    let bit_index: usize = args[2].parse().expect("Failed to parse bit_index as usize");
    let set_to_one: bool = args[3]
        .parse::<i32>()
        .expect("Failed to parse set_to_one as integer")
        != 0;

    // Setting bit
    let new_value = set_bit(value, bit_index, set_to_one);

    println!("New val binary: {:b} (decimals: {})", new_value, new_value);
}
