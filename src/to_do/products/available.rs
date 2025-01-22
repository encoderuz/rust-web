use crate::to_do::products::base::Base;

pub struct Available{
    pub super_struct: Base
}
impl Available{
    pub fn new(name: &str, price: f64, quantity: i32) -> Self{
        let base = Base{
            name: name.to_string(),
            price,
            status: crate::to_do::enums::ProductStatus::AVAILABLE,
            quantity,
        };
        Available{super_struct: base}
    }
}