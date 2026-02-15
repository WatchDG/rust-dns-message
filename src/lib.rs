pub mod header;
mod message;
mod question;
pub mod refs;
pub mod resource_record;

pub use header::Header;
pub use message::Message;
pub use question::{Label, QClass, QName, QNameKind, QType, Question};
use resource_record::ResourceRecord;

pub type Answer<'a> = ResourceRecord<'a>;
pub type Authority<'a> = ResourceRecord<'a>;
pub type Additional<'a> = ResourceRecord<'a>;
