#[macro_export]
macro_rules! impl_methods {
    (
        [$class:ident];
        static fn $fn:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ret:ty)?;
        $($tt:tt)*
    ) => {
        $crate::java::impl_methods!{[$class]; static $fn fn $fn ($($arg : $arg_ty),*) $(-> $ret)?; $($tt)*}
    };

    (
        [$class:ident];
        static $raw:ident fn $fn:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ret:ty)?;
        $($tt:tt)*
    ) => {
        impl $class<'_> {
            pub fn $fn<'local>(env: &mut ::jni::JNIEnv<'local>, $($arg : $arg_ty),*) $(-> $ret)? {
                let mut sig_args = Vec::new();
                $(
                let $arg = $crate::java::IntoJValue::into_jvalue($arg, env);
                sig_args.push(<$arg_ty as $crate::java::JSignature>::sig_type());
                )*

                let sig = ["(", &sig_args.join(""), ")", $crate::java::impl_methods!(!ret $($ret)?)].join("");

                println!("CALLING: {} : {sig}", stringify!($raw));

                let class = Self::class(env);

                let _ret = env
                    .call_static_method(class, stringify!($raw), sig, &[$($arg.borrow()),*])
                    .unwrap();

                $crate::java::impl_methods!(#ret _ret; $($ret)?)
            }
        }

        $crate::java::impl_methods!{[$class]; $($tt)*}
    };

    (
        [$class:ident];
        pub fn $fn:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ret:ty)?;
        $($tt:tt)*
    ) => {
        $crate::java::impl_methods!{[$class]; pub $fn fn $fn ($($arg : $arg_ty),*) $(-> $ret)?; $($tt)*}
    };

    (
        [$class:ident];
        pub $raw:ident fn $fn:ident ( $($arg:ident : $arg_ty:ty),* $(,)? ) $(-> $ret:ty)?;
        $($tt:tt)*
    ) => {
        impl $class<'_> {
            pub fn $fn<'local>(&self, env: &mut ::jni::JNIEnv<'local>, $($arg : $arg_ty),*) $(-> $ret)? {
                #[allow(unused_mut)]
                let mut sig_args: Vec<String> = Vec::new();
                $(
                let $arg = $crate::java::IntoJValue::into_jvalue($arg, env);
                sig_args.push(<$arg_ty as $crate::java::JSignature>::sig_type());
                )*

                let sig = ["(", &sig_args.join(""), ")", $crate::java::impl_methods!(!ret $($ret)?)].join("");

                println!("CALLING instance: {} : {sig} on {}", stringify!($raw), $class::sig());

                let _ret = env
                    .call_method(&self.0, stringify!($raw), sig, &[$($arg.borrow()),*])
                    .unwrap();

                $crate::java::impl_methods!(#ret _ret; $($ret)?)
            }
        }

        $crate::java::impl_methods!{[$class]; $($tt)*}
    };

    ([$class:ident]; ) => {};

    (!ret ()) => {"V"};
    (!ret) => {"V"};
    (!ret $ret:ty) => {&<$ret as $crate::java::JSignature>::sig_type()};

    (#ret $i:ident; ) => {()};
    (#ret $i:ident; ()) => {()};
    (#ret $i:ident; $ret:ty) => {$i.l().unwrap().into()};
}

#[macro_export]
macro_rules! new_class {
    ($name:ident: $jni:literal {
        $($tt:tt)*
    }) => {
        pub struct $name<'local>(pub ::jni::objects::JObject<'local>);

        impl<'local> From<::jni::objects::JObject<'local>> for $name<'local> {
            fn from(value: ::jni::objects::JObject<'local>) -> $name<'local> {
                Self(value)
            }
        }

        impl<'local> Into<::jni::objects::JObject<'local>> for $name<'local> {
            fn into(self) -> ::jni::objects::JObject<'local> {
                self.0
            }
        }

        impl $name<'_> {
            #[allow(dead_code)]
            pub fn new<'local>(env: &mut ::jni::JNIEnv<'local>) -> $name<'local> {
                let class = Self::class(env);

                env.new_object(class, "()V", &[]).unwrap().into()
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn class<'local>(env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JClass<'local> {
                env.find_class(<$name as $crate::java::JSignature>::sig()).unwrap()
            }
        }

        impl $crate::java::JSignature for $name<'_> {
            fn sig() -> String {
                $jni.to_owned()
            }
        }

        impl $crate::java::IntoJValue for $name<'_> {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                env.new_local_ref(&self.0).unwrap().into()
            }
        }

        impl $crate::java::IntoJValue for &$name<'_> {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                env.new_local_ref(&self.0).unwrap().into()
            }
        }

        $crate::java::impl_methods!{[$name]; $($tt)*}
    };
}

#[macro_export]
macro_rules! new_enum {
    ($name:ident [$from:ident]: $jni:literal { }) => {
        compile_error!("Add some variant");
    };

    ($name:ident [$from:ident]: $jni:literal {
        $($k:ident),+
    }) => {
        pub enum $name {
            $($k,)*
        }

        impl $crate::java::JSignature for $name {
            fn sig() -> String {
                $from::sig()
            }

            fn sig_type() -> String {
                $from::sig_type()
            }
        }

        impl $crate::java::IntoJValue for $name {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                let class = env
                    .find_class($jni)
                    .expect(concat!("Cannot get ", stringify!($name), " class"));

                let field = match &self {
                    $($name::$k => stringify!($k)),*
                };

                env
                    .get_static_field(class, field, Self::sig_type())
                    .unwrap()
                    .l()
                    .unwrap()
                    .into()
            }
        }
    };
}

pub use impl_methods;
pub use new_class;
pub use new_enum;
