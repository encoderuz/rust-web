use crate::to_do::enums::{ProductStatus, TaskStatus, UserStatus};
use crate::to_do::{product_factory, to_do_factory, user_type_factory, ItemTypes, ProductStatusTypes, UserTypes};
use crate::to_do::enums::ProductStatus::AVAILABLE;

pub mod to_do;

fn main() {
    let to_do_item = to_do_factory("Buy milk", TaskStatus::DONE);

    match to_do_item {
        ItemTypes::Pending(item) => {
            println!("Item status: {}", item.super_struct.status);
            println!("Item title: {}", item.super_struct.title);
        }
        ItemTypes::Done(item) => {
            println!("Item status: {}", item.super_struct.status);
            println!("Item title: {}", item.super_struct.title);
        }
    }

    let user_items = user_type_factory("Abduqodir", UserStatus::ACTIVE);
    
    match user_items {
        UserTypes::Active(item) => {
            println!("User status: {}", item.super_struct.status);
            println!("User name: {}", item.super_struct.name);
        }
        UserTypes::Inactive(item) => {
            println!("User status: {}", item.super_struct.status);
            println!("User name: {}", item.super_struct.name);
        }
    }

    let product_item = product_factory("Milk", 1.5, 10, AVAILABLE);
    match product_item {
        ProductStatusTypes::Available(item) => {
            println!("Product status: {:?}", item.super_struct.status);
            println!("Product name: {}", item.super_struct.name);
            println!("Product price: {}", item.super_struct.price);
            println!("Product quantity: {}", item.super_struct.quantity);
        }
        ProductStatusTypes::OutOfStock(item) => {
            println!("Product status: {:?}", item.super_struct.status);
            println!("Product name: {}", item.super_struct.name);
            println!("Product price: {}", item.super_struct.price);
            println!("Product quantity: {}", item.super_struct.quantity);
        }
    }
}
