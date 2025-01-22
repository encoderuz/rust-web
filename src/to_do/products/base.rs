use crate::to_do::enums::ProductStatus;

#[derive(Debug)]
pub struct Base{
    pub name: String,
    pub price: f64,
    pub quantity: i32,
    pub status: ProductStatus
}