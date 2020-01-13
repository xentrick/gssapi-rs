use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:36"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/stdlib.h:36"]
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
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
    }
}
pub use self::stddef_h::size_t;
use self::stdlib_h::{malloc, free};
use self::string_h::{strlen, strchr};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/base64.c - base64 encoder and decoder */
/*
 * Copyright (c) 1995-2001 Kungliga Tekniska Högskolan
 * (Royal Institute of Technology, Stockholm, Sweden).
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
#[c2rust::src_loc = "39:19"]
static mut base64_chars: [libc::c_char; 65] =
    [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
     83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105,
     106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
     120, 121, 122, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 43, 47, 0];
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-base64.h - base64 declarations */
/*
 * Copyright (c) 1995, 1996, 1997 Kungliga Tekniska Högskolan
 * (Royal Institute of Technology, Stockholm, Sweden).
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/* base64-encode data and return it in an allocated buffer.  Return NULL if out
 * of memory. */
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn k5_base64_encode(mut data: *const libc::c_void,
                                          mut len: size_t)
 -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut c: libc::c_uint = 0;
    let mut q: *const libc::c_uchar = 0 as *const libc::c_uchar;
    if len >
           (18446744073709551615 as
                libc::c_ulong).wrapping_div(4 as libc::c_int as libc::c_ulong)
       {
        return 0 as *mut libc::c_char
    }
    s =
        malloc(len.wrapping_mul(4 as libc::c_int as
                                    libc::c_ulong).wrapping_div(3 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_add(4
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_ulong))
            as *mut libc::c_char;
    p = s;
    if p.is_null() { return 0 as *mut libc::c_char }
    q = data as *const libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let fresh0 = i;
        i = i.wrapping_add(1);
        c = *q.offset(fresh0 as isize) as libc::c_uint;
        c = c.wrapping_mul(256 as libc::c_int as libc::c_uint);
        if i < len {
            c = c.wrapping_add(*q.offset(i as isize) as libc::c_uint)
        }
        i = i.wrapping_add(1);
        c = c.wrapping_mul(256 as libc::c_int as libc::c_uint);
        if i < len {
            c = c.wrapping_add(*q.offset(i as isize) as libc::c_uint)
        }
        i = i.wrapping_add(1);
        *p.offset(0 as libc::c_int as isize) =
            base64_chars[((c & 0xfc0000 as libc::c_int as libc::c_uint) >>
                              18 as libc::c_int) as usize];
        *p.offset(1 as libc::c_int as isize) =
            base64_chars[((c & 0x3f000 as libc::c_int as libc::c_uint) >>
                              12 as libc::c_int) as usize];
        *p.offset(2 as libc::c_int as isize) =
            base64_chars[((c & 0xfc0 as libc::c_int as libc::c_uint) >>
                              6 as libc::c_int) as usize];
        *p.offset(3 as libc::c_int as isize) =
            base64_chars[((c & 0x3f as libc::c_int as libc::c_uint) >>
                              0 as libc::c_int) as usize];
        if i > len {
            *p.offset(3 as libc::c_int as isize) = '=' as i32 as libc::c_char
        }
        if i > len.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            *p.offset(2 as libc::c_int as isize) = '=' as i32 as libc::c_char
        }
        p = p.offset(4 as libc::c_int as isize)
    }
    *p = '\u{0}' as i32 as libc::c_char;
    return s;
}
/* Decode token, which must be four bytes long. */
#[c2rust::src_loc = "85:1"]
unsafe extern "C" fn decode_token(mut token: *const libc::c_char)
 -> libc::c_uint {
    let mut i: libc::c_int = 0;
    let mut marker: libc::c_int = 0 as libc::c_int;
    let mut val: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        val = val.wrapping_mul(64 as libc::c_int as libc::c_uint);
        if *token.offset(i as isize) as libc::c_int == '=' as i32 {
            marker += 1
        } else if marker > 0 as libc::c_int {
            return 0xffffffff as libc::c_uint
        } else {
            p =
                strchr(base64_chars.as_ptr(),
                       *token.offset(i as isize) as libc::c_int);
            if p.is_null() { return 0xffffffff as libc::c_uint }
            val =
                (val as libc::c_long +
                     p.wrapping_offset_from(base64_chars.as_ptr()) as
                         libc::c_long) as libc::c_uint
        }
        i += 1
    }
    if marker > 2 as libc::c_int { return 0xffffffff as libc::c_uint }
    return (marker << 24 as libc::c_int) as libc::c_uint | val;
}
/*
 * Decode str as base64 and return the result in an allocated buffer, setting
 * *len_out to the length.  Return NULL and *len_out == 0 if out of memory,
 * NULL and *len_out == SIZE_MAX on invalid input.
 */
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn k5_base64_decode(mut str: *const libc::c_char,
                                          mut len_out: *mut size_t)
 -> *mut libc::c_void {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: libc::c_uint = 0;
    let mut marker: libc::c_uint = 0;
    let mut len: size_t = 0;
    *len_out = 18446744073709551615 as libc::c_ulong;
    /* Allocate the output buffer. */
    len = strlen(str);
    if len.wrapping_rem(4 as libc::c_int as libc::c_ulong) != 0 {
        return 0 as *mut libc::c_void
    }
    data =
        malloc(len.wrapping_div(4 as libc::c_int as
                                    libc::c_ulong).wrapping_mul(3 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong))
            as *mut libc::c_uchar;
    q = data;
    if data.is_null() {
        *len_out = 0 as libc::c_int as size_t;
        return 0 as *mut libc::c_void
    }
    /* Decode the string. */
    while *str as libc::c_int != '\u{0}' as i32 {
        val = decode_token(str);
        if val == 0xffffffff as libc::c_uint {
            free(data as *mut libc::c_void);
            return 0 as *mut libc::c_void
        }
        marker =
            val >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint;
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 =
            (val >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        if marker < 2 as libc::c_int as libc::c_uint {
            let fresh2 = q;
            q = q.offset(1);
            *fresh2 =
                (val >> 8 as libc::c_int &
                     0xff as libc::c_int as libc::c_uint) as libc::c_uchar
        }
        if marker < 1 as libc::c_int as libc::c_uint {
            let fresh3 = q;
            q = q.offset(1);
            *fresh3 =
                (val & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar
        }
        str = str.offset(4 as libc::c_int as isize)
    }
    *len_out = q.wrapping_offset_from(data) as libc::c_long as size_t;
    return data as *mut libc::c_void;
}
