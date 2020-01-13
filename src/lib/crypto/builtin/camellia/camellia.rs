use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/camellia/camellia.h:37"]
pub mod camellia_h {
    /* u32 must be 32bit word */
    #[c2rust::src_loc = "48:1"]
    pub type u32_0 = uint32_t;
    #[c2rust::src_loc = "49:1"]
    pub type u8_0 = uint8_t;
    #[c2rust::src_loc = "86:1"]
    pub type cam_fret = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:9"]
    pub struct camellia_ctx {
        pub k_sch: [uint32_t; 68],
        pub keybitlen: libc::c_int,
    }
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    /* HEADER_CAMELLIA_H */
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __uint32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::camellia_h::{u32_0, u8_0, cam_fret, camellia_ctx};
use self::string_h::memcpy;
/* lib/crypto/builtin/camellia/camellia.c - Camellia version 1.2.0 */
/*
 * Copyright (c) 2006,2007,2009
 * NTT (Nippon Telegraph and Telephone Corporation) . All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer as
 *   the first lines of this file unmodified.
 * 2. Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY NTT ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL NTT BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * Algorithm Specification 
 *  https://info.isl.ntt.co.jp/crypt/eng/camellia/specifications.html
 */
/* key constants */
/*
 *  macros
 */
/* not MS-VC */
/* rotation right shift 1byte */
/* rotation left shift 1bit */
/* rotation left shift 1byte */
/*
 * for speed up
 *
 */
#[c2rust::src_loc = "176:18"]
static mut camellia_sp1110: [u32_0; 256] =
    [0x70707000 as libc::c_int as u32_0, 0x82828200 as libc::c_uint,
     0x2c2c2c00 as libc::c_int as u32_0, 0xececec00 as libc::c_uint,
     0xb3b3b300 as libc::c_uint, 0x27272700 as libc::c_int as u32_0,
     0xc0c0c000 as libc::c_uint, 0xe5e5e500 as libc::c_uint,
     0xe4e4e400 as libc::c_uint, 0x85858500 as libc::c_uint,
     0x57575700 as libc::c_int as u32_0, 0x35353500 as libc::c_int as u32_0,
     0xeaeaea00 as libc::c_uint, 0xc0c0c00 as libc::c_int as u32_0,
     0xaeaeae00 as libc::c_uint, 0x41414100 as libc::c_int as u32_0,
     0x23232300 as libc::c_int as u32_0, 0xefefef00 as libc::c_uint,
     0x6b6b6b00 as libc::c_int as u32_0, 0x93939300 as libc::c_uint,
     0x45454500 as libc::c_int as u32_0, 0x19191900 as libc::c_int as u32_0,
     0xa5a5a500 as libc::c_uint, 0x21212100 as libc::c_int as u32_0,
     0xededed00 as libc::c_uint, 0xe0e0e00 as libc::c_int as u32_0,
     0x4f4f4f00 as libc::c_int as u32_0, 0x4e4e4e00 as libc::c_int as u32_0,
     0x1d1d1d00 as libc::c_int as u32_0, 0x65656500 as libc::c_int as u32_0,
     0x92929200 as libc::c_uint, 0xbdbdbd00 as libc::c_uint,
     0x86868600 as libc::c_uint, 0xb8b8b800 as libc::c_uint,
     0xafafaf00 as libc::c_uint, 0x8f8f8f00 as libc::c_uint,
     0x7c7c7c00 as libc::c_int as u32_0, 0xebebeb00 as libc::c_uint,
     0x1f1f1f00 as libc::c_int as u32_0, 0xcecece00 as libc::c_uint,
     0x3e3e3e00 as libc::c_int as u32_0, 0x30303000 as libc::c_int as u32_0,
     0xdcdcdc00 as libc::c_uint, 0x5f5f5f00 as libc::c_int as u32_0,
     0x5e5e5e00 as libc::c_int as u32_0, 0xc5c5c500 as libc::c_uint,
     0xb0b0b00 as libc::c_int as u32_0, 0x1a1a1a00 as libc::c_int as u32_0,
     0xa6a6a600 as libc::c_uint, 0xe1e1e100 as libc::c_uint,
     0x39393900 as libc::c_int as u32_0, 0xcacaca00 as libc::c_uint,
     0xd5d5d500 as libc::c_uint, 0x47474700 as libc::c_int as u32_0,
     0x5d5d5d00 as libc::c_int as u32_0, 0x3d3d3d00 as libc::c_int as u32_0,
     0xd9d9d900 as libc::c_uint, 0x1010100 as libc::c_int as u32_0,
     0x5a5a5a00 as libc::c_int as u32_0, 0xd6d6d600 as libc::c_uint,
     0x51515100 as libc::c_int as u32_0, 0x56565600 as libc::c_int as u32_0,
     0x6c6c6c00 as libc::c_int as u32_0, 0x4d4d4d00 as libc::c_int as u32_0,
     0x8b8b8b00 as libc::c_uint, 0xd0d0d00 as libc::c_int as u32_0,
     0x9a9a9a00 as libc::c_uint, 0x66666600 as libc::c_int as u32_0,
     0xfbfbfb00 as libc::c_uint, 0xcccccc00 as libc::c_uint,
     0xb0b0b000 as libc::c_uint, 0x2d2d2d00 as libc::c_int as u32_0,
     0x74747400 as libc::c_int as u32_0, 0x12121200 as libc::c_int as u32_0,
     0x2b2b2b00 as libc::c_int as u32_0, 0x20202000 as libc::c_int as u32_0,
     0xf0f0f000 as libc::c_uint, 0xb1b1b100 as libc::c_uint,
     0x84848400 as libc::c_uint, 0x99999900 as libc::c_uint,
     0xdfdfdf00 as libc::c_uint, 0x4c4c4c00 as libc::c_int as u32_0,
     0xcbcbcb00 as libc::c_uint, 0xc2c2c200 as libc::c_uint,
     0x34343400 as libc::c_int as u32_0, 0x7e7e7e00 as libc::c_int as u32_0,
     0x76767600 as libc::c_int as u32_0, 0x5050500 as libc::c_int as u32_0,
     0x6d6d6d00 as libc::c_int as u32_0, 0xb7b7b700 as libc::c_uint,
     0xa9a9a900 as libc::c_uint, 0x31313100 as libc::c_int as u32_0,
     0xd1d1d100 as libc::c_uint, 0x17171700 as libc::c_int as u32_0,
     0x4040400 as libc::c_int as u32_0, 0xd7d7d700 as libc::c_uint,
     0x14141400 as libc::c_int as u32_0, 0x58585800 as libc::c_int as u32_0,
     0x3a3a3a00 as libc::c_int as u32_0, 0x61616100 as libc::c_int as u32_0,
     0xdedede00 as libc::c_uint, 0x1b1b1b00 as libc::c_int as u32_0,
     0x11111100 as libc::c_int as u32_0, 0x1c1c1c00 as libc::c_int as u32_0,
     0x32323200 as libc::c_int as u32_0, 0xf0f0f00 as libc::c_int as u32_0,
     0x9c9c9c00 as libc::c_uint, 0x16161600 as libc::c_int as u32_0,
     0x53535300 as libc::c_int as u32_0, 0x18181800 as libc::c_int as u32_0,
     0xf2f2f200 as libc::c_uint, 0x22222200 as libc::c_int as u32_0,
     0xfefefe00 as libc::c_uint, 0x44444400 as libc::c_int as u32_0,
     0xcfcfcf00 as libc::c_uint, 0xb2b2b200 as libc::c_uint,
     0xc3c3c300 as libc::c_uint, 0xb5b5b500 as libc::c_uint,
     0x7a7a7a00 as libc::c_int as u32_0, 0x91919100 as libc::c_uint,
     0x24242400 as libc::c_int as u32_0, 0x8080800 as libc::c_int as u32_0,
     0xe8e8e800 as libc::c_uint, 0xa8a8a800 as libc::c_uint,
     0x60606000 as libc::c_int as u32_0, 0xfcfcfc00 as libc::c_uint,
     0x69696900 as libc::c_int as u32_0, 0x50505000 as libc::c_int as u32_0,
     0xaaaaaa00 as libc::c_uint, 0xd0d0d000 as libc::c_uint,
     0xa0a0a000 as libc::c_uint, 0x7d7d7d00 as libc::c_int as u32_0,
     0xa1a1a100 as libc::c_uint, 0x89898900 as libc::c_uint,
     0x62626200 as libc::c_int as u32_0, 0x97979700 as libc::c_uint,
     0x54545400 as libc::c_int as u32_0, 0x5b5b5b00 as libc::c_int as u32_0,
     0x1e1e1e00 as libc::c_int as u32_0, 0x95959500 as libc::c_uint,
     0xe0e0e000 as libc::c_uint, 0xffffff00 as libc::c_uint,
     0x64646400 as libc::c_int as u32_0, 0xd2d2d200 as libc::c_uint,
     0x10101000 as libc::c_int as u32_0, 0xc4c4c400 as libc::c_uint,
     0 as libc::c_int as u32_0, 0x48484800 as libc::c_int as u32_0,
     0xa3a3a300 as libc::c_uint, 0xf7f7f700 as libc::c_uint,
     0x75757500 as libc::c_int as u32_0, 0xdbdbdb00 as libc::c_uint,
     0x8a8a8a00 as libc::c_uint, 0x3030300 as libc::c_int as u32_0,
     0xe6e6e600 as libc::c_uint, 0xdadada00 as libc::c_uint,
     0x9090900 as libc::c_int as u32_0, 0x3f3f3f00 as libc::c_int as u32_0,
     0xdddddd00 as libc::c_uint, 0x94949400 as libc::c_uint,
     0x87878700 as libc::c_uint, 0x5c5c5c00 as libc::c_int as u32_0,
     0x83838300 as libc::c_uint, 0x2020200 as libc::c_int as u32_0,
     0xcdcdcd00 as libc::c_uint, 0x4a4a4a00 as libc::c_int as u32_0,
     0x90909000 as libc::c_uint, 0x33333300 as libc::c_int as u32_0,
     0x73737300 as libc::c_int as u32_0, 0x67676700 as libc::c_int as u32_0,
     0xf6f6f600 as libc::c_uint, 0xf3f3f300 as libc::c_uint,
     0x9d9d9d00 as libc::c_uint, 0x7f7f7f00 as libc::c_int as u32_0,
     0xbfbfbf00 as libc::c_uint, 0xe2e2e200 as libc::c_uint,
     0x52525200 as libc::c_int as u32_0, 0x9b9b9b00 as libc::c_uint,
     0xd8d8d800 as libc::c_uint, 0x26262600 as libc::c_int as u32_0,
     0xc8c8c800 as libc::c_uint, 0x37373700 as libc::c_int as u32_0,
     0xc6c6c600 as libc::c_uint, 0x3b3b3b00 as libc::c_int as u32_0,
     0x81818100 as libc::c_uint, 0x96969600 as libc::c_uint,
     0x6f6f6f00 as libc::c_int as u32_0, 0x4b4b4b00 as libc::c_int as u32_0,
     0x13131300 as libc::c_int as u32_0, 0xbebebe00 as libc::c_uint,
     0x63636300 as libc::c_int as u32_0, 0x2e2e2e00 as libc::c_int as u32_0,
     0xe9e9e900 as libc::c_uint, 0x79797900 as libc::c_int as u32_0,
     0xa7a7a700 as libc::c_uint, 0x8c8c8c00 as libc::c_uint,
     0x9f9f9f00 as libc::c_uint, 0x6e6e6e00 as libc::c_int as u32_0,
     0xbcbcbc00 as libc::c_uint, 0x8e8e8e00 as libc::c_uint,
     0x29292900 as libc::c_int as u32_0, 0xf5f5f500 as libc::c_uint,
     0xf9f9f900 as libc::c_uint, 0xb6b6b600 as libc::c_uint,
     0x2f2f2f00 as libc::c_int as u32_0, 0xfdfdfd00 as libc::c_uint,
     0xb4b4b400 as libc::c_uint, 0x59595900 as libc::c_int as u32_0,
     0x78787800 as libc::c_int as u32_0, 0x98989800 as libc::c_uint,
     0x6060600 as libc::c_int as u32_0, 0x6a6a6a00 as libc::c_int as u32_0,
     0xe7e7e700 as libc::c_uint, 0x46464600 as libc::c_int as u32_0,
     0x71717100 as libc::c_int as u32_0, 0xbababa00 as libc::c_uint,
     0xd4d4d400 as libc::c_uint, 0x25252500 as libc::c_int as u32_0,
     0xababab00 as libc::c_uint, 0x42424200 as libc::c_int as u32_0,
     0x88888800 as libc::c_uint, 0xa2a2a200 as libc::c_uint,
     0x8d8d8d00 as libc::c_uint, 0xfafafa00 as libc::c_uint,
     0x72727200 as libc::c_int as u32_0, 0x7070700 as libc::c_int as u32_0,
     0xb9b9b900 as libc::c_uint, 0x55555500 as libc::c_int as u32_0,
     0xf8f8f800 as libc::c_uint, 0xeeeeee00 as libc::c_uint,
     0xacacac00 as libc::c_uint, 0xa0a0a00 as libc::c_int as u32_0,
     0x36363600 as libc::c_int as u32_0, 0x49494900 as libc::c_int as u32_0,
     0x2a2a2a00 as libc::c_int as u32_0, 0x68686800 as libc::c_int as u32_0,
     0x3c3c3c00 as libc::c_int as u32_0, 0x38383800 as libc::c_int as u32_0,
     0xf1f1f100 as libc::c_uint, 0xa4a4a400 as libc::c_uint,
     0x40404000 as libc::c_int as u32_0, 0x28282800 as libc::c_int as u32_0,
     0xd3d3d300 as libc::c_uint, 0x7b7b7b00 as libc::c_int as u32_0,
     0xbbbbbb00 as libc::c_uint, 0xc9c9c900 as libc::c_uint,
     0x43434300 as libc::c_int as u32_0, 0xc1c1c100 as libc::c_uint,
     0x15151500 as libc::c_int as u32_0, 0xe3e3e300 as libc::c_uint,
     0xadadad00 as libc::c_uint, 0xf4f4f400 as libc::c_uint,
     0x77777700 as libc::c_int as u32_0, 0xc7c7c700 as libc::c_uint,
     0x80808000 as libc::c_uint, 0x9e9e9e00 as libc::c_uint];
#[c2rust::src_loc = "243:18"]
static mut camellia_sp0222: [u32_0; 256] =
    [0xe0e0e0 as libc::c_int as u32_0, 0x50505 as libc::c_int as u32_0,
     0x585858 as libc::c_int as u32_0, 0xd9d9d9 as libc::c_int as u32_0,
     0x676767 as libc::c_int as u32_0, 0x4e4e4e as libc::c_int as u32_0,
     0x818181 as libc::c_int as u32_0, 0xcbcbcb as libc::c_int as u32_0,
     0xc9c9c9 as libc::c_int as u32_0, 0xb0b0b as libc::c_int as u32_0,
     0xaeaeae as libc::c_int as u32_0, 0x6a6a6a as libc::c_int as u32_0,
     0xd5d5d5 as libc::c_int as u32_0, 0x181818 as libc::c_int as u32_0,
     0x5d5d5d as libc::c_int as u32_0, 0x828282 as libc::c_int as u32_0,
     0x464646 as libc::c_int as u32_0, 0xdfdfdf as libc::c_int as u32_0,
     0xd6d6d6 as libc::c_int as u32_0, 0x272727 as libc::c_int as u32_0,
     0x8a8a8a as libc::c_int as u32_0, 0x323232 as libc::c_int as u32_0,
     0x4b4b4b as libc::c_int as u32_0, 0x424242 as libc::c_int as u32_0,
     0xdbdbdb as libc::c_int as u32_0, 0x1c1c1c as libc::c_int as u32_0,
     0x9e9e9e as libc::c_int as u32_0, 0x9c9c9c as libc::c_int as u32_0,
     0x3a3a3a as libc::c_int as u32_0, 0xcacaca as libc::c_int as u32_0,
     0x252525 as libc::c_int as u32_0, 0x7b7b7b as libc::c_int as u32_0,
     0xd0d0d as libc::c_int as u32_0, 0x717171 as libc::c_int as u32_0,
     0x5f5f5f as libc::c_int as u32_0, 0x1f1f1f as libc::c_int as u32_0,
     0xf8f8f8 as libc::c_int as u32_0, 0xd7d7d7 as libc::c_int as u32_0,
     0x3e3e3e as libc::c_int as u32_0, 0x9d9d9d as libc::c_int as u32_0,
     0x7c7c7c as libc::c_int as u32_0, 0x606060 as libc::c_int as u32_0,
     0xb9b9b9 as libc::c_int as u32_0, 0xbebebe as libc::c_int as u32_0,
     0xbcbcbc as libc::c_int as u32_0, 0x8b8b8b as libc::c_int as u32_0,
     0x161616 as libc::c_int as u32_0, 0x343434 as libc::c_int as u32_0,
     0x4d4d4d as libc::c_int as u32_0, 0xc3c3c3 as libc::c_int as u32_0,
     0x727272 as libc::c_int as u32_0, 0x959595 as libc::c_int as u32_0,
     0xababab as libc::c_int as u32_0, 0x8e8e8e as libc::c_int as u32_0,
     0xbababa as libc::c_int as u32_0, 0x7a7a7a as libc::c_int as u32_0,
     0xb3b3b3 as libc::c_int as u32_0, 0x20202 as libc::c_int as u32_0,
     0xb4b4b4 as libc::c_int as u32_0, 0xadadad as libc::c_int as u32_0,
     0xa2a2a2 as libc::c_int as u32_0, 0xacacac as libc::c_int as u32_0,
     0xd8d8d8 as libc::c_int as u32_0, 0x9a9a9a as libc::c_int as u32_0,
     0x171717 as libc::c_int as u32_0, 0x1a1a1a as libc::c_int as u32_0,
     0x353535 as libc::c_int as u32_0, 0xcccccc as libc::c_int as u32_0,
     0xf7f7f7 as libc::c_int as u32_0, 0x999999 as libc::c_int as u32_0,
     0x616161 as libc::c_int as u32_0, 0x5a5a5a as libc::c_int as u32_0,
     0xe8e8e8 as libc::c_int as u32_0, 0x242424 as libc::c_int as u32_0,
     0x565656 as libc::c_int as u32_0, 0x404040 as libc::c_int as u32_0,
     0xe1e1e1 as libc::c_int as u32_0, 0x636363 as libc::c_int as u32_0,
     0x90909 as libc::c_int as u32_0, 0x333333 as libc::c_int as u32_0,
     0xbfbfbf as libc::c_int as u32_0, 0x989898 as libc::c_int as u32_0,
     0x979797 as libc::c_int as u32_0, 0x858585 as libc::c_int as u32_0,
     0x686868 as libc::c_int as u32_0, 0xfcfcfc as libc::c_int as u32_0,
     0xececec as libc::c_int as u32_0, 0xa0a0a as libc::c_int as u32_0,
     0xdadada as libc::c_int as u32_0, 0x6f6f6f as libc::c_int as u32_0,
     0x535353 as libc::c_int as u32_0, 0x626262 as libc::c_int as u32_0,
     0xa3a3a3 as libc::c_int as u32_0, 0x2e2e2e as libc::c_int as u32_0,
     0x80808 as libc::c_int as u32_0, 0xafafaf as libc::c_int as u32_0,
     0x282828 as libc::c_int as u32_0, 0xb0b0b0 as libc::c_int as u32_0,
     0x747474 as libc::c_int as u32_0, 0xc2c2c2 as libc::c_int as u32_0,
     0xbdbdbd as libc::c_int as u32_0, 0x363636 as libc::c_int as u32_0,
     0x222222 as libc::c_int as u32_0, 0x383838 as libc::c_int as u32_0,
     0x646464 as libc::c_int as u32_0, 0x1e1e1e as libc::c_int as u32_0,
     0x393939 as libc::c_int as u32_0, 0x2c2c2c as libc::c_int as u32_0,
     0xa6a6a6 as libc::c_int as u32_0, 0x303030 as libc::c_int as u32_0,
     0xe5e5e5 as libc::c_int as u32_0, 0x444444 as libc::c_int as u32_0,
     0xfdfdfd as libc::c_int as u32_0, 0x888888 as libc::c_int as u32_0,
     0x9f9f9f as libc::c_int as u32_0, 0x656565 as libc::c_int as u32_0,
     0x878787 as libc::c_int as u32_0, 0x6b6b6b as libc::c_int as u32_0,
     0xf4f4f4 as libc::c_int as u32_0, 0x232323 as libc::c_int as u32_0,
     0x484848 as libc::c_int as u32_0, 0x101010 as libc::c_int as u32_0,
     0xd1d1d1 as libc::c_int as u32_0, 0x515151 as libc::c_int as u32_0,
     0xc0c0c0 as libc::c_int as u32_0, 0xf9f9f9 as libc::c_int as u32_0,
     0xd2d2d2 as libc::c_int as u32_0, 0xa0a0a0 as libc::c_int as u32_0,
     0x555555 as libc::c_int as u32_0, 0xa1a1a1 as libc::c_int as u32_0,
     0x414141 as libc::c_int as u32_0, 0xfafafa as libc::c_int as u32_0,
     0x434343 as libc::c_int as u32_0, 0x131313 as libc::c_int as u32_0,
     0xc4c4c4 as libc::c_int as u32_0, 0x2f2f2f as libc::c_int as u32_0,
     0xa8a8a8 as libc::c_int as u32_0, 0xb6b6b6 as libc::c_int as u32_0,
     0x3c3c3c as libc::c_int as u32_0, 0x2b2b2b as libc::c_int as u32_0,
     0xc1c1c1 as libc::c_int as u32_0, 0xffffff as libc::c_int as u32_0,
     0xc8c8c8 as libc::c_int as u32_0, 0xa5a5a5 as libc::c_int as u32_0,
     0x202020 as libc::c_int as u32_0, 0x898989 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x909090 as libc::c_int as u32_0,
     0x474747 as libc::c_int as u32_0, 0xefefef as libc::c_int as u32_0,
     0xeaeaea as libc::c_int as u32_0, 0xb7b7b7 as libc::c_int as u32_0,
     0x151515 as libc::c_int as u32_0, 0x60606 as libc::c_int as u32_0,
     0xcdcdcd as libc::c_int as u32_0, 0xb5b5b5 as libc::c_int as u32_0,
     0x121212 as libc::c_int as u32_0, 0x7e7e7e as libc::c_int as u32_0,
     0xbbbbbb as libc::c_int as u32_0, 0x292929 as libc::c_int as u32_0,
     0xf0f0f as libc::c_int as u32_0, 0xb8b8b8 as libc::c_int as u32_0,
     0x70707 as libc::c_int as u32_0, 0x40404 as libc::c_int as u32_0,
     0x9b9b9b as libc::c_int as u32_0, 0x949494 as libc::c_int as u32_0,
     0x212121 as libc::c_int as u32_0, 0x666666 as libc::c_int as u32_0,
     0xe6e6e6 as libc::c_int as u32_0, 0xcecece as libc::c_int as u32_0,
     0xededed as libc::c_int as u32_0, 0xe7e7e7 as libc::c_int as u32_0,
     0x3b3b3b as libc::c_int as u32_0, 0xfefefe as libc::c_int as u32_0,
     0x7f7f7f as libc::c_int as u32_0, 0xc5c5c5 as libc::c_int as u32_0,
     0xa4a4a4 as libc::c_int as u32_0, 0x373737 as libc::c_int as u32_0,
     0xb1b1b1 as libc::c_int as u32_0, 0x4c4c4c as libc::c_int as u32_0,
     0x919191 as libc::c_int as u32_0, 0x6e6e6e as libc::c_int as u32_0,
     0x8d8d8d as libc::c_int as u32_0, 0x767676 as libc::c_int as u32_0,
     0x30303 as libc::c_int as u32_0, 0x2d2d2d as libc::c_int as u32_0,
     0xdedede as libc::c_int as u32_0, 0x969696 as libc::c_int as u32_0,
     0x262626 as libc::c_int as u32_0, 0x7d7d7d as libc::c_int as u32_0,
     0xc6c6c6 as libc::c_int as u32_0, 0x5c5c5c as libc::c_int as u32_0,
     0xd3d3d3 as libc::c_int as u32_0, 0xf2f2f2 as libc::c_int as u32_0,
     0x4f4f4f as libc::c_int as u32_0, 0x191919 as libc::c_int as u32_0,
     0x3f3f3f as libc::c_int as u32_0, 0xdcdcdc as libc::c_int as u32_0,
     0x797979 as libc::c_int as u32_0, 0x1d1d1d as libc::c_int as u32_0,
     0x525252 as libc::c_int as u32_0, 0xebebeb as libc::c_int as u32_0,
     0xf3f3f3 as libc::c_int as u32_0, 0x6d6d6d as libc::c_int as u32_0,
     0x5e5e5e as libc::c_int as u32_0, 0xfbfbfb as libc::c_int as u32_0,
     0x696969 as libc::c_int as u32_0, 0xb2b2b2 as libc::c_int as u32_0,
     0xf0f0f0 as libc::c_int as u32_0, 0x313131 as libc::c_int as u32_0,
     0xc0c0c as libc::c_int as u32_0, 0xd4d4d4 as libc::c_int as u32_0,
     0xcfcfcf as libc::c_int as u32_0, 0x8c8c8c as libc::c_int as u32_0,
     0xe2e2e2 as libc::c_int as u32_0, 0x757575 as libc::c_int as u32_0,
     0xa9a9a9 as libc::c_int as u32_0, 0x4a4a4a as libc::c_int as u32_0,
     0x575757 as libc::c_int as u32_0, 0x848484 as libc::c_int as u32_0,
     0x111111 as libc::c_int as u32_0, 0x454545 as libc::c_int as u32_0,
     0x1b1b1b as libc::c_int as u32_0, 0xf5f5f5 as libc::c_int as u32_0,
     0xe4e4e4 as libc::c_int as u32_0, 0xe0e0e as libc::c_int as u32_0,
     0x737373 as libc::c_int as u32_0, 0xaaaaaa as libc::c_int as u32_0,
     0xf1f1f1 as libc::c_int as u32_0, 0xdddddd as libc::c_int as u32_0,
     0x595959 as libc::c_int as u32_0, 0x141414 as libc::c_int as u32_0,
     0x6c6c6c as libc::c_int as u32_0, 0x929292 as libc::c_int as u32_0,
     0x545454 as libc::c_int as u32_0, 0xd0d0d0 as libc::c_int as u32_0,
     0x787878 as libc::c_int as u32_0, 0x707070 as libc::c_int as u32_0,
     0xe3e3e3 as libc::c_int as u32_0, 0x494949 as libc::c_int as u32_0,
     0x808080 as libc::c_int as u32_0, 0x505050 as libc::c_int as u32_0,
     0xa7a7a7 as libc::c_int as u32_0, 0xf6f6f6 as libc::c_int as u32_0,
     0x777777 as libc::c_int as u32_0, 0x939393 as libc::c_int as u32_0,
     0x868686 as libc::c_int as u32_0, 0x838383 as libc::c_int as u32_0,
     0x2a2a2a as libc::c_int as u32_0, 0xc7c7c7 as libc::c_int as u32_0,
     0x5b5b5b as libc::c_int as u32_0, 0xe9e9e9 as libc::c_int as u32_0,
     0xeeeeee as libc::c_int as u32_0, 0x8f8f8f as libc::c_int as u32_0,
     0x10101 as libc::c_int as u32_0, 0x3d3d3d as libc::c_int as u32_0];
#[c2rust::src_loc = "310:18"]
static mut camellia_sp3033: [u32_0; 256] =
    [0x38003838 as libc::c_int as u32_0, 0x41004141 as libc::c_int as u32_0,
     0x16001616 as libc::c_int as u32_0, 0x76007676 as libc::c_int as u32_0,
     0xd900d9d9 as libc::c_uint, 0x93009393 as libc::c_uint,
     0x60006060 as libc::c_int as u32_0, 0xf200f2f2 as libc::c_uint,
     0x72007272 as libc::c_int as u32_0, 0xc200c2c2 as libc::c_uint,
     0xab00abab as libc::c_uint, 0x9a009a9a as libc::c_uint,
     0x75007575 as libc::c_int as u32_0, 0x6000606 as libc::c_int as u32_0,
     0x57005757 as libc::c_int as u32_0, 0xa000a0a0 as libc::c_uint,
     0x91009191 as libc::c_uint, 0xf700f7f7 as libc::c_uint,
     0xb500b5b5 as libc::c_uint, 0xc900c9c9 as libc::c_uint,
     0xa200a2a2 as libc::c_uint, 0x8c008c8c as libc::c_uint,
     0xd200d2d2 as libc::c_uint, 0x90009090 as libc::c_uint,
     0xf600f6f6 as libc::c_uint, 0x7000707 as libc::c_int as u32_0,
     0xa700a7a7 as libc::c_uint, 0x27002727 as libc::c_int as u32_0,
     0x8e008e8e as libc::c_uint, 0xb200b2b2 as libc::c_uint,
     0x49004949 as libc::c_int as u32_0, 0xde00dede as libc::c_uint,
     0x43004343 as libc::c_int as u32_0, 0x5c005c5c as libc::c_int as u32_0,
     0xd700d7d7 as libc::c_uint, 0xc700c7c7 as libc::c_uint,
     0x3e003e3e as libc::c_int as u32_0, 0xf500f5f5 as libc::c_uint,
     0x8f008f8f as libc::c_uint, 0x67006767 as libc::c_int as u32_0,
     0x1f001f1f as libc::c_int as u32_0, 0x18001818 as libc::c_int as u32_0,
     0x6e006e6e as libc::c_int as u32_0, 0xaf00afaf as libc::c_uint,
     0x2f002f2f as libc::c_int as u32_0, 0xe200e2e2 as libc::c_uint,
     0x85008585 as libc::c_uint, 0xd000d0d as libc::c_int as u32_0,
     0x53005353 as libc::c_int as u32_0, 0xf000f0f0 as libc::c_uint,
     0x9c009c9c as libc::c_uint, 0x65006565 as libc::c_int as u32_0,
     0xea00eaea as libc::c_uint, 0xa300a3a3 as libc::c_uint,
     0xae00aeae as libc::c_uint, 0x9e009e9e as libc::c_uint,
     0xec00ecec as libc::c_uint, 0x80008080 as libc::c_uint,
     0x2d002d2d as libc::c_int as u32_0, 0x6b006b6b as libc::c_int as u32_0,
     0xa800a8a8 as libc::c_uint, 0x2b002b2b as libc::c_int as u32_0,
     0x36003636 as libc::c_int as u32_0, 0xa600a6a6 as libc::c_uint,
     0xc500c5c5 as libc::c_uint, 0x86008686 as libc::c_uint,
     0x4d004d4d as libc::c_int as u32_0, 0x33003333 as libc::c_int as u32_0,
     0xfd00fdfd as libc::c_uint, 0x66006666 as libc::c_int as u32_0,
     0x58005858 as libc::c_int as u32_0, 0x96009696 as libc::c_uint,
     0x3a003a3a as libc::c_int as u32_0, 0x9000909 as libc::c_int as u32_0,
     0x95009595 as libc::c_uint, 0x10001010 as libc::c_int as u32_0,
     0x78007878 as libc::c_int as u32_0, 0xd800d8d8 as libc::c_uint,
     0x42004242 as libc::c_int as u32_0, 0xcc00cccc as libc::c_uint,
     0xef00efef as libc::c_uint, 0x26002626 as libc::c_int as u32_0,
     0xe500e5e5 as libc::c_uint, 0x61006161 as libc::c_int as u32_0,
     0x1a001a1a as libc::c_int as u32_0, 0x3f003f3f as libc::c_int as u32_0,
     0x3b003b3b as libc::c_int as u32_0, 0x82008282 as libc::c_uint,
     0xb600b6b6 as libc::c_uint, 0xdb00dbdb as libc::c_uint,
     0xd400d4d4 as libc::c_uint, 0x98009898 as libc::c_uint,
     0xe800e8e8 as libc::c_uint, 0x8b008b8b as libc::c_uint,
     0x2000202 as libc::c_int as u32_0, 0xeb00ebeb as libc::c_uint,
     0xa000a0a as libc::c_int as u32_0, 0x2c002c2c as libc::c_int as u32_0,
     0x1d001d1d as libc::c_int as u32_0, 0xb000b0b0 as libc::c_uint,
     0x6f006f6f as libc::c_int as u32_0, 0x8d008d8d as libc::c_uint,
     0x88008888 as libc::c_uint, 0xe000e0e as libc::c_int as u32_0,
     0x19001919 as libc::c_int as u32_0, 0x87008787 as libc::c_uint,
     0x4e004e4e as libc::c_int as u32_0, 0xb000b0b as libc::c_int as u32_0,
     0xa900a9a9 as libc::c_uint, 0xc000c0c as libc::c_int as u32_0,
     0x79007979 as libc::c_int as u32_0, 0x11001111 as libc::c_int as u32_0,
     0x7f007f7f as libc::c_int as u32_0, 0x22002222 as libc::c_int as u32_0,
     0xe700e7e7 as libc::c_uint, 0x59005959 as libc::c_int as u32_0,
     0xe100e1e1 as libc::c_uint, 0xda00dada as libc::c_uint,
     0x3d003d3d as libc::c_int as u32_0, 0xc800c8c8 as libc::c_uint,
     0x12001212 as libc::c_int as u32_0, 0x4000404 as libc::c_int as u32_0,
     0x74007474 as libc::c_int as u32_0, 0x54005454 as libc::c_int as u32_0,
     0x30003030 as libc::c_int as u32_0, 0x7e007e7e as libc::c_int as u32_0,
     0xb400b4b4 as libc::c_uint, 0x28002828 as libc::c_int as u32_0,
     0x55005555 as libc::c_int as u32_0, 0x68006868 as libc::c_int as u32_0,
     0x50005050 as libc::c_int as u32_0, 0xbe00bebe as libc::c_uint,
     0xd000d0d0 as libc::c_uint, 0xc400c4c4 as libc::c_uint,
     0x31003131 as libc::c_int as u32_0, 0xcb00cbcb as libc::c_uint,
     0x2a002a2a as libc::c_int as u32_0, 0xad00adad as libc::c_uint,
     0xf000f0f as libc::c_int as u32_0, 0xca00caca as libc::c_uint,
     0x70007070 as libc::c_int as u32_0, 0xff00ffff as libc::c_uint,
     0x32003232 as libc::c_int as u32_0, 0x69006969 as libc::c_int as u32_0,
     0x8000808 as libc::c_int as u32_0, 0x62006262 as libc::c_int as u32_0,
     0 as libc::c_int as u32_0, 0x24002424 as libc::c_int as u32_0,
     0xd100d1d1 as libc::c_uint, 0xfb00fbfb as libc::c_uint,
     0xba00baba as libc::c_uint, 0xed00eded as libc::c_uint,
     0x45004545 as libc::c_int as u32_0, 0x81008181 as libc::c_uint,
     0x73007373 as libc::c_int as u32_0, 0x6d006d6d as libc::c_int as u32_0,
     0x84008484 as libc::c_uint, 0x9f009f9f as libc::c_uint,
     0xee00eeee as libc::c_uint, 0x4a004a4a as libc::c_int as u32_0,
     0xc300c3c3 as libc::c_uint, 0x2e002e2e as libc::c_int as u32_0,
     0xc100c1c1 as libc::c_uint, 0x1000101 as libc::c_int as u32_0,
     0xe600e6e6 as libc::c_uint, 0x25002525 as libc::c_int as u32_0,
     0x48004848 as libc::c_int as u32_0, 0x99009999 as libc::c_uint,
     0xb900b9b9 as libc::c_uint, 0xb300b3b3 as libc::c_uint,
     0x7b007b7b as libc::c_int as u32_0, 0xf900f9f9 as libc::c_uint,
     0xce00cece as libc::c_uint, 0xbf00bfbf as libc::c_uint,
     0xdf00dfdf as libc::c_uint, 0x71007171 as libc::c_int as u32_0,
     0x29002929 as libc::c_int as u32_0, 0xcd00cdcd as libc::c_uint,
     0x6c006c6c as libc::c_int as u32_0, 0x13001313 as libc::c_int as u32_0,
     0x64006464 as libc::c_int as u32_0, 0x9b009b9b as libc::c_uint,
     0x63006363 as libc::c_int as u32_0, 0x9d009d9d as libc::c_uint,
     0xc000c0c0 as libc::c_uint, 0x4b004b4b as libc::c_int as u32_0,
     0xb700b7b7 as libc::c_uint, 0xa500a5a5 as libc::c_uint,
     0x89008989 as libc::c_uint, 0x5f005f5f as libc::c_int as u32_0,
     0xb100b1b1 as libc::c_uint, 0x17001717 as libc::c_int as u32_0,
     0xf400f4f4 as libc::c_uint, 0xbc00bcbc as libc::c_uint,
     0xd300d3d3 as libc::c_uint, 0x46004646 as libc::c_int as u32_0,
     0xcf00cfcf as libc::c_uint, 0x37003737 as libc::c_int as u32_0,
     0x5e005e5e as libc::c_int as u32_0, 0x47004747 as libc::c_int as u32_0,
     0x94009494 as libc::c_uint, 0xfa00fafa as libc::c_uint,
     0xfc00fcfc as libc::c_uint, 0x5b005b5b as libc::c_int as u32_0,
     0x97009797 as libc::c_uint, 0xfe00fefe as libc::c_uint,
     0x5a005a5a as libc::c_int as u32_0, 0xac00acac as libc::c_uint,
     0x3c003c3c as libc::c_int as u32_0, 0x4c004c4c as libc::c_int as u32_0,
     0x3000303 as libc::c_int as u32_0, 0x35003535 as libc::c_int as u32_0,
     0xf300f3f3 as libc::c_uint, 0x23002323 as libc::c_int as u32_0,
     0xb800b8b8 as libc::c_uint, 0x5d005d5d as libc::c_int as u32_0,
     0x6a006a6a as libc::c_int as u32_0, 0x92009292 as libc::c_uint,
     0xd500d5d5 as libc::c_uint, 0x21002121 as libc::c_int as u32_0,
     0x44004444 as libc::c_int as u32_0, 0x51005151 as libc::c_int as u32_0,
     0xc600c6c6 as libc::c_uint, 0x7d007d7d as libc::c_int as u32_0,
     0x39003939 as libc::c_int as u32_0, 0x83008383 as libc::c_uint,
     0xdc00dcdc as libc::c_uint, 0xaa00aaaa as libc::c_uint,
     0x7c007c7c as libc::c_int as u32_0, 0x77007777 as libc::c_int as u32_0,
     0x56005656 as libc::c_int as u32_0, 0x5000505 as libc::c_int as u32_0,
     0x1b001b1b as libc::c_int as u32_0, 0xa400a4a4 as libc::c_uint,
     0x15001515 as libc::c_int as u32_0, 0x34003434 as libc::c_int as u32_0,
     0x1e001e1e as libc::c_int as u32_0, 0x1c001c1c as libc::c_int as u32_0,
     0xf800f8f8 as libc::c_uint, 0x52005252 as libc::c_int as u32_0,
     0x20002020 as libc::c_int as u32_0, 0x14001414 as libc::c_int as u32_0,
     0xe900e9e9 as libc::c_uint, 0xbd00bdbd as libc::c_uint,
     0xdd00dddd as libc::c_uint, 0xe400e4e4 as libc::c_uint,
     0xa100a1a1 as libc::c_uint, 0xe000e0e0 as libc::c_uint,
     0x8a008a8a as libc::c_uint, 0xf100f1f1 as libc::c_uint,
     0xd600d6d6 as libc::c_uint, 0x7a007a7a as libc::c_int as u32_0,
     0xbb00bbbb as libc::c_uint, 0xe300e3e3 as libc::c_uint,
     0x40004040 as libc::c_int as u32_0, 0x4f004f4f as libc::c_int as u32_0];
#[c2rust::src_loc = "377:18"]
static mut camellia_sp4404: [u32_0; 256] =
    [0x70700070 as libc::c_int as u32_0, 0x2c2c002c as libc::c_int as u32_0,
     0xb3b300b3 as libc::c_uint, 0xc0c000c0 as libc::c_uint,
     0xe4e400e4 as libc::c_uint, 0x57570057 as libc::c_int as u32_0,
     0xeaea00ea as libc::c_uint, 0xaeae00ae as libc::c_uint,
     0x23230023 as libc::c_int as u32_0, 0x6b6b006b as libc::c_int as u32_0,
     0x45450045 as libc::c_int as u32_0, 0xa5a500a5 as libc::c_uint,
     0xeded00ed as libc::c_uint, 0x4f4f004f as libc::c_int as u32_0,
     0x1d1d001d as libc::c_int as u32_0, 0x92920092 as libc::c_uint,
     0x86860086 as libc::c_uint, 0xafaf00af as libc::c_uint,
     0x7c7c007c as libc::c_int as u32_0, 0x1f1f001f as libc::c_int as u32_0,
     0x3e3e003e as libc::c_int as u32_0, 0xdcdc00dc as libc::c_uint,
     0x5e5e005e as libc::c_int as u32_0, 0xb0b000b as libc::c_int as u32_0,
     0xa6a600a6 as libc::c_uint, 0x39390039 as libc::c_int as u32_0,
     0xd5d500d5 as libc::c_uint, 0x5d5d005d as libc::c_int as u32_0,
     0xd9d900d9 as libc::c_uint, 0x5a5a005a as libc::c_int as u32_0,
     0x51510051 as libc::c_int as u32_0, 0x6c6c006c as libc::c_int as u32_0,
     0x8b8b008b as libc::c_uint, 0x9a9a009a as libc::c_uint,
     0xfbfb00fb as libc::c_uint, 0xb0b000b0 as libc::c_uint,
     0x74740074 as libc::c_int as u32_0, 0x2b2b002b as libc::c_int as u32_0,
     0xf0f000f0 as libc::c_uint, 0x84840084 as libc::c_uint,
     0xdfdf00df as libc::c_uint, 0xcbcb00cb as libc::c_uint,
     0x34340034 as libc::c_int as u32_0, 0x76760076 as libc::c_int as u32_0,
     0x6d6d006d as libc::c_int as u32_0, 0xa9a900a9 as libc::c_uint,
     0xd1d100d1 as libc::c_uint, 0x4040004 as libc::c_int as u32_0,
     0x14140014 as libc::c_int as u32_0, 0x3a3a003a as libc::c_int as u32_0,
     0xdede00de as libc::c_uint, 0x11110011 as libc::c_int as u32_0,
     0x32320032 as libc::c_int as u32_0, 0x9c9c009c as libc::c_uint,
     0x53530053 as libc::c_int as u32_0, 0xf2f200f2 as libc::c_uint,
     0xfefe00fe as libc::c_uint, 0xcfcf00cf as libc::c_uint,
     0xc3c300c3 as libc::c_uint, 0x7a7a007a as libc::c_int as u32_0,
     0x24240024 as libc::c_int as u32_0, 0xe8e800e8 as libc::c_uint,
     0x60600060 as libc::c_int as u32_0, 0x69690069 as libc::c_int as u32_0,
     0xaaaa00aa as libc::c_uint, 0xa0a000a0 as libc::c_uint,
     0xa1a100a1 as libc::c_uint, 0x62620062 as libc::c_int as u32_0,
     0x54540054 as libc::c_int as u32_0, 0x1e1e001e as libc::c_int as u32_0,
     0xe0e000e0 as libc::c_uint, 0x64640064 as libc::c_int as u32_0,
     0x10100010 as libc::c_int as u32_0, 0 as libc::c_int as u32_0,
     0xa3a300a3 as libc::c_uint, 0x75750075 as libc::c_int as u32_0,
     0x8a8a008a as libc::c_uint, 0xe6e600e6 as libc::c_uint,
     0x9090009 as libc::c_int as u32_0, 0xdddd00dd as libc::c_uint,
     0x87870087 as libc::c_uint, 0x83830083 as libc::c_uint,
     0xcdcd00cd as libc::c_uint, 0x90900090 as libc::c_uint,
     0x73730073 as libc::c_int as u32_0, 0xf6f600f6 as libc::c_uint,
     0x9d9d009d as libc::c_uint, 0xbfbf00bf as libc::c_uint,
     0x52520052 as libc::c_int as u32_0, 0xd8d800d8 as libc::c_uint,
     0xc8c800c8 as libc::c_uint, 0xc6c600c6 as libc::c_uint,
     0x81810081 as libc::c_uint, 0x6f6f006f as libc::c_int as u32_0,
     0x13130013 as libc::c_int as u32_0, 0x63630063 as libc::c_int as u32_0,
     0xe9e900e9 as libc::c_uint, 0xa7a700a7 as libc::c_uint,
     0x9f9f009f as libc::c_uint, 0xbcbc00bc as libc::c_uint,
     0x29290029 as libc::c_int as u32_0, 0xf9f900f9 as libc::c_uint,
     0x2f2f002f as libc::c_int as u32_0, 0xb4b400b4 as libc::c_uint,
     0x78780078 as libc::c_int as u32_0, 0x6060006 as libc::c_int as u32_0,
     0xe7e700e7 as libc::c_uint, 0x71710071 as libc::c_int as u32_0,
     0xd4d400d4 as libc::c_uint, 0xabab00ab as libc::c_uint,
     0x88880088 as libc::c_uint, 0x8d8d008d as libc::c_uint,
     0x72720072 as libc::c_int as u32_0, 0xb9b900b9 as libc::c_uint,
     0xf8f800f8 as libc::c_uint, 0xacac00ac as libc::c_uint,
     0x36360036 as libc::c_int as u32_0, 0x2a2a002a as libc::c_int as u32_0,
     0x3c3c003c as libc::c_int as u32_0, 0xf1f100f1 as libc::c_uint,
     0x40400040 as libc::c_int as u32_0, 0xd3d300d3 as libc::c_uint,
     0xbbbb00bb as libc::c_uint, 0x43430043 as libc::c_int as u32_0,
     0x15150015 as libc::c_int as u32_0, 0xadad00ad as libc::c_uint,
     0x77770077 as libc::c_int as u32_0, 0x80800080 as libc::c_uint,
     0x82820082 as libc::c_uint, 0xecec00ec as libc::c_uint,
     0x27270027 as libc::c_int as u32_0, 0xe5e500e5 as libc::c_uint,
     0x85850085 as libc::c_uint, 0x35350035 as libc::c_int as u32_0,
     0xc0c000c as libc::c_int as u32_0, 0x41410041 as libc::c_int as u32_0,
     0xefef00ef as libc::c_uint, 0x93930093 as libc::c_uint,
     0x19190019 as libc::c_int as u32_0, 0x21210021 as libc::c_int as u32_0,
     0xe0e000e as libc::c_int as u32_0, 0x4e4e004e as libc::c_int as u32_0,
     0x65650065 as libc::c_int as u32_0, 0xbdbd00bd as libc::c_uint,
     0xb8b800b8 as libc::c_uint, 0x8f8f008f as libc::c_uint,
     0xebeb00eb as libc::c_uint, 0xcece00ce as libc::c_uint,
     0x30300030 as libc::c_int as u32_0, 0x5f5f005f as libc::c_int as u32_0,
     0xc5c500c5 as libc::c_uint, 0x1a1a001a as libc::c_int as u32_0,
     0xe1e100e1 as libc::c_uint, 0xcaca00ca as libc::c_uint,
     0x47470047 as libc::c_int as u32_0, 0x3d3d003d as libc::c_int as u32_0,
     0x1010001 as libc::c_int as u32_0, 0xd6d600d6 as libc::c_uint,
     0x56560056 as libc::c_int as u32_0, 0x4d4d004d as libc::c_int as u32_0,
     0xd0d000d as libc::c_int as u32_0, 0x66660066 as libc::c_int as u32_0,
     0xcccc00cc as libc::c_uint, 0x2d2d002d as libc::c_int as u32_0,
     0x12120012 as libc::c_int as u32_0, 0x20200020 as libc::c_int as u32_0,
     0xb1b100b1 as libc::c_uint, 0x99990099 as libc::c_uint,
     0x4c4c004c as libc::c_int as u32_0, 0xc2c200c2 as libc::c_uint,
     0x7e7e007e as libc::c_int as u32_0, 0x5050005 as libc::c_int as u32_0,
     0xb7b700b7 as libc::c_uint, 0x31310031 as libc::c_int as u32_0,
     0x17170017 as libc::c_int as u32_0, 0xd7d700d7 as libc::c_uint,
     0x58580058 as libc::c_int as u32_0, 0x61610061 as libc::c_int as u32_0,
     0x1b1b001b as libc::c_int as u32_0, 0x1c1c001c as libc::c_int as u32_0,
     0xf0f000f as libc::c_int as u32_0, 0x16160016 as libc::c_int as u32_0,
     0x18180018 as libc::c_int as u32_0, 0x22220022 as libc::c_int as u32_0,
     0x44440044 as libc::c_int as u32_0, 0xb2b200b2 as libc::c_uint,
     0xb5b500b5 as libc::c_uint, 0x91910091 as libc::c_uint,
     0x8080008 as libc::c_int as u32_0, 0xa8a800a8 as libc::c_uint,
     0xfcfc00fc as libc::c_uint, 0x50500050 as libc::c_int as u32_0,
     0xd0d000d0 as libc::c_uint, 0x7d7d007d as libc::c_int as u32_0,
     0x89890089 as libc::c_uint, 0x97970097 as libc::c_uint,
     0x5b5b005b as libc::c_int as u32_0, 0x95950095 as libc::c_uint,
     0xffff00ff as libc::c_uint, 0xd2d200d2 as libc::c_uint,
     0xc4c400c4 as libc::c_uint, 0x48480048 as libc::c_int as u32_0,
     0xf7f700f7 as libc::c_uint, 0xdbdb00db as libc::c_uint,
     0x3030003 as libc::c_int as u32_0, 0xdada00da as libc::c_uint,
     0x3f3f003f as libc::c_int as u32_0, 0x94940094 as libc::c_uint,
     0x5c5c005c as libc::c_int as u32_0, 0x2020002 as libc::c_int as u32_0,
     0x4a4a004a as libc::c_int as u32_0, 0x33330033 as libc::c_int as u32_0,
     0x67670067 as libc::c_int as u32_0, 0xf3f300f3 as libc::c_uint,
     0x7f7f007f as libc::c_int as u32_0, 0xe2e200e2 as libc::c_uint,
     0x9b9b009b as libc::c_uint, 0x26260026 as libc::c_int as u32_0,
     0x37370037 as libc::c_int as u32_0, 0x3b3b003b as libc::c_int as u32_0,
     0x96960096 as libc::c_uint, 0x4b4b004b as libc::c_int as u32_0,
     0xbebe00be as libc::c_uint, 0x2e2e002e as libc::c_int as u32_0,
     0x79790079 as libc::c_int as u32_0, 0x8c8c008c as libc::c_uint,
     0x6e6e006e as libc::c_int as u32_0, 0x8e8e008e as libc::c_uint,
     0xf5f500f5 as libc::c_uint, 0xb6b600b6 as libc::c_uint,
     0xfdfd00fd as libc::c_uint, 0x59590059 as libc::c_int as u32_0,
     0x98980098 as libc::c_uint, 0x6a6a006a as libc::c_int as u32_0,
     0x46460046 as libc::c_int as u32_0, 0xbaba00ba as libc::c_uint,
     0x25250025 as libc::c_int as u32_0, 0x42420042 as libc::c_int as u32_0,
     0xa2a200a2 as libc::c_uint, 0xfafa00fa as libc::c_uint,
     0x7070007 as libc::c_int as u32_0, 0x55550055 as libc::c_int as u32_0,
     0xeeee00ee as libc::c_uint, 0xa0a000a as libc::c_int as u32_0,
     0x49490049 as libc::c_int as u32_0, 0x68680068 as libc::c_int as u32_0,
     0x38380038 as libc::c_int as u32_0, 0xa4a400a4 as libc::c_uint,
     0x28280028 as libc::c_int as u32_0, 0x7b7b007b as libc::c_int as u32_0,
     0xc9c900c9 as libc::c_uint, 0xc1c100c1 as libc::c_uint,
     0xe3e300e3 as libc::c_uint, 0xf4f400f4 as libc::c_uint,
     0xc7c700c7 as libc::c_uint, 0x9e9e009e as libc::c_uint];
/* *
 * Stuff related to the Camellia key schedule
 */
#[no_mangle]
#[c2rust::src_loc = "451:1"]
pub unsafe extern "C" fn k5_camellia_setup128(mut key: *const libc::c_uchar,
                                              mut subkey: *mut u32_0) {
    let mut kll: u32_0 = 0;
    let mut klr: u32_0 = 0;
    let mut krl: u32_0 = 0;
    let mut krr: u32_0 = 0;
    let mut il: u32_0 = 0;
    let mut ir: u32_0 = 0;
    let mut t0: u32_0 = 0;
    let mut t1: u32_0 = 0;
    let mut w0: u32_0 = 0;
    let mut w1: u32_0 = 0;
    let mut kw4l: u32_0 = 0;
    let mut kw4r: u32_0 = 0;
    let mut dw: u32_0 = 0;
    let mut tl: u32_0 = 0;
    let mut tr: u32_0 = 0;
    let mut subL: [u32_0; 26] = [0; 26];
    let mut subR: [u32_0; 26] = [0; 26];
    /* *
     *  k == kll || klr || krl || krr (|| is concatination)
     */
    kll =
        (*key.offset(0 as libc::c_int as isize) as u32_0) << 24 as libc::c_int
            ^
            (*key.offset(1 as libc::c_int as isize) as u32_0) <<
                16 as libc::c_int ^
            (*key.offset(2 as libc::c_int as isize) as u32_0) <<
                8 as libc::c_int ^
            *key.offset(3 as libc::c_int as isize) as u32_0;
    klr =
        (*key.offset(4 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(4 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(4 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(4 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krl =
        (*key.offset(8 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(8 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(8 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(8 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krr =
        (*key.offset(12 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(12 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(12 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(12 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    /* *
     * generate KL dependent subkeys
     */
    subL[0 as libc::c_int as usize] = kll;
    subR[0 as libc::c_int as usize] = klr;
    subL[1 as libc::c_int as usize] = krl;
    subR[1 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[4 as libc::c_int as usize] = kll;
    subR[4 as libc::c_int as usize] = klr;
    subL[5 as libc::c_int as usize] = krl;
    subR[5 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             30 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    klr =
        (klr <<
             30 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krl =
        (krl <<
             30 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krr =
        (krr <<
             30 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    subL[10 as libc::c_int as usize] = kll;
    subR[10 as libc::c_int as usize] = klr;
    subL[11 as libc::c_int as usize] = krl;
    subR[11 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[13 as libc::c_int as usize] = krl;
    subR[13 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             17 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    klr =
        (klr <<
             17 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krl =
        (krl <<
             17 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krr =
        (krr <<
             17 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    subL[16 as libc::c_int as usize] = kll;
    subR[16 as libc::c_int as usize] = klr;
    subL[17 as libc::c_int as usize] = krl;
    subR[17 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             17 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    klr =
        (klr <<
             17 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krl =
        (krl <<
             17 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krr =
        (krr <<
             17 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    subL[18 as libc::c_int as usize] = kll;
    subR[18 as libc::c_int as usize] = klr;
    subL[19 as libc::c_int as usize] = krl;
    subR[19 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             17 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    klr =
        (klr <<
             17 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krl =
        (krl <<
             17 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krr =
        (krr <<
             17 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    subL[22 as libc::c_int as usize] = kll;
    subR[22 as libc::c_int as usize] = klr;
    subL[23 as libc::c_int as usize] = krl;
    subR[23 as libc::c_int as usize] = krr;
    /* generate KA */
    kll = subL[0 as libc::c_int as usize];
    klr = subR[0 as libc::c_int as usize];
    krl = subL[1 as libc::c_int as usize];
    krr = subR[1 as libc::c_int as usize];
    il = (kll as libc::c_long ^ 0xa09e667f as libc::c_long) as u32_0;
    ir = (klr as libc::c_long ^ 0x3bcc908b as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    w0 =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w1 =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w0 ^= w1;
    w1 = (w1 >> 8 as libc::c_int).wrapping_add(w1 << 24 as libc::c_int);
    w1 ^= w0;
    krl ^= w0;
    krr ^= w1;
    il = (krl as libc::c_long ^ 0xb67ae858 as libc::c_long) as u32_0;
    ir = (krr as libc::c_long ^ 0x4caa73b2 as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    kll =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    klr =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    kll ^= klr;
    klr = (klr >> 8 as libc::c_int).wrapping_add(klr << 24 as libc::c_int);
    klr ^= kll;
    il = (kll as libc::c_long ^ 0xc6ef372f as libc::c_long) as u32_0;
    ir = (klr as libc::c_long ^ 0xe94f82be as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    krl =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    krr =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    krl ^= krr;
    krr = (krr >> 8 as libc::c_int).wrapping_add(krr << 24 as libc::c_int);
    krr ^= krl;
    krl ^= w0;
    krr ^= w1;
    il = (krl as libc::c_long ^ 0x54ff53a5 as libc::c_long) as u32_0;
    ir = (krr as libc::c_long ^ 0xf1d36f1c as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    w0 =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w1 =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w0 ^= w1;
    w1 = (w1 >> 8 as libc::c_int).wrapping_add(w1 << 24 as libc::c_int);
    w1 ^= w0;
    kll ^= w0;
    klr ^= w1;
    /* generate KA dependent subkeys */
    subL[2 as libc::c_int as usize] = kll;
    subR[2 as libc::c_int as usize] = klr;
    subL[3 as libc::c_int as usize] = krl;
    subR[3 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[6 as libc::c_int as usize] = kll;
    subR[6 as libc::c_int as usize] = klr;
    subL[7 as libc::c_int as usize] = krl;
    subR[7 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[8 as libc::c_int as usize] = kll;
    subR[8 as libc::c_int as usize] = klr;
    subL[9 as libc::c_int as usize] = krl;
    subR[9 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[12 as libc::c_int as usize] = kll;
    subR[12 as libc::c_int as usize] = klr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[14 as libc::c_int as usize] = kll;
    subR[14 as libc::c_int as usize] = klr;
    subL[15 as libc::c_int as usize] = krl;
    subR[15 as libc::c_int as usize] = krr;
    w0 = kll;
    w1 = klr;
    kll =
        (klr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krl >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    klr =
        (krl <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krr >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krl =
        (krr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krr =
        (w0 <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    subL[20 as libc::c_int as usize] = kll;
    subR[20 as libc::c_int as usize] = klr;
    subL[21 as libc::c_int as usize] = krl;
    subR[21 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             17 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    klr =
        (klr <<
             17 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krl =
        (krl <<
             17 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krr =
        (krr <<
             17 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    subL[24 as libc::c_int as usize] = kll;
    subR[24 as libc::c_int as usize] = klr;
    subL[25 as libc::c_int as usize] = krl;
    subR[25 as libc::c_int as usize] = krr;
    /* absorb kw2 to other subkeys */
    subL[3 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[3 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[5 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[5 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[7 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[7 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[1 as libc::c_int as usize] ^=
        subR[1 as libc::c_int as usize] & !subR[9 as libc::c_int as usize];
    dw = subL[1 as libc::c_int as usize] & subL[9 as libc::c_int as usize];
    subR[1 as libc::c_int as usize] ^=
        (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[11 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[11 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[13 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[13 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[15 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[15 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[1 as libc::c_int as usize] ^=
        subR[1 as libc::c_int as usize] & !subR[17 as libc::c_int as usize];
    dw = subL[1 as libc::c_int as usize] & subL[17 as libc::c_int as usize];
    subR[1 as libc::c_int as usize] ^=
        (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[19 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[19 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[21 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[21 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[23 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[23 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[24 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[24 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    /* absorb kw4 to other subkeys */
    kw4l = subL[25 as libc::c_int as usize];
    kw4r = subR[25 as libc::c_int as usize];
    subL[22 as libc::c_int as usize] ^= kw4l;
    subR[22 as libc::c_int as usize] ^= kw4r;
    subL[20 as libc::c_int as usize] ^= kw4l;
    subR[20 as libc::c_int as usize] ^= kw4r;
    subL[18 as libc::c_int as usize] ^= kw4l;
    subR[18 as libc::c_int as usize] ^= kw4r;
    kw4l ^= kw4r & !subR[16 as libc::c_int as usize];
    dw = kw4l & subL[16 as libc::c_int as usize];
    kw4r ^= (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[14 as libc::c_int as usize] ^= kw4l;
    subR[14 as libc::c_int as usize] ^= kw4r;
    subL[12 as libc::c_int as usize] ^= kw4l;
    subR[12 as libc::c_int as usize] ^= kw4r;
    subL[10 as libc::c_int as usize] ^= kw4l;
    subR[10 as libc::c_int as usize] ^= kw4r;
    kw4l ^= kw4r & !subR[8 as libc::c_int as usize];
    dw = kw4l & subL[8 as libc::c_int as usize];
    kw4r ^= (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[6 as libc::c_int as usize] ^= kw4l;
    subR[6 as libc::c_int as usize] ^= kw4r;
    subL[4 as libc::c_int as usize] ^= kw4l;
    subR[4 as libc::c_int as usize] ^= kw4r;
    subL[2 as libc::c_int as usize] ^= kw4l;
    subR[2 as libc::c_int as usize] ^= kw4r;
    subL[0 as libc::c_int as usize] ^= kw4l;
    subR[0 as libc::c_int as usize] ^= kw4r;
    /* key XOR is end of F-function */
    *subkey.offset((0 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[0 as libc::c_int as usize] ^ subL[2 as libc::c_int as usize];
    *subkey.offset((0 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[0 as libc::c_int as usize] ^ subR[2 as libc::c_int as usize];
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[3 as libc::c_int as usize];
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[3 as libc::c_int as usize];
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[2 as libc::c_int as usize] ^ subL[4 as libc::c_int as usize];
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[2 as libc::c_int as usize] ^ subR[4 as libc::c_int as usize];
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[3 as libc::c_int as usize] ^ subL[5 as libc::c_int as usize];
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[3 as libc::c_int as usize] ^ subR[5 as libc::c_int as usize];
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[4 as libc::c_int as usize] ^ subL[6 as libc::c_int as usize];
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[4 as libc::c_int as usize] ^ subR[6 as libc::c_int as usize];
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[5 as libc::c_int as usize] ^ subL[7 as libc::c_int as usize];
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[5 as libc::c_int as usize] ^ subR[7 as libc::c_int as usize];
    tl =
        subL[10 as libc::c_int as usize] ^
            subR[10 as libc::c_int as usize] &
                !subR[8 as libc::c_int as usize];
    dw = tl & subL[8 as libc::c_int as usize];
    tr =
        subR[10 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[6 as libc::c_int as usize] ^ tl;
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[6 as libc::c_int as usize] ^ tr;
    *subkey.offset((8 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[8 as libc::c_int as usize];
    *subkey.offset((8 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[8 as libc::c_int as usize];
    *subkey.offset((9 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[9 as libc::c_int as usize];
    *subkey.offset((9 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[9 as libc::c_int as usize];
    tl =
        subL[7 as libc::c_int as usize] ^
            subR[7 as libc::c_int as usize] &
                !subR[9 as libc::c_int as usize];
    dw = tl & subL[9 as libc::c_int as usize];
    tr =
        subR[7 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) =
        tl ^ subL[11 as libc::c_int as usize];
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = tr ^ subR[11 as libc::c_int as usize];
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[10 as libc::c_int as usize] ^ subL[12 as libc::c_int as usize];
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[10 as libc::c_int as usize] ^ subR[12 as libc::c_int as usize];
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[11 as libc::c_int as usize] ^ subL[13 as libc::c_int as usize];
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[11 as libc::c_int as usize] ^ subR[13 as libc::c_int as usize];
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[12 as libc::c_int as usize] ^ subL[14 as libc::c_int as usize];
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[12 as libc::c_int as usize] ^ subR[14 as libc::c_int as usize];
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[13 as libc::c_int as usize] ^ subL[15 as libc::c_int as usize];
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[13 as libc::c_int as usize] ^ subR[15 as libc::c_int as usize];
    tl =
        subL[18 as libc::c_int as usize] ^
            subR[18 as libc::c_int as usize] &
                !subR[16 as libc::c_int as usize];
    dw = tl & subL[16 as libc::c_int as usize];
    tr =
        subR[18 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[14 as libc::c_int as usize] ^ tl;
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[14 as libc::c_int as usize] ^ tr;
    *subkey.offset((16 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[16 as libc::c_int as usize];
    *subkey.offset((16 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[16 as libc::c_int as usize];
    *subkey.offset((17 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[17 as libc::c_int as usize];
    *subkey.offset((17 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[17 as libc::c_int as usize];
    tl =
        subL[15 as libc::c_int as usize] ^
            subR[15 as libc::c_int as usize] &
                !subR[17 as libc::c_int as usize];
    dw = tl & subL[17 as libc::c_int as usize];
    tr =
        subR[15 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) =
        tl ^ subL[19 as libc::c_int as usize];
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = tr ^ subR[19 as libc::c_int as usize];
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[18 as libc::c_int as usize] ^ subL[20 as libc::c_int as usize];
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[18 as libc::c_int as usize] ^ subR[20 as libc::c_int as usize];
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[19 as libc::c_int as usize] ^ subL[21 as libc::c_int as usize];
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[19 as libc::c_int as usize] ^ subR[21 as libc::c_int as usize];
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[20 as libc::c_int as usize] ^ subL[22 as libc::c_int as usize];
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[20 as libc::c_int as usize] ^ subR[22 as libc::c_int as usize];
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[21 as libc::c_int as usize] ^ subL[23 as libc::c_int as usize];
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[21 as libc::c_int as usize] ^ subR[23 as libc::c_int as usize];
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[22 as libc::c_int as usize];
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[22 as libc::c_int as usize];
    *subkey.offset((24 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[24 as libc::c_int as usize] ^ subL[23 as libc::c_int as usize];
    *subkey.offset((24 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[24 as libc::c_int as usize] ^ subR[23 as libc::c_int as usize];
    /* apply the inverse of the last half of P-function */
    dw =
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((2 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as
                               isize); /* left half of key */
    dw =
        (dw <<
             8 as
                 libc::c_int).wrapping_add(dw >>
                                               24 as
                                                   libc::c_int); /* right half of key */
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) ^
            dw; /* temporary variables */
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((3 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((4 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((5 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((6 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((7 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((10 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((11 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((12 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((13 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((14 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((15 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((18 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((19 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((20 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((21 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((22 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((23 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) = dw;
}
#[no_mangle]
#[c2rust::src_loc = "663:1"]
pub unsafe extern "C" fn k5_camellia_setup256(mut key: *const libc::c_uchar,
                                              mut subkey: *mut u32_0) {
    let mut kll: u32_0 = 0;
    let mut klr: u32_0 = 0;
    let mut krl: u32_0 = 0;
    let mut krr: u32_0 = 0;
    let mut krll: u32_0 = 0;
    let mut krlr: u32_0 = 0;
    let mut krrl: u32_0 = 0;
    let mut krrr: u32_0 = 0;
    let mut il: u32_0 = 0;
    let mut ir: u32_0 = 0;
    let mut t0: u32_0 = 0;
    let mut t1: u32_0 = 0;
    let mut w0: u32_0 = 0;
    let mut w1: u32_0 = 0;
    let mut kw4l: u32_0 = 0;
    let mut kw4r: u32_0 = 0;
    let mut dw: u32_0 = 0;
    let mut tl: u32_0 = 0;
    let mut tr: u32_0 = 0;
    let mut subL: [u32_0; 34] = [0; 34];
    let mut subR: [u32_0; 34] = [0; 34];
    /* *
     *  key = (kll || klr || krl || krr || krll || krlr || krrl || krrr)
     *  (|| is concatination)
     */
    kll =
        (*key.offset(0 as libc::c_int as isize) as u32_0) << 24 as libc::c_int
            ^
            (*key.offset(1 as libc::c_int as isize) as u32_0) <<
                16 as libc::c_int ^
            (*key.offset(2 as libc::c_int as isize) as u32_0) <<
                8 as libc::c_int ^
            *key.offset(3 as libc::c_int as isize) as u32_0;
    klr =
        (*key.offset(4 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(4 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(4 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(4 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krl =
        (*key.offset(8 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(8 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(8 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(8 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krr =
        (*key.offset(12 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(12 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(12 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(12 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krll =
        (*key.offset(16 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(16 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(16 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(16 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krlr =
        (*key.offset(20 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(20 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(20 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(20 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krrl =
        (*key.offset(24 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(24 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(24 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(24 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    krrr =
        (*key.offset(28 as libc::c_int as
                         isize).offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*key.offset(28 as libc::c_int as
                             isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*key.offset(28 as libc::c_int as
                             isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *key.offset(28 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as u32_0;
    /* generate KL dependent subkeys */
    subL[0 as libc::c_int as usize] = kll;
    subR[0 as libc::c_int as usize] = klr;
    subL[1 as libc::c_int as usize] = krl;
    subR[1 as libc::c_int as usize] = krr;
    w0 = kll;
    w1 = klr;
    kll =
        (klr <<
             45 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krl >>
                                                   64 as libc::c_int -
                                                       45 as libc::c_int);
    klr =
        (krl <<
             45 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krr >>
                                                   64 as libc::c_int -
                                                       45 as libc::c_int);
    krl =
        (krr <<
             45 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       45 as libc::c_int);
    krr =
        (w0 <<
             45 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       45 as libc::c_int);
    subL[12 as libc::c_int as usize] = kll;
    subR[12 as libc::c_int as usize] = klr;
    subL[13 as libc::c_int as usize] = krl;
    subR[13 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[16 as libc::c_int as usize] = kll;
    subR[16 as libc::c_int as usize] = klr;
    subL[17 as libc::c_int as usize] = krl;
    subR[17 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             17 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    klr =
        (klr <<
             17 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krl =
        (krl <<
             17 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    krr =
        (krr <<
             17 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   17 as libc::c_int);
    subL[22 as libc::c_int as usize] = kll;
    subR[22 as libc::c_int as usize] = klr;
    subL[23 as libc::c_int as usize] = krl;
    subR[23 as libc::c_int as usize] = krr;
    w0 = kll;
    w1 = klr;
    kll =
        (klr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krl >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    klr =
        (krl <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krr >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krl =
        (krr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krr =
        (w0 <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    subL[30 as libc::c_int as usize] = kll;
    subR[30 as libc::c_int as usize] = klr;
    subL[31 as libc::c_int as usize] = krl;
    subR[31 as libc::c_int as usize] = krr;
    /* generate KR dependent subkeys */
    w0 = krll;
    krll =
        (krll <<
             15 as
                 libc::c_int).wrapping_add(krlr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krlr =
        (krlr <<
             15 as
                 libc::c_int).wrapping_add(krrl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krrl =
        (krrl <<
             15 as
                 libc::c_int).wrapping_add(krrr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krrr =
        (krrr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[4 as libc::c_int as usize] = krll;
    subR[4 as libc::c_int as usize] = krlr;
    subL[5 as libc::c_int as usize] = krrl;
    subR[5 as libc::c_int as usize] = krrr;
    w0 = krll;
    krll =
        (krll <<
             15 as
                 libc::c_int).wrapping_add(krlr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krlr =
        (krlr <<
             15 as
                 libc::c_int).wrapping_add(krrl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krrl =
        (krrl <<
             15 as
                 libc::c_int).wrapping_add(krrr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krrr =
        (krrr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[8 as libc::c_int as usize] = krll;
    subR[8 as libc::c_int as usize] = krlr;
    subL[9 as libc::c_int as usize] = krrl;
    subR[9 as libc::c_int as usize] = krrr;
    w0 = krll;
    krll =
        (krll <<
             30 as
                 libc::c_int).wrapping_add(krlr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krlr =
        (krlr <<
             30 as
                 libc::c_int).wrapping_add(krrl >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krrl =
        (krrl <<
             30 as
                 libc::c_int).wrapping_add(krrr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krrr =
        (krrr <<
             30 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    subL[18 as libc::c_int as usize] = krll;
    subR[18 as libc::c_int as usize] = krlr;
    subL[19 as libc::c_int as usize] = krrl;
    subR[19 as libc::c_int as usize] = krrr;
    w0 = krll;
    w1 = krlr;
    krll =
        (krlr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krrl >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krlr =
        (krrl <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krrr >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krrl =
        (krrr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krrr =
        (w0 <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    subL[26 as libc::c_int as usize] = krll;
    subR[26 as libc::c_int as usize] = krlr;
    subL[27 as libc::c_int as usize] = krrl;
    subR[27 as libc::c_int as usize] = krrr;
    w0 = krll;
    w1 = krlr;
    krll =
        (krlr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krrl >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krlr =
        (krrl <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krrr >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krrl =
        (krrr <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    krrr =
        (w0 <<
             34 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       34 as libc::c_int);
    /* generate KA */
    kll = subL[0 as libc::c_int as usize] ^ krll;
    klr = subR[0 as libc::c_int as usize] ^ krlr;
    krl = subL[1 as libc::c_int as usize] ^ krrl;
    krr = subR[1 as libc::c_int as usize] ^ krrr;
    il = (kll as libc::c_long ^ 0xa09e667f as libc::c_long) as u32_0;
    ir = (klr as libc::c_long ^ 0x3bcc908b as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    w0 =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w1 =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w0 ^= w1;
    w1 = (w1 >> 8 as libc::c_int).wrapping_add(w1 << 24 as libc::c_int);
    w1 ^= w0;
    krl ^= w0;
    krr ^= w1;
    il = (krl as libc::c_long ^ 0xb67ae858 as libc::c_long) as u32_0;
    ir = (krr as libc::c_long ^ 0x4caa73b2 as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    kll =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    klr =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    kll ^= klr;
    klr = (klr >> 8 as libc::c_int).wrapping_add(klr << 24 as libc::c_int);
    klr ^= kll;
    kll ^= krll;
    klr ^= krlr;
    il = (kll as libc::c_long ^ 0xc6ef372f as libc::c_long) as u32_0;
    ir = (klr as libc::c_long ^ 0xe94f82be as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    krl =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    krr =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    krl ^= krr;
    krr = (krr >> 8 as libc::c_int).wrapping_add(krr << 24 as libc::c_int);
    krr ^= krl;
    krl ^= w0 ^ krrl;
    krr ^= w1 ^ krrr;
    il = (krl as libc::c_long ^ 0x54ff53a5 as libc::c_long) as u32_0;
    ir = (krr as libc::c_long ^ 0xf1d36f1c as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    w0 =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w1 =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w0 ^= w1;
    w1 = (w1 >> 8 as libc::c_int).wrapping_add(w1 << 24 as libc::c_int);
    w1 ^= w0;
    kll ^= w0;
    klr ^= w1;
    /* generate KB */
    krll ^= kll;
    krlr ^= klr;
    krrl ^= krl;
    krrr ^= krr;
    il = (krll as libc::c_long ^ 0x10e527fa as libc::c_long) as u32_0;
    ir = (krlr as libc::c_long ^ 0xde682d1d as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    w0 =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w1 =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w0 ^= w1;
    w1 = (w1 >> 8 as libc::c_int).wrapping_add(w1 << 24 as libc::c_int);
    w1 ^= w0;
    krrl ^= w0;
    krrr ^= w1;
    il = (krrl as libc::c_long ^ 0xb05688c2 as libc::c_long) as u32_0;
    ir = (krrr as libc::c_long ^ 0xb3e6c1fd as libc::c_long) as u32_0;
    t0 = il >> 16 as libc::c_int;
    t1 = ir >> 16 as libc::c_int;
    w0 =
        camellia_sp1110[(ir & 0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t1 >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(t1 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(ir >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w1 =
        camellia_sp1110[(t0 >> 8 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(t0 & 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(il >> 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(il & 0xff as libc::c_int as libc::c_uint) as
                                usize];
    w0 ^= w1;
    w1 = (w1 >> 8 as libc::c_int).wrapping_add(w1 << 24 as libc::c_int);
    w1 ^= w0;
    krll ^= w0;
    krlr ^= w1;
    /* generate KA dependent subkeys */
    w0 = kll;
    kll =
        (kll <<
             15 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    klr =
        (klr <<
             15 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krl =
        (krl <<
             15 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    krr =
        (krr <<
             15 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   15 as libc::c_int);
    subL[6 as libc::c_int as usize] = kll;
    subR[6 as libc::c_int as usize] = klr;
    subL[7 as libc::c_int as usize] = krl;
    subR[7 as libc::c_int as usize] = krr;
    w0 = kll;
    kll =
        (kll <<
             30 as
                 libc::c_int).wrapping_add(klr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    klr =
        (klr <<
             30 as
                 libc::c_int).wrapping_add(krl >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krl =
        (krl <<
             30 as
                 libc::c_int).wrapping_add(krr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krr =
        (krr <<
             30 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    subL[14 as libc::c_int as usize] = kll;
    subR[14 as libc::c_int as usize] = klr;
    subL[15 as libc::c_int as usize] = krl;
    subR[15 as libc::c_int as usize] = krr;
    subL[24 as libc::c_int as usize] = klr;
    subR[24 as libc::c_int as usize] = krl;
    subL[25 as libc::c_int as usize] = krr;
    subR[25 as libc::c_int as usize] = kll;
    w0 = kll;
    w1 = klr;
    kll =
        (klr <<
             49 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krl >>
                                                   64 as libc::c_int -
                                                       49 as libc::c_int);
    klr =
        (krl <<
             49 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krr >>
                                                   64 as libc::c_int -
                                                       49 as libc::c_int);
    krl =
        (krr <<
             49 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       49 as libc::c_int);
    krr =
        (w0 <<
             49 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       49 as libc::c_int);
    subL[28 as libc::c_int as usize] = kll;
    subR[28 as libc::c_int as usize] = klr;
    subL[29 as libc::c_int as usize] = krl;
    subR[29 as libc::c_int as usize] = krr;
    /* generate KB dependent subkeys */
    subL[2 as libc::c_int as usize] = krll;
    subR[2 as libc::c_int as usize] = krlr;
    subL[3 as libc::c_int as usize] = krrl;
    subR[3 as libc::c_int as usize] = krrr;
    w0 = krll;
    krll =
        (krll <<
             30 as
                 libc::c_int).wrapping_add(krlr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krlr =
        (krlr <<
             30 as
                 libc::c_int).wrapping_add(krrl >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krrl =
        (krrl <<
             30 as
                 libc::c_int).wrapping_add(krrr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krrr =
        (krrr <<
             30 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    subL[10 as libc::c_int as usize] = krll;
    subR[10 as libc::c_int as usize] = krlr;
    subL[11 as libc::c_int as usize] = krrl;
    subR[11 as libc::c_int as usize] = krrr;
    w0 = krll;
    krll =
        (krll <<
             30 as
                 libc::c_int).wrapping_add(krlr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krlr =
        (krlr <<
             30 as
                 libc::c_int).wrapping_add(krrl >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krrl =
        (krrl <<
             30 as
                 libc::c_int).wrapping_add(krrr >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    krrr =
        (krrr <<
             30 as
                 libc::c_int).wrapping_add(w0 >>
                                               32 as libc::c_int -
                                                   30 as libc::c_int);
    subL[20 as libc::c_int as usize] = krll;
    subR[20 as libc::c_int as usize] = krlr;
    subL[21 as libc::c_int as usize] = krrl;
    subR[21 as libc::c_int as usize] = krrr;
    w0 = krll;
    w1 = krlr;
    krll =
        (krlr <<
             51 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krrl >>
                                                   64 as libc::c_int -
                                                       51 as libc::c_int);
    krlr =
        (krrl <<
             51 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(krrr >>
                                                   64 as libc::c_int -
                                                       51 as libc::c_int);
    krrl =
        (krrr <<
             51 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w0 >>
                                                   64 as libc::c_int -
                                                       51 as libc::c_int);
    krrr =
        (w0 <<
             51 as libc::c_int -
                 32 as
                     libc::c_int).wrapping_add(w1 >>
                                                   64 as libc::c_int -
                                                       51 as libc::c_int);
    subL[32 as libc::c_int as usize] = krll;
    subR[32 as libc::c_int as usize] = krlr;
    subL[33 as libc::c_int as usize] = krrl;
    subR[33 as libc::c_int as usize] = krrr;
    /* absorb kw2 to other subkeys */
    subL[3 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[3 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[5 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[5 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[7 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[7 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[1 as libc::c_int as usize] ^=
        subR[1 as libc::c_int as usize] & !subR[9 as libc::c_int as usize];
    dw = subL[1 as libc::c_int as usize] & subL[9 as libc::c_int as usize];
    subR[1 as libc::c_int as usize] ^=
        (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[11 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[11 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[13 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[13 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[15 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[15 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[1 as libc::c_int as usize] ^=
        subR[1 as libc::c_int as usize] & !subR[17 as libc::c_int as usize];
    dw = subL[1 as libc::c_int as usize] & subL[17 as libc::c_int as usize];
    subR[1 as libc::c_int as usize] ^=
        (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[19 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[19 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[21 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[21 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[23 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[23 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[1 as libc::c_int as usize] ^=
        subR[1 as libc::c_int as usize] & !subR[25 as libc::c_int as usize];
    dw = subL[1 as libc::c_int as usize] & subL[25 as libc::c_int as usize];
    subR[1 as libc::c_int as usize] ^=
        (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[27 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[27 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[29 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[29 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[31 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[31 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    subL[32 as libc::c_int as usize] ^= subL[1 as libc::c_int as usize];
    subR[32 as libc::c_int as usize] ^= subR[1 as libc::c_int as usize];
    /* absorb kw4 to other subkeys */
    kw4l = subL[33 as libc::c_int as usize];
    kw4r = subR[33 as libc::c_int as usize];
    subL[30 as libc::c_int as usize] ^= kw4l;
    subR[30 as libc::c_int as usize] ^= kw4r;
    subL[28 as libc::c_int as usize] ^= kw4l;
    subR[28 as libc::c_int as usize] ^= kw4r;
    subL[26 as libc::c_int as usize] ^= kw4l;
    subR[26 as libc::c_int as usize] ^= kw4r;
    kw4l ^= kw4r & !subR[24 as libc::c_int as usize];
    dw = kw4l & subL[24 as libc::c_int as usize];
    kw4r ^= (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[22 as libc::c_int as usize] ^= kw4l;
    subR[22 as libc::c_int as usize] ^= kw4r;
    subL[20 as libc::c_int as usize] ^= kw4l;
    subR[20 as libc::c_int as usize] ^= kw4r;
    subL[18 as libc::c_int as usize] ^= kw4l;
    subR[18 as libc::c_int as usize] ^= kw4r;
    kw4l ^= kw4r & !subR[16 as libc::c_int as usize];
    dw = kw4l & subL[16 as libc::c_int as usize];
    kw4r ^= (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[14 as libc::c_int as usize] ^= kw4l;
    subR[14 as libc::c_int as usize] ^= kw4r;
    subL[12 as libc::c_int as usize] ^= kw4l;
    subR[12 as libc::c_int as usize] ^= kw4r;
    subL[10 as libc::c_int as usize] ^= kw4l;
    subR[10 as libc::c_int as usize] ^= kw4r;
    kw4l ^= kw4r & !subR[8 as libc::c_int as usize];
    dw = kw4l & subL[8 as libc::c_int as usize];
    kw4r ^= (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    subL[6 as libc::c_int as usize] ^= kw4l;
    subR[6 as libc::c_int as usize] ^= kw4r;
    subL[4 as libc::c_int as usize] ^= kw4l;
    subR[4 as libc::c_int as usize] ^= kw4r;
    subL[2 as libc::c_int as usize] ^= kw4l;
    subR[2 as libc::c_int as usize] ^= kw4r;
    subL[0 as libc::c_int as usize] ^= kw4l;
    subR[0 as libc::c_int as usize] ^= kw4r;
    /* key XOR is end of F-function */
    *subkey.offset((0 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[0 as libc::c_int as usize] ^ subL[2 as libc::c_int as usize];
    *subkey.offset((0 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[0 as libc::c_int as usize] ^ subR[2 as libc::c_int as usize];
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[3 as libc::c_int as usize];
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[3 as libc::c_int as usize];
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[2 as libc::c_int as usize] ^ subL[4 as libc::c_int as usize];
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[2 as libc::c_int as usize] ^ subR[4 as libc::c_int as usize];
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[3 as libc::c_int as usize] ^ subL[5 as libc::c_int as usize];
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[3 as libc::c_int as usize] ^ subR[5 as libc::c_int as usize];
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[4 as libc::c_int as usize] ^ subL[6 as libc::c_int as usize];
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[4 as libc::c_int as usize] ^ subR[6 as libc::c_int as usize];
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[5 as libc::c_int as usize] ^ subL[7 as libc::c_int as usize];
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        subR[5 as libc::c_int as usize] ^ subR[7 as libc::c_int as usize];
    tl =
        subL[10 as libc::c_int as usize] ^
            subR[10 as libc::c_int as usize] &
                !subR[8 as libc::c_int as usize];
    dw = tl & subL[8 as libc::c_int as usize];
    tr =
        subR[10 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[6 as libc::c_int as usize] ^ tl;
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[6 as libc::c_int as usize] ^ tr;
    *subkey.offset((8 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[8 as libc::c_int as usize];
    *subkey.offset((8 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[8 as libc::c_int as usize];
    *subkey.offset((9 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[9 as libc::c_int as usize];
    *subkey.offset((9 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) = subR[9 as libc::c_int as usize];
    tl =
        subL[7 as libc::c_int as usize] ^
            subR[7 as libc::c_int as usize] &
                !subR[9 as libc::c_int as usize];
    dw = tl & subL[9 as libc::c_int as usize];
    tr =
        subR[7 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) =
        tl ^ subL[11 as libc::c_int as usize];
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = tr ^ subR[11 as libc::c_int as usize];
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[10 as libc::c_int as usize] ^ subL[12 as libc::c_int as usize];
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[10 as libc::c_int as usize] ^ subR[12 as libc::c_int as usize];
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[11 as libc::c_int as usize] ^ subL[13 as libc::c_int as usize];
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[11 as libc::c_int as usize] ^ subR[13 as libc::c_int as usize];
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[12 as libc::c_int as usize] ^ subL[14 as libc::c_int as usize];
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[12 as libc::c_int as usize] ^ subR[14 as libc::c_int as usize];
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[13 as libc::c_int as usize] ^ subL[15 as libc::c_int as usize];
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[13 as libc::c_int as usize] ^ subR[15 as libc::c_int as usize];
    tl =
        subL[18 as libc::c_int as usize] ^
            subR[18 as libc::c_int as usize] &
                !subR[16 as libc::c_int as usize];
    dw = tl & subL[16 as libc::c_int as usize];
    tr =
        subR[18 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[14 as libc::c_int as usize] ^ tl;
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[14 as libc::c_int as usize] ^ tr;
    *subkey.offset((16 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[16 as libc::c_int as usize];
    *subkey.offset((16 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[16 as libc::c_int as usize];
    *subkey.offset((17 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[17 as libc::c_int as usize];
    *subkey.offset((17 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[17 as libc::c_int as usize];
    tl =
        subL[15 as libc::c_int as usize] ^
            subR[15 as libc::c_int as usize] &
                !subR[17 as libc::c_int as usize];
    dw = tl & subL[17 as libc::c_int as usize];
    tr =
        subR[15 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) =
        tl ^ subL[19 as libc::c_int as usize];
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = tr ^ subR[19 as libc::c_int as usize];
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[18 as libc::c_int as usize] ^ subL[20 as libc::c_int as usize];
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[18 as libc::c_int as usize] ^ subR[20 as libc::c_int as usize];
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[19 as libc::c_int as usize] ^ subL[21 as libc::c_int as usize];
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[19 as libc::c_int as usize] ^ subR[21 as libc::c_int as usize];
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[20 as libc::c_int as usize] ^ subL[22 as libc::c_int as usize];
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[20 as libc::c_int as usize] ^ subR[22 as libc::c_int as usize];
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[21 as libc::c_int as usize] ^ subL[23 as libc::c_int as usize];
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[21 as libc::c_int as usize] ^ subR[23 as libc::c_int as usize];
    tl =
        subL[26 as libc::c_int as usize] ^
            subR[26 as libc::c_int as usize] &
                !subR[24 as libc::c_int as usize];
    dw = tl & subL[24 as libc::c_int as usize];
    tr =
        subR[26 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[22 as libc::c_int as usize] ^ tl;
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[22 as libc::c_int as usize] ^ tr;
    *subkey.offset((24 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[24 as libc::c_int as usize];
    *subkey.offset((24 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[24 as libc::c_int as usize];
    *subkey.offset((25 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[25 as libc::c_int as usize];
    *subkey.offset((25 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[25 as libc::c_int as usize];
    tl =
        subL[23 as libc::c_int as usize] ^
            subR[23 as libc::c_int as usize] &
                !subR[25 as libc::c_int as usize];
    dw = tl & subL[25 as libc::c_int as usize];
    tr =
        subR[23 as libc::c_int as usize] ^
            (dw << 1 as libc::c_int).wrapping_add(dw >> 31 as libc::c_int);
    *subkey.offset((26 as libc::c_int * 2 as libc::c_int) as isize) =
        tl ^ subL[27 as libc::c_int as usize];
    *subkey.offset((26 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = tr ^ subR[27 as libc::c_int as usize];
    *subkey.offset((27 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[26 as libc::c_int as usize] ^ subL[28 as libc::c_int as usize];
    *subkey.offset((27 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[26 as libc::c_int as usize] ^ subR[28 as libc::c_int as usize];
    *subkey.offset((28 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[27 as libc::c_int as usize] ^ subL[29 as libc::c_int as usize];
    *subkey.offset((28 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[27 as libc::c_int as usize] ^ subR[29 as libc::c_int as usize];
    *subkey.offset((29 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[28 as libc::c_int as usize] ^ subL[30 as libc::c_int as usize];
    *subkey.offset((29 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[28 as libc::c_int as usize] ^ subR[30 as libc::c_int as usize];
    *subkey.offset((30 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[29 as libc::c_int as usize] ^ subL[31 as libc::c_int as usize];
    *subkey.offset((30 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[29 as libc::c_int as usize] ^ subR[31 as libc::c_int as usize];
    *subkey.offset((31 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[30 as libc::c_int as usize];
    *subkey.offset((31 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) = subR[30 as libc::c_int as usize];
    *subkey.offset((32 as libc::c_int * 2 as libc::c_int) as isize) =
        subL[32 as libc::c_int as usize] ^ subL[31 as libc::c_int as usize];
    *subkey.offset((32 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        subR[32 as libc::c_int as usize] ^ subR[31 as libc::c_int as usize];
    /* apply the inverse of the last half of P-function */
    dw =
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((2 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((3 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((4 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((5 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((6 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((7 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as
                       isize) =
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((10 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((11 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((12 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((13 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((14 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((15 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((18 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((19 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((20 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((21 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((22 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((23 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((26 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((26 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((26 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((26 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((26 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((27 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((27 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((27 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((27 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((27 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((28 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((28 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((28 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((28 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((28 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((29 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((29 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((29 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((29 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((29 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((30 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((30 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((30 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((30 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((30 as libc::c_int * 2 as libc::c_int) as isize) = dw;
    dw =
        *subkey.offset((31 as libc::c_int * 2 as libc::c_int) as isize) ^
            *subkey.offset((31 as libc::c_int * 2 as libc::c_int +
                                1 as libc::c_int) as isize);
    dw = (dw << 8 as libc::c_int).wrapping_add(dw >> 24 as libc::c_int);
    *subkey.offset((31 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                       as isize) =
        *subkey.offset((31 as libc::c_int * 2 as libc::c_int) as isize) ^ dw;
    *subkey.offset((31 as libc::c_int * 2 as libc::c_int) as isize) = dw;
}
#[no_mangle]
#[c2rust::src_loc = "950:1"]
pub unsafe extern "C" fn k5_camellia_setup192(mut key: *const libc::c_uchar,
                                              mut subkey: *mut u32_0) {
    let mut kk: [libc::c_uchar; 32] = [0; 32];
    let mut krll: u32_0 = 0;
    let mut krlr: u32_0 = 0;
    let mut krrl: u32_0 = 0;
    let mut krrr: u32_0 = 0;
    memcpy(kk.as_mut_ptr() as *mut libc::c_void, key as *const libc::c_void,
           24 as libc::c_int as libc::c_ulong);
    memcpy(&mut krll as *mut u32_0 as *mut libc::c_uchar as *mut libc::c_void,
           key.offset(16 as libc::c_int as isize) as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    memcpy(&mut krlr as *mut u32_0 as *mut libc::c_uchar as *mut libc::c_void,
           key.offset(20 as libc::c_int as isize) as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    krrl = !krll;
    krrr = !krlr;
    memcpy(kk.as_mut_ptr().offset(24 as libc::c_int as isize) as
               *mut libc::c_void,
           &mut krrl as *mut u32_0 as *mut libc::c_uchar as
               *const libc::c_void, 4 as libc::c_int as libc::c_ulong);
    memcpy(kk.as_mut_ptr().offset(28 as libc::c_int as isize) as
               *mut libc::c_void,
           &mut krrr as *mut u32_0 as *mut libc::c_uchar as
               *const libc::c_void, 4 as libc::c_int as libc::c_ulong);
    k5_camellia_setup256(kk.as_mut_ptr(), subkey);
}
/* *
 * Stuff related to camellia encryption/decryption
 *
 * "io" must be 4byte aligned and big-endian data.
 */
#[no_mangle]
#[c2rust::src_loc = "973:1"]
pub unsafe extern "C" fn k5_camellia_encrypt128(mut subkey: *const u32_0,
                                                mut io: *mut u32_0) {
    let mut il: u32_0 = 0;
    let mut ir: u32_0 = 0;
    let mut t0: u32_0 = 0;
    let mut t1: u32_0 = 0;
    /* pre whitening but absorb kw2*/
    let ref mut fresh0 = *io.offset(0 as libc::c_int as isize);
    *fresh0 ^= *subkey.offset((0 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh1 = *io.offset(1 as libc::c_int as isize);
    *fresh1 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    /* main iteration */
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh2 = *io.offset(2 as libc::c_int as isize);
    *fresh2 ^= ir;
    let ref mut fresh3 = *io.offset(3 as libc::c_int as isize);
    *fresh3 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh4 = *io.offset(0 as libc::c_int as isize);
    *fresh4 ^= ir;
    let ref mut fresh5 = *io.offset(1 as libc::c_int as isize);
    *fresh5 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh6 = *io.offset(2 as libc::c_int as isize);
    *fresh6 ^= ir;
    let ref mut fresh7 = *io.offset(3 as libc::c_int as isize);
    *fresh7 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh8 = *io.offset(0 as libc::c_int as isize);
    *fresh8 ^= ir;
    let ref mut fresh9 = *io.offset(1 as libc::c_int as isize);
    *fresh9 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh10 = *io.offset(2 as libc::c_int as isize);
    *fresh10 ^= ir;
    let ref mut fresh11 = *io.offset(3 as libc::c_int as isize);
    *fresh11 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh12 = *io.offset(0 as libc::c_int as isize);
    *fresh12 ^= ir;
    let ref mut fresh13 = *io.offset(1 as libc::c_int as isize);
    *fresh13 ^= il;
    t0 = *subkey.offset((8 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh14 = *io.offset(1 as libc::c_int as isize);
    *fresh14 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((8 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh15 = *io.offset(0 as libc::c_int as isize);
    *fresh15 ^= t1;
    il =
        *subkey.offset((9 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh16 = *io.offset(2 as libc::c_int as isize);
    *fresh16 ^= il;
    ir = *subkey.offset((9 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh17 = *io.offset(3 as libc::c_int as isize);
    *fresh17 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh18 = *io.offset(2 as libc::c_int as isize);
    *fresh18 ^= ir;
    let ref mut fresh19 = *io.offset(3 as libc::c_int as isize);
    *fresh19 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh20 = *io.offset(0 as libc::c_int as isize);
    *fresh20 ^= ir;
    let ref mut fresh21 = *io.offset(1 as libc::c_int as isize);
    *fresh21 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh22 = *io.offset(2 as libc::c_int as isize);
    *fresh22 ^= ir;
    let ref mut fresh23 = *io.offset(3 as libc::c_int as isize);
    *fresh23 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh24 = *io.offset(0 as libc::c_int as isize);
    *fresh24 ^= ir;
    let ref mut fresh25 = *io.offset(1 as libc::c_int as isize);
    *fresh25 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh26 = *io.offset(2 as libc::c_int as isize);
    *fresh26 ^= ir;
    let ref mut fresh27 = *io.offset(3 as libc::c_int as isize);
    *fresh27 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh28 = *io.offset(0 as libc::c_int as isize);
    *fresh28 ^= ir;
    let ref mut fresh29 = *io.offset(1 as libc::c_int as isize);
    *fresh29 ^= il;
    t0 = *subkey.offset((16 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh30 = *io.offset(1 as libc::c_int as isize);
    *fresh30 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((16 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh31 = *io.offset(0 as libc::c_int as isize);
    *fresh31 ^= t1;
    il =
        *subkey.offset((17 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh32 = *io.offset(2 as libc::c_int as isize);
    *fresh32 ^= il;
    ir = *subkey.offset((17 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh33 = *io.offset(3 as libc::c_int as isize);
    *fresh33 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh34 = *io.offset(2 as libc::c_int as isize);
    *fresh34 ^= ir;
    let ref mut fresh35 = *io.offset(3 as libc::c_int as isize);
    *fresh35 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh36 = *io.offset(0 as libc::c_int as isize);
    *fresh36 ^= ir;
    let ref mut fresh37 = *io.offset(1 as libc::c_int as isize);
    *fresh37 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh38 = *io.offset(2 as libc::c_int as isize);
    *fresh38 ^= ir;
    let ref mut fresh39 = *io.offset(3 as libc::c_int as isize);
    *fresh39 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh40 = *io.offset(0 as libc::c_int as isize);
    *fresh40 ^= ir;
    let ref mut fresh41 = *io.offset(1 as libc::c_int as isize);
    *fresh41 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh42 = *io.offset(2 as libc::c_int as isize);
    *fresh42 ^= ir;
    let ref mut fresh43 = *io.offset(3 as libc::c_int as isize);
    *fresh43 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh44 = *io.offset(0 as libc::c_int as isize);
    *fresh44 ^= ir;
    let ref mut fresh45 = *io.offset(1 as libc::c_int as isize);
    *fresh45 ^= il;
    /* post whitening but kw4 */
    let ref mut fresh46 =
        *io.offset(2 as libc::c_int as isize); /* temporary valiables */
    *fresh46 ^=
        *subkey.offset((24 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh47 = *io.offset(3 as libc::c_int as isize);
    *fresh47 ^=
        *subkey.offset((24 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t0 = *io.offset(0 as libc::c_int as isize);
    t1 = *io.offset(1 as libc::c_int as isize);
    *io.offset(0 as libc::c_int as isize) =
        *io.offset(2 as libc::c_int as isize);
    *io.offset(1 as libc::c_int as isize) =
        *io.offset(3 as libc::c_int as isize);
    *io.offset(2 as libc::c_int as isize) = t0;
    *io.offset(3 as libc::c_int as isize) = t1;
}
#[no_mangle]
#[c2rust::src_loc = "1064:1"]
pub unsafe extern "C" fn k5_camellia_decrypt128(mut subkey: *const u32_0,
                                                mut io: *mut u32_0) {
    let mut il: u32_0 = 0;
    let mut ir: u32_0 = 0;
    let mut t0: u32_0 = 0;
    let mut t1: u32_0 = 0;
    /* pre whitening but absorb kw2*/
    let ref mut fresh48 = *io.offset(0 as libc::c_int as isize);
    *fresh48 ^=
        *subkey.offset((24 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh49 = *io.offset(1 as libc::c_int as isize);
    *fresh49 ^=
        *subkey.offset((24 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    /* main iteration */
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh50 = *io.offset(2 as libc::c_int as isize);
    *fresh50 ^= ir;
    let ref mut fresh51 = *io.offset(3 as libc::c_int as isize);
    *fresh51 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh52 = *io.offset(0 as libc::c_int as isize);
    *fresh52 ^= ir;
    let ref mut fresh53 = *io.offset(1 as libc::c_int as isize);
    *fresh53 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh54 = *io.offset(2 as libc::c_int as isize);
    *fresh54 ^= ir;
    let ref mut fresh55 = *io.offset(3 as libc::c_int as isize);
    *fresh55 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh56 = *io.offset(0 as libc::c_int as isize);
    *fresh56 ^= ir;
    let ref mut fresh57 = *io.offset(1 as libc::c_int as isize);
    *fresh57 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh58 = *io.offset(2 as libc::c_int as isize);
    *fresh58 ^= ir;
    let ref mut fresh59 = *io.offset(3 as libc::c_int as isize);
    *fresh59 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh60 = *io.offset(0 as libc::c_int as isize);
    *fresh60 ^= ir;
    let ref mut fresh61 = *io.offset(1 as libc::c_int as isize);
    *fresh61 ^= il;
    t0 = *subkey.offset((17 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh62 = *io.offset(1 as libc::c_int as isize);
    *fresh62 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((17 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh63 = *io.offset(0 as libc::c_int as isize);
    *fresh63 ^= t1;
    il =
        *subkey.offset((16 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh64 = *io.offset(2 as libc::c_int as isize);
    *fresh64 ^= il;
    ir = *subkey.offset((16 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh65 = *io.offset(3 as libc::c_int as isize);
    *fresh65 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh66 = *io.offset(2 as libc::c_int as isize);
    *fresh66 ^= ir;
    let ref mut fresh67 = *io.offset(3 as libc::c_int as isize);
    *fresh67 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh68 = *io.offset(0 as libc::c_int as isize);
    *fresh68 ^= ir;
    let ref mut fresh69 = *io.offset(1 as libc::c_int as isize);
    *fresh69 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh70 = *io.offset(2 as libc::c_int as isize);
    *fresh70 ^= ir;
    let ref mut fresh71 = *io.offset(3 as libc::c_int as isize);
    *fresh71 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh72 = *io.offset(0 as libc::c_int as isize);
    *fresh72 ^= ir;
    let ref mut fresh73 = *io.offset(1 as libc::c_int as isize);
    *fresh73 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh74 = *io.offset(2 as libc::c_int as isize);
    *fresh74 ^= ir;
    let ref mut fresh75 = *io.offset(3 as libc::c_int as isize);
    *fresh75 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh76 = *io.offset(0 as libc::c_int as isize);
    *fresh76 ^= ir;
    let ref mut fresh77 = *io.offset(1 as libc::c_int as isize);
    *fresh77 ^= il;
    t0 = *subkey.offset((9 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh78 = *io.offset(1 as libc::c_int as isize);
    *fresh78 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((9 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh79 = *io.offset(0 as libc::c_int as isize);
    *fresh79 ^= t1;
    il =
        *subkey.offset((8 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh80 = *io.offset(2 as libc::c_int as isize);
    *fresh80 ^= il;
    ir = *subkey.offset((8 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh81 = *io.offset(3 as libc::c_int as isize);
    *fresh81 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh82 = *io.offset(2 as libc::c_int as isize);
    *fresh82 ^= ir;
    let ref mut fresh83 = *io.offset(3 as libc::c_int as isize);
    *fresh83 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh84 = *io.offset(0 as libc::c_int as isize);
    *fresh84 ^= ir;
    let ref mut fresh85 = *io.offset(1 as libc::c_int as isize);
    *fresh85 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh86 = *io.offset(2 as libc::c_int as isize);
    *fresh86 ^= ir;
    let ref mut fresh87 = *io.offset(3 as libc::c_int as isize);
    *fresh87 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh88 = *io.offset(0 as libc::c_int as isize);
    *fresh88 ^= ir;
    let ref mut fresh89 = *io.offset(1 as libc::c_int as isize);
    *fresh89 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh90 = *io.offset(2 as libc::c_int as isize);
    *fresh90 ^= ir;
    let ref mut fresh91 = *io.offset(3 as libc::c_int as isize);
    *fresh91 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh92 = *io.offset(0 as libc::c_int as isize);
    *fresh92 ^= ir;
    let ref mut fresh93 = *io.offset(1 as libc::c_int as isize);
    *fresh93 ^= il;
    /* post whitening but kw4 */
    let ref mut fresh94 = *io.offset(2 as libc::c_int as isize);
    *fresh94 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh95 = *io.offset(3 as libc::c_int as isize);
    *fresh95 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t0 = *io.offset(0 as libc::c_int as isize);
    t1 = *io.offset(1 as libc::c_int as isize);
    *io.offset(0 as libc::c_int as isize) =
        *io.offset(2 as libc::c_int as isize);
    *io.offset(1 as libc::c_int as isize) =
        *io.offset(3 as libc::c_int as isize);
    *io.offset(2 as libc::c_int as isize) = t0;
    *io.offset(3 as libc::c_int as isize) = t1;
}
/* *
 * stuff for 192 and 256bit encryption/decryption
 */
#[no_mangle]
#[c2rust::src_loc = "1158:1"]
pub unsafe extern "C" fn k5_camellia_encrypt256(mut subkey: *const u32_0,
                                                mut io: *mut u32_0) {
    let mut il: u32_0 = 0; /* temporary valiables */
    let mut ir: u32_0 = 0;
    let mut t0: u32_0 = 0;
    let mut t1: u32_0 = 0;
    /* pre whitening but absorb kw2*/
    let ref mut fresh96 = *io.offset(0 as libc::c_int as isize);
    *fresh96 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh97 = *io.offset(1 as libc::c_int as isize);
    *fresh97 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    /* main iteration */
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh98 = *io.offset(2 as libc::c_int as isize);
    *fresh98 ^= ir;
    let ref mut fresh99 = *io.offset(3 as libc::c_int as isize);
    *fresh99 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh100 = *io.offset(0 as libc::c_int as isize);
    *fresh100 ^= ir;
    let ref mut fresh101 = *io.offset(1 as libc::c_int as isize);
    *fresh101 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh102 = *io.offset(2 as libc::c_int as isize);
    *fresh102 ^= ir;
    let ref mut fresh103 = *io.offset(3 as libc::c_int as isize);
    *fresh103 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh104 = *io.offset(0 as libc::c_int as isize);
    *fresh104 ^= ir;
    let ref mut fresh105 = *io.offset(1 as libc::c_int as isize);
    *fresh105 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh106 = *io.offset(2 as libc::c_int as isize);
    *fresh106 ^= ir;
    let ref mut fresh107 = *io.offset(3 as libc::c_int as isize);
    *fresh107 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh108 = *io.offset(0 as libc::c_int as isize);
    *fresh108 ^= ir;
    let ref mut fresh109 = *io.offset(1 as libc::c_int as isize);
    *fresh109 ^= il;
    t0 = *subkey.offset((8 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh110 = *io.offset(1 as libc::c_int as isize);
    *fresh110 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((8 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh111 = *io.offset(0 as libc::c_int as isize);
    *fresh111 ^= t1;
    il =
        *subkey.offset((9 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh112 = *io.offset(2 as libc::c_int as isize);
    *fresh112 ^= il;
    ir = *subkey.offset((9 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh113 = *io.offset(3 as libc::c_int as isize);
    *fresh113 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh114 = *io.offset(2 as libc::c_int as isize);
    *fresh114 ^= ir;
    let ref mut fresh115 = *io.offset(3 as libc::c_int as isize);
    *fresh115 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh116 = *io.offset(0 as libc::c_int as isize);
    *fresh116 ^= ir;
    let ref mut fresh117 = *io.offset(1 as libc::c_int as isize);
    *fresh117 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh118 = *io.offset(2 as libc::c_int as isize);
    *fresh118 ^= ir;
    let ref mut fresh119 = *io.offset(3 as libc::c_int as isize);
    *fresh119 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh120 = *io.offset(0 as libc::c_int as isize);
    *fresh120 ^= ir;
    let ref mut fresh121 = *io.offset(1 as libc::c_int as isize);
    *fresh121 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh122 = *io.offset(2 as libc::c_int as isize);
    *fresh122 ^= ir;
    let ref mut fresh123 = *io.offset(3 as libc::c_int as isize);
    *fresh123 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh124 = *io.offset(0 as libc::c_int as isize);
    *fresh124 ^= ir;
    let ref mut fresh125 = *io.offset(1 as libc::c_int as isize);
    *fresh125 ^= il;
    t0 = *subkey.offset((16 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh126 = *io.offset(1 as libc::c_int as isize);
    *fresh126 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((16 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh127 = *io.offset(0 as libc::c_int as isize);
    *fresh127 ^= t1;
    il =
        *subkey.offset((17 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh128 = *io.offset(2 as libc::c_int as isize);
    *fresh128 ^= il;
    ir = *subkey.offset((17 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh129 = *io.offset(3 as libc::c_int as isize);
    *fresh129 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh130 = *io.offset(2 as libc::c_int as isize);
    *fresh130 ^= ir;
    let ref mut fresh131 = *io.offset(3 as libc::c_int as isize);
    *fresh131 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh132 = *io.offset(0 as libc::c_int as isize);
    *fresh132 ^= ir;
    let ref mut fresh133 = *io.offset(1 as libc::c_int as isize);
    *fresh133 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh134 = *io.offset(2 as libc::c_int as isize);
    *fresh134 ^= ir;
    let ref mut fresh135 = *io.offset(3 as libc::c_int as isize);
    *fresh135 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh136 = *io.offset(0 as libc::c_int as isize);
    *fresh136 ^= ir;
    let ref mut fresh137 = *io.offset(1 as libc::c_int as isize);
    *fresh137 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh138 = *io.offset(2 as libc::c_int as isize);
    *fresh138 ^= ir;
    let ref mut fresh139 = *io.offset(3 as libc::c_int as isize);
    *fresh139 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh140 = *io.offset(0 as libc::c_int as isize);
    *fresh140 ^= ir;
    let ref mut fresh141 = *io.offset(1 as libc::c_int as isize);
    *fresh141 ^= il;
    t0 = *subkey.offset((24 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh142 = *io.offset(1 as libc::c_int as isize);
    *fresh142 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((24 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh143 = *io.offset(0 as libc::c_int as isize);
    *fresh143 ^= t1;
    il =
        *subkey.offset((25 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh144 = *io.offset(2 as libc::c_int as isize);
    *fresh144 ^= il;
    ir = *subkey.offset((25 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh145 = *io.offset(3 as libc::c_int as isize);
    *fresh145 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((26 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((26 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh146 = *io.offset(2 as libc::c_int as isize);
    *fresh146 ^= ir;
    let ref mut fresh147 = *io.offset(3 as libc::c_int as isize);
    *fresh147 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((27 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((27 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh148 = *io.offset(0 as libc::c_int as isize);
    *fresh148 ^= ir;
    let ref mut fresh149 = *io.offset(1 as libc::c_int as isize);
    *fresh149 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((28 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((28 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh150 = *io.offset(2 as libc::c_int as isize);
    *fresh150 ^= ir;
    let ref mut fresh151 = *io.offset(3 as libc::c_int as isize);
    *fresh151 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((29 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((29 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh152 = *io.offset(0 as libc::c_int as isize);
    *fresh152 ^= ir;
    let ref mut fresh153 = *io.offset(1 as libc::c_int as isize);
    *fresh153 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((30 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((30 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh154 = *io.offset(2 as libc::c_int as isize);
    *fresh154 ^= ir;
    let ref mut fresh155 = *io.offset(3 as libc::c_int as isize);
    *fresh155 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((31 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((31 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh156 = *io.offset(0 as libc::c_int as isize);
    *fresh156 ^= ir;
    let ref mut fresh157 = *io.offset(1 as libc::c_int as isize);
    *fresh157 ^= il;
    /* post whitening but kw4 */
    let ref mut fresh158 =
        *io.offset(2 as libc::c_int as isize); /* temporary valiables */
    *fresh158 ^=
        *subkey.offset((32 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh159 = *io.offset(3 as libc::c_int as isize);
    *fresh159 ^=
        *subkey.offset((32 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t0 = *io.offset(0 as libc::c_int as isize);
    t1 = *io.offset(1 as libc::c_int as isize);
    *io.offset(0 as libc::c_int as isize) =
        *io.offset(2 as libc::c_int as isize);
    *io.offset(1 as libc::c_int as isize) =
        *io.offset(3 as libc::c_int as isize);
    *io.offset(2 as libc::c_int as isize) = t0;
    *io.offset(3 as libc::c_int as isize) = t1;
}
#[no_mangle]
#[c2rust::src_loc = "1273:1"]
pub unsafe extern "C" fn k5_camellia_decrypt256(mut subkey: *const u32_0,
                                                mut io: *mut u32_0) {
    let mut il: u32_0 = 0;
    let mut ir: u32_0 = 0;
    let mut t0: u32_0 = 0;
    let mut t1: u32_0 = 0;
    /* pre whitening but absorb kw2*/
    let ref mut fresh160 = *io.offset(0 as libc::c_int as isize);
    *fresh160 ^=
        *subkey.offset((32 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh161 = *io.offset(1 as libc::c_int as isize);
    *fresh161 ^=
        *subkey.offset((32 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    /* main iteration */
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((31 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((31 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh162 = *io.offset(2 as libc::c_int as isize);
    *fresh162 ^= ir;
    let ref mut fresh163 = *io.offset(3 as libc::c_int as isize);
    *fresh163 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((30 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((30 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh164 = *io.offset(0 as libc::c_int as isize);
    *fresh164 ^= ir;
    let ref mut fresh165 = *io.offset(1 as libc::c_int as isize);
    *fresh165 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((29 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((29 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh166 = *io.offset(2 as libc::c_int as isize);
    *fresh166 ^= ir;
    let ref mut fresh167 = *io.offset(3 as libc::c_int as isize);
    *fresh167 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((28 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((28 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh168 = *io.offset(0 as libc::c_int as isize);
    *fresh168 ^= ir;
    let ref mut fresh169 = *io.offset(1 as libc::c_int as isize);
    *fresh169 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((27 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((27 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh170 = *io.offset(2 as libc::c_int as isize);
    *fresh170 ^= ir;
    let ref mut fresh171 = *io.offset(3 as libc::c_int as isize);
    *fresh171 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((26 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((26 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh172 = *io.offset(0 as libc::c_int as isize);
    *fresh172 ^= ir;
    let ref mut fresh173 = *io.offset(1 as libc::c_int as isize);
    *fresh173 ^= il;
    t0 = *subkey.offset((25 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh174 = *io.offset(1 as libc::c_int as isize);
    *fresh174 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((25 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh175 = *io.offset(0 as libc::c_int as isize);
    *fresh175 ^= t1;
    il =
        *subkey.offset((24 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh176 = *io.offset(2 as libc::c_int as isize);
    *fresh176 ^= il;
    ir = *subkey.offset((24 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh177 = *io.offset(3 as libc::c_int as isize);
    *fresh177 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((23 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((23 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh178 = *io.offset(2 as libc::c_int as isize);
    *fresh178 ^= ir;
    let ref mut fresh179 = *io.offset(3 as libc::c_int as isize);
    *fresh179 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((22 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((22 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh180 = *io.offset(0 as libc::c_int as isize);
    *fresh180 ^= ir;
    let ref mut fresh181 = *io.offset(1 as libc::c_int as isize);
    *fresh181 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((21 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((21 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh182 = *io.offset(2 as libc::c_int as isize);
    *fresh182 ^= ir;
    let ref mut fresh183 = *io.offset(3 as libc::c_int as isize);
    *fresh183 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((20 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((20 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh184 = *io.offset(0 as libc::c_int as isize);
    *fresh184 ^= ir;
    let ref mut fresh185 = *io.offset(1 as libc::c_int as isize);
    *fresh185 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((19 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((19 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh186 = *io.offset(2 as libc::c_int as isize);
    *fresh186 ^= ir;
    let ref mut fresh187 = *io.offset(3 as libc::c_int as isize);
    *fresh187 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((18 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((18 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh188 = *io.offset(0 as libc::c_int as isize);
    *fresh188 ^= ir;
    let ref mut fresh189 = *io.offset(1 as libc::c_int as isize);
    *fresh189 ^= il;
    t0 = *subkey.offset((17 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh190 = *io.offset(1 as libc::c_int as isize);
    *fresh190 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((17 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh191 = *io.offset(0 as libc::c_int as isize);
    *fresh191 ^= t1;
    il =
        *subkey.offset((16 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh192 = *io.offset(2 as libc::c_int as isize);
    *fresh192 ^= il;
    ir = *subkey.offset((16 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh193 = *io.offset(3 as libc::c_int as isize);
    *fresh193 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((15 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((15 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh194 = *io.offset(2 as libc::c_int as isize);
    *fresh194 ^= ir;
    let ref mut fresh195 = *io.offset(3 as libc::c_int as isize);
    *fresh195 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((14 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((14 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh196 = *io.offset(0 as libc::c_int as isize);
    *fresh196 ^= ir;
    let ref mut fresh197 = *io.offset(1 as libc::c_int as isize);
    *fresh197 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((13 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((13 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh198 = *io.offset(2 as libc::c_int as isize);
    *fresh198 ^= ir;
    let ref mut fresh199 = *io.offset(3 as libc::c_int as isize);
    *fresh199 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((12 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((12 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh200 = *io.offset(0 as libc::c_int as isize);
    *fresh200 ^= ir;
    let ref mut fresh201 = *io.offset(1 as libc::c_int as isize);
    *fresh201 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((11 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((11 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh202 = *io.offset(2 as libc::c_int as isize);
    *fresh202 ^= ir;
    let ref mut fresh203 = *io.offset(3 as libc::c_int as isize);
    *fresh203 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((10 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((10 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh204 = *io.offset(0 as libc::c_int as isize);
    *fresh204 ^= ir;
    let ref mut fresh205 = *io.offset(1 as libc::c_int as isize);
    *fresh205 ^= il;
    t0 = *subkey.offset((9 as libc::c_int * 2 as libc::c_int) as isize);
    t0 &= *io.offset(0 as libc::c_int as isize);
    let ref mut fresh206 = *io.offset(1 as libc::c_int as isize);
    *fresh206 ^=
        (t0 << 1 as libc::c_int).wrapping_add(t0 >> 31 as libc::c_int);
    t1 =
        *subkey.offset((9 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t1 |= *io.offset(1 as libc::c_int as isize);
    let ref mut fresh207 = *io.offset(0 as libc::c_int as isize);
    *fresh207 ^= t1;
    il =
        *subkey.offset((8 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    il |= *io.offset(3 as libc::c_int as isize);
    let ref mut fresh208 = *io.offset(2 as libc::c_int as isize);
    *fresh208 ^= il;
    ir = *subkey.offset((8 as libc::c_int * 2 as libc::c_int) as isize);
    ir &= *io.offset(2 as libc::c_int as isize);
    let ref mut fresh209 = *io.offset(3 as libc::c_int as isize);
    *fresh209 ^=
        (ir << 1 as libc::c_int).wrapping_add(ir >> 31 as libc::c_int);
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((7 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((7 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh210 = *io.offset(2 as libc::c_int as isize);
    *fresh210 ^= ir;
    let ref mut fresh211 = *io.offset(3 as libc::c_int as isize);
    *fresh211 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((6 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((6 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh212 = *io.offset(0 as libc::c_int as isize);
    *fresh212 ^= ir;
    let ref mut fresh213 = *io.offset(1 as libc::c_int as isize);
    *fresh213 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((5 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((5 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh214 = *io.offset(2 as libc::c_int as isize);
    *fresh214 ^= ir;
    let ref mut fresh215 = *io.offset(3 as libc::c_int as isize);
    *fresh215 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((4 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((4 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh216 = *io.offset(0 as libc::c_int as isize);
    *fresh216 ^= ir;
    let ref mut fresh217 = *io.offset(1 as libc::c_int as isize);
    *fresh217 ^= il;
    ir =
        camellia_sp1110[(*io.offset(1 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(1 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(1 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(1 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(0 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(0 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(0 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(0 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((3 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((3 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh218 = *io.offset(2 as libc::c_int as isize);
    *fresh218 ^= ir;
    let ref mut fresh219 = *io.offset(3 as libc::c_int as isize);
    *fresh219 ^= il;
    ir =
        camellia_sp1110[(*io.offset(3 as libc::c_int as isize) &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(3 as libc::c_int as isize) >>
                                 24 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(3 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(3 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il =
        camellia_sp1110[(*io.offset(2 as libc::c_int as isize) >>
                             24 as libc::c_int &
                             0xff as libc::c_int as libc::c_uint) as usize] ^
            camellia_sp0222[(*io.offset(2 as libc::c_int as isize) >>
                                 16 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp3033[(*io.offset(2 as libc::c_int as isize) >>
                                 8 as libc::c_int &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize] ^
            camellia_sp4404[(*io.offset(2 as libc::c_int as isize) &
                                 0xff as libc::c_int as libc::c_uint) as
                                usize];
    il ^= *subkey.offset((2 as libc::c_int * 2 as libc::c_int) as isize);
    ir ^=
        *subkey.offset((2 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    ir ^= il;
    il = (il >> 8 as libc::c_int).wrapping_add(il << 24 as libc::c_int);
    il ^= ir;
    let ref mut fresh220 = *io.offset(0 as libc::c_int as isize);
    *fresh220 ^= ir;
    let ref mut fresh221 = *io.offset(1 as libc::c_int as isize);
    *fresh221 ^= il;
    /* post whitening but kw4 */
    let ref mut fresh222 = *io.offset(2 as libc::c_int as isize);
    *fresh222 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int) as isize);
    let ref mut fresh223 = *io.offset(3 as libc::c_int as isize);
    *fresh223 ^=
        *subkey.offset((0 as libc::c_int * 2 as libc::c_int +
                            1 as libc::c_int) as isize);
    t0 = *io.offset(0 as libc::c_int as isize);
    t1 = *io.offset(1 as libc::c_int as isize);
    *io.offset(0 as libc::c_int as isize) =
        *io.offset(2 as libc::c_int as isize);
    *io.offset(1 as libc::c_int as isize) =
        *io.offset(3 as libc::c_int as isize);
    *io.offset(2 as libc::c_int as isize) = t0;
    *io.offset(3 as libc::c_int as isize) = t1;
}
/* **
 *
 * API for compatibility
 */
#[no_mangle]
#[c2rust::src_loc = "1393:1"]
pub unsafe extern "C" fn k5_Camellia_Ekeygen(keyBitLength: libc::c_int,
                                             mut rawKey: *const libc::c_uchar,
                                             mut keyTable:
                                                 *mut libc::c_uint) {
    match keyBitLength {
        128 => { k5_camellia_setup128(rawKey, keyTable); }
        192 => { k5_camellia_setup192(rawKey, keyTable); }
        256 => { k5_camellia_setup256(rawKey, keyTable); }
        _ => { }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1414:1"]
pub unsafe extern "C" fn k5_Camellia_EncryptBlock(keyBitLength: libc::c_int,
                                                  mut plaintext:
                                                      *const libc::c_uchar,
                                                  mut keyTable:
                                                      *const libc::c_uint,
                                                  mut ciphertext:
                                                      *mut libc::c_uchar) {
    let mut tmp: [u32_0; 4] = [0; 4];
    tmp[0 as libc::c_int as usize] =
        (*plaintext.offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*plaintext.offset(1 as libc::c_int as isize) as u32_0) <<
                16 as libc::c_int ^
            (*plaintext.offset(2 as libc::c_int as isize) as u32_0) <<
                8 as libc::c_int ^
            *plaintext.offset(3 as libc::c_int as isize) as u32_0;
    tmp[1 as libc::c_int as usize] =
        (*plaintext.offset(4 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
             u32_0) << 24 as libc::c_int ^
            (*plaintext.offset(4 as libc::c_int as
                                   isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*plaintext.offset(4 as libc::c_int as
                                   isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *plaintext.offset(4 as libc::c_int as
                                  isize).offset(3 as libc::c_int as isize) as
                u32_0;
    tmp[2 as libc::c_int as usize] =
        (*plaintext.offset(8 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
             u32_0) << 24 as libc::c_int ^
            (*plaintext.offset(8 as libc::c_int as
                                   isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*plaintext.offset(8 as libc::c_int as
                                   isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *plaintext.offset(8 as libc::c_int as
                                  isize).offset(3 as libc::c_int as isize) as
                u32_0;
    tmp[3 as libc::c_int as usize] =
        (*plaintext.offset(12 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
             u32_0) << 24 as libc::c_int ^
            (*plaintext.offset(12 as libc::c_int as
                                   isize).offset(1 as libc::c_int as isize) as
                 u32_0) << 16 as libc::c_int ^
            (*plaintext.offset(12 as libc::c_int as
                                   isize).offset(2 as libc::c_int as isize) as
                 u32_0) << 8 as libc::c_int ^
            *plaintext.offset(12 as libc::c_int as
                                  isize).offset(3 as libc::c_int as isize) as
                u32_0;
    match keyBitLength {
        128 => { k5_camellia_encrypt128(keyTable, tmp.as_mut_ptr()); }
        192 | 256 => {
            /* fall through */
            k5_camellia_encrypt256(keyTable, tmp.as_mut_ptr());
        }
        _ => { }
    }
    *ciphertext.offset(0 as libc::c_int as isize) =
        (tmp[0 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *ciphertext.offset(1 as libc::c_int as isize) =
        (tmp[0 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *ciphertext.offset(2 as libc::c_int as isize) =
        (tmp[0 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *ciphertext.offset(3 as libc::c_int as isize) =
        tmp[0 as libc::c_int as usize] as u8_0;
    *ciphertext.offset(4 as libc::c_int as
                           isize).offset(0 as libc::c_int as isize) =
        (tmp[1 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *ciphertext.offset(4 as libc::c_int as
                           isize).offset(1 as libc::c_int as isize) =
        (tmp[1 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *ciphertext.offset(4 as libc::c_int as
                           isize).offset(2 as libc::c_int as isize) =
        (tmp[1 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *ciphertext.offset(4 as libc::c_int as
                           isize).offset(3 as libc::c_int as isize) =
        tmp[1 as libc::c_int as usize] as u8_0;
    *ciphertext.offset(8 as libc::c_int as
                           isize).offset(0 as libc::c_int as isize) =
        (tmp[2 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *ciphertext.offset(8 as libc::c_int as
                           isize).offset(1 as libc::c_int as isize) =
        (tmp[2 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *ciphertext.offset(8 as libc::c_int as
                           isize).offset(2 as libc::c_int as isize) =
        (tmp[2 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *ciphertext.offset(8 as libc::c_int as
                           isize).offset(3 as libc::c_int as isize) =
        tmp[2 as libc::c_int as usize] as u8_0;
    *ciphertext.offset(12 as libc::c_int as
                           isize).offset(0 as libc::c_int as isize) =
        (tmp[3 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *ciphertext.offset(12 as libc::c_int as
                           isize).offset(1 as libc::c_int as isize) =
        (tmp[3 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *ciphertext.offset(12 as libc::c_int as
                           isize).offset(2 as libc::c_int as isize) =
        (tmp[3 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *ciphertext.offset(12 as libc::c_int as
                           isize).offset(3 as libc::c_int as isize) =
        tmp[3 as libc::c_int as usize] as u8_0;
}
#[no_mangle]
#[c2rust::src_loc = "1446:1"]
pub unsafe extern "C" fn k5_Camellia_DecryptBlock(keyBitLength: libc::c_int,
                                                  mut ciphertext:
                                                      *const libc::c_uchar,
                                                  mut keyTable:
                                                      *const libc::c_uint,
                                                  mut plaintext:
                                                      *mut libc::c_uchar) {
    let mut tmp: [u32_0; 4] = [0; 4];
    tmp[0 as libc::c_int as usize] =
        (*ciphertext.offset(0 as libc::c_int as isize) as u32_0) <<
            24 as libc::c_int ^
            (*ciphertext.offset(1 as libc::c_int as isize) as u32_0) <<
                16 as libc::c_int ^
            (*ciphertext.offset(2 as libc::c_int as isize) as u32_0) <<
                8 as libc::c_int ^
            *ciphertext.offset(3 as libc::c_int as isize) as u32_0;
    tmp[1 as libc::c_int as usize] =
        (*ciphertext.offset(4 as libc::c_int as
                                isize).offset(0 as libc::c_int as isize) as
             u32_0) << 24 as libc::c_int ^
            (*ciphertext.offset(4 as libc::c_int as
                                    isize).offset(1 as libc::c_int as isize)
                 as u32_0) << 16 as libc::c_int ^
            (*ciphertext.offset(4 as libc::c_int as
                                    isize).offset(2 as libc::c_int as isize)
                 as u32_0) << 8 as libc::c_int ^
            *ciphertext.offset(4 as libc::c_int as
                                   isize).offset(3 as libc::c_int as isize) as
                u32_0;
    tmp[2 as libc::c_int as usize] =
        (*ciphertext.offset(8 as libc::c_int as
                                isize).offset(0 as libc::c_int as isize) as
             u32_0) << 24 as libc::c_int ^
            (*ciphertext.offset(8 as libc::c_int as
                                    isize).offset(1 as libc::c_int as isize)
                 as u32_0) << 16 as libc::c_int ^
            (*ciphertext.offset(8 as libc::c_int as
                                    isize).offset(2 as libc::c_int as isize)
                 as u32_0) << 8 as libc::c_int ^
            *ciphertext.offset(8 as libc::c_int as
                                   isize).offset(3 as libc::c_int as isize) as
                u32_0;
    tmp[3 as libc::c_int as usize] =
        (*ciphertext.offset(12 as libc::c_int as
                                isize).offset(0 as libc::c_int as isize) as
             u32_0) << 24 as libc::c_int ^
            (*ciphertext.offset(12 as libc::c_int as
                                    isize).offset(1 as libc::c_int as isize)
                 as u32_0) << 16 as libc::c_int ^
            (*ciphertext.offset(12 as libc::c_int as
                                    isize).offset(2 as libc::c_int as isize)
                 as u32_0) << 8 as libc::c_int ^
            *ciphertext.offset(12 as libc::c_int as
                                   isize).offset(3 as libc::c_int as isize) as
                u32_0;
    match keyBitLength {
        128 => { k5_camellia_decrypt128(keyTable, tmp.as_mut_ptr()); }
        192 | 256 => {
            /* fall through */
            k5_camellia_decrypt256(keyTable, tmp.as_mut_ptr());
        }
        _ => { }
    }
    *plaintext.offset(0 as libc::c_int as isize) =
        (tmp[0 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *plaintext.offset(1 as libc::c_int as isize) =
        (tmp[0 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *plaintext.offset(2 as libc::c_int as isize) =
        (tmp[0 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *plaintext.offset(3 as libc::c_int as isize) =
        tmp[0 as libc::c_int as usize] as u8_0;
    *plaintext.offset(4 as libc::c_int as
                          isize).offset(0 as libc::c_int as isize) =
        (tmp[1 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *plaintext.offset(4 as libc::c_int as
                          isize).offset(1 as libc::c_int as isize) =
        (tmp[1 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *plaintext.offset(4 as libc::c_int as
                          isize).offset(2 as libc::c_int as isize) =
        (tmp[1 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *plaintext.offset(4 as libc::c_int as
                          isize).offset(3 as libc::c_int as isize) =
        tmp[1 as libc::c_int as usize] as u8_0;
    *plaintext.offset(8 as libc::c_int as
                          isize).offset(0 as libc::c_int as isize) =
        (tmp[2 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *plaintext.offset(8 as libc::c_int as
                          isize).offset(1 as libc::c_int as isize) =
        (tmp[2 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *plaintext.offset(8 as libc::c_int as
                          isize).offset(2 as libc::c_int as isize) =
        (tmp[2 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *plaintext.offset(8 as libc::c_int as
                          isize).offset(3 as libc::c_int as isize) =
        tmp[2 as libc::c_int as usize] as u8_0;
    *plaintext.offset(12 as libc::c_int as
                          isize).offset(0 as libc::c_int as isize) =
        (tmp[3 as libc::c_int as usize] >> 24 as libc::c_int) as u8_0;
    *plaintext.offset(12 as libc::c_int as
                          isize).offset(1 as libc::c_int as isize) =
        (tmp[3 as libc::c_int as usize] >> 16 as libc::c_int) as u8_0;
    *plaintext.offset(12 as libc::c_int as
                          isize).offset(2 as libc::c_int as isize) =
        (tmp[3 as libc::c_int as usize] >> 8 as libc::c_int) as u8_0;
    *plaintext.offset(12 as libc::c_int as
                          isize).offset(3 as libc::c_int as isize) =
        tmp[3 as libc::c_int as usize] as u8_0;
}
#[no_mangle]
#[c2rust::src_loc = "1477:1"]
pub unsafe extern "C" fn krb5int_camellia_blk_len(mut blen: libc::c_uint,
                                                  mut cx: *mut camellia_ctx)
 -> cam_fret {
    if blen != 16 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as cam_fret
    }
    return 1 as libc::c_int as cam_fret;
}
#[no_mangle]
#[c2rust::src_loc = "1483:1"]
pub unsafe extern "C" fn krb5int_camellia_enc_key(mut in_key:
                                                      *const libc::c_uchar,
                                                  mut klen: libc::c_uint,
                                                  mut cx: *mut camellia_ctx)
 -> cam_fret {
    match klen {
        16 => {
            k5_camellia_setup128(in_key, (*cx).k_sch.as_mut_ptr());
            (*cx).keybitlen = 128 as libc::c_int
        }
        24 => {
            k5_camellia_setup192(in_key, (*cx).k_sch.as_mut_ptr());
            (*cx).keybitlen = 192 as libc::c_int
        }
        32 => {
            k5_camellia_setup256(in_key, (*cx).k_sch.as_mut_ptr());
            (*cx).keybitlen = 256 as libc::c_int
        }
        _ => { return 1 as libc::c_int as cam_fret }
    }
    return 1 as libc::c_int as cam_fret;
}
#[no_mangle]
#[c2rust::src_loc = "1505:1"]
pub unsafe extern "C" fn krb5int_camellia_enc_blk(mut in_blk:
                                                      *const libc::c_uchar,
                                                  mut out_blk:
                                                      *mut libc::c_uchar,
                                                  mut cx: *const camellia_ctx)
 -> cam_fret {
    k5_Camellia_EncryptBlock((*cx).keybitlen, in_blk, (*cx).k_sch.as_ptr(),
                             out_blk);
    return 1 as libc::c_int as cam_fret;
}
#[no_mangle]
#[c2rust::src_loc = "1512:1"]
pub unsafe extern "C" fn krb5int_camellia_dec_key(mut in_key:
                                                      *const libc::c_uchar,
                                                  mut klen: libc::c_uint,
                                                  mut cx: *mut camellia_ctx)
 -> cam_fret {
    match klen {
        16 => {
            k5_camellia_setup128(in_key, (*cx).k_sch.as_mut_ptr());
            (*cx).keybitlen = 128 as libc::c_int
        }
        24 => {
            k5_camellia_setup192(in_key, (*cx).k_sch.as_mut_ptr());
            (*cx).keybitlen = 192 as libc::c_int
        }
        32 => {
            k5_camellia_setup256(in_key, (*cx).k_sch.as_mut_ptr());
            (*cx).keybitlen = 256 as libc::c_int
        }
        _ => { return 1 as libc::c_int as cam_fret }
    }
    return 1 as libc::c_int as cam_fret;
}
/* implement normal or DLL functions    */
/* the Camellia context for encryption */
/* the encryption key schedule */
/* bitlength of key */
/* for Kerberos 5 tree -- hide names!  */
#[no_mangle]
#[c2rust::src_loc = "1534:1"]
pub unsafe extern "C" fn krb5int_camellia_dec_blk(mut in_blk:
                                                      *const libc::c_uchar,
                                                  mut out_blk:
                                                      *mut libc::c_uchar,
                                                  mut cx: *const camellia_ctx)
 -> cam_fret {
    k5_Camellia_DecryptBlock((*cx).keybitlen, in_blk, (*cx).k_sch.as_ptr(),
                             out_blk);
    return 1 as libc::c_int as cam_fret;
}
