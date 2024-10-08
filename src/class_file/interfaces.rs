use super::constant_pool::ConstantPool;
use super::file_reader::FileReader;
use anyhow::Result;

use crate::print_debug as p;

pub struct Interfaces {
    pub interfaces: Vec<u16>,
}

impl Interfaces {
    pub fn from(file: &mut FileReader) -> Result<Interfaces> {
        let mut interfaces = Vec::new();

        let interfaces_count = file.read_u2_to_u16()?;
        for _i in 0..interfaces_count {
            interfaces.push(file.read_u2_to_u16()?);
        }
        p!("interfaces: {:?}", interfaces);

        Ok(Interfaces { interfaces })
    }

    pub fn to_string(&self, constant_pool: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("Interfaces:\n");
        for i in &self.interfaces {
            s.push_str(&format!("- {}\n", constant_pool.get_to_string(*i)));
        }
        s
    }
}
