#[repr(C)]
#[derive(Copy, Clone)]
pub struct k5input {
    pub ptr: *const u8,
    pub len: crate::stddef_h::size_t,
    pub status: crate::stdlib::int32_t,
}
