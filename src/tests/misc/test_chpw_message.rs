use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
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
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        /* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
        #[no_mangle]
        #[c2rust::src_loc = "5110:1"]
        pub fn krb5_chpw_message(context: krb5_context,
                                 server_string: *const krb5_data,
                                 message_out: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:28"]
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
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:33"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/locale.h:32"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "329:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __off_t, __off64_t};
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_context, _krb5_context,
                       krb5_init_context, krb5_free_context,
                       krb5_chpw_message};
pub use self::com_err_h::{errcode_t, com_err};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
use self::stdlib_h::{exit, abort, free};
use self::locale_h::setlocale;
use self::stdio_h::{stderr, fprintf, printf};
use self::string_h::strstr;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/misc/test_getpw.c */
/*
 * Copyright (C) 2012 by the Red Hat Inc.
 * All rights reserved.
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
#[c2rust::src_loc = "37:18"]
static mut result_utf8: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 23 as libc::c_int as libc::c_uint,
                       data:
                           b"This is a valid string.\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "41:18"]
static mut result_invalid_utf8: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 19 as libc::c_int as libc::c_uint,
                       data:
                           b"\x00This is not valid.\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "45:18"]
static mut result_ad_complex: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 30 as libc::c_int as libc::c_uint,
                       data:
                           b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "55:18"]
static mut result_ad_length: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 30 as libc::c_int as libc::c_uint,
                       data:
                           b"\x00\x00\x00\x00\x00\r\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "65:18"]
static mut result_ad_history: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 30 as libc::c_int as libc::c_uint,
                       data:
                           b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "75:18"]
static mut result_ad_age: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 30 as libc::c_int as libc::c_uint,
                       data:
                           b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x92T\xd3\x80\x00\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "85:18"]
static mut result_ad_all: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 30 as libc::c_int as libc::c_uint,
                       data:
                           b"\x00\x00\x00\x00\x00\x05\x00\x00\x00\r\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xc9*i\xc0\x00\x00"
                               as *const u8 as *const libc::c_char as
                               *mut libc::c_char,};
        init
    };
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn check(mut code: krb5_error_code) {
    if code != 0 as libc::c_int {
        com_err(b"t_vfy_increds\x00" as *const u8 as *const libc::c_char,
                code as errcode_t,
                b"\x00" as *const u8 as *const libc::c_char);
        abort();
    };
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn check_msg(mut real: *const libc::c_char,
                               mut expected: *const libc::c_char) {
    if strstr(real, expected).is_null() {
        fprintf(stderr,
                b"Expected to see: %s\n\x00" as *const u8 as
                    *const libc::c_char, expected);
        abort();
    };
}
#[c2rust::src_loc = "113:1"]
unsafe fn main_0() -> libc::c_int {
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    setlocale(6 as libc::c_int, b"C\x00" as *const u8 as *const libc::c_char);
    check(krb5_init_context(&mut context));
    /* Valid utf-8 data in the result should be returned as is */
    check(krb5_chpw_message(context, &mut result_utf8, &mut msg));
    printf(b"  UTF8 valid:   %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"This is a valid string.\x00" as *const u8 as
                  *const libc::c_char);
    free(msg as *mut libc::c_void);
    /* Invalid data should have a generic message. */
    check(krb5_chpw_message(context, &mut result_invalid_utf8, &mut msg));
    printf(b"  UTF8 invalid: %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"contact your administrator\x00" as *const u8 as
                  *const libc::c_char);
    free(msg as *mut libc::c_void);
    /* AD data with complex data requirement */
    check(krb5_chpw_message(context, &mut result_ad_complex, &mut msg));
    printf(b"  AD complex:   %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"The password must include numbers or symbols.\x00" as
                  *const u8 as *const libc::c_char);
    check_msg(msg,
              b"Don\'t include any part of your name in the password.\x00" as
                  *const u8 as *const libc::c_char);
    free(msg as *mut libc::c_void);
    /* AD data with min password length */
    check(krb5_chpw_message(context, &mut result_ad_length, &mut msg));
    printf(b"  AD length:    %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"The password must contain at least 13 characters.\x00" as
                  *const u8 as *const libc::c_char);
    free(msg as *mut libc::c_void);
    /* AD data with history requirements */
    check(krb5_chpw_message(context, &mut result_ad_history, &mut msg));
    printf(b"  AD history:   %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"The password must be different from the previous 9 passwords.\x00"
                  as *const u8 as *const libc::c_char);
    free(msg as *mut libc::c_void);
    /* AD data with minimum age */
    check(krb5_chpw_message(context, &mut result_ad_age, &mut msg));
    printf(b"  AD min age:   %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"The password can only be changed every 2 days.\x00" as
                  *const u8 as *const libc::c_char);
    free(msg as *mut libc::c_void);
    /* AD data with all */
    check(krb5_chpw_message(context, &mut result_ad_all, &mut msg));
    printf(b"  AD all:       %s\n\x00" as *const u8 as *const libc::c_char,
           msg);
    check_msg(msg,
              b"The password can only be changed once a day.\x00" as *const u8
                  as *const libc::c_char);
    check_msg(msg,
              b"The password must be different from the previous 13 passwords.\x00"
                  as *const u8 as *const libc::c_char);
    check_msg(msg,
              b"The password must contain at least 5 characters.\x00" as
                  *const u8 as *const libc::c_char);
    check_msg(msg,
              b"The password must include numbers or symbols.\x00" as
                  *const u8 as *const libc::c_char);
    check_msg(msg,
              b"Don\'t include any part of your name in the password.\x00" as
                  *const u8 as *const libc::c_char);
    free(msg as *mut libc::c_void);
    krb5_free_context(context);
    exit(0 as libc::c_int);
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
