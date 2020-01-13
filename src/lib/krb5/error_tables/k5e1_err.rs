use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:39"]
pub mod com_err_h {
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
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct error_table {
        pub msgs: *const *const libc::c_char,
        pub base: libc::c_long,
        pub n_msgs: libc::c_uint,
    }
    extern "C" {
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn add_error_table(_: *const error_table) -> errcode_t;
    }
    /* ! defined(__COM_ERR_H) */
}
pub use self::com_err_h::{errcode_t, error_table, add_error_table};
/* Lclint doesn't handle null annotations on arrays
   properly, so we need this typedef in each
   generated .c file.  */
/*@-redef@*/
#[c2rust::src_loc = "19:1"]
pub type ncptr = *const libc::c_char;
/*@null@*/
/*@=redef@*/
#[c2rust::src_loc = "22:20"]
static mut text: [ncptr; 14] =
    [b"Plugin does not support interface version\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid module specifier\x00" as *const u8 as *const libc::c_char,
     b"Plugin module name not found\x00" as *const u8 as *const libc::c_char,
     b"The KDC should discard this request\x00" as *const u8 as
         *const libc::c_char,
     b"Can\'t create new subsidiary cache\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid keyring anchor name\x00" as *const u8 as *const libc::c_char,
     b"Unknown keyring collection version\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid UID in persistent keyring name\x00" as *const u8 as
         *const libc::c_char,
     b"Malformed reply from KCM daemon\x00" as *const u8 as
         *const libc::c_char,
     b"Mach RPC error communicating with KCM daemon\x00" as *const u8 as
         *const libc::c_char,
     b"KCM daemon reply too big\x00" as *const u8 as *const libc::c_char,
     b"No KCM server found\x00" as *const u8 as *const libc::c_char,
     b"mit-krb5\x00" as *const u8 as *const libc::c_char, 0 as ncptr];
#[no_mangle]
#[c2rust::src_loc = "41:26"]
pub static mut et_k5e1_error_table: error_table =
    unsafe {
        {
            let mut init =
                error_table{msgs: text.as_ptr(),
                            base: -(1750600192 as libc::c_long),
                            n_msgs: 12 as libc::c_int as libc::c_uint,};
            init
        }
    };
/*
 * et-c-k5e1_err.c:
 * This file is automatically generated; please do not edit it.
 */
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn initialize_k5e1_error_table() 
 /*@modifies internalState@*/
 {
    add_error_table(&et_k5e1_error_table);
}