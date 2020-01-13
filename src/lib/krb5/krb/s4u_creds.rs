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
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
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
    /* *< Anonymous realm */
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
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
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
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
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
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
    /* checksum type */
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
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for auth data */
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
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Structure for transited encoding */
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
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Encrypted part of ticket. */
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
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Credentials structure including ticket, session key, and lifetime info. */
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
    /* * Pre-authentication data */
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
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
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
        #[no_mangle]
        #[c2rust::src_loc = "289:1"]
        pub fn krb5_is_referral_realm(r: *const krb5_data) -> krb5_boolean;
        /* *
 * Build an anonymous principal.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_PRINCSTR
 */
        #[no_mangle]
        #[c2rust::src_loc = "309:1"]
        pub fn krb5_anonymous_principal() -> krb5_const_principal;
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
        #[no_mangle]
        #[c2rust::src_loc = "911:1"]
        pub fn krb5_c_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype,
                                    key: *const krb5_keyblock,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_c_verify_checksum(context: krb5_context,
                                      key: *const krb5_keyblock,
                                      usage: krb5_keyusage,
                                      data: *const krb5_data,
                                      cksum: *const krb5_checksum,
                                      valid: *mut krb5_boolean)
         -> krb5_error_code;
        /* *
 * Test whether a checksum type is keyed.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is a keyed checksum type, @c FALSE otherwise.
 */
        #[no_mangle]
        #[c2rust::src_loc = "1080:1"]
        pub fn krb5_c_is_keyed_cksum(ctype: krb5_cksumtype) -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "2421:1"]
        pub fn krb5_cc_store_cred(context: krb5_context, cache: krb5_ccache,
                                  creds: *mut krb5_creds) -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "2457:1"]
        pub fn krb5_cc_retrieve_cred(context: krb5_context,
                                     cache: krb5_ccache, flags: krb5_flags,
                                     mcreds: *mut krb5_creds,
                                     creds: *mut krb5_creds)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3119:1"]
        pub fn krb5_get_credentials(context: krb5_context,
                                    options: krb5_flags, ccache: krb5_ccache,
                                    in_creds: *mut krb5_creds,
                                    out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3468:1"]
        pub fn krb5_parse_name_flags(context: krb5_context,
                                     name: *const libc::c_char,
                                     flags: libc::c_int,
                                     principal_out: *mut krb5_principal)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3650:1"]
        pub fn krb5_realm_compare(context: krb5_context,
                                  princ1: krb5_const_principal,
                                  princ2: krb5_const_principal)
         -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "3682:1"]
        pub fn krb5_principal_compare_any_realm(context: krb5_context,
                                                princ1: krb5_const_principal,
                                                princ2: krb5_const_principal)
         -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3995:1"]
        pub fn krb5_build_principal_ext(context: krb5_context,
                                        princ: *mut krb5_principal,
                                        rlen: libc::c_uint,
                                        realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4666:1"]
        pub fn krb5_free_creds(context: krb5_context, val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
pub mod k5_int_h {
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
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
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
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_checksum, krb5_principal, krb5_data,
                        krb5_context, krb5_pa_data, krb5_preauthtype,
                        krb5_error_code, krb5_ticket, krb5_cksumtype,
                        krb5_creds, krb5_address};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::{memcmp, strlen};
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
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "885:1"]
        pub fn k5_add_pa_data_from_data(list: *mut *mut *mut krb5_pa_data,
                                        pa_type: krb5_preauthtype,
                                        data: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_free_pa_s4u_x509_user(_: krb5_context,
                                          _: *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn encode_krb5_ticket(rep: *const krb5_ticket,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1481:1"]
        pub fn encode_krb5_pa_for_user(_: *const krb5_pa_for_user,
                                       _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1484:1"]
        pub fn encode_krb5_s4u_userid(_: *const krb5_s4u_userid,
                                      _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1487:1"]
        pub fn encode_krb5_pa_s4u_x509_user(_: *const krb5_pa_s4u_x509_user,
                                            _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1547:1"]
        pub fn encode_krb5_pa_pac_options(_: *const krb5_pa_pac_options,
                                          _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1675:1"]
        pub fn decode_krb5_pa_s4u_x509_user(_: *const krb5_data,
                                            _:
                                                *mut *mut krb5_pa_s4u_x509_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1732:1"]
        pub fn decode_krb5_pa_pac_options(_: *const krb5_data,
                                          _: *mut *mut krb5_pa_pac_options)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2068:1"]
        pub fn krb5int_c_mandatory_cksumtype(_: krb5_context, _: krb5_enctype,
                                             _: *mut krb5_cksumtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2105:1"]
        pub fn krb5_get_cred_via_tkt(_: krb5_context, _: *mut krb5_creds,
                                     _: krb5_flags,
                                     _: *const *mut krb5_address,
                                     _: *mut krb5_creds,
                                     _: *mut *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:28"]
pub mod int_proto_h {
    #[c2rust::src_loc = "89:1"]
    pub type k5_pacb_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_keyblock,
                                    _: *mut krb5_kdc_req,
                                    _: *mut libc::c_void) -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_keyblock,
                        krb5_kdc_req, krb5_data, krb5_principal, krb5_flags,
                        krb5_creds, krb5_address, krb5_pa_data,
                        krb5_principal_data};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn krb5int_tgtname(context: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn krb5int_construct_matching_creds(context: krb5_context,
                                                options: krb5_flags,
                                                in_creds: *mut krb5_creds,
                                                mcreds: *mut krb5_creds,
                                                fields: *mut krb5_flags)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn krb5_get_cred_via_tkt_ext(context: krb5_context,
                                         tkt: *mut krb5_creds,
                                         kdcoptions: krb5_flags,
                                         address: *const *mut krb5_address,
                                         in_padata: *mut *mut krb5_pa_data,
                                         in_cred: *mut krb5_creds,
                                         pacb_fn: k5_pacb_fn,
                                         pacb_data: *mut libc::c_void,
                                         out_padata:
                                             *mut *mut *mut krb5_pa_data,
                                         enc_padata:
                                             *mut *mut *mut krb5_pa_data,
                                         out_cred: *mut *mut krb5_creds,
                                         out_subkey: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        /*
 * Make AS requests with the canonicalize flag set, stopping when we get a
 * message indicating which realm the client principal is in.  Set *client_out
 * to a copy of client with the canonical realm.  If subject_cert is non-null,
 * include PA_S4U_X509_USER pa-data with the subject certificate each request.
 * (See [MS-SFU] 3.1.5.1.1.1 and 3.1.5.1.1.2.)
 */
        #[no_mangle]
        #[c2rust::src_loc = "304:1"]
        pub fn k5_identify_realm(context: krb5_context,
                                 client: krb5_principal,
                                 subject_cert: *const krb5_data,
                                 client_out: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
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
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
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
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_msgtype,
                       krb5_kvno, krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_creds, krb5_creds,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, krb5_ccache, _profile_t, _krb5_ccache,
                       krb5_is_referral_realm, krb5_anonymous_principal,
                       krb5_c_make_checksum, krb5_c_verify_checksum,
                       krb5_c_is_keyed_cksum, krb5_cc_store_cred,
                       krb5_cc_retrieve_cred, krb5_get_credentials,
                       krb5_parse_name_flags, krb5_unparse_name,
                       krb5_realm_compare, krb5_principal_compare,
                       krb5_principal_compare_any_realm, krb5_copy_principal,
                       krb5_build_principal_ext, krb5_free_principal,
                       krb5_free_creds, krb5_free_checksum_contents,
                       krb5_free_keyblock, krb5_free_data,
                       krb5_free_data_contents, krb5_free_unparsed_name,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_pa_s4u_x509_user,
                         _krb5_pa_s4u_x509_user, krb5_s4u_userid,
                         _krb5_s4u_userid, _krb5_pa_pac_options,
                         krb5_pa_pac_options, _krb5_pa_for_user,
                         krb5_pa_for_user, data_eq, data_eq_string, make_data,
                         empty_data, k5calloc, k5alloc, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_find_pa_data,
                         k5_add_pa_data_from_data, krb5int_copy_data_contents,
                         krb5_free_pa_s4u_x509_user, encode_krb5_ticket,
                         encode_krb5_pa_for_user, encode_krb5_s4u_userid,
                         encode_krb5_pa_s4u_x509_user,
                         encode_krb5_pa_pac_options,
                         decode_krb5_pa_s4u_x509_user,
                         decode_krb5_pa_pac_options,
                         krb5int_c_mandatory_cksumtype, krb5_get_cred_via_tkt,
                         krb5_free_pa_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::int_proto_h::{k5_pacb_fn, krb5int_tgtname,
                            krb5int_construct_matching_creds,
                            krb5_get_cred_via_tkt_ext, k5_identify_realm};
use self::stdlib_h::{malloc, calloc, free};
use self::libintl_h::dgettext;
use self::string_h::{strlen, memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/s4u_creds.c */
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
/* Convert ticket flags to necessary KDC options */
/*
 * Implements S4U2Self, by which a service can request a ticket to
 * itself on behalf of an arbitrary principal.
 */
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn s4u_identify_user(mut context: krb5_context,
                                       mut in_creds: *mut krb5_creds,
                                       mut subject_cert: *mut krb5_data,
                                       mut canon_user: *mut krb5_principal)
 -> krb5_error_code {
    let mut client: krb5_principal_data =
        krb5_principal_data{magic: 0,
                            realm:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            data: 0 as *mut krb5_data,
                            length: 0,
                            type_0: 0,};
    let mut empty_name: krb5_data = empty_data();
    *canon_user = 0 as krb5_principal;
    if (*in_creds).client.is_null() && subject_cert.is_null() {
        return 22 as libc::c_int
    }
    if !(*in_creds).client.is_null() &&
           (*(*in_creds).client).type_0 != 10 as libc::c_int {
        let mut anonymous: libc::c_int = 0;
        anonymous =
            krb5_principal_compare(context,
                                   (*in_creds).client as krb5_const_principal,
                                   krb5_anonymous_principal()) as libc::c_int;
        return krb5_copy_principal(context,
                                   if anonymous != 0 {
                                       (*in_creds).server
                                   } else { (*in_creds).client } as
                                       krb5_const_principal, canon_user)
    }
    if !(*in_creds).client.is_null() {
        client = *(*in_creds).client;
        client.realm = (*(*in_creds).server).realm;
        /* Don't send subject_cert if we have an enterprise principal. */
        return k5_identify_realm(context, &mut client, 0 as *const krb5_data,
                                 canon_user)
    }
    client.magic = -(1760647423 as libc::c_long) as krb5_magic;
    client.realm = (*(*in_creds).server).realm;
    /*
     * Windows clients send the certificate subject as the client name.
     * However, Windows KDC seem to be happy with an empty string as long as
     * the name-type is NT-X500-PRINCIPAL.
     */
    client.data = &mut empty_name;
    client.length = 1 as libc::c_int;
    client.type_0 = 6 as libc::c_int;
    return k5_identify_realm(context, &mut client, subject_cert, canon_user);
}
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn make_pa_for_user_checksum(mut context: krb5_context,
                                               mut key: *mut krb5_keyblock,
                                               mut req: *mut krb5_pa_for_user,
                                               mut cksum: *mut krb5_checksum)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    data.length = 4 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < (*(*req).user).length {
        data.length =
            data.length.wrapping_add((*(*(*req).user).data.offset(i as
                                                                      isize)).length);
        i += 1
    }
    data.length = data.length.wrapping_add((*(*req).user).realm.length);
    data.length = data.length.wrapping_add((*req).auth_package.length);
    data.data = malloc(data.length as libc::c_ulong) as *mut libc::c_char;
    p = data.data;
    if data.data.is_null() { return 12 as libc::c_int }
    *p.offset(0 as libc::c_int as isize) =
        ((*(*req).user).type_0 >> 0 as libc::c_int & 0xff as libc::c_int) as
            libc::c_char;
    *p.offset(1 as libc::c_int as isize) =
        ((*(*req).user).type_0 >> 8 as libc::c_int & 0xff as libc::c_int) as
            libc::c_char;
    *p.offset(2 as libc::c_int as isize) =
        ((*(*req).user).type_0 >> 16 as libc::c_int & 0xff as libc::c_int) as
            libc::c_char;
    *p.offset(3 as libc::c_int as isize) =
        ((*(*req).user).type_0 >> 24 as libc::c_int & 0xff as libc::c_int) as
            libc::c_char;
    p = p.offset(4 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*(*req).user).length {
        if (*(*(*req).user).data.offset(i as isize)).length >
               0 as libc::c_int as libc::c_uint {
            memcpy(p as *mut libc::c_void,
                   (*(*(*req).user).data.offset(i as isize)).data as
                       *const libc::c_void,
                   (*(*(*req).user).data.offset(i as isize)).length as
                       libc::c_ulong);
        }
        p =
            p.offset((*(*(*req).user).data.offset(i as isize)).length as
                         isize);
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
    /* Per spec, use hmac-md5 checksum regardless of key type. */
    code =
        krb5_c_make_checksum(context, -(138 as libc::c_int), key,
                             17 as libc::c_int, &mut data, cksum);
    free(data.data as *mut libc::c_void);
    return code;
}
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn build_pa_for_user(mut context: krb5_context,
                                       mut tgt: *mut krb5_creds,
                                       mut userid: *mut krb5_s4u_userid,
                                       mut out_padata: *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut padata: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut for_user: krb5_pa_for_user =
        krb5_pa_for_user{user: 0 as *mut krb5_principal_data,
                         cksum:
                             krb5_checksum{magic: 0,
                                           checksum_type: 0,
                                           length: 0,
                                           contents: 0 as *mut krb5_octet,},
                         auth_package:
                             krb5_data{magic: 0,
                                       length: 0,
                                       data: 0 as *mut libc::c_char,},};
    let mut for_user_data: *mut krb5_data = 0 as *mut krb5_data;
    let mut package: [libc::c_char; 9] =
        *::std::mem::transmute::<&[u8; 9],
                                 &mut [libc::c_char; 9]>(b"Kerberos\x00");
    if (*userid).user.is_null() { return 22 as libc::c_int }
    memset(&mut for_user as *mut krb5_pa_for_user as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_pa_for_user>() as libc::c_ulong);
    for_user.user = (*userid).user;
    for_user.auth_package.data = package.as_mut_ptr();
    for_user.auth_package.length =
        (::std::mem::size_of::<[libc::c_char; 9]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_uint;
    code =
        make_pa_for_user_checksum(context, &mut (*tgt).keyblock,
                                  &mut for_user, &mut for_user.cksum);
    if !(code != 0 as libc::c_int) {
        code = encode_krb5_pa_for_user(&mut for_user, &mut for_user_data);
        if !(code != 0 as libc::c_int) {
            padata =
                malloc(::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong)
                    as *mut krb5_pa_data;
            if padata.is_null() {
                code = 12 as libc::c_int
            } else {
                (*padata).magic = -(1760647406 as libc::c_long) as krb5_magic;
                (*padata).pa_type = 129 as libc::c_int;
                (*padata).length = (*for_user_data).length;
                (*padata).contents = (*for_user_data).data as *mut krb5_octet;
                free(for_user_data as *mut libc::c_void);
                for_user_data = 0 as *mut krb5_data;
                *out_padata = padata
            }
        }
    }
    if !for_user.cksum.contents.is_null() {
        krb5_free_checksum_contents(context, &mut for_user.cksum);
    }
    krb5_free_data(context, for_user_data);
    return code;
}
/*
 * This function is invoked by krb5int_make_tgs_request_ext() just before the
 * request is encoded; it gives us access to the nonce and subkey without
 * requiring them to be generated by the caller.
 */
#[c2rust::src_loc = "197:1"]
unsafe extern "C" fn build_pa_s4u_x509_user(mut context: krb5_context,
                                            mut subkey: *mut krb5_keyblock,
                                            mut tgsreq: *mut krb5_kdc_req,
                                            mut gcvt_data: *mut libc::c_void)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut s4u_user: *mut krb5_pa_s4u_x509_user =
        gcvt_data as *mut krb5_pa_s4u_x509_user;
    let mut data: *mut krb5_data = 0 as *mut krb5_data;
    let mut cksumtype: krb5_cksumtype = 0;
    let mut i: libc::c_int = 0;
    if (*s4u_user).cksum.contents.is_null() {
    } else {
        __assert_fail(b"s4u_user->cksum.contents == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"s4u_creds.c\x00" as *const u8 as *const libc::c_char,
                      209 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 94],
                                                &[libc::c_char; 94]>(b"krb5_error_code build_pa_s4u_x509_user(krb5_context, krb5_keyblock *, krb5_kdc_req *, void *)\x00")).as_ptr());
    }
    (*s4u_user).user_id.nonce = (*tgsreq).nonce;
    code = encode_krb5_s4u_userid(&mut (*s4u_user).user_id, &mut data);
    if !(code != 0 as libc::c_int) {
        /* [MS-SFU] 2.2.2: unusual to say the least, but enc_padata secures it */
        if (*subkey).enctype == 0x17 as libc::c_int ||
               (*subkey).enctype == 0x18 as libc::c_int {
            cksumtype = 0x2 as libc::c_int
        } else {
            code =
                krb5int_c_mandatory_cksumtype(context, (*subkey).enctype,
                                              &mut cksumtype)
        }
        if !(code != 0 as libc::c_int) {
            code =
                krb5_c_make_checksum(context, cksumtype, subkey,
                                     26 as libc::c_int, data,
                                     &mut (*s4u_user).cksum);
            if !(code != 0 as libc::c_int) {
                krb5_free_data(context, data);
                data = 0 as *mut krb5_data;
                code = encode_krb5_pa_s4u_x509_user(s4u_user, &mut data);
                if !(code != 0 as libc::c_int) {
                    /* Find the empty PA-S4U-X509-USER element placed in the TGS request padata
     * by krb5_get_self_cred_from_kdc() and replace it with the encoding. */
                    if !(*tgsreq).padata.is_null() {
                    } else {
                        __assert_fail(b"tgsreq->padata != NULL\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"s4u_creds.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      243 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 94],
                                                                &[libc::c_char; 94]>(b"krb5_error_code build_pa_s4u_x509_user(krb5_context, krb5_keyblock *, krb5_kdc_req *, void *)\x00")).as_ptr());
                    }
                    i = 0 as libc::c_int;
                    while !(*(*tgsreq).padata.offset(i as isize)).is_null() {
                        if (**(*tgsreq).padata.offset(i as isize)).pa_type ==
                               130 as libc::c_int {
                            break ;
                        }
                        i += 1
                    }
                    if !(*(*tgsreq).padata.offset(i as isize)).is_null() {
                    } else {
                        __assert_fail(b"tgsreq->padata[i] != NULL\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"s4u_creds.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      248 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 94],
                                                                &[libc::c_char; 94]>(b"krb5_error_code build_pa_s4u_x509_user(krb5_context, krb5_keyblock *, krb5_kdc_req *, void *)\x00")).as_ptr());
                    }
                    free((**(*tgsreq).padata.offset(i as isize)).contents as
                             *mut libc::c_void);
                    (**(*tgsreq).padata.offset(i as isize)).length =
                        (*data).length;
                    let ref mut fresh0 =
                        (**(*tgsreq).padata.offset(i as isize)).contents;
                    *fresh0 = (*data).data as *mut krb5_octet;
                    free(data as *mut libc::c_void);
                    data = 0 as *mut krb5_data
                }
            }
        }
    }
    if code != 0 as libc::c_int && !(*s4u_user).cksum.contents.is_null() {
        krb5_free_checksum_contents(context, &mut (*s4u_user).cksum);
        (*s4u_user).cksum.contents = 0 as *mut krb5_octet
    }
    krb5_free_data(context, data);
    return code;
}
/*
 * Validate the S4U2Self padata in the KDC reply.  If update_req_user is true
 * and the KDC sent S4U-X509-USER padata, replace req_s4u_user->user_id.user
 * with the checksum-protected client name from the KDC.  If update_req_user is
 * false, verify that the client name has not changed.
 */
#[c2rust::src_loc = "271:1"]
unsafe extern "C" fn verify_s4u2self_reply(mut context: krb5_context,
                                           mut subkey: *mut krb5_keyblock,
                                           mut req_s4u_user:
                                               *mut krb5_pa_s4u_x509_user,
                                           mut rep_padata:
                                               *mut *mut krb5_pa_data,
                                           mut enc_padata:
                                               *mut *mut krb5_pa_data,
                                           mut update_req_user: krb5_boolean)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut rep_s4u_padata: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut enc_s4u_padata: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut rep_s4u_user: *mut krb5_pa_s4u_x509_user =
        0 as *mut krb5_pa_s4u_x509_user;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut datap: *mut krb5_data = 0 as *mut krb5_data;
    let mut usage: krb5_keyusage = 0;
    let mut valid: krb5_boolean = 0;
    let mut not_newer: krb5_boolean = 0;
    if !req_s4u_user.is_null() {
    } else {
        __assert_fail(b"req_s4u_user != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"s4u_creds.c\x00" as *const u8 as *const libc::c_char,
                      287 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 142],
                                                &[libc::c_char; 142]>(b"krb5_error_code verify_s4u2self_reply(krb5_context, krb5_keyblock *, krb5_pa_s4u_x509_user *, krb5_pa_data **, krb5_pa_data **, krb5_boolean)\x00")).as_ptr());
    }
    match (*subkey).enctype {
        16 | 6 | 23 | 24 => { not_newer = 1 as libc::c_int as krb5_boolean }
        _ => { not_newer = 0 as libc::c_int as krb5_boolean }
    }
    enc_s4u_padata =
        krb5int_find_pa_data(context, enc_padata, 130 as libc::c_int);
    /* XXX this will break newer enctypes with a MIT 1.7 KDC */
    rep_s4u_padata =
        krb5int_find_pa_data(context, rep_padata, 130 as libc::c_int);
    if rep_s4u_padata.is_null() {
        if not_newer == 0 as libc::c_int as libc::c_uint ||
               !enc_s4u_padata.is_null() {
            return -(1765328237 as libc::c_long) as krb5_error_code
        } else { return 0 as libc::c_int }
    }
    data.length = (*rep_s4u_padata).length;
    data.data = (*rep_s4u_padata).contents as *mut libc::c_char;
    code = decode_krb5_pa_s4u_x509_user(&mut data, &mut rep_s4u_user);
    if !(code != 0 as libc::c_int) {
        if (*rep_s4u_user).user_id.nonce != (*req_s4u_user).user_id.nonce {
            code = -(1765328237 as libc::c_long) as krb5_error_code
        } else {
            code =
                encode_krb5_s4u_userid(&mut (*rep_s4u_user).user_id,
                                       &mut datap);
            if !(code != 0 as libc::c_int) {
                if (*rep_s4u_user).user_id.options & 0x20000000 as libc::c_int
                       != 0 {
                    usage = 27 as libc::c_int
                } else { usage = 26 as libc::c_int }
                code =
                    krb5_c_verify_checksum(context, subkey, usage, datap,
                                           &mut (*rep_s4u_user).cksum,
                                           &mut valid);
                if !(code != 0 as libc::c_int) {
                    if valid == 0 as libc::c_int as libc::c_uint {
                        code =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    } else if (*rep_s4u_user).user_id.user.is_null() ||
                                  (*(*rep_s4u_user).user_id.user).length ==
                                      0 as libc::c_int {
                        code =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    } else {
                        if update_req_user != 0 {
                            krb5_free_principal(context,
                                                (*req_s4u_user).user_id.user);
                            code =
                                krb5_copy_principal(context,
                                                    (*rep_s4u_user).user_id.user
                                                        as
                                                        krb5_const_principal,
                                                    &mut (*req_s4u_user).user_id.user);
                            if code != 0 as libc::c_int {
                                current_block = 11734805974515833756;
                            } else { current_block = 572715077006366937; }
                        } else if krb5_principal_compare(context,
                                                         (*rep_s4u_user).user_id.user
                                                             as
                                                             krb5_const_principal,
                                                         (*req_s4u_user).user_id.user
                                                             as
                                                             krb5_const_principal)
                                      == 0 {
                            code =
                                -(1765328237 as libc::c_long) as
                                    krb5_error_code;
                            current_block = 11734805974515833756;
                        } else { current_block = 572715077006366937; }
                        match current_block {
                            11734805974515833756 => { }
                            _ =>
                            /*
     * KDCs that support KRB5_S4U_OPTS_USE_REPLY_KEY_USAGE also return
     * S4U enc_padata for older (pre-AES) encryption types only.
     */
                            {
                                if not_newer != 0 {
                                    if enc_s4u_padata.is_null() {
                                        if (*rep_s4u_user).user_id.options &
                                               0x20000000 as libc::c_int != 0
                                           {
                                            code =
                                                -(1765328237 as libc::c_long)
                                                    as krb5_error_code
                                        }
                                    } else if (*enc_s4u_padata).length !=
                                                  (*req_s4u_user).cksum.length.wrapping_add((*rep_s4u_user).cksum.length)
                                     {
                                        code =
                                            -(1765328237 as libc::c_long) as
                                                krb5_error_code
                                    } else if memcmp((*enc_s4u_padata).contents
                                                         as
                                                         *const libc::c_void,
                                                     (*req_s4u_user).cksum.contents
                                                         as
                                                         *const libc::c_void,
                                                     (*req_s4u_user).cksum.length
                                                         as libc::c_ulong) !=
                                                  0 ||
                                                  memcmp(&mut *(*enc_s4u_padata).contents.offset((*req_s4u_user).cksum.length
                                                                                                     as
                                                                                                     isize)
                                                             as
                                                             *mut krb5_octet
                                                             as
                                                             *const libc::c_void,
                                                         (*rep_s4u_user).cksum.contents
                                                             as
                                                             *const libc::c_void,
                                                         (*rep_s4u_user).cksum.length
                                                             as libc::c_ulong)
                                                      != 0 {
                                        code =
                                            -(1765328237 as libc::c_long) as
                                                krb5_error_code
                                    }
                                } else if krb5_c_is_keyed_cksum((*rep_s4u_user).cksum.checksum_type)
                                              == 0 {
                                    code =
                                        -(1765328334 as libc::c_long) as
                                            krb5_error_code
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_pa_s4u_x509_user(context, rep_s4u_user);
    krb5_free_data(context, datap);
    return code;
}
/* Unparse princ and re-parse it as an enterprise principal. */
#[c2rust::src_loc = "404:1"]
unsafe extern "C" fn convert_to_enterprise(mut context: krb5_context,
                                           mut princ: krb5_principal,
                                           mut eprinc_out:
                                               *mut krb5_principal)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    *eprinc_out = 0 as krb5_principal;
    code =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut str);
    if code != 0 as libc::c_int { return code }
    code =
        krb5_parse_name_flags(context, str,
                              0x4 as libc::c_int | 0x8 as libc::c_int,
                              eprinc_out);
    krb5_free_unparsed_name(context, str);
    return code;
}
#[c2rust::src_loc = "423:1"]
unsafe extern "C" fn krb5_get_self_cred_from_kdc(mut context: krb5_context,
                                                 mut options: krb5_flags,
                                                 mut ccache: krb5_ccache,
                                                 mut in_creds:
                                                     *mut krb5_creds,
                                                 mut subject_cert:
                                                     *mut krb5_data,
                                                 mut user_realm:
                                                     *mut krb5_data,
                                                 mut out_creds:
                                                     *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut tgs: krb5_principal = 0 as krb5_principal;
    let mut eprinc: krb5_principal = 0 as krb5_principal;
    let mut sprinc: krb5_principal_data =
        krb5_principal_data{magic: 0,
                            realm:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            data: 0 as *mut krb5_data,
                            length: 0,
                            type_0: 0,};
    let mut tgtq: krb5_creds =
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
    let mut s4u_creds: krb5_creds =
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
    let mut tgt: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut tgtptr: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut referral_tgts: [*mut krb5_creds; 10] = [0 as *mut krb5_creds; 10];
    let mut s4u_user: krb5_pa_s4u_x509_user =
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
    let mut referral_count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut kdcopt: krb5_flags = 0;
    memset(&mut tgtq as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    memset(referral_tgts.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut krb5_creds; 10]>() as libc::c_ulong);
    *out_creds = 0 as *mut krb5_creds;
    memset(&mut s4u_user as *mut krb5_pa_s4u_x509_user as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_pa_s4u_x509_user>() as libc::c_ulong);
    if !(*in_creds).client.is_null() &&
           (*(*in_creds).client).length > 0 as libc::c_int {
        if (*(*in_creds).client).type_0 == 10 as libc::c_int {
            code =
                krb5_build_principal_ext(context,
                                         &mut s4u_user.user_id.user as
                                             *mut krb5_principal,
                                         (*user_realm).length,
                                         (*user_realm).data,
                                         (*(*(*in_creds).client).data.offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)).length,
                                         (*(*(*in_creds).client).data.offset(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize)).data,
                                         0 as libc::c_int);
            if code != 0 as libc::c_int {
                current_block = 3534341321926330228;
            } else {
                (*s4u_user.user_id.user).type_0 = 10 as libc::c_int;
                current_block = 7175849428784450219;
            }
        } else {
            code =
                krb5_copy_principal(context,
                                    (*in_creds).client as
                                        krb5_const_principal,
                                    &mut s4u_user.user_id.user);
            if code != 0 as libc::c_int {
                current_block = 3534341321926330228;
            } else { current_block = 7175849428784450219; }
        }
    } else {
        code =
            krb5_build_principal_ext(context,
                                     &mut s4u_user.user_id.user as
                                         *mut krb5_principal,
                                     (*user_realm).length, (*user_realm).data,
                                     0 as libc::c_int);
        if code != 0 as libc::c_int {
            current_block = 3534341321926330228;
        } else { current_block = 7175849428784450219; }
    }
    match current_block {
        7175849428784450219 => {
            if !subject_cert.is_null() {
                s4u_user.user_id.subject_cert = *subject_cert
            }
            s4u_user.user_id.options = 0x20000000 as libc::c_int;
            /* First, acquire a TGT to the user's realm. */
            code =
                krb5int_tgtname(context, user_realm,
                                &mut (*(*in_creds).server).realm, &mut tgs);
            if !(code != 0 as libc::c_int) {
                tgtq.client = (*in_creds).server;
                tgtq.server = tgs;
                code =
                    krb5_get_credentials(context, options, ccache, &mut tgtq,
                                         &mut tgt);
                if !(code != 0 as libc::c_int) {
                    tgtptr = tgt;
                    /* Convert the server principal to an enterprise principal, for use with
     * foreign realms. */
                    code =
                        convert_to_enterprise(context, (*in_creds).server,
                                              &mut eprinc);
                    if !(code != 0 as libc::c_int) {
                        /* Make a shallow copy of in_creds with client pointing to the server
     * principal.  We will set s4u_creds.server for each request. */
                        s4u_creds = *in_creds;
                        s4u_creds.client = (*in_creds).server;
                        /* Then, walk back the referral path to S4U2Self for user */
                        kdcopt = 0 as libc::c_int;
                        if options & 4 as libc::c_int != 0 {
                            kdcopt |= 0x10000 as libc::c_int
                        }
                        if options & 16 as libc::c_int != 0 {
                            kdcopt |= 0x40000000 as libc::c_int
                        }
                        if options & 32 as libc::c_int != 0 {
                            kdcopt |= 0x20 as libc::c_int
                        }
                        referral_count = 0 as libc::c_int;
                        's_195:
                            while referral_count < 10 as libc::c_int {
                                let mut in_padata: *mut *mut krb5_pa_data =
                                    0 as *mut *mut krb5_pa_data;
                                let mut out_padata: *mut *mut krb5_pa_data =
                                    0 as *mut *mut krb5_pa_data;
                                let mut enc_padata: *mut *mut krb5_pa_data =
                                    0 as *mut *mut krb5_pa_data;
                                let mut subkey: *mut krb5_keyblock =
                                    0 as *mut krb5_keyblock;
                                in_padata =
                                    k5calloc(3 as libc::c_int as size_t,
                                             ::std::mem::size_of::<*mut krb5_pa_data>()
                                                 as libc::c_ulong, &mut code)
                                        as *mut *mut krb5_pa_data;
                                if in_padata.is_null() { break ; }
                                let ref mut fresh1 =
                                    *in_padata.offset(0 as libc::c_int as
                                                          isize);
                                *fresh1 =
                                    k5alloc(::std::mem::size_of::<krb5_pa_data>()
                                                as libc::c_ulong, &mut code)
                                        as *mut krb5_pa_data;
                                if (*in_padata.offset(0 as libc::c_int as
                                                          isize)).is_null() {
                                    krb5_free_pa_data(context, in_padata);
                                    break ;
                                } else {
                                    (**in_padata.offset(0 as libc::c_int as
                                                            isize)).magic =
                                        -(1760647406 as libc::c_long) as
                                            krb5_magic;
                                    (**in_padata.offset(0 as libc::c_int as
                                                            isize)).pa_type =
                                        130 as libc::c_int;
                                    (**in_padata.offset(0 as libc::c_int as
                                                            isize)).length =
                                        0 as libc::c_int as libc::c_uint;
                                    let ref mut fresh2 =
                                        (**in_padata.offset(0 as libc::c_int
                                                                as
                                                                isize)).contents;
                                    *fresh2 = 0 as *mut krb5_octet;
                                    if !s4u_user.user_id.user.is_null() &&
                                           (*s4u_user.user_id.user).length !=
                                               0 {
                                        code =
                                            build_pa_for_user(context, tgtptr,
                                                              &mut s4u_user.user_id,
                                                              &mut *in_padata.offset(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         isize));
                                        if code != 0 as libc::c_int {
                                            krb5_free_pa_data(context,
                                                              in_padata);
                                            break ;
                                        }
                                    }
                                    if data_eq(*(*(*tgtptr).server).data.offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                               (*(*in_creds).server).realm) !=
                                           0 {
                                        /* When asking the server realm, use the real principal. */
                                        s4u_creds.server = (*in_creds).server
                                    } else {
                                        /* When asking a foreign realm, use the enterprise principal, with
             * the realm set to the TGS realm. */
                                        sprinc = *eprinc;
                                        sprinc.realm =
                                            *(*(*tgtptr).server).data.offset(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 isize);
                                        s4u_creds.server = &mut sprinc
                                    }
                                    code =
                                        krb5_get_cred_via_tkt_ext(context,
                                                                  tgtptr,
                                                                  0x10000 as
                                                                      libc::c_int
                                                                      |
                                                                      (*tgtptr).ticket_flags
                                                                          &
                                                                          0x54800000
                                                                              as
                                                                              libc::c_int
                                                                      |
                                                                      kdcopt,
                                                                  (*tgtptr).addresses,
                                                                  in_padata,
                                                                  &mut s4u_creds,
                                                                  Some(build_pa_s4u_x509_user
                                                                           as
                                                                           unsafe extern "C" fn(_:
                                                                                                    krb5_context,
                                                                                                _:
                                                                                                    *mut krb5_keyblock,
                                                                                                _:
                                                                                                    *mut krb5_kdc_req,
                                                                                                _:
                                                                                                    *mut libc::c_void)
                                                                               ->
                                                                                   krb5_error_code),
                                                                  &mut s4u_user
                                                                      as
                                                                      *mut krb5_pa_s4u_x509_user
                                                                      as
                                                                      *mut libc::c_void,
                                                                  &mut out_padata,
                                                                  &mut enc_padata,
                                                                  out_creds,
                                                                  &mut subkey);
                                    if code != 0 as libc::c_int {
                                        krb5_free_checksum_contents(context,
                                                                    &mut s4u_user.cksum);
                                        krb5_free_pa_data(context, in_padata);
                                        break ;
                                    } else {
                                        /* Update s4u_user.user_id.user if this is the initial request to the
         * client realm; otherwise verify that it doesn't change. */
                                        code =
                                            verify_s4u2self_reply(context,
                                                                  subkey,
                                                                  &mut s4u_user,
                                                                  out_padata,
                                                                  enc_padata,
                                                                  (referral_count
                                                                       ==
                                                                       0 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      krb5_boolean);
                                        krb5_free_checksum_contents(context,
                                                                    &mut s4u_user.cksum);
                                        krb5_free_pa_data(context, in_padata);
                                        krb5_free_pa_data(context,
                                                          out_padata);
                                        krb5_free_pa_data(context,
                                                          enc_padata);
                                        krb5_free_keyblock(context, subkey);
                                        if code != 0 as libc::c_int {
                                            break ;
                                        }
                                        /* Only include a cert in the initial request to the client realm. */
                                        s4u_user.user_id.subject_cert =
                                            empty_data();
                                        if krb5_principal_compare(context,
                                                                  (*in_creds).server
                                                                      as
                                                                      krb5_const_principal,
                                                                  (**out_creds).server
                                                                      as
                                                                      krb5_const_principal)
                                               != 0 {
                                            /* Verify that the unprotected client name in the reply matches the
             * checksum-protected one from the client realm's KDC padata. */
                                            if krb5_principal_compare(context,
                                                                      (**out_creds).client
                                                                          as
                                                                          krb5_const_principal,
                                                                      s4u_user.user_id.user
                                                                          as
                                                                          krb5_const_principal)
                                                   == 0 {
                                                code =
                                                    -(1765328237 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            } /* XXX */
                                            break ;
                                        } else if (*(**out_creds).server).length
                                                      == 2 as libc::c_int &&
                                                      data_eq_string(*(*(**out_creds).server).data.offset(0
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize),
                                                                     b"krbtgt\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char)
                                                          != 0 {
                                            let mut r1: *mut krb5_data =
                                                &mut *(*(*tgtptr).server).data.offset(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          isize)
                                                    as *mut krb5_data;
                                            let mut r2: *mut krb5_data =
                                                &mut *(*(**out_creds).server).data.offset(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize)
                                                    as *mut krb5_data;
                                            if data_eq(*r1, *r2) != 0 {
                                                krb5_free_creds(context,
                                                                *out_creds);
                                                *out_creds =
                                                    0 as *mut krb5_creds;
                                                code =
                                                    -(1765328167 as
                                                          libc::c_long) as
                                                        krb5_error_code;
                                                break ;
                                            } else {
                                                i = 0 as libc::c_int;
                                                while i < referral_count {
                                                    if krb5_principal_compare(context,
                                                                              (**out_creds).server
                                                                                  as
                                                                                  krb5_const_principal,
                                                                              (*referral_tgts[i
                                                                                                  as
                                                                                                  usize]).server
                                                                                  as
                                                                                  krb5_const_principal)
                                                           != 0 {
                                                        code =
                                                            -(1765328228 as
                                                                  libc::c_long)
                                                                as
                                                                krb5_error_code;
                                                        break 's_195 ;
                                                    } else { i += 1 }
                                                }
                                                tgtptr = *out_creds;
                                                referral_tgts[referral_count
                                                                  as usize] =
                                                    *out_creds;
                                                *out_creds =
                                                    0 as *mut krb5_creds;
                                                referral_count += 1
                                            }
                                        } else {
                                            krb5_free_creds(context,
                                                            *out_creds);
                                            *out_creds = 0 as *mut krb5_creds;
                                            code =
                                                -(1765328240 as libc::c_long)
                                                    as krb5_error_code;
                                            break ;
                                        }
                                    }
                                }
                            }
                    }
                }
            }
        }
        _ => { }
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if !referral_tgts[i as usize].is_null() {
            krb5_free_creds(context, referral_tgts[i as usize]);
        }
        i += 1
    }
    krb5_free_principal(context, tgs);
    krb5_free_principal(context, eprinc);
    krb5_free_creds(context, tgt);
    krb5_free_principal(context, s4u_user.user_id.user);
    krb5_free_checksum_contents(context, &mut s4u_user.cksum);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "641:1"]
pub unsafe extern "C" fn krb5_get_credentials_for_user(mut context:
                                                           krb5_context,
                                                       mut options:
                                                           krb5_flags,
                                                       mut ccache:
                                                           krb5_ccache,
                                                       mut in_creds:
                                                           *mut krb5_creds,
                                                       mut subject_cert:
                                                           *mut krb5_data,
                                                       mut out_creds:
                                                           *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut realm: krb5_principal = 0 as krb5_principal;
    *out_creds = 0 as *mut krb5_creds;
    if options & 64 as libc::c_int != 0 {
        code = 22 as libc::c_int
    } else {
        if !(*in_creds).client.is_null() {
            /* Uncanonicalised check */
            code =
                krb5_get_credentials(context, options | 2 as libc::c_int,
                                     ccache, in_creds, out_creds);
            if code as libc::c_long != -(1765328243 as libc::c_long) &&
                   code as libc::c_long != -(1765328184 as libc::c_long) {
                current_block = 1208317026563730058;
            } else if options & 2 as libc::c_int != 0 &&
                          options & 4 as libc::c_int == 0 {
                current_block = 1208317026563730058;
            } else { current_block = 3276175668257526147; }
        } else { current_block = 3276175668257526147; }
        match current_block {
            1208317026563730058 => { }
            _ => {
                code =
                    s4u_identify_user(context, in_creds, subject_cert,
                                      &mut realm);
                if !(code != 0 as libc::c_int) {
                    if !(*in_creds).client.is_null() &&
                           (*(*in_creds).client).type_0 == 10 as libc::c_int {
                        /* Post-canonicalisation check for enterprise principals */
                        let mut mcreds: krb5_creds = *in_creds;
                        mcreds.client = realm;
                        code =
                            krb5_get_credentials(context,
                                                 options | 2 as libc::c_int,
                                                 ccache, &mut mcreds,
                                                 out_creds);
                        if code as libc::c_long !=
                               -(1765328243 as libc::c_long) &&
                               code as libc::c_long !=
                                   -(1765328184 as libc::c_long) ||
                               options & 2 as libc::c_int != 0 {
                            current_block = 1208317026563730058;
                        } else { current_block = 8831408221741692167; }
                    } else { current_block = 8831408221741692167; }
                    match current_block {
                        1208317026563730058 => { }
                        _ => {
                            code =
                                krb5_get_self_cred_from_kdc(context, options,
                                                            ccache, in_creds,
                                                            subject_cert,
                                                            &mut (*realm).realm,
                                                            out_creds);
                            if !(code != 0 as libc::c_int) {
                                if !(*out_creds).is_null() {
                                } else {
                                    __assert_fail(b"*out_creds != NULL\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  b"s4u_creds.c\x00" as
                                                      *const u8 as
                                                      *const libc::c_char,
                                                  689 as libc::c_int as
                                                      libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 127],
                                                                            &[libc::c_char; 127]>(b"krb5_error_code krb5_get_credentials_for_user(krb5_context, krb5_flags, krb5_ccache, krb5_creds *, krb5_data *, krb5_creds **)\x00")).as_ptr());
                                }
                                /* If we canonicalized the client name or discovered it using subject_cert,
     * check if we had cached credentials and return them if found. */
                                if (*in_creds).client.is_null() ||
                                       krb5_principal_compare(context,
                                                              (*in_creds).client
                                                                  as
                                                                  krb5_const_principal,
                                                              (**out_creds).client
                                                                  as
                                                                  krb5_const_principal)
                                           == 0 {
                                    let mut old_creds: *mut krb5_creds =
                                        0 as *mut krb5_creds;
                                    let mut mcreds_0: krb5_creds = *in_creds;
                                    mcreds_0.client = (**out_creds).client;
                                    code =
                                        krb5_get_credentials(context,
                                                             options |
                                                                 2 as
                                                                     libc::c_int,
                                                             ccache,
                                                             &mut mcreds_0,
                                                             &mut old_creds);
                                    if code == 0 as libc::c_int {
                                        krb5_free_creds(context, *out_creds);
                                        *out_creds = old_creds;
                                        options |= 8 as libc::c_int;
                                        current_block = 1608152415753874203;
                                    } else if code as libc::c_long !=
                                                  -(1765328243 as
                                                        libc::c_long) &&
                                                  code as libc::c_long !=
                                                      -(1765328184 as
                                                            libc::c_long) {
                                        current_block = 1208317026563730058;
                                    } else {
                                        current_block = 1608152415753874203;
                                    }
                                    match current_block {
                                        1208317026563730058 => { }
                                        _ => {
                                            code = 0 as libc::c_int;
                                            current_block =
                                                11932355480408055363;
                                        }
                                    }
                                } else {
                                    current_block = 11932355480408055363;
                                }
                                match current_block {
                                    1208317026563730058 => { }
                                    _ => {
                                        if options & 8 as libc::c_int ==
                                               0 as libc::c_int {
                                            code =
                                                krb5_cc_store_cred(context,
                                                                   ccache,
                                                                   *out_creds);
                                            (code) != 0 as libc::c_int;
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
    if code != 0 as libc::c_int && !(*out_creds).is_null() {
        krb5_free_creds(context, *out_creds);
        *out_creds = 0 as *mut krb5_creds
    }
    krb5_free_principal(context, realm);
    return code;
}
#[c2rust::src_loc = "728:1"]
unsafe extern "C" fn check_rbcd_support(mut context: krb5_context,
                                        mut padata: *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut pac_options: *mut krb5_pa_pac_options =
        0 as *mut krb5_pa_pac_options;
    let mut der_pac_options: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    pa = krb5int_find_pa_data(context, padata, 167 as libc::c_int);
    if pa.is_null() {
        return -(1765328368 as libc::c_long) as krb5_error_code
    }
    der_pac_options =
        make_data((*pa).contents as *mut libc::c_void, (*pa).length);
    code = decode_krb5_pa_pac_options(&mut der_pac_options, &mut pac_options);
    if code != 0 { return code }
    if (*pac_options).options & 0x10000000 as libc::c_int == 0 {
        code = -(1765328368 as libc::c_long) as krb5_error_code
    }
    free(pac_options as *mut libc::c_void);
    return code;
}
#[c2rust::src_loc = "752:1"]
unsafe extern "C" fn add_rbcd_padata(mut context: krb5_context,
                                     mut in_padata:
                                         *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut pac_options: krb5_pa_pac_options =
        krb5_pa_pac_options{options: 0,};
    let mut der_pac_options: *mut krb5_data = 0 as *mut krb5_data;
    memset(&mut pac_options as *mut krb5_pa_pac_options as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_pa_pac_options>() as libc::c_ulong);
    pac_options.options |= 0x10000000 as libc::c_int;
    code = encode_krb5_pa_pac_options(&mut pac_options, &mut der_pac_options);
    if code != 0 { return code }
    code =
        k5_add_pa_data_from_data(in_padata, 167 as libc::c_int,
                                 der_pac_options);
    krb5_free_data(context, der_pac_options);
    return code;
}
/* Set *tgt_out to a local TGT for the client realm retrieved from ccache. */
#[c2rust::src_loc = "773:1"]
unsafe extern "C" fn get_client_tgt(mut context: krb5_context,
                                    mut options: krb5_flags,
                                    mut ccache: krb5_ccache,
                                    mut client: krb5_principal,
                                    mut tgt_out: *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut tgs: krb5_principal = 0 as *mut krb5_principal_data;
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
    *tgt_out = 0 as *mut krb5_creds;
    code =
        krb5int_tgtname(context, &mut (*client).realm, &mut (*client).realm,
                        &mut tgs);
    if code != 0 { return code }
    memset(&mut mcreds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    mcreds.client = client;
    mcreds.server = tgs;
    code =
        krb5_get_credentials(context, options, ccache, &mut mcreds, tgt_out);
    krb5_free_principal(context, tgs);
    return code;
}
/*
 * Copy req_server to *out_server.  If req_server has the referral realm, set
 * the realm of *out_server to realm.  Otherwise the S4U2Proxy request will
 * fail unless the specified realm is the same as the TGT (or an alias to it).
 */
#[c2rust::src_loc = "800:1"]
unsafe extern "C" fn normalize_server_princ(mut context: krb5_context,
                                            mut realm: *const krb5_data,
                                            mut req_server: krb5_principal,
                                            mut out_server:
                                                *mut krb5_principal)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut server: krb5_principal = 0 as *mut krb5_principal_data;
    *out_server = 0 as krb5_principal;
    code =
        krb5_copy_principal(context, req_server as krb5_const_principal,
                            &mut server);
    if code != 0 { return code }
    if krb5_is_referral_realm(&mut (*server).realm) != 0 {
        krb5_free_data_contents(context, &mut (*server).realm);
        code =
            krb5int_copy_data_contents(context, realm, &mut (*server).realm);
        if code != 0 { krb5_free_principal(context, server); return code }
    }
    *out_server = server;
    return 0 as libc::c_int;
}
/* Return an error if server is present in referral_list. */
#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn check_referral_path(mut context: krb5_context,
                                         mut server: krb5_principal,
                                         mut referral_list:
                                             *mut *mut krb5_creds,
                                         mut referral_count: libc::c_int)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < referral_count {
        if krb5_principal_compare(context, server as krb5_const_principal,
                                  (**referral_list.offset(i as isize)).server
                                      as krb5_const_principal) != 0 {
            return -(1765328228 as libc::c_long) as krb5_error_code
        }
        i += 1
    }
    return 0 as libc::c_int;
}
/*
 * Make TGS requests for in_creds using *tgt_inout, following referrals until
 * the requested service ticket is issued.  Replace *tgt_inout with the final
 * TGT used, or free it and set it to NULL on error.  Place the final creds
 * received in *creds_out.
 */
#[c2rust::src_loc = "846:1"]
unsafe extern "C" fn chase_referrals(mut context: krb5_context,
                                     mut in_creds: *mut krb5_creds,
                                     mut kdcopt: krb5_flags,
                                     mut tgt_inout: *mut *mut krb5_creds,
                                     mut creds_out: *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut referral_tgts: [*mut krb5_creds; 10] =
        [0 as *mut krb5_creds, 0 as *mut krb5_creds, 0 as *mut krb5_creds,
         0 as *mut krb5_creds, 0 as *mut krb5_creds, 0 as *mut krb5_creds,
         0 as *mut krb5_creds, 0 as *mut krb5_creds, 0 as *mut krb5_creds,
         0 as *mut krb5_creds];
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
    let mut tgt: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut tkt: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut server: krb5_principal_data =
        krb5_principal_data{magic: 0,
                            realm:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            data: 0 as *mut krb5_data,
                            length: 0,
                            type_0: 0,};
    let mut referral_count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    tgt = *tgt_inout;
    *tgt_inout = 0 as *mut krb5_creds;
    *creds_out = 0 as *mut krb5_creds;
    mcreds = *in_creds;
    server = *(*in_creds).server;
    mcreds.server = &mut server;
    referral_count = 0 as libc::c_int;
    loop  {
        if !(referral_count < 10 as libc::c_int) {
            current_block = 15125582407903384992;
            break ;
        }
        code =
            krb5_get_cred_via_tkt(context, tgt, kdcopt, (*tgt).addresses,
                                  &mut mcreds, &mut tkt);
        if code != 0 { current_block = 8091952067394221362; break ; }
        if krb5_principal_compare_any_realm(context,
                                            mcreds.server as
                                                krb5_const_principal,
                                            (*tkt).server as
                                                krb5_const_principal) != 0 {
            *creds_out = tkt;
            *tgt_inout = tgt;
            tgt = 0 as *mut krb5_creds;
            tkt = tgt;
            current_block = 8091952067394221362;
            break ;
        } else if !((*(*tkt).server).length == 2 as libc::c_int &&
                        data_eq_string(*(*(*tkt).server).data.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize),
                                       b"krbtgt\x00" as *const u8 as
                                           *const libc::c_char) != 0) {
            code = -(1765328240 as libc::c_long) as krb5_error_code;
            current_block = 8091952067394221362;
            break ;
        } else if data_eq(*(*(*tgt).server).data.offset(1 as libc::c_int as
                                                            isize),
                          *(*(*tkt).server).data.offset(1 as libc::c_int as
                                                            isize)) != 0 {
            code = -(1765328167 as libc::c_long) as krb5_error_code;
            current_block = 8091952067394221362;
            break ;
        } else {
            code =
                check_referral_path(context, (*tkt).server,
                                    referral_tgts.as_mut_ptr(),
                                    referral_count);
            if code != 0 { current_block = 8091952067394221362; break ; }
            referral_tgts[referral_count as usize] = tgt;
            tgt = tkt;
            tkt = 0 as *mut krb5_creds;
            server.realm =
                *(*(*tgt).server).data.offset(1 as libc::c_int as isize);
            referral_count += 1
        }
    }
    match current_block {
        15125582407903384992 => {
            /* Max hop count exceeded. */
            code = -(1765328237 as libc::c_long) as krb5_error_code
        }
        _ => { }
    }
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        krb5_free_creds(context, referral_tgts[i as usize]);
        i += 1
    }
    krb5_free_creds(context, tkt);
    krb5_free_creds(context, tgt);
    return code;
}
/*
 * Make non-S4U2Proxy TGS requests for in_creds using *tgt_inout, following
 * referrals until the requested service ticket is returned.  Discard the
 * service ticket, but replace *tgt_inout with the final referral TGT.
 */
#[c2rust::src_loc = "916:1"]
unsafe extern "C" fn get_tgt_to_target_realm(mut context: krb5_context,
                                             mut in_creds: *mut krb5_creds,
                                             mut req_kdcopt: krb5_flags,
                                             mut tgt_inout:
                                                 *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut kdcopt: krb5_flags = 0;
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
    let mut out: *mut krb5_creds = 0 as *mut krb5_creds;
    mcreds = *in_creds;
    mcreds.second_ticket = empty_data();
    kdcopt =
        (**tgt_inout).ticket_flags & 0x54800000 as libc::c_int | req_kdcopt;
    code = chase_referrals(context, &mut mcreds, kdcopt, tgt_inout, &mut out);
    krb5_free_creds(context, out);
    return code;
}
/*
 * Make TGS requests for a cross-TGT to realm using *tgt_inout, following
 * alternate TGS replies until the requested TGT is issued.  Replace *tgt_inout
 * with the result.  Do nothing if *tgt_inout is already a cross-TGT for realm.
 */
#[c2rust::src_loc = "939:1"]
unsafe extern "C" fn get_target_realm_proxy_tgt(mut context: krb5_context,
                                                mut realm: *const krb5_data,
                                                mut req_kdcopt: krb5_flags,
                                                mut tgt_inout:
                                                    *mut *mut krb5_creds)
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
    let mut out: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut tgs: krb5_principal = 0 as *mut krb5_principal_data;
    let mut flags: krb5_flags = 0;
    if data_eq(*realm,
               *(*(**tgt_inout).server).data.offset(1 as libc::c_int as
                                                        isize)) != 0 {
        return 0 as libc::c_int
    }
    code =
        krb5int_tgtname(context, realm,
                        &mut *(*(**tgt_inout).server).data.offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize),
                        &mut tgs);
    if code != 0 { return code }
    memset(&mut mcreds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    mcreds.client = (**tgt_inout).client;
    mcreds.server = tgs;
    flags =
        req_kdcopt | (**tgt_inout).ticket_flags & 0x54800000 as libc::c_int;
    code = chase_referrals(context, &mut mcreds, flags, tgt_inout, &mut out);
    krb5_free_principal(context, tgs);
    if code != 0 { return code }
    krb5_free_creds(context, *tgt_inout);
    *tgt_inout = out;
    return 0 as libc::c_int;
}
/*
 * Make an S4U2Proxy (constrained delegation) request.  in_creds->client is the
 * impersonator principal, and in_creds->second_ticket is the evidence
 * ticket.
 */
#[no_mangle]
#[c2rust::src_loc = "972:1"]
pub unsafe extern "C" fn k5_get_proxy_cred_from_kdc(mut context: krb5_context,
                                                    mut options: krb5_flags,
                                                    mut ccache: krb5_ccache,
                                                    mut in_creds:
                                                        *mut krb5_creds,
                                                    mut out_creds:
                                                        *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut flags: krb5_flags = 0;
    let mut req_kdcopt: krb5_flags = 0 as libc::c_int;
    let mut server: krb5_principal = 0 as krb5_principal;
    let mut in_padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut enc_padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
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
    let mut tgt: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut tkt: *mut krb5_creds = 0 as *mut krb5_creds;
    *out_creds = 0 as *mut krb5_creds;
    if (*in_creds).second_ticket.length == 0 as libc::c_int as libc::c_uint ||
           options & 64 as libc::c_int == 0 as libc::c_int {
        return 22 as libc::c_int
    }
    options &= !(64 as libc::c_int);
    code =
        get_client_tgt(context, options, ccache, (*in_creds).client,
                       &mut tgt);
    if !(code != 0) {
        code =
            normalize_server_princ(context, &mut (*(*in_creds).client).realm,
                                   (*in_creds).server, &mut server);
        if !(code != 0) {
            code = add_rbcd_padata(context, &mut in_padata);
            if !(code != 0) {
                if options & 4 as libc::c_int != 0 {
                    req_kdcopt |= 0x10000 as libc::c_int
                }
                if options & 16 as libc::c_int != 0 {
                    req_kdcopt |= 0x40000000 as libc::c_int
                }
                if options & 32 as libc::c_int != 0 {
                    req_kdcopt |= 0x20 as libc::c_int
                }
                mcreds = *in_creds;
                mcreds.server = server;
                flags =
                    req_kdcopt |
                        (*tgt).ticket_flags & 0x54800000 as libc::c_int |
                        0x20000 as libc::c_int | 0x10000 as libc::c_int;
                code =
                    krb5_get_cred_via_tkt_ext(context, tgt, flags,
                                              (*tgt).addresses, in_padata,
                                              &mut mcreds, None,
                                              0 as *mut libc::c_void,
                                              0 as
                                                  *mut *mut *mut krb5_pa_data,
                                              &mut enc_padata, &mut tkt,
                                              0 as *mut *mut krb5_keyblock);
                /*
     * If the server principal name included a foreign realm which wasn't an
     * alias for the local realm, the KDC won't be able to decrypt the TGT.
     * Windows KDCs will return a BAD_INTEGRITY error in this case, while MIT
     * KDCs will return S_PRINCIPAL_UNKNOWN.  We cannot distinguish the latter
     * error from the service principal actually being unknown in the realm,
     * but set a comprehensible error message for the BAD_INTEGRITY error.
     */
                if code as libc::c_long == -(1765328353 as libc::c_long) &&
                       krb5_realm_compare(context,
                                          (*in_creds).client as
                                              krb5_const_principal,
                                          server as krb5_const_principal) == 0
                   {
                    krb5_set_error_message(context, code,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Realm specified but S4U2Proxy must use referral realm\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
                }
                if !(code != 0) {
                    if krb5_principal_compare_any_realm(context,
                                                        server as
                                                            krb5_const_principal,
                                                        (*tkt).server as
                                                            krb5_const_principal)
                           == 0 {
                        /* Make sure we got a referral. */
                        if !((*(*tkt).server).length == 2 as libc::c_int &&
                                 data_eq_string(*(*(*tkt).server).data.offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize),
                                                b"krbtgt\x00" as *const u8 as
                                                    *const libc::c_char) != 0)
                           {
                            code =
                                -(1765328240 as libc::c_long) as
                                    krb5_error_code;
                            current_block = 9577893318061858114;
                        } else {
                            /*
         * Make sure the KDC supports S4U and resource-based constrained
         * delegation; otherwise we might have gotten a regular TGT referral
         * rather than a proxy TGT referral.
         */
                            code = check_rbcd_support(context, enc_padata);
                            if code != 0 {
                                current_block = 9577893318061858114;
                            } else {
                                krb5_free_pa_data(context, enc_padata);
                                enc_padata = 0 as *mut *mut krb5_pa_data;
                                /*
         * Replace tgt with a regular (not proxy) TGT to the target realm, by
         * making a normal TGS request and following referrals.  Per [MS-SFU]
         * 3.1.5.2.2, we need this TGT to make the final TGS request.
         */
                                code =
                                    get_tgt_to_target_realm(context,
                                                            &mut mcreds,
                                                            req_kdcopt,
                                                            &mut tgt);
                                if code != 0 {
                                    current_block = 9577893318061858114;
                                } else {
                                    /*
         * Replace tkt with a proxy TGT (meaning, one obtained using the
         * referral TGT we got from the first S4U2Proxy request) to the target
         * realm, if it isn't already one.
         */
                                    code =
                                        get_target_realm_proxy_tgt(context,
                                                                   &mut *(*(*tgt).server).data.offset(1
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          isize),
                                                                   req_kdcopt,
                                                                   &mut tkt);
                                    if code != 0 {
                                        current_block = 9577893318061858114;
                                    } else {
                                        krb5_free_data_contents(context,
                                                                &mut (*server).realm);
                                        code =
                                            krb5int_copy_data_contents(context,
                                                                       &mut *(*(*tgt).server).data.offset(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              isize),
                                                                       &mut (*server).realm);
                                        if code != 0 {
                                            current_block =
                                                9577893318061858114;
                                        } else {
                                            /* Make an S4U2Proxy request to the target realm using the regular TGT,
         * with the proxy TGT as the evidence ticket. */
                                            mcreds.second_ticket =
                                                (*tkt).ticket;
                                            (*tkt).ticket = empty_data();
                                            krb5_free_creds(context, tkt);
                                            tkt = 0 as *mut krb5_creds;
                                            flags =
                                                req_kdcopt |
                                                    (*tgt).ticket_flags &
                                                        0x54800000 as
                                                            libc::c_int |
                                                    0x20000 as libc::c_int |
                                                    0x10000 as libc::c_int;
                                            code =
                                                krb5_get_cred_via_tkt_ext(context,
                                                                          tgt,
                                                                          flags,
                                                                          (*tgt).addresses,
                                                                          in_padata,
                                                                          &mut mcreds,
                                                                          None,
                                                                          0 as
                                                                              *mut libc::c_void,
                                                                          0 as
                                                                              *mut *mut *mut krb5_pa_data,
                                                                          &mut enc_padata,
                                                                          &mut tkt,
                                                                          0 as
                                                                              *mut *mut krb5_keyblock);
                                            free(mcreds.second_ticket.data as
                                                     *mut libc::c_void);
                                            if code != 0 {
                                                current_block =
                                                    9577893318061858114;
                                            } else {
                                                code =
                                                    check_rbcd_support(context,
                                                                       enc_padata);
                                                if code != 0 {
                                                    current_block =
                                                        9577893318061858114;
                                                } else if krb5_principal_compare(context,
                                                                                 server
                                                                                     as
                                                                                     krb5_const_principal,
                                                                                 (*tkt).server
                                                                                     as
                                                                                     krb5_const_principal)
                                                              == 0 {
                                                    code =
                                                        -(1765328240 as
                                                              libc::c_long) as
                                                            krb5_error_code;
                                                    current_block =
                                                        9577893318061858114;
                                                } else {
                                                    current_block =
                                                        9627623479216730126;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else { current_block = 9627623479216730126; }
                    match current_block {
                        9577893318061858114 => { }
                        _ => {
                            if krb5_principal_compare(context,
                                                      (*in_creds).server as
                                                          krb5_const_principal,
                                                      (*tkt).server as
                                                          krb5_const_principal)
                                   == 0 {
                                krb5_free_principal(context, (*tkt).server);
                                (*tkt).server = 0 as krb5_principal;
                                code =
                                    krb5_copy_principal(context,
                                                        (*in_creds).server as
                                                            krb5_const_principal,
                                                        &mut (*tkt).server);
                                if code != 0 {
                                    current_block = 9577893318061858114;
                                } else {
                                    current_block = 10380409671385728102;
                                }
                            } else { current_block = 10380409671385728102; }
                            match current_block {
                                9577893318061858114 => { }
                                _ => {
                                    if options & 8 as libc::c_int == 0 {
                                        krb5_cc_store_cred(context, ccache,
                                                           tkt);
                                    }
                                    *out_creds = tkt;
                                    tkt = 0 as *mut krb5_creds
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_creds(context, tgt);
    krb5_free_creds(context, tkt);
    krb5_free_principal(context, server);
    krb5_free_pa_data(context, in_padata);
    krb5_free_pa_data(context, enc_padata);
    return code;
}
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
/* allowable clock skew */
/* Message size above which we'll try TCP first in send-to-kdc
       type code.  Aside from the 2**16 size limit, we put no
       absolute limit on the UDP packet size.  */
/* Use the config-file ktypes instead of app-specified?  */
/* locate_kdc module stuff */
/* preauth module stuff */
/* cache module stuff */
/* localauth module stuff */
/* hostrealm module stuff */
/* TLS module vtable (if loaded) */
/* error detail info */
/* For Sun iprop code; does this really have to be here?  */
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
/* should move into k5-int.h */
/* chk_trans.c */
/* free_rtree.c */
/* Some data comparison and conversion functions.  */
/* Allocate at least one byte since zero-byte allocs may return NULL. */
/* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
/* Allocate at least one byte since zero-byte allocs may return NULL. */
/* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
/* Return a copy of the len bytes of memory at in; set *code to 0 or ENOMEM. */
/* Like k5memdup, but add a final null byte. */
/* Convert a krb5_timestamp to a time_t value, treating the negative range of
 * krb5_timestamp as times between 2038 and 2106 (if time_t is 64-bit). */
/* Return the delta between two timestamps (a - b) as a signed 32-bit value,
 * without relying on undefined behavior. */
/* Increment a timestamp by a signed 32-bit interval, without relying on
 * undefined behavior. */
/* Return true if a comes after b. */
/* Return true if a and b are within d seconds. */
/*
 * Exported API for constrained delegation (S4U2Proxy).
 *
 * This is preferable to using krb5_get_credentials directly because
 * it can perform some additional checks.
 */
#[no_mangle]
#[c2rust::src_loc = "1136:1"]
pub unsafe extern "C" fn krb5_get_credentials_for_proxy(mut context:
                                                            krb5_context,
                                                        mut options:
                                                            krb5_flags,
                                                        mut ccache:
                                                            krb5_ccache,
                                                        mut in_creds:
                                                            *mut krb5_creds,
                                                        mut evidence_tkt:
                                                            *mut krb5_ticket,
                                                        mut out_creds:
                                                            *mut *mut krb5_creds)
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
    let mut ncreds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut fields: krb5_flags = 0;
    let mut evidence_tkt_data: *mut krb5_data = 0 as *mut krb5_data;
    let mut s4u_creds: krb5_creds =
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
    *out_creds = 0 as *mut krb5_creds;
    if in_creds.is_null() || (*in_creds).client.is_null() ||
           evidence_tkt.is_null() {
        code = 22 as libc::c_int
    } else if !(*evidence_tkt).enc_part2.is_null() &&
                  krb5_principal_compare(context,
                                         (*(*evidence_tkt).enc_part2).client
                                             as krb5_const_principal,
                                         (*in_creds).client as
                                             krb5_const_principal) == 0 {
        code = 22 as libc::c_int
    } else {
        code =
            krb5int_construct_matching_creds(context, options, in_creds,
                                             &mut mcreds, &mut fields);
        if !(code != 0 as libc::c_int) {
            ncreds =
                calloc(1 as libc::c_int as libc::c_ulong,
                       ::std::mem::size_of::<krb5_creds>() as libc::c_ulong)
                    as *mut krb5_creds;
            if ncreds.is_null() {
                code = 12 as libc::c_int
            } else {
                (*ncreds).magic = -(1760647394 as libc::c_long) as krb5_magic;
                code =
                    krb5_cc_retrieve_cred(context, ccache, fields,
                                          &mut mcreds, ncreds);
                if code != 0 as libc::c_int {
                    free(ncreds as *mut libc::c_void);
                    ncreds = in_creds
                } else { *out_creds = ncreds }
                if !(code as libc::c_long != -(1765328243 as libc::c_long) &&
                         code as libc::c_long != -(1765328184 as libc::c_long)
                         || options & 2 as libc::c_int != 0) {
                    code =
                        encode_krb5_ticket(evidence_tkt,
                                           &mut evidence_tkt_data);
                    if !(code != 0 as libc::c_int) {
                        s4u_creds = *in_creds;
                        s4u_creds.client = (*evidence_tkt).server;
                        s4u_creds.second_ticket = *evidence_tkt_data;
                        code =
                            k5_get_proxy_cred_from_kdc(context,
                                                       options |
                                                           64 as libc::c_int,
                                                       ccache, &mut s4u_creds,
                                                       out_creds);
                        if !(code != 0 as libc::c_int) {
                            /*
     * Caller should have set in_creds->client to match evidence
     * ticket client.  If we can, verify it before issuing the request.
     */
                            /*
     * Check client name because we couldn't compare that inside
     * krb5_get_credentials() (enc_part2 is unavailable in clear)
     */
                            if krb5_principal_compare(context,
                                                      (*in_creds).client as
                                                          krb5_const_principal,
                                                      (**out_creds).client as
                                                          krb5_const_principal)
                                   == 0 {
                                code =
                                    -(1765328237 as libc::c_long) as
                                        krb5_error_code
                            }
                        }
                    }
                }
            }
        }
    }
    if !(*out_creds).is_null() && code != 0 as libc::c_int {
        krb5_free_creds(context, *out_creds);
        *out_creds = 0 as *mut krb5_creds
    }
    if !evidence_tkt_data.is_null() {
        krb5_free_data(context, evidence_tkt_data);
    }
    return code;
}
