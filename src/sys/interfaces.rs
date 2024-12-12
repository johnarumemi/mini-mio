pub trait SysSelectors {
    fn register(&self, token: Token, event: impl SysEvent, interests: Interests) -> io::Result<()>;
}

pub trait SysEvent {}
