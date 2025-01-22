use crate::to_do::traits::create::Create;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use super::base::Base;
use super::super::enums::TaskStatus;

pub struct Pending {
    pub super_struct: Base,
}
impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        Pending { super_struct: base }
    }
}