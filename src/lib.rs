pub mod header;
mod message;
mod question;
pub mod resource_record;
pub mod traits;

pub use header::Header;
pub use message::Message;
pub use question::{QClass, QType, Question};
use resource_record::ResourceRecord;

pub type Answer<'a> = ResourceRecord<'a>;
pub type Authority<'a> = ResourceRecord<'a>;
pub type Additional<'a> = ResourceRecord<'a>;
