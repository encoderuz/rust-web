use crate::to_do::enums::{ProductStatus, TaskStatus, UserStatus};
use crate::to_do::{product_factory, to_do_factory, user_type_factory, ItemTypes, ProductStatusTypes, UserTypes};
use crate::to_do::enums::ProductStatus::AVAILABLE;
use crate::to_do::traits::get::Get;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
pub mod to_do;
mod state;

fn main() {
    let to_do_items = to_do_factory("washing",
                                    TaskStatus::DONE);
    match to_do_items {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        }
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }
    let users = user_type_factory("John Doe", UserStatus::ACTIVE);
    
    match users {
        UserTypes::Active(item) => {
            item.get(&item.super_struct.name);
            item.delete(&item.super_struct.name);
        }
        UserTypes::Inactive(item) => {
            item.get(&item.super_struct.name);
            item.set_to_active(&item.super_struct.name);
        }
    }
}
