use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:31"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:31"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:31"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:31"]
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
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
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
    /* The name of the Kerberos ticket granting service... and its size */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Pointer to a prompter callback function. */
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
    /* *
 * This flag indicates that the PIN MUST be returned as a separate item. This
 * flag only takes effect if KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN is set. If
 * this flag is not set, the responder may either concatenate PIN + token value
 * and store it as "value" in the answer or it may return them separately. If
 * they are returned separately, they will be concatenated internally.
 */
    /* *
 * PKINIT responder question
 *
 * The PKINIT responder question is asked when the client needs a password
 * that's being used to protect key information, and is formatted as a JSON
 * object.  A specific identity's flags value, if not zero, is the bitwise-OR
 * of one or more of the KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_* flags defined
 * below, and possibly other flags to be added later.  Any resemblance to
 * similarly-named CKF_* values in the PKCS#11 API should not be depended on.
 *
 *  @n {
 *  @n     identity <string> : flags <number>,
 *  @n     ...
 *  @n }
 *
 * The answer to the question MUST be JSON formatted:
 *
 *  @n {
 *  @n     identity <string> : password <string>,
 *  @n     ...
 *  @n }
 *
 * @version New in 1.12
 */
    /* *
 * This flag indicates that an incorrect PIN was supplied at least once since
 * the last time the correct PIN was supplied.
 */
    /* *
 * This flag indicates that supplying an incorrect PIN will cause the token to
 * lock itself.
 */
    /* *
 * This flag indicates that the user PIN is locked, and you can't log in to the
 * token with it.
 */
    /* *
 * A container for a set of preauthentication questions and answers
 *
 * A responder context is supplied by the krb5 authentication system to a @ref
 * krb5_responder_fn callback.  It contains a list of questions and can receive
 * answers.  Questions contained in a responder context can be listed using
 * krb5_responder_list_questions(), retrieved using
 * krb5_responder_get_challenge(), or answered using
 * krb5_responder_set_answer().  The form of a question's challenge and
 * answer depend on the question name.
 *
 * @version New in 1.11
 */
    #[c2rust::src_loc = "6603:1"]
    pub type krb5_responder_context = *mut krb5_responder_context_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6685:16"]
    pub struct _krb5_responder_otp_tokeninfo {
        pub flags: krb5_flags,
        pub format: krb5_int32,
        pub length: krb5_int32,
        pub vendor: *mut libc::c_char,
        pub challenge: *mut libc::c_char,
        pub token_id: *mut libc::c_char,
        pub alg_id: *mut libc::c_char,
    }
    #[c2rust::src_loc = "6685:1"]
    pub type krb5_responder_otp_tokeninfo = _krb5_responder_otp_tokeninfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6695:16"]
    pub struct _krb5_responder_otp_challenge {
        pub service: *mut libc::c_char,
        pub tokeninfo: *mut *mut krb5_responder_otp_tokeninfo,
    }
    #[c2rust::src_loc = "6695:1"]
    pub type krb5_responder_otp_challenge = _krb5_responder_otp_challenge;
    /* -1 when not specified. */
    /* -1 when not specified. */
    /* * Store options for @c _krb5_get_init_creds */
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
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
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
        #[c2rust::src_loc = "6603:16"]
        pub type krb5_responder_context_st;
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
        /* *
 * Retrieve the challenge data for a given question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 *
 * Return a pointer to a C string containing the challenge for @a question
 * within @a rctx, or NULL if the question is not present in @a rctx.  The
 * structure of the question depends on the question name, but will always be
 * printable UTF-8 text.  The returned pointer is an alias, valid only as long
 * as the lifetime of @a rctx, and should not be modified or freed by the
 * caller.
 *
 * @version New in 1.11
 */
        #[no_mangle]
        #[c2rust::src_loc = "6638:1"]
        pub fn krb5_responder_get_challenge(ctx: krb5_context,
                                            rctx: krb5_responder_context,
                                            question: *const libc::c_char)
         -> *const libc::c_char;
        /* *
 * Answer a named question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 * @param [in] answer           The string to set (MUST be printable UTF-8)
 *
 * This function supplies an answer to @a question within @a rctx.  The
 * appropriate form of the answer depends on the question name.
 *
 * @retval EINVAL @a question is not present within @a rctx
 *
 * @version New in 1.11
 */
        #[no_mangle]
        #[c2rust::src_loc = "6657:1"]
        pub fn krb5_responder_set_answer(ctx: krb5_context,
                                         rctx: krb5_responder_context,
                                         question: *const libc::c_char,
                                         answer: *const libc::c_char)
         -> krb5_error_code;
        /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:31"]
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
    #[c2rust::src_loc = "515:1"]
    pub type krb5_otp_tokeninfo = _krb5_otp_tokeninfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "515:16"]
    pub struct _krb5_otp_tokeninfo {
        pub flags: krb5_flags,
        pub vendor: krb5_data,
        pub challenge: krb5_data,
        pub length: krb5_int32,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub supported_hash_alg: *mut *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
    }
    #[c2rust::src_loc = "527:1"]
    pub type krb5_pa_otp_challenge = _krb5_pa_otp_challenge;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "527:16"]
    pub struct _krb5_pa_otp_challenge {
        pub nonce: krb5_data,
        pub service: krb5_data,
        pub tokeninfo: *mut *mut krb5_otp_tokeninfo,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
    }
    #[c2rust::src_loc = "535:1"]
    pub type krb5_pa_otp_req = _krb5_pa_otp_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "535:16"]
    pub struct _krb5_pa_otp_req {
        pub flags: krb5_int32,
        pub nonce: krb5_data,
        pub enc_data: krb5_enc_data,
        pub hash_alg: *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
        pub otp_value: krb5_data,
        pub pin: krb5_data,
        pub challenge: krb5_data,
        pub time: krb5_timestamp,
        pub counter: krb5_data,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub vendor: krb5_data,
    }
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_enc_data, krb5_timestamp,
                        krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::k5_int_pkinit_h::krb5_algorithm_identifier;
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
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "958:1"]
        pub fn k5_free_otp_tokeninfo(context: krb5_context,
                                     val: *mut krb5_otp_tokeninfo);
        #[no_mangle]
        #[c2rust::src_loc = "959:1"]
        pub fn k5_free_pa_otp_challenge(context: krb5_context,
                                        val: *mut krb5_pa_otp_challenge);
        #[no_mangle]
        #[c2rust::src_loc = "961:1"]
        pub fn k5_free_pa_otp_req(context: krb5_context,
                                  val: *mut krb5_pa_otp_req);
        #[no_mangle]
        #[c2rust::src_loc = "1529:1"]
        pub fn encode_krb5_pa_otp_req(_: *const krb5_pa_otp_req,
                                      _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1532:1"]
        pub fn encode_krb5_pa_otp_enc_req(_: *const krb5_data,
                                          _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1711:1"]
        pub fn decode_krb5_pa_otp_challenge(_: *const krb5_data,
                                            _:
                                                *mut *mut krb5_pa_otp_challenge)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:32"]
pub mod k5_json_h {
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    /*
 * Array
 */
    #[c2rust::src_loc = "128:1"]
    pub type k5_json_array = *mut k5_json_array_st;
    /*
 * String
 */
    #[c2rust::src_loc = "186:1"]
    pub type k5_json_string = *mut k5_json_string_st;
    /*
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
    #[c2rust::src_loc = "86:1"]
    pub type k5_json_value = *mut libc::c_void;
    #[c2rust::src_loc = "87:1"]
    pub type k5_json_tid = libc::c_uint;
    /*
 * Number
 */
    #[c2rust::src_loc = "206:1"]
    pub type k5_json_number = *mut k5_json_number_st;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
        #[c2rust::src_loc = "128:16"]
        pub type k5_json_array_st;
        #[c2rust::src_loc = "186:16"]
        pub type k5_json_string_st;
        #[c2rust::src_loc = "206:16"]
        pub type k5_json_number_st;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn k5_json_get_tid(val: k5_json_value) -> k5_json_tid;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_json_release(val: k5_json_value);
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn k5_json_array_create(val_out: *mut k5_json_array)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn k5_json_array_length(array: k5_json_array) -> size_t;
        /* Both of these functions increment the reference count on val. */
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn k5_json_array_add(array: k5_json_array, val: k5_json_value)
         -> libc::c_int;
        /* Get an alias to the idx-th element of array, without incrementing the
 * reference count.  The caller must check idx against the array length. */
        #[no_mangle]
        #[c2rust::src_loc = "139:1"]
        pub fn k5_json_array_get(array: k5_json_array, idx: size_t)
         -> k5_json_value;
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_json_object_create(val_out: *mut k5_json_object)
         -> libc::c_int;
        /*
 * Store val into object at key, incrementing val's reference count and
 * releasing any previous value at key.  If val is NULL, key is removed from
 * obj if it exists, and obj remains unchanged if it does not.
 */
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn k5_json_object_set(obj: k5_json_object,
                                  key: *const libc::c_char,
                                  val: k5_json_value) -> libc::c_int;
        /* Get an alias to the object's value for key, without incrementing the
 * reference count.  Returns NULL if there is no value for key. */
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn k5_json_object_get(obj: k5_json_object,
                                  key: *const libc::c_char) -> k5_json_value;
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn k5_json_string_create(cstring: *const libc::c_char,
                                     val_out: *mut k5_json_string)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn k5_json_string_create_len(data: *const libc::c_void,
                                         len: size_t,
                                         val_out: *mut k5_json_string)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn k5_json_string_utf8(string: k5_json_string)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn k5_json_number_create(number: libc::c_longlong,
                                     val_out: *mut k5_json_number)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "209:1"]
        pub fn k5_json_number_value(number: k5_json_number)
         -> libc::c_longlong;
        /*
 * JSON encoding and decoding
 */
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn k5_json_encode(val: k5_json_value,
                              json_out: *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn k5_json_decode(str: *const libc::c_char,
                              val_out: *mut k5_json_value) -> libc::c_int;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:31"]
pub mod k5_int_pkinit_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * pkinit structures
 */
    /* PKAuthenticator */
    /* (0..999999) */
    /* (0..4294967295) */
    /* AlgorithmIdentifier */
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    use super::krb5_h::{krb5_data, krb5_context, krb5_keyblock, krb5_keyusage,
                        krb5_enc_data, krb5_error_code};
    use super::k5_int_h::_krb5_context;
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
    /* OID */
    /* Optional */
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:36"]
pub mod clpreauth_plugin_h {
    #[c2rust::src_loc = "79:1"]
    pub type krb5_clpreauth_modreq = *mut krb5_clpreauth_modreq_st;
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
    /* Abstract types for module data and per-request module data. */
    #[c2rust::src_loc = "78:1"]
    pub type krb5_clpreauth_moddata = *mut krb5_clpreauth_moddata_st;
    #[c2rust::src_loc = "300:1"]
    pub type krb5_clpreauth_vtable = *mut krb5_clpreauth_vtable_st;
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
    /* Mandatory: name of module. */
    /* Mandatory: pointer to zero-terminated list of pa_types which this module
     * can provide services for. */
    /* Optional: pointer to zero-terminated list of enc_types which this module
     * claims to add support for. */
    /* Minor version 1 ends here. */
    /* Minor version 2 ends here. */
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
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    #[c2rust::src_loc = "83:1"]
    pub type krb5_clpreauth_callbacks = *mut krb5_clpreauth_callbacks_st;
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
    #[c2rust::src_loc = "214:1"]
    pub type krb5_clpreauth_request_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq) -> ()>;
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
    #[c2rust::src_loc = "189:1"]
    pub type krb5_clpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata) -> ()>;
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
    use super::krb5_h::{krb5_error_code, krb5_context,
                        krb5_get_init_creds_opt, krb5_preauthtype,
                        krb5_enctype, krb5_kdc_req, krb5_data, krb5_pa_data,
                        krb5_keyblock, krb5_boolean, krb5_timestamp,
                        krb5_int32, krb5_error, krb5_prompter_fct};
    extern "C" {
        #[c2rust::src_loc = "79:16"]
        pub type krb5_clpreauth_modreq_st;
        #[c2rust::src_loc = "78:16"]
        pub type krb5_clpreauth_moddata_st;
        #[c2rust::src_loc = "75:16"]
        pub type krb5_clpreauth_rock_st;
    }
    /* KRB5_CLPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:31"]
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
#[c2rust::header_src = "/usr/include/ctype.h:37"]
pub mod ctype_h {
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        #[c2rust::src_loc = "108:12"]
        pub fn isalnum(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "111:12"]
        pub fn isdigit(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "118:12"]
        pub fn isxdigit(_: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:31"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:31"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:31"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:31"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:31"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/assert.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:34"]
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
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_enc_data, krb5_enc_data,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _krb5_ticket,
                       krb5_ticket, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_error, krb5_error,
                       _krb5_prompt, krb5_prompt, krb5_prompter_fct,
                       krb5_responder_context, _krb5_responder_otp_tokeninfo,
                       krb5_responder_otp_tokeninfo,
                       _krb5_responder_otp_challenge,
                       krb5_responder_otp_challenge, krb5_get_init_creds_opt,
                       _krb5_get_init_creds_opt, _profile_t,
                       krb5_responder_context_st, krb5_free_data,
                       krb5_free_data_contents, krb5_responder_get_challenge,
                       krb5_responder_set_answer, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_otp_tokeninfo,
                         _krb5_otp_tokeninfo, krb5_pa_otp_challenge,
                         _krb5_pa_otp_challenge, krb5_pa_otp_req,
                         _krb5_pa_otp_req, string2data, empty_data, make_data,
                         data_eq_string, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5int_copy_data_contents, k5_free_otp_tokeninfo,
                         k5_free_pa_otp_challenge, k5_free_pa_otp_req,
                         encode_krb5_pa_otp_req, encode_krb5_pa_otp_enc_req,
                         decode_krb5_pa_otp_challenge};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_json_h::{k5_json_object, k5_json_array, k5_json_string,
                          k5_json_value, k5_json_tid, k5_json_number,
                          k5_json_object_st, k5_json_array_st,
                          k5_json_string_st, k5_json_number_st,
                          k5_json_get_tid, k5_json_release,
                          k5_json_array_create, k5_json_array_length,
                          k5_json_array_add, k5_json_array_get,
                          k5_json_object_create, k5_json_object_set,
                          k5_json_object_get, k5_json_string_create,
                          k5_json_string_create_len, k5_json_string_utf8,
                          k5_json_number_create, k5_json_number_value,
                          k5_json_encode, k5_json_decode};
pub use self::k5_int_pkinit_h::{krb5_algorithm_identifier,
                                _krb5_algorithm_identifier,
                                krb5_encrypt_helper};
pub use self::clpreauth_plugin_h::{krb5_clpreauth_modreq,
                                   krb5_clpreauth_supply_gic_opts_fn,
                                   krb5_clpreauth_moddata,
                                   krb5_clpreauth_vtable,
                                   krb5_clpreauth_vtable_st,
                                   krb5_clpreauth_prep_questions_fn,
                                   krb5_clpreauth_rock,
                                   krb5_clpreauth_callbacks,
                                   krb5_clpreauth_callbacks_st,
                                   krb5_clpreauth_tryagain_fn,
                                   krb5_clpreauth_process_fn,
                                   krb5_clpreauth_request_fini_fn,
                                   krb5_clpreauth_request_init_fn,
                                   krb5_clpreauth_get_flags_fn,
                                   krb5_clpreauth_fini_fn,
                                   krb5_clpreauth_init_fn,
                                   krb5_clpreauth_modreq_st,
                                   krb5_clpreauth_moddata_st,
                                   krb5_clpreauth_rock_st};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::ctype_h::{_ISprint, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc, isalnum,
                        isdigit, isxdigit};
use self::stdio_h::asprintf;
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::stdlib_h::{strtol, calloc, free};
use self::string_h::{strlen, strdup, memcmp, memset};
use self::assert_h::__assert_fail;
use self::os_proto_h::k5_set_prompt_types;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/preauth_otp.c - OTP clpreauth module */
/*
 * Copyright 2011 NORDUnet A/S.  All rights reserved.
 * Copyright 2011 Red Hat, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
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
#[c2rust::src_loc = "39:25"]
static mut otp_client_supported_pa_types: [krb5_preauthtype; 2] =
    [141 as libc::c_int, 0 as libc::c_int];
/* Frees a tokeninfo. */
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn free_tokeninfo(mut ti:
                                        *mut krb5_responder_otp_tokeninfo) {
    if ti.is_null() { return }
    free((*ti).alg_id as *mut libc::c_void);
    free((*ti).challenge as *mut libc::c_void);
    free((*ti).token_id as *mut libc::c_void);
    free((*ti).vendor as *mut libc::c_void);
    free(ti as *mut libc::c_void);
}
/* Converts a property of a json object into a char*. */
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn codec_value_to_string(mut obj: k5_json_object,
                                           mut key: *const libc::c_char,
                                           mut string: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    val = k5_json_object_get(obj, key);
    if val.is_null() { return 2 as libc::c_int }
    if k5_json_get_tid(val) != 131 as libc::c_int as libc::c_uint {
        return 22 as libc::c_int
    }
    str = strdup(k5_json_string_utf8(val as k5_json_string));
    if str.is_null() { return 12 as libc::c_int }
    *string = str;
    return 0 as libc::c_int;
}
/* Converts a property of a json object into a krb5_data struct. */
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn codec_value_to_data(mut obj: k5_json_object,
                                         mut key: *const libc::c_char,
                                         mut data: *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    retval = codec_value_to_string(obj, key, &mut tmp);
    if retval != 0 as libc::c_int { return retval }
    *data = string2data(tmp);
    return 0 as libc::c_int;
}
/* Converts a krb5_data struct into a property of a JSON object. */
#[c2rust::src_loc = "94:1"]
unsafe extern "C" fn codec_data_to_value(mut data: *mut krb5_data,
                                         mut obj: k5_json_object,
                                         mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut str: k5_json_string = 0 as *mut k5_json_string_st;
    if (*data).data.is_null() { return 0 as libc::c_int }
    retval =
        k5_json_string_create_len((*data).data as *const libc::c_void,
                                  (*data).length as size_t, &mut str);
    if retval != 0 { return retval }
    retval = k5_json_object_set(obj, key, str as k5_json_value);
    k5_json_release(str as k5_json_value);
    return retval;
}
/* Converts a property of a json object into a krb5_int32. */
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn codec_value_to_int32(mut obj: k5_json_object,
                                          mut key: *const libc::c_char,
                                          mut int32: *mut krb5_int32)
 -> krb5_error_code {
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    val = k5_json_object_get(obj, key);
    if val.is_null() { return 2 as libc::c_int }
    if k5_json_get_tid(val) != 0 as libc::c_int as libc::c_uint {
        return 22 as libc::c_int
    }
    *int32 = k5_json_number_value(val as k5_json_number) as krb5_int32;
    return 0 as libc::c_int;
}
/* Converts a krb5_int32 into a property of a JSON object. */
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn codec_int32_to_value(mut int32: krb5_int32,
                                          mut obj: k5_json_object,
                                          mut key: *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut num: k5_json_number = 0 as *mut k5_json_number_st;
    if int32 == -(1 as libc::c_int) { return 0 as libc::c_int }
    retval = k5_json_number_create(int32 as libc::c_longlong, &mut num);
    if retval != 0 { return retval }
    retval = k5_json_object_set(obj, key, num as k5_json_value);
    k5_json_release(num as k5_json_value);
    return retval;
}
/* Converts a krb5_otp_tokeninfo into a JSON object. */
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn codec_encode_tokeninfo(mut ti: *mut krb5_otp_tokeninfo,
                                            mut out: *mut k5_json_object)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut obj: k5_json_object = 0 as *mut k5_json_object_st;
    let mut flags: krb5_flags = 0;
    retval = k5_json_object_create(&mut obj);
    if !(retval != 0 as libc::c_int) {
        flags = 0x1 as libc::c_int;
        if (*ti).flags & 0x10000000 as libc::c_int != 0 {
            flags |= 0x2 as libc::c_int;
            if (*ti).flags & 0x2000000 as libc::c_int != 0 {
                flags |= 0x4 as libc::c_int
            }
        }
        if (*ti).flags & 0x40000000 as libc::c_int != 0 {
            flags |= 0x4 as libc::c_int
        }
        retval =
            codec_int32_to_value(flags, obj,
                                 b"flags\x00" as *const u8 as
                                     *const libc::c_char);
        if !(retval != 0 as libc::c_int) {
            retval =
                codec_data_to_value(&mut (*ti).vendor, obj,
                                    b"vendor\x00" as *const u8 as
                                        *const libc::c_char);
            if !(retval != 0 as libc::c_int) {
                retval =
                    codec_data_to_value(&mut (*ti).challenge, obj,
                                        b"challenge\x00" as *const u8 as
                                            *const libc::c_char);
                if !(retval != 0 as libc::c_int) {
                    retval =
                        codec_int32_to_value((*ti).length, obj,
                                             b"length\x00" as *const u8 as
                                                 *const libc::c_char);
                    if !(retval != 0 as libc::c_int) {
                        if (*ti).format != 0x4 as libc::c_int &&
                               (*ti).format != 0x3 as libc::c_int {
                            retval =
                                codec_int32_to_value((*ti).format, obj,
                                                     b"format\x00" as
                                                         *const u8 as
                                                         *const libc::c_char);
                            if retval != 0 as libc::c_int {
                                current_block = 6776018996030340608;
                            } else { current_block = 18317007320854588510; }
                        } else { current_block = 18317007320854588510; }
                        match current_block {
                            6776018996030340608 => { }
                            _ => {
                                retval =
                                    codec_data_to_value(&mut (*ti).token_id,
                                                        obj,
                                                        b"tokenID\x00" as
                                                            *const u8 as
                                                            *const libc::c_char);
                                if !(retval != 0 as libc::c_int) {
                                    retval =
                                        codec_data_to_value(&mut (*ti).alg_id,
                                                            obj,
                                                            b"algID\x00" as
                                                                *const u8 as
                                                                *const libc::c_char);
                                    if !(retval != 0 as libc::c_int) {
                                        *out = obj;
                                        return 0 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return retval;
}
/* Converts a krb5_pa_otp_challenge into a JSON object. */
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn codec_encode_challenge(mut ctx: krb5_context,
                                            mut chl:
                                                *mut krb5_pa_otp_challenge,
                                            mut json: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut obj: k5_json_object = 0 as k5_json_object;
    let mut tmp: k5_json_object = 0 as k5_json_object;
    let mut str: k5_json_string = 0 as k5_json_string;
    let mut arr: k5_json_array = 0 as k5_json_array;
    let mut retval: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    retval = k5_json_object_create(&mut obj);
    if !(retval != 0 as libc::c_int) {
        if !(*chl).service.data.is_null() {
            retval =
                k5_json_string_create_len((*chl).service.data as
                                              *const libc::c_void,
                                          (*chl).service.length as size_t,
                                          &mut str);
            if retval != 0 as libc::c_int {
                current_block = 9328358639563162749;
            } else {
                retval =
                    k5_json_object_set(obj,
                                       b"service\x00" as *const u8 as
                                           *const libc::c_char,
                                       str as k5_json_value);
                k5_json_release(str as k5_json_value);
                if retval != 0 as libc::c_int {
                    current_block = 9328358639563162749;
                } else { current_block = 17965632435239708295; }
            }
        } else { current_block = 17965632435239708295; }
        match current_block {
            9328358639563162749 => { }
            _ => {
                retval = k5_json_array_create(&mut arr);
                if !(retval != 0 as libc::c_int) {
                    i = 0 as libc::c_int;
                    loop  {
                        if (*(*chl).tokeninfo.offset(i as isize)).is_null() {
                            current_block = 12147880666119273379;
                            break ;
                        }
                        retval =
                            codec_encode_tokeninfo(*(*chl).tokeninfo.offset(i
                                                                                as
                                                                                isize),
                                                   &mut tmp);
                        if retval != 0 as libc::c_int {
                            current_block = 9328358639563162749;
                            break ;
                        }
                        retval = k5_json_array_add(arr, tmp as k5_json_value);
                        k5_json_release(tmp as k5_json_value);
                        if retval != 0 as libc::c_int {
                            current_block = 9328358639563162749;
                            break ;
                        }
                        i += 1
                    }
                    match current_block {
                        9328358639563162749 => { }
                        _ => {
                            retval =
                                k5_json_object_set(obj,
                                                   b"tokenInfo\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   arr as k5_json_value);
                            if !(retval != 0 as libc::c_int) {
                                retval =
                                    k5_json_encode(obj as k5_json_value,
                                                   json);
                                (retval) != 0;
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(arr as k5_json_value);
    k5_json_release(obj as k5_json_value);
    return retval;
}
/* Converts a JSON object into a krb5_responder_otp_tokeninfo. */
#[c2rust::src_loc = "264:1"]
unsafe extern "C" fn codec_decode_tokeninfo(mut obj: k5_json_object)
 -> *mut krb5_responder_otp_tokeninfo {
    let mut current_block: u64;
    let mut ti: *mut krb5_responder_otp_tokeninfo =
        0 as *mut krb5_responder_otp_tokeninfo;
    let mut retval: krb5_error_code = 0;
    ti =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_responder_otp_tokeninfo>() as
                   libc::c_ulong) as *mut krb5_responder_otp_tokeninfo;
    if !ti.is_null() {
        retval =
            codec_value_to_int32(obj,
                                 b"flags\x00" as *const u8 as
                                     *const libc::c_char, &mut (*ti).flags);
        if !(retval != 0 as libc::c_int) {
            retval =
                codec_value_to_string(obj,
                                      b"vendor\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut (*ti).vendor);
            if !(retval != 0 as libc::c_int && retval != 2 as libc::c_int) {
                retval =
                    codec_value_to_string(obj,
                                          b"challenge\x00" as *const u8 as
                                              *const libc::c_char,
                                          &mut (*ti).challenge);
                if !(retval != 0 as libc::c_int && retval != 2 as libc::c_int)
                   {
                    retval =
                        codec_value_to_int32(obj,
                                             b"length\x00" as *const u8 as
                                                 *const libc::c_char,
                                             &mut (*ti).length);
                    if retval == 2 as libc::c_int {
                        (*ti).length = -(1 as libc::c_int);
                        current_block = 1856101646708284338;
                    } else if retval != 0 as libc::c_int {
                        current_block = 3643028752360416957;
                    } else { current_block = 1856101646708284338; }
                    match current_block {
                        3643028752360416957 => { }
                        _ => {
                            retval =
                                codec_value_to_int32(obj,
                                                     b"format\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     &mut (*ti).format);
                            if retval == 2 as libc::c_int {
                                (*ti).format = -(1 as libc::c_int);
                                current_block = 8831408221741692167;
                            } else if retval != 0 as libc::c_int {
                                current_block = 3643028752360416957;
                            } else { current_block = 8831408221741692167; }
                            match current_block {
                                3643028752360416957 => { }
                                _ => {
                                    retval =
                                        codec_value_to_string(obj,
                                                              b"tokenID\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              &mut (*ti).token_id);
                                    if !(retval != 0 as libc::c_int &&
                                             retval != 2 as libc::c_int) {
                                        retval =
                                            codec_value_to_string(obj,
                                                                  b"algID\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  &mut (*ti).alg_id);
                                        if !(retval != 0 as libc::c_int &&
                                                 retval != 2 as libc::c_int) {
                                            return ti
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
    free_tokeninfo(ti);
    return 0 as *mut krb5_responder_otp_tokeninfo;
}
/* Converts a JSON object into a krb5_responder_otp_challenge. */
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn codec_decode_challenge(mut ctx: krb5_context,
                                            mut json: *const libc::c_char)
 -> *mut krb5_responder_otp_challenge {
    let mut current_block: u64;
    let mut chl: *mut krb5_responder_otp_challenge =
        0 as *mut krb5_responder_otp_challenge;
    let mut obj: k5_json_value = 0 as *mut libc::c_void;
    let mut arr: k5_json_value = 0 as *mut libc::c_void;
    let mut tmp: k5_json_value = 0 as *mut libc::c_void;
    let mut retval: krb5_error_code = 0;
    let mut i: size_t = 0;
    retval = k5_json_decode(json, &mut obj);
    if !(retval != 0 as libc::c_int) {
        if !(k5_json_get_tid(obj) != 130 as libc::c_int as libc::c_uint) {
            arr =
                k5_json_object_get(obj as k5_json_object,
                                   b"tokenInfo\x00" as *const u8 as
                                       *const libc::c_char);
            if !arr.is_null() {
                if !(k5_json_get_tid(arr) !=
                         129 as libc::c_int as libc::c_uint) {
                    chl =
                        calloc(1 as libc::c_int as libc::c_ulong,
                               ::std::mem::size_of::<krb5_responder_otp_challenge>()
                                   as libc::c_ulong) as
                            *mut krb5_responder_otp_challenge;
                    if !chl.is_null() {
                        (*chl).tokeninfo =
                            calloc(k5_json_array_length(arr as
                                                            k5_json_array).wrapping_add(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_ulong),
                                   ::std::mem::size_of::<*mut krb5_responder_otp_tokeninfo>()
                                       as libc::c_ulong) as
                                *mut *mut krb5_responder_otp_tokeninfo;
                        if !(*chl).tokeninfo.is_null() {
                            retval =
                                codec_value_to_string(obj as k5_json_object,
                                                      b"service\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      &mut (*chl).service);
                            if !(retval != 0 as libc::c_int &&
                                     retval != 2 as libc::c_int) {
                                i = 0 as libc::c_int as size_t;
                                loop  {
                                    if !(i <
                                             k5_json_array_length(arr as
                                                                      k5_json_array))
                                       {
                                        current_block = 4495394744059808450;
                                        break ;
                                    }
                                    tmp =
                                        k5_json_array_get(arr as
                                                              k5_json_array,
                                                          i);
                                    if k5_json_get_tid(tmp) !=
                                           130 as libc::c_int as libc::c_uint
                                       {
                                        current_block = 7770080906459629539;
                                        break ;
                                    }
                                    let ref mut fresh0 =
                                        *(*chl).tokeninfo.offset(i as isize);
                                    *fresh0 =
                                        codec_decode_tokeninfo(tmp as
                                                                   k5_json_object);
                                    if (*(*chl).tokeninfo.offset(i as
                                                                     isize)).is_null()
                                       {
                                        current_block = 7770080906459629539;
                                        break ;
                                    }
                                    i = i.wrapping_add(1)
                                }
                                match current_block {
                                    7770080906459629539 => { }
                                    _ => { k5_json_release(obj); return chl }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !chl.is_null() {
        i = 0 as libc::c_int as size_t;
        while !(*chl).tokeninfo.is_null() &&
                  !(*(*chl).tokeninfo.offset(i as isize)).is_null() {
            free_tokeninfo(*(*chl).tokeninfo.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free((*chl).tokeninfo as *mut libc::c_void);
        free(chl as *mut libc::c_void);
    }
    k5_json_release(obj);
    return 0 as *mut krb5_responder_otp_challenge;
}
/* Decode the responder answer into a tokeninfo, a value and a pin. */
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn codec_decode_answer(mut context: krb5_context,
                                         mut answer: *const libc::c_char,
                                         mut tis:
                                             *mut *mut krb5_otp_tokeninfo,
                                         mut ti: *mut *mut krb5_otp_tokeninfo,
                                         mut value: *mut krb5_data,
                                         mut pin: *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut val: k5_json_value = 0 as *mut libc::c_void;
    let mut indx: krb5_int32 = 0;
    let mut i: krb5_int32 = 0;
    let mut tmp: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    if answer.is_null() { return 74 as libc::c_int }
    retval = k5_json_decode(answer, &mut val);
    if !(retval != 0 as libc::c_int) {
        if !(k5_json_get_tid(val) != 130 as libc::c_int as libc::c_uint) {
            retval =
                codec_value_to_int32(val as k5_json_object,
                                     b"tokeninfo\x00" as *const u8 as
                                         *const libc::c_char, &mut indx);
            if !(retval != 0 as libc::c_int) {
                i = 0 as libc::c_int;
                loop  {
                    if (*tis.offset(i as isize)).is_null() {
                        current_block = 4495394744059808450;
                        break ;
                    }
                    if i == indx {
                        retval =
                            codec_value_to_data(val as k5_json_object,
                                                b"value\x00" as *const u8 as
                                                    *const libc::c_char,
                                                &mut tmp);
                        if retval != 0 as libc::c_int &&
                               retval != 2 as libc::c_int {
                            current_block = 12996488930175181862;
                            break ;
                        }
                        retval =
                            codec_value_to_data(val as k5_json_object,
                                                b"pin\x00" as *const u8 as
                                                    *const libc::c_char, pin);
                        if retval != 0 as libc::c_int &&
                               retval != 2 as libc::c_int {
                            krb5_free_data_contents(context, &mut tmp);
                            current_block = 12996488930175181862;
                            break ;
                        } else {
                            *value = tmp;
                            *ti = *tis.offset(i as isize);
                            retval = 0 as libc::c_int;
                            current_block = 12996488930175181862;
                            break ;
                        }
                    } else { i += 1 }
                }
                match current_block {
                    12996488930175181862 => { }
                    _ => { retval = 22 as libc::c_int }
                }
            }
        }
    }
    k5_json_release(val);
    return retval;
}
/* Takes the nonce from the challenge and encrypts it into the request. */
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn encrypt_nonce(mut ctx: krb5_context,
                                   mut key: *mut krb5_keyblock,
                                   mut chl: *const krb5_pa_otp_challenge,
                                   mut req: *mut krb5_pa_otp_req)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut encdata: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut er: *mut krb5_data = 0 as *mut krb5_data;
    /* Encode the nonce. */
    retval = encode_krb5_pa_otp_enc_req(&(*chl).nonce, &mut er);
    if retval != 0 as libc::c_int { return retval }
    /* Do the encryption. */
    retval =
        krb5_encrypt_helper(ctx, key, 45 as libc::c_int, er, &mut encdata);
    krb5_free_data(ctx, er);
    if retval != 0 as libc::c_int { return retval }
    (*req).enc_data = encdata;
    return 0 as libc::c_int;
}
/* Checks to see if the user-supplied otp value matches the length and format
 * of the supplied tokeninfo. */
#[c2rust::src_loc = "450:1"]
unsafe extern "C" fn otpvalue_matches_tokeninfo(mut otpvalue:
                                                    *const libc::c_char,
                                                mut ti:
                                                    *mut krb5_otp_tokeninfo)
 -> libc::c_int {
    let mut table:
            [Option<unsafe extern "C" fn(_: libc::c_int) -> libc::c_int>; 3] =
        [Some(isdigit as unsafe extern "C" fn(_: libc::c_int) -> libc::c_int),
         Some(isxdigit as
                  unsafe extern "C" fn(_: libc::c_int) -> libc::c_int),
         Some(isalnum as
                  unsafe extern "C" fn(_: libc::c_int) -> libc::c_int)];
    if otpvalue.is_null() || ti.is_null() { return 0 as libc::c_int }
    if (*ti).length >= 0 as libc::c_int &&
           strlen(otpvalue) != (*ti).length as size_t {
        return 0 as libc::c_int
    }
    if (*ti).format >= 0 as libc::c_int && (*ti).format < 3 as libc::c_int {
        while *otpvalue != 0 {
            let fresh1 = otpvalue;
            otpvalue = otpvalue.offset(1);
            if Some((*table.as_mut_ptr().offset((*ti).format as
                                                    isize)).expect("non-null function pointer")).expect("non-null function pointer")(*fresh1
                                                                                                                                         as
                                                                                                                                         libc::c_uchar
                                                                                                                                         as
                                                                                                                                         libc::c_int)
                   == 0 {
                return 0 as libc::c_int
            }
        }
    }
    return 1 as libc::c_int;
}
/* Performs a prompt and saves the response in the out parameter. */
#[c2rust::src_loc = "472:1"]
unsafe extern "C" fn doprompt(mut context: krb5_context,
                              mut prompter: krb5_prompter_fct,
                              mut prompter_data: *mut libc::c_void,
                              mut banner: *const libc::c_char,
                              mut prompttxt: *const libc::c_char,
                              mut out: *mut libc::c_char, mut len: size_t)
 -> krb5_error_code {
    let mut prompt: krb5_prompt =
        krb5_prompt{prompt: 0 as *mut libc::c_char,
                    hidden: 0,
                    reply: 0 as *mut krb5_data,};
    let mut prompt_reply: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut retval: krb5_error_code = 0;
    let mut prompt_type: krb5_prompt_type = 0x4 as libc::c_int;
    if prompttxt.is_null() || out.is_null() { return 22 as libc::c_int }
    memset(out as *mut libc::c_void, 0 as libc::c_int, len);
    prompt_reply = make_data(out as *mut libc::c_void, len as libc::c_uint);
    prompt.reply = &mut prompt_reply;
    prompt.prompt = prompttxt as *mut libc::c_char;
    prompt.hidden = 1 as libc::c_int;
    /* PROMPTER_INVOCATION */
    k5_set_prompt_types(context, &mut prompt_type);
    retval =
        Some(prompter.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                               prompter_data,
                                                                                               0
                                                                                                   as
                                                                                                   *const libc::c_char,
                                                                                               banner,
                                                                                               1
                                                                                                   as
                                                                                                   libc::c_int,
                                                                                               &mut prompt);
    k5_set_prompt_types(context, 0 as *mut krb5_prompt_type);
    if retval != 0 as libc::c_int { return retval }
    return 0 as libc::c_int;
}
/* Forces the user to choose a single tokeninfo via prompting. */
#[c2rust::src_loc = "502:1"]
unsafe extern "C" fn prompt_for_tokeninfo(mut context: krb5_context,
                                          mut prompter: krb5_prompter_fct,
                                          mut prompter_data:
                                              *mut libc::c_void,
                                          mut tis:
                                              *mut *mut krb5_otp_tokeninfo,
                                          mut out_ti:
                                              *mut *mut krb5_otp_tokeninfo)
 -> krb5_error_code {
    let mut banner: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: [libc::c_char; 1024] = [0; 1024];
    let mut ti: *mut krb5_otp_tokeninfo = 0 as *mut krb5_otp_tokeninfo;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !(*tis.offset(i as isize)).is_null() {
        if asprintf(&mut tmp as *mut *mut libc::c_char,
                    b"%s\t%d. %s %.*s\n\x00" as *const u8 as
                        *const libc::c_char,
                    (if !banner.is_null() {
                         banner
                     } else {
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Please choose from the following:\n\x00"
                                      as *const u8 as *const libc::c_char)
                     }), i + 1 as libc::c_int,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Vendor:\x00" as *const u8 as
                                 *const libc::c_char),
                    (**tis.offset(i as isize)).vendor.length,
                    (**tis.offset(i as isize)).vendor.data) < 0 as libc::c_int
           {
            free(banner as *mut libc::c_void);
            return 12 as libc::c_int
        }
        free(banner as *mut libc::c_void);
        banner = tmp;
        i += 1
    }
    loop  {
        retval =
            doprompt(context, prompter, prompter_data, banner,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Enter #\x00" as *const u8 as
                                  *const libc::c_char), response.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong);
        if retval != 0 as libc::c_int {
            free(banner as *mut libc::c_void);
            return retval
        }
        *__errno_location() = 0 as libc::c_int;
        j =
            strtol(response.as_mut_ptr(), 0 as *mut *mut libc::c_char,
                   0 as libc::c_int) as libc::c_int;
        if *__errno_location() != 0 as libc::c_int {
            free(banner as *mut libc::c_void);
            return *__errno_location()
        }
        if !(j < 1 as libc::c_int || j > i) {
            j -= 1;
            ti = *tis.offset(j as isize)
        }
        if !ti.is_null() { break ; }
    }
    free(banner as *mut libc::c_void);
    *out_ti = ti;
    return 0 as libc::c_int;
}
/* Builds a challenge string from the given tokeninfo. */
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn make_challenge(mut ti: *const krb5_otp_tokeninfo,
                                    mut challenge: *mut *mut libc::c_char)
 -> krb5_error_code {
    if challenge.is_null() { return 22 as libc::c_int }
    *challenge = 0 as *mut libc::c_char;
    if ti.is_null() || (*ti).challenge.data.is_null() {
        return 0 as libc::c_int
    }
    if asprintf(challenge,
                b"%s %.*s\n\x00" as *const u8 as *const libc::c_char,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"OTP Challenge:\x00" as *const u8 as
                             *const libc::c_char), (*ti).challenge.length,
                (*ti).challenge.data) < 0 as libc::c_int {
        return 12 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Determines if a pin is required. If it is, it will be prompted for. */
#[inline]
#[c2rust::src_loc = "573:1"]
unsafe extern "C" fn collect_pin(mut context: krb5_context,
                                 mut prompter: krb5_prompter_fct,
                                 mut prompter_data: *mut libc::c_void,
                                 mut ti: *const krb5_otp_tokeninfo,
                                 mut out_pin: *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut otppin: [libc::c_char; 1024] = [0; 1024];
    let mut collect: krb5_flags = 0;
    let mut pin: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    /* If no PIN will be collected, don't prompt. */
    collect =
        (*ti).flags & (0x10000000 as libc::c_int | 0x2000000 as libc::c_int);
    if collect == 0 as libc::c_int {
        *out_pin = empty_data();
        return 0 as libc::c_int
    }
    /* Collect the PIN. */
    retval =
        doprompt(context, prompter, prompter_data, 0 as *const libc::c_char,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"OTP Token PIN\x00" as *const u8 as
                              *const libc::c_char), otppin.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong);
    if retval != 0 as libc::c_int { return retval }
    /* Set the PIN. */
    pin =
        make_data(strdup(otppin.as_mut_ptr()) as *mut libc::c_void,
                  strlen(otppin.as_mut_ptr()) as libc::c_uint);
    if pin.data.is_null() { return 12 as libc::c_int }
    *out_pin = pin;
    return 0 as libc::c_int;
}
/* Builds a request using the specified tokeninfo, value and pin. */
#[c2rust::src_loc = "607:1"]
unsafe extern "C" fn make_request(mut ctx: krb5_context,
                                  mut ti: *mut krb5_otp_tokeninfo,
                                  mut value: *const krb5_data,
                                  mut pin: *const krb5_data,
                                  mut out_req: *mut *mut krb5_pa_otp_req)
 -> krb5_error_code {
    let mut current_block: u64; /* No pin found! */
    let mut req: *mut krb5_pa_otp_req = 0 as *mut krb5_pa_otp_req;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    if ti.is_null() { return 0 as libc::c_int }
    if (*ti).format == 0x4 as libc::c_int { return 95 as libc::c_int }
    req =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_pa_otp_req>() as libc::c_ulong) as
            *mut krb5_pa_otp_req;
    if req.is_null() { return 12 as libc::c_int }
    (*req).flags = (*ti).flags & 0x40000000 as libc::c_int;
    retval =
        krb5int_copy_data_contents(ctx, &mut (*ti).vendor,
                                   &mut (*req).vendor);
    if !(retval != 0 as libc::c_int) {
        (*req).format = (*ti).format;
        retval =
            krb5int_copy_data_contents(ctx, &mut (*ti).token_id,
                                       &mut (*req).token_id);
        if !(retval != 0 as libc::c_int) {
            retval =
                krb5int_copy_data_contents(ctx, &mut (*ti).alg_id,
                                           &mut (*req).alg_id);
            if !(retval != 0 as libc::c_int) {
                retval =
                    krb5int_copy_data_contents(ctx, value,
                                               &mut (*req).otp_value);
                if !(retval != 0 as libc::c_int) {
                    if (*ti).flags & 0x10000000 as libc::c_int != 0 {
                        if (*ti).flags & 0x2000000 as libc::c_int != 0 {
                            if pin.is_null() || (*pin).data.is_null() {
                                retval = 22 as libc::c_int;
                                current_block = 9594981888836540018;
                            } else {
                                retval =
                                    krb5int_copy_data_contents(ctx, pin,
                                                               &mut (*req).pin);
                                if retval != 0 as libc::c_int {
                                    current_block = 9594981888836540018;
                                } else {
                                    current_block = 17281240262373992796;
                                }
                            }
                        } else if !pin.is_null() && !(*pin).data.is_null() {
                            krb5_free_data_contents(ctx,
                                                    &mut (*req).otp_value);
                            retval =
                                asprintf(&mut (*req).otp_value.data as
                                             *mut *mut libc::c_char,
                                         b"%.*s%.*s\x00" as *const u8 as
                                             *const libc::c_char,
                                         (*pin).length, (*pin).data,
                                         (*value).length, (*value).data);
                            if retval < 0 as libc::c_int {
                                retval = 12 as libc::c_int;
                                (*req).otp_value = empty_data();
                                current_block = 9594981888836540018;
                            } else {
                                (*req).otp_value.length =
                                    (*req).pin.length.wrapping_add((*req).otp_value.length);
                                current_block = 17281240262373992796;
                            }
                        } else { current_block = 17281240262373992796; }
                        /* Otherwise, the responder has already combined them. */
                    } else { current_block = 17281240262373992796; }
                    match current_block {
                        9594981888836540018 => { }
                        _ => { *out_req = req; return 0 as libc::c_int }
                    }
                }
            }
        }
    }
    k5_free_pa_otp_req(ctx, req);
    return retval;
}
/*
 * Filters a set of tokeninfos given an otp value.  If the set is reduced to
 * a single tokeninfo, it will be set in out_ti.  Otherwise, a new shallow copy
 * will be allocated in out_filtered.
 */
#[inline]
#[c2rust::src_loc = "681:1"]
unsafe extern "C" fn filter_tokeninfos(mut context: krb5_context,
                                       mut otpvalue: *const libc::c_char,
                                       mut tis: *mut *mut krb5_otp_tokeninfo,
                                       mut out_filtered:
                                           *mut *mut *mut krb5_otp_tokeninfo,
                                       mut out_ti:
                                           *mut *mut krb5_otp_tokeninfo)
 -> krb5_error_code {
    let mut filtered: *mut *mut krb5_otp_tokeninfo =
        0 as *mut *mut krb5_otp_tokeninfo;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    while !(*tis.offset(i as isize)).is_null() { i = i.wrapping_add(1) }
    filtered =
        calloc(i.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*const krb5_otp_tokeninfo>() as
                   libc::c_ulong) as *mut *mut krb5_otp_tokeninfo;
    if filtered.is_null() { return 12 as libc::c_int }
    /* Make a list of tokeninfos that match the value. */
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while !(*tis.offset(i as isize)).is_null() {
        if otpvalue_matches_tokeninfo(otpvalue, *tis.offset(i as isize)) != 0
           {
            let fresh2 = j;
            j = j.wrapping_add(1);
            let ref mut fresh3 = *filtered.offset(fresh2 as isize);
            *fresh3 = *tis.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    /* It is an error if we have no matching tokeninfos. */
    if (*filtered.offset(0 as libc::c_int as isize)).is_null() {
        free(filtered as *mut libc::c_void);
        krb5_set_error_message(context,
                               -(1765328174 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"OTP value doesn\'t match any token formats\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        return -(1765328174 as libc::c_long) as krb5_error_code
        /* We have no supported tokeninfos. */
    }
    /* Otherwise, if we have just one tokeninfo, choose it. */
    if (*filtered.offset(1 as libc::c_int as isize)).is_null() {
        *out_ti = *filtered.offset(0 as libc::c_int as isize);
        *out_filtered = 0 as *mut *mut krb5_otp_tokeninfo;
        free(filtered as *mut libc::c_void);
        return 0 as libc::c_int
    }
    /* Otherwise, we'll return the remaining list. */
    *out_ti = 0 as *mut krb5_otp_tokeninfo;
    *out_filtered = filtered;
    return 0 as libc::c_int;
}
/* Outputs the selected tokeninfo and possibly a value and pin.
 * Prompting may occur. */
#[c2rust::src_loc = "727:1"]
unsafe extern "C" fn prompt_for_token(mut context: krb5_context,
                                      mut prompter: krb5_prompter_fct,
                                      mut prompter_data: *mut libc::c_void,
                                      mut tis: *mut *mut krb5_otp_tokeninfo,
                                      mut out_ti:
                                          *mut *mut krb5_otp_tokeninfo,
                                      mut out_value: *mut krb5_data,
                                      mut out_pin: *mut krb5_data)
 -> krb5_error_code {
    let mut filtered: *mut *mut krb5_otp_tokeninfo =
        0 as *mut *mut krb5_otp_tokeninfo;
    let mut ti: *mut krb5_otp_tokeninfo = 0 as *mut krb5_otp_tokeninfo;
    let mut retval: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut challengers: libc::c_int = 0 as libc::c_int;
    let mut challenge: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut otpvalue: [libc::c_char; 1024] = [0; 1024];
    let mut value: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pin: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    memset(otpvalue.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong);
    if tis.is_null() || (*tis.offset(0 as libc::c_int as isize)).is_null() ||
           out_ti.is_null() {
        return 22 as libc::c_int
    }
    /* Count how many challenges we have. */
    i = 0 as libc::c_int;
    while !(*tis.offset(i as isize)).is_null() {
        if !(**tis.offset(i as isize)).challenge.data.is_null() {
            challengers += 1
        }
        i += 1
    }
    /* If we have only one tokeninfo as input, choose it. */
    if i == 1 as libc::c_int { ti = *tis.offset(0 as libc::c_int as isize) }
    /* Setup our challenge, if present. */
    if challengers > 0 as libc::c_int {
        /* If we have multiple tokeninfos still, choose now. */
        if ti.is_null() {
            retval =
                prompt_for_tokeninfo(context, prompter, prompter_data, tis,
                                     &mut ti);
            if retval != 0 as libc::c_int { return retval }
        }
        /* Create the challenge prompt. */
        retval = make_challenge(ti, &mut challenge);
        if retval != 0 as libc::c_int { return retval }
    }
    /* Prompt for token value. */
    retval =
        doprompt(context, prompter, prompter_data, challenge,
                 dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Enter OTP Token Value\x00" as *const u8 as
                              *const libc::c_char), otpvalue.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong);
    free(challenge as *mut libc::c_void);
    if retval != 0 as libc::c_int { return retval }
    if ti.is_null() {
        /* Filter out tokeninfos that don't match our token value. */
        retval =
            filter_tokeninfos(context, otpvalue.as_mut_ptr(), tis,
                              &mut filtered, &mut ti);
        if retval != 0 as libc::c_int { return retval }
        /* If we still don't have a single tokeninfo, choose now. */
        if !filtered.is_null() {
            retval =
                prompt_for_tokeninfo(context, prompter, prompter_data,
                                     filtered, &mut ti);
            free(filtered as *mut libc::c_void);
            if retval != 0 as libc::c_int { return retval }
        }
    }
    if !ti.is_null() {
    } else {
        __assert_fail(b"ti != NULL\x00" as *const u8 as *const libc::c_char,
                      b"preauth_otp.c\x00" as *const u8 as
                          *const libc::c_char,
                      795 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 146],
                                                &[libc::c_char; 146]>(b"krb5_error_code prompt_for_token(krb5_context, krb5_prompter_fct, void *, krb5_otp_tokeninfo **, krb5_otp_tokeninfo **, krb5_data *, krb5_data *)\x00")).as_ptr());
    }
    /* Set the value. */
    value =
        make_data(strdup(otpvalue.as_mut_ptr()) as *mut libc::c_void,
                  strlen(otpvalue.as_mut_ptr()) as libc::c_uint);
    if value.data.is_null() { return 12 as libc::c_int }
    /* Collect the PIN, if necessary. */
    retval = collect_pin(context, prompter, prompter_data, ti, &mut pin);
    if retval != 0 as libc::c_int {
        krb5_free_data_contents(context, &mut value);
        return retval
    }
    *out_value = value;
    *out_pin = pin;
    *out_ti = ti;
    return 0 as libc::c_int;
}
/* Encode the OTP request into a krb5_pa_data buffer. */
#[c2rust::src_loc = "816:1"]
unsafe extern "C" fn set_pa_data(mut req: *const krb5_pa_otp_req,
                                 mut pa_data_out: *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut out: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut tmp: *mut krb5_data = 0 as *mut krb5_data;
    /* Allocate the preauth data array and one item. */
    out =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut krb5_pa_data>() as libc::c_ulong) as
            *mut *mut krb5_pa_data;
    if !out.is_null() {
        let ref mut fresh4 = *out.offset(0 as libc::c_int as isize);
        *fresh4 =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong) as
                *mut krb5_pa_data;
        let ref mut fresh5 = *out.offset(1 as libc::c_int as isize);
        *fresh5 = 0 as *mut krb5_pa_data;
        if !(*out.offset(0 as libc::c_int as isize)).is_null() {
            /* Encode our request into the preauth data item. */
            memset(*out.offset(0 as libc::c_int as isize) as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong);
            (**out.offset(0 as libc::c_int as isize)).pa_type =
                142 as libc::c_int;
            if !(encode_krb5_pa_otp_req(req, &mut tmp) != 0 as libc::c_int) {
                let ref mut fresh6 =
                    (**out.offset(0 as libc::c_int as isize)).contents;
                *fresh6 = (*tmp).data as *mut krb5_octet;
                (**out.offset(0 as libc::c_int as isize)).length =
                    (*tmp).length;
                free(tmp as *mut libc::c_void);
                *pa_data_out = out;
                return 0 as libc::c_int
            }
        }
    }
    if !out.is_null() {
        free(*out.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        free(out as *mut libc::c_void);
    }
    return 12 as libc::c_int;
}
/* Tests krb5_data to see if it is printable. */
#[c2rust::src_loc = "852:1"]
unsafe extern "C" fn is_printable_string(mut data: *const krb5_data)
 -> krb5_boolean {
    let mut i: libc::c_uint = 0;
    if data.is_null() { return 0 as libc::c_int as krb5_boolean }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*data).length {
        if *(*__ctype_b_loc()).offset(*(*data).data.offset(i as isize) as
                                          libc::c_uchar as libc::c_int as
                                          isize) as libc::c_int &
               _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            return 0 as libc::c_int as krb5_boolean
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int as krb5_boolean;
}
/* Returns TRUE when the given tokeninfo contains the subset of features we
 * support. */
#[c2rust::src_loc = "870:1"]
unsafe extern "C" fn is_tokeninfo_supported(mut ti: *mut krb5_otp_tokeninfo)
 -> krb5_boolean {
    let mut supported_flags: krb5_flags =
        0x10000000 as libc::c_int | 0x8000000 as libc::c_int |
            0x2000000 as libc::c_int;
    /* Flags we don't support... */
    if (*ti).flags & !supported_flags != 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    /* We don't currently support hashing. */
    if !(*ti).supported_hash_alg.is_null() ||
           (*ti).iteration_count >= 0 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    /* Remove tokeninfos with invalid vendor strings. */
    if is_printable_string(&mut (*ti).vendor) == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    /* Remove tokeninfos with non-printable challenges. */
    if is_printable_string(&mut (*ti).challenge) == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    /* We don't currently support base64. */
    if (*ti).format == 0x4 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    return 1 as libc::c_int as krb5_boolean;
}
/* Removes unsupported tokeninfos. Returns an error if no tokeninfos remain. */
#[c2rust::src_loc = "901:1"]
unsafe extern "C" fn filter_supported_tokeninfos(mut context: krb5_context,
                                                 mut tis:
                                                     *mut *mut krb5_otp_tokeninfo)
 -> krb5_error_code {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    /* Filter out any tokeninfos we don't support. */
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while !(*tis.offset(i as isize)).is_null() {
        if is_tokeninfo_supported(*tis.offset(i as isize)) == 0 {
            k5_free_otp_tokeninfo(context, *tis.offset(i as isize));
        } else {
            let fresh7 = j;
            j = j.wrapping_add(1);
            let ref mut fresh8 = *tis.offset(fresh7 as isize);
            *fresh8 = *tis.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    /* Terminate the array. */
    let ref mut fresh9 = *tis.offset(j as isize);
    *fresh9 = 0 as *mut krb5_otp_tokeninfo;
    if !(*tis.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    krb5_set_error_message(context,
                           -(1765328174 as libc::c_long) as krb5_error_code,
                           dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"No supported tokens\x00" as *const u8 as
                                        *const libc::c_char));
    return -(1765328174 as libc::c_long) as krb5_error_code;
    /* We have no supported tokeninfos. */
}
/*
 * Try to find tokeninfos which match configuration data recorded in the input
 * ccache, and if exactly one is found, drop the rest.
 */
#[c2rust::src_loc = "928:1"]
unsafe extern "C" fn filter_config_tokeninfos(mut context: krb5_context,
                                              mut cb:
                                                  krb5_clpreauth_callbacks,
                                              mut rock: krb5_clpreauth_rock,
                                              mut tis:
                                                  *mut *mut krb5_otp_tokeninfo)
 -> krb5_error_code {
    let mut match_0: *mut krb5_otp_tokeninfo = 0 as *mut krb5_otp_tokeninfo;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut vendor: *const libc::c_char = 0 as *const libc::c_char;
    let mut alg_id: *const libc::c_char = 0 as *const libc::c_char;
    let mut token_id: *const libc::c_char = 0 as *const libc::c_char;
    /* Pull up what we know about the token we want to use. */
    vendor =
        (*cb).get_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"vendor\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
    alg_id =
        (*cb).get_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"algID\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
    token_id =
        (*cb).get_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"tokenID\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
    /* Look for a single matching entry. */
    i = 0 as libc::c_int as size_t;
    while !(*tis.offset(i as isize)).is_null() {
        if !(!vendor.is_null() &&
                 (**tis.offset(i as isize)).vendor.length >
                     0 as libc::c_int as libc::c_uint &&
                 data_eq_string((**tis.offset(i as isize)).vendor, vendor) ==
                     0) {
            if !(!alg_id.is_null() &&
                     (**tis.offset(i as isize)).alg_id.length >
                         0 as libc::c_int as libc::c_uint &&
                     data_eq_string((**tis.offset(i as isize)).alg_id, alg_id)
                         == 0) {
                if !(!token_id.is_null() &&
                         (**tis.offset(i as isize)).token_id.length >
                             0 as libc::c_int as libc::c_uint &&
                         data_eq_string((**tis.offset(i as isize)).token_id,
                                        token_id) == 0) {
                    /* Oh, we already had a matching entry. More than one -> no change. */
                    if !match_0.is_null() { return 0 as libc::c_int }
                    match_0 = *tis.offset(i as isize)
                }
            }
        }
        i = i.wrapping_add(1)
    }
    /* No matching entry -> no change. */
    if match_0.is_null() { return 0 as libc::c_int }
    /* Prune out everything except the best match. */
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while !(*tis.offset(i as isize)).is_null() {
        if *tis.offset(i as isize) != match_0 {
            k5_free_otp_tokeninfo(context, *tis.offset(i as isize));
        } else {
            let fresh10 = j;
            j = j.wrapping_add(1);
            let ref mut fresh11 = *tis.offset(fresh10 as isize);
            *fresh11 = *tis.offset(i as isize)
        }
        i = i.wrapping_add(1)
    }
    let ref mut fresh12 = *tis.offset(j as isize);
    *fresh12 = 0 as *mut krb5_otp_tokeninfo;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "976:1"]
unsafe extern "C" fn otp_client_request_init(mut context: krb5_context,
                                             mut moddata:
                                                 krb5_clpreauth_moddata,
                                             mut modreq_out:
                                                 *mut krb5_clpreauth_modreq) {
    *modreq_out =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut krb5_pa_otp_challenge>() as
                   libc::c_ulong) as krb5_clpreauth_modreq;
}
#[c2rust::src_loc = "983:1"]
unsafe extern "C" fn otp_client_prep_questions(mut context: krb5_context,
                                               mut moddata:
                                                   krb5_clpreauth_moddata,
                                               mut modreq:
                                                   krb5_clpreauth_modreq,
                                               mut opt:
                                                   *mut krb5_get_init_creds_opt,
                                               mut cb:
                                                   krb5_clpreauth_callbacks,
                                               mut rock: krb5_clpreauth_rock,
                                               mut request: *mut krb5_kdc_req,
                                               mut encoded_request_body:
                                                   *mut krb5_data,
                                               mut encoded_previous_request:
                                                   *mut krb5_data,
                                               mut pa_data: *mut krb5_pa_data)
 -> krb5_error_code {
    let mut chl: *mut krb5_pa_otp_challenge = 0 as *mut krb5_pa_otp_challenge;
    let mut retval: krb5_error_code = 0;
    let mut tmp: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut json: *mut libc::c_char = 0 as *mut libc::c_char;
    if modreq.is_null() { return 12 as libc::c_int }
    /* Decode the challenge. */
    tmp =
        make_data((*pa_data).contents as *mut libc::c_void,
                  (*pa_data).length);
    retval =
        decode_krb5_pa_otp_challenge(&mut tmp,
                                     modreq as
                                         *mut *mut krb5_pa_otp_challenge);
    if retval != 0 as libc::c_int { return retval }
    chl = *(modreq as *mut *mut krb5_pa_otp_challenge);
    /* Remove unsupported tokeninfos. */
    retval = filter_supported_tokeninfos(context, (*chl).tokeninfo);
    if retval != 0 as libc::c_int { return retval }
    /* Remove tokeninfos that don't match the recorded description, if that
     * results in there being only one that does. */
    retval = filter_config_tokeninfos(context, cb, rock, (*chl).tokeninfo);
    if retval != 0 as libc::c_int { return retval }
    /* Make the JSON representation. */
    retval = codec_encode_challenge(context, chl, &mut json);
    if retval != 0 as libc::c_int { return retval }
    /* Ask the question. */
    retval =
        (*cb).ask_responder_question.expect("non-null function pointer")(context,
                                                                         rock,
                                                                         b"otp\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         json);
    free(json as *mut libc::c_void);
    return retval;
}
/*
 * Save the vendor, algID, and tokenID values for the selected token to the
 * out_ccache, so that later we can try to use them to select the right one
 * without having ot ask the user.
 */
#[c2rust::src_loc = "1038:1"]
unsafe extern "C" fn save_config_tokeninfo(mut context: krb5_context,
                                           mut cb: krb5_clpreauth_callbacks,
                                           mut rock: krb5_clpreauth_rock,
                                           mut ti: *mut krb5_otp_tokeninfo) {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ti).vendor.length > 0 as libc::c_int as libc::c_uint &&
           asprintf(&mut tmp as *mut *mut libc::c_char,
                    b"%.*s\x00" as *const u8 as *const libc::c_char,
                    (*ti).vendor.length, (*ti).vendor.data) >=
               0 as libc::c_int {
        (*cb).set_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"vendor\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                tmp);
        free(tmp as *mut libc::c_void);
    }
    if (*ti).alg_id.length > 0 as libc::c_int as libc::c_uint &&
           asprintf(&mut tmp as *mut *mut libc::c_char,
                    b"%.*s\x00" as *const u8 as *const libc::c_char,
                    (*ti).alg_id.length, (*ti).alg_id.data) >=
               0 as libc::c_int {
        (*cb).set_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"algID\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                tmp);
        free(tmp as *mut libc::c_void);
    }
    if (*ti).token_id.length > 0 as libc::c_int as libc::c_uint &&
           asprintf(&mut tmp as *mut *mut libc::c_char,
                    b"%.*s\x00" as *const u8 as *const libc::c_char,
                    (*ti).token_id.length, (*ti).token_id.data) >=
               0 as libc::c_int {
        (*cb).set_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"tokenID\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                tmp);
        free(tmp as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1062:1"]
unsafe extern "C" fn otp_client_process(mut context: krb5_context,
                                        mut moddata: krb5_clpreauth_moddata,
                                        mut modreq: krb5_clpreauth_modreq,
                                        mut opt: *mut krb5_get_init_creds_opt,
                                        mut cb: krb5_clpreauth_callbacks,
                                        mut rock: krb5_clpreauth_rock,
                                        mut request: *mut krb5_kdc_req,
                                        mut encoded_request_body:
                                            *mut krb5_data,
                                        mut encoded_previous_request:
                                            *mut krb5_data,
                                        mut pa_data: *mut krb5_pa_data,
                                        mut prompter: krb5_prompter_fct,
                                        mut prompter_data: *mut libc::c_void,
                                        mut pa_data_out:
                                            *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut chl: *mut krb5_pa_otp_challenge = 0 as *mut krb5_pa_otp_challenge;
    let mut ti: *mut krb5_otp_tokeninfo = 0 as *mut krb5_otp_tokeninfo;
    let mut as_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut req: *mut krb5_pa_otp_req = 0 as *mut krb5_pa_otp_req;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut value: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pin: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut answer: *const libc::c_char = 0 as *const libc::c_char;
    if modreq.is_null() { return 12 as libc::c_int }
    chl = *(modreq as *mut *mut krb5_pa_otp_challenge);
    *pa_data_out = 0 as *mut *mut krb5_pa_data;
    /* Get FAST armor key. */
    as_key =
        (*cb).fast_armor.expect("non-null function pointer")(context, rock);
    if as_key.is_null() { return 2 as libc::c_int }
    /* Attempt to get token selection from the responder. */
    pin = empty_data();
    value = empty_data();
    answer =
        (*cb).get_responder_answer.expect("non-null function pointer")(context,
                                                                       rock,
                                                                       b"otp\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char);
    retval =
        codec_decode_answer(context, answer, (*chl).tokeninfo, &mut ti,
                            &mut value, &mut pin);
    if retval != 0 as libc::c_int {
        /* If the responder doesn't have a token selection,
         * we need to select the token via prompting. */
        retval =
            prompt_for_token(context, prompter, prompter_data,
                             (*chl).tokeninfo, &mut ti, &mut value, &mut pin);
        if retval != 0 as libc::c_int {
            current_block = 11874539425982058029;
        } else { current_block = 12039483399334584727; }
    } else { current_block = 12039483399334584727; }
    match current_block {
        12039483399334584727 => {
            /* Make the request. */
            retval =
                make_request(context, ti, &mut value, &mut pin, &mut req);
            if !(retval != 0 as libc::c_int) {
                /* Save information about the token which was used. */
                save_config_tokeninfo(context, cb, rock, ti);
                /* Encrypt the challenge's nonce and set it in the request. */
                retval = encrypt_nonce(context, as_key, chl, req);
                if !(retval != 0 as libc::c_int) {
                    /* Use FAST armor key as response key. */
                    retval =
                        (*cb).set_as_key.expect("non-null function pointer")(context,
                                                                             rock,
                                                                             as_key);
                    if !(retval != 0 as libc::c_int) {
                        /* Encode the request into the pa_data output. */
                        retval = set_pa_data(req, pa_data_out);
                        if !(retval != 0 as libc::c_int) {
                            (*cb).disable_fallback.expect("non-null function pointer")(context,
                                                                                       rock);
                        }
                    }
                }
            }
        }
        _ => { }
    }
    krb5_free_data_contents(context, &mut value);
    krb5_free_data_contents(context, &mut pin);
    k5_free_pa_otp_req(context, req);
    return retval;
}
#[c2rust::src_loc = "1137:1"]
unsafe extern "C" fn otp_client_request_fini(mut context: krb5_context,
                                             mut moddata:
                                                 krb5_clpreauth_moddata,
                                             mut modreq:
                                                 krb5_clpreauth_modreq) {
    if modreq.is_null() { return }
    k5_free_pa_otp_challenge(context,
                             *(modreq as *mut *mut krb5_pa_otp_challenge));
    free(modreq as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1148:1"]
pub unsafe extern "C" fn clpreauth_otp_initvt(mut context: krb5_context,
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
        b"otp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*vt).pa_type_list = otp_client_supported_pa_types.as_mut_ptr();
    (*vt).request_init =
        Some(otp_client_request_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: *mut krb5_clpreauth_modreq) -> ());
    (*vt).prep_questions =
        Some(otp_client_prep_questions as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: krb5_clpreauth_modreq,
                                      _: *mut krb5_get_init_creds_opt,
                                      _: krb5_clpreauth_callbacks,
                                      _: krb5_clpreauth_rock,
                                      _: *mut krb5_kdc_req, _: *mut krb5_data,
                                      _: *mut krb5_data, _: *mut krb5_pa_data)
                     -> krb5_error_code);
    (*vt).process =
        Some(otp_client_process as
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
    (*vt).request_fini =
        Some(otp_client_request_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: krb5_clpreauth_modreq) -> ());
    (*vt).gic_opts = None;
    return 0 as libc::c_int;
}
/* *
 * Decode the KRB5_RESPONDER_QUESTION_OTP to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_OTP
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to interact with OTP tokens without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_otp_challenge_free() to
 * be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl             Challenge structure
 *
 * @version New in 1.11
 */
#[no_mangle]
#[c2rust::src_loc = "1169:1"]
pub unsafe extern "C" fn krb5_responder_otp_get_challenge(mut ctx:
                                                              krb5_context,
                                                          mut rctx:
                                                              krb5_responder_context,
                                                          mut chl:
                                                              *mut *mut krb5_responder_otp_challenge)
 -> krb5_error_code {
    let mut answer: *const libc::c_char = 0 as *const libc::c_char;
    let mut challenge: *mut krb5_responder_otp_challenge =
        0 as *mut krb5_responder_otp_challenge;
    answer =
        krb5_responder_get_challenge(ctx, rctx,
                                     b"otp\x00" as *const u8 as
                                         *const libc::c_char);
    if answer.is_null() {
        *chl = 0 as *mut krb5_responder_otp_challenge;
        return 0 as libc::c_int
    }
    challenge = codec_decode_challenge(ctx, answer);
    if challenge.is_null() { return 12 as libc::c_int }
    *chl = challenge;
    return 0 as libc::c_int;
}
/* *
 * Answer the KRB5_RESPONDER_QUESTION_OTP question.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] ti               The index of the tokeninfo selected
 * @param [in] value            The value to set, or NULL for none
 * @param [in] pin              The pin to set, or NULL for none
 *
 * @version New in 1.11
 */
#[no_mangle]
#[c2rust::src_loc = "1192:1"]
pub unsafe extern "C" fn krb5_responder_otp_set_answer(mut ctx: krb5_context,
                                                       mut rctx:
                                                           krb5_responder_context,
                                                       mut ti: size_t,
                                                       mut value:
                                                           *const libc::c_char,
                                                       mut pin:
                                                           *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut obj: k5_json_object = 0 as k5_json_object;
    let mut num: k5_json_number = 0 as *mut k5_json_number_st;
    let mut str: k5_json_string = 0 as *mut k5_json_string_st;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    retval = k5_json_object_create(&mut obj);
    if !(retval != 0 as libc::c_int) {
        retval = k5_json_number_create(ti as libc::c_longlong, &mut num);
        if !(retval != 0 as libc::c_int) {
            retval =
                k5_json_object_set(obj,
                                   b"tokeninfo\x00" as *const u8 as
                                       *const libc::c_char,
                                   num as k5_json_value);
            k5_json_release(num as k5_json_value);
            if !(retval != 0 as libc::c_int) {
                if !value.is_null() {
                    retval = k5_json_string_create(value, &mut str);
                    if retval != 0 as libc::c_int {
                        current_block = 3917637599963829856;
                    } else {
                        retval =
                            k5_json_object_set(obj,
                                               b"value\x00" as *const u8 as
                                                   *const libc::c_char,
                                               str as k5_json_value);
                        k5_json_release(str as k5_json_value);
                        if retval != 0 as libc::c_int {
                            current_block = 3917637599963829856;
                        } else { current_block = 1054647088692577877; }
                    }
                } else { current_block = 1054647088692577877; }
                match current_block {
                    3917637599963829856 => { }
                    _ => {
                        if !pin.is_null() {
                            retval = k5_json_string_create(pin, &mut str);
                            if retval != 0 as libc::c_int {
                                current_block = 3917637599963829856;
                            } else {
                                retval =
                                    k5_json_object_set(obj,
                                                       b"pin\x00" as *const u8
                                                           as
                                                           *const libc::c_char,
                                                       str as k5_json_value);
                                k5_json_release(str as k5_json_value);
                                if retval != 0 as libc::c_int {
                                    current_block = 3917637599963829856;
                                } else {
                                    current_block = 18317007320854588510;
                                }
                            }
                        } else { current_block = 18317007320854588510; }
                        match current_block {
                            3917637599963829856 => { }
                            _ => {
                                retval =
                                    k5_json_encode(obj as k5_json_value,
                                                   &mut tmp);
                                if !(retval != 0 as libc::c_int) {
                                    k5_json_release(obj as k5_json_value);
                                    retval =
                                        krb5_responder_set_answer(ctx, rctx,
                                                                  b"otp\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  tmp);
                                    free(tmp as *mut libc::c_void);
                                    return retval
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    k5_json_release(obj as k5_json_value);
    return retval;
}
/* *
 * Free the value returned by krb5_responder_otp_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.11
 */
#[no_mangle]
#[c2rust::src_loc = "1252:1"]
pub unsafe extern "C" fn krb5_responder_otp_challenge_free(mut ctx:
                                                               krb5_context,
                                                           mut rctx:
                                                               krb5_responder_context,
                                                           mut chl:
                                                               *mut krb5_responder_otp_challenge) {
    let mut i: size_t = 0;
    if chl.is_null() { return }
    i = 0 as libc::c_int as size_t;
    while !(*(*chl).tokeninfo.offset(i as isize)).is_null() {
        free_tokeninfo(*(*chl).tokeninfo.offset(i as isize));
        i = i.wrapping_add(1)
    }
    free((*chl).service as *mut libc::c_void);
    free((*chl).tokeninfo as *mut libc::c_void);
    free(chl as *mut libc::c_void);
}
