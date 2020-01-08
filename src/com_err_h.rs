extern "C" {
    #[no_mangle]
    pub fn error_message(_: crate::com_err_h::errcode_t) -> *const i8;
    /*@modifies internalState@*/
    #[no_mangle]
    pub fn add_error_table(_: *const crate::com_err_h::error_table) -> crate::com_err_h::errcode_t;
    /*@modifies internalState@*/
    #[no_mangle]
    pub fn remove_error_table(
        _: *const crate::com_err_h::error_table,
    ) -> crate::com_err_h::errcode_t;
}
// =============== BEGIN com_err_h ================

/*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */

/* Header file for common error description library. */
pub type errcode_t = isize;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct error_table {
    pub msgs: *const *const i8,
    pub base: isize,
    pub n_msgs: u32,
}
