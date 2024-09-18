use std::{collections::BTreeMap, ops::Div};

fn main() {
    // values from the task
    let temperatures = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    // Hashmap is also a viable solution, I decided to use BTreeMap to have sorted keys at the end
    let mut intervals: BTreeMap<(isize, isize), Vec<f64>> = BTreeMap::new();

    for &temp in temperatures.iter() {
        let lower_bound = temp.div(10.0 as f64).floor() as isize * 10;
        let upper_bound = lower_bound + 10;

        // Adding tempearature to the given interval
        intervals
            .entry((lower_bound, upper_bound))
            .or_insert(Vec::new())
            .push(temp);
    }

    for (range, temps) in intervals {
        println!("[{}, {}): {:?}", range.0, range.1, temps);
    }
}
