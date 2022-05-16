pub trait Product {
    fn convert(&self, s: String) -> String;
}

pub struct Factory;

impl Factory {
    pub fn convert<P, F>(&self, s: String, create_product: F) -> String 
        where P: Product, 
            F: FnOnce() -> P
    {
        create_product().convert(s)
    }
}

pub struct ConcreteProductX;
impl Product for ConcreteProductX {
    fn convert(&self, s: String) -> String {
        s.to_uppercase()
    }
}
