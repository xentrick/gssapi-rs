use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:25"]
pub mod des_int_h {
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
    #[c2rust::src_loc = "92:1"]
    pub type des_cblock = [libc::c_uchar; 8];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct des_ks_struct {
        pub _data: [libc::c_int; 2],
    }
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
    #[c2rust::src_loc = "113:1"]
    pub type des_key_schedule = [des_ks_struct; 16];
    #[c2rust::src_loc = "116:1"]
    pub type mit_des_cblock = des_cblock;
    #[c2rust::src_loc = "117:1"]
    pub type mit_des_key_schedule = des_key_schedule;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "178:1"]
        pub fn mit_des_check_key_parity(_: *mut libc::c_uchar) -> libc::c_int;
        /* weak_key.c */
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn mit_des_is_weak_key(_: *mut libc::c_uchar) -> libc::c_int;
        /* f_sched.c */
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn mit_des_make_key_sched(_: *mut libc::c_uchar,
                                      _: *mut des_ks_struct) -> libc::c_int;
    }
    /*DES_INTERNAL_DEFS*/
}
pub use self::des_int_h::{des_cblock, des_ks_struct, des_key_schedule,
                          mit_des_cblock, mit_des_key_schedule,
                          mit_des_check_key_parity, mit_des_is_weak_key,
                          mit_des_make_key_sched};
/* d3_kysched.c */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1995 by Richard P. Basch.  All Rights Reserved.
 * Copyright 1995 by Lehman Brothers, Inc.  All Rights Reserved.
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
 * the name of Richard P. Basch, Lehman Brothers and M.I.T. not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission.  Richard P. Basch,
 * Lehman Brothers and M.I.T. make no representations about the suitability
 * of this software for any purpose.  It is provided "as is" without
 * express or implied warranty.
 */
#[no_mangle]
#[c2rust::src_loc = "27:1"]
pub unsafe extern "C" fn mit_des3_key_sched(mut k: *mut mit_des_cblock,
                                            mut schedule:
                                                *mut mit_des_key_schedule)
 -> libc::c_int {
    mit_des_make_key_sched((*k.offset(0 as libc::c_int as
                                          isize)).as_mut_ptr(),
                           (*schedule.offset(0 as libc::c_int as
                                                 isize)).as_mut_ptr());
    mit_des_make_key_sched((*k.offset(1 as libc::c_int as
                                          isize)).as_mut_ptr(),
                           (*schedule.offset(1 as libc::c_int as
                                                 isize)).as_mut_ptr());
    mit_des_make_key_sched((*k.offset(2 as libc::c_int as
                                          isize)).as_mut_ptr(),
                           (*schedule.offset(2 as libc::c_int as
                                                 isize)).as_mut_ptr());
    if mit_des_check_key_parity((*k.offset(0 as libc::c_int as
                                               isize)).as_mut_ptr()) == 0 {
        /* bad parity --> return -1 */
        return -(1 as libc::c_int)
    }
    if mit_des_is_weak_key((*k.offset(0 as libc::c_int as
                                          isize)).as_mut_ptr()) != 0 {
        return -(2 as libc::c_int)
    }
    if mit_des_check_key_parity((*k.offset(1 as libc::c_int as
                                               isize)).as_mut_ptr()) == 0 {
        return -(1 as libc::c_int)
    }
    if mit_des_is_weak_key((*k.offset(1 as libc::c_int as
                                          isize)).as_mut_ptr()) != 0 {
        return -(2 as libc::c_int)
    }
    if mit_des_check_key_parity((*k.offset(2 as libc::c_int as
                                               isize)).as_mut_ptr()) == 0 {
        return -(1 as libc::c_int)
    }
    if mit_des_is_weak_key((*k.offset(2 as libc::c_int as
                                          isize)).as_mut_ptr()) != 0 {
        return -(2 as libc::c_int)
    }
    /* if key was good, return 0 */
    return 0 as libc::c_int;
}
