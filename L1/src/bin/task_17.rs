use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

struct AtomicCounter {
    count: AtomicUsize,
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self {
            count: AtomicUsize::new(0),
        }
    }
}

impl AtomicCounter {
    fn new() -> Self {
        AtomicCounter::default()
    }

    fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    fn get_count(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

fn main() {
    let atom_cnt = Arc::new(AtomicCounter::new());

    let worker_count = 5;
    let worker_handlers = (0..worker_count)
        .map(|_| {
            let atom_cnt_cln = atom_cnt.clone();
            thread::spawn(move || {
                for _ in 0..42 {
                    atom_cnt_cln.increment();
                }
            })
        })
        .collect::<Vec<_>>();

    for handle in worker_handlers {
        handle.join().unwrap();
    }

    assert_eq!(atom_cnt.get_count(), 42 * 5);
}
