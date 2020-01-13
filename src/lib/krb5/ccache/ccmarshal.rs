use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:103"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:103"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:103"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:103"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:103"]
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
    /* Do it in macro form so we get the file/line of the invocation if
   the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
   multiple active threads mucking around in our library at this
   point.  So ignore the once_t object and just look at the flag.

   XXX Could we have problems with memory coherence between processors
   if we don't invoke mutex/once routines?  Probably not, the
   application code should already be coordinating things such that
   the library code is not in use by this point, and memory
   synchronization will be needed there.  */
    /* If we're using gcc, if the C++ support works, the compiler should
   build executables and shared libraries that support the use of
   static constructors and destructors.  The C compiler supports a
   function attribute that makes use of the same facility as C++.

   XXX How do we know if the C++ support actually works?  */
    /* Read and write integer values as (unaligned) octet strings in
   specific byte orders.  Add per-platform optimizations as
   needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
   it.  If both are defined, then BYTE_ORDER should be defined and
   match one of them.  Try those symbols, then try again with an
   underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

   GCC's packed structures can be written to with any alignment; the
   compiler will use byte operations, unaligned-word operations, or
   normal memory ops as appropriate for the architecture.

   This assumes the availability of uint##_t types, which should work
   on most of our platforms except Windows, where we're not using
   GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
   with the indicated numbers of bits.

   Linux: byteswap.h, bswap_16 etc.
   Solaris 10: none
   macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
   NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
   source, so casts *from* void* are required.  */
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
    #[inline]
    #[c2rust::src_loc = "726:1"]
    pub unsafe extern "C" fn store_16_n(mut val: libc::c_uint,
                                        mut vp: *mut libc::c_void) {
        let mut n: uint16_t = val as uint16_t;
        memcpy(vp, &mut n as *mut uint16_t as *const libc::c_void,
               2 as libc::c_int as libc::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "732:1"]
    pub unsafe extern "C" fn store_32_n(mut val: libc::c_uint,
                                        mut vp: *mut libc::c_void) {
        let mut n: uint32_t = val;
        memcpy(vp, &mut n as *mut uint32_t as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "744:1"]
    pub unsafe extern "C" fn load_16_n(mut p: *const libc::c_void)
     -> libc::c_ushort {
        let mut n: uint16_t = 0;
        memcpy(&mut n as *mut uint16_t as *mut libc::c_void, p,
               2 as libc::c_int as libc::c_ulong);
        return n;
    }
    #[inline]
    #[c2rust::src_loc = "751:1"]
    pub unsafe extern "C" fn load_32_n(mut p: *const libc::c_void)
     -> libc::c_uint {
        let mut n: uint32_t = 0;
        memcpy(&mut n as *mut uint32_t as *mut libc::c_void, p,
               4 as libc::c_int as libc::c_ulong);
        return n;
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::byteswap_h::{__bswap_16, __bswap_32};
    use super::types_h::__uint16_t;
    use super::string_h::memcpy;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:103"]
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
    use super::stdint_uintn_h::uint8_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:103"]
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
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    /* Like k5memdup, but add a final null byte. */
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::string_h::memcpy;
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
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:103"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:103"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:103"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:103"]
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
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-input.h:104"]
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
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn k5_input_get_byte(mut in_0: *mut k5input)
     -> libc::c_uchar {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 1 as libc::c_int as size_t);
        return if ptr.is_null() {
                   '\u{0}' as i32
               } else { *ptr as libc::c_int } as libc::c_uchar;
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
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn k5_input_get_uint16_n(mut in_0: *mut k5input)
     -> uint16_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 2 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int
               } else { load_16_n(ptr as *const libc::c_void) as libc::c_int }
                   as uint16_t;
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
    #[inline]
    #[c2rust::src_loc = "122:1"]
    pub unsafe extern "C" fn k5_input_get_uint32_n(mut in_0: *mut k5input)
     -> uint32_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 4 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int as libc::c_uint
               } else { load_32_n(ptr as *const libc::c_void) };
    }
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::k5_platform_h::{load_16_be, load_16_n, load_32_be, load_32_n};
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:103"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:103"]
pub mod string_h {
    extern "C" {
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:103"]
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
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
                              C2RustUnnamed_2, store_16_be, store_32_be,
                              load_16_be, load_32_be, store_16_n, store_32_n,
                              load_16_n, load_32_n};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_addrtype,
                       krb5_enctype, krb5_authdatatype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds, _profile_t,
                       krb5_free_principal, krb5_free_cred_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, make_data, empty_data, k5calloc,
                         k5alloc, k5memdup0, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_add_len};
pub use self::k5_input_h::{k5input, k5_input_init, k5_input_set_status,
                           k5_input_get_bytes, k5_input_get_byte,
                           k5_input_get_uint16_be, k5_input_get_uint16_n,
                           k5_input_get_uint32_be, k5_input_get_uint32_n};
use self::stdlib_h::calloc;
use self::string_h::{memset, memcpy};
pub use self::byteswap_h::{__bswap_32, __bswap_16};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/ccache/ccmarshal.c - Functions for serializing creds */
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
 * This file implements marshalling and unmarshalling of krb5 credentials and
 * principals in versions 1 through 4 of the FILE ccache format.  Version 4 is
 * also used for the KEYRING ccache type.
 *
 * The FILE credential cache format uses fixed 16-bit or 32-bit representations
 * of integers.  In versions 1 and 2 these are in host byte order; in later
 * versions they are in big-endian byte order.  Variable-length fields are
 * represented with a 32-bit length followed by the field value.  There is no
 * type tagging; field representations are simply concatenated together.
 *
 * A psuedo-BNF grammar for the credential and principal formats is:
 *
 * credential ::=
 *   client (principal)
 *   server (principal)
 *   keyblock (keyblock)
 *   authtime (32 bits)
 *   starttime (32 bits)
 *   endtime (32 bits)
 *   renew_till (32 bits)
 *   is_skey (1 byte, 0 or 1)
 *   ticket_flags (32 bits)
 *   addresses (addresses)
 *   authdata (authdata)
 *   ticket (data)
 *   second_ticket (data)
 *
 * principal ::=
 *   name type (32 bits) [omitted in version 1]
 *   count of components (32 bits) [includes realm in version 1]
 *   realm (data)
 *   component1 (data)
 *   component2 (data)
 *   ...
 *
 * keyblock ::=
 *   enctype (16 bits) [repeated twice in version 3; see below]
 *   data
 *
 * addresses ::=
 *   count (32 bits)
 *   address1
 *   address2
 *   ...
 *
 * address ::=
 *   addrtype (16 bits)
 *   data
 *
 * authdata ::=
 *   count (32 bits)
 *   authdata1
 *   authdata2
 *   ...
 *
 * authdata ::=
 *   ad_type (16 bits)
 *   data
 *
 * data ::=
 *   length (32 bits)
 *   value (length bytes)
 *
 * When version 3 was current (before release 1.0), the keyblock had separate
 * key type and enctype fields, and both were recorded.  At present we record
 * the enctype field twice when writing the version 3 format and ignore the
 * second value when reading it.
 */
/* Read a 16-bit integer in host byte order for versions 1 and 2, or in
 * big-endian byte order for later versions.*/
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn get16(mut in_0: *mut k5input, mut version: libc::c_int)
 -> uint16_t {
    return if version < 3 as libc::c_int {
               k5_input_get_uint16_n(in_0) as libc::c_int
           } else { k5_input_get_uint16_be(in_0) as libc::c_int } as uint16_t;
}
/* Read a 32-bit integer in host byte order for versions 1 and 2, or in
 * big-endian byte order for later versions.*/
#[c2rust::src_loc = "117:1"]
unsafe extern "C" fn get32(mut in_0: *mut k5input, mut version: libc::c_int)
 -> uint32_t {
    return if version < 3 as libc::c_int {
               k5_input_get_uint32_n(in_0)
           } else { k5_input_get_uint32_be(in_0) };
}
/* Read a 32-bit length and make a copy of that many bytes.  Return NULL on
 * error. */
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn get_len_bytes(mut in_0: *mut k5input,
                                   mut version: libc::c_int,
                                   mut len_out: *mut libc::c_uint)
 -> *mut libc::c_void {
    let mut ret: krb5_error_code = 0;
    let mut len: libc::c_uint = get32(in_0, version);
    let mut bytes: *const libc::c_void =
        k5_input_get_bytes(in_0, len as size_t) as *const libc::c_void;
    let mut copy: *mut libc::c_void = 0 as *mut libc::c_void;
    *len_out = 0 as libc::c_int as libc::c_uint;
    if bytes.is_null() { return 0 as *mut libc::c_void }
    copy = k5memdup0(bytes, len as size_t, &mut ret);
    if copy.is_null() {
        k5_input_set_status(in_0, ret);
        return 0 as *mut libc::c_void
    }
    *len_out = len;
    return copy;
}
/* Like get_len_bytes, but put the result in data. */
#[c2rust::src_loc = "147:1"]
unsafe extern "C" fn get_data(mut in_0: *mut k5input,
                              mut version: libc::c_int,
                              mut data: *mut krb5_data) {
    let mut len: libc::c_uint = 0;
    let mut bytes: *mut libc::c_void = get_len_bytes(in_0, version, &mut len);
    *data =
        if bytes.is_null() { empty_data() } else { make_data(bytes, len) };
}
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn unmarshal_princ(mut in_0: *mut k5input,
                                     mut version: libc::c_int)
 -> krb5_principal {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut i: uint32_t = 0;
    let mut ncomps: uint32_t = 0;
    princ =
        k5alloc(::std::mem::size_of::<krb5_principal_data>() as libc::c_ulong,
                &mut ret) as krb5_principal;
    if princ.is_null() {
        k5_input_set_status(in_0, ret);
        return 0 as krb5_principal
    }
    (*princ).magic = -(1760647423 as libc::c_long) as krb5_magic;
    /* Version 1 does not store the principal name type, and counts the realm
     * in the number of components. */
    (*princ).type_0 =
        if version == 1 as libc::c_int {
            0 as libc::c_int as libc::c_uint
        } else { get32(in_0, version) } as krb5_int32;
    ncomps = get32(in_0, version);
    if version == 1 as libc::c_int { ncomps = ncomps.wrapping_sub(1) }
    if ncomps as libc::c_ulong > (*in_0).len {
        /* Sanity check to avoid large allocations */
        ret = 22 as libc::c_int
    } else {
        if ncomps != 0 as libc::c_int as libc::c_uint {
            (*princ).data =
                k5calloc(ncomps as size_t,
                         ::std::mem::size_of::<krb5_data>() as libc::c_ulong,
                         &mut ret) as *mut krb5_data;
            if (*princ).data.is_null() {
                current_block = 18266894693431972514;
            } else {
                (*princ).length = ncomps as krb5_int32;
                current_block = 5143058163439228106;
            }
        } else { current_block = 5143058163439228106; }
        match current_block {
            18266894693431972514 => { }
            _ => {
                get_data(in_0, version, &mut (*princ).realm);
                i = 0 as libc::c_int as uint32_t;
                while i < ncomps {
                    get_data(in_0, version,
                             &mut *(*princ).data.offset(i as isize));
                    i = i.wrapping_add(1)
                }
                return princ
            }
        }
    }
    k5_input_set_status(in_0, ret);
    krb5_free_principal(0 as krb5_context, princ);
    return 0 as krb5_principal;
}
#[c2rust::src_loc = "196:1"]
unsafe extern "C" fn unmarshal_keyblock(mut in_0: *mut k5input,
                                        mut version: libc::c_int,
                                        mut kb: *mut krb5_keyblock) {
    memset(kb as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    (*kb).magic = -(1760647421 as libc::c_long) as krb5_magic;
    /* enctypes can be negative, so sign-extend the 16-bit result. */
    (*kb).enctype = get16(in_0, version) as int16_t as krb5_enctype;
    /* Version 3 stores the enctype twice. */
    if version == 3 as libc::c_int { get16(in_0, version); }
    (*kb).contents =
        get_len_bytes(in_0, version, &mut (*kb).length) as *mut krb5_octet;
}
#[c2rust::src_loc = "209:1"]
unsafe extern "C" fn unmarshal_addr(mut in_0: *mut k5input,
                                    mut version: libc::c_int)
 -> *mut krb5_address {
    let mut addr: *mut krb5_address = 0 as *mut krb5_address;
    addr =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    if addr.is_null() {
        k5_input_set_status(in_0, 12 as libc::c_int);
        return 0 as *mut krb5_address
    }
    (*addr).magic = -(1760647390 as libc::c_long) as krb5_magic;
    (*addr).addrtype = get16(in_0, version) as krb5_addrtype;
    (*addr).contents =
        get_len_bytes(in_0, version, &mut (*addr).length) as *mut krb5_octet;
    return addr;
}
#[c2rust::src_loc = "225:1"]
unsafe extern "C" fn unmarshal_addrs(mut in_0: *mut k5input,
                                     mut version: libc::c_int)
 -> *mut *mut krb5_address {
    let mut addrs: *mut *mut krb5_address = 0 as *mut *mut krb5_address;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    count = get32(in_0, version) as size_t;
    if count > (*in_0).len {
        /* Sanity check to avoid large allocations */
        k5_input_set_status(in_0, 22 as libc::c_int);
        return 0 as *mut *mut krb5_address
    }
    addrs =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut krb5_address>() as libc::c_ulong) as
            *mut *mut krb5_address;
    if addrs.is_null() {
        k5_input_set_status(in_0, 12 as libc::c_int);
        return 0 as *mut *mut krb5_address
    }
    i = 0 as libc::c_int as size_t;
    while i < count {
        let ref mut fresh0 = *addrs.offset(i as isize);
        *fresh0 = unmarshal_addr(in_0, version);
        i = i.wrapping_add(1)
    }
    return addrs;
}
#[c2rust::src_loc = "246:1"]
unsafe extern "C" fn unmarshal_authdatum(mut in_0: *mut k5input,
                                         mut version: libc::c_int)
 -> *mut krb5_authdata {
    let mut ad: *mut krb5_authdata = 0 as *mut krb5_authdata;
    ad =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_authdata>() as libc::c_ulong) as
            *mut krb5_authdata;
    if ad.is_null() {
        k5_input_set_status(in_0, 12 as libc::c_int);
        return 0 as *mut krb5_authdata
    }
    (*ad).magic = -(1760647390 as libc::c_long) as krb5_magic;
    /* Authdata types can be negative, so sign-extend the get16 result. */
    (*ad).ad_type = get16(in_0, version) as int16_t as krb5_authdatatype;
    (*ad).contents =
        get_len_bytes(in_0, version, &mut (*ad).length) as *mut krb5_octet;
    return ad;
}
#[c2rust::src_loc = "263:1"]
unsafe extern "C" fn unmarshal_authdata(mut in_0: *mut k5input,
                                        mut version: libc::c_int)
 -> *mut *mut krb5_authdata {
    let mut authdata: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    count = get32(in_0, version) as size_t;
    if count > (*in_0).len {
        /* Sanity check to avoid large allocations */
        k5_input_set_status(in_0, 22 as libc::c_int);
        return 0 as *mut *mut krb5_authdata
    }
    authdata =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<*mut krb5_authdata>() as libc::c_ulong)
            as *mut *mut krb5_authdata;
    if authdata.is_null() {
        k5_input_set_status(in_0, 12 as libc::c_int);
        return 0 as *mut *mut krb5_authdata
    }
    i = 0 as libc::c_int as size_t;
    while i < count {
        let ref mut fresh1 = *authdata.offset(i as isize);
        *fresh1 = unmarshal_authdatum(in_0, version);
        i = i.wrapping_add(1)
    }
    return authdata;
}
/* Unmarshal a credential using the specified file ccache version (expressed as
 * an integer from 1 to 4).  Does not check for trailing garbage. */
#[no_mangle]
#[c2rust::src_loc = "286:1"]
pub unsafe extern "C" fn k5_unmarshal_cred(mut data: *const libc::c_uchar,
                                           mut len: size_t,
                                           mut version: libc::c_int,
                                           mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    k5_input_init(&mut in_0, data as *const libc::c_void, len);
    (*creds).client = unmarshal_princ(&mut in_0, version);
    (*creds).server = unmarshal_princ(&mut in_0, version);
    unmarshal_keyblock(&mut in_0, version, &mut (*creds).keyblock);
    (*creds).times.authtime = get32(&mut in_0, version) as krb5_timestamp;
    (*creds).times.starttime = get32(&mut in_0, version) as krb5_timestamp;
    (*creds).times.endtime = get32(&mut in_0, version) as krb5_timestamp;
    (*creds).times.renew_till = get32(&mut in_0, version) as krb5_timestamp;
    (*creds).is_skey = k5_input_get_byte(&mut in_0) as krb5_boolean;
    (*creds).ticket_flags = get32(&mut in_0, version) as krb5_flags;
    (*creds).addresses = unmarshal_addrs(&mut in_0, version);
    (*creds).authdata = unmarshal_authdata(&mut in_0, version);
    get_data(&mut in_0, version, &mut (*creds).ticket);
    get_data(&mut in_0, version, &mut (*creds).second_ticket);
    if in_0.status != 0 {
        krb5_free_cred_contents(0 as krb5_context, creds);
        memset(creds as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    }
    return if in_0.status == 22 as libc::c_int {
               -(1765328185 as libc::c_long)
           } else { in_0.status as libc::c_long } as krb5_error_code;
}
/* Unmarshal a principal using the specified file ccache version (expressed as
 * an integer from 1 to 4).  Does not check for trailing garbage. */
#[no_mangle]
#[c2rust::src_loc = "315:1"]
pub unsafe extern "C" fn k5_unmarshal_princ(mut data: *const libc::c_uchar,
                                            mut len: size_t,
                                            mut version: libc::c_int,
                                            mut princ_out:
                                                *mut krb5_principal)
 -> krb5_error_code {
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    *princ_out = 0 as krb5_principal;
    k5_input_init(&mut in_0, data as *const libc::c_void, len);
    princ = unmarshal_princ(&mut in_0, version);
    if in_0.status != 0 {
        krb5_free_principal(0 as krb5_context, princ);
    } else { *princ_out = princ }
    return if in_0.status == 22 as libc::c_int {
               -(1765328185 as libc::c_long)
           } else { in_0.status as libc::c_long } as krb5_error_code;
}
/* Store a 16-bit integer in host byte order for versions 1 and 2, or in
 * big-endian byte order for later versions.*/
#[c2rust::src_loc = "334:1"]
unsafe extern "C" fn put16(mut buf: *mut k5buf, mut version: libc::c_int,
                           mut num: uint16_t) {
    let mut n: [libc::c_char; 2] = [0; 2];
    if version < 3 as libc::c_int {
        store_16_n(num as libc::c_uint, n.as_mut_ptr() as *mut libc::c_void);
    } else {
        store_16_be(num as libc::c_uint, n.as_mut_ptr() as *mut libc::c_void);
    }
    k5_buf_add_len(buf, n.as_mut_ptr() as *const libc::c_void,
                   2 as libc::c_int as size_t);
}
/* Store a 32-bit integer in host byte order for versions 1 and 2, or in
 * big-endian byte order for later versions.*/
#[c2rust::src_loc = "348:1"]
unsafe extern "C" fn put32(mut buf: *mut k5buf, mut version: libc::c_int,
                           mut num: uint32_t) {
    let mut n: [libc::c_char; 4] = [0; 4];
    if version < 3 as libc::c_int {
        store_32_n(num, n.as_mut_ptr() as *mut libc::c_void);
    } else { store_32_be(num, n.as_mut_ptr() as *mut libc::c_void); }
    k5_buf_add_len(buf, n.as_mut_ptr() as *const libc::c_void,
                   4 as libc::c_int as size_t);
}
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn put_len_bytes(mut buf: *mut k5buf,
                                   mut version: libc::c_int,
                                   mut bytes: *const libc::c_void,
                                   mut len: libc::c_uint) {
    put32(buf, version, len);
    k5_buf_add_len(buf, bytes, len as size_t);
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn put_data(mut buf: *mut k5buf, mut version: libc::c_int,
                              mut data: *mut krb5_data) {
    put_len_bytes(buf, version, (*data).data as *const libc::c_void,
                  (*data).length);
}
#[no_mangle]
#[c2rust::src_loc = "374:1"]
pub unsafe extern "C" fn k5_marshal_princ(mut buf: *mut k5buf,
                                          mut version: libc::c_int,
                                          mut princ: krb5_principal) {
    let mut i: int32_t = 0;
    let mut ncomps: int32_t = 0;
    /* Version 1 does not store the principal name type, and counts the realm
     * in the number of components. */
    if version != 1 as libc::c_int {
        put32(buf, version, (*princ).type_0 as uint32_t);
    }
    ncomps =
        (*princ).length +
            (if version == 1 as libc::c_int {
                 1 as libc::c_int
             } else { 0 as libc::c_int });
    put32(buf, version, ncomps as uint32_t);
    put_data(buf, version, &mut (*princ).realm);
    i = 0 as libc::c_int;
    while i < (*princ).length {
        put_data(buf, version, &mut *(*princ).data.offset(i as isize));
        i += 1
    };
}
#[c2rust::src_loc = "390:1"]
unsafe extern "C" fn marshal_keyblock(mut buf: *mut k5buf,
                                      mut version: libc::c_int,
                                      mut kb: *mut krb5_keyblock) {
    put16(buf, version, (*kb).enctype as uint16_t);
    /* Version 3 stores the enctype twice. */
    if version == 3 as libc::c_int {
        put16(buf, version, (*kb).enctype as uint16_t);
    }
    put_len_bytes(buf, version, (*kb).contents as *const libc::c_void,
                  (*kb).length);
}
#[c2rust::src_loc = "400:1"]
unsafe extern "C" fn marshal_addrs(mut buf: *mut k5buf,
                                   mut version: libc::c_int,
                                   mut addrs: *mut *mut krb5_address) {
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    count = 0 as libc::c_int as size_t;
    while !addrs.is_null() && !(*addrs.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    put32(buf, version, count as uint32_t);
    i = 0 as libc::c_int as size_t;
    while i < count {
        put16(buf, version,
              (**addrs.offset(i as isize)).addrtype as uint16_t);
        put_len_bytes(buf, version,
                      (**addrs.offset(i as isize)).contents as
                          *const libc::c_void,
                      (**addrs.offset(i as isize)).length);
        i = i.wrapping_add(1)
    };
}
#[c2rust::src_loc = "413:1"]
unsafe extern "C" fn marshal_authdata(mut buf: *mut k5buf,
                                      mut version: libc::c_int,
                                      mut authdata: *mut *mut krb5_authdata) {
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    count = 0 as libc::c_int as size_t;
    while !authdata.is_null() && !(*authdata.offset(count as isize)).is_null()
          {
        count = count.wrapping_add(1)
    }
    put32(buf, version, count as uint32_t);
    i = 0 as libc::c_int as size_t;
    while i < count {
        put16(buf, version,
              (**authdata.offset(i as isize)).ad_type as uint16_t);
        put_len_bytes(buf, version,
                      (**authdata.offset(i as isize)).contents as
                          *const libc::c_void,
                      (**authdata.offset(i as isize)).length);
        i = i.wrapping_add(1)
    };
}
/* Marshal a credential using the specified file ccache version (expressed as
 * an integer from 1 to 4). */
#[no_mangle]
#[c2rust::src_loc = "429:1"]
pub unsafe extern "C" fn k5_marshal_cred(mut buf: *mut k5buf,
                                         mut version: libc::c_int,
                                         mut creds: *mut krb5_creds) {
    let mut is_skey: libc::c_char = 0;
    k5_marshal_princ(buf, version, (*creds).client);
    k5_marshal_princ(buf, version, (*creds).server);
    marshal_keyblock(buf, version, &mut (*creds).keyblock);
    put32(buf, version, (*creds).times.authtime as uint32_t);
    put32(buf, version, (*creds).times.starttime as uint32_t);
    put32(buf, version, (*creds).times.endtime as uint32_t);
    put32(buf, version, (*creds).times.renew_till as uint32_t);
    is_skey = (*creds).is_skey as libc::c_char;
    k5_buf_add_len(buf,
                   &mut is_skey as *mut libc::c_char as *const libc::c_void,
                   1 as libc::c_int as size_t);
    put32(buf, version, (*creds).ticket_flags as uint32_t);
    marshal_addrs(buf, version, (*creds).addresses);
    marshal_authdata(buf, version, (*creds).authdata);
    put_data(buf, version, &mut (*creds).ticket);
    put_data(buf, version, &mut (*creds).second_ticket);
}
/* Construct the header flags field for a matching credential for the Heimdal
 * KCM format. */
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn mcred_header(mut mcred: *mut krb5_creds) -> uint32_t {
    let mut header: uint32_t = 0 as libc::c_int as uint32_t;
    if !(*mcred).client.is_null() {
        header |= 0x1 as libc::c_int as libc::c_uint
    }
    if !(*mcred).server.is_null() {
        header |= 0x2 as libc::c_int as libc::c_uint
    }
    if (*mcred).keyblock.enctype != 0 as libc::c_int {
        header |= 0x4 as libc::c_int as libc::c_uint
    }
    if (*mcred).ticket.length > 0 as libc::c_int as libc::c_uint {
        header |= 0x8 as libc::c_int as libc::c_uint
    }
    if (*mcred).second_ticket.length > 0 as libc::c_int as libc::c_uint {
        header |= 0x10 as libc::c_int as libc::c_uint
    }
    if !(*mcred).authdata.is_null() && !(*(*mcred).authdata).is_null() {
        header |= 0x20 as libc::c_int as libc::c_uint
    }
    if !(*mcred).addresses.is_null() && !(*(*mcred).addresses).is_null() {
        header |= 0x40 as libc::c_int as libc::c_uint
    }
    return header;
}
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
/*
 * Cursor for iterating over ccache types
 */
/* reentrant mutex used by krb5_cc_* functions */
/*
 * Marshal a matching credential in the Heimdal KCM format.  Matching
 * credentials are used to identify an existing credential to retrieve or
 * remove from a cache.
 */
#[no_mangle]
#[c2rust::src_loc = "487:1"]
pub unsafe extern "C" fn k5_marshal_mcred(mut buf: *mut k5buf,
                                          mut mcred: *mut krb5_creds) {
    let version: libc::c_int =
        4 as libc::c_int; /* subfields use v4 file format */
    let mut header: uint32_t = 0;
    let mut is_skey: libc::c_char = 0;
    header = mcred_header(mcred);
    put32(buf, version, header);
    if !(*mcred).client.is_null() {
        k5_marshal_princ(buf, version, (*mcred).client);
    }
    if !(*mcred).server.is_null() {
        k5_marshal_princ(buf, version, (*mcred).server);
    }
    if (*mcred).keyblock.enctype != 0 as libc::c_int {
        marshal_keyblock(buf, version, &mut (*mcred).keyblock);
    }
    put32(buf, version, (*mcred).times.authtime as uint32_t);
    put32(buf, version, (*mcred).times.starttime as uint32_t);
    put32(buf, version, (*mcred).times.endtime as uint32_t);
    put32(buf, version, (*mcred).times.renew_till as uint32_t);
    is_skey = (*mcred).is_skey as libc::c_char;
    k5_buf_add_len(buf,
                   &mut is_skey as *mut libc::c_char as *const libc::c_void,
                   1 as libc::c_int as size_t);
    put32(buf, version, (*mcred).ticket_flags as uint32_t);
    if !(*mcred).addresses.is_null() && !(*(*mcred).addresses).is_null() {
        marshal_addrs(buf, version, (*mcred).addresses);
    }
    if !(*mcred).authdata.is_null() && !(*(*mcred).authdata).is_null() {
        marshal_authdata(buf, version, (*mcred).authdata);
    }
    if (*mcred).ticket.length > 0 as libc::c_int as libc::c_uint {
        put_data(buf, version, &mut (*mcred).ticket);
    }
    if (*mcred).second_ticket.length > 0 as libc::c_int as libc::c_uint {
        put_data(buf, version, &mut (*mcred).second_ticket);
    };
}
