pub mod common;
pub mod gprs;
// TODO: work out if this is a part of the 3gpp spec
pub mod http;
pub mod itu;
pub mod ping;
pub mod sms;
// TODO move to sim800 specific crate
pub mod special;
pub mod stk;
pub mod tcpip;
pub mod ue;

// TODO see if this can be in a separate crate
/// For list of supported enum variants, commonly as a response.
#[macro_export]
macro_rules! enum_list {
    ($t:ty) => {
        heapless::Vec<$t, {core::mem::variant_count::<$t>()}>
    };
}

#[macro_export]
macro_rules! bounded_integer_list {
    ($t:ty) => {
        heapless::Vec<$t, {<$t>::RANGE_LEN}>
    };
}
