use std::ops;

use jni::{
    objects::{JClass as JniJClass, JValueOwned},
    JNIEnv,
};

use crate::{
    class::Instance,
    conversion::{FromJValue, IntoJValue},
    JSignature,
};

macro_rules! primitive {
    ($class:ident: $fn:ident, $sig:literal, $sig_type:literal, $type:ident) => {
        impl $crate::JSignature for $class {
            const CLASS: &'static str = $sig_type;

            fn sig() -> String {
                $sig.to_owned()
            }
        }

        impl $crate::conversion::IntoJValue for $class {
            fn into_jvalue<'local>(
                self,
                _: &mut ::jni::JNIEnv<'local>,
            ) -> ::jni::objects::JValueOwned<'local> {
                ::jni::objects::JValueOwned::$type(self.into())
            }
        }

        impl $crate::conversion::FromJValue for $class {
            fn from_jvalue<'local>(value: ::jni::objects::JValueOwned<'local>) -> Self {
                value.$fn().unwrap()
            }
        }
    };
}

// Z 	boolean
primitive!(bool: z, "Z", "java/lang/Boolean", Bool);
// B 	byte
primitive!(i8: b, "B", "java/lang/Byte", Byte);
// C 	char
primitive!(u16: c, "C", "java/lang/Character", Char);
// S 	short
primitive!(i16: s, "S", "java/lang/Short", Short);
// I 	int
primitive!(i32: i, "I", "java/lang/Integer", Int);
// J 	long
primitive!(i64: j, "J", "java/lang/Long", Long);
// F 	float
primitive!(f32: f, "F", "java/lang/Float", Float);
// D 	double
primitive!(f64: d, "D", "java/lang/Double", Double);

// Void

impl JSignature for () {
    const CLASS: &'static str = "";

    fn sig() -> String {
        "V".to_owned()
    }
}

// Char

impl JSignature for char {
    const CLASS: &'static str = "java/lang/Char";

    fn sig() -> String {
        "C".to_owned()
    }
}

impl IntoJValue for char {
    fn into_jvalue<'local>(self, _: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        JValueOwned::Char(self as u16)
    }
}

impl FromJValue for char {
    fn from_jvalue<'local>(value: JValueOwned<'local>) -> Self {
        char::from_u32(value.c().expect("Cannot get char") as u32).expect("Cannot cast to char")
    }
}

// String
impl JSignature for String {
    const CLASS: &'static str = "java/lang/String";
}

impl IntoJValue for String {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        env.new_string(self).unwrap().into()
    }
}

// Function
#[derive(Clone, Copy)]
pub struct Function(pub Instance);

impl JSignature for Function {
    const CLASS: &'static str = "java/util/function/Function";
}

impl From<Instance> for Function {
    fn from(value: Instance) -> Self {
        Function::from_instance(value)
    }
}

impl Into<Instance> for Function {
    fn into(self) -> Instance {
        self.0
    }
}

impl ops::Deref for Function {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Function {
    #[inline(always)]
    pub fn from_instance(instance: Instance) -> Function {
        Self(instance)
    }

    #[inline(always)]
    pub fn cast_unchecked<T: From<Instance>>(&self) -> T {
        T::from(self.0)
    }

    #[inline(always)]
    pub fn class<'local>(env: &mut JNIEnv<'local>) -> JClass {
        Instance::from(env.find_class(Self::CLASS).unwrap().into_raw()).into()
    }
}

impl IntoJValue for Function {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        self.0.into_jvalue(env)
    }
}

impl FromJValue for Function {
    fn from_jvalue<'local>(value: JValueOwned<'local>) -> Self {
        Instance::from_jvalue(value).into()
    }
}

// JClass
#[derive(Clone, Copy)]
pub struct JClass(pub Instance);

impl JSignature for JClass {
    const CLASS: &'static str = "java/lang/Class";
}

impl From<Instance> for JClass {
    fn from(value: Instance) -> Self {
        JClass::from_instance(value)
    }
}

impl Into<Instance> for JClass {
    fn into(self) -> Instance {
        self.0
    }
}

impl<'local> From<JniJClass<'local>> for JClass {
    fn from(class: JniJClass<'local>) -> Self {
        Self::from_class(class)
    }
}

impl<'local> Into<JniJClass<'local>> for JClass {
    fn into(self) -> JniJClass<'local> {
        self.into_class()
    }
}

impl ops::Deref for JClass {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl JClass {
    #[inline(always)]
    pub fn from_instance(instance: Instance) -> JClass {
        Self(instance)
    }

    #[inline(always)]
    pub fn cast_unchecked<T: From<Instance>>(&self) -> T {
        T::from(self.0)
    }

    #[inline(always)]
    pub fn class<'local>(env: &mut JNIEnv<'local>) -> JClass {
        env.find_class(Self::CLASS).unwrap().into()
    }

    pub fn from_class<'local>(class: JniJClass) -> JClass {
        Instance::from(class.into_raw()).into()
    }

    pub fn into_class<'local>(self) -> JniJClass<'local> {
        unsafe { JniJClass::from_raw(self.0.into()) }
    }
}

impl IntoJValue for JClass {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        self.0.into_jvalue(env)
    }
}

impl FromJValue for JClass {
    fn from_jvalue<'local>(value: JValueOwned<'local>) -> Self {
        Instance::from_jvalue(value).into()
    }
}
