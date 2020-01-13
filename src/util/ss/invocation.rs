use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:30"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:30"]
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
        #[c2rust::src_loc = "63:1"]
        pub fn ss_delete_info_dir(_: libc::c_int, _: *mut libc::c_char,
                                  _: *mut libc::c_int);
    }
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/stdio.h:30"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/errno.h:30"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/ss/ss_err.h:30"]
pub mod ss_err_h {
    extern "C" {
        /* for compatibility with older versions... */
        #[no_mangle]
        #[c2rust::src_loc = "26:1"]
        pub fn initialize_ss_error_table();
    }
    /*@modifies internalState@*/
}
pub use self::ss_internal_h::{pointer, ss_data, _ss_data, C2RustUnnamed,
                              ss_abbrev_info, ss_abbrev_list, _ss_abbrev_list,
                              ss_abbrev_entry, _ss_abbrev_entry, _ss_table};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_delete_info_dir};
use self::stdio_h::asprintf;
use self::stdlib_h::{malloc, calloc, realloc, free};
use self::errno_h::__errno_location;
use self::ss_err_h::initialize_ss_error_table;
/* XXX The memory in _ss_table never gets freed up until program exit!
   If you change the code to free it and stick a null pointer into
   _ss_table[sci_idx], make sure you change the allocation routine to
   not assume there are no null pointers in the middle of the
   array.  */
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn ss_create_invocation(mut subsystem_name:
                                                  *mut libc::c_char,
                                              mut version_string:
                                                  *mut libc::c_char,
                                              mut info_ptr: *mut libc::c_char,
                                              mut request_table_ptr:
                                                  *const ss_request_table,
                                              mut code_ptr: *mut libc::c_int)
 -> libc::c_int {
    let mut sci_idx: libc::c_int = 0;
    let mut new_table: *mut ss_data = 0 as *mut ss_data;
    let mut table: *mut *mut ss_data = 0 as *mut *mut ss_data;
    let mut tmp: *mut *mut ss_data = 0 as *mut *mut ss_data;
    *code_ptr = 0 as libc::c_int;
    table = _ss_table;
    new_table =
        malloc(::std::mem::size_of::<ss_data>() as libc::c_ulong) as
            *mut ss_data;
    if new_table.is_null() {
        *code_ptr = *__errno_location();
        return -(1 as libc::c_int)
    }
    if table.is_null() {
        table =
            malloc((2 as libc::c_int as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut ss_data>()
                                                        as libc::c_ulong)) as
                *mut *mut ss_data;
        if table.is_null() {
            *code_ptr = *__errno_location();
            return -(1 as libc::c_int)
        }
        let ref mut fresh0 = *table.offset(1 as libc::c_int as isize);
        *fresh0 = 0 as *mut libc::c_void as *mut ss_data;
        let ref mut fresh1 = *table.offset(0 as libc::c_int as isize);
        *fresh1 = *fresh0;
        _ss_table = table
    }
    initialize_ss_error_table();
    sci_idx = 1 as libc::c_int;
    while !(*table.offset(sci_idx as isize)).is_null() { sci_idx += 1 }
    tmp =
        realloc(table as *mut libc::c_char as *mut libc::c_void,
                ((sci_idx as
                      libc::c_uint).wrapping_add(2 as libc::c_int as
                                                     libc::c_uint) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut ss_data>()
                                                     as libc::c_ulong)) as
            *mut *mut ss_data;
    if tmp.is_null() {
        *code_ptr = *__errno_location();
        return 0 as libc::c_int
    }
    table = tmp;
    _ss_table = table;
    let ref mut fresh2 = *table.offset((sci_idx + 1 as libc::c_int) as isize);
    *fresh2 = 0 as *mut libc::c_void as *mut ss_data;
    let ref mut fresh3 = *table.offset(sci_idx as isize);
    *fresh3 = 0 as *mut ss_data;
    (*new_table).subsystem_name = subsystem_name;
    (*new_table).subsystem_version = version_string;
    (*new_table).argv = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*new_table).current_request =
        0 as *mut libc::c_void as *mut libc::c_char;
    (*new_table).info_dirs =
        malloc(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if (*new_table).info_dirs.is_null() {
        *code_ptr = *__errno_location();
        free(new_table as *mut libc::c_void);
        return 0 as libc::c_int
    }
    *(*new_table).info_dirs = 0 as *mut libc::c_void as *mut libc::c_char;
    (*new_table).info_ptr = info_ptr as pointer;
    if asprintf(&mut (*new_table).prompt as *mut *mut libc::c_char,
                b"%s:  \x00" as *const u8 as *const libc::c_char,
                subsystem_name) < 0 as libc::c_int {
        *code_ptr = *__errno_location();
        free((*new_table).info_dirs as *mut libc::c_void);
        free(new_table as *mut libc::c_void);
        return 0 as libc::c_int
    }
    (*new_table).abbrev_info = 0 as *mut ss_abbrev_info;
    (*new_table).flags.set_escape_disabled(0 as libc::c_int as libc::c_uint);
    (*new_table).flags.set_abbrevs_disabled(0 as libc::c_int as libc::c_uint);
    (*new_table).rqt_tables =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*const ss_request_table>() as
                   libc::c_ulong) as *mut *const ss_request_table;
    if (*new_table).rqt_tables.is_null() {
        *code_ptr = *__errno_location();
        free((*new_table).prompt as *mut libc::c_void);
        free((*new_table).info_dirs as *mut libc::c_void);
        free(new_table as *mut libc::c_void);
        return 0 as libc::c_int
    }
    *(*new_table).rqt_tables = request_table_ptr;
    let ref mut fresh4 =
        *(*new_table).rqt_tables.offset(1 as libc::c_int as isize);
    *fresh4 = 0 as *mut libc::c_void as *const ss_request_table;
    let ref mut fresh5 = *table.offset(sci_idx as isize);
    *fresh5 = new_table;
    return sci_idx;
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
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn ss_delete_invocation(mut sci_idx: libc::c_int) {
    let mut t: *mut ss_data = 0 as *mut ss_data;
    let mut ignored_code: libc::c_int = 0;
    t = *_ss_table.offset(sci_idx as isize);
    free((*t).prompt as *mut libc::c_void);
    free((*t).rqt_tables as *mut libc::c_void);
    while !(*(*t).info_dirs.offset(0 as libc::c_int as isize)).is_null() {
        ss_delete_info_dir(sci_idx,
                           *(*t).info_dirs.offset(0 as libc::c_int as isize),
                           &mut ignored_code);
    }
    free((*t).info_dirs as *mut libc::c_void);
    free(t as *mut libc::c_void);
}
