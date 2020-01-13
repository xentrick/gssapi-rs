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
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
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
    /* * Last request entry */
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
    /* *< LR type */
    /* *< Timestamp */
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
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
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
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
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
        #[no_mangle]
        #[c2rust::src_loc = "560:1"]
        pub fn krb5_c_decrypt(context: krb5_context,
                              key: *const krb5_keyblock, usage: krb5_keyusage,
                              cipher_state: *const krb5_data,
                              input: *const krb5_enc_data,
                              output: *mut krb5_data) -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "727:1"]
        pub fn krb5_c_fx_cf2_simple(context: krb5_context,
                                    k1: *const krb5_keyblock,
                                    pepper1: *const libc::c_char,
                                    k2: *const krb5_keyblock,
                                    pepper2: *const libc::c_char,
                                    out: *mut *mut krb5_keyblock)
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
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "2479:1"]
        pub fn krb5_cc_get_principal(context: krb5_context,
                                     cache: krb5_ccache,
                                     principal: *mut krb5_principal)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3119:1"]
        pub fn krb5_get_credentials(context: krb5_context,
                                    options: krb5_flags, ccache: krb5_ccache,
                                    in_creds: *mut krb5_creds,
                                    out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3201:1"]
        pub fn krb5_mk_req_extended(context: krb5_context,
                                    auth_context: *mut krb5_auth_context,
                                    ap_req_options: krb5_flags,
                                    in_data: *mut krb5_data,
                                    in_creds: *mut krb5_creds,
                                    outbuf: *mut krb5_data)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3766:1"]
        pub fn krb5_copy_keyblock_contents(context: krb5_context,
                                           from: *const krb5_keyblock,
                                           to: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4457:1"]
        pub fn krb5_cc_get_config(context: krb5_context, id: krb5_ccache,
                                  principal: krb5_const_principal,
                                  key: *const libc::c_char,
                                  data: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "4666:1"]
        pub fn krb5_free_creds(context: krb5_context, val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4688:1"]
        pub fn krb5_free_checksum(context: krb5_context,
                                  val: *mut krb5_checksum);
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5815:1"]
        pub fn krb5_auth_con_getsendsubkey(ctx: krb5_context,
                                           ac: krb5_auth_context,
                                           keyblock: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
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
    #[c2rust::src_loc = "805:1"]
    pub type krb5_fast_response = _krb5_fast_response;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "805:16"]
    pub struct _krb5_fast_response {
        pub magic: krb5_magic,
        pub padata: *mut *mut krb5_pa_data,
        pub strengthen_key: *mut krb5_keyblock,
        pub finished: *mut krb5_fast_finished,
        pub nonce: krb5_int32,
    }
    #[c2rust::src_loc = "798:1"]
    pub type krb5_fast_finished = _krb5_fast_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "798:16"]
    pub struct _krb5_fast_finished {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub client: krb5_principal,
        pub ticket_checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "780:1"]
    pub type krb5_fast_armored_req = _krb5_fast_armored_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "780:16"]
    pub struct _krb5_fast_armored_req {
        pub magic: krb5_magic,
        pub armor: *mut krb5_fast_armor,
        pub req_checksum: krb5_checksum,
        pub enc_part: krb5_enc_data,
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
    #[c2rust::src_loc = "772:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "773:5"]
    pub const KRB5_FAST_ARMOR_AP_REQUEST: C2RustUnnamed = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "787:16"]
    pub struct _krb5_fast_req {
        pub magic: krb5_magic,
        pub fast_options: krb5_flags,
        pub req_body: *mut krb5_kdc_req,
    }
    #[c2rust::src_loc = "787:1"]
    pub type krb5_fast_req = _krb5_fast_req;
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
                        krb5_pa_data, krb5_keyblock, krb5_timestamp,
                        krb5_principal, krb5_checksum, krb5_enc_data,
                        krb5_data, krb5_kdc_req, krb5_context,
                        krb5_preauthtype, krb5_ticket, krb5_error_code,
                        krb5_error};
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
        #[no_mangle]
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "945:1"]
        pub fn krb5_free_fast_armor(_: krb5_context, _: *mut krb5_fast_armor);
        #[no_mangle]
        #[c2rust::src_loc = "946:1"]
        pub fn krb5_free_fast_armored_req(_: krb5_context,
                                          _: *mut krb5_fast_armored_req);
        #[no_mangle]
        #[c2rust::src_loc = "950:1"]
        pub fn krb5_free_fast_response(_: krb5_context,
                                       _: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn encode_krb5_ticket(rep: *const krb5_ticket,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1411:1"]
        pub fn encode_krb5_kdc_req_body(rep: *const krb5_kdc_req,
                                        code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1496:1"]
        pub fn encode_krb5_pa_fx_fast_request(_: *const krb5_fast_armored_req,
                                              _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1499:1"]
        pub fn encode_krb5_fast_req(_: *const krb5_fast_req,
                                    _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1636:1"]
        pub fn decode_krb5_checksum(_: *const krb5_data,
                                    _: *mut *mut krb5_checksum)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1651:1"]
        pub fn decode_krb5_padata_sequence(output: *const krb5_data,
                                           rep: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1654:1"]
        pub fn decode_krb5_typed_data(_: *const krb5_data,
                                      _: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1690:1"]
        pub fn decode_krb5_pa_fx_fast_reply(_: *const krb5_data,
                                            _: *mut *mut krb5_enc_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1693:1"]
        pub fn decode_krb5_fast_response(_: *const krb5_data,
                                         _: *mut *mut krb5_fast_response)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2161:1"]
        pub fn krb5_free_enc_data(_: krb5_context, _: *mut krb5_enc_data);
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/fast.h:48"]
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
                        krb5_error_code, krb5_data};
    use super::k5_int_h::krb5_fast_armor;
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
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:28"]
pub mod int_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_data, krb5_principal,
                        krb5_error_code, krb5_get_init_creds_opt, krb5_flags};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn krb5int_tgtname(context: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "343:1"]
        pub fn k5_gic_opt_get_fast_ccache_name(opt:
                                                   *mut krb5_get_init_creds_opt)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "346:1"]
        pub fn k5_gic_opt_get_fast_flags(opt: *mut krb5_get_init_creds_opt)
         -> krb5_flags;
    }
    /* KRB5_INT_FUNC_PROTO__ */
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
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_creds, krb5_creds, _krb5_last_req_entry,
                       krb5_last_req_entry, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, krb5_ccache,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, _krb5_auth_context, _krb5_ccache,
                       krb5_c_decrypt, krb5_c_fx_cf2_simple,
                       krb5_c_make_checksum, krb5_c_verify_checksum,
                       krb5_cc_close, krb5_cc_get_principal,
                       krb5_get_credentials, krb5_mk_req_extended,
                       krb5_copy_keyblock_contents, krb5_cc_resolve,
                       krb5_cc_get_config, krb5_free_principal,
                       krb5_free_error, krb5_free_creds,
                       krb5_free_cred_contents, krb5_free_checksum,
                       krb5_free_keyblock, krb5_free_keyblock_contents,
                       krb5_free_data, krb5_free_data_contents,
                       krb5_auth_con_free, krb5_auth_con_getsendsubkey,
                       krb5_set_error_message, krb5_prepend_error_message,
                       krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_fast_response,
                         _krb5_fast_response, krb5_fast_finished,
                         _krb5_fast_finished, krb5_fast_armored_req,
                         _krb5_fast_armored_req, krb5_fast_armor,
                         _krb5_fast_armor, C2RustUnnamed,
                         KRB5_FAST_ARMOR_AP_REQUEST, _krb5_fast_req,
                         krb5_fast_req, make_data, empty_data, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_find_pa_data,
                         krb5_free_fast_armor, krb5_free_fast_armored_req,
                         krb5_free_fast_response, encode_krb5_ticket,
                         encode_krb5_kdc_req_body,
                         encode_krb5_pa_fx_fast_request, encode_krb5_fast_req,
                         decode_krb5_checksum, decode_krb5_error,
                         decode_krb5_padata_sequence, decode_krb5_typed_data,
                         decode_krb5_pa_fx_fast_reply,
                         decode_krb5_fast_response, krb5_free_pa_data,
                         krb5_free_enc_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::fast_h::{krb5int_fast_request_state, kdc_req_encoder_proc};
use self::stdlib_h::{malloc, calloc, free};
use self::libintl_h::dgettext;
use self::string_h::memset;
use self::assert_h::__assert_fail;
use self::k5_trace_h::krb5int_trace;
use self::k5_int_pkinit_h::krb5_encrypt_helper;
use self::int_proto_h::{krb5int_tgtname, k5_gic_opt_get_fast_ccache_name,
                        k5_gic_opt_get_fast_flags};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/fast.c */
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
/*
 * It is possible to support sending a request that includes both a FAST and
 * normal version.  This would complicate the pre-authentication logic
 * significantly.  You would need to maintain two contexts, one for FAST and
 * one for normal use.  In adition, you would need to manage the security
 * issues surrounding downgrades.  However trying FAST at all requires an armor
 * key.  Generally in obtaining the armor key, the client learns enough to know
 * that FAST is supported.  If not, the client can see FAST in the
 * preauth_required error's padata and retry with FAST.  So, this
 * implementation does not support FAST+normal.
 *
 * We store the outer version of the request to use.  The caller stores the
 * inner version.  We handle the encoding of the request body (and request) and
 * provide encoded request bodies for the caller to use as these may be used
 * for checksums.  In the AS case we also evaluate whether to continue a
 * conversation as one of the important questions there is the presence of a
 * cookie.
 */
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn fast_armor_ap_request(mut context: krb5_context,
                                           mut state:
                                               *mut krb5int_fast_request_state,
                                           mut ccache: krb5_ccache,
                                           mut target_principal:
                                               krb5_principal)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut creds: krb5_creds =
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
    let mut out_creds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut authcontext: krb5_auth_context = 0 as krb5_auth_context;
    let mut encoded_authenticator: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut armor: *mut krb5_fast_armor = 0 as *mut krb5_fast_armor;
    let mut subkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut armor_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    encoded_authenticator.data = 0 as *mut libc::c_char;
    memset(&mut creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    creds.server = target_principal;
    retval = krb5_cc_get_principal(context, ccache, &mut creds.client);
    if retval == 0 as libc::c_int {
        retval =
            krb5_get_credentials(context, 0 as libc::c_int, ccache,
                                 &mut creds, &mut out_creds)
    }
    if retval == 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Armor ccache sesion key: {keyblock}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*out_creds).keyblock as *mut krb5_keyblock);
        }
        retval =
            krb5_mk_req_extended(context, &mut authcontext,
                                 0x1 as libc::c_int, 0 as *mut krb5_data,
                                 out_creds, &mut encoded_authenticator)
    }
    if retval == 0 as libc::c_int {
        retval =
            krb5_auth_con_getsendsubkey(context, authcontext, &mut subkey)
    }
    if retval == 0 as libc::c_int {
        retval =
            krb5_c_fx_cf2_simple(context, subkey,
                                 b"subkeyarmor\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut (*out_creds).keyblock,
                                 b"ticketarmor\x00" as *const u8 as
                                     *const libc::c_char, &mut armor_key)
    }
    if retval == 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"FAST armor key: {keyblock}\x00" as *const u8 as
                              *const libc::c_char, armor_key);
        }
        armor =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_fast_armor>() as libc::c_ulong)
                as *mut krb5_fast_armor;
        if armor.is_null() { retval = 12 as libc::c_int }
    }
    if retval == 0 as libc::c_int {
        (*armor).armor_type = KRB5_FAST_ARMOR_AP_REQUEST as libc::c_int;
        (*armor).armor_value = encoded_authenticator;
        encoded_authenticator.data = 0 as *mut libc::c_char;
        encoded_authenticator.length = 0 as libc::c_int as libc::c_uint;
        (*state).armor = armor;
        armor = 0 as *mut krb5_fast_armor;
        (*state).armor_key = armor_key;
        armor_key = 0 as *mut krb5_keyblock
    }
    krb5_free_keyblock(context, armor_key);
    krb5_free_keyblock(context, subkey);
    if !out_creds.is_null() { krb5_free_creds(context, out_creds); }
    /* target_principal is owned by caller. */
    creds.server = 0 as krb5_principal;
    krb5_free_cred_contents(context, &mut creds);
    if !encoded_authenticator.data.is_null() {
        krb5_free_data_contents(context, &mut encoded_authenticator);
    }
    krb5_auth_con_free(context, authcontext);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn krb5int_fast_tgs_armor(mut context: krb5_context,
                                                mut state:
                                                    *mut krb5int_fast_request_state,
                                                mut subkey:
                                                    *mut krb5_keyblock,
                                                mut session_key:
                                                    *mut krb5_keyblock,
                                                mut ccache: krb5_ccache,
                                                mut target_realm:
                                                    *mut krb5_data)
 -> krb5_error_code {
    let mut target_principal: krb5_principal = 0 as krb5_principal;
    let mut existing_armor: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    if !ccache.is_null() {
        retval =
            krb5int_tgtname(context, target_realm, target_realm,
                            &mut target_principal);
        if retval == 0 as libc::c_int {
            retval =
                fast_armor_ap_request(context, state, ccache,
                                      target_principal)
        }
        if retval == 0 as libc::c_int {
            existing_armor = (*state).armor_key;
            (*state).armor_key = 0 as *mut krb5_keyblock;
            retval =
                krb5_c_fx_cf2_simple(context, existing_armor,
                                     b"explicitarmor\x00" as *const u8 as
                                         *const libc::c_char, subkey,
                                     b"tgsarmor\x00" as *const u8 as
                                         *const libc::c_char,
                                     &mut (*state).armor_key)
        }
    } else {
        retval =
            krb5_c_fx_cf2_simple(context, subkey,
                                 b"subkeyarmor\x00" as *const u8 as
                                     *const libc::c_char, session_key,
                                 b"ticketarmor\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut (*state).armor_key)
    }
    if !target_principal.is_null() {
        krb5_free_principal(context, target_principal);
    }
    krb5_free_keyblock(context, existing_armor);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn krb5int_fast_prep_req_body(mut context: krb5_context,
                                                    mut state:
                                                        *mut krb5int_fast_request_state,
                                                    mut request:
                                                        *mut krb5_kdc_req,
                                                    mut encoded_request_body:
                                                        *mut *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut local_encoded_request_body: *mut krb5_data = 0 as *mut krb5_data;
    if !state.is_null() {
    } else {
        __assert_fail(b"state != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"fast.c\x00" as *const u8 as *const libc::c_char,
                      153 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 124],
                                                &[libc::c_char; 124]>(b"krb5_error_code krb5int_fast_prep_req_body(krb5_context, struct krb5int_fast_request_state *, krb5_kdc_req *, krb5_data **)\x00")).as_ptr());
    }
    *encoded_request_body = 0 as *mut krb5_data;
    if (*state).armor_key.is_null() {
        return encode_krb5_kdc_req_body(request, encoded_request_body)
    }
    (*state).fast_outer_request = *request;
    (*state).fast_outer_request.padata = 0 as *mut *mut krb5_pa_data;
    if retval == 0 as libc::c_int {
        retval =
            encode_krb5_kdc_req_body(&mut (*state).fast_outer_request,
                                     &mut local_encoded_request_body)
    }
    if retval == 0 as libc::c_int {
        *encoded_request_body = local_encoded_request_body;
        local_encoded_request_body = 0 as *mut krb5_data
    }
    if !local_encoded_request_body.is_null() {
        krb5_free_data(context, local_encoded_request_body);
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "171:1"]
pub unsafe extern "C" fn krb5int_fast_as_armor(mut context: krb5_context,
                                               mut state:
                                                   *mut krb5int_fast_request_state,
                                               mut opt:
                                                   *mut krb5_get_init_creds_opt,
                                               mut request: *mut krb5_kdc_req)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    let mut target_principal: krb5_principal = 0 as krb5_principal;
    let mut target_realm: *mut krb5_data = 0 as *mut krb5_data;
    let mut ccname: *const libc::c_char =
        k5_gic_opt_get_fast_ccache_name(opt);
    let mut fast_flags: krb5_flags = 0;
    krb5_clear_error_message(context);
    target_realm = &mut (*(*request).server).realm;
    if !ccname.is_null() {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"FAST armor ccache: {str}\x00" as *const u8 as
                              *const libc::c_char, ccname);
        }
        (*state).fast_state_flags =
            ((*state).fast_state_flags as libc::c_long |
                 (1 as libc::c_long) << 1 as libc::c_int) as krb5_ui_4;
        retval = krb5_cc_resolve(context, ccname, &mut ccache);
        if retval == 0 as libc::c_int {
            retval =
                krb5int_tgtname(context, target_realm, target_realm,
                                &mut target_principal)
        }
        if retval == 0 as libc::c_int {
            let mut config_data: krb5_data =
                krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
            config_data.data = 0 as *mut libc::c_char;
            retval =
                krb5_cc_get_config(context, ccache,
                                   target_principal as krb5_const_principal,
                                   b"fast_avail\x00" as *const u8 as
                                       *const libc::c_char, &mut config_data);
            if retval == 0 as libc::c_int && !config_data.data.is_null() {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"Using FAST due to armor ccache negotiation result\x00"
                                      as *const u8 as *const libc::c_char);
                }
                (*state).fast_state_flags =
                    ((*state).fast_state_flags as libc::c_long |
                         (1 as libc::c_long) << 0 as libc::c_int) as krb5_ui_4
            }
            krb5_free_data_contents(context, &mut config_data);
            retval = 0 as libc::c_int
        }
        fast_flags = k5_gic_opt_get_fast_flags(opt);
        if fast_flags & 0x1 as libc::c_int != 0 {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"Using FAST due to KRB5_FAST_REQUIRED flag\x00"
                                  as *const u8 as *const libc::c_char);
            }
            (*state).fast_state_flags =
                ((*state).fast_state_flags as libc::c_long |
                     (1 as libc::c_long) << 0 as libc::c_int) as krb5_ui_4
        }
        if retval == 0 as libc::c_int &&
               (*state).fast_state_flags as libc::c_long &
                   (1 as libc::c_long) << 0 as libc::c_int != 0 {
            retval =
                fast_armor_ap_request(context, state, ccache,
                                      target_principal)
        }
        if retval != 0 as libc::c_int {
            krb5_prepend_error_message(context, retval,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Error constructing AP-REQ armor\x00"
                                                    as *const u8 as
                                                    *const libc::c_char));
        }
    }
    if !ccache.is_null() { krb5_cc_close(context, ccache); }
    if !target_principal.is_null() {
        krb5_free_principal(context, target_principal);
    }
    return retval;
}
/*
 * Construct a list of outer request padata for a TGS request.  Since we do
 * FAST TGS even when we don't have reason to believe the KDC supports FAST,
 * the outer padata has to contain duplicates of the inner padata (such as
 * S4U2Self padata) as well as the PA-TGS-REQ and PA-FX-FAST padata.  The
 * caller must free *out_padata with free() as it is not a deep copy.
 */
#[c2rust::src_loc = "233:1"]
unsafe extern "C" fn make_tgs_outer_padata(mut tgs: *mut krb5_pa_data,
                                           mut fast: *mut krb5_pa_data,
                                           mut other: *mut *mut krb5_pa_data,
                                           mut out_padata:
                                               *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut pa_list: *mut *mut krb5_pa_data =
        0 as *mut *mut krb5_pa_data; /*owned by state*/
    let mut i: size_t = 0;
    *out_padata = 0 as *mut *mut krb5_pa_data;
    i = 0 as libc::c_int as size_t;
    while !(*other.offset(i as isize)).is_null() { i = i.wrapping_add(1) }
    pa_list =
        calloc(i.wrapping_add(3 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut krb5_pa_data>() as libc::c_ulong) as
            *mut *mut krb5_pa_data;
    if pa_list.is_null() { return 12 as libc::c_int }
    let ref mut fresh0 = *pa_list.offset(0 as libc::c_int as isize);
    *fresh0 = tgs;
    let ref mut fresh1 = *pa_list.offset(1 as libc::c_int as isize);
    *fresh1 = fast;
    i = 0 as libc::c_int as size_t;
    while !(*other.offset(i as isize)).is_null() {
        let ref mut fresh2 =
            *pa_list.offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize);
        *fresh2 = *other.offset(i as isize);
        i = i.wrapping_add(1)
    }
    *out_padata = pa_list;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "253:1"]
pub unsafe extern "C" fn krb5int_fast_prep_req(mut context: krb5_context,
                                               mut state:
                                                   *mut krb5int_fast_request_state,
                                               mut request: *mut krb5_kdc_req,
                                               mut to_be_checksummed:
                                                   *const krb5_data,
                                               mut encoder:
                                                   kdc_req_encoder_proc,
                                               mut encoded_request:
                                                   *mut *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut pa_array: [*mut krb5_pa_data; 2] = [0 as *mut krb5_pa_data; 2];
    let mut pa_tgs_array: *mut *mut krb5_pa_data =
        0 as *mut *mut krb5_pa_data;
    let mut pa: [krb5_pa_data; 2] =
        [krb5_pa_data{magic: 0,
                      pa_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,}; 2];
    let mut fast_req: krb5_fast_req =
        krb5_fast_req{magic: 0,
                      fast_options: 0,
                      req_body: 0 as *mut krb5_kdc_req,};
    let mut tgs: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut armored_req: *mut krb5_fast_armored_req =
        0 as *mut krb5_fast_armored_req;
    let mut encoded_fast_req: *mut krb5_data = 0 as *mut krb5_data;
    let mut encoded_armored_req: *mut krb5_data = 0 as *mut krb5_data;
    let mut local_encoded_result: *mut krb5_data = 0 as *mut krb5_data;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !state.is_null() {
    } else {
        __assert_fail(b"state != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"fast.c\x00" as *const u8 as *const libc::c_char,
                      272 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 160],
                                                &[libc::c_char; 160]>(b"krb5_error_code krb5int_fast_prep_req(krb5_context, struct krb5int_fast_request_state *, krb5_kdc_req *, const krb5_data *, kdc_req_encoder_proc, krb5_data **)\x00")).as_ptr());
    }
    if (*state).fast_outer_request.padata.is_null() {
    } else {
        __assert_fail(b"state->fast_outer_request.padata == NULL\x00" as
                          *const u8 as *const libc::c_char,
                      b"fast.c\x00" as *const u8 as *const libc::c_char,
                      273 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 160],
                                                &[libc::c_char; 160]>(b"krb5_error_code krb5int_fast_prep_req(krb5_context, struct krb5int_fast_request_state *, krb5_kdc_req *, const krb5_data *, kdc_req_encoder_proc, krb5_data **)\x00")).as_ptr());
    }
    memset(pa_array.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[*mut krb5_pa_data; 2]>() as libc::c_ulong);
    if (*state).armor_key.is_null() {
        return encoder.expect("non-null function pointer")(request,
                                                           encoded_request)
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Encoding request body and padata into FAST request\x00"
                          as *const u8 as *const libc::c_char);
    }
    (*state).nonce = (*request).nonce;
    fast_req.req_body = request;
    if (*fast_req.req_body).padata.is_null() {
        (*fast_req.req_body).padata =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<*mut krb5_pa_data>() as
                       libc::c_ulong) as *mut *mut krb5_pa_data;
        if (*fast_req.req_body).padata.is_null() {
            retval = 12 as libc::c_int
        }
    }
    fast_req.fast_options = (*state).fast_options as krb5_flags;
    if retval == 0 as libc::c_int &&
           {
               tgs =
                   krb5int_find_pa_data(context, (*fast_req.req_body).padata,
                                        1 as libc::c_int);
               !tgs.is_null()
           } {
        let mut paptr: *mut *mut krb5_pa_data =
            &mut *(*fast_req.req_body).padata.offset(0 as libc::c_int as
                                                         isize) as
                *mut *mut krb5_pa_data;
        i = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while !(*paptr.offset(j as isize)).is_null() {
            if (**paptr.offset(j as isize)).pa_type == 1 as libc::c_int {
                let ref mut fresh3 = *paptr.offset(j as isize);
                *fresh3 = 0 as *mut krb5_pa_data
            } else {
                let fresh4 = i;
                i = i + 1;
                let ref mut fresh5 = *paptr.offset(fresh4 as isize);
                *fresh5 = *paptr.offset(j as isize)
            }
            j += 1
        }
        let ref mut fresh6 = *paptr.offset(i as isize);
        *fresh6 = 0 as *mut krb5_pa_data
    }
    if retval == 0 as libc::c_int {
        retval = encode_krb5_fast_req(&mut fast_req, &mut encoded_fast_req)
    }
    if retval == 0 as libc::c_int {
        armored_req =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_fast_armored_req>() as
                       libc::c_ulong) as *mut krb5_fast_armored_req;
        if armored_req.is_null() { retval = 12 as libc::c_int }
    }
    if retval == 0 as libc::c_int { (*armored_req).armor = (*state).armor }
    if retval == 0 as libc::c_int {
        retval =
            krb5_c_make_checksum(context, 0 as libc::c_int,
                                 (*state).armor_key, 50 as libc::c_int,
                                 to_be_checksummed,
                                 &mut (*armored_req).req_checksum)
    }
    if retval == 0 as libc::c_int {
        retval =
            krb5_encrypt_helper(context, (*state).armor_key,
                                51 as libc::c_int, encoded_fast_req,
                                &mut (*armored_req).enc_part)
    }
    if retval == 0 as libc::c_int {
        retval =
            encode_krb5_pa_fx_fast_request(armored_req,
                                           &mut encoded_armored_req)
    }
    if retval == 0 as libc::c_int {
        pa[0 as libc::c_int as usize].pa_type = 136 as libc::c_int;
        pa[0 as libc::c_int as usize].contents =
            (*encoded_armored_req).data as *mut libc::c_uchar;
        pa[0 as libc::c_int as usize].length = (*encoded_armored_req).length;
        if !tgs.is_null() {
            retval =
                make_tgs_outer_padata(tgs, pa.as_mut_ptr(), (*request).padata,
                                      &mut pa_tgs_array);
            (*state).fast_outer_request.padata = pa_tgs_array
        } else {
            pa_array[0 as libc::c_int as usize] =
                &mut *pa.as_mut_ptr().offset(0 as libc::c_int as isize) as
                    *mut krb5_pa_data;
            (*state).fast_outer_request.padata = pa_array.as_mut_ptr()
        }
    }
    if retval == 0 as libc::c_int {
        retval =
            encoder.expect("non-null function pointer")(&mut (*state).fast_outer_request,
                                                        &mut local_encoded_result)
    }
    if retval == 0 as libc::c_int {
        *encoded_request = local_encoded_result;
        local_encoded_result = 0 as *mut krb5_data
    }
    if !encoded_armored_req.is_null() {
        krb5_free_data(context, encoded_armored_req);
    }
    if !armored_req.is_null() {
        (*armored_req).armor = 0 as *mut krb5_fast_armor;
        krb5_free_fast_armored_req(context, armored_req);
    }
    if !encoded_fast_req.is_null() {
        krb5_free_data(context, encoded_fast_req);
    }
    if !local_encoded_result.is_null() {
        krb5_free_data(context, local_encoded_result);
    }
    if !tgs.is_null() {
        free((*tgs).contents as *mut libc::c_void);
        free(tgs as *mut libc::c_void);
    }
    (*state).fast_outer_request.padata = 0 as *mut *mut krb5_pa_data;
    free(pa_tgs_array as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "359:1"]
unsafe extern "C" fn decrypt_fast_reply(mut context: krb5_context,
                                        mut state:
                                            *mut krb5int_fast_request_state,
                                        mut in_padata: *mut *mut krb5_pa_data,
                                        mut response:
                                            *mut *mut krb5_fast_response)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut encrypted_response: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    let mut fx_reply: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut local_resp: *mut krb5_fast_response =
        0 as *mut krb5_fast_response;
    if !state.is_null() {
    } else {
        __assert_fail(b"state != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"fast.c\x00" as *const u8 as *const libc::c_char,
                      371 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 126],
                                                &[libc::c_char; 126]>(b"krb5_error_code decrypt_fast_reply(krb5_context, struct krb5int_fast_request_state *, krb5_pa_data **, krb5_fast_response **)\x00")).as_ptr());
    }
    if !(*state).armor_key.is_null() {
    } else {
        __assert_fail(b"state->armor_key\x00" as *const u8 as
                          *const libc::c_char,
                      b"fast.c\x00" as *const u8 as *const libc::c_char,
                      372 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 126],
                                                &[libc::c_char; 126]>(b"krb5_error_code decrypt_fast_reply(krb5_context, struct krb5int_fast_request_state *, krb5_pa_data **, krb5_fast_response **)\x00")).as_ptr());
    }
    fx_reply = krb5int_find_pa_data(context, in_padata, 136 as libc::c_int);
    if fx_reply.is_null() {
        retval = -(1765328132 as libc::c_long) as krb5_error_code
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Decoding FAST response\x00" as *const u8 as
                          *const libc::c_char);
    }
    if retval == 0 as libc::c_int {
        scratch.data = (*fx_reply).contents as *mut libc::c_char;
        scratch.length = (*fx_reply).length;
        retval =
            decode_krb5_pa_fx_fast_reply(&mut scratch,
                                         &mut encrypted_response)
    }
    scratch.data = 0 as *mut libc::c_char;
    if retval == 0 as libc::c_int {
        scratch.data =
            malloc((*encrypted_response).ciphertext.length as libc::c_ulong)
                as *mut libc::c_char;
        if scratch.data.is_null() { retval = 12 as libc::c_int }
        scratch.length = (*encrypted_response).ciphertext.length
    }
    if retval == 0 as libc::c_int {
        retval =
            krb5_c_decrypt(context, (*state).armor_key, 52 as libc::c_int,
                           0 as *const krb5_data, encrypted_response,
                           &mut scratch)
    }
    if retval != 0 as libc::c_int {
        krb5_prepend_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Failed to decrypt FAST reply\x00"
                                                as *const u8 as
                                                *const libc::c_char));
    }
    if retval == 0 as libc::c_int {
        retval = decode_krb5_fast_response(&mut scratch, &mut local_resp)
    }
    if retval == 0 as libc::c_int {
        if (*local_resp).nonce != (*state).nonce {
            retval = -(1765328237 as libc::c_long) as krb5_error_code;
            krb5_set_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"nonce modified in FAST response: KDC response modified\x00"
                                                as *const u8 as
                                                *const libc::c_char));
        }
    }
    if retval == 0 as libc::c_int {
        *response = local_resp;
        local_resp = 0 as *mut krb5_fast_response
    }
    if !scratch.data.is_null() { free(scratch.data as *mut libc::c_void); }
    if !encrypted_response.is_null() {
        krb5_free_enc_data(context, encrypted_response);
    }
    if !local_resp.is_null() { krb5_free_fast_response(context, local_resp); }
    return retval;
}
/*
 * If state contains an armor key and *err_replyptr contains a FAST error,
 * decode it and set *err_replyptr to the inner error and *out_padata to the
 * padata in the FAST response.  Otherwise, leave *err_replyptr alone and set
 * *out_padata to the error e_data decoded as pa-data or typed-data, or to NULL
 * if it doesn't decode as either.  In either case, set *retry to indicate
 * whether the client should try to make a follow-up request.
 */
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn krb5int_fast_process_error(mut context: krb5_context,
                                                    mut state:
                                                        *mut krb5int_fast_request_state,
                                                    mut err_replyptr:
                                                        *mut *mut krb5_error,
                                                    mut out_padata:
                                                        *mut *mut *mut krb5_pa_data,
                                                    mut retry:
                                                        *mut krb5_boolean)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int; /*not FAST*/
    let mut err_reply: *mut krb5_error = *err_replyptr;
    let mut fx_error_pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut result: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut scratch: krb5_data = empty_data();
    let mut fx_error: *mut krb5_error = 0 as *mut krb5_error;
    let mut fast_response: *mut krb5_fast_response =
        0 as *mut krb5_fast_response;
    if !out_padata.is_null() { *out_padata = 0 as *mut *mut krb5_pa_data }
    if !retry.is_null() { *retry = 0 as libc::c_int as krb5_boolean }
    if !(*state).armor_key.is_null() {
        retval =
            decode_krb5_padata_sequence(&mut (*err_reply).e_data,
                                        &mut result);
        if retval == 0 as libc::c_int {
            retval =
                decrypt_fast_reply(context, state, result, &mut fast_response)
        }
        if retval != 0 {
            /*
             * This can happen if the KDC does not understand FAST. We don't
             * expect that, but treating it as the fatal error indicated by the
             * KDC seems reasonable.
             */
            if !retry.is_null() { *retry = 0 as libc::c_int as krb5_boolean }
            krb5_free_pa_data(context, result);
            return 0 as libc::c_int
        }
        if retval == 0 as libc::c_int {
            fx_error_pa =
                krb5int_find_pa_data(context, (*fast_response).padata,
                                     137 as libc::c_int);
            if fx_error_pa.is_null() {
                krb5_set_error_message(context,
                                       -(1765328360 as libc::c_long) as
                                           krb5_error_code,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Expecting FX_ERROR pa-data inside FAST container\x00"
                                                    as *const u8 as
                                                    *const libc::c_char));
                retval = -(1765328360 as libc::c_long) as krb5_error_code
            }
        }
        if retval == 0 as libc::c_int {
            scratch =
                make_data((*fx_error_pa).contents as *mut libc::c_void,
                          (*fx_error_pa).length);
            retval = decode_krb5_error(&mut scratch, &mut fx_error)
        }
        if retval == 0 as libc::c_int {
            krb5_free_error(context, err_reply);
            *err_replyptr = fx_error;
            fx_error = 0 as *mut krb5_error;
            if !out_padata.is_null() {
                *out_padata = (*fast_response).padata;
                (*fast_response).padata = 0 as *mut *mut krb5_pa_data
            }
            /*
             * If there is more than the fx_error padata, then we want
             * to retry the error if a cookie is present
             */
            if !retry.is_null() {
                *retry =
                    (*(*out_padata).offset(1 as libc::c_int as isize) !=
                         0 as *mut libc::c_void as *mut krb5_pa_data) as
                        libc::c_int as krb5_boolean;
                if krb5int_find_pa_data(context, *out_padata,
                                        133 as libc::c_int).is_null() {
                    *retry = 0 as libc::c_int as krb5_boolean
                }
            }
        }
    } else {
        /* Possibly retry if there's any e_data to process. */
        if !retry.is_null() {
            *retry =
                ((*err_reply).e_data.length >
                     0 as libc::c_int as libc::c_uint) as libc::c_int as
                    krb5_boolean
        }
        /* Try to decode e_data as pa-data or typed-data for out_padata. */
        if !out_padata.is_null() {
            retval =
                decode_krb5_padata_sequence(&mut (*err_reply).e_data,
                                            out_padata);
            if retval != 0 as libc::c_int {
                decode_krb5_typed_data(&mut (*err_reply).e_data, out_padata);
                retval = 0 as libc::c_int
            }
        }
    }
    krb5_free_pa_data(context, result);
    krb5_free_fast_response(context, fast_response);
    if !fx_error.is_null() { krb5_free_error(context, fx_error); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "516:1"]
pub unsafe extern "C" fn krb5int_fast_process_response(mut context:
                                                           krb5_context,
                                                       mut state:
                                                           *mut krb5int_fast_request_state,
                                                       mut resp:
                                                           *mut krb5_kdc_rep,
                                                       mut strengthen_key:
                                                           *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut fast_response: *mut krb5_fast_response =
        0 as *mut krb5_fast_response;
    let mut encoded_ticket: *mut krb5_data = 0 as *mut krb5_data;
    let mut cksum_valid: krb5_boolean = 0;
    krb5_clear_error_message(context);
    *strengthen_key = 0 as *mut krb5_keyblock;
    if (*state).armor_key.is_null() { return 0 as libc::c_int }
    retval =
        decrypt_fast_reply(context, state, (*resp).padata,
                           &mut fast_response);
    if retval == 0 as libc::c_int {
        if (*fast_response).finished.is_null() {
            retval = -(1765328237 as libc::c_long) as krb5_error_code;
            krb5_set_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"FAST response missing finish message in KDC reply\x00"
                                                as *const u8 as
                                                *const libc::c_char));
        }
    }
    if retval == 0 as libc::c_int {
        retval = encode_krb5_ticket((*resp).ticket, &mut encoded_ticket)
    }
    if retval == 0 as libc::c_int {
        retval =
            krb5_c_verify_checksum(context, (*state).armor_key,
                                   53 as libc::c_int, encoded_ticket,
                                   &mut (*(*fast_response).finished).ticket_checksum,
                                   &mut cksum_valid)
    }
    if retval == 0 as libc::c_int &&
           cksum_valid == 0 as libc::c_int as libc::c_uint {
        retval = -(1765328237 as libc::c_long) as krb5_error_code;
        krb5_set_error_message(context, retval,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Ticket modified in KDC reply\x00" as
                                            *const u8 as
                                            *const libc::c_char));
    }
    if retval == 0 as libc::c_int {
        krb5_free_principal(context, (*resp).client);
        (*resp).client = (*(*fast_response).finished).client;
        (*(*fast_response).finished).client = 0 as krb5_principal;
        *strengthen_key = (*fast_response).strengthen_key;
        (*fast_response).strengthen_key = 0 as *mut krb5_keyblock;
        krb5_free_pa_data(context, (*resp).padata);
        (*resp).padata = (*fast_response).padata;
        (*fast_response).padata = 0 as *mut *mut krb5_pa_data
    }
    if !fast_response.is_null() {
        krb5_free_fast_response(context, fast_response);
    }
    if !encoded_ticket.is_null() { krb5_free_data(context, encoded_ticket); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "569:1"]
pub unsafe extern "C" fn krb5int_fast_reply_key(mut context: krb5_context,
                                                mut strengthen_key:
                                                    *const krb5_keyblock,
                                                mut existing_key:
                                                    *const krb5_keyblock,
                                                mut out_key:
                                                    *mut krb5_keyblock)
 -> krb5_error_code {
    let mut key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    krb5_free_keyblock_contents(context, out_key);
    if !strengthen_key.is_null() {
        retval =
            krb5_c_fx_cf2_simple(context,
                                 strengthen_key as *mut krb5_keyblock,
                                 b"strengthenkey\x00" as *const u8 as
                                     *const libc::c_char,
                                 existing_key as *mut krb5_keyblock,
                                 b"replykey\x00" as *const u8 as
                                     *const libc::c_char, &mut key);
        if retval == 0 as libc::c_int {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"FAST reply key: {keyblock}\x00" as *const u8
                                  as *const libc::c_char, key);
            }
            *out_key = *key;
            free(key as *mut libc::c_void);
        }
    } else {
        retval = krb5_copy_keyblock_contents(context, existing_key, out_key)
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "595:1"]
pub unsafe extern "C" fn krb5int_fast_make_state(mut context: krb5_context,
                                                 mut state:
                                                     *mut *mut krb5int_fast_request_state)
 -> krb5_error_code {
    let mut local_state: *mut krb5int_fast_request_state =
        0 as *mut krb5int_fast_request_state;
    local_state =
        malloc(::std::mem::size_of::<krb5int_fast_request_state>() as
                   libc::c_ulong) as *mut krb5int_fast_request_state;
    if local_state.is_null() { return 12 as libc::c_int }
    memset(local_state as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5int_fast_request_state>() as
               libc::c_ulong);
    *state = local_state;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "609:1"]
pub unsafe extern "C" fn krb5int_fast_free_state(mut context: krb5_context,
                                                 mut state:
                                                     *mut krb5int_fast_request_state) {
    if state.is_null() { return }
    /*We are responsible for none of the store in the fast_outer_req*/
    krb5_free_keyblock(context, (*state).armor_key);
    krb5_free_fast_armor(context, (*state).armor);
    free(state as *mut libc::c_void);
}
/*
 * Implement FAST negotiation as specified in RFC 6806 section 11.  If
 * the encrypted part of rep sets the enc-pa-rep flag, look for and
 * verify a PA-REQ-ENC-PA-REP entry in the encrypted padata.  If a
 * PA-FX-FAST entry is also present in the encrypted padata, set
 * *fast_avail to true.  This will result in a fast_avail config entry
 * being written to the credential cache, if an output ccache was
 * specified using krb5_get_init_creds_opt_set_out_ccache().  That
 * entry will be detected in the armor ccache by
 * krb5int_fast_as_armor(), allowing us to use FAST without a
 * round-trip for the KDC to indicate support, and without a downgrade
 * attack.
 */
#[no_mangle]
#[c2rust::src_loc = "634:1"]
pub unsafe extern "C" fn krb5int_fast_verify_nego(mut context: krb5_context,
                                                  mut state:
                                                      *mut krb5int_fast_request_state,
                                                  mut rep: *mut krb5_kdc_rep,
                                                  mut request: *mut krb5_data,
                                                  mut decrypting_key:
                                                      *mut krb5_keyblock,
                                                  mut fast_avail:
                                                      *mut krb5_boolean)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        0 as libc::c_int; /* Already using FAST. */
    let mut checksum: *mut krb5_checksum = 0 as *mut krb5_checksum;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut valid: krb5_boolean = 0;
    *fast_avail = 0 as libc::c_int as krb5_boolean;
    if (*(*rep).enc_part2).flags & 0x10000 as libc::c_int != 0 {
        pa =
            krb5int_find_pa_data(context, (*(*rep).enc_part2).enc_padata,
                                 149 as libc::c_int);
        if pa.is_null() {
            retval = -(1765328237 as libc::c_long) as krb5_error_code
        } else {
            scratch.data = (*pa).contents as *mut libc::c_char;
            scratch.length = (*pa).length
        }
        if retval == 0 as libc::c_int {
            retval = decode_krb5_checksum(&mut scratch, &mut checksum)
        }
        if retval == 0 as libc::c_int {
            retval =
                krb5_c_verify_checksum(context, decrypting_key,
                                       56 as libc::c_int, request, checksum,
                                       &mut valid)
        }
        if retval == 0 as libc::c_int &&
               valid == 0 as libc::c_int as libc::c_uint {
            retval = -(1765328237 as libc::c_long) as krb5_error_code
        }
        if retval == 0 as libc::c_int {
            pa =
                krb5int_find_pa_data(context, (*(*rep).enc_part2).enc_padata,
                                     136 as libc::c_int);
            *fast_avail =
                (pa != 0 as *mut libc::c_void as *mut krb5_pa_data) as
                    libc::c_int as krb5_boolean
        }
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"FAST negotiation: {str}available\x00" as *const u8 as
                          *const libc::c_char,
                      if *fast_avail != 0 {
                          b"\x00" as *const u8 as *const libc::c_char
                      } else {
                          b"un\x00" as *const u8 as *const libc::c_char
                      });
    }
    if !checksum.is_null() { krb5_free_checksum(context, checksum); }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "677:1"]
pub unsafe extern "C" fn k5_upgrade_to_fast_p(mut context: krb5_context,
                                              mut state:
                                                  *mut krb5int_fast_request_state,
                                              mut padata:
                                                  *mut *mut krb5_pa_data)
 -> krb5_boolean {
    if !(*state).armor_key.is_null() {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*state).fast_state_flags as libc::c_long &
           (1 as libc::c_long) << 1 as libc::c_int == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    if !krb5int_find_pa_data(context, padata, 136 as libc::c_int).is_null() {
        return 1 as libc::c_int as krb5_boolean
    }
    return 0 as libc::c_int as krb5_boolean;
}
