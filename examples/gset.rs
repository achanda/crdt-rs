extern crate crdt;

use crdt::GSet;

fn main() {
    let mut a: GSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    a.insert(4);
    println!("{:?}", a);

    let b: GSet<i32> = vec!(3i32, 4, 5, 6).into_iter().collect();
    let intersection: GSet<i32> = a.intersection(&b).into_iter().collect();
    println!("{:?}", intersection);
}
