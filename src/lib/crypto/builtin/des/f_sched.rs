use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:32"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:33"]
pub mod des_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct des_ks_struct {
        pub _data: [libc::c_int; 2],
    }
    /*DES_INTERNAL_DEFS*/
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:32"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::__uint32_t;
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::k5_platform_h::{C2RustUnnamed, load_32_be};
pub use self::des_int_h::des_ks_struct;
pub use self::byteswap_h::__bswap_32;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/f_sched.c */
/*
 * Copyright (C) 1990 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
/* DES implementation donated by Dennis Ferguson */
/*
 * des_make_sched.c - permute a DES key, returning the resulting key schedule
 */
/*
 * Permuted choice 1 tables.  These are used to extract bits
 * from the left and right parts of the key to form Ci and Di.
 * The code that uses these tables knows which bits from which
 * part of each key are used to form Ci and Di.
 */
#[c2rust::src_loc = "41:33"]
static mut PC1_CL: [libc::c_uint; 8] =
    [0 as libc::c_int as libc::c_uint, 0x10 as libc::c_int as libc::c_uint,
     0x1000 as libc::c_int as libc::c_uint,
     0x1010 as libc::c_int as libc::c_uint,
     0x100000 as libc::c_int as libc::c_uint,
     0x100010 as libc::c_int as libc::c_uint,
     0x101000 as libc::c_int as libc::c_uint,
     0x101010 as libc::c_int as libc::c_uint];
#[c2rust::src_loc = "46:33"]
static mut PC1_DL: [libc::c_uint; 16] =
    [0 as libc::c_int as libc::c_uint,
     0x100000 as libc::c_int as libc::c_uint,
     0x1000 as libc::c_int as libc::c_uint,
     0x101000 as libc::c_int as libc::c_uint,
     0x10 as libc::c_int as libc::c_uint,
     0x100010 as libc::c_int as libc::c_uint,
     0x1010 as libc::c_int as libc::c_uint,
     0x101010 as libc::c_int as libc::c_uint,
     0x1 as libc::c_int as libc::c_uint,
     0x100001 as libc::c_int as libc::c_uint,
     0x1001 as libc::c_int as libc::c_uint,
     0x101001 as libc::c_int as libc::c_uint,
     0x11 as libc::c_int as libc::c_uint,
     0x100011 as libc::c_int as libc::c_uint,
     0x1011 as libc::c_int as libc::c_uint,
     0x101011 as libc::c_int as libc::c_uint];
#[c2rust::src_loc = "53:33"]
static mut PC1_CR: [libc::c_uint; 16] =
    [0 as libc::c_int as libc::c_uint, 0x1 as libc::c_int as libc::c_uint,
     0x100 as libc::c_int as libc::c_uint,
     0x101 as libc::c_int as libc::c_uint,
     0x10000 as libc::c_int as libc::c_uint,
     0x10001 as libc::c_int as libc::c_uint,
     0x10100 as libc::c_int as libc::c_uint,
     0x10101 as libc::c_int as libc::c_uint,
     0x1000000 as libc::c_int as libc::c_uint,
     0x1000001 as libc::c_int as libc::c_uint,
     0x1000100 as libc::c_int as libc::c_uint,
     0x1000101 as libc::c_int as libc::c_uint,
     0x1010000 as libc::c_int as libc::c_uint,
     0x1010001 as libc::c_int as libc::c_uint,
     0x1010100 as libc::c_int as libc::c_uint,
     0x1010101 as libc::c_int as libc::c_uint];
#[c2rust::src_loc = "60:33"]
static mut PC1_DR: [libc::c_uint; 8] =
    [0 as libc::c_int as libc::c_uint,
     0x1000000 as libc::c_int as libc::c_uint,
     0x10000 as libc::c_int as libc::c_uint,
     0x1010000 as libc::c_int as libc::c_uint,
     0x100 as libc::c_int as libc::c_uint,
     0x1000100 as libc::c_int as libc::c_uint,
     0x10100 as libc::c_int as libc::c_uint,
     0x1010100 as libc::c_int as libc::c_uint];
/*
 * Permuted choice 2 tables.  The first actually produces the low order
 * 24 bits of the subkey Ki from the 28 bit value of Ci.  The second produces
 * the high order 24 bits from Di.  The tables are indexed by six bit
 * segments of Ci and Di respectively.  The code is handcrafted to compute
 * the appropriate 6 bit chunks.
 *
 * Note that for ease of computation, the 24 bit values are produced with
 * six bits going into each byte.  Note also that the table has been byte
 * rearranged to produce keys which match the order we will apply them
 * in in the des code.
 */
#[c2rust::src_loc = "86:33"]
static mut PC2_C: [[libc::c_uint; 64]; 4] =
    [[0 as libc::c_int as libc::c_uint, 0x4 as libc::c_int as libc::c_uint,
      0x10000 as libc::c_int as libc::c_uint,
      0x10004 as libc::c_int as libc::c_uint,
      0x400 as libc::c_int as libc::c_uint,
      0x404 as libc::c_int as libc::c_uint,
      0x10400 as libc::c_int as libc::c_uint,
      0x10404 as libc::c_int as libc::c_uint,
      0x20 as libc::c_int as libc::c_uint,
      0x24 as libc::c_int as libc::c_uint,
      0x10020 as libc::c_int as libc::c_uint,
      0x10024 as libc::c_int as libc::c_uint,
      0x420 as libc::c_int as libc::c_uint,
      0x424 as libc::c_int as libc::c_uint,
      0x10420 as libc::c_int as libc::c_uint,
      0x10424 as libc::c_int as libc::c_uint,
      0x1000000 as libc::c_int as libc::c_uint,
      0x1000004 as libc::c_int as libc::c_uint,
      0x1010000 as libc::c_int as libc::c_uint,
      0x1010004 as libc::c_int as libc::c_uint,
      0x1000400 as libc::c_int as libc::c_uint,
      0x1000404 as libc::c_int as libc::c_uint,
      0x1010400 as libc::c_int as libc::c_uint,
      0x1010404 as libc::c_int as libc::c_uint,
      0x1000020 as libc::c_int as libc::c_uint,
      0x1000024 as libc::c_int as libc::c_uint,
      0x1010020 as libc::c_int as libc::c_uint,
      0x1010024 as libc::c_int as libc::c_uint,
      0x1000420 as libc::c_int as libc::c_uint,
      0x1000424 as libc::c_int as libc::c_uint,
      0x1010420 as libc::c_int as libc::c_uint,
      0x1010424 as libc::c_int as libc::c_uint,
      0x20000 as libc::c_int as libc::c_uint,
      0x20004 as libc::c_int as libc::c_uint,
      0x30000 as libc::c_int as libc::c_uint,
      0x30004 as libc::c_int as libc::c_uint,
      0x20400 as libc::c_int as libc::c_uint,
      0x20404 as libc::c_int as libc::c_uint,
      0x30400 as libc::c_int as libc::c_uint,
      0x30404 as libc::c_int as libc::c_uint,
      0x20020 as libc::c_int as libc::c_uint,
      0x20024 as libc::c_int as libc::c_uint,
      0x30020 as libc::c_int as libc::c_uint,
      0x30024 as libc::c_int as libc::c_uint,
      0x20420 as libc::c_int as libc::c_uint,
      0x20424 as libc::c_int as libc::c_uint,
      0x30420 as libc::c_int as libc::c_uint,
      0x30424 as libc::c_int as libc::c_uint,
      0x1020000 as libc::c_int as libc::c_uint,
      0x1020004 as libc::c_int as libc::c_uint,
      0x1030000 as libc::c_int as libc::c_uint,
      0x1030004 as libc::c_int as libc::c_uint,
      0x1020400 as libc::c_int as libc::c_uint,
      0x1020404 as libc::c_int as libc::c_uint,
      0x1030400 as libc::c_int as libc::c_uint,
      0x1030404 as libc::c_int as libc::c_uint,
      0x1020020 as libc::c_int as libc::c_uint,
      0x1020024 as libc::c_int as libc::c_uint,
      0x1030020 as libc::c_int as libc::c_uint,
      0x1030024 as libc::c_int as libc::c_uint,
      0x1020420 as libc::c_int as libc::c_uint,
      0x1020424 as libc::c_int as libc::c_uint,
      0x1030420 as libc::c_int as libc::c_uint,
      0x1030424 as libc::c_int as libc::c_uint],
     [0 as libc::c_int as libc::c_uint,
      0x2000000 as libc::c_int as libc::c_uint,
      0x800 as libc::c_int as libc::c_uint,
      0x2000800 as libc::c_int as libc::c_uint,
      0x80000 as libc::c_int as libc::c_uint,
      0x2080000 as libc::c_int as libc::c_uint,
      0x80800 as libc::c_int as libc::c_uint,
      0x2080800 as libc::c_int as libc::c_uint,
      0x1 as libc::c_int as libc::c_uint,
      0x2000001 as libc::c_int as libc::c_uint,
      0x801 as libc::c_int as libc::c_uint,
      0x2000801 as libc::c_int as libc::c_uint,
      0x80001 as libc::c_int as libc::c_uint,
      0x2080001 as libc::c_int as libc::c_uint,
      0x80801 as libc::c_int as libc::c_uint,
      0x2080801 as libc::c_int as libc::c_uint,
      0x100 as libc::c_int as libc::c_uint,
      0x2000100 as libc::c_int as libc::c_uint,
      0x900 as libc::c_int as libc::c_uint,
      0x2000900 as libc::c_int as libc::c_uint,
      0x80100 as libc::c_int as libc::c_uint,
      0x2080100 as libc::c_int as libc::c_uint,
      0x80900 as libc::c_int as libc::c_uint,
      0x2080900 as libc::c_int as libc::c_uint,
      0x101 as libc::c_int as libc::c_uint,
      0x2000101 as libc::c_int as libc::c_uint,
      0x901 as libc::c_int as libc::c_uint,
      0x2000901 as libc::c_int as libc::c_uint,
      0x80101 as libc::c_int as libc::c_uint,
      0x2080101 as libc::c_int as libc::c_uint,
      0x80901 as libc::c_int as libc::c_uint,
      0x2080901 as libc::c_int as libc::c_uint,
      0x10000000 as libc::c_int as libc::c_uint,
      0x12000000 as libc::c_int as libc::c_uint,
      0x10000800 as libc::c_int as libc::c_uint,
      0x12000800 as libc::c_int as libc::c_uint,
      0x10080000 as libc::c_int as libc::c_uint,
      0x12080000 as libc::c_int as libc::c_uint,
      0x10080800 as libc::c_int as libc::c_uint,
      0x12080800 as libc::c_int as libc::c_uint,
      0x10000001 as libc::c_int as libc::c_uint,
      0x12000001 as libc::c_int as libc::c_uint,
      0x10000801 as libc::c_int as libc::c_uint,
      0x12000801 as libc::c_int as libc::c_uint,
      0x10080001 as libc::c_int as libc::c_uint,
      0x12080001 as libc::c_int as libc::c_uint,
      0x10080801 as libc::c_int as libc::c_uint,
      0x12080801 as libc::c_int as libc::c_uint,
      0x10000100 as libc::c_int as libc::c_uint,
      0x12000100 as libc::c_int as libc::c_uint,
      0x10000900 as libc::c_int as libc::c_uint,
      0x12000900 as libc::c_int as libc::c_uint,
      0x10080100 as libc::c_int as libc::c_uint,
      0x12080100 as libc::c_int as libc::c_uint,
      0x10080900 as libc::c_int as libc::c_uint,
      0x12080900 as libc::c_int as libc::c_uint,
      0x10000101 as libc::c_int as libc::c_uint,
      0x12000101 as libc::c_int as libc::c_uint,
      0x10000901 as libc::c_int as libc::c_uint,
      0x12000901 as libc::c_int as libc::c_uint,
      0x10080101 as libc::c_int as libc::c_uint,
      0x12080101 as libc::c_int as libc::c_uint,
      0x10080901 as libc::c_int as libc::c_uint,
      0x12080901 as libc::c_int as libc::c_uint],
     [0 as libc::c_int as libc::c_uint,
      0x40000 as libc::c_int as libc::c_uint,
      0x2000 as libc::c_int as libc::c_uint,
      0x42000 as libc::c_int as libc::c_uint,
      0x100000 as libc::c_int as libc::c_uint,
      0x140000 as libc::c_int as libc::c_uint,
      0x102000 as libc::c_int as libc::c_uint,
      0x142000 as libc::c_int as libc::c_uint,
      0x20000000 as libc::c_int as libc::c_uint,
      0x20040000 as libc::c_int as libc::c_uint,
      0x20002000 as libc::c_int as libc::c_uint,
      0x20042000 as libc::c_int as libc::c_uint,
      0x20100000 as libc::c_int as libc::c_uint,
      0x20140000 as libc::c_int as libc::c_uint,
      0x20102000 as libc::c_int as libc::c_uint,
      0x20142000 as libc::c_int as libc::c_uint,
      0x8 as libc::c_int as libc::c_uint,
      0x40008 as libc::c_int as libc::c_uint,
      0x2008 as libc::c_int as libc::c_uint,
      0x42008 as libc::c_int as libc::c_uint,
      0x100008 as libc::c_int as libc::c_uint,
      0x140008 as libc::c_int as libc::c_uint,
      0x102008 as libc::c_int as libc::c_uint,
      0x142008 as libc::c_int as libc::c_uint,
      0x20000008 as libc::c_int as libc::c_uint,
      0x20040008 as libc::c_int as libc::c_uint,
      0x20002008 as libc::c_int as libc::c_uint,
      0x20042008 as libc::c_int as libc::c_uint,
      0x20100008 as libc::c_int as libc::c_uint,
      0x20140008 as libc::c_int as libc::c_uint,
      0x20102008 as libc::c_int as libc::c_uint,
      0x20142008 as libc::c_int as libc::c_uint,
      0x200000 as libc::c_int as libc::c_uint,
      0x240000 as libc::c_int as libc::c_uint,
      0x202000 as libc::c_int as libc::c_uint,
      0x242000 as libc::c_int as libc::c_uint,
      0x300000 as libc::c_int as libc::c_uint,
      0x340000 as libc::c_int as libc::c_uint,
      0x302000 as libc::c_int as libc::c_uint,
      0x342000 as libc::c_int as libc::c_uint,
      0x20200000 as libc::c_int as libc::c_uint,
      0x20240000 as libc::c_int as libc::c_uint,
      0x20202000 as libc::c_int as libc::c_uint,
      0x20242000 as libc::c_int as libc::c_uint,
      0x20300000 as libc::c_int as libc::c_uint,
      0x20340000 as libc::c_int as libc::c_uint,
      0x20302000 as libc::c_int as libc::c_uint,
      0x20342000 as libc::c_int as libc::c_uint,
      0x200008 as libc::c_int as libc::c_uint,
      0x240008 as libc::c_int as libc::c_uint,
      0x202008 as libc::c_int as libc::c_uint,
      0x242008 as libc::c_int as libc::c_uint,
      0x300008 as libc::c_int as libc::c_uint,
      0x340008 as libc::c_int as libc::c_uint,
      0x302008 as libc::c_int as libc::c_uint,
      0x342008 as libc::c_int as libc::c_uint,
      0x20200008 as libc::c_int as libc::c_uint,
      0x20240008 as libc::c_int as libc::c_uint,
      0x20202008 as libc::c_int as libc::c_uint,
      0x20242008 as libc::c_int as libc::c_uint,
      0x20300008 as libc::c_int as libc::c_uint,
      0x20340008 as libc::c_int as libc::c_uint,
      0x20302008 as libc::c_int as libc::c_uint,
      0x20342008 as libc::c_int as libc::c_uint],
     [0 as libc::c_int as libc::c_uint, 0x10 as libc::c_int as libc::c_uint,
      0x8000000 as libc::c_int as libc::c_uint,
      0x8000010 as libc::c_int as libc::c_uint,
      0x200 as libc::c_int as libc::c_uint,
      0x210 as libc::c_int as libc::c_uint,
      0x8000200 as libc::c_int as libc::c_uint,
      0x8000210 as libc::c_int as libc::c_uint,
      0x2 as libc::c_int as libc::c_uint, 0x12 as libc::c_int as libc::c_uint,
      0x8000002 as libc::c_int as libc::c_uint,
      0x8000012 as libc::c_int as libc::c_uint,
      0x202 as libc::c_int as libc::c_uint,
      0x212 as libc::c_int as libc::c_uint,
      0x8000202 as libc::c_int as libc::c_uint,
      0x8000212 as libc::c_int as libc::c_uint,
      0x4000000 as libc::c_int as libc::c_uint,
      0x4000010 as libc::c_int as libc::c_uint,
      0xc000000 as libc::c_int as libc::c_uint,
      0xc000010 as libc::c_int as libc::c_uint,
      0x4000200 as libc::c_int as libc::c_uint,
      0x4000210 as libc::c_int as libc::c_uint,
      0xc000200 as libc::c_int as libc::c_uint,
      0xc000210 as libc::c_int as libc::c_uint,
      0x4000002 as libc::c_int as libc::c_uint,
      0x4000012 as libc::c_int as libc::c_uint,
      0xc000002 as libc::c_int as libc::c_uint,
      0xc000012 as libc::c_int as libc::c_uint,
      0x4000202 as libc::c_int as libc::c_uint,
      0x4000212 as libc::c_int as libc::c_uint,
      0xc000202 as libc::c_int as libc::c_uint,
      0xc000212 as libc::c_int as libc::c_uint,
      0x1000 as libc::c_int as libc::c_uint,
      0x1010 as libc::c_int as libc::c_uint,
      0x8001000 as libc::c_int as libc::c_uint,
      0x8001010 as libc::c_int as libc::c_uint,
      0x1200 as libc::c_int as libc::c_uint,
      0x1210 as libc::c_int as libc::c_uint,
      0x8001200 as libc::c_int as libc::c_uint,
      0x8001210 as libc::c_int as libc::c_uint,
      0x1002 as libc::c_int as libc::c_uint,
      0x1012 as libc::c_int as libc::c_uint,
      0x8001002 as libc::c_int as libc::c_uint,
      0x8001012 as libc::c_int as libc::c_uint,
      0x1202 as libc::c_int as libc::c_uint,
      0x1212 as libc::c_int as libc::c_uint,
      0x8001202 as libc::c_int as libc::c_uint,
      0x8001212 as libc::c_int as libc::c_uint,
      0x4001000 as libc::c_int as libc::c_uint,
      0x4001010 as libc::c_int as libc::c_uint,
      0xc001000 as libc::c_int as libc::c_uint,
      0xc001010 as libc::c_int as libc::c_uint,
      0x4001200 as libc::c_int as libc::c_uint,
      0x4001210 as libc::c_int as libc::c_uint,
      0xc001200 as libc::c_int as libc::c_uint,
      0xc001210 as libc::c_int as libc::c_uint,
      0x4001002 as libc::c_int as libc::c_uint,
      0x4001012 as libc::c_int as libc::c_uint,
      0xc001002 as libc::c_int as libc::c_uint,
      0xc001012 as libc::c_int as libc::c_uint,
      0x4001202 as libc::c_int as libc::c_uint,
      0x4001212 as libc::c_int as libc::c_uint,
      0xc001202 as libc::c_int as libc::c_uint,
      0xc001212 as libc::c_int as libc::c_uint]];
#[c2rust::src_loc = "161:33"]
static mut PC2_D: [[libc::c_uint; 64]; 4] =
    [[0 as libc::c_int as libc::c_uint,
      0x2000000 as libc::c_int as libc::c_uint,
      0x20000 as libc::c_int as libc::c_uint,
      0x2020000 as libc::c_int as libc::c_uint,
      0x100 as libc::c_int as libc::c_uint,
      0x2000100 as libc::c_int as libc::c_uint,
      0x20100 as libc::c_int as libc::c_uint,
      0x2020100 as libc::c_int as libc::c_uint,
      0x8 as libc::c_int as libc::c_uint,
      0x2000008 as libc::c_int as libc::c_uint,
      0x20008 as libc::c_int as libc::c_uint,
      0x2020008 as libc::c_int as libc::c_uint,
      0x108 as libc::c_int as libc::c_uint,
      0x2000108 as libc::c_int as libc::c_uint,
      0x20108 as libc::c_int as libc::c_uint,
      0x2020108 as libc::c_int as libc::c_uint,
      0x200000 as libc::c_int as libc::c_uint,
      0x2200000 as libc::c_int as libc::c_uint,
      0x220000 as libc::c_int as libc::c_uint,
      0x2220000 as libc::c_int as libc::c_uint,
      0x200100 as libc::c_int as libc::c_uint,
      0x2200100 as libc::c_int as libc::c_uint,
      0x220100 as libc::c_int as libc::c_uint,
      0x2220100 as libc::c_int as libc::c_uint,
      0x200008 as libc::c_int as libc::c_uint,
      0x2200008 as libc::c_int as libc::c_uint,
      0x220008 as libc::c_int as libc::c_uint,
      0x2220008 as libc::c_int as libc::c_uint,
      0x200108 as libc::c_int as libc::c_uint,
      0x2200108 as libc::c_int as libc::c_uint,
      0x220108 as libc::c_int as libc::c_uint,
      0x2220108 as libc::c_int as libc::c_uint,
      0x200 as libc::c_int as libc::c_uint,
      0x2000200 as libc::c_int as libc::c_uint,
      0x20200 as libc::c_int as libc::c_uint,
      0x2020200 as libc::c_int as libc::c_uint,
      0x300 as libc::c_int as libc::c_uint,
      0x2000300 as libc::c_int as libc::c_uint,
      0x20300 as libc::c_int as libc::c_uint,
      0x2020300 as libc::c_int as libc::c_uint,
      0x208 as libc::c_int as libc::c_uint,
      0x2000208 as libc::c_int as libc::c_uint,
      0x20208 as libc::c_int as libc::c_uint,
      0x2020208 as libc::c_int as libc::c_uint,
      0x308 as libc::c_int as libc::c_uint,
      0x2000308 as libc::c_int as libc::c_uint,
      0x20308 as libc::c_int as libc::c_uint,
      0x2020308 as libc::c_int as libc::c_uint,
      0x200200 as libc::c_int as libc::c_uint,
      0x2200200 as libc::c_int as libc::c_uint,
      0x220200 as libc::c_int as libc::c_uint,
      0x2220200 as libc::c_int as libc::c_uint,
      0x200300 as libc::c_int as libc::c_uint,
      0x2200300 as libc::c_int as libc::c_uint,
      0x220300 as libc::c_int as libc::c_uint,
      0x2220300 as libc::c_int as libc::c_uint,
      0x200208 as libc::c_int as libc::c_uint,
      0x2200208 as libc::c_int as libc::c_uint,
      0x220208 as libc::c_int as libc::c_uint,
      0x2220208 as libc::c_int as libc::c_uint,
      0x200308 as libc::c_int as libc::c_uint,
      0x2200308 as libc::c_int as libc::c_uint,
      0x220308 as libc::c_int as libc::c_uint,
      0x2220308 as libc::c_int as libc::c_uint],
     [0 as libc::c_int as libc::c_uint, 0x1000 as libc::c_int as libc::c_uint,
      0x20 as libc::c_int as libc::c_uint,
      0x1020 as libc::c_int as libc::c_uint,
      0x100000 as libc::c_int as libc::c_uint,
      0x101000 as libc::c_int as libc::c_uint,
      0x100020 as libc::c_int as libc::c_uint,
      0x101020 as libc::c_int as libc::c_uint,
      0x8000000 as libc::c_int as libc::c_uint,
      0x8001000 as libc::c_int as libc::c_uint,
      0x8000020 as libc::c_int as libc::c_uint,
      0x8001020 as libc::c_int as libc::c_uint,
      0x8100000 as libc::c_int as libc::c_uint,
      0x8101000 as libc::c_int as libc::c_uint,
      0x8100020 as libc::c_int as libc::c_uint,
      0x8101020 as libc::c_int as libc::c_uint,
      0x4 as libc::c_int as libc::c_uint,
      0x1004 as libc::c_int as libc::c_uint,
      0x24 as libc::c_int as libc::c_uint,
      0x1024 as libc::c_int as libc::c_uint,
      0x100004 as libc::c_int as libc::c_uint,
      0x101004 as libc::c_int as libc::c_uint,
      0x100024 as libc::c_int as libc::c_uint,
      0x101024 as libc::c_int as libc::c_uint,
      0x8000004 as libc::c_int as libc::c_uint,
      0x8001004 as libc::c_int as libc::c_uint,
      0x8000024 as libc::c_int as libc::c_uint,
      0x8001024 as libc::c_int as libc::c_uint,
      0x8100004 as libc::c_int as libc::c_uint,
      0x8101004 as libc::c_int as libc::c_uint,
      0x8100024 as libc::c_int as libc::c_uint,
      0x8101024 as libc::c_int as libc::c_uint,
      0x400 as libc::c_int as libc::c_uint,
      0x1400 as libc::c_int as libc::c_uint,
      0x420 as libc::c_int as libc::c_uint,
      0x1420 as libc::c_int as libc::c_uint,
      0x100400 as libc::c_int as libc::c_uint,
      0x101400 as libc::c_int as libc::c_uint,
      0x100420 as libc::c_int as libc::c_uint,
      0x101420 as libc::c_int as libc::c_uint,
      0x8000400 as libc::c_int as libc::c_uint,
      0x8001400 as libc::c_int as libc::c_uint,
      0x8000420 as libc::c_int as libc::c_uint,
      0x8001420 as libc::c_int as libc::c_uint,
      0x8100400 as libc::c_int as libc::c_uint,
      0x8101400 as libc::c_int as libc::c_uint,
      0x8100420 as libc::c_int as libc::c_uint,
      0x8101420 as libc::c_int as libc::c_uint,
      0x404 as libc::c_int as libc::c_uint,
      0x1404 as libc::c_int as libc::c_uint,
      0x424 as libc::c_int as libc::c_uint,
      0x1424 as libc::c_int as libc::c_uint,
      0x100404 as libc::c_int as libc::c_uint,
      0x101404 as libc::c_int as libc::c_uint,
      0x100424 as libc::c_int as libc::c_uint,
      0x101424 as libc::c_int as libc::c_uint,
      0x8000404 as libc::c_int as libc::c_uint,
      0x8001404 as libc::c_int as libc::c_uint,
      0x8000424 as libc::c_int as libc::c_uint,
      0x8001424 as libc::c_int as libc::c_uint,
      0x8100404 as libc::c_int as libc::c_uint,
      0x8101404 as libc::c_int as libc::c_uint,
      0x8100424 as libc::c_int as libc::c_uint,
      0x8101424 as libc::c_int as libc::c_uint],
     [0 as libc::c_int as libc::c_uint,
      0x10000000 as libc::c_int as libc::c_uint,
      0x10000 as libc::c_int as libc::c_uint,
      0x10010000 as libc::c_int as libc::c_uint,
      0x2 as libc::c_int as libc::c_uint,
      0x10000002 as libc::c_int as libc::c_uint,
      0x10002 as libc::c_int as libc::c_uint,
      0x10010002 as libc::c_int as libc::c_uint,
      0x2000 as libc::c_int as libc::c_uint,
      0x10002000 as libc::c_int as libc::c_uint,
      0x12000 as libc::c_int as libc::c_uint,
      0x10012000 as libc::c_int as libc::c_uint,
      0x2002 as libc::c_int as libc::c_uint,
      0x10002002 as libc::c_int as libc::c_uint,
      0x12002 as libc::c_int as libc::c_uint,
      0x10012002 as libc::c_int as libc::c_uint,
      0x40000 as libc::c_int as libc::c_uint,
      0x10040000 as libc::c_int as libc::c_uint,
      0x50000 as libc::c_int as libc::c_uint,
      0x10050000 as libc::c_int as libc::c_uint,
      0x40002 as libc::c_int as libc::c_uint,
      0x10040002 as libc::c_int as libc::c_uint,
      0x50002 as libc::c_int as libc::c_uint,
      0x10050002 as libc::c_int as libc::c_uint,
      0x42000 as libc::c_int as libc::c_uint,
      0x10042000 as libc::c_int as libc::c_uint,
      0x52000 as libc::c_int as libc::c_uint,
      0x10052000 as libc::c_int as libc::c_uint,
      0x42002 as libc::c_int as libc::c_uint,
      0x10042002 as libc::c_int as libc::c_uint,
      0x52002 as libc::c_int as libc::c_uint,
      0x10052002 as libc::c_int as libc::c_uint,
      0x20000000 as libc::c_int as libc::c_uint,
      0x30000000 as libc::c_int as libc::c_uint,
      0x20010000 as libc::c_int as libc::c_uint,
      0x30010000 as libc::c_int as libc::c_uint,
      0x20000002 as libc::c_int as libc::c_uint,
      0x30000002 as libc::c_int as libc::c_uint,
      0x20010002 as libc::c_int as libc::c_uint,
      0x30010002 as libc::c_int as libc::c_uint,
      0x20002000 as libc::c_int as libc::c_uint,
      0x30002000 as libc::c_int as libc::c_uint,
      0x20012000 as libc::c_int as libc::c_uint,
      0x30012000 as libc::c_int as libc::c_uint,
      0x20002002 as libc::c_int as libc::c_uint,
      0x30002002 as libc::c_int as libc::c_uint,
      0x20012002 as libc::c_int as libc::c_uint,
      0x30012002 as libc::c_int as libc::c_uint,
      0x20040000 as libc::c_int as libc::c_uint,
      0x30040000 as libc::c_int as libc::c_uint,
      0x20050000 as libc::c_int as libc::c_uint,
      0x30050000 as libc::c_int as libc::c_uint,
      0x20040002 as libc::c_int as libc::c_uint,
      0x30040002 as libc::c_int as libc::c_uint,
      0x20050002 as libc::c_int as libc::c_uint,
      0x30050002 as libc::c_int as libc::c_uint,
      0x20042000 as libc::c_int as libc::c_uint,
      0x30042000 as libc::c_int as libc::c_uint,
      0x20052000 as libc::c_int as libc::c_uint,
      0x30052000 as libc::c_int as libc::c_uint,
      0x20042002 as libc::c_int as libc::c_uint,
      0x30042002 as libc::c_int as libc::c_uint,
      0x20052002 as libc::c_int as libc::c_uint,
      0x30052002 as libc::c_int as libc::c_uint],
     [0 as libc::c_int as libc::c_uint,
      0x4000000 as libc::c_int as libc::c_uint,
      0x1 as libc::c_int as libc::c_uint,
      0x4000001 as libc::c_int as libc::c_uint,
      0x1000000 as libc::c_int as libc::c_uint,
      0x5000000 as libc::c_int as libc::c_uint,
      0x1000001 as libc::c_int as libc::c_uint,
      0x5000001 as libc::c_int as libc::c_uint,
      0x10 as libc::c_int as libc::c_uint,
      0x4000010 as libc::c_int as libc::c_uint,
      0x11 as libc::c_int as libc::c_uint,
      0x4000011 as libc::c_int as libc::c_uint,
      0x1000010 as libc::c_int as libc::c_uint,
      0x5000010 as libc::c_int as libc::c_uint,
      0x1000011 as libc::c_int as libc::c_uint,
      0x5000011 as libc::c_int as libc::c_uint,
      0x80000 as libc::c_int as libc::c_uint,
      0x4080000 as libc::c_int as libc::c_uint,
      0x80001 as libc::c_int as libc::c_uint,
      0x4080001 as libc::c_int as libc::c_uint,
      0x1080000 as libc::c_int as libc::c_uint,
      0x5080000 as libc::c_int as libc::c_uint,
      0x1080001 as libc::c_int as libc::c_uint,
      0x5080001 as libc::c_int as libc::c_uint,
      0x80010 as libc::c_int as libc::c_uint,
      0x4080010 as libc::c_int as libc::c_uint,
      0x80011 as libc::c_int as libc::c_uint,
      0x4080011 as libc::c_int as libc::c_uint,
      0x1080010 as libc::c_int as libc::c_uint,
      0x5080010 as libc::c_int as libc::c_uint,
      0x1080011 as libc::c_int as libc::c_uint,
      0x5080011 as libc::c_int as libc::c_uint,
      0x800 as libc::c_int as libc::c_uint,
      0x4000800 as libc::c_int as libc::c_uint,
      0x801 as libc::c_int as libc::c_uint,
      0x4000801 as libc::c_int as libc::c_uint,
      0x1000800 as libc::c_int as libc::c_uint,
      0x5000800 as libc::c_int as libc::c_uint,
      0x1000801 as libc::c_int as libc::c_uint,
      0x5000801 as libc::c_int as libc::c_uint,
      0x810 as libc::c_int as libc::c_uint,
      0x4000810 as libc::c_int as libc::c_uint,
      0x811 as libc::c_int as libc::c_uint,
      0x4000811 as libc::c_int as libc::c_uint,
      0x1000810 as libc::c_int as libc::c_uint,
      0x5000810 as libc::c_int as libc::c_uint,
      0x1000811 as libc::c_int as libc::c_uint,
      0x5000811 as libc::c_int as libc::c_uint,
      0x80800 as libc::c_int as libc::c_uint,
      0x4080800 as libc::c_int as libc::c_uint,
      0x80801 as libc::c_int as libc::c_uint,
      0x4080801 as libc::c_int as libc::c_uint,
      0x1080800 as libc::c_int as libc::c_uint,
      0x5080800 as libc::c_int as libc::c_uint,
      0x1080801 as libc::c_int as libc::c_uint,
      0x5080801 as libc::c_int as libc::c_uint,
      0x80810 as libc::c_int as libc::c_uint,
      0x4080810 as libc::c_int as libc::c_uint,
      0x80811 as libc::c_int as libc::c_uint,
      0x4080811 as libc::c_int as libc::c_uint,
      0x1080810 as libc::c_int as libc::c_uint,
      0x5080810 as libc::c_int as libc::c_uint,
      0x1080811 as libc::c_int as libc::c_uint,
      0x5080811 as libc::c_int as libc::c_uint]];
/* f_sched.c */
/*
 * Permute the key to give us our key schedule.
 */
#[no_mangle]
#[c2rust::src_loc = "241:1"]
pub unsafe extern "C" fn mit_des_make_key_sched(mut key: *mut libc::c_uchar,
                                                mut schedule:
                                                    *mut des_ks_struct)
 -> libc::c_int {
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    /*
         * Need a pointer for the keys and a temporary DES_INT32
         */
    let mut k: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut tmp: libc::c_uint = 0;
    /*
         * Fetch the key into something we can work with
         */
    k = key as *const libc::c_uchar;
    /*
         * The first permutted choice gives us the 28 bits for C0 and
         * 28 for D0.  C0 gets 12 bits from the left key and 16 from
         * the right, while D0 gets 16 from the left and 12 from the
         * right.  The code knows which bits go where.
         */
    tmp = load_32_be(k as *const libc::c_void);
    k = k.offset(4 as libc::c_int as isize);
    c =
        PC1_CL[(tmp >> 29 as libc::c_int & 0x7 as libc::c_int as libc::c_uint)
                   as usize] |
            PC1_CL[(tmp >> 21 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint) as usize] <<
                1 as libc::c_int |
            PC1_CL[(tmp >> 13 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint) as usize] <<
                2 as libc::c_int |
            PC1_CL[(tmp >> 5 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint) as usize] <<
                3 as libc::c_int;
    d =
        PC1_DL[(tmp >> 25 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
                   as usize] |
            PC1_DL[(tmp >> 17 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint) as usize] <<
                1 as libc::c_int |
            PC1_DL[(tmp >> 9 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint) as usize] <<
                2 as libc::c_int |
            PC1_DL[(tmp >> 1 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint) as usize] <<
                3 as libc::c_int;
    tmp = load_32_be(k as *const libc::c_void);
    k = k.offset(4 as libc::c_int as isize);
    c |=
        PC1_CR[(tmp >> 28 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
                   as usize] |
            PC1_CR[(tmp >> 20 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint) as usize] <<
                1 as libc::c_int |
            PC1_CR[(tmp >> 12 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint) as usize] <<
                2 as libc::c_int |
            PC1_CR[(tmp >> 4 as libc::c_int &
                        0xf as libc::c_int as libc::c_uint) as usize] <<
                3 as libc::c_int;
    d |=
        PC1_DR[(tmp >> 25 as libc::c_int & 0x7 as libc::c_int as libc::c_uint)
                   as usize] |
            PC1_DR[(tmp >> 17 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint) as usize] <<
                1 as libc::c_int |
            PC1_DR[(tmp >> 9 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint) as usize] <<
                2 as libc::c_int |
            PC1_DR[(tmp >> 1 as libc::c_int &
                        0x7 as libc::c_int as libc::c_uint) as usize] <<
                3 as libc::c_int;
    /*
         * Need several temporaries in here
         */
    let mut ltmp: libc::c_uint = 0;
    let mut rtmp: libc::c_uint = 0;
    let mut k_0: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut two_bit_shifts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /*
         * Now iterate to compute the key schedule.  Note that we
         * record the entire set of subkeys in 6 bit chunks since
         * they are used that way.  At 6 bits/char, we need
         * 48/6 char's/subkey * 16 subkeys/encryption == 128 bytes.
         * The schedule must be this big.
         */
    k_0 = schedule as *mut libc::c_uint;
    two_bit_shifts = 0x7efc as libc::c_int;
    i = 16 as libc::c_int;
    while i > 0 as libc::c_int {
        /*
             * Do the rotation.  One bit and two bit rotations
             * are done separately.  Note C and D are 28 bits.
             */
        if two_bit_shifts & 0x1 as libc::c_int != 0 {
            c =
                c << 2 as libc::c_int &
                    0xffffffc as libc::c_int as libc::c_uint |
                    c >> 26 as libc::c_int;
            d =
                d << 2 as libc::c_int &
                    0xffffffc as libc::c_int as libc::c_uint |
                    d >> 26 as libc::c_int
        } else {
            c =
                c << 1 as libc::c_int &
                    0xffffffe as libc::c_int as libc::c_uint |
                    c >> 27 as libc::c_int;
            d =
                d << 1 as libc::c_int &
                    0xffffffe as libc::c_int as libc::c_uint |
                    d >> 27 as libc::c_int
        }
        two_bit_shifts >>= 1 as libc::c_int;
        /*
             * Apply permutted choice 2 to C to get the first
             * 24 bits worth of keys.  Note that bits 9, 18, 22
             * and 25 (using DES numbering) in C are unused.  The
             * shift-mask stuff is done to delete these bits from
             * the indices, since this cuts the table size in half.
             *
             * The table is torqued, by the way.  If the standard
             * byte order for this (high to low order) is 1234,
             * the table actually gives us 4132.
             */
        ltmp =
            PC2_C[0 as libc::c_int as
                      usize][(c >> 22 as libc::c_int &
                                  0x3f as libc::c_int as libc::c_uint) as
                                 usize] |
                PC2_C[1 as libc::c_int as
                          usize][(c >> 15 as libc::c_int &
                                      0xf as libc::c_int as libc::c_uint |
                                      c >> 16 as libc::c_int &
                                          0x30 as libc::c_int as libc::c_uint)
                                     as usize] |
                PC2_C[2 as libc::c_int as
                          usize][(c >> 4 as libc::c_int &
                                      0x3 as libc::c_int as libc::c_uint |
                                      c >> 9 as libc::c_int &
                                          0x3c as libc::c_int as libc::c_uint)
                                     as usize] |
                PC2_C[3 as libc::c_int as
                          usize][(c & 0x7 as libc::c_int as libc::c_uint |
                                      c >> 4 as libc::c_int &
                                          0x38 as libc::c_int as libc::c_uint)
                                     as usize];
        /*
             * Apply permutted choice 2 to D to get the other half.
             * Here, bits 7, 10, 15 and 26 go unused.  The sqeezing
             * actually turns out to be cheaper here.
             *
             * This table is similarly torqued.  If the standard
             * byte order is 5678, the table has the bytes permuted
             * to give us 7685.
             */
        rtmp =
            PC2_D[0 as libc::c_int as
                      usize][(d >> 22 as libc::c_int &
                                  0x3f as libc::c_int as libc::c_uint) as
                                 usize] |
                PC2_D[1 as libc::c_int as
                          usize][(d >> 14 as libc::c_int &
                                      0xf as libc::c_int as libc::c_uint |
                                      d >> 15 as libc::c_int &
                                          0x30 as libc::c_int as libc::c_uint)
                                     as usize] |
                PC2_D[2 as libc::c_int as
                          usize][(d >> 7 as libc::c_int &
                                      0x3f as libc::c_int as libc::c_uint) as
                                     usize] |
                PC2_D[3 as libc::c_int as
                          usize][(d & 0x3 as libc::c_int as libc::c_uint |
                                      d >> 1 as libc::c_int &
                                          0x3c as libc::c_int as libc::c_uint)
                                     as usize];
        /*
             * Make up two words of the key schedule, with a
             * byte order which is convenient for the DES
             * inner loop.  The high order (first) word will
             * hold bytes 7135 (high to low order) while the
             * second holds bytes 4682.
             */
        let fresh0 = k_0;
        k_0 = k_0.offset(1);
        *fresh0 =
            ltmp & 0xffff00 as libc::c_int as libc::c_uint |
                rtmp & 0xff0000ff as libc::c_uint;
        let fresh1 = k_0;
        k_0 = k_0.offset(1);
        *fresh1 =
            ltmp & 0xff0000ff as libc::c_uint |
                rtmp & 0xffff00 as libc::c_int as libc::c_uint;
        i -= 1
    }
    return 0 as libc::c_int;
}
