#[derive(Debug, PartialEq)]
pub struct Simulation {
    pub name: String,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}
