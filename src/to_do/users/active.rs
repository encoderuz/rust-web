use crate::to_do::enums::UserStatus;
use crate::to_do::users::base::Base;

pub struct Active {
    pub super_struct: Base,
}
impl Active {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: UserStatus::ACTIVE,
        };
        Active { super_struct: base }
    }
}