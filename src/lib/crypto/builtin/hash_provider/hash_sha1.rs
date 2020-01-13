use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
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
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:28"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
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
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/sha1/shs.h:29"]
pub mod shs_h {
    #[c2rust::src_loc = "11:1"]
    pub type SHS_LONG = krb5_ui_4;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:9"]
    pub struct SHS_INFO {
        pub digest: [SHS_LONG; 5],
        pub countLo: SHS_LONG,
        pub countHi: SHS_LONG,
        pub data: [SHS_LONG; 16],
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* Some useful types */
    #[c2rust::src_loc = "10:1"]
    pub type SHS_BYTE = krb5_octet;
    use super::krb5_h::{krb5_ui_4, krb5_octet};
    extern "C" {
        /* Message digest functions (shs.c) */
        #[no_mangle]
        #[c2rust::src_loc = "30:1"]
        pub fn shsInit(shsInfo: *mut SHS_INFO);
        #[no_mangle]
        #[c2rust::src_loc = "31:1"]
        pub fn shsUpdate(shsInfo: *mut SHS_INFO, buffer: *const SHS_BYTE,
                         count: libc::c_uint);
        #[no_mangle]
        #[c2rust::src_loc = "32:1"]
        pub fn shsFinal(shsInfo: *mut SHS_INFO);
    }
    /* _SHS_DEFINED */
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:28"]
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
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, store_32_be};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_cryptotype,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::crypto_int_h::krb5_hash_provider;
pub use self::shs_h::{SHS_LONG, SHS_INFO, SHS_BYTE, shsInit, shsUpdate,
                      shsFinal};
pub use self::byteswap_h::__bswap_32;
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
unsafe extern "C" fn k5_sha1_hash(mut data: *const krb5_crypto_iov,
                                  mut num_data: size_t,
                                  mut output: *mut krb5_data)
 -> krb5_error_code {
    let mut ctx: SHS_INFO =
        SHS_INFO{digest: [0; 5], countLo: 0, countHi: 0, data: [0; 16],};
    let mut i: libc::c_uint = 0;
    if (*output).length != 20 as libc::c_int as libc::c_uint {
        return -(1765328206 as libc::c_long) as krb5_error_code
    }
    shsInit(&mut ctx);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < num_data {
        let mut iov: *const krb5_crypto_iov =
            &*data.offset(i as isize) as *const krb5_crypto_iov;
        if (*iov).flags == 1 as libc::c_int ||
               ((*iov).flags == 2 as libc::c_int ||
                    (*iov).flags == 4 as libc::c_int) ||
               (*iov).flags == 3 as libc::c_int {
            shsUpdate(&mut ctx, (*iov).data.data as *mut libc::c_uchar,
                      (*iov).data.length);
        }
        i = i.wrapping_add(1)
    }
    shsFinal(&mut ctx);
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[SHS_LONG; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<SHS_LONG>()
                                                   as libc::c_ulong) {
        store_32_be(ctx.digest[i as usize],
                    &mut *(*output).data.offset(i.wrapping_mul(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                                    as isize) as
                        *mut libc::c_char as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "57:33"]
pub static mut krb5int_hash_sha1: krb5_hash_provider =
    unsafe {
        {
            let mut init =
                krb5_hash_provider{hash_name: [83, 72, 65, 49, 0, 0, 0, 0],
                                   hashsize: 20 as libc::c_int as size_t,
                                   blocksize: 64 as libc::c_int as size_t,
                                   hash:
                                       Some(k5_sha1_hash as
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
