use crate::amf0::errors::{self, Amf0WriteError};
use crate::chunk::errors::UnpackError;
use crate::messages::errors::MessageError;
use crate::netconnection::errors::NetConnectionError;
use crate::netstream::errors::NetStreamError;
use crate::protocol_control_messages::errors::ControlMessagesError;
use crate::user_control_messages::errors::EventMessagesError;
use crate::chunk::errors::PackError;
use crate::handshake::errors::HandshakeError;

use liverust_lib::netio::bytes_errors::BytesWriteError;
use liverust_lib::netio::netio_errors::NetIOError;

use tokio::time::Elapsed;



// pub struct ServerError {
//     pub value: ServerErrorValue,
// }

// pub enum ServerErrorValue {
//     Amf0WriteError(Amf0WriteError),
//     BytesWriteError(BytesWriteError),
//     TimeoutError(Elapsed),
//     UnPackError(UnpackError),
//     MessageError(MessageError),
//     ControlMessagesError(ControlMessagesError),
//     NetConnectionError(NetConnectionError),
//     NetStreamError(NetStreamError),
//     EventMessagesError(EventMessagesError),
//     NetIOError(NetIOError),
//     PackError(PackError),
//     Amf0ValueCountNotCorrect,
//     Amf0ValueTypeNotCorrect,
// }

// impl From<Amf0WriteError> for ServerError {
//     fn from(error: Amf0WriteError) -> Self {
//         ServerError {
//             value: ServerErrorValue::Amf0WriteError(error),
//         }
//     }
// }

// impl From<BytesWriteError> for ServerError {
//     fn from(error: BytesWriteError) -> Self {
//         ServerError {
//             value: ServerErrorValue::BytesWriteError(error),
//         }
//     }
// }

// impl From<Elapsed> for ServerError {
//     fn from(error: Elapsed) -> Self {
//         ServerError {
//             value: ServerErrorValue::TimeoutError(error),
//         }
//     }
// }

// impl From<UnpackError> for ServerError {
//     fn from(error: UnpackError) -> Self {
//         ServerError {
//             value: ServerErrorValue::UnPackError(error),
//         }
//     }
// }

// impl From<MessageError> for ServerError {
//     fn from(error: MessageError) -> Self {
//         ServerError {
//             value: ServerErrorValue::MessageError(error),
//         }
//     }
// }

// impl From<ControlMessagesError> for ServerError {
//     fn from(error: ControlMessagesError) -> Self {
//         ServerError {
//             value: ServerErrorValue::ControlMessagesError(error),
//         }
//     }
// }

// impl From<NetConnectionError> for ServerError {
//     fn from(error: NetConnectionError) -> Self {
//         ServerError {
//             value: ServerErrorValue::NetConnectionError(error),
//         }
//     }
// }

// impl From<NetStreamError> for ServerError {
//     fn from(error: NetStreamError) -> Self {
//         ServerError {
//             value: ServerErrorValue::NetStreamError(error),
//         }
//     }
// }

// impl From<EventMessagesError> for ServerError {
//     fn from(error: EventMessagesError) -> Self {
//         ServerError {
//             value: ServerErrorValue::EventMessagesError(error),
//         }
//     }
// }

// impl From<NetIOError> for ServerError {
//     fn from(error: NetIOError) -> Self {
//         ServerError {
//             value: ServerErrorValue::NetIOError(error),
//         }
//     }
// }

pub struct SessionError {
    pub value: SessionErrorValue,
}

pub enum SessionErrorValue {
    Amf0WriteError(Amf0WriteError),
    BytesWriteError(BytesWriteError),
    TimeoutError(Elapsed),
    UnPackError(UnpackError),
    MessageError(MessageError),
    ControlMessagesError(ControlMessagesError),
    NetConnectionError(NetConnectionError),
    NetStreamError(NetStreamError),
    EventMessagesError(EventMessagesError),
    NetIOError(NetIOError),
    PackError(PackError),
    HandshakeError(HandshakeError),

    Amf0ValueCountNotCorrect,
    Amf0ValueTypeNotCorrect,
}

impl From<Amf0WriteError> for SessionError {
    fn from(error: Amf0WriteError) -> Self {
        SessionError {
            value: SessionErrorValue::Amf0WriteError(error),
        }
    }
}

impl From<BytesWriteError> for SessionError {
    fn from(error: BytesWriteError) -> Self {
        SessionError {
            value: SessionErrorValue::BytesWriteError(error),
        }
    }
}

impl From<Elapsed> for SessionError {
    fn from(error: Elapsed) -> Self {
        SessionError {
            value: SessionErrorValue::TimeoutError(error),
        }
    }
}

impl From<UnpackError> for SessionError {
    fn from(error: UnpackError) -> Self {
        SessionError {
            value: SessionErrorValue::UnPackError(error),
        }
    }
}

impl From<MessageError> for SessionError {
    fn from(error: MessageError) -> Self {
        SessionError {
            value: SessionErrorValue::MessageError(error),
        }
    }
}

impl From<ControlMessagesError> for SessionError {
    fn from(error: ControlMessagesError) -> Self {
        SessionError {
            value: SessionErrorValue::ControlMessagesError(error),
        }
    }
}

impl From<NetConnectionError> for SessionError {
    fn from(error: NetConnectionError) -> Self {
        SessionError {
            value: SessionErrorValue::NetConnectionError(error),
        }
    }
}

impl From<NetStreamError> for SessionError {
    fn from(error: NetStreamError) -> Self {
        SessionError {
            value: SessionErrorValue::NetStreamError(error),
        }
    }
}

impl From<EventMessagesError> for SessionError {
    fn from(error: EventMessagesError) -> Self {
        SessionError {
            value: SessionErrorValue::EventMessagesError(error),
        }
    }
}

impl From<NetIOError> for SessionError {
    fn from(error: NetIOError) -> Self {
        SessionError {
            value: SessionErrorValue::NetIOError(error),
        }
    }
}

impl From<PackError> for SessionError {
    fn from(error: PackError) -> Self {
        SessionError {
            value: SessionErrorValue::PackError(error),
        }
    }
}

impl From<HandshakeError> for SessionError {
    fn from(error: HandshakeError) -> Self {
        SessionError {
            value: SessionErrorValue::HandshakeError(error),
        }
    }
}

