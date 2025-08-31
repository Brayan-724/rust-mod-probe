pub mod attribute;
pub mod constant_pool;

use serde::ser::SerializeTuple;
use serde::Serialize;

use attribute::AttributeInfo;
use constant_pool::{ConstantPool, ConstantPoolIndex};

pub struct ClassDef {}

pub struct ClassFile {
    constant_pool: ConstantPool,
    access_flags: AccessFlags,
    this_class: ConstantPoolIndex,
    super_class: ConstantPoolIndex,
    interfaces: Vec<ConstantPoolIndex>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

impl Serialize for ClassFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_tuple(0)?;

        let magic = 0xCAFEBABEu32;
        seq.serialize_element(&magic)?;

        let version = (65, 0);
        seq.serialize_element(&version.0)?;
        seq.serialize_element(&version.1)?;

        seq.serialize_element(&self.constant_pool)?;
        seq.serialize_element(&self.access_flags.0)?;
        seq.serialize_element(&self.this_class)?;
        seq.serialize_element(&self.super_class)?;
        seq.serialize_element(&self.interfaces)?;
        seq.serialize_element(&self.fields)?;
        seq.serialize_element(&self.methods)?;
        seq.serialize_element(&self.attributes)?;

        seq.end()
    }
}

#[derive(Serialize)]
pub struct FieldInfo {
    access: AccessFlags,
    /// Index must be a CONSTANT_Utf8_info structure which represents a valid **unqualified name denoting a field**
    name: ConstantPoolIndex,
    /// Index must be a CONSTANT_Utf8_info structure which represents a valid **field descriptor**
    descriptor: ConstantPoolIndex,
    attributes: Vec<AttributeInfo>,
}

#[derive(Serialize)]
pub struct MethodInfo {
    access: AccessFlags,
    name: ConstantPoolIndex,
    descriptor: ConstantPoolIndex,
    attributes: Vec<AttributeInfo>,
}

bitflags::bitflags! {
    #[derive(Serialize)]
    pub struct AccessFlags: u16 {
        const PUBLIC       = 0x0001;
        const PRIVATE      = 0x0002;
        const PROTECTED    = 0x0004;
        const STATIC       = 0x0008;

        const FINAL        = 0x0010;
        const OPEN         = 0x0020;
        const SUPER        = 0x0020;
        const SYNCRONIZED  = 0x0020;
        const BRIDGE       = 0x0040;
        const STATIC_PHASE = 0x0040;
        const VOLATILE     = 0x0040;
        const TRANSIENT    = 0x0080;
        const VARARGS      = 0x0080;

        const NATIVE       = 0x0100;
        const INTERFACE    = 0x0200;
        const ABSTRACT     = 0x0400;
        const STRINCT      = 0x0800;

        const SYNTHETIC    = 0x1000;
        const ANNOTATION   = 0x2000;
        const ENUM         = 0x4000;
        const MANDATED     = 0x8000;
        const MODULE       = 0x8000;
    }
}
