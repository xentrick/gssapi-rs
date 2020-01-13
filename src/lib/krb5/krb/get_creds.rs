use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:40"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:40"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:40"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:40"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:40"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
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
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
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
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2054:16"]
    pub struct _krb5_kdc_req {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub kdc_options: krb5_flags,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub from: krb5_timestamp,
        pub till: krb5_timestamp,
        pub rtime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub nktypes: libc::c_int,
        pub ktype: *mut krb5_enctype,
        pub addresses: *mut *mut krb5_address,
        pub authorization_data: krb5_enc_data,
        pub unenc_authdata: *mut *mut krb5_authdata,
        pub second_ticket: *mut *mut krb5_ticket,
    }
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "7505:1"]
    pub type krb5_tkt_creds_context = *mut _krb5_tkt_creds_context;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::_krb5_tkt_creds_context;
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
        /* *
 * Duplicate keytab handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Key table handle to be duplicated
 * @param [out] out             Key table handle
 *
 * Create a new handle referring to the same key table as @a in.  The new
 * handle and @a in can be closed independently.
 *
 * @version New in 1.12
 */
        /* *
 * Get the default key table name.
 *
 * @param [in]     context      Library context
 * @param [out]    name         Default key table name
 * @param [in]     name_size    Space available in @a name
 *
 * Fill @a name with the name of the default key table for @a context.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_CONFIG_NOTENUFSPACE Buffer is too short
 * @return
 * Kerberos error codes
 */
        /* *
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Resolve the default client key table.
 *
 * @param [in]     context      Library context
 * @param [out]    keytab_out   Key table handle
 *
 * Fill @a keytab_out with a handle to the default client key table.
 *
 * @version New in 1.11
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Free the contents of a key table entry.
 *
 * @param [in] context          Library context
 * @param [in] entry            Key table entry whose contents are to be freed
 *
 * @note The pointer is not freed.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        /* * @deprecated Use krb5_free_keytab_entry_contents instead. */
        /* remove and add are functions, so that they can return NOWRITE
   if not a writable keytab */
        /* *
 * Remove an entry from a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to remove from key table
 *
 * @retval
 * 0 Success
 * @retval
 *  KRB5_KT_NOWRITE     Key table is not writable
 * @return
 * Kerberos error codes
 */
        /* *
 * Add a new entry to a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to be added
 *
 * @retval
 * 0  Success
 * @retval
 *  ENOMEM    Insufficient memory
 * @retval
 *  KRB5_KT_NOWRITE  Key table is not writeable
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* librc.spec--see rcache.h */
        /* libcc.spec */
        /* *
 * Resolve a credential cache name.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Credential cache name to be resolved
 * @param [out] cache           Credential cache handle
 *
 * Fills in @a cache with a @a cache handle that corresponds to the name in @a
 * name.  @a name should be of the form @c type:residual, and @a type must be a
 * type known to the library.  If the @a name does not contain a colon,
 * interpret it as a file name.
 *
 * @code
 * Example: krb5_cc_resolve(context, "MEMORY:C_", &cache);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Duplicate ccache handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Credential cache handle to be duplicated
 * @param [out] out             Credential cache handle
 *
 * Create a new handle referring to the same cache as @a in.
 * The new handle and @a in can be closed independently.
 */
        /* *
 * Return the name of the default credential cache.
 *
 * @param [in] context          Library context
 *
 * Return a pointer to the default credential cache name for @a context, as
 * determined by a prior call to krb5_cc_set_default_name(), by the KRB5CCNAME
 * environment variable, by the default_ccache_name profile variable, or by the
 * operating system or build-time default value.  The returned value must not
 * be modified or freed by the caller.  The returned value becomes invalid when
 * @a context is destroyed krb5_free_context() or if a subsequent call to
 * krb5_cc_set_default_name() is made on @a context.
 *
 * The default credential cache name is cached in @a context between calls to
 * this function, so if the value of KRB5CCNAME changes in the process
 * environment after the first call to this function on, that change will not
 * be reflected in later calls with the same context.  The caller can invoke
 * krb5_cc_set_default_name() with a NULL value of @a name to clear the cached
 * value and force the default name to be recomputed.
 *
 * @return
 * Name of default credential cache for the current user.
 */
        /* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
        /* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
        /* *
 * Copy a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  incc            Credential cache to be copied
 * @param [out] outcc           Copy of credential cache to be filled in
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        /* *
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Test whether a principal is a configuration principal.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal to check
 *
 * @return
 * @c TRUE if the principal is a configuration principal (generated part of
 * krb5_cc_set_config()); @c FALSE otherwise.
 */
        /* *
 * Make a credential cache the primary cache for its collection.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * If the type of @a cache supports it, set @a cache to be the primary
 * credential cache for the collection it belongs to.
 *
 * @retval
 * 0  Success, or the type of @a cache doesn't support switching
 * @return
 * Kerberos error codes
 */
        /* *
 * Determine whether a credential cache type supports switching.
 *
 * @param [in] context          Library context
 * @param [in] type             Credential cache type
 *
 * @version New in 1.10
 *
 * @retval TRUE if @a type supports switching
 * @retval FALSE if it does not or is not a valid credential cache type.
 */
        /* *
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
        /* *
 * Select a credential cache to use with a server principal.
 *
 * @param [in]  context         Library context
 * @param [in]  server          Server principal
 * @param [out] cache_out       Credential cache handle
 * @param [out] princ_out       Client principal
 *
 * Select a cache within the collection containing credentials most appropriate
 * for use with @a server, according to configured rules and heuristics.
 *
 * Use krb5_cc_close() to release @a cache_out when it is no longer needed.
 * Use krb5_free_principal() to release @a princ_out when it is no longer
 * needed.  Note that @a princ_out is set in some error conditions.
 *
 * @return
 * If an appropriate cache is found, 0 is returned, @a cache_out is set to the
 * selected cache, and @a princ_out is set to the default principal of that
 * cache.
 *
 * If the appropriate client principal can be authoritatively determined but
 * the cache collection contains no credentials for that principal, then
 * KRB5_CC_NOTFOUND is returned, @a cache_out is set to NULL, and @a princ_out
 * is set to the appropriate client principal.
 *
 * If no configured mechanism can determine the appropriate cache or principal,
 * KRB5_CC_NOTFOUND is returned and @a cache_out and @a princ_out are set to
 * NULL.
 *
 * Any other error code indicates a fatal error in the processing of a cache
 * selection mechanism.
 *
 * @version New in 1.10
 */
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        /* *
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
        /* *
 * Free the storage assigned to array of authentication data.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of authentication data to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
        /* *
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        /* *
 * Free a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        /* *
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        /* *
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* Free a krb5_octet_data structure (should be unused). */
        /* *
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        /* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
        /* *
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
        /* *
 * Free an array of encryption types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of enctypes to be freed
 *
 * @version New in 1.12
 */
        /* *
 * Free an array of checksum types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of checksum types to be freed
 */
        /* From krb5/os, but needed by the outside world */
/* *
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
        /* *
 * Return all interface addresses for this host.
 *
 * @param [in]  context         Library context
 * @param [out] addr            Array of krb5_address pointers, ending with
 *                              NULL
 *
 * Use krb5_free_addresses() to free @a addr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
        /* *
 * Canonicalize a hostname, possibly using name service.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Input hostname
 * @param [out] canonhost_out   Canonicalized hostname
 *
 * This function canonicalizes orig_hostname, possibly using name service
 * lookups if configuration permits.  Use krb5_free_string() to free @a
 * canonhost_out when it is no longer needed.
 *
 * @version New in 1.15
 */
        /* *
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Test whether a principal matches a matching principal.
 *
 * @param [in]  context         Library context
 * @param [in]  matching        Matching principal
 * @param [in]  princ           Principal to test
 *
 * @note A matching principal is a host-based principal with an empty realm
 * and/or second data component (hostname).  Profile configuration may cause
 * the hostname to be ignored even if it is present.  A principal matches a
 * matching principal if the former has the same non-empty (and non-ignored)
 * components of the latter.
 *
 * If @a matching is NULL, return TRUE.  If @a matching is not a matching
 * principal, return the value of krb5_principal_compare(context, matching,
 * princ).
 *
 * @return
 * TRUE if @a princ matches @a matching, FALSE otherwise.
 */
        /* *
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set a password for a principal using specified credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  creds                Credentials for kadmin/changepw service
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the credentials @a creds to set the password @a newpw for
 * the principal @a change_password_for.  It implements the set password
 * operation of RFC 3244, for interoperability with Microsoft Windows
 * implementations.
 *
 * @note If @a change_password_for is NULL, the change is performed on the
 * current principal. If @a change_password_for is non-null, the change is
 * performed on the principal name passed in @a change_password_for.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @sa krb5_set_password_using_ccache()
 *
 * @retval
 * 0  Success and result_code is set to #KRB5_KPASSWD_SUCCESS.
 * @return
 * Kerberos error codes.
 */
        /* *
 * Set a password for a principal using cached credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  ccache               Credential cache
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the cached credentials from @a ccache to set the password
 * @a newpw for the principal @a change_password_for.  It implements RFC 3244
 * set password operation (interoperable with MS Windows implementations) using
 * the credential cache.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @note If @a change_password_for is set to NULL, the change is performed on
 * the default principal in @a ccache. If @a change_password_for is non null,
 * the change is performed on the specified principal.
 *
 * @sa krb5_set_password()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
        /* *
 * Retrieve configuration profile from the context.
 *
 * @param [in]  context         Library context
 * @param [out] profile         Pointer to data read from a configuration file
 *
 * This function creates a new @a profile object that reflects profile
 * in the supplied @a context.
 *
 * The @a profile object may be freed with profile_release() function.
 * See profile.h and profile API for more details.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* * @deprecated Replaced by krb5_get_init_creds_password().*/
        /* * @deprecated Replaced by krb5_get_init_creds(). */
        /* * @deprecated Replaced by krb5_get_init_creds_keytab(). */
        /* KRB5_DEPRECATED */
        /* *
 * Parse and decrypt a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     inbuf          AP-REQ message to be parsed
 * @param [in]     server         Matching principal for server, or NULL to
 *                                allow any principal in keytab
 * @param [in]     keytab         Key table, or NULL to use the default
 * @param [out]    ap_req_options If non-null, the AP-REQ flags on output
 * @param [out]    ticket         If non-null, ticket from the AP-REQ message
 *
 * This function parses, decrypts and verifies a AP-REQ message from @a inbuf
 * and stores the authenticator in @a auth_context.
 *
 * If a keyblock was specified in @a auth_context using
 * krb5_auth_con_setuseruserkey(), that key is used to decrypt the ticket in
 * AP-REQ message and @a keytab is ignored.  In this case, @a server should be
 * specified as a complete principal name to allow for proper transited-path
 * checking and replay cache selection.
 *
 * Otherwise, the decryption key is obtained from @a keytab, or from the
 * default keytab if it is NULL.  In this case, @a server may be a complete
 * principal name, a matching principal (see krb5_sname_match()), or NULL to
 * match any principal name.  The keys tried against the encrypted part of the
 * ticket are determined as follows:
 *
 * - If @a server is a complete principal name, then its entry in @a keytab is
 *   tried.
 * - Otherwise, if @a keytab is iterable, then all entries in @a keytab which
 *   match @a server are tried.
 * - Otherwise, the server principal in the ticket must match @a server, and
 *   its entry in @a keytab is tried.
 *
 * The client specified in the decrypted authenticator must match the client
 * specified in the decrypted ticket.
 *
 * If the @a remote_addr field of @a auth_context is set, the request must come
 * from that address.
 *
 * If a replay cache handle is provided in the @a auth_context, the
 * authenticator and ticket are verified against it.  If no conflict is found,
 * the new authenticator is then stored in the replay cache of @a auth_context.
 *
 * Various other checks are performed on the decoded data, including
 * cross-realm policy, clockskew, and ticket validation times.
 *
 * On success the authenticator, subkey, and remote sequence number of the
 * request are stored in @a auth_context. If the #AP_OPTS_MUTUAL_REQUIRED
 * bit is set, the local sequence number is XORed with the remote sequence
 * number in the request.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve a service key from a key table.
 *
 * @param [in]  context     Library context
 * @param [in]  keyprocarg  Name of a key table (NULL to use default name)
 * @param [in]  principal   Service principal
 * @param [in]  vno         Key version number (0 for highest available)
 * @param [in]  enctype     Encryption type (0 for any type)
 * @param [out] key         Service key from key table
 *
 * Open and search the specified key table for the entry identified by @a
 * principal, @a enctype, and @a vno. If no key is found, return an error code.
 *
 * The default key table is used, unless @a keyprocarg is non-null.
 * @a keyprocarg designates a specific key table.
 *
 * Use krb5_free_keyblock() to free @a key when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return Kerberos error code if not found or @a keyprocarg is invalid.
 */
        /* *
 * Format a @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data in the message
 * @param [out] der_out         Formatted @c KRB-SAFE buffer
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function creates an integrity protected @c KRB-SAFE message
 * using data supplied by the application.
 *
 * Fields in @a auth_context specify the checksum type, the keyblock that
 * can be used to seed the checksum, full addresses (host and port) for
 * the sender and receiver, and @ref KRB5_AUTH_CONTEXT flags.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-SAFE message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-SAFE message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-SAFE message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-SAFE message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data for @c KRB-PRIV message
 * @param [out] der_out         Formatted @c KRB-PRIV message
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * This function is similar to krb5_mk_safe(), but the message is encrypted and
 * integrity-protected, not just integrity-protected.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-PRIV message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-PRIV message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-PRIV message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-PRIV message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Client function for @c sendauth protocol.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     fd             File descriptor that describes network socket
 * @param [in]     appl_version   Application protocol version to be matched
 *                                with the receiver's application version
 * @param [in]     client         Client principal
 * @param [in]     server         Server principal
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Data to be sent to the server
 * @param [in]     in_creds       Input credentials, or NULL to use @a ccache
 * @param [in]     ccache         Credential cache
 * @param [out]    error          If non-null, contains KRB_ERROR message
 *                                returned from server
 * @param [out]    rep_result     If non-null and @a ap_req_options is
 *                                #AP_OPTS_MUTUAL_REQUIRED, contains the result
 *                                of mutual authentication exchange
 * @param [out]    out_creds      If non-null, the retrieved credentials
 *
 * This function performs the client side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Credentials may be specified in three ways:
 *
 * @li If @a in_creds is NULL, credentials are obtained with
 * krb5_get_credentials() using the principals @a client and @a server.  @a
 * server must be non-null; @a client may NULL to use the default principal of
 * @a ccache.
 *
 * @li If @a in_creds is non-null, but does not contain a ticket, credentials
 * for the exchange are obtained with krb5_get_credentials() using @a in_creds.
 * In this case, the values of @a client and @a server are unused.
 *
 * @li If @a in_creds is a complete credentials structure, it used directly.
 * In this case, the values of @a client, @a server, and @a ccache are unused.
 *
 * If the server is using a different application protocol than that specified
 * in @a appl_version, an error will be returned.
 *
 * Use krb5_free_creds() to free @a out_creds, krb5_free_ap_rep_enc_part() to
 * free @a rep_result, and krb5_free_error() to free @a error when they are no
 * longer needed.
 *
 * @sa krb5_recvauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Server function for @a sendauth protocol.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     appl_version Application protocol version to be matched
 *                              against the client's application version
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Key table containing service keys
 * @param [out]    ticket       Ticket (NULL if not needed)
 *
 * This function performs the server side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @sa krb5_sendauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Server function for @a sendauth protocol with version parameter.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Decryption key
 * @param [out]    ticket       Ticket (NULL if not needed)
 * @param [out]    version      sendauth protocol version (NULL if not needed)
 *
 * This function is similar to krb5_recvauth() with the additional output
 * information place into @a version.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format a @c KRB-CRED message for an array of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Null-terminated array of credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache information (NULL if not needed)
 *
 * This function takes an array of credentials @a creds and formats
 * a @c KRB-CRED message @a der_out to pass to krb5_rd_cred().
 *
 * The local and remote addresses in @a auth_context are optional; if either is
 * specified, they are used to form the sender and receiver addresses in the
 * KRB-CRED message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, an entry
 * for the message is entered in an in-memory replay cache to detect if the
 * message is reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not
 * set, no replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, the timestamp used for the KRB-CRED message is stored in @a
 * rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-CRED message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * The message will be encrypted using the send subkey of @a auth_context if it
 * is present, or the session key otherwise.  If neither key is present, the
 * credentials will not be encrypted, and the message should only be sent over
 * a secure channel.  No replay cache entry is used in this case.
 *
 * @retval
 *  0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *   KRB5_RC_REQUIRED Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
        /* *
 * Format a @c KRB-CRED message for a single set of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Pointer to credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache data (NULL if not needed)
 *
 * This is a convenience function that calls krb5_mk_ncred() with a single set
 * of credentials.
 *
 * @retval
 * 0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *  KRB5_RC_REQUIRED   Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
        /* *
 * Read and validate a @c KRB-CRED message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creddata        @c KRB-CRED message
 * @param [out] creds_out       Null-terminated array of forwarded credentials
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.`
 *
 * @a creddata will be decrypted using the receiving subkey if it is present in
 * @a auth_context, or the session key if the receiving subkey is not present
 * or fails to decrypt the message.
 *
 * Use krb5_free_tgt_creds() to free @a creds_out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Get a forwarded TGT and format a @c KRB-CRED message.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rhost            Remote host
 * @param [in] client           Client principal of TGT
 * @param [in] server           Principal of server to receive TGT
 * @param [in] cc               Credential cache handle (NULL to use default)
 * @param [in] forwardable      Whether TGT should be forwardable
 * @param [out] outbuf          KRB-CRED message
 *
 * Get a TGT for use at the remote host @a rhost and format it into a KRB-CRED
 * message.  If @a rhost is NULL and @a server is of type #KRB5_NT_SRV_HST,
 * the second component of @a server will be used.
 *
 * @retval
 *  0 Success
 * @retval
 *   ENOMEM Insufficient memory
 * @retval
 *   KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 *   KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 *   KRB5_CC_BADNAME Credential cache name or principal name malformed
 * @return
 * Kerberos error codes
 */
        /* *
 * Create and initialize an authentication context.
 *
 * @param [in]  context         Library context
 * @param [out] auth_context    Authentication context
 *
 * This function creates an authentication context to hold configuration and
 * state relevant to krb5 functions for authenticating principals and
 * protecting messages once authentication has occurred.
 *
 * By default, flags for the context are set to enable the use of the replay
 * cache (#KRB5_AUTH_CONTEXT_DO_TIME), but not sequence numbers.  Use
 * krb5_auth_con_setflags() to change the flags.
 *
 * The allocated @a auth_context must be freed with krb5_auth_con_free() when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Free a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context to be freed
 *
 * This function frees an auth context allocated by krb5_auth_con_init().
 *
 * @retval 0  (always)
 */
        /* *
 * Set a flags field in a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] flags            Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
        /* *
 * Retrieve flags from a krb5_auth_context structure.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] flags           Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
        /* *
 * Set a checksum callback in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] func             Checksum callback
 * @param [in] data             Callback argument
 *
 * Set a callback to obtain checksum data in krb5_mk_req().  The callback will
 * be invoked after the subkey and local sequence number are stored in @a
 * auth_context.
 *
 * @retval 0 (always)
 */
        /* *
 * Get the checksum callback from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] func            Checksum callback
 * @param [out] data            Callback argument
 *
 * @retval 0 (always)
 */
        /* *
 * Set the local and remote addresses in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_addr       Local address
 * @param [in] remote_addr      Remote address
 *
 * This function releases the storage assigned to the contents of the local and
 * remote addresses of @a auth_context and then sets them to @a local_addr and
 * @a remote_addr respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve address fields from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] local_addr      Local address (NULL if not needed)
 * @param [out] remote_addr     Remote address (NULL if not needed)
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set local and remote port fields in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_port       Local port
 * @param [in] remote_port      Remote port
 *
 * This function releases the storage assigned to the contents of the local and
 * remote ports of @a auth_context and then sets them to @a local_port and @a
 * remote_port respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set the session key in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] keyblock         User key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the session key from an auth context as a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] keyblock        Session key
 *
 * This function creates a keyblock containing the session key from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the session key from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] key             Session key
 *
 * This function sets @a key to the session key from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 (always)
 */
        /* *
 * Retrieve the send subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Send subkey
 *
 * This function creates a keyblock containing the send subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the send subkey from an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets @a key to the send subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Receiving subkey
 *
 * This function creates a keyblock containing the receiving subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Receiving subkey
 *
 * This function sets @a key to the receiving subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set the send subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Send subkey
 *
 * This function sets the send subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
        /* *
 * Set the send subkey in an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets the send subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set the receiving subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set the receiving subkey in an auth context.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] key              Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* * @deprecated Replaced by krb5_auth_con_getsendsubkey(). */
        /* * @deprecated Replaced by krb5_auth_con_getrecvsubkey(). */
        /* *
 * Retrieve the local sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Local sequence number
 *
 * Retrieve the local sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the remote sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Remote sequence number
 *
 * Retrieve the remote sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Cause an auth context to use cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 *
 * Prepare @a auth_context to use cipher state when krb5_mk_priv() or
 * krb5_rd_priv() encrypt or decrypt data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Set the replay cache in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rcache           Replay cache haddle
 *
 * This function sets the replay cache in @a auth_context to @a rcache.  @a
 * rcache will be closed when @a auth_context is freed, so the caller should
 * relinguish that responsibility.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Retrieve the replay cache from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] rcache          Replay cache handle
 *
 * This function fetches the replay cache from @a auth_context.  The caller
 * should not close @a rcache.
 *
 * @retval 0 (always)
 */
        /* *
 * Retrieve the authenticator from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] authenticator   Authenticator
 *
 * Use krb5_free_authenticator() to free @a authenticator when it is no longer
 * needed.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
        /* *
 * Set checksum type in an an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] cksumtype        Checksum type
 *
 * This function sets the checksum type in @a auth_context to be used by
 * krb5_mk_req() for the authenticator checksum.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
        /*
 * end "func-proto.h"
 */
        /*
 * begin stuff from libos.h
 */
        /* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert a principal name to a local name.
 *
 * @param [in]  context         Library context
 * @param [in]  aname           Principal name
 * @param [in]  lnsize_in       Space available in @a lname
 * @param [out] lname           Local name buffer to be filled in
 *
 * If @a aname does not correspond to any local account, KRB5_LNAME_NOTRANS is
 * returned.  If @a lnsize_in is too small for the local name,
 * KRB5_CONFIG_NOTENUFSPACE is returned.
 *
 * Local names, rather than principal names, can be used by programs that
 * translate to an environment-specific name (for example, a user account
 * name).
 *
 * @retval
 * 0  Success
 * @retval
 *  System errors
 * @return
 * Kerberos error codes
 */
        /* *
 * Get the Kerberos realm names for a host.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Host name (or NULL)
 * @param [out] realmsp         Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names.
 * If there are no known realms for the host, a list containing the referral
 * (empty) realm is returned.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 *
 * @retval
 *  0   Success
 * @retval
 *  ENOMEM  Insufficient memory
 * @return
 * Kerberos error codes
 */
        /* *
 *
 * @param [in] context           Library context
 * @param [in] hdata             Host name (or NULL)
 * @param [out] realmsp          Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names
 * obtained through heuristics or insecure resolution methods which have lower
 * priority than KDC referrals.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 */
        /* *
 * Free the memory allocated by krb5_get_host_realm().
 *
 * @param [in] context          Library context
 * @param [in] realmlist        List of realm names to be released
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6184:1"]
        pub fn krb5_free_host_realm(context: krb5_context,
                                    realmlist: *const *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "289:1"]
        pub fn krb5_is_referral_realm(r: *const krb5_data) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2421:1"]
        pub fn krb5_cc_store_cred(context: krb5_context, cache: krb5_ccache,
                                  creds: *mut krb5_creds) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2457:1"]
        pub fn krb5_cc_retrieve_cred(context: krb5_context,
                                     cache: krb5_ccache, flags: krb5_flags,
                                     mcreds: *mut krb5_creds,
                                     creds: *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4666:1"]
        pub fn krb5_free_creds(context: krb5_context, val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4633:1"]
        pub fn krb5_free_authdata(context: krb5_context,
                                  val: *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6169:1"]
        pub fn krb5_get_fallback_host_realm(context: krb5_context,
                                            hdata: *mut krb5_data,
                                            realmsp:
                                                *mut *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3863:1"]
        pub fn krb5_copy_authdata(context: krb5_context,
                                  in_authdat: *const *mut krb5_authdata,
                                  out: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4354:1"]
        pub fn krb5_cc_dup(context: krb5_context, in_0: krb5_ccache,
                           out: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3782:1"]
        pub fn krb5_copy_creds(context: krb5_context,
                               incred: *const krb5_creds,
                               outcred: *mut *mut krb5_creds)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:40"]
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
    #[c2rust::src_loc = "776:1"]
    pub type krb5_fast_armor = _krb5_fast_armor;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "776:16"]
    pub struct _krb5_fast_armor {
        pub armor_type: krb5_int32,
        pub armor_value: krb5_data,
    }
    #[inline]
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
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
    #[c2rust::src_loc = "2237:1"]
    pub unsafe extern "C" fn data_eq(mut d1: krb5_data, mut d2: krb5_data)
     -> libc::c_int {
        return (d1.length == d2.length &&
                    (d1.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d1.data as *const libc::c_void,
                                d2.data as *const libc::c_void,
                                d1.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2244:1"]
    pub unsafe extern "C" fn data_eq_string(mut d: krb5_data,
                                            mut s: *const libc::c_char)
     -> libc::c_int {
        return (d.length as libc::c_ulong == strlen(s) &&
                    (d.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d.data as *const libc::c_void,
                                s as *const libc::c_void,
                                d.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
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
    #[inline]
    #[c2rust::src_loc = "2326:1"]
    pub unsafe extern "C" fn k5memdup0(mut in_0: *const libc::c_void,
                                       mut len: size_t,
                                       mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void =
            k5alloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_context, krb5_error_code,
                        krb5_timestamp, krb5_principal_data,
                        krb5_const_principal};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::{memcmp, strlen, memcpy};
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
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
        #[no_mangle]
        #[c2rust::src_loc = "1004:1"]
        pub fn krb5int_free_data_list(context: krb5_context,
                                      data: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "616:1"]
        pub fn krb5_sendto_kdc(_: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_data,
                               _: *mut libc::c_int, _: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2090:1"]
        pub fn krb5_get_tgs_ktypes(_: krb5_context, _: krb5_const_principal,
                                   _: *mut *mut krb5_enctype)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:40"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:40"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:40"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/fast.h:43"]
pub mod fast_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/fast.h */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct krb5int_fast_request_state {
        pub fast_outer_request: krb5_kdc_req,
        pub armor_key: *mut krb5_keyblock,
        pub armor: *mut krb5_fast_armor,
        pub fast_state_flags: krb5_ui_4,
        pub fast_options: krb5_ui_4,
        pub nonce: krb5_int32,
    }
    use super::krb5_h::{krb5_kdc_req, krb5_keyblock, krb5_ui_4, krb5_int32,
                        krb5_context, krb5_error_code};
    use super::k5_int_h::{krb5_fast_armor, _krb5_context};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn krb5int_fast_make_state(context: krb5_context,
                                       state:
                                           *mut *mut krb5int_fast_request_state)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn krb5int_fast_free_state(context: krb5_context,
                                       state:
                                           *mut krb5int_fast_request_state);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:41"]
pub mod int_proto_h {
    #[c2rust::src_loc = "89:1"]
    pub type k5_pacb_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_keyblock,
                                    _: *mut krb5_kdc_req,
                                    _: *mut libc::c_void) -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_keyblock,
                        krb5_kdc_req, krb5_data, krb5_principal, krb5_creds,
                        krb5_flags, krb5_address, krb5_pa_data,
                        krb5_timestamp, krb5_int32, _krb5_ccache,
                        krb5_ccache};
    use super::k5_int_h::_krb5_context;
    use super::fast_h::krb5int_fast_request_state;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn krb5int_tgtname(context: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "105:1"]
        pub fn k5_make_tgs_req(context: krb5_context,
                               _: *mut krb5int_fast_request_state,
                               tkt: *mut krb5_creds, kdcoptions: krb5_flags,
                               address: *const *mut krb5_address,
                               in_padata: *mut *mut krb5_pa_data,
                               in_cred: *mut krb5_creds, pacb_fn: k5_pacb_fn,
                               pacb_data: *mut libc::c_void,
                               req_asn1_out: *mut krb5_data,
                               timestamp_out: *mut krb5_timestamp,
                               nonce_out: *mut krb5_int32,
                               subkey_out: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "113:1"]
        pub fn krb5int_process_tgs_reply(context: krb5_context,
                                         _: *mut krb5int_fast_request_state,
                                         response_data: *mut krb5_data,
                                         tkt: *mut krb5_creds,
                                         kdcoptions: krb5_flags,
                                         address: *const *mut krb5_address,
                                         in_padata: *mut *mut krb5_pa_data,
                                         in_cred: *mut krb5_creds,
                                         timestamp: krb5_timestamp,
                                         nonce: krb5_int32,
                                         subkey: *mut krb5_keyblock,
                                         out_padata:
                                             *mut *mut *mut krb5_pa_data,
                                         out_enc_padata:
                                             *mut *mut *mut krb5_pa_data,
                                         out_cred: *mut *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "314:1"]
        pub fn k5_copy_creds_contents(_: krb5_context, _: *const krb5_creds,
                                      _: *mut krb5_creds) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "327:1"]
        pub fn k5_client_realm_path(context: krb5_context,
                                    client: *const krb5_data,
                                    server: *const krb5_data,
                                    rpath_out: *mut *mut krb5_data)
         -> krb5_error_code;
        /*
 * Make an S4U2Proxy (constrained delegation) request.  in_creds->client is the
 * impersonator principal, and in_creds->second_ticket is the evidence
 * ticket.
 */
        #[no_mangle]
        #[c2rust::src_loc = "384:1"]
        pub fn k5_get_proxy_cred_from_kdc(context: krb5_context,
                                          options: krb5_flags,
                                          ccache: krb5_ccache,
                                          in_creds: *mut krb5_creds,
                                          out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src = "/usr/include/assert.h:40"]
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
#[c2rust::header_src = "/usr/include/string.h:40"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:40"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:40"]
pub mod k5_trace_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-trace.h */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
 * This header contains trace macro definitions, which map trace points within
 * the code to krb5int_trace() calls with descriptive text strings.
 *
 * A new trace macro must be defined in this file for each new location to
 * be traced; the TRACE() macro should never be used directly.  This keeps
 * the tracing logic centralized in one place, to facilitate integration with
 * alternate tracing backends such as DTrace.
 *
 * Trace logging is intended to aid power users in diagnosing configuration
 * problems by showing what's going on behind the scenes of complex operations.
 * Although trace logging is sometimes useful to developers, it is not intended
 * as a replacement for a debugger, and it is not desirable to drown the user
 * in output.  Observe the following guidelines when adding trace points:
 *
 *   - Avoid mentioning function or variable names in messages.
 *
 *   - Try to convey what decisions are being made and what external inputs
 *     they are based on, not the process of making decisions.
 *
 *   - It is generally not necessary to trace before returning an unrecoverable
 *     error.  If an error code is unclear by itself, make it clearer with
 *     krb5_set_error_message().
 *
 *   - Keep macros simple.  Add format specifiers to krb5int_trace's formatter
 *     as necessary (and document them here) instead of transforming macro
 *     arguments.
 *
 *   - Like printf, the trace formatter interface is not type-safe.  Check your
 *     formats carefully.  Cast integral arguments to the appropriate type if
 *     they do not already patch.
 *
 * The following specifiers are supported by the formatter (see the
 * implementation in lib/krb5/os/trace.c for details):
 *
 *   {int}         int, in decimal
 *   {long}        long, in decimal
 *   {str}         const char *, display as C string
 *   {lenstr}      size_t and const char *, as a counted string
 *   {hexlenstr}   size_t and const char *, as hex bytes
 *   {hashlenstr}  size_t and const char *, as four-character hex hash
 *   {raddr}       struct remote_address *, show socket type, address, port
 *   {data}        krb5_data *, display as counted string
 *   {hexdata}     krb5_data *, display as hex bytes
 *   {errno}       int, display as number/errorstring
 *   {kerr}        krb5_error_code, display as number/errorstring
 *   {keyblock}    const krb5_keyblock *, display enctype and hash of key
 *   {key}         krb5_key, display enctype and hash of key
 *   {cksum}       const krb5_checksum *, display cksumtype and hex checksum
 *   {princ}       krb5_principal, unparse and display
 *   {ptype}       krb5_int32, krb5_principal type, display name
 *   {patype}      krb5_preauthtype, a single padata type number
 *   {patypes}     krb5_pa_data **, display list of padata type numbers
 *   {etype}       krb5_enctype, display shortest name of enctype
 *   {etypes}      krb5_enctype *, display list of enctypes
 *   {ccache}      krb5_ccache, display type:name
 *   {keytab}      krb5_keytab, display name
 *   {creds}       krb5_creds *, display clientprinc -> serverprinc
 */
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn krb5int_trace(context: krb5_context, fmt: *const libc::c_char,
                             _: ...);
    }
    /* K5_TRACE_H */
    /* DISABLE_TRACING */
    /* Try to optimize away argument evaluation and function call when we're not
 * tracing, if this source file knows the internals of the context. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:42"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_boolean, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn k5_expand_hostname(context: krb5_context,
                                  host: *const libc::c_char,
                                  is_fallback: krb5_boolean,
                                  canonhost_out: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_enc_data, krb5_enc_data,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _krb5_ticket,
                       krb5_ticket, _krb5_creds, krb5_creds, _krb5_pa_data,
                       krb5_pa_data, _krb5_kdc_req, krb5_kdc_req, krb5_ccache,
                       krb5_tkt_creds_context, _profile_t, _krb5_ccache,
                       krb5_free_host_realm, krb5_is_referral_realm,
                       krb5_c_valid_enctype, krb5_cc_close,
                       krb5_cc_store_cred, krb5_cc_retrieve_cred,
                       krb5_free_creds, krb5_free_data_contents,
                       krb5_free_keyblock, krb5_free_principal,
                       krb5_free_authdata, krb5_principal_compare,
                       krb5_timeofday, krb5_get_fallback_host_realm,
                       krb5_copy_authdata, krb5_cc_dup, krb5_copy_principal,
                       krb5_copy_creds};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_fast_armor, _krb5_fast_armor,
                         empty_data, make_data, data_eq, data_eq_string,
                         ts_after, string2data, k5alloc, k5calloc, k5memdup0,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_free_data_list,
                         krb5_sendto_kdc, krb5int_copy_data_contents,
                         krb5_get_tgs_ktypes};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::fast_h::{krb5int_fast_request_state, krb5int_fast_make_state,
                       krb5int_fast_free_state};
pub use self::int_proto_h::{k5_pacb_fn, krb5int_tgtname, k5_make_tgs_req,
                            krb5int_process_tgs_reply, k5_copy_creds_contents,
                            k5_client_realm_path, k5_get_proxy_cred_from_kdc};
use self::assert_h::__assert_fail;
use self::string_h::{memcpy, memset, memcmp, strlen};
use self::stdlib_h::{malloc, calloc, realloc, free};
use self::k5_trace_h::krb5int_trace;
use self::os_proto_h::k5_expand_hostname;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "143:8"]
pub struct _krb5_tkt_creds_context {
    pub state: state,
    pub getting_tgt_for: state,
    pub in_creds: *mut krb5_creds,
    pub client: krb5_principal,
    pub server: krb5_principal,
    pub req_server: krb5_principal,
    pub ccache: krb5_ccache,
    pub req_options: krb5_flags,
    pub req_kdcopt: krb5_flags,
    pub authdata: *mut *mut krb5_authdata,
    pub cur_tgt: *mut krb5_creds,
    pub realms_seen: *mut krb5_data,
    pub tgt_princ: krb5_principal,
    pub tgt_in_creds: krb5_creds,
    pub tgs_in_creds: *mut krb5_creds,
    pub timestamp: krb5_timestamp,
    pub nonce: krb5_int32,
    pub kdcopt: libc::c_int,
    pub subkey: *mut krb5_keyblock,
    pub previous_request: krb5_data,
    pub fast_state: *mut krb5int_fast_request_state,
    pub realm_path: *mut krb5_data,
    pub last_realm: *const krb5_data,
    pub cur_realm: *const krb5_data,
    pub next_realm: *const krb5_data,
    pub offpath_count: libc::c_uint,
    pub referral_count: libc::c_uint,
    pub reply_creds: *mut krb5_creds,
    pub reply_code: krb5_error_code,
    pub caller_out: *mut krb5_data,
    pub caller_realm: *mut krb5_data,
    pub caller_flags: *mut libc::c_uint,
}
/*
 * krb5_tkt_creds_step() is implemented using a tail call style.  Every
 * begin_*, step_*, or *_request function is responsible for returning an
 * error, generating the next request, or delegating to another function using
 * a tail call.
 *
 * The process is divided up into states which govern how the next input token
 * should be interpreted.  Each state has a "begin_<state>" function to set up
 * the context fields related to the state, a "step_<state>" function to
 * process a reply and update the related context fields, and possibly a
 * "<state>_request" function (invoked by the begin_ and step_ functions) to
 * generate the next request.  If it's time to advance to another state, any of
 * the three functions can make a tail call to begin_<nextstate> to do so.
 *
 * The overall process is as follows:
 *   1. Get a TGT for the service principal's realm (STATE_GET_TGT).
 *   2. Make one or more referrals queries (STATE_REFERRALS).
 *   3. In some cases, get a TGT for the fallback realm (STATE_GET_TGT again).
 *   4. In some cases, make a non-referral query (STATE_NON_REFERRAL).
 *
 * STATE_GET_TGT can precede either STATE_REFERRALS or STATE_NON_REFERRAL.  The
 * getting_tgt_for field in the context keeps track of what state we will go to
 * after successfully obtaining the TGT, and the end_get_tgt() function
 * advances to the proper next state.
 */
#[c2rust::src_loc = "134:1"]
pub type state = libc::c_uint;
/* Creds ready for retrieval */
/* Non-referral service ticket request */
#[c2rust::src_loc = "140:5"]
pub const STATE_COMPLETE: state = 5;
/* Retrieving service ticket or referral */
#[c2rust::src_loc = "139:5"]
pub const STATE_NON_REFERRAL: state = 4;
/* Getting TGT via off-path referrals */
#[c2rust::src_loc = "138:5"]
pub const STATE_REFERRALS: state = 3;
/* Getting TGT for service realm */
#[c2rust::src_loc = "137:5"]
pub const STATE_GET_TGT_OFFPATH: state = 2;
/* Initial step (no input token) */
#[c2rust::src_loc = "136:5"]
pub const STATE_GET_TGT: state = 1;
#[c2rust::src_loc = "135:5"]
pub const STATE_BEGIN: state = 0;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/get_creds.c */
/*
 * Copyright 1990, 2008, 2010 by the Massachusetts Institute of Technology.
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
 * Attempts to use the credentials cache or TGS exchange to get an additional
 * ticket for the client identified by in_creds->client, the server identified
 * by in_creds->server, with options options, expiration date specified in
 * in_creds->times.endtime (0 means as long as possible), session key type
 * specified in in_creds->keyblock.enctype (if non-zero)
 *
 * Any returned ticket and intermediate ticket-granting tickets are stored in
 * ccache.
 *
 * Returns errors from encryption routines, system errors.
 */
/*
 * Set *mcreds and *fields to a matching credential and field set for
 * use with krb5_cc_retrieve_cred, based on a set of input credentials
 * and options.  The fields of *mcreds will be aliased to the fields
 * of in_creds, so the contents of *mcreds should not be freed.
 */
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn krb5int_construct_matching_creds(mut context:
                                                              krb5_context,
                                                          mut options:
                                                              krb5_flags,
                                                          mut in_creds:
                                                              *mut krb5_creds,
                                                          mut mcreds:
                                                              *mut krb5_creds,
                                                          mut fields:
                                                              *mut krb5_flags)
 -> krb5_error_code {
    if in_creds.is_null() || (*in_creds).server.is_null() ||
           (*in_creds).client.is_null() {
        return 22 as libc::c_int
    }
    memset(mcreds as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    (*mcreds).magic = -(1760647408 as libc::c_long) as krb5_magic;
    if (*in_creds).times.endtime != 0 as libc::c_int {
        (*mcreds).times.endtime = (*in_creds).times.endtime
    } else {
        let mut retval: krb5_error_code = 0;
        retval = krb5_timeofday(context, &mut (*mcreds).times.endtime);
        if retval != 0 as libc::c_int { return retval }
    }
    (*mcreds).keyblock = (*in_creds).keyblock;
    (*mcreds).authdata = (*in_creds).authdata;
    (*mcreds).server = (*in_creds).server;
    (*mcreds).client = (*in_creds).client;
    *fields = 0x1 as libc::c_int | 0x20 as libc::c_int | 0x200 as libc::c_int;
    if (*mcreds).keyblock.enctype != 0 {
        let mut ktypes: *mut krb5_enctype = 0 as *mut krb5_enctype;
        let mut ret: krb5_error_code = 0;
        let mut i: libc::c_int = 0;
        *fields |= 0x100 as libc::c_int;
        ret =
            krb5_get_tgs_ktypes(context,
                                (*mcreds).server as krb5_const_principal,
                                &mut ktypes);
        i = 0 as libc::c_int;
        while *ktypes.offset(i as isize) != 0 {
            if *ktypes.offset(i as isize) == (*mcreds).keyblock.enctype {
                break ;
            }
            i += 1
        }
        if *ktypes.offset(i as isize) == 0 as libc::c_int {
            ret = -(1765328184 as libc::c_long) as krb5_error_code
        }
        free(ktypes as *mut libc::c_void);
        if ret != 0 { return ret }
    }
    if options & (1 as libc::c_int | 64 as libc::c_int) != 0 {
        /* also match on identical 2nd tkt and tkt encrypted in a
           session key */
        *fields |= 0x80 as libc::c_int;
        if options & 1 as libc::c_int != 0 {
            *fields |= 0x2 as libc::c_int;
            (*mcreds).is_skey = 1 as libc::c_int as krb5_boolean
        }
        (*mcreds).second_ticket = (*in_creds).second_ticket;
        if (*in_creds).second_ticket.length == 0 {
            return -(1765328182 as libc::c_long) as krb5_error_code
        }
    }
    return 0 as libc::c_int;
}
/*
 * Fill in the caller out, realm, and flags output variables.  out is filled in
 * with ctx->previous_request, which the caller should set, and realm is filled
 * in with the realm of ctx->cur_tgt.
 */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn set_caller_request(mut context: krb5_context,
                                        mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut req: *const krb5_data = &mut (*ctx).previous_request;
    let mut realm: *const krb5_data =
        &mut *(*(*(*ctx).cur_tgt).server).data.offset(1 as libc::c_int as
                                                          isize) as
            *mut krb5_data;
    let mut out_copy: krb5_data = empty_data();
    let mut realm_copy: krb5_data = empty_data();
    code = krb5int_copy_data_contents(context, req, &mut out_copy);
    if !(code != 0 as libc::c_int) {
        code = krb5int_copy_data_contents(context, realm, &mut realm_copy);
        if !(code != 0 as libc::c_int) {
            *(*ctx).caller_out = out_copy;
            *(*ctx).caller_realm = realm_copy;
            *(*ctx).caller_flags = 0x1 as libc::c_int as libc::c_uint;
            return 0 as libc::c_int
        }
    }
    krb5_free_data_contents(context, &mut out_copy);
    krb5_free_data_contents(context, &mut realm_copy);
    return code;
}
/* Simple wrapper around krb5_cc_retrieve_cred which allocates the result
 * container. */
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn cache_get(mut context: krb5_context,
                               mut ccache: krb5_ccache, mut flags: krb5_flags,
                               mut in_creds: *mut krb5_creds,
                               mut out_creds: *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut creds: *mut krb5_creds = 0 as *mut krb5_creds;
    *out_creds = 0 as *mut krb5_creds;
    creds =
        malloc(::std::mem::size_of::<krb5_creds>() as libc::c_ulong) as
            *mut krb5_creds;
    if creds.is_null() { return 12 as libc::c_int }
    code = krb5_cc_retrieve_cred(context, ccache, flags, in_creds, creds);
    if code != 0 as libc::c_int {
        free(creds as *mut libc::c_void);
        return code
    }
    *out_creds = creds;
    return 0 as libc::c_int;
}
/*
 * Set up the request given by ctx->tgs_in_creds, using ctx->cur_tgt.  KDC
 * options for the requests are determined by ctx->cur_tgt->ticket_flags and
 * extra_options.
 */
#[c2rust::src_loc = "258:1"]
unsafe extern "C" fn make_request(mut context: krb5_context,
                                  mut ctx: krb5_tkt_creds_context,
                                  mut extra_options: libc::c_int)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut request: krb5_data = empty_data();
    (*ctx).kdcopt =
        extra_options |
            (*(*ctx).cur_tgt).ticket_flags & 0x54800000 as libc::c_int;
    /* XXX This check belongs in gc_via_tgt.c or nowhere. */
    if krb5_c_valid_enctype((*(*ctx).cur_tgt).keyblock.enctype) == 0 {
        return -(1765328234 as libc::c_long) as krb5_error_code
    }
    /* Create a new FAST state structure to store this request's armor key. */
    krb5int_fast_free_state(context, (*ctx).fast_state);
    (*ctx).fast_state = 0 as *mut krb5int_fast_request_state;
    code = krb5int_fast_make_state(context, &mut (*ctx).fast_state);
    if code != 0 { return code }
    krb5_free_keyblock(context, (*ctx).subkey);
    (*ctx).subkey = 0 as *mut krb5_keyblock;
    code =
        k5_make_tgs_req(context, (*ctx).fast_state, (*ctx).cur_tgt,
                        (*ctx).kdcopt, (*(*ctx).cur_tgt).addresses,
                        0 as *mut *mut krb5_pa_data, (*ctx).tgs_in_creds,
                        None, 0 as *mut libc::c_void, &mut request,
                        &mut (*ctx).timestamp, &mut (*ctx).nonce,
                        &mut (*ctx).subkey);
    if code != 0 as libc::c_int { return code }
    krb5_free_data_contents(context, &mut (*ctx).previous_request);
    (*ctx).previous_request = request;
    return set_caller_request(context, ctx);
}
/* Set up a request for a TGT for realm, using ctx->cur_tgt. */
#[c2rust::src_loc = "293:1"]
unsafe extern "C" fn make_request_for_tgt(mut context: krb5_context,
                                          mut ctx: krb5_tkt_creds_context,
                                          mut realm: *const krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    /* Construct the principal krbtgt/<realm>@<cur-tgt-realm>. */
    krb5_free_principal(context, (*ctx).tgt_princ);
    (*ctx).tgt_princ = 0 as krb5_principal;
    code =
        krb5int_tgtname(context, realm,
                        &mut *(*(*(*ctx).cur_tgt).server).data.offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                        &mut (*ctx).tgt_princ);
    if code != 0 as libc::c_int { return code }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Requesting TGT {princ} using TGT {princ}\x00" as
                          *const u8 as *const libc::c_char, (*ctx).tgt_princ,
                      (*(*ctx).cur_tgt).server);
    }
    /* Construct input creds using ctx->tgt_in_creds as a container. */
    memset(&mut (*ctx).tgt_in_creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    (*ctx).tgt_in_creds.client = (*ctx).client;
    (*ctx).tgt_in_creds.server = (*ctx).tgt_princ;
    /* Make a request for the above creds with no extra options. */
    (*ctx).tgs_in_creds = &mut (*ctx).tgt_in_creds;
    code = make_request(context, ctx, 0 as libc::c_int);
    return code;
}
/* Set up a request for the desired service principal, using ctx->cur_tgt.
 * Optionally allow the answer to be a referral. */
#[c2rust::src_loc = "322:1"]
unsafe extern "C" fn make_request_for_service(mut context: krb5_context,
                                              mut ctx: krb5_tkt_creds_context,
                                              mut referral: krb5_boolean)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut extra_options: libc::c_int = 0;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Requesting tickets for {princ}, referrals {str}\x00"
                          as *const u8 as *const libc::c_char, (*ctx).server,
                      if referral != 0 {
                          b"on\x00" as *const u8 as *const libc::c_char
                      } else {
                          b"off\x00" as *const u8 as *const libc::c_char
                      });
    }
    /* Include the caller-specified KDC options in service requests. */
    extra_options = (*ctx).req_kdcopt;
    /* Automatically set the enc-tkt-in-skey flag for user-to-user requests. */
    if (*(*ctx).in_creds).second_ticket.length !=
           0 as libc::c_int as libc::c_uint {
        extra_options |= 0x8 as libc::c_int
    }
    /* Set the canonicalize flag for referral requests. */
    if referral != 0 { extra_options |= 0x10000 as libc::c_int }
    /*
     * Use the profile enctypes for referral requests, since we might get back
     * a TGT.  We'll ask again with context enctypes if we get the actual
     * service ticket and it's not consistent with the context enctypes.
     */
    if referral != 0 {
        (*context).use_conf_ktypes = 1 as libc::c_int as krb5_boolean
    }
    (*ctx).tgs_in_creds = (*ctx).in_creds;
    code = make_request(context, ctx, extra_options);
    if referral != 0 {
        (*context).use_conf_ktypes = 0 as libc::c_int as krb5_boolean
    }
    return code;
}
/* Decode and decrypt a TGS reply, and set the reply_code or reply_creds field
 * of ctx with the result.  Also handle too-big errors. */
#[c2rust::src_loc = "358:1"]
unsafe extern "C" fn get_creds_from_tgs_reply(mut context: krb5_context,
                                              mut ctx: krb5_tkt_creds_context,
                                              mut reply: *mut krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    krb5_free_creds(context, (*ctx).reply_creds);
    (*ctx).reply_creds = 0 as *mut krb5_creds;
    code =
        krb5int_process_tgs_reply(context, (*ctx).fast_state, reply,
                                  (*ctx).cur_tgt, (*ctx).kdcopt,
                                  (*(*ctx).cur_tgt).addresses,
                                  0 as *mut *mut krb5_pa_data,
                                  (*ctx).tgs_in_creds, (*ctx).timestamp,
                                  (*ctx).nonce, (*ctx).subkey,
                                  0 as *mut *mut *mut krb5_pa_data,
                                  0 as *mut *mut *mut krb5_pa_data,
                                  &mut (*ctx).reply_creds);
    if code as libc::c_long == -(1765328332 as libc::c_long) {
        /* Instruct the caller to re-send the request with TCP. */
        code = set_caller_request(context, ctx);
        if code != 0 as libc::c_int { return code }
        return -(1765328332 as libc::c_long) as krb5_error_code
    }
    /* Depending on our state, we may or may not be able to handle an error.
     * For now, store it in the context and return success. */
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"TGS request result: {kerr}\x00" as *const u8 as
                          *const libc::c_char, code);
    }
    (*ctx).reply_code = code;
    return 0 as libc::c_int;
}
/* Add realm to ctx->realms_seen so that we can avoid revisiting it later. */
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn remember_realm(mut context: krb5_context,
                                    mut ctx: krb5_tkt_creds_context,
                                    mut realm: *const krb5_data)
 -> krb5_error_code {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut new_list: *mut krb5_data = 0 as *mut krb5_data;
    if !(*ctx).realms_seen.is_null() {
        len = 0 as libc::c_int as size_t;
        while !(*(*ctx).realms_seen.offset(len as isize)).data.is_null() {
            len = len.wrapping_add(1)
        }
    }
    new_list =
        realloc((*ctx).realms_seen as *mut libc::c_void,
                len.wrapping_add(2 as libc::c_int as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_data>()
                                                                     as
                                                                     libc::c_ulong))
            as *mut krb5_data;
    if new_list.is_null() { return 12 as libc::c_int }
    (*ctx).realms_seen = new_list;
    *new_list.offset(len as isize) = empty_data();
    *new_list.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                         isize) = empty_data();
    return krb5int_copy_data_contents(context, realm,
                                      &mut *new_list.offset(len as isize));
}
/* Return TRUE if realm appears to ctx->realms_seen. */
#[c2rust::src_loc = "408:1"]
unsafe extern "C" fn seen_realm_before(mut context: krb5_context,
                                       mut ctx: krb5_tkt_creds_context,
                                       mut realm: *const krb5_data)
 -> krb5_boolean {
    let mut i: size_t = 0;
    if !(*ctx).realms_seen.is_null() {
        i = 0 as libc::c_int as size_t;
        while !(*(*ctx).realms_seen.offset(i as isize)).data.is_null() {
            if data_eq(*(*ctx).realms_seen.offset(i as isize), *realm) != 0 {
                return 1 as libc::c_int as krb5_boolean
            }
            i = i.wrapping_add(1)
        }
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* **** STATE_COMPLETE *****/
/* Check and cache the desired credential when we receive it.  Expects the
 * received credential to be in ctx->reply_creds. */
#[c2rust::src_loc = "427:1"]
unsafe extern "C" fn complete(mut context: krb5_context,
                              mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Received creds for desired service {princ}\x00" as
                          *const u8 as *const libc::c_char,
                      (*(*ctx).reply_creds).server);
    }
    /* Put the requested server principal in the output creds. */
    krb5_free_principal(context, (*(*ctx).reply_creds).server);
    (*(*ctx).reply_creds).server = (*ctx).req_server;
    (*ctx).req_server = 0 as krb5_principal;
    /* Note the authdata we asked for in the output creds. */
    (*(*ctx).reply_creds).authdata = (*ctx).authdata;
    (*ctx).authdata = 0 as *mut *mut krb5_authdata;
    if (*ctx).req_options & 8 as libc::c_int == 0 {
        /* Try to cache the credential. */
        krb5_cc_store_cred(context, (*ctx).ccache, (*ctx).reply_creds);
    }
    (*ctx).state = STATE_COMPLETE;
    return 0 as libc::c_int;
}
/* **** STATE_NON_REFERRAL *****/
/* Process the response to a non-referral request. */
#[c2rust::src_loc = "453:1"]
unsafe extern "C" fn step_non_referral(mut context: krb5_context,
                                       mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    /* No fallbacks if we didn't get a successful reply. */
    if (*ctx).reply_code != 0 { return (*ctx).reply_code }
    return complete(context, ctx);
}
/* Make a non-referrals request for the desired service ticket. */
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn begin_non_referral(mut context: krb5_context,
                                        mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    (*ctx).state = STATE_NON_REFERRAL;
    return make_request_for_service(context, ctx,
                                    0 as libc::c_int as krb5_boolean);
}
/* **** STATE_REFERRALS *****/
/* Possibly try a non-referral request after a referral request failure.
 * Expects ctx->reply_code to be set to the error from a referral request. */
#[c2rust::src_loc = "475:1"]
unsafe extern "C" fn try_fallback(mut context: krb5_context,
                                  mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut hrealms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* Only fall back if our error was from the first referral request. */
    if (*ctx).referral_count > 1 as libc::c_int as libc::c_uint {
        return (*ctx).reply_code
    }
    /* If the request used a specified realm, make a non-referral request to
     * that realm (in case it's a KDC which rejects KDC_OPT_CANONICALIZE). */
    if krb5_is_referral_realm(&mut (*(*ctx).req_server).realm) == 0 {
        return begin_non_referral(context, ctx)
    }
    if (*(*ctx).server).length < 2 as libc::c_int {
        /* We need a type/host format principal to find a fallback realm. */
        return -(1765328167 as libc::c_long) as krb5_error_code
    }
    /* We expect this to give exactly one answer (XXX clean up interface). */
    code =
        krb5_get_fallback_host_realm(context,
                                     &mut *(*(*ctx).server).data.offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                     &mut hrealms);
    if code != 0 as libc::c_int { return code }
    /* If the fallback realm isn't any different, use the existing TGT. */
    if data_eq_string((*(*ctx).server).realm,
                      *hrealms.offset(0 as libc::c_int as isize)) != 0 {
        krb5_free_host_realm(context, hrealms);
        return begin_non_referral(context, ctx)
    }
    /* Rewrite server->realm to be the fallback realm. */
    krb5_free_data_contents(context, &mut (*(*ctx).server).realm);
    (*(*ctx).server).realm =
        string2data(*hrealms.offset(0 as libc::c_int as isize));
    free(hrealms as *mut libc::c_void);
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Local realm referral failed; trying fallback realm {data}\x00"
                          as *const u8 as *const libc::c_char,
                      &mut (*(*ctx).server).realm as *mut krb5_data);
    }
    /* Obtain a TGT for the new service realm. */
    (*ctx).getting_tgt_for = STATE_NON_REFERRAL;
    return begin_get_tgt(context, ctx);
}
/* Return true if context contains app-provided TGS enctypes and enctype is not
 * one of them. */
#[c2rust::src_loc = "520:1"]
unsafe extern "C" fn wrong_enctype(mut context: krb5_context,
                                   mut enctype: krb5_enctype)
 -> krb5_boolean {
    let mut i: size_t = 0;
    if (*context).tgs_etypes.is_null() {
        return 0 as libc::c_int as krb5_boolean
    }
    i = 0 as libc::c_int as size_t;
    while *(*context).tgs_etypes.offset(i as isize) != 0 as libc::c_int {
        if enctype == *(*context).tgs_etypes.offset(i as isize) {
            return 0 as libc::c_int as krb5_boolean
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int as krb5_boolean;
}
/* Advance the referral request loop. */
#[c2rust::src_loc = "535:1"]
unsafe extern "C" fn step_referrals(mut context: krb5_context,
                                    mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut referral_realm: *const krb5_data = 0 as *const krb5_data;
    /* Possibly try a non-referral fallback request on error. */
    if (*ctx).reply_code != 0 as libc::c_int {
        return try_fallback(context, ctx)
    }
    if krb5_principal_compare(context,
                              (*(*ctx).reply_creds).server as
                                  krb5_const_principal,
                              (*ctx).server as krb5_const_principal) != 0 {
        /* We got the ticket we asked for... but we didn't necessarily ask for
         * it with the right enctypes.  Try a non-referral request if so. */
        if wrong_enctype(context, (*(*ctx).reply_creds).keyblock.enctype) != 0
           {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Retrying TGS request with desired service ticket enctypes\x00"
                                  as *const u8 as *const libc::c_char);
            }
            return begin_non_referral(context, ctx)
        }
        return complete(context, ctx)
    }
    /* Old versions of Active Directory can rewrite the server name instead of
     * returning a referral.  Try a non-referral query if we see this. */
    if !((*(*(*ctx).reply_creds).server).length == 2 as libc::c_int &&
             data_eq_string(*(*(*(*ctx).reply_creds).server).data.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                            b"krbtgt\x00" as *const u8 as *const libc::c_char)
                 != 0) {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Received non-TGT referral response ({princ}); trying again without referrals\x00"
                              as *const u8 as *const libc::c_char,
                          (*(*ctx).reply_creds).server);
        }
        return begin_non_referral(context, ctx)
    }
    /* Active Directory may return a TGT to the local realm.  Try a
     * non-referral query if we see this. */
    referral_realm =
        &mut *(*(*(*ctx).reply_creds).server).data.offset(1 as libc::c_int as
                                                              isize) as
            *mut krb5_data;
    if data_eq(*referral_realm,
               *(*(*(*ctx).cur_tgt).server).data.offset(1 as libc::c_int as
                                                            isize)) != 0 {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Received TGT referral back to same realm ({data}); trying again without referrals\x00"
                              as *const u8 as *const libc::c_char,
                          referral_realm);
        }
        return begin_non_referral(context, ctx)
    }
    if (*ctx).referral_count == 1 as libc::c_int as libc::c_uint {
        /* The authdata in this TGT will be copied into subsequent TGTs or the
         * final credentials, so we don't need to request it again. */
        krb5_free_authdata(context, (*(*ctx).in_creds).authdata);
        (*(*ctx).in_creds).authdata = 0 as *mut *mut krb5_authdata
    }
    /* Give up if we've gotten too many referral TGTs. */
    let fresh0 = (*ctx).referral_count;
    (*ctx).referral_count = (*ctx).referral_count.wrapping_add(1);
    if fresh0 >= 10 as libc::c_int as libc::c_uint {
        return -(1765328228 as libc::c_long) as krb5_error_code
    }
    /* Check for referral loops. */
    if seen_realm_before(context, ctx, referral_realm) != 0 {
        return -(1765328228 as libc::c_long) as krb5_error_code
    }
    code = remember_realm(context, ctx, referral_realm);
    if code != 0 as libc::c_int { return code }
    /* Use the referral TGT for the next request. */
    krb5_free_creds(context, (*ctx).cur_tgt);
    (*ctx).cur_tgt = (*ctx).reply_creds;
    (*ctx).reply_creds = 0 as *mut krb5_creds;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Following referral TGT {princ}\x00" as *const u8 as
                          *const libc::c_char, (*(*ctx).cur_tgt).server);
    }
    /* Rewrite the server realm to be the referral realm. */
    krb5_free_data_contents(context, &mut (*(*ctx).server).realm);
    code =
        krb5int_copy_data_contents(context, referral_realm,
                                   &mut (*(*ctx).server).realm);
    if code != 0 as libc::c_int { return code }
    /* Generate the next referral request. */
    return make_request_for_service(context, ctx,
                                    1 as libc::c_int as krb5_boolean);
}
/*
 * Begin the referrals request loop.  Expects ctx->cur_tgt to be a TGT for
 * ctx->realm->server.
 */
#[c2rust::src_loc = "611:1"]
unsafe extern "C" fn begin_referrals(mut context: krb5_context,
                                     mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    (*ctx).state = STATE_REFERRALS;
    (*ctx).referral_count = 1 as libc::c_int as libc::c_uint;
    /* Empty out the realms-seen list for loop checking. */
    krb5int_free_data_list(context, (*ctx).realms_seen);
    (*ctx).realms_seen = 0 as *mut krb5_data;
    /* Generate the first referral request. */
    return make_request_for_service(context, ctx,
                                    1 as libc::c_int as krb5_boolean);
}
/* **** STATE_GET_TGT_OFFPATH *****/
/*
 * Foreign TGT acquisition can happen either before the referrals loop, if the
 * service principal had an explicitly specified foreign realm, or after it
 * fails, if we wind up using the fallback realm.  end_get_tgt() advances to
 * the appropriate state depending on which we were doing.
 */
#[c2rust::src_loc = "633:1"]
unsafe extern "C" fn end_get_tgt(mut context: krb5_context,
                                 mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    if (*ctx).getting_tgt_for as libc::c_uint ==
           STATE_REFERRALS as libc::c_int as libc::c_uint {
        return begin_referrals(context, ctx)
    } else { return begin_non_referral(context, ctx) };
}
/*
 * We enter STATE_GET_TGT_OFFPATH from STATE_GET_TGT if we receive, from one of
 * the KDCs in the expected path, a TGT for a realm not in the path.  This may
 * happen if the KDC has a different idea of the expected path than we do.  If
 * it happens, we repeatedly ask the KDC of the TGT we have for a destination
 * realm TGT, until we get it, fail, or give up.
 */
/* Advance the process of chasing off-path TGTs. */
#[c2rust::src_loc = "651:1"]
unsafe extern "C" fn step_get_tgt_offpath(mut context: krb5_context,
                                          mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut tgt_realm: *const krb5_data = 0 as *const krb5_data;
    /* We have no fallback if the last request failed, so just give up. */
    if (*ctx).reply_code != 0 as libc::c_int { return (*ctx).reply_code }
    /* Verify that we got a TGT. */
    if !((*(*(*ctx).reply_creds).server).length == 2 as libc::c_int &&
             data_eq_string(*(*(*(*ctx).reply_creds).server).data.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize),
                            b"krbtgt\x00" as *const u8 as *const libc::c_char)
                 != 0) {
        return -(1765328237 as libc::c_long) as krb5_error_code
    }
    /* Use this tgt for the next request. */
    krb5_free_creds(context, (*ctx).cur_tgt);
    (*ctx).cur_tgt = (*ctx).reply_creds;
    (*ctx).reply_creds = 0 as *mut krb5_creds;
    /* Check if we've seen this realm before, and remember it. */
    tgt_realm =
        &mut *(*(*(*ctx).cur_tgt).server).data.offset(1 as libc::c_int as
                                                          isize) as
            *mut krb5_data;
    if seen_realm_before(context, ctx, tgt_realm) != 0 {
        return -(1765328228 as libc::c_long) as krb5_error_code
    }
    code = remember_realm(context, ctx, tgt_realm);
    if code != 0 as libc::c_int { return code }
    if data_eq(*tgt_realm, (*(*ctx).server).realm) != 0 {
        /* We received the server realm TGT we asked for. */
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Received TGT for service realm: {princ}\x00" as
                              *const u8 as *const libc::c_char,
                          (*(*ctx).cur_tgt).server);
        }
        return end_get_tgt(context, ctx)
    } else {
        let fresh1 = (*ctx).offpath_count;
        (*ctx).offpath_count = (*ctx).offpath_count.wrapping_add(1);
        if fresh1 >= 10 as libc::c_int as libc::c_uint {
            /* Time to give up. */
            return -(1765328237 as libc::c_long) as krb5_error_code
        }
    }
    return make_request_for_tgt(context, ctx, &mut (*(*ctx).server).realm);
}
/* Begin chasing off-path referrals, starting from ctx->cur_tgt. */
#[c2rust::src_loc = "691:1"]
unsafe extern "C" fn begin_get_tgt_offpath(mut context: krb5_context,
                                           mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    (*ctx).state = STATE_GET_TGT_OFFPATH;
    (*ctx).offpath_count = 1 as libc::c_int as libc::c_uint;
    return make_request_for_tgt(context, ctx, &mut (*(*ctx).server).realm);
}
/* **** STATE_GET_TGT *****/
/*
 * To obtain a foreign TGT, we first construct a path of realms R1..Rn between
 * the local realm and the target realm, using k5_client_realm_path().  Usually
 * this path is based on the domain hierarchy, but it may be altered by
 * configuration.
 *
 * We begin with cur_realm set to the local realm (R1) and next_realm set to
 * the target realm (Rn).  At each step, we check to see if we have a cached
 * TGT for next_realm; if not, we ask cur_realm to give us a TGT for
 * next_realm.  If that fails, we decrement next_realm until we get a
 * successful answer or reach cur_realm--in which case we've gotten as far as
 * we can, and have to give up.  If we do get back a TGT, it may or may not be
 * for the realm we asked for, so we search for it in the path.  The realm of
 * the TGT we get back becomes cur_realm, and next_realm is reset to the target
 * realm.  Overall, this is an O(n^2) process in the length of the path, but
 * the path length will generally be short and the process will usually end
 * much faster than the worst case.
 *
 * In some cases we may get back a TGT for a realm not in the path.  In that
 * case we enter STATE_GET_TGT_OFFPATH.
 */
/*
 * Point *tgt_out at an allocated credentials structure containing a
 * cross-realm TGT for realm retrieved from ctx->ccache.  Accept any issuing
 * realm (i.e. match only the service principal name).  If the TGT is not found
 * in the cache, return successfully but set *tgt_out to NULL.
 */
#[c2rust::src_loc = "729:1"]
unsafe extern "C" fn get_cached_tgt(mut context: krb5_context,
                                    mut ctx: krb5_tkt_creds_context,
                                    mut realm: *const krb5_data,
                                    mut tgt_out: *mut *mut krb5_creds)
 -> krb5_error_code {
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
    let mut code: krb5_error_code = 0;
    let mut tgtname: krb5_principal = 0 as krb5_principal;
    let mut flags: krb5_flags =
        0x200 as libc::c_int | 0x40 as libc::c_int | 0x1 as libc::c_int;
    let mut now: krb5_timestamp = 0;
    *tgt_out = 0 as *mut krb5_creds;
    code = krb5_timeofday(context, &mut now);
    if code != 0 as libc::c_int { return code }
    /* Construct the TGT principal name (the realm part doesn't matter). */
    code = krb5int_tgtname(context, realm, realm, &mut tgtname);
    if code != 0 as libc::c_int { return code }
    /* Construct a matching cred for the ccache query.  Look for unexpired
     * entries since there could be more than one. */
    memset(&mut mcreds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    mcreds.client = (*ctx).client;
    mcreds.server = tgtname;
    mcreds.times.endtime = now;
    /* Fetch the TGT credential. */
    (*context).use_conf_ktypes = 1 as libc::c_int as krb5_boolean;
    code = cache_get(context, (*ctx).ccache, flags, &mut mcreds, tgt_out);
    (*context).use_conf_ktypes = 0 as libc::c_int as krb5_boolean;
    krb5_free_principal(context, tgtname);
    return if code as libc::c_long == -(1765328243 as libc::c_long) ||
                  code as libc::c_long != -(1765328184 as libc::c_long) {
               0 as libc::c_int
           } else { code };
}
/* Point *tgt_out at an allocated credentials structure containing the local
 * TGT retrieved from ctx->ccache. */
#[c2rust::src_loc = "768:1"]
unsafe extern "C" fn get_cached_local_tgt(mut context: krb5_context,
                                          mut ctx: krb5_tkt_creds_context,
                                          mut tgt_out: *mut *mut krb5_creds)
 -> krb5_error_code {
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
    let mut code: krb5_error_code = 0;
    let mut tgtname: krb5_principal = 0 as krb5_principal;
    let mut flags: krb5_flags = 0x200 as libc::c_int;
    let mut now: krb5_timestamp = 0;
    let mut tgt: *mut krb5_creds = 0 as *mut krb5_creds;
    *tgt_out = 0 as *mut krb5_creds;
    code = krb5_timeofday(context, &mut now);
    if code != 0 as libc::c_int { return code }
    /* Construct the principal name. */
    code =
        krb5int_tgtname(context, &mut (*(*ctx).client).realm,
                        &mut (*(*ctx).client).realm, &mut tgtname);
    if code != 0 as libc::c_int { return code }
    /* Construct a matching cred for the ccache query. */
    memset(&mut mcreds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    mcreds.client = (*ctx).client;
    mcreds.server = tgtname;
    /* Fetch the TGT credential. */
    (*context).use_conf_ktypes = 1 as libc::c_int as krb5_boolean;
    code = cache_get(context, (*ctx).ccache, flags, &mut mcreds, &mut tgt);
    (*context).use_conf_ktypes = 0 as libc::c_int as krb5_boolean;
    krb5_free_principal(context, tgtname);
    if code != 0 { return code }
    /* Check if the TGT is expired before bothering the KDC with it. */
    if ts_after(now, (*tgt).times.endtime) != 0 {
        krb5_free_creds(context, tgt);
        return -(1765328352 as libc::c_long) as krb5_error_code
    }
    *tgt_out = tgt;
    return 0 as libc::c_int;
}
/* Initialize the realm path fields for getting a TGT for
 * ctx->server->realm. */
#[c2rust::src_loc = "816:1"]
unsafe extern "C" fn init_realm_path(mut context: krb5_context,
                                     mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut realm_path: *mut krb5_data = 0 as *mut krb5_data;
    let mut nrealms: size_t = 0;
    /* Get the client realm path and count its length. */
    code =
        k5_client_realm_path(context, &mut (*(*ctx).client).realm,
                             &mut (*(*ctx).server).realm, &mut realm_path);
    if code != 0 as libc::c_int { return code }
    nrealms = 0 as libc::c_int as size_t;
    while !(*realm_path.offset(nrealms as isize)).data.is_null() {
        nrealms = nrealms.wrapping_add(1)
    }
    if nrealms > 1 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"nrealms > 1\x00" as *const u8 as *const libc::c_char,
                      b"get_creds.c\x00" as *const u8 as *const libc::c_char,
                      829 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"krb5_error_code init_realm_path(krb5_context, krb5_tkt_creds_context)\x00")).as_ptr());
    }
    /* Initialize the realm path fields in ctx. */
    krb5int_free_data_list(context, (*ctx).realm_path);
    (*ctx).realm_path = realm_path;
    (*ctx).last_realm =
        realm_path.offset(nrealms as
                              isize).offset(-(1 as libc::c_int as isize));
    (*ctx).cur_realm = realm_path;
    (*ctx).next_realm = (*ctx).last_realm;
    return 0 as libc::c_int;
}
/* Find realm within the portion of ctx->realm_path following
 * ctx->cur_realm.  Return NULL if it is not found. */
#[c2rust::src_loc = "842:1"]
unsafe extern "C" fn find_realm_in_path(mut context: krb5_context,
                                        mut ctx: krb5_tkt_creds_context,
                                        mut realm: *const krb5_data)
 -> *const krb5_data {
    let mut r: *const krb5_data = 0 as *const krb5_data;
    r = (*ctx).cur_realm.offset(1 as libc::c_int as isize);
    while !(*r).data.is_null() {
        if data_eq(*r, *realm) != 0 { return r }
        r = r.offset(1)
    }
    return 0 as *const krb5_data;
}
/*
 * Generate the next request in the path traversal.  If a cached TGT for the
 * target realm appeared in the ccache since we started the TGT acquisition
 * process, this function may invoke end_get_tgt().
 */
#[c2rust::src_loc = "860:1"]
unsafe extern "C" fn get_tgt_request(mut context: krb5_context,
                                     mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut cached_tgt: *mut krb5_creds = 0 as *mut krb5_creds;
    loop  {
        /* Check if we have a cached TGT for the target realm. */
        code =
            get_cached_tgt(context, ctx, (*ctx).next_realm, &mut cached_tgt);
        if code != 0 as libc::c_int { return code }
        if !cached_tgt.is_null() {
            /* Advance the current realm and keep going. */
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Found cached TGT for intermediate realm: {creds}\x00"
                                  as *const u8 as *const libc::c_char,
                              cached_tgt);
            }
            krb5_free_creds(context, (*ctx).cur_tgt);
            (*ctx).cur_tgt = cached_tgt;
            if (*ctx).next_realm == (*ctx).last_realm {
                return end_get_tgt(context, ctx)
            }
            (*ctx).cur_realm = (*ctx).next_realm;
            (*ctx).next_realm = (*ctx).last_realm
        } else {
            return make_request_for_tgt(context, ctx, (*ctx).next_realm)
        }
    };
}
/* Process a TGS reply and advance the path traversal to get a foreign TGT. */
#[c2rust::src_loc = "888:1"]
unsafe extern "C" fn step_get_tgt(mut context: krb5_context,
                                  mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut tgt_realm: *const krb5_data = 0 as *const krb5_data;
    let mut path_realm: *const krb5_data = 0 as *const krb5_data;
    if (*ctx).reply_code != 0 as libc::c_int {
        /* The last request failed.  Try the next-closest realm to
         * ctx->cur_realm. */
        (*ctx).next_realm = (*ctx).next_realm.offset(-1);
        if (*ctx).next_realm == (*ctx).cur_realm {
            /* We've tried all the realms we could and couldn't progress beyond
             * ctx->cur_realm, so it's time to give up. */
            return (*ctx).reply_code
        }
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Trying next closer realm in path: {data}\x00" as
                              *const u8 as *const libc::c_char,
                          (*ctx).next_realm);
        }
    } else {
        /* Verify that we got a TGT. */
        if !((*(*(*ctx).reply_creds).server).length == 2 as libc::c_int &&
                 data_eq_string(*(*(*(*ctx).reply_creds).server).data.offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize),
                                b"krbtgt\x00" as *const u8 as
                                    *const libc::c_char) != 0) {
            return -(1765328237 as libc::c_long) as krb5_error_code
        }
        /* Use this tgt for the next request regardless of what it is. */
        krb5_free_creds(context, (*ctx).cur_tgt);
        (*ctx).cur_tgt = (*ctx).reply_creds;
        (*ctx).reply_creds = 0 as *mut krb5_creds;
        /* Remember that we saw this realm. */
        tgt_realm =
            &mut *(*(*(*ctx).cur_tgt).server).data.offset(1 as libc::c_int as
                                                              isize) as
                *mut krb5_data;
        code = remember_realm(context, ctx, tgt_realm);
        if code != 0 as libc::c_int { return code }
        /* See where we wound up on the path (or off it). */
        path_realm = find_realm_in_path(context, ctx, tgt_realm);
        if !path_realm.is_null() {
            /* Only cache the TGT if we asked for it, to avoid duplicates. */
            if path_realm == (*ctx).next_realm {
                krb5_cc_store_cred(context, (*ctx).ccache, (*ctx).cur_tgt);
            }
            if path_realm == (*ctx).last_realm {
                /* We received a TGT for the target realm. */
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"Received TGT for service realm: {princ}\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*(*ctx).cur_tgt).server);
                }
                return end_get_tgt(context, ctx)
            } else {
                if !path_realm.is_null() {
                    /* We still have further to go; advance the traversal. */
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"Received TGT for {data}; advancing current realm\x00"
                                          as *const u8 as *const libc::c_char,
                                      tgt_realm);
                    }
                    (*ctx).cur_realm = path_realm;
                    (*ctx).next_realm = (*ctx).last_realm
                }
            }
        } else if data_eq(*tgt_realm, (*(*ctx).client).realm) != 0 {
            /* We were referred back to the local realm, which is bad. */
            return -(1765328237 as libc::c_long) as krb5_error_code
        } else {
            /* We went off the path; start the off-path chase. */
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Received TGT for offpath realm {data}\x00" as
                                  *const u8 as *const libc::c_char,
                              tgt_realm);
            }
            return begin_get_tgt_offpath(context, ctx)
        }
    }
    /* Generate the next request in the path traversal. */
    return get_tgt_request(context, ctx);
}
/* Caller's flags parameter */
/* Convert ticket flags to necessary KDC options */
/*
 * Begin the process of getting a foreign TGT, either for the explicitly
 * specified server realm or for the fallback realm.  Expects that
 * ctx->server->realm is the realm of the desired TGT, and that
 * ctx->getting_tgt_for is the state we should advance to after we have the
 * desired TGT.
 */
#[c2rust::src_loc = "957:1"]
unsafe extern "C" fn begin_get_tgt(mut context: krb5_context,
                                   mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut cached_tgt: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut is_local_service: krb5_boolean = 0;
    (*ctx).state = STATE_GET_TGT;
    is_local_service =
        data_eq((*(*ctx).client).realm, (*(*ctx).server).realm) as
            krb5_boolean;
    if is_local_service == 0 {
        /* See if we have a cached TGT for the server realm. */
        code =
            get_cached_tgt(context, ctx, &mut (*(*ctx).server).realm,
                           &mut cached_tgt);
        if code != 0 as libc::c_int { return code }
        if !cached_tgt.is_null() {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Found cached TGT for service realm: {creds}\x00"
                                  as *const u8 as *const libc::c_char,
                              cached_tgt);
            }
            krb5_free_creds(context, (*ctx).cur_tgt);
            (*ctx).cur_tgt = cached_tgt;
            return end_get_tgt(context, ctx)
        }
    }
    /* Start with the local tgt. */
    krb5_free_creds(context, (*ctx).cur_tgt);
    (*ctx).cur_tgt = 0 as *mut krb5_creds;
    code = get_cached_local_tgt(context, ctx, &mut (*ctx).cur_tgt);
    if code != 0 as libc::c_int { return code }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Starting with TGT for client realm: {creds}\x00" as
                          *const u8 as *const libc::c_char, (*ctx).cur_tgt);
    }
    if is_local_service != 0 { return end_get_tgt(context, ctx) }
    /* Initialize the realm path. */
    code = init_realm_path(context, ctx);
    if code != 0 as libc::c_int { return code }
    /* Empty out the realms-seen list for loop checking. */
    krb5int_free_data_list(context, (*ctx).realms_seen);
    (*ctx).realms_seen = 0 as *mut krb5_data;
    /* Generate the first request. */
    return get_tgt_request(context, ctx);
}
/* **** STATE_BEGIN *****/
/*
 * Look for the desired credentials in the cache, if possible.  If we find
 * them, put them in ctx->reply_creds and advance the state to STATE_COMPLETE.
 * Return successfully even if creds are not found, unless the caller only
 * wanted cached creds.
 */
#[c2rust::src_loc = "1012:1"]
unsafe extern "C" fn check_cache(mut context: krb5_context,
                                 mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
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
    let mut fields: krb5_flags = 0;
    /* Perform the cache lookup. */
    code =
        krb5int_construct_matching_creds(context, (*ctx).req_options,
                                         (*ctx).in_creds, &mut mcreds,
                                         &mut fields);
    if code != 0 { return code }
    code =
        cache_get(context, (*ctx).ccache, fields, &mut mcreds,
                  &mut (*ctx).reply_creds);
    if code == 0 as libc::c_int {
        (*ctx).state = STATE_COMPLETE;
        return 0 as libc::c_int
    }
    /* Stop on unexpected cache errors. */
    if code as libc::c_long != -(1765328243 as libc::c_long) &&
           code as libc::c_long != -(1765328184 as libc::c_long) {
        return code
    }
    /* Stop if the caller only wanted cached creds. */
    if (*ctx).req_options & 2 as libc::c_int != 0 { return code }
    return 0 as libc::c_int;
}
/* Decide where to begin the acquisition process. */
#[c2rust::src_loc = "1042:1"]
unsafe extern "C" fn begin(mut context: krb5_context,
                           mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    code = check_cache(context, ctx);
    if code != 0 as libc::c_int ||
           (*ctx).state as libc::c_uint ==
               STATE_COMPLETE as libc::c_int as libc::c_uint {
        return code
    }
    /* If the server realm is unspecified, start with the client realm. */
    if krb5_is_referral_realm(&mut (*(*ctx).server).realm) != 0 {
        krb5_free_data_contents(context, &mut (*(*ctx).server).realm);
        code =
            krb5int_copy_data_contents(context, &mut (*(*ctx).client).realm,
                                       &mut (*(*ctx).server).realm);
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Server has referral realm; starting with {princ}\x00"
                              as *const u8 as *const libc::c_char,
                          (*ctx).server);
        }
        if code != 0 as libc::c_int { return code }
    }
    /* Obtain a TGT for the service realm. */
    (*ctx).getting_tgt_for = STATE_REFERRALS;
    return begin_get_tgt(context, ctx);
}
/* **** API functions *****/
#[no_mangle]
#[c2rust::src_loc = "1068:1"]
pub unsafe extern "C" fn krb5_tkt_creds_init(mut context: krb5_context,
                                             mut ccache: krb5_ccache,
                                             mut in_creds: *mut krb5_creds,
                                             mut options: krb5_flags,
                                             mut pctx:
                                                 *mut krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut ctx: krb5_tkt_creds_context = 0 as krb5_tkt_creds_context;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Getting credentials {creds} using ccache {ccache}\x00"
                          as *const u8 as *const libc::c_char, in_creds,
                      ccache);
    }
    ctx =
        k5alloc(::std::mem::size_of::<_krb5_tkt_creds_context>() as
                    libc::c_ulong, &mut code) as krb5_tkt_creds_context;
    if !ctx.is_null() {
        (*ctx).req_options = options;
        (*ctx).req_kdcopt = 0 as libc::c_int;
        if options & 4 as libc::c_int != 0 {
            (*ctx).req_kdcopt |= 0x10000 as libc::c_int
        }
        if options & 16 as libc::c_int != 0 {
            (*ctx).req_kdcopt |= 0x40000000 as libc::c_int
        }
        if options & 32 as libc::c_int != 0 {
            (*ctx).req_kdcopt |= 0x20 as libc::c_int
        }
        (*ctx).state = STATE_BEGIN;
        code = krb5_copy_creds(context, in_creds, &mut (*ctx).in_creds);
        if !(code != 0 as libc::c_int) {
            (*ctx).client = (*(*ctx).in_creds).client;
            (*ctx).server = (*(*ctx).in_creds).server;
            code =
                krb5_copy_principal(context,
                                    (*ctx).server as krb5_const_principal,
                                    &mut (*ctx).req_server);
            if !(code != 0 as libc::c_int) {
                code = krb5_cc_dup(context, ccache, &mut (*ctx).ccache);
                if !(code != 0 as libc::c_int) {
                    code =
                        krb5_copy_authdata(context, (*in_creds).authdata,
                                           &mut (*ctx).authdata);
                    if !(code != 0 as libc::c_int) {
                        *pctx = ctx;
                        ctx = 0 as krb5_tkt_creds_context
                    }
                }
            }
        }
    }
    krb5_tkt_creds_free(context, ctx);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1115:1"]
pub unsafe extern "C" fn krb5_tkt_creds_get_creds(mut context: krb5_context,
                                                  mut ctx:
                                                      krb5_tkt_creds_context,
                                                  mut creds: *mut krb5_creds)
 -> krb5_error_code {
    if (*ctx).state as libc::c_uint !=
           STATE_COMPLETE as libc::c_int as libc::c_uint {
        return -(1765328241 as libc::c_long) as krb5_error_code
    }
    return k5_copy_creds_contents(context, (*ctx).reply_creds, creds);
}
#[no_mangle]
#[c2rust::src_loc = "1124:1"]
pub unsafe extern "C" fn krb5_tkt_creds_get_times(mut context: krb5_context,
                                                  mut ctx:
                                                      krb5_tkt_creds_context,
                                                  mut times:
                                                      *mut krb5_ticket_times)
 -> krb5_error_code {
    if (*ctx).state as libc::c_uint !=
           STATE_COMPLETE as libc::c_int as libc::c_uint {
        return -(1765328241 as libc::c_long) as krb5_error_code
    }
    *times = (*(*ctx).reply_creds).times;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1134:1"]
pub unsafe extern "C" fn krb5_tkt_creds_free(mut context: krb5_context,
                                             mut ctx:
                                                 krb5_tkt_creds_context) {
    if ctx.is_null() { return }
    krb5int_fast_free_state(context, (*ctx).fast_state);
    krb5_free_creds(context, (*ctx).in_creds);
    krb5_cc_close(context, (*ctx).ccache);
    krb5_free_principal(context, (*ctx).req_server);
    krb5_free_authdata(context, (*ctx).authdata);
    krb5_free_creds(context, (*ctx).cur_tgt);
    krb5int_free_data_list(context, (*ctx).realms_seen);
    krb5_free_principal(context, (*ctx).tgt_princ);
    krb5_free_keyblock(context, (*ctx).subkey);
    krb5_free_data_contents(context, &mut (*ctx).previous_request);
    krb5int_free_data_list(context, (*ctx).realm_path);
    krb5_free_creds(context, (*ctx).reply_creds);
    free(ctx as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1154:1"]
pub unsafe extern "C" fn krb5_tkt_creds_get(mut context: krb5_context,
                                            mut ctx: krb5_tkt_creds_context)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut request: krb5_data = empty_data();
    let mut reply: krb5_data = empty_data();
    let mut realm: krb5_data = empty_data();
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tcp_only: libc::c_int = 0 as libc::c_int;
    let mut use_master: libc::c_int = 0;
    loop  {
        /* Get the next request and realm.  Turn on TCP if necessary. */
        code =
            krb5_tkt_creds_step(context, ctx, &mut reply, &mut request,
                                &mut realm, &mut flags);
        if code as libc::c_long == -(1765328332 as libc::c_long) &&
               tcp_only == 0 {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Request or response is too big for UDP; retrying with TCP\x00"
                                  as *const u8 as *const libc::c_char);
            }
            tcp_only = 1 as libc::c_int
        } else if code != 0 as libc::c_int ||
                      flags & 0x1 as libc::c_int as libc::c_uint == 0 {
            break ;
        }
        krb5_free_data_contents(context, &mut reply);
        /* Send it to a KDC for the appropriate realm. */
        use_master = 0 as libc::c_int;
        code =
            krb5_sendto_kdc(context, &mut request, &mut realm, &mut reply,
                            &mut use_master, tcp_only);
        if code != 0 as libc::c_int { break ; }
        krb5_free_data_contents(context, &mut request);
        krb5_free_data_contents(context, &mut realm);
    }
    krb5_free_data_contents(context, &mut request);
    krb5_free_data_contents(context, &mut reply);
    krb5_free_data_contents(context, &mut realm);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1191:1"]
pub unsafe extern "C" fn krb5_tkt_creds_step(mut context: krb5_context,
                                             mut ctx: krb5_tkt_creds_context,
                                             mut in_0: *mut krb5_data,
                                             mut out: *mut krb5_data,
                                             mut realm: *mut krb5_data,
                                             mut flags: *mut libc::c_uint)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut no_input: krb5_boolean =
        (in_0.is_null() || (*in_0).length == 0 as libc::c_int as libc::c_uint)
            as libc::c_int as krb5_boolean;
    *out = empty_data();
    *realm = empty_data();
    *flags = 0 as libc::c_int as libc::c_uint;
    /* We should receive an empty input on the first step only, and should not
     * get called after completion. */
    if no_input !=
           ((*ctx).state as libc::c_uint ==
                STATE_BEGIN as libc::c_int as libc::c_uint) as libc::c_int as
               libc::c_uint ||
           (*ctx).state as libc::c_uint ==
               STATE_COMPLETE as libc::c_int as libc::c_uint {
        return 22 as libc::c_int
    }
    (*ctx).caller_out = out;
    (*ctx).caller_realm = realm;
    (*ctx).caller_flags = flags;
    if no_input == 0 {
        /* Convert the input token into a credential and store it in ctx. */
        code = get_creds_from_tgs_reply(context, ctx, in_0);
        if code != 0 as libc::c_int { return code }
    }
    if (*ctx).state as libc::c_uint ==
           STATE_BEGIN as libc::c_int as libc::c_uint {
        return begin(context, ctx)
    } else if (*ctx).state as libc::c_uint ==
                  STATE_GET_TGT as libc::c_int as libc::c_uint {
        return step_get_tgt(context, ctx)
    } else if (*ctx).state as libc::c_uint ==
                  STATE_GET_TGT_OFFPATH as libc::c_int as libc::c_uint {
        return step_get_tgt_offpath(context, ctx)
    } else if (*ctx).state as libc::c_uint ==
                  STATE_REFERRALS as libc::c_int as libc::c_uint {
        return step_referrals(context, ctx)
    } else if (*ctx).state as libc::c_uint ==
                  STATE_NON_REFERRAL as libc::c_int as libc::c_uint {
        return step_non_referral(context, ctx)
    } else { return 22 as libc::c_int };
}
#[c2rust::src_loc = "1234:1"]
unsafe extern "C" fn try_get_creds(mut context: krb5_context,
                                   mut options: krb5_flags,
                                   mut ccache: krb5_ccache,
                                   mut in_creds: *mut krb5_creds,
                                   mut creds_out: *mut krb5_creds)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut ctx: krb5_tkt_creds_context = 0 as krb5_tkt_creds_context;
    code = krb5_tkt_creds_init(context, ccache, in_creds, options, &mut ctx);
    if !(code != 0) {
        code = krb5_tkt_creds_get(context, ctx);
        if !(code != 0) {
            code = krb5_tkt_creds_get_creds(context, ctx, creds_out)
        }
    }
    krb5_tkt_creds_free(context, ctx);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1254:1"]
pub unsafe extern "C" fn krb5_get_credentials(mut context: krb5_context,
                                              mut options: krb5_flags,
                                              mut ccache: krb5_ccache,
                                              mut in_creds: *mut krb5_creds,
                                              mut out_creds:
                                                  *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut ncreds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut canon_creds: krb5_creds =
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
    let mut store_creds: krb5_creds =
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
    let mut canon_server: krb5_principal_data =
        krb5_principal_data{magic: 0,
                            realm:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            data: 0 as *mut krb5_data,
                            length: 0,
                            type_0: 0,};
    let mut canon_components: [krb5_data; 2] =
        [krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,}; 2];
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon_hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    *out_creds = 0 as *mut krb5_creds;
    /* If S4U2Proxy is requested, use the synchronous implementation in
     * s4u_creds.c. */
    if options & 64 as libc::c_int != 0 {
        return k5_get_proxy_cred_from_kdc(context, options, ccache, in_creds,
                                          out_creds)
    }
    /* Allocate a container. */
    ncreds =
        k5alloc(::std::mem::size_of::<krb5_creds>() as libc::c_ulong,
                &mut code) as *mut krb5_creds;
    if !ncreds.is_null() {
        code = try_get_creds(context, options, ccache, in_creds, ncreds);
        if code == 0 { *out_creds = ncreds; return 0 as libc::c_int }
        /* Possibly try again with the canonicalized hostname, if the server is
     * host-based and we are configured for fallback canonicalization. */
        if !(code as libc::c_long != -(1765328377 as libc::c_long)) {
            if !((*context).dns_canonicalize_hostname as libc::c_uint !=
                     CANONHOST_FALLBACK as libc::c_int as libc::c_uint) {
                if !((*(*in_creds).server).type_0 != 3 as libc::c_int ||
                         (*(*in_creds).server).length != 2 as libc::c_int) {
                    hostname =
                        k5memdup0((*(*(*in_creds).server).data.offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).data
                                      as *const libc::c_void,
                                  (*(*(*in_creds).server).data.offset(1 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)).length
                                      as size_t, &mut code) as
                            *mut libc::c_char;
                    if !hostname.is_null() {
                        code =
                            k5_expand_hostname(context, hostname,
                                               1 as libc::c_int as
                                                   krb5_boolean,
                                               &mut canon_hostname);
                        if !(code != 0) {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"Falling back to canonicalized server hostname {str}\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              canon_hostname);
                            }
                            /* Make shallow copies of in_creds and its server to alter the hostname. */
                            canon_components[0 as libc::c_int as usize] =
                                *(*(*in_creds).server).data.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize);
                            canon_components[1 as libc::c_int as usize] =
                                string2data(canon_hostname);
                            canon_server = *(*in_creds).server;
                            canon_server.data = canon_components.as_mut_ptr();
                            canon_creds = *in_creds;
                            canon_creds.server = &mut canon_server;
                            code =
                                try_get_creds(context,
                                              options | 8 as libc::c_int,
                                              ccache, &mut canon_creds,
                                              ncreds);
                            if !(code != 0) {
                                if options & 8 as libc::c_int == 0 {
                                    /* Store the creds under the originally requested server name.  The
         * ccache layer will also store them under the ticket server name. */
                                    store_creds = *ncreds;
                                    store_creds.server = (*in_creds).server;
                                    krb5_cc_store_cred(context, ccache,
                                                       &mut store_creds);
                                }
                                *out_creds = ncreds;
                                ncreds = 0 as *mut krb5_creds
                            }
                        }
                    }
                }
            }
        }
    }
    free(hostname as *mut libc::c_void);
    free(canon_hostname as *mut libc::c_void);
    krb5_free_creds(context, ncreds);
    return code;
}
