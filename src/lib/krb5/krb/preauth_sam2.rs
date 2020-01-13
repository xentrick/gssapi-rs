use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    #[c2rust::src_loc = "7313:1"]
    pub type krb5_init_creds_context = *mut _krb5_init_creds_context;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::init_creds_ctx_h::_krb5_init_creds_context;
    use super::stddef_h::size_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "531:1"]
        pub fn krb5_c_encrypt(context: krb5_context,
                              key: *const krb5_keyblock, usage: krb5_keyusage,
                              cipher_state: *const krb5_data,
                              input: *const krb5_data,
                              output: *mut krb5_enc_data) -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "578:1"]
        pub fn krb5_c_encrypt_length(context: krb5_context,
                                     enctype: krb5_enctype, inputlen: size_t,
                                     length: *mut size_t) -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "839:1"]
        pub fn krb5_c_string_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    string: *const krb5_data,
                                    salt: *const krb5_data,
                                    key: *mut krb5_keyblock)
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
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        /* *
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:28"]
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
    #[c2rust::src_loc = "460:16"]
    pub struct _krb5_sam_challenge_2 {
        pub sam_challenge_2_body: krb5_data,
        pub sam_cksum: *mut *mut krb5_checksum,
    }
    #[c2rust::src_loc = "460:1"]
    pub type krb5_sam_challenge_2 = _krb5_sam_challenge_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "465:16"]
    pub struct _krb5_sam_challenge_2_body {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_type_name: krb5_data,
        pub sam_track_id: krb5_data,
        pub sam_challenge_label: krb5_data,
        pub sam_challenge: krb5_data,
        pub sam_response_prompt: krb5_data,
        pub sam_pk_for_sad: krb5_data,
        pub sam_nonce: krb5_int32,
        pub sam_etype: krb5_enctype,
    }
    #[c2rust::src_loc = "465:1"]
    pub type krb5_sam_challenge_2_body = _krb5_sam_challenge_2_body;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "479:16"]
    pub struct _krb5_sam_response_2 {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_track_id: krb5_data,
        pub sam_enc_nonce_or_sad: krb5_enc_data,
        pub sam_nonce: krb5_int32,
    }
    #[c2rust::src_loc = "479:1"]
    pub type krb5_sam_response_2 = _krb5_sam_response_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "488:16"]
    pub struct _krb5_enc_sam_response_enc_2 {
        pub magic: krb5_magic,
        pub sam_nonce: krb5_int32,
        pub sam_sad: krb5_data,
    }
    #[c2rust::src_loc = "488:1"]
    pub type krb5_enc_sam_response_enc_2 = _krb5_enc_sam_response_enc_2;
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_checksum, krb5_enc_data, krb5_context,
                        krb5_error_code};
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
        #[c2rust::src_loc = "904:1"]
        pub fn krb5_free_sam_challenge_2(_: krb5_context,
                                         _: *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "907:1"]
        pub fn krb5_free_sam_challenge_2_body(_: krb5_context,
                                              _:
                                                  *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "1467:1"]
        pub fn encode_krb5_enc_sam_response_enc_2(_:
                                                      *const krb5_enc_sam_response_enc_2,
                                                  _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1471:1"]
        pub fn encode_krb5_sam_response_2(_: *const krb5_sam_response_2,
                                          _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1554:1"]
        pub fn decode_krb5_sam_challenge_2(_: *const krb5_data,
                                           _: *mut *mut krb5_sam_challenge_2)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1557:1"]
        pub fn decode_krb5_sam_challenge_2_body(_: *const krb5_data,
                                                _:
                                                    *mut *mut krb5_sam_challenge_2_body)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:28"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/init_creds_ctx.h:32"]
pub mod init_creds_ctx_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct krb5_responder_context_st {
        pub items: *mut k5_response_items,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:8"]
    pub struct _krb5_init_creds_context {
        pub opt: *mut krb5_get_init_creds_opt,
        pub opt_storage: krb5_get_init_creds_opt,
        pub identify_realm: krb5_boolean,
        pub subject_cert: *const krb5_data,
        pub in_tkt_service: *mut libc::c_char,
        pub prompter: krb5_prompter_fct,
        pub prompter_data: *mut libc::c_void,
        pub gak_fct: get_as_key_fn,
        pub gak_data: *mut libc::c_void,
        pub request_time: krb5_timestamp,
        pub start_time: krb5_deltat,
        pub tkt_life: krb5_deltat,
        pub renew_life: krb5_deltat,
        pub complete: krb5_boolean,
        pub loopcount: libc::c_uint,
        pub gakpw: gak_password,
        pub err_reply: *mut krb5_error,
        pub err_padata: *mut *mut krb5_pa_data,
        pub cred: krb5_creds,
        pub request: *mut krb5_kdc_req,
        pub reply: *mut krb5_kdc_rep,
        pub outer_request_body: *mut krb5_data,
        pub inner_request_body: *mut krb5_data,
        pub encoded_previous_request: *mut krb5_data,
        pub fast_state: *mut krb5int_fast_request_state,
        pub optimistic_padata: *mut *mut krb5_pa_data,
        pub method_padata: *mut *mut krb5_pa_data,
        pub more_padata: *mut *mut krb5_pa_data,
        pub default_salt: krb5_boolean,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
        pub as_key: krb5_keyblock,
        pub etype: krb5_enctype,
        pub info_pa_permitted: krb5_boolean,
        pub restarted: krb5_boolean,
        pub fallback_disabled: krb5_boolean,
        pub encts_disabled: krb5_boolean,
        pub rctx: krb5_responder_context_st,
        pub selected_preauth_type: krb5_preauthtype,
        pub allowed_preauth_type: krb5_preauthtype,
        pub cc_config_in: k5_json_object,
        pub cc_config_out: k5_json_object,
        pub pa_offset: krb5_timestamp,
        pub pa_offset_usec: krb5_int32,
        pub pa_offset_state: C2RustUnnamed,
        pub preauth_reqctx: krb5_preauth_req_context,
    }
    #[c2rust::src_loc = "9:1"]
    pub type krb5_preauth_req_context = *mut krb5_preauth_req_context_st;
    #[c2rust::src_loc = "75:5"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "75:42"]
    pub const AUTH_OFFSET: C2RustUnnamed = 2;
    #[c2rust::src_loc = "75:27"]
    pub const UNAUTH_OFFSET: C2RustUnnamed = 1;
    #[c2rust::src_loc = "75:12"]
    pub const NO_OFFSET: C2RustUnnamed = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:8"]
    pub struct gak_password {
        pub storage: krb5_data,
        pub password: *const krb5_data,
    }
    use super::int_proto_h::{k5_response_items, get_as_key_fn,
                             krb5int_fast_request_state};
    use super::krb5_h::{krb5_get_init_creds_opt, krb5_boolean, krb5_data,
                        krb5_prompter_fct, krb5_timestamp, krb5_deltat,
                        krb5_error, krb5_pa_data, krb5_creds, krb5_kdc_req,
                        krb5_kdc_rep, krb5_keyblock, krb5_enctype,
                        krb5_preauthtype, krb5_int32};
    use super::k5_json_h::k5_json_object;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
        #[c2rust::src_loc = "9:16"]
        pub type krb5_preauth_req_context_st;
    }
    /* !KRB5_INIT_CREDS_CONTEXT */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:30"]
pub mod int_proto_h {
    #[c2rust::src_loc = "32:1"]
    pub type k5_response_items = k5_response_items_st;
    #[c2rust::src_loc = "34:1"]
    pub type get_as_key_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_principal,
                                    _: krb5_enctype, _: krb5_prompter_fct,
                                    _: *mut libc::c_void, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut krb5_keyblock,
                                    _: *mut libc::c_void,
                                    _: *mut k5_response_items)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_principal,
                        krb5_enctype, krb5_prompter_fct, krb5_data,
                        krb5_keyblock};
    extern "C" {
        #[c2rust::src_loc = "32:16"]
        pub type k5_response_items_st;
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/int-proto.h - Prototypes for libkrb5 internal functions */
/*
 * Copyright 1990,1991 the Massachusetts Institute of Technology.
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
        #[c2rust::src_loc = "30:8"]
        pub type krb5int_fast_request_state;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:32"]
pub mod k5_json_h {
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    extern "C" {
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:28"]
pub mod plugin_h {
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
        /* Generic declarations for dynamic modules implementing krb5 plugin
 * modules. */
        /* krb5_plugin_vtable is an abstract type.  Module initvt functions will cast
 * it to the appropriate interface-specific vtable type. */
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:29"]
pub mod clpreauth_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 2006 Red Hat, Inc.
 * Portions copyright (c) 2006, 2011 Massachusetts Institute of Technology
 * All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *  * Neither the name of Red Hat, Inc., nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    /*
 * Declarations for clpreauth plugin module implementors.
 *
 * The clpreauth interface has a single supported major version, which is
 * 1.  Major version 1 has a current minor version of 2.  clpreauth modules
 * should define a function named clpreauth_<modulename>_initvt, matching
 * the signature:
 *
 *   krb5_error_code
 *   clpreauth_modname_initvt(krb5_context context, int maj_ver,
 *                            int min_ver, krb5_plugin_vtable vtable);
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     maj_ver == 1: Cast to krb5_clpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* clpreauth mechanism property flags */
    /* Provides a real answer which we can send back to the KDC.  The client
 * assumes that one real answer will be enough. */
    /* Doesn't provide a real answer, but must be given a chance to run before any
 * REAL mechanism callbacks. */
    /* Abstract type for a client request information handle. */
    #[c2rust::src_loc = "75:1"]
    pub type krb5_clpreauth_rock = *mut krb5_clpreauth_rock_st;
    /* Abstract types for module data and per-request module data. */
    #[c2rust::src_loc = "78:1"]
    pub type krb5_clpreauth_moddata = *mut krb5_clpreauth_moddata_st;
    #[c2rust::src_loc = "79:1"]
    pub type krb5_clpreauth_modreq = *mut krb5_clpreauth_modreq_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:16"]
    pub struct krb5_clpreauth_callbacks_st {
        pub vers: libc::c_int,
        pub get_etype: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_clpreauth_rock)
                                  -> krb5_enctype>,
        pub fast_armor: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_clpreauth_rock)
                                   -> *mut krb5_keyblock>,
        pub get_as_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_clpreauth_rock,
                                                    _:
                                                        *mut *mut krb5_keyblock)
                                   -> krb5_error_code>,
        pub set_as_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_clpreauth_rock,
                                                    _: *const krb5_keyblock)
                                   -> krb5_error_code>,
        pub get_preauth_time: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_clpreauth_rock,
                                                          _: krb5_boolean,
                                                          _:
                                                              *mut krb5_timestamp,
                                                          _: *mut krb5_int32)
                                         -> krb5_error_code>,
        pub ask_responder_question: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    krb5_clpreauth_rock,
                                                                _:
                                                                    *const libc::c_char,
                                                                _:
                                                                    *const libc::c_char)
                                               -> krb5_error_code>,
        pub get_responder_answer: Option<unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_clpreauth_rock,
                                                              _:
                                                                  *const libc::c_char)
                                             -> *const libc::c_char>,
        pub need_as_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_clpreauth_rock)
                                    -> ()>,
        pub get_cc_config: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_clpreauth_rock,
                                                       _: *const libc::c_char)
                                      -> *const libc::c_char>,
        pub set_cc_config: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_clpreauth_rock,
                                                       _: *const libc::c_char,
                                                       _: *const libc::c_char)
                                      -> krb5_error_code>,
        pub disable_fallback: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_clpreauth_rock)
                                         -> ()>,
    }
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    #[c2rust::src_loc = "83:1"]
    pub type krb5_clpreauth_callbacks = *mut krb5_clpreauth_callbacks_st;
    /*
     * If an AS-REP has been received, return the enctype of the AS-REP
     * encrypted part.  Otherwise return the enctype chosen from etype-info, or
     * the first requested enctype if no etype-info was received.
     */
    /* Get a pointer to the FAST armor key, or NULL if the client is not using
     * FAST.  The returned pointer is an alias and should not be freed. */
    /*
     * Get a pointer to the client-supplied reply key, possibly invoking the
     * prompter to ask for a password if this has not already been done.  The
     * returned pointer is an alias and should not be freed.
     */
    /* Replace the reply key to be used to decrypt the AS response. */
    /* End of version 1 clpreauth callbacks. */
    /*
     * Get the current time for use in a preauth response.  If
     * allow_unauth_time is true and the library has been configured to allow
     * it, the current time will be offset using unauthenticated timestamp
     * information received from the KDC in the preauth-required error, if one
     * has been received.  Otherwise, the timestamp in a preauth-required error
     * will only be used if it is protected by a FAST channel.  Only set
     * allow_unauth_time if using an unauthenticated time offset would not
     * create a security issue.
     */
    /* Set a question to be answered by the responder and optionally provide
     * a challenge. */
    /* Get an answer from the responder, or NULL if the question was
     * unanswered. */
    /* Indicate interest in the AS key through the responder interface. */
    /*
     * Get a configuration/state item from an input ccache, which may allow it
     * to retrace the steps it took last time.  The returned data string is an
     * alias and should not be freed.
     */
    /*
     * Set a configuration/state item which will be recorded to an output
     * ccache, if the calling application supplied one.  Both key and data
     * should be valid UTF-8 text.
     */
    /* End of version 2 clpreauth callbacks (added in 1.11). */
    /*
     * Prevent further fallbacks to other preauth mechanisms if the KDC replies
     * with an error.  (The module itself can still respond to errors with its
     * tryagain method, or continue after KDC_ERR_MORE_PREAUTH_DATA_REQUIRED
     * errors with its process method.)  A module should invoke this callback
     * from the process method when it generates an authenticated request using
     * credentials; often this will be the first or only client message
     * generated by the mechanism.
     */
    /* End of version 3 clpreauth callbacks (added in 1.17). */
    /*
 * Optional: per-plugin initialization/cleanup.  The init function is called by
 * libkrb5 when the plugin is loaded, and the fini function is called before
 * the plugin is unloaded.  These may be called multiple times in case the
 * plugin is used in multiple contexts.  The returned context lives the
 * lifetime of the krb5_context.
 */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_clpreauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_clpreauth_moddata)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "189:1"]
    pub type krb5_clpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata) -> ()>;
    /*
 * Optional (mandatory before MIT krb5 1.12): pa_type will be a member of the
 * vtable's pa_type_list.  Return PA_REAL if pa_type is a real
 * preauthentication type or PA_INFO if it is an informational type.  If this
 * function is not defined in 1.12 or later, all pa_type values advertised by
 * the module will be assumed to be real.
 */
    #[c2rust::src_loc = "200:1"]
    pub type krb5_clpreauth_get_flags_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                   -> libc::c_int>;
    /*
 * Optional: per-request initialization/cleanup.  The request_init function is
 * called when beginning to process a get_init_creds request and the
 * request_fini function is called when processing of the request is complete.
 * This is optional.  It may be called multiple times in the lifetime of a
 * krb5_context.
 */
    #[c2rust::src_loc = "210:1"]
    pub type krb5_clpreauth_request_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: *mut krb5_clpreauth_modreq) -> ()>;
    #[c2rust::src_loc = "214:1"]
    pub type krb5_clpreauth_request_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq) -> ()>;
    /*
 * Optional: process server-supplied data in pa_data and set responder
 * questions.
 *
 * encoded_previous_request may be NULL if there has been no previous request
 * in the AS exchange.
 */
    #[c2rust::src_loc = "226:1"]
    pub type krb5_clpreauth_prep_questions_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq,
                                    _: *mut krb5_get_init_creds_opt,
                                    _: krb5_clpreauth_callbacks,
                                    _: krb5_clpreauth_rock,
                                    _: *mut krb5_kdc_req, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut krb5_pa_data)
                   -> krb5_error_code>;
    /*
 * Mandatory: process server-supplied data in pa_data and return created data
 * in pa_data_out.  Also called after the AS-REP is received if the AS-REP
 * includes preauthentication data of the associated type.
 *
 * as_key contains the client-supplied key if known, or an empty keyblock if
 * not.  If it is empty, the module may use gak_fct to fill it in.
 *
 * encoded_previous_request may be NULL if there has been no previous request
 * in the AS exchange.
 */
    #[c2rust::src_loc = "249:1"]
    pub type krb5_clpreauth_process_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq,
                                    _: *mut krb5_get_init_creds_opt,
                                    _: krb5_clpreauth_callbacks,
                                    _: krb5_clpreauth_rock,
                                    _: *mut krb5_kdc_req, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut krb5_pa_data,
                                    _: krb5_prompter_fct,
                                    _: *mut libc::c_void,
                                    _: *mut *mut *mut krb5_pa_data)
                   -> krb5_error_code>;
    /*
 * Optional: Attempt to use error and error_padata to try to recover from the
 * given error.  To work with both FAST and non-FAST errors, an implementation
 * should generally consult error_padata rather than decoding error->e_data.
 * For non-FAST errors, it contains the e_data decoded as either pa-data or
 * typed-data.
 *
 * If this function is provided, and it returns 0 and stores data in
 * pa_data_out, then the client library will retransmit the request.
 */
    #[c2rust::src_loc = "273:1"]
    pub type krb5_clpreauth_tryagain_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq,
                                    _: *mut krb5_get_init_creds_opt,
                                    _: krb5_clpreauth_callbacks,
                                    _: krb5_clpreauth_rock,
                                    _: *mut krb5_kdc_req, _: *mut krb5_data,
                                    _: *mut krb5_data, _: krb5_preauthtype,
                                    _: *mut krb5_error,
                                    _: *mut *mut krb5_pa_data,
                                    _: krb5_prompter_fct,
                                    _: *mut libc::c_void,
                                    _: *mut *mut *mut krb5_pa_data)
                   -> krb5_error_code>;
    /*
 * Optional: receive krb5_get_init_creds_opt information.  The attr and value
 * information supplied should be copied into moddata by the module if it
 * wishes to reference it after returning from this call.
 */
    #[c2rust::src_loc = "294:1"]
    pub type krb5_clpreauth_supply_gic_opts_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: *mut krb5_get_init_creds_opt,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "300:16"]
    pub struct krb5_clpreauth_vtable_st {
        pub name: *mut libc::c_char,
        pub pa_type_list: *mut krb5_preauthtype,
        pub enctype_list: *mut krb5_enctype,
        pub init: krb5_clpreauth_init_fn,
        pub fini: krb5_clpreauth_fini_fn,
        pub flags: krb5_clpreauth_get_flags_fn,
        pub request_init: krb5_clpreauth_request_init_fn,
        pub request_fini: krb5_clpreauth_request_fini_fn,
        pub process: krb5_clpreauth_process_fn,
        pub tryagain: krb5_clpreauth_tryagain_fn,
        pub gic_opts: krb5_clpreauth_supply_gic_opts_fn,
        pub prep_questions: krb5_clpreauth_prep_questions_fn,
    }
    #[c2rust::src_loc = "300:1"]
    pub type krb5_clpreauth_vtable = *mut krb5_clpreauth_vtable_st;
    use super::krb5_h::{krb5_enctype, krb5_context, krb5_keyblock,
                        krb5_error_code, krb5_boolean, krb5_timestamp,
                        krb5_int32, krb5_preauthtype, krb5_get_init_creds_opt,
                        krb5_kdc_req, krb5_data, krb5_pa_data,
                        krb5_prompter_fct, krb5_error};
    extern "C" {
        #[c2rust::src_loc = "75:16"]
        pub type krb5_clpreauth_rock_st;
        #[c2rust::src_loc = "78:16"]
        pub type krb5_clpreauth_moddata_st;
        #[c2rust::src_loc = "79:16"]
        pub type krb5_clpreauth_modreq_st;
    }
    /* Mandatory: name of module. */
    /* Mandatory: pointer to zero-terminated list of pa_types which this module
     * can provide services for. */
    /* Optional: pointer to zero-terminated list of enc_types which this module
     * claims to add support for. */
    /* Minor version 1 ends here. */
    /* Minor version 2 ends here. */
    /* KRB5_CLPREAUTH_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
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
#[c2rust::header_src = "/usr/include/stdio.h:28"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:28"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:31"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_prompt_type};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn k5_set_prompt_types(_: krb5_context, _: *mut krb5_prompt_type);
    }
    /* KRB5_LIBOS_INT_PROTO__ */
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
                       _krb5_ticket, krb5_ticket, _krb5_creds, krb5_creds,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_prompt, krb5_prompt,
                       krb5_prompter_fct, _krb5_get_init_creds_opt,
                       krb5_get_init_creds_opt, krb5_init_creds_context,
                       _profile_t, krb5_c_encrypt, krb5_c_encrypt_length,
                       krb5_c_string_to_key, krb5_c_verify_checksum,
                       krb5_c_valid_enctype, krb5_c_is_keyed_cksum,
                       krb5_principal2salt, krb5_free_keyblock_contents,
                       krb5_free_data, krb5_free_data_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_sam_challenge_2,
                         krb5_sam_challenge_2, _krb5_sam_challenge_2_body,
                         krb5_sam_challenge_2_body, _krb5_sam_response_2,
                         krb5_sam_response_2, _krb5_enc_sam_response_enc_2,
                         krb5_enc_sam_response_enc_2, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_free_sam_challenge_2,
                         krb5_free_sam_challenge_2_body,
                         encode_krb5_enc_sam_response_enc_2,
                         encode_krb5_sam_response_2,
                         decode_krb5_sam_challenge_2,
                         decode_krb5_sam_challenge_2_body};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::init_creds_ctx_h::{krb5_responder_context_st,
                                 _krb5_init_creds_context,
                                 krb5_preauth_req_context, C2RustUnnamed,
                                 AUTH_OFFSET, UNAUTH_OFFSET, NO_OFFSET,
                                 gak_password, krb5_preauth_req_context_st};
pub use self::int_proto_h::{k5_response_items, get_as_key_fn,
                            k5_response_items_st, krb5int_fast_request_state};
pub use self::k5_json_h::{k5_json_object, k5_json_object_st};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::clpreauth_plugin_h::{krb5_clpreauth_rock,
                                   krb5_clpreauth_moddata,
                                   krb5_clpreauth_modreq,
                                   krb5_clpreauth_callbacks_st,
                                   krb5_clpreauth_callbacks,
                                   krb5_clpreauth_init_fn,
                                   krb5_clpreauth_fini_fn,
                                   krb5_clpreauth_get_flags_fn,
                                   krb5_clpreauth_request_init_fn,
                                   krb5_clpreauth_request_fini_fn,
                                   krb5_clpreauth_prep_questions_fn,
                                   krb5_clpreauth_process_fn,
                                   krb5_clpreauth_tryagain_fn,
                                   krb5_clpreauth_supply_gic_opts_fn,
                                   krb5_clpreauth_vtable_st,
                                   krb5_clpreauth_vtable,
                                   krb5_clpreauth_rock_st,
                                   krb5_clpreauth_moddata_st,
                                   krb5_clpreauth_modreq_st};
use self::stdlib_h::{malloc, free};
use self::stdio_h::snprintf;
use self::libintl_h::dgettext;
use self::string_h::{strlen, memset};
use self::os_proto_h::k5_set_prompt_types;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/preauth_sam2.c - SAM-2 clpreauth module */
/*
 * Copyright 1995, 2003, 2008, 2012 by the Massachusetts Institute of Technology.  All
 * Rights Reserved.
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
 *
 */
/* this macro expands to the int,ptr necessary for "%.*s" in an sprintf */
#[c2rust::src_loc = "42:1"]
unsafe extern "C" fn sam_challenge_banner(mut sam_type: krb5_int32)
 -> *mut libc::c_char {
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    match sam_type {
        1 => {
            /* Enigma Logic */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Enigma Logic mechanism\x00" as
                             *const u8 as *const libc::c_char)
        }
        2 | 8 => {
            /*  Digital Pathways */
            /*  Digital Pathways */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Digital Pathways mechanism\x00" as
                             *const u8 as *const libc::c_char)
        }
        6 | 7 => {
            /*  Digital Pathways */
            /*  Digital Pathways */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Activcard mechanism\x00" as *const u8
                             as *const libc::c_char)
        }
        3 => {
            /*  S/key where  KDC has key 0 */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Enhanced S/Key mechanism\x00" as
                             *const u8 as *const libc::c_char)
        }
        4 => {
            /*  Traditional S/Key */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Traditional S/Key mechanism\x00" as
                             *const u8 as *const libc::c_char)
        }
        5 => {
            /*  Security Dynamics */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Security Dynamics mechanism\x00" as
                             *const u8 as *const libc::c_char)
        }
        129 => {
            /* predictive Security Dynamics */
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge for Security Dynamics mechanism\x00" as
                             *const u8 as *const libc::c_char)
        }
        _ => {
            label =
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Challenge from authentication server\x00" as
                             *const u8 as *const libc::c_char)
        }
    }
    return label;
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn sam2_process(mut context: krb5_context,
                                  mut moddata: krb5_clpreauth_moddata,
                                  mut modreq: krb5_clpreauth_modreq,
                                  mut opt: *mut krb5_get_init_creds_opt,
                                  mut cb: krb5_clpreauth_callbacks,
                                  mut rock: krb5_clpreauth_rock,
                                  mut request: *mut krb5_kdc_req,
                                  mut encoded_request_body: *mut krb5_data,
                                  mut encoded_previous_request:
                                      *mut krb5_data,
                                  mut padata: *mut krb5_pa_data,
                                  mut prompter: krb5_prompter_fct,
                                  mut prompter_data: *mut libc::c_void,
                                  mut out_padata: *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    let mut retval: krb5_error_code = 0;
    let mut sc2: *mut krb5_sam_challenge_2 = 0 as *mut krb5_sam_challenge_2;
    let mut sc2b: *mut krb5_sam_challenge_2_body =
        0 as *mut krb5_sam_challenge_2_body;
    let mut tmp_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut response_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut name: [libc::c_char; 100] = [0; 100];
    let mut banner: [libc::c_char; 100] = [0; 100];
    let mut prompt: [libc::c_char; 100] = [0; 100];
    let mut response: [libc::c_char; 100] = [0; 100];
    let mut kprompt: krb5_prompt =
        krb5_prompt{prompt: 0 as *mut libc::c_char,
                    hidden: 0,
                    reply: 0 as *mut krb5_data,};
    let mut prompt_type: krb5_prompt_type = 0;
    let mut defsalt: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut salt: *mut krb5_data = 0 as *mut krb5_data;
    let mut cksum: *mut *mut krb5_checksum = 0 as *mut *mut krb5_checksum;
    let mut scratch: *mut krb5_data = 0 as *mut krb5_data;
    let mut valid_cksum: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut enc_sam_response_enc_2: krb5_enc_sam_response_enc_2 =
        krb5_enc_sam_response_enc_2{magic: 0,
                                    sam_nonce: 0,
                                    sam_sad:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},};
    let mut sr2: krb5_sam_response_2 =
        krb5_sam_response_2{magic: 0,
                            sam_type: 0,
                            sam_flags: 0,
                            sam_track_id:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            sam_enc_nonce_or_sad:
                                krb5_enc_data{magic: 0,
                                              enctype: 0,
                                              kvno: 0,
                                              ciphertext:
                                                  krb5_data{magic: 0,
                                                            length: 0,
                                                            data:
                                                                0 as
                                                                    *mut libc::c_char,},},
                            sam_nonce: 0,};
    let mut ciph_len: size_t = 0;
    let mut sam_padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    if prompter.is_none() {
        return -(1765328254 as libc::c_long) as krb5_error_code
    }
    tmp_data.length = (*padata).length;
    tmp_data.data = (*padata).contents as *mut libc::c_char;
    retval = decode_krb5_sam_challenge_2(&mut tmp_data, &mut sc2);
    if retval != 0 { return retval }
    retval =
        decode_krb5_sam_challenge_2_body(&mut (*sc2).sam_challenge_2_body,
                                         &mut sc2b);
    if retval != 0 { krb5_free_sam_challenge_2(context, sc2); return retval }
    if (*sc2).sam_cksum.is_null() || (*(*sc2).sam_cksum).is_null() {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        return -(1765328157 as libc::c_long) as krb5_error_code
    }
    if (*sc2b).sam_flags & 0x20000000 as libc::c_int != 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        return -(1765328159 as libc::c_long) as krb5_error_code
    }
    if krb5_c_valid_enctype((*sc2b).sam_etype) == 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        return -(1765328158 as libc::c_long) as krb5_error_code
    }
    /* All of the above error checks are KDC-specific, that is, they     */
    /* assume a failure in the KDC reply.  By returning anything other   */
    /* than KRB5_KDC_UNREACH, KRB5_PREAUTH_FAILED,               */
    /* KRB5_LIBOS_PWDINTR, or KRB5_REALM_CANT_RESOLVE, the client will   */
    /* most likely go on to try the AS_REQ against master KDC            */
    if (*sc2b).sam_flags as libc::c_uint & 0x80000000 as libc::c_uint == 0 {
        /* We will need the password to obtain the key used for */
        /* the checksum, and encryption of the sam_response.    */
        /* Go ahead and get it now, preserving the ordering of  */
        /* prompts for the user.                                */
        salt =
            if (*ctx).default_salt != 0 {
                0 as *mut krb5_data
            } else { &mut (*ctx).salt };
        retval =
            (*ctx).gak_fct.expect("non-null function pointer")(context,
                                                               (*request).client,
                                                               (*sc2b).sam_etype,
                                                               prompter,
                                                               prompter_data,
                                                               salt,
                                                               &mut (*ctx).s2kparams,
                                                               &mut (*ctx).as_key,
                                                               (*ctx).gak_data,
                                                               (*ctx).rctx.items);
        if retval != 0 {
            krb5_free_sam_challenge_2(context, sc2);
            krb5_free_sam_challenge_2_body(context, sc2b);
            return retval
        }
    }
    snprintf(name.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
             b"%.*s\x00" as *const u8 as *const libc::c_char,
             if (*sc2b).sam_type_name.length != 0 {
                 if (*sc2b).sam_type_name.length as libc::c_ulong <=
                        (::std::mem::size_of::<[libc::c_char; 100]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) {
                     (*sc2b).sam_type_name.length as libc::c_ulong
                 } else {
                     strlen(dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"SAM Authentication\x00" as *const u8 as
                                         *const libc::c_char))
                 }
             } else {
                 strlen(dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"SAM Authentication\x00" as *const u8 as
                                     *const libc::c_char))
             } as libc::c_int,
             if (*sc2b).sam_type_name.length != 0 {
                 if (*sc2b).sam_type_name.length as libc::c_ulong <=
                        (::std::mem::size_of::<[libc::c_char; 100]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) {
                     (*sc2b).sam_type_name.data
                 } else {
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"SAM Authentication\x00" as *const u8 as
                                  *const libc::c_char)
                 }
             } else {
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"SAM Authentication\x00" as *const u8 as
                              *const libc::c_char)
             });
    snprintf(banner.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
             b"%.*s\x00" as *const u8 as *const libc::c_char,
             if (*sc2b).sam_challenge_label.length != 0 {
                 if (*sc2b).sam_challenge_label.length as libc::c_ulong <=
                        (::std::mem::size_of::<[libc::c_char; 100]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) {
                     (*sc2b).sam_challenge_label.length as libc::c_ulong
                 } else { strlen(sam_challenge_banner((*sc2b).sam_type)) }
             } else { strlen(sam_challenge_banner((*sc2b).sam_type)) } as
                 libc::c_int,
             if (*sc2b).sam_challenge_label.length != 0 {
                 if (*sc2b).sam_challenge_label.length as libc::c_ulong <=
                        (::std::mem::size_of::<[libc::c_char; 100]>() as
                             libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) {
                     (*sc2b).sam_challenge_label.data
                 } else { sam_challenge_banner((*sc2b).sam_type) }
             } else { sam_challenge_banner((*sc2b).sam_type) });
    snprintf(prompt.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
             b"%s%.*s%s%.*s\x00" as *const u8 as *const libc::c_char,
             if (*sc2b).sam_challenge.length != 0 {
                 b"Challenge is [\x00" as *const u8 as *const libc::c_char
             } else { b"\x00" as *const u8 as *const libc::c_char },
             if (*sc2b).sam_challenge.length != 0 {
                 if (*sc2b).sam_challenge.length <=
                        20 as libc::c_int as libc::c_uint {
                     (*sc2b).sam_challenge.length as libc::c_ulong
                 } else {
                     strlen(b"\x00" as *const u8 as *const libc::c_char)
                 }
             } else { strlen(b"\x00" as *const u8 as *const libc::c_char) } as
                 libc::c_int,
             if (*sc2b).sam_challenge.length != 0 {
                 if (*sc2b).sam_challenge.length <=
                        20 as libc::c_int as libc::c_uint {
                     (*sc2b).sam_challenge.data as *const libc::c_char
                 } else { b"\x00" as *const u8 as *const libc::c_char }
             } else { b"\x00" as *const u8 as *const libc::c_char },
             if (*sc2b).sam_challenge.length != 0 {
                 b"], \x00" as *const u8 as *const libc::c_char
             } else { b"\x00" as *const u8 as *const libc::c_char },
             if (*sc2b).sam_response_prompt.length != 0 {
                 if (*sc2b).sam_response_prompt.length <=
                        55 as libc::c_int as libc::c_uint {
                     (*sc2b).sam_response_prompt.length as libc::c_ulong
                 } else {
                     strlen(b"passcode\x00" as *const u8 as
                                *const libc::c_char)
                 }
             } else {
                 strlen(b"passcode\x00" as *const u8 as *const libc::c_char)
             } as libc::c_int,
             if (*sc2b).sam_response_prompt.length != 0 {
                 if (*sc2b).sam_response_prompt.length <=
                        55 as libc::c_int as libc::c_uint {
                     (*sc2b).sam_response_prompt.data as *const libc::c_char
                 } else {
                     b"passcode\x00" as *const u8 as *const libc::c_char
                 }
             } else { b"passcode\x00" as *const u8 as *const libc::c_char });
    response_data.data = response.as_mut_ptr();
    response_data.length =
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as
            libc::c_uint;
    kprompt.prompt = prompt.as_mut_ptr();
    kprompt.hidden = 1 as libc::c_int;
    kprompt.reply = &mut response_data;
    prompt_type = 0x4 as libc::c_int;
    k5_set_prompt_types(context, &mut prompt_type);
    retval =
        Some(prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                               prompter_data,
                                                                                               name.as_mut_ptr(),
                                                                                               banner.as_mut_ptr(),
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int,
                                                                                               &mut kprompt);
    if retval != 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        k5_set_prompt_types(context, 0 as *mut krb5_prompt_type);
        return retval
    }
    k5_set_prompt_types(context, 0 as *mut krb5_prompt_type);
    /* Generate salt used by string_to_key() */
    if (*ctx).default_salt != 0 {
        retval =
            krb5_principal2salt(context,
                                (*request).client as krb5_const_principal,
                                &mut defsalt);
        if retval != 0 {
            krb5_free_sam_challenge_2(context, sc2);
            krb5_free_sam_challenge_2_body(context, sc2b);
            return retval
        }
        salt = &mut defsalt
    } else {
        salt = &mut (*ctx).salt;
        defsalt.length = 0 as libc::c_int as libc::c_uint
    }
    /* Get encryption key to be used for checksum and sam_response */
    if (*sc2b).sam_flags as libc::c_uint & 0x80000000 as libc::c_uint == 0 {
        /* Retain as_key from above gak_fct call. */
        if defsalt.length != 0 { free(defsalt.data as *mut libc::c_void); }
        if (*sc2b).sam_flags & 0x40000000 as libc::c_int == 0 {
            /*
             * If no flags are set, the protocol calls for us to combine the
             * initial reply key with the SAD, using a method which is only
             * specified for DES and 3DES enctypes.  We no longer support this
             * case.
             */
            krb5_free_sam_challenge_2(context, sc2);
            krb5_free_sam_challenge_2_body(context, sc2b);
            return -(1765328159 as libc::c_long) as krb5_error_code
        }
    } else {
        /* as_key = string_to_key(SAD) */
        if (*ctx).as_key.length != 0 {
            krb5_free_keyblock_contents(context, &mut (*ctx).as_key);
            (*ctx).as_key.length = 0 as libc::c_int as libc::c_uint
        }
        /* generate a key using the supplied password */
        retval =
            krb5_c_string_to_key(context, (*sc2b).sam_etype,
                                 &mut response_data, salt,
                                 &mut (*ctx).as_key);
        if defsalt.length != 0 { free(defsalt.data as *mut libc::c_void); }
        if retval != 0 {
            krb5_free_sam_challenge_2(context, sc2);
            krb5_free_sam_challenge_2_body(context, sc2b);
            return retval
        }
    }
    /* Now we have a key, verify the checksum on the sam_challenge */
    cksum = (*sc2).sam_cksum;
    while !(*cksum).is_null() {
        if !(krb5_c_is_keyed_cksum((**cksum).checksum_type) == 0) {
            /* Check this cksum */
            retval =
                krb5_c_verify_checksum(context, &mut (*ctx).as_key,
                                       25 as libc::c_int,
                                       &mut (*sc2).sam_challenge_2_body,
                                       *cksum, &mut valid_cksum);
            if retval != 0 {
                krb5_free_data(context, scratch);
                krb5_free_sam_challenge_2(context, sc2);
                krb5_free_sam_challenge_2_body(context, sc2b);
                return retval
            }
            if valid_cksum != 0 { break ; }
        }
        cksum = cksum.offset(1)
    }
    if valid_cksum == 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        /*
         * Note: We return AP_ERR_BAD_INTEGRITY so upper-level applications
         * can interpret that as "password incorrect", which is probably
         * the best error we can return in this situation.
         */
        return -(1765328353 as libc::c_long) as krb5_error_code
    }
    /* fill in enc_sam_response_enc_2 */
    enc_sam_response_enc_2.magic =
        -(1760647374 as libc::c_long) as krb5_magic;
    enc_sam_response_enc_2.sam_nonce = (*sc2b).sam_nonce;
    if (*sc2b).sam_flags & 0x40000000 as libc::c_int != 0 {
        enc_sam_response_enc_2.sam_sad = response_data
    } else {
        enc_sam_response_enc_2.sam_sad.data = 0 as *mut libc::c_char;
        enc_sam_response_enc_2.sam_sad.length =
            0 as libc::c_int as libc::c_uint
    }
    /* encode and encrypt enc_sam_response_enc_2 with as_key */
    retval =
        encode_krb5_enc_sam_response_enc_2(&mut enc_sam_response_enc_2,
                                           &mut scratch);
    if retval != 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        return retval
    }
    /* Fill in sam_response_2 */
    memset(&mut sr2 as *mut krb5_sam_response_2 as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_sam_response_2>() as libc::c_ulong);
    sr2.sam_type = (*sc2b).sam_type;
    sr2.sam_flags = (*sc2b).sam_flags;
    sr2.sam_track_id = (*sc2b).sam_track_id;
    sr2.sam_nonce = (*sc2b).sam_nonce;
    /* Now take care of sr2.sam_enc_nonce_or_sad by encrypting encoded   */
    /* enc_sam_response_enc_2 from above */
    retval =
        krb5_c_encrypt_length(context, (*ctx).as_key.enctype,
                              (*scratch).length as size_t, &mut ciph_len);
    if retval != 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        krb5_free_data(context, scratch);
        return retval
    }
    sr2.sam_enc_nonce_or_sad.ciphertext.length = ciph_len as libc::c_uint;
    sr2.sam_enc_nonce_or_sad.ciphertext.data =
        malloc(sr2.sam_enc_nonce_or_sad.ciphertext.length as libc::c_ulong) as
            *mut libc::c_char;
    if sr2.sam_enc_nonce_or_sad.ciphertext.data.is_null() {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        krb5_free_data(context, scratch);
        return 12 as libc::c_int
    }
    retval =
        krb5_c_encrypt(context, &mut (*ctx).as_key, 27 as libc::c_int,
                       0 as *const krb5_data, scratch,
                       &mut sr2.sam_enc_nonce_or_sad);
    if retval != 0 {
        krb5_free_sam_challenge_2(context, sc2);
        krb5_free_sam_challenge_2_body(context, sc2b);
        krb5_free_data(context, scratch);
        krb5_free_data_contents(context,
                                &mut sr2.sam_enc_nonce_or_sad.ciphertext);
        return retval
    }
    krb5_free_data(context, scratch);
    scratch = 0 as *mut krb5_data;
    /* Encode the sam_response_2 */
    retval = encode_krb5_sam_response_2(&mut sr2, &mut scratch);
    krb5_free_sam_challenge_2(context, sc2);
    krb5_free_sam_challenge_2_body(context, sc2b);
    krb5_free_data_contents(context,
                            &mut sr2.sam_enc_nonce_or_sad.ciphertext);
    if retval != 0 { return retval }
    /* Almost there, just need to make padata !  */
    sam_padata =
        malloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_pa_data>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_pa_data;
    if sam_padata.is_null() {
        krb5_free_data(context, scratch);
        return 12 as libc::c_int
    }
    let ref mut fresh0 = *sam_padata.offset(0 as libc::c_int as isize);
    *fresh0 =
        malloc(::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong) as
            *mut krb5_pa_data;
    if (*sam_padata.offset(0 as libc::c_int as isize)).is_null() {
        krb5_free_data(context, scratch);
        free(sam_padata as *mut libc::c_void);
        return 12 as libc::c_int
    }
    (**sam_padata.offset(0 as libc::c_int as isize)).magic =
        -(1760647406 as libc::c_long) as krb5_magic;
    (**sam_padata.offset(0 as libc::c_int as isize)).pa_type =
        31 as libc::c_int;
    (**sam_padata.offset(0 as libc::c_int as isize)).length =
        (*scratch).length;
    let ref mut fresh1 =
        (**sam_padata.offset(0 as libc::c_int as isize)).contents;
    *fresh1 = (*scratch).data as *mut krb5_octet;
    free(scratch as *mut libc::c_void);
    let ref mut fresh2 = *sam_padata.offset(1 as libc::c_int as isize);
    *fresh2 = 0 as *mut krb5_pa_data;
    *out_padata = sam_padata;
    (*cb).disable_fallback.expect("non-null function pointer")(context, rock);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "381:25"]
static mut sam2_pa_types: [krb5_preauthtype; 2] =
    [30 as libc::c_int, 0 as libc::c_int];
#[no_mangle]
#[c2rust::src_loc = "384:1"]
pub unsafe extern "C" fn clpreauth_sam2_initvt(mut context: krb5_context,
                                               mut maj_ver: libc::c_int,
                                               mut min_ver: libc::c_int,
                                               mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_clpreauth_vtable = 0 as *mut krb5_clpreauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_clpreauth_vtable;
    (*vt).name =
        b"sam2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*vt).pa_type_list = sam2_pa_types.as_mut_ptr();
    (*vt).process =
        Some(sam2_process as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: krb5_clpreauth_modreq,
                                      _: *mut krb5_get_init_creds_opt,
                                      _: krb5_clpreauth_callbacks,
                                      _: krb5_clpreauth_rock,
                                      _: *mut krb5_kdc_req, _: *mut krb5_data,
                                      _: *mut krb5_data, _: *mut krb5_pa_data,
                                      _: krb5_prompter_fct,
                                      _: *mut libc::c_void,
                                      _: *mut *mut *mut krb5_pa_data)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
