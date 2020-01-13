use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:61"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:61"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:61"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:61"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:61"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:61"]
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
                        krb5_data, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:61"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:61"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:61"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:61"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:62"]
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
#[c2rust::header_src = "/usr/include/stdio.h:61"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:61"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:61"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:61"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/test/common.h:63"]
pub mod common_h {
    use super::stddef_h::size_t;
    use super::krb5_h::krb5_pa_data;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn make_pa_list(contents: *const libc::c_char, len: size_t)
         -> *mut *mut krb5_pa_data;
    }
    /* COMMON_H */
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
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, krb5_c_decrypt, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, alloc_data, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
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
use self::stdio_h::printf;
use self::string_h::{strlen, strdup, strcmp, memcmp};
use self::assert_h::__assert_fail;
use self::stdlib_h::{malloc, calloc, free};
use self::common_h::make_pa_list;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "67:8"]
pub struct client_state {
    pub indicators: *mut libc::c_char,
    pub fail_optimistic: krb5_boolean,
    pub fail_2rt: krb5_boolean,
    pub fail_tryagain: krb5_boolean,
    pub disable_fallback: krb5_boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "75:8"]
pub struct client_request_state {
    pub second_round_trip: krb5_boolean,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/preauth/test/cltest.c - Test clpreauth module */
/*
 * Copyright (C) 2015, 2017 by the Massachusetts Institute of Technology.
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
 * This module is used to test preauth interface features.  At this time, the
 * clpreauth module does the following:
 *
 * - It decrypts a message from the initial KDC pa-data using the reply key and
 *   prints it to stdout.  (The unencrypted message "no key" can also be
 *   displayed.)
 *
 * - If a second round trip is requested, it prints the pa-data contents
 *   accompanying the second round trip request.
 *
 * - It pulls an "indicators" attribute from the gic preauth options and sends
 *   it to the server, instructing the kdcpreauth module to assert one or more
 *   space-separated authentication indicators.  (This string is sent on both
 *   round trips if a second round trip is requested.)
 *
 * - If a KDC_ERR_ENCTYPE_NOSUPP error with e-data is received, it prints the
 *   accompanying error padata and sends a follow-up request containing
 *   "tryagain".
 *
 * - If the "fail_optimistic", "fail_2rt", or "fail_tryagain" gic options are
 *   set, it fails with a recognizable error string at the requested point in
 *   processing.
 *
 * - If the "disable_fallback" gic option is set, fallback is disabled when a
 *   client message is generated.
 */
#[c2rust::src_loc = "65:25"]
static mut pa_types: [krb5_preauthtype; 2] =
    [-(123 as libc::c_int), 0 as libc::c_int];
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn test_init(mut context: krb5_context,
                               mut moddata_out: *mut krb5_clpreauth_moddata)
 -> krb5_error_code {
    let mut st: *mut client_state = 0 as *mut client_state;
    st =
        malloc(::std::mem::size_of::<client_state>() as libc::c_ulong) as
            *mut client_state;
    if !st.is_null() {
    } else {
        __assert_fail(b"st != NULL\x00" as *const u8 as *const libc::c_char,
                      b"cltest.c\x00" as *const u8 as *const libc::c_char,
                      85 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"krb5_error_code test_init(krb5_context, krb5_clpreauth_moddata *)\x00")).as_ptr());
    }
    (*st).indicators = 0 as *mut libc::c_char;
    (*st).fail_tryagain = 0 as libc::c_int as krb5_boolean;
    (*st).fail_2rt = (*st).fail_tryagain;
    (*st).fail_optimistic = (*st).fail_2rt;
    (*st).disable_fallback = 0 as libc::c_int as krb5_boolean;
    *moddata_out = st as krb5_clpreauth_moddata;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn test_fini(mut context: krb5_context,
                               mut moddata: krb5_clpreauth_moddata) {
    let mut st: *mut client_state = moddata as *mut client_state;
    free((*st).indicators as *mut libc::c_void);
    free(st as *mut libc::c_void);
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn test_request_init(mut context: krb5_context,
                                       mut moddata: krb5_clpreauth_moddata,
                                       mut modreq_out:
                                           *mut krb5_clpreauth_modreq) {
    let mut reqst: *mut client_request_state = 0 as *mut client_request_state;
    reqst =
        malloc(::std::mem::size_of::<client_request_state>() as libc::c_ulong)
            as *mut client_request_state;
    if !reqst.is_null() {
    } else {
        __assert_fail(b"reqst != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"cltest.c\x00" as *const u8 as *const libc::c_char,
                      109 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"void test_request_init(krb5_context, krb5_clpreauth_moddata, krb5_clpreauth_modreq *)\x00")).as_ptr());
    }
    (*reqst).second_round_trip = 0 as libc::c_int as krb5_boolean;
    *modreq_out = reqst as krb5_clpreauth_modreq;
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn test_request_fini(mut context: krb5_context,
                                       mut moddata: krb5_clpreauth_moddata,
                                       mut modreq: krb5_clpreauth_modreq) {
    free(modreq as *mut libc::c_void);
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn test_process(mut context: krb5_context,
                                  mut moddata: krb5_clpreauth_moddata,
                                  mut modreq: krb5_clpreauth_modreq,
                                  mut opt: *mut krb5_get_init_creds_opt,
                                  mut cb: krb5_clpreauth_callbacks,
                                  mut rock: krb5_clpreauth_rock,
                                  mut request: *mut krb5_kdc_req,
                                  mut encoded_request_body: *mut krb5_data,
                                  mut encoded_previous_request:
                                      *mut krb5_data,
                                  mut pa_data: *mut krb5_pa_data,
                                  mut prompter: krb5_prompter_fct,
                                  mut prompter_data: *mut libc::c_void,
                                  mut out_pa_data:
                                      *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut st: *mut client_state = moddata as *mut client_state;
    let mut reqst: *mut client_request_state =
        modreq as *mut client_request_state;
    let mut ret: krb5_error_code = 0;
    let mut k: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut enc: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut plain: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut indstr: *const libc::c_char = 0 as *const libc::c_char;
    if (*pa_data).length == 0 as libc::c_int as libc::c_uint {
        /* This is an optimistic preauth test.  Send a recognizable padata
         * value so the KDC knows not to expect a cookie. */
        if (*st).fail_optimistic != 0 {
            krb5_set_error_message(context,
                                   -(1765328174 as libc::c_long) as
                                       krb5_error_code,
                                   b"induced optimistic fail\x00" as *const u8
                                       as *const libc::c_char);
            return -(1765328174 as libc::c_long) as krb5_error_code
        }
        *out_pa_data =
            make_pa_list(b"optimistic\x00" as *const u8 as
                             *const libc::c_char,
                         10 as libc::c_int as size_t);
        if (*st).disable_fallback != 0 {
            (*cb).disable_fallback.expect("non-null function pointer")(context,
                                                                       rock);
        }
        return 0 as libc::c_int
    } else {
        if (*reqst).second_round_trip != 0 {
            printf(b"2rt: %.*s\n\x00" as *const u8 as *const libc::c_char,
                   (*pa_data).length, (*pa_data).contents);
            if (*st).fail_2rt != 0 {
                krb5_set_error_message(context,
                                       -(1765328174 as libc::c_long) as
                                           krb5_error_code,
                                       b"induced 2rt fail\x00" as *const u8 as
                                           *const libc::c_char);
                return -(1765328174 as libc::c_long) as krb5_error_code
            }
        } else if (*pa_data).length == 6 as libc::c_int as libc::c_uint &&
                      memcmp((*pa_data).contents as *const libc::c_void,
                             b"no key\x00" as *const u8 as *const libc::c_char
                                 as *const libc::c_void,
                             6 as libc::c_int as libc::c_ulong) ==
                          0 as libc::c_int {
            printf(b"no key\n\x00" as *const u8 as *const libc::c_char);
        } else {
            /* This fails during s4u_identify_user(), so don't assert. */
            ret =
                (*cb).get_as_key.expect("non-null function pointer")(context,
                                                                     rock,
                                                                     &mut k);
            if ret != 0 { return ret }
            ret = alloc_data(&mut plain, (*pa_data).length);
            if ret == 0 {
            } else {
                __assert_fail(b"!ret\x00" as *const u8 as *const libc::c_char,
                              b"cltest.c\x00" as *const u8 as
                                  *const libc::c_char,
                              164 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 267],
                                                        &[libc::c_char; 267]>(b"krb5_error_code test_process(krb5_context, krb5_clpreauth_moddata, krb5_clpreauth_modreq, krb5_get_init_creds_opt *, krb5_clpreauth_callbacks, krb5_clpreauth_rock, krb5_kdc_req *, krb5_data *, krb5_data *, krb5_pa_data *, krb5_prompter_fct, void *, krb5_pa_data ***)\x00")).as_ptr());
            }
            enc.enctype = (*k).enctype;
            enc.ciphertext =
                make_data((*pa_data).contents as *mut libc::c_void,
                          (*pa_data).length);
            ret =
                krb5_c_decrypt(context, k, 1024 as libc::c_int,
                               0 as *const krb5_data, &mut enc, &mut plain);
            if ret == 0 {
            } else {
                __assert_fail(b"!ret\x00" as *const u8 as *const libc::c_char,
                              b"cltest.c\x00" as *const u8 as
                                  *const libc::c_char,
                              168 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 267],
                                                        &[libc::c_char; 267]>(b"krb5_error_code test_process(krb5_context, krb5_clpreauth_moddata, krb5_clpreauth_modreq, krb5_get_init_creds_opt *, krb5_clpreauth_callbacks, krb5_clpreauth_rock, krb5_kdc_req *, krb5_data *, krb5_data *, krb5_pa_data *, krb5_prompter_fct, void *, krb5_pa_data ***)\x00")).as_ptr());
            }
            printf(b"%.*s\n\x00" as *const u8 as *const libc::c_char,
                   plain.length, plain.data);
            free(plain.data as *mut libc::c_void);
        }
    }
    (*reqst).second_round_trip = 1 as libc::c_int as krb5_boolean;
    indstr =
        if !(*st).indicators.is_null() {
            (*st).indicators as *const libc::c_char
        } else { b"\x00" as *const u8 as *const libc::c_char };
    *out_pa_data = make_pa_list(indstr, strlen(indstr));
    if (*st).disable_fallback != 0 {
        (*cb).disable_fallback.expect("non-null function pointer")(context,
                                                                   rock);
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "181:1"]
unsafe extern "C" fn test_tryagain(mut context: krb5_context,
                                   mut moddata: krb5_clpreauth_moddata,
                                   mut modreq: krb5_clpreauth_modreq,
                                   mut opt: *mut krb5_get_init_creds_opt,
                                   mut cb: krb5_clpreauth_callbacks,
                                   mut rock: krb5_clpreauth_rock,
                                   mut request: *mut krb5_kdc_req,
                                   mut enc_req: *mut krb5_data,
                                   mut enc_prev: *mut krb5_data,
                                   mut pa_type: krb5_preauthtype,
                                   mut error: *mut krb5_error,
                                   mut padata: *mut *mut krb5_pa_data,
                                   mut prompter: krb5_prompter_fct,
                                   mut prompter_data: *mut libc::c_void,
                                   mut padata_out:
                                       *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut st: *mut client_state = moddata as *mut client_state;
    let mut i: libc::c_int = 0;
    *padata_out = 0 as *mut *mut krb5_pa_data;
    if (*st).fail_tryagain != 0 {
        krb5_set_error_message(context,
                               -(1765328174 as libc::c_long) as
                                   krb5_error_code,
                               b"induced tryagain fail\x00" as *const u8 as
                                   *const libc::c_char);
        return -(1765328174 as libc::c_long) as krb5_error_code
    }
    if (*error).error != 14 as libc::c_int as libc::c_uint {
        return -(1765328174 as libc::c_long) as krb5_error_code
    }
    i = 0 as libc::c_int;
    while !(*padata.offset(i as isize)).is_null() {
        if (**padata.offset(i as isize)).pa_type == -(123 as libc::c_int) {
            printf(b"tryagain: %.*s\n\x00" as *const u8 as
                       *const libc::c_char,
                   (**padata.offset(i as isize)).length,
                   (**padata.offset(i as isize)).contents);
        }
        i += 1
    }
    *padata_out =
        make_pa_list(b"tryagain\x00" as *const u8 as *const libc::c_char,
                     8 as libc::c_int as size_t);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "208:1"]
unsafe extern "C" fn test_gic_opt(mut kcontext: krb5_context,
                                  mut moddata: krb5_clpreauth_moddata,
                                  mut opt: *mut krb5_get_init_creds_opt,
                                  mut attr: *const libc::c_char,
                                  mut value: *const libc::c_char)
 -> krb5_error_code {
    let mut st: *mut client_state = moddata as *mut client_state;
    if strcmp(attr, b"indicators\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        free((*st).indicators as *mut libc::c_void);
        (*st).indicators = strdup(value);
        if !(*st).indicators.is_null() {
        } else {
            __assert_fail(b"st->indicators != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"cltest.c\x00" as *const u8 as *const libc::c_char,
                          217 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 122],
                                                    &[libc::c_char; 122]>(b"krb5_error_code test_gic_opt(krb5_context, krb5_clpreauth_moddata, krb5_get_init_creds_opt *, const char *, const char *)\x00")).as_ptr());
        }
    } else if strcmp(attr,
                     b"fail_optimistic\x00" as *const u8 as
                         *const libc::c_char) == 0 as libc::c_int {
        (*st).fail_optimistic = 1 as libc::c_int as krb5_boolean
    } else if strcmp(attr,
                     b"fail_2rt\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        (*st).fail_2rt = 1 as libc::c_int as krb5_boolean
    } else if strcmp(attr,
                     b"fail_tryagain\x00" as *const u8 as *const libc::c_char)
                  == 0 as libc::c_int {
        (*st).fail_tryagain = 1 as libc::c_int as krb5_boolean
    } else if strcmp(attr,
                     b"disable_fallback\x00" as *const u8 as
                         *const libc::c_char) == 0 as libc::c_int {
        (*st).disable_fallback = 1 as libc::c_int as krb5_boolean
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "234:1"]
pub unsafe extern "C" fn clpreauth_test_initvt(mut context: krb5_context,
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
        b"test\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*vt).pa_type_list = pa_types.as_mut_ptr();
    (*vt).init =
        Some(test_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut krb5_clpreauth_moddata)
                     -> krb5_error_code);
    (*vt).fini =
        Some(test_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata) -> ());
    (*vt).request_init =
        Some(test_request_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: *mut krb5_clpreauth_modreq) -> ());
    (*vt).request_fini =
        Some(test_request_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: krb5_clpreauth_modreq) -> ());
    (*vt).process =
        Some(test_process as
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
    (*vt).tryagain =
        Some(test_tryagain as
                 unsafe extern "C" fn(_: krb5_context,
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
                     -> krb5_error_code);
    (*vt).gic_opts =
        Some(test_gic_opt as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: *mut krb5_get_init_creds_opt,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
