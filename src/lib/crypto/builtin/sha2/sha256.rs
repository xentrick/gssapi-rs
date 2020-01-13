use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:36"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:36"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:36"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:36"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:36"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/sha2/sha2.h:37"]
pub mod sha2_h {
    /* SHA-384 and SHA-512 use the same state object. */
    #[c2rust::src_loc = "62:1"]
    pub type SHA256_CTX = sha256state;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/sha2/sha2.h - SHA-256 declarations */
/*
 * Copyright (c) 1995 - 2001 Kungliga Tekniska Högskolan
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct sha256state {
        pub sz: [libc::c_uint; 2],
        pub counter: [uint32_t; 8],
        pub save: [libc::c_uchar; 64],
    }
    use super::stdint_uintn_h::uint32_t;
    /* SHA2_H */
}
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data};
pub use self::sha2_h::{SHA256_CTX, sha256state};
use self::string_h::{memset, memcpy};
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
#[c2rust::src_loc = "188:8"]
pub struct x32 {
    #[bitfield(name = "a", ty = "libc::c_uint", bits = "0..=31")]
    #[bitfield(name = "b", ty = "libc::c_uint", bits = "32..=63")]
    pub a_b: [u8; 8],
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/sha2/sha256.c - SHA-256 implementation */
/*
 * Copyright (c) 2006 Kungliga Tekniska Högskolan
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
/* Vector Crays doesn't have a good 32-bit type, or more precisely,
 * int32_t as defined by <bind/bitypes.h> isn't 32 bits, and we don't
 * want to depend in being able to redefine this type.  To cope with
 * this we have to clamp the result in some places to [0,2^32); no
 * need to do this on other machines.  Did I say this was a mess?
 */
#[inline]
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn cshift(mut x: uint32_t, mut n: libc::c_uint)
 -> uint32_t {
    x = x;
    return x << n | x >> (32 as libc::c_int as libc::c_uint).wrapping_sub(n);
}
#[c2rust::src_loc = "86:23"]
static mut constant_256: [uint32_t; 64] =
    [0x428a2f98 as libc::c_int as uint32_t,
     0x71374491 as libc::c_int as uint32_t, 0xb5c0fbcf as libc::c_uint,
     0xe9b5dba5 as libc::c_uint, 0x3956c25b as libc::c_int as uint32_t,
     0x59f111f1 as libc::c_int as uint32_t, 0x923f82a4 as libc::c_uint,
     0xab1c5ed5 as libc::c_uint, 0xd807aa98 as libc::c_uint,
     0x12835b01 as libc::c_int as uint32_t,
     0x243185be as libc::c_int as uint32_t,
     0x550c7dc3 as libc::c_int as uint32_t,
     0x72be5d74 as libc::c_int as uint32_t, 0x80deb1fe as libc::c_uint,
     0x9bdc06a7 as libc::c_uint, 0xc19bf174 as libc::c_uint,
     0xe49b69c1 as libc::c_uint, 0xefbe4786 as libc::c_uint,
     0xfc19dc6 as libc::c_int as uint32_t,
     0x240ca1cc as libc::c_int as uint32_t,
     0x2de92c6f as libc::c_int as uint32_t,
     0x4a7484aa as libc::c_int as uint32_t,
     0x5cb0a9dc as libc::c_int as uint32_t,
     0x76f988da as libc::c_int as uint32_t, 0x983e5152 as libc::c_uint,
     0xa831c66d as libc::c_uint, 0xb00327c8 as libc::c_uint,
     0xbf597fc7 as libc::c_uint, 0xc6e00bf3 as libc::c_uint,
     0xd5a79147 as libc::c_uint, 0x6ca6351 as libc::c_int as uint32_t,
     0x14292967 as libc::c_int as uint32_t,
     0x27b70a85 as libc::c_int as uint32_t,
     0x2e1b2138 as libc::c_int as uint32_t,
     0x4d2c6dfc as libc::c_int as uint32_t,
     0x53380d13 as libc::c_int as uint32_t,
     0x650a7354 as libc::c_int as uint32_t,
     0x766a0abb as libc::c_int as uint32_t, 0x81c2c92e as libc::c_uint,
     0x92722c85 as libc::c_uint, 0xa2bfe8a1 as libc::c_uint,
     0xa81a664b as libc::c_uint, 0xc24b8b70 as libc::c_uint,
     0xc76c51a3 as libc::c_uint, 0xd192e819 as libc::c_uint,
     0xd6990624 as libc::c_uint, 0xf40e3585 as libc::c_uint,
     0x106aa070 as libc::c_int as uint32_t,
     0x19a4c116 as libc::c_int as uint32_t,
     0x1e376c08 as libc::c_int as uint32_t,
     0x2748774c as libc::c_int as uint32_t,
     0x34b0bcb5 as libc::c_int as uint32_t,
     0x391c0cb3 as libc::c_int as uint32_t,
     0x4ed8aa4a as libc::c_int as uint32_t,
     0x5b9cca4f as libc::c_int as uint32_t,
     0x682e6ff3 as libc::c_int as uint32_t,
     0x748f82ee as libc::c_int as uint32_t,
     0x78a5636f as libc::c_int as uint32_t, 0x84c87814 as libc::c_uint,
     0x8cc70208 as libc::c_uint, 0x90befffa as libc::c_uint,
     0xa4506ceb as libc::c_uint, 0xbef9a3f7 as libc::c_uint,
     0xc67178f2 as libc::c_uint];
#[no_mangle]
#[c2rust::src_loc = "105:1"]
pub unsafe extern "C" fn k5_sha256_init(mut m: *mut SHA256_CTX) {
    (*m).sz[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    (*m).sz[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uint;
    (*m).counter[0 as libc::c_int as usize] =
        0x6a09e667 as libc::c_int as uint32_t;
    (*m).counter[1 as libc::c_int as usize] = 0xbb67ae85 as libc::c_uint;
    (*m).counter[2 as libc::c_int as usize] =
        0x3c6ef372 as libc::c_int as uint32_t;
    (*m).counter[3 as libc::c_int as usize] = 0xa54ff53a as libc::c_uint;
    (*m).counter[4 as libc::c_int as usize] =
        0x510e527f as libc::c_int as uint32_t;
    (*m).counter[5 as libc::c_int as usize] = 0x9b05688c as libc::c_uint;
    (*m).counter[6 as libc::c_int as usize] =
        0x1f83d9ab as libc::c_int as uint32_t;
    (*m).counter[7 as libc::c_int as usize] =
        0x5be0cd19 as libc::c_int as uint32_t;
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn calc(mut m: *mut SHA256_CTX, mut in_0: *mut uint32_t) {
    let mut AA: uint32_t = 0;
    let mut BB: uint32_t = 0;
    let mut CC: uint32_t = 0;
    let mut DD: uint32_t = 0;
    let mut EE: uint32_t = 0;
    let mut FF: uint32_t = 0;
    let mut GG: uint32_t = 0;
    let mut HH: uint32_t = 0;
    let mut data: [uint32_t; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    AA = (*m).counter[0 as libc::c_int as usize];
    BB = (*m).counter[1 as libc::c_int as usize];
    CC = (*m).counter[2 as libc::c_int as usize];
    DD = (*m).counter[3 as libc::c_int as usize];
    EE = (*m).counter[4 as libc::c_int as usize];
    FF = (*m).counter[5 as libc::c_int as usize];
    GG = (*m).counter[6 as libc::c_int as usize];
    HH = (*m).counter[7 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        data[i as usize] = *in_0.offset(i as isize);
        i += 1
    }
    i = 16 as libc::c_int;
    while i < 64 as libc::c_int {
        data[i as usize] =
            ((data[(i - 2 as libc::c_int) as usize] >> 17 as libc::c_int |
                  data[(i - 2 as libc::c_int) as usize] <<
                      32 as libc::c_int - 17 as libc::c_int) ^
                 (data[(i - 2 as libc::c_int) as usize] >> 19 as libc::c_int |
                      data[(i - 2 as libc::c_int) as usize] <<
                          32 as libc::c_int - 19 as libc::c_int) ^
                 data[(i - 2 as libc::c_int) as usize] >>
                     10 as
                         libc::c_int).wrapping_add(data[(i - 7 as libc::c_int)
                                                            as
                                                            usize]).wrapping_add((data[(i
                                                                                            -
                                                                                            15
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           usize]
                                                                                      >>
                                                                                      7
                                                                                          as
                                                                                          libc::c_int
                                                                                      |
                                                                                      data[(i
                                                                                                -
                                                                                                15
                                                                                                    as
                                                                                                    libc::c_int)
                                                                                               as
                                                                                               usize]
                                                                                          <<
                                                                                          32
                                                                                              as
                                                                                              libc::c_int
                                                                                              -
                                                                                              7
                                                                                                  as
                                                                                                  libc::c_int)
                                                                                     ^
                                                                                     (data[(i
                                                                                                -
                                                                                                15
                                                                                                    as
                                                                                                    libc::c_int)
                                                                                               as
                                                                                               usize]
                                                                                          >>
                                                                                          18
                                                                                              as
                                                                                              libc::c_int
                                                                                          |
                                                                                          data[(i
                                                                                                    -
                                                                                                    15
                                                                                                        as
                                                                                                        libc::c_int)
                                                                                                   as
                                                                                                   usize]
                                                                                              <<
                                                                                              32
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  -
                                                                                                  18
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                     ^
                                                                                     data[(i
                                                                                               -
                                                                                               15
                                                                                                   as
                                                                                                   libc::c_int)
                                                                                              as
                                                                                              usize]
                                                                                         >>
                                                                                         3
                                                                                             as
                                                                                             libc::c_int).wrapping_add(data[(i
                                                                                                                                 -
                                                                                                                                 16
                                                                                                                                     as
                                                                                                                                     libc::c_int)
                                                                                                                                as
                                                                                                                                usize]);
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let mut T1: uint32_t = 0;
        let mut T2: uint32_t = 0;
        T1 =
            HH.wrapping_add((EE >> 6 as libc::c_int |
                                 EE << 32 as libc::c_int - 6 as libc::c_int) ^
                                (EE >> 11 as libc::c_int |
                                     EE <<
                                         32 as libc::c_int -
                                             11 as libc::c_int) ^
                                (EE >> 25 as libc::c_int |
                                     EE <<
                                         32 as libc::c_int -
                                             25 as
                                                 libc::c_int)).wrapping_add(EE
                                                                                &
                                                                                FF
                                                                                ^
                                                                                !EE
                                                                                    &
                                                                                    GG).wrapping_add(constant_256[i
                                                                                                                      as
                                                                                                                      usize]).wrapping_add(data[i
                                                                                                                                                    as
                                                                                                                                                    usize]);
        T2 =
            ((AA >> 2 as libc::c_int |
                  AA << 32 as libc::c_int - 2 as libc::c_int) ^
                 (AA >> 13 as libc::c_int |
                      AA << 32 as libc::c_int - 13 as libc::c_int) ^
                 (AA >> 22 as libc::c_int |
                      AA <<
                          32 as libc::c_int -
                              22 as
                                  libc::c_int)).wrapping_add(AA & BB ^ AA & CC
                                                                 ^ BB & CC);
        HH = GG;
        GG = FF;
        FF = EE;
        EE = DD.wrapping_add(T1);
        DD = CC;
        CC = BB;
        BB = AA;
        AA = T1.wrapping_add(T2);
        i += 1
    }
    (*m).counter[0 as libc::c_int as usize] =
        ((*m).counter[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(AA) as uint32_t as uint32_t;
    (*m).counter[1 as libc::c_int as usize] =
        ((*m).counter[1 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(BB) as uint32_t as uint32_t;
    (*m).counter[2 as libc::c_int as usize] =
        ((*m).counter[2 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(CC) as uint32_t as uint32_t;
    (*m).counter[3 as libc::c_int as usize] =
        ((*m).counter[3 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(DD) as uint32_t as uint32_t;
    (*m).counter[4 as libc::c_int as usize] =
        ((*m).counter[4 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(EE) as uint32_t as uint32_t;
    (*m).counter[5 as libc::c_int as usize] =
        ((*m).counter[5 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(FF) as uint32_t as uint32_t;
    (*m).counter[6 as libc::c_int as usize] =
        ((*m).counter[6 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(GG) as uint32_t as uint32_t;
    (*m).counter[7 as libc::c_int as usize] =
        ((*m).counter[7 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(HH) as uint32_t as uint32_t;
}
/*
 * From `Performance analysis of MD5' by Joseph D. Touch <touch@isi.edu>
 */
#[inline]
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn swap_uint32_t(mut t: uint32_t) -> uint32_t {
    let mut temp1: uint32_t = 0;
    let mut temp2: uint32_t = 0;
    temp1 = cshift(t, 16 as libc::c_int as libc::c_uint);
    temp2 = temp1 >> 8 as libc::c_int;
    temp1 &= 0xff00ff as libc::c_int as libc::c_uint;
    temp2 &= 0xff00ff as libc::c_int as libc::c_uint;
    temp1 <<= 8 as libc::c_int;
    return temp1 | temp2;
}
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn k5_sha256_update(mut m: *mut SHA256_CTX,
                                          mut v: *const libc::c_void,
                                          mut len: size_t) {
    let mut p: *const libc::c_uchar = v as *const libc::c_uchar;
    let mut old_sz: size_t = (*m).sz[0 as libc::c_int as usize] as size_t;
    let mut offset: size_t = 0;
    (*m).sz[0 as libc::c_int as usize] =
        ((*m).sz[0 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(len.wrapping_mul(8 as libc::c_int as
                                                              libc::c_ulong))
            as libc::c_uint as libc::c_uint;
    if ((*m).sz[0 as libc::c_int as usize] as libc::c_ulong) < old_sz {
        (*m).sz[1 as libc::c_int as usize] =
            (*m).sz[1 as libc::c_int as usize].wrapping_add(1)
    }
    offset =
        old_sz.wrapping_div(8 as libc::c_int as
                                libc::c_ulong).wrapping_rem(64 as libc::c_int
                                                                as
                                                                libc::c_ulong);
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut l: size_t =
            if len > (64 as libc::c_int as libc::c_ulong).wrapping_sub(offset)
               {
                (64 as libc::c_int as libc::c_ulong).wrapping_sub(offset)
            } else { len };
        memcpy((*m).save.as_mut_ptr().offset(offset as isize) as
                   *mut libc::c_void, p as *const libc::c_void, l);
        offset =
            (offset as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        p = p.offset(l as isize);
        len = (len as libc::c_ulong).wrapping_sub(l) as size_t as size_t;
        if offset == 64 as libc::c_int as libc::c_ulong {
            let mut i: libc::c_int = 0;
            let mut current: [uint32_t; 16] = [0; 16];
            let mut u: *mut x32 =
                (*m).save.as_mut_ptr() as *mut libc::c_void as *mut x32;
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                current[(2 as libc::c_int * i + 0 as libc::c_int) as usize] =
                    swap_uint32_t((*u.offset(i as isize)).a());
                current[(2 as libc::c_int * i + 1 as libc::c_int) as usize] =
                    swap_uint32_t((*u.offset(i as isize)).b());
                i += 1
            }
            calc(m, current.as_mut_ptr());
            offset = 0 as libc::c_int as size_t
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "228:1"]
pub unsafe extern "C" fn k5_sha256_final(mut res: *mut libc::c_void,
                                         mut m: *mut SHA256_CTX) {
    let mut zeros: [libc::c_uchar; 72] = [0; 72];
    let mut offset: libc::c_uint =
        (*m).sz[0 as libc::c_int as
                    usize].wrapping_div(8 as libc::c_int as
                                            libc::c_uint).wrapping_rem(64 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint);
    let mut dstart: libc::c_uint =
        (120 as libc::c_int as
             libc::c_uint).wrapping_sub(offset).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(64
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint).wrapping_add(1
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint);
    *zeros.as_mut_ptr() = 0x80 as libc::c_int as libc::c_uchar;
    memset(zeros.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<[libc::c_uchar; 72]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                libc::c_ulong));
    zeros[dstart.wrapping_add(7 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 0 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(6 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(5 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(4 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 0 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    zeros[dstart.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    k5_sha256_update(m, zeros.as_mut_ptr() as *const libc::c_void,
                     dstart.wrapping_add(8 as libc::c_int as libc::c_uint) as
                         size_t);
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_uchar = res as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *r.offset((4 as libc::c_int * i + 3 as libc::c_int) as isize) =
            ((*m).counter[i as usize] & 0xff as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        *r.offset((4 as libc::c_int * i + 2 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 8 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *r.offset((4 as libc::c_int * i + 1 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 16 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *r.offset((4 as libc::c_int * i) as isize) =
            ((*m).counter[i as usize] >> 24 as libc::c_int &
                 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        i += 1
    };
}
#[no_mangle]
#[c2rust::src_loc = "259:1"]
pub unsafe extern "C" fn k5_sha256(mut in_0: *const krb5_data, mut n: size_t,
                                   mut out: *mut uint8_t) -> krb5_error_code {
    let mut ctx: SHA256_CTX =
        SHA256_CTX{sz: [0; 2], counter: [0; 8], save: [0; 64],};
    let mut i: size_t = 0;
    k5_sha256_init(&mut ctx);
    i = 0 as libc::c_int as size_t;
    while i < n {
        k5_sha256_update(&mut ctx,
                         (*in_0.offset(i as isize)).data as
                             *const libc::c_void,
                         (*in_0.offset(i as isize)).length as size_t);
        i = i.wrapping_add(1)
    }
    k5_sha256_final(out as *mut libc::c_void, &mut ctx);
    return 0 as libc::c_int;
}
