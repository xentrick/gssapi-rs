use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:18"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:18"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:18"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:18"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:18"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
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
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-utf8.h:19"]
pub mod k5_utf8_h {
    #[c2rust::src_loc = "74:1"]
    pub type krb5_ucs4 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn krb5int_utf8_to_ucs4(p: *const libc::c_char,
                                    out: *mut krb5_ucs4) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn krb5int_ucs4_to_utf8(c: krb5_ucs4, buf: *mut libc::c_char)
         -> size_t;
        /* Optimizations */
        #[no_mangle]
        #[c2rust::src_loc = "143:19"]
        pub static krb5int_utf8_lentab: [libc::c_char; 128];
        #[no_mangle]
        #[c2rust::src_loc = "144:19"]
        pub static krb5int_utf8_mintab: [libc::c_char; 32];
    }
    /* K5_UTF8_H */
    /*
 * these macros assume 'x' is an ASCII x
 * and assume the "C" locale
 */
    /* For symmetry */
    /* This is like CHARLEN but additionally validates to make sure
 * the char used the shortest possible encoding.
 * 'l' is used to temporarily hold the result of CHARLEN.
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-unicode.h:20"]
pub mod k5_unicode_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology,
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
 * Copyright 1998-2008 The OpenLDAP Foundation.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <https://www.OpenLDAP.org/license.html>.
 */
/*
 * Copyright (C) 2000 Novell, Inc. All Rights Reserved.
 *
 * THIS WORK IS SUBJECT TO U.S. AND INTERNATIONAL COPYRIGHT LAWS AND TREATIES.
 * USE, MODIFICATION, AND REDISTRIBUTION OF THIS WORK IS SUBJECT TO VERSION
 * 2.0.1 OF THE OPENLDAP PUBLIC LICENSE, A COPY OF WHICH IS AVAILABLE AT
 * HTTPS://WWW.OPENLDAP.ORG/LICENSE.HTML OR IN THE FILE "LICENSE" IN THE
 * TOP-LEVEL DIRECTORY OF THE DISTRIBUTION. ANY USE OR EXPLOITATION OF THIS
 * WORK OTHER THAN AS AUTHORIZED IN VERSION 2.0.1 OF THE OPENLDAP PUBLIC
 * LICENSE, OR OTHER PRIOR WRITTEN CONSENT FROM NOVELL, COULD SUBJECT THE
 * PERPETRATOR TO CRIMINAL AND CIVIL LIABILITY.
 */
    /* This work is part of OpenLDAP Software <https://www.openldap.org/>. */
    #[c2rust::src_loc = "88:1"]
    pub type krb5_unicode = krb5_ucs4;
    use super::k5_utf8_h::krb5_ucs4;
    /* K5_UNICODE_H */
}
#[c2rust::header_src = "/usr/include/ctype.h:23"]
pub mod ctype_h {
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "122:12"]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:18"]
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
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:18"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:18"]
pub mod k5_int_h {
    #[inline]
    #[c2rust::src_loc = "2326:1"]
    pub unsafe extern "C" fn k5memdup0(mut in_0: *const libc::c_void,
                                       mut len: size_t,
                                       mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void =
            k5alloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
    }
    use super::stddef_h::size_t;
    use super::krb5_h::krb5_error_code;
    use super::string_h::memcpy;
    use super::stdlib_h::calloc;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/unicode/ucdata/ucdata.h:21"]
pub mod ucdata_h {
    use super::krb5_h::krb5_ui_4;
    extern "C" {
        /*
 * Directionality macros.
 */
        /*
 * Other macros inspired by John Cowan.
 */
        /*
 * Other miscellaneous character property macros.
 */
        /* *************************************************************************
 *
 * Functions for case conversion.
 *
 **************************************************************************/
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn uctoupper(code: krb5_ui_4) -> krb5_ui_4;
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn uctolower(code: krb5_ui_4) -> krb5_ui_4;
        /*
 * Does canonical composition on the string str with length len, and returns
 * the length of the composed string.
 */
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn uccanoncomp(str: *mut krb5_ui_4, len: libc::c_int)
         -> libc::c_int;
        /*
 * Equivalent to uccanondecomp() except that it includes compatibility
 * decompositions.
 */
        #[no_mangle]
        #[c2rust::src_loc = "280:1"]
        pub fn uccompatdecomp(in_0: *const krb5_ui_4, inlen: libc::c_int,
                              out: *mut *mut krb5_ui_4,
                              outlen: *mut libc::c_int) -> libc::c_int;
    }
    /* _h_ucdata */
}
pub use self::types_h::{__int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::krb5_h::{krb5_int32, krb5_ui_4, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data};
pub use self::k5_utf8_h::{krb5_ucs4, krb5int_utf8_to_ucs4,
                          krb5int_ucs4_to_utf8, krb5int_utf8_lentab,
                          krb5int_utf8_mintab};
pub use self::k5_unicode_h::krb5_unicode;
pub use self::ctype_h::{_ISupper, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, __ctype_b_loc, tolower};
use self::stdlib_h::{malloc, calloc, realloc, free};
use self::string_h::memcpy;
pub use self::k5_int_h::{k5memdup0, k5alloc, k5calloc};
use self::ucdata_h::{uctoupper, uctolower, uccanoncomp, uccompatdecomp};
/*
 * Copyright 1998-2008 The OpenLDAP Foundation. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP Public
 * License.
 *
 * A copy of this license is available in file LICENSE in the top-level
 * directory of the distribution or, alternatively, at
 * <https://www.OpenLDAP.org/license.html>.
 */
/*
 * This work is part of OpenLDAP Software <https://www.openldap.org/>.
 * $OpenLDAP: pkg/ldap/libraries/liblunicode/ucstr.c,v 1.40 2008/03/04 06:24:05 hyc Exp $
 */
#[no_mangle]
#[c2rust::src_loc = "25:1"]
pub unsafe extern "C" fn krb5int_ucstrncmp(mut u1: *const krb5_unicode,
                                           mut u2: *const krb5_unicode,
                                           mut n: size_t) -> libc::c_int {
    while (0 as libc::c_int as libc::c_ulong) < n {
        if *u1 != *u2 {
            return if *u1 < *u2 {
                       -(1 as libc::c_int)
                   } else { 1 as libc::c_int }
        }
        if *u1 == 0 as libc::c_int as libc::c_uint { return 0 as libc::c_int }
        u1 = u1.offset(1);
        u2 = u2.offset(1);
        n = n.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn krb5int_ucstrncasecmp(mut u1: *const krb5_unicode,
                                               mut u2: *const krb5_unicode,
                                               mut n: size_t) -> libc::c_int {
    while (0 as libc::c_int as libc::c_ulong) < n {
        let mut uu1: krb5_unicode = uctolower(*u1);
        let mut uu2: krb5_unicode = uctolower(*u2);
        if uu1 != uu2 {
            return if uu1 < uu2 {
                       -(1 as libc::c_int)
                   } else { 1 as libc::c_int }
        }
        if uu1 == 0 as libc::c_int as libc::c_uint { return 0 as libc::c_int }
        u1 = u1.offset(1);
        u2 = u2.offset(1);
        n = n.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn krb5int_ucstrnchr(mut u: *const krb5_unicode,
                                           mut n: size_t, mut c: krb5_unicode)
 -> *mut krb5_unicode {
    while (0 as libc::c_int as libc::c_ulong) < n {
        if *u == c { return u as *mut krb5_unicode }
        u = u.offset(1);
        n = n.wrapping_sub(1)
    }
    return 0 as *mut krb5_unicode;
}
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn krb5int_ucstrncasechr(mut u: *const krb5_unicode,
                                               mut n: size_t,
                                               mut c: krb5_unicode)
 -> *mut krb5_unicode {
    c = uctolower(c);
    while (0 as libc::c_int as libc::c_ulong) < n {
        if uctolower(*u) == c { return u as *mut krb5_unicode }
        u = u.offset(1);
        n = n.wrapping_sub(1)
    }
    return 0 as *mut krb5_unicode;
}
#[no_mangle]
#[c2rust::src_loc = "93:1"]
pub unsafe extern "C" fn krb5int_ucstr2upper(mut u: *mut krb5_unicode,
                                             mut n: size_t) {
    while (0 as libc::c_int as libc::c_ulong) < n {
        *u = uctoupper(*u);
        u = u.offset(1);
        n = n.wrapping_sub(1)
    };
}
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub unsafe extern "C" fn krb5int_utf8_normalize(mut data: *const krb5_data,
                                                mut newdataptr:
                                                    *mut *mut krb5_data,
                                                mut flags: libc::c_uint)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut clen: libc::c_int = 0;
    let mut outpos: libc::c_int = 0 as libc::c_int;
    let mut ucsoutlen: libc::c_int = 0;
    let mut outsize: libc::c_int = 0;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outtmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ucs: *mut krb5_ucs4 = 0 as *mut krb5_ucs4;
    let mut p: *mut krb5_ucs4 = 0 as *mut krb5_ucs4;
    let mut ucsout: *mut krb5_ucs4 = 0 as *mut krb5_ucs4;
    let mut newdata: *mut krb5_data = 0 as *mut krb5_data;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    static mut mask: [libc::c_uchar; 7] =
        [0 as libc::c_int as libc::c_uchar,
         0x7f as libc::c_int as libc::c_uchar,
         0x1f as libc::c_int as libc::c_uchar,
         0xf as libc::c_int as libc::c_uchar,
         0x7 as libc::c_int as libc::c_uchar,
         0x3 as libc::c_int as libc::c_uchar,
         0x1 as libc::c_int as libc::c_uchar];
    let mut casefold: libc::c_uint = flags & 0x1 as libc::c_uint;
    let mut approx: libc::c_uint = flags & 0x8 as libc::c_uint;
    *newdataptr = 0 as *mut krb5_data;
    s = (*data).data;
    len = (*data).length as libc::c_int;
    newdata =
        malloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if newdata.is_null() { return 12 as libc::c_int }
    /*
     * Should first check to see if string is already in proper normalized
     * form. This is almost as time consuming as the normalization though.
     */
    /* finish off everything up to character before first non-ascii */
    if *(s as *const libc::c_uchar) as libc::c_int & 0x80 as libc::c_int == 0
       {
        if casefold != 0 {
            outsize = len + 7 as libc::c_int;
            out = malloc(outsize as libc::c_ulong) as *mut libc::c_char;
            if out.is_null() {
                retval = 12 as libc::c_int;
                current_block = 1572756231462267285;
            } else {
                i = 1 as libc::c_int;
                while i < len &&
                          *(s.offset(i as isize) as *const libc::c_uchar) as
                              libc::c_int & 0x80 as libc::c_int == 0 {
                    let fresh0 = outpos;
                    outpos = outpos + 1;
                    *out.offset(fresh0 as isize) =
                        if *(*__ctype_b_loc()).offset(*s.offset((i -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as isize)
                                                          as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISupper as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            tolower(*s.offset((i - 1 as libc::c_int) as isize)
                                        as libc::c_int)
                        } else {
                            *s.offset((i - 1 as libc::c_int) as isize) as
                                libc::c_int
                        } as libc::c_char;
                    i += 1
                }
                if i == len {
                    let fresh1 = outpos;
                    outpos = outpos + 1;
                    *out.offset(fresh1 as isize) =
                        if *(*__ctype_b_loc()).offset(*s.offset((len -
                                                                     1 as
                                                                         libc::c_int)
                                                                    as isize)
                                                          as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISupper as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                            tolower(*s.offset((len - 1 as libc::c_int) as
                                                  isize) as libc::c_int)
                        } else {
                            *s.offset((len - 1 as libc::c_int) as isize) as
                                libc::c_int
                        } as libc::c_char;
                    current_block = 1572756231462267285;
                } else { current_block = 1622411330066726685; }
            }
        } else {
            i = 1 as libc::c_int;
            while i < len &&
                      *(s.offset(i as isize) as *const libc::c_uchar) as
                          libc::c_int & 0x80 as libc::c_int == 0 {
                /* empty */
                i += 1
            }
            if i == len {
                (*newdata).length = len as libc::c_uint;
                (*newdata).data =
                    k5memdup0(s as *const libc::c_void, len as size_t,
                              &mut retval) as *mut libc::c_char;
                if (*newdata).data.is_null() {
                    current_block = 1572756231462267285;
                } else { *newdataptr = newdata; return 0 as libc::c_int }
            } else {
                outsize = len + 7 as libc::c_int;
                out = malloc(outsize as libc::c_ulong) as *mut libc::c_char;
                if out.is_null() {
                    retval = 12 as libc::c_int;
                    current_block = 1572756231462267285;
                } else {
                    outpos = i - 1 as libc::c_int;
                    memcpy(out as *mut libc::c_void, s as *const libc::c_void,
                           outpos as libc::c_ulong);
                    current_block = 1622411330066726685;
                }
            }
        }
    } else {
        outsize = len + 7 as libc::c_int;
        out = malloc(outsize as libc::c_ulong) as *mut libc::c_char;
        if out.is_null() {
            retval = 12 as libc::c_int;
            current_block = 1572756231462267285;
        } else { i = 0 as libc::c_int; current_block = 1622411330066726685; }
    }
    match current_block {
        1622411330066726685 => {
            ucs =
                malloc((len as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_ucs4>()
                                                            as libc::c_ulong))
                    as *mut krb5_ucs4;
            p = ucs;
            if ucs.is_null() {
                retval = 12 as libc::c_int
            } else {
                /* convert character before first non-ascii to ucs-4 */
                if i > 0 as libc::c_int {
                    *p =
                        if casefold != 0 {
                            if *(*__ctype_b_loc()).offset(*s.offset((i -
                                                                         1 as
                                                                             libc::c_int)
                                                                        as
                                                                        isize)
                                                              as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISupper as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 {
                                tolower(*s.offset((i - 1 as libc::c_int) as
                                                      isize) as libc::c_int)
                            } else {
                                *s.offset((i - 1 as libc::c_int) as isize) as
                                    libc::c_int
                            }
                        } else {
                            *s.offset((i - 1 as libc::c_int) as isize) as
                                libc::c_int
                        } as krb5_ucs4;
                    p = p.offset(1)
                }
                /* s[i] is now first non-ascii character */
                's_268:
                    loop 
                         /* s[i] is non-ascii */
	/* convert everything up to next ascii to ucs-4 */
                         {
                        if i < len {
                            clen =
                                (if *(s.offset(i as isize) as
                                          *const libc::c_uchar) as libc::c_int
                                        & 0x80 as libc::c_int == 0 {
                                     1 as libc::c_int
                                 } else {
                                     krb5int_utf8_lentab[(*(s.offset(i as
                                                                         isize)
                                                                as
                                                                *const libc::c_uchar)
                                                              as libc::c_int ^
                                                              0x80 as
                                                                  libc::c_int)
                                                             as usize] as
                                         libc::c_int
                                 });
                            clen =
                                if clen < 3 as libc::c_int ||
                                       krb5int_utf8_mintab[(*(s.offset(i as
                                                                           isize)
                                                                  as
                                                                  *const libc::c_uchar)
                                                                as libc::c_int
                                                                &
                                                                0x1f as
                                                                    libc::c_int)
                                                               as usize] as
                                           libc::c_int &
                                           *s.offset(i as
                                                         isize).offset(1 as
                                                                           libc::c_int
                                                                           as
                                                                           isize)
                                               as libc::c_int != 0 {
                                    clen
                                } else { 0 as libc::c_int };
                            if clen == 0 as libc::c_int {
                                retval =
                                    -(1765328133 as libc::c_long) as
                                        krb5_error_code;
                                break ;
                            } else if !(clen == 1 as libc::c_int) {
                                *p =
                                    (*s.offset(i as isize) as libc::c_int &
                                         mask[clen as usize] as libc::c_int)
                                        as krb5_ucs4;
                                i += 1;
                                j = 1 as libc::c_int;
                                while j < clen {
                                    if *s.offset(i as isize) as libc::c_int &
                                           0xc0 as libc::c_int !=
                                           0x80 as libc::c_int {
                                        retval =
                                            -(1765328133 as libc::c_long) as
                                                krb5_error_code;
                                        break 's_268 ;
                                    } else {
                                        *p <<= 6 as libc::c_int;
                                        *p |=
                                            (*s.offset(i as isize) as
                                                 libc::c_int &
                                                 0x3f as libc::c_int) as
                                                libc::c_uint;
                                        i += 1;
                                        j += 1
                                    }
                                }
                                if casefold != 0 { *p = uctolower(*p) }
                                p = p.offset(1);
                                continue ;
                            }
                        }
                        /* ascii */
                        /* normalize ucs of length p - ucs */
                        uccompatdecomp(ucs,
                                       p.wrapping_offset_from(ucs) as
                                           libc::c_long as libc::c_int,
                                       &mut ucsout, &mut ucsoutlen);
                        if approx != 0 {
                            j = 0 as libc::c_int;
                            while j < ucsoutlen {
                                if *ucsout.offset(j as isize) <
                                       0x80 as libc::c_int as libc::c_uint {
                                    let fresh2 = outpos;
                                    outpos = outpos + 1;
                                    *out.offset(fresh2 as isize) =
                                        *ucsout.offset(j as isize) as
                                            libc::c_char
                                }
                                j += 1
                            }
                        } else {
                            ucsoutlen = uccanoncomp(ucsout, ucsoutlen);
                            /* convert ucs to utf-8 and store in out */
                            j = 0 as libc::c_int;
                            while j < ucsoutlen {
                                /*
		 * allocate more space if not enough room for 6 bytes and
		 * terminator
		 */
                                if outsize - outpos < 7 as libc::c_int {
                                    outsize =
                                        ucsoutlen - j + outpos +
                                            6 as libc::c_int;
                                    outtmp =
                                        realloc(out as *mut libc::c_void,
                                                outsize as libc::c_ulong) as
                                            *mut libc::c_char;
                                    if outtmp.is_null() {
                                        retval = 12 as libc::c_int;
                                        break 's_268 ;
                                    } else { out = outtmp }
                                }
                                outpos =
                                    (outpos as
                                         libc::c_ulong).wrapping_add(krb5int_ucs4_to_utf8(*ucsout.offset(j
                                                                                                             as
                                                                                                             isize),
                                                                                          &mut *out.offset(outpos
                                                                                                               as
                                                                                                               isize)))
                                        as libc::c_int as libc::c_int;
                                j += 1
                            }
                        }
                        free(ucsout as *mut libc::c_void);
                        ucsout = 0 as *mut krb5_ucs4;
                        if i == len { break ; }
                        /* Allocate more space in out if necessary */
                        if len - i >= outsize - outpos {
                            outsize +=
                                1 as libc::c_int +
                                    (len - i - (outsize - outpos));
                            outtmp =
                                realloc(out as *mut libc::c_void,
                                        outsize as libc::c_ulong) as
                                    *mut libc::c_char;
                            if outtmp.is_null() {
                                retval = 12 as libc::c_int;
                                break ;
                            } else { out = outtmp }
                        }
                        /* s[i] is ascii */
	/* finish off everything up to char before next non-ascii */
                        i += 1;
                        while i < len &&
                                  *(s.offset(i as isize) as
                                        *const libc::c_uchar) as libc::c_int &
                                      0x80 as libc::c_int == 0 {
                            let fresh3 = outpos;
                            outpos = outpos + 1;
                            *out.offset(fresh3 as isize) =
                                if casefold != 0 {
                                    if *(*__ctype_b_loc()).offset(*s.offset((i
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                           as libc::c_int &
                                           _ISupper as libc::c_int as
                                               libc::c_ushort as libc::c_int
                                           != 0 {
                                        tolower(*s.offset((i -
                                                               1 as
                                                                   libc::c_int)
                                                              as isize) as
                                                    libc::c_int)
                                    } else {
                                        *s.offset((i - 1 as libc::c_int) as
                                                      isize) as libc::c_int
                                    }
                                } else {
                                    *s.offset((i - 1 as libc::c_int) as isize)
                                        as libc::c_int
                                } as libc::c_char;
                            i += 1
                        }
                        if i == len {
                            let fresh4 = outpos;
                            outpos = outpos + 1;
                            *out.offset(fresh4 as isize) =
                                if casefold != 0 {
                                    if *(*__ctype_b_loc()).offset(*s.offset((len
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                           as libc::c_int &
                                           _ISupper as libc::c_int as
                                               libc::c_ushort as libc::c_int
                                           != 0 {
                                        tolower(*s.offset((len -
                                                               1 as
                                                                   libc::c_int)
                                                              as isize) as
                                                    libc::c_int)
                                    } else {
                                        *s.offset((len - 1 as libc::c_int) as
                                                      isize) as libc::c_int
                                    }
                                } else {
                                    *s.offset((len - 1 as libc::c_int) as
                                                  isize) as libc::c_int
                                } as libc::c_char;
                            break ;
                        } else {
                            /* convert character before next non-ascii to ucs-4 */
                            *ucs =
                                if casefold != 0 {
                                    if *(*__ctype_b_loc()).offset(*s.offset((i
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                                                                                as
                                                                                isize)
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                           as libc::c_int &
                                           _ISupper as libc::c_int as
                                               libc::c_ushort as libc::c_int
                                           != 0 {
                                        tolower(*s.offset((i -
                                                               1 as
                                                                   libc::c_int)
                                                              as isize) as
                                                    libc::c_int)
                                    } else {
                                        *s.offset((i - 1 as libc::c_int) as
                                                      isize) as libc::c_int
                                    }
                                } else {
                                    *s.offset((i - 1 as libc::c_int) as isize)
                                        as libc::c_int
                                } as krb5_ucs4;
                            p = ucs.offset(1 as libc::c_int as isize)
                        }
                    }
            }
        }
        _ => { }
    }
    free(ucs as *mut libc::c_void);
    free(ucsout as *mut libc::c_void);
    if retval != 0 {
        free(out as *mut libc::c_void);
        free(newdata as *mut libc::c_void);
        return retval
    }
    *out.offset(outpos as isize) = '\u{0}' as i32 as libc::c_char;
    (*newdata).data = out;
    (*newdata).length = outpos as libc::c_uint;
    *newdataptr = newdata;
    return 0 as libc::c_int;
}
/* compare UTF8-strings, optionally ignore casing */
/* slow, should be optimized */
#[no_mangle]
#[c2rust::src_loc = "304:1"]
pub unsafe extern "C" fn krb5int_utf8_normcmp(mut data1: *const krb5_data,
                                              mut data2: *const krb5_data,
                                              mut flags: libc::c_uint)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ulen: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut done: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ucs: *mut krb5_ucs4 = 0 as *mut krb5_ucs4;
    let mut ucsout1: *mut krb5_ucs4 = 0 as *mut krb5_ucs4;
    let mut ucsout2: *mut krb5_ucs4 = 0 as *mut krb5_ucs4;
    let mut casefold: libc::c_uint = flags & 0x1 as libc::c_uint;
    let mut norm1: libc::c_uint = flags & 0x2 as libc::c_uint;
    let mut norm2: libc::c_uint = flags & 0x4 as libc::c_uint;
    if data1.is_null() {
        return if data2.is_null() {
                   0 as libc::c_int
               } else { -(1 as libc::c_int) }
    } else { if data2.is_null() { return 1 as libc::c_int } }
    l1 = (*data1).length as libc::c_int;
    l2 = (*data2).length as libc::c_int;
    len = if l1 < l2 { l1 } else { l2 };
    if len == 0 as libc::c_int {
        return if l1 == 0 as libc::c_int {
                   if l2 == 0 as libc::c_int {
                       0 as libc::c_int
                   } else { -(1 as libc::c_int) }
               } else { 1 as libc::c_int }
    }
    s1 = (*data1).data;
    s2 = (*data2).data;
    done = s1.offset(len as isize);
    while s1 < done &&
              *(s1 as *const libc::c_uchar) as libc::c_int &
                  0x80 as libc::c_int == 0 &&
              *(s2 as *const libc::c_uchar) as libc::c_int &
                  0x80 as libc::c_int == 0 {
        if casefold != 0 {
            let mut c1: libc::c_char =
                if *(*__ctype_b_loc()).offset(*s1 as libc::c_int as isize) as
                       libc::c_int &
                       _ISupper as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    tolower(*s1 as libc::c_int)
                } else { *s1 as libc::c_int } as libc::c_char;
            let mut c2: libc::c_char =
                if *(*__ctype_b_loc()).offset(*s2 as libc::c_int as isize) as
                       libc::c_int &
                       _ISupper as libc::c_int as libc::c_ushort as
                           libc::c_int != 0 {
                    tolower(*s2 as libc::c_int)
                } else { *s2 as libc::c_int } as libc::c_char;
            res = c1 as libc::c_int - c2 as libc::c_int
        } else { res = *s1 as libc::c_int - *s2 as libc::c_int }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
        if !(res != 0) { continue ; }
        /* done unless next character in s1 or s2 is non-ascii */
        if s1 < done {
            if *(s1 as *const libc::c_uchar) as libc::c_int &
                   0x80 as libc::c_int != 0 ||
                   *(s2 as *const libc::c_uchar) as libc::c_int &
                       0x80 as libc::c_int != 0 {
                break ;
            }
        } else if len < l1 &&
                      *(s1 as *const libc::c_uchar) as libc::c_int &
                          0x80 as libc::c_int != 0 ||
                      len < l2 &&
                          *(s2 as *const libc::c_uchar) as libc::c_int &
                              0x80 as libc::c_int != 0 {
            break ;
        }
        return res
    }
    /* We have encountered non-ascii or strings equal up to len */
    /* set i to number of iterations */
    i =
        (s1.wrapping_offset_from(done) as libc::c_long + len as libc::c_long)
            as libc::c_int;
    /* passed through loop at least once? */
    if i > 0 as libc::c_int {
        if res == 0 && s1 == done &&
               (len == l1 ||
                    *(s1 as *const libc::c_uchar) as libc::c_int &
                        0x80 as libc::c_int == 0) &&
               (len == l2 ||
                    *(s2 as *const libc::c_uchar) as libc::c_int &
                        0x80 as libc::c_int == 0) {
            /* all ascii and equal up to len */
            return l1 - l2
        }
        /* rewind one char, and do normalized compare from there */
        s1 = s1.offset(-1);
        s2 = s2.offset(-1);
        l1 -= i - 1 as libc::c_int;
        l2 -= i - 1 as libc::c_int
    }
    /*
     * Should first check to see if strings are already in proper normalized
     * form.
     */
    ucs =
        malloc(((if norm1 != 0 || l1 > l2 { l1 } else { l2 }) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_ucs4>()
                                                    as libc::c_ulong)) as
            *mut krb5_ucs4;
    if ucs.is_null() {
        return if l1 > l2 { 1 as libc::c_int } else { -(1 as libc::c_int) }
        /* what to do??? */
    }
    /*
     * XXYYZ: we convert to ucs4 even though -llunicode
     * expects ucs2 in an ac_uint4
     */
    /* convert and normalize 1st string */
    i = 0 as libc::c_int;
    ulen = 0 as libc::c_int;
    while i < l1 {
        if krb5int_utf8_to_ucs4(s1.offset(i as isize),
                                &mut *ucs.offset(ulen as isize)) ==
               -(1 as libc::c_int) {
            free(ucs as *mut libc::c_void);
            return -(1 as libc::c_int)
            /* what to do??? */
        }
        len =
            if *(s1.offset(i as isize) as *const libc::c_uchar) as libc::c_int
                   & 0x80 as libc::c_int == 0 {
                1 as libc::c_int
            } else {
                krb5int_utf8_lentab[(*(s1.offset(i as isize) as
                                           *const libc::c_uchar) as
                                         libc::c_int ^ 0x80 as libc::c_int) as
                                        usize] as libc::c_int
            };
        i += len;
        ulen += 1
    }
    if norm1 != 0 {
        ucsout1 = ucs;
        l1 = ulen;
        ucs =
            malloc((l2 as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_ucs4>()
                                                        as libc::c_ulong)) as
                *mut krb5_ucs4;
        if ucs.is_null() {
            free(ucsout1 as *mut libc::c_void);
            return if l1 > l2 {
                       1 as libc::c_int
                   } else { -(1 as libc::c_int) }
            /* what to do??? */
        }
    } else {
        uccompatdecomp(ucs, ulen, &mut ucsout1, &mut l1);
        l1 = uccanoncomp(ucsout1, l1)
    }
    /* convert and normalize 2nd string */
    i = 0 as libc::c_int;
    ulen = 0 as libc::c_int;
    while i < l2 {
        if krb5int_utf8_to_ucs4(s2.offset(i as isize),
                                &mut *ucs.offset(ulen as isize)) ==
               -(1 as libc::c_int) {
            free(ucsout1 as *mut libc::c_void);
            free(ucs as *mut libc::c_void);
            return 1 as libc::c_int
            /* what to do??? */
        }
        len =
            if *(s2.offset(i as isize) as *const libc::c_uchar) as libc::c_int
                   & 0x80 as libc::c_int == 0 {
                1 as libc::c_int
            } else {
                krb5int_utf8_lentab[(*(s2.offset(i as isize) as
                                           *const libc::c_uchar) as
                                         libc::c_int ^ 0x80 as libc::c_int) as
                                        usize] as libc::c_int
            };
        i += len;
        ulen += 1
    }
    if norm2 != 0 {
        ucsout2 = ucs;
        l2 = ulen
    } else {
        uccompatdecomp(ucs, ulen, &mut ucsout2, &mut l2);
        l2 = uccanoncomp(ucsout2, l2);
        free(ucs as *mut libc::c_void);
    }
    res =
        if casefold != 0 {
            krb5int_ucstrncasecmp(ucsout1, ucsout2,
                                  if l1 < l2 { l1 } else { l2 } as size_t)
        } else {
            krb5int_ucstrncmp(ucsout1, ucsout2,
                              if l1 < l2 { l1 } else { l2 } as size_t)
        };
    free(ucsout1 as *mut libc::c_void);
    free(ucsout2 as *mut libc::c_void);
    if res != 0 as libc::c_int { return res }
    if l1 == l2 { return 0 as libc::c_int }
    return if l1 > l2 { 1 as libc::c_int } else { -(1 as libc::c_int) };
}
