use atat::atat_derive::AtatResp;
use heapless::String;

use crate::{bounded_integer_list, command::common::data::Enabled, enum_list};

use super::data::*;

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestStartMultiIpConnectionResponse {
    #[at_arg(position = 0)]
    pub connections: enum_list!(IpConnection),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadStartMultiIpConnectionResponse {
    #[at_arg(position = 0)]
    pub connection: IpConnection,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct SetApnUserNamePasswordResponse {
    #[at_arg(position = 0)]
    pub apn: GprsString,
    #[at_arg(position = 1)]
    pub user_name: GprsString,
    #[at_arg(position = 2)]
    pub password: GprsString,
}

// TODO: verify this works
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ExecuteGetLocalIpAddressResponse {
    #[at_arg(position = 0)]
    pub ip_address: String<64>,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetAutoSendingTimerResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(Enabled),
    #[at_arg(position = 1)]
    pub times: bounded_integer_list!(Seconds),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetAutoSendingTimerResponse {
    #[at_arg(position = 0)]
    pub mode: Enabled,
    #[at_arg(position = 1)]
    pub time: Seconds,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetPromptResponse {
    #[at_arg(position = 0)]
    pub send_prompts: enum_list!(SendPrompt),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetPromptResponse {
    #[at_arg(position = 0)]
    pub send_prompt: SendPrompt,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetCheckGprsNetworkTimingResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(Enabled),
    #[at_arg(position = 1)]
    pub intervals: bounded_integer_list!(Interval),
    #[at_arg(position = 2)]
    pub timers: bounded_integer_list!(Timer),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetCheckGprsNetworkTimingResponse {
    #[at_arg(position = 0)]
    pub mode: Enabled,
    #[at_arg(position = 1)]
    pub interval: Interval,
    #[at_arg(position = 2)]
    pub timer: Timer,
}

// TODO: verify this works
#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSelectTcpipApplicationModeResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(TcpipApplicationMode),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSelectTcpipApplicationModeResponse {
    #[at_arg(position = 0)]
    pub mode: TcpipApplicationMode,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSetRemoteDelayTimerResponse {
    #[at_arg(position = 0)]
    pub singles: bounded_integer_list!(TimerSingle),
    #[at_arg(position = 1)]
    pub multis: bounded_integer_list!(TimerMulti),
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct ReadSetRemoteDelayTimerResponse {
    #[at_arg(position = 0)]
    pub single: TimerSingle,
    #[at_arg(position = 1)]
    pub multi: TimerMulti,
}

#[derive(Clone, PartialEq, Eq, AtatResp)]
pub struct TestSelectGprsPdpContextResponse {
    #[at_arg(position = 0)]
    pub modes: enum_list!(GprsPdpContext),
}
