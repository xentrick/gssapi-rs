use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:41"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:41"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:41"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:41"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_iovec.h:41"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/unistd.h:41"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:41"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    use super::pthreadtypes_h::pthread_mutex_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:41"]
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
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_1 {
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
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_1)).i);
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::byteswap_h::{__bswap_16, __bswap_32};
    use super::types_h::__uint16_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:41"]
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
        #[no_mangle]
        #[c2rust::src_loc = "4380:1"]
        pub fn krb5_cc_default_name(context: krb5_context)
         -> *const libc::c_char;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:41"]
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
                        krb5_context};
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
        #[c2rust::src_loc = "2216:1"]
        pub fn krb5_net_read(_: krb5_context, _: libc::c_int,
                             _: *mut libc::c_char, _: libc::c_int)
         -> libc::c_int;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:41"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:41"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:41"]
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
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/ccache/cc-int.h:43"]
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
    /* reentrant mutex used by krb5_cc_* functions */
    #[c2rust::src_loc = "75:1"]
    pub type k5_cc_mutex = _k5_cc_mutex;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:16"]
    pub struct _k5_cc_mutex {
        pub lock: k5_mutex_t,
        pub owner: krb5_context,
        pub refcount: krb5_int32,
    }
    use super::krb5_h::{krb5_magic, krb5_pointer, krb5_context, krb5_ccache,
                        krb5_error_code, krb5_principal, krb5_creds,
                        krb5_flags, krb5_cc_cursor, krb5_timestamp,
                        krb5_int32, krb5_principal_data};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    use super::k5_buf_h::k5buf;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn k5_cc_retrieve_cred_default(_: krb5_context, _: krb5_ccache,
                                           _: krb5_flags, _: *mut krb5_creds,
                                           _: *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn k5_cc_mutex_init(m: *mut k5_cc_mutex) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn k5_cc_mutex_lock(context: krb5_context, m: *mut k5_cc_mutex);
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn k5_cc_mutex_unlock(context: krb5_context, m: *mut k5_cc_mutex);
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn k5_unmarshal_cred(data: *const libc::c_uchar, len: size_t,
                                 version: libc::c_int, creds: *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn k5_unmarshal_princ(data: *const libc::c_uchar, len: size_t,
                                  version: libc::c_int,
                                  princ_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "158:1"]
        pub fn k5_marshal_cred(buf: *mut k5buf, version: libc::c_int,
                               creds: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "161:1"]
        pub fn k5_marshal_mcred(buf: *mut k5buf, mcred: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_marshal_princ(buf: *mut k5buf, version: libc::c_int,
                                princ: krb5_principal);
    }
    /* __KRB5_CCACHE_H__ */
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:41"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:41"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:41"]
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
#[c2rust::header_src = "/usr/include/sys/socket.h:41"]
pub mod sys_socket_h {
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
    use super::un_h::sockaddr_un;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
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
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                       __len: socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/un.h:47"]
pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:8"]
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [libc::c_char; 108],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:41"]
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
        pub __in6_u: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed_2 {
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
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/port-sockets.h:41"]
pub mod port_sockets_h {
    #[c2rust::src_loc = "204:1"]
    pub type sg_buf = iovec;
    #[inline]
    #[c2rust::src_loc = "223:1"]
    pub unsafe extern "C" fn socket_connect(mut fd: libc::c_int,
                                            mut addr: *const sockaddr,
                                            mut addrlen: socklen_t)
     -> libc::c_int {
        let mut st: libc::c_int = 0;
        st = connect(fd, __CONST_SOCKADDR_ARG{__sockaddr__: addr,}, addrlen);
        if st == -(1 as libc::c_int) { return st }
        return st;
    }
    use super::struct_iovec_h::iovec;
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    use super::sys_socket_h::connect;
    /*_PORT_SOCKET_H*/
    /* UNIX or ...?  */
    /* _WIN32 */
    /* Use TMP to avoid compiler warnings and keep things consistent with
 * Windows version. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:41"]
pub mod k5_buf_h {
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
    #[c2rust::src_loc = "129:1"]
    pub unsafe extern "C" fn k5_buf_add_uint32_be(mut buf: *mut k5buf,
                                                  mut val: uint32_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 4 as libc::c_int as size_t);
        if !p.is_null() { store_32_be(val, p); };
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_platform_h::store_32_be;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
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
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-input.h:42"]
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
    use super::stdint_uintn_h::uint32_t;
    use super::k5_platform_h::load_32_be;
    /* K5_BUF_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kcm.h:44"]
pub mod kcm_h {
    #[c2rust::src_loc = "63:9"]
    pub type kcm_opcode = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const KCM_OP_GET_NTLM_USER_LIST: kcm_opcode = 28;
    #[c2rust::src_loc = "91:5"]
    pub const KCM_OP_DO_NTLM_AUTH: kcm_opcode = 27;
    #[c2rust::src_loc = "90:5"]
    pub const KCM_OP_DEL_NTLM_CRED: kcm_opcode = 26;
    #[c2rust::src_loc = "89:5"]
    pub const KCM_OP_HAVE_NTLM_CRED: kcm_opcode = 25;
    #[c2rust::src_loc = "88:5"]
    pub const KCM_OP_ADD_NTLM_CRED: kcm_opcode = 24;
    #[c2rust::src_loc = "87:5"]
    pub const KCM_OP_SET_KDC_OFFSET: kcm_opcode = 23;
    #[c2rust::src_loc = "86:5"]
    pub const KCM_OP_GET_KDC_OFFSET: kcm_opcode = 22;
    #[c2rust::src_loc = "85:5"]
    pub const KCM_OP_SET_DEFAULT_CACHE: kcm_opcode = 21;
    #[c2rust::src_loc = "84:5"]
    pub const KCM_OP_GET_DEFAULT_CACHE: kcm_opcode = 20;
    #[c2rust::src_loc = "83:5"]
    pub const KCM_OP_GET_CACHE_BY_UUID: kcm_opcode = 19;
    #[c2rust::src_loc = "82:5"]
    pub const KCM_OP_GET_CACHE_UUID_LIST: kcm_opcode = 18;
    #[c2rust::src_loc = "81:5"]
    pub const KCM_OP_MOVE_CACHE: kcm_opcode = 17;
    #[c2rust::src_loc = "80:5"]
    pub const KCM_OP_GET_TICKET: kcm_opcode = 16;
    #[c2rust::src_loc = "79:5"]
    pub const KCM_OP_GET_INITIAL_TICKET: kcm_opcode = 15;
    #[c2rust::src_loc = "78:5"]
    pub const KCM_OP_CHMOD: kcm_opcode = 14;
    #[c2rust::src_loc = "77:5"]
    pub const KCM_OP_CHOWN: kcm_opcode = 13;
    #[c2rust::src_loc = "76:5"]
    pub const KCM_OP_SET_FLAGS: kcm_opcode = 12;
    #[c2rust::src_loc = "75:5"]
    pub const KCM_OP_REMOVE_CRED: kcm_opcode = 11;
    #[c2rust::src_loc = "74:5"]
    pub const KCM_OP_GET_CRED_BY_UUID: kcm_opcode = 10;
    #[c2rust::src_loc = "73:5"]
    pub const KCM_OP_GET_CRED_UUID_LIST: kcm_opcode = 9;
    #[c2rust::src_loc = "72:5"]
    pub const KCM_OP_GET_PRINCIPAL: kcm_opcode = 8;
    #[c2rust::src_loc = "71:5"]
    pub const KCM_OP_RETRIEVE: kcm_opcode = 7;
    #[c2rust::src_loc = "70:5"]
    pub const KCM_OP_STORE: kcm_opcode = 6;
    #[c2rust::src_loc = "69:5"]
    pub const KCM_OP_DESTROY: kcm_opcode = 5;
    #[c2rust::src_loc = "68:5"]
    pub const KCM_OP_INITIALIZE: kcm_opcode = 4;
    #[c2rust::src_loc = "67:5"]
    pub const KCM_OP_GEN_NEW: kcm_opcode = 3;
    #[c2rust::src_loc = "66:5"]
    pub const KCM_OP_RESOLVE: kcm_opcode = 2;
    #[c2rust::src_loc = "65:5"]
    pub const KCM_OP_GET_NAME: kcm_opcode = 1;
    #[c2rust::src_loc = "64:5"]
    pub const KCM_OP_NOOP: kcm_opcode = 0;
    /* KCM_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:41"]
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
#[c2rust::header_src = "/usr/include/errno.h:41"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:41"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:41"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:41"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:45"]
pub mod os_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    use super::port_sockets_h::sg_buf;
    extern "C" {
        /* The io vector is *not* const here, unlike writev()!  */
        #[no_mangle]
        #[c2rust::src_loc = "159:1"]
        pub fn krb5int_net_writev(_: krb5_context, _: libc::c_int,
                                  _: *mut sg_buf, _: libc::c_int)
         -> libc::c_int;
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __socklen_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_iovec_h::iovec;
pub use self::unistd_h::{socklen_t, close};
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_os_mutex_destroy};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
                              store_16_be, store_32_be, load_32_be,
                              krb5int_strlcpy};
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
                       krb5_cc_default_name, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_net_read};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_get_string,
                          profile_release_string};
pub use self::cc_int_h::{_krb5_ccache, _krb5_cc_ops, krb5_cc_ptcursor,
                         krb5_cc_ptcursor_s, k5_cc_mutex, _k5_cc_mutex,
                         k5_cc_retrieve_cred_default, k5_cc_mutex_init,
                         k5_cc_mutex_lock, k5_cc_mutex_unlock,
                         k5_unmarshal_cred, k5_unmarshal_princ,
                         k5_marshal_cred, k5_marshal_mcred, k5_marshal_princ};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::sys_socket_h::{__CONST_SOCKADDR_ARG, sockaddr_x25, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, connect};
pub use self::un_h::sockaddr_un;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_2, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::port_sockets_h::{sg_buf, socket_connect};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf,
                         k5_buf_add_uint32_be, k5_buf_init_dynamic,
                         k5_buf_add_len, k5_buf_get_space, k5_buf_status,
                         k5_buf_free};
pub use self::k5_input_h::{k5input, k5_input_init, k5_input_set_status,
                           k5_input_get_bytes, k5_input_get_uint32_be};
pub use self::kcm_h::{kcm_opcode, KCM_OP_GET_NTLM_USER_LIST,
                      KCM_OP_DO_NTLM_AUTH, KCM_OP_DEL_NTLM_CRED,
                      KCM_OP_HAVE_NTLM_CRED, KCM_OP_ADD_NTLM_CRED,
                      KCM_OP_SET_KDC_OFFSET, KCM_OP_GET_KDC_OFFSET,
                      KCM_OP_SET_DEFAULT_CACHE, KCM_OP_GET_DEFAULT_CACHE,
                      KCM_OP_GET_CACHE_BY_UUID, KCM_OP_GET_CACHE_UUID_LIST,
                      KCM_OP_MOVE_CACHE, KCM_OP_GET_TICKET,
                      KCM_OP_GET_INITIAL_TICKET, KCM_OP_CHMOD, KCM_OP_CHOWN,
                      KCM_OP_SET_FLAGS, KCM_OP_REMOVE_CRED,
                      KCM_OP_GET_CRED_BY_UUID, KCM_OP_GET_CRED_UUID_LIST,
                      KCM_OP_GET_PRINCIPAL, KCM_OP_RETRIEVE, KCM_OP_STORE,
                      KCM_OP_DESTROY, KCM_OP_INITIALIZE, KCM_OP_GEN_NEW,
                      KCM_OP_RESOLVE, KCM_OP_GET_NAME, KCM_OP_NOOP};
use self::stdlib_h::{malloc, calloc, free};
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::string_h::{strlen, strdup, strncmp, strcmp, memchr, memset, memcpy};
pub use self::byteswap_h::{__bswap_32, __bswap_16};
use self::os_proto_h::krb5int_net_writev;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "58:8"]
pub struct uuid_list {
    pub uuidbytes: *mut libc::c_uchar,
    pub count: size_t,
    pub pos: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "80:8"]
pub struct kcm_cache_data {
    pub residual: *mut libc::c_char,
    pub lock: k5_cc_mutex,
    pub io: *mut kcmio,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "64:8"]
pub struct kcmio {
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "86:8"]
pub struct kcm_ptcursor {
    pub residual: *mut libc::c_char,
    pub uuids: *mut uuid_list,
    pub io: *mut kcmio,
    pub first: krb5_boolean,
}
/* This structure bundles together a KCM request and reply, to minimize how
 * much we have to declare and clean up in each method. */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "73:8"]
pub struct kcmreq {
    pub reqbuf: k5buf,
    pub reply: k5input,
    pub reply_mem: *mut libc::c_void,
}
/* Map EINVAL or KRB5_CC_FORMAT to KRB5_KCM_MALFORMED_REPLY; pass through all
 * other codes. */
#[inline]
#[c2rust::src_loc = "95:1"]
unsafe extern "C" fn map_invalid(mut code: krb5_error_code)
 -> krb5_error_code {
    return if code == 22 as libc::c_int ||
                  code as libc::c_long == -(1765328185 as libc::c_long) {
               -(1750600184 as libc::c_long)
           } else { code as libc::c_long } as krb5_error_code;
}
/* Begin a request for the given opcode.  If cache is non-null, supply the
 * cache name as a request parameter. */
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn kcmreq_init(mut req: *mut kcmreq, mut opcode: kcm_opcode,
                                 mut cache: krb5_ccache) {
    let mut bytes: [libc::c_uchar; 4] = [0; 4];
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    memset(req as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kcmreq>() as libc::c_ulong);
    bytes[0 as libc::c_int as usize] = 2 as libc::c_int as libc::c_uchar;
    bytes[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    store_16_be(opcode as libc::c_uint,
                bytes.as_mut_ptr().offset(2 as libc::c_int as isize) as
                    *mut libc::c_void);
    k5_buf_init_dynamic(&mut (*req).reqbuf);
    k5_buf_add_len(&mut (*req).reqbuf,
                   bytes.as_mut_ptr() as *const libc::c_void,
                   4 as libc::c_int as size_t);
    if !cache.is_null() {
        name = (*((*cache).data as *mut kcm_cache_data)).residual;
        k5_buf_add_len(&mut (*req).reqbuf, name as *const libc::c_void,
                       strlen(name).wrapping_add(1 as libc::c_int as
                                                     libc::c_ulong));
    };
}
/* __APPLE__ */
/* Connect to the KCM daemon via a Unix domain socket. */
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn kcmio_unix_socket_connect(mut context: krb5_context,
                                               mut io: *mut kcmio)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut fd: libc::c_int = !(0 as libc::c_int);
    let mut addr: sockaddr_un =
        sockaddr_un{sun_family: 0, sun_path: [0; 108],};
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    ret =
        profile_get_string((*context).profile,
                           b"libdefaults\x00" as *const u8 as
                               *const libc::c_char,
                           b"kcm_socket\x00" as *const u8 as
                               *const libc::c_char, 0 as *const libc::c_char,
                           b"/var/run/.heim_org.h5l.kcm-socket\x00" as
                               *const u8 as *const libc::c_char, &mut path) as
            krb5_error_code;
    if !(ret != 0) {
        if strcmp(path, b"-\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            ret = -(1750600181 as libc::c_long) as krb5_error_code
        } else {
            fd =
                socket(1 as libc::c_int, SOCK_STREAM as libc::c_int,
                       0 as libc::c_int);
            if fd == !(0 as libc::c_int) {
                ret = *__errno_location()
            } else {
                memset(&mut addr as *mut sockaddr_un as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong);
                addr.sun_family = 1 as libc::c_int as sa_family_t;
                krb5int_strlcpy(addr.sun_path.as_mut_ptr(), path,
                                ::std::mem::size_of::<[libc::c_char; 108]>()
                                    as libc::c_ulong);
                if socket_connect(fd,
                                  &mut addr as *mut sockaddr_un as
                                      *mut sockaddr,
                                  ::std::mem::size_of::<sockaddr_un>() as
                                      libc::c_ulong as socklen_t) !=
                       0 as libc::c_int {
                    ret =
                        if *__errno_location() == 2 as libc::c_int {
                            -(1750600181 as libc::c_long)
                        } else { *__errno_location() as libc::c_long } as
                            krb5_error_code
                } else { (*io).fd = fd; fd = !(0 as libc::c_int) }
            }
        }
    }
    if fd != !(0 as libc::c_int) { close(fd); }
    profile_release_string(path);
    return ret;
}
/* Write a KCM request: 4-byte big-endian length, then the marshalled
 * request. */
#[c2rust::src_loc = "285:1"]
unsafe extern "C" fn kcmio_unix_socket_write(mut context: krb5_context,
                                             mut io: *mut kcmio,
                                             mut request: *mut libc::c_void,
                                             mut len: size_t)
 -> krb5_error_code {
    let mut lenbytes: [libc::c_char; 4] = [0; 4];
    let mut sg: [sg_buf; 2] =
        [sg_buf{iov_base: 0 as *mut libc::c_void, iov_len: 0,}; 2];
    let mut ret: libc::c_int = 0;
    let mut reconnected: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    sg[0 as libc::c_int as usize].iov_base =
        lenbytes.as_mut_ptr() as *mut libc::c_void;
    sg[0 as libc::c_int as usize].iov_len =
        ::std::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong;
    sg[1 as libc::c_int as usize].iov_base =
        request as *mut libc::c_char as *mut libc::c_void;
    sg[1 as libc::c_int as usize].iov_len = len;
    store_32_be(len as libc::c_uint,
                lenbytes.as_mut_ptr() as *mut libc::c_void);
    loop  {
        ret =
            krb5int_net_writev(context, (*io).fd, sg.as_mut_ptr(),
                               2 as libc::c_int);
        if ret >= 0 as libc::c_int { return 0 as libc::c_int }
        ret = *__errno_location();
        if ret != 32 as libc::c_int || reconnected != 0 { return ret }
        /*
         * Try once to reconnect on an EPIPE, in case the server has an idle
         * timeout (like sssd does) and we went too long between ccache
         * operations.  Reconnecting might also help if the server was
         * restarted for an upgrade--although the server must be designed to
         * always listen for connections on the socket during upgrades, or a
         * single reconnect attempt won't be robust.
         */
        close((*io).fd);
        ret = kcmio_unix_socket_connect(context, io);
        if ret != 0 { return ret }
        reconnected = 1 as libc::c_int as krb5_boolean
    };
}
/* Read a KCM reply: 4-byte big-endian length, 4-byte big-endian status code,
 * then the marshalled reply. */
#[c2rust::src_loc = "324:1"]
unsafe extern "C" fn kcmio_unix_socket_read(mut context: krb5_context,
                                            mut io: *mut kcmio,
                                            mut reply_out:
                                                *mut *mut libc::c_void,
                                            mut len_out: *mut size_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut lenbytes: [libc::c_char; 4] = [0; 4];
    let mut codebytes: [libc::c_char; 4] = [0; 4];
    let mut reply: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut st: libc::c_int = 0;
    *reply_out = 0 as *mut libc::c_void;
    *len_out = 0 as libc::c_int as size_t;
    st =
        krb5_net_read(context, (*io).fd, lenbytes.as_mut_ptr(),
                      4 as libc::c_int);
    if st != 4 as libc::c_int {
        return if st == -(1 as libc::c_int) {
                   *__errno_location() as libc::c_long
               } else { -(1765328191 as libc::c_long) } as krb5_error_code
    }
    len = load_32_be(lenbytes.as_mut_ptr() as *const libc::c_void) as size_t;
    if len >
           (10 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as
               libc::c_ulong {
        return -(1750600182 as libc::c_long) as krb5_error_code
    }
    st =
        krb5_net_read(context, (*io).fd, codebytes.as_mut_ptr(),
                      4 as libc::c_int);
    if st != 4 as libc::c_int {
        return if st == -(1 as libc::c_int) {
                   *__errno_location() as libc::c_long
               } else { -(1765328191 as libc::c_long) } as krb5_error_code
    }
    code =
        load_32_be(codebytes.as_mut_ptr() as *const libc::c_void) as
            krb5_error_code;
    if code != 0 as libc::c_int { return code }
    reply = malloc(len) as *mut libc::c_char;
    if reply.is_null() { return 12 as libc::c_int }
    st = krb5_net_read(context, (*io).fd, reply, len as libc::c_int);
    if st == -(1 as libc::c_int) || st as size_t != len {
        free(reply as *mut libc::c_void);
        return if st < 0 as libc::c_int {
                   *__errno_location() as libc::c_long
               } else { -(1765328191 as libc::c_long) } as krb5_error_code
    }
    *reply_out = reply as *mut libc::c_void;
    *len_out = len;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "364:1"]
unsafe extern "C" fn kcmio_connect(mut context: krb5_context,
                                   mut io_out: *mut *mut kcmio)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut io: *mut kcmio = 0 as *mut kcmio;
    *io_out = 0 as *mut kcmio;
    io =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<kcmio>() as libc::c_ulong) as *mut kcmio;
    if io.is_null() { return 12 as libc::c_int }
    (*io).fd = !(0 as libc::c_int);
    /* Try Mach RPC (macOS only), then fall back to Unix domain sockets */
    ret = 22 as libc::c_int;
    if ret != 0 { ret = kcmio_unix_socket_connect(context, io) }
    if ret != 0 { free(io as *mut libc::c_void); return ret }
    *io_out = io;
    return 0 as libc::c_int;
}
/* Check req->reqbuf for an error condition and return it.  Otherwise, send the
 * request to the KCM daemon and get a response. */
#[c2rust::src_loc = "391:1"]
unsafe extern "C" fn kcmio_call(mut context: krb5_context, mut io: *mut kcmio,
                                mut req: *mut kcmreq) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut reply_len: size_t = 0 as libc::c_int as size_t;
    if k5_buf_status(&mut (*req).reqbuf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    if (*io).fd != !(0 as libc::c_int) {
        ret =
            kcmio_unix_socket_write(context, io, (*req).reqbuf.data,
                                    (*req).reqbuf.len);
        if ret != 0 { return ret }
        ret =
            kcmio_unix_socket_read(context, io, &mut (*req).reply_mem,
                                   &mut reply_len);
        if ret != 0 { return ret }
    } else {
        /* We must be using Mach RPC. */
        ret = 22 as libc::c_int;
        if ret != 0 { return ret }
    }
    /* Read the status code from the marshalled reply. */
    k5_input_init(&mut (*req).reply, (*req).reply_mem, reply_len);
    ret = k5_input_get_uint32_be(&mut (*req).reply) as krb5_error_code;
    return if (*req).reply.status != 0 {
               -(1750600184 as libc::c_long)
           } else { ret as libc::c_long } as krb5_error_code;
}
#[c2rust::src_loc = "422:1"]
unsafe extern "C" fn kcmio_close(mut io: *mut kcmio) {
    if !io.is_null() {
        if (*io).fd != !(0 as libc::c_int) { close((*io).fd); }
        free(io as *mut libc::c_void);
    };
}
/* Fetch a zero-terminated name string from req->reply.  The returned pointer
 * is an alias and must not be freed by the caller. */
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn kcmreq_get_name(mut req: *mut kcmreq,
                                     mut name_out: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut end: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut in_0: *mut k5input = &mut (*req).reply;
    *name_out = 0 as *const libc::c_char;
    end =
        memchr((*in_0).ptr as *const libc::c_void, '\u{0}' as i32,
               (*in_0).len) as *const libc::c_uchar;
    if end.is_null() {
        return -(1750600184 as libc::c_long) as krb5_error_code
    }
    *name_out = (*in_0).ptr as *const libc::c_char;
    k5_input_get_bytes(in_0,
                       end.offset(1 as libc::c_int as
                                      isize).wrapping_offset_from((*in_0).ptr)
                           as libc::c_long as size_t);
    return 0 as libc::c_int;
}
/* Fetch a UUID list from req->reply.  UUID lists are not delimited, so we
 * consume the rest of the input. */
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn kcmreq_get_uuid_list(mut req: *mut kcmreq,
                                          mut uuids_out: *mut *mut uuid_list)
 -> krb5_error_code {
    let mut uuids: *mut uuid_list = 0 as *mut uuid_list;
    *uuids_out = 0 as *mut uuid_list;
    if (*req).reply.len.wrapping_rem(16 as libc::c_int as libc::c_ulong) !=
           0 as libc::c_int as libc::c_ulong {
        return -(1750600184 as libc::c_long) as krb5_error_code
    }
    uuids =
        malloc(::std::mem::size_of::<uuid_list>() as libc::c_ulong) as
            *mut uuid_list;
    if uuids.is_null() { return 12 as libc::c_int }
    (*uuids).count =
        (*req).reply.len.wrapping_div(16 as libc::c_int as libc::c_ulong);
    (*uuids).pos = 0 as libc::c_int as size_t;
    if (*req).reply.len > 0 as libc::c_int as libc::c_ulong {
        (*uuids).uuidbytes = malloc((*req).reply.len) as *mut libc::c_uchar;
        if (*uuids).uuidbytes.is_null() {
            free(uuids as *mut libc::c_void);
            return 12 as libc::c_int
        }
        memcpy((*uuids).uuidbytes as *mut libc::c_void,
               (*req).reply.ptr as *const libc::c_void, (*req).reply.len);
        k5_input_get_bytes(&mut (*req).reply, (*req).reply.len);
    } else { (*uuids).uuidbytes = 0 as *mut libc::c_uchar }
    *uuids_out = uuids;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "484:1"]
unsafe extern "C" fn free_uuid_list(mut uuids: *mut uuid_list) {
    if !uuids.is_null() { free((*uuids).uuidbytes as *mut libc::c_void); }
    free(uuids as *mut libc::c_void);
}
#[c2rust::src_loc = "492:1"]
unsafe extern "C" fn kcmreq_free(mut req: *mut kcmreq) {
    k5_buf_free(&mut (*req).reqbuf);
    free((*req).reply_mem);
}
/* Create a krb5_ccache structure.  If io is NULL, make a new connection for
 * the cache.  Otherwise, always take ownership of io. */
#[c2rust::src_loc = "501:1"]
unsafe extern "C" fn make_cache(mut context: krb5_context,
                                mut residual: *const libc::c_char,
                                mut io: *mut kcmio,
                                mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut cache: krb5_ccache = 0 as krb5_ccache;
    let mut data: *mut kcm_cache_data = 0 as *mut kcm_cache_data;
    let mut residual_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    *cache_out = 0 as krb5_ccache;
    if io.is_null() {
        ret = kcmio_connect(context, &mut io);
        if ret != 0 { return ret }
    }
    cache =
        malloc(::std::mem::size_of::<_krb5_ccache>() as libc::c_ulong) as
            krb5_ccache;
    if !cache.is_null() {
        data =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<kcm_cache_data>() as libc::c_ulong)
                as *mut kcm_cache_data;
        if !data.is_null() {
            residual_copy = strdup(residual);
            if !residual_copy.is_null() {
                if !(k5_cc_mutex_init(&mut (*data).lock) != 0 as libc::c_int)
                   {
                    (*data).residual = residual_copy;
                    (*data).io = io;
                    (*cache).ops = &krb5_kcm_ops;
                    (*cache).data = data as krb5_pointer;
                    (*cache).magic =
                        -(1760647380 as libc::c_long) as krb5_magic;
                    *cache_out = cache;
                    return 0 as libc::c_int
                }
            }
        }
    }
    free(cache as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(residual_copy as *mut libc::c_void);
    kcmio_close(io);
    return 12 as libc::c_int;
}
/* Lock cache's I/O structure and use it to call the KCM daemon. */
#[c2rust::src_loc = "547:1"]
unsafe extern "C" fn cache_call(mut context: krb5_context,
                                mut cache: krb5_ccache, mut req: *mut kcmreq)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut data: *mut kcm_cache_data = (*cache).data as *mut kcm_cache_data;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    ret = kcmio_call(context, (*data).io, req);
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return ret;
}
/* Try to propagate the KDC time offset from the cache to the krb5 context. */
#[c2rust::src_loc = "560:1"]
unsafe extern "C" fn get_kdc_offset(mut context: krb5_context,
                                    mut cache: krb5_ccache) {
    let mut req: kcmreq =
        {
            let mut init =
                kcmreq{reqbuf:
                           {
                               let mut init =
                                   k5buf{buftype: K5BUF_ERROR,
                                         data: 0 as *mut libc::c_void,
                                         space: 0,
                                         len: 0,};
                               init
                           },
                       reply:
                           k5input{ptr: 0 as *const libc::c_uchar,
                                   len: 0,
                                   status: 0,},
                       reply_mem: 0 as *mut libc::c_void,};
            init
        };
    let mut time_offset: int32_t = 0;
    kcmreq_init(&mut req, KCM_OP_GET_KDC_OFFSET, cache);
    if !(cache_call(context, cache, &mut req) != 0 as libc::c_int) {
        time_offset = k5_input_get_uint32_be(&mut req.reply) as int32_t;
        if !(req.reply.status != 0) {
            (*context).os_context.time_offset = time_offset;
            (*context).os_context.usec_offset = 0 as libc::c_int;
            (*context).os_context.os_flags &= !(2 as libc::c_int);
            (*context).os_context.os_flags |= 1 as libc::c_int
        }
    }
    kcmreq_free(&mut req);
}
/* Try to propagate the KDC offset from the krb5 context to the cache. */
#[c2rust::src_loc = "582:1"]
unsafe extern "C" fn set_kdc_offset(mut context: krb5_context,
                                    mut cache: krb5_ccache) {
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    if (*context).os_context.os_flags & 1 as libc::c_int != 0 {
        kcmreq_init(&mut req, KCM_OP_SET_KDC_OFFSET, cache);
        k5_buf_add_uint32_be(&mut req.reqbuf,
                             (*context).os_context.time_offset as uint32_t);
        cache_call(context, cache, &mut req);
        kcmreq_free(&mut req);
    };
}
#[c2rust::src_loc = "595:1"]
unsafe extern "C" fn kcm_get_name(mut context: krb5_context,
                                  mut cache: krb5_ccache)
 -> *const libc::c_char {
    return (*((*cache).data as *mut kcm_cache_data)).residual;
}
#[c2rust::src_loc = "601:1"]
unsafe extern "C" fn kcm_resolve(mut context: krb5_context,
                                 mut cache_out: *mut krb5_ccache,
                                 mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        {
            let mut init =
                kcmreq{reqbuf:
                           {
                               let mut init =
                                   k5buf{buftype: K5BUF_ERROR,
                                         data: 0 as *mut libc::c_void,
                                         space: 0,
                                         len: 0,};
                               init
                           },
                       reply:
                           k5input{ptr: 0 as *const libc::c_uchar,
                                   len: 0,
                                   status: 0,},
                       reply_mem: 0 as *mut libc::c_void,};
            init
        };
    let mut io: *mut kcmio = 0 as *mut kcmio;
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    *cache_out = 0 as krb5_ccache;
    ret = kcmio_connect(context, &mut io);
    if !(ret != 0) {
        if *residual as libc::c_int == '\u{0}' as i32 {
            kcmreq_init(&mut req, KCM_OP_GET_DEFAULT_CACHE, 0 as krb5_ccache);
            ret = kcmio_call(context, io, &mut req);
            if ret != 0 {
                current_block = 4152304949744080010;
            } else {
                ret = kcmreq_get_name(&mut req, &mut defname);
                if ret != 0 {
                    current_block = 4152304949744080010;
                } else {
                    residual = defname;
                    current_block = 5399440093318478209;
                }
            }
        } else { current_block = 5399440093318478209; }
        match current_block {
            4152304949744080010 => { }
            _ => {
                ret = make_cache(context, residual, io, cache_out);
                io = 0 as *mut kcmio
            }
        }
    }
    kcmio_close(io);
    kcmreq_free(&mut req);
    return ret;
}
#[c2rust::src_loc = "635:1"]
unsafe extern "C" fn kcm_gen_new(mut context: krb5_context,
                                 mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        {
            let mut init =
                kcmreq{reqbuf:
                           {
                               let mut init =
                                   k5buf{buftype: K5BUF_ERROR,
                                         data: 0 as *mut libc::c_void,
                                         space: 0,
                                         len: 0,};
                               init
                           },
                       reply:
                           k5input{ptr: 0 as *const libc::c_uchar,
                                   len: 0,
                                   status: 0,},
                       reply_mem: 0 as *mut libc::c_void,};
            init
        };
    let mut io: *mut kcmio = 0 as *mut kcmio;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    *cache_out = 0 as krb5_ccache;
    ret = kcmio_connect(context, &mut io);
    if !(ret != 0) {
        kcmreq_init(&mut req, KCM_OP_GEN_NEW, 0 as krb5_ccache);
        ret = kcmio_call(context, io, &mut req);
        if !(ret != 0) {
            ret = kcmreq_get_name(&mut req, &mut name);
            if !(ret != 0) {
                ret = make_cache(context, name, io, cache_out);
                io = 0 as *mut kcmio
            }
        }
    }
    kcmreq_free(&mut req);
    kcmio_close(io);
    return ret;
}
#[c2rust::src_loc = "664:1"]
unsafe extern "C" fn kcm_initialize(mut context: krb5_context,
                                    mut cache: krb5_ccache,
                                    mut princ: krb5_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    kcmreq_init(&mut req, KCM_OP_INITIALIZE, cache);
    k5_marshal_princ(&mut req.reqbuf, 4 as libc::c_int, princ);
    ret = cache_call(context, cache, &mut req);
    kcmreq_free(&mut req);
    set_kdc_offset(context, cache);
    return ret;
}
#[c2rust::src_loc = "678:1"]
unsafe extern "C" fn kcm_close(mut context: krb5_context,
                               mut cache: krb5_ccache) -> krb5_error_code {
    let mut data: *mut kcm_cache_data = (*cache).data as *mut kcm_cache_data;
    k5_os_mutex_destroy(&mut (*data).lock.lock);
    kcmio_close((*data).io);
    free((*data).residual as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(cache as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "691:1"]
unsafe extern "C" fn kcm_destroy(mut context: krb5_context,
                                 mut cache: krb5_ccache) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    kcmreq_init(&mut req, KCM_OP_DESTROY, cache);
    ret = cache_call(context, cache, &mut req);
    kcmreq_free(&mut req);
    kcm_close(context, cache);
    return ret;
}
#[c2rust::src_loc = "704:1"]
unsafe extern "C" fn kcm_store(mut context: krb5_context,
                               mut cache: krb5_ccache,
                               mut cred: *mut krb5_creds) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    kcmreq_init(&mut req, KCM_OP_STORE, cache);
    k5_marshal_cred(&mut req.reqbuf, 4 as libc::c_int, cred);
    ret = cache_call(context, cache, &mut req);
    kcmreq_free(&mut req);
    return ret;
}
#[c2rust::src_loc = "717:1"]
unsafe extern "C" fn kcm_retrieve(mut context: krb5_context,
                                  mut cache: krb5_ccache,
                                  mut flags: krb5_flags,
                                  mut mcred: *mut krb5_creds,
                                  mut cred_out: *mut krb5_creds)
 -> krb5_error_code {
    /* There is a KCM opcode for retrieving creds, but Heimdal's client doesn't
     * use it.  It causes the KCM daemon to actually make a TGS request. */
    return k5_cc_retrieve_cred_default(context, cache, flags, mcred,
                                       cred_out);
}
#[c2rust::src_loc = "726:1"]
unsafe extern "C" fn kcm_get_princ(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut princ_out: *mut krb5_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    let mut data: *mut kcm_cache_data = (*cache).data as *mut kcm_cache_data;
    kcmreq_init(&mut req, KCM_OP_GET_PRINCIPAL, cache);
    ret = cache_call(context, cache, &mut req);
    /* Heimdal KCM can respond with code 0 and no principal. */
    if ret == 0 && req.reply.len == 0 as libc::c_int as libc::c_ulong {
        ret = -(1765328189 as libc::c_long) as krb5_error_code
    }
    if ret as libc::c_long == -(1765328189 as libc::c_long) {
        krb5_set_error_message(context, ret,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Credentials cache \'KCM:%s\' not found\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               (*data).residual);
    }
    if ret == 0 {
        ret =
            k5_unmarshal_princ(req.reply.ptr, req.reply.len, 4 as libc::c_int,
                               princ_out)
    }
    kcmreq_free(&mut req);
    return map_invalid(ret);
}
#[c2rust::src_loc = "750:1"]
unsafe extern "C" fn kcm_start_seq_get(mut context: krb5_context,
                                       mut cache: krb5_ccache,
                                       mut cursor_out: *mut krb5_cc_cursor)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        {
            let mut init =
                kcmreq{reqbuf:
                           {
                               let mut init =
                                   k5buf{buftype: K5BUF_ERROR,
                                         data: 0 as *mut libc::c_void,
                                         space: 0,
                                         len: 0,};
                               init
                           },
                       reply:
                           k5input{ptr: 0 as *const libc::c_uchar,
                                   len: 0,
                                   status: 0,},
                       reply_mem: 0 as *mut libc::c_void,};
            init
        };
    let mut uuids: *mut uuid_list = 0 as *mut uuid_list;
    *cursor_out = 0 as *mut libc::c_void;
    get_kdc_offset(context, cache);
    kcmreq_init(&mut req, KCM_OP_GET_CRED_UUID_LIST, cache);
    ret = cache_call(context, cache, &mut req);
    if !(ret != 0) {
        ret = kcmreq_get_uuid_list(&mut req, &mut uuids);
        if !(ret != 0) { *cursor_out = uuids as krb5_cc_cursor }
    }
    kcmreq_free(&mut req);
    return ret;
}
#[c2rust::src_loc = "776:1"]
unsafe extern "C" fn kcm_next_cred(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut cursor: *mut krb5_cc_cursor,
                                   mut cred_out: *mut krb5_creds)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    let mut uuids: *mut uuid_list = *cursor as *mut uuid_list;
    memset(cred_out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    if (*uuids).pos >= (*uuids).count {
        return -(1765328242 as libc::c_long) as krb5_error_code
    }
    kcmreq_init(&mut req, KCM_OP_GET_CRED_BY_UUID, cache);
    k5_buf_add_len(&mut req.reqbuf,
                   (*uuids).uuidbytes.offset((*uuids).pos.wrapping_mul(16 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                                 as isize) as
                       *const libc::c_void, 16 as libc::c_int as size_t);
    (*uuids).pos = (*uuids).pos.wrapping_add(1);
    ret = cache_call(context, cache, &mut req);
    if ret == 0 {
        ret =
            k5_unmarshal_cred(req.reply.ptr, req.reply.len, 4 as libc::c_int,
                              cred_out)
    }
    kcmreq_free(&mut req);
    return map_invalid(ret);
}
#[c2rust::src_loc = "800:1"]
unsafe extern "C" fn kcm_end_seq_get(mut context: krb5_context,
                                     mut cache: krb5_ccache,
                                     mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    free_uuid_list(*cursor as *mut uuid_list);
    *cursor = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "809:1"]
unsafe extern "C" fn kcm_remove_cred(mut context: krb5_context,
                                     mut cache: krb5_ccache,
                                     mut flags: krb5_flags,
                                     mut mcred: *mut krb5_creds)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    kcmreq_init(&mut req, KCM_OP_REMOVE_CRED, cache);
    k5_buf_add_uint32_be(&mut req.reqbuf, flags as uint32_t);
    k5_marshal_mcred(&mut req.reqbuf, mcred);
    ret = cache_call(context, cache, &mut req);
    kcmreq_free(&mut req);
    return ret;
}
#[c2rust::src_loc = "824:1"]
unsafe extern "C" fn kcm_set_flags(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut flags: krb5_flags) -> krb5_error_code {
    /* We don't currently care about any flags for this type. */
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "831:1"]
unsafe extern "C" fn kcm_get_flags(mut context: krb5_context,
                                   mut cache: krb5_ccache,
                                   mut flags_out: *mut krb5_flags)
 -> krb5_error_code {
    /* We don't currently have any operational flags for this type. */
    *flags_out = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/* Construct a per-type cursor, always taking ownership of io and uuids. */
#[c2rust::src_loc = "840:1"]
unsafe extern "C" fn make_ptcursor(mut residual: *const libc::c_char,
                                   mut uuids: *mut uuid_list,
                                   mut io: *mut kcmio,
                                   mut cursor_out: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut cursor: krb5_cc_ptcursor = 0 as krb5_cc_ptcursor;
    let mut data: *mut kcm_ptcursor = 0 as *mut kcm_ptcursor;
    let mut residual_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    *cursor_out = 0 as krb5_cc_ptcursor;
    if !residual.is_null() {
        residual_copy = strdup(residual);
        if residual_copy.is_null() {
            current_block = 5937662909346385725;
        } else { current_block = 11875828834189669668; }
    } else { current_block = 11875828834189669668; }
    match current_block {
        11875828834189669668 => {
            cursor =
                malloc(::std::mem::size_of::<krb5_cc_ptcursor_s>() as
                           libc::c_ulong) as krb5_cc_ptcursor;
            if !cursor.is_null() {
                data =
                    malloc(::std::mem::size_of::<kcm_ptcursor>() as
                               libc::c_ulong) as *mut kcm_ptcursor;
                if !data.is_null() {
                    (*data).residual = residual_copy;
                    (*data).uuids = uuids;
                    (*data).io = io;
                    (*data).first = 1 as libc::c_int as krb5_boolean;
                    (*cursor).ops = &krb5_kcm_ops;
                    (*cursor).data = data as krb5_pointer;
                    *cursor_out = cursor;
                    return 0 as libc::c_int
                }
            }
        }
        _ => { }
    }
    kcmio_close(io);
    free_uuid_list(uuids);
    free(residual_copy as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(cursor as *mut libc::c_void);
    return 12 as libc::c_int;
}
#[c2rust::src_loc = "880:1"]
unsafe extern "C" fn kcm_ptcursor_new(mut context: krb5_context,
                                      mut cursor_out: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        {
            let mut init =
                kcmreq{reqbuf:
                           {
                               let mut init =
                                   k5buf{buftype: K5BUF_ERROR,
                                         data: 0 as *mut libc::c_void,
                                         space: 0,
                                         len: 0,};
                               init
                           },
                       reply:
                           k5input{ptr: 0 as *const libc::c_uchar,
                                   len: 0,
                                   status: 0,},
                       reply_mem: 0 as *mut libc::c_void,};
            init
        };
    let mut io: *mut kcmio = 0 as *mut kcmio;
    let mut uuids: *mut uuid_list = 0 as *mut uuid_list;
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    let mut primary: *const libc::c_char = 0 as *const libc::c_char;
    *cursor_out = 0 as krb5_cc_ptcursor;
    /* Don't try to use KCM for the cache collection unless the default cache
     * name has the KCM type. */
    defname = krb5_cc_default_name(context);
    if defname.is_null() ||
           strncmp(defname, b"KCM:\x00" as *const u8 as *const libc::c_char,
                   4 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
        return make_ptcursor(0 as *const libc::c_char, 0 as *mut uuid_list,
                             0 as *mut kcmio, cursor_out)
    }
    ret = kcmio_connect(context, &mut io);
    if ret != 0 { return ret }
    /* If defname is a subsidiary cache, return a singleton cursor. */
    if strlen(defname) > 4 as libc::c_int as libc::c_ulong {
        return make_ptcursor(defname.offset(4 as libc::c_int as isize),
                             0 as *mut uuid_list, io, cursor_out)
    }
    kcmreq_init(&mut req, KCM_OP_GET_CACHE_UUID_LIST, 0 as krb5_ccache);
    ret = kcmio_call(context, io, &mut req);
    if ret as libc::c_long == -(1765328189 as libc::c_long) {
        /* There are no accessible caches; return an empty cursor. */
        ret =
            make_ptcursor(0 as *const libc::c_char, 0 as *mut uuid_list,
                          0 as *mut kcmio, cursor_out)
    } else if !(ret != 0) {
        ret = kcmreq_get_uuid_list(&mut req, &mut uuids);
        if !(ret != 0) {
            kcmreq_free(&mut req);
            kcmreq_init(&mut req, KCM_OP_GET_DEFAULT_CACHE, 0 as krb5_ccache);
            ret = kcmio_call(context, io, &mut req);
            if !(ret != 0) {
                ret = kcmreq_get_name(&mut req, &mut primary);
                if !(ret != 0) {
                    ret = make_ptcursor(primary, uuids, io, cursor_out);
                    uuids = 0 as *mut uuid_list;
                    io = 0 as *mut kcmio
                }
            }
        }
    }
    free_uuid_list(uuids);
    kcmio_close(io);
    kcmreq_free(&mut req);
    return ret;
}
/* Return true if name is an initialized cache. */
#[c2rust::src_loc = "939:1"]
unsafe extern "C" fn name_exists(mut context: krb5_context,
                                 mut io: *mut kcmio,
                                 mut name: *const libc::c_char)
 -> krb5_boolean {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    kcmreq_init(&mut req, KCM_OP_GET_PRINCIPAL, 0 as krb5_ccache);
    k5_buf_add_len(&mut req.reqbuf, name as *const libc::c_void,
                   strlen(name).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong));
    ret = kcmio_call(context, io, &mut req);
    kcmreq_free(&mut req);
    return (ret == 0 as libc::c_int) as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "952:1"]
unsafe extern "C" fn kcm_ptcursor_next(mut context: krb5_context,
                                       mut cursor: krb5_cc_ptcursor,
                                       mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut req: kcmreq =
        {
            let mut init =
                kcmreq{reqbuf:
                           {
                               let mut init =
                                   k5buf{buftype: K5BUF_ERROR,
                                         data: 0 as *mut libc::c_void,
                                         space: 0,
                                         len: 0,};
                               init
                           },
                       reply:
                           k5input{ptr: 0 as *const libc::c_uchar,
                                   len: 0,
                                   status: 0,},
                       reply_mem: 0 as *mut libc::c_void,};
            init
        };
    let mut data: *mut kcm_ptcursor = (*cursor).data as *mut kcm_ptcursor;
    let mut uuids: *mut uuid_list = 0 as *mut uuid_list;
    let mut id: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    *cache_out = 0 as krb5_ccache;
    /* Return the primary or specified subsidiary cache if we haven't yet. */
    if (*data).first != 0 && !(*data).residual.is_null() {
        (*data).first = 0 as libc::c_int as krb5_boolean;
        if name_exists(context, (*data).io, (*data).residual) != 0 {
            return make_cache(context, (*data).residual, 0 as *mut kcmio,
                              cache_out)
        }
    }
    uuids = (*data).uuids;
    if uuids.is_null() { return 0 as libc::c_int }
    while (*uuids).pos < (*uuids).count {
        /* Get the name of the next cache. */
        let fresh0 = (*uuids).pos;
        (*uuids).pos = (*uuids).pos.wrapping_add(1);
        id =
            &mut *(*uuids).uuidbytes.offset((16 as libc::c_int as
                                                 libc::c_ulong).wrapping_mul(fresh0)
                                                as isize) as
                *mut libc::c_uchar;
        kcmreq_free(&mut req);
        kcmreq_init(&mut req, KCM_OP_GET_CACHE_BY_UUID, 0 as krb5_ccache);
        k5_buf_add_len(&mut req.reqbuf, id as *const libc::c_void,
                       16 as libc::c_int as size_t);
        ret = kcmio_call(context, (*data).io, &mut req);
        /* Continue if the cache has been deleted. */
        if ret as libc::c_long == -(1765328242 as libc::c_long) { continue ; }
        if ret != 0 { break ; }
        ret = kcmreq_get_name(&mut req, &mut name);
        if ret != 0 { break ; }
        /* Don't yield the primary cache twice. */
        if strcmp(name, (*data).residual) == 0 as libc::c_int { continue ; }
        ret = make_cache(context, name, 0 as *mut kcmio, cache_out);
        break ;
    }
    kcmreq_free(&mut req);
    return ret;
}
#[c2rust::src_loc = "1005:1"]
unsafe extern "C" fn kcm_ptcursor_free(mut context: krb5_context,
                                       mut cursor: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut data: *mut kcm_ptcursor = (**cursor).data as *mut kcm_ptcursor;
    free((*data).residual as *mut libc::c_void);
    free_uuid_list((*data).uuids);
    kcmio_close((*data).io);
    free(data as *mut libc::c_void);
    free(*cursor as *mut libc::c_void);
    *cursor = 0 as krb5_cc_ptcursor;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1019:1"]
unsafe extern "C" fn kcm_lock(mut context: krb5_context,
                              mut cache: krb5_ccache) -> krb5_error_code {
    k5_cc_mutex_lock(context,
                     &mut (*((*cache).data as *mut kcm_cache_data)).lock);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1026:1"]
unsafe extern "C" fn kcm_unlock(mut context: krb5_context,
                                mut cache: krb5_ccache) -> krb5_error_code {
    k5_cc_mutex_unlock(context,
                       &mut (*((*cache).data as *mut kcm_cache_data)).lock);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1033:1"]
unsafe extern "C" fn kcm_switch_to(mut context: krb5_context,
                                   mut cache: krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req: kcmreq =
        kcmreq{reqbuf:
                   k5buf{buftype: K5BUF_ERROR,
                         data: 0 as *mut libc::c_void,
                         space: 0,
                         len: 0,},
               reply:
                   k5input{ptr: 0 as *const libc::c_uchar,
                           len: 0,
                           status: 0,},
               reply_mem: 0 as *mut libc::c_void,};
    kcmreq_init(&mut req, KCM_OP_SET_DEFAULT_CACHE, cache);
    ret = cache_call(context, cache, &mut req);
    kcmreq_free(&mut req);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1045:19"]
pub static mut krb5_kcm_ops: krb5_cc_ops =
    unsafe {
        {
            let mut init =
                _krb5_cc_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"KCM\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             get_name:
                                 Some(kcm_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> *const libc::c_char),
                             resolve:
                                 Some(kcm_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache,
                                                               _:
                                                                   *const libc::c_char)
                                              -> krb5_error_code),
                             gen_new:
                                 Some(kcm_gen_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             init:
                                 Some(kcm_initialize as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   krb5_principal)
                                              -> krb5_error_code),
                             destroy:
                                 Some(kcm_destroy as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             close:
                                 Some(kcm_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             store:
                                 Some(kcm_store as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             retrieve:
                                 Some(kcm_retrieve as
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
                                 Some(kcm_get_princ as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_principal)
                                              -> krb5_error_code),
                             get_first:
                                 Some(kcm_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(kcm_next_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             end_get:
                                 Some(kcm_end_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             remove_cred:
                                 Some(kcm_remove_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             set_flags:
                                 Some(kcm_set_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags)
                                              -> krb5_error_code),
                             get_flags:
                                 Some(kcm_get_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_flags)
                                              -> krb5_error_code),
                             ptcursor_new:
                                 Some(kcm_ptcursor_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             ptcursor_next:
                                 Some(kcm_ptcursor_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_cc_ptcursor,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             ptcursor_free:
                                 Some(kcm_ptcursor_free as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             move_0: None,
                             wasdefault: None,
                             lock:
                                 Some(kcm_lock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             unlock:
                                 Some(kcm_unlock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             switch_to:
                                 Some(kcm_switch_to as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),};
            init
        }
    };
/* not _WIN32 */
