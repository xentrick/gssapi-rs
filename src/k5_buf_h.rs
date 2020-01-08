extern "C" {
    #[no_mangle]
    pub fn k5_buf_truncate(buf: *mut crate::k5_buf_h::k5buf, len: crate::stddef_h::size_t);
    /* Initialize a k5buf using a fixed-sized, existing buffer.  SPACE must be
     * more than zero, or an assertion failure will result. */
    /* Initialize a k5buf using an internally allocated dynamic buffer. */
    /* Initialize a k5buf using an internally allocated dynamic buffer, zeroing
     * memory when reallocating or freeing. */
    /* Add a C string to BUF. */
    /* Add a counted series of bytes to BUF. */
    /* Add sprintf-style formatted data to BUF. */
    /* Add sprintf-style formatted data to BUF, with a va_list.  The value of ap is
     * undefined after the call. */
    /* Extend the length of buf by len and return a pointer to the reserved space,
     * to be filled in by the caller.  Return NULL on error. */
    /* Truncate BUF.  LEN must be between 0 and the existing buffer
     * length, or an assertion failure will result. */
    /* Return ENOMEM if buf is in an error state, 0 otherwise. */
    /*
     * Free the storage used in the dynamic buffer BUF.  The caller may choose to
     * take responsibility for freeing the data pointer instead of using this
     * function.  If BUF is a fixed buffer, an assertion failure will result.
     * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
     * or a zeroed k5buf structure is a no-op.
     */
    #[no_mangle]
    pub fn k5_buf_free(buf: *mut crate::k5_buf_h::k5buf);

    #[no_mangle]
    pub fn k5_buf_get_space(
        buf: *mut crate::k5_buf_h::k5buf,
        len: crate::stddef_h::size_t,
    ) -> *mut libc::c_void;

    #[no_mangle]
    pub fn k5_buf_status(buf: *mut crate::k5_buf_h::k5buf) -> i32;

    #[no_mangle]
    pub fn k5_buf_add_fmt(buf: *mut crate::k5_buf_h::k5buf, fmt: *const i8, _: ...);

    #[no_mangle]
    pub fn k5_buf_add_len(
        buf: *mut crate::k5_buf_h::k5buf,
        data: *const libc::c_void,
        len: crate::stddef_h::size_t,
    );

    #[no_mangle]
    pub fn k5_buf_add(buf: *mut crate::k5_buf_h::k5buf, data: *const i8);

    #[no_mangle]
    pub fn k5_buf_init_dynamic(buf: *mut crate::k5_buf_h::k5buf);
}
// =============== BEGIN k5_buf_h ================
pub type k5buftype = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k5buf {
    pub buftype: crate::k5_buf_h::k5buftype,
    pub data: *mut libc::c_void,
    pub space: crate::stddef_h::size_t,
    pub len: crate::stddef_h::size_t,
}
pub const K5BUF_DYNAMIC_ZAP: crate::k5_buf_h::k5buftype = 3;
pub const K5BUF_DYNAMIC: crate::k5_buf_h::k5buftype = 2;
pub const K5BUF_FIXED: crate::k5_buf_h::k5buftype = 1;
pub const K5BUF_ERROR: crate::k5_buf_h::k5buftype = 0;
