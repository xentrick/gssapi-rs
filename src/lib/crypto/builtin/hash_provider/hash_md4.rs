use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "183:1"]
    pub type krb5_cryptotype = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "415:16"]
    pub struct _krb5_crypto_iov {
        pub flags: krb5_cryptotype,
        pub data: krb5_data,
    }
    /* *
 * Structure to describe a region of text to be encrypted or decrypted.
 *
 * The @a flags member describes the type of the iov.
 * The @a data member points to the memory that will be manipulated.
 * All iov APIs take a pointer to the first element of an array of krb5_crypto_iov's
 * along with the size of that array. Buffer contents are manipulated in-place;
 * data is overwritten. Callers must allocate the right number of krb5_crypto_iov
 * structures before calling into an iov API.
 */
    #[c2rust::src_loc = "415:1"]
    pub type krb5_crypto_iov = _krb5_crypto_iov;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    /* *< @ref KRB5_CRYPTO_TYPE type of the iov */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:28"]
pub mod crypto_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:8"]
    pub struct krb5_hash_provider {
        pub hash_name: [libc::c_char; 8],
        pub hashsize: size_t,
        pub blocksize: size_t,
        pub hash: Option<unsafe extern "C" fn(_: *const krb5_crypto_iov,
                                              _: size_t, _: *mut krb5_data)
                             -> krb5_error_code>,
    }
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_crypto_iov, krb5_data};
    /* CRYPTO_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/md4/rsa-md4.h:29"]
pub mod rsa_md4_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:9"]
    pub struct krb5_MD4_CTX {
        pub i: [krb5_ui_4; 2],
        pub buf: [krb5_ui_4; 4],
        pub in_0: [libc::c_uchar; 64],
        pub digest: [libc::c_uchar; 16],
    }
    use super::krb5_h::krb5_ui_4;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn krb5int_MD4Init(_: *mut krb5_MD4_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn krb5int_MD4Update(_: *mut krb5_MD4_CTX,
                                 _: *const libc::c_uchar, _: libc::c_uint);
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn krb5int_MD4Final(_: *mut krb5_MD4_CTX);
    }
    /* __KRB5_RSA_MD4_H__ */
    /*
**********************************************************************
** End of md4.h                                                     **
******************************* (cut) ********************************
*/
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::krb5_h::{krb5_int32, krb5_ui_4, krb5_cryptotype,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::crypto_int_h::krb5_hash_provider;
pub use self::rsa_md4_h::{krb5_MD4_CTX, krb5int_MD4Init, krb5int_MD4Update,
                          krb5int_MD4Final};
use self::string_h::memcpy;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
#[c2rust::src_loc = "31:1"]
unsafe extern "C" fn k5_md4_hash(mut data: *const krb5_crypto_iov,
                                 mut num_data: size_t,
                                 mut output: *mut krb5_data)
 -> krb5_error_code {
    let mut ctx: krb5_MD4_CTX =
        krb5_MD4_CTX{i: [0; 2], buf: [0; 4], in_0: [0; 64], digest: [0; 16],};
    let mut i: libc::c_uint = 0;
    if (*output).length != 16 as libc::c_int as libc::c_uint {
        return -(1765328206 as libc::c_long) as krb5_error_code
    }
    krb5int_MD4Init(&mut ctx);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < num_data {
        let mut iov: *const krb5_crypto_iov =
            &*data.offset(i as isize) as *const krb5_crypto_iov;
        if (*iov).flags == 1 as libc::c_int ||
               ((*iov).flags == 2 as libc::c_int ||
                    (*iov).flags == 4 as libc::c_int) ||
               (*iov).flags == 3 as libc::c_int {
            krb5int_MD4Update(&mut ctx,
                              (*iov).data.data as *mut libc::c_uchar,
                              (*iov).data.length);
        }
        i = i.wrapping_add(1)
    }
    krb5int_MD4Final(&mut ctx);
    memcpy((*output).data as *mut libc::c_void,
           ctx.digest.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "56:33"]
pub static mut krb5int_hash_md4: krb5_hash_provider =
    unsafe {
        {
            let mut init =
                krb5_hash_provider{hash_name: [77, 68, 52, 0, 0, 0, 0, 0],
                                   hashsize: 16 as libc::c_int as size_t,
                                   blocksize: 64 as libc::c_int as size_t,
                                   hash:
                                       Some(k5_md4_hash as
                                                unsafe extern "C" fn(_:
                                                                         *const krb5_crypto_iov,
                                                                     _:
                                                                         size_t,
                                                                     _:
                                                                         *mut krb5_data)
                                                    -> krb5_error_code),};
            init
        }
    };
