use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:73"]
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
static mut text: [ncptr; 48] =
    [b"$Id$\x00" as *const u8 as *const libc::c_char,
     b"Entry already exists in database\x00" as *const u8 as
         *const libc::c_char,
     b"Database store error\x00" as *const u8 as *const libc::c_char,
     b"Database read error\x00" as *const u8 as *const libc::c_char,
     b"Insufficient access to perform requested operation\x00" as *const u8 as
         *const libc::c_char,
     b"No such entry in the database\x00" as *const u8 as *const libc::c_char,
     b"Illegal use of wildcard\x00" as *const u8 as *const libc::c_char,
     b"Database is locked or in use--try again later\x00" as *const u8 as
         *const libc::c_char,
     b"Database was modified during read\x00" as *const u8 as
         *const libc::c_char,
     b"Database record is incomplete or corrupted\x00" as *const u8 as
         *const libc::c_char,
     b"Attempt to lock database twice\x00" as *const u8 as
         *const libc::c_char,
     b"Attempt to unlock database when not locked\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid kdb lock mode\x00" as *const u8 as *const libc::c_char,
     b"Database has not been initialized\x00" as *const u8 as
         *const libc::c_char,
     b"Database has already been initialized\x00" as *const u8 as
         *const libc::c_char,
     b"Bad direction for converting keys\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot find master key record in database\x00" as *const u8 as
         *const libc::c_char,
     b"Master key does not match database\x00" as *const u8 as
         *const libc::c_char,
     b"Key size in database is invalid\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot find/read stored master key\x00" as *const u8 as
         *const libc::c_char,
     b"Stored master key is corrupted\x00" as *const u8 as
         *const libc::c_char,
     b"Cannot find active master key\x00" as *const u8 as *const libc::c_char,
     b"KVNO of new master key does not match expected value\x00" as *const u8
         as *const libc::c_char,
     b"Stored master key is not current\x00" as *const u8 as
         *const libc::c_char,
     b"Insufficient access to lock database\x00" as *const u8 as
         *const libc::c_char,
     b"Database format error\x00" as *const u8 as *const libc::c_char,
     b"Unsupported version in database entry\x00" as *const u8 as
         *const libc::c_char,
     b"Unsupported salt type\x00" as *const u8 as *const libc::c_char,
     b"Unsupported encryption type\x00" as *const u8 as *const libc::c_char,
     b"Bad database creation flags\x00" as *const u8 as *const libc::c_char,
     b"No matching key in entry having a permitted enctype\x00" as *const u8
         as *const libc::c_char,
     b"No matching key in entry\x00" as *const u8 as *const libc::c_char,
     b"Unable to find requested database type\x00" as *const u8 as
         *const libc::c_char,
     b"Database type not supported\x00" as *const u8 as *const libc::c_char,
     b"Database library failed to initialize\x00" as *const u8 as
         *const libc::c_char,
     b"Server error\x00" as *const u8 as *const libc::c_char,
     b"Unable to access Kerberos database\x00" as *const u8 as
         *const libc::c_char,
     b"Kerberos database internal error\x00" as *const u8 as
         *const libc::c_char,
     b"Kerberos database constraints violated\x00" as *const u8 as
         *const libc::c_char,
     b"Update log conversion error\x00" as *const u8 as *const libc::c_char,
     b"Update log is unstable\x00" as *const u8 as *const libc::c_char,
     b"Update log is corrupt\x00" as *const u8 as *const libc::c_char,
     b"Generic update log error\x00" as *const u8 as *const libc::c_char,
     b"Database module does not match KDC version\x00" as *const u8 as
         *const libc::c_char,
     b"Policy is in use\x00" as *const u8 as *const libc::c_char,
     b"Too much string mapping data\x00" as *const u8 as *const libc::c_char,
     b"mit-krb5\x00" as *const u8 as *const libc::c_char, 0 as ncptr];
#[no_mangle]
#[c2rust::src_loc = "75:26"]
pub static mut et_kdb5_error_table: error_table =
    unsafe {
        {
            let mut init =
                error_table{msgs: text.as_ptr(),
                            base: -(1780008448 as libc::c_long),
                            n_msgs: 46 as libc::c_int as libc::c_uint,};
            init
        }
    };
/*
 * et-c-kdb5_err.c:
 * This file is automatically generated; please do not edit it.
 */
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn initialize_kdb5_error_table() 
 /*@modifies internalState@*/
 {
    add_error_table(&et_kdb5_error_table);
}
