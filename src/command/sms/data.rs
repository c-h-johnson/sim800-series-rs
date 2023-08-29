use atat::{atat_derive::AtatEnum, AtatLen};
use convert_by_name::ConvertByName;
use enumscribe::{EnumDeserialize, EnumSerialize};
use heapless::String;
use serde::Serialize;

// FIXME work out what this should be
pub const FORMATTED_SMS_RECIPIENT_LEN: usize = 256;

pub const SMS_MESSAGE_TEXT_LEN: usize = 160;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DeleteFlag {
    DeleteIndex = 0,
    DeleteReadFromPreferredStorage = 1,
    DeleteReadFromPreferredStorageAndSent = 2,
    DeleteAllRead = 3,
    DeleteAll = 4,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SmsMessageFormat {
    Pdu = 0,
    Text = 1,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum ListSmsMessagesStat {
    ReceivedUnread = 0,
    ReceivedRead = 1,
    StoredUnsent = 2,
    StoredSent = 3,
    All = 4,
}

#[derive(EnumSerialize, EnumDeserialize, Clone, PartialEq, Eq, Debug, ConvertByName)]
#[from(ListSmsMessagesStat)]
#[into(ListSmsMessagesStat)]
pub enum ListSmsMessagesTextStat {
    #[enumscribe(str = "REC UNREAD")]
    ReceivedUnread,
    #[enumscribe(str = "REC READ")]
    ReceivedRead,
    #[enumscribe(str = "STO UNSENT")]
    StoredUnsent,
    #[enumscribe(str = "STO SENT")]
    StoredSent,
    #[enumscribe(str = "ALL")]
    All,
}

impl AtatLen for ListSmsMessagesTextStat {
    const LEN: usize = "REC UNREAD".len();
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SmsMessageStatusMode {
    Normal = 0,
    NoChangeStatus = 1,
}

#[derive(Clone, Serialize, Debug)]
pub struct SmsRecipient {
    data: String<FORMATTED_SMS_RECIPIENT_LEN>,
}

impl SmsRecipient {
    /// recipient must be 10 digits
    ///
    /// country code must be 2 digits
    pub fn new_text(recipient: &str) -> Self {
        let data = String::from(recipient);

        Self { data }
    }

    // pub fn new_pdu(message: &str, recipient: &str, country_code: &str) -> Self {
    //     let data = String::new();

    //     // TODO https://www.gsmfavorites.com/documents/sms/pdutext/

    //     Self { data }
    // }
}

impl AtatLen for SmsRecipient {
    const LEN: usize = FORMATTED_SMS_RECIPIENT_LEN;
}

pub type SmsMessage = String<SMS_MESSAGE_TEXT_LEN>;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NewSmsMessageIndicationsMode {
    BufferUrcInTa = 0,
    DiscardWhenLinkReserved = 1,
    BufferUrcInTaWhenLinkReserved = 2,
    ForwardUrcToTa = 3,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NewSmsMessageIndicationsDeliver {
    NoIndications = 0,
    IndicateMemoryLocation = 1,
    DirectToTe = 2,
    ChoosePerClass = 3,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NewSmsMessageIndicationsCbm {
    NoIndications = 0,
    DirectToTe = 2,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NewSmsMessageIndicationsStatusReports {
    NoIndications = 0,
    DirectToTe = 1,
}

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum NewSmsMessageIndicationsBuffer {
    FlushToTe = 0,
    Clear = 1,
}
