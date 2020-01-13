use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
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
        #[no_mangle]
        #[c2rust::src_loc = "3797:1"]
        pub fn krb5_copy_data(context: krb5_context, indata: *const krb5_data,
                              outdata: *mut *mut krb5_data)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3829:1"]
        pub fn krb5_copy_addresses(context: krb5_context,
                                   inaddr: *const *mut krb5_address,
                                   outaddr: *mut *mut *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
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
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:32"]
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
    #[c2rust::src_loc = "2368:1"]
    pub unsafe extern "C" fn ts_within(mut a: krb5_timestamp,
                                       mut b: krb5_timestamp,
                                       mut d: krb5_deltat) -> krb5_boolean {
        return (ts_after(a, ts_incr(b, d)) == 0 &&
                    ts_after(b, ts_incr(a, d)) == 0) as libc::c_int as
                   krb5_boolean;
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
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
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
                        krb5_data, krb5_context, krb5_error_code,
                        krb5_pa_data, krb5_preauthtype, krb5_ticket,
                        krb5_error, krb5_kdc_rep, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stdint_uintn_h::uint32_t;
    use super::string_h::{strlen, memcmp};
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
        #[c2rust::src_loc = "616:1"]
        pub fn krb5_sendto_kdc(_: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_data,
                               _: *mut libc::c_int, _: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn encode_krb5_ticket(rep: *const krb5_ticket,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2151:1"]
        pub fn krb5_free_kdc_rep(_: krb5_context, _: *mut krb5_kdc_rep);
    }
    /* _KRB5_INT_H */
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:33"]
pub mod int_proto_h {
    #[c2rust::src_loc = "89:1"]
    pub type k5_pacb_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_keyblock,
                                    _: *mut krb5_kdc_req,
                                    _: *mut libc::c_void) -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_keyblock,
                        krb5_kdc_req, krb5_creds, krb5_flags, krb5_address,
                        krb5_pa_data, krb5_data, krb5_timestamp, krb5_int32,
                        krb5_keyusage, krb5_kdc_rep};
    use super::k5_int_h::_krb5_context;
    use super::fast_h::krb5int_fast_request_state;
    extern "C" {
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
        /* The subkey field is an output parameter; if a
 * tgs-rep is received then the subkey will be filled
 * in with the subkey needed to decrypt the TGS
 * response. Otherwise it will be set to null.
 */
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn krb5int_decode_tgs_rep(_: krb5_context,
                                      _: *mut krb5int_fast_request_state,
                                      _: *mut krb5_data,
                                      _: *const krb5_keyblock,
                                      _: krb5_keyusage,
                                      _: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/fast.h:34"]
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
                        krb5_context, krb5_error, krb5_pa_data, krb5_boolean,
                        krb5_error_code};
    use super::k5_int_h::{krb5_fast_armor, _krb5_context};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn krb5int_fast_process_error(context: krb5_context,
                                          state:
                                              *mut krb5int_fast_request_state,
                                          err_replyptr: *mut *mut krb5_error,
                                          out_padata:
                                              *mut *mut *mut krb5_pa_data,
                                          retry: *mut krb5_boolean)
         -> krb5_error_code;
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
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
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
#[c2rust::header_src = "/usr/include/libintl.h:32"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
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
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:32"]
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
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_creds, krb5_creds, _krb5_last_req_entry,
                       krb5_last_req_entry, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _profile_t, krb5_unparse_name,
                       krb5_principal_compare, krb5_copy_keyblock_contents,
                       krb5_copy_data, krb5_copy_principal,
                       krb5_copy_addresses, krb5_free_error,
                       krb5_free_keyblock, krb5_free_keyblock_contents,
                       krb5_free_data_contents, krb5_free_unparsed_name,
                       krb5_set_error_message, krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_fast_armor, _krb5_fast_armor,
                         ts_within, ts_incr, ts_after, data_eq_string,
                         data_eq, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_sendto_kdc, krb5int_find_pa_data,
                         encode_krb5_ticket, decode_krb5_error,
                         krb5_free_kdc_rep};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::int_proto_h::{k5_pacb_fn, k5_make_tgs_req,
                            krb5int_decode_tgs_rep};
pub use self::fast_h::{krb5int_fast_request_state, krb5int_fast_process_error,
                       krb5int_fast_make_state, krb5int_fast_free_state};
use self::stdlib_h::{calloc, free};
use self::libintl_h::dgettext;
use self::string_h::{strlen, memcmp, memset};
use self::k5_trace_h::krb5int_trace;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/gc_via_tkt.c */
/*
 * Copyright 1990,1991,2007-2009 by the Massachusetts Institute of Technology.
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
 * Given a tkt, and a target cred, get it.
 * Assumes that the kdc_rep has been decrypted.
 */
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn kdcrep2creds(mut context: krb5_context,
                                  mut pkdcrep: *mut krb5_kdc_rep,
                                  mut address: *const *mut krb5_address,
                                  mut is_skey: krb5_boolean,
                                  mut psectkt: *mut krb5_data,
                                  mut ppcreds: *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64; /* not used */
    let mut retval: krb5_error_code = 0;
    let mut pdata: *mut krb5_data = 0 as *mut krb5_data;
    *ppcreds =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_creds>() as libc::c_ulong) as
            *mut krb5_creds;
    if (*ppcreds).is_null() { return 12 as libc::c_int }
    retval =
        krb5_copy_principal(context,
                            (*pkdcrep).client as krb5_const_principal,
                            &mut (**ppcreds).client);
    if !(retval != 0) {
        retval =
            krb5_copy_principal(context,
                                (*(*pkdcrep).enc_part2).server as
                                    krb5_const_principal,
                                &mut (**ppcreds).server);
        if !(retval != 0) {
            retval =
                krb5_copy_keyblock_contents(context,
                                            (*(*pkdcrep).enc_part2).session,
                                            &mut (**ppcreds).keyblock);
            if !(retval != 0) {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"TGS reply is for {princ} -> {princ} with session key {keyblock}\x00"
                                      as *const u8 as *const libc::c_char,
                                  (**ppcreds).client, (**ppcreds).server,
                                  &mut (**ppcreds).keyblock as
                                      *mut krb5_keyblock);
                }
                retval = krb5_copy_data(context, psectkt, &mut pdata);
                if !(retval != 0) {
                    (**ppcreds).second_ticket = *pdata;
                    free(pdata as *mut libc::c_void);
                    (**ppcreds).ticket_flags = (*(*pkdcrep).enc_part2).flags;
                    (**ppcreds).times = (*(*pkdcrep).enc_part2).times;
                    (**ppcreds).magic =
                        -(1760647408 as libc::c_long) as krb5_magic;
                    (**ppcreds).authdata = 0 as *mut *mut krb5_authdata;
                    (**ppcreds).is_skey = is_skey;
                    if !(*(*pkdcrep).enc_part2).caddrs.is_null() {
                        retval =
                            krb5_copy_addresses(context,
                                                (*(*pkdcrep).enc_part2).caddrs,
                                                &mut (**ppcreds).addresses);
                        if retval != 0 {
                            current_block = 1349400641705233371;
                        } else { current_block = 14576567515993809846; }
                    } else {
                        /* no addresses in the list means we got what we had */
                        retval =
                            krb5_copy_addresses(context, address,
                                                &mut (**ppcreds).addresses);
                        if retval != 0 {
                            current_block = 1349400641705233371;
                        } else { current_block = 14576567515993809846; }
                    }
                    match current_block {
                        1349400641705233371 => { }
                        _ => {
                            retval =
                                encode_krb5_ticket((*pkdcrep).ticket,
                                                   &mut pdata);
                            if !(retval != 0) {
                                (**ppcreds).ticket = *pdata;
                                free(pdata as *mut libc::c_void);
                                return 0 as libc::c_int
                            }
                        }
                    }
                }
                krb5_free_keyblock_contents(context,
                                            &mut (**ppcreds).keyblock);
            }
        }
    }
    free(*ppcreds as *mut libc::c_void);
    *ppcreds = 0 as *mut krb5_creds;
    return retval;
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn check_reply_server(mut context: krb5_context,
                                        mut kdcoptions: krb5_flags,
                                        mut in_cred: *mut krb5_creds,
                                        mut dec_rep: *mut krb5_kdc_rep)
 -> krb5_error_code {
    if krb5_principal_compare(context,
                              (*(*dec_rep).ticket).server as
                                  krb5_const_principal,
                              (*(*dec_rep).enc_part2).server as
                                  krb5_const_principal) == 0 {
        return -(1765328237 as libc::c_long) as krb5_error_code
    }
    /* Reply is self-consistent. */
    if krb5_principal_compare(context,
                              (*(*dec_rep).ticket).server as
                                  krb5_const_principal,
                              (*in_cred).server as krb5_const_principal) != 0
       {
        return 0 as libc::c_int
    }
    /* Server in reply differs from what we requested. */
    if kdcoptions & 0x10000 as libc::c_int != 0 {
        /* in_cred server differs from ticket returned, but ticket
           returned is consistent and we requested canonicalization. */
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Reply server {princ} differs from requested {princ}\x00"
                              as *const u8 as *const libc::c_char,
                          (*(*dec_rep).enc_part2).server, (*in_cred).server);
        }
        return 0 as libc::c_int
    }
    /* We didn't request canonicalization. */
    if !((*(*in_cred).server).length == 2 as libc::c_int &&
             data_eq_string(*(*(*in_cred).server).data.offset(0 as libc::c_int
                                                                  as isize),
                            b"krbtgt\x00" as *const u8 as *const libc::c_char)
                 != 0) ||
           !((*(*(*dec_rep).ticket).server).length == 2 as libc::c_int &&
                 data_eq_string(*(*(*(*dec_rep).ticket).server).data.offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                b"krbtgt\x00" as *const u8 as
                                    *const libc::c_char) != 0) {
        /* Canonicalization not requested, and not a TGS referral. */
        return -(1765328237 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
/* Return true if a TGS credential is for the client's local realm. */
#[inline]
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn tgt_is_local_realm(mut tgt: *mut krb5_creds)
 -> libc::c_int {
    return ((*(*tgt).server).length == 2 as libc::c_int &&
                data_eq_string(*(*(*tgt).server).data.offset(0 as libc::c_int
                                                                 as isize),
                               b"krbtgt\x00" as *const u8 as
                                   *const libc::c_char) != 0 &&
                data_eq(*(*(*tgt).server).data.offset(1 as libc::c_int as
                                                          isize),
                        (*(*tgt).client).realm) != 0 &&
                data_eq((*(*tgt).server).realm, (*(*tgt).client).realm) != 0)
               as libc::c_int;
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
#[no_mangle]
#[c2rust::src_loc = "148:1"]
pub unsafe extern "C" fn krb5_get_cred_via_tkt(mut context: krb5_context,
                                               mut tkt: *mut krb5_creds,
                                               mut kdcoptions: krb5_flags,
                                               mut address:
                                                   *const *mut krb5_address,
                                               mut in_cred: *mut krb5_creds,
                                               mut out_cred:
                                                   *mut *mut krb5_creds)
 -> krb5_error_code {
    return krb5_get_cred_via_tkt_ext(context, tkt, kdcoptions, address,
                                     0 as *mut *mut krb5_pa_data, in_cred,
                                     None, 0 as *mut libc::c_void,
                                     0 as *mut *mut *mut krb5_pa_data,
                                     0 as *mut *mut *mut krb5_pa_data,
                                     out_cred, 0 as *mut *mut krb5_keyblock);
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn krb5int_process_tgs_reply(mut context: krb5_context,
                                                   mut fast_state:
                                                       *mut krb5int_fast_request_state,
                                                   mut response_data:
                                                       *mut krb5_data,
                                                   mut tkt: *mut krb5_creds,
                                                   mut kdcoptions: krb5_flags,
                                                   mut address:
                                                       *const *mut krb5_address,
                                                   mut in_padata:
                                                       *mut *mut krb5_pa_data,
                                                   mut in_cred:
                                                       *mut krb5_creds,
                                                   mut timestamp:
                                                       krb5_timestamp,
                                                   mut nonce: krb5_int32,
                                                   mut subkey:
                                                       *mut krb5_keyblock,
                                                   mut out_padata:
                                                       *mut *mut *mut krb5_pa_data,
                                                   mut out_enc_padata:
                                                       *mut *mut *mut krb5_pa_data,
                                                   mut out_cred:
                                                       *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut dec_rep: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    let mut err_reply: *mut krb5_error = 0 as *mut krb5_error;
    let mut s4u2self: krb5_boolean = 0;
    let mut is_skey: krb5_boolean = 0;
    s4u2self =
        (!krb5int_find_pa_data(context, in_padata,
                               130 as libc::c_int).is_null() ||
             !krb5int_find_pa_data(context, in_padata,
                                   129 as libc::c_int).is_null()) as
            libc::c_int as krb5_boolean;
    if !response_data.is_null() && (*response_data).length != 0 &&
           *(*response_data).data.offset(0 as libc::c_int as isize) as
               libc::c_int & !(0x20 as libc::c_int) ==
               30 as libc::c_int | 0x40 as libc::c_int {
        retval = decode_krb5_error(response_data, &mut err_reply);
        if !(retval != 0 as libc::c_int) {
            retval =
                krb5int_fast_process_error(context, fast_state,
                                           &mut err_reply,
                                           0 as *mut *mut *mut krb5_pa_data,
                                           0 as *mut krb5_boolean);
            if !(retval != 0) {
                retval =
                    ((*err_reply).error as krb5_error_code as libc::c_long +
                         -(1765328384 as libc::c_long)) as krb5_error_code;
                if (*err_reply).text.length > 0 as libc::c_int as libc::c_uint
                   {
                    match (*err_reply).error {
                        60 => {
                            krb5_set_error_message(context, retval,
                                                   dgettext(b"mit-krb5\x00" as
                                                                *const u8 as
                                                                *const libc::c_char,
                                                            b"KDC returned error string: %.*s\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char),
                                                   (*err_reply).text.length,
                                                   (*err_reply).text.data);
                        }
                        7 => {
                            let mut s_name: *mut libc::c_char =
                                0 as *mut libc::c_char;
                            if !(*err_reply).server.is_null() &&
                                   krb5_unparse_name(context,
                                                     (*err_reply).server as
                                                         krb5_const_principal,
                                                     &mut s_name) ==
                                       0 as libc::c_int {
                                krb5_set_error_message(context, retval,
                                                       dgettext(b"mit-krb5\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                b"Server %s not found in Kerberos database\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char),
                                                       s_name);
                                krb5_free_unparsed_name(context, s_name);
                            } else {
                                /* In case there's a stale S_PRINCIPAL_UNKNOWN
                       report already noted.  */
                                krb5_clear_error_message(context);
                            }
                        }
                        _ => { }
                    }
                }
                krb5_free_error(context, err_reply);
            }
        }
    } else if !(!response_data.is_null() && (*response_data).length != 0 &&
                    *(*response_data).data.offset(0 as libc::c_int as isize)
                        as libc::c_int & !(0x20 as libc::c_int) ==
                        13 as libc::c_int | 0x40 as libc::c_int) {
        retval = -(1765328344 as libc::c_long) as krb5_error_code
    } else {
        /* Unfortunately, Heimdal at least up through 1.2  encrypts using
       the session key not the subsession key.  So we try both. */
        retval =
            krb5int_decode_tgs_rep(context, fast_state, response_data, subkey,
                                   9 as libc::c_int, &mut dec_rep);
        if retval != 0 {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"TGS reply didn\'t decode with subkey; trying session key ({keyblock)}\x00"
                                  as *const u8 as *const libc::c_char,
                              &mut (*tkt).keyblock as *mut krb5_keyblock);
            }
            if krb5int_decode_tgs_rep(context, fast_state, response_data,
                                      &mut (*tkt).keyblock, 8 as libc::c_int,
                                      &mut dec_rep) == 0 as libc::c_int {
                retval = 0 as libc::c_int;
                current_block = 8704759739624374314;
            } else { current_block = 6977356278672029926; }
        } else { current_block = 8704759739624374314; }
        match current_block {
            6977356278672029926 => { }
            _ => {
                if (*dec_rep).msg_type != 13 as libc::c_int as krb5_msgtype {
                    retval = -(1765328344 as libc::c_long) as krb5_error_code
                } else {
                    /*
     * Don't trust the ok-as-delegate flag from foreign KDCs unless the
     * cross-realm TGT also had the ok-as-delegate flag set.
     */
                    if tgt_is_local_realm(tkt) == 0 &&
                           (*tkt).ticket_flags & 0x40000 as libc::c_int == 0 {
                        (*(*dec_rep).enc_part2).flags &=
                            !(0x40000 as libc::c_int)
                    }
                    /* make sure the response hasn't been tampered with..... */
                    retval = 0 as libc::c_int;
                    if s4u2self != 0 &&
                           !((*(*(*dec_rep).ticket).server).length ==
                                 2 as libc::c_int &&
                                 data_eq_string(*(*(*(*dec_rep).ticket).server).data.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize),
                                                b"krbtgt\x00" as *const u8 as
                                                    *const libc::c_char) != 0)
                       {
                        /* Final hop, check whether KDC supports S4U2Self */
                        if krb5_principal_compare(context,
                                                  (*dec_rep).client as
                                                      krb5_const_principal,
                                                  (*in_cred).server as
                                                      krb5_const_principal) !=
                               0 {
                            retval =
                                -(1765328368 as libc::c_long) as
                                    krb5_error_code
                        }
                    } else if kdcoptions & 0x20000 as libc::c_int ==
                                  0 as libc::c_int ||
                                  (*(*(*dec_rep).ticket).server).length ==
                                      2 as libc::c_int &&
                                      data_eq_string(*(*(*(*dec_rep).ticket).server).data.offset(0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize),
                                                     b"krbtgt\x00" as
                                                         *const u8 as
                                                         *const libc::c_char)
                                          != 0 {
                        /*
         * For constrained delegation this check must be performed by caller,
         * as we can't decrypt the evidence ticket.  However, if it is a
         * referral the client should match the TGT client like normal.
         */
                        if krb5_principal_compare(context,
                                                  (*dec_rep).client as
                                                      krb5_const_principal,
                                                  (*tkt).client as
                                                      krb5_const_principal) ==
                               0 {
                            retval =
                                -(1765328237 as libc::c_long) as
                                    krb5_error_code
                        }
                    }
                    if retval == 0 as libc::c_int {
                        retval =
                            check_reply_server(context, kdcoptions, in_cred,
                                               dec_rep)
                    }
                    if (*(*dec_rep).enc_part2).nonce != nonce {
                        retval =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    }
                    if kdcoptions & 0x2000000 as libc::c_int != 0 &&
                           (*in_cred).times.starttime != 0 as libc::c_int &&
                           (*in_cred).times.starttime !=
                               (*(*dec_rep).enc_part2).times.starttime {
                        retval =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    }
                    if (*in_cred).times.endtime != 0 as libc::c_int &&
                           ts_after((*(*dec_rep).enc_part2).times.endtime,
                                    (*in_cred).times.endtime) != 0 {
                        retval =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    }
                    if kdcoptions & 0x800000 as libc::c_int != 0 &&
                           (*in_cred).times.renew_till != 0 as libc::c_int &&
                           ts_after((*(*dec_rep).enc_part2).times.renew_till,
                                    (*in_cred).times.renew_till) != 0 {
                        retval =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    }
                    if kdcoptions & 0x10 as libc::c_int != 0 &&
                           (*(*dec_rep).enc_part2).flags &
                               0x800000 as libc::c_int != 0 &&
                           (*in_cred).times.endtime != 0 as libc::c_int &&
                           ts_after((*(*dec_rep).enc_part2).times.renew_till,
                                    (*in_cred).times.endtime) != 0 {
                        retval =
                            -(1765328237 as libc::c_long) as krb5_error_code
                    }
                    if !(retval != 0 as libc::c_int) {
                        if (*in_cred).times.starttime == 0 &&
                               ts_within((*(*dec_rep).enc_part2).times.starttime,
                                         timestamp, (*context).clockskew) == 0
                           {
                            retval =
                                -(1765328236 as libc::c_long) as
                                    krb5_error_code
                        } else {
                            if !out_padata.is_null() {
                                *out_padata = (*dec_rep).padata;
                                (*dec_rep).padata =
                                    0 as *mut *mut krb5_pa_data
                            }
                            if !out_enc_padata.is_null() {
                                *out_enc_padata =
                                    (*(*dec_rep).enc_part2).enc_padata;
                                (*(*dec_rep).enc_part2).enc_padata =
                                    0 as *mut *mut krb5_pa_data
                            }
                            is_skey =
                                (kdcoptions & 0x8 as libc::c_int) as
                                    krb5_boolean;
                            retval =
                                kdcrep2creds(context, dec_rep, address,
                                             is_skey,
                                             &mut (*in_cred).second_ticket,
                                             out_cred);
                            (retval) != 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    if !dec_rep.is_null() {
        memset((*(*(*dec_rep).enc_part2).session).contents as
                   *mut libc::c_void, 0 as libc::c_int,
               (*(*(*dec_rep).enc_part2).session).length as libc::c_ulong);
        krb5_free_kdc_rep(context, dec_rep);
    }
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "333:1"]
pub unsafe extern "C" fn krb5_get_cred_via_tkt_ext(mut context: krb5_context,
                                                   mut tkt: *mut krb5_creds,
                                                   mut kdcoptions: krb5_flags,
                                                   mut address:
                                                       *const *mut krb5_address,
                                                   mut in_padata:
                                                       *mut *mut krb5_pa_data,
                                                   mut in_cred:
                                                       *mut krb5_creds,
                                                   mut pacb_fn: k5_pacb_fn,
                                                   mut pacb_data:
                                                       *mut libc::c_void,
                                                   mut out_padata:
                                                       *mut *mut *mut krb5_pa_data,
                                                   mut out_enc_padata:
                                                       *mut *mut *mut krb5_pa_data,
                                                   mut out_cred:
                                                       *mut *mut krb5_creds,
                                                   mut out_subkey:
                                                       *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut request_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut response_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut timestamp: krb5_timestamp = 0;
    let mut nonce: krb5_int32 = 0;
    let mut subkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut tcp_only: libc::c_int = 0 as libc::c_int;
    let mut use_master: libc::c_int = 0 as libc::c_int;
    let mut fast_state: *mut krb5int_fast_request_state =
        0 as *mut krb5int_fast_request_state;
    request_data.data = 0 as *mut libc::c_char;
    request_data.length = 0 as libc::c_int as libc::c_uint;
    response_data.data = 0 as *mut libc::c_char;
    response_data.length = 0 as libc::c_int as libc::c_uint;
    retval = krb5int_fast_make_state(context, &mut fast_state);
    if !(retval != 0) {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Get cred via TGT {princ} after requesting {princ} (canonicalize {str})\x00"
                              as *const u8 as *const libc::c_char,
                          (*tkt).server, (*in_cred).server,
                          if kdcoptions & 0x10000 as libc::c_int != 0 {
                              b"on\x00" as *const u8 as *const libc::c_char
                          } else {
                              b"off\x00" as *const u8 as *const libc::c_char
                          });
        }
        retval =
            k5_make_tgs_req(context, fast_state, tkt, kdcoptions, address,
                            in_padata, in_cred, pacb_fn, pacb_data,
                            &mut request_data, &mut timestamp, &mut nonce,
                            &mut subkey);
        if !(retval != 0 as libc::c_int) {
            loop  {
                use_master = 0 as libc::c_int;
                retval =
                    krb5_sendto_kdc(context, &mut request_data,
                                    &mut (*(*in_cred).server).realm,
                                    &mut response_data, &mut use_master,
                                    tcp_only);
                if !(retval == 0 as libc::c_int) {
                    current_block = 15637249698150952053;
                    break ;
                }
                if !(!(&mut response_data as *mut krb5_data).is_null() &&
                         response_data.length != 0 &&
                         *response_data.data.offset(0 as libc::c_int as isize)
                             as libc::c_int & !(0x20 as libc::c_int) ==
                             30 as libc::c_int | 0x40 as libc::c_int) {
                    current_block = 18377268871191777778;
                    break ;
                }
                if !(tcp_only == 0) {
                    current_block = 18377268871191777778;
                    break ;
                }
                let mut err_reply: *mut krb5_error = 0 as *mut krb5_error;
                retval =
                    decode_krb5_error(&mut response_data, &mut err_reply);
                if retval != 0 as libc::c_int {
                    current_block = 15637249698150952053;
                    break ;
                }
                retval =
                    krb5int_fast_process_error(context, fast_state,
                                               &mut err_reply,
                                               0 as
                                                   *mut *mut *mut krb5_pa_data,
                                               0 as *mut krb5_boolean);
                if retval != 0 {
                    current_block = 15637249698150952053;
                    break ;
                }
                if (*err_reply).error == 52 as libc::c_int as libc::c_uint {
                    tcp_only = 1 as libc::c_int;
                    krb5_free_error(context, err_reply);
                    krb5_free_data_contents(context, &mut response_data);
                } else {
                    krb5_free_error(context, err_reply);
                    current_block = 18377268871191777778;
                    break ;
                }
            }
            match current_block {
                15637249698150952053 => { }
                _ => {
                    retval =
                        krb5int_process_tgs_reply(context, fast_state,
                                                  &mut response_data, tkt,
                                                  kdcoptions, address,
                                                  in_padata, in_cred,
                                                  timestamp, nonce, subkey,
                                                  out_padata, out_enc_padata,
                                                  out_cred);
                    (retval) != 0 as libc::c_int;
                }
            }
        }
    }
    krb5int_fast_free_state(context, fast_state);
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Got cred; {kerr}\x00" as *const u8 as
                          *const libc::c_char, retval);
    }
    krb5_free_data_contents(context, &mut request_data);
    krb5_free_data_contents(context, &mut response_data);
    if !subkey.is_null() {
        if retval == 0 as libc::c_int && !out_subkey.is_null() {
            *out_subkey = subkey
        } else { krb5_free_keyblock(context, subkey); }
    }
    return retval;
}
