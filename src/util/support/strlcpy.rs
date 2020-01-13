use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:20"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/string.h:20"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::stddef_h::size_t;
use self::string_h::strlen;
/* -*- mode: c; c-file-style: "bsd"; indent-tabs-mode: t -*- */
/*
 * Copyright (c) 1998 Todd C. Miller <Todd.Miller@courtesan.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
/* Provide strlcpy and strlcat for platforms that don't have them. */
/*
 * Copy src to string dst of size siz.  At most siz-1 characters
 * will be copied.  Always NUL terminates (unless siz == 0).
 * Returns strlen(src); if retval >= siz, truncation occurred.
 */
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5int_strlcpy(mut dst: *mut libc::c_char,
                                         mut src: *const libc::c_char,
                                         mut siz: size_t) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = siz;
    /* Copy as many bytes as will fit */
    if n != 0 as libc::c_int as libc::c_ulong {
        loop  {
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_int as libc::c_ulong) { break ; }
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = *fresh0;
            if *fresh1 as libc::c_int == '\u{0}' as i32 { break ; }
        }
    }
    /* Not enough room in dst, add NUL and traverse rest of src */
    if n == 0 as libc::c_int as libc::c_ulong {
        if siz != 0 as libc::c_int as libc::c_ulong {
            *d = '\u{0}' as i32 as libc::c_char
        } /* NUL-terminate dst */
        loop  {
            let fresh2 = s;
            s = s.offset(1);
            if !(*fresh2 != 0) { break ; }
        }
    }
    return (s.wrapping_offset_from(src) as libc::c_long -
                1 as libc::c_int as libc::c_long) as size_t;
    /* count does not include NUL */
}
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
/*
 * Appends src to string dst of size siz (unlike strncat, siz is the
 * full size of dst, not space left).  At most siz-1 characters
 * will be copied.  Always NUL terminates (unless siz <= strlen(dst)).
 * Returns strlen(src) + MIN(siz, strlen(initial dst)).
 * If retval >= siz, truncation occurred.
 */
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn krb5int_strlcat(mut dst: *mut libc::c_char,
                                         mut src: *const libc::c_char,
                                         mut siz: size_t) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = siz;
    let mut dlen: size_t = 0;
    loop 
         /* Find the end of dst and adjust bytes left but don't go past end */
         {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 != 0 as libc::c_int as libc::c_ulong &&
                 *d as libc::c_int != '\u{0}' as i32) {
            break ;
        }
        d = d.offset(1)
    }
    dlen = d.wrapping_offset_from(dst) as libc::c_long as size_t;
    n = siz.wrapping_sub(dlen);
    if n == 0 as libc::c_int as libc::c_ulong {
        return dlen.wrapping_add(strlen(s))
    }
    while *s as libc::c_int != '\u{0}' as i32 {
        if n != 1 as libc::c_int as libc::c_ulong {
            let fresh4 = d;
            d = d.offset(1);
            *fresh4 = *s;
            n = n.wrapping_sub(1)
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return dlen.wrapping_add(s.wrapping_offset_from(src) as libc::c_long as
                                 libc::c_ulong);
    /* count does not include NUL */
}
