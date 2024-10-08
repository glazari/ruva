use super::attributes::Attribute;
use super::attributes::Attributes;
use super::attributes::CodeAttribute;
use super::constant_pool::ConstantPool;
use super::file_reader::FileReader;
use anyhow::Result;

#[derive(Debug)]
pub struct Methods {
    pub methods: Vec<Method>,
}

#[derive(Debug)]
pub struct Method {
    pub access_flags: AccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Attributes,
}

#[derive(Debug)]
pub struct AccessFlags(u16);

#[derive(Debug, PartialEq)]
pub enum AccessFlag {
    Public,
    Private,
    Protected,
    Static,
    Final,
    Synchronized,
    Bridge,
    Varargs,
    Native,
    Abstract,
    Strict,
    Synthetic,
}

impl Methods {
    pub fn from(file: &mut FileReader, cp: &ConstantPool) -> Result<Methods> {
        let mut methods = Vec::new();

        let methods_count = file.read_u2_to_u16()?;
        for _i in 0..methods_count {
            let access_flags = AccessFlags::new(file.read_u2_to_u16()?);
            let name_index = file.read_u2_to_u16()?;
            let descriptor_index = file.read_u2_to_u16()?;
            let attributes = Attributes::from(file, cp)?;

            methods.push(Method {
                access_flags,
                name_index,
                descriptor_index,
                attributes,
            });
        }

        Ok(Methods { methods })
    }

    pub fn to_string(&self, cp: &ConstantPool) -> String {
        let mut s = String::new();
        s.push_str("Methods:\n");
        for method in &self.methods {
            s.push_str(&format!(
                "\n\n- access_flags: {:?}\n",
                method.access_flags.flag_vector()
            ));
            s.push_str(&format!(
                "- name: {}\n",
                cp.get_to_string(method.name_index)
            ));
            s.push_str(&format!(
                "- descriptor: {}\n",
                cp.get_to_string(method.descriptor_index)
            ));
            //s.push_str(&format!("{:?}", &method.attributes));
            s.push_str(&format!("{}\n", method.attributes.to_string(cp)));
        }
        s
    }
}

impl Method {
    pub fn get_code(&self) -> Option<&CodeAttribute> {
        for attribute in &self.attributes.attributes {
            if let Attribute::Code(code) = attribute {
                return Some(code);
            }
        }
        None
    }
}

impl AccessFlags {
    pub fn new(flags: u16) -> AccessFlags {
        AccessFlags(flags)
    }

    pub fn flag_vector(&self) -> Vec<AccessFlag> {
        let mut flags = Vec::new();
        add_flag(&mut flags, self.0, 0x0001, AccessFlag::Public);
        add_flag(&mut flags, self.0, 0x0002, AccessFlag::Private);
        add_flag(&mut flags, self.0, 0x0004, AccessFlag::Protected);
        add_flag(&mut flags, self.0, 0x0008, AccessFlag::Static);
        add_flag(&mut flags, self.0, 0x0010, AccessFlag::Final);
        add_flag(&mut flags, self.0, 0x0020, AccessFlag::Synchronized);
        add_flag(&mut flags, self.0, 0x0040, AccessFlag::Bridge);
        add_flag(&mut flags, self.0, 0x0080, AccessFlag::Varargs);
        add_flag(&mut flags, self.0, 0x0100, AccessFlag::Native);
        add_flag(&mut flags, self.0, 0x0400, AccessFlag::Abstract);
        add_flag(&mut flags, self.0, 0x0800, AccessFlag::Strict);
        add_flag(&mut flags, self.0, 0x1000, AccessFlag::Synthetic);
        flags
    }
}

impl AccessFlag {
    pub fn to_str(&self) -> &'static str {
        match self {
            AccessFlag::Public => "public",
            AccessFlag::Private => "private",
            AccessFlag::Protected => "protected",
            AccessFlag::Static => "static",
            AccessFlag::Final => "final",
            AccessFlag::Synchronized => "synchronized",
            AccessFlag::Bridge => "bridge",
            AccessFlag::Varargs => "varargs",
            AccessFlag::Native => "native",
            AccessFlag::Abstract => "abstract",
            AccessFlag::Strict => "strict",
            AccessFlag::Synthetic => "synthetic",
        }
    }
}

fn add_flag(flags: &mut Vec<AccessFlag>, bit_flags: u16, bit_mask: u16, access_flag: AccessFlag) {
    if bit_flags & bit_mask == bit_mask {
        flags.push(access_flag);
    }
}
