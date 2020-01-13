use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/sha2/sha2.h:35"]
pub mod sha2_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct sha512state {
        pub sz: [uint64_t; 2],
        pub counter: [uint64_t; 8],
        pub save: [libc::c_uchar; 128],
    }
    #[c2rust::src_loc = "63:1"]
    pub type SHA384_CTX = sha512state;
    #[c2rust::src_loc = "64:1"]
    pub type SHA512_CTX = sha512state;
    use super::stdint_uintn_h::uint64_t;
    /* SHA2_H */
}
#[c2rust::header_src = "/usr/include/string.h:34"]
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
pub use self::types_h::__uint64_t;
pub use self::stddef_h::size_t;
pub use self::stdint_uintn_h::uint64_t;
pub use self::sha2_h::{sha512state, SHA384_CTX, SHA512_CTX};
use self::string_h::{memset, memcpy};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "193:8"]
pub struct x64 {
    pub a: uint64_t,
    pub b: uint64_t,
}
/*
 * Copyright (c) 2006, 2010 Kungliga Tekniska Högskolan
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
#[c2rust::src_loc = "64:23"]
static mut constant_512: [uint64_t; 80] =
    [0x428a2f98d728ae22 as libc::c_ulonglong as uint64_t,
     0x7137449123ef65cd as libc::c_ulonglong as uint64_t,
     0xb5c0fbcfec4d3b2f as libc::c_ulonglong as uint64_t,
     0xe9b5dba58189dbbc as libc::c_ulonglong as uint64_t,
     0x3956c25bf348b538 as libc::c_ulonglong as uint64_t,
     0x59f111f1b605d019 as libc::c_ulonglong as uint64_t,
     0x923f82a4af194f9b as libc::c_ulonglong as uint64_t,
     0xab1c5ed5da6d8118 as libc::c_ulonglong as uint64_t,
     0xd807aa98a3030242 as libc::c_ulonglong as uint64_t,
     0x12835b0145706fbe as libc::c_ulonglong as uint64_t,
     0x243185be4ee4b28c as libc::c_ulonglong as uint64_t,
     0x550c7dc3d5ffb4e2 as libc::c_ulonglong as uint64_t,
     0x72be5d74f27b896f as libc::c_ulonglong as uint64_t,
     0x80deb1fe3b1696b1 as libc::c_ulonglong as uint64_t,
     0x9bdc06a725c71235 as libc::c_ulonglong as uint64_t,
     0xc19bf174cf692694 as libc::c_ulonglong as uint64_t,
     0xe49b69c19ef14ad2 as libc::c_ulonglong as uint64_t,
     0xefbe4786384f25e3 as libc::c_ulonglong as uint64_t,
     0xfc19dc68b8cd5b5 as libc::c_ulonglong as uint64_t,
     0x240ca1cc77ac9c65 as libc::c_ulonglong as uint64_t,
     0x2de92c6f592b0275 as libc::c_ulonglong as uint64_t,
     0x4a7484aa6ea6e483 as libc::c_ulonglong as uint64_t,
     0x5cb0a9dcbd41fbd4 as libc::c_ulonglong as uint64_t,
     0x76f988da831153b5 as libc::c_ulonglong as uint64_t,
     0x983e5152ee66dfab as libc::c_ulonglong as uint64_t,
     0xa831c66d2db43210 as libc::c_ulonglong as uint64_t,
     0xb00327c898fb213f as libc::c_ulonglong as uint64_t,
     0xbf597fc7beef0ee4 as libc::c_ulonglong as uint64_t,
     0xc6e00bf33da88fc2 as libc::c_ulonglong as uint64_t,
     0xd5a79147930aa725 as libc::c_ulonglong as uint64_t,
     0x6ca6351e003826f as libc::c_ulonglong as uint64_t,
     0x142929670a0e6e70 as libc::c_ulonglong as uint64_t,
     0x27b70a8546d22ffc as libc::c_ulonglong as uint64_t,
     0x2e1b21385c26c926 as libc::c_ulonglong as uint64_t,
     0x4d2c6dfc5ac42aed as libc::c_ulonglong as uint64_t,
     0x53380d139d95b3df as libc::c_ulonglong as uint64_t,
     0x650a73548baf63de as libc::c_ulonglong as uint64_t,
     0x766a0abb3c77b2a8 as libc::c_ulonglong as uint64_t,
     0x81c2c92e47edaee6 as libc::c_ulonglong as uint64_t,
     0x92722c851482353b as libc::c_ulonglong as uint64_t,
     0xa2bfe8a14cf10364 as libc::c_ulonglong as uint64_t,
     0xa81a664bbc423001 as libc::c_ulonglong as uint64_t,
     0xc24b8b70d0f89791 as libc::c_ulonglong as uint64_t,
     0xc76c51a30654be30 as libc::c_ulonglong as uint64_t,
     0xd192e819d6ef5218 as libc::c_ulonglong as uint64_t,
     0xd69906245565a910 as libc::c_ulonglong as uint64_t,
     0xf40e35855771202a as libc::c_ulonglong as uint64_t,
     0x106aa07032bbd1b8 as libc::c_ulonglong as uint64_t,
     0x19a4c116b8d2d0c8 as libc::c_ulonglong as uint64_t,
     0x1e376c085141ab53 as libc::c_ulonglong as uint64_t,
     0x2748774cdf8eeb99 as libc::c_ulonglong as uint64_t,
     0x34b0bcb5e19b48a8 as libc::c_ulonglong as uint64_t,
     0x391c0cb3c5c95a63 as libc::c_ulonglong as uint64_t,
     0x4ed8aa4ae3418acb as libc::c_ulonglong as uint64_t,
     0x5b9cca4f7763e373 as libc::c_ulonglong as uint64_t,
     0x682e6ff3d6b2b8a3 as libc::c_ulonglong as uint64_t,
     0x748f82ee5defb2fc as libc::c_ulonglong as uint64_t,
     0x78a5636f43172f60 as libc::c_ulonglong as uint64_t,
     0x84c87814a1f0ab72 as libc::c_ulonglong as uint64_t,
     0x8cc702081a6439ec as libc::c_ulonglong as uint64_t,
     0x90befffa23631e28 as libc::c_ulonglong as uint64_t,
     0xa4506cebde82bde9 as libc::c_ulonglong as uint64_t,
     0xbef9a3f7b2c67915 as libc::c_ulonglong as uint64_t,
     0xc67178f2e372532b as libc::c_ulonglong as uint64_t,
     0xca273eceea26619c as libc::c_ulonglong as uint64_t,
     0xd186b8c721c0c207 as libc::c_ulonglong as uint64_t,
     0xeada7dd6cde0eb1e as libc::c_ulonglong as uint64_t,
     0xf57d4f7fee6ed178 as libc::c_ulonglong as uint64_t,
     0x6f067aa72176fba as libc::c_ulonglong as uint64_t,
     0xa637dc5a2c898a6 as libc::c_ulonglong as uint64_t,
     0x113f9804bef90dae as libc::c_ulonglong as uint64_t,
     0x1b710b35131c471b as libc::c_ulonglong as uint64_t,
     0x28db77f523047d84 as libc::c_ulonglong as uint64_t,
     0x32caab7b40c72493 as libc::c_ulonglong as uint64_t,
     0x3c9ebe0a15c9bebc as libc::c_ulonglong as uint64_t,
     0x431d67c49c100d4c as libc::c_ulonglong as uint64_t,
     0x4cc5d4becb3e42b6 as libc::c_ulonglong as uint64_t,
     0x597f299cfc657e2a as libc::c_ulonglong as uint64_t,
     0x5fcb6fab3ad6faec as libc::c_ulonglong as uint64_t,
     0x6c44198c4a475817 as libc::c_ulonglong as uint64_t];
#[inline]
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn cshift64(mut x: uint64_t, mut n: libc::c_uint)
 -> uint64_t {
    return x << n as uint64_t |
               x >>
                   (64 as libc::c_int as
                        uint64_t).wrapping_sub(n as uint64_t);
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn k5_sha512_init(mut m: *mut SHA512_CTX) {
    (*m).sz[0 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*m).sz[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*m).counter[0 as libc::c_int as usize] =
        0x6a09e667f3bcc908 as libc::c_ulonglong as uint64_t;
    (*m).counter[1 as libc::c_int as usize] =
        0xbb67ae8584caa73b as libc::c_ulonglong as uint64_t;
    (*m).counter[2 as libc::c_int as usize] =
        0x3c6ef372fe94f82b as libc::c_ulonglong as uint64_t;
    (*m).counter[3 as libc::c_int as usize] =
        0xa54ff53a5f1d36f1 as libc::c_ulonglong as uint64_t;
    (*m).counter[4 as libc::c_int as usize] =
        0x510e527fade682d1 as libc::c_ulonglong as uint64_t;
    (*m).counter[5 as libc::c_int as usize] =
        0x9b05688c2b3e6c1f as libc::c_ulonglong as uint64_t;
    (*m).counter[6 as libc::c_int as usize] =
        0x1f83d9abfb41bd6b as libc::c_ulonglong as uint64_t;
    (*m).counter[7 as libc::c_int as usize] =
        0x5be0cd19137e2179 as libc::c_ulonglong as uint64_t;
}
#[c2rust::src_loc = "128:1"]
unsafe extern "C" fn calc(mut m: *mut SHA512_CTX, mut in_0: *mut uint64_t) {
    let mut AA: uint64_t = 0;
    let mut BB: uint64_t = 0;
    let mut CC: uint64_t = 0;
    let mut DD: uint64_t = 0;
    let mut EE: uint64_t = 0;
    let mut FF: uint64_t = 0;
    let mut GG: uint64_t = 0;
    let mut HH: uint64_t = 0;
    let mut data: [uint64_t; 80] = [0; 80];
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
    while i < 80 as libc::c_int {
        data[i as usize] =
            ((data[(i - 2 as libc::c_int) as usize] >> 19 as libc::c_int |
                  data[(i - 2 as libc::c_int) as usize] <<
                      64 as libc::c_int - 19 as libc::c_int) ^
                 (data[(i - 2 as libc::c_int) as usize] >> 61 as libc::c_int |
                      data[(i - 2 as libc::c_int) as usize] <<
                          64 as libc::c_int - 61 as libc::c_int) ^
                 data[(i - 2 as libc::c_int) as usize] >>
                     6 as
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
                                                                                      1
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
                                                                                          64
                                                                                              as
                                                                                              libc::c_int
                                                                                              -
                                                                                              1
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
                                                                                          8
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
                                                                                              64
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  -
                                                                                                  8
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
                                                                                         7
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
    while i < 80 as libc::c_int {
        let mut T1: uint64_t = 0;
        let mut T2: uint64_t = 0;
        T1 =
            HH.wrapping_add((EE >> 14 as libc::c_int |
                                 EE << 64 as libc::c_int - 14 as libc::c_int)
                                ^
                                (EE >> 18 as libc::c_int |
                                     EE <<
                                         64 as libc::c_int -
                                             18 as libc::c_int) ^
                                (EE >> 41 as libc::c_int |
                                     EE <<
                                         64 as libc::c_int -
                                             41 as
                                                 libc::c_int)).wrapping_add(EE
                                                                                &
                                                                                FF
                                                                                ^
                                                                                !EE
                                                                                    &
                                                                                    GG).wrapping_add(constant_512[i
                                                                                                                      as
                                                                                                                      usize]).wrapping_add(data[i
                                                                                                                                                    as
                                                                                                                                                    usize]);
        T2 =
            ((AA >> 28 as libc::c_int |
                  AA << 64 as libc::c_int - 28 as libc::c_int) ^
                 (AA >> 34 as libc::c_int |
                      AA << 64 as libc::c_int - 34 as libc::c_int) ^
                 (AA >> 39 as libc::c_int |
                      AA <<
                          64 as libc::c_int -
                              39 as
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
             libc::c_ulong).wrapping_add(AA) as uint64_t as uint64_t;
    (*m).counter[1 as libc::c_int as usize] =
        ((*m).counter[1 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(BB) as uint64_t as uint64_t;
    (*m).counter[2 as libc::c_int as usize] =
        ((*m).counter[2 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(CC) as uint64_t as uint64_t;
    (*m).counter[3 as libc::c_int as usize] =
        ((*m).counter[3 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(DD) as uint64_t as uint64_t;
    (*m).counter[4 as libc::c_int as usize] =
        ((*m).counter[4 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(EE) as uint64_t as uint64_t;
    (*m).counter[5 as libc::c_int as usize] =
        ((*m).counter[5 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(FF) as uint64_t as uint64_t;
    (*m).counter[6 as libc::c_int as usize] =
        ((*m).counter[6 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(GG) as uint64_t as uint64_t;
    (*m).counter[7 as libc::c_int as usize] =
        ((*m).counter[7 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(HH) as uint64_t as uint64_t;
}
/*
 * From `Performance analysis of MD5' by Joseph D. Touch <touch@isi.edu>
 */
#[inline]
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn swap_uint64_t(mut t: uint64_t) -> uint64_t {
    let mut temp: uint64_t = 0;
    temp = cshift64(t, 32 as libc::c_int as libc::c_uint);
    temp =
        ((temp as libc::c_ulonglong & 0xff00ff00ff00ff00 as libc::c_ulonglong)
             >> 8 as libc::c_int |
             (temp as libc::c_ulonglong &
                  0xff00ff00ff00ff as libc::c_ulonglong) << 8 as libc::c_int)
            as uint64_t;
    return ((temp as libc::c_ulonglong &
                 0xffff0000ffff0000 as libc::c_ulonglong) >> 16 as libc::c_int
                |
                (temp as libc::c_ulonglong &
                     0xffff0000ffff as libc::c_ulonglong) <<
                    16 as libc::c_int) as uint64_t;
}
#[no_mangle]
#[c2rust::src_loc = "199:1"]
pub unsafe extern "C" fn k5_sha512_update(mut m: *mut SHA512_CTX,
                                          mut v: *const libc::c_void,
                                          mut len: size_t) {
    let mut p: *const libc::c_uchar = v as *const libc::c_uchar;
    let mut old_sz: size_t = (*m).sz[0 as libc::c_int as usize];
    let mut offset: size_t = 0;
    (*m).sz[0 as libc::c_int as usize] =
        ((*m).sz[0 as libc::c_int as usize] as
             libc::c_ulong).wrapping_add(len.wrapping_mul(8 as libc::c_int as
                                                              libc::c_ulong))
            as uint64_t as uint64_t;
    if (*m).sz[0 as libc::c_int as usize] < old_sz {
        (*m).sz[1 as libc::c_int as usize] =
            (*m).sz[1 as libc::c_int as usize].wrapping_add(1)
    }
    offset =
        old_sz.wrapping_div(8 as libc::c_int as
                                libc::c_ulong).wrapping_rem(128 as libc::c_int
                                                                as
                                                                libc::c_ulong);
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut l: size_t =
            if len >
                   (128 as libc::c_int as libc::c_ulong).wrapping_sub(offset)
               {
                (128 as libc::c_int as libc::c_ulong).wrapping_sub(offset)
            } else { len };
        memcpy((*m).save.as_mut_ptr().offset(offset as isize) as
                   *mut libc::c_void, p as *const libc::c_void, l);
        offset =
            (offset as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        p = p.offset(l as isize);
        len = (len as libc::c_ulong).wrapping_sub(l) as size_t as size_t;
        if offset == 128 as libc::c_int as libc::c_ulong {
            let mut i: libc::c_int = 0;
            let mut current: [uint64_t; 16] = [0; 16];
            let mut us: *mut x64 =
                (*m).save.as_mut_ptr() as *mut libc::c_void as *mut x64;
            i = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                current[(2 as libc::c_int * i + 0 as libc::c_int) as usize] =
                    swap_uint64_t((*us.offset(i as isize)).a);
                current[(2 as libc::c_int * i + 1 as libc::c_int) as usize] =
                    swap_uint64_t((*us.offset(i as isize)).b);
                i += 1
            }
            calc(m, current.as_mut_ptr());
            offset = 0 as libc::c_int as size_t
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "234:1"]
pub unsafe extern "C" fn k5_sha512_final(mut res: *mut libc::c_void,
                                         mut m: *mut SHA512_CTX) {
    let mut zeros: [libc::c_uchar; 144] = [0; 144];
    let mut offset: libc::c_uint =
        (*m).sz[0 as libc::c_int as
                    usize].wrapping_div(8 as libc::c_int as
                                            libc::c_ulong).wrapping_rem(128 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
            as libc::c_uint;
    let mut dstart: libc::c_uint =
        (240 as libc::c_int as
             libc::c_uint).wrapping_sub(offset).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_rem(128
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
           (::std::mem::size_of::<[libc::c_uchar; 144]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                libc::c_ulong));
    zeros[dstart.wrapping_add(15 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 0 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(14 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(13 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(12 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(11 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 32 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(10 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 40 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(9 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 48 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(8 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[0 as libc::c_int as usize] >> 56 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(7 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 0 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(6 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 8 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(5 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 16 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(4 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 24 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(3 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 32 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(2 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 40 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 48 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    zeros[dstart.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
        ((*m).sz[1 as libc::c_int as usize] >> 56 as libc::c_int &
             0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    k5_sha512_update(m, zeros.as_mut_ptr() as *const libc::c_void,
                     dstart.wrapping_add(16 as libc::c_int as libc::c_uint) as
                         size_t);
    let mut i: libc::c_int = 0;
    let mut r: *mut libc::c_uchar = res as *mut libc::c_uchar;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        *r.offset((8 as libc::c_int * i + 7 as libc::c_int) as isize) =
            ((*m).counter[i as usize] & 0xff as libc::c_int as libc::c_ulong)
                as libc::c_uchar;
        *r.offset((8 as libc::c_int * i + 6 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 8 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *r.offset((8 as libc::c_int * i + 5 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 16 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *r.offset((8 as libc::c_int * i + 4 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 24 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *r.offset((8 as libc::c_int * i + 3 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 32 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *r.offset((8 as libc::c_int * i + 2 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 40 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *r.offset((8 as libc::c_int * i + 1 as libc::c_int) as isize) =
            ((*m).counter[i as usize] >> 48 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *r.offset((8 as libc::c_int * i) as isize) =
            ((*m).counter[i as usize] >> 56 as libc::c_int &
                 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        i += 1
    };
}
#[no_mangle]
#[c2rust::src_loc = "278:1"]
pub unsafe extern "C" fn k5_sha384_init(mut m: *mut SHA384_CTX) {
    (*m).sz[0 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*m).sz[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    (*m).counter[0 as libc::c_int as usize] =
        0xcbbb9d5dc1059ed8 as libc::c_ulonglong as uint64_t;
    (*m).counter[1 as libc::c_int as usize] =
        0x629a292a367cd507 as libc::c_ulonglong as uint64_t;
    (*m).counter[2 as libc::c_int as usize] =
        0x9159015a3070dd17 as libc::c_ulonglong as uint64_t;
    (*m).counter[3 as libc::c_int as usize] =
        0x152fecd8f70e5939 as libc::c_ulonglong as uint64_t;
    (*m).counter[4 as libc::c_int as usize] =
        0x67332667ffc00b31 as libc::c_ulonglong as uint64_t;
    (*m).counter[5 as libc::c_int as usize] =
        0x8eb44a8768581511 as libc::c_ulonglong as uint64_t;
    (*m).counter[6 as libc::c_int as usize] =
        0xdb0c2e0d64f98fa7 as libc::c_ulonglong as uint64_t;
    (*m).counter[7 as libc::c_int as usize] =
        0x47b5481dbefa4fa4 as libc::c_ulonglong as uint64_t;
}
#[no_mangle]
#[c2rust::src_loc = "293:1"]
pub unsafe extern "C" fn k5_sha384_update(mut m: *mut SHA384_CTX,
                                          mut v: *const libc::c_void,
                                          mut len: size_t) {
    k5_sha512_update(m, v, len);
}
#[no_mangle]
#[c2rust::src_loc = "299:1"]
pub unsafe extern "C" fn k5_sha384_final(mut res: *mut libc::c_void,
                                         mut m: *mut SHA384_CTX) {
    let mut data: [libc::c_uchar; 64] = [0; 64];
    k5_sha512_final(data.as_mut_ptr() as *mut libc::c_void, m);
    memcpy(res, data.as_mut_ptr() as *const libc::c_void,
           48 as libc::c_int as libc::c_ulong);
}
