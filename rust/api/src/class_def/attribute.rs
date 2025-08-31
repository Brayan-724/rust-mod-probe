use serde::Serialize;

use crate::class_def::AccessFlags;

use super::constant_pool::ConstantPoolIndex;

#[derive(Serialize)]
pub struct Attribute {
    /// [super::ConstantPoolInfo::Utf8] representing the **name of the attribute**
    attribute_name: ConstantPoolIndex,
    info: AttributeInfo,
}

#[derive(Serialize)]
pub enum AttributeInfo {
    PermittedSubclasses {
        classes: Vec<u16>,
    },
    Record {
        components: Vec<RecordComponent>,
    },
    NestMembers {
        classes: Vec<u16>,
    },
    NestHost {
        host_class: ConstantPoolIndex,
    },
    ModuleMainClass {
        main_class: ConstantPoolIndex,
    },
    ModulePackages {
        package_index: Vec<u16>,
    },
    Module {
        module_name: ConstantPoolIndex,
        module_flags: AccessFlags,
        module_version: ConstantPoolIndex,
        requires: Vec<Requires>,
        exports: Vec<Exports>,
        opens: Vec<Opens>,
        uses_index: Vec<u16>,
        provides: Vec<Provides>,
    },
    MethodParameters {
        parameters: Vec<Parameters>,
    },
    BootstrapMethods {
        bootstrap_methods: Vec<BootstrapMethods>,
    },
    // NOTE: Annotations will be ignored for now
    //
    // AnnotationDefault {
    //     default_value: element_value,
    // },
    //
    // RuntimeInvisibleTypeAnnotations {
    //     annotations: Vec<type_annotation>,
    // },
    // RuntimeVisibleTypeAnnotations {
    //     annotations: Vec<type_annotation>,
    // },
    // RuntimeInvisibleParameterAnnotations {
    //     parameter_annotations: Vec<ParameterAnnotations>,
    // },
    // RuntimeVisibleParameterAnnotations {
    //     parameter_annotations: Vec<ParameterAnnotations>,
    // },
    // RuntimeInvisibleAnnotations {
    //     annotations: Vec<Annotation>,
    // },
    // RuntimeVisibleAnnotations {
    //     annotations: Vec<Annotation>,
    // },
    Deprecated,
    LocalVariableTypeTable {
        local_variable_type_table: Vec<LocalVariableType>,
    },
    LocalVariableTable {
        local_variable_table: Vec<LocalVariable>,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumber>,
    },
    SourceDebugExtension {
        debug_extension: Vec<u8>,
    },
    SourceFile {
        sourcefile: ConstantPoolIndex,
    },
    Signature {
        signature: ConstantPoolIndex,
    },
    Synthetic,
    EnclosingMethod {
        class: ConstantPoolIndex,
        method: ConstantPoolIndex,
    },
    InnerClasses {
        classes: Vec<Classes>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    // StackMapTable {
    //     entries: Vec<stack_map_frame>,
    // },
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<Exception>,
        attributes: Vec<AttributeInfo>,
    },
    ConstantValue {
        constantvalue: ConstantPoolIndex,
    },
}

// NOTE: Annotations will be ignored for now
//
// #[derive(Serialize)]
// pub struct Annotation {
//     type_index: ConstantPoolIndex,
//     element_value_pairs: Vec<ElementValuePair>,
// }
//
// #[derive(Serialize)]
// pub struct ElementValuePair {
//     element_name_index: ConstantPoolIndex,
//     value: element_value,
// }
//
// #[derive(Serialize)]
// pub struct ParameterAnnotations {
//     pub annotations: Vec<Annotation>,
// }

#[derive(Serialize)]
pub struct RecordComponent {
    name: ConstantPoolIndex,
    descriptor: ConstantPoolIndex,
    attributes: Vec<AttributeInfo>,
}

#[derive(Serialize)]
pub struct Requires {
    pub module: ConstantPoolIndex,
    pub flags: AccessFlags,
    pub version: ConstantPoolIndex,
}

#[derive(Serialize)]
pub struct Exports {
    pub package: ConstantPoolIndex,
    pub flags: AccessFlags,
    pub to: Vec<ConstantPoolIndex>,
}

#[derive(Serialize)]
pub struct Opens {
    pub package: ConstantPoolIndex,
    pub flags: AccessFlags,
    pub to: Vec<ConstantPoolIndex>,
}

#[derive(Serialize)]
pub struct Provides {
    pub class: ConstantPoolIndex,
    pub with: Vec<ConstantPoolIndex>,
}

#[derive(Serialize)]
pub struct Parameters {
    pub name: ConstantPoolIndex,
    pub access_flags: AccessFlags,
}

#[derive(Serialize)]
pub struct BootstrapMethods {
    pub bootstrap_method_ref: ConstantPoolIndex,
    pub bootstrap_arguments: Vec<ConstantPoolIndex>,
}

#[derive(Serialize)]
pub struct LocalVariableType {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantPoolIndex,
    pub signature: ConstantPoolIndex,
    pub index: u16,
}

#[derive(Serialize)]
pub struct LocalVariable {
    pub start_pc: u16,
    pub length: u16,
    pub name: ConstantPoolIndex,
    pub descriptor: ConstantPoolIndex,
    pub index: u16,
}

#[derive(Serialize)]
pub struct LineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Serialize)]
pub struct Classes {
    pub inner_class_info: ConstantPoolIndex,
    pub outer_class_info: ConstantPoolIndex,
    pub inner_name: ConstantPoolIndex,
    pub inner_class_access_flags: AccessFlags,
}

#[derive(Serialize)]
pub struct Exception {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: ConstantPoolIndex,
}
