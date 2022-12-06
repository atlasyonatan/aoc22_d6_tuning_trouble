use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
mod sliding_window;
use sliding_window::SlidingWindow;

fn main() {
    let path = Path::new("../input.txt");
    let sizes = [4, 14];
    for (i, size) in sizes.iter().enumerate() {
        let file = File::open(path).unwrap();
        let items = io::BufReader::new(file)
            .bytes()
            .map(|r| r.unwrap())
            .enumerate();
        let mut sw = SlidingWindow::with_capacity(*size);
        'items: for item in items {
            sw.push(item);
            if sw.is_full() {
                let mut set: HashSet<u8> = HashSet::new();
                for (_, byte) in sw.items() {
                    if set.contains(byte) {
                        continue 'items;
                    }
                    set.insert(*byte);
                }
                break 'items;
            }
        }
        println!("part {}: {}", i + 1, sw.get(size - 1).unwrap().0 + 1)
    }
}
