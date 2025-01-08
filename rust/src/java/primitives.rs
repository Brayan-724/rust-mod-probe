use crate::new_class;

use super::JSignature;

macro_rules! primitive {
    ($class:ident, $sig:literal, $sig_type:literal) => {
        impl $crate::java::JSignature for $class {
            fn sig() -> String {
                $sig_type.to_owned()
            }

            fn sig_type() -> String {
                $sig.to_owned()
            }
        }
    };
}

// Z 	boolean
primitive! { bool, "Z", "java/lang/Boolean" }
// B 	byte
primitive! { i8, "B", "java/lang/Byte" }
// C 	char
primitive! { char, "C", "java/lang/Character" }
// S 	short
primitive! { i16, "S", "java/lang/Short" }
// I 	int
primitive! { i32, "I", "java/lang/Integer" }
// J 	long
primitive! { i64, "J", "java/lang/Long" }
// F 	float
primitive! { f32, "F", "java/lang/Float" }
// D 	double
primitive! { f64, "D", "java/lang/Double" }

impl JSignature for String {
    fn sig() -> String {
        "java/lang/String".to_owned()
    }
}

new_class! {Function: "java/util/function/Function" {}}
