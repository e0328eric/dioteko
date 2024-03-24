// A macro wrapper that makes windows _bool type to "real" bool
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! to_bool {
    ($fnt: expr) => {
        $fnt != 0
    };
}
#[cfg(not(target_os = "windows"))]
macro_rules! to_bool {
    ($fnt: expr) => {
        $fnt
    };
}

#[macro_export]
macro_rules! impl_raylib_enum {
    ($enum_name: ident => $(($name: ident, $ffi: ident),)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $enum_name {
            $($name,)+
        }

        impl From<i32> for $enum_name {
            #[inline]
            fn from(ffi_enum: i32) -> $enum_name {
                #[cfg(target_os = "windows")]
                match ffi_enum {
                    $(ffi::$ffi => $enum_name::$name,)+
                    _ => panic!("[dioteko internal error]: Unreatchable"),
                }
                #[cfg(not(target_os = "windows"))]
                match ffi_enum as u32 {
                    $(ffi::$ffi => $enum_name::$name,)+
                    _ => panic!("[dioteko internal error]: Unreatchable"),
                }
            }
        }

        impl From<$enum_name> for i32 {
            #[inline]
            fn from(r#enum: $enum_name) -> i32 {
                #[cfg(target_os = "windows")]
                match r#enum {
                    $($enum_name::$name => ffi::$ffi,)+
                }
                #[cfg(not(target_os = "windows"))]
                match r#enum {
                    $($enum_name::$name => ffi::$ffi as i32,)+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vec {
    ($vec_name: ident | $($components: ident),+) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $vec_name {
            $(pub $components: f32,)+
        }

        impl $vec_name {
            pub fn new($($components: f32,)+) -> Self {
                Self { $($components,)+ }
            }
        }

        impl From<ffi::$vec_name> for $vec_name {
            #[inline]
            fn from(vec: ffi::$vec_name) -> Self {
                Self {
                    $($components: vec.$components,)+
                }
            }
        }

        impl From<$vec_name> for ffi::$vec_name {
            #[inline]
            fn from(vec: $vec_name) -> ffi::$vec_name {
                ffi::$vec_name {
                    $($components: vec.$components,)+
                }
            }
        }

        // Addition of Vectors
        impl Add for $vec_name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    $($components: self.$components + rhs.$components,)+
                }
            }
        }

        // Subtraction of Vectors
        impl Sub for $vec_name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    $($components: self.$components - rhs.$components,)+
                }
            }
        }

        impl AddAssign for $vec_name {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl SubAssign for $vec_name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // Scalar multiplication of Vectors
        impl Mul<$vec_name> for f32 {
            type Output = $vec_name;
            fn mul(self, rhs: $vec_name) -> Self::Output {
                $vec_name {
                    $($components: self * rhs.$components,)+
                }
            }
        }
    };
}
