use crate::to_do::enums::PostStatus;

pub struct Base{
    pub title: String,
    pub description: String,
    pub status: PostStatus
}