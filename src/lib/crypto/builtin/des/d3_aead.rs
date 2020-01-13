use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:25"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:25"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:25"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:25"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:25"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_0)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:25"]
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn k5_iov_cursor_put(cursor: *mut iov_cursor,
                                 block: *mut libc::c_uchar);
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn k5_iov_cursor_get(cursor: *mut iov_cursor,
                                 block: *mut libc::c_uchar) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn k5_iov_cursor_init(cursor: *mut iov_cursor,
                                  iov: *const krb5_crypto_iov, count: size_t,
                                  block_size: size_t, signing: krb5_boolean);
    }
    /* CRYPTO_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/des_int.h:26"]
pub mod des_int_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct des_ks_struct {
        pub _data: [libc::c_int; 2],
    }
    #[c2rust::src_loc = "116:1"]
    pub type mit_des_cblock = des_cblock;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "162:29"]
        pub static krb5int_c_mit_des_zeroblock: mit_des_cblock;
    }
    /*DES_INTERNAL_DEFS*/
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:25"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/des/f_tables.h:27"]
pub mod f_tables_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/des/f_tables.h */
/*
 * Copyright (C) 1990 by the Massachusetts Institute of Technology.
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
 * DES implementation donated by Dennis Ferguson
 */
        /*
 * des_tables.h - declarations to import the DES tables, used internally
 *                by some of the library routines.
 */
        /* nothing */
        /*
 * These may be declared const if you wish.  Be sure to change the
 * declarations in des_tables.c as well.
 */
        #[no_mangle]
        #[c2rust::src_loc = "43:33"]
        pub static des_IP_table: [libc::c_uint; 256];
        #[no_mangle]
        #[c2rust::src_loc = "44:33"]
        pub static des_FP_table: [libc::c_uint; 256];
        #[no_mangle]
        #[c2rust::src_loc = "45:33"]
        pub static des_SP_table: [[libc::c_uint; 64]; 8];
    }
    /* __DES_TABLES_H__ */
    /* Shorthand that we'll need in several places, for creating values that
   really can hold 32 bits regardless of the prevailing int size.  */
    /*
 * These are handy dandy utility thingies for straightening out bytes.
 * Included here because they're used a couple of places.
 */
    /*
 * Finally, as a sample of how all this might be held together, the
 * following two macros do in-place encryptions and decryptions.  left
 * and right are two unsigned DES_INT32 variables which at the beginning
 * are expected to hold the clear (encrypted) block in host byte order
 * (left the high order four bytes, right the low order).  At the end
 * they will contain the encrypted (clear) block.  temp is an unsigned DES_INT32
 * used as a temporary.  kp is an unsigned DES_INT32 pointer pointing at
 * the start of the key schedule.  All these should be in registers.
 *
 * You can probably do better than these by rewriting for particular
 * situations.  These aren't bad, though.
 *
 * The DEB macros enable debugging when this code breaks (typically
 * when a buggy compiler breaks it), by printing the intermediate values
 * at each stage of the encryption, so that by comparing the output to
 * a known good machine, the location of the first error can be found.
 */
    /*
 * Here is a sample final permutation.  Note that there is a trick
 * here.  DES requires swapping the left and right parts after the
 * last cipher round but before the final permutation.  We do this
 * swapping internally, which is why left and right are confused
 * at the beginning.
 */
    /*
 * Now the final permutation stuff.  The same comments apply to
 * this as to the initial permutation, except that we use different
 * bits and shifts.
 *
 * The inserted cast to unsigned DES_INT32 circumvents a bug in
 * the Macintosh MPW 3.2 C compiler which loses the unsignedness and
 * propagates the high-order bit in the shift.
 */
    /*
 * The following macro does an in-place initial permutation given
 * the current left and right parts of the block and a single
 * temporary.  Use this more as a guide for rolling your own, though.
 * The best way to do the IP depends on the form of the data you
 * are dealing with.  If you use this, though, try to make left,
 * right and temp unsigned DES_INT32s.
 */
    /*
 * Macros to help deal with the initial permutation table.  Note
 * the IP table only deals with 32 bits at a time, allowing us to
 * collect the bits we need to deal with each half into an unsigned
 * DES_INT32.  By carefully selecting how the bits are ordered we also
 * take advantages of symmetries in the table so that we can use a
 * single table to compute the permutation of all bytes.  This sounds
 * complicated, but if you go through the process of designing the
 * table you'll find the symmetries fall right out.
 *
 * The follow macros compute the set of bits used to index the
 * table for produce the left and right permuted result.
 *
 * The inserted cast to unsigned DES_INT32 circumvents a bug in
 * the Macintosh MPW 3.2 C compiler which loses the unsignedness and
 * propagates the high-order bit in the shift.
 */
    /*
 * Code to do a DES round using the tables.  Note that the E expansion
 * is easy to compute algorithmically, especially if done out-of-order.
 * Take a look at its form and compare it to everything involving temp
 * below.  Since SP[0-7] don't have any bits in common set it is okay
 * to do the successive xor's.
 *
 * Note too that the SP table has been reordered to match the order of
 * the keys (if the original order of SP was 12345678, the reordered
 * table is 71354682).  This is unnecessary, but was done since some
 * compilers seem to like you going through the matrix from beginning
 * to end.
 *
 * There is a difference in the best way to do this depending on whether
 * one is encrypting or decrypting.  If encrypting we move forward through
 * the keys and hence should move forward through the table.  If decrypting
 * we go back.  Part of the need for this comes from trying to emulate
 * existing software which generates a single key schedule and uses it
 * both for encrypting and decrypting.  Generating separate encryption
 * and decryption key schedules would allow one to use the same code
 * for both.
 *
 * left, right and temp should be unsigned DES_INT32 values.  left and right
 * should be the high and low order parts of the cipher block at the
 * current stage of processing (this makes sense if you read the spec).
 * kp should be an unsigned DES_INT32 pointer which points at the current
 * set of subkeys in the key schedule.  It is advanced to the next set
 * (i.e. by 8 bytes) when this is done.
 *
 * This occurs in the innermost loop of the DES function.  The four
 * variables should really be in registers.
 *
 * When using this, the inner loop of the DES function might look like:
 *
 *      for (i = 0; i < 8; i++) {
 *              DES_SP_{EN,DE}CRYPT_ROUND(left, right, temp, kp);
 *              DES_SP_{EN,DE}CRYPT_ROUND(right, left, temp, kp);
 *      }
 *
 * Note the trick above.  You are supposed to do 16 rounds, swapping
 * left and right at the end of each round.  By doing two rounds at
 * a time and swapping left and right in the code we can avoid the
 * swaps altogether.
 */
    /* nothing */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be};
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
                             iov_cursor, k5_iov_cursor_put, k5_iov_cursor_get,
                             k5_iov_cursor_init};
pub use self::des_int_h::{des_cblock, des_ks_struct, mit_des_cblock,
                          krb5int_c_mit_des_zeroblock};
pub use self::byteswap_h::__bswap_32;
use self::f_tables_h::{des_IP_table, des_FP_table, des_SP_table};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology.
 * Copyright 1995 by Richard P. Basch.  All Rights Reserved.
 * Copyright 1995 by Lehman Brothers, Inc.  All Rights Reserved.
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
 * the name of Richard P. Basch, Lehman Brothers and M.I.T. not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission.  Richard P. Basch,
 * Lehman Brothers and M.I.T. make no representations about the suitability
 * of this software for any purpose.  It is provided "as is" without
 * express or implied warranty.
 */
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5int_des3_cbc_encrypt(mut data:
                                                      *mut krb5_crypto_iov,
                                                  mut num_data: libc::c_ulong,
                                                  mut ks1:
                                                      *const des_ks_struct,
                                                  mut ks2:
                                                      *const des_ks_struct,
                                                  mut ks3:
                                                      *const des_ks_struct,
                                                  mut ivec:
                                                      *mut libc::c_uchar) {
    let mut left: libc::c_uint = 0;
    let mut right: libc::c_uint = 0;
    let mut kp1: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut kp2: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut kp3: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut ip: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut cursor: iov_cursor =
        iov_cursor{iov: 0 as *const krb5_crypto_iov,
                   iov_count: 0,
                   block_size: 0,
                   signing: 0,
                   in_iov: 0,
                   in_pos: 0,
                   out_iov: 0,
                   out_pos: 0,};
    let mut block: [libc::c_uchar; 8] = [0; 8];
    /* Get key pointers here.  These won't need to be reinitialized. */
    kp1 = ks1 as *const libc::c_uint;
    kp2 = ks2 as *const libc::c_uint;
    kp3 = ks3 as *const libc::c_uint;
    /* Initialize left and right with the contents of the initial vector. */
    ip =
        if !ivec.is_null() {
            ivec as *const libc::c_uchar
        } else { krb5int_c_mit_des_zeroblock.as_ptr() };
    left = load_32_be(ip as *const libc::c_void);
    right =
        load_32_be(ip.offset(4 as libc::c_int as isize) as
                       *const libc::c_void);
    k5_iov_cursor_init(&mut cursor, data, num_data,
                       (8 as libc::c_int as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_octet>()
                                                            as libc::c_ulong),
                       0 as libc::c_int as krb5_boolean);
    while k5_iov_cursor_get(&mut cursor, block.as_mut_ptr()) != 0 {
        /* xor this block with the previous ciphertext. */
        left ^= load_32_be(block.as_mut_ptr() as *const libc::c_void);
        right ^=
            load_32_be(block.as_mut_ptr().offset(4 as libc::c_int as isize) as
                           *const libc::c_void);
        /* Encrypt what we have and store it back into block. */
        let mut i: libc::c_int = 0;
        let mut temp1: libc::c_uint = 0;
        temp1 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp1 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp1 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp1 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp1 & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let fresh0 = kp1;
            kp1 = kp1.offset(1);
            temp1 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *fresh0;
            left ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh1 = kp1;
            kp1 = kp1.offset(1);
            temp1 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *fresh1;
            left ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh2 = kp1;
            kp1 = kp1.offset(1);
            temp1 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *fresh2;
            right ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh3 = kp1;
            kp1 = kp1.offset(1);
            temp1 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^
                    *fresh3;
            right ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i += 1
        }
        temp1 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp1 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp1 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp1 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp1 & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        kp1 = kp1.offset(-((2 as libc::c_int * 16 as libc::c_int) as isize));
        let mut i_0: libc::c_int = 0;
        let mut temp2: libc::c_uint = 0;
        temp2 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp2 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp2 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp2 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp2 & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        kp2 = kp2.offset((2 as libc::c_int * 16 as libc::c_int) as isize);
        i_0 = 0 as libc::c_int;
        while i_0 < 8 as libc::c_int {
            kp2 = kp2.offset(-1);
            temp2 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *kp2;
            left ^=
                des_SP_table[7 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[4 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp2 = kp2.offset(-1);
            temp2 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *kp2;
            left ^=
                des_SP_table[3 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[0 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp2 = kp2.offset(-1);
            temp2 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^ *kp2;
            right ^=
                des_SP_table[7 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[4 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp2 = kp2.offset(-1);
            temp2 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *kp2;
            right ^=
                des_SP_table[3 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[0 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i_0 += 1
        }
        temp2 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp2 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp2 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp2 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp2 & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        let mut i_1: libc::c_int = 0;
        let mut temp1_0: libc::c_uint = 0;
        temp1_0 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp1_0 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp1_0 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp1_0 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp1_0 & 0xff as libc::c_int as libc::c_uint)
                                 as usize] << 3 as libc::c_int;
        i_1 = 0 as libc::c_int;
        while i_1 < 8 as libc::c_int {
            let fresh4 = kp3;
            kp3 = kp3.offset(1);
            temp1_0 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *fresh4;
            left ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1_0 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1_0 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh5 = kp3;
            kp3 = kp3.offset(1);
            temp1_0 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *fresh5;
            left ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1_0 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1_0 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh6 = kp3;
            kp3 = kp3.offset(1);
            temp1_0 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *fresh6;
            right ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1_0 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1_0 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh7 = kp3;
            kp3 = kp3.offset(1);
            temp1_0 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^
                    *fresh7;
            right ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1_0 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1_0 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i_1 += 1
        }
        temp1_0 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp1_0 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp1_0 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp1_0 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp1_0 & 0xff as libc::c_int as libc::c_uint)
                                 as usize];
        kp3 = kp3.offset(-((2 as libc::c_int * 16 as libc::c_int) as isize));
        store_32_be(left, block.as_mut_ptr() as *mut libc::c_void);
        store_32_be(right,
                    block.as_mut_ptr().offset(4 as libc::c_int as isize) as
                        *mut libc::c_void);
        k5_iov_cursor_put(&mut cursor, block.as_mut_ptr());
    }
    if !ivec.is_null() {
        store_32_be(left, ivec as *mut libc::c_void);
        store_32_be(right,
                    ivec.offset(4 as libc::c_int as isize) as
                        *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn krb5int_des3_cbc_decrypt(mut data:
                                                      *mut krb5_crypto_iov,
                                                  mut num_data: libc::c_ulong,
                                                  mut ks1:
                                                      *const des_ks_struct,
                                                  mut ks2:
                                                      *const des_ks_struct,
                                                  mut ks3:
                                                      *const des_ks_struct,
                                                  mut ivec:
                                                      *mut libc::c_uchar) {
    let mut left: libc::c_uint = 0;
    let mut right: libc::c_uint = 0;
    let mut kp1: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut kp2: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut kp3: *const libc::c_uint = 0 as *const libc::c_uint;
    let mut ip: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ocipherl: libc::c_uint = 0;
    let mut ocipherr: libc::c_uint = 0;
    let mut cipherl: libc::c_uint = 0;
    let mut cipherr: libc::c_uint = 0;
    let mut cursor: iov_cursor =
        iov_cursor{iov: 0 as *const krb5_crypto_iov,
                   iov_count: 0,
                   block_size: 0,
                   signing: 0,
                   in_iov: 0,
                   in_pos: 0,
                   out_iov: 0,
                   out_pos: 0,};
    let mut block: [libc::c_uchar; 8] = [0; 8];
    /* Get key pointers here.  These won't need to be reinitialized. */
    kp1 = ks1 as *const libc::c_uint;
    kp2 = ks2 as *const libc::c_uint;
    kp3 = ks3 as *const libc::c_uint;
    /*
     * Decrypting is harder than encrypting because of
     * the necessity of remembering a lot more things.
     * Should think about this a little more...
     */
    /* Prime the old cipher with ivec.*/
    ip =
        if !ivec.is_null() {
            ivec as *const libc::c_uchar
        } else { krb5int_c_mit_des_zeroblock.as_ptr() };
    ocipherl = load_32_be(ip as *const libc::c_void);
    ocipherr =
        load_32_be(ip.offset(4 as libc::c_int as isize) as
                       *const libc::c_void);
    k5_iov_cursor_init(&mut cursor, data, num_data,
                       (8 as libc::c_int as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_octet>()
                                                            as libc::c_ulong),
                       0 as libc::c_int as krb5_boolean);
    while k5_iov_cursor_get(&mut cursor, block.as_mut_ptr()) != 0 {
        /* Split this block into left and right. */
        left = load_32_be(block.as_mut_ptr() as *const libc::c_void);
        cipherl = left;
        right =
            load_32_be(block.as_mut_ptr().offset(4 as libc::c_int as isize) as
                           *const libc::c_void);
        cipherr = right;
        /* Decrypt and xor with the old cipher to get plain text. */
        let mut i: libc::c_int = 0;
        let mut temp2: libc::c_uint = 0;
        temp2 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp2 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp2 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp2 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp2 & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        kp3 = kp3.offset((2 as libc::c_int * 16 as libc::c_int) as isize);
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            kp3 = kp3.offset(-1);
            temp2 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *kp3;
            left ^=
                des_SP_table[7 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[4 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp3 = kp3.offset(-1);
            temp2 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *kp3;
            left ^=
                des_SP_table[3 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[0 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp3 = kp3.offset(-1);
            temp2 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^ *kp3;
            right ^=
                des_SP_table[7 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[4 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp3 = kp3.offset(-1);
            temp2 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *kp3;
            right ^=
                des_SP_table[3 as libc::c_int as
                                 usize][(temp2 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp2 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp2 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[0 as libc::c_int as
                                     usize][(temp2 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i += 1
        }
        temp2 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp2 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp2 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp2 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp2 & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        let mut i_0: libc::c_int = 0;
        let mut temp1: libc::c_uint = 0;
        temp1 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp1 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp1 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp1 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp1 & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        i_0 = 0 as libc::c_int;
        while i_0 < 8 as libc::c_int {
            let fresh8 = kp2;
            kp2 = kp2.offset(1);
            temp1 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *fresh8;
            left ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh9 = kp2;
            kp2 = kp2.offset(1);
            temp1 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *fresh9;
            left ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh10 = kp2;
            kp2 = kp2.offset(1);
            temp1 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *fresh10;
            right ^=
                des_SP_table[0 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[3 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            let fresh11 = kp2;
            kp2 = kp2.offset(1);
            temp1 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^
                    *fresh11;
            right ^=
                des_SP_table[4 as libc::c_int as
                                 usize][(temp1 >> 24 as libc::c_int &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp1 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp1 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[7 as libc::c_int as
                                     usize][(temp1 &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i_0 += 1
        }
        temp1 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp1 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp1 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp1 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp1 & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        kp2 = kp2.offset(-((2 as libc::c_int * 16 as libc::c_int) as isize));
        let mut i_1: libc::c_int = 0;
        let mut temp2_0: libc::c_uint = 0;
        temp2_0 =
            left & 0xaaaaaaaa as libc::c_uint |
                (right & 0xaaaaaaaa as libc::c_uint) >> 1 as libc::c_int;
        right =
            (left & 0x55555555 as libc::c_int as libc::c_uint) <<
                1 as libc::c_int |
                right & 0x55555555 as libc::c_int as libc::c_uint;
        left =
            des_IP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize] << 3 as libc::c_int;
        right =
            des_IP_table[(temp2_0 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize] |
                des_IP_table[(temp2_0 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 1 as libc::c_int |
                des_IP_table[(temp2_0 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_IP_table[(temp2_0 & 0xff as libc::c_int as libc::c_uint)
                                 as usize] << 3 as libc::c_int;
        kp1 = kp1.offset((2 as libc::c_int * 16 as libc::c_int) as isize);
        i_1 = 0 as libc::c_int;
        while i_1 < 8 as libc::c_int {
            kp1 = kp1.offset(-1);
            temp2_0 =
                (right >> 23 as libc::c_int | right << 9 as libc::c_int) ^
                    *kp1;
            left ^=
                des_SP_table[7 as libc::c_int as
                                 usize][(temp2_0 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp2_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp2_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[4 as libc::c_int as
                                     usize][(temp2_0 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp1 = kp1.offset(-1);
            temp2_0 =
                (right >> 11 as libc::c_int | right << 21 as libc::c_int) ^
                    *kp1;
            left ^=
                des_SP_table[3 as libc::c_int as
                                 usize][(temp2_0 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp2_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp2_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[0 as libc::c_int as
                                     usize][(temp2_0 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp1 = kp1.offset(-1);
            temp2_0 =
                (left >> 23 as libc::c_int | left << 9 as libc::c_int) ^ *kp1;
            right ^=
                des_SP_table[7 as libc::c_int as
                                 usize][(temp2_0 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[6 as libc::c_int as
                                     usize][(temp2_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[5 as libc::c_int as
                                     usize][(temp2_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[4 as libc::c_int as
                                     usize][(temp2_0 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            kp1 = kp1.offset(-1);
            temp2_0 =
                (left >> 11 as libc::c_int | left << 21 as libc::c_int) ^
                    *kp1;
            right ^=
                des_SP_table[3 as libc::c_int as
                                 usize][(temp2_0 &
                                             0x3f as libc::c_int as
                                                 libc::c_uint) as usize] |
                    des_SP_table[2 as libc::c_int as
                                     usize][(temp2_0 >> 8 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[1 as libc::c_int as
                                     usize][(temp2_0 >> 16 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize] |
                    des_SP_table[0 as libc::c_int as
                                     usize][(temp2_0 >> 24 as libc::c_int &
                                                 0x3f as libc::c_int as
                                                     libc::c_uint) as usize];
            i_1 += 1
        }
        temp2_0 =
            right & 0xf0f0f0f0 as libc::c_uint |
                (left & 0xf0f0f0f0 as libc::c_uint) >> 4 as libc::c_int;
        right =
            (right & 0xf0f0f0f as libc::c_int as libc::c_uint) <<
                4 as libc::c_int |
                left & 0xf0f0f0f as libc::c_int as libc::c_uint;
        left =
            des_FP_table[(right >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(right >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(right >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(right & 0xff as libc::c_int as libc::c_uint) as
                                 usize];
        right =
            des_FP_table[(temp2_0 >> 24 as libc::c_int &
                              0xff as libc::c_int as libc::c_uint) as usize]
                << 6 as libc::c_int |
                des_FP_table[(temp2_0 >> 16 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 4 as libc::c_int |
                des_FP_table[(temp2_0 >> 8 as libc::c_int &
                                  0xff as libc::c_int as libc::c_uint) as
                                 usize] << 2 as libc::c_int |
                des_FP_table[(temp2_0 & 0xff as libc::c_int as libc::c_uint)
                                 as usize];
        left ^= ocipherl;
        right ^= ocipherr;
        /* Store the encrypted halves back into block. */
        store_32_be(left, block.as_mut_ptr() as *mut libc::c_void);
        store_32_be(right,
                    block.as_mut_ptr().offset(4 as libc::c_int as isize) as
                        *mut libc::c_void);
        /* Save current cipher block halves. */
        ocipherl = cipherl;
        ocipherr = cipherr;
        k5_iov_cursor_put(&mut cursor, block.as_mut_ptr());
    }
    if !ivec.is_null() {
        store_32_be(ocipherl, ivec as *mut libc::c_void);
        store_32_be(ocipherr,
                    ivec.offset(4 as libc::c_int as isize) as
                        *mut libc::c_void);
    };
}
