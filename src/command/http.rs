//! AT Commands for HTTP Application

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::response::NoResponse;

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPINIT=?", NoResponse)]
pub struct TestInitializeHttpService {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPINIT", NoResponse)]
pub struct ExecuteInitializeHttpService {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPTERM=?", NoResponse)]
pub struct TestTerminateHttpService {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPTERM", NoResponse)]
pub struct ExecuteTerminateHttpService {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPPARA=?", TestSetHttpParametersValueResponse)]
pub struct TestSetHttpParametersValue {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPPARA?", ReadSetHttpParametersValueResponse)]
pub struct ReadSetHttpParametersValue {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPPARA", NoResponse)]
pub struct WriteSetHttpParametersValue {
    #[at_arg(position = 0)]
    pub tag: HttpParameterTag,
    #[at_arg(position = 1)]
    pub value: HttpParameterValue,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPDATA=?", TestInputHttpDataResponse)]
pub struct TestInputHttpData {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPDATA", NoResponse)]
pub struct WriteInputHttpData {
    #[at_arg(position = 0)]
    pub size: Size,
    #[at_arg(position = 1)]
    pub time: Milliseconds,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPACTION=?", TestHttpMethodActionResponse)]
pub struct TestHttpMethodAction {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPACTION", NoResponse)]
pub struct WriteHttpMethodAction {
    #[at_arg(position = 0)]
    pub method: HttpMethod,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPREAD=?", TestHttpServerResponseResponse)]
pub struct TestHttpServerResponse {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPREAD", HttpServerResponseResponse)]
pub struct WriteHttpServerResponse {
    #[at_arg(position = 0)]
    pub start_address: Size,
    #[at_arg(position = 1)]
    pub byte_size: DataLength,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+HTTPREAD", HttpServerResponseResponse)]
pub struct ExecuteHttpServerResponse {}
