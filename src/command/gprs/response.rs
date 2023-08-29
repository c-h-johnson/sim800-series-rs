use atat::atat_derive::AtatResp;

use crate::enum_list;

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestAttachDetachGprsServiceResponse {
    #[at_arg(position = 0)]
    pub states: enum_list!(GprsAttachment),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadAttachDetachGprsServiceResponse {
    #[at_arg(position = 0)]
    pub state: GprsAttachment,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestEnterDataStateResponse {
    #[at_arg(position = 0)]
    pub protocol: enum_list!(Level2Protocol),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestGprsMobileStationClassResponse {
    #[at_arg(position = 0)]
    pub classes: enum_list!(GprsMobileClass),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadGprsMobileStationClassResponse {
    #[at_arg(position = 0)]
    pub class: GprsMobileClass,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestServiceForMoSmsMessagesResponse {
    #[at_arg(position = 0)]
    pub services: enum_list!(Service),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadServiceForMoSmsMessagesResponse {
    #[at_arg(position = 0)]
    pub service: Service,
}
