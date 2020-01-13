use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:54"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:54"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:54"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:54"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
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
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
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
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
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
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
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
    #[c2rust::src_loc = "1993:16"]
    pub struct _krb5_authenticator {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub checksum: *mut krb5_checksum,
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "1993:1"]
    pub type krb5_authenticator = _krb5_authenticator;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    #[c2rust::src_loc = "2031:1"]
    pub type krb5_last_req_entry = _krb5_last_req_entry;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2079:16"]
    pub struct _krb5_enc_kdc_rep_part {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub session: *mut krb5_keyblock,
        pub last_req: *mut *mut krb5_last_req_entry,
        pub nonce: krb5_int32,
        pub key_exp: krb5_timestamp,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub server: krb5_principal,
        pub caddrs: *mut *mut krb5_address,
        pub enc_padata: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "2079:1"]
    pub type krb5_enc_kdc_rep_part = _krb5_enc_kdc_rep_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2095:16"]
    pub struct _krb5_kdc_rep {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub client: krb5_principal,
        pub ticket: *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_kdc_rep_part,
    }
    #[c2rust::src_loc = "2095:1"]
    pub type krb5_kdc_rep = _krb5_kdc_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2107:16"]
    pub struct _krb5_error {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub susec: krb5_int32,
        pub stime: krb5_timestamp,
        pub error: krb5_ui_4,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub text: krb5_data,
        pub e_data: krb5_data,
    }
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
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
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[no_mangle]
        #[c2rust::src_loc = "309:1"]
        pub fn krb5_anonymous_principal() -> krb5_const_principal;
        #[no_mangle]
        #[c2rust::src_loc = "882:1"]
        pub fn krb5_c_enctype_compare(context: krb5_context, e1: krb5_enctype,
                                      e2: krb5_enctype,
                                      similar: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "911:1"]
        pub fn krb5_c_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype,
                                    key: *const krb5_keyblock,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_c_verify_checksum(context: krb5_context,
                                      key: *const krb5_keyblock,
                                      usage: krb5_keyusage,
                                      data: *const krb5_data,
                                      cksum: *const krb5_checksum,
                                      valid: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "1059:1"]
        pub fn krb5_c_valid_cksumtype(ctype: krb5_cksumtype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "1070:1"]
        pub fn krb5_c_is_coll_proof_cksum(ctype: krb5_cksumtype)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "1080:1"]
        pub fn krb5_c_is_keyed_cksum(ctype: krb5_cksumtype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "3995:1"]
        pub fn krb5_build_principal_ext(context: krb5_context,
                                        princ: *mut krb5_principal,
                                        rlen: libc::c_uint,
                                        realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3885:1"]
        pub fn krb5_find_authdata(context: krb5_context,
                                  ticket_authdata: *const *mut krb5_authdata,
                                  ap_req_authdata: *const *mut krb5_authdata,
                                  ad_type: krb5_authdatatype,
                                  results: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3711:1"]
        pub fn krb5_principal_compare_flags(context: krb5_context,
                                            princ1: krb5_const_principal,
                                            princ2: krb5_const_principal,
                                            flags: libc::c_int)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "3682:1"]
        pub fn krb5_principal_compare_any_realm(context: krb5_context,
                                                princ1: krb5_const_principal,
                                                princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "3650:1"]
        pub fn krb5_realm_compare(context: krb5_context,
                                  princ1: krb5_const_principal,
                                  princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3292:1"]
        pub fn krb5_mk_error(context: krb5_context,
                             dec_err: *const krb5_error,
                             enc_err: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6035:1"]
        pub fn krb5_auth_con_getauthenticator(context: krb5_context,
                                              auth_context: krb5_auth_context,
                                              authenticator:
                                                  *mut *mut krb5_authenticator)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5847:1"]
        pub fn krb5_auth_con_getrecvsubkey(ctx: krb5_context,
                                           ac: krb5_auth_context,
                                           keyblock: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5765:1"]
        pub fn krb5_auth_con_setuseruserkey(context: krb5_context,
                                            auth_context: krb5_auth_context,
                                            keyblock: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5718:1"]
        pub fn krb5_auth_con_setaddrs(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      local_addr: *mut krb5_address,
                                      remote_addr: *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5644:1"]
        pub fn krb5_auth_con_setflags(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      flags: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5613:1"]
        pub fn krb5_auth_con_init(context: krb5_context,
                                  auth_context: *mut krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
        #[no_mangle]
        #[c2rust::src_loc = "4633:1"]
        pub fn krb5_free_authdata(context: krb5_context,
                                  val: *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "4607:1"]
        pub fn krb5_free_authenticator(context: krb5_context,
                                       val: *mut krb5_authenticator);
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:54"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "564:16"]
    pub struct _krb5_pa_pac_options {
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "564:1"]
    pub type krb5_pa_pac_options = _krb5_pa_pac_options;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "751:16"]
    pub struct _krb5_pa_for_user {
        pub user: krb5_principal,
        pub cksum: krb5_checksum,
        pub auth_package: krb5_data,
    }
    #[c2rust::src_loc = "751:1"]
    pub type krb5_pa_for_user = _krb5_pa_for_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    #[inline]
    #[c2rust::src_loc = "2354:1"]
    pub unsafe extern "C" fn ts_incr(mut ts: krb5_timestamp,
                                     mut delta: krb5_deltat)
     -> krb5_timestamp {
        return (ts as uint32_t).wrapping_add(delta as uint32_t) as
                   krb5_timestamp;
    }
    #[inline]
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_principal,
                        krb5_checksum, krb5_data, krb5_timestamp,
                        krb5_auth_context, krb5_ap_req, krb5_principal_data,
                        krb5_ticket, krb5_pa_data, krb5_kdc_req,
                        krb5_preauthtype};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stdint_uintn_h::uint32_t;
    use super::string_h::{strlen, memcmp};
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "2400:1"]
        pub fn k5_etypes_contains(list: *const krb5_enctype,
                                  etype: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "2395:1"]
        pub fn krb5int_parse_enctype_list(context: krb5_context,
                                          profkey: *const libc::c_char,
                                          profstr: *mut libc::c_char,
                                          default_list: *mut krb5_enctype,
                                          result: *mut *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2172:1"]
        pub fn krb5_rd_req_decoded_anyflag(_: krb5_context,
                                           _: *mut krb5_auth_context,
                                           _: *const krb5_ap_req,
                                           _: krb5_const_principal,
                                           _: krb5_keytab, _: *mut krb5_flags,
                                           _: *mut *mut krb5_ticket)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2155:1"]
        pub fn krb5_free_ap_req(_: krb5_context, _: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "2135:1"]
        pub fn krb5_check_transited_list(_: krb5_context,
                                         trans: *const krb5_data,
                                         realm1: *const krb5_data,
                                         realm2: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2096:1"]
        pub fn krb5int_c_deprecated_enctype(_: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "2093:1"]
        pub fn krb5_is_permitted_enctype(_: krb5_context, _: krb5_enctype)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "1732:1"]
        pub fn decode_krb5_pa_pac_options(_: *const krb5_data,
                                          _: *mut *mut krb5_pa_pac_options)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1675:1"]
        pub fn decode_krb5_pa_s4u_x509_user(_: *const krb5_data,
                                            _:
                                                *mut *mut krb5_pa_s4u_x509_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1672:1"]
        pub fn decode_krb5_pa_for_user(_: *const krb5_data,
                                       _: *mut *mut krb5_pa_for_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1605:1"]
        pub fn decode_krb5_ap_req(output: *const krb5_data,
                                  rep: *mut *mut krb5_ap_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1547:1"]
        pub fn encode_krb5_pa_pac_options(_: *const krb5_pa_pac_options,
                                          _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1487:1"]
        pub fn encode_krb5_pa_s4u_x509_user(_: *const krb5_pa_s4u_x509_user,
                                            _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1484:1"]
        pub fn encode_krb5_s4u_userid(_: *const krb5_s4u_userid,
                                      _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1433:1"]
        pub fn encode_krb5_checksum(_: *const krb5_checksum,
                                    _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1411:1"]
        pub fn encode_krb5_kdc_req_body(rep: *const krb5_kdc_req,
                                        code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "871:1"]
        pub fn k5_alloc_pa_data(pa_type: krb5_preauthtype, len: size_t,
                                out: *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "875:1"]
        pub fn k5_free_pa_data_element(pa: *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "880:1"]
        pub fn k5_add_pa_data_element(list: *mut *mut *mut krb5_pa_data,
                                      pa: *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "885:1"]
        pub fn k5_add_pa_data_from_data(list: *mut *mut *mut krb5_pa_data,
                                        pa_type: krb5_preauthtype,
                                        data: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "890:1"]
        pub fn k5_add_empty_pa_data(list: *mut *mut *mut krb5_pa_data,
                                    pa_type: krb5_preauthtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "933:1"]
        pub fn krb5_free_pa_for_user(_: krb5_context,
                                     _: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_free_pa_s4u_x509_user(_: krb5_context,
                                          _: *mut krb5_pa_s4u_x509_user);
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:54"]
pub mod k5_buf_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a C string to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const libc::c_char);
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:55"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_magic,
                        krb5_ui_4, krb5_flags, krb5_deltat, krb5_timestamp,
                        krb5_kvno, krb5_principal, krb5_data, krb5_context,
                        krb5_principal_data, krb5_const_principal,
                        krb5_error_code, krb5_kdc_req, krb5_pa_data,
                        krb5_int32, krb5_keyblock, krb5_authdata};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn krb5_db_get_principal(kcontext: krb5_context,
                                     search_for: krb5_const_principal,
                                     flags: libc::c_uint,
                                     entry: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn krb5_db_check_transited_realms(kcontext: krb5_context,
                                              tr_contents: *const krb5_data,
                                              client_realm: *const krb5_data,
                                              server_realm: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "683:1"]
        pub fn krb5_db_check_policy_as(kcontext: krb5_context,
                                       request: *mut krb5_kdc_req,
                                       client: *mut krb5_db_entry,
                                       server: *mut krb5_db_entry,
                                       kdc_time: krb5_timestamp,
                                       status: *mut *const libc::c_char,
                                       e_data: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5_dbe_find_enctype(kcontext: krb5_context,
                                     dbentp: *mut krb5_db_entry,
                                     ktype: krb5_int32, stype: krb5_int32,
                                     kvno: krb5_int32,
                                     kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "704:1"]
        pub fn krb5_db_refresh_config(kcontext: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "706:1"]
        pub fn krb5_db_check_allowed_to_delegate(kcontext: krb5_context,
                                                 client: krb5_const_principal,
                                                 server: *const krb5_db_entry,
                                                 proxy: krb5_const_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "582:1"]
        pub fn krb5_dbe_get_string(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   key: *const libc::c_char,
                                   value_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "711:1"]
        pub fn krb5_db_get_s4u_x509_principal(kcontext: krb5_context,
                                              client_cert: *const krb5_data,
                                              in_princ: krb5_const_principal,
                                              flags: libc::c_uint,
                                              entry: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "717:1"]
        pub fn krb5_db_allowed_to_delegate_from(context: krb5_context,
                                                client: krb5_const_principal,
                                                server: krb5_const_principal,
                                                server_ad_info:
                                                    *mut libc::c_void,
                                                proxy: *const krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "723:1"]
        pub fn krb5_db_get_authdata_info(context: krb5_context,
                                         flags: libc::c_uint,
                                         in_authdata: *mut *mut krb5_authdata,
                                         client_princ: krb5_const_principal,
                                         server_princ: krb5_const_principal,
                                         server_key: *mut krb5_keyblock,
                                         krbtgt_key: *mut krb5_keyblock,
                                         krbtgt: *mut krb5_db_entry,
                                         authtime: krb5_timestamp,
                                         ad_info_out: *mut *mut libc::c_void,
                                         client_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "866:1"]
        pub fn krb5_dbe_free_string(_: krb5_context, _: *mut libc::c_char);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/realm_data.h:55"]
pub mod realm_data_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/realm_data.h */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    #[c2rust::src_loc = "36:1"]
    pub type kdc_realm_t = __kdc_realm_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct __kdc_realm_data {
        pub realm_name: *mut libc::c_char,
        pub realm_context: krb5_context,
        pub realm_keytab: krb5_keytab,
        pub realm_hostbased: *mut libc::c_char,
        pub realm_no_referral: *mut libc::c_char,
        pub realm_stash: *mut libc::c_char,
        pub realm_mpname: *mut libc::c_char,
        pub realm_mprinc: krb5_principal,
        pub realm_mkey: krb5_keyblock,
        pub realm_tgsprinc: krb5_principal,
        pub realm_listen: *mut libc::c_char,
        pub realm_tcp_listen: *mut libc::c_char,
        pub realm_maxlife: krb5_deltat,
        pub realm_maxrlife: krb5_deltat,
        pub realm_reject_bad_transit: krb5_boolean,
        pub realm_restrict_anon: krb5_boolean,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:8"]
    pub struct server_handle {
        pub kdc_realmlist: *mut *mut kdc_realm_t,
        pub kdc_numrealms: libc::c_int,
        pub kdc_err_context: krb5_context,
    }
    use super::krb5_h::{krb5_context, krb5_keytab, krb5_principal,
                        krb5_keyblock, krb5_deltat, krb5_boolean};
    /*
     * General Kerberos per-realm data.
     */
    /* Realm name                       */
    /* XXX the real context should go away once the db_context is done.
 * The db_context is then associated with the realm keytab using
 * krb5_ktkdb_resolv(). There should be nothing in the context which
 * cannot span multiple realms -- proven */
    /* Context to be used for realm     */
    /* keytab to be used for this realm */
    /* referral services for NT-UNKNOWN */
    /* non-referral services         */
    /*
     * Database per-realm data.
     */
    /* Stash file name for realm        */
    /* Master principal name for realm  */
    /* Master principal for realm       */
    /*
     * Note realm_mkey is mkey read from stash or keyboard and may not be the
     * latest.
     */
    /* Master key for this realm        */
    /*
     * TGS per-realm data.
     */
    /* TGS principal for this realm     */
    /*
     * Other per-realm data.
     */
    /* Per-realm KDC UDP listen */
    /* Per-realm KDC TCP listen */
    /*
     * Per-realm parameters.
     */
    /* Maximum ticket life for realm    */
    /* Maximum renewable life for realm */
    /* Accept unverifiable transited_realm ? */
    /* Anon to local TGT only */
    /* REALM_DATA_H */
    /*
 * These macros used to refer to a global pointer to the active realm state
 * structure for a request.  They now refer to a local variable that must be
 * properly declared in each function that uses these macros.
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/net-server.h:55"]
pub mod net_server_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/net-server.h */
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
    /* Declarations for "API" of network listener/dispatcher in libapputils. */
    /* The delimeter characters supported by the addresses string. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct _krb5_fulladdr {
        pub address: *mut krb5_address,
        pub port: krb5_ui_4,
    }
    #[c2rust::src_loc = "37:1"]
    pub type krb5_fulladdr = _krb5_fulladdr;
    use super::krb5_h::{krb5_address, krb5_ui_4};
    /* NET_SERVER_H */
}
#[c2rust::header_src = "/usr/include/string.h:54"]
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
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "345:1"]
        pub fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                        __save_ptr: *mut *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
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
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:54"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
        /* int k5_getpwnam_r(const char *, blah blah) */
        /* POSIX */
        /* no getpwnam_r, or can't figure out #args or return type */
        /* int k5_getpwuid_r(uid_t, blah blah) */
        /* POSIX */
        /* no getpwuid_r, or can't figure out #args or return type */
        /* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
        /* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
        /* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
        /* Do nothing.  */
        /* Provide strlcpy/strlcat interfaces. */
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/libintl.h:54"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_util.h:55"]
pub mod kdc_util_h {
    use super::krb5_h::{krb5_data, krb5_boolean};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "117:1"]
        pub fn authind_contains(indicators: *const *mut krb5_data,
                                ind: *const libc::c_char) -> krb5_boolean;
    }
    /* __KRB5_KDC_UTIL__ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kdc/extern.h:56"]
pub mod extern_h {
    use super::krb5_h::krb5_timestamp;
    extern "C" {
        /* an empty string */
        #[no_mangle]
        #[c2rust::src_loc = "31:25"]
        pub static mut kdc_infinity: krb5_timestamp;
    }
    /* __KRB5_KDC_EXTERN__ */
    /* maximum datagram size */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:61"]
pub mod adm_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_authenticator, krb5_authenticator,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_ap_req, krb5_ap_req,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       _krb5_auth_context, krb5_anonymous_principal,
                       krb5_c_enctype_compare, krb5_c_make_checksum,
                       krb5_c_verify_checksum, krb5_c_valid_enctype,
                       krb5_c_valid_cksumtype, krb5_c_is_coll_proof_cksum,
                       krb5_c_is_keyed_cksum, krb5_build_principal_ext,
                       krb5_find_authdata, krb5_copy_principal,
                       krb5_principal_compare_flags,
                       krb5_principal_compare_any_realm,
                       krb5_principal_compare, krb5_realm_compare,
                       krb5_unparse_name, krb5_mk_error, krb5_enctype_to_name,
                       krb5_auth_con_getauthenticator,
                       krb5_auth_con_getrecvsubkey,
                       krb5_auth_con_setuseruserkey, krb5_auth_con_setaddrs,
                       krb5_auth_con_setflags, krb5_auth_con_free,
                       krb5_auth_con_init, krb5_us_timeofday, krb5_free_data,
                       krb5_free_keyblock, krb5_free_checksum_contents,
                       krb5_free_authdata, krb5_free_authenticator,
                       krb5_free_principal, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         _krb5_pa_pac_options, krb5_pa_pac_options,
                         _krb5_pa_for_user, krb5_pa_for_user,
                         _krb5_s4u_userid, krb5_s4u_userid,
                         _krb5_pa_s4u_x509_user, krb5_pa_s4u_x509_user,
                         ts_after, ts_incr, ts_delta, make_data,
                         data_eq_string, data_eq, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, k5_etypes_contains,
                         krb5int_parse_enctype_list,
                         krb5_rd_req_decoded_anyflag, krb5_free_pa_data,
                         krb5_free_ap_req, krb5_check_transited_list,
                         krb5int_c_deprecated_enctype,
                         krb5_is_permitted_enctype,
                         decode_krb5_pa_pac_options,
                         decode_krb5_pa_s4u_x509_user,
                         decode_krb5_pa_for_user, decode_krb5_ap_req,
                         encode_krb5_pa_pac_options,
                         encode_krb5_pa_s4u_x509_user, encode_krb5_s4u_userid,
                         encode_krb5_checksum, encode_krb5_kdc_req_body,
                         krb5int_find_pa_data, k5_alloc_pa_data,
                         k5_free_pa_data_element, k5_add_pa_data_element,
                         k5_add_pa_data_from_data, k5_add_empty_pa_data,
                         krb5_free_pa_for_user, krb5_free_pa_s4u_x509_user};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_fmt};
pub use self::kdb_h::{_krb5_key_data, krb5_key_data, krb5_db_entry,
                      _krb5_db_entry_new, krb5_tl_data, _krb5_tl_data,
                      _krb5_keysalt, krb5_keysalt, krb5_db_free_principal,
                      krb5_db_get_principal, krb5_db_check_transited_realms,
                      krb5_db_check_policy_as, krb5_dbe_find_enctype,
                      krb5_dbe_decrypt_key_data, krb5_db_refresh_config,
                      krb5_db_check_allowed_to_delegate, krb5_dbe_get_string,
                      krb5_db_get_s4u_x509_principal,
                      krb5_db_allowed_to_delegate_from,
                      krb5_db_get_authdata_info, krb5_dbe_free_string};
pub use self::realm_data_h::{kdc_realm_t, __kdc_realm_data, server_handle};
pub use self::net_server_h::{_krb5_fulladdr, krb5_fulladdr};
use self::string_h::{memcpy, memset, memcmp, strdup, strtok_r, strlen};
use self::stdlib_h::{malloc, calloc, free};
use self::k5_platform_h::krb5int_strlcpy;
use self::libintl_h::dgettext;
use self::kdc_util_h::authind_contains;
use self::extern_h::kdc_infinity;
use self::adm_proto_h::krb5_klog_syslog;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_util.c - Utility functions for the KDC implementation */
/*
 * Copyright 1990,1991,2007,2008,2009 by the Massachusetts Institute of Technology.
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
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
#[no_mangle]
#[c2rust::src_loc = "68:11"]
pub static mut vague_errors: libc::c_int = 0 as libc::c_int;
/*
 * concatenate first two authdata arrays, returning an allocated replacement.
 * The replacement should be freed with krb5_free_authdata().
 */
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn concat_authorization_data(mut context: krb5_context,
                                                   mut first:
                                                       *mut *mut krb5_authdata,
                                                   mut second:
                                                       *mut *mut krb5_authdata,
                                                   mut output:
                                                       *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ptr: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut retdata: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    /* count up the entries */
    i = 0 as libc::c_int; /* null-terminated array */
    if !first.is_null() {
        ptr = first;
        while !(*ptr).is_null() { i += 1; ptr = ptr.offset(1) }
    }
    if !second.is_null() {
        ptr = second;
        while !(*ptr).is_null() { i += 1; ptr = ptr.offset(1) }
    }
    retdata =
        malloc(((i + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_authdata;
    if retdata.is_null() { return 12 as libc::c_int }
    let ref mut fresh0 = *retdata.offset(i as isize);
    *fresh0 = 0 as *mut krb5_authdata;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    ptr = first;
    while j < 2 as libc::c_int {
        while !ptr.is_null() && !(*ptr).is_null() {
            /* now walk & copy */
            let ref mut fresh1 = *retdata.offset(i as isize);
            *fresh1 =
                malloc(::std::mem::size_of::<krb5_authdata>() as
                           libc::c_ulong) as *mut krb5_authdata;
            if (*retdata.offset(i as isize)).is_null() {
                krb5_free_authdata(context, retdata);
                return 12 as libc::c_int
            }
            **retdata.offset(i as isize) = **ptr;
            let ref mut fresh2 = (**retdata.offset(i as isize)).contents;
            *fresh2 =
                malloc((**retdata.offset(i as isize)).length as libc::c_ulong)
                    as *mut krb5_octet;
            if (*fresh2).is_null() {
                free(*retdata.offset(i as isize) as *mut libc::c_void);
                let ref mut fresh3 = *retdata.offset(i as isize);
                *fresh3 = 0 as *mut krb5_authdata;
                krb5_free_authdata(context, retdata);
                return 12 as libc::c_int
            }
            memcpy((**retdata.offset(i as isize)).contents as
                       *mut libc::c_void,
                   (**ptr).contents as *const libc::c_void,
                   (**retdata.offset(i as isize)).length as libc::c_ulong);
            ptr = ptr.offset(1);
            i += 1
        }
        ptr = second;
        j += 1
    }
    *output = retdata;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "131:1"]
pub unsafe extern "C" fn is_local_principal(mut kdc_active_realm:
                                                *mut kdc_realm_t,
                                            mut princ1: krb5_const_principal)
 -> krb5_boolean {
    return krb5_realm_compare((*kdc_active_realm).realm_context, princ1,
                              (*kdc_active_realm).realm_tgsprinc as
                                  krb5_const_principal);
}
/*
 * Returns TRUE if the kerberos principal is the name of a Kerberos ticket
 * service.
 */
#[no_mangle]
#[c2rust::src_loc = "141:1"]
pub unsafe extern "C" fn krb5_is_tgs_principal(mut principal:
                                                   krb5_const_principal)
 -> krb5_boolean {
    if (*principal).length != 2 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    if data_eq_string(*if (0 as libc::c_int) < (*principal).length {
                           (*principal).data.offset(0 as libc::c_int as isize)
                       } else { 0 as *mut krb5_data },
                      b"krbtgt\x00" as *const u8 as *const libc::c_char) != 0
       {
        return 1 as libc::c_int as krb5_boolean
    } else { return 0 as libc::c_int as krb5_boolean };
}
/* Returns TRUE if principal is the name of a cross-realm TGS. */
#[no_mangle]
#[c2rust::src_loc = "154:1"]
pub unsafe extern "C" fn is_cross_tgs_principal(mut principal:
                                                    krb5_const_principal)
 -> krb5_boolean {
    if krb5_is_tgs_principal(principal) == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    if data_eq(*if (1 as libc::c_int) < (*principal).length {
                    (*principal).data.offset(1 as libc::c_int as isize)
                } else { 0 as *mut krb5_data }, (*principal).realm) == 0 {
        return 1 as libc::c_int as krb5_boolean
    } else { return 0 as libc::c_int as krb5_boolean };
}
/*
 * given authentication data (provides seed for checksum), verify checksum
 * for source data.
 */
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn comp_cksum(mut kcontext: krb5_context,
                                mut source: *mut krb5_data,
                                mut ticket: *mut krb5_ticket,
                                mut his_cksum: *mut krb5_checksum)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut valid: krb5_boolean = 0;
    if krb5_c_valid_cksumtype((*his_cksum).checksum_type) == 0 {
        return -(1765328369 as libc::c_long) as krb5_error_code
    }
    /* must be collision proof */
    if krb5_c_is_coll_proof_cksum((*his_cksum).checksum_type) == 0 {
        return -(1765328334 as libc::c_long) as krb5_error_code
    }
    /* verify checksum */
    retval =
        krb5_c_verify_checksum(kcontext, (*(*ticket).enc_part2).session,
                               6 as libc::c_int, source, his_cksum,
                               &mut valid);
    if retval != 0 { return retval }
    if valid == 0 { return -(1765328353 as libc::c_long) as krb5_error_code }
    return 0 as libc::c_int;
}
/* Return true if padata contains an entry of either S4U2Self type. */
#[inline]
#[c2rust::src_loc = "197:1"]
unsafe extern "C" fn has_s4u2self_padata(mut padata: *mut *mut krb5_pa_data)
 -> krb5_boolean {
    if !krb5int_find_pa_data(0 as krb5_context, padata,
                             129 as libc::c_int).is_null() {
        return 1 as libc::c_int as krb5_boolean
    }
    if !krb5int_find_pa_data(0 as krb5_context, padata,
                             130 as libc::c_int).is_null() {
        return 1 as libc::c_int as krb5_boolean
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* If a header ticket is decrypted, *ticket_out is filled in even on error. */
#[no_mangle]
#[c2rust::src_loc = "208:1"]
pub unsafe extern "C" fn kdc_process_tgs_req(mut kdc_active_realm:
                                                 *mut kdc_realm_t,
                                             mut request: *mut krb5_kdc_req,
                                             mut from: *const krb5_fulladdr,
                                             mut pkt: *mut krb5_data,
                                             mut ticket_out:
                                                 *mut *mut krb5_ticket,
                                             mut krbtgt_ptr:
                                                 *mut *mut krb5_db_entry,
                                             mut tgskey:
                                                 *mut *mut krb5_keyblock,
                                             mut subkey:
                                                 *mut *mut krb5_keyblock,
                                             mut pa_tgs_req:
                                                 *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut tmppa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut apreq: *mut krb5_ap_req = 0 as *mut krb5_ap_req;
    let mut retval: krb5_error_code = 0;
    let mut authdata: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut scratch1: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut scratch: *mut krb5_data = 0 as *mut krb5_data;
    let mut foreign_server: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut auth_context: krb5_auth_context = 0 as krb5_auth_context;
    let mut authenticator: *mut krb5_authenticator =
        0 as *mut krb5_authenticator;
    let mut his_cksum: *mut krb5_checksum = 0 as *mut krb5_checksum;
    let mut krbtgt: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    *ticket_out = 0 as *mut krb5_ticket;
    *krbtgt_ptr = 0 as *mut krb5_db_entry;
    *tgskey = 0 as *mut krb5_keyblock;
    tmppa =
        krb5int_find_pa_data((*kdc_active_realm).realm_context,
                             (*request).padata, 1 as libc::c_int);
    if tmppa.is_null() {
        return -(1765328368 as libc::c_long) as krb5_error_code
    }
    scratch1.length = (*tmppa).length;
    scratch1.data = (*tmppa).contents as *mut libc::c_char;
    retval = decode_krb5_ap_req(&mut scratch1, &mut apreq);
    if retval != 0 { return retval }
    ticket = (*apreq).ticket;
    if (*apreq).ap_options & 0x40000000 as libc::c_int != 0 ||
           (*apreq).ap_options & 0x20000000 as libc::c_int != 0 {
        krb5_klog_syslog(6 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"TGS_REQ: SESSION KEY or MUTUAL\x00" as
                                      *const u8 as *const libc::c_char));
        retval = -(1765328372 as libc::c_long) as krb5_error_code
    } else {
        /* If the "server" principal in the ticket is not something
       in the local realm, then we must refuse to service the request
       if the client claims to be from the local realm.

       If we don't do this, then some other realm's nasty KDC can
       claim to be authenticating a client from our realm, and we'll
       give out tickets concurring with it!

       we set a flag here for checking below.
    */
        foreign_server =
            (is_local_principal(kdc_active_realm,
                                (*(*apreq).ticket).server as
                                    krb5_const_principal) == 0) as libc::c_int
                as krb5_boolean;
        retval =
            krb5_auth_con_init((*kdc_active_realm).realm_context,
                               &mut auth_context);
        if !(retval != 0) {
            /* Don't use a replay cache. */
            retval =
                krb5_auth_con_setflags((*kdc_active_realm).realm_context,
                                       auth_context, 0 as libc::c_int);
            if !(retval != 0) {
                retval =
                    krb5_auth_con_setaddrs((*kdc_active_realm).realm_context,
                                           auth_context,
                                           0 as *mut krb5_address,
                                           (*from).address);
                if !(retval != 0) {
                    retval =
                        kdc_rd_ap_req(kdc_active_realm, apreq, auth_context,
                                      &mut krbtgt, tgskey);
                    if !(retval != 0) {
                        /* "invalid flag" tickets can must be used to validate */
                        if (*(*ticket).enc_part2).flags &
                               0x1000000 as libc::c_int != 0 &&
                               (*request).kdc_options & 0x1 as libc::c_int ==
                                   0 {
                            retval =
                                -(1765328239 as libc::c_long) as
                                    krb5_error_code
                        } else {
                            retval =
                                krb5_auth_con_getrecvsubkey((*kdc_active_realm).realm_context,
                                                            auth_context,
                                                            subkey);
                            if !(retval != 0) {
                                retval =
                                    krb5_auth_con_getauthenticator((*kdc_active_realm).realm_context,
                                                                   auth_context,
                                                                   &mut authenticator);
                                if !(retval != 0) {
                                    retval =
                                        krb5_find_authdata((*kdc_active_realm).realm_context,
                                                           (*(*ticket).enc_part2).authorization_data,
                                                           (*authenticator).authorization_data,
                                                           71 as libc::c_int,
                                                           &mut authdata);
                                    if !(retval != 0 as libc::c_int) {
                                        if !authdata.is_null() &&
                                               !(*authdata.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).is_null()
                                           {
                                            krb5_set_error_message((*kdc_active_realm).realm_context,
                                                                   -(1765328372
                                                                         as
                                                                         libc::c_long)
                                                                       as
                                                                       krb5_error_code,
                                                                   b"ticket valid only as FAST armor\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char);
                                            retval =
                                                -(1765328372 as libc::c_long)
                                                    as krb5_error_code;
                                            krb5_free_authdata((*kdc_active_realm).realm_context,
                                                               authdata);
                                        } else {
                                            krb5_free_authdata((*kdc_active_realm).realm_context,
                                                               authdata);
                                            /* Check for a checksum */
                                            his_cksum =
                                                (*authenticator).checksum;
                                            if his_cksum.is_null() {
                                                retval =
                                                    -(1765328334 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            } else if foreign_server != 0 &&
                                                          has_s4u2self_padata((*request).padata)
                                                              == 0 &&
                                                          is_local_principal(kdc_active_realm,
                                                                             (*(*ticket).enc_part2).client
                                                                                 as
                                                                                 krb5_const_principal)
                                                              != 0 {
                                                /* make sure the client is of proper lineage (see above) */
                                                /* someone in a foreign realm claiming to be local */
                                                krb5_klog_syslog(6 as
                                                                     libc::c_int,
                                                                 dgettext(b"mit-krb5\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          b"PROCESS_TGS: failed lineage check\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char));
                                                retval =
                                                    -(1765328372 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            } else {
                                                /*
     * Check application checksum vs. tgs request
     *
     * We try checksumming the req-body two different ways: first we
     * try reaching into the raw asn.1 stream (if available), and
     * checksum that directly; if that fails, then we try encoding
     * using our local asn.1 library.
     */
                                                if !pkt.is_null() &&
                                                       fetch_asn1_field((*pkt).data
                                                                            as
                                                                            *mut libc::c_uchar,
                                                                        1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint,
                                                                        4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint,
                                                                        &mut scratch1)
                                                           >= 0 as libc::c_int
                                                   {
                                                    if comp_cksum((*kdc_active_realm).realm_context,
                                                                  &mut scratch1,
                                                                  ticket,
                                                                  his_cksum)
                                                           != 0 {
                                                        retval =
                                                            encode_krb5_kdc_req_body(request,
                                                                                     &mut scratch);
                                                        if retval == 0 {
                                                            retval =
                                                                comp_cksum((*kdc_active_realm).realm_context,
                                                                           scratch,
                                                                           ticket,
                                                                           his_cksum)
                                                        }
                                                        krb5_free_data((*kdc_active_realm).realm_context,
                                                                       scratch);
                                                        if retval != 0 {
                                                            current_block =
                                                                12142807504732625935;
                                                        } else {
                                                            current_block =
                                                                5891011138178424807;
                                                        }
                                                    } else {
                                                        current_block =
                                                            5891011138178424807;
                                                    }
                                                } else {
                                                    current_block =
                                                        5891011138178424807;
                                                }
                                                match current_block {
                                                    12142807504732625935 => {
                                                    }
                                                    _ => {
                                                        *pa_tgs_req = tmppa;
                                                        *krbtgt_ptr = krbtgt;
                                                        krbtgt =
                                                            0 as
                                                                *mut krb5_db_entry
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    krb5_free_authenticator((*kdc_active_realm).realm_context,
                                                            authenticator);
                                }
                            }
                        }
                    }
                }
                krb5_auth_con_free((*kdc_active_realm).realm_context,
                                   auth_context);
            }
        }
    }
    if retval != 0 as libc::c_int {
        krb5_free_keyblock((*kdc_active_realm).realm_context, *tgskey);
        *tgskey = 0 as *mut krb5_keyblock
    }
    if !(*(*apreq).ticket).enc_part2.is_null() {
        /* Steal the decrypted ticket pointer, even on error. */
        *ticket_out = (*apreq).ticket;
        (*apreq).ticket = 0 as *mut krb5_ticket
    }
    krb5_free_ap_req((*kdc_active_realm).realm_context, apreq);
    krb5_db_free_principal((*kdc_active_realm).realm_context, krbtgt);
    return retval;
}
/*
 * This is a KDC wrapper around krb5_rd_req_decoded_anyflag().
 *
 * We can't depend on KDB-as-keytab for handling the AP-REQ here for
 * optimization reasons: we want to minimize the number of KDB lookups.  We'll
 * need the KDB entry for the TGS principal, and the TGS key used to decrypt
 * the TGT, elsewhere in the TGS code.
 *
 * This function also implements key rollover support for kvno 0 cross-realm
 * TGTs issued by AD.
 */
#[c2rust::src_loc = "382:1"]
unsafe extern "C" fn kdc_rd_ap_req(mut kdc_active_realm: *mut kdc_realm_t,
                                   mut apreq: *mut krb5_ap_req,
                                   mut auth_context: krb5_auth_context,
                                   mut server: *mut *mut krb5_db_entry,
                                   mut tgskey: *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut search_enctype: krb5_enctype =
        (*(*apreq).ticket).enc_part.enctype;
    let mut match_enctype: krb5_boolean = 1 as libc::c_int as krb5_boolean;
    let mut kvno: krb5_kvno = 0;
    let mut tries: size_t = 3 as libc::c_int as size_t;
    /*
     * When we issue tickets we use the first key in the principals' highest
     * kvno keyset.  For non-cross-realm krbtgt principals we want to only
     * allow the use of the first key of the principal's keyset that matches
     * the given kvno.
     */
    if krb5_is_tgs_principal((*(*apreq).ticket).server as
                                 krb5_const_principal) != 0 &&
           is_cross_tgs_principal((*(*apreq).ticket).server as
                                      krb5_const_principal) == 0 {
        search_enctype = -(1 as libc::c_int);
        match_enctype = 0 as libc::c_int as krb5_boolean
    }
    retval =
        kdc_get_server_key((*kdc_active_realm).realm_context, (*apreq).ticket,
                           0 as libc::c_int as libc::c_uint, match_enctype,
                           server, 0 as *mut *mut krb5_keyblock,
                           0 as *mut krb5_kvno);
    if retval != 0 { return retval }
    *tgskey = 0 as *mut krb5_keyblock;
    kvno = (*(*apreq).ticket).enc_part.kvno;
    loop  {
        krb5_free_keyblock((*kdc_active_realm).realm_context, *tgskey);
        retval =
            find_server_key((*kdc_active_realm).realm_context, *server,
                            search_enctype, kvno, tgskey, &mut kvno);
        if !(retval != 0) {
            /* Make the TGS key available to krb5_rd_req_decoded_anyflag() */
            retval =
                krb5_auth_con_setuseruserkey((*kdc_active_realm).realm_context,
                                             auth_context, *tgskey);
            if retval != 0 { return retval }
            retval =
                krb5_rd_req_decoded_anyflag((*kdc_active_realm).realm_context,
                                            &mut auth_context, apreq,
                                            (*(*apreq).ticket).server as
                                                krb5_const_principal,
                                            (*kdc_active_realm).realm_keytab,
                                            0 as *mut krb5_flags,
                                            0 as *mut *mut krb5_ticket);
            /* If the ticket was decrypted, don't try any more keys. */
            if !(*(*apreq).ticket).enc_part2.is_null() { break ; }
        }
        if !(retval != 0 &&
                 (*(*apreq).ticket).enc_part.kvno ==
                     0 as libc::c_int as libc::c_uint &&
                 {
                     let fresh4 = kvno;
                     kvno = kvno.wrapping_sub(1);
                     (fresh4) > 1 as libc::c_int as libc::c_uint
                 } &&
                 {
                     tries = tries.wrapping_sub(1);
                     (tries) > 0 as libc::c_int as libc::c_ulong
                 }) {
            break ;
        }
    }
    return retval;
}
/*
 * The KDC should take the keytab associated with the realm and pass
 * that to the krb5_rd_req_decoded_anyflag(), but we still need to use
 * the service (TGS, here) key elsewhere.  This approach is faster than
 * the KDB keytab approach too.
 *
 * This is also used by do_tgs_req() for u2u auth.
 */
#[no_mangle]
#[c2rust::src_loc = "449:1"]
pub unsafe extern "C" fn kdc_get_server_key(mut context: krb5_context,
                                            mut ticket: *mut krb5_ticket,
                                            mut flags: libc::c_uint,
                                            mut match_enctype: krb5_boolean,
                                            mut server_ptr:
                                                *mut *mut krb5_db_entry,
                                            mut key: *mut *mut krb5_keyblock,
                                            mut kvno: *mut krb5_kvno)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut server: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut search_enctype: krb5_enctype = -(1 as libc::c_int);
    let mut search_kvno: krb5_kvno = -(1 as libc::c_int) as krb5_kvno;
    if match_enctype != 0 { search_enctype = (*ticket).enc_part.enctype }
    if (*ticket).enc_part.kvno != 0 { search_kvno = (*ticket).enc_part.kvno }
    *server_ptr = 0 as *mut krb5_db_entry;
    retval =
        krb5_db_get_principal(context,
                              (*ticket).server as krb5_const_principal, flags,
                              &mut server);
    if retval as libc::c_long == -(1780008443 as libc::c_long) {
        let mut sname: *mut libc::c_char = 0 as *mut libc::c_char;
        if krb5_unparse_name(context,
                             (*ticket).server as krb5_const_principal,
                             &mut sname) == 0 {
            limit_string(sname);
            krb5_klog_syslog(3 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"TGS_REQ: UNKNOWN SERVER: server=\'%s\'\x00"
                                          as *const u8 as
                                          *const libc::c_char), sname);
            free(sname as *mut libc::c_void);
        }
        return -(1765328377 as libc::c_long) as krb5_error_code
    } else { if retval != 0 { return retval } }
    if (*server).attributes & 0x1000 as libc::c_int != 0 ||
           (*server).attributes & 0x40 as libc::c_int != 0 {
        retval = -(1765328377 as libc::c_long) as krb5_error_code
    } else {
        if !key.is_null() {
            retval =
                find_server_key(context, server, search_enctype, search_kvno,
                                key, kvno);
            if retval != 0 {
                current_block = 13126051560704620981;
            } else { current_block = 14576567515993809846; }
        } else { current_block = 14576567515993809846; }
        match current_block {
            13126051560704620981 => { }
            _ => {
                *server_ptr = server;
                server = 0 as *mut krb5_db_entry;
                return 0 as libc::c_int
            }
        }
    }
    krb5_db_free_principal(context, server);
    return retval;
}
/*
 * A utility function to get the right key from a KDB entry.  Used in handling
 * of kvno 0 TGTs, for example.
 */
#[c2rust::src_loc = "505:1"]
unsafe extern "C" fn find_server_key(mut context: krb5_context,
                                     mut server: *mut krb5_db_entry,
                                     mut enctype: krb5_enctype,
                                     mut kvno: krb5_kvno,
                                     mut key_out: *mut *mut krb5_keyblock,
                                     mut kvno_out: *mut krb5_kvno)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut server_key: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    *key_out = 0 as *mut krb5_keyblock;
    retval =
        krb5_dbe_find_enctype(context, server, enctype, -(1 as libc::c_int),
                              if kvno != 0 {
                                  kvno as krb5_int32
                              } else { -(1 as libc::c_int) },
                              &mut server_key);
    if retval != 0 { return retval }
    if server_key.is_null() {
        return -(1765328377 as libc::c_long) as krb5_error_code
    }
    key =
        malloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    if key.is_null() { return 12 as libc::c_int }
    retval =
        krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock,
                                  server_key, key, 0 as *mut krb5_keysalt);
    if !(retval != 0) {
        if enctype != -(1 as libc::c_int) {
            let mut similar: krb5_boolean = 0;
            retval =
                krb5_c_enctype_compare(context, enctype, (*key).enctype,
                                       &mut similar);
            if retval != 0 {
                current_block = 8346171171892174778;
            } else if similar == 0 {
                retval = -(1780008418 as libc::c_long) as krb5_error_code;
                current_block = 8346171171892174778;
            } else {
                (*key).enctype = enctype;
                current_block = 17833034027772472439;
            }
        } else { current_block = 17833034027772472439; }
        match current_block {
            8346171171892174778 => { }
            _ => {
                *key_out = key;
                key = 0 as *mut krb5_keyblock;
                if !kvno_out.is_null() {
                    *kvno_out = (*server_key).key_data_kvno as krb5_kvno
                }
            }
        }
    }
    krb5_free_keyblock(context, key);
    return retval;
}
/*
 * If candidate is the local TGT for realm, set *alias_out to candidate and
 * *storage_out to NULL.  Otherwise, load the local TGT into *storage_out and
 * set *alias_out to *storage_out.  In either case, set *key_out to the
 * decrypted first key of the local TGT.
 *
 * In the future we might generalize this to a small per-request principal
 * cache.  For now, it saves a load operation in the common case where the AS
 * server or TGS header ticket server is the local TGT.
 */
#[no_mangle]
#[c2rust::src_loc = "559:1"]
pub unsafe extern "C" fn get_local_tgt(mut context: krb5_context,
                                       mut realm: *const krb5_data,
                                       mut candidate: *mut krb5_db_entry,
                                       mut alias_out: *mut *mut krb5_db_entry,
                                       mut storage_out:
                                           *mut *mut krb5_db_entry,
                                       mut key_out: *mut krb5_keyblock)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut storage: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut tgt: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    *alias_out = 0 as *mut krb5_db_entry;
    *storage_out = 0 as *mut krb5_db_entry;
    memset(key_out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    ret =
        krb5_build_principal_ext(context, &mut princ as *mut krb5_principal,
                                 (*realm).length, (*realm).data,
                                 6 as libc::c_int,
                                 b"krbtgt\x00" as *const u8 as
                                     *const libc::c_char, (*realm).length,
                                 (*realm).data, 0 as libc::c_int);
    if !(ret != 0) {
        if krb5_principal_compare(context,
                                  (*candidate).princ as krb5_const_principal,
                                  princ as krb5_const_principal) == 0 {
            ret =
                krb5_db_get_principal(context, princ as krb5_const_principal,
                                      0 as libc::c_int as libc::c_uint,
                                      &mut storage);
            if ret != 0 {
                current_block = 17003125052895396149;
            } else { tgt = storage; current_block = 5399440093318478209; }
        } else { tgt = candidate; current_block = 5399440093318478209; }
        match current_block {
            17003125052895396149 => { }
            _ => {
                if (*tgt).n_key_data as libc::c_int == 0 as libc::c_int {
                    ret = -(1780008417 as libc::c_long) as krb5_error_code
                } else {
                    ret =
                        krb5_dbe_decrypt_key_data(context,
                                                  0 as *const krb5_keyblock,
                                                  &mut *(*tgt).key_data.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize),
                                                  key_out,
                                                  0 as *mut krb5_keysalt);
                    if !(ret != 0) {
                        *alias_out = tgt;
                        *storage_out = storage;
                        storage = 0 as *mut krb5_db_entry
                    }
                }
            }
        }
    }
    krb5_db_free_principal(context, storage);
    krb5_free_principal(context, princ);
    return ret;
}
/* This probably wants to be updated if you support last_req stuff */
#[c2rust::src_loc = "608:28"]
static mut nolrentry: krb5_last_req_entry =
    {
        let mut init =
            _krb5_last_req_entry{magic:
                                     -(1760647407 as libc::c_long) as
                                         krb5_magic,
                                 lr_type: 0 as libc::c_int,
                                 value: 0 as libc::c_int,};
        init
    };
#[c2rust::src_loc = "609:29"]
static mut nolrarray: [*mut krb5_last_req_entry; 2] =
    unsafe {
        [&nolrentry as *const krb5_last_req_entry as *mut krb5_last_req_entry,
         0 as *const krb5_last_req_entry as *mut krb5_last_req_entry]
    };
#[no_mangle]
#[c2rust::src_loc = "611:1"]
pub unsafe extern "C" fn fetch_last_req_info(mut dbentry: *mut krb5_db_entry,
                                             mut lrentry:
                                                 *mut *mut *mut krb5_last_req_entry)
 -> krb5_error_code {
    *lrentry = nolrarray.as_mut_ptr();
    return 0 as libc::c_int;
}
/* XXX!  This is a temporary place-holder */
#[no_mangle]
#[c2rust::src_loc = "621:1"]
pub unsafe extern "C" fn check_hot_list(mut ticket: *mut krb5_ticket)
 -> krb5_error_code {
    return 0 as libc::c_int;
}
/* Convert an API error code to a protocol error code. */
#[no_mangle]
#[c2rust::src_loc = "629:1"]
pub unsafe extern "C" fn errcode_to_protocol(mut code: krb5_error_code)
 -> libc::c_int {
    let mut protcode: libc::c_int = 0;
    protcode =
        (code as libc::c_long - -(1765328384 as libc::c_long)) as libc::c_int;
    return if protcode >= 0 as libc::c_int && protcode <= 128 as libc::c_int {
               protcode
           } else { 60 as libc::c_int };
}
/* Return -1 if the AS or TGS request is disallowed due to KDC policy on
 * anonymous tickets. */
#[no_mangle]
#[c2rust::src_loc = "640:1"]
pub unsafe extern "C" fn check_anon(mut kdc_active_realm: *mut kdc_realm_t,
                                    mut client: krb5_principal,
                                    mut server: krb5_principal)
 -> libc::c_int {
    /* If restrict_anon is set, reject requests from anonymous to principals
     * other than the local TGT. */
    if (*kdc_active_realm).realm_restrict_anon != 0 &&
           krb5_principal_compare_any_realm((*kdc_active_realm).realm_context,
                                            client as krb5_const_principal,
                                            krb5_anonymous_principal()) != 0
           &&
           krb5_principal_compare((*kdc_active_realm).realm_context,
                                  server as krb5_const_principal,
                                  (*kdc_active_realm).realm_tgsprinc as
                                      krb5_const_principal) == 0 {
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "663:1"]
pub unsafe extern "C" fn validate_as_request(mut kdc_active_realm:
                                                 *mut kdc_realm_t,
                                             mut request: *mut krb5_kdc_req,
                                             mut client: krb5_db_entry,
                                             mut server: krb5_db_entry,
                                             mut kdc_time: krb5_timestamp,
                                             mut status:
                                                 *mut *const libc::c_char,
                                             mut e_data:
                                                 *mut *mut *mut krb5_pa_data)
 -> libc::c_int {
    let mut ret: krb5_error_code = 0;
    /*
     * If an option is set that is only allowed in TGS requests, complain.
     */
    if (*request).kdc_options &
           (0x20000000 as libc::c_int | 0x8000000 as libc::c_int |
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int |
                0x20000 as libc::c_int) != 0 {
        *status =
            b"INVALID AS OPTIONS\x00" as *const u8 as *const libc::c_char;
        return 13 as libc::c_int
    }
    /* The client must not be expired */
    if client.expiration != 0 && ts_after(kdc_time, client.expiration) != 0 {
        *status = b"CLIENT EXPIRED\x00" as *const u8 as *const libc::c_char;
        if vague_errors != 0 {
            return 60 as libc::c_int
        } else { return 1 as libc::c_int }
    }
    /* The client's password must not be expired, unless the server is
       a KRB5_KDC_PWCHANGE_SERVICE. */
    if client.pw_expiration != 0 &&
           ts_after(kdc_time, client.pw_expiration) != 0 &&
           server.attributes & 0x2000 as libc::c_int == 0 {
        *status =
            b"CLIENT KEY EXPIRED\x00" as *const u8 as *const libc::c_char;
        if vague_errors != 0 {
            return 60 as libc::c_int
        } else { return 23 as libc::c_int }
    }
    /* The server must not be expired */
    if server.expiration != 0 && ts_after(kdc_time, server.expiration) != 0 {
        *status = b"SERVICE EXPIRED\x00" as *const u8 as *const libc::c_char;
        return 2 as libc::c_int
    }
    /*
     * If the client requires password changing, then only allow the
     * pwchange service.
     */
    if client.attributes & 0x200 as libc::c_int != 0 &&
           server.attributes & 0x2000 as libc::c_int == 0 {
        *status =
            b"REQUIRED PWCHANGE\x00" as *const u8 as *const libc::c_char;
        return 23 as libc::c_int
    }
    /* Client and server must allow postdating tickets */
    if ((*request).kdc_options & 0x4000000 as libc::c_int != 0 ||
            (*request).kdc_options & 0x2000000 as libc::c_int != 0) &&
           (client.attributes & 0x1 as libc::c_int != 0 ||
                server.attributes & 0x1 as libc::c_int != 0) {
        *status =
            b"POSTDATE NOT ALLOWED\x00" as *const u8 as *const libc::c_char;
        return 10 as libc::c_int
    }
    /* Check to see if client is locked out */
    if client.attributes & 0x40 as libc::c_int != 0 {
        *status =
            b"CLIENT LOCKED OUT\x00" as *const u8 as *const libc::c_char;
        return 18 as libc::c_int
    }
    /* Check to see if server is locked out */
    if server.attributes & 0x40 as libc::c_int != 0 {
        *status =
            b"SERVICE LOCKED OUT\x00" as *const u8 as *const libc::c_char;
        return 7 as libc::c_int
    }
    /* Check to see if server is allowed to be a service */
    if server.attributes & 0x1000 as libc::c_int != 0 {
        *status =
            b"SERVICE NOT ALLOWED\x00" as *const u8 as *const libc::c_char;
        return 27 as libc::c_int
    }
    if check_anon(kdc_active_realm, client.princ, (*request).server) !=
           0 as libc::c_int {
        *status =
            b"ANONYMOUS NOT ALLOWED\x00" as *const u8 as *const libc::c_char;
        return 12 as libc::c_int
    }
    /* Perform KDB module policy checks. */
    ret =
        krb5_db_check_policy_as((*kdc_active_realm).realm_context, request,
                                &mut client, &mut server, kdc_time, status,
                                e_data);
    if ret != 0 && ret as libc::c_long != -(1765328134 as libc::c_long) {
        return errcode_to_protocol(ret)
    }
    return 0 as libc::c_int;
}
/*
 * Compute ticket flags based on the request, the client and server DB entry
 * (which may prohibit forwardable or proxiable tickets), and the header
 * ticket.  client may be NULL for a TGS request (although it may be set, such
 * as for an S4U2Self request).  header_enc may be NULL for an AS request.
 */
#[no_mangle]
#[c2rust::src_loc = "762:1"]
pub unsafe extern "C" fn get_ticket_flags(mut reqflags: krb5_flags,
                                          mut client: *mut krb5_db_entry,
                                          mut server: *mut krb5_db_entry,
                                          mut header_enc:
                                              *mut krb5_enc_tkt_part)
 -> krb5_flags {
    let mut flags: krb5_flags = 0;
    /* Indicate support for encrypted padata (RFC 6806), and set flags based on
     * request options and the header ticket. */
    flags =
        reqflags &
            (0x40000000 as libc::c_int | 0x20000000 as libc::c_int |
                 0x10000000 as libc::c_int | 0x8000000 as libc::c_int |
                 0x4000000 as libc::c_int | 0x2000000 as libc::c_int |
                 0x8000 as libc::c_int) | 0x10000 as libc::c_int;
    if reqflags & 0x2000000 as libc::c_int != 0 {
        flags |= 0x1000000 as libc::c_int
    }
    if !header_enc.is_null() {
        flags |=
            (*header_enc).flags &
                (0x20000000 as libc::c_int | 0x8000000 as libc::c_int |
                     0x200000 as libc::c_int | 0x100000 as libc::c_int |
                     0x8000 as libc::c_int)
    }
    if header_enc.is_null() { flags |= 0x400000 as libc::c_int }
    /* For TGS requests, indicate if the service is marked ok-as-delegate. */
    if !header_enc.is_null() &&
           (*server).attributes & 0x100000 as libc::c_int != 0 {
        flags |= 0x40000 as libc::c_int
    }
    /* Unset PROXIABLE if it is disallowed. */
    if !client.is_null() && (*client).attributes & 0x10 as libc::c_int != 0 {
        flags &= !(0x10000000 as libc::c_int)
    }
    if (*server).attributes & 0x10 as libc::c_int != 0 {
        flags &= !(0x10000000 as libc::c_int)
    }
    if !header_enc.is_null() &&
           (*header_enc).flags & 0x10000000 as libc::c_int == 0 {
        flags &= !(0x10000000 as libc::c_int)
    }
    /* Unset FORWARDABLE if it is disallowed. */
    if !client.is_null() && (*client).attributes & 0x2 as libc::c_int != 0 {
        flags &= !(0x40000000 as libc::c_int)
    }
    if (*server).attributes & 0x2 as libc::c_int != 0 {
        flags &= !(0x40000000 as libc::c_int)
    }
    if !header_enc.is_null() &&
           (*header_enc).flags & 0x40000000 as libc::c_int == 0 {
        flags &= !(0x40000000 as libc::c_int)
    }
    /* We don't currently handle issuing anonymous tickets based on
     * non-anonymous ones. */
    if !header_enc.is_null() &&
           (*header_enc).flags & 0x8000 as libc::c_int == 0 {
        flags &= !(0x8000 as libc::c_int)
    }
    return flags;
}
/* Return KRB5KDC_ERR_POLICY if indicators does not contain the required auth
 * indicators for server, ENOMEM on allocation error, 0 otherwise. */
#[no_mangle]
#[c2rust::src_loc = "808:1"]
pub unsafe extern "C" fn check_indicators(mut context: krb5_context,
                                          mut server: *mut krb5_db_entry,
                                          mut indicators:
                                              *const *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ind: *mut libc::c_char = 0 as *mut libc::c_char;
    ret =
        krb5_dbe_get_string(context, server,
                            b"require_auth\x00" as *const u8 as
                                *const libc::c_char, &mut str);
    if !(ret != 0 || str.is_null()) {
        copy = strdup(str);
        if copy.is_null() {
            ret = 12 as libc::c_int
        } else {
            /* Look for any of the space-separated strings in indicators. */
            ind =
                strtok_r(copy, b" \x00" as *const u8 as *const libc::c_char,
                         &mut save);
            loop  {
                if ind.is_null() {
                    current_block = 13536709405535804910;
                    break ;
                }
                if authind_contains(indicators, ind) != 0 {
                    current_block = 8811130597803074716;
                    break ;
                }
                ind =
                    strtok_r(0 as *mut libc::c_char,
                             b" \x00" as *const u8 as *const libc::c_char,
                             &mut save)
            }
            match current_block {
                8811130597803074716 => { }
                _ => {
                    ret = -(1765328372 as libc::c_long) as krb5_error_code;
                    krb5_set_error_message(context, ret,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Required auth indicators not present in ticket: %s\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           str);
                }
            }
        }
    }
    krb5_dbe_free_string(context, str);
    free(copy as *mut libc::c_void);
    return ret;
}
/*
 * asn1length - return encoded length of value.
 *
 * passed a pointer into the asn.1 stream, which is updated
 * to point right after the length bits.
 *
 * returns -1 on failure.
 */
#[c2rust::src_loc = "862:1"]
unsafe extern "C" fn asn1length(mut astream: *mut *mut libc::c_uchar)
 -> libc::c_int {
    let mut length: libc::c_int = 0; /* resulting length */
    let mut sublen: libc::c_int = 0; /* sublengths */
    let mut blen: libc::c_int = 0; /* bytes of length */
    let mut p: *mut libc::c_uchar =
        0 as *mut libc::c_uchar; /* substring searching */
    if **astream as libc::c_int & 0x80 as libc::c_int != 0 {
        blen = **astream as libc::c_int & 0x7f as libc::c_int;
        if blen > 3 as libc::c_int { return -(1 as libc::c_int) }
        *astream = (*astream).offset(1);
        length = 0 as libc::c_int;
        while blen != 0 {
            length = length << 8 as libc::c_int | **astream as libc::c_int;
            *astream = (*astream).offset(1);
            blen -= 1
        }
        if length == 0 as libc::c_int {
            /* indefinite length, figure out by hand */
            p = *astream;
            p = p.offset(1);
            loop  {
                /* compute value length. */
                sublen = asn1length(&mut p);
                if sublen < 0 as libc::c_int { return -(1 as libc::c_int) }
                p = p.offset(sublen as isize);
                /* check for termination */
                let fresh5 = p;
                p = p.offset(1);
                if !(*fresh5 == 0 && *p == 0) { continue ; }
                p = p.offset(1);
                break ;
            }
            length =
                p.wrapping_offset_from(*astream) as libc::c_long as
                    libc::c_int
        }
    } else {
        length = **astream as libc::c_int;
        *astream = (*astream).offset(1)
    }
    return length;
}
/*
 * fetch_asn1_field - return raw asn.1 stream of subfield.
 *
 * this routine is passed a context-dependent tag number and "level" and returns
 * the size and length of the corresponding level subfield.
 *
 * levels and are numbered starting from 1.
 *
 * returns 0 on success, -1 otherwise.
 */
#[no_mangle]
#[c2rust::src_loc = "913:1"]
pub unsafe extern "C" fn fetch_asn1_field(mut astream: *mut libc::c_uchar,
                                          mut level: libc::c_uint,
                                          mut field: libc::c_uint,
                                          mut data: *mut krb5_data)
 -> libc::c_int {
    let mut estream: *mut libc::c_uchar =
        0 as *mut libc::c_uchar; /* end of stream */
    let mut classes: libc::c_int = 0; /* # classes seen so far this level */
    let mut levels: libc::c_uint =
        0 as libc::c_int as libc::c_uint; /* levels seen so far */
    let mut lastlevel: libc::c_int =
        1000 as libc::c_int; /* last level seen */
    let mut length: libc::c_int = 0; /* various lengths */
    let mut tag: libc::c_int = 0; /* tag number */
    let mut savelen: libc::c_uchar = 0; /* saved length of our field */
    classes = -(1 as libc::c_int);
    /* we assume that the first identifier/length will tell us
       how long the entire stream is. */
    astream = astream.offset(1);
    estream = astream;
    length = asn1length(&mut astream);
    if length < 0 as libc::c_int { return -(1 as libc::c_int) }
    estream = estream.offset(length as isize);
    /* search down the stream, checking identifiers.  we process identifiers
       until we hit the "level" we want, and then process that level for our
       subfield, always making sure we don't go off the end of the stream.  */
    while astream < estream {
        if *astream as libc::c_int & 0x20 as libc::c_int == 0 {
            return -(1 as libc::c_int)
        }
        if (*astream as libc::c_int & 0xc0 as libc::c_int) >> 6 as libc::c_int
               == 2 as libc::c_int {
            tag = *astream as libc::c_int & 0x1f as libc::c_int;
            if tag <= lastlevel {
                levels = levels.wrapping_add(1);
                classes = -(1 as libc::c_int)
            }
            lastlevel = tag;
            if levels == level {
                /* in our context-dependent class, is this the one we're looking for ? */
                if tag == field as libc::c_int {
                    /* return length and data */
                    astream = astream.offset(1);
                    savelen = *astream;
                    length = asn1length(&mut astream);
                    if length < 0 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    (*data).length = length as libc::c_uint;
                    /* if the field length is indefinite, we will have to subtract two
                       (terminating octets) from the length returned since we don't want
                       to pass any info from the "wrapper" back.  asn1length will always return
                       the *total* length of the field, not just what's contained in it */
                    if savelen as libc::c_int & 0xff as libc::c_int ==
                           0x80 as libc::c_int {
                        (*data).length =
                            (*data).length.wrapping_sub(2 as libc::c_int as
                                                            libc::c_uint)
                    }
                    (*data).data = astream as *mut libc::c_char;
                    return 0 as libc::c_int
                } else {
                    if tag <= classes {
                        /* we've seen this class before, something must be wrong */
                        return -(1 as libc::c_int)
                    } else { classes = tag }
                }
            }
        }
        /* if we're not on our level yet, process this value.  otherwise skip over it */
        astream = astream.offset(1);
        length = asn1length(&mut astream);
        if length < 0 as libc::c_int { return -(1 as libc::c_int) }
        if levels == level { astream = astream.offset(length as isize) }
    }
    return -(1 as libc::c_int);
}
/* Return true if we believe server can support enctype as a session key. */
#[c2rust::src_loc = "987:1"]
unsafe extern "C" fn dbentry_supports_enctype(mut kdc_active_realm:
                                                  *mut kdc_realm_t,
                                              mut server: *mut krb5_db_entry,
                                              mut enctype: krb5_enctype)
 -> krb5_boolean {
    let mut retval: krb5_error_code = 0;
    let mut datap: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut etypes_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut default_enctypes: [krb5_enctype; 1] = [0 as libc::c_int];
    let mut etypes: *mut krb5_enctype = 0 as *mut krb5_enctype;
    let mut in_list: krb5_boolean = 0;
    /* Look up the supported session key enctypes list in the KDB. */
    retval =
        krb5_dbe_get_string((*kdc_active_realm).realm_context, server,
                            b"session_enctypes\x00" as *const u8 as
                                *const libc::c_char, &mut etypes_str);
    if retval == 0 as libc::c_int && !etypes_str.is_null() &&
           *etypes_str as libc::c_int != '\u{0}' as i32 {
        /* Pass a fake profile key for tracing of unrecognized tokens. */
        retval =
            krb5int_parse_enctype_list((*kdc_active_realm).realm_context,
                                       b"KDB-session_etypes\x00" as *const u8
                                           as *const libc::c_char, etypes_str,
                                       default_enctypes.as_mut_ptr(),
                                       &mut etypes);
        if retval == 0 as libc::c_int && !etypes.is_null() &&
               *etypes.offset(0 as libc::c_int as isize) != 0 {
            in_list = k5_etypes_contains(etypes, enctype);
            free(etypes_str as *mut libc::c_void);
            free(etypes as *mut libc::c_void);
            return in_list
        }
        /* Fall through on error or empty list */
    }
    free(etypes_str as *mut libc::c_void);
    free(etypes as *mut libc::c_void);
    /* Assume the server supports any enctype it has a long-term key for. */
    return (krb5_dbe_find_enctype((*kdc_active_realm).realm_context, server,
                                  enctype, -(1 as libc::c_int),
                                  0 as libc::c_int, &mut datap) == 0) as
               libc::c_int as krb5_boolean;
}
/*
 * This function returns the keytype which should be selected for the
 * session key.  It is based on the ordered list which the user
 * requested, and what the KDC and the application server can support.
 */
#[no_mangle]
#[c2rust::src_loc = "1027:1"]
pub unsafe extern "C" fn select_session_keytype(mut kdc_active_realm:
                                                    *mut kdc_realm_t,
                                                mut server:
                                                    *mut krb5_db_entry,
                                                mut nktypes: libc::c_int,
                                                mut ktype: *mut krb5_enctype)
 -> krb5_enctype {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nktypes {
        if !(krb5_c_valid_enctype(*ktype.offset(i as isize)) == 0) {
            if !(krb5_is_permitted_enctype((*kdc_active_realm).realm_context,
                                           *ktype.offset(i as isize)) == 0) {
                if dbentry_supports_enctype(kdc_active_realm, server,
                                            *ktype.offset(i as isize)) != 0 {
                    return *ktype.offset(i as isize)
                }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1053:1"]
pub unsafe extern "C" fn limit_string(mut name: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if name.is_null() { return }
    if strlen(name) < 128 as libc::c_int as libc::c_ulong { return }
    i = 128 as libc::c_int - 4 as libc::c_int;
    let fresh6 = i;
    i = i + 1;
    *name.offset(fresh6 as isize) = '.' as i32 as libc::c_char;
    let fresh7 = i;
    i = i + 1;
    *name.offset(fresh7 as isize) = '.' as i32 as libc::c_char;
    let fresh8 = i;
    i = i + 1;
    *name.offset(fresh8 as isize) = '.' as i32 as libc::c_char;
    *name.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
}
/* Wrapper of krb5_enctype_to_name() to include the PKINIT types. */
#[c2rust::src_loc = "1072:1"]
unsafe extern "C" fn enctype_name(mut ktype: krb5_enctype,
                                  mut buf: *mut libc::c_char,
                                  mut buflen: size_t) -> krb5_error_code {
    let mut name: *const libc::c_char =
        0 as
            *const libc::c_char; /* ensure these are always valid C-strings */
    let mut prefix: *const libc::c_char =
        b"\x00" as *const u8 as *const libc::c_char;
    let mut len: size_t = 0;
    if buflen == 0 as libc::c_int as libc::c_ulong {
        return 22 as libc::c_int
    }
    *buf = '\u{0}' as i32 as libc::c_char;
    if krb5_c_valid_enctype(ktype) == 0 {
        prefix = b"UNSUPPORTED:\x00" as *const u8 as *const libc::c_char
    } else if krb5int_c_deprecated_enctype(ktype) != 0 {
        prefix = b"DEPRECATED:\x00" as *const u8 as *const libc::c_char
    }
    len = krb5int_strlcpy(buf, prefix, buflen);
    if len >= buflen { return 12 as libc::c_int }
    buflen = (buflen as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
    buf = buf.offset(len as isize);
    /* rfc4556 recommends that clients wishing to indicate support for these
     * pkinit algorithms include them in the etype field of the AS-REQ. */
    if ktype == 0x9 as libc::c_int {
        name =
            b"id-dsa-with-sha1-CmsOID\x00" as *const u8 as *const libc::c_char
    } else if ktype == 0xa as libc::c_int {
        name =
            b"md5WithRSAEncryption-CmsOID\x00" as *const u8 as
                *const libc::c_char
    } else if ktype == 0xb as libc::c_int {
        name =
            b"sha-1WithRSAEncryption-CmsOID\x00" as *const u8 as
                *const libc::c_char
    } else if ktype == 0xc as libc::c_int {
        name = b"rc2-cbc-EnvOID\x00" as *const u8 as *const libc::c_char
    } else if ktype == 0xd as libc::c_int {
        name = b"rsaEncryption-EnvOID\x00" as *const u8 as *const libc::c_char
    } else if ktype == 0xe as libc::c_int {
        name = b"id-RSAES-OAEP-EnvOID\x00" as *const u8 as *const libc::c_char
    } else if ktype == 0xf as libc::c_int {
        name = b"des-ede3-cbc-EnvOID\x00" as *const u8 as *const libc::c_char
    } else {
        return krb5_enctype_to_name(ktype, 0 as libc::c_int as krb5_boolean,
                                    buf, buflen)
    }
    if krb5int_strlcpy(buf, name, buflen) >= buflen {
        return 12 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1116:1"]
pub unsafe extern "C" fn ktypes2str(mut ktype: *mut krb5_enctype,
                                    mut nktypes: libc::c_int)
 -> *mut libc::c_char {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    if nktypes < 0 as libc::c_int { return 0 as *mut libc::c_char }
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add_fmt(&mut buf as *mut k5buf,
                   b"%d etypes {\x00" as *const u8 as *const libc::c_char,
                   nktypes);
    i = 0 as libc::c_int;
    while i < nktypes {
        enctype_name(*ktype.offset(i as isize), name.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 64]>() as
                         libc::c_ulong);
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b"%s%s(%ld)\x00" as *const u8 as *const libc::c_char,
                       if i != 0 {
                           b", \x00" as *const u8 as *const libc::c_char
                       } else { b"\x00" as *const u8 as *const libc::c_char },
                       name.as_mut_ptr(),
                       *ktype.offset(i as isize) as libc::c_long);
        i += 1
    }
    k5_buf_add(&mut buf, b"}\x00" as *const u8 as *const libc::c_char);
    return buf.data as *mut libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "1136:1"]
pub unsafe extern "C" fn rep_etypes2str(mut rep: *mut krb5_kdc_rep)
 -> *mut libc::c_char {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut etype: krb5_enctype = 0;
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add(&mut buf,
               b"etypes {rep=\x00" as *const u8 as *const libc::c_char);
    enctype_name((*rep).enc_part.enctype, name.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 64]>() as
                     libc::c_ulong);
    k5_buf_add_fmt(&mut buf as *mut k5buf,
                   b"%s(%ld)\x00" as *const u8 as *const libc::c_char,
                   name.as_mut_ptr(),
                   (*rep).enc_part.enctype as libc::c_long);
    if !(*rep).ticket.is_null() {
        etype = (*(*rep).ticket).enc_part.enctype;
        enctype_name(etype, name.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 64]>() as
                         libc::c_ulong);
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b", tkt=%s(%ld)\x00" as *const u8 as
                           *const libc::c_char, name.as_mut_ptr(),
                       etype as libc::c_long);
    }
    if !(*rep).ticket.is_null() && !(*(*rep).ticket).enc_part2.is_null() &&
           !(*(*(*rep).ticket).enc_part2).session.is_null() {
        etype = (*(*(*(*rep).ticket).enc_part2).session).enctype;
        enctype_name(etype, name.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 64]>() as
                         libc::c_ulong);
        k5_buf_add_fmt(&mut buf as *mut k5buf,
                       b", ses=%s(%ld)\x00" as *const u8 as
                           *const libc::c_char, name.as_mut_ptr(),
                       etype as libc::c_long);
    }
    k5_buf_add(&mut buf, b"}\x00" as *const u8 as *const libc::c_char);
    return buf.data as *mut libc::c_char;
}
#[c2rust::src_loc = "1165:1"]
unsafe extern "C" fn verify_for_user_checksum(mut context: krb5_context,
                                              mut key: *mut krb5_keyblock,
                                              mut req: *mut krb5_pa_for_user)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut name_type: krb5_int32 = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut valid: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    if krb5_c_is_keyed_cksum((*req).cksum.checksum_type) == 0 {
        return -(1765328334 as libc::c_long) as krb5_error_code
    }
    /*
     * Checksum is over name type and string components of
     * client principal name and auth_package.
     */
    data.length = 4 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < (*(*req).user).length {
        data.length =
            data.length.wrapping_add((*if i < (*(*req).user).length {
                                           (*(*req).user).data.offset(i as
                                                                          isize)
                                       } else {
                                           0 as *mut krb5_data
                                       }).length);
        i += 1
    }
    data.length = data.length.wrapping_add((*(*req).user).realm.length);
    data.length = data.length.wrapping_add((*req).auth_package.length);
    data.data = malloc(data.length as libc::c_ulong) as *mut libc::c_char;
    p = data.data;
    if data.data.is_null() { return 12 as libc::c_int }
    name_type = (*(*req).user).type_0;
    *p.offset(0 as libc::c_int as isize) =
        (name_type >> 0 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *p.offset(1 as libc::c_int as isize) =
        (name_type >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    *p.offset(2 as libc::c_int as isize) =
        (name_type >> 16 as libc::c_int & 0xff as libc::c_int) as
            libc::c_char;
    *p.offset(3 as libc::c_int as isize) =
        (name_type >> 24 as libc::c_int & 0xff as libc::c_int) as
            libc::c_char;
    p = p.offset(4 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*(*req).user).length {
        if (*(if i < (*(*req).user).length {
                  (*(*req).user).data.offset(i as isize)
              } else { 0 as *mut krb5_data })).length >
               0 as libc::c_int as libc::c_uint {
            memcpy(p as *mut libc::c_void,
                   (*if i < (*(*req).user).length {
                         (*(*req).user).data.offset(i as isize)
                     } else { 0 as *mut krb5_data }).data as
                       *const libc::c_void,
                   (*if i < (*(*req).user).length {
                         (*(*req).user).data.offset(i as isize)
                     } else { 0 as *mut krb5_data }).length as libc::c_ulong);
        }
        p =
            p.offset((*if i < (*(*req).user).length {
                           (*(*req).user).data.offset(i as isize)
                       } else { 0 as *mut krb5_data }).length as isize);
        i += 1
    }
    if (*(*req).user).realm.length > 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void,
               (*(*req).user).realm.data as *const libc::c_void,
               (*(*req).user).realm.length as libc::c_ulong);
    }
    p = p.offset((*(*req).user).realm.length as isize);
    if (*req).auth_package.length > 0 as libc::c_int as libc::c_uint {
        memcpy(p as *mut libc::c_void,
               (*req).auth_package.data as *const libc::c_void,
               (*req).auth_package.length as libc::c_ulong);
    }
    p = p.offset((*req).auth_package.length as isize);
    code =
        krb5_c_verify_checksum(context, key, 17 as libc::c_int, &mut data,
                               &mut (*req).cksum, &mut valid);
    if code == 0 as libc::c_int && valid == 0 as libc::c_int as libc::c_uint {
        code = -(1765328343 as libc::c_long) as krb5_error_code
    }
    free(data.data as *mut libc::c_void);
    return code;
}
/*
 * Legacy protocol transition (Windows 2003 and above)
 */
#[c2rust::src_loc = "1240:1"]
unsafe extern "C" fn kdc_process_for_user(mut kdc_active_realm:
                                              *mut kdc_realm_t,
                                          mut pa_data: *mut krb5_pa_data,
                                          mut tgs_session: *mut krb5_keyblock,
                                          mut s4u_x509_user:
                                              *mut *mut krb5_pa_s4u_x509_user,
                                          mut status:
                                              *mut *const libc::c_char)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut for_user: *mut krb5_pa_for_user = 0 as *mut krb5_pa_for_user;
    let mut req_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    req_data.length = (*pa_data).length;
    req_data.data = (*pa_data).contents as *mut libc::c_char;
    code = decode_krb5_pa_for_user(&mut req_data, &mut for_user);
    if code != 0 {
        *status =
            b"DECODE_PA_FOR_USER\x00" as *const u8 as *const libc::c_char;
        return code
    }
    code =
        verify_for_user_checksum((*kdc_active_realm).realm_context,
                                 tgs_session, for_user);
    if code != 0 {
        *status =
            b"INVALID_S4U2SELF_CHECKSUM\x00" as *const u8 as
                *const libc::c_char;
        krb5_free_pa_for_user((*kdc_active_realm).realm_context, for_user);
        return code
    }
    *s4u_x509_user =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_pa_s4u_x509_user>() as
                   libc::c_ulong) as *mut krb5_pa_s4u_x509_user;
    if (*s4u_x509_user).is_null() {
        krb5_free_pa_for_user((*kdc_active_realm).realm_context, for_user);
        return 12 as libc::c_int
    }
    (**s4u_x509_user).user_id.user = (*for_user).user;
    (*for_user).user = 0 as krb5_principal;
    krb5_free_pa_for_user((*kdc_active_realm).realm_context, for_user);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1280:1"]
unsafe extern "C" fn verify_s4u_x509_user_checksum(mut context: krb5_context,
                                                   mut key:
                                                       *mut krb5_keyblock,
                                                   mut req_data:
                                                       *mut krb5_data,
                                                   mut kdc_req_nonce:
                                                       krb5_int32,
                                                   mut req:
                                                       *mut krb5_pa_s4u_x509_user)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut valid: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    if enctype_requires_etype_info_2((*key).enctype) != 0 &&
           krb5_c_is_keyed_cksum((*req).cksum.checksum_type) == 0 {
        return -(1765328334 as libc::c_long) as krb5_error_code
    }
    if (*req).user_id.nonce != kdc_req_nonce {
        return -(1765328343 as libc::c_long) as krb5_error_code
    }
    /*
     * Verify checksum over the encoded userid. If that fails,
     * re-encode, and verify that. This is similar to the
     * behaviour in kdc_process_tgs_req().
     */
    if fetch_asn1_field((*req_data).data as *mut libc::c_uchar,
                        1 as libc::c_int as libc::c_uint,
                        0 as libc::c_int as libc::c_uint, &mut scratch) <
           0 as libc::c_int {
        return 1859794441 as libc::c_long as krb5_error_code
    }
    code =
        krb5_c_verify_checksum(context, key, 26 as libc::c_int, &mut scratch,
                               &mut (*req).cksum, &mut valid);
    if code != 0 as libc::c_int { return code }
    if valid == 0 as libc::c_int as libc::c_uint {
        let mut data: *mut krb5_data = 0 as *mut krb5_data;
        code = encode_krb5_s4u_userid(&mut (*req).user_id, &mut data);
        if code != 0 as libc::c_int { return code }
        code =
            krb5_c_verify_checksum(context, key, 26 as libc::c_int, data,
                                   &mut (*req).cksum, &mut valid);
        krb5_free_data(context, data);
        if code != 0 as libc::c_int { return code }
    }
    return if valid != 0 {
               0 as libc::c_int as libc::c_long
           } else { -(1765328343 as libc::c_long) } as krb5_error_code;
}
/*
 * New protocol transition request (Windows 2008 and above)
 */
#[c2rust::src_loc = "1341:1"]
unsafe extern "C" fn kdc_process_s4u_x509_user(mut context: krb5_context,
                                               mut request: *mut krb5_kdc_req,
                                               mut pa_data: *mut krb5_pa_data,
                                               mut tgs_subkey:
                                                   *mut krb5_keyblock,
                                               mut tgs_session:
                                                   *mut krb5_keyblock,
                                               mut s4u_x509_user:
                                                   *mut *mut krb5_pa_s4u_x509_user,
                                               mut status:
                                                   *mut *const libc::c_char)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut req_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    req_data.length = (*pa_data).length;
    req_data.data = (*pa_data).contents as *mut libc::c_char;
    code = decode_krb5_pa_s4u_x509_user(&mut req_data, s4u_x509_user);
    if code != 0 {
        *status =
            b"DECODE_PA_S4U_X509_USER\x00" as *const u8 as
                *const libc::c_char;
        return code
    }
    code =
        verify_s4u_x509_user_checksum(context,
                                      if !tgs_subkey.is_null() {
                                          tgs_subkey
                                      } else { tgs_session }, &mut req_data,
                                      (*request).nonce, *s4u_x509_user);
    if code != 0 {
        *status =
            b"INVALID_S4U2SELF_CHECKSUM\x00" as *const u8 as
                *const libc::c_char;
        krb5_free_pa_s4u_x509_user(context, *s4u_x509_user);
        *s4u_x509_user = 0 as *mut krb5_pa_s4u_x509_user;
        return code
    }
    if (*(**s4u_x509_user).user_id.user).length == 0 as libc::c_int &&
           (**s4u_x509_user).user_id.subject_cert.length ==
               0 as libc::c_int as libc::c_uint {
        *status =
            b"INVALID_S4U2SELF_REQUEST\x00" as *const u8 as
                *const libc::c_char;
        krb5_free_pa_s4u_x509_user(context, *s4u_x509_user);
        *s4u_x509_user = 0 as *mut krb5_pa_s4u_x509_user;
        return -(1765328378 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1386:1"]
pub unsafe extern "C" fn kdc_make_s4u2self_rep(mut context: krb5_context,
                                               mut tgs_subkey:
                                                   *mut krb5_keyblock,
                                               mut tgs_session:
                                                   *mut krb5_keyblock,
                                               mut req_s4u_user:
                                                   *mut krb5_pa_s4u_x509_user,
                                               mut reply: *mut krb5_kdc_rep,
                                               mut reply_encpart:
                                                   *mut krb5_enc_kdc_rep_part)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut der_user_id: *mut krb5_data = 0 as *mut krb5_data;
    let mut der_s4u_x509_user: *mut krb5_data = 0 as *mut krb5_data;
    let mut rep_s4u_user: krb5_pa_s4u_x509_user =
        krb5_pa_s4u_x509_user{user_id:
                                  krb5_s4u_userid{nonce: 0,
                                                  user:
                                                      0 as
                                                          *mut krb5_principal_data,
                                                  subject_cert:
                                                      krb5_data{magic: 0,
                                                                length: 0,
                                                                data:
                                                                    0 as
                                                                        *mut libc::c_char,},
                                                  options: 0,},
                              cksum:
                                  krb5_checksum{magic: 0,
                                                checksum_type: 0,
                                                length: 0,
                                                contents:
                                                    0 as *mut krb5_octet,},};
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut enctype: krb5_enctype = 0;
    let mut usage: krb5_keyusage = 0;
    memset(&mut rep_s4u_user as *mut krb5_pa_s4u_x509_user as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_pa_s4u_x509_user>() as libc::c_ulong);
    rep_s4u_user.user_id.nonce = (*req_s4u_user).user_id.nonce;
    rep_s4u_user.user_id.user = (*req_s4u_user).user_id.user;
    rep_s4u_user.user_id.options =
        (*req_s4u_user).user_id.options & 0x20000000 as libc::c_int;
    code =
        encode_krb5_s4u_userid(&mut rep_s4u_user.user_id, &mut der_user_id);
    if !(code != 0 as libc::c_int) {
        if (*req_s4u_user).user_id.options & 0x20000000 as libc::c_int != 0 {
            usage = 27 as libc::c_int
        } else { usage = 26 as libc::c_int }
        code =
            krb5_c_make_checksum(context, (*req_s4u_user).cksum.checksum_type,
                                 if !tgs_subkey.is_null() {
                                     tgs_subkey
                                 } else { tgs_session }, usage, der_user_id,
                                 &mut rep_s4u_user.cksum);
        if !(code != 0 as libc::c_int) {
            code =
                encode_krb5_pa_s4u_x509_user(&mut rep_s4u_user,
                                             &mut der_s4u_x509_user);
            if !(code != 0 as libc::c_int) {
                code =
                    k5_add_pa_data_from_data(&mut (*reply).padata,
                                             130 as libc::c_int,
                                             der_s4u_x509_user);
                if !(code != 0 as libc::c_int) {
                    if !tgs_subkey.is_null() {
                        enctype = (*tgs_subkey).enctype
                    } else { enctype = (*tgs_session).enctype }
                    /*
     * Owing to a bug in Windows, unkeyed checksums were used for older
     * enctypes, including rc4-hmac. A forthcoming workaround for this
     * includes the checksum bytes in the encrypted padata.
     */
                    if (*req_s4u_user).user_id.options &
                           0x20000000 as libc::c_int != 0 &&
                           enctype_requires_etype_info_2(enctype) ==
                               0 as libc::c_int as libc::c_uint {
                        code =
                            k5_alloc_pa_data(130 as libc::c_int,
                                             (*req_s4u_user).cksum.length.wrapping_add(rep_s4u_user.cksum.length)
                                                 as size_t, &mut pa);
                        if !(code != 0 as libc::c_int) {
                            memcpy((*pa).contents as *mut libc::c_void,
                                   (*req_s4u_user).cksum.contents as
                                       *const libc::c_void,
                                   (*req_s4u_user).cksum.length as
                                       libc::c_ulong);
                            memcpy(&mut *(*pa).contents.offset((*req_s4u_user).cksum.length
                                                                   as isize)
                                       as *mut krb5_octet as
                                       *mut libc::c_void,
                                   rep_s4u_user.cksum.contents as
                                       *const libc::c_void,
                                   rep_s4u_user.cksum.length as
                                       libc::c_ulong);
                            code =
                                k5_add_pa_data_element(&mut (*reply_encpart).enc_padata,
                                                       &mut pa);
                            (code) != 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    if !rep_s4u_user.cksum.contents.is_null() {
        krb5_free_checksum_contents(context, &mut rep_s4u_user.cksum);
    }
    krb5_free_data(context, der_user_id);
    krb5_free_data(context, der_s4u_x509_user);
    k5_free_pa_data_element(pa);
    return code;
}
/*
 * Protocol transition (S4U2Self)
 */
#[no_mangle]
#[c2rust::src_loc = "1471:1"]
pub unsafe extern "C" fn kdc_process_s4u2self_req(mut kdc_active_realm:
                                                      *mut kdc_realm_t,
                                                  mut request:
                                                      *mut krb5_kdc_req,
                                                  mut client_princ:
                                                      krb5_const_principal,
                                                  mut c_flags: libc::c_uint,
                                                  mut server:
                                                      *const krb5_db_entry,
                                                  mut tgs_subkey:
                                                      *mut krb5_keyblock,
                                                  mut tgs_session:
                                                      *mut krb5_keyblock,
                                                  mut kdc_time:
                                                      krb5_timestamp,
                                                  mut s4u_x509_user:
                                                      *mut *mut krb5_pa_s4u_x509_user,
                                                  mut princ_ptr:
                                                      *mut *mut krb5_db_entry,
                                                  mut status:
                                                      *mut *const libc::c_char)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut pa_data: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut flags: libc::c_int = 0;
    let mut princ: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut id: *mut krb5_s4u_userid = 0 as *mut krb5_s4u_userid;
    *princ_ptr = 0 as *mut krb5_db_entry;
    pa_data =
        krb5int_find_pa_data((*kdc_active_realm).realm_context,
                             (*request).padata, 130 as libc::c_int);
    if !pa_data.is_null() {
        code =
            kdc_process_s4u_x509_user((*kdc_active_realm).realm_context,
                                      request, pa_data, tgs_subkey,
                                      tgs_session, s4u_x509_user, status);
        if code != 0 as libc::c_int { return code }
    } else {
        pa_data =
            krb5int_find_pa_data((*kdc_active_realm).realm_context,
                                 (*request).padata, 129 as libc::c_int);
        if !pa_data.is_null() {
            code =
                kdc_process_for_user(kdc_active_realm, pa_data, tgs_session,
                                     s4u_x509_user, status);
            if code != 0 as libc::c_int { return code }
        } else { return 0 as libc::c_int }
    }
    id = &mut (**s4u_x509_user).user_id;
    /*
     * We need to compare the client name in the TGT with the requested
     * server name. Supporting server name aliases without assuming a
     * global name service makes this difficult to do.
     *
     * The comparison below handles the following cases (note that the
     * term "principal name" below excludes the realm).
     *
     * (1) The requested service is a host-based service with two name
     *     components, in which case we assume the principal name to
     *     contain sufficient qualifying information. The realm is
     *     ignored for the purpose of comparison.
     *
     * (2) The requested service name is an enterprise principal name:
     *     the service principal name is compared with the unparsed
     *     form of the client name (including its realm).
     *
     * (3) The requested service is some other name type: an exact
     *     match is required.
     *
     * An alternative would be to look up the server once again with
     * FLAG_CANONICALIZE | FLAG_CLIENT_REFERRALS_ONLY set, do an exact
     * match between the returned name and client_princ. However, this
     * assumes that the client set FLAG_CANONICALIZE when requesting
     * the TGT and that we have a global name service.
     */
    flags = 0 as libc::c_int;
    match (*(*request).server).type_0 {
        3 => {
            /* (1) */
            if (*(*request).server).length == 2 as libc::c_int {
                flags |= 1 as libc::c_int
            }
        }
        10 => {
            /* (2) */
            flags |= 2 as libc::c_int
        }
        _ => { }
    }
    if krb5_principal_compare_flags((*kdc_active_realm).realm_context,
                                    (*request).server as krb5_const_principal,
                                    client_princ, flags) == 0 {
        *status =
            b"INVALID_S4U2SELF_REQUEST\x00" as *const u8 as
                *const libc::c_char;
        return -(1765328378 as libc::c_long) as krb5_error_code
        /* match Windows error code */
    }
    /*
     * Protocol transition is mutually exclusive with renew/forward/etc
     * as well as user-to-user and constrained delegation. This check
     * is also made in validate_as_request().
     *
     * We can assert from this check that the header ticket was a TGT, as
     * that is validated previously in validate_tgs_request().
     */
    if (*request).kdc_options &
           (0x20000000 as libc::c_int | 0x8000000 as libc::c_int |
                0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int |
                0x20000 as libc::c_int) != 0 {
        *status =
            b"INVALID AS OPTIONS\x00" as *const u8 as *const libc::c_char;
        return -(1765328371 as libc::c_long) as krb5_error_code
    }
    /*
     * Valid S4U2Self requests can occur in the following combinations:
     *
     * (1) local TGT, local user, local server
     * (2) cross TGT, local user, issuing referral
     * (3) cross TGT, non-local user, issuing referral
     * (4) cross TGT, non-local user, local server
     *
     * The first case is for a single-realm S4U2Self scenario; the second,
     * third, and fourth cases are for the initial, intermediate (if any), and
     * final cross-realm requests in a multi-realm scenario.
     */
    if c_flags & 0x1000 as libc::c_int as libc::c_uint == 0 &&
           c_flags & 0x4000 as libc::c_int as libc::c_uint != 0 {
        /* The requesting server appears to no longer exist, and we found
         * a referral instead.  Treat this as a server lookup failure. */
        *status =
            b"LOOKING_UP_SERVER\x00" as *const u8 as *const libc::c_char;
        return -(1765328377 as libc::c_long) as krb5_error_code
    }
    /*
     * Do not attempt to lookup principals in foreign realms.
     */
    if is_local_principal(kdc_active_realm,
                          (*id).user as krb5_const_principal) != 0 {
        let mut no_server: krb5_db_entry =
            krb5_db_entry{magic: 0,
                          len: 0,
                          mask: 0,
                          attributes: 0,
                          max_life: 0,
                          max_renewable_life: 0,
                          expiration: 0,
                          pw_expiration: 0,
                          last_success: 0,
                          last_failed: 0,
                          fail_auth_count: 0,
                          n_tl_data: 0,
                          n_key_data: 0,
                          e_length: 0,
                          e_data: 0 as *mut krb5_octet,
                          princ: 0 as *mut krb5_principal_data,
                          tl_data: 0 as *mut krb5_tl_data,
                          key_data: 0 as *mut krb5_key_data,};
        let mut e_data: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
        if c_flags & 0x1000 as libc::c_int as libc::c_uint != 0 &&
               c_flags & 0x4000 as libc::c_int as libc::c_uint == 0 {
            /* A local server should not need a cross-realm TGT to impersonate
             * a local principal. */
            *status =
                b"NOT_CROSS_REALM_REQUEST\x00" as *const u8 as
                    *const libc::c_char;
            return -(1765328378 as libc::c_long) as krb5_error_code
            /* match Windows error */
        }
        if (*id).subject_cert.length != 0 as libc::c_int as libc::c_uint {
            code =
                krb5_db_get_s4u_x509_principal((*kdc_active_realm).realm_context,
                                               &mut (*id).subject_cert,
                                               (*id).user as
                                                   krb5_const_principal,
                                               0x20 as libc::c_int as
                                                   libc::c_uint, &mut princ);
            if code == 0 as libc::c_int &&
                   (*(*id).user).length == 0 as libc::c_int {
                krb5_free_principal((*kdc_active_realm).realm_context,
                                    (*id).user);
                code =
                    krb5_copy_principal((*kdc_active_realm).realm_context,
                                        (*princ).princ as
                                            krb5_const_principal,
                                        &mut (*id).user)
            }
        } else {
            code =
                krb5_db_get_principal((*kdc_active_realm).realm_context,
                                      (*id).user as krb5_const_principal,
                                      0x20 as libc::c_int as libc::c_uint,
                                      &mut princ)
        }
        if code as libc::c_long == -(1780008443 as libc::c_long) {
            *status =
                b"UNKNOWN_S4U2SELF_PRINCIPAL\x00" as *const u8 as
                    *const libc::c_char;
            return -(1765328378 as libc::c_long) as krb5_error_code
        } else {
            if code != 0 {
                *status =
                    b"LOOKING_UP_S4U2SELF_PRINCIPAL\x00" as *const u8 as
                        *const libc::c_char;
                return code
                /* caller can free for_user */
            }
        }
        memset(&mut no_server as *mut krb5_db_entry as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<krb5_db_entry>() as libc::c_ulong);
        /* Ignore password expiration and needchange attributes (as Windows
         * does), since S4U2Self is not password authentication. */
        (*princ).pw_expiration = 0 as libc::c_int;
        (*princ).attributes &= !(0x200 as libc::c_int);
        code =
            validate_as_request(kdc_active_realm, request, *princ, no_server,
                                kdc_time, status, &mut e_data);
        if code != 0 {
            krb5_db_free_principal((*kdc_active_realm).realm_context, princ);
            krb5_free_pa_data((*kdc_active_realm).realm_context, e_data);
            return code
        }
        *princ_ptr = princ
    } else if c_flags & 0x1000 as libc::c_int as libc::c_uint == 0 {
        /*
         * The server is asking to impersonate a principal from another realm,
         * using a local TGT.  It should instead ask that principal's realm and
         * follow referrals back to us.
         */
        *status =
            b"S4U2SELF_CLIENT_NOT_OURS\x00" as *const u8 as
                *const libc::c_char;
        return -(1765328372 as libc::c_long) as krb5_error_code
        /* match Windows error */
    } else {
        if (*(*id).user).length == 0 as libc::c_int {
            /*
         * Only a KDC in the client realm can handle a certificate-only
         * S4U2Self request.  Other KDCs require a principal name and ignore
         * the subject-certificate field.
         */
            *status =
                b"INVALID_XREALM_S4U2SELF_REQUEST\x00" as *const u8 as
                    *const libc::c_char;
            return -(1765328372 as libc::c_long) as krb5_error_code
            /* match Windows error */
        }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1675:1"]
unsafe extern "C" fn check_allowed_to_delegate_to(mut context: krb5_context,
                                                  mut client:
                                                      krb5_const_principal,
                                                  mut server:
                                                      *const krb5_db_entry,
                                                  mut proxy:
                                                      krb5_const_principal)
 -> krb5_error_code {
    /* Must be in same realm */
    if krb5_realm_compare(context, (*server).princ as krb5_const_principal,
                          proxy) == 0 {
        return -(1765328372 as libc::c_long) as krb5_error_code
    }
    return krb5_db_check_allowed_to_delegate(context, client, server, proxy);
}
#[c2rust::src_loc = "1687:1"]
unsafe extern "C" fn check_rbcd_policy(mut kdc_active_realm: *mut kdc_realm_t,
                                       mut flags: libc::c_uint,
                                       stkt_client_princ: krb5_principal,
                                       mut stkt_authdata_client:
                                           krb5_principal,
                                       mut stkt_server: *const krb5_db_entry,
                                       mut header_client_princ:
                                           krb5_const_principal,
                                       mut header_ad_info: *mut libc::c_void,
                                       mut proxy: *const krb5_db_entry)
 -> krb5_error_code {
    let mut client_princ: krb5_principal = stkt_client_princ;
    /* Ensure that either the evidence ticket server or the client matches the
     * TGT client. */
    if flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
        /*
         * Check that the proxy server is local, that the second ticket is a
         * cross realm TGT, and that the second ticket client matches the
         * header ticket client.
         */
        if flags & 0x4000 as libc::c_int as libc::c_uint != 0 ||
               is_cross_tgs_principal((*stkt_server).princ as
                                          krb5_const_principal) == 0 ||
               krb5_principal_compare((*kdc_active_realm).realm_context,
                                      stkt_client_princ as
                                          krb5_const_principal,
                                      header_client_princ) == 0 {
            return -(1765328371 as libc::c_long) as krb5_error_code
        }
        /* The KDB module must be able to recover the reply ticket client name
         * from the evidence ticket authorization data. */
        if stkt_authdata_client.is_null() ||
               (*stkt_authdata_client).realm.length ==
                   0 as libc::c_int as libc::c_uint {
            return -(1765328371 as libc::c_long) as krb5_error_code
        }
        client_princ = stkt_authdata_client
    } else if krb5_principal_compare((*kdc_active_realm).realm_context,
                                     (*stkt_server).princ as
                                         krb5_const_principal,
                                     header_client_princ) == 0 {
        return -(1765328371 as libc::c_long) as krb5_error_code
    }
    /* If we are issuing a referral, the KDC in the resource realm will check
     * if delegation is allowed. */
    if flags & 0x4000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    return krb5_db_allowed_to_delegate_from((*kdc_active_realm).realm_context,
                                            client_princ as
                                                krb5_const_principal,
                                            header_client_princ,
                                            header_ad_info, proxy);
}
#[no_mangle]
#[c2rust::src_loc = "1732:1"]
pub unsafe extern "C" fn kdc_process_s4u2proxy_req(mut kdc_active_realm:
                                                       *mut kdc_realm_t,
                                                   mut flags: libc::c_uint,
                                                   mut request:
                                                       *mut krb5_kdc_req,
                                                   mut t2enc:
                                                       *const krb5_enc_tkt_part,
                                                   mut krbtgt:
                                                       *mut krb5_db_entry,
                                                   mut krbtgt_key:
                                                       *mut krb5_keyblock,
                                                   mut server:
                                                       *const krb5_db_entry,
                                                   mut server_key:
                                                       *mut krb5_keyblock,
                                                   mut server_princ:
                                                       krb5_const_principal,
                                                   mut proxy:
                                                       *const krb5_db_entry,
                                                   mut proxy_princ:
                                                       krb5_const_principal,
                                                   mut ad_info:
                                                       *mut libc::c_void,
                                                   mut stkt_ad_info:
                                                       *mut *mut libc::c_void,
                                                   mut stkt_authdata_client:
                                                       *mut krb5_principal,
                                                   mut status:
                                                       *mut *const libc::c_char)
 -> krb5_error_code {
    let mut errcode: krb5_error_code = 0;
    let mut support_rbcd: krb5_boolean = 0;
    /*
     * Constrained delegation is mutually exclusive with renew/forward/etc.
     * We can assert from this check that the header ticket was a TGT, as
     * that is validated previously in validate_tgs_request().
     */
    if (*request).kdc_options &
           (0x20000000 as libc::c_int | 0x8000000 as libc::c_int |
                0x2 as libc::c_int | 0x1 as libc::c_int | 0x8 as libc::c_int)
           != 0 {
        *status =
            b"INVALID_S4U2PROXY_OPTIONS\x00" as *const u8 as
                *const libc::c_char;
        return -(1765328371 as libc::c_long) as krb5_error_code
    }
    /* Can't get a TGT (otherwise it would be unconstrained delegation). */
    if krb5_is_tgs_principal(proxy_princ) != 0 {
        *status =
            b"NOT_ALLOWED_TO_DELEGATE\x00" as *const u8 as
                *const libc::c_char;
        return -(1765328372 as libc::c_long) as krb5_error_code
    }
    errcode =
        krb5_db_get_authdata_info((*kdc_active_realm).realm_context, flags,
                                  (*t2enc).authorization_data,
                                  (*t2enc).client as krb5_const_principal,
                                  proxy_princ, server_key, krbtgt_key, krbtgt,
                                  (*t2enc).times.authtime, stkt_ad_info,
                                  stkt_authdata_client);
    if errcode != 0 &&
           errcode as libc::c_long != -(1765328134 as libc::c_long) {
        *status =
            b"NOT_ALLOWED_TO_DELEGATE\x00" as *const u8 as
                *const libc::c_char;
        return errcode
    }
    errcode =
        kdc_get_pa_pac_rbcd((*kdc_active_realm).realm_context,
                            (*request).padata, &mut support_rbcd);
    if errcode != 0 { return errcode }
    if support_rbcd != 0 && !ad_info.is_null() {
        errcode =
            check_rbcd_policy(kdc_active_realm, flags, (*t2enc).client,
                              *stkt_authdata_client, server, server_princ,
                              ad_info, proxy);
        if errcode == 0 as libc::c_int { return 0 as libc::c_int }
        if errcode as libc::c_long != -(1765328372 as libc::c_long) &&
               errcode as libc::c_long != -(1765328134 as libc::c_long) {
            *status =
                b"INVALID_S4U2PROXY_XREALM_REQUEST\x00" as *const u8 as
                    *const libc::c_char;
            return errcode
        }
        /* Fall back to old constrained delegation. */
    }
    /* Ensure that evidence ticket server matches TGT client */
    if krb5_principal_compare((*kdc_active_realm).realm_context,
                              (*server).princ as krb5_const_principal,
                              server_princ) == 0 {
        *status =
            b"EVIDENCE_TICKET_MISMATCH\x00" as *const u8 as
                *const libc::c_char;
        return -(1765328358 as libc::c_long) as krb5_error_code
    }
    if (*t2enc).flags & 0x40000000 as libc::c_int == 0 {
        *status =
            b"EVIDENCE_TKT_NOT_FORWARDABLE\x00" as *const u8 as
                *const libc::c_char;
        return -(1765328163 as libc::c_long) as krb5_error_code
    }
    /* Backend policy check */
    errcode =
        check_allowed_to_delegate_to((*kdc_active_realm).realm_context,
                                     (*t2enc).client as krb5_const_principal,
                                     server, proxy_princ);
    if errcode != 0 {
        *status =
            b"NOT_ALLOWED_TO_DELEGATE\x00" as *const u8 as
                *const libc::c_char;
        return errcode
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1820:1"]
pub unsafe extern "C" fn kdc_check_transited_list(mut kdc_active_realm:
                                                      *mut kdc_realm_t,
                                                  mut trans: *const krb5_data,
                                                  mut realm1:
                                                      *const krb5_data,
                                                  mut realm2:
                                                      *const krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    /* Check against the KDB module.  Treat this answer as authoritative if the
     * method is supported and doesn't explicitly pass control. */
    code =
        krb5_db_check_transited_realms((*kdc_active_realm).realm_context,
                                       trans, realm1, realm2);
    if code as libc::c_long != -(1765328134 as libc::c_long) &&
           code as libc::c_long != -(1765328135 as libc::c_long) {
        return code
    }
    /* Check using krb5.conf [capaths] or hierarchical relationships. */
    return krb5_check_transited_list((*kdc_active_realm).realm_context, trans,
                                     realm1, realm2);
}
#[no_mangle]
#[c2rust::src_loc = "1838:1"]
pub unsafe extern "C" fn enctype_requires_etype_info_2(mut enctype:
                                                           krb5_enctype)
 -> krb5_boolean {
    match enctype {
        16 | 6 | 23 | 24 => { return 0 as libc::c_int as krb5_boolean }
        _ => { return krb5_c_valid_enctype(enctype) }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1852:1"]
pub unsafe extern "C" fn kdc_get_ticket_endtime(mut kdc_active_realm:
                                                    *mut kdc_realm_t,
                                                mut starttime: krb5_timestamp,
                                                mut endtime: krb5_timestamp,
                                                mut till: krb5_timestamp,
                                                mut client:
                                                    *mut krb5_db_entry,
                                                mut server:
                                                    *mut krb5_db_entry,
                                                mut out_endtime:
                                                    *mut krb5_timestamp) {
    let mut until: krb5_timestamp = 0;
    let mut life: krb5_deltat = 0;
    if till == 0 as libc::c_int { till = kdc_infinity }
    until = if ts_after(till, endtime) != 0 { endtime } else { till };
    /* Determine the requested lifetime, capped at the maximum valid time
     * interval. */
    life = ts_delta(until, starttime);
    if ts_after(until, starttime) != 0 && life < 0 as libc::c_int {
        life = 2147483647 as libc::c_int
    }
    if !client.is_null() && (*client).max_life != 0 as libc::c_int {
        life =
            if life < (*client).max_life { life } else { (*client).max_life }
    }
    if (*server).max_life != 0 as libc::c_int {
        life =
            if life < (*server).max_life { life } else { (*server).max_life }
    }
    if (*kdc_active_realm).realm_maxlife != 0 as libc::c_int {
        life =
            if life < (*kdc_active_realm).realm_maxlife {
                life
            } else { (*kdc_active_realm).realm_maxlife }
    }
    *out_endtime = ts_incr(starttime, life);
}
/*
 * Set tkt->renew_till to the requested renewable lifetime as modified by
 * policy.  Set the TKT_FLG_RENEWABLE flag if we set a nonzero renew_till.
 * client and tgt may be NULL.
 */
#[no_mangle]
#[c2rust::src_loc = "1890:1"]
pub unsafe extern "C" fn kdc_get_ticket_renewtime(mut realm: *mut kdc_realm_t,
                                                  mut request:
                                                      *mut krb5_kdc_req,
                                                  mut tgt:
                                                      *mut krb5_enc_tkt_part,
                                                  mut client:
                                                      *mut krb5_db_entry,
                                                  mut server:
                                                      *mut krb5_db_entry,
                                                  mut tkt:
                                                      *mut krb5_enc_tkt_part) {
    let mut rtime: krb5_timestamp = 0;
    let mut max_rlife: krb5_timestamp = 0;
    (*tkt).flags &= !(0x800000 as libc::c_int);
    (*tkt).times.renew_till = 0 as libc::c_int;
    /* Don't issue renewable tickets if the client or server don't allow it,
     * or if this is a TGS request and the TGT isn't renewable. */
    if (*server).attributes & 0x8 as libc::c_int != 0 { return }
    if !client.is_null() && (*client).attributes & 0x8 as libc::c_int != 0 {
        return
    }
    if !tgt.is_null() && (*tgt).flags & 0x800000 as libc::c_int == 0 {
        return
    }
    /* Determine the requested renewable time. */
    if (*request).kdc_options & 0x800000 as libc::c_int != 0 {
        rtime =
            if (*request).rtime != 0 {
                (*request).rtime
            } else { kdc_infinity }
    } else if (*request).kdc_options & 0x10 as libc::c_int != 0 &&
                  ts_after((*request).till, (*tkt).times.endtime) != 0 {
        rtime = (*request).till
    } else { return }
    /* Truncate it to the allowable renewable time. */
    if !tgt.is_null() {
        rtime =
            if ts_after(rtime, (*tgt).times.renew_till) != 0 {
                (*tgt).times.renew_till
            } else { rtime }
    }
    max_rlife =
        if (*server).max_renewable_life < (*realm).realm_maxrlife {
            (*server).max_renewable_life
        } else { (*realm).realm_maxrlife };
    if !client.is_null() {
        max_rlife =
            if max_rlife < (*client).max_renewable_life {
                max_rlife
            } else { (*client).max_renewable_life }
    }
    rtime =
        if ts_after(rtime, ts_incr((*tkt).times.starttime, max_rlife)) != 0 {
            ts_incr((*tkt).times.starttime, max_rlife)
        } else { rtime };
    /* If the client only specified renewable-ok, don't issue a renewable
     * ticket unless the truncated renew time exceeds the ticket end time. */
    if (*request).kdc_options & 0x800000 as libc::c_int == 0 &&
           ts_after(rtime, (*tkt).times.endtime) == 0 {
        return
    }
    (*tkt).flags |= 0x800000 as libc::c_int;
    (*tkt).times.renew_till = rtime;
}
/* *
 * Handle protected negotiation of FAST using enc_padata
 * - If ENCPADATA_REQ_ENC_PA_REP is present, then:
 * - Return ENCPADATA_REQ_ENC_PA_REP with checksum of AS-REQ from client
 * - Include PADATA_FX_FAST in the enc_padata to indicate FAST
 * @pre @c out_enc_padata has space for at least two more padata
 * @param index in/out index into @c out_enc_padata for next item
 */
#[no_mangle]
#[c2rust::src_loc = "1944:1"]
pub unsafe extern "C" fn kdc_handle_protected_negotiation(mut context:
                                                              krb5_context,
                                                          mut req_pkt:
                                                              *mut krb5_data,
                                                          mut request:
                                                              *mut krb5_kdc_req,
                                                          mut reply_key:
                                                              *const krb5_keyblock,
                                                          mut out_enc_padata:
                                                              *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut checksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut der_cksum: *mut krb5_data = 0 as *mut krb5_data;
    let mut pa_in: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    memset(&mut checksum as *mut krb5_checksum as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    pa_in =
        krb5int_find_pa_data(context, (*request).padata, 149 as libc::c_int);
    if pa_in.is_null() { return 0 as libc::c_int }
    /* Compute and encode a checksum over the AS-REQ. */
    retval =
        krb5_c_make_checksum(context, 0 as libc::c_int, reply_key,
                             56 as libc::c_int, req_pkt, &mut checksum);
    if !(retval != 0 as libc::c_int) {
        retval = encode_krb5_checksum(&mut checksum, &mut der_cksum);
        if !(retval != 0 as libc::c_int) {
            retval =
                k5_add_pa_data_from_data(out_enc_padata, 149 as libc::c_int,
                                         der_cksum);
            if !(retval != 0) {
                /* Add a zero-length PA-FX-FAST element to the list. */
                retval =
                    k5_add_empty_pa_data(out_enc_padata, 136 as libc::c_int)
            }
        }
    }
    krb5_free_checksum_contents(context, &mut checksum);
    krb5_free_data(context, der_cksum);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "1986:1"]
pub unsafe extern "C" fn kdc_get_pa_pac_options(mut context: krb5_context,
                                                mut in_padata:
                                                    *mut *mut krb5_pa_data,
                                                mut pac_options_out:
                                                    *mut *mut krb5_pa_pac_options)
 -> krb5_error_code {
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut der_pac_options: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    *pac_options_out = 0 as *mut krb5_pa_pac_options;
    pa = krb5int_find_pa_data(context, in_padata, 167 as libc::c_int);
    if pa.is_null() { return 0 as libc::c_int }
    der_pac_options =
        make_data((*pa).contents as *mut libc::c_void, (*pa).length);
    return decode_krb5_pa_pac_options(&mut der_pac_options, pac_options_out);
}
#[no_mangle]
#[c2rust::src_loc = "2003:1"]
pub unsafe extern "C" fn kdc_add_pa_pac_options(mut context: krb5_context,
                                                mut request:
                                                    *mut krb5_kdc_req,
                                                mut out_enc_padata:
                                                    *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut pac_options: *mut krb5_pa_pac_options =
        0 as *mut krb5_pa_pac_options;
    let mut der_pac_options: *mut krb5_data = 0 as *mut krb5_data;
    ret =
        kdc_get_pa_pac_options(context, (*request).padata, &mut pac_options);
    if ret != 0 || pac_options.is_null() { return ret }
    /* Only return supported PAC options (currently only resource-based
     * constrained delegation support). */
    (*pac_options).options &= 0x10000000 as libc::c_int;
    if (*pac_options).options == 0 as libc::c_int {
        free(pac_options as *mut libc::c_void);
        return 0 as libc::c_int
    }
    ret = encode_krb5_pa_pac_options(pac_options, &mut der_pac_options);
    free(pac_options as *mut libc::c_void);
    if ret != 0 { return ret }
    ret =
        k5_add_pa_data_from_data(out_enc_padata, 167 as libc::c_int,
                                 der_pac_options);
    krb5_free_data(context, der_pac_options);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "2034:1"]
pub unsafe extern "C" fn kdc_get_pa_pac_rbcd(mut context: krb5_context,
                                             mut in_padata:
                                                 *mut *mut krb5_pa_data,
                                             mut supported: *mut krb5_boolean)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut pac_options: *mut krb5_pa_pac_options =
        0 as *mut krb5_pa_pac_options;
    *supported = 0 as libc::c_int as krb5_boolean;
    retval = kdc_get_pa_pac_options(context, in_padata, &mut pac_options);
    if retval != 0 || pac_options.is_null() { return retval }
    if (*pac_options).options & 0x10000000 as libc::c_int != 0 {
        *supported = 1 as libc::c_int as krb5_boolean
    }
    free(pac_options as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * Although the KDC doesn't call this function directly,
 * process_tcp_connection_read() in net-server.c does call it.
 */
#[no_mangle]
#[c2rust::src_loc = "2058:1"]
pub unsafe extern "C" fn make_toolong_error(mut handle: *mut libc::c_void,
                                            mut out: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut errpkt: krb5_error =
        krb5_error{magic: 0,
                   ctime: 0,
                   cusec: 0,
                   susec: 0,
                   stime: 0,
                   error: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   text:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},};
    let mut retval: krb5_error_code = 0;
    let mut scratch: *mut krb5_data = 0 as *mut krb5_data;
    let mut h: *mut server_handle = handle as *mut server_handle;
    retval =
        krb5_us_timeofday((*h).kdc_err_context, &mut errpkt.stime,
                          &mut errpkt.susec);
    if retval != 0 { return retval }
    errpkt.error = 61 as libc::c_int as krb5_ui_4;
    errpkt.server =
        (**(*h).kdc_realmlist.offset(0 as libc::c_int as
                                         isize)).realm_tgsprinc;
    errpkt.client = 0 as krb5_principal;
    errpkt.cusec = 0 as libc::c_int;
    errpkt.ctime = 0 as libc::c_int;
    errpkt.text.length = 0 as libc::c_int as libc::c_uint;
    errpkt.text.data = 0 as *mut libc::c_char;
    errpkt.e_data.length = 0 as libc::c_int as libc::c_uint;
    errpkt.e_data.data = 0 as *mut libc::c_char;
    scratch =
        malloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if scratch.is_null() { return 12 as libc::c_int }
    retval = krb5_mk_error((*h).kdc_err_context, &mut errpkt, scratch);
    if retval != 0 { free(scratch as *mut libc::c_void); return retval }
    *out = scratch;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "2092:1"]
pub unsafe extern "C" fn reset_for_hangup(mut ctx: *mut libc::c_void) {
    let mut k: libc::c_int = 0;
    let mut h: *mut server_handle = ctx as *mut server_handle;
    k = 0 as libc::c_int;
    while k < (*h).kdc_numrealms {
        krb5_db_refresh_config((**(*h).kdc_realmlist.offset(k as
                                                                isize)).realm_context);
        k += 1
    };
}
