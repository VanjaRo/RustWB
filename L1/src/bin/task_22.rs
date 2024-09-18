// deleting the element without preserving same order
fn delete_ith_from_vec<T>(val: &mut Vec<T>, idx: usize) -> Result<(), String> {
    let val_len = val.len();
    if idx >= val.len() {
        return Err(format!("Given idx: {} is out of range", idx));
    }

    val.swap(idx, val_len - 1);
    val.pop();
    Ok(())
}

fn main() {
    let mut initial_vec = vec![1, 2, 3, 4, 5];
    println!("Vec before: {:?}", initial_vec);
    delete_ith_from_vec(&mut initial_vec, 0).unwrap();
    println!("Vec after: {:?}", initial_vec);
}
