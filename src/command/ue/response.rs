use atat::atat_derive::AtatResp;
use heapless::Vec;

use crate::enum_list;

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSelectTeCharacterSetResponse {
    #[at_arg(position = 0)]
    pub chsets: enum_list!(CharacterSet),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSelectTeCharacterSetResponse {
    #[at_arg(position = 0)]
    pub chset: CharacterSet,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestOperatorSelectionResponse {
    // TODO check this is deserialized correctly
    #[at_arg(position = 0)]
    pub operators: Vec<
        (
            enum_list!(OperatorStatus),
            Operator,
            Operator,
            NumericOperator,
        ),
        MAX_OPERATORS,
    >,
    #[at_arg(position = 1)]
    pub modes: Option<enum_list!(OperatorMode)>,
    #[at_arg(position = 2)]
    pub formats: Option<enum_list!(OperatorFormat)>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadOperatorSelectionResponse {
    #[at_arg(position = 0)]
    pub mode: OperatorMode,
    #[at_arg(position = 1)]
    pub format: Option<OperatorFormat>,
    #[at_arg(position = 2)]
    pub operator: Option<Operator>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestNetworkRegistrationResponse {
    #[at_arg(position = 0)]
    pub urc_modes: enum_list!(NetworkRegistrationUrcMode),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadNetworkRegistrationResponse {
    #[at_arg(position = 0)]
    pub urc_mode: NetworkRegistrationUrcMode,
    #[at_arg(position = 1)]
    pub status: NetworkRegistrationStatus,
    #[at_arg(position = 2)]
    pub location: Option<LocationAreaCode>,
    #[at_arg(position = 3)]
    pub cell: Option<CellId>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSignalQualityReportResponse {
    #[at_arg(position = 0)]
    pub signal_strengths: Vec<u8, 100>,
    #[at_arg(position = 1)]
    pub error_rates: Vec<u8, 100>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ExecuteSignalQualityReportResponse {
    #[at_arg(position = 0)]
    pub signal_strength: u8,
    #[at_arg(position = 1)]
    pub error_rate: u8,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ExecuteReadOperatorNamesResponse {
    // TODO: make sure this can handle extra [<CR><LF>+COPN: <numeric2>,<alpha2>[…]]
    #[at_arg(position = 0)]
    pub operators: Vec<(NumericOperator, Operator), MAX_OPERATORS>,
}

// TODO test
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestBatteryChargeResponse {
    #[at_arg(position = 0)]
    pub statuses: enum_list!(ChargeStatus),
    #[at_arg(position = 1)]
    pub levels: Vec<u8, 100>,
    #[at_arg(position = 2)]
    pub voltage: u16,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ExecuteBatteryChargeResponse {
    #[at_arg(position = 0)]
    pub status: ChargeStatus,
    #[at_arg(position = 1)]
    pub level: u8,
    #[at_arg(position = 2)]
    // TODO verify this is integer format
    pub voltage: u16,
}
