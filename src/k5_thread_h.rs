extern "C" {
    #[no_mangle]
    pub fn krb5int_getspecific(_: crate::k5_thread_h::k5_key_t) -> *mut libc::c_void;

    #[no_mangle]
    pub fn krb5int_setspecific(_: crate::k5_thread_h::k5_key_t, _: *mut libc::c_void) -> i32;

    #[no_mangle]
    pub fn k5_os_mutex_destroy(m_0: *mut crate::k5_thread_h::k5_os_mutex) -> i32;

    #[no_mangle]
    pub fn k5_os_mutex_lock(m_0: *mut crate::k5_thread_h::k5_os_mutex) -> i32;

    #[no_mangle]
    pub fn k5_os_mutex_unlock(m_0: *mut crate::k5_thread_h::k5_os_mutex) -> i32;

    #[no_mangle]
    pub fn k5_once(
        once: *mut crate::k5_thread_h::k5_once_t,
        fn_0: Option<unsafe extern "C" fn() -> ()>,
    ) -> i32;
    /* rename shorthand symbols for export */
    #[no_mangle]
    pub fn krb5int_key_register(
        _: crate::k5_thread_h::k5_key_t,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    ) -> i32;

    #[no_mangle]
    pub fn krb5int_key_delete(_: crate::k5_thread_h::k5_key_t) -> i32;

    #[no_mangle]
    pub fn k5_os_mutex_init(m: *mut crate::k5_thread_h::k5_os_mutex) -> i32;
}
// =============== BEGIN k5_thread_h ================

/* Thread-specific data; implemented in a support file, because we'll
need to keep track of some global data for cleanup purposes.

Note that the callback function type is such that the C library
routine free() is a valid callback.  */
pub type k5_key_t = u32;
pub const K5_KEY_MAX: crate::k5_thread_h::k5_key_t = 5;
pub const K5_KEY_GSS_SPNEGO_STATUS: crate::k5_thread_h::k5_key_t = 4;
pub const K5_KEY_GSS_KRB5_ERROR_MESSAGE: crate::k5_thread_h::k5_key_t = 3;
pub const K5_KEY_GSS_KRB5_CCACHE_NAME: crate::k5_thread_h::k5_key_t = 2;
pub const K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME: crate::k5_thread_h::k5_key_t = 1;
pub const K5_KEY_COM_ERR: crate::k5_thread_h::k5_key_t = 0;
pub type k5_os_mutex = crate::stdlib::pthread_mutex_t;
pub type k5_mutex_t = crate::k5_thread_h::k5_os_mutex;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k5_once_t {
    pub o: crate::stdlib::pthread_once_t,
    pub n: crate::k5_thread_h::k5_os_nothread_once_t,
}
pub type k5_os_nothread_once_t = u8;
