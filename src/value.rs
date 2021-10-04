
pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>
}

impl ValueArray {

    pub fn init_value() -> Self {
        ValueArray {
            values: Vec::new()
        }
    }

    pub fn write_value(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn get_values(&self) -> &Vec<Value> {
        &self.values
    }

    pub fn print_value(value: &Value) {
        println!("{}", value);
    }

}
