use mopa::{Any, mopafy};


pub trait  Obj: ObjClone + Any  {
    fn get_type(&self) -> &ObjType;
}

mopafy!(Obj);

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

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
#[allow(non_camel_case_types)]
pub enum ObjType {
    OBJ_STRING
}

#[derive(Clone, Hash, PartialEq, Eq)]
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

impl ObjString {
    pub fn get_string(&self) -> &str {
        &self.string
    }
}
