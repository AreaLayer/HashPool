use crate::types::types::*;
use crate::types::traits::*;
use crate::types::Data:;

fn main Data {
    let x = 5;
    let y = 6;
    let z = x + y;
    println!("Hello, world! {} + {} = {}", x, y, z);

}

impl Data {
    fn add(self, other: Self) -> Self {
        self + other
    }
}

fn main() {
    let x = 5;
    let y = 6;
    let z = x.add(y);
    println!("Hello, world! {} + {} = {}", x, y, z);
}
