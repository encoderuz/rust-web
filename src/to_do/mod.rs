use crate::to_do::enums::{PostStatus, ProductStatus, TaskStatus, UserStatus};
use crate::to_do::products::available::Available;
use crate::to_do::products::out_of_stock::OutOfStock;
use crate::to_do::structs::done::Done;
use crate::to_do::structs::pending::Pending;
use crate::to_do::users::active::Active;
use crate::to_do::users::inactive::Inactive;

pub mod structs;
pub mod enums;
pub mod users;
mod posts;
mod products;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}
pub enum UserTypes {
    Active(Active),
    Inactive(Inactive),
}
pub enum PostTypes {
    Draft(posts::draft::Draft),
    Published(posts::published::Published),
}
pub enum ProductStatusTypes {
    Available(Available),
    OutOfStock(OutOfStock),
}
pub fn product_factory(name: &str, price: f64, quantity: i32, status: ProductStatus) -> ProductStatusTypes {
    match status {
        ProductStatus::AVAILABLE => {
            ProductStatusTypes::Available(Available::new(name, price, quantity))
        }
        ProductStatus::OUT_OF_STOCK => {
            ProductStatusTypes::OutOfStock(OutOfStock::new(name, price, quantity))
        }
    }
}
pub fn post_factory(title: &str, description: &str, status: PostStatus) -> PostTypes {
    match status {
        PostStatus::DRAFT => {
            PostTypes::Draft(posts::draft::Draft::new(title, description))
        }
        PostStatus::PUBLISHED => {
            PostTypes::Published(posts::published::Published::new(title, description))
        }
    }
}
pub fn user_type_factory(name: &str,
                         status: UserStatus) -> UserTypes {
    match status {
        UserStatus::ACTIVE => {
            UserTypes::Active(Active::new(name))
        }
        UserStatus::INACTIVE => {
            UserTypes::Inactive(Inactive::new(name))
        }
    }
}
pub fn to_do_factory(title: &str,
                     status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        }
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        }
    }
}