use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:10"]
pub mod ss_internal_h {
    #[c2rust::src_loc = "13:1"]
    pub type pointer = *mut libc::c_void;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "41:16"]
    pub struct _ss_abbrev_entry {
        pub name: *mut libc::c_char,
        pub abbrev: *mut *mut libc::c_char,
        #[bitfield(name = "beginning_of_line", ty = "libc::c_uint", bits =
                   "0..=0")]
        pub beginning_of_line: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[c2rust::src_loc = "41:1"]
    pub type ss_abbrev_entry = _ss_abbrev_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct _ss_abbrev_list {
        pub n_abbrevs: libc::c_int,
        pub first_abbrev: *mut ss_abbrev_entry,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ss_abbrev_list = _ss_abbrev_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct ss_abbrev_info {
        pub abbrevs: [ss_abbrev_list; 127],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:16"]
    pub struct _ss_data {
        pub subsystem_name: *mut libc::c_char,
        pub subsystem_version: *mut libc::c_char,
        pub argc: libc::c_int,
        pub argv: *mut *mut libc::c_char,
        pub current_request: *const libc::c_char,
        pub info_dirs: *mut *mut libc::c_char,
        pub info_ptr: pointer,
        pub prompt: *mut libc::c_char,
        pub rqt_tables: *mut *const ss_request_table,
        pub abbrev_info: *mut ss_abbrev_info,
        pub flags: C2RustUnnamed,
        pub abort: libc::c_int,
        pub exit_status: libc::c_int,
    }
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "73:5"]
    pub struct C2RustUnnamed {
        #[bitfield(name = "escape_disabled", ty = "libc::c_uint", bits =
                   "0..=0")]
        #[bitfield(name = "abbrevs_disabled", ty = "libc::c_uint", bits =
                   "1..=1")]
        pub escape_disabled_abbrevs_disabled: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
    }
    #[c2rust::src_loc = "57:1"]
    pub type ss_data = _ss_data;
    use super::ss_h::ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "98:18"]
        pub static mut _ss_table: *mut *mut ss_data;
    }
    /* _ss_internal_h */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:10"]
pub mod ss_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:27"]
    pub struct _ss_request_entry {
        pub command_names: *const *const libc::c_char,
        pub function: Option<unsafe extern "C" fn(_: libc::c_int,
                                                  _:
                                                      *const *const libc::c_char,
                                                  _: libc::c_int,
                                                  _: *mut libc::c_void)
                                 -> ()>,
        pub info_string: *const libc::c_char,
        pub flags: libc::c_int,
    }
    #[c2rust::src_loc = "22:1"]
    pub type ss_request_entry = _ss_request_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:27"]
    pub struct _ss_request_table {
        pub version: libc::c_int,
        pub requests: *const ss_request_entry,
    }
    #[c2rust::src_loc = "29:1"]
    pub type ss_request_table = _ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn ss_perror(_: libc::c_int, _: libc::c_long,
                         _: *const libc::c_char);
    }
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/stdio.h:9"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
pub use self::ss_internal_h::{pointer, _ss_abbrev_entry, ss_abbrev_entry,
                              _ss_abbrev_list, ss_abbrev_list, ss_abbrev_info,
                              _ss_data, C2RustUnnamed, ss_data, _ss_table};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_perror};
use self::stdio_h::printf;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/ss/requests.c */
/*
 * Copyright 1987, 1988, 1989 by MIT
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
/*
 * ss_self_identify -- assigned by default to the "." request
 */
#[no_mangle]
#[c2rust::src_loc = "17:1"]
pub unsafe extern "C" fn ss_self_identify(mut argc: libc::c_int,
                                          mut argv:
                                              *const *const libc::c_char,
                                          mut sci_idx: libc::c_int,
                                          mut info_ptr: pointer) {
    let mut info: *mut ss_data = *_ss_table.offset(sci_idx as isize);
    printf(b"%s version %s\n\x00" as *const u8 as *const libc::c_char,
           (*info).subsystem_name, (*info).subsystem_version);
}
/*
 * ss_subsystem_name -- print name of subsystem
 */
#[no_mangle]
#[c2rust::src_loc = "27:1"]
pub unsafe extern "C" fn ss_subsystem_name(mut argc: libc::c_int,
                                           mut argv:
                                               *const *const libc::c_char,
                                           mut sci_idx: libc::c_int,
                                           mut info_ptr: pointer) {
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           (**_ss_table.offset(sci_idx as isize)).subsystem_name);
}
/*
 * ss_subsystem_version -- print version of subsystem
 */
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn ss_subsystem_version(mut argc: libc::c_int,
                                              mut argv:
                                                  *const *const libc::c_char,
                                              mut sci_idx: libc::c_int,
                                              mut info_ptr: pointer) {
    printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
           (**_ss_table.offset(sci_idx as isize)).subsystem_version);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
/* LOCAL_ALLOC stuff */
/* abbrev name */
/* new tokens to insert */
/*    char *path; */
/* init values */
/* this subsystem */
/* current request info */
/* arg list */
/* primary name */
/* info directory for 'help' */
/* to be extracted by subroutines */
/* (void *) NULL */
/* for ss_listen processing */
/* to get out */
/* exit subsystem */
/*
 * ss_unimplemented -- routine not implemented (should be
 * set up as (dont_list,dont_summarize))
 */
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn ss_unimplemented(mut argc: libc::c_int,
                                          mut argv:
                                              *const *const libc::c_char,
                                          mut sci_idx: libc::c_int,
                                          mut info_ptr: pointer) {
    ss_perror(sci_idx, 748811 as libc::c_long,
              b"\x00" as *const u8 as *const libc::c_char);
}
