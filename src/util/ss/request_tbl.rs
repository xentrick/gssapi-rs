use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:9"]
pub mod ss_internal_h {
    #[c2rust::src_loc = "13:1"]
    pub type pointer = *mut libc::c_void;
    #[c2rust::src_loc = "57:1"]
    pub type ss_data = _ss_data;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:9"]
    pub struct ss_abbrev_info {
        pub abbrevs: [ss_abbrev_list; 127],
    }
    #[c2rust::src_loc = "47:1"]
    pub type ss_abbrev_list = _ss_abbrev_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct _ss_abbrev_list {
        pub n_abbrevs: libc::c_int,
        pub first_abbrev: *mut ss_abbrev_entry,
    }
    #[c2rust::src_loc = "41:1"]
    pub type ss_abbrev_entry = _ss_abbrev_entry;
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
    use super::ss_h::ss_request_table;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "98:18"]
        pub static mut _ss_table: *mut *mut ss_data;
    }
    /* _ss_internal_h */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:9"]
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
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/stdlib.h:9"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:9"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
pub use self::ss_internal_h::{pointer, ss_data, _ss_data, C2RustUnnamed,
                              ss_abbrev_info, ss_abbrev_list, _ss_abbrev_list,
                              ss_abbrev_entry, _ss_abbrev_entry, _ss_table};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table};
use self::stdlib_h::realloc;
use self::errno_h::__errno_location;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
/* for some readable code... */
#[no_mangle]
#[c2rust::src_loc = "13:1"]
pub unsafe extern "C" fn ss_add_request_table(mut sci_idx: libc::c_int,
                                              mut rqtbl_ptr:
                                                  *const ss_request_table,
                                              mut position: libc::c_int,
                                              mut code_ptr:
                                                  *mut libc::c_int) {
    let mut info: *mut ss_data = 0 as *mut ss_data;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    info = *_ss_table.offset(sci_idx as isize);
    size = 0 as libc::c_int;
    while !(*(*info).rqt_tables.offset(size as isize)).is_null() { size += 1 }
    /* size == C subscript of NULL == #elements */
    size += 2 as libc::c_int; /* new element, and NULL */
    (*info).rqt_tables =
        realloc((*info).rqt_tables as *mut libc::c_void,
                (size as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*const ss_request_table>()
                                                     as libc::c_ulong)) as
            *mut *const ss_request_table;
    if (*info).rqt_tables.is_null() {
        *code_ptr = *__errno_location();
        return
    }
    if position > size - 2 as libc::c_int {
        position = size - 2 as libc::c_int
    }
    if size > 1 as libc::c_int {
        i = size - 2 as libc::c_int;
        while i >= position {
            let ref mut fresh0 =
                *(*info).rqt_tables.offset((i + 1 as libc::c_int) as isize);
            *fresh0 = *(*info).rqt_tables.offset(i as isize);
            i -= 1
        }
    }
    let ref mut fresh1 = *(*info).rqt_tables.offset(position as isize);
    *fresh1 = rqtbl_ptr;
    let ref mut fresh2 =
        *(*info).rqt_tables.offset((size - 1 as libc::c_int) as isize);
    *fresh2 = 0 as *mut libc::c_void as *const ss_request_table;
    *code_ptr = 0 as libc::c_int;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
/* whatever */
/* foo */
/* NULL */
/* 0 */
/* DEFAULT VALUES */
/* SS_RP_V1 */
/* call for unknown command */
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn ss_delete_request_table(mut sci_idx: libc::c_int,
                                                 mut rqtbl_ptr:
                                                     *const ss_request_table,
                                                 mut code_ptr:
                                                     *mut libc::c_int) {
    let mut info: *mut ss_data = 0 as *mut ss_data;
    let mut rt1: *mut *const ss_request_table =
        0 as *mut *const ss_request_table;
    let mut rt2: *mut *const ss_request_table =
        0 as *mut *const ss_request_table;
    *code_ptr = 748808 as libc::c_long as libc::c_int;
    info = *_ss_table.offset(sci_idx as isize);
    rt1 = (*info).rqt_tables;
    rt2 = rt1;
    while !(*rt1).is_null() {
        if *rt1 != rqtbl_ptr {
            let fresh3 = rt2;
            rt2 = rt2.offset(1);
            *fresh3 = *rt1;
            *code_ptr = 0 as libc::c_int
        }
        rt1 = rt1.offset(1)
    }
    *rt2 = 0 as *mut libc::c_void as *const ss_request_table;
}
