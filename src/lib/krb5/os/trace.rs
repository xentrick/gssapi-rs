use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types.h:40"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:40"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:40"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:40"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/include/unistd.h:40"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::{__socklen_t, __pid_t};
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:40"]
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
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
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
    /* *
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
    #[c2rust::src_loc = "379:1"]
    pub type krb5_key = *mut krb5_key_st;
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
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
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
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, krb5_key_st, _krb5_kt};
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
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
        #[no_mangle]
        #[c2rust::src_loc = "1457:1"]
        pub fn krb5_k_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype, key: krb5_key,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "2330:1"]
        pub fn krb5_cc_get_name(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        /* *
 * Retrieve the type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @return The type of a credential cache as an alias that must not be modified
 * or freed by the caller.
 */
        #[no_mangle]
        #[c2rust::src_loc = "2598:1"]
        pub fn krb5_cc_get_type(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
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
        #[no_mangle]
        #[c2rust::src_loc = "2770:1"]
        pub fn krb5_kt_get_name(context: krb5_context, keytab: krb5_keytab,
                                name: *mut libc::c_char,
                                namelen: libc::c_uint) -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:40"]
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
    #[c2rust::src_loc = "628:8"]
    pub struct krb5_key_st {
        pub keyblock: krb5_keyblock,
        pub refcount: libc::c_int,
        pub derived: *mut derived_key,
        pub cache: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "621:8"]
    pub struct derived_key {
        pub constant: krb5_data,
        pub dkey: krb5_key,
        pub next: *mut derived_key,
    }
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
                        krb5_keyblock, krb5_data, krb5_key, krb5_pointer,
                        krb5_error_code, krb5_context, krb5_keytab,
                        krb5_const_principal, krb5_kvno, krb5_keytab_entry,
                        krb5_kt_cursor, krb5_timestamp};
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
        #[c2rust::src_loc = "696:1"]
        pub fn krb5_crypto_us_timeofday(_: *mut krb5_timestamp,
                                        _: *mut krb5_int32)
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:40"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:40"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:8"]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:40"]
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
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:41"]
pub mod os_proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:8"]
    pub struct remote_address {
        pub transport: k5_transport,
        pub family: libc::c_int,
        pub len: socklen_t,
        pub saddr: sockaddr_storage,
    }
    #[c2rust::src_loc = "41:9"]
    pub type k5_transport = libc::c_uint;
    #[c2rust::src_loc = "45:5"]
    pub const HTTPS: k5_transport = 3;
    #[c2rust::src_loc = "44:5"]
    pub const UDP: k5_transport = 2;
    #[c2rust::src_loc = "43:5"]
    pub const TCP: k5_transport = 1;
    #[c2rust::src_loc = "42:5"]
    pub const TCP_OR_UDP: k5_transport = 0;
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr_storage;
    /* KRB5_LIBOS_INT_PROTO__ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:40"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn secure_getenv(__name: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:40"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:40"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:40"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:40"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* Provide fnmatch interface. */
        /* Provide [v]asprintf interfaces.  */
        /* no vsnprintf */
        /* have vasprintf and prototype? */
        /* Return true if the snprintf return value RESULT reflects a buffer
   overflow for the buffer size SIZE.

   We cast the result to unsigned int for two reasons.  First, old
   implementations of snprintf (such as the one in Solaris 9 and
   prior) return -1 on a buffer overflow.  Casting the result to -1
   will convert that value to UINT_MAX, which should compare larger
   than any reasonable buffer size.  Second, comparing signed and
   unsigned integers will generate warnings with some compilers, and
   can have unpredictable results, particularly when the relative
   widths of the types is not known (size_t may be the same width as
   int or larger).
*/
        #[no_mangle]
        #[c2rust::src_loc = "990:1"]
        pub fn k5_strerror_r(errnum: libc::c_int, buf: *mut libc::c_char,
                             buflen: size_t) -> libc::c_int;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:40"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "272:15"]
        pub fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:40"]
pub mod netdb_h {
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getnameinfo(__sa: *const sockaddr, __salen: socklen_t,
                           __host: *mut libc::c_char, __hostlen: socklen_t,
                           __serv: *mut libc::c_char, __servlen: socklen_t,
                           __flags: libc::c_int) -> libc::c_int;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__uint8_t, __int32_t, __pid_t, __ssize_t,
                        __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdarg_h::va_list;
pub use self::stdint_uintn_h::uint8_t;
pub use self::unistd_h::{socklen_t, close, write, getpid};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       krb5_key, _krb5_checksum, krb5_checksum,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds, _krb5_pa_data,
                       krb5_pa_data, krb5_ccache, krb5_kt_cursor,
                       krb5_keytab_entry_st, krb5_keytab_entry, krb5_keytab,
                       _profile_t, _krb5_ccache, krb5_k_make_checksum,
                       krb5_cc_get_name, krb5_cc_get_type, krb5_kt_get_name,
                       krb5_unparse_name, krb5_free_checksum_contents,
                       krb5_free_unparsed_name, krb5_enctype_to_name,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_key_st, derived_key, _krb5_kt,
                         _krb5_kt_ops, make_data, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_crypto_us_timeofday};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_len, k5_buf_add_fmt};
pub use self::os_proto_h::{remote_address, k5_transport, HTTPS, UDP, TCP,
                           TCP_OR_UDP};
use self::stdlib_h::{malloc, free, secure_getenv};
use self::stdio_h::{snprintf, asprintf};
use self::fcntl_h::open;
use self::errno_h::__errno_location;
use self::k5_platform_h::k5_strerror_r;
use self::string_h::{strlen, strcspn, strcmp, memcpy};
use self::netdb_h::getnameinfo;
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn buf_is_printable(mut p: *const libc::c_char,
                                      mut len: size_t) -> krb5_boolean {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        if (*p.offset(i as isize) as libc::c_int) < 32 as libc::c_int ||
               *p.offset(i as isize) as libc::c_int > 126 as libc::c_int {
            break ;
        }
        i = i.wrapping_add(1)
    }
    return (i == len) as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn buf_add_printable_len(mut buf: *mut k5buf,
                                           mut p: *const libc::c_char,
                                           mut len: size_t) {
    let mut text: [libc::c_char; 5] = [0; 5];
    let mut i: size_t = 0;
    if buf_is_printable(p, len) != 0 {
        k5_buf_add_len(buf, p as *const libc::c_void, len);
    } else {
        i = 0 as libc::c_int as size_t;
        while i < len {
            if buf_is_printable(p.offset(i as isize),
                                1 as libc::c_int as size_t) != 0 {
                k5_buf_add_len(buf,
                               p.offset(i as isize) as *const libc::c_void,
                               1 as libc::c_int as size_t);
            } else {
                snprintf(text.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 5]>() as
                             libc::c_ulong,
                         b"\\x%02x\x00" as *const u8 as *const libc::c_char,
                         (*p.offset(i as isize) as libc::c_int &
                              0xff as libc::c_int) as libc::c_uint);
                k5_buf_add_len(buf, text.as_mut_ptr() as *const libc::c_void,
                               4 as libc::c_int as size_t);
            }
            i = i.wrapping_add(1)
        }
    };
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn buf_add_printable(mut buf: *mut k5buf,
                                       mut p: *const libc::c_char) {
    buf_add_printable_len(buf, p, strlen(p));
}
/* Return a four-byte hex string from the first two bytes of a SHA-1 hash of a
 * byte array.  Return NULL on failure. */
#[c2rust::src_loc = "89:1"]
unsafe extern "C" fn hash_bytes(mut context: krb5_context,
                                mut ptr: *const libc::c_void, mut len: size_t)
 -> *mut libc::c_char {
    let mut cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut d: krb5_data =
        make_data(ptr as *mut libc::c_void, len as libc::c_uint);
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if krb5_k_make_checksum(context, 0x9 as libc::c_int, 0 as krb5_key,
                            0 as libc::c_int, &mut d, &mut cksum) !=
           0 as libc::c_int {
        return 0 as *mut libc::c_char
    }
    if cksum.length >= 2 as libc::c_int as libc::c_uint {
        asprintf(&mut s as *mut *mut libc::c_char,
                 b"%02X%02X\x00" as *const u8 as *const libc::c_char,
                 *cksum.contents.offset(0 as libc::c_int as isize) as
                     libc::c_int,
                 *cksum.contents.offset(1 as libc::c_int as isize) as
                     libc::c_int);
    }
    krb5_free_checksum_contents(context, &mut cksum);
    return s;
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn principal_type_string(mut type_0: krb5_int32)
 -> *mut libc::c_char {
    match type_0 {
        0 => {
            return b"unknown\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        1 => {
            return b"principal\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"service instance\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        3 => {
            return b"service with host as instance\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        4 => {
            return b"service with host as components\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        5 => {
            return b"unique ID\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        6 => {
            return b"X.509\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        7 => {
            return b"SMTP email\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        10 => {
            return b"Windows 2000 UPN\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        11 => {
            return b"well-known\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        -128 => {
            return b"Windows 2000 UPN and SID\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        -129 => {
            return b"NT 4 style name\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        -130 => {
            return b"NT 4 style name and SID\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        _ => {
            return b"?\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn padata_type_string(mut type_0: krb5_preauthtype)
 -> *mut libc::c_char {
    match type_0 {
        1 => {
            return b"PA-TGS-REQ\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"PA-ENC-TIMESTAMP\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        3 => {
            return b"PA-PW-SALT\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        5 => {
            return b"PA-ENC-UNIX-TIME\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        6 => {
            return b"PA-SANDIA-SECUREID\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        7 => {
            return b"PA-SESAME\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        8 => {
            return b"PA-OSF-DCE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        9 => {
            return b"PA-CYBERSAFE-SECUREID\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        10 => {
            return b"PA-AFS3-SALT\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        11 => {
            return b"PA-ETYPE-INFO\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        12 => {
            return b"PA-SAM-CHALLENGE\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        13 => {
            return b"PA-SAM-RESPONSE\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        14 => {
            return b"PA-PK-AS-REQ_OLD\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        15 => {
            return b"PA-PK-AS-REP_OLD\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        16 => {
            return b"PA-PK-AS-REQ\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        17 => {
            return b"PA-PK-AS-REP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        19 => {
            return b"PA-ETYPE-INFO2\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        20 => {
            return b"PA-SVR-REFERRAL-INFO\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        21 => {
            return b"PA-SAM-REDIRECT\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        22 => {
            return b"PA-GET-FROM-TYPED-DATA\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        30 => {
            return b"PA-SAM-CHALLENGE2\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        31 => {
            return b"PA-SAM-RESPONSE2\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        128 => {
            return b"PA-PAC-REQUEST\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        129 => {
            return b"PA-FOR_USER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        130 => {
            return b"PA-FOR-X509-USER\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        132 => {
            return b"PA-AS-CHECKSUM\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        133 => {
            return b"PA-FX-COOKIE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        136 => {
            return b"PA-FX-FAST\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        137 => {
            return b"PA-FX-ERROR\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        138 => {
            return b"PA-ENCRYPTED-CHALLENGE\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        141 => {
            return b"PA-OTP-CHALLENGE\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        142 => {
            return b"PA-OTP-REQUEST\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        144 => {
            return b"PA-OTP-PIN-CHANGE\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        147 => {
            return b"PA-PKINIT-KX\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        149 => {
            return b"PA-REQ-ENC-PA-REP\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char
        }
        150 => {
            return b"PA_AS_FRESHNESS\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char
        }
        151 => {
            return b"PA-SPAKE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => { return 0 as *mut libc::c_char }
    };
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn trace_format(mut context: krb5_context,
                                  mut fmt: *const libc::c_char,
                                  mut ap: ::std::ffi::VaList)
 -> *mut libc::c_char {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut kerr: krb5_error_code = 0;
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut err: libc::c_int = 0;
    let mut ra: *mut remote_address = 0 as *mut remote_address;
    let mut d: *const krb5_data = 0 as *const krb5_data;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut addrbuf: [libc::c_char; 1025] = [0; 1025];
    let mut portbuf: [libc::c_char; 32] = [0; 32];
    let mut tmpbuf: [libc::c_char; 200] = [0; 200];
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut princ: krb5_const_principal = 0 as *const krb5_principal_data;
    let mut keyblock: *const krb5_keyblock = 0 as *const krb5_keyblock;
    let mut key: krb5_key = 0 as *mut krb5_key_st;
    let mut cksum: *const krb5_checksum = 0 as *const krb5_checksum;
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut pa_type: krb5_preauthtype = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ccache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut keytab: krb5_keytab = 0 as *mut _krb5_kt;
    let mut creds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut etypes: *mut krb5_enctype = 0 as *mut krb5_enctype;
    let mut etype: krb5_enctype = 0;
    k5_buf_init_dynamic(&mut buf);
    loop  {
        /* Advance to the next word in braces. */
        len = strcspn(fmt, b"{\x00" as *const u8 as *const libc::c_char);
        k5_buf_add_len(&mut buf, fmt as *const libc::c_void, len);
        if *fmt.offset(len as isize) as libc::c_int == '\u{0}' as i32 {
            break ;
        }
        fmt =
            fmt.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                           isize);
        len = strcspn(fmt, b"}\x00" as *const u8 as *const libc::c_char);
        if *fmt.offset(len as isize) as libc::c_int == '\u{0}' as i32 ||
               len >
                   (::std::mem::size_of::<[libc::c_char; 200]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) {
            break ;
        }
        memcpy(tmpbuf.as_mut_ptr() as *mut libc::c_void,
               fmt as *const libc::c_void, len);
        tmpbuf[len as usize] = '\u{0}' as i32 as libc::c_char;
        fmt =
            fmt.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                           isize);
        /* Process the format word. */
        if strcmp(tmpbuf.as_mut_ptr(),
                  b"int\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"%d\x00" as *const u8 as *const libc::c_char,
                           ap.as_va_list().arg::<libc::c_int>());
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"long\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"%ld\x00" as *const u8 as *const libc::c_char,
                           ap.as_va_list().arg::<libc::c_long>());
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"str\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            p = ap.as_va_list().arg::<*const libc::c_char>();
            buf_add_printable(&mut buf,
                              if p.is_null() {
                                  b"(null)\x00" as *const u8 as
                                      *const libc::c_char
                              } else { p });
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"lenstr\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            len = ap.as_va_list().arg::<size_t>();
            p = ap.as_va_list().arg::<*const libc::c_char>();
            if p.is_null() && len != 0 as libc::c_int as libc::c_ulong {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else if !p.is_null() {
                buf_add_printable_len(&mut buf, p, len);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"hexlenstr\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            len = ap.as_va_list().arg::<size_t>();
            p = ap.as_va_list().arg::<*const libc::c_char>();
            if p.is_null() && len != 0 as libc::c_int as libc::c_ulong {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else {
                i = 0 as libc::c_int as size_t;
                while i < len {
                    k5_buf_add_fmt(&mut buf as *mut k5buf,
                                   b"%02X\x00" as *const u8 as
                                       *const libc::c_char,
                                   *p.offset(i as isize) as libc::c_uchar as
                                       libc::c_int);
                    i = i.wrapping_add(1)
                }
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"hashlenstr\x00" as *const u8 as
                             *const libc::c_char) == 0 as libc::c_int {
            len = ap.as_va_list().arg::<size_t>();
            p = ap.as_va_list().arg::<*const libc::c_char>();
            if p.is_null() && len != 0 as libc::c_int as libc::c_ulong {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else {
                str = hash_bytes(context, p as *const libc::c_void, len);
                if !str.is_null() { k5_buf_add(&mut buf, str); }
                free(str as *mut libc::c_void);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"raddr\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            ra = ap.as_va_list().arg::<*mut remote_address>();
            if (*ra).transport as libc::c_uint ==
                   UDP as libc::c_int as libc::c_uint {
                k5_buf_add(&mut buf,
                           b"dgram\x00" as *const u8 as *const libc::c_char);
            } else if (*ra).transport as libc::c_uint ==
                          TCP as libc::c_int as libc::c_uint {
                k5_buf_add(&mut buf,
                           b"stream\x00" as *const u8 as *const libc::c_char);
            } else if (*ra).transport as libc::c_uint ==
                          HTTPS as libc::c_int as libc::c_uint {
                k5_buf_add(&mut buf,
                           b"https\x00" as *const u8 as *const libc::c_char);
            } else {
                k5_buf_add_fmt(&mut buf as *mut k5buf,
                               b"transport%d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*ra).transport as libc::c_uint);
            }
            if getnameinfo(&mut (*ra).saddr as *mut sockaddr_storage as
                               *mut sockaddr, (*ra).len, addrbuf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 1025]>() as
                               libc::c_ulong as socklen_t,
                           portbuf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 32]>() as
                               libc::c_ulong as socklen_t,
                           1 as libc::c_int | 2 as libc::c_int) !=
                   0 as libc::c_int {
                if (*ra).family == 0 as libc::c_int {
                    k5_buf_add(&mut buf,
                               b" AF_UNSPEC\x00" as *const u8 as
                                   *const libc::c_char);
                } else {
                    k5_buf_add_fmt(&mut buf as *mut k5buf,
                                   b" af%d\x00" as *const u8 as
                                       *const libc::c_char, (*ra).family);
                }
            } else {
                k5_buf_add_fmt(&mut buf as *mut k5buf,
                               b" %s:%s\x00" as *const u8 as
                                   *const libc::c_char, addrbuf.as_mut_ptr(),
                               portbuf.as_mut_ptr());
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"data\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            d = ap.as_va_list().arg::<*mut krb5_data>();
            if d.is_null() ||
                   (*d).length != 0 as libc::c_int as libc::c_uint &&
                       (*d).data.is_null() {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else {
                buf_add_printable_len(&mut buf, (*d).data,
                                      (*d).length as size_t);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"hexdata\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            d = ap.as_va_list().arg::<*mut krb5_data>();
            if d.is_null() {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else {
                subfmt(context, &mut buf as *mut k5buf,
                       b"{hexlenstr}\x00" as *const u8 as *const libc::c_char,
                       (*d).length, (*d).data);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"errno\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            err = ap.as_va_list().arg::<libc::c_int>();
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"%d\x00" as *const u8 as *const libc::c_char,
                           err);
            if k5_strerror_r(err, tmpbuf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 200]>() as
                                 libc::c_ulong) == 0 as libc::c_int {
                k5_buf_add_fmt(&mut buf as *mut k5buf,
                               b"/%s\x00" as *const u8 as *const libc::c_char,
                               tmpbuf.as_mut_ptr());
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"kerr\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            kerr = ap.as_va_list().arg::<krb5_error_code>();
            p = krb5_get_error_message(context, kerr);
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"%ld/%s\x00" as *const u8 as *const libc::c_char,
                           kerr as libc::c_long,
                           if kerr != 0 {
                               p
                           } else {
                               b"Success\x00" as *const u8 as
                                   *const libc::c_char
                           });
            krb5_free_error_message(context, p);
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"keyblock\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            keyblock = ap.as_va_list().arg::<*const krb5_keyblock>();
            if keyblock.is_null() {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else {
                subfmt(context, &mut buf as *mut k5buf,
                       b"{etype}/{hashlenstr}\x00" as *const u8 as
                           *const libc::c_char, (*keyblock).enctype,
                       (*keyblock).length, (*keyblock).contents);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"key\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            key = ap.as_va_list().arg::<krb5_key>();
            if key.is_null() {
                k5_buf_add(&mut buf,
                           b"(null)\x00" as *const u8 as *const libc::c_char);
            } else {
                subfmt(context, &mut buf as *mut k5buf,
                       b"{keyblock}\x00" as *const u8 as *const libc::c_char,
                       &mut (*key).keyblock as *mut krb5_keyblock);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"cksum\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            cksum = ap.as_va_list().arg::<*const krb5_checksum>();
            data =
                make_data((*cksum).contents as *mut libc::c_void,
                          (*cksum).length);
            subfmt(context, &mut buf as *mut k5buf,
                   b"{int}/{hexdata}\x00" as *const u8 as *const libc::c_char,
                   (*cksum).checksum_type, &mut data as *mut krb5_data);
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"princ\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            princ =
                ap.as_va_list().arg::<krb5_principal>() as
                    krb5_const_principal;
            if krb5_unparse_name(context, princ, &mut str) == 0 as libc::c_int
               {
                k5_buf_add(&mut buf, str);
                krb5_free_unparsed_name(context, str);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"ptype\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            p = principal_type_string(ap.as_va_list().arg::<krb5_int32>());
            k5_buf_add(&mut buf, p);
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"patypes\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            padata = ap.as_va_list().arg::<*mut *mut krb5_pa_data>();
            if padata.is_null() || (*padata).is_null() {
                k5_buf_add(&mut buf,
                           b"(empty)\x00" as *const u8 as
                               *const libc::c_char);
            }
            while !padata.is_null() && !(*padata).is_null() {
                pa_type = (**padata).pa_type;
                name = padata_type_string(pa_type);
                if !name.is_null() {
                    k5_buf_add_fmt(&mut buf as *mut k5buf,
                                   b"%s (%d)\x00" as *const u8 as
                                       *const libc::c_char, name, pa_type);
                } else {
                    k5_buf_add_fmt(&mut buf as *mut k5buf,
                                   b"%d\x00" as *const u8 as
                                       *const libc::c_char, pa_type);
                }
                if !(*padata.offset(1 as libc::c_int as isize)).is_null() {
                    k5_buf_add(&mut buf,
                               b", \x00" as *const u8 as *const libc::c_char);
                }
                padata = padata.offset(1)
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"patype\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            pa_type = ap.as_va_list().arg::<krb5_preauthtype>();
            name = padata_type_string(pa_type);
            if !name.is_null() {
                k5_buf_add_fmt(&mut buf as *mut k5buf,
                               b"%s (%d)\x00" as *const u8 as
                                   *const libc::c_char, name, pa_type);
            } else {
                k5_buf_add_fmt(&mut buf as *mut k5buf,
                               b"%d\x00" as *const u8 as *const libc::c_char,
                               pa_type);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"etype\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            etype = ap.as_va_list().arg::<krb5_enctype>();
            if krb5_enctype_to_name(etype, 1 as libc::c_int as krb5_boolean,
                                    tmpbuf.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 200]>()
                                        as libc::c_ulong) == 0 as libc::c_int
               {
                k5_buf_add(&mut buf, tmpbuf.as_mut_ptr());
            } else {
                k5_buf_add_fmt(&mut buf as *mut k5buf,
                               b"%d\x00" as *const u8 as *const libc::c_char,
                               etype);
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"etypes\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            etypes = ap.as_va_list().arg::<*mut krb5_enctype>();
            if etypes.is_null() || *etypes == 0 as libc::c_int {
                k5_buf_add(&mut buf,
                           b"(empty)\x00" as *const u8 as
                               *const libc::c_char);
            }
            while !etypes.is_null() && *etypes != 0 as libc::c_int {
                subfmt(context, &mut buf as *mut k5buf,
                       b"{etype}\x00" as *const u8 as *const libc::c_char,
                       *etypes);
                if *etypes.offset(1 as libc::c_int as isize) !=
                       0 as libc::c_int {
                    k5_buf_add(&mut buf,
                               b", \x00" as *const u8 as *const libc::c_char);
                }
                etypes = etypes.offset(1)
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"ccache\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            ccache =
                ap.as_va_list().arg::<*mut libc::c_void>() as krb5_ccache;
            k5_buf_add(&mut buf, krb5_cc_get_type(context, ccache));
            k5_buf_add(&mut buf,
                       b":\x00" as *const u8 as *const libc::c_char);
            k5_buf_add(&mut buf, krb5_cc_get_name(context, ccache));
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"keytab\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            keytab = ap.as_va_list().arg::<krb5_keytab>();
            if krb5_kt_get_name(context, keytab, tmpbuf.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 200]>()
                                    as libc::c_ulong as libc::c_uint) ==
                   0 as libc::c_int {
                k5_buf_add(&mut buf, tmpbuf.as_mut_ptr());
            }
        } else if strcmp(tmpbuf.as_mut_ptr(),
                         b"creds\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
            creds = ap.as_va_list().arg::<*mut krb5_creds>();
            subfmt(context, &mut buf as *mut k5buf,
                   b"{princ} -> {princ}\x00" as *const u8 as
                       *const libc::c_char, (*creds).client, (*creds).server);
        }
    }
    return buf.data as *mut libc::c_char;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/os/trace.c - krb5int_trace implementation */
/*
 * Copyright 2009 by the Massachusetts Institute of Technology.
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
 * krb5int_trace is defined in k5-trace.h as a macro or static inline
 * function, and is called like so:
 *
 *   void krb5int_trace(krb5_context context, const char *fmt, ...)
 *
 * Arguments may or may not be evaluated, so don't pass argument
 * expressions with side effects.  Tracing support and calls can be
 * explicitly compiled out with DISABLE_TRACING, but compile-time
 * support is enabled by default.  Tracing calls use a custom
 * formatter supporting the following format specifications:
 */
/* Allows trace_format formatters to be represented in terms of other
 * formatters. */
#[c2rust::src_loc = "373:1"]
unsafe extern "C" fn subfmt(mut context: krb5_context, mut buf: *mut k5buf,
                            mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    str = trace_format(context, fmt, ap.as_va_list());
    if !str.is_null() { k5_buf_add(buf, str); }
    free(str as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "387:1"]
pub unsafe extern "C" fn k5_init_trace(mut context: krb5_context) {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    filename =
        secure_getenv(b"KRB5_TRACE\x00" as *const u8 as *const libc::c_char);
    if !filename.is_null() { krb5_set_trace_filename(context, filename); };
}
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
#[c2rust::src_loc = "397:1"]
pub unsafe extern "C" fn krb5int_trace(mut context: krb5_context,
                                       mut fmt: *const libc::c_char,
                                       mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut info: krb5_trace_info =
        krb5_trace_info{message: 0 as *const libc::c_char,};
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sec: krb5_timestamp = 0;
    let mut usec: krb5_int32 = 0;
    if context.is_null() || (*context).trace_callback.is_none() { return }
    ap = args.clone();
    str = trace_format(context, fmt, ap.as_va_list());
    if !str.is_null() {
        if !(krb5_crypto_us_timeofday(&mut sec, &mut usec) !=
                 0 as libc::c_int) {
            if !(asprintf(&mut msg as *mut *mut libc::c_char,
                          b"[%d] %u.%d: %s\n\x00" as *const u8 as
                              *const libc::c_char, getpid(),
                          sec as libc::c_uint, usec, str) < 0 as libc::c_int)
               {
                info.message = msg;
                (*context).trace_callback.expect("non-null function pointer")(context,
                                                                              &mut info,
                                                                              (*context).trace_callback_data);
            }
        }
    }
    free(str as *mut libc::c_void);
    free(msg as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn krb5_set_trace_callback(mut context: krb5_context,
                                                 mut fn_0:
                                                     krb5_trace_callback,
                                                 mut cb_data:
                                                     *mut libc::c_void)
 -> krb5_error_code {
    /* Allow the old callback to destroy its data if necessary. */
    if (*context).trace_callback.is_some() {
        (*context).trace_callback.expect("non-null function pointer")(context,
                                                                      0 as
                                                                          *const krb5_trace_info,
                                                                      (*context).trace_callback_data);
    }
    (*context).trace_callback = fn_0;
    (*context).trace_callback_data = cb_data;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "437:1"]
unsafe extern "C" fn file_trace_cb(mut context: krb5_context,
                                   mut info: *const krb5_trace_info,
                                   mut data: *mut libc::c_void) {
    let mut fd: *mut libc::c_int = data as *mut libc::c_int;
    if info.is_null() {
        /* Null info means destroy the callback data. */
        close(*fd);
        free(fd as *mut libc::c_void);
        return
    }
    write(*fd, (*info).message as *const libc::c_void,
          strlen((*info).message));
}
/* *
 * Set allowable encryption types in initial credential options.
 *
 * @param [in] opt               Options structure
 * @param [in] etype_list        Array of encryption types
 * @param [in] etype_list_length Length of @a etype_list
 */
/* *
 * Set address restrictions in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] addresses        Null-terminated array of addresses
 */
/* *
 * Set preauthentication types in initial credential options.
 *
 * @param [in] opt                 Options structure
 * @param [in] preauth_list        Array of preauthentication types
 * @param [in] preauth_list_length Length of @a preauth_list
 *
 * This function can be used to perform optimistic preauthentication when
 * getting initial credentials, in combination with
 * krb5_get_init_creds_opt_set_salt() and krb5_get_init_creds_opt_set_pa().
 */
/* *
 * Set salt for optimistic preauthentication in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] salt             Salt data
 *
 * When getting initial credentials with a password, a salt string it used to
 * convert the password to a key.  Normally this salt is obtained from the
 * first KDC reply, but when performing optimistic preauthentication, the
 * client may need to supply the salt string with this function.
 */
/* *
 * Set or unset change-password-prompt flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] prompt           Whether to prompt to change password
 *
 * This flag is on by default.  It controls whether
 * krb5_get_init_creds_password() will react to an expired-password error by
 * prompting for a new password and attempting to change the old one.
 */
/* * Generic preauth option attribute/value pairs */
/* *
 * Supply options for preauthentication in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] attr             Preauthentication option name
 * @param [in] value            Preauthentication option value
 *
 * This function allows the caller to supply options for preauthentication.
 * The values of @a attr and @a value are supplied to each preauthentication
 * module available within @a context.
 */
/* *
 * Set location of FAST armor ccache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] fast_ccache_name Credential cache name
 *
 * Sets the location of a credential cache containing an armor ticket to
 * protect an initial credential exchange using the FAST protocol extension.
 *
 * In version 1.7, setting an armor ccache requires that FAST be used for the
 * exchange.  In version 1.8 or later, setting the armor ccache causes FAST to
 * be used if the KDC supports it; krb5_get_init_creds_opt_set_fast_flags()
 * must be used to require that FAST be used.
 */
/* *
 * Set FAST armor cache in initial credential options.
 *
 * @param [in] context           Library context
 * @param [in] opt               Options
 * @param [in] ccache            Credential cache handle
 *
 * This function is similar to krb5_get_init_creds_opt_set_fast_ccache_name(),
 * but uses a credential cache handle instead of a name.
 *
 * @version New in 1.9
 */
/* *
 * Set an input credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an input credential cache is set, then the krb5_get_init_creds family of
 * APIs will read settings from it.  Setting an input ccache is desirable when
 * the application wishes to perform authentication in the same way (using the
 * same preauthentication mechanisms, and making the same non-security-
 * sensitive choices) as the previous authentication attempt, which stored
 * information in the passed-in ccache.
 *
 * @version New in 1.11
 */
/* *
 * Set an output credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an output credential cache is set, then the krb5_get_init_creds family of
 * APIs will write credentials to it.  Setting an output ccache is desirable
 * both because it simplifies calling code and because it permits the
 * krb5_get_init_creds APIs to write out configuration information about the
 * realm to the ccache.
 */
/* *
 * @brief Ask the KDC to include or not include a PAC in the ticket
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] req_pac          Whether to request a PAC or not
 *
 * If this option is set, the AS request will include a PAC-REQUEST pa-data
 * item explicitly asking the KDC to either include or not include a privilege
 * attribute certificate in the ticket authorization data.  By default, no
 * request is made; typically the KDC will default to including a PAC if it
 * supports them.
 *
 * @version New in 1.15
 */
/* *
 * Set FAST flags in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] flags            FAST flags
 *
 * The following flag values are valid:
 * @li #KRB5_FAST_REQUIRED - Require FAST to be used
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
/* *
 * Retrieve FAST flags from initial credential options.
 *
 * @param [in]  context         Library context
 * @param [in]  opt             Options
 * @param [out] out_flags       FAST flags
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
/* Fast flags*/
/* *< Require KDC to support FAST*/
/* *
 * Set an expiration callback in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] cb               Callback function
 * @param [in] data             Callback argument
 *
 * Set a callback to receive password and account expiration times.
 *
 * This option only applies to krb5_get_init_creds_password().  @a cb will be
 * invoked if and only if credentials are successfully acquired.  The callback
 * will receive the @a context from the krb5_get_init_creds_password() call and
 * the @a data argument supplied with this API.  The remaining arguments should
 * be interpreted as follows:
 *
 * If @a is_last_req is true, then the KDC reply contained last-req entries
 * which unambiguously indicated the password expiration, account expiration,
 * or both.  (If either value was not present, the corresponding argument will
 * be 0.)  Furthermore, a non-zero @a password_expiration should be taken as a
 * suggestion from the KDC that a warning be displayed.
 *
 * If @a is_last_req is false, then @a account_expiration will be 0 and @a
 * password_expiration will contain the expiration time of either the password
 * or account, or 0 if no expiration time was indicated in the KDC reply.  The
 * callback should independently decide whether to display a password
 * expiration warning.
 *
 * Note that @a cb may be invoked even if credentials are being acquired for
 * the kadmin/changepw service in order to change the password.  It is the
 * caller's responsibility to avoid displaying a password expiry warning in
 * this case.
 *
 * @warning Setting an expire callback with this API will cause
 * krb5_get_init_creds_password() not to send password expiry warnings to the
 * prompter, as it ordinarily may.
 *
 * @version New in 1.9
 */
/* *
 * Set the responder function in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] responder        Responder function
 * @param [in] data             Responder data argument
 *
 * @version New in 1.11
 */
/* *
 * Get initial credentials using a password.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  password        Password (or NULL)
 * @param [in]  prompter        Prompter function
 * @param [in]  data            Prompter callback data
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using @a
 * password.  If @a password is NULL, a password will be prompted for using @a
 * prompter if necessary.  If @a in_tkt_service is specified, it is parsed as a
 * principal name (with the realm ignored) and used as the service principal
 * for the request; otherwise the ticket-granting service is used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 *  0    Success
 * @retval
 *  EINVAL Invalid argument
 * @retval
 *  KRB5_KDC_UNREACH Cannot contact any KDC for requested realm
 * @retval
 *  KRB5_PREAUTH_FAILED Generic Pre-athentication failure
 * @retval
 *  KRB5_LIBOS_PWDINTR Password read interrupted
 * @retval
 *  KRB5_REALM_CANT_RESOLVE Cannot resolve network address for KDC in requested realm
 * @retval
 *  KRB5KDC_ERR_KEY_EXP Password has expired
 * @retval
 *  KRB5_LIBOS_BADPWDMATCH Password mismatch
 * @retval
 *  KRB5_CHPW_PWDNULL New password cannot be zero length
 * @retval
 *  KRB5_CHPW_FAIL Password change failed
 * @return
 * Kerberos error codes
 */
/* *
 * Retrieve enctype, salt and s2kparams from KDC
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal whose information is requested
 * @param [in]  opt             Initial credential options
 * @param [out] enctype_out     The enctype chosen by KDC
 * @param [out] salt_out        Salt returned from KDC
 * @param [out] s2kparams_out   String-to-key parameters returned from KDC
 *
 * Send an initial ticket request for @a principal and extract the encryption
 * type, salt type, and string-to-key parameters from the KDC response.  If the
 * KDC provides no etype-info, set @a enctype_out to @c ENCTYPE_NULL and set @a
 * salt_out and @a s2kparams_out to empty.  If the KDC etype-info provides no
 * salt, compute the default salt and place it in @a salt_out.  If the KDC
 * etype-info provides no string-to-key parameters, set @a s2kparams_out to
 * empty.
 *
 * @a opt may be used to specify options which affect the initial request, such
 * as request encryption types or a FAST armor cache (see
 * krb5_get_init_creds_opt_set_etype_list() and
 * krb5_get_init_creds_opt_set_fast_ccache_name()).
 *
 * Use krb5_free_data_contents() to free @a salt_out and @a s2kparams_out when
 * they are no longer needed.
 *
 * @version New in 1.17
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
/* *< More responses needed */
/* *
 * Free an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 */
/* *
 * Acquire credentials using an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_init_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_init_creds_get_creds().
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve acquired credentials from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] creds           Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_init_creds_get() or
 * krb5_init_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Get the last error from KDC from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] error           Error from KDC, or NULL if none was received
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Create a context for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal to get initial creds for
 * @param [in]  prompter        Prompter callback
 * @param [in]  data            Prompter callback argument
 * @param [in]  start_time      Time when credentials become valid (0 for now)
 * @param [in]  options         Options structure (NULL for default)
 * @param [out] ctx             New initial credentials context
 *
 * This function creates a new context for acquiring initial credentials.  Use
 * krb5_init_creds_free() to free @a ctx when it is no longer needed.
 *
 * Any subsequent calls to krb5_init_creds_step(), krb5_init_creds_get(), or
 * krb5_init_creds_free() for this initial credentials context must use the
 * same @a context argument as the one passed to this function.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Specify a keytab to use for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] keytab           Key table handle
 *
 * This function supplies a keytab containing the client key for an initial
 * credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Get the next KDC request for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [in]  in              KDC response (empty on the first call)
 * @param [out] out             Next KDC request
 * @param [out] realm           Realm for next KDC request
 * @param [out] flags           Output flags
 *
 * This function constructs the next KDC request in an initial credential
 * exchange, allowing the caller to control the transport of KDC requests and
 * replies.  On the first call, @a in should be set to an empty buffer; on
 * subsequent calls, it should be set to the KDC's reply to the previous
 * request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in
 * @a out.  If no more requests are needed, @a flags will not contain
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the initial credential exchange has failed.
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Set a password for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] password         Password
 *
 * This function supplies a password to be used to construct the client key for
 * an initial credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Specify a service principal for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] service          Service principal string
 *
 * This function supplies a service principal string to acquire initial
 * credentials for instead of the default krbtgt service.  @a service is parsed
 * as a principal name; any realm part is ignored.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve ticket times from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] times           Ticket times for acquired credentials
 *
 * The initial credentials context must have completed obtaining credentials
 * via either krb5_init_creds_get() or krb5_init_creds_step().
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Create a context to get credentials from a KDC's Ticket Granting Service.
 *
 * @param[in]  context          Library context
 * @param[in]  ccache           Credential cache handle
 * @param[in]  creds            Input credentials
 * @param[in]  options          @ref KRB5_GC options for this request.
 * @param[out] ctx              New TGS request context
 *
 * This function prepares to obtain credentials matching @a creds, either by
 * retrieving them from @a ccache or by making requests to ticket-granting
 * services beginning with a ticket-granting ticket for the client principal's
 * realm.
 *
 * The resulting TGS acquisition context can be used asynchronously with
 * krb5_tkt_creds_step() or synchronously with krb5_tkt_creds_get().  See also
 * krb5_get_credentials() for synchronous use.
 *
 * Use krb5_tkt_creds_free() to free @a ctx when it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Synchronously obtain credentials using a TGS request context.
 *
 * @param[in] context           Library context
 * @param[in] ctx               TGS request context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_tkt_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_tkt_creds_get_creds().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve acquired credentials from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] creds            Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_tkt_creds_get() or
 * krb5_tkt_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
/* *< More responses needed */
/* *
 * Get the next KDC request in a TGS exchange.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[in]  in               KDC response (empty on the first call)
 * @param[out] out              Next KDC request
 * @param[out] realm            Realm for next KDC request
 * @param[out] flags            Output flags
 *
 * This function constructs the next KDC request for a TGS exchange, allowing
 * the caller to control the transport of KDC requests and replies.  On the
 * first call, @a in should be set to an empty buffer; on subsequent calls, it
 * should be set to the KDC's reply to the previous request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in @a
 * out.  If no more requests are needed, @a flags will not contain
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the TGS exchange has failed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve ticket times from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] times            Ticket times for acquired credentials
 *
 * The TGS request context must have completed obtaining credentials via either
 * krb5_tkt_creds_get() or krb5_tkt_creds_step().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Get initial credentials using a key table.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  arg_keytab      Key table handle
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using a
 * client key stored in @a arg_keytab.  If @a in_tkt_service is specified, it
 * is parsed as a principal name (with the realm ignored) and used as the
 * service principal for the request; otherwise the ticket-granting service is
 * used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
/* *< boolean */
/* *
 * Initialize a credential verification options structure.
 *
 * @param [in] k5_vic_options   Verification options structure
 */
/* *
 * Set whether credential verification is required.
 *
 * @param [in] k5_vic_options   Verification options structure
 * @param [in] ap_req_nofail    Whether to require successful verification
 *
 * This function determines how krb5_verify_init_creds() behaves if no keytab
 * information is available.  If @a ap_req_nofail is @c FALSE, verification
 * will be skipped in this case and krb5_verify_init_creds() will return
 * successfully.  If @a ap_req_nofail is @c TRUE, krb5_verify_init_creds() will
 * not return successfully unless verification can be performed.
 *
 * If this function is not used, the behavior of krb5_verify_init_creds() is
 * determined through configuration.
 */
/* *
 * Verify initial credentials against a keytab.
 *
 * @param [in] context          Library context
 * @param [in] creds            Initial credentials to be verified
 * @param [in] server           Server principal (or NULL)
 * @param [in] keytab           Key table (NULL to use default keytab)
 * @param [in] ccache           Credential cache for fetched creds (or NULL)
 * @param [in] options          Verification options (NULL for default options)
 *
 * This function attempts to verify that @a creds were obtained from a KDC with
 * knowledge of a key in @a keytab, or the default keytab if @a keytab is NULL.
 * If @a server is provided, the highest-kvno key entry for that principal name
 * is used to verify the credentials; otherwise, all unique "host" service
 * principals in the keytab are tried.
 *
 * If the specified keytab does not exist, or is empty, or cannot be read, or
 * does not contain an entry for @a server, then credential verification may be
 * skipped unless configuration demands that it succeed.  The caller can
 * control this behavior by providing a verification options structure; see
 * krb5_verify_init_creds_opt_init() and
 * krb5_verify_init_creds_opt_set_ap_req_nofail().
 *
 * If @a ccache is NULL, any additional credentials fetched during the
 * verification process will be destroyed.  If @a ccache points to NULL, a
 * memory ccache will be created for the additional credentials and returned in
 * @a ccache.  If @a ccache points to a valid credential cache handle, the
 * additional credentials will be stored in that cache.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Get validated credentials from the KDC.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Validated credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a validated credential using a postdated credential from
 * @a ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential;
 * otherwise, the ticket-granting service is used.
 *
 * If successful, the validated credential is placed in @a creds.
 *
 * @sa krb5_get_renewed_creds()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_NO_2ND_TKT Request missing second ticket
 * @retval
 * KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 * KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 * KRB5_KDCREP_MODIFIED KDC reply did not match expectations
 * @retval
 * KRB5_KDCREP_SKEW Clock skew too great in KDC reply
 * @return
 * Kerberos error codes
 */
/* *
 * Get renewed credential from KDC using an existing credential.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Renewed credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a renewed credential using an existing one from @a
 * ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential; otherwise,
 * the ticket-granting service is used.
 *
 * If successful, the renewed credential is placed in @a creds.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
/* *
 * Decode an ASN.1-formatted ticket.
 *
 * @param [in]  code            ASN.1-formatted ticket
 * @param [out] rep             Decoded ticket information
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
/* *
 * Retrieve a string value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       String value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_boolean()
 */
/* *
 * Retrieve a boolean value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       Boolean value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_string()
 */
/*
 * Prompter enhancements
 */
/* * Prompt for password */
/* * Prompt for new password (during password change) */
/* * Prompt for new password again */
/* * Prompt for preauthentication data (such as an OTP value) */
/* *
 * Get prompt types array from a context.
 *
 * @param [in] context          Library context
 *
 * @return
 * Pointer to an array of prompt types corresponding to the prompter's @a
 * prompts arguments.  Each type has one of the following values:
 *  @li #KRB5_PROMPT_TYPE_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN
 *  @li #KRB5_PROMPT_TYPE_PREAUTH
 */
/* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
/* *
 * Set an extended error message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] args             List of vprintf(3) style arguments
 */
/* *
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
/* *
 * Add a prefix to the message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_prepend_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
/* *
 * Add a prefix to a different error code's message.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the message for @a old_code.  The prefix
 * will be separated from the old message with a colon and space.  Set the
 * resulting message as the extended error message for @a code.
 */
/* *
 * Add a prefix to a different error code's message using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_wrap_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
/* *
 * Copy the most recent extended error message from one context to another.
 *
 * @param [in] dest_ctx         Library context to copy message to
 * @param [in] src_ctx          Library context with current message
 */
/* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
/* *
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
/* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
/* *
 * Unwrap authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  container       Authorization data to be decoded
 * @param [out] authdata        List of decoded authorization data
 *
 * @sa krb5_encode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Wrap authorization data in a container.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  authdata        List of authorization data to be encoded
 * @param [out] container       List of encoded authorization data
 *
 * The result is returned in @a container as a single-element list.
 *
 * @sa krb5_decode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/*
 * AD-KDCIssued
 */
/* *
 * Encode and sign AD-KDCIssued authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Session key
 * @param [in]  issuer          The name of the issuing principal
 * @param [in]  authdata        List of authorization data to be signed
 * @param [out] ad_kdcissued    List containing AD-KDCIssued authdata
 *
 * This function wraps a list of authorization data entries @a authdata in an
 * AD-KDCIssued container (see RFC 4120 section 5.2.6.2) signed with @a key.
 * The result is returned in @a ad_kdcissued as a single-element list.
 */
/* *
 * Unwrap and verify AD-KDCIssued authorization data.
 *
 * @param [in] context          Library context
 * @param [in] key              Session key
 * @param [in] ad_kdcissued     AD-KDCIssued authorization data to be unwrapped
 * @param [out] issuer          Name of issuing principal (or NULL)
 * @param [out] authdata        Unwrapped list of authorization data
 *
 * This function unwraps an AD-KDCIssued authdatum (see RFC 4120 section
 * 5.2.6.2) and verifies its signature against @a key.  The issuer field of the
 * authdatum element is returned in @a issuer, and the unwrapped list of
 * authdata is returned in @a authdata.
 */
/*
 * Windows PAC
 */
/* Microsoft defined types of data */
/* *< Logon information */
/* *< Credentials information */
/* *< Server checksum */
/* *< KDC checksum */
/* *< Client name and ticket info */
/* *< Constrained delegation info */
/* *< User principal name and DNS info */
/* * PAC data structure to convey authorization information */
/* *
 * Add a buffer to a PAC handle.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] type             Buffer type
 * @param [in] data             contents
 *
 * This function adds a buffer of type @a type and contents @a data to @a pac
 * if there isn't already a buffer of this type present.
 *
 * The valid values of @a type is one of the following:
 * @li #KRB5_PAC_LOGON_INFO         -  Logon information
 * @li #KRB5_PAC_CREDENTIALS_INFO   -  Credentials information
 * @li #KRB5_PAC_SERVER_CHECKSUM    -  Server checksum
 * @li #KRB5_PAC_PRIVSVR_CHECKSUM   -  KDC checksum
 * @li #KRB5_PAC_CLIENT_INFO        -  Client name and ticket information
 * @li #KRB5_PAC_DELEGATION_INFO    -  Constrained delegation information
 * @li #KRB5_PAC_UPN_DNS_INFO       -  User principal name and DNS information
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Free a PAC handle.
 *
 * @param [in] context         Library context
 * @param [in] pac             PAC to be freed
 *
 * This function frees the contents of @a pac and the structure itself.
 */
/* *
 * Retrieve a buffer value from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  type            Type of buffer to retrieve
 * @param [out] data            Buffer value
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Return an array of buffer types in a PAC handle.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] len             Number of entries in @a types
 * @param [out] types           Array of buffer types
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Create an empty Privilege Attribute Certificate (PAC) handle.
 *
 * @param [in]  context         Library context
 * @param [out] pac             New PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Unparse an encoded PAC into a new handle.
 *
 * @param [in]  context         Library context
 * @param [in]  ptr             PAC buffer
 * @param [in]  len             Length of @a ptr
 * @param [out] pac             PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Verify a PAC.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 *
 * This function validates @a pac against the supplied @a server, @a privsvr,
 * @a principal and @a authtime.  If @a principal is NULL, the principal and
 * authtime are not verified.  If @a server or @a privsvr is NULL, the
 * corresponding checksum is not verified.
 *
 * If successful, @a pac is marked as verified.
 *
 * @note A checksum mismatch can occur if the PAC was copied from a cross-realm
 * TGT by an ignorant KDC; also macOS Server Open Directory (as of 10.6)
 * generates PACs with no server checksum at all.  One should consider not
 * failing the whole authentication because of this reason, but, instead,
 * treating the ticket as if it did not contain a PAC or marking the PAC
 * information as non-verified.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
/* *
 * Verify a PAC, possibly from a specified realm.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 * @param [in] with_realm       If true, expect the realm of @a principal
 *
 * This function is similar to krb5_pac_verify(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field is
 * expected to include the realm of @a principal as well as the name.  This
 * flag is necessary to verify PACs in cross-realm S4U2Self referral TGTs.
 *
 * @version New in 1.17
 */
/* *
 * Sign a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Expected principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [out] data            Signed PAC encoding
 *
 * This function signs @a pac using the keys @a server_key and @a privsvr_key
 * and returns the signed encoding in @a data.  @a pac is modified to include
 * the server and KDC checksum buffers.  Use krb5_free_data_contents() to free
 * @a data when it is no longer needed.
 *
 * @version New in 1.10
 */
/* *
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
/*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
/* *
 * Allow the appplication to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
/* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
/* *
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
/* *
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
#[no_mangle]
#[c2rust::src_loc = "452:1"]
pub unsafe extern "C" fn krb5_set_trace_filename(mut context: krb5_context,
                                                 mut filename:
                                                     *const libc::c_char)
 -> krb5_error_code {
    let mut fd: *mut libc::c_int = 0 as *mut libc::c_int;
    /* Create callback data containing a file descriptor. */
    fd =
        malloc(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) as
            *mut libc::c_int;
    if fd.is_null() { return 12 as libc::c_int }
    *fd =
        open(filename,
             0o1 as libc::c_int | 0o100 as libc::c_int |
                 0o2000 as libc::c_int, 0o600 as libc::c_int);
    if *fd == -(1 as libc::c_int) {
        free(fd as *mut libc::c_void);
        return *__errno_location()
    }
    return krb5_set_trace_callback(context,
                                   Some(file_trace_cb as
                                            unsafe extern "C" fn(_:
                                                                     krb5_context,
                                                                 _:
                                                                     *const krb5_trace_info,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> ()),
                                   fd as *mut libc::c_void);
}
/* DISABLE_TRACING */
/* DISABLE_TRACING */
