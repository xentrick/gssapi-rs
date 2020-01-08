extern "C" {
    #[no_mangle]
    pub fn k5_bcmp(
        p1: *const libc::c_void,
        p2: *const libc::c_void,
        n: crate::stddef_h::size_t,
    ) -> i32;
}
// =============== BEGIN k5_platform_h ================
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub i: crate::stdlib::uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k5_init_t {
    pub once: crate::k5_thread_h::k5_once_t,
    pub error: i32,
    pub did_run: i32,
    pub fn_0: Option<unsafe extern "C" fn() -> ()>,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub i: crate::stdlib::uint16_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub i: crate::stdlib::uint32_t,
}
