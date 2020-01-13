use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:28"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:28"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "27:1"]
    pub type pthread_t = libc::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub union pthread_mutexattr_t {
        pub __size: [libc::c_char; 4],
        pub __align: libc::c_int,
    }
    #[c2rust::src_loc = "49:1"]
    pub type pthread_key_t = libc::c_uint;
    #[c2rust::src_loc = "53:1"]
    pub type pthread_once_t = libc::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:7"]
    pub union pthread_attr_t {
        pub __size: [libc::c_char; 56],
        pub __align: libc::c_long,
    }
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:28"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:28"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:28"]
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
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
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
    use super::pthreadtypes_h::{pthread_once_t, pthread_mutex_t};
    use super::{k5_os_mutex_init, k5_os_mutex_lock, k5_os_mutex_unlock};
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:28"]
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
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:28"]
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
#[c2rust::header_src = "/usr/include/assert.h:28"]
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
#[c2rust::header_src = "/usr/include/pthread.h:28"]
pub mod pthread_h {
    use super::pthreadtypes_h::{pthread_t, pthread_attr_t, pthread_once_t,
                                pthread_mutex_t, pthread_mutexattr_t,
                                pthread_key_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "234:1"]
        pub fn pthread_create(__newthread: *mut pthread_t,
                              __attr: *const pthread_attr_t,
                              __start_routine:
                                  Option<unsafe extern "C" fn(_:
                                                                  *mut libc::c_void)
                                             -> *mut libc::c_void>,
                              __arg: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn pthread_join(__th: pthread_t,
                            __thread_return: *mut *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "276:1"]
        pub fn pthread_self() -> pthread_t;
        #[no_mangle]
        #[c2rust::src_loc = "279:1"]
        pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "495:1"]
        pub fn pthread_once(__once_control: *mut pthread_once_t,
                            __init_routine:
                                Option<unsafe extern "C" fn() -> ()>)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "750:1"]
        pub fn pthread_mutex_init(__mutex: *mut pthread_mutex_t,
                                  __mutexattr: *const pthread_mutexattr_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "755:1"]
        pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "763:1"]
        pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "781:1"]
        pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1148:1"]
        pub fn pthread_key_create(__key: *mut pthread_key_t,
                                  __destr_function:
                                      Option<unsafe extern "C" fn(_:
                                                                      *mut libc::c_void)
                                                 -> ()>) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1153:1"]
        pub fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1156:1"]
        pub fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "1159:1"]
        pub fn pthread_setspecific(__key: pthread_key_t,
                                   __pointer: *const libc::c_void)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/support/supp-int.h:30"]
pub mod supp_int_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:1"]
        pub fn krb5int_err_init() -> libc::c_int;
    }
    /* KRB5_SUPP_INT_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/support/cache-addrinfo.h:404"]
pub mod cache_addrinfo_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2004 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
        /*
 * Approach overview:
 *
 * If a system version is available but buggy, save handles to it,
 * redefine the names to refer to static functions defined here, and
 * in those functions, call the system versions and fix up the
 * returned data.  Use the native data structures and flag values.
 *
 * If no system version exists, use gethostby* and fake it.  Define
 * the data structures and flag values locally.
 *
 *
 * On macOS, getaddrinfo results aren't cached (though gethostbyname
 * results are), so we need to build a cache here.  Now things are
 * getting really messy.  Because the cache is in use, we use
 * getservbyname, and throw away thread safety.  (Not that the cache
 * is thread safe, but when we get locking support, that'll be dealt
 * with.)  This code needs tearing down and rebuilding, soon.
 *
 *
 * Note that recent Windows developers' code has an interesting hack:
 * When you include the right header files, with the right set of
 * macros indicating system versions, you'll get an inline function
 * that looks for getaddrinfo (or whatever) in the system library, and
 * calls it if it's there.  If it's not there, it fakes it with
 * gethostby* calls.
 *
 * We're taking a simpler approach: A system provides these routines or
 * it does not.
 *
 * Someday, we may want to take into account different versions (say,
 * different revs of GNU libc) where some are broken in one way, and
 * some work or are broken in another way.  Cross that bridge when we
 * come to it.
 */
        /* To do, maybe:
 *
 * + For AIX 4.3.3, using the RFC 2133 definition: Implement
 *   AI_NUMERICHOST.  It's not defined in the header file.
 *
 *   For certain (old?) versions of GNU libc, AI_NUMERICHOST is
 *   defined but not implemented.
 *
 * + Use gethostbyname2, inet_aton and other IPv6 or thread-safe
 *   functions if available.  But, see
 *   https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=135182 for one
 *   gethostbyname2 problem on Linux.  And besides, if a platform is
 *   supporting IPv6 at all, they really should be doing getaddrinfo
 *   by now.
 *
 * + inet_ntop, inet_pton
 *
 * + Conditionally export/import the function definitions, so a
 *   library can have a single copy instead of multiple.
 *
 * + Upgrade host requirements to include working implementations of
 *   these functions, and throw all this away.  Pleeease?  :-)
 */
        /* fake addrinfo cache */
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn krb5int_init_fac() -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__off_t, __off64_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::{pthread_t, pthread_mutexattr_t, pthread_key_t,
                               pthread_once_t, pthread_attr_t,
                               pthread_mutex_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_nothread_once_t, k5_once_t, k5_os_mutex,
                            k5_mutex_t, k5_key_t, K5_KEY_MAX,
                            K5_KEY_GSS_SPNEGO_STATUS,
                            K5_KEY_GSS_KRB5_ERROR_MESSAGE,
                            K5_KEY_GSS_KRB5_CCACHE_NAME,
                            K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                            K5_KEY_COM_ERR, k5_mutex_init,
                            k5_mutex_finish_init, k5_mutex_lock,
                            k5_mutex_unlock};
pub use self::k5_platform_h::k5_init_t;
use self::string_h::strerror;
use self::stdlib_h::{malloc, free};
use self::stdio_h::{stderr, fprintf};
use self::assert_h::__assert_fail;
use self::pthread_h::{pthread_create, pthread_join, pthread_self,
                      pthread_equal, pthread_once, pthread_mutex_init,
                      pthread_mutex_destroy, pthread_mutex_lock,
                      pthread_mutex_unlock, pthread_key_create,
                      pthread_key_delete, pthread_getspecific,
                      pthread_setspecific};
use self::supp_int_h::krb5int_err_init;
use self::cache_addrinfo_h::krb5int_init_fac;
/* This is not safe yet!

   Thread termination concurrent with key deletion can cause two
   threads to interfere.  It's a bit tricky, since one of the threads
   will want to remove this structure from the list being walked by
   the other.

   Other cases, like looking up data while the library owning the key
   is in the process of being unloaded, we don't worry about.  */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "109:8"]
pub struct tsd_block {
    pub next: *mut tsd_block,
    pub values: [*mut libc::c_void; 5],
}
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn krb5int_thread_support_init__aux() {
    krb5int_thread_support_init__once.did_run = 1 as libc::c_int;
    krb5int_thread_support_init__once.error = krb5int_thread_support_init();
}
#[c2rust::src_loc = "32:1"]
static mut krb5int_thread_support_init__once: k5_init_t =
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
                              Some(krb5int_thread_support_init__aux as
                                       unsafe extern "C" fn() -> ()),};
            init
        }
    };
/* no thread support */
/* POSIX threads */
/* Must support register/delete/register sequence, e.g., if krb5 is
   loaded so this support code stays in the process, and gssapi is
   loaded, unloaded, and loaded again.  */
#[c2rust::src_loc = "95:19"]
static mut key_lock: k5_mutex_t =
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
#[c2rust::src_loc = "96:15"]
static mut destructors:
       [Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>; 5] =
    [None; 5];
#[c2rust::src_loc = "97:22"]
static mut destructors_set: [libc::c_uchar; 5] = [0; 5];
#[c2rust::src_loc = "129:21"]
static mut flag_pthread_loaded: libc::c_int = -(1 as libc::c_int);
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn loaded_test_aux() {
    if flag_pthread_loaded == -(1 as libc::c_int) {
        ::std::ptr::write_volatile(&mut flag_pthread_loaded as
                                       *mut libc::c_int, 1 as libc::c_int)
    } else {
        /* Could we have been called twice?  */
        ::std::ptr::write_volatile(&mut flag_pthread_loaded as
                                       *mut libc::c_int, 0 as libc::c_int)
    };
}
#[c2rust::src_loc = "138:23"]
static mut loaded_test_once: pthread_once_t = 0 as libc::c_int;
/* This function used to be referenced from elsewhere in the tree, but is now
 * only used internally.  Keep it linker-visible for now. */
#[no_mangle]
#[c2rust::src_loc = "139:1"]
pub unsafe extern "C" fn krb5int_pthread_loaded() -> libc::c_int {
    let mut x: libc::c_int = flag_pthread_loaded;
    if x != -(1 as libc::c_int) { return x }
    if Some(pthread_getspecific as
                unsafe extern "C" fn(_: pthread_key_t)
                    -> *mut libc::c_void).is_none() ||
           Some(pthread_setspecific as
                    unsafe extern "C" fn(_: pthread_key_t,
                                         _: *const libc::c_void)
                        -> libc::c_int).is_none() ||
           Some(pthread_key_create as
                    unsafe extern "C" fn(_: *mut pthread_key_t,
                                         _:
                                             Option<unsafe extern "C" fn(_:
                                                                             *mut libc::c_void)
                                                        -> ()>)
                        -> libc::c_int).is_none() ||
           Some(pthread_key_delete as
                    unsafe extern "C" fn(_: pthread_key_t)
                        -> libc::c_int).is_none() ||
           Some(pthread_once as
                    unsafe extern "C" fn(_: *mut pthread_once_t,
                                         _:
                                             Option<unsafe extern "C" fn()
                                                        -> ()>)
                        -> libc::c_int).is_none() ||
           Some(pthread_mutex_lock as
                    unsafe extern "C" fn(_: *mut pthread_mutex_t)
                        -> libc::c_int).is_none() ||
           Some(pthread_mutex_unlock as
                    unsafe extern "C" fn(_: *mut pthread_mutex_t)
                        -> libc::c_int).is_none() ||
           Some(pthread_mutex_destroy as
                    unsafe extern "C" fn(_: *mut pthread_mutex_t)
                        -> libc::c_int).is_none() ||
           Some(pthread_mutex_init as
                    unsafe extern "C" fn(_: *mut pthread_mutex_t,
                                         _: *const pthread_mutexattr_t)
                        -> libc::c_int).is_none() ||
           Some(pthread_self as unsafe extern "C" fn() -> pthread_t).is_none()
           ||
           Some(pthread_equal as
                    unsafe extern "C" fn(_: pthread_t, _: pthread_t)
                        -> libc::c_int).is_none() ||
           Some(pthread_create as
                    unsafe extern "C" fn(_: *mut pthread_t,
                                         _: *const pthread_attr_t,
                                         _:
                                             Option<unsafe extern "C" fn(_:
                                                                             *mut libc::c_void)
                                                        -> *mut libc::c_void>,
                                         _: *mut libc::c_void)
                        -> libc::c_int).is_none() ||
           Some(pthread_join as
                    unsafe extern "C" fn(_: pthread_t,
                                         _: *mut *mut libc::c_void)
                        -> libc::c_int).is_none() ||
           pthread_once(&mut loaded_test_once,
                        Some(loaded_test_aux as unsafe extern "C" fn() -> ()))
               != 0 as libc::c_int ||
           pthread_once(&mut loaded_test_once,
                        Some(loaded_test_aux as unsafe extern "C" fn() -> ()))
               != 0 as libc::c_int || flag_pthread_loaded < 0 as libc::c_int {
        ::std::ptr::write_volatile(&mut flag_pthread_loaded as
                                       *mut libc::c_int, 0 as libc::c_int);
        return 0 as libc::c_int
    }
    /* If we wanted to be super-paranoid, we could try testing whether
       pthread_get/setspecific work, too.  I don't know -- so far --
       of any system with non-functional stubs for those.  */
    return flag_pthread_loaded;
}
#[c2rust::src_loc = "178:25"]
static mut tsd_if_single: tsd_block =
    tsd_block{next: 0 as *const tsd_block as *mut tsd_block,
              values: [0 as *const libc::c_void as *mut libc::c_void; 5],};
#[c2rust::src_loc = "190:22"]
static mut key: pthread_key_t = 0;
#[c2rust::src_loc = "193:1"]
unsafe extern "C" fn thread_termination(mut tptr: *mut libc::c_void) {
    let mut i: libc::c_int = 0;
    let mut pass: libc::c_int = 0;
    let mut none_found: libc::c_int = 0;
    let mut t: *mut tsd_block = tptr as *mut tsd_block;
    k5_mutex_lock(&mut key_lock);
    /*
     * Make multiple passes in case, for example, a libkrb5 cleanup
     * function wants to print out an error message, which causes
     * com_err to allocate a thread-specific buffer, after we just
     * freed up the old one.
     *
     * Shouldn't actually happen, if we're careful, but check just in
     * case.
     */
    pass = 0 as libc::c_int;
    none_found = 0 as libc::c_int;
    while pass < 4 as libc::c_int && none_found == 0 {
        none_found = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while i < K5_KEY_MAX as libc::c_int {
            if destructors_set[i as usize] as libc::c_int != 0 &&
                   destructors[i as usize].is_some() &&
                   !(*t).values[i as usize].is_null() {
                let mut v: *mut libc::c_void = (*t).values[i as usize];
                (*t).values[i as usize] = 0 as *mut libc::c_void;
                Some((*destructors.as_mut_ptr().offset(i as
                                                           isize)).expect("non-null function pointer")).expect("non-null function pointer")(v);
                none_found = 0 as libc::c_int
            }
            i += 1
        }
    }
    free(t as *mut libc::c_void);
    k5_mutex_unlock(&mut key_lock);
    /* remove thread from global linked list */
}
/* no threads vs Win32 vs POSIX */
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn krb5int_getspecific(mut keynum: k5_key_t)
 -> *mut libc::c_void {
    let mut t: *mut tsd_block = 0 as *mut tsd_block;
    let mut err: libc::c_int = 0;
    err =
        ({
             let mut k5int_i: *mut k5_init_t =
                 &mut krb5int_thread_support_init__once;
             let mut k5int_err: libc::c_int =
                 k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
             if k5int_err != 0 {
                 k5int_err
             } else {
                 if (*k5int_i).did_run != 0 as libc::c_int {
                 } else {
                     __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8
                                       as *const libc::c_char,
                                   b"threads.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   236 as libc::c_int as libc::c_uint,
                                   (*::std::mem::transmute::<&[u8; 36],
                                                             &[libc::c_char; 36]>(b"void *krb5int_getspecific(k5_key_t)\x00")).as_ptr());
                 }
                 (*k5int_i).error
             }
         });
    if err != 0 { return 0 as *mut libc::c_void }
    if destructors_set[keynum as usize] as libc::c_int == 1 as libc::c_int {
    } else {
        __assert_fail(b"destructors_set[keynum] == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"threads.c\x00" as *const u8 as *const libc::c_char,
                      240 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void *krb5int_getspecific(k5_key_t)\x00")).as_ptr());
    }
    /* POSIX */
    if krb5int_pthread_loaded() != 0 {
        t = pthread_getspecific(key) as *mut tsd_block
    } else { t = &mut tsd_if_single }
    if t.is_null() { return 0 as *mut libc::c_void }
    return (*t).values[keynum as usize];
}
#[no_mangle]
#[c2rust::src_loc = "264:1"]
pub unsafe extern "C" fn krb5int_setspecific(mut keynum: k5_key_t,
                                             mut value: *mut libc::c_void)
 -> libc::c_int {
    let mut t: *mut tsd_block = 0 as *mut tsd_block;
    let mut err: libc::c_int = 0;
    err =
        ({
             let mut k5int_i: *mut k5_init_t =
                 &mut krb5int_thread_support_init__once;
             let mut k5int_err: libc::c_int =
                 k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
             if k5int_err != 0 {
                 k5int_err
             } else {
                 if (*k5int_i).did_run != 0 as libc::c_int {
                 } else {
                     __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8
                                       as *const libc::c_char,
                                   b"threads.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   269 as libc::c_int as libc::c_uint,
                                   (*::std::mem::transmute::<&[u8; 42],
                                                             &[libc::c_char; 42]>(b"int krb5int_setspecific(k5_key_t, void *)\x00")).as_ptr());
                 }
                 (*k5int_i).error
             }
         });
    if err != 0 { return err }
    if destructors_set[keynum as usize] as libc::c_int == 1 as libc::c_int {
    } else {
        __assert_fail(b"destructors_set[keynum] == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"threads.c\x00" as *const u8 as *const libc::c_char,
                      273 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"int krb5int_setspecific(k5_key_t, void *)\x00")).as_ptr());
    }
    /* POSIX */
    if krb5int_pthread_loaded() != 0 {
        t = pthread_getspecific(key) as *mut tsd_block;
        if t.is_null() {
            let mut i: libc::c_int = 0;
            t =
                malloc(::std::mem::size_of::<tsd_block>() as libc::c_ulong) as
                    *mut tsd_block;
            if t.is_null() { return 12 as libc::c_int }
            i = 0 as libc::c_int;
            while i < K5_KEY_MAX as libc::c_int {
                (*t).values[i as usize] = 0 as *mut libc::c_void;
                i += 1
            }
            /* add to global linked list */
            (*t).next = 0 as *mut tsd_block;
            err = pthread_setspecific(key, t as *const libc::c_void);
            if err != 0 { free(t as *mut libc::c_void); return err }
        }
    } else { t = &mut tsd_if_single }
    (*t).values[keynum as usize] = value;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "327:1"]
pub unsafe extern "C" fn krb5int_key_register(mut keynum: k5_key_t,
                                              mut destructor:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  *mut libc::c_void)
                                                             -> ()>)
 -> libc::c_int {
    let mut err: libc::c_int = 0;
    err =
        ({
             let mut k5int_i: *mut k5_init_t =
                 &mut krb5int_thread_support_init__once;
             let mut k5int_err: libc::c_int =
                 k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
             if k5int_err != 0 {
                 k5int_err
             } else {
                 if (*k5int_i).did_run != 0 as libc::c_int {
                 } else {
                     __assert_fail(b"k5int_i->did_run != 0\x00" as *const u8
                                       as *const libc::c_char,
                                   b"threads.c\x00" as *const u8 as
                                       *const libc::c_char,
                                   331 as libc::c_int as libc::c_uint,
                                   (*::std::mem::transmute::<&[u8; 53],
                                                             &[libc::c_char; 53]>(b"int krb5int_key_register(k5_key_t, void (*)(void *))\x00")).as_ptr());
                 }
                 (*k5int_i).error
             }
         });
    if err != 0 { return err }
    /* POSIX */
    k5_mutex_lock(&mut key_lock);
    if destructors_set[keynum as usize] as libc::c_int == 0 as libc::c_int {
    } else {
        __assert_fail(b"destructors_set[keynum] == 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"threads.c\x00" as *const u8 as *const libc::c_char,
                      353 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"int krb5int_key_register(k5_key_t, void (*)(void *))\x00")).as_ptr());
    }
    destructors_set[keynum as usize] = 1 as libc::c_int as libc::c_uchar;
    destructors[keynum as usize] = destructor;
    k5_mutex_unlock(&mut key_lock);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "362:1"]
pub unsafe extern "C" fn krb5int_key_delete(mut keynum: k5_key_t)
 -> libc::c_int {
    /* POSIX */
    /* XXX RESOURCE LEAK: Need to destroy the allocated objects first!  */
    k5_mutex_lock(&mut key_lock);
    if destructors_set[keynum as usize] as libc::c_int == 1 as libc::c_int {
    } else {
        __assert_fail(b"destructors_set[keynum] == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"threads.c\x00" as *const u8 as *const libc::c_char,
                      389 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"int krb5int_key_delete(k5_key_t)\x00")).as_ptr());
    }
    destructors_set[keynum as usize] = 0 as libc::c_int as libc::c_uchar;
    destructors[keynum as usize] = None;
    k5_mutex_unlock(&mut key_lock);
    return 0 as libc::c_int;
}
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
#[c2rust::src_loc = "399:1"]
pub unsafe extern "C" fn krb5int_call_thread_support_init() -> libc::c_int {
    return ({
                let mut k5int_i: *mut k5_init_t =
                    &mut krb5int_thread_support_init__once;
                let mut k5int_err: libc::c_int =
                    k5_once(&mut (*k5int_i).once, (*k5int_i).fn_0);
                if k5int_err != 0 {
                    k5int_err
                } else {
                    if (*k5int_i).did_run != 0 as libc::c_int {
                    } else {
                        __assert_fail(b"k5int_i->did_run != 0\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"threads.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      401 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 43],
                                                                &[libc::c_char; 43]>(b"int krb5int_call_thread_support_init(void)\x00")).as_ptr());
                    }
                    (*k5int_i).error
                }
            });
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/threads.c - Portable thread support */
/*
 * Copyright 2004,2005,2006,2007,2008 by the Massachusetts Institute of
 * Technology.  All Rights Reserved.
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
#[c2rust::src_loc = "406:1"]
unsafe extern "C" fn krb5int_thread_support_init() -> libc::c_int {
    let mut err: libc::c_int = 0;
    /* POSIX */
    err = k5_mutex_finish_init(&mut key_lock);
    if err != 0 { return err }
    if krb5int_pthread_loaded() != 0 {
        err =
            pthread_key_create(&mut key,
                               Some(thread_termination as
                                        unsafe extern "C" fn(_:
                                                                 *mut libc::c_void)
                                            -> ()));
        if err != 0 { return err }
    }
    err = krb5int_init_fac();
    if err != 0 { return err }
    err = krb5int_err_init();
    if err != 0 { return err }
    return 0 as libc::c_int;
}
/* Mutex allocation functions, for use in plugins that may not know
   what options a given set of libraries was compiled with.  */
#[no_mangle]
#[c2rust::src_loc = "483:1"]
pub unsafe extern "C" fn krb5int_mutex_alloc(mut m: *mut *mut k5_mutex_t)
 -> libc::c_int {
    let mut ptr: *mut k5_mutex_t = 0 as *mut k5_mutex_t;
    let mut err: libc::c_int = 0;
    ptr =
        malloc(::std::mem::size_of::<k5_mutex_t>() as libc::c_ulong) as
            *mut k5_mutex_t;
    if ptr.is_null() { return 12 as libc::c_int }
    err = k5_mutex_init(ptr);
    if err != 0 { free(ptr as *mut libc::c_void); return err }
    *m = ptr;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "501:1"]
pub unsafe extern "C" fn krb5int_mutex_free(mut m: *mut k5_mutex_t) {
    k5_os_mutex_destroy(m);
    free(m as *mut libc::c_void);
}
/* Callable versions of the various macros.  */
#[no_mangle]
#[c2rust::src_loc = "509:1"]
pub unsafe extern "C" fn krb5int_mutex_lock(mut m: *mut k5_mutex_t) {
    k5_mutex_lock(m);
}
/* is pthreads always available? */
/* Thread-specific data; implemented in a support file, because we'll
   need to keep track of some global data for cleanup purposes.

   Note that the callback function type is such that the C library
   routine free() is a valid callback.  */
/* rename shorthand symbols for export */
#[no_mangle]
#[c2rust::src_loc = "514:1"]
pub unsafe extern "C" fn krb5int_mutex_unlock(mut m: *mut k5_mutex_t) {
    k5_mutex_unlock(m);
}
#[no_mangle]
#[c2rust::src_loc = "522:1"]
pub unsafe extern "C" fn k5_os_mutex_init(mut m: *mut k5_os_mutex)
 -> libc::c_int {
    if krb5int_pthread_loaded() != 0 {
        return pthread_mutex_init(m, 0 as *const pthread_mutexattr_t)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "531:1"]
pub unsafe extern "C" fn k5_os_mutex_destroy(mut m: *mut k5_os_mutex)
 -> libc::c_int {
    if krb5int_pthread_loaded() != 0 {
        return pthread_mutex_destroy(m)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "540:1"]
pub unsafe extern "C" fn k5_os_mutex_lock(mut m: *mut k5_os_mutex)
 -> libc::c_int {
    if krb5int_pthread_loaded() != 0 {
        return pthread_mutex_lock(m)
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "549:1"]
pub unsafe extern "C" fn k5_os_mutex_unlock(mut m: *mut k5_os_mutex)
 -> libc::c_int {
    if krb5int_pthread_loaded() != 0 {
        return pthread_mutex_unlock(m)
    } else { return 0 as libc::c_int };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-thread.h - Preliminary portable thread support */
/*
 * Copyright 2004,2005,2006,2007,2008 by the Massachusetts Institute of Technology.
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
/* Interface (tentative):

     Mutex support:

     // Between these two, we should be able to do pure compile-time
     // and pure run-time initialization.
     //   POSIX:   partial initializer is PTHREAD_MUTEX_INITIALIZER,
     //            finish does nothing
     //   Windows: partial initializer is an invalid handle,
     //            finish does the real initialization work
     k5_mutex_t foo_mutex = K5_MUTEX_PARTIAL_INITIALIZER;
     int k5_mutex_finish_init(k5_mutex_t *);
     // for dynamic allocation
     int k5_mutex_init(k5_mutex_t *);
     // Must work for both kinds of alloc, even if it means adding flags.
     int k5_mutex_destroy(k5_mutex_t *);

     // As before.
     int k5_mutex_lock(k5_mutex_t *);
     int k5_mutex_unlock(k5_mutex_t *);

     In each library, one new function to finish the static mutex init,
     and any other library-wide initialization that might be desired.
     On POSIX, this function would be called via the second support
     function (see below).  On Windows, it would be called at library
     load time.  These functions, or functions they calls, should be the
     only places that k5_mutex_finish_init gets called.

     A second function or macro called at various possible "first" entry
     points which either calls pthread_once on the first function
     (POSIX), or checks some flag set by the first function (Windows),
     and possibly returns an error.  (In the non-threaded case, a simple
     flag can be used to avoid multiple invocations, and the mutexes
     don't need run-time initialization anyways.)

     A third function for library termination calls mutex_destroy on
     each mutex for the library.  This function would be called
     automatically at library unload time.  If it turns out to be needed
     at exit time for libraries that don't get unloaded, perhaps we
     should also use atexit().  Any static mutexes should be cleaned up
     with k5_mutex_destroy here.

     How does that second support function invoke the first support
     function only once?  Through something modelled on pthread_once
     that I haven't written up yet.  Probably:

     k5_once_t foo_once = K5_ONCE_INIT;
     k5_once(k5_once_t *, void (*)(void));

     For POSIX: Map onto pthread_once facility.
     For non-threaded case: A simple flag.
     For Windows: Not needed; library init code takes care of it.

     XXX: A general k5_once mechanism isn't possible for Windows,
     without faking it through named mutexes or mutexes initialized at
     startup.  I was only using it in one place outside these headers,
     so I'm dropping the general scheme.  Eventually the existing uses
     in k5-thread.h and k5-platform.h will be converted to pthread_once
     or static variables.


     Thread-specific data:

     // TSD keys are limited in number in gssapi/krb5/com_err; enumerate
     // them all.  This allows support code init to allocate the
     // necessary storage for pointers all at once, and avoids any
     // possible error in key creation.
     enum { ... } k5_key_t;
     // Register destructor function.  Called in library init code.
     int k5_key_register(k5_key_t, void (*destructor)(void *));
     // Returns NULL or data.
     void *k5_getspecific(k5_key_t);
     // Returns error if key out of bounds, or the pointer table can't
     // be allocated.  A call to k5_key_register must have happened first.
     // This may trigger the calling of pthread_setspecific on POSIX.
     int k5_setspecific(k5_key_t, void *);
     // Called in library termination code.
     // Trashes data in all threads, calling the registered destructor
     // (but calling it from the current thread).
     int k5_key_delete(k5_key_t);

     For the non-threaded version, the support code will have a static
     array indexed by k5_key_t values, and get/setspecific simply access
     the array elements.

     The TSD destructor table is global state, protected by a mutex if
     threads are enabled.


     Any actual external symbols will use the krb5int_ prefix.  The k5_
     names will be simple macros or inline functions to rename the
     external symbols, or slightly more complex ones to expand the
     implementation inline (e.g., map to POSIX versions and/or debug
     code using __FILE__ and the like).


     More to be added, perhaps.  */
/* The mutex structure we use, k5_mutex_t, is defined to some
   OS-specific bits.  The use of multiple layers of typedefs are an
   artifact resulting from debugging code we once used, implemented as
   wrappers around the OS mutex scheme.

   The OS specific bits, in k5_os_mutex, break down into three primary
   implementations, POSIX threads, Windows threads, and no thread
   support.  However, the POSIX thread version is further subdivided:
   In one case, we can determine at run time whether the thread
   library is linked into the application, and use it only if it is
   present; in the other case, we cannot, and the thread library must
   be linked in always, but can be used unconditionally.  In the
   former case, the k5_os_mutex structure needs to hold both the POSIX
   and the non-threaded versions.

   The various k5_os_mutex_* operations are the OS-specific versions,
   applied to the OS-specific data, and k5_mutex_* uses k5_os_mutex_*
   to do the OS-specific parts of the work.  */
/* Define the OS mutex bit.  */
/* Empty inline functions avoid the "statement with no effect"
   warnings, and do better type-checking than functions that don't use
   their arguments.  */
/* Values:
   2 - function has not been run
   3 - function has been run
   4 - function is being run -- deadlock detected */
/* Weak reference support, etc.

   Linux: Stub mutex routines exist, but pthread_once does not.

   Solaris <10: In libc there's a pthread_once that doesn't seem to do
   anything.  Bleah.  But pthread_mutexattr_setrobust_np is defined
   only in libpthread.  However, some version of GNU libc (Red Hat's
   Fedora Core 5, reportedly) seems to have that function, but no
   declaration, so we'd have to declare it in order to test for its
   address.  We now have tests to see if pthread_once actually works,
   so stick with that for now.

   Solaris 10: The real thread support now lives in libc, and
   libpthread is just a filter object.  So we might as well use the
   real functions unconditionally.  Since we haven't got a test for
   this property yet, we use NO_WEAK_PTHREADS defined in aclocal.m4
   depending on the OS type.

   IRIX 6.5 stub pthread support in libc is really annoying.  The
   pthread_mutex_lock function returns ENOSYS for a program not linked
   against -lpthread.  No link-time failure, no weak symbols, etc.
   The C library doesn't provide pthread_once; we can use weak
   reference support for that.

   If weak references are not available, then for now, we assume that
   the pthread support routines will always be available -- either the
   real thing, or functional stubs that merely prohibit creating
   threads.

   If we find a platform with non-functional stubs and no weak
   references, we may have to resort to some hack like dlsym on the
   symbol tables of the current process.  */
/* Can't rely on useful stubs -- see above regarding Solaris.  */
#[no_mangle]
#[c2rust::src_loc = "558:1"]
pub unsafe extern "C" fn k5_once(mut once: *mut k5_once_t,
                                 mut fn_0:
                                     Option<unsafe extern "C" fn() -> ()>)
 -> libc::c_int {
    if krb5int_pthread_loaded() != 0 {
        return pthread_once(&mut (*once).o, fn_0)
    } else {
        return if (*once).n as libc::c_int == 3 as libc::c_int {
                   0 as libc::c_int
               } else if (*once).n as libc::c_int == 2 as libc::c_int {
                   (*once).n = 4 as libc::c_int as k5_os_nothread_once_t;
                   fn_0.expect("non-null function pointer")();
                   (*once).n = 3 as libc::c_int as k5_os_nothread_once_t;
                   0 as libc::c_int
               } else {
                   if (*once).n as libc::c_int != 4 as libc::c_int {
                   } else {
                       __assert_fail(b"*(&once->n) != 4\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"threads.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     564 as libc::c_int as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 41],
                                                               &[libc::c_char; 41]>(b"int k5_once(k5_once_t *, void (*)(void))\x00")).as_ptr());
                   }
                   if (*once).n as libc::c_int == 2 as libc::c_int ||
                          (*once).n as libc::c_int == 3 as libc::c_int {
                   } else {
                       __assert_fail(b"*(&once->n) == 2 || *(&once->n) == 3\x00"
                                         as *const u8 as *const libc::c_char,
                                     b"threads.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     564 as libc::c_int as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 41],
                                                               &[libc::c_char; 41]>(b"int k5_once(k5_once_t *, void (*)(void))\x00")).as_ptr());
                   }
                   0 as libc::c_int
               }
    };
}
/* not USE_CONDITIONAL_PTHREADS */
/* USE_CONDITIONAL_PTHREADS */
