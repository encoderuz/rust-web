use crate::to_do::enums::{TaskStatus, TellMe};

pub mod to_do;

fn main() {
    println!("{}", TaskStatus::DONE);
    println!("{}", TaskStatus::PENDING);
    let outcome = TaskStatus::DONE.to_string();
    println!("{}", outcome);

}
