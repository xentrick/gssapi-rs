use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/ss/ss.h:29"]
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
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * For copyright information, see mit-sipb-copyright.h.
 */
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
        #[no_mangle]
        #[c2rust::src_loc = "59:1"]
        pub fn ss_listen(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn ss_create_invocation(_: *mut libc::c_char,
                                    _: *mut libc::c_char,
                                    _: *mut libc::c_char,
                                    _: *const ss_request_table,
                                    _: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn ss_execute_command(sci_idx: libc::c_int,
                                  _: *mut *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn ss_execute_line(_: libc::c_int, _: *mut libc::c_char)
         -> libc::c_int;
    }
    /* whatever */
    /* foo */
    /* NULL */
    /* 0 */
    /* _ss_h */
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:26"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:26"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:28"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/cli/kadmin.h:30"]
pub mod kadmin_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn quit() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "36:1"]
        pub fn kadmin_startup(argc: libc::c_int, argv: *mut *mut libc::c_char,
                              request_out: *mut *mut libc::c_char,
                              args_out: *mut *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "85:14"]
        pub static mut whoami: *mut libc::c_char;
    }
    /* __KADMIN_H__ */
}
pub use self::types_h::__int32_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_error_code};
pub use self::ss_h::{_ss_request_entry, ss_request_entry, _ss_request_table,
                     ss_request_table, ss_perror, ss_listen,
                     ss_create_invocation, ss_execute_command,
                     ss_execute_line};
use self::stdlib_h::exit;
use self::string_h::strrchr;
use self::libintl_h::dgettext;
use self::locale_h::setlocale;
use self::kadmin_h::{quit, kadmin_startup, whoami};
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
    #[no_mangle]
    #[c2rust::src_loc = "36:25"]
    pub static kadmin_cmds: ss_request_table;
    #[no_mangle]
    #[c2rust::src_loc = "37:12"]
    pub static mut exit_status: libc::c_int;
}
#[c2rust::src_loc = "40:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut request: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut sci_idx: libc::c_int = 0;
    let mut code: libc::c_int = 0 as libc::c_int;
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    whoami = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    whoami =
        if !whoami.is_null() {
            whoami.offset(1 as libc::c_int as isize)
        } else { *argv.offset(0 as libc::c_int as isize) };
    kadmin_startup(argc, argv, &mut request, &mut args);
    sci_idx =
        ss_create_invocation(whoami,
                             b"5.0\x00" as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, 0 as *mut libc::c_char,
                             &kadmin_cmds, &mut retval);
    if retval != 0 {
        ss_perror(sci_idx, retval as libc::c_long,
                  dgettext(b"mit-krb5\x00" as *const u8 as
                               *const libc::c_char,
                           b"creating invocation\x00" as *const u8 as
                               *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !(*args).is_null() {
        /* Execute post-option arguments as a single script-mode command. */
        code = ss_execute_command(sci_idx, args);
        if code != 0 {
            ss_perror(sci_idx, code as libc::c_long, *args);
            exit_status = 1 as libc::c_int
        }
    } else if !request.is_null() {
        /* Execute the -q option as a single interactive command. */
        code = ss_execute_line(sci_idx, request);
        if code != 0 as libc::c_int {
            ss_perror(sci_idx, code as libc::c_long, request);
            exit_status = 1 as libc::c_int
        }
    } else {
        /* Prompt for commands. */
        ss_listen(sci_idx);
    }
    return if quit() != 0 { 1 as libc::c_int } else { exit_status };
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
