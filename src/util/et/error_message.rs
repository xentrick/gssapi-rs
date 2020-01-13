use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:23"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:23"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:23"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:23"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "53:1"]
    pub type pthread_once_t = libc::c_int;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:23"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:23"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:23"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "188:1"]
    pub type k5_os_nothread_once_t = libc::c_uchar;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "256:9"]
    pub struct k5_once_t {
        pub o: pthread_once_t,
        pub n: k5_os_nothread_once_t,
    }
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[c2rust::src_loc = "401:9"]
    pub type k5_key_t = libc::c_uint;
    #[c2rust::src_loc = "410:5"]
    pub const K5_KEY_MAX: k5_key_t = 5;
    #[c2rust::src_loc = "406:5"]
    pub const K5_KEY_GSS_SPNEGO_STATUS: k5_key_t = 4;
    #[c2rust::src_loc = "405:5"]
    pub const K5_KEY_GSS_KRB5_ERROR_MESSAGE: k5_key_t = 3;
    #[c2rust::src_loc = "404:5"]
    pub const K5_KEY_GSS_KRB5_CCACHE_NAME: k5_key_t = 2;
    #[c2rust::src_loc = "403:5"]
    pub const K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME: k5_key_t = 1;
    #[c2rust::src_loc = "402:5"]
    pub const K5_KEY_COM_ERR: k5_key_t = 0;
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
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    use super::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "419:1"]
        pub fn krb5int_setspecific(_: k5_key_t, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5int_getspecific(_: k5_key_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "417:1"]
        pub fn krb5int_key_register(_: k5_key_t,
                                    _:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> ()>) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn k5_once(once: *mut k5_once_t,
                       fn_0: Option<unsafe extern "C" fn() -> ()>)
         -> libc::c_int;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:23"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "237:9"]
    pub struct k5_init_t {
        pub once: k5_once_t,
        pub error: libc::c_int,
        pub did_run: libc::c_int,
        pub fn_0: Option<unsafe extern "C" fn() -> ()>,
    }
    use super::k5_thread_h::k5_once_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "990:1"]
        pub fn k5_strerror_r(errnum: libc::c_int, buf: *mut libc::c_char,
                             buflen: size_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/et/com_err.h:24"]
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
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/et/error_table.h:25"]
pub mod error_table_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:8"]
    pub struct et_list {
        pub next: *mut et_list,
        pub table: *const error_table,
    }
    use super::com_err_h::error_table;
    use super::k5_thread_h::k5_mutex_t;
    use super::thread_shared_types_h::{__pthread_mutex_s, __pthread_list_t,
                                       __pthread_internal_list};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "25:1"]
        pub fn error_table_name_r(_: libc::c_ulong, outbuf: *mut libc::c_char)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "28:19"]
        pub static mut com_err_hook_lock: k5_mutex_t;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:23"]
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
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:23"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn bindtextdomain(__domainname: *const libc::c_char,
                              __dirname: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:23"]
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
#[c2rust::header_src = "/usr/include/string.h:23"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:23"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__off_t, __off64_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_os_mutex,
                            k5_mutex_t, k5_key_t, K5_KEY_MAX,
                            K5_KEY_GSS_SPNEGO_STATUS,
                            K5_KEY_GSS_KRB5_ERROR_MESSAGE,
                            K5_KEY_GSS_KRB5_CCACHE_NAME,
                            K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                            K5_KEY_COM_ERR, k5_mutex_unlock, k5_mutex_lock,
                            k5_mutex_finish_init, krb5int_setspecific,
                            krb5int_getspecific, krb5int_key_register,
                            k5_os_mutex_unlock, k5_os_mutex_lock, k5_once};
pub use self::k5_platform_h::{k5_init_t, k5_strerror_r, krb5int_strlcpy};
pub use self::com_err_h::{errcode_t, error_table};
pub use self::error_table_h::{et_list, error_table_name_r, com_err_hook_lock};
use self::stdio_h::{stderr, fprintf};
use self::libintl_h::{bindtextdomain, dgettext};
use self::assert_h::__assert_fail;
use self::string_h::strerror;
use self::stdlib_h::{malloc, free, abort};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1997,2000,2001,2004,2008 by Massachusetts Institute of Technology
 *
 * Copyright 1987, 1988 by MIT Student Information Processing Board
 *
 * Permission to use, copy, modify, and distribute this software
 * and its documentation for any purpose and without fee is
 * hereby granted, provided that the above copyright notice
 * appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation,
 * and that the names of M.I.T. and the M.I.T. S.I.P.B. not be
 * used in advertising or publicity pertaining to distribution
 * of the software without specific, written prior permission.
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. and the M.I.T. S.I.P.B. make no representations about
 * the suitability of this software for any purpose.  It is
 * provided "as is" without express or implied warranty.
 */
#[c2rust::src_loc = "27:24"]
static mut et_list: *mut et_list = 0 as *const et_list as *mut et_list;
#[c2rust::src_loc = "28:19"]
static mut et_list_lock: k5_mutex_t =
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
#[c2rust::src_loc = "29:12"]
static mut terminated: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "31:1"]
static mut com_err_initialize__once: k5_init_t =
    unsafe {
        {
            let mut init =
                k5_init_t{once:
                              {
                                  let mut init =
                                      k5_once_t{o: 0 as libc::c_int,
                                                n:
                                                    2 as libc::c_int as
                                                        k5_os_nothread_once_t,};
                                  init
                              },
                          error: 0 as libc::c_int,
                          did_run: 0 as libc::c_int,
                          fn_0:
                              Some(com_err_initialize__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
#[c2rust::src_loc = "31:1"]
unsafe extern "C" fn com_err_initialize__aux() {
    com_err_initialize__once.did_run = 1 as libc::c_int;
    com_err_initialize__once.error = com_err_initialize();
}
/* for debugging shlib fini sequence errors */
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn com_err_initialize() -> libc::c_int {
    let mut err: libc::c_int = 0;
    terminated = 0 as libc::c_int;
    err = k5_mutex_finish_init(&mut et_list_lock);
    if err != 0 { return err }
    err = k5_mutex_finish_init(&mut com_err_hook_lock);
    if err != 0 { return err }
    err =
        krb5int_key_register(K5_KEY_COM_ERR,
                             Some(free as
                                      unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> ()));
    if err != 0 { return err }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn get_thread_buffer() -> *mut libc::c_char {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    cp = krb5int_getspecific(K5_KEY_COM_ERR) as *mut libc::c_char;
    if cp.is_null() {
        cp =
            malloc(1024 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        if cp.is_null() { return 0 as *mut libc::c_char }
        if krb5int_setspecific(K5_KEY_COM_ERR, cp as *mut libc::c_void) !=
               0 as libc::c_int {
            free(cp as *mut libc::c_void);
            return 0 as *mut libc::c_char
        }
    }
    return cp;
}
#[no_mangle]
#[c2rust::src_loc = "101:1"]
pub unsafe extern "C" fn error_message(mut code: libc::c_long)
 -> *const libc::c_char {
    let mut current_block: u64;
    let mut offset: libc::c_ulong = 0;
    let mut l_offset: libc::c_ulong = 0;
    let mut e: *mut et_list = 0 as *mut et_list;
    let mut table_num: libc::c_ulong = 0;
    let mut started: libc::c_int = 0 as libc::c_int;
    let mut divisor: libc::c_uint = 100 as libc::c_int as libc::c_uint;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut table: *const error_table = 0 as *const error_table;
    if ({
            let mut k5int_i: *mut k5_init_t =
                &mut com_err_initialize__once as *mut k5_init_t;
            let mut k5int_err: libc::c_int =
                k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
            if k5int_err != 0 {
                k5int_err
            } else {
                if (*k5int_i).did_run != 0 as libc::c_int {
                } else {
                    __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"error_message.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  113 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 32],
                                                            &[libc::c_char; 32]>(b"const char *error_message(long)\x00")).as_ptr());
                }
                (*k5int_i).error
            }
        }) != 0 {
        return 0 as *const libc::c_char
    }
    l_offset =
        code as libc::c_ulong &
            (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as
                libc::c_ulong;
    offset = l_offset;
    table_num =
        (code as libc::c_ulong).wrapping_sub(l_offset) &
            0xffffffff as libc::c_ulong;
    if table_num == 0 as libc::c_int as libc::c_ulong {
        if !(code == 0 as libc::c_int as libc::c_long) {
            /* This could trip if int is 16 bits.  */
            if code as libc::c_int as libc::c_ulong != code as libc::c_ulong {
                abort();
            }
            cp = get_thread_buffer();
            if !cp.is_null() &&
                   k5_strerror_r(code as libc::c_int, cp,
                                 1024 as libc::c_int as size_t) ==
                       0 as libc::c_int {
                return cp
            }
            return strerror(code as libc::c_int)
        }
    } else {
        k5_mutex_lock(&mut et_list_lock);
        e = et_list;
        loop  {
            if e.is_null() { current_block = 676744132399875722; break ; }
            if (*(*e).table).base as libc::c_ulong &
                   0xffffffff as libc::c_ulong == table_num {
                table = (*e).table;
                current_block = 4667569057004253068;
                break ;
            } else { e = (*e).next }
        }
        match current_block {
            4667569057004253068 => {
                k5_mutex_unlock(&mut et_list_lock);
                /* This is the right table */
                /* This could trip if int is 16 bits.  */
                if !(offset as libc::c_uint as libc::c_ulong != offset) {
                    if !((*table).n_msgs <= offset as libc::c_uint) {
                        /* If there's a string at the end of the table, it's a text domain. */
                        if !(*(*table).msgs.offset((*table).n_msgs as
                                                       isize)).is_null() {
                            return dgettext(*(*table).msgs.offset((*table).n_msgs
                                                                      as
                                                                      isize),
                                            *(*table).msgs.offset(offset as
                                                                      isize))
                        } else {
                            return *(*table).msgs.offset(offset as isize)
                        }
                    }
                }
            }
            _ => { }
        }
        k5_mutex_unlock(&mut et_list_lock);
    }
    cp = get_thread_buffer();
    if cp.is_null() {
        return b"Unknown error code\x00" as *const u8 as *const libc::c_char
    }
    cp1 = cp;
    krb5int_strlcpy(cp,
                    b"Unknown code \x00" as *const u8 as *const libc::c_char,
                    1024 as libc::c_int as size_t);
    cp =
        cp.offset((::std::mem::size_of::<[libc::c_char; 14]>() as
                       libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                      isize);
    if table_num != 0 as libc::c_long as libc::c_ulong {
        error_table_name_r(table_num, cp);
        while *cp as libc::c_int != '\u{0}' as i32 { cp = cp.offset(1) }
        let fresh0 = cp;
        cp = cp.offset(1);
        *fresh0 = ' ' as i32 as libc::c_char
    }
    while divisor > 1 as libc::c_int as libc::c_uint {
        if started != 0 as libc::c_int || offset >= divisor as libc::c_ulong {
            let fresh1 = cp;
            cp = cp.offset(1);
            *fresh1 =
                ('0' as i32 as
                     libc::c_ulong).wrapping_add(offset.wrapping_div(divisor
                                                                         as
                                                                         libc::c_ulong))
                    as libc::c_char;
            offset = offset.wrapping_rem(divisor as libc::c_ulong);
            started += 1
        }
        divisor = divisor.wrapping_div(10 as libc::c_int as libc::c_uint)
    }
    let fresh2 = cp;
    cp = cp.offset(1);
    *fresh2 =
        ('0' as i32 as libc::c_ulong).wrapping_add(offset) as libc::c_char;
    *cp = '\u{0}' as i32 as libc::c_char;
    return cp1;
}
/*@modifies internalState@*/
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn add_error_table(mut et: *const error_table)
 -> errcode_t {
    let mut e: *mut et_list = 0 as *mut et_list;
    if ({
            let mut k5int_i: *mut k5_init_t =
                &mut com_err_initialize__once as *mut k5_init_t;
            let mut k5int_err: libc::c_int =
                k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
            if k5int_err != 0 {
                k5int_err
            } else {
                if (*k5int_i).did_run != 0 as libc::c_int {
                } else {
                    __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"error_message.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  256 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 54],
                                                            &[libc::c_char; 54]>(b"errcode_t add_error_table(const struct error_table *)\x00")).as_ptr());
                }
                (*k5int_i).error
            }
        }) != 0 {
        return 0 as libc::c_int as errcode_t
    }
    e =
        malloc(::std::mem::size_of::<et_list>() as libc::c_ulong) as
            *mut et_list;
    if e.is_null() { return 12 as libc::c_int as errcode_t }
    (*e).table = et;
    k5_mutex_lock(&mut et_list_lock);
    (*e).next = et_list;
    et_list = e;
    /* If there are two strings at the end of the table, they are a text domain
     * and locale dir, and we are supposed to call bindtextdomain. */
    if !(*(*et).msgs.offset((*et).n_msgs as isize)).is_null() &&
           !(*(*et).msgs.offset((*et).n_msgs.wrapping_add(1 as libc::c_int as
                                                              libc::c_uint) as
                                    isize)).is_null() {
        bindtextdomain(*(*et).msgs.offset((*et).n_msgs as isize),
                       *(*et).msgs.offset((*et).n_msgs.wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                              as isize));
    }
    k5_mutex_unlock(&mut et_list_lock);
    return 0 as libc::c_int as errcode_t;
}
/*@modifies internalState@*/
#[no_mangle]
#[c2rust::src_loc = "278:1"]
pub unsafe extern "C" fn remove_error_table(mut et: *const error_table)
 -> errcode_t {
    let mut ep: *mut *mut et_list = 0 as *mut *mut et_list;
    let mut e: *mut et_list = 0 as *mut et_list;
    if ({
            let mut k5int_i: *mut k5_init_t =
                &mut com_err_initialize__once as *mut k5_init_t;
            let mut k5int_err: libc::c_int =
                k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
            if k5int_err != 0 {
                k5int_err
            } else {
                if (*k5int_i).did_run != 0 as libc::c_int {
                } else {
                    __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"error_message.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  283 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 57],
                                                            &[libc::c_char; 57]>(b"errcode_t remove_error_table(const struct error_table *)\x00")).as_ptr());
                }
                (*k5int_i).error
            }
        }) != 0 {
        return 0 as libc::c_int as errcode_t
    }
    k5_mutex_lock(&mut et_list_lock);
    /* Remove the entry that matches the error table instance. */
    ep = &mut et_list;
    while !(*ep).is_null() {
        if (**ep).table == et {
            e = *ep;
            *ep = (*e).next;
            free(e as *mut libc::c_void);
            k5_mutex_unlock(&mut et_list_lock);
            return 0 as libc::c_int as errcode_t
        }
        ep = &mut (**ep).next
    }
    k5_mutex_unlock(&mut et_list_lock);
    return 2 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "301:1"]
pub unsafe extern "C" fn com_err_finish_init() -> libc::c_int {
    return ({
                let mut k5int_i: *mut k5_init_t =
                    &mut com_err_initialize__once;
                let mut k5int_err: libc::c_int =
                    k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
                if k5int_err != 0 {
                    k5int_err
                } else {
                    if (*k5int_i).did_run != 0 as libc::c_int {
                    } else {
                        __assert_fail(b"k5int_i->did_run != 0\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"error_message.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      303 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 26],
                                                                &[libc::c_char; 26]>(b"int com_err_finish_init()\x00")).as_ptr());
                    }
                    (*k5int_i).error
                }
            });
}
