use crate::to_do::posts::base::Base;

pub struct Published {
    pub super_struct: Base,
}
impl Published {
    pub fn new(title: &str, description: &str) -> Self {
        let base = Base {
            title: title.to_string(),
            description: description.to_string(),
            status: crate::to_do::enums::PostStatus::PUBLISHED,
        };
        Published { super_struct: base }
    }
}