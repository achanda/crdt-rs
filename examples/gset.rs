extern crate crdt;

use crdt::GSet;

fn main() {
    let mut a: GSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    a.insert(4);
    println!("{:?}", a);
}
