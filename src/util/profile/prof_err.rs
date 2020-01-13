use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:65"]
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
static mut text: [ncptr; 40] =
    [b"Profile version 0.0\x00" as *const u8 as *const libc::c_char,
     b"Bad magic value in profile_node\x00" as *const u8 as
         *const libc::c_char,
     b"Profile section not found\x00" as *const u8 as *const libc::c_char,
     b"Profile relation not found\x00" as *const u8 as *const libc::c_char,
     b"Attempt to add a relation to node which is not a section\x00" as
         *const u8 as *const libc::c_char,
     b"A profile section header has a non-zero value\x00" as *const u8 as
         *const libc::c_char,
     b"Bad linked list in profile structures\x00" as *const u8 as
         *const libc::c_char,
     b"Bad group level in profile strctures\x00" as *const u8 as
         *const libc::c_char,
     b"Bad parent pointer in profile strctures\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic value in profile iterator\x00" as *const u8 as
         *const libc::c_char,
     b"Can\'t set value on section node\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid argument passed to profile library\x00" as *const u8 as
         *const libc::c_char,
     b"Attempt to modify read-only profile\x00" as *const u8 as
         *const libc::c_char,
     b"Profile section header not at top level\x00" as *const u8 as
         *const libc::c_char,
     b"Syntax error in profile section header\x00" as *const u8 as
         *const libc::c_char,
     b"Syntax error in profile relation\x00" as *const u8 as
         *const libc::c_char,
     b"Extra closing brace in profile\x00" as *const u8 as
         *const libc::c_char,
     b"Missing open brace in profile\x00" as *const u8 as *const libc::c_char,
     b"Bad magic value in profile_t\x00" as *const u8 as *const libc::c_char,
     b"Bad magic value in profile_section_t\x00" as *const u8 as
         *const libc::c_char,
     b"Iteration through all top level section not supported\x00" as *const u8
         as *const libc::c_char,
     b"Invalid profile_section object\x00" as *const u8 as
         *const libc::c_char,
     b"No more sections\x00" as *const u8 as *const libc::c_char,
     b"Bad nameset passed to query routine\x00" as *const u8 as
         *const libc::c_char,
     b"No profile file open\x00" as *const u8 as *const libc::c_char,
     b"Bad magic value in profile_file_t\x00" as *const u8 as
         *const libc::c_char,
     b"Couldn\'t open profile file\x00" as *const u8 as *const libc::c_char,
     b"Section already exists\x00" as *const u8 as *const libc::c_char,
     b"Invalid boolean value\x00" as *const u8 as *const libc::c_char,
     b"Invalid integer value\x00" as *const u8 as *const libc::c_char,
     b"Bad magic value in profile_file_data_t\x00" as *const u8 as
         *const libc::c_char,
     b"Included profile file could not be read\x00" as *const u8 as
         *const libc::c_char,
     b"Included profile directory could not be read\x00" as *const u8 as
         *const libc::c_char,
     b"Operation not supported on this profile\x00" as *const u8 as
         *const libc::c_char,
     b"Bad magic value in profile iterator\x00" as *const u8 as
         *const libc::c_char,
     b"Unexpected module declaration in profile\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid syntax of module declaration in profile\x00" as *const u8 as
         *const libc::c_char,
     b"Invalid profile module\x00" as *const u8 as *const libc::c_char,
     b"mit-krb5\x00" as *const u8 as *const libc::c_char, 0 as ncptr];
#[no_mangle]
#[c2rust::src_loc = "67:26"]
pub static mut et_prof_error_table: error_table =
    unsafe {
        {
            let mut init =
                error_table{msgs: text.as_ptr(),
                            base: -(1429577728 as libc::c_long),
                            n_msgs: 38 as libc::c_int as libc::c_uint,};
            init
        }
    };
/*
 * et-c-prof_err.c:
 * This file is automatically generated; please do not edit it.
 */
#[no_mangle]
#[c2rust::src_loc = "70:1"]
pub unsafe extern "C" fn initialize_prof_error_table() 
 /*@modifies internalState@*/
 {
    add_error_table(&et_prof_error_table);
}
