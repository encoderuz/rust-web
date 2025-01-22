use crate::to_do::posts::base::Base;

pub struct Draft{
    pub super_struct: Base
}
impl Draft {
    pub fn new(title: &str, description: &str) -> Self {
        let base = Base {
            title: title.to_string(),
            description: description.to_string(),
            status: crate::to_do::enums::PostStatus::DRAFT,
        };
        Draft { super_struct: base }
    }
}