use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:49"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:49"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:49"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:49"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:49"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:49"]
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
    use super::pthreadtypes_h::pthread_mutex_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/netinet/in.h:51"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/support/cache-addrinfo.h:56"]
pub mod cache_addrinfo_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct face {
        pub addrs4: *mut in_addr,
        pub addrs6: *mut in6_addr,
        pub naddrs4: libc::c_uint,
        pub naddrs6: libc::c_uint,
        pub expiration: time_t,
        pub canonname: *mut libc::c_char,
        pub name: *mut libc::c_char,
        pub next: *mut face,
    }
    /* fake addrinfo cache */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:8"]
    pub struct fac {
        pub lock: k5_mutex_t,
        pub data: *mut face,
    }
    use super::in_h::{in_addr, in6_addr};
    use super::time_t_h::time_t;
    use super::k5_thread_h::k5_mutex_t;
}
pub use self::types_h::{__uint8_t, __uint16_t, __uint32_t, __time_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::time_t_h::time_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_finish_init,
                            k5_os_mutex_destroy};
pub use self::in_h::{in6_addr, C2RustUnnamed, in_addr, in_addr_t};
pub use self::cache_addrinfo_h::{face, fac};
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
/* Stuff that needs initialization for fake-addrinfo.c.

   Separated out, so that static linking against this library doesn't
   require pulling in socket/nsl/whatever libraries for code not using
   getaddrinfo.  */
#[no_mangle]
#[c2rust::src_loc = "58:12"]
pub static mut krb5int_fac: fac =
    {
        let mut init =
            fac{lock:
                    pthread_mutex_t{__data:
                                        {
                                            let mut init =
                                                __pthread_mutex_s{__lock:
                                                                      0 as
                                                                          libc::c_int,
                                                                  __count:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint,
                                                                  __owner:
                                                                      0 as
                                                                          libc::c_int,
                                                                  __nusers:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint,
                                                                  __kind:
                                                                      0 as
                                                                          libc::c_int,
                                                                  __spins:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_short,
                                                                  __elision:
                                                                      0 as
                                                                          libc::c_int
                                                                          as
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
                                        },},
                data: 0 as *const face as *mut face,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "60:1"]
pub unsafe extern "C" fn krb5int_init_fac() -> libc::c_int {
    return k5_mutex_finish_init(&mut krb5int_fac.lock);
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn krb5int_fini_fac() {
    k5_os_mutex_destroy(&mut krb5int_fac.lock);
}
