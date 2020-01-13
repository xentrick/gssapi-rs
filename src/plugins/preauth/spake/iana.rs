use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/spake/iana.h:33"]
pub mod iana_h {
    #[c2rust::src_loc = "43:9"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "47:5"]
    pub const SPAKE_GROUP_P521: C2RustUnnamed = 4;
    #[c2rust::src_loc = "46:5"]
    pub const SPAKE_GROUP_P384: C2RustUnnamed = 3;
    #[c2rust::src_loc = "45:5"]
    pub const SPAKE_GROUP_P256: C2RustUnnamed = 2;
    #[c2rust::src_loc = "44:5"]
    pub const SPAKE_GROUP_EDWARDS25519: C2RustUnnamed = 1;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/spake/iana.h - SPAKE IANA registry contents */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:9"]
    pub struct spake_iana {
        pub id: int32_t,
        pub name: *const libc::c_char,
        pub mult_len: size_t,
        pub elem_len: size_t,
        pub m: *const uint8_t,
        pub n: *const uint8_t,
        pub hash_len: size_t,
    }
    use super::stdint_intn_h::int32_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint8_t;
    /* IANA_H */
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::stddef_h::size_t;
pub use self::iana_h::{C2RustUnnamed, SPAKE_GROUP_P521, SPAKE_GROUP_P384,
                       SPAKE_GROUP_P256, SPAKE_GROUP_EDWARDS25519,
                       spake_iana};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/spake/iana.c - SPAKE IANA registry contents */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
#[c2rust::src_loc = "35:16"]
static mut edwards25519_M: [uint8_t; 32] =
    [0xd0 as libc::c_int as uint8_t, 0x48 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x2c as libc::c_int as uint8_t,
     0x6e as libc::c_int as uint8_t, 0xa0 as libc::c_int as uint8_t,
     0xb6 as libc::c_int as uint8_t, 0xd6 as libc::c_int as uint8_t,
     0x97 as libc::c_int as uint8_t, 0xdd as libc::c_int as uint8_t,
     0xc2 as libc::c_int as uint8_t, 0xe8 as libc::c_int as uint8_t,
     0x6b as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0x85 as libc::c_int as uint8_t, 0xa3 as libc::c_int as uint8_t,
     0x3a as libc::c_int as uint8_t, 0xda as libc::c_int as uint8_t,
     0xc9 as libc::c_int as uint8_t, 0x20 as libc::c_int as uint8_t,
     0xf1 as libc::c_int as uint8_t, 0xbf as libc::c_int as uint8_t,
     0x18 as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0xb0 as libc::c_int as uint8_t, 0xc6 as libc::c_int as uint8_t,
     0xd1 as libc::c_int as uint8_t, 0x66 as libc::c_int as uint8_t,
     0xa5 as libc::c_int as uint8_t, 0xce as libc::c_int as uint8_t,
     0xcd as libc::c_int as uint8_t, 0xaf as libc::c_int as uint8_t];
#[c2rust::src_loc = "41:16"]
static mut edwards25519_N: [uint8_t; 32] =
    [0xd3 as libc::c_int as uint8_t, 0xbf as libc::c_int as uint8_t,
     0xb5 as libc::c_int as uint8_t, 0x18 as libc::c_int as uint8_t,
     0xf4 as libc::c_int as uint8_t, 0x4f as libc::c_int as uint8_t,
     0x34 as libc::c_int as uint8_t, 0x30 as libc::c_int as uint8_t,
     0xf2 as libc::c_int as uint8_t, 0x9d as libc::c_int as uint8_t,
     0xc as libc::c_int as uint8_t, 0x92 as libc::c_int as uint8_t,
     0xaf as libc::c_int as uint8_t, 0x50 as libc::c_int as uint8_t,
     0x38 as libc::c_int as uint8_t, 0x65 as libc::c_int as uint8_t,
     0xa1 as libc::c_int as uint8_t, 0xed as libc::c_int as uint8_t,
     0x32 as libc::c_int as uint8_t, 0x81 as libc::c_int as uint8_t,
     0xdc as libc::c_int as uint8_t, 0x69 as libc::c_int as uint8_t,
     0xb3 as libc::c_int as uint8_t, 0x5d as libc::c_int as uint8_t,
     0xd8 as libc::c_int as uint8_t, 0x68 as libc::c_int as uint8_t,
     0xba as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0xf8 as libc::c_int as uint8_t, 0x86 as libc::c_int as uint8_t,
     0xc4 as libc::c_int as uint8_t, 0xab as libc::c_int as uint8_t];
#[c2rust::src_loc = "47:16"]
static mut P256_M: [uint8_t; 33] =
    [0x2 as libc::c_int as uint8_t, 0x88 as libc::c_int as uint8_t,
     0x6e as libc::c_int as uint8_t, 0x2f as libc::c_int as uint8_t,
     0x97 as libc::c_int as uint8_t, 0xac as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0x6e as libc::c_int as uint8_t,
     0x55 as libc::c_int as uint8_t, 0xba as libc::c_int as uint8_t,
     0x9d as libc::c_int as uint8_t, 0xd7 as libc::c_int as uint8_t,
     0x24 as libc::c_int as uint8_t, 0x25 as libc::c_int as uint8_t,
     0x79 as libc::c_int as uint8_t, 0xf2 as libc::c_int as uint8_t,
     0x99 as libc::c_int as uint8_t, 0x3b as libc::c_int as uint8_t,
     0x64 as libc::c_int as uint8_t, 0xe1 as libc::c_int as uint8_t,
     0x6e as libc::c_int as uint8_t, 0xf3 as libc::c_int as uint8_t,
     0xdc as libc::c_int as uint8_t, 0xab as libc::c_int as uint8_t,
     0x95 as libc::c_int as uint8_t, 0xaf as libc::c_int as uint8_t,
     0xd4 as libc::c_int as uint8_t, 0x97 as libc::c_int as uint8_t,
     0x33 as libc::c_int as uint8_t, 0x3d as libc::c_int as uint8_t,
     0x8f as libc::c_int as uint8_t, 0xa1 as libc::c_int as uint8_t,
     0x2f as libc::c_int as uint8_t];
#[c2rust::src_loc = "53:16"]
static mut P256_N: [uint8_t; 33] =
    [0x3 as libc::c_int as uint8_t, 0xd8 as libc::c_int as uint8_t,
     0xbb as libc::c_int as uint8_t, 0xd6 as libc::c_int as uint8_t,
     0xc6 as libc::c_int as uint8_t, 0x39 as libc::c_int as uint8_t,
     0xc6 as libc::c_int as uint8_t, 0x29 as libc::c_int as uint8_t,
     0x37 as libc::c_int as uint8_t, 0xb0 as libc::c_int as uint8_t,
     0x4d as libc::c_int as uint8_t, 0x99 as libc::c_int as uint8_t,
     0x7f as libc::c_int as uint8_t, 0x38 as libc::c_int as uint8_t,
     0xc3 as libc::c_int as uint8_t, 0x77 as libc::c_int as uint8_t,
     0x7 as libc::c_int as uint8_t, 0x19 as libc::c_int as uint8_t,
     0xc6 as libc::c_int as uint8_t, 0x29 as libc::c_int as uint8_t,
     0xd7 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x4d as libc::c_int as uint8_t, 0x49 as libc::c_int as uint8_t,
     0xa2 as libc::c_int as uint8_t, 0x4b as libc::c_int as uint8_t,
     0x4f as libc::c_int as uint8_t, 0x98 as libc::c_int as uint8_t,
     0xba as libc::c_int as uint8_t, 0xa1 as libc::c_int as uint8_t,
     0x29 as libc::c_int as uint8_t, 0x2b as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t];
#[c2rust::src_loc = "59:16"]
static mut P384_M: [uint8_t; 49] =
    [0x3 as libc::c_int as uint8_t, 0xf as libc::c_int as uint8_t,
     0xf0 as libc::c_int as uint8_t, 0x89 as libc::c_int as uint8_t,
     0x5a as libc::c_int as uint8_t, 0xe5 as libc::c_int as uint8_t,
     0xeb as libc::c_int as uint8_t, 0xf6 as libc::c_int as uint8_t,
     0x18 as libc::c_int as uint8_t, 0x70 as libc::c_int as uint8_t,
     0x80 as libc::c_int as uint8_t, 0xa8 as libc::c_int as uint8_t,
     0x2d as libc::c_int as uint8_t, 0x82 as libc::c_int as uint8_t,
     0xb4 as libc::c_int as uint8_t, 0x2e as libc::c_int as uint8_t,
     0x27 as libc::c_int as uint8_t, 0x65 as libc::c_int as uint8_t,
     0xe3 as libc::c_int as uint8_t, 0xb2 as libc::c_int as uint8_t,
     0xf8 as libc::c_int as uint8_t, 0x74 as libc::c_int as uint8_t,
     0x9c as libc::c_int as uint8_t, 0x7e as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0xeb as libc::c_int as uint8_t,
     0xa3 as libc::c_int as uint8_t, 0x66 as libc::c_int as uint8_t,
     0x43 as libc::c_int as uint8_t, 0x4b as libc::c_int as uint8_t,
     0x36 as libc::c_int as uint8_t, 0x3d as libc::c_int as uint8_t,
     0x3d as libc::c_int as uint8_t, 0xc3 as libc::c_int as uint8_t,
     0x6f as libc::c_int as uint8_t, 0x15 as libc::c_int as uint8_t,
     0x31 as libc::c_int as uint8_t, 0x47 as libc::c_int as uint8_t,
     0x39 as libc::c_int as uint8_t, 0x7 as libc::c_int as uint8_t,
     0x4d as libc::c_int as uint8_t, 0x2e as libc::c_int as uint8_t,
     0xb8 as libc::c_int as uint8_t, 0x61 as libc::c_int as uint8_t,
     0x3f as libc::c_int as uint8_t, 0xce as libc::c_int as uint8_t,
     0xec as libc::c_int as uint8_t, 0x28 as libc::c_int as uint8_t,
     0x53 as libc::c_int as uint8_t];
#[c2rust::src_loc = "67:16"]
static mut P384_N: [uint8_t; 49] =
    [0x2 as libc::c_int as uint8_t, 0xc7 as libc::c_int as uint8_t,
     0x2c as libc::c_int as uint8_t, 0xf2 as libc::c_int as uint8_t,
     0xe3 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0x85 as libc::c_int as uint8_t, 0x3a as libc::c_int as uint8_t,
     0x1c as libc::c_int as uint8_t, 0x1c as libc::c_int as uint8_t,
     0x4a as libc::c_int as uint8_t, 0xd8 as libc::c_int as uint8_t,
     0x16 as libc::c_int as uint8_t, 0xa6 as libc::c_int as uint8_t,
     0x2f as libc::c_int as uint8_t, 0xd1 as libc::c_int as uint8_t,
     0x58 as libc::c_int as uint8_t, 0x24 as libc::c_int as uint8_t,
     0xf5 as libc::c_int as uint8_t, 0x60 as libc::c_int as uint8_t,
     0x78 as libc::c_int as uint8_t, 0x91 as libc::c_int as uint8_t,
     0x8f as libc::c_int as uint8_t, 0x43 as libc::c_int as uint8_t,
     0xf9 as libc::c_int as uint8_t, 0x22 as libc::c_int as uint8_t,
     0xca as libc::c_int as uint8_t, 0x21 as libc::c_int as uint8_t,
     0x51 as libc::c_int as uint8_t, 0x8f as libc::c_int as uint8_t,
     0x9c as libc::c_int as uint8_t, 0x54 as libc::c_int as uint8_t,
     0x3b as libc::c_int as uint8_t, 0xb2 as libc::c_int as uint8_t,
     0x52 as libc::c_int as uint8_t, 0xc5 as libc::c_int as uint8_t,
     0x49 as libc::c_int as uint8_t, 0x2 as libc::c_int as uint8_t,
     0x14 as libc::c_int as uint8_t, 0xcf as libc::c_int as uint8_t,
     0x9a as libc::c_int as uint8_t, 0xa3 as libc::c_int as uint8_t,
     0xf0 as libc::c_int as uint8_t, 0xba as libc::c_int as uint8_t,
     0xab as libc::c_int as uint8_t, 0x4b as libc::c_int as uint8_t,
     0x66 as libc::c_int as uint8_t, 0x5c as libc::c_int as uint8_t,
     0x10 as libc::c_int as uint8_t];
#[c2rust::src_loc = "75:16"]
static mut P521_M: [uint8_t; 67] =
    [0x2 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0x3f as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xf3 as libc::c_int as uint8_t, 0x81 as libc::c_int as uint8_t,
     0x31 as libc::c_int as uint8_t, 0xb2 as libc::c_int as uint8_t,
     0xba as libc::c_int as uint8_t, 0x26 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x79 as libc::c_int as uint8_t,
     0x1e as libc::c_int as uint8_t, 0x82 as libc::c_int as uint8_t,
     0x48 as libc::c_int as uint8_t, 0x8e as libc::c_int as uint8_t,
     0x8d as libc::c_int as uint8_t, 0x20 as libc::c_int as uint8_t,
     0xab as libc::c_int as uint8_t, 0x88 as libc::c_int as uint8_t,
     0x9a as libc::c_int as uint8_t, 0xf7 as libc::c_int as uint8_t,
     0x53 as libc::c_int as uint8_t, 0xa4 as libc::c_int as uint8_t,
     0x18 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0xc5 as libc::c_int as uint8_t, 0xdb as libc::c_int as uint8_t,
     0x18 as libc::c_int as uint8_t, 0xd3 as libc::c_int as uint8_t,
     0x7d as libc::c_int as uint8_t, 0x85 as libc::c_int as uint8_t,
     0x60 as libc::c_int as uint8_t, 0x8c as libc::c_int as uint8_t,
     0xfa as libc::c_int as uint8_t, 0xe0 as libc::c_int as uint8_t,
     0x6b as libc::c_int as uint8_t, 0x82 as libc::c_int as uint8_t,
     0xe4 as libc::c_int as uint8_t, 0xa7 as libc::c_int as uint8_t,
     0x2c as libc::c_int as uint8_t, 0xd7 as libc::c_int as uint8_t,
     0x44 as libc::c_int as uint8_t, 0xc7 as libc::c_int as uint8_t,
     0x19 as libc::c_int as uint8_t, 0x19 as libc::c_int as uint8_t,
     0x35 as libc::c_int as uint8_t, 0x62 as libc::c_int as uint8_t,
     0xa6 as libc::c_int as uint8_t, 0x53 as libc::c_int as uint8_t,
     0xea as libc::c_int as uint8_t, 0x1f as libc::c_int as uint8_t,
     0x11 as libc::c_int as uint8_t, 0x9e as libc::c_int as uint8_t,
     0xef as libc::c_int as uint8_t, 0x93 as libc::c_int as uint8_t,
     0x56 as libc::c_int as uint8_t, 0x90 as libc::c_int as uint8_t,
     0x7e as libc::c_int as uint8_t, 0xdc as libc::c_int as uint8_t,
     0x9b as libc::c_int as uint8_t, 0x56 as libc::c_int as uint8_t,
     0x97 as libc::c_int as uint8_t, 0x99 as libc::c_int as uint8_t,
     0x62 as libc::c_int as uint8_t, 0xd7 as libc::c_int as uint8_t,
     0xaa as libc::c_int as uint8_t];
#[c2rust::src_loc = "84:16"]
static mut P521_N: [uint8_t; 67] =
    [0x2 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0xc7 as libc::c_int as uint8_t, 0x92 as libc::c_int as uint8_t,
     0x4b as libc::c_int as uint8_t, 0x9e as libc::c_int as uint8_t,
     0xc0 as libc::c_int as uint8_t, 0x17 as libc::c_int as uint8_t,
     0xf3 as libc::c_int as uint8_t, 0x9 as libc::c_int as uint8_t,
     0x45 as libc::c_int as uint8_t, 0x62 as libc::c_int as uint8_t,
     0x89 as libc::c_int as uint8_t, 0x43 as libc::c_int as uint8_t,
     0x36 as libc::c_int as uint8_t, 0xa5 as libc::c_int as uint8_t,
     0x3c as libc::c_int as uint8_t, 0x50 as libc::c_int as uint8_t,
     0x16 as libc::c_int as uint8_t, 0x7b as libc::c_int as uint8_t,
     0xa8 as libc::c_int as uint8_t, 0xc5 as libc::c_int as uint8_t,
     0x96 as libc::c_int as uint8_t, 0x38 as libc::c_int as uint8_t,
     0x76 as libc::c_int as uint8_t, 0x88 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x42 as libc::c_int as uint8_t,
     0xbc as libc::c_int as uint8_t, 0x66 as libc::c_int as uint8_t,
     0x9e as libc::c_int as uint8_t, 0x49 as libc::c_int as uint8_t,
     0x4b as libc::c_int as uint8_t, 0x25 as libc::c_int as uint8_t,
     0x32 as libc::c_int as uint8_t, 0xd7 as libc::c_int as uint8_t,
     0x6c as libc::c_int as uint8_t, 0x5b as libc::c_int as uint8_t,
     0x53 as libc::c_int as uint8_t, 0xdf as libc::c_int as uint8_t,
     0xb3 as libc::c_int as uint8_t, 0x49 as libc::c_int as uint8_t,
     0xfd as libc::c_int as uint8_t, 0xf6 as libc::c_int as uint8_t,
     0x91 as libc::c_int as uint8_t, 0x54 as libc::c_int as uint8_t,
     0xb9 as libc::c_int as uint8_t, 0xe0 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0x8c as libc::c_int as uint8_t,
     0x58 as libc::c_int as uint8_t, 0xa4 as libc::c_int as uint8_t,
     0x2e as libc::c_int as uint8_t, 0x8e as libc::c_int as uint8_t,
     0xd0 as libc::c_int as uint8_t, 0x4c as libc::c_int as uint8_t,
     0xef as libc::c_int as uint8_t, 0x5 as libc::c_int as uint8_t,
     0x2a as libc::c_int as uint8_t, 0x3b as libc::c_int as uint8_t,
     0xc3 as libc::c_int as uint8_t, 0x49 as libc::c_int as uint8_t,
     0xd9 as libc::c_int as uint8_t, 0x55 as libc::c_int as uint8_t,
     0x75 as libc::c_int as uint8_t, 0xcd as libc::c_int as uint8_t,
     0x25 as libc::c_int as uint8_t];
#[no_mangle]
#[c2rust::src_loc = "93:18"]
pub static mut spake_iana_edwards25519: spake_iana =
    unsafe {
        {
            let mut init =
                spake_iana{id: SPAKE_GROUP_EDWARDS25519 as libc::c_int,
                           name:
                               b"edwards25519\x00" as *const u8 as
                                   *const libc::c_char,
                           mult_len: 32 as libc::c_int as size_t,
                           elem_len: 32 as libc::c_int as size_t,
                           m: edwards25519_M.as_ptr() as *mut _,
                           n: edwards25519_N.as_ptr() as *mut _,
                           hash_len: 32 as libc::c_int as size_t,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "98:18"]
pub static mut spake_iana_p256: spake_iana =
    unsafe {
        {
            let mut init =
                spake_iana{id: SPAKE_GROUP_P256 as libc::c_int,
                           name:
                               b"P-256\x00" as *const u8 as
                                   *const libc::c_char,
                           mult_len: 32 as libc::c_int as size_t,
                           elem_len: 33 as libc::c_int as size_t,
                           m: P256_M.as_ptr() as *mut _,
                           n: P256_N.as_ptr() as *mut _,
                           hash_len: 32 as libc::c_int as size_t,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "102:18"]
pub static mut spake_iana_p384: spake_iana =
    unsafe {
        {
            let mut init =
                spake_iana{id: SPAKE_GROUP_P384 as libc::c_int,
                           name:
                               b"P-384\x00" as *const u8 as
                                   *const libc::c_char,
                           mult_len: 48 as libc::c_int as size_t,
                           elem_len: 49 as libc::c_int as size_t,
                           m: P384_M.as_ptr() as *mut _,
                           n: P384_N.as_ptr() as *mut _,
                           hash_len: 48 as libc::c_int as size_t,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "106:18"]
pub static mut spake_iana_p521: spake_iana =
    unsafe {
        {
            let mut init =
                spake_iana{id: SPAKE_GROUP_P521 as libc::c_int,
                           name:
                               b"P-521\x00" as *const u8 as
                                   *const libc::c_char,
                           mult_len: 66 as libc::c_int as size_t,
                           elem_len: 67 as libc::c_int as size_t,
                           m: P521_M.as_ptr() as *mut _,
                           n: P521_N.as_ptr() as *mut _,
                           hash_len: 64 as libc::c_int as size_t,};
            init
        }
    };
