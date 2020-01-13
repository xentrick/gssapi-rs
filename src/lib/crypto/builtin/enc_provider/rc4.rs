use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:10"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:10"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:10"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:10"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:10"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:10"]
pub mod k5_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
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
    /*
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
    /* KRB5_GENERAL__ */
    /*
 * Begin "k5-config.h"
 */
    /*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
    /* From autoconf.h */
    /* HAVE_SYS_TYPES_H */
    /* HAVE_SYS_TYPES_H */
    /* KRB5_SYSTYPES__ */
    /* one day */
    /* one week */
    /* Thu Jan  1 00:00:00 2038 UTC */
    /*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
    /* #define KRB5_OLD_CRYPTO is done in krb5.h */
    /* KRB5_CONFIG__ */
    /*
 * End "k5-config.h"
 */
    /*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
    /* Get mutex support; currently used only for the replay cache.  */
    /* Get error info support.  */
    /* Get string buffer support. */
    /* Define tracing macros. */
    /* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
    /* Cache configuration variables */
    /* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
    /* KDC errors */
    /* No error */
    /* Client's entry in DB expired */
    /* Server's entry in DB expired */
    /* Requested pvno not supported */
    /* C's key encrypted in old master */
    /* S's key encrypted in old master */
    /* Client not found in Kerberos DB */
    /* Server not found in Kerberos DB */
    /* Multiple entries in Kerberos DB */
    /* The C or S has a null key */
    /* Tkt ineligible for postdating */
    /* Requested starttime > endtime */
    /* KDC policy rejects request */
    /* KDC can't do requested opt. */
    /* No support for encryption type */
    /* No support for checksum type */
    /* No support for padata type */
    /* No support for transited type */
    /* C's creds have been revoked */
    /* S's creds have been revoked */
    /* TGT has been revoked */
    /* C not yet valid */
    /* S not yet valid */
    /* Password has expired */
    /* Preauthentication failed */
    /* Additional preauthentication */
                                           /* required */
    /* Requested server and */
                                           /* ticket don't match*/
    /* Server principal valid for */
                                           /*   user2user only */
    /* KDC policy rejected transited */
                                           /*   path */
    /* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
    /* Application errors */
    /* Decrypt integrity check failed */
    /* Ticket expired */
    /* Ticket not yet valid */
    /* Request is a replay */
    /* The ticket isn't for us */
    /* Ticket/authenticator don't match */
    /* Clock skew too great */
    /* Incorrect net address */
    /* Protocol version mismatch */
    /* Invalid message type */
    /* Message stream modified */
    /* Message out of order */
    /* Key version is not available */
    /* Service key not available */
    /* Mutual authentication failed */
    /* Incorrect message direction */
    /* Alternative authentication */
                                        /* method required */
    /* Incorrect sequence numnber */
                                        /* in message */
    /* Inappropriate type of */
                                        /* checksum in message */
    /* Policy rejects transited path */
    /* Response too big for UDP, */
                                        /*   retry with TCP */
    /* other errors */
    /* Generic error (description */
                                        /* in e-text) */
    /* Field is too long for impl. */
    /* PKINIT server-reported errors */
    /* client cert not trusted */
    /* client signature verify failed */
    /* invalid Diffie-Hellman parameters */
    /* client cert not verifiable to */
                                                   /* trusted root cert */
    /* client cert had invalid signature */
    /* client cert was revoked */
    /* client cert revoked, reason unknown */
    /* mismatch between client cert and */
                                                   /* principal name */
    /* bad extended key use */
    /* bad digest algorithm in client cert */
    /* missing paChecksum in PA-PK-AS-REQ */
    /* bad digest algorithm in SignedData */
    /* The IAKERB proxy could
                                                      not find a KDC */
    /* The KDC did not respond
                                                      to the IAKERB proxy */
    /* RFC 6113 */
    /* RFC 6113 */
    /* err table base max offset for protocol err codes */
    /*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
    /*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
    /* RFC 4537 */
    /* sam_type values -- informational only */
    /*  Enigma Logic */
    /*  Digital Pathways */
    /*  S/key where  KDC has key 0 */
    /*  Traditional S/Key */
    /*  Security Dynamics */
    /*  CRYPTOCard */
    /* XXX need to figure out who has which numbers assigned */
    /*  ActivCard decimal mode */
    /*  ActivCard hex mode */
    /*  Digital Pathways hex mode */
    /* experimental */
    /* testing */
    /* special */
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
    /*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    /* In PAC options, indicates Resource-Based Constrained Delegation support. */
    /* struct stat, stat() */
    /* MAXPATHLEN */
    /* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
    /* libos.spec */
    /* Internal structure of an opaque key identifier */
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
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key};
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:10"]
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn krb5int_default_free_state(state: *mut krb5_data);
    }
    /* CRYPTO_INT_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:10"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:10"]
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
pub use self::k5_int_h::{krb5_key_st, derived_key, zapfree};
pub use self::crypto_int_h::{krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             krb5_hash_provider, krb5_enc_provider,
                             krb5_cksumtypes, verify_func, checksum_func,
                             krb5int_default_free_state};
use self::stdlib_h::{malloc, free};
use self::string_h::{explicit_bzero, memset};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "12:9"]
pub struct ArcfourContext {
    pub x: libc::c_uint,
    pub y: libc::c_uint,
    pub state: [libc::c_uchar; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "19:9"]
pub struct ArcFourCipherState {
    pub initialized: libc::c_int,
    pub ctx: ArcfourContext,
}
/* gets the next byte from the PRNG */
#[inline]
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn k5_arcfour_byte(mut ctx: *mut ArcfourContext)
 -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    let mut y: libc::c_uint = 0;
    let mut sx: libc::c_uint = 0;
    let mut sy: libc::c_uint = 0;
    let mut state: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    state = (*ctx).state.as_mut_ptr();
    x =
        (*ctx).x.wrapping_add(1 as libc::c_int as libc::c_uint) &
            0xff as libc::c_int as libc::c_uint;
    sx = *state.offset(x as isize) as libc::c_uint;
    y = sx.wrapping_add((*ctx).y) & 0xff as libc::c_int as libc::c_uint;
    sy = *state.offset(y as isize) as libc::c_uint;
    (*ctx).x = x;
    (*ctx).y = y;
    *state.offset(y as isize) = sx as libc::c_uchar;
    *state.offset(x as isize) = sy as libc::c_uchar;
    return *state.offset((sx.wrapping_add(sy) &
                              0xff as libc::c_int as libc::c_uint) as isize)
               as libc::c_uint;
}
/* Encrypts/decrypts data. */
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn k5_arcfour_crypt(mut ctx: *mut ArcfourContext,
                                      mut dest: *mut libc::c_uchar,
                                      mut src: *const libc::c_uchar,
                                      mut len: libc::c_uint) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < len {
        *dest.offset(i as isize) =
            (*src.offset(i as isize) as libc::c_uint ^ k5_arcfour_byte(ctx))
                as libc::c_uchar;
        i = i.wrapping_add(1)
    };
}
/* gcc inlines*/
/* Initializes the context and sets the key. */
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn k5_arcfour_init(mut ctx: *mut ArcfourContext,
                                     mut key: *const libc::c_uchar,
                                     mut key_len: libc::c_uint)
 -> krb5_error_code {
    let mut t: libc::c_uint =
        0; /*this is probably not the correct error code
                                     to return */
    let mut u: libc::c_uint = 0;
    let mut keyindex: libc::c_uint = 0;
    let mut stateindex: libc::c_uint = 0;
    let mut state: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut counter: libc::c_uint = 0;
    if key_len != 16 as libc::c_int as libc::c_uint {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    state =
        &mut *(*ctx).state.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut libc::c_uchar;
    (*ctx).x = 0 as libc::c_int as libc::c_uint;
    (*ctx).y = 0 as libc::c_int as libc::c_uint;
    counter = 0 as libc::c_int as libc::c_uint;
    while counter < 256 as libc::c_int as libc::c_uint {
        *state.offset(counter as isize) = counter as libc::c_uchar;
        counter = counter.wrapping_add(1)
    }
    keyindex = 0 as libc::c_int as libc::c_uint;
    stateindex = 0 as libc::c_int as libc::c_uint;
    counter = 0 as libc::c_int as libc::c_uint;
    while counter < 256 as libc::c_int as libc::c_uint {
        t = *state.offset(counter as isize) as libc::c_uint;
        stateindex =
            stateindex.wrapping_add(*key.offset(keyindex as isize) as
                                        libc::c_uint).wrapping_add(t) &
                0xff as libc::c_int as libc::c_uint;
        u = *state.offset(stateindex as isize) as libc::c_uint;
        *state.offset(stateindex as isize) = t as libc::c_uchar;
        *state.offset(counter as isize) = u as libc::c_uchar;
        keyindex = keyindex.wrapping_add(1);
        if keyindex >= key_len { keyindex = 0 as libc::c_int as libc::c_uint }
        counter = counter.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn k5_arcfour_docrypt(mut key: krb5_key,
                                        mut state: *const krb5_data,
                                        mut data: *mut krb5_crypto_iov,
                                        mut num_data: size_t)
 -> krb5_error_code {
    let mut arcfour_ctx: *mut ArcfourContext = 0 as *mut ArcfourContext;
    let mut cipher_state: *mut ArcFourCipherState =
        0 as *mut ArcFourCipherState;
    let mut ret: krb5_error_code = 0;
    let mut i: size_t = 0;
    if (*key).keyblock.length != 16 as libc::c_int as libc::c_uint {
        return -(1765328195 as libc::c_long) as krb5_error_code
    }
    if !state.is_null() &&
           (*state).length as libc::c_ulong !=
               ::std::mem::size_of::<ArcFourCipherState>() as libc::c_ulong {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    if !state.is_null() {
        cipher_state =
            (*state).data as *mut libc::c_void as *mut ArcFourCipherState;
        arcfour_ctx = &mut (*cipher_state).ctx;
        if (*cipher_state).initialized == 0 as libc::c_int {
            ret =
                k5_arcfour_init(arcfour_ctx, (*key).keyblock.contents,
                                (*key).keyblock.length);
            if ret != 0 as libc::c_int { return ret }
            (*cipher_state).initialized = 1 as libc::c_int
        }
    } else {
        arcfour_ctx =
            malloc(::std::mem::size_of::<ArcfourContext>() as libc::c_ulong)
                as *mut ArcfourContext;
        if arcfour_ctx.is_null() { return 12 as libc::c_int }
        ret =
            k5_arcfour_init(arcfour_ctx, (*key).keyblock.contents,
                            (*key).keyblock.length);
        if ret != 0 as libc::c_int {
            free(arcfour_ctx as *mut libc::c_void);
            return ret
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < num_data {
        let mut iov: *mut krb5_crypto_iov =
            &mut *data.offset(i as isize) as *mut krb5_crypto_iov;
        if (*iov).flags == 1 as libc::c_int ||
               ((*iov).flags == 2 as libc::c_int ||
                    (*iov).flags == 4 as libc::c_int) {
            k5_arcfour_crypt(arcfour_ctx,
                             (*iov).data.data as *mut libc::c_uchar,
                             (*iov).data.data as *const libc::c_uchar,
                             (*iov).data.length);
        }
        i = i.wrapping_add(1)
    }
    if state.is_null() {
        zapfree(arcfour_ctx as *mut libc::c_void,
                ::std::mem::size_of::<ArcfourContext>() as libc::c_ulong);
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn k5_arcfour_init_state(mut key: *const krb5_keyblock,
                                           mut keyusage: krb5_keyusage,
                                           mut new_state: *mut krb5_data)
 -> krb5_error_code {
    /* Note that we can't actually set up the state here  because the key
     * will change  between now and when encrypt is called
     * because  it is data dependent.  Yeah, this has strange
     * properties. --SDH
     */
    (*new_state).length =
        ::std::mem::size_of::<ArcFourCipherState>() as libc::c_ulong as
            libc::c_uint;
    (*new_state).data =
        malloc((*new_state).length as libc::c_ulong) as *mut libc::c_char;
    if !(*new_state).data.is_null() {
        memset((*new_state).data as *mut libc::c_void, 0 as libc::c_int,
               (*new_state).length as libc::c_ulong);
        /* That will set initialized to zero*/
    } else { return 12 as libc::c_int }
    return 0 as libc::c_int;
}
/* Since the arcfour cipher is identical going forwards and backwards,
   we just call "docrypt" directly
*/
#[no_mangle]
#[c2rust::src_loc = "176:32"]
pub static mut krb5int_enc_arcfour: krb5_enc_provider =
    unsafe {
        {
            let mut init =
                krb5_enc_provider{block_size: 1 as libc::c_int as size_t,
                                  keybytes: 16 as libc::c_int as size_t,
                                  keylength: 16 as libc::c_int as size_t,
                                  encrypt:
                                      Some(k5_arcfour_docrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  decrypt:
                                      Some(k5_arcfour_docrypt as
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
                                      Some(k5_arcfour_init_state as
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
