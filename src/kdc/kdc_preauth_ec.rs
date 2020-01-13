use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
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
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
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
    /* Originally used to recognize AFS and default salts.  No longer used. */
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
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
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
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /*
 * end "rcache.h"
 */
    /*
 * begin "keytab.h"
 */
    /* XXX */
    /* *< Long enough for MAXPATHLEN + some extra */
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
    /* * A key table entry. */
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    extern "C" {
        /* *< Principal of this key */
        /* *< Time entry written to keytable */
        /* *< Key version number */
        /* *< The secret key */
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
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
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
        #[no_mangle]
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "4854:1"]
        pub fn krb5_check_clockskew(context: krb5_context,
                                    date: krb5_timestamp) -> krb5_error_code;
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
    #[c2rust::src_loc = "746:1"]
    pub type krb5_pa_enc_ts = _krb5_pa_enc_ts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "746:16"]
    pub struct _krb5_pa_enc_ts {
        pub patimestamp: krb5_timestamp,
        pub pausec: krb5_int32,
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_timestamp,
                        krb5_data, krb5_enc_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::string_h::memcpy;
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
        #[c2rust::src_loc = "930:1"]
        pub fn krb5_free_pa_enc_ts(_: krb5_context, _: *mut krb5_pa_enc_ts);
        #[no_mangle]
        #[c2rust::src_loc = "1457:1"]
        pub fn encode_krb5_pa_enc_ts(_: *const krb5_pa_enc_ts,
                                     _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1663:1"]
        pub fn decode_krb5_enc_data(output: *const krb5_data,
                                    rep: *mut *mut krb5_enc_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1666:1"]
        pub fn decode_krb5_pa_enc_ts(output: *const krb5_data,
                                     rep: *mut *mut krb5_pa_enc_ts)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2161:1"]
        pub fn krb5_free_enc_data(_: krb5_context, _: *mut krb5_enc_data);
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn profile_get_string(profile: profile_t,
                                  name: *const libc::c_char,
                                  subname: *const libc::c_char,
                                  subsubname: *const libc::c_char,
                                  def_val: *const libc::c_char,
                                  ret_string: *mut *mut libc::c_char)
         -> libc::c_long;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kdcpreauth_plugin.h:33"]
pub mod kdcpreauth_plugin_h {
    #[c2rust::src_loc = "115:1"]
    pub type krb5_kdcpreauth_modreq = *mut krb5_kdcpreauth_modreq_st;
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
 * Declarations for kdcpreauth plugin module implementors.
 *
 * The kdcpreauth interface has a single supported major version, which is 1.
 * Major version 1 has a current minor version of 2.  kdcpreauth modules should
 * define a function named kdcpreauth_<modulename>_initvt, matching the
 * signature:
 *
 *   krb5_error_code
 *   kdcpreauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                             krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     kdcpreauth, maj_ver == 1: Cast to krb5_kdcpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* kdcpreauth mechanism property flags */
    /*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring hardware-based
 * preauthentication.
 */
    /*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring preauthentication,
 * and to fail preauthentication if we can't verify the client data.  The
 * flipside of PA_SUFFICIENT.
 */
    /*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring preauthentication,
 * and to mark preauthentication as successful if we can verify the client
 * data.  The flipside of PA_REQUIRED.
 */
    /*
 * Marks this preauthentication mechanism as one which changes the key which is
 * used for encrypting the response to the client.  Modules which have this
 * flag have their server_return_fn called before modules which do not, and are
 * passed over if a previously-called module has modified the encrypting key.
 */
    /*
 * Not really a padata type, so don't include it in any list of preauth types
 * which gets sent over the wire.
 */
    /*
 * Indicates that e_data in non-FAST errors should be encoded as typed data
 * instead of padata.
 */
    /* Abstract type for a KDC callback data handle. */
    #[c2rust::src_loc = "111:1"]
    pub type krb5_kdcpreauth_rock = *mut krb5_kdcpreauth_rock_st;
    /* Abstract type for module data and per-request module data. */
    #[c2rust::src_loc = "114:1"]
    pub type krb5_kdcpreauth_moddata = *mut krb5_kdcpreauth_moddata_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:16"]
    pub struct krb5_kdcpreauth_callbacks_st {
        pub vers: libc::c_int,
        pub max_time_skew: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_kdcpreauth_rock)
                                      -> krb5_deltat>,
        pub client_keys: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_kdcpreauth_rock,
                                                     _:
                                                         *mut *mut krb5_keyblock)
                                    -> krb5_error_code>,
        pub free_keys: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_kdcpreauth_rock,
                                                   _: *mut krb5_keyblock)
                                  -> ()>,
        pub request_body: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: krb5_kdcpreauth_rock)
                                     -> *mut krb5_data>,
        pub fast_armor: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock)
                                   -> *mut krb5_keyblock>,
        pub get_string: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock,
                                                    _: *const libc::c_char,
                                                    _: *mut *mut libc::c_char)
                                   -> krb5_error_code>,
        pub free_string: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_kdcpreauth_rock,
                                                     _: *mut libc::c_char)
                                    -> ()>,
        pub client_entry: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: krb5_kdcpreauth_rock)
                                     -> *mut libc::c_void>,
        pub event_context: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_kdcpreauth_rock)
                                      -> *mut verto_ctx>,
        pub have_client_keys: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_kdcpreauth_rock)
                                         -> krb5_boolean>,
        pub client_keyblock: Option<unsafe extern "C" fn(_: krb5_context,
                                                         _:
                                                             krb5_kdcpreauth_rock)
                                        -> *const krb5_keyblock>,
        pub add_auth_indicator: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                krb5_kdcpreauth_rock,
                                                            _:
                                                                *const libc::c_char)
                                           -> krb5_error_code>,
        pub get_cookie: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock,
                                                    _: krb5_preauthtype,
                                                    _: *mut krb5_data)
                                   -> krb5_boolean>,
        pub set_cookie: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock,
                                                    _: krb5_preauthtype,
                                                    _: *const krb5_data)
                                   -> krb5_error_code>,
        pub match_client: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: krb5_kdcpreauth_rock,
                                                      _: krb5_principal)
                                     -> krb5_boolean>,
        pub client_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_kdcpreauth_rock)
                                    -> krb5_principal>,
        pub send_freshness_token: Option<unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_kdcpreauth_rock)
                                             -> ()>,
        pub check_freshness_token: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_kdcpreauth_rock,
                                                               _:
                                                                   *const krb5_data)
                                              -> krb5_error_code>,
    }
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    #[c2rust::src_loc = "123:1"]
    pub type krb5_kdcpreauth_callbacks = *mut krb5_kdcpreauth_callbacks_st;
    /*
     * Get an array of krb5_keyblock structures containing the client keys
     * matching the request enctypes, terminated by an entry with key type = 0.
     * Returns ENOENT if no keys are available for the request enctypes.  Free
     * the resulting object with the free_keys callback.
     */
    /* Free the result of client_keys. */
    /*
     * Get the encoded request body, which is sometimes needed for checksums.
     * For a FAST request this is the encoded inner request body.  The returned
     * pointer is an alias and should not be freed.
     */
    /* Get a pointer to the FAST armor key, or NULL if the request did not use
     * FAST.  The returned pointer is an alias and should not be freed. */
    /* Retrieve a string attribute from the client DB entry, or NULL if no such
     * attribute is set.  Free the result with the free_string callback. */
    /* Free the result of get_string. */
    /* Get a pointer to the client DB entry (returned as a void pointer to
     * avoid a dependency on a libkdb5 type). */
    /* Get a pointer to the verto context which should be used by an
     * asynchronous edata or verify method. */
    /* End of version 1 kdcpreauth callbacks. */
    /* Return true if the client DB entry contains any keys matching the
     * request enctypes. */
    /* End of version 2 kdcpreauth callbacks. */
    /*
     * Get the decrypted client long-term key chosen according to the request
     * enctype list, or NULL if no matching key was found.  The returned
     * pointer is an alias and should not be freed.  If invoked from
     * return_padata, the result will be the same as the encrypting_key
     * parameter if it is not NULL, and will therefore reflect the modified
     * reply key if a return_padata handler has replaced the reply key.
     */
    /* Assert an authentication indicator in the AS-REP authdata.  Duplicate
     * indicators will be ignored. */
    /*
     * Read a data value for pa_type from the request cookie, placing it in
     * *out.  The value placed there is an alias and must not be freed.
     * Returns true if a value for pa_type was retrieved, false if not.
     */
    /*
     * Set a data value for pa_type to be sent in a secure cookie in the next
     * error response.  If pa_type is already present, the value is ignored.
     * If the preauth mechanism has different preauth types for requests and
     * responses, use the request type.  Secure cookies are encrypted in a key
     * known only to the KDCs, but can be replayed within a short time window
     * for requests using the same client principal.
     */
    /* End of version 3 kdcpreauth callbacks. */
    /*
     * Return true if princ matches the principal named in the request or the
     * client principal (possibly canonicalized).  If princ does not match,
     * attempt a database lookup of princ with aliases allowed and compare the
     * result to the client principal, returning true if it matches.
     * Otherwise, return false.
     */
    /*
     * Get an alias to the client DB entry principal (possibly canonicalized).
     */
    /* End of version 4 kdcpreauth callbacks. */
    /*
     * Instruct the KDC to send a freshness token in the method data
     * accompanying a PREAUTH_REQUIRED or PREAUTH_FAILED error, if the client
     * indicated support for freshness tokens.  This callback should only be
     * invoked from the edata method.
     */
    /* Validate a freshness token sent by the client.  Return 0 on success,
     * KRB5KDC_ERR_PREAUTH_EXPIRED on error. */
    /* End of version 5 kdcpreauth callbacks. */
    /* Optional: preauth plugin initialization function. */
    #[c2rust::src_loc = "263:1"]
    pub type krb5_kdcpreauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_kdcpreauth_moddata,
                                    _: *mut *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: preauth plugin cleanup function. */
    #[c2rust::src_loc = "269:1"]
    pub type krb5_kdcpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata) -> ()>;
    /*
 * Optional: return the flags which the KDC should use for this module.  This
 * is a callback instead of a static value because the module may or may not
 * wish to count itself as a hardware preauthentication module (in other words,
 * the flags may be affected by the configuration, for example if a site
 * administrator can force a particular preauthentication type to be supported
 * using only hardware).  This function is called for each entry entry in the
 * server_pa_type_list.
 */
    #[c2rust::src_loc = "282:1"]
    pub type krb5_kdcpreauth_flags_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                   -> libc::c_int>;
    /*
 * Responder for krb5_kdcpreauth_edata_fn.  If invoked with a non-zero code, pa
 * will be ignored and the padata type will not be included in the hint list.
 * If invoked with a zero code and a null pa value, the padata type will be
 * included in the list with an empty value.  If invoked with a zero code and a
 * non-null pa value, pa will be included in the hint list and will later be
 * freed by the KDC.
 */
    #[c2rust::src_loc = "293:1"]
    pub type krb5_kdcpreauth_edata_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: *mut krb5_pa_data) -> ()>;
    /*
 * Optional: provide pa_data to send to the client as part of the "you need to
 * use preauthentication" error.  The implementation must invoke the respond
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 *
 * This function is not allowed to create a modreq object because we have no
 * guarantee that the client will ever make a follow-up request, or that it
 * will hit this KDC if it does.
 */
    #[c2rust::src_loc = "307:1"]
    pub type krb5_kdcpreauth_edata_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_kdc_req,
                                    _: krb5_kdcpreauth_callbacks,
                                    _: krb5_kdcpreauth_rock,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_preauthtype,
                                    _: krb5_kdcpreauth_edata_respond_fn,
                                    _: *mut libc::c_void) -> ()>;
    /*
 * Responder for krb5_kdcpreauth_verify_fn.  Invoke with the arg parameter
 * supplied to verify, the error code (0 for success), an optional module
 * request state object to be consumed by return_fn or free_modreq_fn, optional
 * e_data to be passed to the caller if code is nonzero, and optional
 * authorization data to be included in the ticket.  In non-FAST replies,
 * e_data will be encoded as typed-data if the module sets the PA_TYPED_E_DATA
 * flag, and as pa-data otherwise.  e_data and authz_data will be freed by the
 * KDC.
 */
    #[c2rust::src_loc = "326:1"]
    pub type krb5_kdcpreauth_verify_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: krb5_kdcpreauth_modreq,
                                    _: *mut *mut krb5_pa_data,
                                    _: *mut *mut krb5_authdata) -> ()>;
    /*
 * Optional: verify preauthentication data sent by the client, setting the
 * TKT_FLG_PRE_AUTH or TKT_FLG_HW_AUTH flag in the enc_tkt_reply's "flags"
 * field as appropriate.  The implementation must invoke the respond function
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 */
    #[c2rust::src_loc = "339:1"]
    pub type krb5_kdcpreauth_verify_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_data,
                                    _: *mut krb5_kdc_req,
                                    _: *mut krb5_enc_tkt_part,
                                    _: *mut krb5_pa_data,
                                    _: krb5_kdcpreauth_callbacks,
                                    _: krb5_kdcpreauth_rock,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_verify_respond_fn,
                                    _: *mut libc::c_void) -> ()>;
    /*
 * Optional: generate preauthentication response data to send to the client as
 * part of the AS-REP.  If it needs to override the key which is used to
 * encrypt the response, it can do so.
 */
    #[c2rust::src_loc = "355:1"]
    pub type krb5_kdcpreauth_return_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_pa_data,
                                    _: *mut krb5_data, _: *mut krb5_kdc_req,
                                    _: *mut krb5_kdc_rep,
                                    _: *mut krb5_keyblock,
                                    _: *mut *mut krb5_pa_data,
                                    _: krb5_kdcpreauth_callbacks,
                                    _: krb5_kdcpreauth_rock,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_modreq)
                   -> krb5_error_code>;
    /* Optional: free a per-request context. */
    #[c2rust::src_loc = "369:1"]
    pub type krb5_kdcpreauth_free_modreq_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_modreq) -> ()>;
    /* Optional: invoked after init_fn to provide the module with a pointer to the
 * verto main loop. */
    #[c2rust::src_loc = "376:1"]
    pub type krb5_kdcpreauth_loop_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata,
                                    _: *mut verto_ctx) -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "381:16"]
    pub struct krb5_kdcpreauth_vtable_st {
        pub name: *mut libc::c_char,
        pub pa_type_list: *mut krb5_preauthtype,
        pub init: krb5_kdcpreauth_init_fn,
        pub fini: krb5_kdcpreauth_fini_fn,
        pub flags: krb5_kdcpreauth_flags_fn,
        pub edata: krb5_kdcpreauth_edata_fn,
        pub verify: krb5_kdcpreauth_verify_fn,
        pub return_padata: krb5_kdcpreauth_return_fn,
        pub free_modreq: krb5_kdcpreauth_free_modreq_fn,
        pub loop_0: krb5_kdcpreauth_loop_fn,
    }
    #[c2rust::src_loc = "381:1"]
    pub type krb5_kdcpreauth_vtable = *mut krb5_kdcpreauth_vtable_st;
    use super::kdc_util_h::krb5_kdcpreauth_rock_st;
    use super::krb5_h::{krb5_deltat, krb5_context, krb5_error_code,
                        krb5_keyblock, krb5_data, krb5_boolean,
                        krb5_preauthtype, krb5_principal, krb5_pa_data,
                        krb5_kdc_req, krb5_authdata, krb5_enc_tkt_part,
                        krb5_kdc_rep};
    extern "C" {
        #[c2rust::src_loc = "115:16"]
        pub type krb5_kdcpreauth_modreq_st;
        /* Mandatory: name of module. */
        /* Mandatory: pointer to zero-terminated list of pa_types which this module
     * can provide services for. */
        /* Minor 1 ends here. */
        /* Minor 2 ends here. */
        /* The verto context structure type (typedef is in verto.h; we want to avoid a
 * header dependency for the moment). */
        #[c2rust::src_loc = "119:8"]
        pub type verto_ctx;
        #[c2rust::src_loc = "114:16"]
        pub type krb5_kdcpreauth_moddata_st;
    }
    /* KRB5_KDCPREAUTH_PLUGIN_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:34"]
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
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
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
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
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
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_magic,
                        krb5_ui_4, krb5_flags, krb5_deltat, krb5_timestamp,
                        krb5_kvno, krb5_principal};
    /* NOT saved */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_util.h:34"]
pub mod kdc_util_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_util.h */
/*
 * Portions Copyright (C) 2007 Apple Inc.
 * Copyright 1990, 2007, 2014 by the Massachusetts Institute of Technology.
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
 *
 * Declarations for policy.c
 */
    /* authind.c */
    /* cammac.c */
    /* do_as_req.c */
    /* do_tgs_req.c */
    /* dispatch.c */
    /* kdc_preauth.c */
    /* kdc_preauth_ec.c */
    /* kdc_preauth_enctsc.c */
    /* kdc_authdata.c */
    /* replay.c */
    /* kdc_util.c */
    /* FAST*/
    /*
 * If *requestptr contains FX_FAST padata, compute the armor key, verify the
 * checksum over checksummed_data, decode the FAST request, and substitute
 * *requestptr with the inner request.  Set the armor_key, cookie, and
 * fast_options fields in state.  state->cookie will be set for a non-FAST
 * request if it contains FX_COOKIE padata.  If inner_body_out is non-NULL, set
 * *inner_body_out to a copy of the encoded inner body, or to NULL if the
 * request is not a FAST request.
 */
    /* Information handle for kdcpreauth callbacks.  All pointers are aliases. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "424:8"]
    pub struct krb5_kdcpreauth_rock_st {
        pub request: *mut krb5_kdc_req,
        pub inner_body: *mut krb5_data,
        pub client: *mut krb5_db_entry,
        pub local_tgt: *mut krb5_db_entry,
        pub client_key: *mut krb5_key_data,
        pub client_keyblock: *mut krb5_keyblock,
        pub rstate: *mut kdc_request_state,
        pub vctx: *mut verto_ctx,
        pub auth_indicators: *mut *mut *mut krb5_data,
        pub send_freshness_token: krb5_boolean,
    }
    use super::krb5_h::{krb5_kdc_req, krb5_data, krb5_keyblock, krb5_boolean};
    use super::kdb_h::{krb5_db_entry, krb5_key_data};
    use super::reqstate_h::kdc_request_state;
    use super::kdcpreauth_plugin_h::verto_ctx;
    /* __KRB5_KDC_UTIL__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/reqstate.h:34"]
pub mod reqstate_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct kdc_request_state {
        pub armor_key: *mut krb5_keyblock,
        pub strengthen_key: *mut krb5_keyblock,
        pub in_cookie_padata: *mut *mut krb5_pa_data,
        pub out_cookie_padata: *mut *mut krb5_pa_data,
        pub fast_options: krb5_int32,
        pub fast_internal_flags: krb5_int32,
        pub realm_data: *mut kdc_realm_t,
    }
    use super::krb5_h::{krb5_keyblock, krb5_pa_data, krb5_int32};
    use super::realm_data_h::kdc_realm_t;
    /* REQSTATE_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/realm_data.h:34"]
pub mod realm_data_h {
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
    use super::krb5_h::{krb5_context, krb5_keytab, krb5_principal,
                        krb5_keyblock, krb5_deltat, krb5_boolean};
    /* REALM_DATA_H */
    /*
 * These macros used to refer to a global pointer to the active realm state
 * structure for a request.  They now refer to a local variable that must be
 * properly declared in each function that uses these macros.
 */
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
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
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:32"]
pub mod k5_int_pkinit_h {
    use super::krb5_h::{krb5_enc_data, krb5_data, krb5_error_code,
                        krb5_context, krb5_keyblock, krb5_keyusage};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn encode_krb5_enc_data(_: *const krb5_enc_data,
                                    _: *mut *mut krb5_data)
         -> krb5_error_code;
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
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_keyusage, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       krb5_c_decrypt, krb5_c_fx_cf2_simple,
                       krb5_free_keyblock, krb5_free_data,
                       krb5_free_data_contents, krb5_us_timeofday,
                       krb5_check_clockskew, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         krb5_pa_enc_ts, _krb5_pa_enc_ts, k5memdup0, k5alloc,
                         k5calloc, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_free_pa_enc_ts, encode_krb5_pa_enc_ts,
                         decode_krb5_enc_data, decode_krb5_pa_enc_ts,
                         krb5_free_enc_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_get_string};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::kdcpreauth_plugin_h::{krb5_kdcpreauth_modreq,
                                    krb5_kdcpreauth_rock,
                                    krb5_kdcpreauth_moddata,
                                    krb5_kdcpreauth_callbacks_st,
                                    krb5_kdcpreauth_callbacks,
                                    krb5_kdcpreauth_init_fn,
                                    krb5_kdcpreauth_fini_fn,
                                    krb5_kdcpreauth_flags_fn,
                                    krb5_kdcpreauth_edata_respond_fn,
                                    krb5_kdcpreauth_edata_fn,
                                    krb5_kdcpreauth_verify_respond_fn,
                                    krb5_kdcpreauth_verify_fn,
                                    krb5_kdcpreauth_return_fn,
                                    krb5_kdcpreauth_free_modreq_fn,
                                    krb5_kdcpreauth_loop_fn,
                                    krb5_kdcpreauth_vtable_st,
                                    krb5_kdcpreauth_vtable,
                                    krb5_kdcpreauth_modreq_st, verto_ctx,
                                    krb5_kdcpreauth_moddata_st};
pub use self::kdb_h::{_krb5_key_data, krb5_key_data, krb5_db_entry,
                      _krb5_db_entry_new, krb5_tl_data, _krb5_tl_data};
pub use self::kdc_util_h::krb5_kdcpreauth_rock_st;
pub use self::reqstate_h::kdc_request_state;
pub use self::realm_data_h::{kdc_realm_t, __kdc_realm_data};
use self::stdlib_h::{malloc, calloc, free};
use self::libintl_h::dgettext;
use self::string_h::memcpy;
use self::k5_int_pkinit_h::{encode_krb5_enc_data, krb5_encrypt_helper};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_preauth_ec.c - Encrypted challenge kdcpreauth module */
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
 * Implement Encrypted Challenge fast factor from
 * draft-ietf-krb-wg-preauth-framework
 */
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn ec_edata(mut context: krb5_context,
                              mut request: *mut krb5_kdc_req,
                              mut cb: krb5_kdcpreauth_callbacks,
                              mut rock: krb5_kdcpreauth_rock,
                              mut moddata: krb5_kdcpreauth_moddata,
                              mut pa_type: krb5_preauthtype,
                              mut respond: krb5_kdcpreauth_edata_respond_fn,
                              mut arg: *mut libc::c_void) {
    let mut armor_key: *mut krb5_keyblock =
        (*cb).fast_armor.expect("non-null function pointer")(context, rock);
    /* Encrypted challenge only works with FAST, and requires a client key. */
    if armor_key.is_null() ||
           (*cb).have_client_keys.expect("non-null function pointer")(context,
                                                                      rock) ==
               0 {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              2
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              0
                                                                                                  as
                                                                                                  *mut krb5_pa_data);
    } else {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              0
                                                                                                  as
                                                                                                  *mut krb5_pa_data);
    };
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn ec_verify(mut context: krb5_context,
                               mut req_pkt: *mut krb5_data,
                               mut request: *mut krb5_kdc_req,
                               mut enc_tkt_reply: *mut krb5_enc_tkt_part,
                               mut data: *mut krb5_pa_data,
                               mut cb: krb5_kdcpreauth_callbacks,
                               mut rock: krb5_kdcpreauth_rock,
                               mut moddata: krb5_kdcpreauth_moddata,
                               mut respond: krb5_kdcpreauth_verify_respond_fn,
                               mut arg: *mut libc::c_void) {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut enc: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut plain: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut armor_key: *mut krb5_keyblock =
        (*cb).fast_armor.expect("non-null function pointer")(context, rock);
    let mut ts: *mut krb5_pa_enc_ts = 0 as *mut krb5_pa_enc_ts;
    let mut client_keys: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut challenge_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut kdc_challenge_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut modreq: krb5_kdcpreauth_modreq = 0 as krb5_kdcpreauth_modreq;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ai: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut realmstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut realm: krb5_data = (*(*request).server).realm;
    plain.data = 0 as *mut libc::c_char;
    if armor_key.is_null() {
        retval = 2 as libc::c_int;
        krb5_set_error_message(context, 2 as libc::c_int,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Encrypted Challenge used outside of FAST tunnel\x00"
                                            as *const u8 as
                                            *const libc::c_char));
    }
    scratch.data = (*data).contents as *mut libc::c_char;
    scratch.length = (*data).length;
    if retval == 0 as libc::c_int {
        retval = decode_krb5_enc_data(&mut scratch, &mut enc)
    }
    if retval == 0 as libc::c_int {
        plain.data =
            malloc((*enc).ciphertext.length as libc::c_ulong) as
                *mut libc::c_char;
        plain.length = (*enc).ciphertext.length;
        if plain.data.is_null() { retval = 12 as libc::c_int }
    }
    /* Check for a configured FAST ec auth indicator. */
    realmstr =
        k5memdup0(realm.data as *const libc::c_void, realm.length as size_t,
                  &mut retval) as *mut libc::c_char;
    if !realmstr.is_null() {
        retval =
            profile_get_string((*context).profile,
                               b"realms\x00" as *const u8 as
                                   *const libc::c_char, realmstr,
                               b"encrypted_challenge_indicator\x00" as
                                   *const u8 as *const libc::c_char,
                               0 as *const libc::c_char, &mut ai) as
                krb5_error_code
    }
    if retval == 0 as libc::c_int {
        retval =
            (*cb).client_keys.expect("non-null function pointer")(context,
                                                                  rock,
                                                                  &mut client_keys)
    }
    if retval == 0 as libc::c_int {
        i = 0 as libc::c_int;
        while (*client_keys.offset(i as isize)).enctype != 0 &&
                  retval == 0 as libc::c_int {
            retval =
                krb5_c_fx_cf2_simple(context, armor_key,
                                     b"clientchallengearmor\x00" as *const u8
                                         as *const libc::c_char,
                                     &mut *client_keys.offset(i as isize),
                                     b"challengelongterm\x00" as *const u8 as
                                         *const libc::c_char,
                                     &mut challenge_key);
            if retval == 0 as libc::c_int {
                retval =
                    krb5_c_decrypt(context, challenge_key, 54 as libc::c_int,
                                   0 as *const krb5_data, enc, &mut plain)
            }
            if !challenge_key.is_null() {
                krb5_free_keyblock(context, challenge_key);
            }
            challenge_key = 0 as *mut krb5_keyblock;
            if retval == 0 as libc::c_int { break ; }
            /*We failed to decrypt. Try next key*/
            retval = 0 as libc::c_int;
            i += 1
        }
        if (*client_keys.offset(i as isize)).enctype == 0 as libc::c_int {
            retval = -(1765328360 as libc::c_long) as krb5_error_code;
            krb5_set_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Incorrect password in encrypted challenge\x00"
                                                as *const u8 as
                                                *const libc::c_char));
        }
    }
    if retval == 0 as libc::c_int {
        retval = decode_krb5_pa_enc_ts(&mut plain, &mut ts)
    }
    if retval == 0 as libc::c_int {
        retval = krb5_check_clockskew(context, (*ts).patimestamp)
    }
    if retval == 0 as libc::c_int {
        (*enc_tkt_reply).flags |= 0x200000 as libc::c_int;
        /*
         * If this fails, we won't generate a reply to the client.  That may
         * cause the client to fail, but at this point the KDC has considered
         * this a success, so the return value is ignored.
         */
        if krb5_c_fx_cf2_simple(context, armor_key,
                                b"kdcchallengearmor\x00" as *const u8 as
                                    *const libc::c_char,
                                &mut *client_keys.offset(i as isize),
                                b"challengelongterm\x00" as *const u8 as
                                    *const libc::c_char,
                                &mut kdc_challenge_key) == 0 as libc::c_int {
            modreq =
                kdc_challenge_key as
                    krb5_kdcpreauth_modreq; /* In case of error pass through */
            if !ai.is_null() {
                (*cb).add_auth_indicator.expect("non-null function pointer")(context,
                                                                             rock,
                                                                             ai);
            }
        }
    }
    (*cb).free_keys.expect("non-null function pointer")(context, rock,
                                                        client_keys);
    if !plain.data.is_null() { free(plain.data as *mut libc::c_void); }
    if !enc.is_null() { krb5_free_enc_data(context, enc); }
    if !ts.is_null() { krb5_free_pa_enc_ts(context, ts); }
    free(realmstr as *mut libc::c_void);
    free(ai as *mut libc::c_void);
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          retval,
                                                                                          modreq,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_pa_data,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_authdata);
}
#[c2rust::src_loc = "155:1"]
unsafe extern "C" fn ec_return(mut context: krb5_context,
                               mut padata: *mut krb5_pa_data,
                               mut req_pkt: *mut krb5_data,
                               mut request: *mut krb5_kdc_req,
                               mut reply: *mut krb5_kdc_rep,
                               mut encrypting_key: *mut krb5_keyblock,
                               mut send_pa: *mut *mut krb5_pa_data,
                               mut cb: krb5_kdcpreauth_callbacks,
                               mut rock: krb5_kdcpreauth_rock,
                               mut moddata: krb5_kdcpreauth_moddata,
                               mut modreq: krb5_kdcpreauth_modreq)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut challenge_key: *mut krb5_keyblock = modreq as *mut krb5_keyblock;
    let mut ts: krb5_pa_enc_ts = krb5_pa_enc_ts{patimestamp: 0, pausec: 0,};
    let mut plain: *mut krb5_data = 0 as *mut krb5_data;
    let mut enc: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut encoded: *mut krb5_data = 0 as *mut krb5_data;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    if challenge_key.is_null() { return 0 as libc::c_int }
    enc.ciphertext.data = 0 as *mut libc::c_char;
    retval = krb5_us_timeofday(context, &mut ts.patimestamp, &mut ts.pausec);
    if retval == 0 as libc::c_int {
        retval = encode_krb5_pa_enc_ts(&mut ts, &mut plain)
    }
    if retval == 0 as libc::c_int {
        retval =
            krb5_encrypt_helper(context, challenge_key, 55 as libc::c_int,
                                plain, &mut enc)
    }
    if retval == 0 as libc::c_int {
        retval = encode_krb5_enc_data(&mut enc, &mut encoded)
    }
    if retval == 0 as libc::c_int {
        pa =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong) as
                *mut krb5_pa_data;
        if pa.is_null() { retval = 12 as libc::c_int }
    }
    if retval == 0 as libc::c_int {
        (*pa).pa_type = 138 as libc::c_int;
        (*pa).contents = (*encoded).data as *mut libc::c_uchar;
        (*pa).length = (*encoded).length;
        (*encoded).data = 0 as *mut libc::c_char;
        *send_pa = pa;
        pa = 0 as *mut krb5_pa_data
    }
    if !challenge_key.is_null() {
        krb5_free_keyblock(context, challenge_key);
    }
    if !encoded.is_null() { krb5_free_data(context, encoded); }
    if !plain.is_null() { krb5_free_data(context, plain); }
    if !enc.ciphertext.data.is_null() {
        krb5_free_data_contents(context, &mut enc.ciphertext);
    }
    return retval;
}
#[c2rust::src_loc = "207:25"]
static mut ec_types: [krb5_preauthtype; 2] =
    [138 as libc::c_int, 0 as libc::c_int];
#[no_mangle]
#[c2rust::src_loc = "210:1"]
pub unsafe extern "C" fn kdcpreauth_encrypted_challenge_initvt(mut context:
                                                                   krb5_context,
                                                               mut maj_ver:
                                                                   libc::c_int,
                                                               mut min_ver:
                                                                   libc::c_int,
                                                               mut vtable:
                                                                   krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_kdcpreauth_vtable = 0 as *mut krb5_kdcpreauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_kdcpreauth_vtable;
    (*vt).name =
        b"encrypted_challenge\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*vt).pa_type_list = ec_types.as_mut_ptr();
    (*vt).edata =
        Some(ec_edata as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_kdc_req,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_preauthtype,
                                      _: krb5_kdcpreauth_edata_respond_fn,
                                      _: *mut libc::c_void) -> ());
    (*vt).verify =
        Some(ec_verify as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_data,
                                      _: *mut krb5_kdc_req,
                                      _: *mut krb5_enc_tkt_part,
                                      _: *mut krb5_pa_data,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_kdcpreauth_verify_respond_fn,
                                      _: *mut libc::c_void) -> ());
    (*vt).return_padata =
        Some(ec_return as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_pa_data,
                                      _: *mut krb5_data, _: *mut krb5_kdc_req,
                                      _: *mut krb5_kdc_rep,
                                      _: *mut krb5_keyblock,
                                      _: *mut *mut krb5_pa_data,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_kdcpreauth_modreq)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
