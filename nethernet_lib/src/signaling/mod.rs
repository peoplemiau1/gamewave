use crate::error::Result;
use crate::protocol::Signal;
use futures::Stream;
use std::pin::Pin;

pub mod lan;



pub trait Signaling: Send + Sync {
    
    fn signal(&self, signal: Signal) -> impl std::future::Future<Output = Result<()>> + Send;

    
    fn signals(&self) -> Pin<Box<dyn Stream<Item = Signal> + Send>>;

    
    fn network_id(&self) -> String;

    
    fn set_pong_data(&self, data: Vec<u8>);
}


pub trait Notifier: Send + Sync {
    
    fn notify_signal(&self, signal: Signal);

    
    fn notify_error(&self, error: crate::error::NethernetError);
}
