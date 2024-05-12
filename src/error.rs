#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
/// Common error for HDLC actions.
pub enum HDLCError {
    /// Catches duplicate special characters.   
    #[cfg_attr(
        feature = "thiserror",
        error("Catches duplicate special characters.   ")
    )]
    DuplicateSpecialChar,
    /// Catches a random sync char in the data.
    #[cfg_attr(
        feature = "thiserror",
        error("Catches a random sync char in the data.")
    )]
    FendCharInData,
    /// Catches a random swap char, `fesc`, in the data with no `tfend` or `tfesc`.
    #[cfg_attr(
        feature = "thiserror",
        error("Catches a random swap char, `fesc`, in the data with no `tfend` or `tfesc`.")
    )]
    MissingTradeChar,
    /// No first fend on the message.    
    #[cfg_attr(feature = "thiserror", error("No first fend on the message.    "))]
    MissingFirstFend,
    /// No final fend on the message.
    #[cfg_attr(feature = "thiserror", error("No final fend on the message."))]
    MissingFinalFend,
    /// Too much data to be converted into a SHDLC frame
    #[cfg_attr(
        feature = "thiserror",
        error("Too much data to be converted into a SHDLC frame")
    )]
    TooMuchData,
    /// Too few data to be converted from a SHDLC frame
    #[cfg_attr(
        feature = "thiserror",
        error("Too few data to be converted from a SHDLC frame")
    )]
    TooFewData,
    /// Checksum for decoded Frame is invalid
    #[cfg_attr(feature = "thiserror", error("Checksum for decoded Frame is invalid"))]
    InvalidChecksum,
    /// More than 259 bytes resulted after decoding SHDLC frame
    #[cfg_attr(
        feature = "thiserror",
        error("More than 259 bytes resulted after decoding SHDLC frame")
    )]
    TooMuchDecodedData,
}
