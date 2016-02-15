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

    // Example of union
    let mut b = PSet::new();
    b.insert(5);
    b.insert(6);
    b.insert(7);
    b.remove(7);
    println!("Set B contains: {:?}", b.contents());
    println!("Union: {:?}", a.union(&b).contents());

    // Example of intersection
    println!("Intersection: {:?}", a.intersection(&b).contents());
}
