use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:7"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:7"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:7"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:7"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:7"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:7"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:8"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
    /*
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:7"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn vasprintf(__ptr: *mut *mut libc::c_char,
                         __f: *const libc::c_char, __arg: ::std::ffi::VaList)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:7"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/assert.h:7"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:7"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/support/supp-int.h:10"]
pub mod supp_int_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/supp-int.h */
/*
 * Copyright (C) 2006 by the Massachusetts Institute of Technology.
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
 *
 *  Internal prototypes for the krb5support library
 */
        #[no_mangle]
        #[c2rust::src_loc = "34:1"]
        pub fn krb5int_call_thread_support_init() -> libc::c_int;
    }
    /* KRB5_SUPP_INT_H__ */
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::stddef_h::size_t;
pub use self::stdarg_h::va_list;
pub use self::types_h::{__off_t, __off64_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_finish_init,
                            k5_mutex_lock, k5_mutex_unlock, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::k5_err_h::errinfo;
use self::stdio_h::{stderr, fprintf, snprintf, vasprintf};
use self::stdlib_h::free;
use self::assert_h::__assert_fail;
use self::string_h::{strerror, strdup};
use self::libintl_h::dgettext;
use self::supp_int_h::krb5int_call_thread_support_init;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Can't include krb5.h here, or k5-int.h which includes it, because krb5.h
 * needs to be generated with error tables, after util/et, which builds after
 * this directory.
 */
/*
 * It would be nice to just use error_message() always.  Pity that it's defined
 * in a library that depends on this one, and we're not allowed to make
 * circular dependencies.
 */
/*
 * We really want a rwlock here, since we should hold it while calling the
 * function and copying out its results.  But I haven't implemented shims for
 * rwlock yet.
 */
#[c2rust::src_loc = "22:19"]
static mut krb5int_error_info_support_mutex: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[c2rust::src_loc = "24:36"]
static mut fptr:
       Option<unsafe extern "C" fn(_: libc::c_long) -> *const libc::c_char> =
    None;
/* = &error_message */
/* Fallback error message if we cannot allocate a copy of the real one. */
#[c2rust::src_loc = "27:14"]
static mut oom_msg: *mut libc::c_char =
    b"Out of memory\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5int_err_init() -> libc::c_int {
    return k5_mutex_finish_init(&mut krb5int_error_info_support_mutex);
}
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn k5_set_error(mut ep: *mut errinfo,
                                      mut code: libc::c_long,
                                      mut fmt: *const libc::c_char,
                                      mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    k5_vset_error(ep, code, fmt, args_0.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "48:1"]
pub unsafe extern "C" fn k5_vset_error(mut ep: *mut errinfo,
                                       mut code: libc::c_long,
                                       mut fmt: *const libc::c_char,
                                       mut args: ::std::ffi::VaList) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    k5_clear_error(ep);
    (*ep).code = code;
    if vasprintf(&mut str, fmt, args.as_va_list()) < 0 as libc::c_int {
        return
    }
    (*ep).msg = str;
}
#[inline]
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn oom_check(mut str: *const libc::c_char)
 -> *const libc::c_char {
    return if str.is_null() { oom_msg as *const libc::c_char } else { str };
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn k5_get_error(mut ep: *mut errinfo,
                                      mut code: libc::c_long)
 -> *const libc::c_char {
    let mut r: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if code == (*ep).code && !(*ep).msg.is_null() {
        return oom_check(strdup((*ep).msg))
    }
    if krb5int_call_thread_support_init() != 0 {
        return oom_check(strdup(dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"Kerberos library initialization failure\x00"
                                             as *const u8 as
                                             *const libc::c_char)))
    }
    k5_mutex_lock(&mut krb5int_error_info_support_mutex);
    if fptr.is_none() {
        /* Should be rare; fptr should be set whenever libkrb5 is loaded. */
        k5_mutex_unlock(&mut krb5int_error_info_support_mutex);
        return oom_check(strdup(dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"Error code translation unavailable\x00"
                                             as *const u8 as
                                             *const libc::c_char)))
    }
    r = fptr.expect("non-null function pointer")(code);
    if r.is_null() {
        k5_mutex_unlock(&mut krb5int_error_info_support_mutex);
        snprintf(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 128]>() as
                     libc::c_ulong,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"error %ld\x00" as *const u8 as
                              *const libc::c_char), code);
        return oom_check(strdup(buf.as_mut_ptr()))
    }
    r = strdup(r);
    k5_mutex_unlock(&mut krb5int_error_info_support_mutex);
    return oom_check(r);
}
#[no_mangle]
#[c2rust::src_loc = "101:1"]
pub unsafe extern "C" fn k5_free_error(mut ep: *mut errinfo,
                                       mut msg: *const libc::c_char) {
    if msg != oom_msg as *const libc::c_char {
        free(msg as *mut libc::c_char as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "108:1"]
pub unsafe extern "C" fn k5_clear_error(mut ep: *mut errinfo) {
    k5_free_error(ep, (*ep).msg);
    (*ep).msg = 0 as *mut libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn k5_set_error_info_callout_fn(mut f:
                                                          Option<unsafe extern "C" fn(_:
                                                                                          libc::c_long)
                                                                     ->
                                                                         *const libc::c_char>) {
    krb5int_call_thread_support_init();
    k5_mutex_lock(&mut krb5int_error_info_support_mutex);
    fptr = f;
    k5_mutex_unlock(&mut krb5int_error_info_support_mutex);
}
