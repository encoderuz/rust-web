use crate::to_do::enums::ProductStatus;
use crate::to_do::products::base::Base;

pub struct OutOfStock {
    pub super_struct: Base,
}

impl OutOfStock{
    pub fn new(name: &str, price: f64, quantity: i32) -> Self {
        let base = Base {
            name: name.to_string(),
            price,
            quantity,
            status: ProductStatus::OUT_OF_STOCK,
        };
        OutOfStock { super_struct: base }
    }
}