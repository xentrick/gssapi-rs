use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/aes/aes.h:34"]
pub mod aes_h {
    #[c2rust::src_loc = "58:1"]
    pub type aes_fret = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub struct aes_ctx {
        pub k_sch: [uint32_t; 64],
        pub n_rnd: uint32_t,
        pub n_blk: uint32_t,
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/aes/aesopt.h:34"]
pub mod aesopt_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "776:16"]
        pub static krb5int_im_tab: [[uint32_t; 256]; 4];
        #[no_mangle]
        #[c2rust::src_loc = "736:16"]
        pub static krb5int_fl_tab: [[uint32_t; 256]; 4];
        #[no_mangle]
        #[c2rust::src_loc = "713:17"]
        pub static krb5int_rcon_tab: [uint32_t; 29];
    }
    /* not currently used */
    /* not currently used */
    /* perform forward and inverse column mix operation on four bytes in long word x in */
/* parallel. NOTE: x must be a simple variable, NOT an expression in these macros.  */
    /* generic definitions of Rijndael macros that use of tables    */
    /* Set the number of columns in nc.  Note that it is important  */
/* that nc is a constant which is known at compile time if the  */
/* highest speed version of the code is needed                  */
}
pub use self::types_h::{__uint8_t, __uint16_t, __uint32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::aes_h::{aes_fret, aes_ctx};
use self::aesopt_h::{krb5int_im_tab, krb5int_fl_tab, krb5int_rcon_tab};
/*
 * Copyright (c) 2001, Dr Brian Gladman <brg@gladman.uk.net>, Worcester, UK.
 * All rights reserved.
 *
 * LICENSE TERMS
 *
 * The free distribution and use of this software in both source and binary
 * form is allowed (with or without changes) provided that:
 *
 *   1. distributions of this source code include the above copyright
 *      notice, this list of conditions and the following disclaimer;
 *
 *   2. distributions in binary form include the above copyright
 *      notice, this list of conditions and the following disclaimer
 *      in the documentation and/or other associated materials;
 *
 *   3. the copyright holder's name is not used to endorse products
 *      built using this software without specific written permission.
 *
 * DISCLAIMER
 *
 * This software is provided 'as is' with no explcit or implied warranties
 * in respect of any properties, including, but not limited to, correctness
 * and fitness for purpose.
 */
/*
 * Issue Date: 21/01/2002
 *
 * This file contains the code for implementing the key schedule for AES
 * (Rijndael) for block and key sizes of 16, 24, and 32 bytes.
 */
/* Subroutine to set the block size (if variable) in bytes, legal
   values being 16, 24 and 32.
*/
/* Initialise the key schedule from the user supplied key. The key
   length is now specified in bytes - 16, 24 or 32 as appropriate.
   This corresponds to bit lengths of 128, 192 and 256 bits, and
   to Nk values of 4, 6 and 8 respectively.

   The following macros implement a single cycle in the key
   schedule generation process. The number of cycles needed
   for each cx->n_col and nk value is:

    nk =             4  5  6  7  8
    ------------------------------
    cx->n_col = 4   10  9  8  7  7
    cx->n_col = 5   14 11 10  9  9
    cx->n_col = 6   19 15 12 11 11
    cx->n_col = 7   21 19 16 13 14
    cx->n_col = 8   29 23 19 17 14
*/
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn krb5int_aes_enc_key(mut in_key: *const libc::c_uchar,
                                             mut klen: libc::c_uint,
                                             mut cx: *mut aes_ctx)
 -> aes_fret {
    let mut ss: [uint32_t; 8] = [0; 8];
    (*cx).n_blk = 16 as libc::c_int as uint32_t;
    (*cx).n_blk =
        (*cx).n_blk & !(3 as libc::c_uint) | 1 as libc::c_int as libc::c_uint;
    ss[0 as libc::c_int as usize] =
        (*in_key.offset(3 as libc::c_int as isize) as uint32_t) <<
            24 as libc::c_int |
            (*in_key.offset(2 as libc::c_int as isize) as uint32_t) <<
                16 as libc::c_int |
            (*in_key.offset(1 as libc::c_int as isize) as uint32_t) <<
                8 as libc::c_int |
            *in_key.offset(0 as libc::c_int as isize) as libc::c_uint;
    (*cx).k_sch[0 as libc::c_int as usize] = ss[0 as libc::c_int as usize];
    ss[1 as libc::c_int as usize] =
        (*in_key.offset(4 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as
             uint32_t) << 24 as libc::c_int |
            (*in_key.offset(4 as libc::c_int as
                                isize).offset(2 as libc::c_int as isize) as
                 uint32_t) << 16 as libc::c_int |
            (*in_key.offset(4 as libc::c_int as
                                isize).offset(1 as libc::c_int as isize) as
                 uint32_t) << 8 as libc::c_int |
            *in_key.offset(4 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
                libc::c_uint;
    (*cx).k_sch[1 as libc::c_int as usize] = ss[1 as libc::c_int as usize];
    ss[2 as libc::c_int as usize] =
        (*in_key.offset(8 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as
             uint32_t) << 24 as libc::c_int |
            (*in_key.offset(8 as libc::c_int as
                                isize).offset(2 as libc::c_int as isize) as
                 uint32_t) << 16 as libc::c_int |
            (*in_key.offset(8 as libc::c_int as
                                isize).offset(1 as libc::c_int as isize) as
                 uint32_t) << 8 as libc::c_int |
            *in_key.offset(8 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
                libc::c_uint;
    (*cx).k_sch[2 as libc::c_int as usize] = ss[2 as libc::c_int as usize];
    ss[3 as libc::c_int as usize] =
        (*in_key.offset(12 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as
             uint32_t) << 24 as libc::c_int |
            (*in_key.offset(12 as libc::c_int as
                                isize).offset(2 as libc::c_int as isize) as
                 uint32_t) << 16 as libc::c_int |
            (*in_key.offset(12 as libc::c_int as
                                isize).offset(1 as libc::c_int as isize) as
                 uint32_t) << 8 as libc::c_int |
            *in_key.offset(12 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
                libc::c_uint;
    (*cx).k_sch[3 as libc::c_int as usize] = ss[3 as libc::c_int as usize];
    match klen {
        16 => {
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[3 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[4 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[5 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[6 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[7 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[8 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[9 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            (*cx).n_rnd = 10 as libc::c_int as uint32_t
        }
        24 => {
            ss[4 as libc::c_int as usize] =
                (*in_key.offset(16 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(16 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[4 as libc::c_int as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] =
                (*in_key.offset(20 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(20 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[5 as libc::c_int as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[5 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[6 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[7 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            (*cx).n_rnd = 12 as libc::c_int as uint32_t
        }
        32 => {
            ss[4 as libc::c_int as usize] =
                (*in_key.offset(16 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(16 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[4 as libc::c_int as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] =
                (*in_key.offset(20 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(20 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[5 as libc::c_int as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                (*in_key.offset(24 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(24 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(24 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(24 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[6 as libc::c_int as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] =
                (*in_key.offset(28 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(28 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(28 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(28 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[7 as libc::c_int as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             12 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             13 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             14 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             15 as libc::c_int) as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             12 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             13 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             14 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             15 as libc::c_int) as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             12 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             13 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             14 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             15 as libc::c_int) as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[3 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             12 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             13 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             14 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             15 as libc::c_int) as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             12 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             13 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             14 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             15 as libc::c_int) as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             12 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             13 as libc::c_int) as usize] =
                ss[5 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             14 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             15 as libc::c_int) as usize] =
                ss[7 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            (*cx).n_rnd = 14 as libc::c_int as uint32_t
        }
        _ => {
            (*cx).n_rnd = 0 as libc::c_int as uint32_t;
            return 0 as libc::c_int as aes_fret
        }
    }
    return 1 as libc::c_int as aes_fret;
}
/*
 * Copyright (c) 2001, Dr Brian Gladman <brg@gladman.uk.net>, Worcester, UK.
 * All rights reserved.
 *
 * LICENSE TERMS
 *
 * The free distribution and use of this software in both source and binary
 * form is allowed (with or without changes) provided that:
 *
 *   1. distributions of this source code include the above copyright
 *      notice, this list of conditions and the following disclaimer;
 *
 *   2. distributions in binary form include the above copyright
 *      notice, this list of conditions and the following disclaimer
 *      in the documentation and/or other associated materials;
 *
 *   3. the copyright holder's name is not used to endorse products
 *      built using this software without specific written permission.
 *
 * DISCLAIMER
 *
 * This software is provided 'as is' with no explcit or implied warranties
 * in respect of any properties, including, but not limited to, correctness
 * and fitness for purpose.
 */
/*
 * Issue Date: 21/01/2002
 *
 * This file contains the definitions required to use AES (Rijndael) in C.
 */
/*  BLOCK_SIZE is in BYTES: 16, 24, 32 or undefined for aes.c and 16, 20,
    24, 28, 32 or undefined for aespp.c.  When left undefined a slower
    version that provides variable block length is compiled.
*/
/* key schedule length (in 32-bit words)    */
/* type for function return value       */
/* bad function return value            */
/* good function return value           */
/* implement normal or DLL functions    */
/* the AES context for encryption   */
/* the encryption key schedule      */
/* the number of cipher rounds      */
/* the number of bytes in the state */
/* for Kerberos 5 tree -- hide names!  */
#[no_mangle]
#[c2rust::src_loc = "280:1"]
pub unsafe extern "C" fn krb5int_aes_dec_key(mut in_key: *const libc::c_uchar,
                                             mut klen: libc::c_uint,
                                             mut cx: *mut aes_ctx)
 -> aes_fret {
    let mut ss: [uint32_t; 8] = [0; 8];
    (*cx).n_blk = 16 as libc::c_int as uint32_t;
    (*cx).n_blk =
        (*cx).n_blk & !(3 as libc::c_uint) | 2 as libc::c_int as libc::c_uint;
    ss[0 as libc::c_int as usize] =
        (*in_key.offset(3 as libc::c_int as isize) as uint32_t) <<
            24 as libc::c_int |
            (*in_key.offset(2 as libc::c_int as isize) as uint32_t) <<
                16 as libc::c_int |
            (*in_key.offset(1 as libc::c_int as isize) as uint32_t) <<
                8 as libc::c_int |
            *in_key.offset(0 as libc::c_int as isize) as libc::c_uint;
    (*cx).k_sch[0 as libc::c_int as usize] = ss[0 as libc::c_int as usize];
    ss[1 as libc::c_int as usize] =
        (*in_key.offset(4 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as
             uint32_t) << 24 as libc::c_int |
            (*in_key.offset(4 as libc::c_int as
                                isize).offset(2 as libc::c_int as isize) as
                 uint32_t) << 16 as libc::c_int |
            (*in_key.offset(4 as libc::c_int as
                                isize).offset(1 as libc::c_int as isize) as
                 uint32_t) << 8 as libc::c_int |
            *in_key.offset(4 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
                libc::c_uint;
    (*cx).k_sch[1 as libc::c_int as usize] = ss[1 as libc::c_int as usize];
    ss[2 as libc::c_int as usize] =
        (*in_key.offset(8 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as
             uint32_t) << 24 as libc::c_int |
            (*in_key.offset(8 as libc::c_int as
                                isize).offset(2 as libc::c_int as isize) as
                 uint32_t) << 16 as libc::c_int |
            (*in_key.offset(8 as libc::c_int as
                                isize).offset(1 as libc::c_int as isize) as
                 uint32_t) << 8 as libc::c_int |
            *in_key.offset(8 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
                libc::c_uint;
    (*cx).k_sch[2 as libc::c_int as usize] = ss[2 as libc::c_int as usize];
    ss[3 as libc::c_int as usize] =
        (*in_key.offset(12 as libc::c_int as
                            isize).offset(3 as libc::c_int as isize) as
             uint32_t) << 24 as libc::c_int |
            (*in_key.offset(12 as libc::c_int as
                                isize).offset(2 as libc::c_int as isize) as
                 uint32_t) << 16 as libc::c_int |
            (*in_key.offset(12 as libc::c_int as
                                isize).offset(1 as libc::c_int as isize) as
                 uint32_t) << 8 as libc::c_int |
            *in_key.offset(12 as libc::c_int as
                               isize).offset(0 as libc::c_int as isize) as
                libc::c_uint;
    (*cx).k_sch[3 as libc::c_int as usize] = ss[3 as libc::c_int as usize];
    match klen {
        16 => {
            ss[0 as libc::c_int as usize] =
                ss[0 as libc::c_int as usize] ^ ss[2 as libc::c_int as usize]
                    ^ ss[1 as libc::c_int as usize] ^
                    ss[3 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] =
                ss[1 as libc::c_int as usize] ^ ss[3 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] =
                ss[2 as libc::c_int as usize] ^ ss[3 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] = ss[3 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((0 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((0 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((0 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((0 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[0 as libc::c_int as usize];
            ss[(0 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             4 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             5 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             6 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 0 as libc::c_int +
                             7 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((1 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((1 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((1 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((1 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[1 as libc::c_int as usize];
            ss[(1 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 1 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((2 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((2 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((2 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((2 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[2 as libc::c_int as usize];
            ss[(2 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 2 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((3 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((3 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((3 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((3 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[3 as libc::c_int as usize];
            ss[(3 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 3 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((4 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((4 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((4 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((4 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[4 as libc::c_int as usize];
            ss[(4 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 4 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((5 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((5 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((5 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((5 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[5 as libc::c_int as usize];
            ss[(5 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 5 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((6 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((6 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((6 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((6 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[6 as libc::c_int as usize];
            ss[(6 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 6 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((7 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((7 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((7 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((7 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[7 as libc::c_int as usize];
            ss[(7 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 7 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((8 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((8 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((8 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((8 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[8 as libc::c_int as usize];
            ss[(8 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^=
                (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(4 as libc::c_int * 8 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[4 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[((9 as libc::c_int +
                                                    3 as libc::c_int) %
                                                   4 as libc::c_int) as usize]
                                               >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[((9 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[((9 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[((9 as libc::c_int +
                                                        3 as libc::c_int) %
                                                       4 as libc::c_int) as
                                                      usize] >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[9 as libc::c_int as usize];
            ss[(9 as libc::c_int % 4 as libc::c_int) as usize] ^=
                ss[4 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             4 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize] ^ ss[2 as libc::c_int as usize]
                    ^ ss[3 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             5 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize] ^ ss[3 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            (*cx).k_sch[(4 as libc::c_int * 9 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            (*cx).n_rnd = 10 as libc::c_int as uint32_t
        }
        24 => {
            ss[4 as libc::c_int as usize] =
                (*in_key.offset(16 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(16 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[5 as libc::c_int as usize] =
                (*in_key.offset(20 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(20 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[5 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             6 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[0 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[0 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[0 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[0 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             7 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[1 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[1 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[1 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[1 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             8 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[2 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[2 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[2 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[2 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             9 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             10 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 0 as libc::c_int +
                             11 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[1 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 1 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[2 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 2 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 3 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[4 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 4 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 5 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[6 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[4 as libc::c_int as usize] ^= ss[3 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            ss[6 as libc::c_int as usize] ^=
                (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(6 as libc::c_int * 6 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[6 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[7 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             6 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             7 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(6 as libc::c_int * 7 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            (*cx).n_rnd = 12 as libc::c_int as uint32_t
        }
        32 => {
            ss[4 as libc::c_int as usize] =
                (*in_key.offset(16 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(16 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(16 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[4 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[5 as libc::c_int as usize] =
                (*in_key.offset(20 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(20 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(20 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[5 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] =
                (*in_key.offset(24 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(24 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(24 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(24 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[6 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[7 as libc::c_int as usize] =
                (*in_key.offset(28 as libc::c_int as
                                    isize).offset(3 as libc::c_int as isize)
                     as uint32_t) << 24 as libc::c_int |
                    (*in_key.offset(28 as libc::c_int as
                                        isize).offset(2 as libc::c_int as
                                                          isize) as uint32_t)
                        << 16 as libc::c_int |
                    (*in_key.offset(28 as libc::c_int as
                                        isize).offset(1 as libc::c_int as
                                                          isize) as uint32_t)
                        << 8 as libc::c_int |
                    *in_key.offset(28 as libc::c_int as
                                       isize).offset(0 as libc::c_int as
                                                         isize) as
                        libc::c_uint;
            (*cx).k_sch[7 as libc::c_int as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             8 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[0 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[0 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[0 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[0 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             9 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[1 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[1 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[1 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[1 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             10 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[2 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[2 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[2 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[2 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             11 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[4 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             12 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[4 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[4 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             13 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[5 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[5 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             14 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[6 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[6 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 0 as libc::c_int +
                             15 as libc::c_int) as usize] =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            let mut g: uint32_t =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[1 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= g;
            g =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g ^= (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             8 as libc::c_int) as usize] = g;
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             9 as libc::c_int) as usize] = g;
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             10 as libc::c_int) as usize] = g;
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             11 as libc::c_int) as usize] = g;
            g =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            ss[4 as libc::c_int as usize] ^= g;
            g =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             12 as libc::c_int) as usize] = g;
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             13 as libc::c_int) as usize] = g;
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 6 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             14 as libc::c_int) as usize] = g;
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            g ^=
                (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                                 7 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 1 as libc::c_int +
                             15 as libc::c_int) as usize] = g;
            let mut g_0: uint32_t =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[2 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= g_0;
            g_0 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_0 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_0 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_0 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_0 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             8 as libc::c_int) as usize] = g_0;
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             9 as libc::c_int) as usize] = g_0;
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             10 as libc::c_int) as usize] = g_0;
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             11 as libc::c_int) as usize] = g_0;
            g_0 =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            ss[4 as libc::c_int as usize] ^= g_0;
            g_0 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_0 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_0 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_0 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_0 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             12 as libc::c_int) as usize] = g_0;
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             13 as libc::c_int) as usize] = g_0;
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 6 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             14 as libc::c_int) as usize] = g_0;
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            g_0 ^=
                (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                                 7 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 2 as libc::c_int +
                             15 as libc::c_int) as usize] = g_0;
            let mut g_1: uint32_t =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[3 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= g_1;
            g_1 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_1 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_1 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_1 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_1 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             8 as libc::c_int) as usize] = g_1;
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             9 as libc::c_int) as usize] = g_1;
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             10 as libc::c_int) as usize] = g_1;
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             11 as libc::c_int) as usize] = g_1;
            g_1 =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            ss[4 as libc::c_int as usize] ^= g_1;
            g_1 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_1 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_1 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_1 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_1 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             12 as libc::c_int) as usize] = g_1;
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             13 as libc::c_int) as usize] = g_1;
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 6 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             14 as libc::c_int) as usize] = g_1;
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            g_1 ^=
                (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                                 7 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 3 as libc::c_int +
                             15 as libc::c_int) as usize] = g_1;
            let mut g_2: uint32_t =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[4 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= g_2;
            g_2 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_2 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_2 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_2 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_2 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             8 as libc::c_int) as usize] = g_2;
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             9 as libc::c_int) as usize] = g_2;
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             10 as libc::c_int) as usize] = g_2;
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             11 as libc::c_int) as usize] = g_2;
            g_2 =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            ss[4 as libc::c_int as usize] ^= g_2;
            g_2 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_2 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_2 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_2 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_2 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             12 as libc::c_int) as usize] = g_2;
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             13 as libc::c_int) as usize] = g_2;
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 6 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             14 as libc::c_int) as usize] = g_2;
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            g_2 ^=
                (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                                 7 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 4 as libc::c_int +
                             15 as libc::c_int) as usize] = g_2;
            let mut g_3: uint32_t =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[5 as libc::c_int as usize];
            ss[0 as libc::c_int as usize] ^= g_3;
            g_3 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_3 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_3 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_3 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_3 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             8 as libc::c_int) as usize] = g_3;
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 1 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             9 as libc::c_int) as usize] = g_3;
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 2 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             10 as libc::c_int) as usize] = g_3;
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 3 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             11 as libc::c_int) as usize] = g_3;
            g_3 =
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[3 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        0 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[3 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            0 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize];
            ss[4 as libc::c_int as usize] ^= g_3;
            g_3 =
                krb5int_im_tab[0 as libc::c_int as
                                   usize][(g_3 >>
                                               8 as libc::c_int *
                                                   0 as libc::c_int) as
                                              uint8_t as usize] ^
                    krb5int_im_tab[1 as libc::c_int as
                                       usize][(g_3 >>
                                                   8 as libc::c_int *
                                                       1 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[2 as libc::c_int as
                                       usize][(g_3 >>
                                                   8 as libc::c_int *
                                                       2 as libc::c_int) as
                                                  uint8_t as usize] ^
                    krb5int_im_tab[3 as libc::c_int as
                                       usize][(g_3 >>
                                                   8 as libc::c_int *
                                                       3 as libc::c_int) as
                                                  uint8_t as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 4 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             12 as libc::c_int) as usize] = g_3;
            ss[5 as libc::c_int as usize] ^= ss[4 as libc::c_int as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 5 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             13 as libc::c_int) as usize] = g_3;
            ss[6 as libc::c_int as usize] ^= ss[5 as libc::c_int as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 6 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             14 as libc::c_int) as usize] = g_3;
            ss[7 as libc::c_int as usize] ^= ss[6 as libc::c_int as usize];
            g_3 ^=
                (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                                 7 as libc::c_int) as usize];
            (*cx).k_sch[(8 as libc::c_int * 5 as libc::c_int +
                             15 as libc::c_int) as usize] = g_3;
            ss[0 as libc::c_int as usize] ^=
                krb5int_fl_tab[0 as libc::c_int as
                                   usize][(ss[7 as libc::c_int as usize] >>
                                               8 as libc::c_int *
                                                   (0 as libc::c_int -
                                                        3 as libc::c_int &
                                                        3 as libc::c_int)) as
                                              uint8_t as usize] ^
                    krb5int_fl_tab[1 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (1 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[2 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (2 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_fl_tab[3 as libc::c_int as
                                       usize][(ss[7 as libc::c_int as usize]
                                                   >>
                                                   8 as libc::c_int *
                                                       (3 as libc::c_int -
                                                            3 as libc::c_int &
                                                            3 as libc::c_int))
                                                  as uint8_t as usize] ^
                    krb5int_rcon_tab[6 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             8 as libc::c_int) as usize] =
                ss[0 as libc::c_int as usize];
            ss[1 as libc::c_int as usize] ^= ss[0 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             9 as libc::c_int) as usize] =
                ss[1 as libc::c_int as usize];
            ss[2 as libc::c_int as usize] ^= ss[1 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             10 as libc::c_int) as usize] =
                ss[2 as libc::c_int as usize];
            ss[3 as libc::c_int as usize] ^= ss[2 as libc::c_int as usize];
            (*cx).k_sch[(8 as libc::c_int * 6 as libc::c_int +
                             11 as libc::c_int) as usize] =
                ss[3 as libc::c_int as usize];
            (*cx).n_rnd = 14 as libc::c_int as uint32_t
        }
        _ => {
            (*cx).n_rnd = 0 as libc::c_int as uint32_t;
            return 0 as libc::c_int as aes_fret
        }
    }
    return 1 as libc::c_int as aes_fret;
}
