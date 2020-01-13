use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "420:1"]
        pub fn strerror_r(__errnum: libc::c_int, __buf: *mut libc::c_char,
                          __buflen: size_t) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:33"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
        /* int k5_getpwnam_r(const char *, blah blah) */
        /* POSIX */
        /* no getpwnam_r, or can't figure out #args or return type */
        /* int k5_getpwuid_r(uid_t, blah blah) */
        /* POSIX */
        /* no getpwuid_r, or can't figure out #args or return type */
        /* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
        /* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
        /* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
        /* Do nothing.  */
        /* Provide strlcpy/strlcat interfaces. */
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
pub use self::stddef_h::size_t;
use self::string_h::strerror_r;
use self::errno_h::__errno_location;
use self::k5_platform_h::krb5int_strlcpy;
/* Provide fnmatch interface. */
/* Provide [v]asprintf interfaces.  */
/* no vsnprintf */
/* have vasprintf and prototype? */
/* Return true if the snprintf return value RESULT reflects a buffer
   overflow for the buffer size SIZE.

   We cast the result to unsigned int for two reasons.  First, old
   implementations of snprintf (such as the one in Solaris 9 and
   prior) return -1 on a buffer overflow.  Casting the result to -1
   will convert that value to UINT_MAX, which should compare larger
   than any reasonable buffer size.  Second, comparing signed and
   unsigned integers will generate warnings with some compilers, and
   can have unpredictable results, particularly when the relative
   widths of the types is not known (size_t may be the same width as
   int or larger).
*/
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/strerror_r.c - strerror_r compatibility shim */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
 * Implement the POSIX strerror_r API in terms of the GNU strerror_r, which
 * returns a pointer to either the caller buffer or a constant string.  This is
 * the default version on glibc systems when _GNU_SOURCE is defined.
 */
#[no_mangle]
#[c2rust::src_loc = "72:1"]
pub unsafe extern "C" fn k5_strerror_r(mut errnum: libc::c_int,
                                       mut buf: *mut libc::c_char,
                                       mut buflen: size_t) -> libc::c_int {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    str = strerror_r(errnum, buf, buflen);
    if str != buf as *const libc::c_char {
        if krb5int_strlcpy(buf, str, buflen) >= buflen {
            *__errno_location() = 34 as libc::c_int;
            return -(1 as libc::c_int)
        }
    }
    return 0 as libc::c_int;
}
