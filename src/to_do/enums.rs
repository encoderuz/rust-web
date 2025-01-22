use std::fmt;
pub enum TaskStatus{
    DONE,
    PENDING
}
#[derive(Debug)]
pub enum UserStatus {
    ACTIVE,
    INACTIVE
}
#[derive(Debug)]
pub enum PostStatus{
    PUBLISHED,
    DRAFT
}
#[derive(Debug)]
pub enum ProductStatus{
    AVAILABLE,
    OUT_OF_STOCK
}
impl UserStatus{
    pub fn to_string(&self) -> String{
        match self{
            &Self::ACTIVE => "ACTIVE".to_string(),
            &Self::INACTIVE => "INACTIVE".to_string()
        }
    }
}
impl fmt::Display for UserStatus{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            UserStatus::ACTIVE => {write!(f, "ACTIVE")}
            UserStatus::INACTIVE => {write!(f, "INACTIVE")}
        }
    }
}
impl TaskStatus{
    pub fn to_string(&self) -> String{
        match self{
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string()
        }
    }
}
impl fmt::Display for TaskStatus{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TaskStatus::DONE => {write!(f, "DONE")}
            TaskStatus::PENDING => {write!(f, "PENDING")}
        }
    }
}