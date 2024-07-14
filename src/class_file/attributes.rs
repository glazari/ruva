use super::code_attribute::CodeAttribute;
use super::constant_pool::ConstantPool;
use super::file_reader::FileReader;
use anyhow::Result;

#[derive(Debug)]
pub struct Attributes {
    attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub enum Attribute {
    Code(CodeAttribute),
    GenericAttribute(GenericAttribute),
}

#[derive(Debug)]
pub struct AttStart {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

#[derive(Debug)]
pub struct GenericAttribute {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl Attributes {
    pub fn from(file: &mut FileReader, cp: &ConstantPool) -> Result<Attributes> {
        let mut attributes = Vec::new();

        let attributes_count = file.read_u2_to_u16()?;
        for _i in 0..attributes_count {
            let attribute_name_index = file.read_u2_to_u16()?;
            let attribute_length = file.read_u4_to_u32()?;

            let att_start = AttStart {
                attribute_name_index,
                attribute_length,
            };

            let name = cp.get_to_string(attribute_name_index);
            match name.as_str() {
                "Code" => {
                    let att = CodeAttribute::parse(file, &att_start, cp)?;
                    attributes.push(Attribute::Code(att));
                }
                _ => {
                    let att = GenericAttribute::parse(file, &att_start)?;
                    attributes.push(Attribute::GenericAttribute(att));
                }
            }
        }

        Ok(Attributes { attributes })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();

        s.push_str("Attributes\n");
        for att in &self.attributes {
            s.push_str(&att.to_string(cp));
        }
        s
    }
}

impl GenericAttribute {
    fn parse(file: &mut FileReader, att_start: &AttStart) -> Result<GenericAttribute> {
        let mut info = Vec::new();
        for _j in 0..att_start.attribute_length {
            info.push(file.read_u1()?);
        }
        Ok(GenericAttribute {
            attribute_name_index: att_start.attribute_name_index,
            attribute_length: att_start.attribute_length,
            info,
        })
    }
}

impl Attribute {
    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        match self {
            Attribute::GenericAttribute(att) => {
                s.push_str(&format!(
                    "name {}\n",
                    cp.get_to_string(att.attribute_name_index)
                ));
                s.push_str(&format!("length {}\n", att.attribute_length));
            }
            Attribute::Code(att) => {
                s.push_str(&att.to_string(cp));
            }
        }
        s
    }
}
