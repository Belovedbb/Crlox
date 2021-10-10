#![macro_use]
use crate::object::Obj;

#[derive(PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ValueType {
    VAL_BOOLEAN,
    VAL_NIL,
    VAL_NUMBER,
    VAL_OBJ
}

#[derive(Clone)]
pub struct AsValue {
    pub boolean: Option<bool>,
    pub number: Option<f64>,
    pub obj: Option<Box<dyn Obj>>
}

impl AsValue {

    pub fn get_boolean_ref(&self) -> &Option<bool> {
        &self.boolean
    }
    pub fn get_number_ref(&self) -> &Option<f64> {
        &self.number
    }

    pub fn get_obj_ref(&self) -> &Option<Box<dyn Obj>> {
        &self.obj
    }
}

#[derive(Clone)]
pub struct Value {
    pub type_: ValueType,
    pub as_: AsValue
}

impl Value {
    pub fn get_type_ref(&self) -> &ValueType {
        &self.type_
    }

    pub fn get_as_ref(&self) -> &AsValue {
        &self.as_
    }
}

macro_rules! boolean_val {
    ($a: expr) => {
        {
            Value {
                type_: ValueType::VAL_BOOLEAN,
                as_: AsValue {
                    boolean: Some($a),
                    obj: None, 
                    number: None
                }
            }
        }
    };
}

macro_rules! nill {
    () => {
        {
            Value {
                type_: ValueType::VAL_NIL,
                as_: AsValue {
                    boolean: None,
                    obj: None,
                    number: None
                }
            }
        }
    };
}

macro_rules! number_val {
    ($a: expr) => {
        {
            Value {
                type_: ValueType::VAL_NUMBER,
                as_: AsValue {
                    boolean: None,
                    obj: None,
                    number: Some($a)
                }
            }
        }
    };
}

macro_rules! obj_val {
    ($a: expr) => {
        {
            Value {
                type_: ValueType::VAL_OBJ,
                as_: AsValue {
                    boolean: None,
                    number: None
                    obj: Some($a)
                }
            }
        }
    };
}

macro_rules! as_number {
    ($a: expr) => {
        {
            (*(*$a.get_as_ref()).get_number_ref()).unwrap()
        }
    };
}

macro_rules! as_boolean {
    ($a: expr) => {
        {
            (*(*$a.get_as_ref()).get_boolean_ref()).unwrap()
        }
    };
}

macro_rules! as_obj {
    ($a: expr) => {
        {
            (*(*$a.get_as_ref()).get_obj_ref()).clone().unwrap()
        }
    };
}

macro_rules! is_boolean {
    ($a: expr) => {
        {
            $a.type_ == ValueType::VAL_BOOLEAN
        }
    };
}

macro_rules! is_number {
    ($a: expr) => {
        {
            *$a.get_type_ref() == ValueType::VAL_NUMBER
        }
    };
}

macro_rules! is_nill {
    ($a: expr) => {
        {
            $a.type_ == ValueType::VAL_NIL
        }
    };
}

macro_rules! is_obj {
    ($a: expr) => {
        {
            $a.type_ == ValueType::VAL_OBJ
        }
    };
}

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
        match *value.get_type_ref() {
            ValueType::VAL_BOOLEAN =>
                if let true =  (*value.get_as_ref()).get_boolean_ref().unwrap() { 
                    println!("true")
                }else {
                    println!("false");
                },
            ValueType::VAL_NIL => println!("nill"),
            ValueType::VAL_NUMBER => println!("{}", as_number!(value)),
            _ => ()
        }
        
    }

}
