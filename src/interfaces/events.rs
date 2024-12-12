use crate::interests::Interest;
use crate::interfaces::Token;

pub struct Event {
    pub token: Token,
    pub flags: Interest,
}
