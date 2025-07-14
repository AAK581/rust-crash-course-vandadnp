fn concatenate_strings(val1: &String, val2: &String) {
    println!("{val1} {val2}")
}

fn main() {
    let part1 = String::from("Sekaaaaaai");
    let part2 = String::from("Deee");
    // Borrowing the strings for this function via referencing instead of moving them there
    concatenate_strings(&part1, &part2);
    // Wouldn't work without borrowing
    println!("{part1}");
    println!("{part2}");
}