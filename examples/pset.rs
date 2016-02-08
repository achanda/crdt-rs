extern crate crdt_rs as crdt;

use crdt::PSet;

fn main() {
    let mut a = PSet::new();
    a.insert(4);
    a.insert(3);
    a.insert(5);
    a.remove(4);
    println!("Set A: {:?}", a);
    println!("Does it contain 4? {:?}", a.contains(&4));

    // Example of contents
    println!("Set A contains: {:?}", a.contents());
}
