use crate::to_do::users::base::Base;

pub struct Inactive {
    pub super_struct: Base,
}

impl Inactive {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            name: input_title.to_string(),
            status: crate::to_do::enums::UserStatus::INACTIVE,
        };
        Inactive { super_struct: base }
    }
}