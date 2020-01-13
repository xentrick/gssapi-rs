use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:90"]
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
static mut text: [ncptr; 65] =
    [b"Operation failed for unspecified reason\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires ``get\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires ``add\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires ``modify\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires ``delete\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Insufficient authorization for operation\x00" as *const u8 as
         *const libc::c_char,
     b"Database inconsistency detected\x00" as *const u8 as
         *const libc::c_char,
     b"Principal or policy already exists\x00" as *const u8 as
         *const libc::c_char,
     b"Communication failure with server\x00" as *const u8 as
         *const libc::c_char,
     b"No administration server found for realm\x00" as *const u8 as
         *const libc::c_char,
     b"Password history principal key version mismatch\x00" as *const u8 as
         *const libc::c_char,
     b"Connection to server not initialized\x00" as *const u8 as
         *const libc::c_char,
     b"Principal does not exist\x00" as *const u8 as *const libc::c_char,
     b"Policy does not exist\x00" as *const u8 as *const libc::c_char,
     b"Invalid field mask for operation\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid number of character classes\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid password length\x00" as *const u8 as *const libc::c_char,
     b"Illegal policy name\x00" as *const u8 as *const libc::c_char,
     b"Illegal principal name\x00" as *const u8 as *const libc::c_char,
     b"Invalid auxillary attributes\x00" as *const u8 as *const libc::c_char,
     b"Invalid password history count\x00" as *const u8 as
         *const libc::c_char,
     b"Password minimum life is greater than password maximum life\x00" as
         *const u8 as *const libc::c_char,
     b"Password is too short\x00" as *const u8 as *const libc::c_char,
     b"Password does not contain enough character classes\x00" as *const u8 as
         *const libc::c_char,
     b"Password is in the password dictionary\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot reuse password\x00" as *const u8 as *const libc::c_char,
     b"Current password\'s minimum life has not expired\x00" as *const u8 as
         *const libc::c_char,
     b"Policy is in use\x00" as *const u8 as *const libc::c_char,
     b"Connection to server already initialized\x00" as *const u8 as
         *const libc::c_char,
     b"Incorrect password\x00" as *const u8 as *const libc::c_char,
     b"Cannot change protected principal\x00" as *const u8 as
         *const libc::c_char,
     b"Programmer error! Bad Admin server handle\x00" as *const u8 as
         *const libc::c_char,
     b"Programmer error! Bad API structure version\x00" as *const u8 as
         *const libc::c_char,
     b"API structure version specified by application is no longer supported (to fix, recompile application against current KADM5 API header files and libraries)\x00"
         as *const u8 as *const libc::c_char,
     b"API structure version specified by application is unknown to libraries (to fix, obtain current KADM5 API header files and libraries and recompile application)\x00"
         as *const u8 as *const libc::c_char,
     b"Programmer error! Bad API version\x00" as *const u8 as
         *const libc::c_char,
     b"API version specified by application is no longer supported by libraries (to fix, update application to adhere to current API version and recompile)\x00"
         as *const u8 as *const libc::c_char,
     b"API version specified by application is no longer supported by server (to fix, update application to adhere to current API version and recompile)\x00"
         as *const u8 as *const libc::c_char,
     b"API version specified by application is unknown to libraries (to fix, obtain current KADM5 API header files and libraries and recompile application)\x00"
         as *const u8 as *const libc::c_char,
     b"API version specified by application is unknown to server (to fix, obtain and install newest KADM5 Admin Server)\x00"
         as *const u8 as *const libc::c_char,
     b"Database error! Required KADM5 principal missing\x00" as *const u8 as
         *const libc::c_char,
     b"The salt type of the specified principal does not support renaming\x00"
         as *const u8 as *const libc::c_char,
     b"Illegal configuration parameter for remote KADM5 client\x00" as
         *const u8 as *const libc::c_char,
     b"Illegal configuration parameter for local KADM5 client\x00" as
         *const u8 as *const libc::c_char,
     b"Operation requires ``list\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires ``change-password\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"GSS-API (or Kerberos) error\x00" as *const u8 as *const libc::c_char,
     b"Programmer error! Illegal tagged data list type\x00" as *const u8 as
         *const libc::c_char,
     b"Required parameters in kdc.conf missing\x00" as *const u8 as
         *const libc::c_char,
     b"Bad krb5 admin server hostname\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires ``set-key\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Multiple values for single or folded enctype\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid enctype for setv4key\x00" as *const u8 as *const libc::c_char,
     b"Mismatched enctypes for setkey3\x00" as *const u8 as
         *const libc::c_char,
     b"Missing parameters in krb5.conf required for kadmin client\x00" as
         *const u8 as *const libc::c_char,
     b"XDR encoding error\x00" as *const u8 as *const libc::c_char,
     b"Cannot resolve network address for admin server in requested realm\x00"
         as *const u8 as *const libc::c_char,
     b"Unspecified password quality failure\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid key/salt tuples\x00" as *const u8 as *const libc::c_char,
     b"Invalid multiple or duplicate kvnos in setkey operation\x00" as
         *const u8 as *const libc::c_char,
     b"Operation requires ``extract-keys\'\' privilege\x00" as *const u8 as
         *const libc::c_char,
     b"Principal keys are locked down\x00" as *const u8 as
         *const libc::c_char,
     b"Operation requires initial ticket\x00" as *const u8 as
         *const libc::c_char,
     b"mit-krb5\x00" as *const u8 as *const libc::c_char, 0 as ncptr];
#[no_mangle]
#[c2rust::src_loc = "92:26"]
pub static mut et_ovk_error_table: error_table =
    unsafe {
        {
            let mut init =
                error_table{msgs: text.as_ptr(),
                            base: 43787520 as libc::c_long,
                            n_msgs: 63 as libc::c_int as libc::c_uint,};
            init
        }
    };
/*
 * et-c-kadm_err.c:
 * This file is automatically generated; please do not edit it.
 */
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn initialize_ovk_error_table() 
 /*@modifies internalState@*/
 {
    add_error_table(&et_ovk_error_table);
}
