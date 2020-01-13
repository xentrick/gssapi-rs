use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/sys/types.h:32"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:32"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    #[c2rust::src_loc = "808:1"]
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "820:1"]
        pub fn bsearch(__key: *const libc::c_void,
                       __base: *const libc::c_void, __nmemb: size_t,
                       __size: size_t, __compar: __compar_fn_t)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "830:1"]
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
}
#[c2rust::header_src = "/usr/include/bits/stat.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/pwqual_plugin.h:33"]
pub mod pwqual_plugin_h {
    #[c2rust::src_loc = "62:1"]
    pub type krb5_pwqual_moddata = *mut krb5_pwqual_moddata_st;
    /* ** Method type declarations ***/
    /* Optional: Initialize module data.  dictfile is the realm's configured
 * dictionary filename. */
    #[c2rust::src_loc = "68:1"]
    pub type krb5_pwqual_open_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                    _: *mut krb5_pwqual_moddata)
                   -> krb5_error_code>;
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
    #[c2rust::src_loc = "89:1"]
    pub type krb5_pwqual_check_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: krb5_principal,
                                    _: *mut *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: Release resources used by module data. */
    #[c2rust::src_loc = "95:1"]
    pub type krb5_pwqual_close_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata)
                   -> ()>;
    /* ** vtable declarations **/
    /* Password quality plugin vtable for major version 1. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "101:16"]
    pub struct krb5_pwqual_vtable_st {
        pub name: *const libc::c_char,
        pub open: krb5_pwqual_open_fn,
        pub check: krb5_pwqual_check_fn,
        pub close: krb5_pwqual_close_fn,
    }
    #[c2rust::src_loc = "101:1"]
    pub type krb5_pwqual_vtable = *mut krb5_pwqual_vtable_st;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_principal};
    extern "C" {
        /* Minor version 1 ends here. */
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
        #[c2rust::src_loc = "62:16"]
        pub type krb5_pwqual_moddata_st;
    }
    /* KRB5_PWQUAL_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:32"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:32"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:32"]
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
#[c2rust::header_src = "/usr/include/fcntl.h:32"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:32"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:35"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:38"]
pub mod adm_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __time_t, __blksize_t,
                        __blkcnt_t, __ssize_t, __syscall_slong_t};
pub use self::stdint_intn_h::int32_t;
pub use self::sys_types_h::ssize_t;
pub use self::struct_timespec_h::timespec;
pub use self::stdlib_h::{__compar_fn_t, malloc, free, bsearch, qsort};
pub use self::stat_h::stat;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_context, _krb5_context};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::pwqual_plugin_h::{krb5_pwqual_moddata, krb5_pwqual_open_fn,
                                krb5_pwqual_check_fn, krb5_pwqual_close_fn,
                                krb5_pwqual_vtable_st, krb5_pwqual_vtable,
                                krb5_pwqual_moddata_st};
use self::string_h::{memchr, strlen};
use self::strings_h::strcasecmp;
use self::errno_h::__errno_location;
use self::unistd_h::{close, read};
use self::fcntl_h::{fcntl, open};
use self::libintl_h::dgettext;
use self::sys_stat_h::fstat;
use self::adm_proto_h::krb5_klog_syslog;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/srv/pwqual_dict.c */
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
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 */
/* Password quality module to look up passwords within the realm dictionary. */
#[c2rust::src_loc = "42:1"]
pub type dict_moddata = *mut dict_moddata_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "42:16"]
pub struct dict_moddata_st {
    pub word_list: *mut *mut libc::c_char,
    pub word_block: *mut libc::c_char,
    pub word_count: libc::c_uint,
}
/* list of word pointers */
/* actual word data */
/* number of words */
/*
 * Function: word_compare
 *
 * Purpose: compare two words in the dictionary.
 *
 * Arguments:
 *      w1              (input) pointer to first word
 *      w2              (input) pointer to second word
 *      <return value>  result of strcmp
 *
 * Requires:
 *      w1 and w2 to point to valid memory
 *
 */
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn word_compare(mut s1: *const libc::c_void,
                                  mut s2: *const libc::c_void)
 -> libc::c_int {
    return strcasecmp(*(s1 as *mut *const libc::c_char),
                      *(s2 as *mut *const libc::c_char));
}
/*
 * Function: init-dict
 *
 * Purpose: Initialize in memory word dictionary
 *
 * Arguments:
 *          none
 *          <return value> KADM5_OK on success errno on failure;
 *                         (but success on ENOENT)
 *
 * Requires:
 *      If WORDFILE exists, it must contain a list of words,
 *      one word per-line.
 *
 * Effects:
 *      If WORDFILE exists, it is read into memory sorted for future
 * use.  If it does not exist, it syslogs an error message and returns
 * success.
 *
 * Modifies:
 *      word_list to point to a chunck of allocated memory containing
 *      pointers to words
 *      word_block to contain the dictionary.
 *
 */
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn init_dict(mut dict: dict_moddata,
                               mut dict_file: *const libc::c_char)
 -> libc::c_int {
    let mut fd: libc::c_int = 0;
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
    if dict_file.is_null() {
        krb5_klog_syslog(6 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"No dictionary file specified, continuing without one.\x00"
                                      as *const u8 as *const libc::c_char));
        return 0 as libc::c_int
    }
    fd = open(dict_file, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        if *__errno_location() == 2 as libc::c_int {
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"WARNING!  Cannot find dictionary file %s, continuing without one.\x00"
                                          as *const u8 as
                                          *const libc::c_char), dict_file);
            return 0 as libc::c_int
        } else { return *__errno_location() }
    }
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
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
    p = (*dict).word_block;
    len = sb.st_size as size_t;
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
        (*dict).word_count = (*dict).word_count.wrapping_add(1)
    }
    (*dict).word_list =
        malloc(((*dict).word_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if (*dict).word_list.is_null() { return 12 as libc::c_int }
    p = (*dict).word_block;
    i = 0 as libc::c_int as size_t;
    while i < (*dict).word_count as libc::c_ulong {
        let ref mut fresh0 = *(*dict).word_list.offset(i as isize);
        *fresh0 = p;
        p =
            p.offset(strlen(p).wrapping_add(1 as libc::c_int as libc::c_ulong)
                         as isize);
        i = i.wrapping_add(1)
    }
    qsort((*dict).word_list as *mut libc::c_void,
          (*dict).word_count as size_t,
          ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
          Some(word_compare as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    return 0 as libc::c_int;
}
/*
 * Function: destroy_dict
 *
 * Purpose: destroy in-core copy of dictionary.
 *
 * Arguments:
 *          none
 *          <return value>  none
 * Requires:
 *          nothing
 * Effects:
 *      frees up memory occupied by word_list and word_block
 *      sets count back to 0, and resets the pointers to NULL
 *
 * Modifies:
 *      word_list, word_block, and word_count.
 *
 */
#[c2rust::src_loc = "169:1"]
unsafe extern "C" fn destroy_dict(mut dict: dict_moddata) {
    if dict.is_null() { return }
    free((*dict).word_list as *mut libc::c_void);
    free((*dict).word_block as *mut libc::c_void);
    free(dict as *mut libc::c_void);
}
/* Implement the password quality open method by reading in dict_file. */
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn dict_open(mut context: krb5_context,
                               mut dict_file: *const libc::c_char,
                               mut data: *mut krb5_pwqual_moddata)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dict: dict_moddata = 0 as *mut dict_moddata_st;
    *data = 0 as krb5_pwqual_moddata;
    /* Allocate and initialize a dictionary structure. */
    dict =
        malloc(::std::mem::size_of::<dict_moddata_st>() as libc::c_ulong) as
            dict_moddata;
    if dict.is_null() { return 12 as libc::c_int }
    (*dict).word_list = 0 as *mut *mut libc::c_char;
    (*dict).word_block = 0 as *mut libc::c_char;
    (*dict).word_count = 0 as libc::c_int as libc::c_uint;
    /* Fill in the dictionary structure with data from dict_file. */
    ret = init_dict(dict, dict_file);
    if ret != 0 as libc::c_int { destroy_dict(dict); return ret }
    *data = dict as krb5_pwqual_moddata;
    return 0 as libc::c_int;
}
/* Implement the password quality check method by checking the password
 * against the dictionary, as well as against principal components. */
#[c2rust::src_loc = "211:1"]
unsafe extern "C" fn dict_check(mut context: krb5_context,
                                mut data: krb5_pwqual_moddata,
                                mut password: *const libc::c_char,
                                mut policy_name: *const libc::c_char,
                                mut princ: krb5_principal,
                                mut languages: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut dict: dict_moddata = data as dict_moddata;
    /* Don't check the dictionary for principals with no password policy. */
    if policy_name.is_null() { return 0 as libc::c_int }
    /* Check against words in the dictionary if we successfully loaded one. */
    if !(*dict).word_list.is_null() &&
           !bsearch(&mut password as *mut *const libc::c_char as
                        *const libc::c_void,
                    (*dict).word_list as *const libc::c_void,
                    (*dict).word_count as size_t,
                    ::std::mem::size_of::<*mut libc::c_char>() as
                        libc::c_ulong,
                    Some(word_compare as
                             unsafe extern "C" fn(_: *const libc::c_void,
                                                  _: *const libc::c_void)
                                 -> libc::c_int)).is_null() {
        return 43787544 as libc::c_long as krb5_error_code
    }
    return 0 as libc::c_int;
}
/* Implement the password quality close method. */
#[c2rust::src_loc = "232:1"]
unsafe extern "C" fn dict_close(mut context: krb5_context,
                                mut data: krb5_pwqual_moddata) {
    destroy_dict(data as dict_moddata);
}
/* ** initvt functions for built-in password quality modules ***/
/* The dict module checks passwords against the realm's dictionary. */
#[no_mangle]
#[c2rust::src_loc = "238:1"]
pub unsafe extern "C" fn pwqual_dict_initvt(mut context: krb5_context,
                                            mut maj_ver: libc::c_int,
                                            mut min_ver: libc::c_int,
                                            mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_pwqual_vtable = 0 as *mut krb5_pwqual_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_pwqual_vtable;
    (*vt).name = b"dict\x00" as *const u8 as *const libc::c_char;
    (*vt).open =
        Some(dict_open as
                 unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                      _: *mut krb5_pwqual_moddata)
                     -> krb5_error_code);
    (*vt).check =
        Some(dict_check as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char,
                                      _: krb5_principal,
                                      _: *mut *const libc::c_char)
                     -> krb5_error_code);
    (*vt).close =
        Some(dict_close as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_pwqual_moddata)
                     -> ());
    return 0 as libc::c_int;
}
