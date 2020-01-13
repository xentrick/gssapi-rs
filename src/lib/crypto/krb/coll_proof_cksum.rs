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
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
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
    /* ** Prototypes for enctype table functions ***/
    /* Length */
    /* Encrypt */
    /* Decrypt */
    /* String to key */
    /* Random to key */
    /* Pseudo-random function */
    /* ** Prototypes for cksumtype handler functions ***/
    /* ** Key derivation functions ***/
    /* RFC 3961 section 5.1 */
    /* NIST SP 800-108 with CMAC as PRF */
    /* NIST SP 800-108 with HMAC as PRF */
    /* ** Miscellaneous prototypes ***/
    /* nfold algorithm from RFC 3961 */
    /* Compute a CMAC checksum over data. */
    /* Translate an RFC 3961 key usage to a Microsoft RC4 usage. */
    /* Ensure library initialization has occurred. */
    /* DES default state initialization handler (used by module enc providers). */
    /* Default state cleanup handler (used by module enc providers). */
    /* ** Input/output vector processing declarations **/
    /* iov array we are iterating over */
    /* size of iov array */
    /* size of blocks we will be obtaining */
    /* should we process SIGN_ONLY blocks */
    /* read index into iov array */
    /* read index into iov contents */
    /* write index into iov array */
    /* write index into iov contents */
    /* ** Crypto module declarations ***/
    /* Modules must implement the k5_sha256() function prototyped in k5-int.h. */
    /* Modules must implement the following enc_providers and hash_providers: */
    /* Modules must implement the following functions. */
    /* Set the parity bits to the correct values in keybits. */
    /* Return true if keybits is a weak or semi-weak DES key. */
    /* Compute an HMAC using the provided hash function, key, and data, storing the
 * result into output (caller-allocated). */
    /* As above, using a keyblock as the key input. */
    /*
 * Compute the PBKDF2 (see RFC 2898) of password and salt, with the specified
 * count, using HMAC with the specified hash as the pseudo-random function,
 * storing the result into out (caller-allocated).
 */
    /* The following are used by test programs and are just handler functions from
 * the AES and Camellia enc providers. */
    /* These can be used to safely set up and tear down module global state. */
    /*
 * Modules must provide a crypto_mod.h header at the top level.  To work with
 * the default PRNG module (prng_fortuna.c), crypto_mod.h must #define or
 * prototype the following symbols:
 *
 *   aes_ctx - Stack-allocatable type for an AES-128 or AES-256 key schedule
 *   krb5int_aes_enc_key(key, keybits, ctxptr) -- initialize a key schedule
 *   krb5int_aes_enc_blk(in, out, ctxptr) -- encrypt a block
 *   SHA256_CTX - Stack-allocatable type for a SHA-256 hash state
 *   k5_sha256_init(ctxptr) - Initialize a hash state
 *   k5_sha256_update(ctxptr, data, size) -- Hash some data
 *   k5_sha256_final(ctxptr, out) -- Finalize a state, writing hash into out
 *
 * These functions must never fail on valid inputs, and contexts must remain
 * valid across forks.  If the module cannot meet those constraints, then it
 * should provide its own PRNG module and the build system should ensure that
 * it is used.
 *
 * The function symbols named above are also in the library export list (so
 * they can be used by the t_fortuna.c test code), so even if the module
 * defines them away or doesn't work with Fortuna, the module must provide
 * stubs; see stubs.c in the openssl module for examples.
 */
    /* ** PRNG module declarations ***/
    /*
 * PRNG modules must implement the following APIs from krb5.h:
 *   krb5_c_random_add_entropy
 *   krb5_c_random_make_octets
 *   krb5_c_random_os_entropy
 *
 * PRNG modules should implement these functions.  They are called from the
 * crypto library init and cleanup functions, and can be used to setup and tear
 * down static state without thread safety concerns.
 */
    /* Used by PRNG modules to gather OS entropy.  Returns true on success. */
    /* ** Inline helper functions ***/
    /* Find an enctype by number in the enctypes table. */
    /* Find a checksum type by number in the cksumtypes table. */
    #[inline]
    #[c2rust::src_loc = "551:1"]
    pub unsafe extern "C" fn find_cksumtype(mut ctype: krb5_cksumtype)
     -> *const krb5_cksumtypes {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < krb5int_cksumtypes_length {
            if (*krb5int_cksumtypes_list.as_ptr().offset(i as isize)).ctype ==
                   ctype {
                break ;
            }
            i = i.wrapping_add(1)
        }
        if i == krb5int_cksumtypes_length {
            return 0 as *const krb5_cksumtypes
        }
        return &*krb5int_cksumtypes_list.as_ptr().offset(i as isize) as
                   *const krb5_cksumtypes;
    }
    use super::krb5_h::{krb5_cksumtype, krb5_flags, krb5_error_code, krb5_key,
                        krb5_keyusage, krb5_crypto_iov, krb5_data,
                        krb5_boolean, krb5_keyblock, krb5_enctype,
                        krb5_cryptotype};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "177:37"]
        pub static krb5int_cksumtypes_list: [krb5_cksumtypes; 0];
        #[no_mangle]
        #[c2rust::src_loc = "178:21"]
        pub static krb5int_cksumtypes_length: size_t;
    }
    /* CRYPTO_INT_H */
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
pub use self::crypto_int_h::{krb5_cksumtypes, verify_func, checksum_func,
                             krb5_hash_provider, krb5_enc_provider,
                             krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             find_cksumtype, krb5int_cksumtypes_list,
                             krb5int_cksumtypes_length};
/* *
 * Test whether a checksum type is collision-proof.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is collision-proof, @c FALSE if it is not
 * collision-proof or not a valid checksum type.
 */
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
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn krb5_c_is_coll_proof_cksum(mut ctype: krb5_cksumtype)
 -> krb5_boolean {
    let mut ctp: *const krb5_cksumtypes = 0 as *const krb5_cksumtypes;
    ctp = find_cksumtype(ctype);
    return (!ctp.is_null() && (*ctp).flags & 0x2 as libc::c_int == 0) as
               libc::c_int as krb5_boolean;
}
