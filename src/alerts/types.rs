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
