use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

pub mod to_do;

fn main() {
    let done = Done::new("Done Task");
    let pending = Pending::new("Pending Task");

    println!("Done Task: {}", done.super_struct.title);
    println!("Pending Task: {}", pending.super_struct.title);
}
