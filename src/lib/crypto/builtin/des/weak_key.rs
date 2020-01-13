use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:37"]
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
    #[c2rust::src_loc = "116:1"]
    pub type mit_des_cblock = des_cblock;
    /*DES_INTERNAL_DEFS*/
}
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
pub use self::des_int_h::{des_cblock, mit_des_cblock};
use self::string_h::memcmp;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/weak_key.c */
/*
 * Copyright 1989,1990 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
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
 * Under U.S. law, this software may not be exported outside the US
 * without license from the U.S. Commerce department.
 *
 * These routines form the library interface to the DES facilities.
 *
 * Originally written 8/85 by Steve Miller, MIT Project Athena.
 */
/*
 * The following are the weak DES keys:
 */
#[c2rust::src_loc = "42:29"]
static mut weak: [mit_des_cblock; 16] =
    [[0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar],
     [0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar],
     [0x1f as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar],
     [0xe0 as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar],
     [0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar],
     [0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar],
     [0x1f as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar],
     [0xe0 as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar],
     [0x1 as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar],
     [0xe0 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar],
     [0x1f as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar],
     [0xfe as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar],
     [0x1 as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar],
     [0x1f as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0x1f as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar,
      0xe as libc::c_int as libc::c_uchar,
      0x1 as libc::c_int as libc::c_uchar],
     [0xe0 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar],
     [0xfe as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xe0 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar,
      0xfe as libc::c_int as libc::c_uchar,
      0xf1 as libc::c_int as libc::c_uchar]];
/* weak_key.c */
/*
 * mit_des_is_weak_key: returns true iff key is a [semi-]weak des key.
 *
 * Requires: key has correct odd parity.
 */
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn mit_des_is_weak_key(mut key: *mut libc::c_uchar)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut weak_p: *const mit_des_cblock = weak.as_ptr();
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[mit_des_cblock; 16]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<mit_des_cblock>()
                                                   as libc::c_ulong) {
        let fresh0 = weak_p;
        weak_p = weak_p.offset(1);
        if memcmp(fresh0 as *const libc::c_void, key as *const libc::c_void,
                  ::std::mem::size_of::<mit_des_cblock>() as libc::c_ulong) ==
               0 {
            return 1 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
