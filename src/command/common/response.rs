use atat::atat_derive::AtatResp;

use crate::enum_list;

use super::data::Enabled;

#[derive(Clone, AtatResp)]
pub struct NoResponse;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestEnabledResponse {
    #[at_arg(position = 0)]
    pub states: enum_list!(Enabled),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadEnabledResponse {
    #[at_arg(position = 0)]
    pub state: Enabled,
}
