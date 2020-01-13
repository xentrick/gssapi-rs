use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
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
    #[c2rust::src_loc = "1993:1"]
    pub type krb5_authenticator = _krb5_authenticator;
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
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Requested options */
        /* *< Ticket */
        /* *< Encrypted authenticator */
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
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn krb5_c_random_make_octets(context: krb5_context,
                                         data: *mut krb5_data)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context: krb5_context, val: *mut krb5_ticket);
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
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "776:16"]
    pub struct _krb5_fast_armor {
        pub armor_type: krb5_int32,
        pub armor_value: krb5_data,
    }
    #[c2rust::src_loc = "776:1"]
    pub type krb5_fast_armor = _krb5_fast_armor;
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
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
    #[inline]
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_error_code, krb5_authenticator,
                        krb5_ap_req, krb5_kdc_req, krb5_authdata, krb5_ticket,
                        krb5_context, krb5_principal_data,
                        krb5_const_principal, krb5_keyblock, krb5_pa_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::string_h::{memcpy, explicit_bzero};
    use super::stdlib_h::{calloc, free};
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
        #[c2rust::src_loc = "1375:1"]
        pub fn encode_krb5_authenticator(rep: *const krb5_authenticator,
                                         code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1396:1"]
        pub fn encode_krb5_ap_req(rep: *const krb5_ap_req,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1408:1"]
        pub fn encode_krb5_tgs_req(rep: *const krb5_kdc_req,
                                   code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1442:1"]
        pub fn encode_krb5_authdata(rep: *const *mut krb5_authdata,
                                    code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1586:1"]
        pub fn decode_krb5_ticket(code: *const krb5_data,
                                  rep: *mut *mut krb5_ticket)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2090:1"]
        pub fn krb5_get_tgs_ktypes(_: krb5_context, _: krb5_const_principal,
                                   _: *mut *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2115:1"]
        pub fn krb5_generate_subkey(_: krb5_context, _: *const krb5_keyblock,
                                    _: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
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
                        krb5_kdc_req};
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/fast.h:29"]
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
    #[c2rust::src_loc = "51:1"]
    pub type kdc_req_encoder_proc
        =
        Option<unsafe extern "C" fn(_: *const krb5_kdc_req,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_kdc_req, krb5_keyblock, krb5_ui_4, krb5_int32,
                        krb5_error_code, krb5_data, krb5_context,
                        _krb5_ccache, krb5_ccache};
    use super::k5_int_h::{krb5_fast_armor, _krb5_context};
    extern "C" {
        /* Perform FAST */
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn krb5int_fast_prep_req_body(context: krb5_context,
                                          state:
                                              *mut krb5int_fast_request_state,
                                          request: *mut krb5_kdc_req,
                                          encoded_req_body:
                                              *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn krb5int_fast_prep_req(context: krb5_context,
                                     state: *mut krb5int_fast_request_state,
                                     request: *mut krb5_kdc_req,
                                     to_be_checksummed: *const krb5_data,
                                     encoder: kdc_req_encoder_proc,
                                     encoded_request: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "106:1"]
        pub fn krb5int_fast_tgs_armor(context: krb5_context,
                                      state: *mut krb5int_fast_request_state,
                                      subkey: *mut krb5_keyblock,
                                      session_key: *mut krb5_keyblock,
                                      ccache: krb5_ccache,
                                      target_realm: *mut krb5_data)
         -> krb5_error_code;
    }
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:27"]
pub mod k5_platform_h {
    #[inline]
    #[c2rust::src_loc = "751:1"]
    pub unsafe extern "C" fn load_32_n(mut p: *const libc::c_void)
     -> libc::c_uint {
        let mut n: uint32_t = 0;
        memcpy(&mut n as *mut uint32_t as *mut libc::c_void, p,
               4 as libc::c_int as libc::c_ulong);
        return n;
    }
    use super::stdint_uintn_h::uint32_t;
    use super::string_h::memcpy;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:27"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:27"]
pub mod k5_int_pkinit_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_keyblock, krb5_keyusage, krb5_data,
                        krb5_enc_data, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn krb5_encrypt_helper(context: krb5_context,
                                   key: *const krb5_keyblock,
                                   keyusage: krb5_keyusage,
                                   plain: *const krb5_data,
                                   cipher: *mut krb5_enc_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_PKINIT_H */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_cksumtype, krb5_authdatatype, krb5_keyusage,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_authenticator,
                       krb5_authenticator, _krb5_creds, krb5_creds,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_ap_req, krb5_ap_req, krb5_ccache,
                       _profile_t, _krb5_ccache, krb5_c_random_make_octets,
                       krb5_c_make_checksum, krb5_c_valid_enctype,
                       krb5_principal_compare, krb5_free_ticket,
                       krb5_free_keyblock, krb5_free_data,
                       krb5_free_data_contents, krb5_us_timeofday,
                       krb5_timeofday};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_fast_armor, krb5_fast_armor,
                         k5memdup, k5alloc, k5calloc, zapfree, make_data,
                         empty_data, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         encode_krb5_authenticator, encode_krb5_ap_req,
                         encode_krb5_tgs_req, encode_krb5_authdata,
                         decode_krb5_ticket, krb5_get_tgs_ktypes,
                         krb5_generate_subkey, krb5_free_pa_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::int_proto_h::k5_pacb_fn;
pub use self::fast_h::{krb5int_fast_request_state, kdc_req_encoder_proc,
                       krb5int_fast_prep_req_body, krb5int_fast_prep_req,
                       krb5int_fast_tgs_armor};
use self::stdlib_h::{calloc, free};
pub use self::k5_platform_h::load_32_n;
use self::string_h::{explicit_bzero, memset, memcpy};
use self::k5_trace_h::krb5int_trace;
use self::k5_int_pkinit_h::krb5_encrypt_helper;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/send_tgs.c - Construct a TGS request */
/*
 * Copyright 1990,1991,2009,2013 by the Massachusetts Institute of Technology.
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
/* Choose a random nonce for an AS or TGS request. */
#[no_mangle]
#[c2rust::src_loc = "32:1"]
pub unsafe extern "C" fn k5_generate_nonce(mut context: krb5_context,
                                           mut out: *mut int32_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut random_buf: [libc::c_uchar; 4] = [0; 4];
    let mut random_data: krb5_data =
        make_data(random_buf.as_mut_ptr() as *mut libc::c_void,
                  4 as libc::c_int as libc::c_uint);
    *out = 0 as libc::c_int;
    /* We and Heimdal incorrectly encode nonces as signed, so make sure we use
     * a non-negative value to avoid interoperability issues. */
    ret = krb5_c_random_make_octets(context, &mut random_data);
    if ret != 0 { return ret }
    *out =
        (0x7fffffff as libc::c_int as libc::c_uint &
             load_32_n(random_buf.as_mut_ptr() as *const libc::c_void)) as
            int32_t;
    return 0 as libc::c_int;
}
/* Construct an AP-REQ message for a TGS request. */
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn tgs_construct_ap_req(mut context: krb5_context,
                                          mut checksum_data: *mut krb5_data,
                                          mut tgt: *mut krb5_creds,
                                          mut subkey: *mut krb5_keyblock,
                                          mut ap_req_asn1_out:
                                              *mut *mut krb5_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut checksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut authent: krb5_authenticator =
        krb5_authenticator{magic: 0,
                           client: 0 as *mut krb5_principal_data,
                           checksum: 0 as *mut krb5_checksum,
                           cusec: 0,
                           ctime: 0,
                           subkey: 0 as *mut krb5_keyblock,
                           seq_number: 0,
                           authorization_data: 0 as *mut *mut krb5_authdata,};
    let mut ap_req: krb5_ap_req =
        krb5_ap_req{magic: 0,
                    ap_options: 0,
                    ticket: 0 as *mut krb5_ticket,
                    authenticator:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},};
    let mut authent_asn1: *mut krb5_data = 0 as *mut krb5_data;
    let mut ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    let mut authent_enc: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    *ap_req_asn1_out = 0 as *mut krb5_data;
    memset(&mut checksum as *mut krb5_checksum as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    memset(&mut ap_req as *mut krb5_ap_req as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_ap_req>() as libc::c_ulong);
    memset(&mut authent_enc as *mut krb5_enc_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_enc_data>() as libc::c_ulong);
    /* Generate checksum. */
    ret =
        krb5_c_make_checksum(context, 0 as libc::c_int, &mut (*tgt).keyblock,
                             6 as libc::c_int, checksum_data, &mut checksum);
    if !(ret != 0) {
        /* Construct, encode, and encrypt an authenticator. */
        authent.subkey = subkey;
        authent.seq_number = 0 as libc::c_int as krb5_ui_4;
        authent.checksum = &mut checksum;
        authent.client = (*tgt).client;
        authent.authorization_data = (*tgt).authdata;
        ret =
            krb5_us_timeofday(context, &mut authent.ctime,
                              &mut authent.cusec);
        if !(ret != 0) {
            ret = encode_krb5_authenticator(&mut authent, &mut authent_asn1);
            if !(ret != 0) {
                ret =
                    krb5_encrypt_helper(context, &mut (*tgt).keyblock,
                                        7 as libc::c_int, authent_asn1,
                                        &mut authent_enc);
                if !(ret != 0) {
                    ret = decode_krb5_ticket(&mut (*tgt).ticket, &mut ticket);
                    if !(ret != 0) {
                        /* Encode the AP-REQ. */
                        ap_req.authenticator = authent_enc;
                        ap_req.ticket = ticket;
                        ret = encode_krb5_ap_req(&mut ap_req, ap_req_asn1_out)
                    }
                }
            }
        }
    }
    free(checksum.contents as *mut libc::c_void);
    krb5_free_ticket(context, ticket);
    krb5_free_data_contents(context, &mut authent_enc.ciphertext);
    if !authent_asn1.is_null() {
        zapfree((*authent_asn1).data as *mut libc::c_void,
                (*authent_asn1).length as size_t);
    }
    free(authent_asn1 as *mut libc::c_void);
    return ret;
}
/*
 * Construct a TGS request and return its ASN.1 encoding as well as the
 * timestamp, nonce, and subkey used.  The pacb_fn callback allows the caller
 * to amend the request padata after the nonce and subkey are determined.
 */
#[no_mangle]
#[c2rust::src_loc = "118:1"]
pub unsafe extern "C" fn k5_make_tgs_req(mut context: krb5_context,
                                         mut fast_state:
                                             *mut krb5int_fast_request_state,
                                         mut tgt: *mut krb5_creds,
                                         mut kdcoptions: krb5_flags,
                                         mut addrs: *const *mut krb5_address,
                                         mut in_padata:
                                             *mut *mut krb5_pa_data,
                                         mut desired: *mut krb5_creds,
                                         mut pacb_fn: k5_pacb_fn,
                                         mut pacb_data: *mut libc::c_void,
                                         mut req_asn1_out: *mut krb5_data,
                                         mut timestamp_out:
                                             *mut krb5_timestamp,
                                         mut nonce_out: *mut krb5_int32,
                                         mut subkey_out:
                                             *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut req: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut authdata_asn1: *mut krb5_data = 0 as *mut krb5_data;
    let mut req_body_asn1: *mut krb5_data = 0 as *mut krb5_data;
    let mut ap_req_asn1: *mut krb5_data = 0 as *mut krb5_data;
    let mut tgs_req_asn1: *mut krb5_data = 0 as *mut krb5_data;
    let mut sec_ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    let mut sec_ticket_arr: [*mut krb5_ticket; 2] =
        [0 as *mut krb5_ticket; 2];
    let mut time_now: krb5_timestamp = 0;
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut subkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut authdata_enc: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut enctypes: [krb5_enctype; 2] = [0; 2];
    let mut defenctypes: *mut krb5_enctype = 0 as *mut krb5_enctype;
    let mut count: size_t = 0;
    let mut i: size_t = 0;
    *req_asn1_out = empty_data();
    *timestamp_out = 0 as libc::c_int;
    *nonce_out = 0 as libc::c_int;
    *subkey_out = 0 as *mut krb5_keyblock;
    memset(&mut req as *mut krb5_kdc_req as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_kdc_req>() as libc::c_ulong);
    memset(&mut authdata_enc as *mut krb5_enc_data as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_enc_data>() as libc::c_ulong);
    /* tgt's client principal must match the desired client principal. */
    if krb5_principal_compare(context, (*tgt).client as krb5_const_principal,
                              (*desired).client as krb5_const_principal) == 0
       {
        return -(1765328238 as libc::c_long) as krb5_error_code
    }
    /* tgt must be an actual credential, not a template. */
    if (*tgt).ticket.length == 0 {
        return -(1765328241 as libc::c_long) as krb5_error_code
    }
    req.kdc_options = kdcoptions;
    req.server = (*desired).server;
    req.from = (*desired).times.starttime;
    req.till =
        if (*desired).times.endtime != 0 {
            (*desired).times.endtime
        } else { (*tgt).times.endtime };
    req.rtime = (*desired).times.renew_till;
    ret = k5_generate_nonce(context, &mut req.nonce);
    if ret != 0 { return ret }
    *nonce_out = req.nonce;
    ret = krb5_timeofday(context, &mut time_now);
    if ret != 0 { return ret }
    *timestamp_out = time_now;
    req.addresses = addrs as *mut *mut krb5_address;
    /* Generate subkey. */
    ret = krb5_generate_subkey(context, &mut (*tgt).keyblock, &mut subkey);
    if ret != 0 { return ret }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Generated subkey for TGS request: {keyblock}\x00" as
                          *const u8 as *const libc::c_char, subkey);
    }
    ret =
        krb5int_fast_tgs_armor(context, fast_state, subkey,
                               &mut (*tgt).keyblock, 0 as krb5_ccache,
                               0 as *mut krb5_data);
    if !(ret != 0) {
        if !(*desired).authdata.is_null() {
            ret =
                encode_krb5_authdata((*desired).authdata, &mut authdata_asn1);
            if ret != 0 {
                current_block = 1208317026563730058;
            } else {
                ret =
                    krb5_encrypt_helper(context, subkey, 5 as libc::c_int,
                                        authdata_asn1, &mut authdata_enc);
                if ret != 0 {
                    current_block = 1208317026563730058;
                } else {
                    req.authorization_data = authdata_enc;
                    current_block = 10891380440665537214;
                }
            }
        } else { current_block = 10891380440665537214; }
        match current_block {
            1208317026563730058 => { }
            _ => {
                if (*desired).keyblock.enctype != 0 as libc::c_int {
                    if krb5_c_valid_enctype((*desired).keyblock.enctype) == 0
                       {
                        ret =
                            -(1765328234 as libc::c_long) as krb5_error_code;
                        current_block = 1208317026563730058;
                    } else {
                        enctypes[0 as libc::c_int as usize] =
                            (*desired).keyblock.enctype;
                        enctypes[1 as libc::c_int as usize] =
                            0 as libc::c_int;
                        req.ktype = enctypes.as_mut_ptr();
                        req.nktypes = 1 as libc::c_int;
                        current_block = 10753070352654377903;
                    }
                } else {
                    /* Get the default TGS enctypes. */
                    ret =
                        krb5_get_tgs_ktypes(context,
                                            (*desired).server as
                                                krb5_const_principal,
                                            &mut defenctypes);
                    if ret != 0 {
                        current_block = 1208317026563730058;
                    } else {
                        count = 0 as libc::c_int as size_t;
                        while *defenctypes.offset(count as isize) != 0 {
                            count = count.wrapping_add(1)
                        }
                        req.ktype = defenctypes;
                        req.nktypes = count as libc::c_int;
                        current_block = 10753070352654377903;
                    }
                }
                match current_block {
                    1208317026563730058 => { }
                    _ => {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"etypes requested in TGS request: {etypes}\x00"
                                              as *const u8 as
                                              *const libc::c_char, req.ktype);
                        }
                        if kdcoptions &
                               (0x8 as libc::c_int | 0x20000 as libc::c_int)
                               != 0 {
                            if (*desired).second_ticket.length ==
                                   0 as libc::c_int as libc::c_uint {
                                ret =
                                    -(1765328182 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 1208317026563730058;
                            } else {
                                ret =
                                    decode_krb5_ticket(&mut (*desired).second_ticket,
                                                       &mut sec_ticket);
                                if ret != 0 {
                                    current_block = 1208317026563730058;
                                } else {
                                    sec_ticket_arr[0 as libc::c_int as usize]
                                        = sec_ticket;
                                    sec_ticket_arr[1 as libc::c_int as usize]
                                        = 0 as *mut krb5_ticket;
                                    req.second_ticket =
                                        sec_ticket_arr.as_mut_ptr();
                                    current_block = 12264624100856317061;
                                }
                            }
                        } else { current_block = 12264624100856317061; }
                        match current_block {
                            1208317026563730058 => { }
                            _ => {
                                /* Encode the request body. */
                                ret =
                                    krb5int_fast_prep_req_body(context,
                                                               fast_state,
                                                               &mut req,
                                                               &mut req_body_asn1);
                                if !(ret != 0) {
                                    ret =
                                        tgs_construct_ap_req(context,
                                                             req_body_asn1,
                                                             tgt, subkey,
                                                             &mut ap_req_asn1);
                                    if !(ret != 0) {
                                        count = 0 as libc::c_int as size_t;
                                        while !in_padata.is_null() &&
                                                  !(*in_padata.offset(count as
                                                                          isize)).is_null()
                                              {
                                            count = count.wrapping_add(1)
                                        }
                                        /* Construct a padata array for the request, beginning with the ap-req. */
                                        padata =
                                            k5calloc(count.wrapping_add(2 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong),
                                                     ::std::mem::size_of::<*mut krb5_pa_data>()
                                                         as libc::c_ulong,
                                                     &mut ret) as
                                                *mut *mut krb5_pa_data;
                                        if !padata.is_null() {
                                            let ref mut fresh0 =
                                                *padata.offset(0 as
                                                                   libc::c_int
                                                                   as isize);
                                            *fresh0 =
                                                k5alloc(::std::mem::size_of::<krb5_pa_data>()
                                                            as libc::c_ulong,
                                                        &mut ret) as
                                                    *mut krb5_pa_data;
                                            if !(*padata.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).is_null()
                                               {
                                                (**padata.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).pa_type
                                                    = 1 as libc::c_int;
                                                let ref mut fresh1 =
                                                    (**padata.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).contents;
                                                *fresh1 =
                                                    k5memdup((*ap_req_asn1).data
                                                                 as
                                                                 *const libc::c_void,
                                                             (*ap_req_asn1).length
                                                                 as size_t,
                                                             &mut ret) as
                                                        *mut krb5_octet;
                                                if !(*padata.offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)).is_null()
                                                   {
                                                    (**padata.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).length
                                                        =
                                                        (*ap_req_asn1).length;
                                                    /* Append copies of any other supplied padata. */
                                                    i =
                                                        0 as libc::c_int as
                                                            size_t;
                                                    loop  {
                                                        if !(!in_padata.is_null()
                                                                 &&
                                                                 !(*in_padata.offset(i
                                                                                         as
                                                                                         isize)).is_null())
                                                           {
                                                            current_block =
                                                                5687667889785024198;
                                                            break ;
                                                        }
                                                        pa =
                                                            k5alloc(::std::mem::size_of::<krb5_pa_data>()
                                                                        as
                                                                        libc::c_ulong,
                                                                    &mut ret)
                                                                as
                                                                *mut krb5_pa_data;
                                                        if pa.is_null() {
                                                            current_block =
                                                                1208317026563730058;
                                                            break ;
                                                        }
                                                        (*pa).pa_type =
                                                            (**in_padata.offset(i
                                                                                    as
                                                                                    isize)).pa_type;
                                                        (*pa).length =
                                                            (**in_padata.offset(i
                                                                                    as
                                                                                    isize)).length;
                                                        (*pa).contents =
                                                            k5memdup((**in_padata.offset(i
                                                                                             as
                                                                                             isize)).contents
                                                                         as
                                                                         *const libc::c_void,
                                                                     (**in_padata.offset(i
                                                                                             as
                                                                                             isize)).length
                                                                         as
                                                                         size_t,
                                                                     &mut ret)
                                                                as
                                                                *mut krb5_octet;
                                                        if (*pa).contents.is_null()
                                                           {
                                                            current_block =
                                                                1208317026563730058;
                                                            break ;
                                                        }
                                                        let ref mut fresh2 =
                                                            *padata.offset(i.wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_ulong)
                                                                               as
                                                                               isize);
                                                        *fresh2 = pa;
                                                        i = i.wrapping_add(1)
                                                    }
                                                    match current_block {
                                                        1208317026563730058 =>
                                                        {
                                                        }
                                                        _ => {
                                                            req.padata =
                                                                padata;
                                                            if pacb_fn.is_some()
                                                               {
                                                                ret =
                                                                    Some(pacb_fn.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                                                                          subkey,
                                                                                                                                                          &mut req,
                                                                                                                                                          pacb_data);
                                                                if ret != 0 {
                                                                    current_block
                                                                        =
                                                                        1208317026563730058;
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        17958840340921835115;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    17958840340921835115;
                                                            }
                                                            match current_block
                                                                {
                                                                1208317026563730058
                                                                => {
                                                                }
                                                                _ => {
                                                                    /* Encode the TGS-REQ.  Discard the krb5_data container. */
                                                                    ret =
                                                                        krb5int_fast_prep_req(context,
                                                                                              fast_state,
                                                                                              &mut req,
                                                                                              ap_req_asn1,
                                                                                              Some(encode_krb5_tgs_req
                                                                                                       as
                                                                                                       unsafe extern "C" fn(_:
                                                                                                                                *const krb5_kdc_req,
                                                                                                                            _:
                                                                                                                                *mut *mut krb5_data)
                                                                                                           ->
                                                                                                               krb5_error_code),
                                                                                              &mut tgs_req_asn1);
                                                                    if !(ret
                                                                             !=
                                                                             0)
                                                                       {
                                                                        *req_asn1_out
                                                                            =
                                                                            *tgs_req_asn1;
                                                                        free(tgs_req_asn1
                                                                                 as
                                                                                 *mut libc::c_void);
                                                                        tgs_req_asn1
                                                                            =
                                                                            0
                                                                                as
                                                                                *mut krb5_data;
                                                                        *subkey_out
                                                                            =
                                                                            subkey;
                                                                        subkey
                                                                            =
                                                                            0
                                                                                as
                                                                                *mut krb5_keyblock
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
                        }
                    }
                }
            }
        }
    }
    krb5_free_data(context, authdata_asn1);
    krb5_free_data(context, req_body_asn1);
    krb5_free_data(context, ap_req_asn1);
    krb5_free_pa_data(context, req.padata);
    krb5_free_ticket(context, sec_ticket);
    krb5_free_data_contents(context, &mut authdata_enc.ciphertext);
    krb5_free_keyblock(context, subkey);
    free(defenctypes as *mut libc::c_void);
    return ret;
}
