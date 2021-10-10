use crate::value::{Value, ValueType};


pub trait  Obj: ObjClone  {
    fn get_type(&self) -> &ObjType;
}

pub trait ObjClone {
    fn clone_box(&self) -> Box<dyn Obj>;
}

impl<T> ObjClone for T where T : 'static + Obj + Clone   {
    fn clone_box(&self) -> Box<dyn Obj> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Obj> {
    fn clone(&self) -> Box<dyn Obj> {
        self.clone_box()
    }
}

#[derive(Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ObjType {
    OBJ_STRING
}

// get object type
macro_rules! obj_type {
    ($x: expr) => {
        {
            *as_obj!($x).get_type()
        }
    };
}

//is string type
macro_rules! is_str {
    ($value: expr) => {
        {
            is_obj_type(&$value, &ObjType::OBJ_STRING)
        }
    };
}

pub fn is_obj_type(value: &Value, type_: &ObjType) -> bool {
    is_obj!(*value) && *as_obj!(*value).get_type() == *type_
}

//convert value to string
macro_rules! as_str {
    ($value: expr) => {
        {
            *as_obj!(*value) as ObjString
        }
    };
}

#[derive(Clone)]
pub struct ObjString {
    obj: ObjType,
    string: String
}


impl From<String> for ObjString {
    fn from(string: String) -> Self {
        ObjString {
            obj: ObjType::OBJ_STRING,
            string
        }
    }
}

impl Obj for ObjString {
    fn get_type(&self) -> &ObjType {
        &self.obj
    }
}
