use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss_internal.h:8"]
pub mod ss_internal_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see copyright.h.
 */
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
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn ss_parse(_: libc::c_int, _: *mut libc::c_char,
                        _: *mut libc::c_int) -> *mut *mut libc::c_char;
    }
    /* _ss_internal_h */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/util/ss/ss.h:8"]
pub mod ss_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
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
    /* 0 */
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/stdlib.h:8"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "784:1"]
        pub fn system(__command: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:8"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
pub use self::ss_internal_h::{pointer, ss_data, _ss_data, C2RustUnnamed,
                              ss_abbrev_info, ss_abbrev_list, _ss_abbrev_list,
                              ss_abbrev_entry, _ss_abbrev_entry, _ss_table,
                              ss_parse};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table};
use self::stdlib_h::{malloc, free, system};
use self::string_h::strcmp;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988, 1989 by Massachusetts Institute of Technology
 *
 * For copyright info, see copyright.h.
 */
/*
 * get_request(tbl, idx)
 *
 * Function:
 *      Gets the idx'th request from the request table pointed to
 *      by tbl.
 * Arguments:
 *      tbl (ss_request_table *)
 *              pointer to request table
 *      idx (int)
 *              index into table
 * Returns:
 *      (ss_request_entry *)
 *              pointer to request table entry
 * Notes:
 *      Has been replaced by a macro.
 */
/*
 * check_request_table(rqtbl, argc, argv, sci_idx)
 *
 * Function:
 *      If the command string in argv[0] is in the request table, execute
 *      the commands and return error code 0.  Otherwise, return error
 *      code ss_et_command_not_found.
 * Arguments:
 *      rqtbl (ss_request_table *)
 *              pointer to request table
 *      argc (int)
 *              number of elements in argv[]
 *      argv (char *[])
 *              argument string array
 *      sci_idx (int)
 *              ss-internal index for subsystem control info structure
 * Returns:
 *      (int)
 *              zero if command found, ss_et_command_not_found otherwise
 * Notes:
 */
#[c2rust::src_loc = "55:1"]
unsafe extern "C" fn check_request_table(mut rqtbl: *const ss_request_table,
                                         mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char,
                                         mut sci_idx: libc::c_int)
 -> libc::c_int {
    let mut request: *const ss_request_entry = 0 as *const ss_request_entry;
    let mut info: *mut ss_data = 0 as *mut ss_data;
    let mut name: *const *const libc::c_char =
        0 as *const *const libc::c_char;
    let mut string: *mut libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    info = *_ss_table.offset(sci_idx as isize);
    (*info).argc = argc;
    (*info).argv = argv;
    i = 0 as libc::c_int;
    loop  {
        request = (*rqtbl).requests.offset(i as isize);
        if (*request).command_names.is_null() { break ; }
        name = (*request).command_names;
        while !(*name).is_null() {
            if strcmp(*name, string) == 0 {
                (*info).current_request =
                    *(*request).command_names.offset(0 as libc::c_int as
                                                         isize);
                (*request).function.expect("non-null function pointer")(argc,
                                                                        argv
                                                                            as
                                                                            *const *const libc::c_char,
                                                                        sci_idx,
                                                                        (*info).info_ptr);
                (*info).current_request =
                    0 as *mut libc::c_void as *mut libc::c_char;
                return 0 as libc::c_int
            }
            name = name.offset(1)
        }
        i += 1
    }
    return 748804 as libc::c_long as libc::c_int;
}
/*
 * really_execute_command(sci_idx, argc, argv)
 *
 * Function:
 *      Fills in the argc, argv values in the subsystem entry and
 *      call the appropriate routine.
 * Arguments:
 *      sci_idx (int)
 *              ss-internal index for subsystem control info structure
 *      argc (int)
 *              number of arguments in argument list
 *      argv (char **[])
 *              pointer to parsed argument list (may be reallocated
 *              on abbrev expansion)
 *
 * Returns:
 *      (int)
 *              Zero if successful, ss_et_command_not_found otherwise.
 * Notes:
 */
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn really_execute_command(mut sci_idx: libc::c_int,
                                            mut argc: libc::c_int,
                                            mut argv:
                                                *mut *mut *mut libc::c_char)
 -> libc::c_int {
    let mut rqtbl: *mut *const ss_request_table =
        0 as *mut *const ss_request_table;
    let mut info: *mut ss_data = 0 as *mut ss_data;
    info = *_ss_table.offset(sci_idx as isize);
    rqtbl = (*info).rqt_tables;
    while !(*rqtbl).is_null() {
        if check_request_table(*rqtbl, argc, *argv, sci_idx) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        rqtbl = rqtbl.offset(1)
    }
    return 748804 as libc::c_long as libc::c_int;
}
/*
 * ss_execute_command(sci_idx, argv)
 *
 * Function:
 *      Executes a parsed command list within the subsystem.
 * Arguments:
 *      sci_idx (int)
 *              ss-internal index for subsystem control info structure
 *      argv (char *[])
 *              parsed argument list
 * Returns:
 *      (int)
 *              Zero if successful, ss_et_command_not_found otherwise.
 * Notes:
 */
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub unsafe extern "C" fn ss_execute_command(mut sci_idx: libc::c_int,
                                            mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut argc: libc::c_uint = 0;
    let mut argp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    argc = 0 as libc::c_int as libc::c_uint;
    argp = argv;
    while !(*argp).is_null() {
        argc = argc.wrapping_add(1);
        argp = argp.offset(1)
    }
    argp =
        malloc((argc.wrapping_add(1 as libc::c_int as libc::c_uint) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if argp.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as libc::c_uint;
    while i <= argc {
        let ref mut fresh0 = *argp.offset(i as isize);
        *fresh0 = *argv.offset(i as isize);
        i = i.wrapping_add(1)
    }
    ret = really_execute_command(sci_idx, argc as libc::c_int, &mut argp);
    free(argp as *mut libc::c_void);
    return ret;
}
/*
 * ss_execute_line(sci_idx, line_ptr)
 *
 * Function:
 *      Parses and executes a command line within a subsystem.
 * Arguments:
 *      sci_idx (int)
 *              ss-internal index for subsystem control info structure
 *      line_ptr (char *)
 *              Pointer to command line to be parsed.
 * Returns:
 *      (int)
 *              Error code.
 * Notes:
 */
#[no_mangle]
#[c2rust::src_loc = "175:1"]
pub unsafe extern "C" fn ss_execute_line(mut sci_idx: libc::c_int,
                                         mut line_ptr: *mut libc::c_char)
 -> libc::c_int {
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    /* flush leading whitespace */
    while *line_ptr.offset(0 as libc::c_int as isize) as libc::c_int ==
              ' ' as i32 ||
              *line_ptr.offset(0 as libc::c_int as isize) as libc::c_int ==
                  '\t' as i32 {
        line_ptr = line_ptr.offset(1)
    }
    /* check if it should be sent to operating system for execution */
    if *line_ptr as libc::c_int == '!' as i32 {
        if (**_ss_table.offset(sci_idx as isize)).flags.escape_disabled() != 0
           {
            return 748810 as libc::c_long as libc::c_int
        } else {
            line_ptr = line_ptr.offset(1);
            system(line_ptr);
            return 0 as libc::c_int
        }
    }
    /* parse it */
    argv = ss_parse(sci_idx, line_ptr, &mut argc);
    if argc == 0 as libc::c_int { return 0 as libc::c_int }
    /* look it up in the request tables, execute if found */
    ret = really_execute_command(sci_idx, argc, &mut argv);
    free(argv as *mut libc::c_void);
    return ret;
}
