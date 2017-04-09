#[doc(hidden)]
#[macro_export]
macro_rules! forward_to_deserialize_method {
    ($func:ident($($arg:ty),*)) => {
        #[inline]
        fn $func<__V>(self, $(_: $arg,)* visitor: __V) -> $crate::export::Result<__V::Value, Self::Error>
            where __V: $crate::de::Visitor<'de>
        {
            self.deserialize(visitor)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! forward_to_deserialize_helper {
    (bool) => {
        forward_to_deserialize_method!{deserialize_bool()}
    };
    (u8) => {
        forward_to_deserialize_method!{deserialize_u8()}
    };
    (u16) => {
        forward_to_deserialize_method!{deserialize_u16()}
    };
    (u32) => {
        forward_to_deserialize_method!{deserialize_u32()}
    };
    (u64) => {
        forward_to_deserialize_method!{deserialize_u64()}
    };
    (i8) => {
        forward_to_deserialize_method!{deserialize_i8()}
    };
    (i16) => {
        forward_to_deserialize_method!{deserialize_i16()}
    };
    (i32) => {
        forward_to_deserialize_method!{deserialize_i32()}
    };
    (i64) => {
        forward_to_deserialize_method!{deserialize_i64()}
    };
    (f32) => {
        forward_to_deserialize_method!{deserialize_f32()}
    };
    (f64) => {
        forward_to_deserialize_method!{deserialize_f64()}
    };
    (char) => {
        forward_to_deserialize_method!{deserialize_char()}
    };
    (str) => {
        forward_to_deserialize_method!{deserialize_str()}
    };
    (string) => {
        forward_to_deserialize_method!{deserialize_string()}
    };
    (unit) => {
        forward_to_deserialize_method!{deserialize_unit()}
    };
    (option) => {
        forward_to_deserialize_method!{deserialize_option()}
    };
    (seq) => {
        forward_to_deserialize_method!{deserialize_seq()}
    };
    (seq_fixed_size) => {
        forward_to_deserialize_method!{deserialize_seq_fixed_size(usize)}
    };
    (bytes) => {
        forward_to_deserialize_method!{deserialize_bytes()}
    };
    (byte_buf) => {
        forward_to_deserialize_method!{deserialize_byte_buf()}
    };
    (map) => {
        forward_to_deserialize_method!{deserialize_map()}
    };
    (unit_struct) => {
        forward_to_deserialize_method!{deserialize_unit_struct(&'static str)}
    };
    (newtype_struct) => {
        forward_to_deserialize_method!{deserialize_newtype_struct(&'static str)}
    };
    (tuple_struct) => {
        forward_to_deserialize_method!{deserialize_tuple_struct(&'static str, usize)}
    };
    (struct) => {
        forward_to_deserialize_method!{deserialize_struct(&'static str, &'static [&'static str])}
    };
    (identifier) => {
        forward_to_deserialize_method!{deserialize_identifier()}
    };
    (tuple) => {
        forward_to_deserialize_method!{deserialize_tuple(usize)}
    };
    (enum) => {
        forward_to_deserialize_method!{deserialize_enum(&'static str, &'static [&'static str])}
    };
    (ignored_any) => {
        forward_to_deserialize_method!{deserialize_ignored_any()}
    };
}

// Super explicit first paragraph because this shows up at the top level and
// trips up people who are just looking for basic Serialize / Deserialize
// documentation.
//
/// Helper macro when implementing the `Deserializer` part of a new data format
/// for Serde.
///
/// Some `Deserializer` implementations for self-describing formats do not care
/// what hint the `Visitor` gives them, they just want to blindly call the
/// `Visitor` method corresponding to the data they can tell is in the input.
/// This requires repetitive implementations of all the `Deserializer` trait
/// methods.
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde;
/// #
/// # use serde::de::{value, Deserializer, Visitor};
/// #
/// # struct MyDeserializer;
/// #
/// # impl<'de> Deserializer<'de> for MyDeserializer {
/// #     type Error = value::Error;
/// #
/// #     fn deserialize<V>(self, _: V) -> Result<V::Value, Self::Error>
/// #         where V: Visitor<'de>
/// #     {
/// #         unimplemented!()
/// #     }
/// #
/// #[inline]
/// fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
///     where V: Visitor<'de>
/// {
///     self.deserialize(visitor)
/// }
/// #
/// #     forward_to_deserialize! {
/// #         u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
/// #         seq seq_fixed_size bytes byte_buf map unit_struct newtype_struct
/// #         tuple_struct struct identifier tuple enum ignored_any
/// #     }
/// # }
/// #
/// # fn main() {}
/// ```
///
/// The `forward_to_deserialize!` macro implements these simple forwarding
/// methods so that they forward directly to `Deserializer::deserialize`. You
/// can choose which methods to forward.
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde;
/// #
/// # use serde::de::{value, Deserializer, Visitor};
/// #
/// # struct MyDeserializer;
/// #
/// impl<'de> Deserializer<'de> for MyDeserializer {
/// #   type Error = value::Error;
/// #
///     fn deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error>
///         where V: Visitor<'de>
///     {
///         /* ... */
/// #       let _ = visitor;
/// #       unimplemented!()
///     }
///
///     forward_to_deserialize! {
///         bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
///         seq seq_fixed_size bytes byte_buf map unit_struct newtype_struct
///         tuple_struct struct identifier tuple enum ignored_any
///     }
/// }
/// #
/// # fn main() {}
/// ```
///
/// The macro assumes the convention that your `Deserializer` lifetime parameter
/// is called `'de`. It will not work if the `Deserializer` lifetime parameter
/// is called something different.
#[macro_export]
macro_rules! forward_to_deserialize {
    ($($func:ident)*) => {
        $(forward_to_deserialize_helper!{$func})*
    };
}
