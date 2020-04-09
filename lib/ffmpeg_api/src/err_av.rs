use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum AvInternalError {
    #[error("Bitstream filter not found")]
    BitstreamFilterNotFound = 0x465342F8,
    #[error("Internal bug, also see AVERROR_BUG2")]
    Bug = 0x21475542,
    #[error("Internal bug, also see AVERROR_BUG")]
    Bug2 = 0x20475542,
    #[error("Buffer too small")]
    BufferTooSmall = 0x53465542,
    #[error("Decoder not found")]
    DecoderNotFound = 0x434544F8,
    #[error("Demuxer not found")]
    DemuxerNotFound = 0x4D4544F8,
    #[error("Encoder not found")]
    EncoderNotFound = 0x434E45F8,
    #[error("End of file")]
    EndOfFile = 0x20464F45,
    #[error("Immediate exit was requested; the called function should not be restarted")]
    Exit = 0x54495845,
    #[error("Generic error in an external library")]
    External = 0x20545845,
    #[error("Filter not found")]
    FilterNotFound = 0x4C4946F8,
    #[error("Input changed between calls. Reconfiguration is required.")]
    InputChanged = 0x636e6701,
    #[error("Invalid data found when processing input")]
    InvalidData = 0x41444E49,
    #[error("Muxer not found")]
    MuxerNotFound = 0x58554DF8,
    #[error("Option not found")]
    OptionNotFound = 0x54504FF8,
    #[error("Output changed between calls. Reconfiguration is required.")]
    OutputChanged = 0x636e6702,
    #[error("Not yet implemented in FFmpeg, patches welcome")]
    NotImplemented = 0x45574150,
    #[error("Protocol not found")]
    ProtocolNotFound = 0x4F5250F8,
    #[error("Stream not found")]
    StreamNotFound = 0x525453F8,
    #[error("Unknown error, typically from an external library")]
    Unknown = 0x4E4B4E55,
    #[error("Requested feature is flagged experimental. Set strict_std_compliance if you really want to use it.")]
    Experimental = 0x2bb2afa8,
    #[error("Input and output changed between calls. Reconfiguration is required.")]
    InputAndOutputChanged = 0x636e6703,
    #[error("HTTP: Bad Request")]
    HttpBadRequest = 0x303034F8,
    #[error("HTTP: Unauthorized")]
    HttpUnauthorized = 0x313034F8,
    #[error("HTTP: Forbidden")]
    HttpForbidden = 0x333034F8,
    #[error("HTTP: Not Found")]
    HttpNotFound = 0x343034F8,
    #[error("HTTP: Other 4xx error")]
    HttpOther4xx = 0x585834F8,
    #[error("HTTP: Other 5xx error")]
    HttpServerError = 0x585835F8,
}