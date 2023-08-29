use atat::atat_derive::AtatResp;
use heapless::String;

use crate::{bounded_integer_list, enum_list};

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetHttpParametersValueResponse {
    #[at_arg(position = 0)]
    pub tag: String<64>,
    #[at_arg(position = 1)]
    pub value: String<64>,
}

// TODO: verify this works (it probably doesn't)
// TODO: consider implementing a custom deserializer
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetHttpParametersValueResponse {
    #[at_arg(position = 0)]
    pub tag: HttpParameterTag,
    #[at_arg(position = 1)]
    pub value: HttpParameterValue,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestInputHttpDataResponse {
    #[at_arg(position = 0)]
    pub sizes: bounded_integer_list!(Size),
    #[at_arg(position = 1)]
    pub times: bounded_integer_list!(Milliseconds),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestHttpMethodActionResponse {
    #[at_arg(position = 0)]
    pub methods: enum_list!(HttpMethod),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestHttpServerResponseResponse {
    #[at_arg(position = 0)]
    pub start_addresses: bounded_integer_list!(Size),
    #[at_arg(position = 1)]
    pub byte_sizes: bounded_integer_list!(DataLength),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct HttpServerResponseResponse {
    #[at_arg(position = 0)]
    pub data_length: DataLength,
    // TODO: test
    #[at_arg(position = 1)]
    pub data: HttpData,
}

// TODO: 11.2.7 AT+HTTPSCONT Save HTTP Application Context
// TODO: 11.2.8 AT+HTTPSTATUS Read HTTP Status
