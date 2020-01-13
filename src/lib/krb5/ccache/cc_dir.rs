use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:48"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:48"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:48"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:48"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:48"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:48"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:48"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:48"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:48"]
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
    /* KRB5_OLD_CRYPTO */
    /*
 * end "encryption.h"
 */
    /*
 * begin "fieldbits.h"
 */
    /* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
    /* #define      KDC_OPT_UNUSED          0x01000000 */
    /* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
    /* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
    /* #define      KDC_OPT_UNUSED          0x00000004 */
    /*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivitive tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
    /* definitions for ap_options fields */
    /* * @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
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
    /*
 * end "safepriv.h"
 */
    /*
 * begin "ccache.h"
 */
    /* * Cursor for sequential lookup */
    #[c2rust::src_loc = "2278:1"]
    pub type krb5_cc_cursor = krb5_pointer;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2283:1"]
    pub type krb5_cc_ops = _krb5_cc_ops;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::cc_int_h::{_krb5_ccache, _krb5_cc_ops};
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
        #[no_mangle]
        #[c2rust::src_loc = "4380:1"]
        pub fn krb5_cc_default_name(context: krb5_context)
         -> *const libc::c_char;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:48"]
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
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
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
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:48"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/ccache/cc-int.h:49"]
pub mod cc_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/ccache/cc-int.h */
/*
 * Copyright 1990,1991 by the Massachusetts Institute of Technology.
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
    /* This file contains constant and function declarations used in the
 * file-based credential cache routines. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:8"]
    pub struct _krb5_ccache {
        pub magic: krb5_magic,
        pub ops: *const _krb5_cc_ops,
        pub data: krb5_pointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct _krb5_cc_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_ccache)
                                 -> *const libc::c_char>,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut krb5_ccache,
                                                 _: *const libc::c_char)
                                -> krb5_error_code>,
        pub gen_new: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut krb5_ccache)
                                -> krb5_error_code>,
        pub init: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_ccache,
                                              _: krb5_principal)
                             -> krb5_error_code>,
        pub destroy: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_ccache)
                                -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_ccache)
                              -> krb5_error_code>,
        pub store: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_ccache,
                                               _: *mut krb5_creds)
                              -> krb5_error_code>,
        pub retrieve: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_ccache,
                                                  _: krb5_flags,
                                                  _: *mut krb5_creds,
                                                  _: *mut krb5_creds)
                                 -> krb5_error_code>,
        pub get_princ: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: *mut krb5_principal)
                                  -> krb5_error_code>,
        pub get_first: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: *mut krb5_cc_cursor)
                                  -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_ccache,
                                                  _: *mut krb5_cc_cursor,
                                                  _: *mut krb5_creds)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_ccache,
                                                 _: *mut krb5_cc_cursor)
                                -> krb5_error_code>,
        pub remove_cred: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_ccache,
                                                     _: krb5_flags,
                                                     _: *mut krb5_creds)
                                    -> krb5_error_code>,
        pub set_flags: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: krb5_flags)
                                  -> krb5_error_code>,
        pub get_flags: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: *mut krb5_flags)
                                  -> krb5_error_code>,
        pub ptcursor_new: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _:
                                                          *mut krb5_cc_ptcursor)
                                     -> krb5_error_code>,
        pub ptcursor_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_cc_ptcursor,
                                                       _: *mut krb5_ccache)
                                      -> krb5_error_code>,
        pub ptcursor_free: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           *mut krb5_cc_ptcursor)
                                      -> krb5_error_code>,
        pub move_0: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_ccache,
                                                _: krb5_ccache)
                               -> krb5_error_code>,
        pub wasdefault: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_ccache,
                                                    _: *mut krb5_timestamp)
                                   -> krb5_error_code>,
        pub lock: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_ccache)
                             -> krb5_error_code>,
        pub unlock: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_ccache)
                               -> krb5_error_code>,
        pub switch_to: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache)
                                  -> krb5_error_code>,
    }
    #[c2rust::src_loc = "174:1"]
    pub type krb5_cc_ptcursor = *mut krb5_cc_ptcursor_s;
    /*
 * Per-type ccache cursor.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:8"]
    pub struct krb5_cc_ptcursor_s {
        pub ops: *const _krb5_cc_ops,
        pub data: krb5_pointer,
    }
    use super::krb5_h::{krb5_magic, krb5_pointer, krb5_context, krb5_ccache,
                        krb5_error_code, krb5_principal, krb5_creds,
                        krb5_flags, krb5_cc_cursor, krb5_timestamp};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn krb5int_fcc_new_unique(context: krb5_context,
                                      template: *mut libc::c_char,
                                      id: *mut krb5_ccache)
         -> krb5_error_code;
    }
    /* __KRB5_CCACHE_H__ */
}
#[c2rust::header_src = "/usr/include/dirent.h:62"]
pub mod include_dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/usr/include/bits/dirent.h:62"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct dirent {
        pub d_ino: __ino_t,
        pub d_off: __off_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    use super::types_h::{__ino_t, __off_t};
}
#[c2rust::header_src = "/usr/include/stdlib.h:48"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "688:1"]
        pub fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:48"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "279:1"]
        pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:48"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:48"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:48"]
pub mod k5_platform_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1066:1"]
        pub fn k5_path_split(path: *const libc::c_char,
                             parent_out: *mut *mut libc::c_char,
                             basename_out: *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "1074:1"]
        pub fn k5_path_join(path1: *const libc::c_char,
                            path2: *const libc::c_char,
                            path_out: *mut *mut libc::c_char) -> libc::c_long;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/libintl.h:48"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:48"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:48"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    use super::types_h::__mode_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "280:1"]
        pub fn chmod(__file: *const libc::c_char, __mode: __mode_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "317:1"]
        pub fn mkdir(__path: *const libc::c_char, __mode: __mode_t)
         -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __dev_t, __uid_t, __gid_t,
                        __ino_t, __mode_t, __nlink_t, __off_t, __off64_t,
                        __time_t, __blksize_t, __blkcnt_t, __syscall_slong_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::struct_timespec_h::timespec;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_addrtype,
                       krb5_enctype, krb5_authdatatype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_creds, krb5_creds,
                       krb5_cc_cursor, krb5_ccache, krb5_cc_ops, _profile_t,
                       krb5_cc_default_name, krb5_set_error_message,
                       krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::cc_int_h::{_krb5_ccache, _krb5_cc_ops, krb5_cc_ptcursor,
                         krb5_cc_ptcursor_s, krb5int_fcc_new_unique};
pub use self::include_dirent_h::{DIR, __dirstream, opendir, closedir,
                                 readdir};
pub use self::dirent_h::dirent;
use self::stdlib_h::{malloc, free, mkstemp};
use self::stdio_h::{rename, fclose, fopen, fdopen, fprintf, asprintf, fgets};
use self::errno_h::__errno_location;
use self::unistd_h::close;
use self::k5_platform_h::{k5_path_split, k5_path_join};
use self::libintl_h::dgettext;
use self::string_h::{strlen, strchr, strdup, strncmp, strcmp};
use self::sys_stat_h::{stat, chmod, mkdir};
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/ccache/cc_dir.c - Directory-based credential cache collection */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
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
 * This credential cache type represents a set of file-based caches with a
 * switchable primary cache.  An alternate form of the type represents a
 * subsidiary file cache within the directory.
 *
 * A cache name of the form DIR:dirname identifies a directory containing the
 * cache set.  Resolving a name of this form results in dirname's primary
 * cache.  If a context's default cache is of this form, the global cache
 * collection will contain dirname's cache set, and new unique caches of type
 * DIR will be created within dirname.
 *
 * A cache name of the form DIR::filepath represents a single cache within the
 * directory.  Switching to a ccache of this type causes the directory's
 * primary cache to be set to the named cache.
 *
 * Within the directory, cache names begin with 'tkt'.  The file "primary"
 * contains a single line naming the primary cache.  The directory must already
 * exist when the DIR ccache is resolved, but the primary file will be created
 * automatically if it does not exist.
 */
    /* This is Unix-only for now.  To work on Windows, we will need opendir/readdir
 * replacements and possibly more flexible newline handling. */
    #[no_mangle]
    #[c2rust::src_loc = "65:26"]
    pub static krb5_fcc_ops: krb5_cc_ops;
}
/* Fields are not modified after creation, so no lock is necessary. */
#[c2rust::src_loc = "68:1"]
pub type dcc_data = dcc_data_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "68:16"]
pub struct dcc_data_st {
    pub residual: *mut libc::c_char,
    pub fcc: krb5_ccache,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "542:8"]
pub struct dcc_ptcursor_data {
    pub primary: *mut libc::c_char,
    pub dirname: *mut libc::c_char,
    pub dir: *mut DIR,
    pub first: krb5_boolean,
}
#[inline]
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn filename_is_cache(mut filename: *const libc::c_char)
 -> krb5_boolean {
    return (strncmp(filename, b"tkt\x00" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong) == 0 as libc::c_int) as
               libc::c_int as krb5_boolean;
}
/* dirname or :filename */
/* File cache for actual cache ops */
/* Compose the pathname of the primary file within a cache directory. */
#[inline]
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn primary_pathname(mut dirname: *const libc::c_char,
                                      mut path_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    return k5_path_join(dirname,
                        b"primary\x00" as *const u8 as *const libc::c_char,
                        path_out) as krb5_error_code;
}
/* Compose a residual string for a subsidiary path with the specified directory
 * name and filename. */
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn subsidiary_residual(mut dirname: *const libc::c_char,
                                         mut filename: *const libc::c_char,
                                         mut out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    *out = 0 as *mut libc::c_char;
    ret = k5_path_join(dirname, filename, &mut path) as krb5_error_code;
    if ret != 0 { return ret }
    ret =
        asprintf(&mut residual as *mut *mut libc::c_char,
                 b":%s\x00" as *const u8 as *const libc::c_char, path);
    free(path as *mut libc::c_void);
    if ret < 0 as libc::c_int { return 12 as libc::c_int }
    *out = residual;
    return 0 as libc::c_int;
}
#[inline]
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn split_path(mut context: krb5_context,
                                mut path: *const libc::c_char,
                                mut dirname_out: *mut *mut libc::c_char,
                                mut filename_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    *dirname_out = 0 as *mut libc::c_char;
    *filename_out = 0 as *mut libc::c_char;
    ret = k5_path_split(path, &mut dirname, &mut filename) as krb5_error_code;
    if ret != 0 { return ret }
    if *dirname as libc::c_int == '\u{0}' as i32 {
        ret = -(1765328245 as libc::c_long) as krb5_error_code;
        krb5_set_error_message(context, ret,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Subsidiary cache path %s has no parent directory\x00"
                                            as *const u8 as
                                            *const libc::c_char), path);
    } else if filename_is_cache(filename) == 0 {
        ret = -(1765328245 as libc::c_long) as krb5_error_code;
        krb5_set_error_message(context, ret,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Subsidiary cache path %s filename does not begin with \"tkt\"\x00"
                                            as *const u8 as
                                            *const libc::c_char), path);
    } else {
        *dirname_out = dirname;
        *filename_out = filename;
        return 0 as libc::c_int
    }
    free(dirname as *mut libc::c_void);
    free(filename as *mut libc::c_void);
    return ret;
}
/* Read the primary file and compose the residual string for the primary
 * subsidiary cache file. */
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn read_primary_file(mut context: krb5_context,
                                       mut primary_path: *const libc::c_char,
                                       mut dirname: *const libc::c_char,
                                       mut residual_out:
                                           *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    *residual_out = 0 as *mut libc::c_char;
    /* Open the file and read its first line. */
    fp = fopen(primary_path, b"r\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() { return 2 as libc::c_int }
    ret =
        fgets(buf.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                  libc::c_int, fp);
    fclose(fp);
    if ret.is_null() {
        return -(1765328191 as libc::c_long) as krb5_error_code
    }
    len = strlen(buf.as_mut_ptr());
    /* Check if line is too long, doesn't look like a subsidiary cache
     * filename, or isn't a single-component filename. */
    if buf[len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as
           libc::c_int != '\n' as i32 ||
           filename_is_cache(buf.as_mut_ptr()) == 0 ||
           !strchr(buf.as_mut_ptr(), '/' as i32).is_null() ||
           !strchr(buf.as_mut_ptr(), '\\' as i32).is_null() {
        krb5_set_error_message(context,
                               -(1765328185 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"%s contains invalid filename\x00" as
                                            *const u8 as *const libc::c_char),
                               primary_path);
        return -(1765328185 as libc::c_long) as krb5_error_code
    }
    buf[len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
        '\u{0}' as i32 as libc::c_char;
    return subsidiary_residual(dirname, buf.as_mut_ptr(), residual_out);
}
/* Create or update the primary file with a line containing contents. */
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn write_primary_file(mut primary_path: *const libc::c_char,
                                        mut contents: *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code =
        -(1765328191 as libc::c_long) as krb5_error_code;
    let mut newpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut status: libc::c_int = 0;
    if asprintf(&mut newpath as *mut *mut libc::c_char,
                b"%s.XXXXXX\x00" as *const u8 as *const libc::c_char,
                primary_path) < 0 as libc::c_int {
        return 12 as libc::c_int
    }
    fd = mkstemp(newpath);
    if !(fd < 0 as libc::c_int) {
        chmod(newpath,
              (0o400 as libc::c_int | 0o200 as libc::c_int) as __mode_t);
        fp = fdopen(fd, b"w\x00" as *const u8 as *const libc::c_char);
        if !fp.is_null() {
            fd = -(1 as libc::c_int);
            if !(fprintf(fp, b"%s\n\x00" as *const u8 as *const libc::c_char,
                         contents) < 0 as libc::c_int) {
                status = fclose(fp);
                fp = 0 as *mut FILE;
                if !(status == -(1 as libc::c_int)) {
                    fp = 0 as *mut FILE;
                    if !(rename(newpath, primary_path) != 0 as libc::c_int) {
                        ret = 0 as libc::c_int
                    }
                }
            }
        }
    }
    if fd >= 0 as libc::c_int { close(fd); }
    if !fp.is_null() { fclose(fp); }
    free(newpath as *mut libc::c_void);
    return ret;
}
/* Verify or create a cache directory path. */
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn verify_dir(mut context: krb5_context,
                                mut dirname: *const libc::c_char)
 -> krb5_error_code {
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    if stat(dirname, &mut st) < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int &&
               mkdir(dirname,
                     (0o400 as libc::c_int | 0o200 as libc::c_int |
                          0o100 as libc::c_int) as __mode_t) ==
                   0 as libc::c_int {
            return 0 as libc::c_int
        }
        krb5_set_error_message(context,
                               -(1765328189 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Credential cache directory %s does not exist\x00"
                                            as *const u8 as
                                            *const libc::c_char), dirname);
        return -(1765328189 as libc::c_long) as krb5_error_code
    }
    if !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
             0o40000 as libc::c_int as libc::c_uint) {
        krb5_set_error_message(context,
                               -(1765328185 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Credential cache directory %s exists but is not a directory\x00"
                                            as *const u8 as
                                            *const libc::c_char), dirname);
        return -(1765328185 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
/*
 * If the default ccache name for context is a directory collection, set
 * *dirname_out to the directory name for that collection.  Otherwise set
 * *dirname_out to NULL.
 */
#[c2rust::src_loc = "247:1"]
unsafe extern "C" fn get_context_default_dir(mut context: krb5_context,
                                             mut dirname_out:
                                                 *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    *dirname_out = 0 as *mut libc::c_char;
    defname = krb5_cc_default_name(context);
    if defname.is_null() { return 0 as libc::c_int }
    if strncmp(defname, b"DIR:\x00" as *const u8 as *const libc::c_char,
               4 as libc::c_int as libc::c_ulong) != 0 as libc::c_int ||
           *defname.offset(4 as libc::c_int as isize) as libc::c_int ==
               ':' as i32 ||
           *defname.offset(4 as libc::c_int as isize) as libc::c_int ==
               '\u{0}' as i32 {
        return 0 as libc::c_int
    }
    dirname = strdup(defname.offset(4 as libc::c_int as isize));
    if dirname.is_null() { return 12 as libc::c_int }
    *dirname_out = dirname;
    return 0 as libc::c_int;
}
/*
 * If the default ccache name for context is a subsidiary file in a directory
 * collection, set *subsidiary_out to the residual value.  Otherwise set
 * *subsidiary_out to NULL.
 */
#[c2rust::src_loc = "272:1"]
unsafe extern "C" fn get_context_subsidiary_file(mut context: krb5_context,
                                                 mut subsidiary_out:
                                                     *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    *subsidiary_out = 0 as *mut libc::c_char;
    defname = krb5_cc_default_name(context);
    if defname.is_null() ||
           strncmp(defname, b"DIR::\x00" as *const u8 as *const libc::c_char,
                   5 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    residual = strdup(defname.offset(4 as libc::c_int as isize));
    if residual.is_null() { return 12 as libc::c_int }
    *subsidiary_out = residual;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "289:1"]
unsafe extern "C" fn dcc_get_name(mut context: krb5_context,
                                  mut cache: krb5_ccache)
 -> *const libc::c_char {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return (*data).residual;
}
/* Construct a cache object given a residual string and file ccache.  Take
 * ownership of fcc on success. */
#[c2rust::src_loc = "299:1"]
unsafe extern "C" fn make_cache(mut residual: *const libc::c_char,
                                mut fcc: krb5_ccache,
                                mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut cache: krb5_ccache = 0 as krb5_ccache;
    let mut data: *mut dcc_data = 0 as *mut dcc_data;
    let mut residual_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    cache =
        malloc(::std::mem::size_of::<_krb5_ccache>() as libc::c_ulong) as
            krb5_ccache;
    if !cache.is_null() {
        data =
            malloc(::std::mem::size_of::<dcc_data>() as libc::c_ulong) as
                *mut dcc_data;
        if !data.is_null() {
            residual_copy = strdup(residual);
            if !residual_copy.is_null() {
                (*data).residual = residual_copy;
                (*data).fcc = fcc;
                (*cache).ops = &krb5_dcc_ops;
                (*cache).data = data as krb5_pointer;
                (*cache).magic = -(1760647380 as libc::c_long) as krb5_magic;
                *cache_out = cache;
                return 0 as libc::c_int
            }
        }
    }
    free(cache as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(residual_copy as *mut libc::c_void);
    return 12 as libc::c_int;
}
#[c2rust::src_loc = "331:1"]
unsafe extern "C" fn dcc_resolve(mut context: krb5_context,
                                 mut cache_out: *mut krb5_ccache,
                                 mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut fcc: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut primary_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sresidual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    *cache_out = 0 as krb5_ccache;
    if *residual as libc::c_int == ':' as i32 {
        /* This is a subsidiary cache within the directory. */
        ret =
            split_path(context, residual.offset(1 as libc::c_int as isize),
                       &mut dirname, &mut filename);
        if ret != 0 { return ret }
        ret = verify_dir(context, dirname);
        free(dirname as *mut libc::c_void);
        free(filename as *mut libc::c_void);
        if ret != 0 { return ret }
        current_block = 14401909646449704462;
    } else {
        /* This is the directory itself; resolve to the primary cache. */
        ret = verify_dir(context, residual);
        if ret != 0 { return ret }
        ret = primary_pathname(residual, &mut primary_path);
        if ret != 0 {
            current_block = 866547892567815944;
        } else {
            ret =
                read_primary_file(context, primary_path, residual,
                                  &mut sresidual);
            if ret == 2 as libc::c_int {
                /* Create an initial primary file. */
                ret =
                    write_primary_file(primary_path,
                                       b"tkt\x00" as *const u8 as
                                           *const libc::c_char);
                if ret != 0 {
                    current_block = 866547892567815944;
                } else {
                    ret =
                        subsidiary_residual(residual,
                                            b"tkt\x00" as *const u8 as
                                                *const libc::c_char,
                                            &mut sresidual);
                    current_block = 4808432441040389987;
                }
            } else { current_block = 4808432441040389987; }
            match current_block {
                866547892567815944 => { }
                _ => {
                    if ret != 0 {
                        current_block = 866547892567815944;
                    } else {
                        residual = sresidual;
                        current_block = 14401909646449704462;
                    }
                }
            }
        }
    }
    match current_block {
        14401909646449704462 => {
            ret =
                krb5_fcc_ops.resolve.expect("non-null function pointer")(context,
                                                                         &mut fcc,
                                                                         residual.offset(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             isize));
            if !(ret != 0) {
                ret = make_cache(residual, fcc, cache_out);
                if ret != 0 {
                    krb5_fcc_ops.close.expect("non-null function pointer")(context,
                                                                           fcc);
                }
            }
        }
        _ => { }
    }
    free(primary_path as *mut libc::c_void);
    free(sresidual as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "387:1"]
unsafe extern "C" fn dcc_gen_new(mut context: krb5_context,
                                 mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut template: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fcc: krb5_ccache = 0 as krb5_ccache;
    *cache_out = 0 as krb5_ccache;
    ret = get_context_default_dir(context, &mut dirname);
    if ret != 0 { return ret }
    if dirname.is_null() {
        krb5_set_error_message(context,
                               -(1750600188 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Can\'t create new subsidiary cache because default cache is not a directory collection\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        return -(1750600188 as libc::c_long) as krb5_error_code
    }
    ret = verify_dir(context, dirname);
    if !(ret != 0) {
        ret =
            k5_path_join(dirname,
                         b"tktXXXXXX\x00" as *const u8 as *const libc::c_char,
                         &mut template) as krb5_error_code;
        if !(ret != 0) {
            ret = krb5int_fcc_new_unique(context, template, &mut fcc);
            if !(ret != 0) {
                if asprintf(&mut residual as *mut *mut libc::c_char,
                            b":%s\x00" as *const u8 as *const libc::c_char,
                            template) < 0 as libc::c_int {
                    ret = 12 as libc::c_int
                } else {
                    ret = make_cache(residual, fcc, cache_out);
                    if !(ret != 0) { fcc = 0 as krb5_ccache }
                }
            }
        }
    }
    if !fcc.is_null() {
        krb5_fcc_ops.destroy.expect("non-null function pointer")(context,
                                                                 fcc);
    }
    free(dirname as *mut libc::c_void);
    free(template as *mut libc::c_void);
    free(residual as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "431:1"]
unsafe extern "C" fn dcc_init(mut context: krb5_context,
                              mut cache: krb5_ccache,
                              mut princ: krb5_principal) -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.init.expect("non-null function pointer")(context,
                                                                 (*data).fcc,
                                                                 princ);
}
#[c2rust::src_loc = "439:1"]
unsafe extern "C" fn dcc_destroy(mut context: krb5_context,
                                 mut cache: krb5_ccache) -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    let mut ret: krb5_error_code = 0;
    ret =
        krb5_fcc_ops.destroy.expect("non-null function pointer")(context,
                                                                 (*data).fcc);
    free((*data).residual as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(cache as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn dcc_close(mut context: krb5_context,
                               mut cache: krb5_ccache) -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    let mut ret: krb5_error_code = 0;
    ret =
        krb5_fcc_ops.close.expect("non-null function pointer")(context,
                                                               (*data).fcc);
    free((*data).residual as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(cache as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "465:1"]
unsafe extern "C" fn dcc_store(mut context: krb5_context,
                               mut cache: krb5_ccache,
                               mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.store.expect("non-null function pointer")(context,
                                                                  (*data).fcc,
                                                                  creds);
}
#[c2rust::src_loc = "473:1"]
unsafe extern "C" fn dcc_retrieve(mut context: krb5_context,
                                  mut cache: krb5_ccache,
                                  mut flags: krb5_flags,
                                  mut mcreds: *mut krb5_creds,
                                  mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.retrieve.expect("non-null function pointer")(context,
                                                                     (*data).fcc,
                                                                     flags,
                                                                     mcreds,
                                                                     creds);
}
#[c2rust::src_loc = "483:1"]
unsafe extern "C" fn dcc_get_princ(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut princ_out: *mut krb5_principal)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.get_princ.expect("non-null function pointer")(context,
                                                                      (*data).fcc,
                                                                      princ_out);
}
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn dcc_get_first(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.get_first.expect("non-null function pointer")(context,
                                                                      (*data).fcc,
                                                                      cursor);
}
#[c2rust::src_loc = "500:1"]
unsafe extern "C" fn dcc_get_next(mut context: krb5_context,
                                  mut cache: krb5_ccache,
                                  mut cursor: *mut krb5_cc_cursor,
                                  mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.get_next.expect("non-null function pointer")(context,
                                                                     (*data).fcc,
                                                                     cursor,
                                                                     creds);
}
#[c2rust::src_loc = "509:1"]
unsafe extern "C" fn dcc_end_get(mut context: krb5_context,
                                 mut cache: krb5_ccache,
                                 mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.end_get.expect("non-null function pointer")(context,
                                                                    (*data).fcc,
                                                                    cursor);
}
#[c2rust::src_loc = "517:1"]
unsafe extern "C" fn dcc_remove_cred(mut context: krb5_context,
                                     mut cache: krb5_ccache,
                                     mut flags: krb5_flags,
                                     mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.remove_cred.expect("non-null function pointer")(context,
                                                                        (*data).fcc,
                                                                        flags,
                                                                        creds);
}
#[c2rust::src_loc = "526:1"]
unsafe extern "C" fn dcc_set_flags(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut flags: krb5_flags) -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.set_flags.expect("non-null function pointer")(context,
                                                                      (*data).fcc,
                                                                      flags);
}
#[c2rust::src_loc = "534:1"]
unsafe extern "C" fn dcc_get_flags(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut flags_out: *mut krb5_flags)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.get_flags.expect("non-null function pointer")(context,
                                                                      (*data).fcc,
                                                                      flags_out);
}
/* Construct a cursor, taking ownership of dirname, primary, and dir on
 * success. */
#[c2rust::src_loc = "551:1"]
unsafe extern "C" fn make_cursor(mut dirname: *mut libc::c_char,
                                 mut primary: *mut libc::c_char,
                                 mut dir: *mut DIR,
                                 mut cursor_out: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut cursor: krb5_cc_ptcursor = 0 as *mut krb5_cc_ptcursor_s;
    let mut data: *mut dcc_ptcursor_data = 0 as *mut dcc_ptcursor_data;
    *cursor_out = 0 as krb5_cc_ptcursor;
    data =
        malloc(::std::mem::size_of::<dcc_ptcursor_data>() as libc::c_ulong) as
            *mut dcc_ptcursor_data;
    if data.is_null() { return 12 as libc::c_int }
    cursor =
        malloc(::std::mem::size_of::<krb5_cc_ptcursor_s>() as libc::c_ulong)
            as krb5_cc_ptcursor;
    if cursor.is_null() {
        free(data as *mut libc::c_void);
        return 12 as libc::c_int
    }
    (*data).dirname = dirname;
    (*data).primary = primary;
    (*data).dir = dir;
    (*data).first = 1 as libc::c_int as krb5_boolean;
    (*cursor).ops = &krb5_dcc_ops;
    (*cursor).data = data as krb5_pointer;
    *cursor_out = cursor;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "579:1"]
unsafe extern "C" fn dcc_ptcursor_new(mut context: krb5_context,
                                      mut cursor_out: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut primary_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut primary: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    *cursor_out = 0 as krb5_cc_ptcursor;
    /* If the default cache is a subsidiary file, make a cursor with the
     * specified file as the primary but with no directory collection. */
    ret = get_context_subsidiary_file(context, &mut primary);
    if !(ret != 0) {
        if !primary.is_null() {
            ret =
                make_cursor(0 as *mut libc::c_char, primary, 0 as *mut DIR,
                            cursor_out);
            if ret != 0 { free(primary as *mut libc::c_void); }
            return ret
        }
        /* Open the directory for the context's default cache. */
        ret = get_context_default_dir(context, &mut dirname);
        if !(ret != 0 || dirname.is_null()) {
            dir = opendir(dirname);
            if !dir.is_null() {
                /* Fetch the primary cache name if possible. */
                ret = primary_pathname(dirname, &mut primary_path);
                if !(ret != 0) {
                    ret =
                        read_primary_file(context, primary_path, dirname,
                                          &mut primary);
                    if ret != 0 { krb5_clear_error_message(context); }
                    ret = make_cursor(dirname, primary, dir, cursor_out);
                    if !(ret != 0) {
                        primary = 0 as *mut libc::c_char;
                        dirname = primary;
                        dir = 0 as *mut DIR
                    }
                }
            }
        }
    }
    free(dirname as *mut libc::c_void);
    free(primary_path as *mut libc::c_void);
    free(primary as *mut libc::c_void);
    if !dir.is_null() { closedir(dir); }
    /* Return an empty cursor if we fail for any reason. */
    if (*cursor_out).is_null() {
        return make_cursor(0 as *mut libc::c_char, 0 as *mut libc::c_char,
                           0 as *mut DIR, cursor_out)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "634:1"]
unsafe extern "C" fn dcc_ptcursor_next(mut context: krb5_context,
                                       mut cursor: krb5_cc_ptcursor,
                                       mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut data: *mut dcc_ptcursor_data =
        (*cursor).data as *mut dcc_ptcursor_data;
    let mut ent: *mut dirent = 0 as *mut dirent;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: krb5_error_code = 0;
    let mut sb: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    *cache_out = 0 as krb5_ccache;
    /* Return the primary or specified subsidiary cache if we haven't yet. */
    if (*data).first != 0 {
        (*data).first = 0 as libc::c_int as krb5_boolean;
        if !(*data).primary.is_null() &&
               stat((*data).primary.offset(1 as libc::c_int as isize),
                    &mut sb) == 0 as libc::c_int {
            return dcc_resolve(context, cache_out, (*data).primary)
        }
    }
    if (*data).dir.is_null() {
        /* No directory collection */
        return 0 as libc::c_int
    }
    loop 
         /* Look for the next filename of the correct form, without repeating the
     * primary cache. */
         {
        ent = readdir((*data).dir);
        if ent.is_null() { break ; }
        if filename_is_cache((*ent).d_name.as_mut_ptr()) == 0 { continue ; }
        ret =
            subsidiary_residual((*data).dirname, (*ent).d_name.as_mut_ptr(),
                                &mut residual);
        if ret != 0 { return ret }
        if !(*data).primary.is_null() &&
               strcmp(residual, (*data).primary) == 0 as libc::c_int {
            free(residual as *mut libc::c_void);
        } else {
            ret = dcc_resolve(context, cache_out, residual);
            free(residual as *mut libc::c_void);
            return ret
        }
    }
    /* We exhausted the directory without finding a cache to yield. */
    closedir((*data).dir);
    (*data).dir = 0 as *mut DIR;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "679:1"]
unsafe extern "C" fn dcc_ptcursor_free(mut context: krb5_context,
                                       mut cursor: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut data: *mut dcc_ptcursor_data =
        (**cursor).data as *mut dcc_ptcursor_data;
    if !(*data).dir.is_null() { closedir((*data).dir); }
    free((*data).dirname as *mut libc::c_void);
    free((*data).primary as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(*cursor as *mut libc::c_void);
    *cursor = 0 as krb5_cc_ptcursor;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "694:1"]
unsafe extern "C" fn dcc_lock(mut context: krb5_context,
                              mut cache: krb5_ccache) -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.lock.expect("non-null function pointer")(context,
                                                                 (*data).fcc);
}
#[c2rust::src_loc = "702:1"]
unsafe extern "C" fn dcc_unlock(mut context: krb5_context,
                                mut cache: krb5_ccache) -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    return krb5_fcc_ops.unlock.expect("non-null function pointer")(context,
                                                                   (*data).fcc);
}
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn dcc_switch_to(mut context: krb5_context,
                                   mut cache: krb5_ccache)
 -> krb5_error_code {
    let mut data: *mut dcc_data = (*cache).data as *mut dcc_data;
    let mut primary_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: krb5_error_code = 0;
    ret =
        split_path(context,
                   (*data).residual.offset(1 as libc::c_int as isize),
                   &mut dirname, &mut filename);
    if ret != 0 { return ret }
    ret = primary_pathname(dirname, &mut primary_path);
    if !(ret != 0) { ret = write_primary_file(primary_path, filename) }
    free(primary_path as *mut libc::c_void);
    free(dirname as *mut libc::c_void);
    free(filename as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "734:19"]
pub static mut krb5_dcc_ops: krb5_cc_ops =
    unsafe {
        {
            let mut init =
                _krb5_cc_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"DIR\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             get_name:
                                 Some(dcc_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> *const libc::c_char),
                             resolve:
                                 Some(dcc_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache,
                                                               _:
                                                                   *const libc::c_char)
                                              -> krb5_error_code),
                             gen_new:
                                 Some(dcc_gen_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             init:
                                 Some(dcc_init as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   krb5_principal)
                                              -> krb5_error_code),
                             destroy:
                                 Some(dcc_destroy as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             close:
                                 Some(dcc_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             store:
                                 Some(dcc_store as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             retrieve:
                                 Some(dcc_retrieve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags,
                                                               _:
                                                                   *mut krb5_creds,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             get_princ:
                                 Some(dcc_get_princ as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_principal)
                                              -> krb5_error_code),
                             get_first:
                                 Some(dcc_get_first as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(dcc_get_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             end_get:
                                 Some(dcc_end_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             remove_cred:
                                 Some(dcc_remove_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             set_flags:
                                 Some(dcc_set_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags)
                                              -> krb5_error_code),
                             get_flags:
                                 Some(dcc_get_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_flags)
                                              -> krb5_error_code),
                             ptcursor_new:
                                 Some(dcc_ptcursor_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             ptcursor_next:
                                 Some(dcc_ptcursor_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_cc_ptcursor,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             ptcursor_free:
                                 Some(dcc_ptcursor_free as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             move_0: None,
                             wasdefault: None,
                             lock:
                                 Some(dcc_lock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             unlock:
                                 Some(dcc_unlock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             switch_to:
                                 Some(dcc_switch_to as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),};
            init
        }
    };
/* not _WIN32 */
