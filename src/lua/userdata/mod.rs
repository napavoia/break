mod color;
mod game;
mod vector;
mod window;

#[macro_export]
macro_rules! impl_method {
    ($methods:ident; $f:ident) => {
        impl_method!($methods; stringify!($f) => $f())
    };
    ($methods:ident; $name:expr => $f:ident()) => {
        $methods.add_method_mut($name, |_, this, _: MultiValue| {
            Ok(this.$f())
        })
    };
    ($methods:ident;; $f:ident($($v:ident: $t:ty),*)) => {
        impl_method!($methods; stringify!($f) => $f($($v: $t),*))
    };
    ($methods:ident; $name:expr => $f:ident($($v:ident: $t:ty),*)) => {
        $methods.add_method_mut($name, |_, this, ($($v,)*): ($($t,)*)| {
            this.$f($($v,)*)
                .map_err(|e| e.external().context($name))
        })
    };

    ($methods:ident;; $f:ident($($v:ident: $t:ty),*) -> Ok) => {
        impl_method!($methods; stringify!($f) => $f($($v: $t),*) -> Ok)
    };
    ($methods:ident; $name:expr => $f:ident($($v:ident: $t:ty),*) -> Ok) => {
        $methods.add_method_mut($name, |_, this, ($($v,)*): ($($t,)*)| {
            Ok(this.$f($($v,)*))
        })
    };

    ($methods:ident; $name: literal) => {
        $methods.add_method_mut($name, |_, _, _: MultiValue| Result::<(), mlua::Error>::Err(mlua::Error::RuntimeError(stringify!($name not yet implemented).to_string())))
    };
}

#[macro_export]
macro_rules! impl_gs {
    (get $fields:ident; $f:ident) => {
        impl_gs!(get $fields; stringify!($f) => $f)
    };
    (get $fields:ident; $name:expr => $f:ident) => {
        $fields.add_field_method_get($name, |_, this| Ok(this.0.$f))
    };
    (set $fields:ident; $f:ident is $t:ty) => {
        impl_gs!(set $fields; stringify!($f) => $f is $t)
    };
    (set $fields:ident; $name:expr => $f:ident is $t:ty) => {
        $fields.add_field_method_set($name, |_, this, v: $t| {
            this.0.$f = v.0;
            Ok(())
        });
    };

    (getset $fields:ident; $f:ident is $t:ty) => {
        impl_gs!(getset $fields; stringify!($f) => $f is $t)
    };
    (getset $fields:ident; $name:expr => $f:ident is $t:ty) => {
        impl_gs!(get $fields; stringify!($f) => $f);
        impl_gs!(set $fields; stringify!($f) => $f is $t);
    };
}

#[macro_export]
macro_rules! impl_const {
    ($methods:ident; $f:ident) => {
        impl_const!($methods; stringify!($f) => $f)
    };
    ($methods:ident; $name:expr => $f:ident) => {
        $methods.add_function($name, |_, _: Value| Ok(Self::$f))
    };
}

#[macro_export]
macro_rules! impl_display {
    ($methods:ident) => {
        use mlua::{MetaMethod::ToString, MultiValue};
        $methods.add_meta_method(ToString, |_, this, _: MultiValue| Ok(this.to_string()));
    };
}
