use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:27"]
pub mod crypto_int_h {
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
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_key, krb5_data, krb5_crypto_iov,
                        krb5_keyblock, krb5_keyusage, krb5_enctype,
                        krb5_cksumtype, krb5_flags, krb5_cryptotype};
    use super::k5_int_h::krb5_key_st;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "439:40"]
        pub static krb5int_hash_sha1: krb5_hash_provider;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn krb5int_hmac(hash: *const krb5_hash_provider, key: krb5_key,
                            data: *const krb5_crypto_iov, num_data: size_t,
                            output: *mut krb5_data) -> krb5_error_code;
    }
    /* CRYPTO_INT_H */
}
#[c2rust::header_src = "/usr/include/assert.h:27"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_enctype, krb5_cksumtype,
                       krb5_keyusage, krb5_cryptotype, krb5_flags,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key};
pub use self::crypto_int_h::{krb5_enc_provider, krb5_hash_provider,
                             krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             krb5int_hash_sha1, krb5int_hmac};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/krb/crypto_int.h - Master libk5crypto internal header */
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
/* This header is the entry point for libk5crypto sources, and also documents
 * requirements for crypto modules and PRNG modules.  */
/* Enc providers and hash providers specify well-known ciphers and hashes to be
 * implemented by the crypto module. */
/* keybytes is the input size to make_key;
       keylength is the output size */
/* May be NULL if the cipher is not used for a cbc-mac checksum. */
/* May be NULL if there is no key-derived data cached.  */
/* ** RFC 3961 enctypes table ***/
/*
 * "Weak" means the enctype is believed to be vulnerable to practical attacks,
 * and will be disabled unless allow_weak_crypto is set to true.  "Deprecated"
 * means the enctype has been deprecated by the IETF, and affects display and
 * logging.
 */
/* ** RFC 3961 checksum types table ***/
/*
 * Compute a checksum over the header, data, padding, and sign-only fields of
 * the iov array data (of size num_data).  The output buffer will already be
 * allocated with ctp->compute_size bytes available; the handler just needs to
 * fill in the contents.  If ctp->enc is not NULL, the handler can assume that
 * key is a valid-length key of an enctype which uses that enc provider.
 */
/*
 * Verify a checksum over the header, data, padding, and sign-only fields of
 * the iov array data (of size num_data), and store the boolean result in
 * *valid.  The handler can assume that hash has length ctp->output_size.  If
 * ctp->enc is not NULL, the handler can assume that key a valid-length key of
 * an enctype which uses that enc provider.
 */
/* NULL means recompute checksum and compare */
/* Allocation size for checksum computation */
/* Possibly truncated output size */
/* ** Prototypes for enctype table functions ***/
/* Length */
/* Encrypt */
/* Decrypt */
/* String to key */
/* Random to key */
/* Pseudo-random function */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/krb/prf_rc4.c */
/*
 * Copyright (C) 2009 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5int_arcfour_prf(mut ktp: *const krb5_keytypes,
                                             mut key: krb5_key,
                                             mut in_0: *const krb5_data,
                                             mut out: *mut krb5_data)
 -> krb5_error_code {
    let mut iov: krb5_crypto_iov =
        krb5_crypto_iov{flags: 0,
                        data:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},};
    if (*out).length == 20 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"out->length == 20\x00" as *const u8 as
                          *const libc::c_char,
                      b"prf_rc4.c\x00" as *const u8 as *const libc::c_char,
                      35 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 108],
                                                &[libc::c_char; 108]>(b"krb5_error_code krb5int_arcfour_prf(const struct krb5_keytypes *, krb5_key, const krb5_data *, krb5_data *)\x00")).as_ptr());
    }
    iov.flags = 2 as libc::c_int;
    iov.data = *in_0;
    return krb5int_hmac(&krb5int_hash_sha1, key, &mut iov,
                        1 as libc::c_int as size_t, out);
}
