use crate::Printable;

pub struct StructC {
    pub data: i32,
}

impl Printable for StructC {
    fn print(&self) {
        println!("Printing StructC: {}", self.data);
    }
}

