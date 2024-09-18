use std::collections::HashSet;

fn main() {
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [5, 6, 7, 8, 1, 2].iter().cloned().collect();

    let result = set1.intersection(&set2).collect::<HashSet<_>>();

    println!("Intersection: {:?}", result);
}
