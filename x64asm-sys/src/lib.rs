#[allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals, deref_nullptr)]
pub mod sys {
    type _CharT = i8;
    type _Traits = ();
    type _RehashPolicy = ();
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use sys::*;