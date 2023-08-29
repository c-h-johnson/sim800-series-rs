use atat::atat_derive::AtatResp;
use heapless::String;
use heapless_bytes::Bytes;

// TODO: add all optional parameters ([,<tooa>,<fo>,<pid>,<dcs>,<sca>,<tosca>,<length>])
#[derive(AtatResp, Debug, Clone, Eq, PartialEq)]
pub struct MessageReceivedUrc {
    #[at_arg(position = 0)]
    pub sender: String<16>,
    #[at_arg(position = 1)]
    pub sender_name: String<32>,
    // TODO: make this into date/time type
    #[at_arg(position = 2)]
    pub time: String<32>,
    #[at_arg(position = 3)]
    pub message: Bytes<256>,
}
