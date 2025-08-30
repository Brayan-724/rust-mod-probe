use jni::{
    objects::{JObject, JValueOwned},
    JNIEnv,
};

pub trait IntoJValue {
    fn into_jvalue<'local>(self, env: &mut JNIEnv<'local>) -> JValueOwned<'local>;
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
