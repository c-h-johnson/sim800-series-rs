use atat::atat_derive::AtatResp;

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadRingsBeforeAutomaticallyAnsweringCallResponse {
    #[at_arg(position = 0)]
    pub rings: Rings,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSecondsResponse {
    #[at_arg(position = 0)]
    pub seconds: Seconds,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadDisconnectDelayAfterDataCarrierAbsenceResponse {
    #[at_arg(position = 0)]
    pub delay: Deciseconds,
}
