use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use crate::to_do::users::base::Base;

pub struct Inactive {
    pub super_struct: Base,
}
impl Get for Inactive{}
impl Edit for Inactive{}
impl Delete for Inactive{}
impl Inactive {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            name: input_title.to_string(),
            status: crate::to_do::enums::UserStatus::INACTIVE,
        };
        Inactive { super_struct: base }
    }
}