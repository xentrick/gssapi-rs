use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:28"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:28"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/unistd.h:28"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    }
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
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
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
    #[c2rust::src_loc = "2246:16"]
    pub struct krb5_replay_data {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq: krb5_ui_4,
    }
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
        /* *
 * Return the type of a key table.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @return The type of a key table as an alias that must not be modified or
 * freed by the caller.
 */
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
        /* *
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
        /* *
 * Get an entry from a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [in]  principal       Principal name
 * @param [in]  vno             Key version number (0 for highest available)
 * @param [in]  enctype         Encryption type (0 zero for any enctype)
 * @param [out] entry           Returned entry from key table
 *
 * Retrieve an entry from a key table which matches the @a keytab, @a
 * principal, @a vno, and @a enctype.  If @a vno is zero, retrieve the
 * highest-numbered kvno matching the other fields.  If @a enctype is 0, match
 * any enctype.
 *
 * Use krb5_free_keytab_entry_contents() to free @a entry when it is no longer
 * needed.
 *
 * @note If @a vno is zero, the function retrieves the highest-numbered-kvno
 * entry that matches the specified principal.
 *
 * @retval
 * 0 Success
 * @retval
 * Kerberos error codes on failure
 */
        /* *
 * Start a sequential retrieval of key table entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * Prepare to read sequentially every key in the specified key table.  Use
 * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
 *
 * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Retrieve the next entry from the key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] entry           Returned key table entry
 * @param [in]  cursor          Key table cursor
 *
 * Return the next sequential entry in @a keytab and advance @a cursor.
 * Callers must release the returned entry with krb5_kt_free_entry().
 *
 * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_END - if the last entry was reached
 * @return
 * Kerberos error codes
 */
        /* *
 * Release a keytab cursor.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * This function should be called to release the cursor created by
 * krb5_kt_start_seq_get().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Check if a keytab exists and contains entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 *
 * @version New in 1.11
 *
 * @retval 0 Keytab exists and contains entries
 * @retval KRB5_KT_NOTFOUND Keytab does not contain entries
 */
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Create a krb5 library context using only configuration files.
 *
 * @param [out] context         Library context
 *
 * Create a context structure, using only system configuration files.  All
 * information passed through the environment variables is ignored.
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Create a krb5 library context using a specified profile.
 *
 * @param [in]  profile         Profile object (NULL to create default profile)
 * @param [in]  flags           Context initialization flags
 * @param [out] context         Library context
 *
 * Create a context structure, optionally using a specified profile and
 * initialization flags.  If @a profile is NULL, the default profile will be
 * created from config files.  If @a profile is non-null, a copy of it will be
 * made for the new context; the caller should still clean up its copy.  Valid
 * flag values are:
 *
 * @li #KRB5_INIT_CONTEXT_SECURE Ignore environment variables
 * @li #KRB5_INIT_CONTEXT_KDC    Use KDC configuration if creating profile
 */
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        /* *
 * Copy a krb5_context structure.
 *
 * @param [in]  ctx             Library context
 * @param [out] nctx_out        New context structure
 *
 * The newly created context must be released by calling krb5_free_context()
 * when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Set default TGS encryption types in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] etypes           Encryption type(s) to set
 *
 * This function sets the default enctype list for TGS requests
 * made using @a context to @a etypes.
 *
 * @note This overrides the default list (from config file or built-in).
 *
 * @retval
 *  0    Success
 * @retval
 *  KRB5_PROG_ETYPE_NOSUPP Program lacks support for encryption type
 * @return
 * Kerberos error codes
 */
        /* *
 * Return a list of encryption types permitted for session keys.
 *
 * @param [in]  context         Library context
 * @param [out] ktypes          Zero-terminated list of encryption types
 *
 * This function returns the list of encryption types permitted for session
 * keys within @a context, as determined by configuration or by a previous call
 * to krb5_set_default_tgs_enctypes().
 *
 * Use krb5_free_enctypes() to free @a ktypes when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Test whether the Kerberos library was built with multithread support.
 *
 * @retval
 * TRUE if the library is threadsafe; FALSE otherwise
 */
        /* libkrb.spec */
        /* *
 * Decrypt a ticket using the specified key table.
 *
 * @param [in] context          Library context
 * @param [in] kt               Key table
 * @param [in] ticket           Ticket to be decrypted
 *
 * This function takes a @a ticket as input and decrypts it using
 * key data from @a kt.  The result is placed into @a ticket->enc_part2.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Free an array of credential structures.
 *
 * @param [in] context          Library context
 * @param [in] tgts             Null-terminated array of credentials to free
 *
 * @note The last entry in the array @a tgts must be a NULL pointer.
 */
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
        /* * @deprecated Replaced by krb5_get_validated_creds. */
        /* * @deprecated Replaced by krb5_get_renewed_creds. */
        /* *
 * Create a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     service        Service name, or NULL to use @c "host"
 * @param [in]     hostname       Host name, or NULL to use local hostname
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     ccache         Credential cache used to obtain credentials
 *                                for the desired service.
 * @param [out]    outbuf         @c AP-REQ message
 *
 * This function is similar to krb5_mk_req_extended() except that it uses a
 * given @a hostname, @a service, and @a ccache to construct a service
 * principal name and obtain credentials.
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
        /* *
 * Format and encrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * This function fills in @a outbuf with an AP-REP message using information
 * from @a auth_context.
 *
 * If the flags in @a auth_context indicate that a sequence number should be
 * used (either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or
 * #KRB5_AUTH_CONTEXT_RET_SEQUENCE) and the local sequence number in @a
 * auth_context is 0, a new number will be generated with
 * krb5_generate_seq_number().
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format and encrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Parse and decrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] repl            Decrypted reply message
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a repl with a pointer to allocated memory containing the fields
 * from the encrypted response.
 *
 * Use krb5_free_ap_rep_enc_part() to free @a repl when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Parse and decrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] nonce           Sequence number from the decrypted reply
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a nonce with a decrypted reply sequence number.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Format and encode a @c KRB_ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  dec_err         Error structure to be encoded
 * @param [out] enc_err         Encoded error structure
 *
 * This function creates a @c KRB_ERROR message in @a enc_err.  Use
 * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Decode a @c KRB-ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  enc_errbuf      Encoded error message
 * @param [out] dec_error       Decoded error message
 *
 * This function processes @c KRB-ERROR message @a enc_errbuf and returns
 * an allocated structure @a dec_error containing the error message.
 * Use krb5_free_error() to free @a dec_error when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Process @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           @c KRB-SAFE message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-SAFE message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-SAFE message, verifies its integrity, and
 * stores its data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-SAFE message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-SAFE message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-SAFE message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Process a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication structure
 * @param [in]  inbuf           @c KRB-PRIV message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-PRIV message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-PRIV message, verifies its integrity, and
 * stores its unencrypted data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-PRIV message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-PRIV message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-PRIV message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        /* *< Error if realm is present */
        /* *< Error if realm is not present */
        /* *< Create single-component
                                                  enterprise principle */
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
        /* *
 * Convert krb5_principal structure to string and length.
 *
 * @param [in]     context      Library context
 * @param [in]     principal    Principal
 * @param [in,out] name         String representation of principal name
 * @param [in,out] size         Size of unparsed name
 *
 * This function is similar to krb5_unparse_name(), but allows the use of an
 * existing buffer for the result.  If size is not NULL, then @a name must
 * point to either NULL or an existing buffer of at least the size pointed to
 * by @a size.  The buffer will be allocated or resized if necessary, with the
 * new pointer stored into @a name.  Whether or not the buffer is resized, the
 * necessary space for the result, including null terminator, will be stored
 * into @a size.
 *
 * If size is NULL, this function behaves exactly as krb5_unparse_name().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        /* *< Omit realm if it is the local realm */
        /* *< Omit realm always */
        /* *< Don't escape special characters */
        /* *
 * Convert krb5_principal structure to a string with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            String representation of principal name
 *
 * Similar to krb5_unparse_name(), this function converts a krb5_principal
 * structure to a string representation.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_UNPARSE_SHORT - omit realm if it is the local realm
 * @li #KRB5_PRINCIPAL_UNPARSE_NO_REALM - omit realm
 * @li #KRB5_PRINCIPAL_UNPARSE_DISPLAY - do not quote special characters
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        /* *
 * Convert krb5_principal structure to string format with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            Single string format of principal name
 * @param [out] size            Size of unparsed name buffer
 *
 * @sa krb5_unparse_name() krb5_unparse_name_flags() krb5_unparse_name_ext()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        /* *
 * Set the realm field of a principal
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] realm            Realm name
 *
 * Set the realm name part of @a principal to @a realm, overwriting the
 * previous realm.
 *
 * @retval
 * 0   Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Search a list of addresses for a specified address.
 *
 * @param [in] context          Library context
 * @param [in] addr             Address to search for
 * @param [in] addrlist         Address list to be searched (or NULL)
 *
 * @note If @a addrlist contains only a NetBIOS addresses, it will be treated
 *       as a null list.
 *
 * @return
 * TRUE if @a addr is listed in @a addrlist, or @c addrlist is NULL; FALSE
 * otherwise
 */
        /* *
 * Compare two Kerberos addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address to be compared
 * @param [in] addr2            Second address to be compared
 *
 * @return
 * TRUE if the addresses are the same, FALSE otherwise
 */
        /* *
 * Return an ordering of the specified addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address
 * @param [in] addr2            Second address
 *
 * @retval
 *  0 The two addresses are the same
 * @retval
 *  \< 0 First address is less than second
 * @retval
 *  \> 0 First address is greater than second
 */
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
        /* *< ignore realm component */
        /* *< UPNs as real principals */
        /* *< case-insensitive */
        /* *< treat principals as UTF-8 */
        /* *
 * Compare two principals with additional flags.
 *
 * @param [in] context           Library context
 * @param [in] princ1            First principal
 * @param [in] princ2            Second principal
 * @param [in] flags             Flags
 *
 * Valid flags are:
 * @li #KRB5_PRINCIPAL_COMPARE_IGNORE_REALM - ignore realm component
 * @li #KRB5_PRINCIPAL_COMPARE_ENTERPRISE - UPNs as real principals
 * @li #KRB5_PRINCIPAL_COMPARE_CASEFOLD case-insensitive
 * @li #KRB5_PRINCIPAL_COMPARE_UTF8 - treat principals as UTF-8
 *
 * @sa krb5_principal_compare()
 *
 * @retval
 * TRUE if the principal names are the same; FALSE otherwise
 */
        /* *
 * Initialize an empty @c krb5_keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  length          Length of keyblock (or 0)
 * @param [out] out             New keyblock structure
 *
 * Initialize a new keyblock and allocate storage for the contents of the key.
 * It is legal to pass in a length of 0, in which case contents are left
 * unallocated.  Use krb5_free_keyblock() to free @a out when it is no longer
 * needed.
 *
 * @note If @a length is set to 0, contents are left unallocated.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Keyblock to be copied
 * @param [out] to              Copy of keyblock @a from
 *
 * This function creates a new keyblock with the same contents as @a from.  Use
 * krb5_free_keyblock() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
        /* *
 * Copy a krb5_creds structure.
 *
 * @param [in]  context         Library context
 * @param [in]  incred          Credentials structure to be copied
 * @param [out] outcred         Copy of @a incred
 *
 * This function creates a new credential with the contents of @a incred.  Use
 * krb5_free_creds() to free @a outcred when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
        /* *
 * Copy a krb5_ticket structure.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Ticket to be copied
 * @param [out] pto             Copy of ticket
 *
 * This function creates a new krb5_ticket structure containing the contents of
 * @a from.  Use krb5_free_ticket() to free @a pto when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy an authorization data list.
 *
 * @param [in]  context         Library context
 * @param [in]  in_authdat      List of @a krb5_authdata structures
 * @param [out] out             New array of @a krb5_authdata structures
 *
 * This function creates a new authorization data list containing a copy of @a
 * in_authdat, which must be null-terminated.  Use krb5_free_authdata() to free
 * @a out when it is no longer needed.
 *
 * @note The last array entry in @a in_authdat must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
        /* *
 * Merge two authorization data lists into a new list.
 *
 * @param [in]  context         Library context
 * @param [in]  inauthdat1      First list of @a krb5_authdata structures
 * @param [in]  inauthdat2      Second list of @a krb5_authdata structures
 * @param [out] outauthdat      Merged list of @a krb5_authdata structures
 *
 * Merge two authdata arrays, such as the array from a ticket
 * and authenticator.
 * Use krb5_free_authdata() to free @a outauthdat when it is no longer needed.
 *
 * @note The last array entry in @a inauthdat1 and @a inauthdat2
 * must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_authenticator structure.
 *
 * @param [in]  context         Library context
 * @param [in]  authfrom        krb5_authenticator structure to be copied
 * @param [out] authto          Copy of krb5_authenticator structure
 *
 * This function creates a new krb5_authenticator structure with the content of
 * @a authfrom.  Use krb5_free_authenticator() to free @a authto when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Copy a krb5_checksum structure.
 *
 * @param [in]  context         Library context
 * @param [in]  ckfrom          Checksum to be copied
 * @param [out] ckto            Copy of krb5_checksum structure
 *
 * This function creates a new krb5_checksum structure with the contents of @a
 * ckfrom.  Use krb5_free_checksum() to free @a ckto when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Generate a replay cache object for server use and open it.
 *
 * @param [in]  context         Library context
 * @param [in]  piece           Unused (replay cache identifier)
 * @param [out] rcptr           Handle to an open rcache
 *
 * This function creates a handle to the default replay cache.  Use
 * krb5_rc_close() to close @a rcptr when it is no longer needed.
 *
 * @version Prior to release 1.18, this function creates a handle to a
 * different replay cache for each unique value of @a piece.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
        /* *
 * Build a principal name using null-terminated strings.
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal name
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ...             List of char * components, ending with NULL
 *
 * Call krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
 * same task.  krb5_build_principal() takes variadic arguments.
 * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
 *
 * @code
 * Example of how to build principal H/S@R
 *     krb5_build_principal(context, &principal,
 *                          strlen("R"), "R", "H", "S", (char*)NULL);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* * @deprecated Replaced by krb5_build_principal_alloc_va(). */
        /* *
 * Build a principal name, using a precomputed variable argument list
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal structure
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ap              List of char * components, ending with NULL
 *
 * Similar to krb5_build_principal(), this function builds a principal name,
 * but its name components are specified as a va_list.
 *
 * Use krb5_free_principal() to deallocate @a princ when it is no longer
 * needed.
 *
 * @code
 * Function usage example:
 *   va_list ap;
 *   va_start(ap, realm);
 *   krb5_build_principal_alloc_va(context, princ, rlen, realm, ap);
 *   va_end(ap);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert a Kerberos V4 principal to a Kerberos V5 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  name            V4 name
 * @param [in]  instance        V4 instance
 * @param [in]  realm           Realm
 * @param [out] princ           V5 principal
 *
 * This function builds a @a princ from V4 specification based on given input
 * @a name.instance\@realm.
 *
 * Use krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* *
 * Convert a Kerberos V5 principal to a Kerberos V4 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  princ           V5 Principal
 * @param [out] name            V4 principal's name to be filled in
 * @param [out] inst            V4 principal's instance name to be filled in
 * @param [out] realm           Principal's realm name to be filled in
 *
 * This function separates a V5 principal @a princ into @a name, @a instance,
 * and @a realm.
 *
 * @retval
 *  0  Success
 * @retval
 *  KRB5_INVALID_PRINCIPAL   Invalid principal name
 * @retval
 *  KRB5_CONFIG_CANTOPEN     Can't open or find Kerberos configuration file
 * @return
 * Kerberos error codes
 */
        /* *
 *@deprecated
 */
        /* *
 * Convert a Kerberos V5 credentials to a Kerberos V4 credentials
 *
 * @note Not implemented
 *
 * @retval KRB524_KRB4_DISABLED (always)
 */
        /* libkt.spec */
        /* *
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Duplicate keytab handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Key table handle to be duplicated
 * @param [out] out             Key table handle
 *
 * Create a new handle referring to the same key table as @a in.  The new
 * handle and @a in can be closed independently.
 *
 * @version New in 1.12
 */
        /* *
 * Get the default key table name.
 *
 * @param [in]     context      Library context
 * @param [out]    name         Default key table name
 * @param [in]     name_size    Space available in @a name
 *
 * Fill @a name with the name of the default key table for @a context.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_CONFIG_NOTENUFSPACE Buffer is too short
 * @return
 * Kerberos error codes
 */
        /* *
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Resolve the default client key table.
 *
 * @param [in]     context      Library context
 * @param [out]    keytab_out   Key table handle
 *
 * Fill @a keytab_out with a handle to the default client key table.
 *
 * @version New in 1.11
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Free the contents of a key table entry.
 *
 * @param [in] context          Library context
 * @param [in] entry            Key table entry whose contents are to be freed
 *
 * @note The pointer is not freed.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        /* * @deprecated Use krb5_free_keytab_entry_contents instead. */
        /* remove and add are functions, so that they can return NOWRITE
   if not a writable keytab */
        /* *
 * Remove an entry from a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to remove from key table
 *
 * @retval
 * 0 Success
 * @retval
 *  KRB5_KT_NOWRITE     Key table is not writable
 * @return
 * Kerberos error codes
 */
        /* *
 * Add a new entry to a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to be added
 *
 * @retval
 * 0  Success
 * @retval
 *  ENOMEM    Insufficient memory
 * @retval
 *  KRB5_KT_NOWRITE  Key table is not writeable
 * @return
 * Kerberos error codes
 */
        /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        /* librc.spec--see rcache.h */
        /* libcc.spec */
        /* *
 * Resolve a credential cache name.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Credential cache name to be resolved
 * @param [out] cache           Credential cache handle
 *
 * Fills in @a cache with a @a cache handle that corresponds to the name in @a
 * name.  @a name should be of the form @c type:residual, and @a type must be a
 * type known to the library.  If the @a name does not contain a colon,
 * interpret it as a file name.
 *
 * @code
 * Example: krb5_cc_resolve(context, "MEMORY:C_", &cache);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Duplicate ccache handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Credential cache handle to be duplicated
 * @param [out] out             Credential cache handle
 *
 * Create a new handle referring to the same cache as @a in.
 * The new handle and @a in can be closed independently.
 */
        /* *
 * Return the name of the default credential cache.
 *
 * @param [in] context          Library context
 *
 * Return a pointer to the default credential cache name for @a context, as
 * determined by a prior call to krb5_cc_set_default_name(), by the KRB5CCNAME
 * environment variable, by the default_ccache_name profile variable, or by the
 * operating system or build-time default value.  The returned value must not
 * be modified or freed by the caller.  The returned value becomes invalid when
 * @a context is destroyed krb5_free_context() or if a subsequent call to
 * krb5_cc_set_default_name() is made on @a context.
 *
 * The default credential cache name is cached in @a context between calls to
 * this function, so if the value of KRB5CCNAME changes in the process
 * environment after the first call to this function on, that change will not
 * be reflected in later calls with the same context.  The caller can invoke
 * krb5_cc_set_default_name() with a NULL value of @a name to clear the cached
 * value and force the default name to be recomputed.
 *
 * @return
 * Name of default credential cache for the current user.
 */
        /* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
        /* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
        /* *
 * Copy a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  incc            Credential cache to be copied
 * @param [out] outcc           Copy of credential cache to be filled in
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        /* *
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        /* *
 * Test whether a principal is a configuration principal.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal to check
 *
 * @return
 * @c TRUE if the principal is a configuration principal (generated part of
 * krb5_cc_set_config()); @c FALSE otherwise.
 */
        /* *
 * Make a credential cache the primary cache for its collection.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * If the type of @a cache supports it, set @a cache to be the primary
 * credential cache for the collection it belongs to.
 *
 * @retval
 * 0  Success, or the type of @a cache doesn't support switching
 * @return
 * Kerberos error codes
 */
        /* *
 * Determine whether a credential cache type supports switching.
 *
 * @param [in] context          Library context
 * @param [in] type             Credential cache type
 *
 * @version New in 1.10
 *
 * @retval TRUE if @a type supports switching
 * @retval FALSE if it does not or is not a valid credential cache type.
 */
        /* *
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
        /* *
 * Select a credential cache to use with a server principal.
 *
 * @param [in]  context         Library context
 * @param [in]  server          Server principal
 * @param [out] cache_out       Credential cache handle
 * @param [out] princ_out       Client principal
 *
 * Select a cache within the collection containing credentials most appropriate
 * for use with @a server, according to configured rules and heuristics.
 *
 * Use krb5_cc_close() to release @a cache_out when it is no longer needed.
 * Use krb5_free_principal() to release @a princ_out when it is no longer
 * needed.  Note that @a princ_out is set in some error conditions.
 *
 * @return
 * If an appropriate cache is found, 0 is returned, @a cache_out is set to the
 * selected cache, and @a princ_out is set to the default principal of that
 * cache.
 *
 * If the appropriate client principal can be authoritatively determined but
 * the cache collection contains no credentials for that principal, then
 * KRB5_CC_NOTFOUND is returned, @a cache_out is set to NULL, and @a princ_out
 * is set to the appropriate client principal.
 *
 * If no configured mechanism can determine the appropriate cache or principal,
 * KRB5_CC_NOTFOUND is returned and @a cache_out and @a princ_out are set to
 * NULL.
 *
 * Any other error code indicates a fatal error in the processing of a cache
 * selection mechanism.
 *
 * @version New in 1.10
 */
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        /* *
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
        /* *
 * Free the storage assigned to array of authentication data.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of authentication data to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
        /* *
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        /* *
 * Free a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        /* *
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        /* *
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        /* Free a krb5_octet_data structure (should be unused). */
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
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "3119:1"]
        pub fn krb5_get_credentials(context: krb5_context,
                                    options: krb5_flags, ccache: krb5_ccache,
                                    in_creds: *mut krb5_creds,
                                    out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3201:1"]
        pub fn krb5_mk_req_extended(context: krb5_context,
                                    auth_context: *mut krb5_auth_context,
                                    ap_req_options: krb5_flags,
                                    in_data: *mut krb5_data,
                                    in_creds: *mut krb5_creds,
                                    outbuf: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4666:1"]
        pub fn krb5_free_creds(context: krb5_context, val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "5293:1"]
        pub fn krb5_mk_safe(context: krb5_context,
                            auth_context: krb5_auth_context,
                            userdata: *const krb5_data,
                            der_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5613:1"]
        pub fn krb5_auth_con_init(context: krb5_context,
                                  auth_context: *mut krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5644:1"]
        pub fn krb5_auth_con_setflags(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      flags: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6222:1"]
        pub fn krb5_auth_con_genaddrs(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      infd: libc::c_int, flags: libc::c_int)
         -> krb5_error_code;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_pointer, krb5_data,
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
        #[c2rust::src_loc = "2215:1"]
        pub fn krb5_write_message(_: krb5_context, _: krb5_pointer,
                                  _: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2214:1"]
        pub fn krb5_read_message(_: krb5_context, _: krb5_pointer,
                                 _: *mut krb5_data) -> krb5_error_code;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:28"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:28"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:28"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:28"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:28"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:9"]
    pub union __SOCKADDR_ARG {
        pub __sockaddr__: *mut sockaddr,
        pub __sockaddr_at__: *mut sockaddr_at,
        pub __sockaddr_ax25__: *mut sockaddr_ax25,
        pub __sockaddr_dl__: *mut sockaddr_dl,
        pub __sockaddr_eon__: *mut sockaddr_eon,
        pub __sockaddr_in__: *mut sockaddr_in,
        pub __sockaddr_in6__: *mut sockaddr_in6,
        pub __sockaddr_inarp__: *mut sockaddr_inarp,
        pub __sockaddr_ipx__: *mut sockaddr_ipx,
        pub __sockaddr_iso__: *mut sockaddr_iso,
        pub __sockaddr_ns__: *mut sockaddr_ns,
        pub __sockaddr_un__: *mut sockaddr_un,
        pub __sockaddr_x25__: *mut sockaddr_x25,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_un;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                      __addr_len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:28"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:28"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "255:8"]
    pub struct servent {
        pub s_name: *mut libc::c_char,
        pub s_aliases: *mut *mut libc::c_char,
        pub s_port: libc::c_int,
        pub s_proto: *mut libc::c_char,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "288:1"]
        pub fn getservbyname(__name: *const libc::c_char,
                             __proto: *const libc::c_char) -> *mut servent;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:28"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:28"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t, __off_t,
                        __off64_t, __socklen_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::unistd_h::{socklen_t, close, dup2};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       _krb5_address, krb5_address, krb5_post_recv_fn,
                       krb5_context, krb5_pre_send_fn, krb5_trace_callback,
                       krb5_trace_info, _krb5_trace_info, krb5_prompt_type,
                       krb5_auth_context, _krb5_keyblock, krb5_keyblock,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds,
                       krb5_replay_data, krb5_ccache, _profile_t,
                       _krb5_auth_context, _krb5_ccache, krb5_cc_close,
                       krb5_cc_get_principal, krb5_free_data_contents,
                       krb5_init_context, krb5_free_context,
                       krb5_get_credentials, krb5_mk_req_extended,
                       krb5_parse_name, krb5_cc_default, krb5_free_creds,
                       krb5_free_cred_contents, krb5_mk_safe,
                       krb5_auth_con_init, krb5_auth_con_free,
                       krb5_auth_con_setflags, krb5_auth_con_genaddrs};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_write_message, krb5_read_message};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, getsockname,
                             setsockopt, listen, accept};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, htons};
pub use self::netdb_h::{servent, getservbyname};
use self::stdlib_h::{atoi, free, exit};
use self::stdio_h::{stdout, fflush, printf};
use self::errno_h::__errno_location;
use self::string_h::memset;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* appl/user_user/server.c - One end of user-user client-server pair */
/*
 * Copyright 1991 by the Massachusetts Institute of Technology.
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
/* fd 0 is a tcp socket used to talk to the client */
#[c2rust::src_loc = "42:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut pname_data: krb5_data =
        krb5_data{magic: 0,
                  length: 0,
                  data: 0 as *mut libc::c_char,}; /* local, foreign address */
    let mut tkt_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut sock: libc::c_int = 0 as libc::c_int;
    let mut l: socklen_t = 0;
    let mut retval: libc::c_int = 0;
    let mut l_inaddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut f_inaddr: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
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
    let mut new_creds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut cc: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut msgtext: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut msg: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut auth_context: krb5_auth_context = 0 as krb5_auth_context;
    retval = krb5_init_context(&mut context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while initializing krb5\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    let mut one: libc::c_int = 1 as libc::c_int;
    let mut acc: libc::c_int = 0;
    let mut sp: *mut servent = 0 as *mut servent;
    let mut namelen: socklen_t =
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    sock =
        socket(2 as libc::c_int, SOCK_STREAM as libc::c_int,
               0 as libc::c_int);
    if sock < 0 as libc::c_int {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                *__errno_location() as errcode_t,
                b"creating socket\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    l_inaddr.sin_family = 2 as libc::c_int as sa_family_t;
    l_inaddr.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    if argc == 2 as libc::c_int {
        l_inaddr.sin_port =
            htons(atoi(*argv.offset(1 as libc::c_int as isize)) as uint16_t)
    } else {
        sp =
            getservbyname(b"uu-sample\x00" as *const u8 as
                              *const libc::c_char,
                          b"tcp\x00" as *const u8 as *const libc::c_char);
        if sp.is_null() {
            com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as errcode_t,
                    b"can\'t find uu-sample/tcp service\x00" as *const u8 as
                        *const libc::c_char);
            exit(3 as libc::c_int);
        }
        l_inaddr.sin_port = (*sp).s_port as in_port_t
    }
    setsockopt(sock, 1 as libc::c_int, 2 as libc::c_int,
               &mut one as *mut libc::c_int as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                   socklen_t);
    if bind(sock,
            __CONST_SOCKADDR_ARG{__sockaddr__:
                                     &mut l_inaddr as *mut sockaddr_in as
                                         *mut sockaddr,},
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                *__errno_location() as errcode_t,
                b"binding socket\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    if listen(sock, 1 as libc::c_int) == -(1 as libc::c_int) {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                *__errno_location() as errcode_t,
                b"listening\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    printf(b"Server started\n\x00" as *const u8 as *const libc::c_char);
    fflush(stdout);
    acc =
        accept(sock,
               __SOCKADDR_ARG{__sockaddr__:
                                  &mut f_inaddr as *mut sockaddr_in as
                                      *mut sockaddr,}, &mut namelen);
    if acc == -(1 as libc::c_int) {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                *__errno_location() as errcode_t,
                b"accepting\x00" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    dup2(acc, 0 as libc::c_int);
    close(sock);
    sock = 0 as libc::c_int;
    /* principal name must be sent null-terminated. */
    retval =
        krb5_read_message(context,
                          &mut sock as *mut libc::c_int as krb5_pointer,
                          &mut pname_data);
    if retval != 0 || pname_data.length == 0 as libc::c_int as libc::c_uint ||
           *pname_data.data.offset(pname_data.length.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                       as isize) as libc::c_int !=
               '\u{0}' as i32 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"reading pname\x00" as *const u8 as *const libc::c_char);
        return 2 as libc::c_int
    }
    retval =
        krb5_read_message(context,
                          &mut sock as *mut libc::c_int as krb5_pointer,
                          &mut tkt_data);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"reading ticket data\x00" as *const u8 as
                    *const libc::c_char);
        return 2 as libc::c_int
    }
    retval = krb5_cc_default(context, &mut cc);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"getting credentials cache\x00" as *const u8 as
                    *const libc::c_char);
        return 4 as libc::c_int
    }
    memset(&mut creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    retval = krb5_cc_get_principal(context, cc, &mut creds.client);
    if retval != 0 {
        com_err(b"uu-client\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"getting principal name\x00" as *const u8 as
                    *const libc::c_char);
        return 6 as libc::c_int
    }
    /* client sends it already null-terminated. */
    printf(b"uu-server: client principal is \"%s\".\n\x00" as *const u8 as
               *const libc::c_char, pname_data.data);
    retval = krb5_parse_name(context, pname_data.data, &mut creds.server);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"parsing client name\x00" as *const u8 as
                    *const libc::c_char);
        return 3 as libc::c_int
    }
    creds.second_ticket = tkt_data;
    printf(b"uu-server: client ticket is %d bytes.\n\x00" as *const u8 as
               *const libc::c_char, creds.second_ticket.length);
    retval =
        krb5_get_credentials(context, 1 as libc::c_int, cc, &mut creds,
                             &mut new_creds);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"getting user-user ticket\x00" as *const u8 as
                    *const libc::c_char);
        return 5 as libc::c_int
    }
    l = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    if getsockname(0 as libc::c_int,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut l_inaddr as *mut sockaddr_in as
                                          *mut sockaddr,}, &mut l) ==
           -(1 as libc::c_int) {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                *__errno_location() as errcode_t,
                b"getting local address\x00" as *const u8 as
                    *const libc::c_char);
        return 6 as libc::c_int
    }
    /* send a ticket/authenticator to the other side, so it can get the key
       we're using for the krb_safe below. */
    retval = krb5_auth_con_init(context, &mut auth_context);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"making auth_context\x00" as *const u8 as
                    *const libc::c_char);
        return 8 as libc::c_int
    }
    retval =
        krb5_auth_con_setflags(context, auth_context, 0x4 as libc::c_int);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"initializing the auth_context flags\x00" as *const u8 as
                    *const libc::c_char);
        return 8 as libc::c_int
    }
    retval =
        krb5_auth_con_genaddrs(context, auth_context, sock,
                               0x4 as libc::c_int | 0x8 as libc::c_int);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"generating addrs for auth_context\x00" as *const u8 as
                    *const libc::c_char);
        return 9 as libc::c_int
    }
    retval =
        krb5_mk_req_extended(context, &mut auth_context,
                             0x40000000 as libc::c_int, 0 as *mut krb5_data,
                             new_creds, &mut msg);
    if retval != 0 {
        com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"making AP_REQ\x00" as *const u8 as *const libc::c_char);
        return 8 as libc::c_int
    }
    retval =
        krb5_write_message(context,
                           &mut sock as *mut libc::c_int as krb5_pointer,
                           &mut msg);
    if !(retval != 0) {
        free(msg.data as *mut libc::c_void);
        msgtext.length = 32 as libc::c_int as libc::c_uint;
        msgtext.data =
            b"Hello, other end of connection.\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        retval =
            krb5_mk_safe(context, auth_context, &mut msgtext, &mut msg,
                         0 as *mut krb5_replay_data);
        if retval != 0 {
            com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
                    retval as errcode_t,
                    b"encoding message to client\x00" as *const u8 as
                        *const libc::c_char);
            return 6 as libc::c_int
        }
        retval =
            krb5_write_message(context,
                               &mut sock as *mut libc::c_int as krb5_pointer,
                               &mut msg);
        if !(retval != 0) {
            krb5_free_data_contents(context, &mut msg);
            krb5_free_data_contents(context, &mut pname_data);
            /* tkt_data freed with creds */
            krb5_free_cred_contents(context, &mut creds);
            krb5_free_creds(context, new_creds);
            krb5_cc_close(context, cc);
            krb5_auth_con_free(context, auth_context);
            krb5_free_context(context);
            return 0 as libc::c_int
        }
    }
    com_err(b"uu-server\x00" as *const u8 as *const libc::c_char,
            retval as errcode_t,
            b"writing message to client\x00" as *const u8 as
                *const libc::c_char);
    return 7 as libc::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
