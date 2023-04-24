//! Conversion to and from ipld.
use crate::error::TypeErrorType;
use crate::ipld::Ipld;
use crate::{cid::Cid, error::TypeError};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

impl TryFrom<Ipld> for () {
    type Error = TypeError;

    fn try_from(ipld: Ipld) -> Result<Self, Self::Error> {
        match ipld {
            Ipld::Null => Ok(()),
            _ => {
                return Err(TypeError {
                    expected: TypeErrorType::Null,
                    found: ipld.into(),
                })
            }
        }
    }
}

macro_rules! derive_from_ipld_option {
    ($enum:ident, $ty:ty) => {
        impl TryFrom<Ipld> for Option<$ty> {
            type Error = TypeError;

            fn try_from(ipld: Ipld) -> Result<Self, Self::Error> {
                match ipld {
                    Ipld::Null => Ok(None),
                    Ipld::$enum(value) => Ok(Some(value as _)),
                    _ => {
                        return Err(TypeError {
                            expected: TypeErrorType::$enum,
                            found: ipld.into(),
                        })
                    }
                }
            }
        }
    };
}

macro_rules! derive_from_ipld {
    ($enum:ident, $ty:ty) => {
        impl TryFrom<Ipld> for $ty {
            type Error = TypeError;

            fn try_from(ipld: Ipld) -> Result<Self, Self::Error> {
                match ipld {
                    Ipld::$enum(value) => Ok(value as _),
                    _ => {
                        return Err(TypeError {
                            expected: TypeErrorType::$enum,
                            found: ipld.into(),
                        })
                    }
                }
            }
        }
    };
}

macro_rules! derive_to_ipld_prim {
    ($enum:ident, $ty:ty, $fn:ident) => {
        impl From<$ty> for Ipld {
            fn from(t: $ty) -> Self {
                Ipld::$enum(t.$fn() as _)
            }
        }
    };
}

macro_rules! derive_to_ipld {
    ($enum:ident, $ty:ty, $($fn:ident),*) => {
        impl From<$ty> for Ipld {
            fn from(t: $ty) -> Self {
                Ipld::$enum(t$(.$fn())*)
            }
        }
    };
}

derive_to_ipld!(Bool, bool, clone);
derive_to_ipld_prim!(Integer, i8, clone);
derive_to_ipld_prim!(Integer, i16, clone);
derive_to_ipld_prim!(Integer, i32, clone);
derive_to_ipld_prim!(Integer, i64, clone);
derive_to_ipld_prim!(Integer, i128, clone);
derive_to_ipld_prim!(Integer, isize, clone);
derive_to_ipld_prim!(Integer, u8, clone);
derive_to_ipld_prim!(Integer, u16, clone);
derive_to_ipld_prim!(Integer, u32, clone);
derive_to_ipld_prim!(Integer, u64, clone);
derive_to_ipld_prim!(Integer, usize, clone);
derive_to_ipld_prim!(Float, f32, clone);
derive_to_ipld_prim!(Float, f64, clone);
derive_to_ipld!(String, String, into);
derive_to_ipld!(String, &str, to_string);
derive_to_ipld!(Bytes, Box<[u8]>, into_vec);
derive_to_ipld!(Bytes, Vec<u8>, into);
derive_to_ipld!(Bytes, &[u8], to_vec);
derive_to_ipld!(List, Vec<Ipld>, into);
derive_to_ipld!(Map, BTreeMap<String, Ipld>, to_owned);
derive_to_ipld!(Link, Cid, clone);
derive_to_ipld!(Link, &Cid, to_owned);

derive_from_ipld!(Bool, bool);
derive_from_ipld!(Integer, i8);
derive_from_ipld!(Integer, i16);
derive_from_ipld!(Integer, i32);
derive_from_ipld!(Integer, i64);
derive_from_ipld!(Integer, i128);
derive_from_ipld!(Integer, isize);
derive_from_ipld!(Integer, u8);
derive_from_ipld!(Integer, u16);
derive_from_ipld!(Integer, u32);
derive_from_ipld!(Integer, u64);
derive_from_ipld!(Integer, u128);
derive_from_ipld!(Integer, usize);
derive_from_ipld!(Float, f32);
derive_from_ipld!(Float, f64);
derive_from_ipld!(String, String);
derive_from_ipld!(Bytes, Vec<u8>);
derive_from_ipld!(List, Vec<Ipld>);
derive_from_ipld!(Map, BTreeMap<String, Ipld>);
derive_from_ipld!(Link, Cid);

derive_from_ipld_option!(Bool, bool);
derive_from_ipld_option!(Integer, i8);
derive_from_ipld_option!(Integer, i16);
derive_from_ipld_option!(Integer, i32);
derive_from_ipld_option!(Integer, i64);
derive_from_ipld_option!(Integer, i128);
derive_from_ipld_option!(Integer, isize);
derive_from_ipld_option!(Integer, u8);
derive_from_ipld_option!(Integer, u16);
derive_from_ipld_option!(Integer, u32);
derive_from_ipld_option!(Integer, u64);
derive_from_ipld_option!(Integer, u128);
derive_from_ipld_option!(Integer, usize);
derive_from_ipld_option!(Float, f32);
derive_from_ipld_option!(Float, f64);
derive_from_ipld_option!(String, String);
derive_from_ipld_option!(Bytes, Vec<u8>);
derive_from_ipld_option!(List, Vec<Ipld>);
derive_from_ipld_option!(Map, BTreeMap<String, Ipld>);
derive_from_ipld_option!(Link, Cid);
