use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:13"]
pub mod des_int_h {
    #[c2rust::src_loc = "92:1"]
    pub type des_cblock = [libc::c_uchar; 8];
    #[c2rust::src_loc = "116:1"]
    pub type mit_des_cblock = des_cblock;
    /*DES_INTERNAL_DEFS*/
}
pub use self::des_int_h::{des_cblock, mit_des_cblock};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * These routines check and fix parity of encryption keys for the DES
 * algorithm.
 *
 * They are a replacement for routines in key_parity.c, that don't require
 * the table building that they do.
 *
 * Mark Eichin -- Cygnus Support
 */
/*
 * des_fixup_key_parity: Forces odd parity per byte; parity is bits
 *                       8,16,...64 in des order, implies 0, 8, 16, ...
 *                       vax order.
 */
#[no_mangle]
#[c2rust::src_loc = "24:1"]
pub unsafe extern "C" fn mit_des_fixup_key_parity(mut key:
                                                      *mut libc::c_uchar) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              ::std::mem::size_of::<mit_des_cblock>() as libc::c_ulong {
        let ref mut fresh0 = *key.offset(i as isize);
        *fresh0 =
            (*fresh0 as libc::c_int & 0xfe as libc::c_int) as libc::c_uchar;
        let ref mut fresh1 = *key.offset(i as isize);
        *fresh1 =
            (*fresh1 as libc::c_int |
                 1 as libc::c_int ^
                     (((*key.offset(i as isize) as libc::c_int &
                            ((1 as libc::c_int) << 4 as libc::c_int) -
                                1 as libc::c_int ^
                            *key.offset(i as isize) as libc::c_int >>
                                4 as libc::c_int &
                                ((1 as libc::c_int) << 4 as libc::c_int) -
                                    1 as libc::c_int) &
                           ((1 as libc::c_int) << 2 as libc::c_int) -
                               1 as libc::c_int ^
                           (*key.offset(i as isize) as libc::c_int &
                                ((1 as libc::c_int) << 4 as libc::c_int) -
                                    1 as libc::c_int ^
                                *key.offset(i as isize) as libc::c_int >>
                                    4 as libc::c_int &
                                    ((1 as libc::c_int) << 4 as libc::c_int) -
                                        1 as libc::c_int) >> 2 as libc::c_int
                               &
                               ((1 as libc::c_int) << 2 as libc::c_int) -
                                   1 as libc::c_int) &
                          ((1 as libc::c_int) << 1 as libc::c_int) -
                              1 as libc::c_int ^
                          ((*key.offset(i as isize) as libc::c_int &
                                ((1 as libc::c_int) << 4 as libc::c_int) -
                                    1 as libc::c_int ^
                                *key.offset(i as isize) as libc::c_int >>
                                    4 as libc::c_int &
                                    ((1 as libc::c_int) << 4 as libc::c_int) -
                                        1 as libc::c_int) &
                               ((1 as libc::c_int) << 2 as libc::c_int) -
                                   1 as libc::c_int ^
                               (*key.offset(i as isize) as libc::c_int &
                                    ((1 as libc::c_int) << 4 as libc::c_int) -
                                        1 as libc::c_int ^
                                    *key.offset(i as isize) as libc::c_int >>
                                        4 as libc::c_int &
                                        ((1 as libc::c_int) <<
                                             4 as libc::c_int) -
                                            1 as libc::c_int) >>
                                   2 as libc::c_int &
                                   ((1 as libc::c_int) << 2 as libc::c_int) -
                                       1 as libc::c_int) >> 1 as libc::c_int &
                              ((1 as libc::c_int) << 1 as libc::c_int) -
                                  1 as libc::c_int)) as libc::c_uchar;
        i = i.wrapping_add(1)
    };
}
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
/* f_cbc.c (used by test programs) */
/* fin_rndkey.c */
/* finish_key.c */
/* init_rkey.c */
/* key_parity.c */
/*
 * des_check_key_parity: returns true iff key has the correct des parity.
 *                       See des_fix_key_parity for the definition of
 *                       correct des parity.
 */
#[no_mangle]
#[c2rust::src_loc = "42:1"]
pub unsafe extern "C" fn mit_des_check_key_parity(mut key: *mut libc::c_uchar)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              ::std::mem::size_of::<mit_des_cblock>() as libc::c_ulong {
        if *key.offset(i as isize) as libc::c_int & 1 as libc::c_int ==
               ((0xfe as libc::c_int & *key.offset(i as isize) as libc::c_int
                     &
                     ((1 as libc::c_int) << 4 as libc::c_int) -
                         1 as libc::c_int ^
                     (0xfe as libc::c_int &
                          *key.offset(i as isize) as libc::c_int) >>
                         4 as libc::c_int &
                         ((1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int) &
                    ((1 as libc::c_int) << 2 as libc::c_int) -
                        1 as libc::c_int ^
                    (0xfe as libc::c_int &
                         *key.offset(i as isize) as libc::c_int &
                         ((1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int ^
                         (0xfe as libc::c_int &
                              *key.offset(i as isize) as libc::c_int) >>
                             4 as libc::c_int &
                             ((1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int) >> 2 as libc::c_int &
                        ((1 as libc::c_int) << 2 as libc::c_int) -
                            1 as libc::c_int) &
                   ((1 as libc::c_int) << 1 as libc::c_int) - 1 as libc::c_int
                   ^
                   ((0xfe as libc::c_int &
                         *key.offset(i as isize) as libc::c_int &
                         ((1 as libc::c_int) << 4 as libc::c_int) -
                             1 as libc::c_int ^
                         (0xfe as libc::c_int &
                              *key.offset(i as isize) as libc::c_int) >>
                             4 as libc::c_int &
                             ((1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int) &
                        ((1 as libc::c_int) << 2 as libc::c_int) -
                            1 as libc::c_int ^
                        (0xfe as libc::c_int &
                             *key.offset(i as isize) as libc::c_int &
                             ((1 as libc::c_int) << 4 as libc::c_int) -
                                 1 as libc::c_int ^
                             (0xfe as libc::c_int &
                                  *key.offset(i as isize) as libc::c_int) >>
                                 4 as libc::c_int &
                                 ((1 as libc::c_int) << 4 as libc::c_int) -
                                     1 as libc::c_int) >> 2 as libc::c_int &
                            ((1 as libc::c_int) << 2 as libc::c_int) -
                                1 as libc::c_int) >> 1 as libc::c_int &
                       ((1 as libc::c_int) << 1 as libc::c_int) -
                           1 as libc::c_int {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
