#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerIdentifierKind<'id> {
    Global(&'id GlobalId),
    Local(&'id LocalId),
    Temp(&'id TempId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub u32);

impl GlobalId {
    pub fn as_identifier(&self) -> PointerIdentifierKind<'_> {
        PointerIdentifierKind::Global(self)
    }
}

impl LocalId {
    pub fn as_identifier(&self) -> PointerIdentifierKind<'_> {
        PointerIdentifierKind::Local(self)
    }
}

impl TempId {
    pub fn as_identifier(&self) -> PointerIdentifierKind<'_> {
        PointerIdentifierKind::Temp(self)
    }
}

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

impl From<&u32> for TempId {
    fn from(value: &u32) -> Self {
        Self(*value)
    }
}

impl From<&u32> for GlobalId {
    fn from(value: &u32) -> Self {
        Self(*value)
    }
}

impl From<&u32> for LocalId {
    fn from(value: &u32) -> Self {
        Self(*value)
    }
}