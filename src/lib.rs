#![no_std]
#![feature(variant_count)]

pub mod command;
mod error;

use core::marker::PhantomData;

use atat::atat_derive::AtatUrc;
use atat::blocking::AtatClient;
use atat::{self, AtatCmd};
use heapless::String;

use command::sms::data::SmsRecipient;
use command::sms::{
    response::WriteSendSmsMessageResponse, urc::MessageReceivedUrc, WriteSendSmsMessage1,
    WriteSendSmsMessage2,
};
pub use error::{Error, Result};

// TODO add params
// TODO make sure these don't belong in the gsm module
// TODO requires https://github.com/rust-bakery/nom/pull/1556 to include all commands
#[derive(Debug, PartialEq, Clone, AtatUrc)]
pub enum Urc {
    /// Indication of a call that is currently waiting and can be accepted
    #[at_urc("+CCWA")]
    CallWaiting,
    /// The calling line identity (CLI) of the calling party when receiving a mobile terminated call
    #[at_urc("+CLIP")]
    Cli,
    /// Indicates incoming call to the TE if extended format is enabled
    #[at_urc("+CRING")]
    Ring,
    // TODO: currectly this conflicts with the response of AT+CREG?
    /// There is a change in the MT network registration status or a change of the network cell
    // #[at_urc("+CREG")]
    // RegistrationStatus,
    // /// Shortly before the ACM (Accumulated Call Meter) maximum value is reached
    // #[at_urc("+CCWV")]
    // AcmMaxed,
    /// Indicates that new message has been received
    #[at_urc("+CMTI")]
    MessageReceivedIndex,
    // TODO: currectly this conflicts with the response of AT+CMTE?
    /// Indicates that new message has been received (PDU/text)
    #[at_urc("+CMT")]
    MessageReceived(MessageReceivedUrc),
    /// Indicates that new cell broadcast message has been received (PDU/text)
    #[at_urc("+CBM")]
    CellBroadcastMessageRecieved,
    /// Indicates that new SMS status report has been received (PDU/text)
    #[at_urc("+CDS")]
    SmsStatusReportReceived,
    /// The presentation of the COL (Connected Line) at the TE for a mobile originated call
    #[at_urc("+COLP")]
    ColPresentation,
    /// Presentation status during a mobile terminated call setup or during a call, or when a forward check supplementary service notification is received
    #[at_urc("+CSSU")]
    PresentationStatusDuringCall,
    /// Presentation status after a mobile originated call setup
    #[at_urc("+CSSI")]
    PresentationStatusAfterCall,
    /// Report a list of current calls of ME automatically when the current call status changes
    #[at_urc("+CLCC")]
    CurrentCalls,
    /// Refresh network name by network
    #[at_urc("*PSNWID")]
    RefreshNetworkName,
    /// Refresh time and time zone by network
    #[at_urc("*PSUTTZ")]
    RefreshNetworkTime,
    /// Refresh network time zone by network
    #[at_urc("+CTZV")]
    RefreshNetworkTimeZone,
    /// Refresh Network Daylight Saving Time by network
    #[at_urc("+DST")]
    RefreshNetworkDst,
    /// Indicates whether SIM card has been inserted
    // #[at_urc("+CSMINS")]
    // SimCardInserted,
    /// Indicates whether a CS voice call, CS data has been terminated
    #[at_urc("+CDRIND")]
    CsTerminated,
    // /// Indicates the current channel
    // #[at_urc("+CHF")]
    // CurrentChannel,
    // /// Report of network information
    // #[at_urc("+CENG")]
    // NetworkInformation,
    /// Shows call state of mobile originated call: the call is alerted
    #[at_urc("MO RING")]
    CallAlerted,
    /// Shows call state of mobile originated call: the call is established
    #[at_urc("MO CONNECTED")]
    CallEstablished,
    // /// Indicates whether some password is required or not
    // #[at_urc("+CPIN")]
    // PasswordRequired,
    // /// Displays signal strength and channel bit error rate when <rssi>,<ber>values change
    // #[at_urc("+CSQN")]
    // SignalStrength,
    // /// The generated tone playing is stopped or completed
    // #[at_urc("+SIMTONE")]
    // SimTone,
    // /// The SIM Toolkit tone playing is stopped or completed
    // #[at_urc("+STTONE")]
    // StPlayingStopped,
    // /// An intermediate result code is transmitted during connect negotiation when the TA has determined the speed and quality of service to be used, before any error control or data compression reports are transmitted, and before any final result code (e.g. CONNECT) appears
    // #[at_urc("+CR")]
    // IntermediateResultCode,
    // /// Indicates an USSD response from the network, or network initiated operation
    // #[at_urc("+CUSD")]
    // UssdResponse,
    /// An incoming call signal from network is detected
    #[at_urc("RING")]
    IncomingCall,
    // /// SIM800 is powered down by the PWRKEY pin or AT command 'AT+CPOWD=1'
    // #[at_urc("NORMAL POWER DOWN")]
    // NormalPowerDown,
    // /// The module temperature is abnormal
    // #[at_urc("+CMTE")]
    // AbnormalTemperature,
    // /// Under-voltage automatic power down
    // #[at_urc("UNDER-VOLTAGE POWER DOWN")]
    // UnderVoltagePowerDown,
    // /// Under-voltage warning
    // #[at_urc("UNDER-VOLTAGE WARNNING")] // TODO: double N?
    // UnderVoltageWarning,
    // /// Over-voltage automatic power down
    // #[at_urc("OVER-VOLTAGE POWER DOWN")]
    // OverVoltagePowerDown,
    // /// Over-voltage warning
    // #[at_urc("OVER-VOLTAGE WARNNING")] // TODO: double N?
    // OverVoltageWarning,
    // /// The module is charging by charger (requires hardware support)
    // #[at_urc("CHARGE-ONLY MODE")]
    // ChargeOnlyMode,
    // /// Power on procedure is completed, and the module is ready to operate at fixed baud rate (This URC does not appear when auto-bauding function is active)
    // #[at_urc("RDY")]
    // Ready,
    /// Module is powered on and phonebook initialization procedure is over
    #[at_urc("Call Ready")]
    CallReady,
    /// This is not documented but does appear after `CallReady`
    #[at_urc("SMS Ready")]
    SmsReady,
    // /// Phone functionality indication (This URC does not appear when auto-bauding function is active)
    // #[at_urc("+CFUN")]
    // PhoneFunctionality,
    // /// TCP/ UDP connection is successful
    // #[at_urc("CONNECT OK")]
    // ConnectionSuccess,
    // /// TCP/UDP connection in channel mode is successful
    // #[at_urc("CONNECT")]
    // ChannelConnectionSuccess,
    // /// TCP/UDP connection fails
    // #[at_urc("CONNECT FAIL")]
    // ConnectionFail,
    // /// TCP/UDP connection exists
    // #[at_urc("ALREADY CONNECT")]
    // ConnectionExists,
    // /// Data sending is successful
    // #[at_urc("SEND OK")]
    // DataSendSuccess,
    // /// TCP/UDP connection is closed
    // #[at_urc("CLOSED")]
    // ConnectionClosed,
    // /// Shows remote IP address and port (only in single connection mode)
    // #[at_urc("RECV FROM")]
    // ReceiveFrom,
    // /// Display transfer protocol in IP header to received data or not (only in single connection mode)
    // #[at_urc("+IPD")]
    // DisplayTransferProtocol,
    // /// Received data from remote client (only in multiple connection mode)
    // #[at_urc("+RECEIVE")]
    // ReceivedData,
    // /// Remote client connected in
    // #[at_urc("REMOTE IP")]
    // RemoteIp,
    // /// DNS successful/failed
    // #[at_urc("+CDNSGIP")]
    // DnsSuccess,
    // /// GPRS is disconnected by network
    // #[at_urc("+PDP DEACT")]
    // GprsDisconnected,
    // /// The bearer based on IP connection of SIMCom application is deactivated
    // #[at_urc("+SAPBR")]
    // BearerDeactivated,
    // /// Indicates HTTP method, Status Code responded by remote server and the length of data got
    // #[at_urc("+HTTPACTION")]
    // HttpResponse,
    // /// FTPGET session
    // #[at_urc("+FTPGET")]
    // FtpGet,
    // /// FTP is ready to upload data or returned result
    // #[at_urc("+FTPPUT")]
    // FtpPut,
    // /// FTP delete session
    // #[at_urc("+FTPDELE")]
    // FtpDelete,
    // /// FTP size session
    // #[at_urc("+FTPSIZE")]
    // FtpSize,
    // /// FTP create directory (not supported for all versions)
    // #[at_urc("+FTPMKD")]
    // FtpMakeDirectory,
    // /// FTP delete directory (not supported for all versions)
    // #[at_urc("+FTPRMD")]
    // FtpRemoveDirectory,
    // /// FTP list session (not supported for all versions)
    // #[at_urc("+FTPLIST")]
    // FtpList,
}

pub struct UartWrite<W, Error> {
    uart: W,
    _unused: PhantomData<Error>,
}

impl<W, Error> UartWrite<W, Error> {
    pub fn new(uart: W) -> Self {
        Self {
            uart,
            _unused: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum SmsError {
    SmsError,
}

impl From<nb::Error<core::convert::Infallible>> for SmsError {
    fn from(_value: nb::Error<core::convert::Infallible>) -> Self {
        Self::SmsError
    }
}

impl From<core::convert::Infallible> for SmsError {
    fn from(_value: core::convert::Infallible) -> Self {
        Self::SmsError
    }
}

impl embedded_io::Error for SmsError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<W, Error> embedded_io::ErrorType for UartWrite<W, Error>
where
    Error: embedded_io::Error,
{
    type Error = Error;
}

impl<W, Error> embedded_io::Write for UartWrite<W, Error>
where
    W: embedded_hal::serial::Write<u8>,
    Error: embedded_io::Error
        + core::convert::From<
            nb::Error<<W as embedded_hal::prelude::_embedded_hal_serial_Write<u8>>::Error>,
        > + core::convert::From<<W as embedded_hal::prelude::_embedded_hal_serial_Write<u8>>::Error>,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for i in buf {
            nb::block!(self.uart.write(*i))?
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(nb::block!(self.uart.flush())?)
    }
}

pub struct Sim800<C>
where
    C: AtatClient,
{
    client: C,
}

impl<C> Sim800<C>
where
    C: AtatClient,
{
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn send_sms_message(
        &mut self,
        message: &str,
        recipient: &str,
    ) -> Result<WriteSendSmsMessageResponse, atat::Error> {
        let recipient = SmsRecipient::new_text(recipient);
        let command = WriteSendSmsMessage1 { recipient };
        self.send(command)?;
        let command = WriteSendSmsMessage2 {
            message: String::from(message),
        };
        self.send(command)
    }

    pub fn send<T, const LEN: usize>(&mut self, command: T) -> Result<T::Response, atat::Error>
    where
        T: AtatCmd<LEN>,
    {
        self.client.send(&command)
    }
}
