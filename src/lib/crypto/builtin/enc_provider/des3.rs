use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
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
    use super::types_h::__uint8_t;
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
    #[c2rust::src_loc = "183:1"]
    pub type krb5_cryptotype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
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
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    /* *
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
    #[c2rust::src_loc = "379:1"]
    pub type krb5_key = *mut krb5_key_st;
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
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::krb5_key_st;
    /* *< @ref KRB5_CRYPTO_TYPE type of the iov */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:28"]
pub mod k5_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "628:8"]
    pub struct krb5_key_st {
        pub keyblock: krb5_keyblock,
        pub refcount: libc::c_int,
        pub derived: *mut derived_key,
        pub cache: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "621:8"]
    pub struct derived_key {
        pub constant: krb5_data,
        pub dkey: krb5_key,
        pub next: *mut derived_key,
    }
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key};
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:28"]
pub mod crypto_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct krb5_keytypes {
        pub etype: krb5_enctype,
        pub name: *mut libc::c_char,
        pub aliases: [*mut libc::c_char; 2],
        pub out_string: *mut libc::c_char,
        pub enc: *const krb5_enc_provider,
        pub hash: *const krb5_hash_provider,
        pub prf_length: size_t,
        pub crypto_length: crypto_length_func,
        pub encrypt: crypt_func,
        pub decrypt: crypt_func,
        pub str2key: str2key_func,
        pub rand2key: rand2key_func,
        pub prf: prf_func,
        pub required_ctype: krb5_cksumtype,
        pub flags: krb5_flags,
        pub ssf: libc::c_uint,
    }
    #[c2rust::src_loc = "94:1"]
    pub type prf_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes, _: krb5_key,
                                    _: *const krb5_data, _: *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "91:1"]
    pub type rand2key_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_data,
                                    _: *mut krb5_keyblock)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "85:1"]
    pub type str2key_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *const krb5_data,
                                    _: *mut krb5_keyblock)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "80:1"]
    pub type crypt_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes, _: krb5_key,
                                    _: krb5_keyusage, _: *const krb5_data,
                                    _: *mut krb5_crypto_iov, _: size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "77:1"]
    pub type crypto_length_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes,
                                    _: krb5_cryptotype) -> libc::c_uint>;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:8"]
    pub struct krb5_enc_provider {
        pub block_size: size_t,
        pub keybytes: size_t,
        pub keylength: size_t,
        pub encrypt: Option<unsafe extern "C" fn(_: krb5_key,
                                                 _: *const krb5_data,
                                                 _: *mut krb5_crypto_iov,
                                                 _: size_t)
                                -> krb5_error_code>,
        pub decrypt: Option<unsafe extern "C" fn(_: krb5_key,
                                                 _: *const krb5_data,
                                                 _: *mut krb5_crypto_iov,
                                                 _: size_t)
                                -> krb5_error_code>,
        pub cbc_mac: Option<unsafe extern "C" fn(_: krb5_key,
                                                 _: *const krb5_crypto_iov,
                                                 _: size_t,
                                                 _: *const krb5_data,
                                                 _: *mut krb5_data)
                                -> krb5_error_code>,
        pub init_state: Option<unsafe extern "C" fn(_: *const krb5_keyblock,
                                                    _: krb5_keyusage,
                                                    _: *mut krb5_data)
                                   -> krb5_error_code>,
        pub free_state: Option<unsafe extern "C" fn(_: *mut krb5_data) -> ()>,
        pub key_cleanup: Option<unsafe extern "C" fn(_: krb5_key) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "160:8"]
    pub struct krb5_cksumtypes {
        pub ctype: krb5_cksumtype,
        pub name: *mut libc::c_char,
        pub aliases: [*mut libc::c_char; 2],
        pub out_string: *mut libc::c_char,
        pub enc: *const krb5_enc_provider,
        pub hash: *const krb5_hash_provider,
        pub checksum: checksum_func,
        pub verify: verify_func,
        pub compute_size: libc::c_uint,
        pub output_size: libc::c_uint,
        pub flags: krb5_flags,
    }
    #[c2rust::src_loc = "153:1"]
    pub type verify_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_cksumtypes, _: krb5_key,
                                    _: krb5_keyusage,
                                    _: *const krb5_crypto_iov, _: size_t,
                                    _: *const krb5_data, _: *mut krb5_boolean)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "140:1"]
    pub type checksum_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_cksumtypes, _: krb5_key,
                                    _: krb5_keyusage,
                                    _: *const krb5_crypto_iov, _: size_t,
                                    _: *mut krb5_data) -> krb5_error_code>;
    #[inline]
    #[c2rust::src_loc = "600:1"]
    pub unsafe extern "C" fn iov_total_length(mut data:
                                                  *const krb5_crypto_iov,
                                              mut num_data: size_t,
                                              mut signing: krb5_boolean)
     -> size_t {
        let mut i: size_t = 0;
        let mut total: size_t = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < num_data {
            if if signing != 0 {
                   ((*data.offset(i as isize)).flags == 1 as libc::c_int ||
                        ((*data.offset(i as isize)).flags == 2 as libc::c_int
                             ||
                             (*data.offset(i as isize)).flags ==
                                 4 as libc::c_int) ||
                        (*data.offset(i as isize)).flags == 3 as libc::c_int)
                       as libc::c_int
               } else {
                   ((*data.offset(i as isize)).flags == 1 as libc::c_int ||
                        ((*data.offset(i as isize)).flags == 2 as libc::c_int
                             ||
                             (*data.offset(i as isize)).flags ==
                                 4 as libc::c_int)) as libc::c_int
               } != 0 {
                total =
                    (total as
                         libc::c_ulong).wrapping_add((*data.offset(i as
                                                                       isize)).data.length
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
            i = i.wrapping_add(1)
        }
        return total;
    }
    use super::krb5_h::{krb5_enctype, krb5_cksumtype, krb5_flags,
                        krb5_error_code, krb5_key, krb5_data, krb5_keyblock,
                        krb5_keyusage, krb5_crypto_iov, krb5_cryptotype,
                        krb5_boolean};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn krb5int_default_free_state(state: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn krb5int_des_init_state(key: *const krb5_keyblock,
                                      keyusage: krb5_keyusage,
                                      state_out: *mut krb5_data)
         -> krb5_error_code;
    }
    /* CRYPTO_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:29"]
pub mod des_int_h {
    #[c2rust::src_loc = "121:1"]
    pub type mit_des3_key_schedule = [mit_des_key_schedule; 3];
    #[c2rust::src_loc = "117:1"]
    pub type mit_des_key_schedule = des_key_schedule;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct des_ks_struct {
        pub _data: [libc::c_int; 2],
    }
    #[c2rust::src_loc = "116:1"]
    pub type mit_des_cblock = des_cblock;
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
    /* Triple-DES structures */
    #[c2rust::src_loc = "120:1"]
    pub type mit_des3_cblock = [mit_des_cblock; 3];
    use super::krb5_h::krb5_crypto_iov;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub fn krb5int_des3_cbc_encrypt(data: *mut krb5_crypto_iov,
                                        num_data: libc::c_ulong,
                                        ks1: *const des_ks_struct,
                                        ks2: *const des_ks_struct,
                                        ks3: *const des_ks_struct,
                                        ivec: *mut libc::c_uchar);
        #[no_mangle]
        #[c2rust::src_loc = "236:1"]
        pub fn krb5int_des3_cbc_decrypt(data: *mut krb5_crypto_iov,
                                        num_data: libc::c_ulong,
                                        ks1: *const des_ks_struct,
                                        ks2: *const des_ks_struct,
                                        ks3: *const des_ks_struct,
                                        ivec: *mut libc::c_uchar);
        /* d3_kysched.c */
        #[no_mangle]
        #[c2rust::src_loc = "263:1"]
        pub fn mit_des3_key_sched(key: *mut mit_des_cblock,
                                  schedule: *mut mit_des_key_schedule)
         -> libc::c_int;
    }
    /*DES_INTERNAL_DEFS*/
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    }
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_cksumtype, krb5_keyusage, krb5_cryptotype,
                       krb5_flags, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key};
pub use self::crypto_int_h::{krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             krb5_hash_provider, krb5_enc_provider,
                             krb5_cksumtypes, verify_func, checksum_func,
                             iov_total_length, krb5int_default_free_state,
                             krb5int_des_init_state};
pub use self::des_int_h::{mit_des3_key_schedule, mit_des_key_schedule,
                          des_key_schedule, des_ks_struct, mit_des_cblock,
                          des_cblock, mit_des3_cblock,
                          krb5int_des3_cbc_encrypt, krb5int_des3_cbc_decrypt,
                          mit_des3_key_sched};
use self::string_h::explicit_bzero;
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
unsafe extern "C" fn validate_and_schedule(mut key: krb5_key,
                                           mut ivec: *const krb5_data,
                                           mut data: *const krb5_crypto_iov,
                                           mut num_data: size_t,
                                           mut schedule:
                                               *mut mit_des3_key_schedule)
 -> krb5_error_code {
    if (*key).keyblock.length != 24 as libc::c_int as libc::c_uint {
        return -(1765328195 as libc::c_long) as krb5_error_code
    }
    if iov_total_length(data, num_data,
                        0 as libc::c_int as
                            krb5_boolean).wrapping_rem(8 as libc::c_int as
                                                           libc::c_ulong) !=
           0 as libc::c_int as libc::c_ulong {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    if !ivec.is_null() && (*ivec).length != 8 as libc::c_int as libc::c_uint {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    match mit_des3_key_sched((*((*key).keyblock.contents as
                                    *mut mit_des3_cblock)).as_mut_ptr(),
                             (*schedule).as_mut_ptr()) {
        -1 => { return -(1765328198 as libc::c_long) as krb5_error_code }
        -2 => { return -(1765328197 as libc::c_long) as krb5_error_code }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn k5_des3_encrypt(mut key: krb5_key,
                                     mut ivec: *const krb5_data,
                                     mut data: *mut krb5_crypto_iov,
                                     mut num_data: size_t)
 -> krb5_error_code {
    let mut schedule: mit_des3_key_schedule =
        [[des_ks_struct{_data: [0; 2],}; 16]; 3];
    let mut err: krb5_error_code = 0;
    err = validate_and_schedule(key, ivec, data, num_data, &mut schedule);
    if err != 0 { return err }
    /* this has a return value, but the code always returns zero */
    krb5int_des3_cbc_encrypt(data, num_data,
                             schedule[0 as libc::c_int as usize].as_mut_ptr()
                                 as *const des_ks_struct,
                             schedule[1 as libc::c_int as usize].as_mut_ptr()
                                 as *const des_ks_struct,
                             schedule[2 as libc::c_int as usize].as_mut_ptr()
                                 as *const des_ks_struct,
                             if !ivec.is_null() {
                                 (*ivec).data as *mut libc::c_uchar
                             } else { 0 as *mut libc::c_uchar });
    explicit_bzero(schedule.as_mut_ptr() as *mut libc::c_void,
                   ::std::mem::size_of::<mit_des3_key_schedule>() as
                       libc::c_ulong);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn k5_des3_decrypt(mut key: krb5_key,
                                     mut ivec: *const krb5_data,
                                     mut data: *mut krb5_crypto_iov,
                                     mut num_data: size_t)
 -> krb5_error_code {
    let mut schedule: mit_des3_key_schedule =
        [[des_ks_struct{_data: [0; 2],}; 16]; 3];
    let mut err: krb5_error_code = 0;
    err = validate_and_schedule(key, ivec, data, num_data, &mut schedule);
    if err != 0 { return err }
    /* this has a return value, but the code always returns zero */
    krb5int_des3_cbc_decrypt(data, num_data,
                             schedule[0 as libc::c_int as usize].as_mut_ptr()
                                 as *const des_ks_struct,
                             schedule[1 as libc::c_int as usize].as_mut_ptr()
                                 as *const des_ks_struct,
                             schedule[2 as libc::c_int as usize].as_mut_ptr()
                                 as *const des_ks_struct,
                             if !ivec.is_null() {
                                 (*ivec).data as *mut libc::c_uchar
                             } else { 0 as *mut libc::c_uchar });
    explicit_bzero(schedule.as_mut_ptr() as *mut libc::c_void,
                   ::std::mem::size_of::<mit_des3_key_schedule>() as
                       libc::c_ulong);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "97:32"]
pub static mut krb5int_enc_des3: krb5_enc_provider =
    unsafe {
        {
            let mut init =
                krb5_enc_provider{block_size: 8 as libc::c_int as size_t,
                                  keybytes: 21 as libc::c_int as size_t,
                                  keylength: 24 as libc::c_int as size_t,
                                  encrypt:
                                      Some(k5_des3_encrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  decrypt:
                                      Some(k5_des3_decrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  cbc_mac: None,
                                  init_state:
                                      Some(krb5int_des_init_state as
                                               unsafe extern "C" fn(_:
                                                                        *const krb5_keyblock,
                                                                    _:
                                                                        krb5_keyusage,
                                                                    _:
                                                                        *mut krb5_data)
                                                   -> krb5_error_code),
                                  free_state:
                                      Some(krb5int_default_free_state as
                                               unsafe extern "C" fn(_:
                                                                        *mut krb5_data)
                                                   -> ()),
                                  key_cleanup: None,};
            init
        }
    };
