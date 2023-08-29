//! AT Commands for TCPIP Application Toolkit

pub mod data;
pub mod response;

use atat::atat_derive::AtatCmd;

use data::*;
use response::*;

use crate::command::common::response::NoResponse;

use super::common::{
    data::Enabled,
    response::{ReadEnabledResponse, TestEnabledResponse},
};

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPMUX=?", TestStartMultiIpConnectionResponse)]
pub struct TestStartMultiIpConnection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPMUX?", ReadStartMultiIpConnectionResponse)]
pub struct ReadStartMultiIpConnection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPMUX", NoResponse)]
pub struct WriteStartMultiIpConnection {
    #[at_arg(position = 0)]
    pub connection: IpConnection,
}

// TODO: 8.2.2 AT+CIPSTART Start Up TCP or UDP Connection
// TODO: 8.2.3 AT+CIPSEND Send Data Through TCP or UDP Connection
// TODO: 8.2.4 AT+CIPQSEND Select Data Transmitting Mode
// TODO: 8.2.5 AT+CIPACK Query Previous Connection Data Transmitting State
// TODO: 8.2.6 AT+CIPCLOSE Close TCP or UDP Connection
// TODO: 8.2.7 AT+CIPSHUT Deactivate GPRS PDP Context
// TODO: 8.2.8 AT+CLPORT Set Local Port

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSTT=?", SetApnUserNamePasswordResponse)]
pub struct TestSetApnUserNamePassword {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSTT?", SetApnUserNamePasswordResponse)]
pub struct ReadSetApnUserNamePassword {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSTT", NoResponse)]
pub struct WriteSetApnUserNamePassword {
    #[at_arg(position = 0)]
    pub apn: GprsString,
    #[at_arg(position = 1)]
    pub user_name: GprsString,
    #[at_arg(position = 2)]
    pub password: GprsString,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CSTT", NoResponse)]
pub struct ExecuteSetApnUserNamePassword {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIICR=?", NoResponse)]
pub struct TestBringUpWirelessConnection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIICR", NoResponse)]
pub struct ExecuteBringUpWirelessConnection {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIFSR=?", NoResponse)]
pub struct TestGetLocalIpAddress {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIFSR", ExecuteGetLocalIpAddressResponse)]
pub struct ExecuteGetLocalIpAddress {}

// TODO: 8.2.12 AT+CIPSTATUS Query Current Connection Status
// TODO: 8.2.13 AT+CDNSCFG Configure Domain Name Server
// TODO: 8.2.14 AT+CDNSGIP Query the IP Address of Given Domain Name
// TODO: 8.2.15 AT+CIPHEAD Add an IP Head at the Beginning of a Package Received

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPATS=?", TestSetAutoSendingTimerResponse)]
pub struct TestSetAutoSendingTimer {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPATS?", ReadSetAutoSendingTimerResponse)]
pub struct ReadSetAutoSendingTimer {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPATS", NoResponse)]
pub struct WriteSetAutoSendingTimer {
    #[at_arg(position = 0)]
    pub mode: Enabled,
    #[at_arg(position = 1)]
    pub time: Option<Seconds>,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSPRT=?", TestSetPromptResponse)]
pub struct TestSetPrompt {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSPRT?", ReadSetPromptResponse)]
pub struct ReadSetPrompt {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSPRT", NoResponse)]
pub struct WriteSetPrompt {
    #[at_arg(position = 0)]
    pub send_prompt: SendPrompt,
}

// TODO: 8.2.18 AT+CIPSERVER Configure Module as Server
// TODO: 8.2.19 AT+CIPCSGP Set CSD or GPRS for Connection Mode

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSRIP=?", TestEnabledResponse)]
pub struct TestShowRemoteAddress {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSRIP?", ReadEnabledResponse)]
pub struct ReadShowRemoteAddress {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSRIP", NoResponse)]
pub struct WriteShowRemoteAddress {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPDPDP=?", TestSetCheckGprsNetworkTimingResponse)]
pub struct TestSetCheckGprsNetworkTiming {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPDPDP?", ReadSetCheckGprsNetworkTimingResponse)]
pub struct ReadSetCheckGprsNetworkTiming {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPDPDP", NoResponse)]
pub struct WriteSetCheckGprsNetworkTiming {
    #[at_arg(position = 0)]
    pub mode: Enabled,
    #[at_arg(position = 1)]
    pub interval: Interval,
    #[at_arg(position = 2)]
    pub timer: Timer,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPMODE=?", TestSelectTcpipApplicationModeResponse)]
pub struct TestSelectTcpipApplicationMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPMODE?", ReadSelectTcpipApplicationModeResponse)]
pub struct ReadSelectTcpipApplicationMode {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPMODE", NoResponse)]
pub struct WriteSelectTcpipApplicationMode {
    #[at_arg(position = 0)]
    pub mode: TcpipApplicationMode,
}

// TODO: 8.2.23 AT+CIPCCFG Configure Transparent Transfer Mode

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSHOWTP=?", TestEnabledResponse)]
pub struct TestDisplayTransferProtocolInIpHead {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSHOWTP?", ReadEnabledResponse)]
pub struct ReadDisplayTransferProtocolInIpHead {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSHOWTP", NoResponse)]
pub struct WriteDisplayTransferProtocolInIpHead {
    #[at_arg(position = 0)]
    pub mode: Enabled,
}

// TODO: 8.2.25 AT+CIPUDPMODE UDP Extended Mode
// TODO: 8.2.26 AT+CIPRXGET Get Data from Network Manually
// TODO: 8.2.27 AT+CIPSCONT Save TCPIP Application Context

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPRDTIMER=?", TestSetRemoteDelayTimerResponse)]
pub struct TestSetRemoteDelayTimer {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPRDTIMER?", ReadSetRemoteDelayTimerResponse)]
pub struct ReadSetRemoteDelayTimer {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPRDTIMER", NoResponse)]
pub struct WriteSetRemoteDelayTimer {
    #[at_arg(position = 0)]
    pub single: TimerSingle,
    #[at_arg(position = 1)]
    pub multi: TimerMulti,
}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSGTXT=?", TestSelectGprsPdpContextResponse)]
pub struct TestSelectGprsPdpContext {}

#[derive(Clone, AtatCmd)]
#[at_cmd("+CIPSGTXT", NoResponse)]
pub struct WriteSelectGprsPdpContext {
    #[at_arg(position = 0)]
    pub mode: GprsPdpContext,
}
