use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // HashMap with Mutex
    hasmap_mut();
    println!("=======SEPARATOR=======");
    // Dashmap
    dashmap();
}

fn hasmap_mut() {
    // Wrap HashMap into Arc and Mutex
    // could potentially replece Mutex with RwLock to optimize the read accessibility
    let map = Arc::new(Mutex::new(HashMap::new()));

    let worker_count = 10;
    let worker_handlers = (0..worker_count)
        .map(|id| {
            let map_cln = map.clone();
            thread::spawn(move || {
                // try to access lock
                let mut map = map_cln.lock().unwrap();
                map.insert(id, id * id);
            })
        })
        .collect::<Vec<_>>();

    for handle in worker_handlers {
        handle.join().unwrap();
    }

    // Check results
    let map = map.lock().unwrap();
    for (key, value) in map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}

fn dashmap() {
    let map = Arc::new(DashMap::new());

    let worker_count = 10;
    let worker_handlers = (0..worker_count)
        .map(|id| {
            let map_cln = map.clone();
            thread::spawn(move || {
                map_cln.insert(id, id * id);
            })
        })
        .collect::<Vec<_>>();

    for handle in worker_handlers {
        handle.join().unwrap();
    }

    // Check results
    for entry in map.iter() {
        println!("Key: {}, Value: {}", entry.key(), entry.value());
    }
}
