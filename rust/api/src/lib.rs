pub mod class;
pub mod conversion;
pub mod primitives;

use class::Instance;
use conversion::{FromJValue, IntoJValue};
use jni::JNIEnv;
pub use rosttasse_macros::*;

pub trait JSignature {
    const CLASS: &'static str;
    const CLASS_LEN: usize = Self::CLASS.len();

    fn sig_class() -> &'static str {
        Self::CLASS
    }

    fn sig() -> String {
        let mut out = String::with_capacity(Self::CLASS_LEN + 2);

        out.push('L');
        out.push_str(Self::CLASS);
        out.push(';');

        out
    }

    #[inline(always)]
    fn class<'local>(env: &mut JNIEnv<'local>) -> primitives::JClass {
        env.find_class(Self::CLASS).unwrap().into()
    }
}

pub trait JavaClass: JSignature + IntoJValue + FromJValue {
    fn get_raw(&self) -> Instance;
    fn from_raw(raw: Instance) -> Self;
}

pub mod prelude {
    pub use crate::class::*;
    pub use crate::conversion::*;
    pub use crate::primitives::*;
    pub use crate::*;
}
