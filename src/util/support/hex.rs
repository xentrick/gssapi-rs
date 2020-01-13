use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/include/ctype.h:35"]
pub mod ctype_h {
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
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
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
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
#[c2rust::header_src = "/usr/include/assert.h:33"]
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
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint8_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::ctype_h::{_ISdigit, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::stdlib_h::{malloc, free};
use self::assert_h::__assert_fail;
use self::string_h::strlen;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/hex.c - hex encoding/decoding implementation */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
#[inline]
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn hex_digit(mut bval: uint8_t, mut uppercase: libc::c_int)
 -> libc::c_char {
    if bval as libc::c_int <= 0xf as libc::c_int {
    } else {
        __assert_fail(b"bval <= 0xF\x00" as *const u8 as *const libc::c_char,
                      b"hex.c\x00" as *const u8 as *const libc::c_char,
                      40 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"char hex_digit(uint8_t, int)\x00")).as_ptr());
    }
    if (bval as libc::c_int) < 10 as libc::c_int {
        return ('0' as i32 + bval as libc::c_int) as libc::c_char
    } else if uppercase != 0 {
        return ('A' as i32 + (bval as libc::c_int - 10 as libc::c_int)) as
                   libc::c_char
    } else {
        return ('a' as i32 + (bval as libc::c_int - 10 as libc::c_int)) as
                   libc::c_char
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-hex.h - libkrb5support hex encoding/decoding declarations */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
 * Encode len bytes in hex, placing the result in allocated storage in
 * *hex_out.  Use uppercase hex digits if uppercase is non-zero.  Return 0 on
 * success, ENOMEM on error.
 */
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn k5_hex_encode(mut bytes: *const libc::c_void,
                                       mut len: size_t,
                                       mut uppercase: libc::c_int,
                                       mut hex_out: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: size_t = 0;
    let mut p: *const uint8_t = bytes as *const uint8_t;
    let mut hex: *mut libc::c_char = 0 as *mut libc::c_char;
    *hex_out = 0 as *mut libc::c_char;
    hex =
        malloc(len.wrapping_mul(2 as libc::c_int as
                                    libc::c_ulong).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong))
            as *mut libc::c_char;
    if hex.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as size_t;
    while i < len {
        *hex.offset(i.wrapping_mul(2 as libc::c_int as libc::c_ulong) as
                        isize) =
            hex_digit((*p.offset(i as isize) as libc::c_int >>
                           4 as libc::c_int) as uint8_t, uppercase);
        *hex.offset(i.wrapping_mul(2 as libc::c_int as
                                       libc::c_ulong).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong)
                        as isize) =
            hex_digit((*p.offset(i as isize) as libc::c_int &
                           0xf as libc::c_int) as uint8_t, uppercase);
        i = i.wrapping_add(1)
    }
    *hex.offset(len.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
        = '\u{0}' as i32 as libc::c_char;
    *hex_out = hex;
    return 0 as libc::c_int;
}
/* Decode a hex digit.  Return 0-15 on success, -1 on invalid input. */
#[inline]
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn decode_hexchar(mut c: libc::c_uchar) -> libc::c_int {
    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as libc::c_int &
           _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        return c as libc::c_int - '0' as i32
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }
    return -(1 as libc::c_int);
}
/*
 * Decode hex bytes, placing the result in allocated storage in *bytes_out and
 * *len_out.  Null-terminate the result (primarily for decoding passwords in
 * libkdb_ldap).  Return 0 on success, ENOMEM or EINVAL on error.
 */
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn k5_hex_decode(mut hex: *const libc::c_char,
                                       mut bytes_out: *mut *mut uint8_t,
                                       mut len_out: *mut size_t)
 -> libc::c_int {
    let mut hexlen: size_t = 0;
    let mut i: size_t = 0;
    let mut h1: libc::c_int = 0;
    let mut h2: libc::c_int = 0;
    let mut bytes: *mut uint8_t = 0 as *mut uint8_t;
    *bytes_out = 0 as *mut uint8_t;
    *len_out = 0 as libc::c_int as size_t;
    hexlen = strlen(hex);
    if hexlen.wrapping_rem(2 as libc::c_int as libc::c_ulong) !=
           0 as libc::c_int as libc::c_ulong {
        return 22 as libc::c_int
    }
    bytes =
        malloc(hexlen.wrapping_div(2 as libc::c_int as
                                       libc::c_ulong).wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong))
            as *mut uint8_t;
    if bytes.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as size_t;
    while i < hexlen.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        h1 =
            decode_hexchar(*hex.offset(i.wrapping_mul(2 as libc::c_int as
                                                          libc::c_ulong) as
                                           isize) as libc::c_uchar);
        h2 =
            decode_hexchar(*hex.offset(i.wrapping_mul(2 as libc::c_int as
                                                          libc::c_ulong).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong)
                                           as isize) as libc::c_uchar);
        if h1 == -(1 as libc::c_int) || h2 == -(1 as libc::c_int) {
            free(bytes as *mut libc::c_void);
            return 22 as libc::c_int
        }
        *bytes.offset(i as isize) = (h1 * 16 as libc::c_int + h2) as uint8_t;
        i = i.wrapping_add(1)
    }
    *bytes.offset(i as isize) = 0 as libc::c_int as uint8_t;
    *bytes_out = bytes;
    *len_out = hexlen.wrapping_div(2 as libc::c_int as libc::c_ulong);
    return 0 as libc::c_int;
}
