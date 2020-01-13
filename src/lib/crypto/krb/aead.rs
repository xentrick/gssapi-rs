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
    #[inline]
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key, krb5_magic};
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:27"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "391:8"]
    pub struct iov_cursor {
        pub iov: *const krb5_crypto_iov,
        pub iov_count: size_t,
        pub block_size: size_t,
        pub signing: krb5_boolean,
        pub in_iov: size_t,
        pub in_pos: size_t,
        pub out_iov: size_t,
        pub out_pos: size_t,
    }
    use super::krb5_h::{krb5_enctype, krb5_cksumtype, krb5_flags,
                        krb5_error_code, krb5_key, krb5_data, krb5_keyblock,
                        krb5_keyusage, krb5_crypto_iov, krb5_cryptotype,
                        krb5_boolean};
    use super::stddef_h::size_t;
    /* CRYPTO_INT_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
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
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
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
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_cksumtype, krb5_keyusage, krb5_cryptotype,
                       krb5_flags, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key, make_data, empty_data};
pub use self::crypto_int_h::{krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             krb5_hash_provider, krb5_enc_provider,
                             krb5_cksumtypes, verify_func, checksum_func,
                             iov_cursor};
use self::stdlib_h::{calloc, free};
use self::string_h::{memset, memcpy};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/krb/aead.c */
/*
 * Copyright 2008 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5int_c_locate_iov(mut data: *mut krb5_crypto_iov,
                                              mut num_data: size_t,
                                              mut type_0: krb5_cryptotype)
 -> *mut krb5_crypto_iov {
    let mut i: size_t = 0;
    let mut iov: *mut krb5_crypto_iov = 0 as *mut krb5_crypto_iov;
    if data.is_null() { return 0 as *mut krb5_crypto_iov }
    i = 0 as libc::c_int as size_t;
    while i < num_data {
        if (*data.offset(i as isize)).flags == type_0 {
            if iov.is_null() {
                iov = &mut *data.offset(i as isize) as *mut krb5_crypto_iov
            } else { return 0 as *mut krb5_crypto_iov }
            /* can't appear twice */
        } /* takes place of STREAM */
        i = i.wrapping_add(1)
    }
    return iov;
}
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn krb5int_c_iov_decrypt_stream(mut ktp:
                                                          *const krb5_keytypes,
                                                      mut key: krb5_key,
                                                      mut keyusage:
                                                          krb5_keyusage,
                                                      mut ivec:
                                                          *const krb5_data,
                                                      mut data:
                                                          *mut krb5_crypto_iov,
                                                      mut num_data: size_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut header_len: libc::c_uint = 0;
    let mut trailer_len: libc::c_uint = 0;
    let mut iov: *mut krb5_crypto_iov = 0 as *mut krb5_crypto_iov;
    let mut stream: *mut krb5_crypto_iov = 0 as *mut krb5_crypto_iov;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut got_data: libc::c_int = 0 as libc::c_int;
    stream = krb5int_c_locate_iov(data, num_data, 7 as libc::c_int);
    if !stream.is_null() {
    } else {
        __assert_fail(b"stream != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"aead.c\x00" as *const u8 as *const libc::c_char,
                      64 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 146],
                                                &[libc::c_char; 146]>(b"krb5_error_code krb5int_c_iov_decrypt_stream(const struct krb5_keytypes *, krb5_key, krb5_keyusage, const krb5_data *, krb5_crypto_iov *, size_t)\x00")).as_ptr());
    }
    header_len =
        (*ktp).crypto_length.expect("non-null function pointer")(ktp,
                                                                 1 as
                                                                     libc::c_int);
    trailer_len =
        (*ktp).crypto_length.expect("non-null function pointer")(ktp,
                                                                 5 as
                                                                     libc::c_int);
    if (*stream).data.length < header_len.wrapping_add(trailer_len) {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    iov =
        calloc(num_data.wrapping_add(2 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<krb5_crypto_iov>() as libc::c_ulong) as
            *mut krb5_crypto_iov;
    if iov.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as size_t;
    (*iov.offset(i as isize)).flags = 1 as libc::c_int;
    (*iov.offset(i as isize)).data =
        make_data((*stream).data.data as *mut libc::c_void, header_len);
    i = i.wrapping_add(1);
    j = 0 as libc::c_int as size_t;
    while j < num_data {
        if (*data.offset(j as isize)).flags == 2 as libc::c_int {
            if got_data != 0 {
                free(iov as *mut libc::c_void);
                return -(1765328194 as libc::c_long) as krb5_error_code
            }
            got_data += 1;
            let ref mut fresh0 = (*data.offset(j as isize)).data.data;
            *fresh0 = (*stream).data.data.offset(header_len as isize);
            (*data.offset(j as isize)).data.length =
                (*stream).data.length.wrapping_sub(header_len).wrapping_sub(trailer_len)
        }
        if (*data.offset(j as isize)).flags == 3 as libc::c_int ||
               (*data.offset(j as isize)).flags == 2 as libc::c_int {
            let fresh1 = i;
            i = i.wrapping_add(1);
            *iov.offset(fresh1 as isize) = *data.offset(j as isize)
        }
        j = j.wrapping_add(1)
    }
    /* Use empty padding since tokens don't indicate the padding length. */
    (*iov.offset(i as isize)).flags = 4 as libc::c_int;
    (*iov.offset(i as isize)).data = empty_data();
    i = i.wrapping_add(1);
    (*iov.offset(i as isize)).flags = 5 as libc::c_int;
    (*iov.offset(i as isize)).data =
        make_data((*stream).data.data.offset((*stream).data.length as
                                                 isize).offset(-(trailer_len
                                                                     as
                                                                     isize))
                      as *mut libc::c_void, trailer_len);
    i = i.wrapping_add(1);
    if i <= num_data.wrapping_add(2 as libc::c_int as libc::c_ulong) {
    } else {
        __assert_fail(b"i <= num_data + 2\x00" as *const u8 as
                          *const libc::c_char,
                      b"aead.c\x00" as *const u8 as *const libc::c_char,
                      110 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 146],
                                                &[libc::c_char; 146]>(b"krb5_error_code krb5int_c_iov_decrypt_stream(const struct krb5_keytypes *, krb5_key, krb5_keyusage, const krb5_data *, krb5_crypto_iov *, size_t)\x00")).as_ptr());
    }
    ret =
        (*ktp).decrypt.expect("non-null function pointer")(ktp, key, keyusage,
                                                           ivec, iov, i);
    free(iov as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "117:1"]
pub unsafe extern "C" fn krb5int_c_padding_length(mut ktp:
                                                      *const krb5_keytypes,
                                                  mut data_length: size_t)
 -> libc::c_uint {
    let mut header: libc::c_uint = 0;
    let mut padding: libc::c_uint = 0;
    /*
     * Add in the header length since the header is encrypted along with the
     * data.  (arcfour violates this assumption since not all of the header is
     * encrypted, but that's okay since it has no padding.  If there is ever an
     * enctype using a similar token format and a block cipher, we will have to
     * move this logic into an enctype-dependent function.)
     */
    header =
        (*ktp).crypto_length.expect("non-null function pointer")(ktp,
                                                                 1 as
                                                                     libc::c_int);
    data_length =
        (data_length as libc::c_ulong).wrapping_add(header as libc::c_ulong)
            as size_t as size_t;
    padding =
        (*ktp).crypto_length.expect("non-null function pointer")(ktp,
                                                                 4 as
                                                                     libc::c_int);
    if padding == 0 as libc::c_int as libc::c_uint ||
           data_length.wrapping_rem(padding as libc::c_ulong) ==
               0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as libc::c_uint
    } else {
        return (padding as
                    libc::c_ulong).wrapping_sub(data_length.wrapping_rem(padding
                                                                             as
                                                                             libc::c_ulong))
                   as libc::c_uint
    };
}
/* Return the next iov (starting from ind) which cursor should process, or
 * cursor->iov_count if there are none remaining. */
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn next_iov_to_process(mut cursor: *mut iov_cursor,
                                         mut ind: size_t) -> size_t {
    let mut iov: *const krb5_crypto_iov = 0 as *const krb5_crypto_iov;
    while ind < (*cursor).iov_count {
        iov = &*(*cursor).iov.offset(ind as isize) as *const krb5_crypto_iov;
        if if (*cursor).signing != 0 {
               ((*iov).flags == 1 as libc::c_int ||
                    ((*iov).flags == 2 as libc::c_int ||
                         (*iov).flags == 4 as libc::c_int) ||
                    (*iov).flags == 3 as libc::c_int) as libc::c_int
           } else {
               ((*iov).flags == 1 as libc::c_int ||
                    ((*iov).flags == 2 as libc::c_int ||
                         (*iov).flags == 4 as libc::c_int)) as libc::c_int
           } != 0 {
            break ;
        }
        ind = ind.wrapping_add(1)
    }
    return ind;
}
#[no_mangle]
#[c2rust::src_loc = "154:1"]
pub unsafe extern "C" fn k5_iov_cursor_init(mut cursor: *mut iov_cursor,
                                            mut iov: *const krb5_crypto_iov,
                                            mut count: size_t,
                                            mut block_size: size_t,
                                            mut signing: krb5_boolean) {
    (*cursor).iov = iov;
    (*cursor).iov_count = count;
    (*cursor).block_size = block_size;
    (*cursor).signing = signing;
    (*cursor).in_iov =
        next_iov_to_process(cursor, 0 as libc::c_int as size_t);
    (*cursor).out_iov = (*cursor).in_iov;
    (*cursor).out_pos = 0 as libc::c_int as size_t;
    (*cursor).in_pos = (*cursor).out_pos;
}
/* Fetch one block from cursor's input position. */
#[no_mangle]
#[c2rust::src_loc = "168:1"]
pub unsafe extern "C" fn k5_iov_cursor_get(mut cursor: *mut iov_cursor,
                                           mut block: *mut libc::c_uchar)
 -> krb5_boolean {
    let mut nbytes: size_t = 0;
    let mut bsz: size_t = (*cursor).block_size;
    let mut remain: size_t = (*cursor).block_size;
    let mut iov: *const krb5_crypto_iov = 0 as *const krb5_crypto_iov;
    remain = (*cursor).block_size;
    while remain > 0 as libc::c_int as libc::c_ulong &&
              (*cursor).in_iov < (*cursor).iov_count {
        iov =
            &*(*cursor).iov.offset((*cursor).in_iov as isize) as
                *const krb5_crypto_iov;
        nbytes =
            ((*iov).data.length as
                 libc::c_ulong).wrapping_sub((*cursor).in_pos);
        if nbytes > remain { nbytes = remain }
        memcpy(block.offset(bsz as isize).offset(-(remain as isize)) as
                   *mut libc::c_void,
               (*iov).data.data.offset((*cursor).in_pos as isize) as
                   *const libc::c_void, nbytes);
        (*cursor).in_pos =
            ((*cursor).in_pos as libc::c_ulong).wrapping_add(nbytes) as size_t
                as size_t;
        remain =
            (remain as libc::c_ulong).wrapping_sub(nbytes) as size_t as
                size_t;
        if (*cursor).in_pos == (*iov).data.length as libc::c_ulong {
            (*cursor).in_iov =
                next_iov_to_process(cursor,
                                    (*cursor).in_iov.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong));
            (*cursor).in_pos = 0 as libc::c_int as size_t
        }
    }
    if remain == bsz { return 0 as libc::c_int as krb5_boolean }
    if remain > 0 as libc::c_int as libc::c_ulong {
        memset(block.offset(bsz as isize).offset(-(remain as isize)) as
                   *mut libc::c_void, 0 as libc::c_int, remain);
    }
    return 1 as libc::c_int as krb5_boolean;
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
/* Write a block to a cursor's output position. */
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn k5_iov_cursor_put(mut cursor: *mut iov_cursor,
                                           mut block: *mut libc::c_uchar) {
    let mut nbytes: size_t = 0;
    let mut bsz: size_t = (*cursor).block_size;
    let mut remain: size_t = (*cursor).block_size;
    let mut iov: *const krb5_crypto_iov = 0 as *const krb5_crypto_iov;
    remain = (*cursor).block_size;
    while remain > 0 as libc::c_int as libc::c_ulong &&
              (*cursor).out_iov < (*cursor).iov_count {
        iov =
            &*(*cursor).iov.offset((*cursor).out_iov as isize) as
                *const krb5_crypto_iov;
        nbytes =
            ((*iov).data.length as
                 libc::c_ulong).wrapping_sub((*cursor).out_pos);
        if nbytes > remain { nbytes = remain }
        memcpy((*iov).data.data.offset((*cursor).out_pos as isize) as
                   *mut libc::c_void,
               block.offset(bsz as isize).offset(-(remain as isize)) as
                   *const libc::c_void, nbytes);
        (*cursor).out_pos =
            ((*cursor).out_pos as libc::c_ulong).wrapping_add(nbytes) as
                size_t as size_t;
        remain =
            (remain as libc::c_ulong).wrapping_sub(nbytes) as size_t as
                size_t;
        if (*cursor).out_pos == (*iov).data.length as libc::c_ulong {
            (*cursor).out_iov =
                next_iov_to_process(cursor,
                                    (*cursor).out_iov.wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_ulong));
            (*cursor).out_pos = 0 as libc::c_int as size_t
        }
    };
}
