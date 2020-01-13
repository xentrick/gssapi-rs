use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:32"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __caddr_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:32"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/unistd.h:32"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2013:16"]
    pub struct _krb5_creds {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub keyblock: krb5_keyblock,
        pub times: krb5_ticket_times,
        pub is_skey: krb5_boolean,
        pub ticket_flags: krb5_flags,
        pub addresses: *mut *mut krb5_address,
        pub ticket: krb5_data,
        pub second_ticket: krb5_data,
        pub authdata: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2724:1"]
    pub type krb5_kt_cursor = krb5_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2727:16"]
    pub struct krb5_keytab_entry_st {
        pub magic: krb5_magic,
        pub principal: krb5_principal,
        pub timestamp: krb5_timestamp,
        pub vno: krb5_kvno,
        pub key: krb5_keyblock,
    }
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
    }
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6811:16"]
    pub struct _krb5_get_init_creds_opt {
        pub flags: krb5_flags,
        pub tkt_life: krb5_deltat,
        pub renew_life: krb5_deltat,
        pub forwardable: libc::c_int,
        pub proxiable: libc::c_int,
        pub etype_list: *mut krb5_enctype,
        pub etype_list_length: libc::c_int,
        pub address_list: *mut *mut krb5_address,
        pub preauth_list: *mut krb5_preauthtype,
        pub preauth_list_length: libc::c_int,
        pub salt: *mut krb5_data,
    }
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        /* typedef struct _profile_t *profile_t; */
        /*
 * begin wordsize.h
 */
        /*
 * Word-size related definition.
 */
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
        /* This may change, later on */
        /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
        /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
        /* Originally introduced for PKINIT; now unused.  Do not use this. */
        /* Originally used to recognize AFS and default salts.  No longer used. */
        /* *< An array of strings */
        /*
 * Per V5 spec on definition of principal types
 */
        /* *<  Name type not known */
        /* *< Just the name of the principal
                                      as in DCE, or for users */
        /* *< Service and other unique instance (krbtgt) */
        /* *< Service with host name as instance
                                      (telnet, rcommands) */
        /* *< Service with host as remaining components */
        /* *< Unique ID */
        /* *< PKINIT */
        /* *< Name in form of SMTP email name */
        /* *< Windows 2000 UPN */
        /* *< Well-known (special) principal */
        /* *< First component of
                                                NT_WELLKNOWN principals */
        /* *< Windows 2000 UPN and SID */
        /* *< NT 4 style name */
        /* *< NT 4 style name and SID */
        /* * Constant version of krb5_principal_data */
        /* * Constant for realm referrals. */
        /*
 * Referral-specific functions.
 */
        /* *
 * Check for a match with KRB5_REFERRAL_REALM.
 *
 * @param [in] r                Realm to check
 *
 * @return @c TRUE if @a r is zero-length, @c FALSE otherwise
 */
        /* *
 * Return an anonymous realm data.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_REALMSTR
 */
        /* *
 * Build an anonymous principal.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_PRINCSTR
 */
        /* *< Anonymous realm */
        /* *< Anonymous principal name */
        /*
 * end "base-defs.h"
 */
        /*
 * begin "hostaddr.h"
 */
        /* * Structure for address */
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        /*
 * begin "encryption.h"
 */
        /* * Exposed contents of a key. */
        /* *
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
        /* to call krb5_encrypt_size, you need
                                           this.  it was a pointer, but it
                                           doesn't have to be.  gross. */
        /* checksum type */
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
        /* *< @ref KRB5_CRYPTO_TYPE type of the iov */
        /* per Kerberos v5 protocol spec */
        /* *< @deprecated no longer supported */
        /* *< @deprecated no longer supported */
        /* *< @deprecated no longer supported */
        /* *< @deprecated no longer supported */
        /* *< @deprecated DES-3 cbc with SHA1 */
        /* *< @deprecated DES-3 cbc mode raw */
        /* *< @deprecated no longer supported */
        /* PKINIT */
        /* *< DSA with SHA1, CMS signature */
        /* *< MD5 with RSA, CMS signature */
        /* *< SHA1 with RSA, CMS signature */
        /* *< RC2 cbc mode, CMS enveloped data */
        /* *< RSA encryption, CMS enveloped data */
        /* *< RSA w/OEAP encryption, CMS enveloped data */
        /* *< DES-3 cbc mode, CMS enveloped data */
        /* *< RFC 3962 */
        /* *< RFC 3962 */
        /* *< RFC 8009 */
        /* *< RFC 8009 */
        /* *< RFC 4757 */
        /* *< RFC 4757 */
        /* *< RFC 6803 */
        /* *< RFC 6803 */
        /* des-mac-k */
/* rsa-md4-des-k */
        /* *< RFC 3962. Used with
                                                ENCTYPE_AES128_CTS_HMAC_SHA1_96 */
        /* *< RFC 3962. Used with
                                                ENCTYPE_AES256_CTS_HMAC_SHA1_96 */
        /* *< RFC 8009 */
        /* *< RFC 8009 */
        /* *< RFC 6803 */
        /* *< RFC 6803 */
        /* Microsoft netlogon */
        /* *< RFC 4757 */
        /*
 * The following are entropy source designations. Whenever
 * krb5_C_random_add_entropy is called, one of these source ids is passed in.
 * This allows the library to better estimate bits of entropy in the sample and
 * to keep track of what sources of entropy have contributed enough entropy.
 * Sources marked internal MUST NOT be used by applications outside the
 * Kerberos library
 */
        /*calls to krb5_C_RANDOM_SEED (INTERNAL)*/
        /* /dev/random or equivalent (internal)*/
        /* From KDC or other trusted party*/
        /*
     * This source should be used carefully; data in this category
     * should be from a third party trusted to give random bits
     * For example keys issued by the KDC in the application server.
     */
        /* Timing of operations*/
        /*Protocol data possibly from attacker*/
        /*Do not use; maximum source ID*/
        /* round x up to nearest multiple of y */
        /* roundup */
        /* macro function definitions to help clean up code */
        /* *
 * Encrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Data to be encrypted
 * @param [out]    output       Encrypted data
 *
 * This function encrypts the data block @a input and stores the output into @a
 * output.  The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result (using krb5_c_encrypt_length() to determine the amount
 * of space needed).  @a output->length will be set to the actual length of the
 * ciphertext.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Decrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Compute encrypted data length.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  inputlen        Length of the data to be encrypted
 * @param [out] length          Length of the encrypted data
 *
 * This function computes the length of the ciphertext produced by encrypting
 * @a inputlen bytes including padding, confounder, and checksum.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Return cipher block size.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] blocksize       Block size for @a enctype
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Return length of the specified key in bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] keybytes        Number of bytes required to make a key
 * @param [out] keylength       Length of final key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Initialize a new cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Key
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [out] new_state       New cipher state
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Free a cipher state previously allocated by krb5_c_init_state().
 *
 * @param [in] context          Library context
 * @param [in] key              Key
 * @param [in] state            Cipher state to be freed
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate enctype-specific pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  keyblock        Key
 * @param [in]  input           Input data
 * @param [out] output          Output data
 *
 * This function selects a pseudo-random function based on @a keyblock and
 * computes its value over @a input, placing the result into @a output.
 * The caller must preinitialize @a output and allocate space for the
 * result, using krb5_c_prf_length() to determine the required length.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Get the output length of pseudo-random functions for an encryption type.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] len             Length of PRF output
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate pseudo-random bytes using RFC 6113 PRF+.
 *
 * @param [in]  context         Library context
 * @param [in]  k               KDC contribution key
 * @param [in]  input           Input data
 * @param [out] output          Pseudo-random output buffer
 *
 * This function fills @a output with PRF+(k, input) as defined in RFC 6113
 * section 5.1.  The caller must preinitialize @a output and allocate the
 * desired amount of space.  The length of the pseudo-random output will match
 * the length of @a output.
 *
 * @note RFC 4402 defines a different PRF+ operation.  This function does not
 * implement that operation.
 *
 * @return 0 on success, @c E2BIG if output->length is too large for PRF+ to
 * generate, @c ENOMEM on allocation failure, or an error code from
 * krb5_c_prf()
 */
        /* *
 * Derive a key using some input data (via RFC 6113 PRF+).
 *
 * @param [in]  context         Library context
 * @param [in]  k               KDC contribution key
 * @param [in]  input           Input string
 * @param [in]  enctype         Output key enctype (or @c ENCTYPE_NULL)
 * @param [out] out             Derived keyblock
 *
 * This function uses PRF+ as defined in RFC 6113 to derive a key from another
 * key and an input string.  If @a enctype is @c ENCTYPE_NULL, the output key
 * will have the same enctype as the input key.
 */
        /* *
 * Compute the KRB-FX-CF2 combination of two keys and pepper strings.
 *
 * @param [in]  context         Library context
 * @param [in]  k1              KDC contribution key
 * @param [in]  pepper1         String "PKINIT"
 * @param [in]  k2              Reply key
 * @param [in]  pepper2         String "KeyExchange"
 * @param [out] out             Output key
 *
 * This function computes the KRB-FX-CF2 function over its inputs and places
 * the results in a newly allocated keyblock.  This function is simple in that
 * it assumes that @a pepper1 and @a pepper2 are C strings with no internal
 * nulls and that the enctype of the result will be the same as that of @a k1.
 * @a k1 and @a k2 may be of different enctypes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate an enctype-specific random encryption key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type of the generated key
 * @param [out] k5_random_key   An allocated and initialized keyblock
 *
 * Use krb5_free_keyblock_contents() to free @a k5_random_key when
 * no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate an enctype-specific key from random data.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  random_data     Random input data
 * @param [out] k5_random_key   Resulting key
 *
 * This function takes random input data @a random_data and produces a valid
 * key @a k5_random_key for a given @a enctype.
 *
 * @note It is assumed that @a k5_random_key has already been initialized and
 * @a k5_random_key->contents has been allocated with the correct length.
 *
 * @sa krb5_c_keylengths()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Add entropy to the pseudo-random number generator.
 *
 * @param [in] context          Library context
 * @param [in] randsource       Entropy source (see KRB5_RANDSOURCE types)
 * @param [in] data             Data
 *
 * Contribute entropy to the PRNG used by krb5 crypto operations.  This may or
 * may not affect the output of the next crypto operation requiring random
 * data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [out] data            Random data
 *
 * Fills in @a data with bytes from the PRNG used by krb5 crypto operations.
 * The caller must preinitialize @a data and allocate the desired amount of
 * space.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Collect entropy from the OS if possible.
 *
 * @param [in]  context         Library context
 * @param [in]  strong          Strongest available source of entropy
 * @param [out] success         1 if OS provides entropy, 0 otherwise
 *
 * If @a strong is non-zero, this function attempts to use the strongest
 * available source of entropy.  Setting this flag may cause the function to
 * block on some operating systems.  Good uses include seeding the PRNG for
 * kadmind and realm setup.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* *
 * Convert a string (such a password) to a key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [out] key             Generated key
 *
 * This function converts @a string to a @a key of encryption type @a enctype,
 * using the specified @a salt.  The newly created @a key must be released by
 * calling krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Convert a string (such as a password) to a key with additional parameters.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [in]  params          Parameters
 * @param [out] key             Generated key
 *
 * This function is similar to krb5_c_string_to_key(), but also takes
 * parameters which may affect the algorithm in an enctype-dependent way.  The
 * newly created @a key must be released by calling
 * krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Compare two encryption types.
 *
 * @param [in]  context         Library context
 * @param [in]  e1              First encryption type
 * @param [in]  e2              Second encryption type
 * @param [out] similar         @c TRUE if types are similar, @c FALSE if not
 *
 * This function determines whether two encryption types use the same kind of
 * keys.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Compute a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_k_make_checksum(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Verify a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_k_verify_checksum(), but operates
 * on keyblock @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Return the length of checksums for a checksum type.
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type
 * @param [out] length          Checksum length
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Return a list of keyed checksum types usable with an encryption type.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] count           Count of allowable checksum types
 * @param [out] cksumtypes      Array of allowable checksum types
 *
 * Use krb5_free_cksumtypes() to free @a cksumtypes when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* * @defgroup KRB5_KEYUSAGE KRB5_KEYUSAGE
 * @{
 */
        /* XXX need to register these */
        /* Defined in Integrating SAM Mechanisms with Kerberos draft */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY */
        /* Defined in [MS-SFU] */
/* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_RESPONSE */
        /* unused */
        /* *< See RFC 6560 section 4.2 */
        /* define in draft-ietf-krb-wg-preauth-framework*/
        /* Key usage values 512-1023 are reserved for uses internal to a Kerberos
 * implementation. */
        /* *< Used for encrypted FAST cookies */
        /* *< Used for freshness tokens */
        /* * @} */
        /* end of KRB5_KEYUSAGE group */
        /* *
 * Verify that a specified encryption type is a valid Kerberos encryption type.
 *
 * @param [in] ktype            Encryption type
 *
 * @return @c TRUE if @a ktype is valid, @c FALSE if not
 */
        /* *
 * Verify that specified checksum type is a valid Kerberos checksum type.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is valid, @c FALSE if not
 */
        /* *
 * Test whether a checksum type is collision-proof.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is collision-proof, @c FALSE if it is not
 * collision-proof or not a valid checksum type.
 */
        /* *
 * Test whether a checksum type is keyed.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is a keyed checksum type, @c FALSE otherwise.
 */
        /* AEAD APIs */
/* * @defgroup KRB5_CRYPTO_TYPE KRB5_CRYPTO_TYPE
 * @{
 */
        /* *< [in] ignored */
        /* *< [out] header */
        /* *< [in, out] plaintext */
        /* *< [in] associated data */
        /* *< [out] padding */
        /* *< [out] checksum for encrypt */
        /* *< [out] checksum for MIC */
        /* *< [in] entire message without
                                           decomposing the structure into
                                           header, data and trailer buffers */
        /* * @} */
        /* end of KRB5_CRYPTO_TYPE group */
        /* *
 * Fill in a checksum element in IOV array (operates on keyblock)
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] data            IOV array
 * @param [in]     num_data        Size of @a data
 *
 * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
 * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
 * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
 *
 * @note This function is similar to krb5_k_make_checksum_iov(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Validate a checksum element in IOV array (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     data            IOV array
 * @param [in]     num_data        Size of @a data
 * @param [out]    valid           Non-zero for success, zero for failure
 *
 * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
 * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
 * regions in the iov.
 *
 * @note This function is similar to krb5_k_verify_checksum_iov(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_make_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Encrypt data in place supporting AEAD (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     keyblock        Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function encrypts the data block @a data and stores the output in-place.
 * The actual encryption key will be derived from @a keyblock and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_k_encrypt_iov(), but operates
 * on keyblock @a keyblock.
 *
 * @sa krb5_c_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Decrypt data in place supporting AEAD (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     keyblock        Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function decrypts the data block @a data and stores the output in-place.
 * The actual decryption key will be derived from @a keyblock and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_k_decrypt_iov(), but operates
 * on keyblock @a keyblock.
 *
 * @sa krb5_c_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Return a length of a message field specific to the encryption type.
 *
 * @param [in]  context      Library context
 * @param [in]  enctype      Encryption type
 * @param [in]  type         Type field (See @ref KRB5_CRYPTO_TYPE types)
 * @param [out] size         Length of the @a type specific to @a enctype
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Fill in lengths for header, trailer and padding in a IOV array.
 *
 * @param [in]      context      Library context
 * @param [in]      enctype      Encryption type
 * @param [in,out]  data         IOV array
 * @param [in]      num_data     Size of @a data
 *
 * Padding is set to the actual padding required based on the provided
 * @a data buffers. Typically this API is used after setting up the data
 * buffers and #KRB5_CRYPTO_TYPE_SIGN_ONLY buffers, but before actually
 * allocating header, trailer and padding.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Return a number of padding octets.
 *
 * @param [in]  context      Library context
 * @param [in]  enctype      Encryption type
 * @param [in]  data_length  Length of the plaintext to pad
 * @param [out] size         Number of padding octets
 *
 * This function returns the number of the padding octets required to pad
 * @a data_length octets of plaintext.
 *
 * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
 */
        /* *
 * Create a krb5_key from the enctype and key data in a keyblock.
 *
 * @param [in]  context      Library context
 * @param [in]  key_data     Keyblock
 * @param [out] out          Opaque key
 *
 * The reference count on a key @a out is set to 1.
 * Use krb5_k_free_key() to free @a out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
 */
        /* * Increment the reference count on a key. */
        /* * Decrement the reference count on a key and free it if it hits zero. */
        /* * Retrieve a copy of the keyblock from a krb5_key structure. */
        /* * Retrieve the enctype of a krb5_key structure. */
        /* *
 * Encrypt data using a key (operates on opaque key).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Data to be encrypted
 * @param [out]    output       Encrypted data
 *
 * This function encrypts the data block @a input and stores the output into @a
 * output.  The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result (using krb5_c_encrypt_length() to determine the amount
 * of space needed).  @a output->length will be set to the actual length of the
 * ciphertext.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Encrypt data in place supporting AEAD (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     key             Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function encrypts the data block @a data and stores the output in-place.
 * The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_c_encrypt_iov(), but operates
 * on opaque key @a key.
 *
 * @sa krb5_k_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Decrypt data using a key (operates on opaque key).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Decrypt data in place supporting AEAD (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     key             Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function decrypts the data block @a data and stores the output in-place.
 * The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_c_decrypt_iov(), but operates
 * on opaque key @a key.
 *
 * @sa krb5_k_encrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Compute a checksum (operates on opaque key).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_c_make_checksum(), but operates
 * on opaque @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Fill in a checksum element in IOV array (operates on opaque key)
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] data            IOV array
 * @param [in]     num_data        Size of @a data
 *
 * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
 * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
 * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
 *
 * @note This function is similar to krb5_c_make_checksum_iov(), but operates
 * on opaque @a key.
 *
 * @sa krb5_k_verify_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Verify a checksum (operates on opaque key).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_c_verify_checksum(), but operates
 * on opaque @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Validate a checksum element in IOV array (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     data            IOV array
 * @param [in]     num_data        Size of @a data
 * @param [out]    valid           Non-zero for success, zero for failure
 *
 * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
 * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
 * regions in the iov.
 *
 * @note This function is similar to krb5_c_verify_checksum_iov(), but operates
 * on opaque @a key.
 *
 * @sa krb5_k_make_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate enctype-specific pseudo-random bytes (operates on opaque key).
 *
 * @param [in]  context      Library context
 * @param [in]  key          Key
 * @param [in]  input        Input data
 * @param [out] output       Output data
 *
 * This function selects a pseudo-random function based on @a key and
 * computes its value over @a input, placing the result into @a output.
 * The caller must preinitialize @a output and allocate space for the
 * result.
 *
 * @note This function is similar to krb5_c_prf(), but operates
 * on opaque @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /*
 * old cryptosystem routine prototypes.  These are now layered
 * on top of the functions above.
 */
/* * @deprecated Replaced by krb5_c_* API family.*/
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated See krb5_c_string_to_key() */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated Replaced by krb5_c_* API family. */
        /* * @deprecated See krb5_c_checksum_length() */
        /* * @deprecated See krb5_c_make_checksum() */
        /* * @deprecated See krb5_c_verify_checksum() */
        /* KRB5_OLD_CRYPTO */
        /*
 * end "encryption.h"
 */
        /*
 * begin "fieldbits.h"
 */
        /* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
        /* #define      KDC_OPT_UNUSED          0x01000000 */
        /* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
        /* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
        /* #define      KDC_OPT_UNUSED          0x00000004 */
        /*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivitive tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
        /* definitions for ap_options fields */
        /* * @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
        /* *< Use session key */
        /* *< Perform a mutual
                                                 authentication exchange */
        /* *< Generate a subsession key
                                                 from the current session key
                                                 obtained from the
                                                 credentials */
        /* #define      AP_OPTS_RESERVED        0x10000000 */
/* #define      AP_OPTS_RESERVED        0x08000000 */
/* #define      AP_OPTS_RESERVED        0x04000000 */
/* #define      AP_OPTS_RESERVED        0x02000000 */
/* #define      AP_OPTS_RESERVED        0x01000000 */
/* #define      AP_OPTS_RESERVED        0x00800000 */
/* #define      AP_OPTS_RESERVED        0x00400000 */
/* #define      AP_OPTS_RESERVED        0x00200000 */
/* #define      AP_OPTS_RESERVED        0x00100000 */
/* #define      AP_OPTS_RESERVED        0x00080000 */
/* #define      AP_OPTS_RESERVED        0x00040000 */
/* #define      AP_OPTS_RESERVED        0x00020000 */
/* #define      AP_OPTS_RESERVED        0x00010000 */
/* #define      AP_OPTS_RESERVED        0x00008000 */
/* #define      AP_OPTS_RESERVED        0x00004000 */
/* #define      AP_OPTS_RESERVED        0x00002000 */
/* #define      AP_OPTS_RESERVED        0x00001000 */
/* #define      AP_OPTS_RESERVED        0x00000800 */
/* #define      AP_OPTS_RESERVED        0x00000400 */
/* #define      AP_OPTS_RESERVED        0x00000200 */
/* #define      AP_OPTS_RESERVED        0x00000100 */
/* #define      AP_OPTS_RESERVED        0x00000080 */
/* #define      AP_OPTS_RESERVED        0x00000040 */
/* #define      AP_OPTS_RESERVED        0x00000020 */
/* #define      AP_OPTS_RESERVED        0x00000010 */
/* #define      AP_OPTS_RESERVED        0x00000008 */
/* #define      AP_OPTS_RESERVED        0x00000004 */
        /* * @} */
        /* end of AP_OPTS group */
        /* definitions for ad_type fields. */
        /* Ticket flags */
/* flags are 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      TKT_FLG_RESERVED        0x80000000 */
        /* #define      TKT_FLG_RESERVED        0x00004000 */
/* #define      TKT_FLG_RESERVED        0x00002000 */
/* #define      TKT_FLG_RESERVED        0x00001000 */
/* #define      TKT_FLG_RESERVED        0x00000800 */
/* #define      TKT_FLG_RESERVED        0x00000400 */
/* #define      TKT_FLG_RESERVED        0x00000200 */
/* #define      TKT_FLG_RESERVED        0x00000100 */
/* #define      TKT_FLG_RESERVED        0x00000080 */
/* #define      TKT_FLG_RESERVED        0x00000040 */
/* #define      TKT_FLG_RESERVED        0x00000020 */
/* #define      TKT_FLG_RESERVED        0x00000010 */
/* #define      TKT_FLG_RESERVED        0x00000008 */
/* #define      TKT_FLG_RESERVED        0x00000004 */
/* #define      TKT_FLG_RESERVED        0x00000002 */
/* #define      TKT_FLG_RESERVED        0x00000001 */
        /* definitions for lr_type fields. */
        /* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */
        /*
 * end "fieldbits.h"
 */
        /*
 * begin "proto.h"
 */
        /* * Protocol version number */
        /* Message types */
        /* *< Initial authentication request */
        /* *< Response to AS request */
        /* *< Ticket granting server request */
        /* *< Response to TGS request */
        /* *< Auth req to application server */
        /* *< Response to mutual AP request */
        /* *< Safe application message */
        /* *< Private application message */
        /* *< Cred forwarding message */
        /* *< Error response */
        /* LastReq types */
        /* PADATA types */
        /* *< RFC 4120 */
        /* *< RFC 4120 */
        /* Not used */
        /* *< timestamp encrypted in key. RFC 4120 */
        /* *< SecurId passcode. RFC 4120 */
        /* *< Sesame project. RFC 4120 */
        /* *< OSF DCE. RFC 4120 */
        /* *< Cybersafe. RFC 4120 */
        /* *< Cygnus. RFC 4120, 3961 */
        /* *< Etype info for preauth. RFC 4120 */
        /* *< SAM/OTP */
        /* *< SAM/OTP */
        /* *< PKINIT */
        /* *< PKINIT */
        /* *< PKINIT. RFC 4556 */
        /* *< PKINIT. RFC 4556 */
        /* *< RFC 4120 */
        /* *< RFC 4120 */
        /* *< Windows 2000 referrals. RFC 6820 */
        /* *< SAM/OTP. RFC 4120 */
        /* *< Embedded in typed data. RFC 4120 */
        /* *< draft referral system */
        /* *< draft challenge system, updated */
        /* *< draft challenge system, updated */
        /* MS-KILE */
        /* *< include Windows PAC */
        /* *< username protocol transition request */
        /* *< certificate protocol transition request */
        /* *< AS checksum */
        /* *< RFC 6113 */
        /* *< RFC 6113 */
        /* *< RFC 6113 */
        /* *< RFC 6113 */
        /* *< RFC 6560 section 4.1 */
        /* *< RFC 6560 section 4.2 */
        /* *< RFC 6560 section 4.3 */
        /* *< RFC 6112 */
        /* *< RFC 6806 */
        /* *< RFC 8070 */
        /* *< MS-KILE and MS-SFU */
        /* *< currently must be zero */
        /* * Transited encoding types */
        /* * alternate authentication types */
        /* authorization data types. See RFC 4120 section 5.2.6 */
        /* * @defgroup KRB5_AUTHDATA KRB5_AUTHDATA
 * @{
 */
        /* *< RFC 4537 */
        /* *< formerly 142 in krb5 1.8 */
        /* * @} */
        /* end of KRB5_AUTHDATA group */
        /* password change constants */
        /* *< Success */
        /* *< Malformed request */
        /* *< Server error */
        /* *< Authentication error */
        /* *< Password change rejected */
        /* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
        /* *< Not authorized */
        /* *< Unknown RPC version */
        /* * The presented credentials were not obtained using a password directly */
        /*
 * end "proto.h"
 */
        /* Time set */
/* * Ticket start time, end time, and renewal duration. */
        /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
        /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
        /* *< optional in ticket, if not present, use @a authtime */
        /* *< Ticket expiration time */
        /* *< Latest time at which renewal of ticket can be valid */
        /* * Structure for auth data */
        /* *< ADTYPE */
        /* *< Length of data  */
        /* *< Data */
        /* * Structure for transited encoding */
        /* *< Transited encoding type */
        /* *< Contents */
        /* * Encrypted part of ticket. */
        /* to-be-encrypted portion */
        /* *< flags */
        /* *< session key: includes enctype */
        /* *< client name/realm */
        /* *< list of transited realms */
        /* *< auth, start, end, renew_till */
        /* *< array of ptrs to addresses */
        /* *< auth data */
        /* *
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
        /* cleartext portion */
        /* *< server name/realm */
        /* *< encryption type, kvno, encrypted encoding */
        /* *< ptr to decrypted version, if available */
        /* the unencrypted version */
/* *
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
        /* *< client name/realm */
        /* *< checksum, includes type, optional */
        /* *< client usec portion */
        /* *< client sec portion */
        /* *< true session key, optional */
        /* *< sequence #, optional */
        /* *< authoriazation data */
        /* * Ticket authentication data. */
        /* * Credentials structure including ticket, session key, and lifetime info. */
        /* *< client's principal identifier */
        /* *< server's principal identifier */
        /* *< session encryption key info */
        /* *< lifetime info */
        /* *< true if ticket is encrypted in
                                           another ticket's skey */
        /* *< flags in ticket */
        /* *< addrs in ticket */
        /* *< ticket string itself */
        /* *< second ticket, if related to
                                           ticket (via DUPLICATE-SKEY or
                                           ENC-TKT-IN-SKEY) */
        /* *< authorization data */
        /* * Last request entry */
        /* *< LR type */
        /* *< Timestamp */
        /* * Pre-authentication data */
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
        /* Don't use this; use krb5_pa_data instead. */
        /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
        /* *< KRB5_AS_REQ or KRB5_TGS_REQ */
        /* *< Preauthentication data */
        /* real body */
        /* *< Requested options */
        /* *< Client principal and realm */
        /* *< Server principal and realm */
        /* *< Requested start time */
        /* *< Requested end time */
        /* *< Requested renewable end time */
        /* *< Nonce to match request and response */
        /* *< Number of enctypes */
        /* *< Requested enctypes */
        /* *< Requested addresses (optional) */
        /* *< Encrypted authz data (optional) */
        /* *< Unencrypted authz data */
        /* *< Second ticket array (optional) */
        /* *
 * C representation of @c EncKDCRepPart protocol message.
 *
 * This is the cleartext message that is encrypted and inserted in @c KDC-REP.
 */
        /* encrypted part: */
        /* *< krb5 message type */
        /* *< Session key */
        /* *< Array of pointers to entries */
        /* *< Nonce from request */
        /* *< Expiration date */
        /* *< Ticket flags */
        /* *< Lifetime info */
        /* *< Server's principal identifier */
        /* *< Array of ptrs to addrs, optional */
        /* *< Encrypted preauthentication data */
        /* * Representation of the @c KDC-REP protocol message. */
        /* cleartext part: */
        /* *< KRB5_AS_REP or KRB5_KDC_REP */
        /* *< Preauthentication data from KDC */
        /* *< Client principal and realm */
        /* *< Ticket */
        /* *< Encrypted part of reply */
        /* *< Unencrypted version, if available */
        /* * Error message structure */
        /* some of these may be meaningless in certain contexts */
        /* *< Client sec portion; optional */
        /* *< Client usec portion; optional */
        /* *< Server usec portion */
        /* *< Server sec portion */
        /* *< Error code (protocol error #'s) */
        /* *< Client principal and realm */
        /* *< Server principal and realm */
        /* *< Descriptive text */
        /* *< Additional error-describing data */
        /* * Authentication header. */
        /* *< Requested options */
        /* *< Ticket */
        /* *< Encrypted authenticator */
        /* *
 * C representaton of AP-REP message.
 *
 * The server's response to a client's request for mutual authentication.
 */
        /* *< Ciphertext of ApRepEncPart */
        /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
        /* *< Client time, seconds portion */
        /* *< Client time, microseconds portion */
        /* *< Subkey (optional) */
        /* *< Sequence number */
        /* Unused */
        /* * Credentials information inserted into @c EncKrbCredPart. */
        /* *< Session key used to encrypt ticket */
        /* *< Client principal and realm */
        /* *< Server principal and realm */
        /* *< Ticket flags */
        /* *< Auth, start, end, renew_till */
        /* *< Array of pointers to addrs (optional) */
        /* * Cleartext credentials information.  */
        /* *< Nonce (optional) */
        /* *< Generation time, seconds portion */
        /* *< Generation time, microseconds portion */
        /* *< Sender address (optional) */
        /* *< Recipient address (optional) */
        /* * Credentials data structure.*/
        /* *< Tickets */
        /* *< Encrypted part */
        /* *< Unencrypted version, if available */
        /* Unused, but here for API compatibility. */
        /* Unused, but here for API compatibility. */
        /* Unused, but here for API compatibility. */
        /* * Referred name, only realm is required */
        /* Unused, but here for API compatibility. */
        /* * TRUE if a PAC should be included in TGS-REP */
        /*
 * begin "safepriv.h"
 */
        /* * @defgroup KRB5_AUTH_CONTEXT KRB5_AUTH_CONTEXT
 * @{
 */
/* * Prevent replays with timestamps and replay cache. */
        /* * Save timestamps for application. */
        /* * Prevent replays with sequence numbers. */
        /* * Save sequence numbers for application. */
        /* * @} */
        /* end of KRB5_AUTH_CONTEXT group */
        /* *
 * Replay data.
 *
 * Sequence number and timestamp information output by krb5_rd_priv() and
 * krb5_rd_safe().
 */
        /* *< Timestamp, seconds portion */
        /* *< Timestamp, microseconds portion */
        /* *< Sequence number  */
        /* Flags for krb5_auth_con_genaddrs(). */
        /* * Generate the local network address. */
        /* * Generate the remote network address.  */
        /* * Generate the local network address and the local port. */
        /* * Generate the remote network address and the remote port. */
        /* * Type of function used as a callback to generate checksum data for mk_req */
        /*
 * end "safepriv.h"
 */
        /*
 * begin "ccache.h"
 */
        /* * Cursor for sequential lookup */
        /* * Cursor for iterating over all ccaches */
        /* Flags for krb5_cc_retrieve_cred. */
/* * The requested lifetime must be at least as great as the time specified. */
        /* * The is_skey field must match exactly. */
        /* * All the flags set in the match credentials must be set. */
        /* * All the time fields must match exactly. */
        /* * All the flags must match exactly. */
        /* * The authorization data must match. */
        /* * Only the name portion of the principal name must match. */
        /* * The second ticket must match. */
        /* * The encryption key type must match. */
        /* * The supported key types must match. */
        /* Flags for krb5_cc_set_flags and similar. */
/* * Open and close the file for each cache operation. */
        /* *< @deprecated has no effect */
        /* *
 * Retrieve the name, but not type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @warning Returns the name of the credential cache.  The result is an alias
 * into @a cache and should not be freed or modified by the caller.  This name
 * does not include the cache type, so should not be used as input to
 * krb5_cc_resolve().
 *
 * @return
 * On success - the name of the credential cache.
 */
        /* *
 * Retrieve the full name of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] fullname_out    Full name of cache
 *
 * Use krb5_free_string() to free @a fullname_out when it is no longer needed.
 *
 * @version New in 1.10
 */
        /* KRB5_DEPRECATED */
        /* *
 * Initialize a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] principal        Default principal name
 *
 * Destroy any existing contents of @a cache and initialize it for the default
 * principal @a principal.
 *
 * @retval
 *  0  Success
 * @return
 *  System errors; Permission errors; Kerberos error codes
 */
        /* *
 * Destroy a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function destroys any existing contents of @a cache and closes the
 * handle to it.
 *
 * @retval
 * 0  Success
 * @return
 * Permission errors
 */
        /* *
 * Close a credential cache handle.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function closes a credential cache handle @a cache without affecting
 * the contents of the cache.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Store credentials in a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] creds            Credentials to be stored in cache
 *
 * This function stores @a creds into @a cache.  If @a creds->server and the
 * server in the decoded ticket @a creds->ticket differ, the credentials will
 * be stored under both server principal names.
 *
 * @retval
 *  0  Success
 * @return Permission errors; storage failure errors; Kerberos error codes
 */
        /* *
 * Retrieve a specified credentials from a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  flags           Flags bit mask
 * @param [in]  mcreds          Credentials to match
 * @param [out] creds           Credentials matching the requested value
 *
 * This function searches a credential cache for credentials matching @a mcreds
 * and returns it if found.
 *
 * Valid values for @a flags are:
 *
 * @li #KRB5_TC_MATCH_TIMES        The requested lifetime must be at least as
 *                                 great as in @a mcreds .
 * @li #KRB5_TC_MATCH_IS_SKEY      The @a is_skey field much match exactly.
 * @li #KRB5_TC_MATCH_FLAGS        Flags set in @a mcreds must be set.
 * @li #KRB5_TC_MATCH_TIMES_EXACT  The requested lifetime must match exactly.
 * @li #KRB5_TC_MATCH_FLAGS_EXACT  Flags must match exactly.
 * @li #KRB5_TC_MATCH_AUTHDATA     The authorization data must match.
 * @li #KRB5_TC_MATCH_SRV_NAMEONLY Only the name portion of the principal
 *                                 name must match, not the realm.
 * @li #KRB5_TC_MATCH_2ND_TKT      The second tickets must match.
 * @li #KRB5_TC_MATCH_KTYPE        The encryption key types must match.
 * @li #KRB5_TC_SUPPORTED_KTYPES   Check all matching entries that have any
 *                                 supported encryption type and return the
 *                                 one with the encryption type listed earliest.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Get the default principal of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] principal       Primary principal
 *
 * Returns the default client principal of a credential cache as set by
 * krb5_cc_initialize().
 *
 * Use krb5_free_principal() to free @a principal when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Prepare to sequentially read every credential in a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] cursor          Cursor
 *
 * krb5_cc_end_seq_get() must be called to complete the retrieve operation.
 *
 * @note If the cache represented by @a cache is modified between the time of
 * the call to this function and the time of the final krb5_cc_end_seq_get(),
 * these changes may not be reflected in the results of krb5_cc_next_cred()
 * calls.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the next entry from the credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  cursor          Cursor
 * @param [out] creds           Next credential cache entry
 *
 * This function fills in @a creds with the next entry in @a cache and advances
 * @a cursor.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @sa krb5_cc_start_seq_get(), krb5_end_seq_get()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Finish a series of sequential processing credential cache entries.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] cursor           Cursor
 *
 * This function finishes processing credential cache entries and invalidates
 * @a cursor.
 *
 * @sa krb5_cc_start_seq_get(), krb5_cc_next_cred()
 *
 * @retval 0 (always)
 */
        /* *
 * Remove credentials from a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] flags            Bitwise-ORed search flags
 * @param [in] creds            Credentials to be matched
 *
 * @warning This function is not implemented for some cache types.
 *
 * This function accepts the same flag values as krb5_cc_retrieve_cred().
 *
 * @retval KRB5_CC_NOSUPP Not implemented for this cache type
 * @return No matches found; Data cannot be deleted; Kerberos error codes
 */
        /* *
 * Set options flags on a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] flags            Flag bit mask
 *
 * This function resets @a cache flags to @a flags.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve flags from a credential cache structure.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] flags           Flag bit mask
 *
 * @warning For memory credential cache always returns a flag mask of 0.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @return The type of a credential cache as an alias that must not be modified
 * or freed by the caller.
 */
        /* *
 * Move a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] src              The credential cache to move the content from
 * @param [in] dst              The credential cache to move the content to
 *
 * This function reinitializes @a dst and populates it with the credentials and
 * default principal of @a src; then, if successful, destroys @a src.
 *
 * @retval
 * 0 Success; @a src is closed.
 * @return
 * Kerberos error codes; @a src is still allocated.
 */
        /* *
 * Prepare to iterate over the collection of known credential caches.
 *
 * @param [in]  context         Library context
 * @param [out] cursor          Cursor
 *
 * Get a new cache iteration @a cursor that will iterate over all known
 * credential caches independent of type.
 *
 * Use krb5_cccol_cursor_free() to release @a cursor when it is no longer
 * needed.
 *
 * @sa krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Get the next credential cache in the collection.
 *
 * @param [in]  context         Library context
 * @param [in]  cursor          Cursor
 * @param [out] ccache          Credential cache handle
 *
 * @note When all caches are iterated over and the end of the list is reached,
 * @a ccache is set to NULL.
 *
 * Use krb5_cc_close() to close @a ccache when it is no longer needed.
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_free()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Free a credential cache collection cursor.
 *
 * @param [in] context          Library context
 * @param [in] cursor           Cursor
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Check if the credential cache collection contains any credentials.
 *
 * @param [in]  context         Library context
 *
 * @version New in 1.11
 *
 * @retval 0 Credentials are available in the collection
 * @retval KRB5_CC_NOTFOUND The collection contains no credentials
 */
        /* *
 * Create a new credential cache of the specified type with a unique name.
 *
 * @param [in]  context         Library context
 * @param [in]  type            Credential cache type name
 * @param [in]  hint            Unused
 * @param [out] id              Credential cache handle
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /*
 * end "ccache.h"
 */
        /*
 * begin "rcache.h"
 */
        /*
 * end "rcache.h"
 */
        /*
 * begin "keytab.h"
 */
        /* XXX */
        /* *< Long enough for MAXPATHLEN + some extra */
        /* * A key table entry. */
        /* *< Principal of this key */
        /* *< Time entry written to keytable */
        /* *< Key version number */
        /* *< The secret key */
        /* *
 * Return the type of a key table.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @return The type of a key table as an alias that must not be modified or
 * freed by the caller.
 */
        /* *
 * Get a key table name.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] name            Key table name
 * @param [in]  namelen         Maximum length to fill in name
 *
 * Fill @a name with the name of @a keytab including the type and delimiter.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_NAME_TOOLONG  Key table name does not fit in @a namelen bytes
 *
 * @return
 * Kerberos error codes
 */
        /* *
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
        /* *
 * Get an entry from a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [in]  principal       Principal name
 * @param [in]  vno             Key version number (0 for highest available)
 * @param [in]  enctype         Encryption type (0 zero for any enctype)
 * @param [out] entry           Returned entry from key table
 *
 * Retrieve an entry from a key table which matches the @a keytab, @a
 * principal, @a vno, and @a enctype.  If @a vno is zero, retrieve the
 * highest-numbered kvno matching the other fields.  If @a enctype is 0, match
 * any enctype.
 *
 * Use krb5_free_keytab_entry_contents() to free @a entry when it is no longer
 * needed.
 *
 * @note If @a vno is zero, the function retrieves the highest-numbered-kvno
 * entry that matches the specified principal.
 *
 * @retval
 * 0 Success
 * @retval
 * Kerberos error codes on failure
 */
        /* *
 * Start a sequential retrieval of key table entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * Prepare to read sequentially every key in the specified key table.  Use
 * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
 *
 * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Retrieve the next entry from the key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] entry           Returned key table entry
 * @param [in]  cursor          Key table cursor
 *
 * Return the next sequential entry in @a keytab and advance @a cursor.
 * Callers must release the returned entry with krb5_kt_free_entry().
 *
 * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_END - if the last entry was reached
 * @return
 * Kerberos error codes
 */
        /* *
 * Release a keytab cursor.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * This function should be called to release the cursor created by
 * krb5_kt_start_seq_get().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Check if a keytab exists and contains entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 *
 * @version New in 1.11
 *
 * @retval 0 Keytab exists and contains entries
 * @retval KRB5_KT_NOTFOUND Keytab does not contain entries
 */
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Create a krb5 library context using only configuration files.
 *
 * @param [out] context         Library context
 *
 * Create a context structure, using only system configuration files.  All
 * information passed through the environment variables is ignored.
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Create a krb5 library context using a specified profile.
 *
 * @param [in]  profile         Profile object (NULL to create default profile)
 * @param [in]  flags           Context initialization flags
 * @param [out] context         Library context
 *
 * Create a context structure, optionally using a specified profile and
 * initialization flags.  If @a profile is NULL, the default profile will be
 * created from config files.  If @a profile is non-null, a copy of it will be
 * made for the new context; the caller should still clean up its copy.  Valid
 * flag values are:
 *
 * @li #KRB5_INIT_CONTEXT_SECURE Ignore environment variables
 * @li #KRB5_INIT_CONTEXT_KDC    Use KDC configuration if creating profile
 */
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        /* *
 * Copy a krb5_context structure.
 *
 * @param [in]  ctx             Library context
 * @param [out] nctx_out        New context structure
 *
 * The newly created context must be released by calling krb5_free_context()
 * when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Set default TGS encryption types in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] etypes           Encryption type(s) to set
 *
 * This function sets the default enctype list for TGS requests
 * made using @a context to @a etypes.
 *
 * @note This overrides the default list (from config file or built-in).
 *
 * @retval
 *  0    Success
 * @retval
 *  KRB5_PROG_ETYPE_NOSUPP Program lacks support for encryption type
 * @return
 * Kerberos error codes
 */
        /* *
 * Return a list of encryption types permitted for session keys.
 *
 * @param [in]  context         Library context
 * @param [out] ktypes          Zero-terminated list of encryption types
 *
 * This function returns the list of encryption types permitted for session
 * keys within @a context, as determined by configuration or by a previous call
 * to krb5_set_default_tgs_enctypes().
 *
 * Use krb5_free_enctypes() to free @a ktypes when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Test whether the Kerberos library was built with multithread support.
 *
 * @retval
 * TRUE if the library is threadsafe; FALSE otherwise
 */
        /* libkrb.spec */
        /* *
 * Decrypt a ticket using the specified key table.
 *
 * @param [in] context          Library context
 * @param [in] kt               Key table
 * @param [in] ticket           Ticket to be decrypted
 *
 * This function takes a @a ticket as input and decrypts it using
 * key data from @a kt.  The result is placed into @a ticket->enc_part2.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Free an array of credential structures.
 *
 * @param [in] context          Library context
 * @param [in] tgts             Null-terminated array of credentials to free
 *
 * @note The last entry in the array @a tgts must be a NULL pointer.
 */
        /* * @defgroup KRB5_GC  KRB5_GC
 * @{
 */
        /* *< Want user-user ticket */
        /* *< Want cached ticket only */
        /* *< Set canonicalize KDC option */
        /* *< Do not store in credential cache */
        /* *< Acquire forwardable tickets */
        /* *< Disable transited check */
        /* *< Constrained delegation */
        /* * @} */
        /* end of KRB5_GC group */
        /* *
 * Get an additional ticket.
 *
 * @param [in]  context         Library context
 * @param [in]  options         Options
 * @param [in]  ccache          Credential cache handle
 * @param [in]  in_creds        Input credentials
 * @param [out] out_creds       Output updated credentials
 *
 * Use @a ccache or a TGS exchange to get a service ticket matching @a
 * in_creds.
 *
 * Valid values for @a options are:
 * @li #KRB5_GC_CACHED     Search only credential cache for the ticket
 * @li #KRB5_GC_USER_USER  Return a user to user authentication ticket
 *
 * @a in_creds must be non-null.  @a in_creds->client and @a in_creds->server
 * must be filled in to specify the client and the server respectively.  If any
 * authorization data needs to be requested for the service ticket (such as
 * restrictions on how the ticket can be used), specify it in @a
 * in_creds->authdata; otherwise set @a in_creds->authdata to NULL.  The
 * session key type is specified in @a in_creds->keyblock.enctype, if it is
 * nonzero.
 *
 * The expiration date is specified in @a in_creds->times.endtime.
 * The KDC may return tickets with an earlier expiration date.
 * If @a in_creds->times.endtime is set to 0, the latest possible
 * expiration date will be requested.
 *
 * Any returned ticket and intermediate ticket-granting tickets are stored
 * in @a ccache.
 *
 * Use krb5_free_creds() to free @a out_creds when it is no longer needed.
 *
 * @retval
 *  0  Success
 * @return
 * Kerberos error codes
 */
        /* * @deprecated Replaced by krb5_get_validated_creds. */
        /* * @deprecated Replaced by krb5_get_renewed_creds. */
        /* *
 * Create a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     service        Service name, or NULL to use @c "host"
 * @param [in]     hostname       Host name, or NULL to use local hostname
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     ccache         Credential cache used to obtain credentials
 *                                for the desired service.
 * @param [out]    outbuf         @c AP-REQ message
 *
 * This function is similar to krb5_mk_req_extended() except that it uses a
 * given @a hostname, @a service, and @a ccache to construct a service
 * principal name and obtain credentials.
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Create a @c KRB_AP_REQ message using supplied credentials.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     in_creds       Credentials for the service with valid ticket
 *                                and key
 * @param [out]    outbuf         @c AP-REQ message
 *
 * Valid @a ap_req_options are:
 * @li #AP_OPTS_USE_SESSION_KEY - Use the session key when creating the
 *                                request used for user to user
 *                                authentication.
 * @li #AP_OPTS_MUTUAL_REQUIRED - Request a mutual authentication packet from
 *                                the reciever.
 * @li #AP_OPTS_USE_SUBKEY      - Generate a subsession key from the current
 *                                session key obtained from the credentials.
 *
 * This function creates a KRB_AP_REQ message using supplied credentials @a
 * in_creds.  @a auth_context may point to an existing auth context or to NULL,
 * in which case a new one will be created.  If @a in_data is non-null, a
 * checksum of it will be included in the authenticator contained in the
 * KRB_AP_REQ message.  Use krb5_free_data_contents() to free @a outbuf when it
 * is no longer needed.
 *
 * On successful return, the authenticator is stored in @a auth_context with
 * the @a client and @a checksum fields nulled out.  (This is to prevent
 * pointer-sharing problems; the caller should not need these fields anyway,
 * since the caller supplied them.)
 *
 * @sa krb5_mk_req()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format and encrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * This function fills in @a outbuf with an AP-REP message using information
 * from @a auth_context.
 *
 * If the flags in @a auth_context indicate that a sequence number should be
 * used (either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or
 * #KRB5_AUTH_CONTEXT_RET_SEQUENCE) and the local sequence number in @a
 * auth_context is 0, a new number will be generated with
 * krb5_generate_seq_number().
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format and encrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Parse and decrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] repl            Decrypted reply message
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a repl with a pointer to allocated memory containing the fields
 * from the encrypted response.
 *
 * Use krb5_free_ap_rep_enc_part() to free @a repl when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Parse and decrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] nonce           Sequence number from the decrypted reply
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a nonce with a decrypted reply sequence number.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format and encode a @c KRB_ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  dec_err         Error structure to be encoded
 * @param [out] enc_err         Encoded error structure
 *
 * This function creates a @c KRB_ERROR message in @a enc_err.  Use
 * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Decode a @c KRB-ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  enc_errbuf      Encoded error message
 * @param [out] dec_error       Decoded error message
 *
 * This function processes @c KRB-ERROR message @a enc_errbuf and returns
 * an allocated structure @a dec_error containing the error message.
 * Use krb5_free_error() to free @a dec_error when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Process @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           @c KRB-SAFE message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-SAFE message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-SAFE message, verifies its integrity, and
 * stores its data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-SAFE message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-SAFE message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-SAFE message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Process a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication structure
 * @param [in]  inbuf           @c KRB-PRIV message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-PRIV message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-PRIV message, verifies its integrity, and
 * stores its unencrypted data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-PRIV message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-PRIV message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-PRIV message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *< Error if realm is present */
        /* *< Error if realm is not present */
        /* *< Create single-component
                                                  enterprise principle */
        /* *< Ignore realm if present */
        /* *
 * Convert a string principal name to a krb5_principal with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [in]  flags           Flag
 * @param [out] principal_out   New principal
 *
 * Similar to krb5_parse_name(), this function converts a single-string
 * representation of a principal name to a krb5_principal structure.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_PARSE_NO_REALM - no realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_REQUIRE_REALM - realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_ENTERPRISE - create single-component enterprise
 *                                        principal
 * @li #KRB5_PRINCIPAL_PARSE_IGNORE_REALM - ignore realm if present in @a name
 *
 * If @c KRB5_PRINCIPAL_PARSE_NO_REALM or @c KRB5_PRINCIPAL_PARSE_IGNORE_REALM
 * is specified in @a flags, the realm of the new principal will be empty.
 * Otherwise, the default realm for @a context will be used if @a name does not
 * specify a realm.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert a krb5_principal structure to a string representation.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [out] name            String representation of principal name
 *
 * The resulting string representation uses the format and quoting conventions
 * described for krb5_parse_name().
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert krb5_principal structure to string and length.
 *
 * @param [in]     context      Library context
 * @param [in]     principal    Principal
 * @param [in,out] name         String representation of principal name
 * @param [in,out] size         Size of unparsed name
 *
 * This function is similar to krb5_unparse_name(), but allows the use of an
 * existing buffer for the result.  If size is not NULL, then @a name must
 * point to either NULL or an existing buffer of at least the size pointed to
 * by @a size.  The buffer will be allocated or resized if necessary, with the
 * new pointer stored into @a name.  Whether or not the buffer is resized, the
 * necessary space for the result, including null terminator, will be stored
 * into @a size.
 *
 * If size is NULL, this function behaves exactly as krb5_unparse_name().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        /* *< Omit realm if it is the local realm */
        /* *< Omit realm always */
        /* *< Don't escape special characters */
        /* *
 * Convert krb5_principal structure to a string with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            String representation of principal name
 *
 * Similar to krb5_unparse_name(), this function converts a krb5_principal
 * structure to a string representation.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_UNPARSE_SHORT - omit realm if it is the local realm
 * @li #KRB5_PRINCIPAL_UNPARSE_NO_REALM - omit realm
 * @li #KRB5_PRINCIPAL_UNPARSE_DISPLAY - do not quote special characters
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        /* *
 * Convert krb5_principal structure to string format with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            Single string format of principal name
 * @param [out] size            Size of unparsed name buffer
 *
 * @sa krb5_unparse_name() krb5_unparse_name_flags() krb5_unparse_name_ext()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        /* *
 * Set the realm field of a principal
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] realm            Realm name
 *
 * Set the realm name part of @a principal to @a realm, overwriting the
 * previous realm.
 *
 * @retval
 * 0   Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Search a list of addresses for a specified address.
 *
 * @param [in] context          Library context
 * @param [in] addr             Address to search for
 * @param [in] addrlist         Address list to be searched (or NULL)
 *
 * @note If @a addrlist contains only a NetBIOS addresses, it will be treated
 *       as a null list.
 *
 * @return
 * TRUE if @a addr is listed in @a addrlist, or @c addrlist is NULL; FALSE
 * otherwise
 */
        /* *
 * Compare two Kerberos addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address to be compared
 * @param [in] addr2            Second address to be compared
 *
 * @return
 * TRUE if the addresses are the same, FALSE otherwise
 */
        /* *
 * Return an ordering of the specified addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address
 * @param [in] addr2            Second address
 *
 * @retval
 *  0 The two addresses are the same
 * @retval
 *  \< 0 First address is less than second
 * @retval
 *  \> 0 First address is greater than second
 */
        /* *
 * Compare the realms of two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the realm names are the same; FALSE otherwise
 */
        /* *
 * Compare two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        /* *
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        /* *< ignore realm component */
        /* *< UPNs as real principals */
        /* *< case-insensitive */
        /* *< treat principals as UTF-8 */
        /* *
 * Compare two principals with additional flags.
 *
 * @param [in] context           Library context
 * @param [in] princ1            First principal
 * @param [in] princ2            Second principal
 * @param [in] flags             Flags
 *
 * Valid flags are:
 * @li #KRB5_PRINCIPAL_COMPARE_IGNORE_REALM - ignore realm component
 * @li #KRB5_PRINCIPAL_COMPARE_ENTERPRISE - UPNs as real principals
 * @li #KRB5_PRINCIPAL_COMPARE_CASEFOLD case-insensitive
 * @li #KRB5_PRINCIPAL_COMPARE_UTF8 - treat principals as UTF-8
 *
 * @sa krb5_principal_compare()
 *
 * @retval
 * TRUE if the principal names are the same; FALSE otherwise
 */
        /* *
 * Initialize an empty @c krb5_keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  length          Length of keyblock (or 0)
 * @param [out] out             New keyblock structure
 *
 * Initialize a new keyblock and allocate storage for the contents of the key.
 * It is legal to pass in a length of 0, in which case contents are left
 * unallocated.  Use krb5_free_keyblock() to free @a out when it is no longer
 * needed.
 *
 * @note If @a length is set to 0, contents are left unallocated.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Keyblock to be copied
 * @param [out] to              Copy of keyblock @a from
 *
 * This function creates a new keyblock with the same contents as @a from.  Use
 * krb5_free_keyblock() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy the contents of a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Key to be copied
 * @param [out] to              Output key
 *
 * This function copies the contents of @a from to @a to.  Use
 * krb5_free_keyblock_contents() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_creds structure.
 *
 * @param [in]  context         Library context
 * @param [in]  incred          Credentials structure to be copied
 * @param [out] outcred         Copy of @a incred
 *
 * This function creates a new credential with the contents of @a incred.  Use
 * krb5_free_creds() to free @a outcred when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy an array of addresses.
 *
 * @param [in]  context         Library context
 * @param [in]  inaddr          Array of addresses to be copied
 * @param [out] outaddr         Copy of array of addresses
 *
 * This function creates a new address array containing a copy of @a inaddr.
 * Use krb5_free_addresses() to free @a outaddr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_ticket structure.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Ticket to be copied
 * @param [out] pto             Copy of ticket
 *
 * This function creates a new krb5_ticket structure containing the contents of
 * @a from.  Use krb5_free_ticket() to free @a pto when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy an authorization data list.
 *
 * @param [in]  context         Library context
 * @param [in]  in_authdat      List of @a krb5_authdata structures
 * @param [out] out             New array of @a krb5_authdata structures
 *
 * This function creates a new authorization data list containing a copy of @a
 * in_authdat, which must be null-terminated.  Use krb5_free_authdata() to free
 * @a out when it is no longer needed.
 *
 * @note The last array entry in @a in_authdat must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
        /* *
 * Merge two authorization data lists into a new list.
 *
 * @param [in]  context         Library context
 * @param [in]  inauthdat1      First list of @a krb5_authdata structures
 * @param [in]  inauthdat2      Second list of @a krb5_authdata structures
 * @param [out] outauthdat      Merged list of @a krb5_authdata structures
 *
 * Merge two authdata arrays, such as the array from a ticket
 * and authenticator.
 * Use krb5_free_authdata() to free @a outauthdat when it is no longer needed.
 *
 * @note The last array entry in @a inauthdat1 and @a inauthdat2
 * must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_authenticator structure.
 *
 * @param [in]  context         Library context
 * @param [in]  authfrom        krb5_authenticator structure to be copied
 * @param [out] authto          Copy of krb5_authenticator structure
 *
 * This function creates a new krb5_authenticator structure with the content of
 * @a authfrom.  Use krb5_free_authenticator() to free @a authto when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_checksum structure.
 *
 * @param [in]  context         Library context
 * @param [in]  ckfrom          Checksum to be copied
 * @param [out] ckto            Copy of krb5_checksum structure
 *
 * This function creates a new krb5_checksum structure with the contents of @a
 * ckfrom.  Use krb5_free_checksum() to free @a ckto when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate a replay cache object for server use and open it.
 *
 * @param [in]  context         Library context
 * @param [in]  piece           Unused (replay cache identifier)
 * @param [out] rcptr           Handle to an open rcache
 *
 * This function creates a handle to the default replay cache.  Use
 * krb5_rc_close() to close @a rcptr when it is no longer needed.
 *
 * @version Prior to release 1.18, this function creates a handle to a
 * different replay cache for each unique value of @a piece.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Build a principal name using length-counted strings.
 *
 * @param [in]  context  Library context
 * @param [out] princ    Principal name
 * @param [in]  rlen     Realm name length
 * @param [in]  realm    Realm name
 * @param [in]  ...      List of unsigned int/char * components, followed by 0
 *
 * This function creates a principal from a length-counted string and a
 * variable-length list of length-counted components.  The list of components
 * ends with the first 0 length argument (so it is not possible to specify an
 * empty component with this function).  Call krb5_free_principal() to free
 * allocated memory for principal when it is no longer needed.
 *
 * @code
 * Example of how to build principal WELLKNOWN/ANONYMOUS@R
 *     krb5_build_principal_ext(context, &principal, strlen("R"), "R",
 *         (unsigned int)strlen(KRB5_WELLKNOWN_NAMESTR),
 *         KRB5_WELLKNOWN_NAMESTR,
 *         (unsigned int)strlen(KRB5_ANONYMOUS_PRINCSTR),
 *         KRB5_ANONYMOUS_PRINCSTR, 0);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Build a principal name using null-terminated strings.
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal name
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ...             List of char * components, ending with NULL
 *
 * Call krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
 * same task.  krb5_build_principal() takes variadic arguments.
 * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
 *
 * @code
 * Example of how to build principal H/S@R
 *     krb5_build_principal(context, &principal,
 *                          strlen("R"), "R", "H", "S", (char*)NULL);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* * @deprecated Replaced by krb5_build_principal_alloc_va(). */
        /* *
 * Build a principal name, using a precomputed variable argument list
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal structure
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ap              List of char * components, ending with NULL
 *
 * Similar to krb5_build_principal(), this function builds a principal name,
 * but its name components are specified as a va_list.
 *
 * Use krb5_free_principal() to deallocate @a princ when it is no longer
 * needed.
 *
 * @code
 * Function usage example:
 *   va_list ap;
 *   va_start(ap, realm);
 *   krb5_build_principal_alloc_va(context, princ, rlen, realm, ap);
 *   va_end(ap);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert a Kerberos V4 principal to a Kerberos V5 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  name            V4 name
 * @param [in]  instance        V4 instance
 * @param [in]  realm           Realm
 * @param [out] princ           V5 principal
 *
 * This function builds a @a princ from V4 specification based on given input
 * @a name.instance\@realm.
 *
 * Use krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Convert a Kerberos V5 principal to a Kerberos V4 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  princ           V5 Principal
 * @param [out] name            V4 principal's name to be filled in
 * @param [out] inst            V4 principal's instance name to be filled in
 * @param [out] realm           Principal's realm name to be filled in
 *
 * This function separates a V5 principal @a princ into @a name, @a instance,
 * and @a realm.
 *
 * @retval
 *  0  Success
 * @retval
 *  KRB5_INVALID_PRINCIPAL   Invalid principal name
 * @retval
 *  KRB5_CONFIG_CANTOPEN     Can't open or find Kerberos configuration file
 * @return
 * Kerberos error codes
 */
        /* *
 *@deprecated
 */
        /* *
 * Convert a Kerberos V5 credentials to a Kerberos V4 credentials
 *
 * @note Not implemented
 *
 * @retval KRB524_KRB4_DISABLED (always)
 */
        /* libkt.spec */
        /* *
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2330:1"]
        pub fn krb5_cc_get_name(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "2368:1"]
        pub fn krb5_cc_initialize(context: krb5_context, cache: krb5_ccache,
                                  principal: krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2386:1"]
        pub fn krb5_cc_destroy(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2457:1"]
        pub fn krb5_cc_retrieve_cred(context: krb5_context,
                                     cache: krb5_ccache, flags: krb5_flags,
                                     mcreds: *mut krb5_creds,
                                     creds: *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2598:1"]
        pub fn krb5_cc_get_type(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3468:1"]
        pub fn krb5_parse_name_flags(context: krb5_context,
                                     name: *const libc::c_char,
                                     flags: libc::c_int,
                                     principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3586:1"]
        pub fn krb5_set_principal_realm(context: krb5_context,
                                        principal: krb5_principal,
                                        realm: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "6932:1"]
        pub fn krb5_get_init_creds_opt_set_anonymous(opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     anonymous: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6907:1"]
        pub fn krb5_get_init_creds_opt_set_proxiable(opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     proxiable: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6897:1"]
        pub fn krb5_get_init_creds_opt_set_forwardable(opt:
                                                           *mut krb5_get_init_creds_opt,
                                                       forwardable:
                                                           libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6863:1"]
        pub fn krb5_get_init_creds_opt_free(context: krb5_context,
                                            opt:
                                                *mut krb5_get_init_creds_opt);
        #[no_mangle]
        #[c2rust::src_loc = "6851:1"]
        pub fn krb5_get_init_creds_opt_alloc(context: krb5_context,
                                             opt:
                                                 *mut *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6461:1"]
        pub fn krb5_prompter_posix(context: krb5_context,
                                   data: *mut libc::c_void,
                                   name: *const libc::c_char,
                                   banner: *const libc::c_char,
                                   num_prompts: libc::c_int,
                                   prompts: *mut krb5_prompt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7097:1"]
        pub fn krb5_get_init_creds_opt_set_out_ccache(context: krb5_context,
                                                      opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      ccache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7268:1"]
        pub fn krb5_get_init_creds_password(context: krb5_context,
                                            creds: *mut krb5_creds,
                                            client: krb5_principal,
                                            password: *const libc::c_char,
                                            prompter: krb5_prompter_fct,
                                            data: *mut libc::c_void,
                                            start_time: krb5_deltat,
                                            in_tkt_service:
                                                *const libc::c_char,
                                            k5_gic_options:
                                                *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7661:1"]
        pub fn krb5_get_init_creds_keytab(context: krb5_context,
                                          creds: *mut krb5_creds,
                                          client: krb5_principal,
                                          arg_keytab: krb5_keytab,
                                          start_time: krb5_deltat,
                                          in_tkt_service: *const libc::c_char,
                                          k5_gic_options:
                                              *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:32"]
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
    /*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
    /* Write the SHA-256 hash of in (containing n elements) to out. */
    /* Convenience function: zap and free ptr if it is non-NULL. */
    /* Convenience function: zap and free zero-terminated str if it is non-NULL. */
    /* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
    /*
 * End "los-proto.h"
 */
    /*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
    /* lock mode flags */
    /*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
    /* check logon hour restrictions */
    /* sign with usage 27 instead of 26 */
    /* padata from req_body is used*/
    /* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
    /*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
    /* Does not return a copy; original padata sequence responsible for freeing*/
    /* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
    /* Free a single pa-data object. */
    /* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
    /* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
    /* Add an empty pa-data element of type pa_type to *list. */
    /* KRB5_PREAUTH__ */
    /*
 * End "preauth.h"
 */
    /* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
    /* ** Plugin framework ***/
    /*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
    /* Holds krb5_context information about each pluggable interface. */
    /* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
    /* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
    /* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
    /* Release a module list allocated by k5_plugin_load_all. */
    /* Register a plugin module of type interface_id and name modname. */
    /*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
    /* Destroy the module state within context; used by krb5_free_context. */
    /* private, in kdb5.h */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    /* could be used in a table to find an etype and initialize a block */
    /* internal message representations */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time,
                                           optional */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* data integrity checksum */
    /* encrypted part */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time, opt. */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /*
 * Begin "asn1.h"
 */
    /* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
/* here we use some knowledge of ASN.1 encodings */
/*
  Ticket is APPLICATION 1.
  Authenticator is APPLICATION 2.
  AS_REQ is APPLICATION 10.
  AS_REP is APPLICATION 11.
  TGS_REQ is APPLICATION 12.
  TGS_REP is APPLICATION 13.
  AP_REQ is APPLICATION 14.
  AP_REP is APPLICATION 15.
  KRB_SAFE is APPLICATION 20.
  KRB_PRIV is APPLICATION 21.
  KRB_CRED is APPLICATION 22.
  EncASRepPart is APPLICATION 25.
  EncTGSRepPart is APPLICATION 26.
  EncAPRepPart is APPLICATION 27.
  EncKrbPrivPart is APPLICATION 28.
  EncKrbCredPart is APPLICATION 29.
  KRB_ERROR is APPLICATION 30.
*/
/* allow either constructed or primitive encoding, so check for bit 6
   set or reset */
    /* ************************************************************************
 * Prototypes for krb5_encode.c
 *************************************************************************/
    /*
  krb5_error_code encode_krb5_structure(const krb5_structure *rep,
  krb5_data **code);
  modifies  *code
  effects   Returns the ASN.1 encoding of *rep in **code.
  Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
  Returns ENOMEM if memory runs out.
*/
    /* yes, the translation is identical to that used for KDC__REP */
    /* yes, the translation is identical to that used for KDC__REP */
    /* ************************************************************************
 * End of prototypes for krb5_encode.c
 *************************************************************************/
    /* ************************************************************************
 * Prototypes for krb5_decode.c
 *************************************************************************/
/*
  krb5_error_code decode_krb5_structure(const krb5_data *code,
  krb5_structure **rep);

  requires  Expects **rep to not have been allocated;
  a new *rep is allocated regardless of the old value.
  effects   Decodes *code into **rep.
  Returns ENOMEM if memory is exhausted.
  Returns asn1 and krb5 errors.
*/
    /* kdb.h */
    /* Master key version number */
    /* kvno of key_data elements (all the same) */
    /* ************************************************************************
 * End of prototypes for krb5_decode.c
 *************************************************************************/
    /* KRB5_ASN1__ */
    /*
 * End "asn1.h"
 */
    /*
 * Internal krb5 library routines
 */
    /* Return true if s is non-empty and composed solely of digits. */
    /*
 * Initialization routines.
 */
    /* [De]serialize 4-byte integer */
    /* [De]serialize 8-byte integer */
    /* [De]serialize byte string */
    /* Fill in the buffer with random alpha-numeric data. */
    /* value to use when requesting a keytab entry and KVNO doesn't matter */
    /* value to use when requesting a keytab entry and enctype doesn't matter */
    /* To keep happy libraries which are (for now) accessing internal stuff */
    /* Make sure to increment by one when changing the struct */
    /* Used for KDB LDAP back end.  */
    /*
     * pkinit asn.1 encode/decode functions
     */
    /* Set *tag_out to the integrity tag of *enc.  (Does not allocate memory;
 * returned buffer is a subrange of *ctext.) */
    /*
 * This structure was exposed and used in macros in krb5 1.2, so do not
 * change its ABI.
 */
    /* routines always present */
    /* routines to be included on extended version (write routines) */
    /* Not sure it's ready for exposure just yet.  */
    /*
 * Referral definitions and subfunctions.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2076:8"]
    pub struct _krb5_kt {
        pub magic: krb5_magic,
        pub ops: *const _krb5_kt_ops,
        pub data: krb5_pointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2040:16"]
    pub struct _krb5_kt_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *const libc::c_char,
                                                 _: *mut krb5_keytab)
                                -> krb5_error_code>,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_uint)
                                 -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_keytab)
                              -> krb5_error_code>,
        pub get: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: krb5_const_principal,
                                             _: krb5_kvno, _: krb5_enctype,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub start_seq_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_keytab,
                                                       _: *mut krb5_kt_cursor)
                                      -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut krb5_keytab_entry,
                                                  _: *mut krb5_kt_cursor)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_keytab,
                                                 _: *mut krb5_kt_cursor)
                                -> krb5_error_code>,
        pub add: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub remove: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_keytab,
                                                _: *mut krb5_keytab_entry)
                               -> krb5_error_code>,
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:32"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
    /*
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:32"]
pub mod k5_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
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
    /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
    /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:32"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:32"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:32"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:32"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:32"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_un;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                       __len: socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:32"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
}
#[c2rust::header_src = "/usr/include/netdb.h:32"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "565:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:41"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno, krb5_context, krb5_error_code};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "294:1"]
        pub fn kadm5_get_config_params(context: krb5_context,
                                       use_kdc_config: libc::c_int,
                                       params_in: *mut kadm5_config_params,
                                       params_out: *mut kadm5_config_params)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "299:1"]
        pub fn kadm5_free_config_params(context: krb5_context,
                                        params: *mut kadm5_config_params)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "302:1"]
        pub fn kadm5_get_admin_service_name(_: krb5_context,
                                            _: *mut libc::c_char,
                                            _: *mut libc::c_char, _: size_t)
         -> krb5_error_code;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:41"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
    /* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)types.h 1.18 87/07/24 SMI      */
    /*
 * Rpc additions to <sys/types.h>
 */
    /*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
    /* #include <netdb.h> */
    /* Get htonl(), ntohl(), etc. */
    /* Define if we need to fake up some BSD type aliases. */
    /* Allow application to override. */
    /* #undef GSSRPC__BSD_TYPEALIASES */
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:41"]
pub mod xdr_h {
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct XDR {
        pub x_op: xdr_op,
        pub x_ops: *mut xdr_ops,
        pub x_public: caddr_t,
        pub x_private: *mut libc::c_void,
        pub x_base: caddr_t,
        pub x_handy: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct xdr_ops {
        pub x_getlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_putlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_getbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_putbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_getpostn: Option<unsafe extern "C" fn(_: *mut XDR) -> u_int>,
        pub x_setpostn: Option<unsafe extern "C" fn(_: *mut XDR, _: u_int)
                                   -> libc::c_int>,
        pub x_inline: Option<unsafe extern "C" fn(_: *mut XDR, _: libc::c_int)
                                 -> *mut rpc_inline_t>,
        pub x_destroy: Option<unsafe extern "C" fn(_: *mut XDR) -> ()>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:41"]
pub mod auth_h {
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:7"]
    pub union des_block {
        pub c: [libc::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct AUTH {
        pub ah_cred: opaque_auth,
        pub ah_verf: opaque_auth,
        pub ah_key: des_block,
        pub ah_ops: *mut auth_ops,
        pub ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:9"]
    pub struct auth_ops {
        pub ah_nextverf: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_marshal: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR)
                                   -> libc::c_int>,
        pub ah_validate: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                     _: *mut opaque_auth)
                                    -> libc::c_int>,
        pub ah_refresh: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                    _: *mut rpc_msg)
                                   -> libc::c_int>,
        pub ah_destroy: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_wrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                 _: xdrproc_t, _: caddr_t)
                                -> libc::c_int>,
        pub ah_unwrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                   _: xdrproc_t, _: caddr_t)
                                  -> libc::c_int>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::xdr_h::{XDR, xdrproc_t};
    use super::rpc_msg_h::rpc_msg;
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
    /* GSS-API style */
    /* des style (encrypted timestamps) */
    /* short hand unix style */
    /* unix style (uid, gids) */
    /* backward compatibility */
    /* no authentication */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:41"]
pub mod rpc_msg_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:8"]
    pub struct rpc_msg {
        pub rm_xid: uint32_t,
        pub rm_direction: msg_type,
        pub ru: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_0 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_1 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_2 {
        pub RJ_versions: C2RustUnnamed_3,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_3 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_4 {
        pub AR_versions: C2RustUnnamed_6,
        pub AR_results: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_5 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_6 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "65:1"]
    pub type accept_stat = libc::c_uint;
    #[c2rust::src_loc = "71:2"]
    pub const SYSTEM_ERR: accept_stat = 5;
    #[c2rust::src_loc = "70:2"]
    pub const GARBAGE_ARGS: accept_stat = 4;
    #[c2rust::src_loc = "69:2"]
    pub const PROC_UNAVAIL: accept_stat = 3;
    #[c2rust::src_loc = "68:2"]
    pub const PROG_MISMATCH: accept_stat = 2;
    #[c2rust::src_loc = "67:2"]
    pub const PROG_UNAVAIL: accept_stat = 1;
    #[c2rust::src_loc = "66:2"]
    pub const SUCCESS: accept_stat = 0;
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct call_body {
        pub cb_rpcvers: rpcvers_t,
        pub cb_prog: rpcprog_t,
        pub cb_vers: rpcvers_t,
        pub cb_proc: rpcproc_t,
        pub cb_cred: opaque_auth,
        pub cb_verf: opaque_auth,
    }
    #[c2rust::src_loc = "55:1"]
    pub type msg_type = libc::c_uint;
    #[c2rust::src_loc = "57:2"]
    pub const REPLY: msg_type = 1;
    #[c2rust::src_loc = "56:2"]
    pub const CALL: msg_type = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::auth_h::{auth_stat, opaque_auth};
    use super::gssrpc_types_h::{rpcvers_t, rpcprog_t, rpcproc_t};
    use super::sys_types_h::caddr_t;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:41"]
pub mod clnt_h {
    #[c2rust::src_loc = "48:1"]
    pub type clnt_stat = libc::c_uint;
    #[c2rust::src_loc = "83:2"]
    pub const RPC_FAILED: clnt_stat = 16;
    #[c2rust::src_loc = "79:2"]
    pub const RPC_PROGNOTREGISTERED: clnt_stat = 15;
    #[c2rust::src_loc = "78:2"]
    pub const RPC_PMAPFAILURE: clnt_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPC_UNKNOWNPROTO: clnt_stat = 17;
    #[c2rust::src_loc = "72:2"]
    pub const RPC_UNKNOWNHOST: clnt_stat = 13;
    #[c2rust::src_loc = "67:2"]
    pub const RPC_SYSTEMERROR: clnt_stat = 12;
    #[c2rust::src_loc = "66:2"]
    pub const RPC_CANTDECODEARGS: clnt_stat = 11;
    #[c2rust::src_loc = "65:2"]
    pub const RPC_PROCUNAVAIL: clnt_stat = 10;
    #[c2rust::src_loc = "64:2"]
    pub const RPC_PROGVERSMISMATCH: clnt_stat = 9;
    #[c2rust::src_loc = "63:2"]
    pub const RPC_PROGUNAVAIL: clnt_stat = 8;
    #[c2rust::src_loc = "62:2"]
    pub const RPC_AUTHERROR: clnt_stat = 7;
    #[c2rust::src_loc = "61:2"]
    pub const RPC_VERSMISMATCH: clnt_stat = 6;
    #[c2rust::src_loc = "57:2"]
    pub const RPC_TIMEDOUT: clnt_stat = 5;
    #[c2rust::src_loc = "56:2"]
    pub const RPC_CANTRECV: clnt_stat = 4;
    #[c2rust::src_loc = "55:2"]
    pub const RPC_CANTSEND: clnt_stat = 3;
    #[c2rust::src_loc = "54:2"]
    pub const RPC_CANTDECODERES: clnt_stat = 2;
    #[c2rust::src_loc = "53:2"]
    pub const RPC_CANTENCODEARGS: clnt_stat = 1;
    #[c2rust::src_loc = "49:2"]
    pub const RPC_SUCCESS: clnt_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:8"]
    pub struct rpc_err {
        pub re_status: clnt_stat,
        pub ru: C2RustUnnamed_7,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed_7 {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_9,
        pub RE_lb: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_8 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_9 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:16"]
    pub struct CLIENT {
        pub cl_auth: *mut AUTH,
        pub cl_ops: *mut clnt_ops,
        pub cl_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct clnt_ops {
        pub cl_call: Option<unsafe extern "C" fn(_: *mut CLIENT, _: rpcproc_t,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: timeval) -> clnt_stat>,
        pub cl_abort: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_geterr: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                   _: *mut rpc_err) -> ()>,
        pub cl_freeres: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub cl_destroy: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_control: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
    }
    use super::auth_h::{auth_stat, AUTH};
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::{rpcvers_t, rpcproc_t, rpcprog_t};
    use super::xdr_h::xdrproc_t;
    use super::struct_timeval_h::timeval;
    use super::in_h::sockaddr_in;
    use super::sys_types_h::u_int;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn gssrpc_clnttcp_create(_: *mut sockaddr_in, _: rpcprog_t,
                                     _: rpcvers_t, _: *mut libc::c_int,
                                     _: u_int, _: u_int) -> *mut CLIENT;
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:41"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        /* Function Prototypes */
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                _: OM_uint32, _: gss_OID_set,
                                _: gss_cred_usage_t, _: *mut gss_cred_id_t,
                                _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gss.h:41"]
pub mod auth_gss_h {
    /* RPCSEC_GSS services. */
    #[c2rust::src_loc = "59:9"]
    pub type rpc_gss_svc_t = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const RPCSEC_GSS_SVC_PRIVACY: rpc_gss_svc_t = 3;
    #[c2rust::src_loc = "61:2"]
    pub const RPCSEC_GSS_SVC_INTEGRITY: rpc_gss_svc_t = 2;
    #[c2rust::src_loc = "60:2"]
    pub const RPCSEC_GSS_SVC_NONE: rpc_gss_svc_t = 1;
    /* RPCSEC_GSS security triple. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct rpc_gss_sec {
        pub mech: gss_OID,
        pub qop: gss_qop_t,
        pub svc: rpc_gss_svc_t,
        pub cred: gss_cred_id_t,
        pub req_flags: uint32_t,
    }
    use super::gssapi_h::{gss_OID, gss_qop_t, gss_cred_id_t, gss_name_struct,
                          gss_name_t};
    use super::stdint_uintn_h::uint32_t;
    use super::clnt_h::CLIENT;
    use super::auth_h::AUTH;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:1"]
        pub fn gssrpc_authgss_create(_: *mut CLIENT, _: gss_name_t,
                                     _: *mut rpc_gss_sec) -> *mut AUTH;
    }
    /* !defined(GSSRPC_AUTH_GSS_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:41"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_enctype, krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kadm5/clnt/client_internal.h:43"]
pub mod client_internal_h {
    #[c2rust::src_loc = "68:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub cache_name: *mut libc::c_char,
        pub destroy_cache: libc::c_int,
        pub clnt: *mut CLIENT,
        pub client_socket: libc::c_int,
        pub context: krb5_context,
        pub cred: gss_cred_id_t,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
    }
    use super::krb5_h::{krb5_ui_4, krb5_context};
    use super::clnt_h::CLIENT;
    use super::gssapi_h::gss_cred_id_t;
    use super::admin_h::kadm5_config_params;
    /* __KADM5_CLIENT_INTERNAL_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_rpc.h:42"]
pub mod kadm_rpc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:8"]
    pub struct generic_ret {
        pub api_version: krb5_ui_4,
        pub code: kadm5_ret_t,
    }
    use super::krb5_h::krb5_ui_4;
    use super::admin_h::kadm5_ret_t;
    use super::clnt_h::{CLIENT, clnt_stat};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "304:1"]
        pub fn init_2(_: *mut libc::c_void, _: *mut generic_ret,
                      _: *mut CLIENT) -> clnt_stat;
    }
    /* __KADM_RPC_H__ */
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "329:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
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
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:38"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2002,2003,2004 by the Massachusetts Institute of Technology,
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
        /* Approach overview:

   If a system version is available but buggy, save handles to it (via
   inline functions in a support library), redefine the names to refer
   to library functions, and in those functions, call the system
   versions and fix up the returned data.  Use the native data
   structures and flag values.

   If no system version exists, use gethostby* and fake it.  Define
   the data structures and flag values locally.


   On macOS, getaddrinfo results aren't cached (though
   gethostbyname results are), so we need to build a cache here.  Now
   things are getting really messy.  Because the cache is in use, we
   use getservbyname, and throw away thread safety.  (Not that the
   cache is thread safe, but when we get locking support, that'll be
   dealt with.)  This code needs tearing down and rebuilding, soon.


   Note that recent Windows developers' code has an interesting hack:
   When you include the right header files, with the right set of
   macros indicating system versions, you'll get an inline function
   that looks for getaddrinfo (or whatever) in the system library, and
   calls it if it's there.  If it's not there, it fakes it with
   gethostby* calls.

   We're taking a simpler approach: A system provides these routines or
   it does not.

   Someday, we may want to take into account different versions (say,
   different revs of GNU libc) where some are broken in one way, and
   some work or are broken in another way.  Cross that bridge when we
   come to it.  */
        /* To do, maybe:

   + For AIX 4.3.3, using the RFC 2133 definition: Implement
   AI_NUMERICHOST.  It's not defined in the header file.

   For certain (old?) versions of GNU libc, AI_NUMERICHOST is
   defined but not implemented.

   + Use gethostbyname2, inet_aton and other IPv6 or thread-safe
   functions if available.  But, see
   https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=135182 for one
   gethostbyname2 problem on Linux.  And besides, if a platform is
   supporting IPv6 at all, they really should be doing getaddrinfo
   by now.

   + inet_ntop, inet_pton

   + Conditionally export/import the function definitions, so a
   library can have a single copy instead of multiple.

   + Upgrade host requirements to include working implementations of
   these functions, and throw all this away.  Pleeease?  :-)  */
        /* ! HAVE_GETADDRINFO */
        /* Fudge things on older gai implementations.  */
/* AIX 4.3.3 is based on RFC 2133; no AI_NUMERICHOST.  */
        /* Partial RFC 2553 implementations may not have AI_ADDRCONFIG and
   friends, which RFC 3493 says are now part of the getaddrinfo
   interface, and we'll want to use.  */
        /* Call out to stuff defined in libkrb5support.  */
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn krb5int_getaddrinfo(node: *const libc::c_char,
                                   service: *const libc::c_char,
                                   hints: *const addrinfo,
                                   aip: *mut *mut addrinfo) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn krb5int_freeaddrinfo(ai: *mut addrinfo);
    }
    /* FAI_DEFINED */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/kadm_err.h:41"]
pub mod kadm_err_h {
    extern "C" {
        /* for compatibility with older versions... */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn initialize_ovk_error_table();
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/chpass_util_strings.h:41"]
pub mod chpass_util_strings_h {
    extern "C" {
        /* for compatibility with older versions... */
        #[no_mangle]
        #[c2rust::src_loc = "30:1"]
        pub fn initialize_ovku_error_table();
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:49"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "90:33"]
        pub static gss_nt_krb5_principal: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn gss_krb5_ccache_name(minor_status: *mut OM_uint32,
                                    name: *const libc::c_char,
                                    out_name: *mut *const libc::c_char)
         -> OM_uint32;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth_gssapi.h:50"]
pub mod auth_gssapi_h {
    use super::clnt_h::CLIENT;
    use super::gssapi_h::{OM_uint32, gss_cred_id_struct, gss_cred_id_t,
                          gss_name_struct, gss_name_t, gss_OID_desc_struct,
                          gss_OID};
    use super::auth_h::AUTH;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn gssrpc_auth_gssapi_create(clnt: *mut CLIENT,
                                         major_status: *mut OM_uint32,
                                         minor_status: *mut OM_uint32,
                                         claimant_cred_handle: gss_cred_id_t,
                                         target_name: gss_name_t,
                                         mech_type: gss_OID,
                                         req_flags: OM_uint32,
                                         time_req: OM_uint32,
                                         actual_mech_type: *mut gss_OID,
                                         ret_flags: *mut OM_uint32,
                                         time_rec: *mut OM_uint32)
         -> *mut AUTH;
    }
    /* !defined(GSSRPC_AUTH_GSSAPI_H) */
}
pub use self::types_h::{__u_int, __uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __time_t, __suseconds_t, __caddr_t, __socklen_t};
pub use self::sys_types_h::{u_int, caddr_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::unistd_h::{socklen_t, close};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds, krb5_ccache,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _krb5_prompt,
                       krb5_prompt, krb5_prompter_fct,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, _krb5_ccache, krb5_kt_resolve,
                       krb5_cc_get_name, krb5_cc_initialize, krb5_cc_destroy,
                       krb5_cc_close, krb5_cc_retrieve_cred, krb5_cc_get_type,
                       krb5_kt_close, krb5_init_context, krb5_parse_name,
                       krb5_parse_name_flags, krb5_set_principal_realm,
                       krb5_cc_resolve, krb5_free_principal,
                       krb5_free_cred_contents,
                       krb5_get_init_creds_opt_set_anonymous,
                       krb5_get_init_creds_opt_set_proxiable,
                       krb5_get_init_creds_opt_set_forwardable,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_alloc, krb5_prompter_posix,
                       krb5_get_init_creds_opt_set_out_ccache,
                       krb5_get_init_creds_password,
                       krb5_get_init_creds_keytab};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::sys_socket_h::{__CONST_SOCKADDR_ARG, sockaddr_x25, sockaddr_un,
                             sockaddr_ns, sockaddr_iso, sockaddr_ipx,
                             sockaddr_inarp, sockaddr_eon, sockaddr_dl,
                             sockaddr_ax25, sockaddr_at, socket, connect};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::netdb_h::addrinfo;
pub use self::admin_h::{kadm5_ret_t, _kadm5_config_params,
                        kadm5_config_params, kadm5_get_config_params,
                        kadm5_free_config_params,
                        kadm5_get_admin_service_name};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_0, reply_body,
                          C2RustUnnamed_1, rejected_reply, C2RustUnnamed_2,
                          C2RustUnnamed_3, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_4,
                          C2RustUnnamed_5, C2RustUnnamed_6, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_7, C2RustUnnamed_8,
                       C2RustUnnamed_9, CLIENT, clnt_ops,
                       gssrpc_clnttcp_create};
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_uint32, OM_uint32,
                         gss_OID_desc_struct, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_qop_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_acquire_cred, gss_release_cred, gss_import_name,
                         gss_release_name};
pub use self::auth_gss_h::{rpc_gss_svc_t, RPCSEC_GSS_SVC_PRIVACY,
                           RPCSEC_GSS_SVC_INTEGRITY, RPCSEC_GSS_SVC_NONE,
                           rpc_gss_sec, gssrpc_authgss_create};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple};
pub use self::client_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t};
pub use self::kadm_rpc_h::{generic_ret, init_2};
use self::string_h::{memset, strncpy, strdup, strstr};
use self::stdlib_h::{malloc, free};
use self::stdio_h::{snprintf, asprintf};
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_freeaddrinfo};
use self::kadm_err_h::initialize_ovk_error_table;
use self::chpass_util_strings_h::initialize_ovku_error_table;
use self::gssapi_krb5_h::{gss_nt_krb5_principal, gss_mech_krb5,
                          gss_krb5_ccache_name};
use self::auth_gssapi_h::gssrpc_auth_gssapi_create;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
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
#[c2rust::src_loc = "54:1"]
pub type init_type = libc::c_uint;
#[c2rust::src_loc = "54:52"]
pub const INIT_ANONYMOUS: init_type = 3;
#[c2rust::src_loc = "54:40"]
pub const INIT_CREDS: init_type = 2;
#[c2rust::src_loc = "54:29"]
pub const INIT_SKEY: init_type = 1;
#[c2rust::src_loc = "54:18"]
pub const INIT_PASS: init_type = 0;
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn kadm5_init_with_creds(mut context: krb5_context,
                                               mut client_name:
                                                   *mut libc::c_char,
                                               mut ccache: krb5_ccache,
                                               mut service_name:
                                                   *mut libc::c_char,
                                               mut params:
                                                   *mut kadm5_config_params,
                                               mut struct_version: krb5_ui_4,
                                               mut api_version: krb5_ui_4,
                                               mut db_args:
                                                   *mut *mut libc::c_char,
                                               mut server_handle:
                                                   *mut *mut libc::c_void)
 -> kadm5_ret_t {
    return init_any(context, client_name, INIT_CREDS, 0 as *mut libc::c_char,
                    ccache, service_name, params, struct_version, api_version,
                    db_args, server_handle);
}
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn kadm5_init_with_password(mut context: krb5_context,
                                                  mut client_name:
                                                      *mut libc::c_char,
                                                  mut pass: *mut libc::c_char,
                                                  mut service_name:
                                                      *mut libc::c_char,
                                                  mut params:
                                                      *mut kadm5_config_params,
                                                  mut struct_version:
                                                      krb5_ui_4,
                                                  mut api_version: krb5_ui_4,
                                                  mut db_args:
                                                      *mut *mut libc::c_char,
                                                  mut server_handle:
                                                      *mut *mut libc::c_void)
 -> kadm5_ret_t {
    return init_any(context, client_name, INIT_PASS, pass, 0 as krb5_ccache,
                    service_name, params, struct_version, api_version,
                    db_args, server_handle);
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn kadm5_init_anonymous(mut context: krb5_context,
                                              mut client_name:
                                                  *mut libc::c_char,
                                              mut service_name:
                                                  *mut libc::c_char,
                                              mut params:
                                                  *mut kadm5_config_params,
                                              mut struct_version: krb5_ui_4,
                                              mut api_version: krb5_ui_4,
                                              mut db_args:
                                                  *mut *mut libc::c_char,
                                              mut server_handle:
                                                  *mut *mut libc::c_void)
 -> kadm5_ret_t {
    return init_any(context, client_name, INIT_ANONYMOUS,
                    0 as *mut libc::c_char, 0 as krb5_ccache, service_name,
                    params, struct_version, api_version, db_args,
                    server_handle);
}
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub unsafe extern "C" fn kadm5_init(mut context: krb5_context,
                                    mut client_name: *mut libc::c_char,
                                    mut pass: *mut libc::c_char,
                                    mut service_name: *mut libc::c_char,
                                    mut params: *mut kadm5_config_params,
                                    mut struct_version: krb5_ui_4,
                                    mut api_version: krb5_ui_4,
                                    mut db_args: *mut *mut libc::c_char,
                                    mut server_handle: *mut *mut libc::c_void)
 -> kadm5_ret_t {
    return init_any(context, client_name, INIT_PASS, pass, 0 as krb5_ccache,
                    service_name, params, struct_version, api_version,
                    db_args, server_handle);
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn kadm5_init_with_skey(mut context: krb5_context,
                                              mut client_name:
                                                  *mut libc::c_char,
                                              mut keytab: *mut libc::c_char,
                                              mut service_name:
                                                  *mut libc::c_char,
                                              mut params:
                                                  *mut kadm5_config_params,
                                              mut struct_version: krb5_ui_4,
                                              mut api_version: krb5_ui_4,
                                              mut db_args:
                                                  *mut *mut libc::c_char,
                                              mut server_handle:
                                                  *mut *mut libc::c_void)
 -> kadm5_ret_t {
    return init_any(context, client_name, INIT_SKEY, keytab, 0 as krb5_ccache,
                    service_name, params, struct_version, api_version,
                    db_args, server_handle);
}
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn init_any(mut context: krb5_context,
                              mut client_name: *mut libc::c_char,
                              mut init_type: init_type,
                              mut pass: *mut libc::c_char,
                              mut ccache_in: krb5_ccache,
                              mut service_name: *mut libc::c_char,
                              mut params_in: *mut kadm5_config_params,
                              mut struct_version: krb5_ui_4,
                              mut api_version: krb5_ui_4,
                              mut db_args: *mut *mut libc::c_char,
                              mut server_handle: *mut *mut libc::c_void)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut minor_stat: OM_uint32 = 0;
    let mut iprop_enable: krb5_boolean = 0;
    let mut port: libc::c_int = 0;
    let mut rpc_prog: rpcprog_t = 0;
    let mut rpc_vers: rpcvers_t = 0;
    let mut ccache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut client: krb5_principal = 0 as krb5_principal;
    let mut server: krb5_principal = 0 as krb5_principal;
    let mut timeout: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut handle: kadm5_server_handle_t = 0 as *mut _kadm5_server_handle_t;
    let mut params_local: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut r: generic_ret =
        {
            let mut init =
                generic_ret{api_version: 0 as libc::c_int as krb5_ui_4,
                            code: 0 as libc::c_int as kadm5_ret_t,};
            init
        };
    initialize_ovk_error_table();
    initialize_ovku_error_table();
    if server_handle.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    handle =
        malloc(::std::mem::size_of::<_kadm5_server_handle_t>() as
                   libc::c_ulong) as kadm5_server_handle_t;
    if handle.is_null() { return 12 as libc::c_int as kadm5_ret_t }
    memset(handle as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_kadm5_server_handle_t>() as libc::c_ulong);
    (*handle).lhandle =
        malloc(::std::mem::size_of::<_kadm5_server_handle_t>() as
                   libc::c_ulong) as *mut _kadm5_server_handle_t;
    if (*handle).lhandle.is_null() {
        free(handle as *mut libc::c_void);
        return 12 as libc::c_int as kadm5_ret_t
    }
    (*handle).magic_number = 0x12345800 as libc::c_int as krb5_ui_4;
    (*handle).struct_version = struct_version;
    (*handle).api_version = api_version;
    (*handle).clnt = 0 as *mut CLIENT;
    (*handle).client_socket = -(1 as libc::c_int);
    (*handle).cache_name = 0 as *mut libc::c_char;
    (*handle).destroy_cache = 0 as libc::c_int;
    (*handle).context = 0 as krb5_context;
    (*handle).cred = 0 as gss_cred_id_t;
    *(*handle).lhandle = *handle;
    (*(*handle).lhandle).api_version =
        (0x12345700 as libc::c_int | 0x4 as libc::c_int) as krb5_ui_4;
    (*(*handle).lhandle).struct_version =
        (0x12345600 as libc::c_int | 0x1 as libc::c_int) as krb5_ui_4;
    (*(*handle).lhandle).lhandle = (*handle).lhandle;
    (*handle).context = context;
    if client_name.is_null() {
        free(handle as *mut libc::c_void);
        return 22 as libc::c_int as kadm5_ret_t
    }
    /*
     * Verify the version numbers before proceeding; we can't use
     * CHECK_HANDLE because not all fields are set yet.
     */
    let mut srvr: kadm5_server_handle_t = handle;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    memset(&mut params_local as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    code =
        kadm5_get_config_params((*handle).context, 0 as libc::c_int,
                                params_in, &mut (*handle).params);
    if code != 0 {
        free(handle as *mut libc::c_void);
        return code as kadm5_ret_t
    }
    if (*handle).params.mask &
           (0x1 as libc::c_int | 0x10000 as libc::c_int |
                0x4000 as libc::c_int) as libc::c_long !=
           (0x1 as libc::c_int | 0x10000 as libc::c_int |
                0x4000 as libc::c_int) as libc::c_long {
        free(handle as *mut libc::c_void);
        return 43787574 as libc::c_long
    }
    code = krb5_parse_name((*handle).context, client_name, &mut client);
    if code != 0 {
        current_block = 7154420501029828768;
    } else {
        /*
     * Get credentials.  Also does some fallbacks in case kadmin/fqdn
     * principal doesn't exist.
     */
        code =
            get_init_creds(handle, client, init_type, pass, ccache_in,
                           service_name, (*handle).params.realm, &mut server)
                as libc::c_int;
        if code != 0 {
            current_block = 7154420501029828768;
        } else {
            /* If the service_name and client_name are iprop-centric, use the iprop
     * port and RPC identifiers. */
            iprop_enable =
                (!service_name.is_null() &&
                     !strstr(service_name,
                             b"kiprop\x00" as *const u8 as
                                 *const libc::c_char).is_null() &&
                     !strstr(client_name,
                             b"kiprop\x00" as *const u8 as
                                 *const libc::c_char).is_null()) as
                    libc::c_int as krb5_boolean;
            if iprop_enable != 0 {
                port = (*handle).params.iprop_port;
                rpc_prog = 100423 as libc::c_int as rpcprog_t;
                rpc_vers = 1 as libc::c_int as rpcvers_t
            } else {
                port = (*handle).params.kadmind_port;
                rpc_prog = 2112 as libc::c_int as rpcprog_t;
                rpc_vers = 2 as libc::c_int as rpcvers_t
            }
            code =
                connect_to_server((*handle).params.admin_server, port,
                                  &mut fd) as libc::c_int;
            if code != 0 {
                current_block = 7154420501029828768;
            } else {
                (*handle).clnt =
                    gssrpc_clnttcp_create(0 as *mut sockaddr_in, rpc_prog,
                                          rpc_vers, &mut fd,
                                          0 as libc::c_int as u_int,
                                          0 as libc::c_int as u_int);
                if (*handle).clnt.is_null() {
                    code = 43787528 as libc::c_long as libc::c_int;
                    current_block = 7154420501029828768;
                } else {
                    /* Set a one-hour timeout. */
                    timeout.tv_sec = 3600 as libc::c_int as __time_t;
                    timeout.tv_usec = 0 as libc::c_int as __suseconds_t;
                    Some((*(*(*handle).clnt).cl_ops).cl_control.expect("non-null function pointer")).expect("non-null function pointer")((*handle).clnt,
                                                                                                                                         1
                                                                                                                                             as
                                                                                                                                             libc::c_int,
                                                                                                                                         &mut timeout
                                                                                                                                             as
                                                                                                                                             *mut timeval
                                                                                                                                             as
                                                                                                                                             *mut libc::c_void);
                    (*handle).client_socket = fd;
                    (*(*handle).lhandle).clnt = (*handle).clnt;
                    (*(*handle).lhandle).client_socket = fd;
                    /* now that handle->clnt is set, we can check the handle */
                    code = _kadm5_check_handle(handle as *mut libc::c_void);
                    if code != 0 {
                        current_block = 7154420501029828768;
                    } else {
                        /*
     * The RPC connection is open; establish the GSS-API
     * authentication context.
     */
                        code =
                            setup_gss(handle, params_in,
                                      if init_type as libc::c_uint ==
                                             INIT_CREDS as libc::c_int as
                                                 libc::c_uint {
                                          client
                                      } else { 0 as krb5_principal }, server)
                                as libc::c_int;
                        if code != 0 {
                            current_block = 7154420501029828768;
                        } else if iprop_enable != 0 {
                            code = 0 as libc::c_int;
                            *server_handle = handle as *mut libc::c_void;
                            current_block = 18293657245023003428;
                        } else if init_2(&mut (*handle).api_version as
                                             *mut krb5_ui_4 as
                                             *mut libc::c_void, &mut r,
                                         (*handle).clnt) as u64 != 0 {
                            code = 43787528 as libc::c_long as libc::c_int;
                            current_block = 7154420501029828768;
                        } else {
                            /*
     * Bypass the remainder of the code and return straightaway
     * if the gss service requested is kiprop
     */
                            /* Drop down to v3 wire protocol if server does not support v4 */
                            if r.code == 43787559 as libc::c_long &&
                                   (*handle).api_version ==
                                       (0x12345700 as libc::c_int |
                                            0x4 as libc::c_int) as
                                           libc::c_uint {
                                (*handle).api_version =
                                    (0x12345700 as libc::c_int |
                                         0x3 as libc::c_int) as krb5_ui_4;
                                memset(&mut r as *mut generic_ret as
                                           *mut libc::c_void,
                                       0 as libc::c_int,
                                       ::std::mem::size_of::<generic_ret>() as
                                           libc::c_ulong);
                                if init_2(&mut (*handle).api_version as
                                              *mut krb5_ui_4 as
                                              *mut libc::c_void, &mut r,
                                          (*handle).clnt) as u64 != 0 {
                                    code =
                                        43787528 as libc::c_long as
                                            libc::c_int;
                                    current_block = 7154420501029828768;
                                } else {
                                    current_block = 2616667235040759262;
                                }
                            } else { current_block = 2616667235040759262; }
                            match current_block {
                                7154420501029828768 => { }
                                _ =>
                                /* Drop down to v2 wire protocol if server does not support v3 */
                                {
                                    if r.code == 43787559 as libc::c_long &&
                                           (*handle).api_version ==
                                               (0x12345700 as libc::c_int |
                                                    0x3 as libc::c_int) as
                                                   libc::c_uint {
                                        (*handle).api_version =
                                            (0x12345700 as libc::c_int |
                                                 0x2 as libc::c_int) as
                                                krb5_ui_4;
                                        memset(&mut r as *mut generic_ret as
                                                   *mut libc::c_void,
                                               0 as libc::c_int,
                                               ::std::mem::size_of::<generic_ret>()
                                                   as libc::c_ulong);
                                        if init_2(&mut (*handle).api_version
                                                      as *mut krb5_ui_4 as
                                                      *mut libc::c_void,
                                                  &mut r, (*handle).clnt) as
                                               u64 != 0 {
                                            code =
                                                43787528 as libc::c_long as
                                                    libc::c_int;
                                            current_block =
                                                7154420501029828768;
                                        } else {
                                            current_block =
                                                13349765058737954042;
                                        }
                                    } else {
                                        current_block = 13349765058737954042;
                                    }
                                    match current_block {
                                        7154420501029828768 => { }
                                        _ => {
                                            if r.code != 0 {
                                                code = r.code as libc::c_int;
                                                current_block =
                                                    7154420501029828768;
                                            } else {
                                                *server_handle =
                                                    handle as
                                                        *mut libc::c_void;
                                                current_block =
                                                    18293657245023003428;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        7154420501029828768 => {
            /*
     * Note that it is illegal for this code to execute if "handle"
     * has not been allocated and initialized.  I.e., don't use "goto
     * error" before the block of code at the top of the function
     * that allocates and initializes "handle".
     */
            if (*handle).destroy_cache != 0 && !(*handle).cache_name.is_null()
               {
                if krb5_cc_resolve((*handle).context, (*handle).cache_name,
                                   &mut ccache) == 0 as libc::c_int {
                    krb5_cc_destroy((*handle).context, ccache);
                }
            }
            if !(*handle).cache_name.is_null() {
                free((*handle).cache_name as *mut libc::c_void);
            }
            gss_release_cred(&mut minor_stat, &mut (*handle).cred);
            if !(*handle).clnt.is_null() &&
                   !(*(*handle).clnt).cl_auth.is_null() {
                Some((*(*(*(*handle).clnt).cl_auth).ah_ops).ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*(*handle).clnt).cl_auth);
            }
            if !(*handle).clnt.is_null() {
                Some((*(*(*handle).clnt).cl_ops).cl_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*handle).clnt);
            }
            if fd != -(1 as libc::c_int) { close(fd); }
            free((*handle).lhandle as *mut libc::c_void);
            kadm5_free_config_params((*handle).context,
                                     &mut (*handle).params);
        }
        _ => { }
    }
    krb5_free_principal((*handle).context, client);
    krb5_free_principal((*handle).context, server);
    if code != 0 { free(handle as *mut libc::c_void); }
    return code as kadm5_ret_t;
}
/* Get initial credentials for authenticating to server.  Perform fallback from
 * kadmin/fqdn to kadmin/admin if svcname_in is NULL. */
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn get_init_creds(mut handle: kadm5_server_handle_t,
                                    mut client: krb5_principal,
                                    mut init_type: init_type,
                                    mut pass: *mut libc::c_char,
                                    mut ccache_in: krb5_ccache,
                                    mut svcname_in: *mut libc::c_char,
                                    mut realm: *mut libc::c_char,
                                    mut server_out: *mut krb5_principal)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut code: kadm5_ret_t = 0;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    let mut svcname: [libc::c_char; 8192] = [0; 8192];
    *server_out = 0 as krb5_principal;
    /* NULL svcname means use host-based. */
    if svcname_in.is_null() {
        code =
            kadm5_get_admin_service_name((*handle).context,
                                         (*handle).params.realm,
                                         svcname.as_mut_ptr(),
                                         ::std::mem::size_of::<[libc::c_char; 8192]>()
                                             as libc::c_ulong) as kadm5_ret_t;
        if code != 0 {
            current_block = 15345524250823600465;
        } else { current_block = 8515828400728868193; }
    } else {
        strncpy(svcname.as_mut_ptr(), svcname_in,
                ::std::mem::size_of::<[libc::c_char; 8192]>() as
                    libc::c_ulong);
        svcname[(::std::mem::size_of::<[libc::c_char; 8192]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong) as usize]
            = '\u{0}' as i32 as libc::c_char;
        current_block = 8515828400728868193;
    }
    match current_block {
        8515828400728868193 =>
        /*
     * Acquire a service ticket for svcname@realm for client, using password
     * pass (which could be NULL), and create a ccache to store them in.  If
     * INIT_CREDS, use the ccache we were provided instead.
     */
        {
            if init_type as libc::c_uint ==
                   INIT_CREDS as libc::c_int as libc::c_uint {
                ccache = ccache_in;
                if asprintf(&mut (*handle).cache_name as
                                *mut *mut libc::c_char,
                            b"%s:%s\x00" as *const u8 as *const libc::c_char,
                            krb5_cc_get_type((*handle).context, ccache),
                            krb5_cc_get_name((*handle).context, ccache)) <
                       0 as libc::c_int {
                    (*handle).cache_name = 0 as *mut libc::c_char;
                    code = 12 as libc::c_int as kadm5_ret_t;
                    current_block = 15345524250823600465;
                } else { current_block = 11194104282611034094; }
            } else {
                static mut counter: libc::c_int = 0 as libc::c_int;
                let fresh0 = counter;
                counter = counter + 1;
                if asprintf(&mut (*handle).cache_name as
                                *mut *mut libc::c_char,
                            b"MEMORY:kadm5_%u\x00" as *const u8 as
                                *const libc::c_char, fresh0) <
                       0 as libc::c_int {
                    (*handle).cache_name = 0 as *mut libc::c_char;
                    code = 12 as libc::c_int as kadm5_ret_t;
                    current_block = 15345524250823600465;
                } else {
                    code =
                        krb5_cc_resolve((*handle).context,
                                        (*handle).cache_name, &mut ccache) as
                            kadm5_ret_t;
                    if code != 0 {
                        current_block = 15345524250823600465;
                    } else {
                        code =
                            krb5_cc_initialize((*handle).context, ccache,
                                               client) as kadm5_ret_t;
                        if code != 0 {
                            current_block = 15345524250823600465;
                        } else {
                            (*handle).destroy_cache = 1 as libc::c_int;
                            current_block = 11194104282611034094;
                        }
                    }
                }
            }
            match current_block {
                15345524250823600465 => { }
                _ => {
                    (*(*handle).lhandle).cache_name = (*handle).cache_name;
                    code =
                        gic_iter(handle, init_type, ccache, client, pass,
                                 svcname.as_mut_ptr(), realm, server_out);
                    if (code == -(1765328377 as libc::c_long) ||
                            code == -(1765328243 as libc::c_long)) &&
                           svcname_in.is_null() {
                        /* Retry with old host-independent service principal. */
                        code =
                            gic_iter(handle, init_type, ccache, client, pass,
                                     b"kadmin/admin\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char, realm, server_out)
                    }
                    /* Improved error messages */
                    if code == -(1765328353 as libc::c_long) {
                        code = 43787549 as libc::c_long
                    }
                    if code == -(1765328377 as libc::c_long) {
                        code = 43787560 as libc::c_long
                    }
                }
            }
        }
        _ => { }
    }
    if !ccache.is_null() &&
           init_type as libc::c_uint !=
               INIT_CREDS as libc::c_int as libc::c_uint {
        krb5_cc_close((*handle).context, ccache);
    }
    return code;
}
/* Perform one iteration of attempting to get credentials.  This includes
 * searching existing ccache for requested service if INIT_CREDS. */
#[c2rust::src_loc = "447:1"]
unsafe extern "C" fn gic_iter(mut handle: kadm5_server_handle_t,
                              mut init_type: init_type,
                              mut ccache: krb5_ccache,
                              mut client: krb5_principal,
                              mut pass: *mut libc::c_char,
                              mut svcname: *mut libc::c_char,
                              mut realm: *mut libc::c_char,
                              mut server_out: *mut krb5_principal)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut code: kadm5_ret_t = 0;
    let mut ctx: krb5_context = 0 as *mut _krb5_context;
    let mut kt: krb5_keytab = 0 as *mut _krb5_kt;
    let mut opt: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut mcreds: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    let mut outcreds: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    *server_out = 0 as krb5_principal;
    ctx = (*handle).context;
    kt = 0 as krb5_keytab;
    memset(&mut opt as *mut *mut krb5_get_init_creds_opt as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<*mut krb5_get_init_creds_opt>() as
               libc::c_ulong);
    memset(&mut mcreds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    memset(&mut outcreds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    /* Credentials for kadmin don't need to be forwardable or proxiable. */
    if init_type as libc::c_uint != INIT_CREDS as libc::c_int as libc::c_uint
       {
        code = krb5_get_init_creds_opt_alloc(ctx, &mut opt) as kadm5_ret_t;
        if code != 0 {
            current_block = 13188323066503985336;
        } else {
            krb5_get_init_creds_opt_set_forwardable(opt, 0 as libc::c_int);
            krb5_get_init_creds_opt_set_proxiable(opt, 0 as libc::c_int);
            krb5_get_init_creds_opt_set_out_ccache(ctx, opt, ccache);
            if init_type as libc::c_uint ==
                   INIT_ANONYMOUS as libc::c_int as libc::c_uint {
                krb5_get_init_creds_opt_set_anonymous(opt, 1 as libc::c_int);
            }
            current_block = 9606288038608642794;
        }
    } else { current_block = 9606288038608642794; }
    match current_block {
        9606288038608642794 => {
            if init_type as libc::c_uint ==
                   INIT_PASS as libc::c_int as libc::c_uint ||
                   init_type as libc::c_uint ==
                       INIT_ANONYMOUS as libc::c_int as libc::c_uint {
                code =
                    krb5_get_init_creds_password(ctx, &mut outcreds, client,
                                                 pass,
                                                 Some(krb5_prompter_posix as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   *mut libc::c_void,
                                                                               _:
                                                                                   *const libc::c_char,
                                                                               _:
                                                                                   *const libc::c_char,
                                                                               _:
                                                                                   libc::c_int,
                                                                               _:
                                                                                   *mut krb5_prompt)
                                                              ->
                                                                  krb5_error_code),
                                                 0 as *mut libc::c_void,
                                                 0 as libc::c_int, svcname,
                                                 opt) as kadm5_ret_t;
                if code != 0 {
                    current_block = 13188323066503985336;
                } else { current_block = 14072441030219150333; }
            } else if init_type as libc::c_uint ==
                          INIT_SKEY as libc::c_int as libc::c_uint {
                if !pass.is_null() {
                    code = krb5_kt_resolve(ctx, pass, &mut kt) as kadm5_ret_t;
                    if code != 0 {
                        current_block = 13188323066503985336;
                    } else { current_block = 5634871135123216486; }
                } else { current_block = 5634871135123216486; }
                match current_block {
                    13188323066503985336 => { }
                    _ => {
                        code =
                            krb5_get_init_creds_keytab(ctx, &mut outcreds,
                                                       client, kt,
                                                       0 as libc::c_int,
                                                       svcname, opt) as
                                kadm5_ret_t;
                        if !pass.is_null() { krb5_kt_close(ctx, kt); }
                        if code != 0 {
                            current_block = 13188323066503985336;
                        } else { current_block = 14072441030219150333; }
                    }
                }
            } else if init_type as libc::c_uint ==
                          INIT_CREDS as libc::c_int as libc::c_uint {
                mcreds.client = client;
                code =
                    krb5_parse_name_flags(ctx, svcname, 0x8 as libc::c_int,
                                          &mut mcreds.server) as kadm5_ret_t;
                if code != 0 {
                    current_block = 13188323066503985336;
                } else {
                    code =
                        krb5_set_principal_realm(ctx, mcreds.server, realm) as
                            kadm5_ret_t;
                    if code != 0 {
                        current_block = 13188323066503985336;
                    } else {
                        code =
                            krb5_cc_retrieve_cred(ctx, ccache,
                                                  0 as libc::c_int,
                                                  &mut mcreds, &mut outcreds)
                                as kadm5_ret_t;
                        krb5_free_principal(ctx, mcreds.server);
                        if code != 0 {
                            current_block = 13188323066503985336;
                        } else { current_block = 14072441030219150333; }
                    }
                }
            } else {
                code = 22 as libc::c_int as kadm5_ret_t;
                current_block = 13188323066503985336;
            }
            match current_block {
                13188323066503985336 => { }
                _ => {
                    /* Steal the server principal of the creds we acquired and return it to the
     * caller, which needs to knows what service to authenticate to. */
                    *server_out = outcreds.server;
                    outcreds.server = 0 as krb5_principal
                }
            }
        }
        _ => { }
    }
    krb5_free_cred_contents(ctx, &mut outcreds);
    if !opt.is_null() { krb5_get_init_creds_opt_free(ctx, opt); }
    return code;
}
/* Set *fd to a socket connected to hostname and port. */
#[c2rust::src_loc = "529:1"]
unsafe extern "C" fn connect_to_server(mut hostname: *const libc::c_char,
                                       mut port: libc::c_int,
                                       mut fd: *mut libc::c_int)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut hint: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut addrs: *mut addrinfo = 0 as *mut addrinfo;
    let mut a: *mut addrinfo = 0 as *mut addrinfo;
    let mut portbuf: [libc::c_char; 32] = [0; 32];
    let mut err: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut code: kadm5_ret_t = 0;
    /* Look up the server's addresses. */
    snprintf(portbuf.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
             b"%d\x00" as *const u8 as *const libc::c_char, port);
    memset(&mut hint as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hint.ai_socktype = SOCK_STREAM as libc::c_int;
    hint.ai_flags = 0x20 as libc::c_int;
    hint.ai_flags |= 0x400 as libc::c_int;
    err =
        krb5int_getaddrinfo(hostname, portbuf.as_mut_ptr(), &mut hint,
                            &mut addrs);
    if err != 0 as libc::c_int { return 43787576 as libc::c_long }
    /* Try to connect to each address until we succeed. */
    a = addrs;
    loop  {
        if a.is_null() { current_block = 2838571290723028321; break ; }
        s = socket((*a).ai_family, (*a).ai_socktype, 0 as libc::c_int);
        if s == -(1 as libc::c_int) {
            code = 43787520 as libc::c_long;
            current_block = 4144075667789361827;
            break ;
        } else {
            err =
                connect(s, __CONST_SOCKADDR_ARG{__sockaddr__: (*a).ai_addr,},
                        (*a).ai_addrlen);
            if err == 0 as libc::c_int {
                *fd = s;
                code = 0 as libc::c_int as kadm5_ret_t;
                current_block = 4144075667789361827;
                break ;
            } else { close(s); a = (*a).ai_next }
        }
    }
    match current_block {
        2838571290723028321 => {
            /* We didn't succeed on any address. */
            code = 43787528 as libc::c_long
        }
        _ => { }
    }
    krb5int_freeaddrinfo(addrs);
    return code;
}
/* Acquire GSSAPI credentials and set up RPC auth flavor. */
#[c2rust::src_loc = "573:1"]
unsafe extern "C" fn setup_gss(mut handle: kadm5_server_handle_t,
                               mut params_in: *mut kadm5_config_params,
                               mut client: krb5_principal,
                               mut server: krb5_principal) -> kadm5_ret_t {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut gss_client: gss_name_t = 0 as *mut gss_name_struct;
    let mut gss_target: gss_name_t = 0 as *mut gss_name_struct;
    let mut c_ccname_orig: *const libc::c_char = 0 as *const libc::c_char;
    let mut ccname_orig: *mut libc::c_char = 0 as *mut libc::c_char;
    ccname_orig = 0 as *mut libc::c_char;
    gss_target = 0 as gss_name_t;
    gss_client = gss_target;
    /* Temporarily use the kadm5 cache. */
    gssstat =
        gss_krb5_ccache_name(&mut minor_stat, (*handle).cache_name,
                             &mut c_ccname_orig);
    if !(gssstat != 0 as libc::c_int as libc::c_uint) {
        if !c_ccname_orig.is_null() {
            ccname_orig = strdup(c_ccname_orig)
        } else { ccname_orig = 0 as *mut libc::c_char }
        buf.value = &mut server as *mut krb5_principal as *mut libc::c_void;
        buf.length = ::std::mem::size_of::<krb5_principal>() as libc::c_ulong;
        gssstat =
            gss_import_name(&mut minor_stat, &mut buf, gss_nt_krb5_principal,
                            &mut gss_target);
        if !(gssstat != 0 as libc::c_int as libc::c_uint) {
            if !client.is_null() {
                buf.value =
                    &mut client as *mut krb5_principal as *mut libc::c_void;
                buf.length =
                    ::std::mem::size_of::<krb5_principal>() as libc::c_ulong;
                gssstat =
                    gss_import_name(&mut minor_stat, &mut buf,
                                    gss_nt_krb5_principal, &mut gss_client)
            } else { gss_client = 0 as gss_name_t }
            if !(gssstat != 0 as libc::c_int as libc::c_uint) {
                gssstat =
                    gss_acquire_cred(&mut minor_stat, gss_client,
                                     0 as libc::c_int as OM_uint32,
                                     0 as gss_OID_set, 1 as libc::c_int,
                                     &mut (*handle).cred,
                                     0 as *mut gss_OID_set,
                                     0 as *mut OM_uint32);
                if !(gssstat != 0 as libc::c_int as libc::c_uint) {
                    /*
     * Do actual creation of RPC auth handle.  Implements auth flavor
     * fallback.
     */
                    rpc_auth(handle, params_in, (*handle).cred, gss_target);
                }
            }
        }
    }
    if !gss_client.is_null() {
        gss_release_name(&mut minor_stat, &mut gss_client);
    }
    if !gss_target.is_null() {
        gss_release_name(&mut minor_stat, &mut gss_target);
    }
    /* Revert to prior gss_krb5 ccache. */
    if !ccname_orig.is_null() {
        gssstat =
            gss_krb5_ccache_name(&mut minor_stat, ccname_orig,
                                 0 as *mut *const libc::c_char);
        if gssstat != 0 { return 43787566 as libc::c_long }
        free(ccname_orig as *mut libc::c_void);
    } else {
        gssstat =
            gss_krb5_ccache_name(&mut minor_stat, 0 as *const libc::c_char,
                                 0 as *mut *const libc::c_char);
        if gssstat != 0 { return 43787566 as libc::c_long }
    }
    if (*(*handle).clnt).cl_auth.is_null() { return 43787566 as libc::c_long }
    return 0 as libc::c_int as kadm5_ret_t;
}
/* Create RPC auth handle.  Do auth flavor fallback if needed. */
#[c2rust::src_loc = "653:1"]
unsafe extern "C" fn rpc_auth(mut handle: kadm5_server_handle_t,
                              mut params_in: *mut kadm5_config_params,
                              mut gss_client_creds: gss_cred_id_t,
                              mut gss_target: gss_name_t) {
    let mut gssstat: OM_uint32 = 0;
    let mut minor_stat: OM_uint32 = 0;
    let mut sec: rpc_gss_sec =
        rpc_gss_sec{mech: 0 as *mut gss_OID_desc_struct,
                    qop: 0,
                    svc: 0 as rpc_gss_svc_t,
                    cred: 0 as *mut gss_cred_id_struct,
                    req_flags: 0,};
    /* Allow unauthenticated option for testing. */
    if !params_in.is_null() &&
           (*params_in).mask & 0x200000 as libc::c_int as libc::c_long != 0 {
        return
    }
    /* Use RPCSEC_GSS by default. */
    if params_in.is_null() ||
           (*params_in).mask & 0x100000 as libc::c_int as libc::c_long == 0 {
        sec.mech = gss_mech_krb5;
        sec.qop = 0 as libc::c_int as gss_qop_t;
        sec.svc = RPCSEC_GSS_SVC_PRIVACY;
        sec.cred = gss_client_creds;
        sec.req_flags = (2 as libc::c_int | 4 as libc::c_int) as uint32_t;
        (*(*handle).clnt).cl_auth =
            gssrpc_authgss_create((*handle).clnt, gss_target, &mut sec);
        if !(*(*handle).clnt).cl_auth.is_null() { return }
    }
    if !params_in.is_null() &&
           (*params_in).mask & 0x400000 as libc::c_int as libc::c_long != 0 {
        return
    }
    /* Fall back to old AUTH_GSSAPI. */
    (*(*handle).clnt).cl_auth =
        gssrpc_auth_gssapi_create((*handle).clnt, &mut gssstat,
                                  &mut minor_stat, gss_client_creds,
                                  gss_target, gss_mech_krb5,
                                  (2 as libc::c_int | 4 as libc::c_int) as
                                      OM_uint32,
                                  0 as libc::c_int as OM_uint32,
                                  0 as *mut gss_OID, 0 as *mut OM_uint32,
                                  0 as *mut OM_uint32);
}
#[no_mangle]
#[c2rust::src_loc = "694:1"]
pub unsafe extern "C" fn kadm5_destroy(mut server_handle: *mut libc::c_void)
 -> kadm5_ret_t {
    let mut minor_stat: OM_uint32 = 0;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    let mut code: libc::c_int = 0 as libc::c_int;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).cache_name.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    if (*handle).destroy_cache != 0 && !(*handle).cache_name.is_null() {
        code =
            krb5_cc_resolve((*handle).context, (*handle).cache_name,
                            &mut ccache);
        if code == 0 as libc::c_int {
            code = krb5_cc_destroy((*handle).context, ccache)
        }
    }
    if !(*handle).cache_name.is_null() {
        free((*handle).cache_name as *mut libc::c_void);
    }
    if !(*handle).cred.is_null() {
        gss_release_cred(&mut minor_stat, &mut (*handle).cred);
    }
    if !(*handle).clnt.is_null() && !(*(*handle).clnt).cl_auth.is_null() {
        Some((*(*(*(*handle).clnt).cl_auth).ah_ops).ah_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*(*handle).clnt).cl_auth);
    }
    if !(*handle).clnt.is_null() {
        Some((*(*(*handle).clnt).cl_ops).cl_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*handle).clnt);
    }
    if (*handle).client_socket != -(1 as libc::c_int) {
        close((*handle).client_socket);
    }
    if !(*handle).lhandle.is_null() {
        free((*handle).lhandle as *mut libc::c_void);
    }
    kadm5_free_config_params((*handle).context, &mut (*handle).params);
    (*handle).magic_number = 0 as libc::c_int as krb5_ui_4;
    free(handle as *mut libc::c_void);
    return code as kadm5_ret_t;
}
/* not supported on client */
#[no_mangle]
#[c2rust::src_loc = "731:1"]
pub unsafe extern "C" fn kadm5_lock(mut server_handle: *mut libc::c_void)
 -> kadm5_ret_t {
    return 22 as libc::c_int as kadm5_ret_t;
}
/* not supported on client */
#[no_mangle]
#[c2rust::src_loc = "737:1"]
pub unsafe extern "C" fn kadm5_unlock(mut server_handle: *mut libc::c_void)
 -> kadm5_ret_t {
    return 22 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "742:1"]
pub unsafe extern "C" fn kadm5_flush(mut server_handle: *mut libc::c_void)
 -> kadm5_ret_t {
    return 0 as libc::c_int as kadm5_ret_t;
}
/*
 * _KADM5_CHECK_HANDLE calls the function _kadm5_check_handle and
 * returns any non-zero error code that function returns.
 * _kadm5_check_handle, in client_handle.c and server_handle.c, exists
 * in both the server- and client- side libraries.  In each library,
 * it calls CHECK_HANDLE, which is defined by the appropriate
 * _internal.h header file to call GENERIC_CHECK_HANDLE as well as
 * CLIENT_CHECK_HANDLE and SERVER_CHECK_HANDLE.
 *
 * _KADM5_CHECK_HANDLE should be used by a function that needs to
 * check the handle but wants to be the same code in both the client
 * and server library; it makes a function call to the right handle
 * checker.  Code that only exists in one library can call the
 * CHECK_HANDLE macro, which inlines the test instead of making
 * another function call.
 *
 * Got that?
 */
#[no_mangle]
#[c2rust::src_loc = "747:1"]
pub unsafe extern "C" fn _kadm5_check_handle(mut handle: *mut libc::c_void)
 -> libc::c_int {
    let mut srvr: kadm5_server_handle_t = handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long as libc::c_int }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long as libc::c_int
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long as libc::c_int
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long as libc::c_int
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long as libc::c_int
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long as libc::c_int
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787556 as libc::c_long as libc::c_int
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787558 as libc::c_long as libc::c_int
    }
    let mut srvr_0: kadm5_server_handle_t = handle as kadm5_server_handle_t;
    if (*srvr_0).clnt.is_null() {
        return 43787551 as libc::c_long as libc::c_int
    }
    if (*srvr_0).cache_name.is_null() {
        return 43787551 as libc::c_long as libc::c_int
    }
    if (*srvr_0).lhandle.is_null() {
        return 43787551 as libc::c_long as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "753:1"]
pub unsafe extern "C" fn kadm5_init_krb5_context(mut ctx: *mut krb5_context)
 -> krb5_error_code {
    return krb5_init_context(ctx);
}
/*
 * Stub function for kadmin.  It was created to eliminate the dependency on
 * libkdb's ulog functions.  The srv equivalent makes the actual calls.
 */
#[no_mangle]
#[c2rust::src_loc = "762:1"]
pub unsafe extern "C" fn kadm5_init_iprop(mut handle: *mut libc::c_void,
                                          mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    return 0 as libc::c_int;
}
