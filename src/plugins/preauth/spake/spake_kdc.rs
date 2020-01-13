use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "561:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "608:12"]
    pub struct C2RustUnnamed_1 {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_2 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn store_16_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_16(val as __uint16_t);
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "601:1"]
    pub unsafe extern "C" fn load_16_be(mut cvp: *const libc::c_void)
     -> libc::c_ushort {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_16((*(p as *const C2RustUnnamed_1)).i);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_2)).i);
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::byteswap_h::{__bswap_16, __bswap_32};
    use super::types_h::__uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
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
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* cleartext part: */
        /* *< KRB5_AS_REP or KRB5_KDC_REP */
        /* *< Preauthentication data from KDC */
        /* *< Client principal and realm */
        /* *< Ticket */
        /* *< Encrypted part of reply */
        /* *< Unencrypted version, if available */
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
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:33"]
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
    #[inline]
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        let mut ptr: *mut libc::c_char =
            calloc(if len > 0 as libc::c_int as libc::c_uint {
                       len
                   } else { 1 as libc::c_int as libc::c_uint } as
                       libc::c_ulong, 1 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if ptr.is_null() { return 12 as libc::c_int }
        (*data).magic = -(1760647422 as libc::c_long) as krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_error_code, krb5_data, krb5_context};
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
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:33"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:33"]
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
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn k5_buf_add_uint16_be(mut buf: *mut k5buf,
                                                  mut val: uint16_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 2 as libc::c_int as size_t);
        if !p.is_null() { store_16_be(val as libc::c_uint, p); };
    }
    #[inline]
    #[c2rust::src_loc = "129:1"]
    pub unsafe extern "C" fn k5_buf_add_uint32_be(mut buf: *mut k5buf,
                                                  mut val: uint32_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 4 as libc::c_int as size_t);
        if !p.is_null() { store_32_be(val, p); };
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::k5_platform_h::{store_16_be, store_32_be};
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer, zeroing
 * memory when reallocating or freeing. */
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn k5_buf_init_dynamic_zap(buf: *mut k5buf);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn k5_buf_get_space(buf: *mut k5buf, len: size_t)
         -> *mut libc::c_void;
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-input.h:34"]
pub mod k5_input_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-input.h - k5input helper functions */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
    /*
 * The k5input module defines helpers for safely consuming a fixed-sized block
 * of memory.  If an overrun or allocation failure occurs at any step,
 * subsequent functions will return default values until the error is detected
 * by looking at the status field.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct k5input {
        pub ptr: *const libc::c_uchar,
        pub len: size_t,
        pub status: int32_t,
    }
    #[inline]
    #[c2rust::src_loc = "51:1"]
    pub unsafe extern "C" fn k5_input_init(mut in_0: *mut k5input,
                                           mut ptr: *const libc::c_void,
                                           mut len: size_t) {
        (*in_0).ptr = ptr as *const libc::c_uchar;
        (*in_0).len = len;
        (*in_0).status = 0 as libc::c_int;
    }
    /* Only set the status value of in if it hasn't already been set, so status
 * reflects the first thing to go wrong. */
    #[inline]
    #[c2rust::src_loc = "61:1"]
    pub unsafe extern "C" fn k5_input_set_status(mut in_0: *mut k5input,
                                                 mut status: int32_t) {
        if (*in_0).status == 0 { (*in_0).status = status };
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn k5_input_get_bytes(mut in_0: *mut k5input,
                                                mut len: size_t)
     -> *const libc::c_uchar {
        if (*in_0).len < len { k5_input_set_status(in_0, 22 as libc::c_int); }
        if (*in_0).status != 0 { return 0 as *const libc::c_uchar }
        (*in_0).len =
            ((*in_0).len as libc::c_ulong).wrapping_sub(len) as size_t as
                size_t;
        (*in_0).ptr = (*in_0).ptr.offset(len as isize);
        return (*in_0).ptr.offset(-(len as isize));
    }
    #[inline]
    #[c2rust::src_loc = "87:1"]
    pub unsafe extern "C" fn k5_input_get_uint16_be(mut in_0: *mut k5input)
     -> uint16_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 2 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int
               } else {
                   load_16_be(ptr as *const libc::c_void) as libc::c_int
               } as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "108:1"]
    pub unsafe extern "C" fn k5_input_get_uint32_be(mut in_0: *mut k5input)
     -> uint32_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 4 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int as libc::c_uint
               } else { load_32_be(ptr as *const libc::c_void) };
    }
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::k5_platform_h::{load_16_be, load_32_be};
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-spake.h:35"]
pub mod k5_spake_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct krb5_spake_factor_st {
        pub type_0: int32_t,
        pub data: *mut krb5_data,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-spake.h - SPAKE preauth mech declarations */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
    /*
 * The SPAKE preauth mechanism allows long-term client keys to be used for
 * preauthentication without exposing them to offline dictionary attacks.  The
 * negotiated key can also be used for second-factor authentication.  This
 * header file declares structures and encoder/decoder functions for the
 * mechanism's padata messages.
 */
    /* SPAKESecondFactor is contained within a SPAKEChallenge, SPAKEResponse, or
 * EncryptedData message and contains a second-factor challenge or response. */
    #[c2rust::src_loc = "48:1"]
    pub type krb5_spake_factor = krb5_spake_factor_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct krb5_spake_support_st {
        pub ngroups: int32_t,
        pub groups: *mut int32_t,
    }
    /* SPAKESupport is sent from the client to the KDC to indicate which group the
 * client supports. */
    #[c2rust::src_loc = "55:1"]
    pub type krb5_spake_support = krb5_spake_support_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct krb5_spake_challenge_st {
        pub group: int32_t,
        pub pubkey: krb5_data,
        pub factors: *mut *mut krb5_spake_factor,
    }
    /* SPAKEChallenge is sent from the KDC to the client to communicate its group
 * selection, public value, and second-factor challenge options. */
    #[c2rust::src_loc = "62:1"]
    pub type krb5_spake_challenge = krb5_spake_challenge_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct krb5_spake_response_st {
        pub pubkey: krb5_data,
        pub factor: krb5_enc_data,
    }
    /* SPAKEResponse is sent from the client to the KDC to communicate its public
 * value and encrypted second-factor response. */
    #[c2rust::src_loc = "70:1"]
    pub type krb5_spake_response = krb5_spake_response_st;
    #[c2rust::src_loc = "75:1"]
    pub type krb5_spake_msgtype = libc::c_int;
    #[c2rust::src_loc = "80:5"]
    pub const SPAKE_MSGTYPE_ENCDATA: krb5_spake_msgtype = 3;
    #[c2rust::src_loc = "79:5"]
    pub const SPAKE_MSGTYPE_RESPONSE: krb5_spake_msgtype = 2;
    #[c2rust::src_loc = "78:5"]
    pub const SPAKE_MSGTYPE_CHALLENGE: krb5_spake_msgtype = 1;
    #[c2rust::src_loc = "77:5"]
    pub const SPAKE_MSGTYPE_SUPPORT: krb5_spake_msgtype = 0;
    #[c2rust::src_loc = "76:5"]
    pub const SPAKE_MSGTYPE_UNKNOWN: krb5_spake_msgtype = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:16"]
    pub struct krb5_pa_spake_st {
        pub choice: krb5_spake_msgtype,
        pub u: krb5_spake_message_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:11"]
    pub union krb5_spake_message_choices {
        pub support: krb5_spake_support,
        pub challenge: krb5_spake_challenge,
        pub response: krb5_spake_response,
        pub encdata: krb5_enc_data,
    }
    /* PA-SPAKE is a choice among the message types which can appear in a PA-SPAKE
 * padata element. */
    #[c2rust::src_loc = "85:1"]
    pub type krb5_pa_spake = krb5_pa_spake_st;
    use super::stdint_intn_h::int32_t;
    use super::krb5_h::{krb5_data, krb5_enc_data, krb5_error_code,
                        krb5_context};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn decode_krb5_spake_factor(code: *const krb5_data,
                                        val_out: *mut *mut krb5_spake_factor)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn k5_free_spake_factor(context: krb5_context,
                                    val: *mut krb5_spake_factor);
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn encode_krb5_pa_spake(val: *const krb5_pa_spake,
                                    code_out: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "103:1"]
        pub fn decode_krb5_pa_spake(code: *const krb5_data,
                                    val_out: *mut *mut krb5_pa_spake)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "105:1"]
        pub fn k5_free_pa_spake(context: krb5_context,
                                val: *mut krb5_pa_spake);
    }
    /* K5_SPAKE_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/spake/iana.h:37"]
pub mod iana_h {
    #[c2rust::src_loc = "39:9"]
    pub type C2RustUnnamed_3 = libc::c_uint;
    #[c2rust::src_loc = "40:5"]
    pub const SPAKE_SF_NONE: C2RustUnnamed_3 = 1;
    /* IANA_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/spake/groups.h:37"]
pub mod groups_h {
    #[c2rust::src_loc = "39:1"]
    pub type groupstate = groupstate_st;
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_boolean, krb5_error_code,
                        krb5_int32, krb5_data};
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[c2rust::src_loc = "39:16"]
        pub type groupstate_st;
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn group_init_state(context: krb5_context, is_kdc: krb5_boolean,
                                out: *mut *mut groupstate) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn group_free_state(gstate: *mut groupstate);
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn group_is_permitted(gstate: *mut groupstate, group: int32_t)
         -> krb5_boolean;
        /* Return the KDC optimistic challenge group if one is configured.  Valid for
 * KDC groupstate objects only. */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn group_optimistic_challenge(gstate: *mut groupstate)
         -> krb5_int32;
        /*
 * Generate a SPAKE private scalar (x or y) and public element (T or S), given
 * an input multiplier wbytes.  Use constant M if gstate is a KDC groupstate
 * object, N if it is a client object.  Allocate storage and place the results
 * in *priv_out and *pub_out.
 */
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn group_keygen(context: krb5_context, gstate: *mut groupstate,
                            group: int32_t, wbytes: *const krb5_data,
                            priv_out: *mut krb5_data, pub_out: *mut krb5_data)
         -> krb5_error_code;
        /*
 * Compute the SPAKE result K from our private scalar (x or y) and their public
 * key (S or T), deriving the input scalar w from ikey.  Use the other party's
 * constant, N if gstate is a KDC groupstate object or M if it is a client
 * object.  Allocate storage and place the result in *spakeresult_out.
 */
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn group_result(context: krb5_context, gstate: *mut groupstate,
                            group: int32_t, wbytes: *const krb5_data,
                            ourpriv: *const krb5_data,
                            theirpub: *const krb5_data,
                            spakeresult_out: *mut krb5_data)
         -> krb5_error_code;
    }
    /* GROUPS_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kdcpreauth_plugin.h:42"]
pub mod kdcpreauth_plugin_h {
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
    #[c2rust::src_loc = "115:1"]
    pub type krb5_kdcpreauth_modreq = *mut krb5_kdcpreauth_modreq_st;
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
    use super::krb5_h::{krb5_deltat, krb5_context, krb5_error_code,
                        krb5_keyblock, krb5_data, krb5_boolean,
                        krb5_preauthtype, krb5_principal, krb5_pa_data,
                        krb5_kdc_req, krb5_authdata, krb5_enc_tkt_part,
                        krb5_kdc_rep};
    extern "C" {
        #[c2rust::src_loc = "111:16"]
        pub type krb5_kdcpreauth_rock_st;
        #[c2rust::src_loc = "114:16"]
        pub type krb5_kdcpreauth_moddata_st;
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
    }
    /* KRB5_KDCPREAUTH_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
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
#[c2rust::header_src = "/usr/include/libintl.h:33"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:33"]
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:33"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::{__uint32_t, __uint16_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:33"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/spake/util.h:40"]
pub mod util_h {
    use super::krb5_h::{krb5_data, krb5_pa_data, krb5_error_code,
                        krb5_context, krb5_keyblock};
    use super::k5_int_h::_krb5_context;
    use super::groups_h::groupstate;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/spake/internal.h - SPAKE internal function declarations */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn convert_to_padata(data: *mut krb5_data,
                                 pa_out: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn update_thash(context: krb5_context, gstate: *mut groupstate,
                            group: int32_t, thash: *mut krb5_data,
                            data1: *const krb5_data, data2: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn derive_wbytes(context: krb5_context, group: int32_t,
                             ikey: *const krb5_keyblock,
                             wbytes_out: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn derive_key(context: krb5_context, gstate: *mut groupstate,
                          group: int32_t, ikey: *const krb5_keyblock,
                          wbytes: *const krb5_data,
                          spakeresult: *const krb5_data,
                          thash: *const krb5_data, der_req: *const krb5_data,
                          n: uint32_t, out: *mut *mut krb5_keyblock)
         -> krb5_error_code;
    }
    /* UTIL_H */
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
                              C2RustUnnamed_2, store_16_be, store_32_be,
                              load_16_be, load_32_be};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_msgtype,
                       krb5_kvno, krb5_addrtype, krb5_enctype,
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
                       krb5_ticket, _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _profile_t, krb5_c_decrypt,
                       krb5_copy_keyblock_contents, krb5_free_keyblock,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_free_data_contents, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5memdup0, k5alloc, k5calloc,
                         alloc_data, empty_data, zapfree, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_copy_data_contents};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf,
                         k5_buf_add_uint16_be, k5_buf_add_uint32_be,
                         k5_buf_init_dynamic_zap, k5_buf_add_len,
                         k5_buf_get_space};
pub use self::k5_input_h::{k5input, k5_input_init, k5_input_set_status,
                           k5_input_get_bytes, k5_input_get_uint16_be,
                           k5_input_get_uint32_be};
pub use self::k5_spake_h::{krb5_spake_factor_st, krb5_spake_factor,
                           krb5_spake_support_st, krb5_spake_support,
                           krb5_spake_challenge_st, krb5_spake_challenge,
                           krb5_spake_response_st, krb5_spake_response,
                           krb5_spake_msgtype, SPAKE_MSGTYPE_ENCDATA,
                           SPAKE_MSGTYPE_RESPONSE, SPAKE_MSGTYPE_CHALLENGE,
                           SPAKE_MSGTYPE_SUPPORT, SPAKE_MSGTYPE_UNKNOWN,
                           krb5_pa_spake_st, krb5_spake_message_choices,
                           krb5_pa_spake, decode_krb5_spake_factor,
                           k5_free_spake_factor, encode_krb5_pa_spake,
                           decode_krb5_pa_spake, k5_free_pa_spake};
pub use self::iana_h::{C2RustUnnamed_3, SPAKE_SF_NONE};
pub use self::groups_h::{groupstate, groupstate_st, group_init_state,
                         group_free_state, group_is_permitted,
                         group_optimistic_challenge, group_keygen,
                         group_result};
pub use self::kdcpreauth_plugin_h::{krb5_kdcpreauth_rock,
                                    krb5_kdcpreauth_moddata,
                                    krb5_kdcpreauth_modreq,
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
                                    krb5_kdcpreauth_rock_st,
                                    krb5_kdcpreauth_moddata_st,
                                    krb5_kdcpreauth_modreq_st, verto_ctx};
use self::stdlib_h::{calloc, free};
use self::libintl_h::dgettext;
use self::string_h::{explicit_bzero, memcpy};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::{__bswap_32, __bswap_16};
use self::k5_trace_h::krb5int_trace;
use self::util_h::{convert_to_padata, update_thash, derive_wbytes,
                   derive_key};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/spake/spake_kdc.c - SPAKE kdcpreauth module */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
/*
 * The SPAKE kdcpreauth module uses a secure cookie containing the following
 * concatenated fields (all integer fields are big-endian):
 *
 *     version (16-bit unsigned integer)
 *     stage (16-bit unsigned integer)
 *     group (32-bit signed integer)
 *     SPAKE value (32-bit unsigned length, followed by data)
 *     Transcript hash (32-bit unsigned length, followed by data)
 *     Zero or more instances of:
 *         second-factor number (32-bit signed integer)
 *         second-factor data (32-bit unsigned length, followed by data)
 *
 * The only currently supported version is 1.  stage is 0 if the cookie was
 * sent with a challenge message.  stage is n>0 if the cookie was sent with an
 * encdata message encrypted in K'[2n].  group indicates the group number used
 * in the SPAKE challenge.  The SPAKE value is the KDC private key for a
 * stage-0 cookie, represented in the scalar marshalling form of the group; for
 * other cookies, the SPAKE value is the SPAKE result K, represented in the
 * group element marshalling form.  The transcript hash is the intermediate
 * hash after updating with the support and challenge messages for a stage-0
 * cookie, or the final hash for other cookies.  For a stage 0 cookie, there
 * may be any number of second-factor records, including none (no record is
 * generated for SF-NONE); for other cookies, there must be exactly one
 * second-factor record corresponding to the factor type chosen by the client.
 */
/* From a k5input structure representing the remainder of a secure cookie
 * plaintext, parse a four-byte length and data. */
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn parse_data(mut in_0: *mut k5input,
                                mut out: *mut krb5_data) {
    (*out).length = k5_input_get_uint32_be(in_0);
    (*out).data =
        k5_input_get_bytes(in_0, (*out).length as size_t) as
            *mut libc::c_char;
    (*out).magic = -(1760647422 as libc::c_long) as krb5_magic;
}
/* Parse a received cookie into its components.  The pointers stored in the
 * krb5_data outputs are aliases into cookie and should not be freed. */
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn parse_cookie(mut cookie: *const krb5_data,
                                  mut stage_out: *mut libc::c_int,
                                  mut group_out: *mut int32_t,
                                  mut spake_out: *mut krb5_data,
                                  mut thash_out: *mut krb5_data,
                                  mut factors_out: *mut krb5_data)
 -> krb5_error_code {
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut version: libc::c_int = 0;
    let mut stage: libc::c_int = 0;
    let mut group: int32_t = 0;
    let mut thash: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut spake: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut factors: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    *factors_out = empty_data();
    *thash_out = *factors_out;
    *spake_out = *thash_out;
    k5_input_init(&mut in_0, (*cookie).data as *const libc::c_void,
                  (*cookie).length as size_t);
    /* Parse and check the version, and read the other integer fields. */
    version = k5_input_get_uint16_be(&mut in_0) as libc::c_int;
    if version != 1 as libc::c_int {
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    stage = k5_input_get_uint16_be(&mut in_0) as libc::c_int;
    group = k5_input_get_uint32_be(&mut in_0) as int32_t;
    /* Parse the data fields.  The factor data is anything remaining after the
     * transcript hash. */
    parse_data(&mut in_0, &mut spake);
    parse_data(&mut in_0, &mut thash);
    if in_0.status != 0 { return in_0.status }
    factors =
        make_data(in_0.ptr as *mut libc::c_char as *mut libc::c_void,
                  in_0.len as libc::c_uint);
    *stage_out = stage;
    *group_out = group;
    *spake_out = spake;
    *thash_out = thash;
    *factors_out = factors;
    return 0 as libc::c_int;
}
/* Marshal data into buf as a four-byte length followed by the contents. */
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn marshal_data(mut buf: *mut k5buf,
                                  mut data: *const krb5_data) {
    k5_buf_add_uint32_be(buf, (*data).length);
    k5_buf_add_len(buf, (*data).data as *const libc::c_void,
                   (*data).length as size_t);
}
/* Marshal components into a cookie. */
#[c2rust::src_loc = "128:1"]
unsafe extern "C" fn make_cookie(mut stage: libc::c_int, mut group: int32_t,
                                 mut spake: *const krb5_data,
                                 mut thash: *const krb5_data,
                                 mut cookie_out: *mut krb5_data)
 -> krb5_error_code {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    *cookie_out = empty_data();
    k5_buf_init_dynamic_zap(&mut buf);
    /* Marshal the version, stage, and group. */
    k5_buf_add_uint16_be(&mut buf, 1 as libc::c_int as uint16_t);
    k5_buf_add_uint16_be(&mut buf, stage as uint16_t);
    k5_buf_add_uint32_be(&mut buf, group as uint32_t);
    /* Marshal the data fields. */
    marshal_data(&mut buf, spake);
    marshal_data(&mut buf, thash);
    /* When second factor support is implemented, we should add factor data
     * here. */
    if buf.data.is_null() { return 12 as libc::c_int }
    *cookie_out = make_data(buf.data, buf.len as libc::c_uint);
    return 0 as libc::c_int;
}
/* Add authentication indicators if any are configured for SPAKE. */
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn add_indicators(mut context: krb5_context,
                                    mut realm: *const krb5_data,
                                    mut cb: krb5_kdcpreauth_callbacks,
                                    mut rock: krb5_kdcpreauth_rock)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut keys: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    let mut realmstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indicators: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ind: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    realmstr =
        k5memdup0((*realm).data as *const libc::c_void,
                  (*realm).length as size_t, &mut ret) as *mut libc::c_char;
    if realmstr.is_null() { return ret }
    keys[0 as libc::c_int as usize] =
        b"realms\x00" as *const u8 as *const libc::c_char;
    keys[1 as libc::c_int as usize] = realmstr;
    keys[2 as libc::c_int as usize] =
        b"spake_preauth_indicator\x00" as *const u8 as *const libc::c_char;
    keys[3 as libc::c_int as usize] = 0 as *const libc::c_char;
    ret =
        profile_get_values((*context).profile, keys.as_mut_ptr(),
                           &mut indicators) as krb5_error_code;
    free(realmstr as *mut libc::c_void);
    if ret as libc::c_long == -(1429577725 as libc::c_long) {
        return 0 as libc::c_int
    }
    if ret != 0 { return ret }
    ind = indicators;
    while !(*ind).is_null() && ret == 0 {
        ret =
            (*cb).add_auth_indicator.expect("non-null function pointer")(context,
                                                                         rock,
                                                                         *ind);
        ind = ind.offset(1)
    }
    profile_free_list(indicators);
    return ret;
}
/* Initialize a SPAKE module data object. */
#[c2rust::src_loc = "186:1"]
unsafe extern "C" fn spake_init(mut context: krb5_context,
                                mut moddata_out: *mut krb5_kdcpreauth_moddata,
                                mut realmnames: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut gstate: *mut groupstate = 0 as *mut groupstate;
    ret =
        group_init_state(context, 1 as libc::c_int as krb5_boolean,
                         &mut gstate);
    if ret != 0 { return ret }
    *moddata_out = gstate as krb5_kdcpreauth_moddata;
    return 0 as libc::c_int;
}
/* Release a SPAKE module data object. */
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn spake_fini(mut context: krb5_context,
                                mut moddata: krb5_kdcpreauth_moddata) {
    group_free_state(moddata as *mut groupstate);
}
/*
 * Generate a SPAKE challenge message for the specified group.  Use cb and rock
 * to retrieve the initial reply key and to set a stage-0 cookie.  Invoke
 * either erespond or vrespond with the result.
 */
#[c2rust::src_loc = "212:1"]
unsafe extern "C" fn send_challenge(mut context: krb5_context,
                                    mut gstate: *mut groupstate,
                                    mut group: int32_t,
                                    mut cb: krb5_kdcpreauth_callbacks,
                                    mut rock: krb5_kdcpreauth_rock,
                                    mut support: *const krb5_data,
                                    mut erespond:
                                        krb5_kdcpreauth_edata_respond_fn,
                                    mut vrespond:
                                        krb5_kdcpreauth_verify_respond_fn,
                                    mut arg: *mut libc::c_void) {
    let mut ret: krb5_error_code = 0;
    let mut ikey: *const krb5_keyblock = 0 as *const krb5_keyblock;
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut kdcpriv: krb5_data = empty_data();
    let mut kdcpub: krb5_data = empty_data();
    let mut der_msg: *mut krb5_data = 0 as *mut krb5_data;
    let mut thash: krb5_data = empty_data();
    let mut cookie: krb5_data = empty_data();
    let mut wbytes: krb5_data = empty_data();
    let mut f: krb5_spake_factor =
        krb5_spake_factor{type_0: 0, data: 0 as *mut krb5_data,};
    let mut flist: [*mut krb5_spake_factor; 2] =
        [0 as *mut krb5_spake_factor; 2];
    let mut msg: krb5_pa_spake =
        krb5_pa_spake{choice: SPAKE_MSGTYPE_SUPPORT,
                      u:
                          krb5_spake_message_choices{support:
                                                         krb5_spake_support{ngroups:
                                                                                0,
                                                                            groups:
                                                                                0
                                                                                    as
                                                                                    *mut int32_t,},},};
    ikey =
        (*cb).client_keyblock.expect("non-null function pointer")(context,
                                                                  rock);
    if ikey.is_null() {
        ret = -(1765328370 as libc::c_long) as krb5_error_code
    } else {
        ret = derive_wbytes(context, group, ikey, &mut wbytes);
        if !(ret != 0) {
            ret =
                group_keygen(context, gstate, group, &mut wbytes,
                             &mut kdcpriv, &mut kdcpub);
            if !(ret != 0) {
                /* Encode the challenge.  When second factor support is implemented, we
     * should construct a factor list instead of hardcoding SF-NONE. */
                f.type_0 = SPAKE_SF_NONE as libc::c_int;
                f.data = 0 as *mut krb5_data;
                flist[0 as libc::c_int as usize] = &mut f;
                flist[1 as libc::c_int as usize] =
                    0 as *mut krb5_spake_factor;
                msg.choice = SPAKE_MSGTYPE_CHALLENGE;
                msg.u.challenge.group = group;
                msg.u.challenge.pubkey = kdcpub;
                msg.u.challenge.factors = flist.as_mut_ptr();
                ret = encode_krb5_pa_spake(&mut msg, &mut der_msg);
                if !(ret != 0) {
                    /* Initialize and update the transcript hash with the support message (if
     * we received one) and challenge message. */
                    ret =
                        update_thash(context, gstate, group, &mut thash,
                                     support, der_msg);
                    if !(ret != 0) {
                        /* Save the group, transcript hash, and private key in a stage-0 cookie.
     * When second factor support is implemented, also save factor state. */
                        ret =
                            make_cookie(0 as libc::c_int, group, &mut kdcpriv,
                                        &mut thash, &mut cookie);
                        if !(ret != 0) {
                            ret =
                                (*cb).set_cookie.expect("non-null function pointer")(context,
                                                                                     rock,
                                                                                     151
                                                                                         as
                                                                                         libc::c_int,
                                                                                     &mut cookie);
                            if !(ret != 0) {
                                ret = convert_to_padata(der_msg, &mut padata);
                                der_msg = 0 as *mut krb5_data;
                                if (*context).trace_callback.is_some() {
                                    krb5int_trace(context,
                                                  b"Sending SPAKE challenge with group {int}\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  group);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    zapfree(wbytes.data as *mut libc::c_void, wbytes.length as size_t);
    zapfree(kdcpriv.data as *mut libc::c_void, kdcpriv.length as size_t);
    zapfree(cookie.data as *mut libc::c_void, cookie.length as size_t);
    krb5_free_data_contents(context, &mut kdcpub);
    krb5_free_data_contents(context, &mut thash);
    krb5_free_data(context, der_msg);
    if erespond.is_some() {
        if vrespond.is_none() {
        } else {
            __assert_fail(b"vrespond == NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"spake_kdc.c\x00" as *const u8 as
                              *const libc::c_char,
                          283 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 202],
                                                    &[libc::c_char; 202]>(b"void send_challenge(krb5_context, groupstate *, int32_t, krb5_kdcpreauth_callbacks, krb5_kdcpreauth_rock, const krb5_data *, krb5_kdcpreauth_edata_respond_fn, krb5_kdcpreauth_verify_respond_fn, void *)\x00")).as_ptr());
        }
        /* Grab the first pa-data element from the list, if we made one. */
        pa =
            if padata.is_null() {
                0 as *mut krb5_pa_data
            } else { *padata.offset(0 as libc::c_int as isize) };
        free(padata as *mut libc::c_void);
        Some(erespond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                               ret,
                                                                                               pa);
    } else {
        if vrespond.is_some() {
        } else {
            __assert_fail(b"vrespond != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"spake_kdc.c\x00" as *const u8 as
                              *const libc::c_char,
                          289 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 202],
                                                    &[libc::c_char; 202]>(b"void send_challenge(krb5_context, groupstate *, int32_t, krb5_kdcpreauth_callbacks, krb5_kdcpreauth_rock, const krb5_data *, krb5_kdcpreauth_edata_respond_fn, krb5_kdcpreauth_verify_respond_fn, void *)\x00")).as_ptr());
        }
        if ret == 0 { ret = -(1765328293 as libc::c_long) as krb5_error_code }
        Some(vrespond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                               ret,
                                                                                               0
                                                                                                   as
                                                                                                   krb5_kdcpreauth_modreq,
                                                                                               padata,
                                                                                               0
                                                                                                   as
                                                                                                   *mut *mut krb5_authdata);
    };
}
/* Generate the METHOD-DATA entry indicating support for SPAKE.  Include an
 * optimistic challenge if configured to do so. */
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn spake_edata(mut context: krb5_context,
                                 mut req: *mut krb5_kdc_req,
                                 mut cb: krb5_kdcpreauth_callbacks,
                                 mut rock: krb5_kdcpreauth_rock,
                                 mut moddata: krb5_kdcpreauth_moddata,
                                 mut pa_type: krb5_preauthtype,
                                 mut respond:
                                     krb5_kdcpreauth_edata_respond_fn,
                                 mut arg: *mut libc::c_void) {
    let mut ikey: *const krb5_keyblock = 0 as *const krb5_keyblock;
    let mut gstate: *mut groupstate = moddata as *mut groupstate;
    let mut empty: krb5_data = empty_data();
    let mut group: int32_t = 0;
    /* SPAKE requires a client key, which cannot be a single-DES key. */
    ikey =
        (*cb).client_keyblock.expect("non-null function pointer")(context,
                                                                  rock);
    if ikey.is_null() {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              -(1765328370
                                                                                                    as
                                                                                                    libc::c_long)
                                                                                                  as
                                                                                                  krb5_error_code,
                                                                                              0
                                                                                                  as
                                                                                                  *mut krb5_pa_data);
        return
    }
    group = group_optimistic_challenge(gstate);
    if group != 0 {
        send_challenge(context, gstate, group, cb, rock, &mut empty, respond,
                       None, arg);
    } else {
        /* No optimistic challenge configured; send an empty pa-data value. */
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              0
                                                                                                  as
                                                                                                  *mut krb5_pa_data);
    };
}
/* Choose a group from the client's support message and generate a
 * challenge. */
#[c2rust::src_loc = "328:1"]
unsafe extern "C" fn verify_support(mut context: krb5_context,
                                    mut gstate: *mut groupstate,
                                    mut support: *mut krb5_spake_support,
                                    mut der_msg: *const krb5_data,
                                    mut cb: krb5_kdcpreauth_callbacks,
                                    mut rock: krb5_kdcpreauth_rock,
                                    mut respond:
                                        krb5_kdcpreauth_verify_respond_fn,
                                    mut arg: *mut libc::c_void) {
    let mut ret: krb5_error_code = 0;
    let mut i: int32_t = 0;
    let mut group: int32_t = 0;
    i = 0 as libc::c_int;
    while i < (*support).ngroups {
        if group_is_permitted(gstate, *(*support).groups.offset(i as isize))
               != 0 {
            break ;
        }
        i += 1
    }
    if i == (*support).ngroups {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"SPAKE support message rejected\x00" as *const u8
                              as *const libc::c_char);
        }
        ret = -(1765328360 as libc::c_long) as krb5_error_code;
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              ret,
                                                                                              0
                                                                                                  as
                                                                                                  krb5_kdcpreauth_modreq,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_pa_data,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_authdata);
        return;
    } else {
        group = *(*support).groups.offset(i as isize);
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"SPAKE support message received, selected group {int}\x00"
                              as *const u8 as *const libc::c_char, group);
        }
        send_challenge(context, gstate, group, cb, rock, der_msg, None,
                       respond, arg);
        return
    };
}
/*
 * From the client's response message, compute the SPAKE result and decrypt the
 * factor reply.  On success, either mark the reply as pre-authenticated and
 * set a reply key in the pre-request module data, or generate an additional
 * factor challenge and ask for another round of pre-authentication.
 */
#[c2rust::src_loc = "363:1"]
unsafe extern "C" fn verify_response(mut context: krb5_context,
                                     mut gstate: *mut groupstate,
                                     mut resp: *mut krb5_spake_response,
                                     mut realm: *const krb5_data,
                                     mut cb: krb5_kdcpreauth_callbacks,
                                     mut rock: krb5_kdcpreauth_rock,
                                     mut enc_tkt_reply:
                                         *mut krb5_enc_tkt_part,
                                     mut respond:
                                         krb5_kdcpreauth_verify_respond_fn,
                                     mut arg: *mut libc::c_void) {
    let mut ret: krb5_error_code = 0;
    let mut ikey: *const krb5_keyblock = 0 as *const krb5_keyblock;
    let mut k1: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut reply_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut cookie: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut thash_in: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut kdcpriv: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut factors: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut der_req: *mut krb5_data = 0 as *mut krb5_data;
    let mut thash: krb5_data = empty_data();
    let mut der_factor: krb5_data = empty_data();
    let mut wbytes: krb5_data = empty_data();
    let mut spakeresult: krb5_data = empty_data();
    let mut factor: *mut krb5_spake_factor = 0 as *mut krb5_spake_factor;
    let mut stage: libc::c_int = 0;
    let mut group: int32_t = 0;
    ikey =
        (*cb).client_keyblock.expect("non-null function pointer")(context,
                                                                  rock);
    if ikey.is_null() {
        ret = -(1765328370 as libc::c_long) as krb5_error_code
    } else if (*cb).get_cookie.expect("non-null function pointer")(context,
                                                                   rock,
                                                                   151 as
                                                                       libc::c_int,
                                                                   &mut cookie)
                  == 0 {
        ret = -(1765328360 as libc::c_long) as krb5_error_code
    } else {
        ret =
            parse_cookie(&mut cookie, &mut stage, &mut group, &mut kdcpriv,
                         &mut thash_in, &mut factors);
        if !(ret != 0) {
            if stage != 0 as libc::c_int {
                /* Fetch the stage-0 cookie and parse it.  (All of the krb5_data results
     * are aliases into memory owned by rock). */
                /* The received cookie wasn't sent with a challenge. */
                ret = -(1765328360 as libc::c_long) as krb5_error_code
            } else {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"SPAKE response received with pubkey {hexdata}\x00"
                                      as *const u8 as *const libc::c_char,
                                  &mut (*resp).pubkey as *mut krb5_data);
                }
                /* Update the transcript hash with the client public key. */
                ret =
                    krb5int_copy_data_contents(context, &mut thash_in,
                                               &mut thash);
                if !(ret != 0) {
                    ret =
                        update_thash(context, gstate, group, &mut thash,
                                     &mut (*resp).pubkey,
                                     0 as *const krb5_data);
                    if !(ret != 0) {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"SPAKE final transcript hash: {hexdata}\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          &mut thash as *mut krb5_data);
                        }
                        ret =
                            derive_wbytes(context, group, ikey, &mut wbytes);
                        if !(ret != 0) {
                            ret =
                                group_result(context, gstate, group,
                                             &mut wbytes, &mut kdcpriv,
                                             &mut (*resp).pubkey,
                                             &mut spakeresult);
                            if !(ret != 0) {
                                /* Decrypt the response factor field using K'[1].  If the decryption
     * integrity check fails, the client probably used the wrong password. */
                                der_req =
                                    (*cb).request_body.expect("non-null function pointer")(context,
                                                                                           rock);
                                ret =
                                    derive_key(context, gstate, group, ikey,
                                               &mut wbytes, &mut spakeresult,
                                               &mut thash, der_req,
                                               1 as libc::c_int as uint32_t,
                                               &mut k1);
                                if !(ret != 0) {
                                    ret =
                                        alloc_data(&mut der_factor,
                                                   (*resp).factor.ciphertext.length);
                                    if !(ret != 0) {
                                        ret =
                                            krb5_c_decrypt(context, k1,
                                                           65 as libc::c_int,
                                                           0 as
                                                               *const krb5_data,
                                                           &mut (*resp).factor,
                                                           &mut der_factor);
                                        if ret as libc::c_long ==
                                               -(1765328353 as libc::c_long) {
                                            ret =
                                                -(1765328360 as libc::c_long)
                                                    as krb5_error_code
                                        }
                                        if !(ret != 0) {
                                            ret =
                                                decode_krb5_spake_factor(&mut der_factor,
                                                                         &mut factor);
                                            if !(ret != 0) {
                                                /*
     * When second factor support is implemented, we should verify the factor
     * data here, and possibly generate an encdata message for another hop.
     * This function may need to be split at this point to allow for
     * asynchronous verification of the second-factor value.  We might also
     * need to collect authentication indicators from the second-factor module;
     * alternatively the module could have access to cb and rock so that it can
     * add indicators itself.
     */
                                                if (*factor).type_0 !=
                                                       SPAKE_SF_NONE as
                                                           libc::c_int {
                                                    ret =
                                                        -(1765328360 as
                                                              libc::c_long) as
                                                            krb5_error_code
                                                } else {
                                                    ret =
                                                        add_indicators(context,
                                                                       realm,
                                                                       cb,
                                                                       rock);
                                                    if !(ret != 0) {
                                                        (*enc_tkt_reply).flags
                                                            |=
                                                            0x200000 as
                                                                libc::c_int;
                                                        ret =
                                                            derive_key(context,
                                                                       gstate,
                                                                       group,
                                                                       ikey,
                                                                       &mut wbytes,
                                                                       &mut spakeresult,
                                                                       &mut thash,
                                                                       der_req,
                                                                       0 as
                                                                           libc::c_int
                                                                           as
                                                                           uint32_t,
                                                                       &mut reply_key)
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
    zapfree(wbytes.data as *mut libc::c_void, wbytes.length as size_t);
    zapfree(der_factor.data as *mut libc::c_void,
            der_factor.length as size_t);
    zapfree(spakeresult.data as *mut libc::c_void,
            spakeresult.length as size_t);
    krb5_free_data_contents(context, &mut thash);
    krb5_free_keyblock(context, k1);
    k5_free_spake_factor(context, factor);
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          ret,
                                                                                          reply_key
                                                                                              as
                                                                                              krb5_kdcpreauth_modreq,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_pa_data,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_authdata);
}
/*
 * Decrypt and validate an additional second-factor reply.  On success, either
 * mark the reply as pre-authenticated and set a reply key in the pre-request
 * module data, or generate an additional factor challenge and ask for another
 * round of pre-authentication.
 */
#[c2rust::src_loc = "478:1"]
unsafe extern "C" fn verify_encdata(mut context: krb5_context,
                                    mut enc: *mut krb5_enc_data,
                                    mut cb: krb5_kdcpreauth_callbacks,
                                    mut rock: krb5_kdcpreauth_rock,
                                    mut enc_tkt_reply: *mut krb5_enc_tkt_part,
                                    mut respond:
                                        krb5_kdcpreauth_verify_respond_fn,
                                    mut arg: *mut libc::c_void) {
    /*
     * When second factor support is implemented, we should process encdata
     * message according to the factor type recorded in the cookie.  If the
     * second factor exchange finishes successfully, we should set
     * TKT_FLG_PRE_AUTH, set the reply key to K'[0], and add any auth
     * indicators from configuration (with a call to add_indicators()) or the
     * second factor module (unless the module has access to cb and rock and
     * can add indicators itself).
     */
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          -(1765328360
                                                                                                as
                                                                                                libc::c_long)
                                                                                              as
                                                                                              krb5_error_code,
                                                                                          0
                                                                                              as
                                                                                              krb5_kdcpreauth_modreq,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_pa_data,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_authdata);
}
/*
 * Respond to a client padata message, either by generating a SPAKE challenge,
 * generating an additional second-factor challenge, or marking the reply as
 * pre-authenticated and setting an additional reply key in the pre-request
 * module data.
 */
#[c2rust::src_loc = "502:1"]
unsafe extern "C" fn spake_verify(mut context: krb5_context,
                                  mut req_pkt: *mut krb5_data,
                                  mut request: *mut krb5_kdc_req,
                                  mut enc_tkt_reply: *mut krb5_enc_tkt_part,
                                  mut data: *mut krb5_pa_data,
                                  mut cb: krb5_kdcpreauth_callbacks,
                                  mut rock: krb5_kdcpreauth_rock,
                                  mut moddata: krb5_kdcpreauth_moddata,
                                  mut respond:
                                      krb5_kdcpreauth_verify_respond_fn,
                                  mut arg: *mut libc::c_void) {
    let mut ret: krb5_error_code = 0;
    let mut pa_spake: *mut krb5_pa_spake = 0 as *mut krb5_pa_spake;
    let mut in_data: krb5_data =
        make_data((*data).contents as *mut libc::c_void, (*data).length);
    let mut gstate: *mut groupstate = moddata as *mut groupstate;
    ret = decode_krb5_pa_spake(&mut in_data, &mut pa_spake);
    if ret != 0 {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              ret,
                                                                                              0
                                                                                                  as
                                                                                                  krb5_kdcpreauth_modreq,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_pa_data,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_authdata);
    } else if (*pa_spake).choice as libc::c_int ==
                  SPAKE_MSGTYPE_SUPPORT as libc::c_int {
        verify_support(context, gstate, &mut (*pa_spake).u.support,
                       &mut in_data, cb, rock, respond, arg);
    } else if (*pa_spake).choice as libc::c_int ==
                  SPAKE_MSGTYPE_RESPONSE as libc::c_int {
        verify_response(context, gstate, &mut (*pa_spake).u.response,
                        &mut (*(*request).server).realm, cb, rock,
                        enc_tkt_reply, respond, arg);
    } else if (*pa_spake).choice as libc::c_int ==
                  SPAKE_MSGTYPE_ENCDATA as libc::c_int {
        verify_encdata(context, &mut (*pa_spake).u.encdata, cb, rock,
                       enc_tkt_reply, respond, arg);
    } else {
        ret = -(1765328360 as libc::c_long) as krb5_error_code;
        krb5_set_error_message(context, ret,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Unknown SPAKE request type\x00" as
                                            *const u8 as
                                            *const libc::c_char));
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              ret,
                                                                                              0
                                                                                                  as
                                                                                                  krb5_kdcpreauth_modreq,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_pa_data,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_authdata);
    }
    k5_free_pa_spake(context, pa_spake);
}
/* If a key was set in the per-request module data, replace the reply key.  Do
 * not generate any pa-data to include with the KDC reply. */
#[c2rust::src_loc = "538:1"]
unsafe extern "C" fn spake_return(mut context: krb5_context,
                                  mut padata: *mut krb5_pa_data,
                                  mut req_pkt: *mut krb5_data,
                                  mut request: *mut krb5_kdc_req,
                                  mut reply: *mut krb5_kdc_rep,
                                  mut encrypting_key: *mut krb5_keyblock,
                                  mut send_pa_out: *mut *mut krb5_pa_data,
                                  mut cb: krb5_kdcpreauth_callbacks,
                                  mut rock: krb5_kdcpreauth_rock,
                                  mut moddata: krb5_kdcpreauth_moddata,
                                  mut modreq: krb5_kdcpreauth_modreq)
 -> krb5_error_code {
    let mut reply_key: *mut krb5_keyblock = modreq as *mut krb5_keyblock;
    if reply_key.is_null() { return 0 as libc::c_int }
    krb5_free_keyblock_contents(context, encrypting_key);
    return krb5_copy_keyblock_contents(context, reply_key, encrypting_key);
}
/* Release a per-request module data object. */
#[c2rust::src_loc = "554:1"]
unsafe extern "C" fn spake_free_modreq(mut context: krb5_context,
                                       mut moddata: krb5_kdcpreauth_moddata,
                                       mut modreq: krb5_kdcpreauth_modreq) {
    krb5_free_keyblock(context, modreq as *mut krb5_keyblock);
}
#[no_mangle]
#[c2rust::src_loc = "565:1"]
pub unsafe extern "C" fn kdcpreauth_spake_initvt(mut context: krb5_context,
                                                 mut maj_ver: libc::c_int,
                                                 mut min_ver: libc::c_int,
                                                 mut vtable:
                                                     krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_kdcpreauth_vtable = 0 as *mut krb5_kdcpreauth_vtable_st;
    static mut pa_types: [krb5_preauthtype; 2] =
        [151 as libc::c_int, 0 as libc::c_int];
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_kdcpreauth_vtable;
    (*vt).name =
        b"spake\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*vt).pa_type_list = pa_types.as_mut_ptr();
    (*vt).init =
        Some(spake_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut krb5_kdcpreauth_moddata,
                                      _: *mut *const libc::c_char)
                     -> krb5_error_code);
    (*vt).fini =
        Some(spake_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_kdcpreauth_moddata) -> ());
    (*vt).edata =
        Some(spake_edata as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_kdc_req,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_preauthtype,
                                      _: krb5_kdcpreauth_edata_respond_fn,
                                      _: *mut libc::c_void) -> ());
    (*vt).verify =
        Some(spake_verify as
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
        Some(spake_return as
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
    (*vt).free_modreq =
        Some(spake_free_modreq as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_kdcpreauth_modreq) -> ());
    return 0 as libc::c_int;
}
