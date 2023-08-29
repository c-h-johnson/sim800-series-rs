//! AT Commands for GPRS Support

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::{data::Integer, response::NoResponse};

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGATT=?", TestAttachDetachGprsServiceResponse)]
pub struct TestAttachDetachGprsService {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGATT?", ReadAttachDetachGprsServiceResponse)]
pub struct ReadAttachDetachGprsService {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGATT", NoResponse, timeout_ms = 10000)]
pub struct WriteAttachDetachGprsService {
    #[at_arg(position = 0)]
    pub state: GprsAttachment,
}

// TODO: 7.2.2 AT+CGDCONT Define PDP Context
// TODO: 7.2.3 AT+CGQMIN Quality of Service Profile (Minimum Acceptable)
// TODO: 7.2.4 AT+CGQREQ Quality of Service Profile (Requested)
// TODO: 7.2.5 AT+CGACT PDP Context Activate or Deactivate

// TODO: verify this works
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGDATA=?", TestEnterDataStateResponse)]
pub struct TestEnterDataState {}

// TODO: verify this works
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGDATA", NoResponse)]
pub struct WriteEnterDataState {
    #[at_arg(position = 0)]
    pub protocol: Level2Protocol,
    #[at_arg(position = 1)]
    pub cid: Option<Integer>,
}

// TODO: 7.2.7 AT+CGPADDR Show PDP Address

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGCLASS=?", TestGprsMobileStationClassResponse)]
pub struct TestGprsMobileStationClass {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGCLASS?", ReadGprsMobileStationClassResponse)]
pub struct ReadGprsMobileStationClass {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGCLASS", NoResponse)]
pub struct WriteGprsMobileStationClass {
    #[at_arg(position = 0)]
    pub class: GprsMobileClass,
}

// TODO: 7.2.9 AT+CGEREP Control Unsolicited GPRS Event Reporting
// TODO: 7.2.10 AT+CGREG Network Registration Status

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGSMS=?", TestServiceForMoSmsMessagesResponse)]
pub struct TestServiceForMoSmsMessages {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGSMS?", ReadServiceForMoSmsMessagesResponse)]
pub struct ReadServiceForMoSmsMessages {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CGSMS", NoResponse)]
pub struct WriteServiceForMoSmsMessages {
    #[at_arg(position = 0)]
    pub service: Service,
}
