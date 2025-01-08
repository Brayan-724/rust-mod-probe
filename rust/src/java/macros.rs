#[macro_export]
macro_rules! impl_methods {
    (
        [$class:ident];
        let $field:ident: $ty:ty;
        $($tt:tt)*
    ) => {
        $crate::java::impl_methods!{[$class]; let $field: $ty = $field; $($tt)*}
    };

    (
        [$class:ident];
        let $field:ident: $ty:ty = $raw:ident;
        $($tt:tt)*
    ) => {
        impl $class {
            pub fn $field(&self) -> $crate::java::Field<$ty> {
                let sig = $crate::java::impl_methods!(!ret $ty);

                println!("GETTING: {} : {sig}", stringify!($raw));

                $crate::java::Field(self.0, stringify!($raw), ::core::marker::PhantomData::default())
            }
        }

        $crate::java::impl_methods!{[$class]; $($tt)*}
    };
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
        impl $class {
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
        impl $class {
            pub fn $fn<'local>(&self, env: &mut ::jni::JNIEnv<'local>, $($arg : $arg_ty),*) $(-> $ret)? {
                #[allow(unused_mut)]
                let mut sig_args: Vec<String> = Vec::new();
                $(
                let $arg = $crate::java::IntoJValue::into_jvalue($arg, env);
                sig_args.push(<$arg_ty as $crate::java::JSignature>::sig_type());
                )*

                let sig = ["(", &sig_args.join(""), ")", $crate::java::impl_methods!(!ret $($ret)?)].join("");

                println!("CALLING instance: {} : {sig} on {}", stringify!($raw), <$class as $crate::java::JSignature>::sig());

                let _ret = env
                    .call_method(unsafe {::jni::objects::JObject::<'_>::from_raw(self.0) }, stringify!($raw), sig, &[$($arg.borrow()),*])
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
    (#ret $i:ident; bool) => {$i.z().unwrap()};
    (#ret $i:ident; $ret:ty) => {<$ret as $crate::java::FromJValue>::from_jvalue($i)};
}

#[macro_export]
macro_rules! new_class {
    ($name:ident $(( $($args:tt)* ))?: $jni:literal {
        $($tt:tt)*
    }) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub ::jni::sys::jobject);

        impl From<::jni::sys::jobject> for $name {
            fn from(value: ::jni::sys::jobject) -> $name {
                Self(value)
            }
        }

        impl<'local> From<::jni::objects::JObject<'local>> for $name {
            fn from(value: ::jni::objects::JObject<'local>) -> $name {
                Self(value.as_raw())
            }
        }

        impl $crate::java::FromJValue for $name {
            fn from_jvalue<'local>(value: ::jni::objects::JValueOwned<'local>) -> Self {
                let obj = value.l().unwrap();
                Self(obj.as_raw())
            }
        }

        impl<'local> Into<::jni::objects::JObject<'local>> for $name {
            fn into(self) -> ::jni::objects::JObject<'local> {
                unsafe { ::jni::objects::JObject::<'local>::from_raw(self.0) }
            }
        }

        impl<'local> Into<::jni::objects::JObject<'local>> for &$name {
            fn into(self) -> ::jni::objects::JObject<'local> {
                unsafe { ::jni::objects::JObject::<'local>::from_raw(self.0) }
            }
        }

        $crate::java::new_class!{!new [$name]; $($($args)*)?}

        impl $name {
            #[allow(dead_code)]
            #[inline(always)]
            pub fn class<'local>(env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JClass<'local> {
                env.find_class(<$name as $crate::java::JSignature>::sig()).unwrap()
            }

            #[allow(dead_code)]
            #[inline(always)]
            pub fn cast_by_object<T: From<::jni::sys::jobject>>(&self) -> T {
                T::from(self.0)
            }
        }

        impl $crate::java::JSignature for $name {
            fn sig() -> String {
                $jni.to_owned()
            }
        }

        impl $crate::java::IntoJValue for $name {
            fn into_jvalue<'local>(self, _: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                <&$name as Into<::jni::objects::JObject<'local>>>::into(&self).into()
            }
        }

        impl $crate::java::IntoJValue for &$name {
            fn into_jvalue<'local>(self, _: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                <&$name as Into<::jni::objects::JObject<'local>>>::into(self).into()
            }
        }

        $crate::java::impl_methods!{[$name]; $($tt)*}
    };

    (!new [$class:ident]; ) => {
        impl $class {
            #[allow(dead_code)]
            pub fn new<'local>(env: &mut ::jni::JNIEnv<'local>) -> $class {
                let class = Self::class(env);

                env.new_object(class, "()V", &[]).unwrap().into()
            }
        }
    };

    (!new [$class:ident]; $($arg:ident : $arg_ty:ty),+) => {
        impl $class {
            #[allow(dead_code)]
            pub fn new<'local>(env: &mut ::jni::JNIEnv<'local>, $($arg : $arg_ty),+) -> $class {
                let class = Self::class(env);

                $(
                let $arg = $crate::java::IntoJValue::into_jvalue($arg, env);
                )*

                let sig = ["(", $(&<$arg_ty as $crate::java::JSignature>::sig_type()),+ , ")V"].join("");
                env.new_object(class, sig, &[$($arg.borrow()),+]).unwrap().into()
            }
        }
    };
}

#[macro_export]
macro_rules! new_enum {
    ($name:ident: $jni:literal { }) => {
        compile_error!("Add some variant");
    };

    ($name:ident: $jni:literal {
        $($k:ident : $ty:ty),+
    }) => {
        pub enum $name {
            $($k,)*
        }

        impl $crate::java::JSignature for $name {
            fn sig() -> String {
                $jni.to_owned()
            }
        }

        impl $crate::java::IntoJValue for $name {
            fn into_jvalue<'local>(self, env: &mut ::jni::JNIEnv<'local>) -> ::jni::objects::JValueOwned<'local> {
                let class = env
                    .find_class($jni)
                    .expect(concat!("Cannot get ", stringify!($name), " class"));

                let (field, ty) = match &self {
                    $($name::$k => (stringify!($k), stringify!($ty))),*
                };

                env
                    .get_static_field(class, field, format!("L{}${ty};", <$name as $crate::java::JSignature>::sig()))
                    .unwrap()
                    .l()
                    .unwrap()
                    .into()
            }
        }
    };

    ($name:ident: $jni:literal {
        $($k:ident),+
    }) => {
        pub enum $name {
            $($k,)*
        }

        impl $crate::java::JSignature for $name {
            fn sig() -> String {
                $jni.to_owned()
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
                    .get_static_field(class, field, <$name as $crate::java::JSignature>::sig_type())
                    .unwrap()
                    .l()
                    .unwrap()
                    .into()
            }
        }
    };

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
                    .get_static_field(class, field, <$name as $crate::java::JSignature>::sig_type())
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
