use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:32"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_0)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    use super::stdint_uintn_h::uint8_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:32"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/f_tables.h:33"]
pub mod f_tables_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/f_tables.h */
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
        /*
 * DES implementation donated by Dennis Ferguson
 */
        /*
 * des_tables.h - declarations to import the DES tables, used internally
 *                by some of the library routines.
 */
        /* nothing */
        /*
 * These may be declared const if you wish.  Be sure to change the
 * declarations in des_tables.c as well.
 */
        #[no_mangle]
        #[c2rust::src_loc = "43:33"]
        pub static des_IP_table: [libc::c_uint; 256];
        #[no_mangle]
        #[c2rust::src_loc = "44:33"]
        pub static des_FP_table: [libc::c_uint; 256];
        #[no_mangle]
        #[c2rust::src_loc = "45:33"]
        pub static des_SP_table: [[libc::c_uint; 64]; 8];
    }
    /* __DES_TABLES_H__ */
}
pub use self::types_h::{__uint8_t, __uint32_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be};
pub use self::krb5_h::krb5_octet;
pub use self::des_int_h::des_ks_struct;
pub use self::byteswap_h::__bswap_32;
use self::f_tables_h::{des_IP_table, des_FP_table, des_SP_table};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/des_int.h */
/*
 * Copyright 1987, 1988, 1990, 2002 by the Massachusetts Institute of
 * Technology.  All Rights Reserved.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/* Private include file for the Data Encryption Standard library. */
/* only do the whole thing once  */
/*
 * Begin "mit-des.h"
 */
/* defined(__MACH__) && defined(__APPLE__) */
/* Macro to add deprecated attribute to DES types and functions */
/* Currently only defined on macOS 10.5 and later.              */
/* crypto-block size */
/*
 * Key schedule.
 *
 * This used to be
 *
 * typedef struct des_ks_struct {
 *     union { DES_INT32 pad; des_cblock _;} __;
 * } des_key_schedule[16];
 *
 * but it would cause trouble if DES_INT32 were ever more than 4
 * bytes.  The reason is that all the encryption functions cast it to
 * (DES_INT32 *), and treat it as if it were DES_INT32[32].  If
 * 2*sizeof(DES_INT32) is ever more than sizeof(des_cblock), the
 * caller-allocated des_key_schedule will be overflowed by the key
 * scheduling functions.  We can't assume that every platform will
 * have an exact 32-bit int, and nothing should be looking inside a
 * des_key_schedule anyway.
 */
/* Triple-DES structures */
/* the first byte of the key is already in the keyblock */
/* This used to be 8*sizeof(krb5_octet) */
/* KRB5_MIT_DES__ */
/*
 * End "mit-des.h"
 */
/* afsstring2key.c */
/* f_cksum.c */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/f_cksum.c */
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
 * des_cbc_cksum.c - compute an 8 byte checksum using DES in CBC mode
 */
/*
 * This routine performs DES cipher-block-chaining checksum operation,
 * a.k.a.  Message Authentication Code.  It ALWAYS encrypts from input
 * to a single 64 bit output MAC checksum.
 *
 * The key schedule is passed as an arg, as well as the cleartext or
 * ciphertext. The cleartext and ciphertext should be in host order.
 *
 * NOTE-- the output is ALWAYS 8 bytes long.  If not enough space was
 * provided, your program will get trashed.
 *
 * The input is null padded, at the end (highest addr), to an integral
 * multiple of eight bytes.
 */
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn mit_des_cbc_cksum(mut in_0: *const krb5_octet,
                                           mut out: *mut krb5_octet,
                                           mut length: libc::c_ulong,
                                           mut schedule: *const des_ks_struct,
                                           mut ivec: *const krb5_octet)
 -> libc::c_ulong {
    let mut left: libc::c_uint = 0;
    let mut right: libc::c_uint = 0;
    let mut kp: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut ip: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut op: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut len: libc::c_int = 0;
    /*
     * Initialize left and right with the contents of the initial
     * vector.
     */
    ip = ivec;
    left = load_32_be(ip as *const libc::c_void);
    ip = ip.offset(4 as libc::c_int as isize);
    right = load_32_be(ip as *const libc::c_void);
    ip = ip.offset(4 as libc::c_int as isize);
    /*
     * Suitably initialized, now work the length down 8 bytes
     * at a time.
     */
    ip = in_0;
    len = length as libc::c_int;
    while len > 0 as libc::c_int {
        /*
         * Get more input, xor it in.  If the length is
         * greater than or equal to 8 this is straight
         * forward.  Otherwise we have to fart around.
         */
        if len >= 8 as libc::c_int {
            let mut temp: libc::c_uint = 0;
            temp = load_32_be(ip as *const libc::c_void);
            ip = ip.offset(4 as libc::c_int as isize);
            left ^= temp;
            temp = load_32_be(ip as *const libc::c_void);
            ip = ip.offset(4 as libc::c_int as isize);
            right ^= temp;
            len -= 8 as libc::c_int
        } else {
            /*
             * Oh, shoot.  We need to pad the
             * end with zeroes.  Work backwards
             * to do this.
             */
            ip = ip.offset(len as isize);
            let mut current_block_18: u64;
            match len {
                7 => {
                    ip = ip.offset(-1);
                    right ^=
                        (*ip as libc::c_uint &
                             0xff as libc::c_int as libc::c_uint) <<
                            8 as libc::c_int;
                    current_block_18 = 17209346603216611719;
                }
                6 => { current_block_18 = 17209346603216611719; }
                5 => { current_block_18 = 4975994749620060321; }
                4 => { current_block_18 = 13384921752283636185; }
                3 => { current_block_18 = 7082164433507571853; }
                2 => { current_block_18 = 10998935872428792708; }
                1 => { current_block_18 = 12017400968762035948; }
                _ => { current_block_18 = 15768484401365413375; }
            }
            match current_block_18 {
                17209346603216611719 => {
                    ip = ip.offset(-1);
                    right ^=
                        (*ip as libc::c_uint &
                             0xff as libc::c_int as libc::c_uint) <<
                            16 as libc::c_int;
                    current_block_18 = 4975994749620060321;
                }
                _ => { }
            }
            match current_block_18 {
                4975994749620060321 => {
                    ip = ip.offset(-1);
                    right ^=
                        (*ip as libc::c_uint &
                             0xff as libc::c_int as libc::c_uint) <<
                            24 as libc::c_int;
                    current_block_18 = 13384921752283636185;
                }
                _ => { }
            }
            match current_block_18 {
                13384921752283636185 => {
                    ip = ip.offset(-1);
                    left ^=
                        *ip as libc::c_uint &
                            0xff as libc::c_int as libc::c_uint;
                    current_block_18 = 7082164433507571853;
                }
                _ => { }
            }
            match current_block_18 {
                7082164433507571853 => {
                    ip = ip.offset(-1);
                    left ^=
                        (*ip as libc::c_uint &
                             0xff as libc::c_int as libc::c_uint) <<
                            8 as libc::c_int;
                    current_block_18 = 10998935872428792708;
                }
                _ => { }
            }
            match current_block_18 {
                10998935872428792708 => {
                    ip = ip.offset(-1);
                    left ^=
                        (*ip as libc::c_uint &
                             0xff as libc::c_int as libc::c_uint) <<
                            16 as libc::c_int;
                    current_block_18 = 12017400968762035948;
                }
                _ => { }
            }
            match current_block_18 {
                12017400968762035948 => {
                    ip = ip.offset(-1);
                    left ^=
                        (*ip as libc::c_uint &
                             0xff as libc::c_int as libc::c_uint) <<
                            24 as libc::c_int
                }
                _ => { }
            }
            len = 0 as libc::c_int
        }
        /*
         * Encrypt what we have
         */
        kp = schedule as *const libc::c_uint;
        let mut i: libc::c_int = 0;
        let mut temp1: libc::c_uint = 0;
        temp1 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp1 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp1 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp1 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp1 & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let fresh0 = kp;
            kp = kp.offset(1);
            temp1 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *fresh0;
            left ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh1 = kp;
            kp = kp.offset(1);
            temp1 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *fresh1;
            left ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh2 = kp;
            kp = kp.offset(1);
            temp1 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *fresh2;
            right ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh3 = kp;
            kp = kp.offset(1);
            temp1 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^
                    *fresh3;
            right ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i += 1
        }
        temp1 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp1 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp1 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp1 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp1 & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        kp = kp.offset(-((2 as libc::c_int * 16 as libc::c_int) as isize))
    }
    /*
     * Done.  Left and right have the checksum.  Put it into
     * the output.
     */
    op = out;
    store_32_be(left, op as *mut libc::c_void);
    op = op.offset(4 as libc::c_int as isize);
    store_32_be(right, op as *mut libc::c_void);
    op = op.offset(4 as libc::c_int as isize);
    /*
     * Return right.  I'll bet the MIT code returns this
     * inconsistantly (with the low order byte of the checksum
     * not always in the low order byte of the DES_INT32).  We won't.
     */
    return right as libc::c_ulong & 0xffffffff as libc::c_ulong;
}
