use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:47"]
pub mod des_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct des_ks_struct {
        pub _data: [libc::c_int; 2],
    }
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
pub use self::des_int_h::{des_ks_struct, mit_des_check_key_parity,
                          mit_des_is_weak_key, mit_des_make_key_sched};
/* key_sched.c */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/key_sched.c */
/*
 * Copyright 1985, 1986, 1987, 1988, 1990 by the Massachusetts Institute
 * of Technology.
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
 * This routine computes the DES key schedule given a key.  The
 * permutations and shifts have been done at compile time, resulting
 * in a direct one-step mapping from the input key to the key
 * schedule.
 *
 * Also checks parity and weak keys.
 *
 * Watch out for the subscripts -- most effectively start at 1 instead
 * of at zero.  Maybe some bugs in that area.
 *
 * In case the user wants to cache the computed key schedule, it is
 * passed as an arg.  Also implies that caller has explicit control
 * over zeroing both the key schedule and the key.
 *
 * Originally written 6/85 by Steve Miller, MIT Project Athena.
 */
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn mit_des_key_sched(mut k: *mut libc::c_uchar,
                                           mut schedule: *mut des_ks_struct)
 -> libc::c_int {
    mit_des_make_key_sched(k, schedule);
    if mit_des_check_key_parity(k) == 0 {
        /* bad parity --> return -1 */
        return -(1 as libc::c_int)
    }
    if mit_des_is_weak_key(k) != 0 { return -(2 as libc::c_int) }
    /* if key was good, return 0 */
    return 0 as libc::c_int;
}
