use crate::to_do::enums::UserStatus;
use crate::to_do::traits::create::Create;
use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;
use crate::to_do::users::base::Base;

pub struct Active {
    pub super_struct: Base,
}
impl Get for Active{}
impl Edit for Active{}
impl Delete for Active{}
impl Active {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            name: input_title.to_string(),
            status: UserStatus::ACTIVE,
        };
        Active { super_struct: base }
    }
}