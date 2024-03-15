use axum::extract::ws::Message;
use std::{any::Any, error::Error};

#[derive(Debug)]
pub enum WsMessageError {
    WsError,
}

impl std::fmt::Display for WsMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error with ws message")
    }
}

impl From<axum::Error> for WsMessageError {
    fn from(value: axum::Error) -> Self {
        WsMessageError::WsError
    }
}

impl Error for WsMessageError {}

#[derive(Debug)]
pub enum WsMessage {
    KeyUp,
    KeyDown,
    Hello,
}

impl std::fmt::Display for WsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyUp => f.write_str("Message(k)"),
            Self::KeyDown => f.write_str("Message(j)"),
            Self::Hello => f.write_str("Message(hello)"),
            _ => f.write_str("Unkonwn message type"),
        }
    }
}

impl From<Message> for WsMessage {
    fn from(val: Message) -> Self {
        match val
            .into_text()
            .expect("Error returning text for message")
            .as_str()
        {
            "k" => WsMessage::KeyUp,
            "j" => WsMessage::KeyDown,
            "hello" => WsMessage::Hello,
            t => panic!("Error turning message into WsMessage for {}", t),
        }
    }
}
