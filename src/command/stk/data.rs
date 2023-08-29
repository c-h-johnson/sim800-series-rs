use atat::atat_derive::AtatEnum;

#[derive(AtatEnum, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum StkCommand {
    StkSetup = 0,
    StkSetupNoIcon = 4,
    SessionTerminated = 16,
    NoResponse = 18,
    UnableToProcessCommand = 32,
    RejectSetupCall = 34,
    NotUnderstood = 50,
}
