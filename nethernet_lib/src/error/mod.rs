use std::io;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum NethernetError {
    
    #[error("WebRTC error: {0}")]
    WebRtc(#[from] webrtc::Error),

    
    #[error("ICE error: {0}")]
    Ice(String),

    
    #[error("DTLS error: {0}")]
    Dtls(String),

    
    #[error("SCTP error: {0}")]
    Sctp(String),

    
    #[error("Signaling error: {0}")]
    Signaling(#[from] SignalingError),

    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    
    #[error("Connection closed")]
    ConnectionClosed,

    
    #[error("Data channel error: {0}")]
    DataChannel(String),

    
    #[error("Message parse error: {0}")]
    MessageParse(String),

    
    #[error("Message too large: exceeds maximum size of {0} bytes")]
    MessageTooLarge(usize),

    
    #[error("Operation timed out")]
    Timeout,

    
    #[error("Invalid state: {0}")]
    InvalidState(String),

    
    #[error("{0}")]
    Other(String),
}


#[derive(Debug, Error)]
pub enum SignalingError {
    
    #[error("Failed to send signal: {0}")]
    SendFailed(String),

    
    #[error("Failed to receive signal: {0}")]
    ReceiveFailed(String),

    
    #[error("Invalid signal: {0}")]
    InvalidSignal(String),

    
    #[error("Signaling stopped")]
    Stopped,

    
    #[error("Network ID not found: {0}")]
    NetworkIdNotFound(u64),

    
    #[error("Credential error: {0}")]
    CredentialError(String),

    
    #[error("Parse error: {0}")]
    ParseError(String),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SignalErrorCode {
    None = 0,
    DestinationNotLoggedIn = 1,
    NegotiationTimeout = 2,
    WrongTransportVersion = 3,
    FailedToCreatePeerConnection = 4,
    Ice = 5,
    ConnectRequest = 6,
    ConnectResponse = 7,
    CandidateAdd = 8,
    InactivityTimeout = 9,
    FailedToCreateOffer = 10,
    FailedToCreateAnswer = 11,
    FailedToSetLocalDescription = 12,
    FailedToSetRemoteDescription = 13,
    NegotiationTimeoutWaitingForResponse = 14,
    NegotiationTimeoutWaitingForAccept = 15,
    IncomingConnectionIgnored = 16,
    SignalingParsingFailure = 17,
    SignalingUnknownError = 18,
    SignalingUnicastMessageDeliveryFailed = 19,
    SignalingBroadcastDeliveryFailed = 20,
    SignalingMessageDeliveryFailed = 21,
    SignalingTurnAuthFailed = 22,
    SignalingFallbackToBestEffortDelivery = 23,
    NoSignalingChannel = 24,
    NotLoggedIn = 25,
    SignalingFailedToSend = 26,
}

impl From<u32> for SignalErrorCode {
    fn from(code: u32) -> Self {
        match code {
            0 => SignalErrorCode::None,
            1 => SignalErrorCode::DestinationNotLoggedIn,
            2 => SignalErrorCode::NegotiationTimeout,
            3 => SignalErrorCode::WrongTransportVersion,
            4 => SignalErrorCode::FailedToCreatePeerConnection,
            5 => SignalErrorCode::Ice,
            6 => SignalErrorCode::ConnectRequest,
            7 => SignalErrorCode::ConnectResponse,
            8 => SignalErrorCode::CandidateAdd,
            9 => SignalErrorCode::InactivityTimeout,
            10 => SignalErrorCode::FailedToCreateOffer,
            11 => SignalErrorCode::FailedToCreateAnswer,
            12 => SignalErrorCode::FailedToSetLocalDescription,
            13 => SignalErrorCode::FailedToSetRemoteDescription,
            14 => SignalErrorCode::NegotiationTimeoutWaitingForResponse,
            15 => SignalErrorCode::NegotiationTimeoutWaitingForAccept,
            16 => SignalErrorCode::IncomingConnectionIgnored,
            17 => SignalErrorCode::SignalingParsingFailure,
            18 => SignalErrorCode::SignalingUnknownError,
            19 => SignalErrorCode::SignalingUnicastMessageDeliveryFailed,
            20 => SignalErrorCode::SignalingBroadcastDeliveryFailed,
            21 => SignalErrorCode::SignalingMessageDeliveryFailed,
            22 => SignalErrorCode::SignalingTurnAuthFailed,
            23 => SignalErrorCode::SignalingFallbackToBestEffortDelivery,
            24 => SignalErrorCode::NoSignalingChannel,
            25 => SignalErrorCode::NotLoggedIn,
            26 => SignalErrorCode::SignalingFailedToSend,
            _ => SignalErrorCode::SignalingUnknownError,
        }
    }
}

impl From<SignalErrorCode> for u32 {
    fn from(code: SignalErrorCode) -> Self {
        code as u32
    }
}

pub type Result<T> = std::result::Result<T, NethernetError>;
