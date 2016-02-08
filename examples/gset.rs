extern crate crdt_rs as crdt;

use crdt::GSet;

fn main() {
    let mut a: GSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    a.insert(4);
    println!("Set A: {:?}", a);

    let b: GSet<i32> = vec!(3i32, 4, 5, 6).into_iter().collect();

    // Intersection example
    let intersection: GSet<i32> = a.intersection(&b).into_iter().collect();
    println!("Set B: {:?}", b);
    println!("Intersection: {:?}", intersection);

    // Difference example
    let difference: GSet<i32> = a.difference(&b).into_iter().collect();
    println!("Difference: {:?}", difference);

    // Union example
    let union: GSet<i32> = a.union(&b).into_iter().collect();
    println!("Union: {:?}", union);
}
