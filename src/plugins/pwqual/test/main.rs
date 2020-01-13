use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:39"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:39"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:39"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/sys/types.h:39"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:39"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stat.h:39"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:40"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
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
        /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:40"]
pub mod plugin_h {
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
        /* Generic declarations for dynamic modules implementing krb5 plugin
 * modules. */
        /* krb5_plugin_vtable is an abstract type.  Module initvt functions will cast
 * it to the appropriate interface-specific vtable type. */
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/pwqual_plugin.h:40"]
pub mod pwqual_plugin_h {
    #[c2rust::src_loc = "62:1"]
    pub type krb5_pwqual_moddata = *mut krb5_pwqual_moddata_st;
    #[c2rust::src_loc = "68:1"]
    pub type krb5_pwqual_open_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                    _: *mut krb5_pwqual_moddata)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "89:1"]
    pub type krb5_pwqual_check_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: krb5_principal,
                                    _: *mut *const libc::c_char)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "95:1"]
    pub type krb5_pwqual_close_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata)
                   -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "101:16"]
    pub struct krb5_pwqual_vtable_st {
        pub name: *const libc::c_char,
        pub open: krb5_pwqual_open_fn,
        pub check: krb5_pwqual_check_fn,
        pub close: krb5_pwqual_close_fn,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
    /*
 * Declarations for password quality plugin module implementors.
 *
 * The password quality pluggable interface currently has only one supported
 * major version, which is 1.  Major version 1 has a current minor version
 * number of 1.
 *
 * Password quality plugin modules should define a function named
 * pwqual_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   pwqual_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                         krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_pwqual_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* An abstract type for password quality module data. */
    /* ** Method type declarations ***/
    /* Optional: Initialize module data.  dictfile is the realm's configured
 * dictionary filename. */
    /*
 * Mandatory: Check a password for the principal princ, which has an associated
 * password policy named policy_name (or no associated policy if policy_name is
 * NULL).  The parameter languages, if not NULL, contains a null-terminated
 * list of client-specified language tags as defined in RFC 5646.  The method
 * should return one of the following errors if the password fails quality
 * standards:
 *
 * - KADM5_PASS_Q_TOOSHORT: password should be longer
 * - KADM5_PASS_Q_CLASS:    password must have more character classes
 * - KADM5_PASS_Q_DICT:     password contains dictionary words
 * - KADM5_PASS_Q_GENERIC:  unspecified quality failure
 *
 * The module should also set an extended error message with
 * krb5_set_error_message().  The message may be localized according to one of
 * the language tags in languages.
 */
    /* Optional: Release resources used by module data. */
    /* ** vtable declarations **/
    /* Password quality plugin vtable for major version 1. */
    #[c2rust::src_loc = "101:1"]
    pub type krb5_pwqual_vtable = *mut krb5_pwqual_vtable_st;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_principal};
    extern "C" {
        #[c2rust::src_loc = "62:16"]
        pub type krb5_pwqual_moddata_st;
    }
    /* Mandatory: name of module. */
    /* Minor version 1 ends here. */
    /* KRB5_PWQUAL_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:39"]
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
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/strings.h:39"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "120:12"]
        pub fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                           _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:39"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:39"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:39"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:39"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:42"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __time_t, __blksize_t,
                        __blkcnt_t, __ssize_t, __syscall_slong_t};
pub use self::stdint_intn_h::int32_t;
pub use self::sys_types_h::ssize_t;
pub use self::struct_timespec_h::timespec;
pub use self::stat_h::stat;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_context, _krb5_context, krb5_set_error_message};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::pwqual_plugin_h::{krb5_pwqual_moddata, krb5_pwqual_open_fn,
                                krb5_pwqual_check_fn, krb5_pwqual_close_fn,
                                krb5_pwqual_vtable_st, krb5_pwqual_vtable,
                                krb5_pwqual_moddata_st};
use self::stdlib_h::{malloc, calloc, free};
use self::strings_h::{strncasecmp, strcasecmp};
use self::string_h::{strlen, memchr};
use self::fcntl_h::open;
use self::errno_h::__errno_location;
use self::unistd_h::{close, read};
use self::sys_stat_h::fstat;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "46:16"]
pub struct combo_moddata_st {
    pub word_list: *mut *const libc::c_char,
    pub word_block: *mut libc::c_char,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/pwqual/test/main.c - test module for password quality interface */
/*
 * Copyright (C) 2010,2013 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * This file implements a module named "combo" which tests whether a password
 * matches a pair of words in the dictionary.  It also implements several dummy
 * modules named "dyn1", "dyn2", and "dyn3" which are used for ordering tests.
 */
#[c2rust::src_loc = "46:1"]
pub type combo_moddata = *mut combo_moddata_st;
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn init_dict(mut dict: combo_moddata,
                               mut dict_file: *const libc::c_char)
 -> krb5_error_code {
    let mut fd: libc::c_int = 0;
    let mut count: size_t = 0;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    /* list of word pointers */
    /* actual word data */
    /* Read the dictionary file into memory in one blob. */
    if dict_file.is_null() { return 0 as libc::c_int }
    fd = open(dict_file, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return if *__errno_location() == 2 as libc::c_int {
                   0 as libc::c_int
               } else { *__errno_location() }
    }
    if fstat(fd, &mut sb) == -(1 as libc::c_int) {
        close(fd);
        return *__errno_location()
    }
    (*dict).word_block =
        malloc((sb.st_size + 1 as libc::c_int as libc::c_long) as
                   libc::c_ulong) as *mut libc::c_char;
    if (*dict).word_block.is_null() { return 12 as libc::c_int }
    if read(fd, (*dict).word_block as *mut libc::c_void, sb.st_size as size_t)
           != sb.st_size {
        return *__errno_location()
    }
    close(fd);
    *(*dict).word_block.offset(sb.st_size as isize) =
        '\u{0}' as i32 as libc::c_char;
    /* Decompose the blob into newline-separated words. */
    p = (*dict).word_block;
    len = sb.st_size as size_t;
    count = 0 as libc::c_int as size_t;
    while len > 0 as libc::c_int as libc::c_ulong &&
              {
                  t =
                      memchr(p as *const libc::c_void, '\n' as i32, len) as
                          *mut libc::c_char;
                  !t.is_null()
              } {
        *t = '\u{0}' as i32 as libc::c_char;
        len =
            (len as
                 libc::c_ulong).wrapping_sub((t.wrapping_offset_from(p) as
                                                  libc::c_long +
                                                  1 as libc::c_int as
                                                      libc::c_long) as
                                                 libc::c_ulong) as size_t as
                size_t;
        p = t.offset(1 as libc::c_int as isize);
        count = count.wrapping_add(1)
    }
    (*dict).word_list =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *const libc::c_char;
    if (*dict).word_list.is_null() { return 12 as libc::c_int }
    p = (*dict).word_block;
    i = 0 as libc::c_int as size_t;
    while i < count {
        let ref mut fresh0 = *(*dict).word_list.offset(i as isize);
        *fresh0 = p;
        p =
            p.offset(strlen(p).wrapping_add(1 as libc::c_int as libc::c_ulong)
                         as isize);
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn destroy_dict(mut dict: combo_moddata) {
    if dict.is_null() { return }
    free((*dict).word_list as *mut libc::c_void);
    free((*dict).word_block as *mut libc::c_void);
    free(dict as *mut libc::c_void);
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn combo_open(mut context: krb5_context,
                                mut dict_file: *const libc::c_char,
                                mut data: *mut krb5_pwqual_moddata)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dict: combo_moddata = 0 as *mut combo_moddata_st;
    *data = 0 as krb5_pwqual_moddata;
    /* Allocate and initialize a dictionary structure. */
    dict =
        malloc(::std::mem::size_of::<combo_moddata_st>() as libc::c_ulong) as
            combo_moddata;
    if dict.is_null() { return 12 as libc::c_int }
    (*dict).word_list = 0 as *mut *const libc::c_char;
    (*dict).word_block = 0 as *mut libc::c_char;
    /* Fill in the dictionary structure with data from dict_file. */
    ret = init_dict(dict, dict_file);
    if ret != 0 as libc::c_int { destroy_dict(dict); return ret }
    *data = dict as krb5_pwqual_moddata;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "135:1"]
unsafe extern "C" fn combo_check(mut context: krb5_context,
                                 mut data: krb5_pwqual_moddata,
                                 mut password: *const libc::c_char,
                                 mut policy_name: *const libc::c_char,
                                 mut princ: krb5_principal,
                                 mut languages: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut dict: combo_moddata = data as combo_moddata;
    let mut remainder: *const libc::c_char = 0 as *const libc::c_char;
    let mut word1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut word2: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    if (*dict).word_list.is_null() { return 0 as libc::c_int }
    word1 = (*dict).word_list;
    while !(*word1).is_null() {
        if !(strncasecmp(password, *word1, strlen(*word1)) !=
                 0 as libc::c_int) {
            remainder = password.offset(strlen(*word1) as isize);
            word2 = (*dict).word_list;
            while !(*word2).is_null() {
                if strcasecmp(remainder, *word2) == 0 as libc::c_int {
                    krb5_set_error_message(context,
                                           43787544 as libc::c_long as
                                               krb5_error_code,
                                           b"Password may not be a pair of dictionary words\x00"
                                               as *const u8 as
                                               *const libc::c_char);
                    return 43787544 as libc::c_long as krb5_error_code
                }
                word2 = word2.offset(1)
            }
        }
        word1 = word1.offset(1)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "163:1"]
unsafe extern "C" fn combo_close(mut context: krb5_context,
                                 mut data: krb5_pwqual_moddata) {
    destroy_dict(data as combo_moddata);
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn pwqual_combo_initvt(mut context: krb5_context,
                                             mut maj_ver: libc::c_int,
                                             mut min_ver: libc::c_int,
                                             mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_pwqual_vtable = 0 as *mut krb5_pwqual_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_pwqual_vtable;
    (*vt).name = b"combo\x00" as *const u8 as *const libc::c_char;
    (*vt).open =
        Some(combo_open as
                 unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                      _: *mut krb5_pwqual_moddata)
                     -> krb5_error_code);
    (*vt).check =
        Some(combo_check as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char,
                                      _: krb5_principal,
                                      _: *mut *const libc::c_char)
                     -> krb5_error_code);
    (*vt).close =
        Some(combo_close as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata)
                     -> ());
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "198:1"]
pub unsafe extern "C" fn pwqual_dyn1_initvt(mut context: krb5_context,
                                            mut maj_ver: libc::c_int,
                                            mut min_ver: libc::c_int,
                                            mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let ref mut fresh1 = (*(vtable as krb5_pwqual_vtable)).name;
    *fresh1 = b"dyn1\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "206:1"]
pub unsafe extern "C" fn pwqual_dyn2_initvt(mut context: krb5_context,
                                            mut maj_ver: libc::c_int,
                                            mut min_ver: libc::c_int,
                                            mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let ref mut fresh2 = (*(vtable as krb5_pwqual_vtable)).name;
    *fresh2 = b"dyn2\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "214:1"]
pub unsafe extern "C" fn pwqual_dyn3_initvt(mut context: krb5_context,
                                            mut maj_ver: libc::c_int,
                                            mut min_ver: libc::c_int,
                                            mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let ref mut fresh3 = (*(vtable as krb5_pwqual_vtable)).name;
    *fresh3 = b"dyn3\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
