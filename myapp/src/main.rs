#![deny(clippy::all)]

use intutils::subtraction::sub;
use intutils::addition::add;


fn main() {
    let added = add(1, 2);
    let subtracted = sub(1, 2);
    println!("Added is {added} and subtracted is {subtracted}");
}