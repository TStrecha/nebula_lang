#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(u32);

impl From<u32> for TempId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u32> for GlobalId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u32> for LocalId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
