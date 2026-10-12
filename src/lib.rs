use bitcode::{Decode, Encode};
use crossfire::{MAsyncTx, mpsc};
use std::error::Error;

pub struct SystemMessage {
    sender: Option<MAsyncTx<mpsc::Array<Command>>>,
    command: Command,
}

impl SystemMessage {
    pub fn new(command: Command) -> Self {
        Self {
            sender: None,
            command,
        }
    }

    pub fn with_sender(mut self, sender: MAsyncTx<mpsc::Array<Command>>) -> Self {
        self.sender = Some(sender);
        self
    }

    pub fn sender(&self) -> &Option<MAsyncTx<mpsc::Array<Command>>> {
        &self.sender
    }

    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn into_parts(self) -> (Option<MAsyncTx<mpsc::Array<Command>>>, Command) {
        (self.sender, self.command)
    }
}

#[derive(Debug, Encode, Decode)]
pub enum Command {
    Publish(PublishCommand),
    Subscribe(SubscribeCommand),
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct PublishCommand {
    topic: String,
    data: Vec<u8>,
}

impl PublishCommand {
    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug, Encode, Decode)]
pub struct SubscribeCommand {
    topic: String,
}

impl SubscribeCommand {
    pub fn topic(&self) -> &str {
        &self.topic
    }
}

#[derive(Debug, Encode, Decode)]
pub struct SubscriptionEndpoint {
    address: String,
    port: u16,
}

impl SubscriptionEndpoint {
    pub fn new(address: &str, port: u16) -> Self {
        Self {
            address: address.into(),
            port,
        }
    }
}

#[derive(Debug)]
pub struct HotlineError(pub String);

impl std::fmt::Display for HotlineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal error: {}", self.0)
    }
}

impl Error for HotlineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
