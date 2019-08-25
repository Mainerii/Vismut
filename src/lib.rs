#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Connection {
    pub src: (u32, u16),
    pub dst: (u32, u16),
}
