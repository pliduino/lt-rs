#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum Event {
    None = 0,
    Completed,
    Started,
    Stopped,
    Paused,
}

pub struct PieceIndex(i32);

impl PieceIndex {
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    pub fn to_inner(self) -> i32 {
        self.0
    }
}

impl Event {
    pub(crate) fn from_u8(value: u8) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(feature = "safe_enums")] {
                value.into()
            } else {
                unsafe { std::mem::transmute(value) }
            }
        }
    }
}
