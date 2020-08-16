use std::convert::TryInto;

/// Compile-time Type safe conversion from usize.
pub trait FromUsize: Sized {
    fn from_usize(value: usize) -> Self;
}

// usize can be converted to u16, u32, or u64
#[cfg(target_pointer_width = "16")]
impl FromUsize for u16 {
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "16")]
impl FromUsize for u32 {
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "16")]
impl FromUsize for u64 {
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}

// usize can be converted u32 or u64
#[cfg(target_pointer_width = "32")]
impl FromUsize for u32 {
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "32")]
impl FromUsize for u64 {
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}

// Usize can be converted into a u64.
#[cfg(target_pointer_width = "64")]
impl FromUsize for u64 {
    fn from_usize(value: usize) -> Self {
        value.try_into().unwrap()
    }
}

impl FromUsize for usize {
    fn from_usize(value: usize) -> Self {
        value
    }
}

/// Compile-time type safe conversion into usize
pub trait IntoUsize: Sized {
    fn into_usize(self) -> usize;
}

#[cfg(target_pointer_width = "16")]
impl IntoUsize for u8 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "16")]
impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}

#[cfg(target_pointer_width = "32")]
impl IntoUsize for u8 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "32")]
impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "32")]
impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}

#[cfg(target_pointer_width = "64")]
impl IntoUsize for u8 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "64")]
impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "64")]
impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}
#[cfg(target_pointer_width = "64")]
impl IntoUsize for u64 {
    fn into_usize(self) -> usize {
        self.try_into().unwrap()
    }
}

impl IntoUsize for usize {
    fn into_usize(self) -> usize {
        self
    }
}
