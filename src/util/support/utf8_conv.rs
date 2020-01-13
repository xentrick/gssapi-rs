use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:61"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:61"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:61"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:61"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:61"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "644:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "691:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint16_t,
    }
    #[inline]
    #[c2rust::src_loc = "639:1"]
    pub unsafe extern "C" fn store_16_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = val as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "686:1"]
    pub unsafe extern "C" fn load_16_le(mut cvp: *const libc::c_void)
     -> libc::c_ushort {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_0)).i;
    }
    use super::stdint_uintn_h::uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-utf8.h:62"]
pub mod k5_utf8_h {
    #[c2rust::src_loc = "74:1"]
    pub type krb5_ucs4 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:63"]
pub mod k5_buf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn k5_buf_add_uint16_le(mut buf: *mut k5buf,
                                                  mut val: uint16_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 2 as libc::c_int as size_t);
        if !p.is_null() { store_16_le(val as libc::c_uint, p); };
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint16_t;
    use super::k5_platform_h::store_16_le;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Initialize a k5buf using an internally allocated dynamic buffer, zeroing
 * memory when reallocating or freeing. */
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn k5_buf_init_dynamic_zap(buf: *mut k5buf);
        /* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn k5_buf_get_space(buf: *mut k5buf, len: size_t)
         -> *mut libc::c_void;
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-input.h:64"]
pub mod k5_input_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-input.h - k5input helper functions */
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
 * The k5input module defines helpers for safely consuming a fixed-sized block
 * of memory.  If an overrun or allocation failure occurs at any step,
 * subsequent functions will return default values until the error is detected
 * by looking at the status field.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct k5input {
        pub ptr: *const libc::c_uchar,
        pub len: size_t,
        pub status: int32_t,
    }
    #[inline]
    #[c2rust::src_loc = "51:1"]
    pub unsafe extern "C" fn k5_input_init(mut in_0: *mut k5input,
                                           mut ptr: *const libc::c_void,
                                           mut len: size_t) {
        (*in_0).ptr = ptr as *const libc::c_uchar;
        (*in_0).len = len;
        (*in_0).status = 0 as libc::c_int;
    }
    /* Only set the status value of in if it hasn't already been set, so status
 * reflects the first thing to go wrong. */
    #[inline]
    #[c2rust::src_loc = "61:1"]
    pub unsafe extern "C" fn k5_input_set_status(mut in_0: *mut k5input,
                                                 mut status: int32_t) {
        if (*in_0).status == 0 { (*in_0).status = status };
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn k5_input_get_bytes(mut in_0: *mut k5input,
                                                mut len: size_t)
     -> *const libc::c_uchar {
        if (*in_0).len < len { k5_input_set_status(in_0, 22 as libc::c_int); }
        if (*in_0).status != 0 { return 0 as *const libc::c_uchar }
        (*in_0).len =
            ((*in_0).len as libc::c_ulong).wrapping_sub(len) as size_t as
                size_t;
        (*in_0).ptr = (*in_0).ptr.offset(len as isize);
        return (*in_0).ptr.offset(-(len as isize));
    }
    #[inline]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn k5_input_get_uint16_le(mut in_0: *mut k5input)
     -> uint16_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 2 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int
               } else {
                   load_16_le(ptr as *const libc::c_void) as libc::c_int
               } as uint16_t;
    }
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint16_t;
    use super::k5_platform_h::load_16_le;
    /* K5_BUF_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_16_le,
                              load_16_le};
pub use self::k5_utf8_h::{krb5_ucs4, krb5int_ucs4_to_utf8,
                          krb5int_utf8_lentab, krb5int_utf8_mintab};
pub use self::k5_buf_h::{k5buf, k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5_buf_add_uint16_le,
                         k5_buf_init_dynamic, k5_buf_init_dynamic_zap,
                         k5_buf_get_space, k5_buf_free};
pub use self::k5_input_h::{k5input, k5_input_init, k5_input_set_status,
                           k5_input_get_bytes, k5_input_get_uint16_le};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/utf8_conv.c */
/*
 * Copyright 2008, 2017 by the Massachusetts Institute of Technology.
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
 * Copyright 1998-2008 The OpenLDAP Foundation.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in the file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <https://www.OpenLDAP.org/license.html>.
 */
/* Copyright (C) 1999, 2000 Novell, Inc. All Rights Reserved.
 *
 * THIS WORK IS SUBJECT TO U.S. AND INTERNATIONAL COPYRIGHT LAWS AND
 * TREATIES. USE, MODIFICATION, AND REDISTRIBUTION OF THIS WORK IS SUBJECT
 * TO VERSION 2.0.1 OF THE OPENLDAP PUBLIC LICENSE, A COPY OF WHICH IS
 * AVAILABLE AT HTTP://WWW.OPENLDAP.ORG/LICENSE.HTML OR IN THE FILE "LICENSE"
 * IN THE TOP-LEVEL DIRECTORY OF THE DISTRIBUTION. ANY USE OR EXPLOITATION
 * OF THIS WORK OTHER THAN AS AUTHORIZED IN VERSION 2.0.1 OF THE OPENLDAP
 * PUBLIC LICENSE, OR OTHER PRIOR WRITTEN CONSENT FROM NOVELL, COULD SUBJECT
 * THE PERPETRATOR TO CRIMINAL AND CIVIL LIABILITY.
 */
/* This work is based on OpenLDAP Software <https://www.openldap.org/>. */
/*
 * These routines convert between UTF-16 and UTF-8.  UTF-16 encodes a Unicode
 * character in either two or four bytes.  Characters in the Basic Multilingual
 * Plane (hex 0..D7FF and E000..FFFF) are encoded as-is in two bytes.
 * Characters in the Supplementary Planes (10000..10FFFF) are split into a high
 * surrogate and a low surrogate, each containing ten bits of the character
 * value, and encoded in four bytes.
 */
#[c2rust::src_loc = "67:22"]
static mut mask: [libc::c_uchar; 7] =
    [0 as libc::c_int as libc::c_uchar, 0x7f as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar, 0x7 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar];
/*
 * Convert a UTF-8 string to an allocated little-endian UTF-16 string.  The
 * resulting length is in bytes and will always be even.  Return EINVAL on
 * invalid input, ENOMEM on out of memory, or 0 on success.
 */
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn k5_utf8_to_utf16le(mut utf8: *const libc::c_char,
                                            mut utf16_out: *mut *mut uint8_t,
                                            mut nbytes_out: *mut size_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut ch: krb5_ucs4 = 0;
    let mut chlen: size_t = 0;
    let mut i: size_t = 0;
    *utf16_out = 0 as *mut uint8_t;
    *nbytes_out = 0 as libc::c_int as size_t;
    /* UTF-16 conversion is used for RC4 string-to-key, so treat this data as
     * sensitive. */
    k5_buf_init_dynamic_zap(&mut buf);
    's_23:
        loop 
             /* Examine next UTF-8 character. */
             {
            if !(*utf8 as libc::c_int != '\u{0}' as i32) {
                current_block = 15089075282327824602;
                break ;
            }
            /* Get UTF-8 sequence length from first byte. */
            chlen =
                (if *(utf8 as *const libc::c_uchar) as libc::c_int &
                        0x80 as libc::c_int == 0 {
                     1 as libc::c_int
                 } else {
                     krb5int_utf8_lentab[(*(utf8 as *const libc::c_uchar) as
                                              libc::c_int ^
                                              0x80 as libc::c_int) as usize]
                         as libc::c_int
                 }) as size_t;
            chlen =
                if chlen < 3 as libc::c_int as libc::c_ulong ||
                       krb5int_utf8_mintab[(*(utf8 as *const libc::c_uchar) as
                                                libc::c_int &
                                                0x1f as libc::c_int) as usize]
                           as libc::c_int &
                           *utf8.offset(1 as libc::c_int as isize) as
                               libc::c_int != 0 {
                    chlen
                } else { 0 as libc::c_int as libc::c_ulong };
            if chlen == 0 as libc::c_int as libc::c_ulong {
                current_block = 9195980619532155589;
                break ;
            }
            /* First byte minus length tag */
            ch =
                (*utf8.offset(0 as libc::c_int as isize) as libc::c_int &
                     mask[chlen as usize] as libc::c_int) as krb5_ucs4;
            i = 1 as libc::c_int as size_t;
            while i < chlen {
                /* Subsequent bytes must start with 10. */
                if *utf8.offset(i as isize) as libc::c_int &
                       0xc0 as libc::c_int != 0x80 as libc::c_int {
                    current_block = 9195980619532155589;
                    break 's_23 ;
                }
                /* 6 bits of data in each subsequent byte */
                ch <<= 6 as libc::c_int;
                ch |=
                    (*utf8.offset(i as isize) as libc::c_int &
                         0x3f as libc::c_int) as krb5_ucs4;
                i = i.wrapping_add(1)
            }
            if !(ch <= 0x10ffff as libc::c_int as libc::c_uint &&
                     !(ch >= 0xd800 as libc::c_int as libc::c_uint &&
                           ch <= 0xdfff as libc::c_int as libc::c_uint)) {
                current_block = 9195980619532155589;
                break ;
            }
            /* Characters in the basic multilingual plane are encoded using two
         * bytes; other characters are encoded using four bytes. */
            if ch <= 0xffff as libc::c_int as libc::c_uint &&
                   !(ch >= 0xd800 as libc::c_int as libc::c_uint &&
                         ch <= 0xdfff as libc::c_int as libc::c_uint) {
                k5_buf_add_uint16_le(&mut buf, ch as uint16_t);
            } else {
                /* 0x10000 is subtracted from ch; then the high ten bits plus
             * 0xD800 and the low ten bits plus 0xDC00 are the surrogates. */
                k5_buf_add_uint16_le(&mut buf,
                                     (0xd800 as libc::c_int as libc::c_uint |
                                          ch.wrapping_sub(0x10000 as
                                                              libc::c_int as
                                                              libc::c_uint) >>
                                              10 as libc::c_int) as uint16_t);
                k5_buf_add_uint16_le(&mut buf,
                                     (0xdc00 as libc::c_int as libc::c_uint |
                                          ch.wrapping_sub(0x10000 as
                                                              libc::c_int as
                                                              libc::c_uint) &
                                              0x3ff as libc::c_int as
                                                  libc::c_uint) as uint16_t);
            }
            /* Move to next UTF-8 character. */
            utf8 = utf8.offset(chlen as isize)
        }
    match current_block {
        9195980619532155589 => {
            k5_buf_free(&mut buf);
            return 22 as libc::c_int
        }
        _ => {
            *utf16_out = buf.data as *mut uint8_t;
            *nbytes_out = buf.len;
            return 0 as libc::c_int
        }
    };
}
/*
 * Convert a little-endian UTF-16 string to an allocated null-terminated UTF-8
 * string.  nbytes is the length of ucs2bytes in bytes, and must be an even
 * number.  Return EINVAL on invalid input, ENOMEM on out of memory, or 0 on
 * success.
 */
#[no_mangle]
#[c2rust::src_loc = "151:1"]
pub unsafe extern "C" fn k5_utf16le_to_utf8(mut utf16bytes: *const uint8_t,
                                            mut nbytes: size_t,
                                            mut utf8_out:
                                                *mut *mut libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut ch1: uint16_t = 0;
    let mut ch2: uint16_t = 0;
    let mut ch: krb5_ucs4 = 0;
    let mut chlen: size_t = 0;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    *utf8_out = 0 as *mut libc::c_char;
    if nbytes.wrapping_rem(2 as libc::c_int as libc::c_ulong) !=
           0 as libc::c_int as libc::c_ulong {
        return 22 as libc::c_int
    }
    k5_buf_init_dynamic(&mut buf);
    k5_input_init(&mut in_0, utf16bytes as *const libc::c_void, nbytes);
    loop  {
        if !(in_0.status == 0 && in_0.len > 0 as libc::c_int as libc::c_ulong)
           {
            current_block = 4495394744059808450;
            break ;
        }
        /* Get the next character or high surrogate.  A low surrogate without a
         * preceding high surrogate is invalid. */
        ch1 = k5_input_get_uint16_le(&mut in_0);
        if ch1 as libc::c_int >= 0xdc00 as libc::c_int &&
               ch1 as libc::c_int <= 0xdfff as libc::c_int {
            current_block = 2687857153341325290;
            break ;
        }
        if ch1 as libc::c_int >= 0xd800 as libc::c_int &&
               ch1 as libc::c_int <= 0xdbff as libc::c_int {
            /* Get the low surrogate and combine the pair. */
            ch2 = k5_input_get_uint16_le(&mut in_0);
            if !(ch2 as libc::c_int >= 0xdc00 as libc::c_int &&
                     ch2 as libc::c_int <= 0xdfff as libc::c_int) {
                current_block = 2687857153341325290;
                break ;
            }
            ch =
                (0x10000 as libc::c_int +
                     ((ch1 as libc::c_int & 0x3ff as libc::c_int) <<
                          10 as libc::c_int |
                          ch2 as libc::c_int & 0x3ff as libc::c_int)) as
                    krb5_ucs4
        } else { ch = ch1 as krb5_ucs4 }
        chlen = krb5int_ucs4_to_utf8(ch, 0 as *mut libc::c_char);
        p = k5_buf_get_space(&mut buf, chlen);
        if p.is_null() { return 12 as libc::c_int }
        krb5int_ucs4_to_utf8(ch, p as *mut libc::c_char);
    }
    match current_block {
        4495394744059808450 => {
            if !(in_0.status != 0) {
                *utf8_out = buf.data as *mut libc::c_char;
                return 0 as libc::c_int
            }
        }
        _ => { }
    }
    k5_buf_free(&mut buf);
    return 22 as libc::c_int;
}
