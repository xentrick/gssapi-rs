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
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key, krb5_magic,
                        krb5_error_code};
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::{free, calloc};
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
    use super::krb5_h::{krb5_enctype, krb5_cksumtype, krb5_flags,
                        krb5_error_code, krb5_key, krb5_data, krb5_keyblock,
                        krb5_keyusage, krb5_crypto_iov, krb5_cryptotype,
                        krb5_boolean};
    use super::stddef_h::size_t;
    /* CRYPTO_INT_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
pub use self::k5_int_h::{krb5_key_st, derived_key, zapfree, make_data,
                         k5calloc, k5alloc};
pub use self::crypto_int_h::{krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             krb5_hash_provider, krb5_enc_provider,
                             krb5_cksumtypes, verify_func, checksum_func};
use self::stdlib_h::{calloc, free};
use self::string_h::{explicit_bzero, memset, memcpy};
/* As above, using a keyblock as the key input. */
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
/*
 * Because our built-in HMAC implementation doesn't need to invoke any
 * encryption or keyed hash functions, it is simplest to define it in terms of
 * keyblocks, and then supply a simple wrapper for the "normal" krb5_key-using
 * interfaces.  The keyblock interfaces are useful for code which creates
 * intermediate keyblocks.
 */
/*
 * The HMAC transform looks like:
 *
 * H(K XOR opad, H(K XOR ipad, text))
 *
 * where H is a cryptographic hash
 * K is an n byte key
 * ipad is the byte 0x36 repeated blocksize times
 * opad is the byte 0x5c repeated blocksize times
 * and text is the data being protected
 */
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn krb5int_hmac_keyblock(mut hash:
                                                   *const krb5_hash_provider,
                                               mut keyblock:
                                                   *const krb5_keyblock,
                                               mut data:
                                                   *const krb5_crypto_iov,
                                               mut num_data: size_t,
                                               mut output: *mut krb5_data)
 -> krb5_error_code {
    let mut xorkey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ihash: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_uint = 0;
    let mut ihash_iov: *mut krb5_crypto_iov = 0 as *mut krb5_crypto_iov;
    let mut ohash_iov: [krb5_crypto_iov; 2] =
        [krb5_crypto_iov{flags: 0,
                         data:
                             krb5_data{magic: 0,
                                       length: 0,
                                       data: 0 as *mut libc::c_char,},}; 2];
    let mut hashout: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut ret: krb5_error_code = 0;
    if (*keyblock).length as libc::c_ulong > (*hash).blocksize {
        return -(1765328206 as libc::c_long) as krb5_error_code
    }
    if ((*output).length as libc::c_ulong) < (*hash).hashsize {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    /* Allocate space for the xor key, hash input vector, and inner hash */
    xorkey = k5alloc((*hash).blocksize, &mut ret) as *mut libc::c_uchar;
    if !xorkey.is_null() {
        ihash = k5alloc((*hash).hashsize, &mut ret) as *mut libc::c_uchar;
        if !ihash.is_null() {
            ihash_iov =
                k5calloc(num_data.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong),
                         ::std::mem::size_of::<krb5_crypto_iov>() as
                             libc::c_ulong, &mut ret) as *mut krb5_crypto_iov;
            if !ihash_iov.is_null() {
                /* Create the inner padded key. */
                memset(xorkey as *mut libc::c_void, 0x36 as libc::c_int,
                       (*hash).blocksize);
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*keyblock).length {
                    let ref mut fresh0 = *xorkey.offset(i as isize);
                    *fresh0 =
                        (*fresh0 as libc::c_int ^
                             *(*keyblock).contents.offset(i as isize) as
                                 libc::c_int) as libc::c_uchar;
                    i = i.wrapping_add(1)
                }
                /* Compute the inner hash over the inner key and input data. */
                (*ihash_iov.offset(0 as libc::c_int as isize)).flags =
                    2 as libc::c_int;
                (*ihash_iov.offset(0 as libc::c_int as isize)).data =
                    make_data(xorkey as *mut libc::c_void,
                              (*hash).blocksize as libc::c_uint);
                memcpy(ihash_iov.offset(1 as libc::c_int as isize) as
                           *mut libc::c_void, data as *const libc::c_void,
                       num_data.wrapping_mul(::std::mem::size_of::<krb5_crypto_iov>()
                                                 as libc::c_ulong));
                hashout =
                    make_data(ihash as *mut libc::c_void,
                              (*hash).hashsize as libc::c_uint);
                ret =
                    (*hash).hash.expect("non-null function pointer")(ihash_iov,
                                                                     num_data.wrapping_add(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_ulong),
                                                                     &mut hashout);
                if !(ret != 0 as libc::c_int) {
                    /* Create the outer padded key. */
                    memset(xorkey as *mut libc::c_void, 0x5c as libc::c_int,
                           (*hash).blocksize);
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < (*keyblock).length {
                        let ref mut fresh1 = *xorkey.offset(i as isize);
                        *fresh1 =
                            (*fresh1 as libc::c_int ^
                                 *(*keyblock).contents.offset(i as isize) as
                                     libc::c_int) as libc::c_uchar;
                        i = i.wrapping_add(1)
                    }
                    /* Compute the outer hash over the outer key and inner hash value. */
                    ohash_iov[0 as libc::c_int as usize].flags =
                        2 as libc::c_int;
                    ohash_iov[0 as libc::c_int as usize].data =
                        make_data(xorkey as *mut libc::c_void,
                                  (*hash).blocksize as libc::c_uint);
                    ohash_iov[1 as libc::c_int as usize].flags =
                        2 as libc::c_int;
                    ohash_iov[1 as libc::c_int as usize].data =
                        make_data(ihash as *mut libc::c_void,
                                  (*hash).hashsize as libc::c_uint);
                    (*output).length = (*hash).hashsize as libc::c_uint;
                    ret =
                        (*hash).hash.expect("non-null function pointer")(ohash_iov.as_mut_ptr(),
                                                                         2 as
                                                                             libc::c_int
                                                                             as
                                                                             size_t,
                                                                         output);
                    if ret != 0 as libc::c_int {
                        memset((*output).data as *mut libc::c_void,
                               0 as libc::c_int,
                               (*output).length as libc::c_ulong);
                    }
                }
            }
        }
    }
    zapfree(xorkey as *mut libc::c_void, (*hash).blocksize);
    zapfree(ihash as *mut libc::c_void, (*hash).hashsize);
    free(ihash_iov as *mut libc::c_void);
    return ret;
}
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
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn krb5int_hmac(mut hash: *const krb5_hash_provider,
                                      mut key: krb5_key,
                                      mut data: *const krb5_crypto_iov,
                                      mut num_data: size_t,
                                      mut output: *mut krb5_data)
 -> krb5_error_code {
    return krb5int_hmac_keyblock(hash, &mut (*key).keyblock, data, num_data,
                                 output);
}
