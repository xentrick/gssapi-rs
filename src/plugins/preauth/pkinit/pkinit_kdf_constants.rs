use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/pkinit/pkinit_kdf_constants.c */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
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
 * pkinit_kdf_test.c -- Structures and constants for implementation of
 * pkinit algorithm agility.  Includes definitions of algorithm identifiers
 * for SHA-1, SHA-256 and SHA-512.
 */
/* statically declare OID constants for all three algorithms */
#[no_mangle]
#[c2rust::src_loc = "36:18"]
pub static mut krb5_pkinit_sha1_oid: [krb5_octet; 8] =
    [0x2b as libc::c_int as krb5_octet, 0x6 as libc::c_int as krb5_octet,
     0x1 as libc::c_int as krb5_octet, 0x5 as libc::c_int as krb5_octet,
     0x2 as libc::c_int as krb5_octet, 0x3 as libc::c_int as krb5_octet,
     0x6 as libc::c_int as krb5_octet, 0x1 as libc::c_int as krb5_octet];
#[no_mangle]
#[c2rust::src_loc = "38:14"]
pub static mut krb5_pkinit_sha1_oid_len: size_t = 8 as libc::c_int as size_t;
#[no_mangle]
#[c2rust::src_loc = "39:18"]
pub static mut krb5_pkinit_sha256_oid: [krb5_octet; 8] =
    [0x2b as libc::c_int as krb5_octet, 0x6 as libc::c_int as krb5_octet,
     0x1 as libc::c_int as krb5_octet, 0x5 as libc::c_int as krb5_octet,
     0x2 as libc::c_int as krb5_octet, 0x3 as libc::c_int as krb5_octet,
     0x6 as libc::c_int as krb5_octet, 0x2 as libc::c_int as krb5_octet];
#[no_mangle]
#[c2rust::src_loc = "41:14"]
pub static mut krb5_pkinit_sha256_oid_len: size_t =
    8 as libc::c_int as size_t;
#[no_mangle]
#[c2rust::src_loc = "42:18"]
pub static mut krb5_pkinit_sha512_oid: [krb5_octet; 8] =
    [0x2b as libc::c_int as krb5_octet, 0x6 as libc::c_int as krb5_octet,
     0x1 as libc::c_int as krb5_octet, 0x5 as libc::c_int as krb5_octet,
     0x2 as libc::c_int as krb5_octet, 0x3 as libc::c_int as krb5_octet,
     0x6 as libc::c_int as krb5_octet, 0x3 as libc::c_int as krb5_octet];
#[no_mangle]
#[c2rust::src_loc = "44:14"]
pub static mut krb5_pkinit_sha512_oid_len: size_t =
    8 as libc::c_int as size_t;
#[no_mangle]
#[c2rust::src_loc = "49:13"]
pub static mut sha1_id: krb5_data =
    unsafe {
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length:
                               ::std::mem::size_of::<[krb5_octet; 8]>() as
                                   libc::c_ulong as libc::c_uint,
                           data:
                               krb5_pkinit_sha1_oid.as_ptr() as
                                   *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "50:13"]
pub static mut sha256_id: krb5_data =
    unsafe {
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length:
                               ::std::mem::size_of::<[krb5_octet; 8]>() as
                                   libc::c_ulong as libc::c_uint,
                           data:
                               krb5_pkinit_sha256_oid.as_ptr() as
                                   *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "51:13"]
pub static mut sha512_id: krb5_data =
    unsafe {
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length:
                               ::std::mem::size_of::<[krb5_octet; 8]>() as
                                   libc::c_ulong as libc::c_uint,
                           data:
                               krb5_pkinit_sha512_oid.as_ptr() as
                                   *mut libc::c_char,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "54:25"]
pub static mut supported_kdf_alg_ids: [*const krb5_data; 4] =
    unsafe {
        [&sha256_id as *const krb5_data, &sha1_id as *const krb5_data,
         &sha512_id as *const krb5_data, 0 as *const krb5_data]
    };
