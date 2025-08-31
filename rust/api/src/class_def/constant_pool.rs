use serde::ser::{SerializeSeq as _, SerializeTuple as _};
use serde::Serialize;

#[derive(Serialize)]
#[repr(transparent)]
pub struct ConstantPoolIndex(pub u16);

#[repr(transparent)]
pub struct ConstantPool(pub Vec<ConstantPoolInfo>);

impl Serialize for ConstantPool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len() - 1))?;

        for item in &self.0 {
            seq.serialize_element(item)?;
        }

        seq.end()
    }
}

pub enum ConstantPoolInfo {
    Utf8(Vec<u8>),
    Integer(u32),
    Float(u32),
    /// SEE https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5
    Long(u32, u32),
    /// SEE https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.5
    Double(u32, u32),
    /// [ConstantPoolInfo::Utf8] representing the class name
    Class(ConstantPoolIndex),
    /// [ConstantPoolInfo::Utf8] representing the **sequence of Unicode code points to which the String object is to be initialized**
    String(ConstantPoolIndex),
    Fieldref {
        /// [ConstantPoolInfo::Class]
        class: ConstantPoolIndex,
        /// [ConstantPoolInfo::NameAndType]
        name_and_type: ConstantPoolIndex,
    },
    Methodref {
        /// [ConstantPoolInfo::Class]
        class: ConstantPoolIndex,
        /// [ConstantPoolInfo::NameAndType]
        name_and_type: ConstantPoolIndex,
    },
    InterfaceMethodref {
        /// [ConstantPoolInfo::Class]
        class: ConstantPoolIndex,
        /// [ConstantPoolInfo::NameAndType]
        name_and_type: ConstantPoolIndex,
    },
    NameAndType {
        /// [ConstantPoolInfo::Utf8]
        name: ConstantPoolIndex,
        /// [ConstantPoolInfo::Utf8]
        descriptor: ConstantPoolIndex,
    },
    MethodHandle {
        // TODO: SEE https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-5.html#jvms-5.4.3.5
        ref_kind: u8,
        /// SEE https://docs.oracle.com/javase/specs/jvms/se21/html/jvms-4.html#jvms-4.4.8
        ref_: ConstantPoolIndex,
    },
    /// [ConstantPoolInfo::Utf8] representing a method descriptor
    MethodType(ConstantPoolIndex),
    Dynamic {
        /// Bootstrap_methods array of the bootstrap method table of this class file
        bootstrap_method_attr: ConstantPoolIndex,
        /// Indicated descriptor must be a field descriptor
        name_and_type_index: ConstantPoolIndex,
    },
    InvokeDynamic {
        /// Bootstrap_methods array of the bootstrap method table of this class file
        bootstrap_method_attr: ConstantPoolIndex,
        /// Indicated descriptor must be a method descriptor
        name_and_type_index: ConstantPoolIndex,
    },
    /// [ConstantPoolInfo::Utf8] representing a valid module name
    Module(ConstantPoolIndex),
    /// [ConstantPoolInfo::Utf8] representing a valid package name encoded in internal form
    Package(ConstantPoolIndex),
}

impl Serialize for ConstantPoolInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tag: u8 = match self {
            ConstantPoolInfo::Utf8(..) => 1,
            ConstantPoolInfo::Integer(..) => 3,
            ConstantPoolInfo::Float(..) => 4,
            ConstantPoolInfo::Long(..) => 5,
            ConstantPoolInfo::Double(..) => 6,
            ConstantPoolInfo::Class(..) => 7,
            ConstantPoolInfo::String(..) => 8,
            ConstantPoolInfo::Fieldref { .. } => 9,
            ConstantPoolInfo::Methodref { .. } => 10,
            ConstantPoolInfo::InterfaceMethodref { .. } => 11,
            ConstantPoolInfo::NameAndType { .. } => 12,
            ConstantPoolInfo::MethodHandle { .. } => 15,
            ConstantPoolInfo::MethodType(..) => 16,
            ConstantPoolInfo::Dynamic { .. } => 17,
            ConstantPoolInfo::InvokeDynamic { .. } => 18,
            ConstantPoolInfo::Module(..) => 19,
            ConstantPoolInfo::Package(..) => 20,
        };

        let mut seq = serializer.serialize_tuple(0)?;
        seq.serialize_element(&tag)?;

        match self {
            ConstantPoolInfo::Utf8(items) => {
                seq.serialize_element(&items)?;
            }
            // Numbers
            ConstantPoolInfo::Integer(n) | ConstantPoolInfo::Float(n) => {
                seq.serialize_element(n)?;
            }
            ConstantPoolInfo::Long(h, l) | ConstantPoolInfo::Double(h, l) => {
                seq.serialize_element(h)?;
                seq.serialize_element(l)?;
            }
            // Indexes
            ConstantPoolInfo::Class(idx)
            | ConstantPoolInfo::String(idx)
            | ConstantPoolInfo::MethodType(idx)
            | ConstantPoolInfo::Module(idx)
            | ConstantPoolInfo::Package(idx) => {
                seq.serialize_element(idx)?;
            }

            // Class + NameAndType
            ConstantPoolInfo::Fieldref {
                class,
                name_and_type,
            }
            | ConstantPoolInfo::Methodref {
                class,
                name_and_type,
            }
            | ConstantPoolInfo::InterfaceMethodref {
                class,
                name_and_type,
            } => {
                seq.serialize_element(class)?;
                seq.serialize_element(name_and_type)?;
            }

            ConstantPoolInfo::NameAndType { name, descriptor } => {
                seq.serialize_element(&name)?;
                seq.serialize_element(&descriptor)?;
            }
            ConstantPoolInfo::MethodHandle { ref_kind, ref_ } => {
                seq.serialize_element(&ref_kind)?;
                seq.serialize_element(&ref_)?;
            }
            ConstantPoolInfo::Dynamic {
                bootstrap_method_attr,
                name_and_type_index,
            }
            | ConstantPoolInfo::InvokeDynamic {
                bootstrap_method_attr,
                name_and_type_index,
            } => {
                seq.serialize_element(&bootstrap_method_attr)?;
                seq.serialize_element(&name_and_type_index)?;
            }
        }

        seq.end()
    }
}
