extern crate crdt;

use crdt::PSet;

fn main() {
    let mut a = PSet::new();
    a.insert(4);
    a.insert(5);
    a.remove(4);
    println!("{:?}", a.contains(&4));
}
