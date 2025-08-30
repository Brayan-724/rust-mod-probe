use jni::{
    objects::{JObject, JValueOwned},
    JNIEnv,
};

use crate::prelude::AsJni;

pub trait IntoJValue {
    type JniType<'local>: AsJni<'local> = JObject<'local>;

    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local>;

    fn into_jni<'local>(self, env: &mut JNIEnv<'local>) -> Self::JniType<'local>
    where
        Self: Sized,
    {
        Self::JniType::<'local>::as_jni(self.into_jvalue(env))
    }
}

impl IntoJValue for JObject<'_> {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local> {
        env.new_local_ref(self).unwrap().into()
    }
}

pub trait FromJValue {
    fn from_jvalue<'local>(value: JValueOwned<'local>) -> Self;
}

pub trait Resolve<T> {
    fn resolve<'local>(self, env: &mut JNIEnv<'local>) -> T;
}

impl<T> Resolve<T> for T {
    fn resolve<'local>(self, _: &mut JNIEnv<'local>) -> T {
        self
    }
}
