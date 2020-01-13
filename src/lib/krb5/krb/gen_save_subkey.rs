use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
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
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
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
    /* to-be-encrypted portion */
    /* *< flags */
    /* *< session key: includes enctype */
    /* *< client name/realm */
    /* *< list of transited realms */
    /* *< auth, start, end, renew_till */
    /* *< array of ptrs to addresses */
    /* *< auth data */
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
    /* Flags for krb5_auth_con_genaddrs(). */
    /* * Generate the local network address. */
    /* * Generate the remote network address.  */
    /* * Generate the local network address and the local port. */
    /* * Generate the remote network address and the remote port. */
    /* * Type of function used as a callback to generate checksum data for mk_req */
    #[c2rust::src_loc = "2264:1"]
    pub type krb5_mk_req_checksum_func
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_auth_context,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "2710:1"]
    pub type krb5_rcache = *mut krb5_rc_st;
    /* the unencrypted version */
/* *
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
    #[c2rust::src_loc = "1993:1"]
    pub type krb5_authenticator = _krb5_authenticator;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1993:16"]
    pub struct _krb5_authenticator {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub checksum: *mut krb5_checksum,
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
    /* checksum type */
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
    /* per Kerberos v5 protocol spec */
    /* *< @deprecated no longer supported */
    /* *< @deprecated no longer supported */
    /* *< @deprecated no longer supported */
    /* *< @deprecated no longer supported */
    /* *< @deprecated DES-3 cbc with SHA1 */
    /* *< @deprecated DES-3 cbc mode raw */
    /* *< @deprecated no longer supported */
    /* PKINIT */
    /* *< DSA with SHA1, CMS signature */
    /* *< MD5 with RSA, CMS signature */
    /* *< SHA1 with RSA, CMS signature */
    /* *< RC2 cbc mode, CMS enveloped data */
    /* *< RSA encryption, CMS enveloped data */
    /* *< RSA w/OEAP encryption, CMS enveloped data */
    /* *< DES-3 cbc mode, CMS enveloped data */
    /* *< RFC 3962 */
    /* *< RFC 3962 */
    /* *< RFC 8009 */
    /* *< RFC 8009 */
    /* *< RFC 4757 */
    /* *< RFC 4757 */
    /* *< RFC 6803 */
    /* *< RFC 6803 */
    /* des-mac-k */
/* rsa-md4-des-k */
    /* *< RFC 3962. Used with
                                                ENCTYPE_AES128_CTS_HMAC_SHA1_96 */
    /* *< RFC 3962. Used with
                                                ENCTYPE_AES256_CTS_HMAC_SHA1_96 */
    /* *< RFC 8009 */
    /* *< RFC 8009 */
    /* *< RFC 6803 */
    /* *< RFC 6803 */
    /* Microsoft netlogon */
    /* *< RFC 4757 */
    /*
 * The following are entropy source designations. Whenever
 * krb5_C_random_add_entropy is called, one of these source ids is passed in.
 * This allows the library to better estimate bits of entropy in the sample and
 * to keep track of what sources of entropy have contributed enough entropy.
 * Sources marked internal MUST NOT be used by applications outside the
 * Kerberos library
 */
    #[c2rust::src_loc = "479:1"]
    pub type C2RustUnnamed = libc::c_uint;
    /*Do not use; maximum source ID*/
    /*Protocol data possibly from attacker*/
    #[c2rust::src_loc = "490:5"]
    pub const KRB5_C_RANDSOURCE_MAX: C2RustUnnamed = 5;
    /* Timing of operations*/
    #[c2rust::src_loc = "489:5"]
    pub const KRB5_C_RANDSOURCE_EXTERNAL_PROTOCOL: C2RustUnnamed = 4;
    /* From KDC or other trusted party*/
    /*
     * This source should be used carefully; data in this category
     * should be from a third party trusted to give random bits
     * For example keys issued by the KDC in the application server.
     */
    #[c2rust::src_loc = "488:5"]
    pub const KRB5_C_RANDSOURCE_TIMING: C2RustUnnamed = 3;
    /* /dev/random or equivalent (internal)*/
    #[c2rust::src_loc = "482:5"]
    pub const KRB5_C_RANDSOURCE_TRUSTEDPARTY: C2RustUnnamed = 2;
    /*calls to krb5_C_RANDOM_SEED (INTERNAL)*/
    #[c2rust::src_loc = "481:5"]
    pub const KRB5_C_RANDSOURCE_OSRAND: C2RustUnnamed = 1;
    #[c2rust::src_loc = "480:5"]
    pub const KRB5_C_RANDSOURCE_OLDAPI: C2RustUnnamed = 0;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, krb5_key_st};
    use super::auth_con_h::_krb5_auth_context;
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
        /*
 * end "ccache.h"
 */
        /*
 * begin "rcache.h"
 */
        #[c2rust::src_loc = "2709:8"]
        pub type krb5_rc_st;
        /* *
 * Add entropy to the pseudo-random number generator.
 *
 * @param [in] context          Library context
 * @param [in] randsource       Entropy source (see KRB5_RANDSOURCE types)
 * @param [in] data             Data
 *
 * Contribute entropy to the PRNG used by krb5 crypto operations.  This may or
 * may not affect the output of the next crypto operation requiring random
 * data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "784:1"]
        pub fn krb5_c_random_add_entropy(context: krb5_context,
                                         randsource: libc::c_uint,
                                         data: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "5876:1"]
        pub fn krb5_auth_con_setsendsubkey(ctx: krb5_context,
                                           ac: krb5_auth_context,
                                           keyblock: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5908:1"]
        pub fn krb5_auth_con_setrecvsubkey(ctx: krb5_context,
                                           ac: krb5_auth_context,
                                           keyblock: *mut krb5_keyblock)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
    #[c2rust::src_loc = "996:1"]
    pub type krb5_authdata_context = *mut _krb5_authdata_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "978:8"]
    pub struct _krb5_authdata_context {
        pub magic: krb5_magic,
        pub n_modules: libc::c_int,
        pub modules: *mut _krb5_authdata_context_module,
        pub plugins: plugin_dir_handle,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "981:12"]
    pub struct _krb5_authdata_context_module {
        pub ad_type: krb5_authdatatype,
        pub plugin_context: *mut libc::c_void,
        pub client_fini: authdata_client_plugin_fini_proc,
        pub flags: krb5_flags,
        pub ftable: *mut krb5plugin_authdata_client_ftable_v0,
        pub client_req_init: authdata_client_request_init_proc,
        pub client_req_fini: authdata_client_request_fini_proc,
        pub name: *const libc::c_char,
        pub request_context: *mut libc::c_void,
        pub request_context_pp: *mut *mut libc::c_void,
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_authdatatype, krb5_keyblock, krb5_data, krb5_key,
                        krb5_timestamp, krb5_error_code, krb5_context};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
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
        #[no_mangle]
        #[c2rust::src_loc = "2117:1"]
        pub fn krb5_generate_subkey_extended(_: krb5_context,
                                             _: *const krb5_keyblock,
                                             _: krb5_enctype,
                                             _: *mut *mut krb5_keyblock)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/auth_con.h:29"]
pub mod auth_con_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct _krb5_auth_context {
        pub magic: krb5_magic,
        pub remote_addr: *mut krb5_address,
        pub remote_port: *mut krb5_address,
        pub local_addr: *mut krb5_address,
        pub local_port: *mut krb5_address,
        pub key: krb5_key,
        pub send_subkey: krb5_key,
        pub recv_subkey: krb5_key,
        pub auth_context_flags: krb5_int32,
        pub remote_seq_number: krb5_ui_4,
        pub local_seq_number: krb5_ui_4,
        pub authentp: *mut krb5_authenticator,
        pub req_cksumtype: krb5_cksumtype,
        pub safe_cksumtype: krb5_cksumtype,
        pub cstate: krb5_data,
        pub rcache: krb5_rcache,
        pub memrcache: k5_memrcache,
        pub permitted_etypes: *mut krb5_enctype,
        pub checksum_func: krb5_mk_req_checksum_func,
        pub checksum_func_data: *mut libc::c_void,
        pub negotiated_etype: krb5_enctype,
        pub ad_context: krb5_authdata_context,
    }
    use super::krb5_h::{krb5_magic, krb5_address, krb5_key, krb5_int32,
                        krb5_ui_4, krb5_authenticator, krb5_cksumtype,
                        krb5_data, krb5_rcache, krb5_enctype,
                        krb5_mk_req_checksum_func};
    use super::memrcache_h::k5_memrcache;
    use super::k5_int_h::krb5_authdata_context;
    /* Internal auth_context_flags */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:27"]
pub mod authdata_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2007 Apple Inc.  All Rights Reserved.
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
 * Authorization data plugin definitions for Kerberos 5.
 * This is considered an INTERNAL interface at this time.
 *
 * Some work is needed before exporting it:
 *
 * + Documentation.
 * + Sample code.
 * + Test cases (preferably automated testing under "make check").
 *
 * Other changes that would be nice to have, but not necessarily
 * before making this interface public:
 *
 * + Library support for AD-IF-RELEVANT and similar wrappers.  (We can
 *   make the plugin construct them if it wants them.)
 * + KDC could combine/optimize wrapped AD elements provided by
 *   multiple plugins, e.g., two IF-RELEVANT sequences could be
 *   merged.  (The preauth plugin API also has this bug, we're going
 *   to need a general fix.)
 */
    #[c2rust::src_loc = "80:1"]
    pub type authdata_client_request_fini_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "74:1"]
    pub type authdata_client_request_init_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "193:16"]
    pub struct krb5plugin_authdata_client_ftable_v0 {
        pub name: *mut libc::c_char,
        pub ad_type_list: *mut krb5_authdatatype,
        pub init: authdata_client_plugin_init_proc,
        pub fini: authdata_client_plugin_fini_proc,
        pub flags: authdata_client_plugin_flags_proc,
        pub request_init: authdata_client_request_init_proc,
        pub request_fini: authdata_client_request_fini_proc,
        pub get_attribute_types: authdata_client_get_attribute_types_proc,
        pub get_attribute: authdata_client_get_attribute_proc,
        pub set_attribute: authdata_client_set_attribute_proc,
        pub delete_attribute: authdata_client_delete_attribute_proc,
        pub export_authdata: authdata_client_export_authdata_proc,
        pub import_authdata: authdata_client_import_authdata_proc,
        pub export_internal: authdata_client_export_internal_proc,
        pub free_internal: authdata_client_free_internal_proc,
        pub verify: authdata_client_verify_proc,
        pub size: authdata_client_size_proc,
        pub externalize: authdata_client_externalize_proc,
        pub internalize: authdata_client_internalize_proc,
        pub copy: authdata_client_copy_proc,
    }
    #[c2rust::src_loc = "185:1"]
    pub type authdata_client_copy_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void) -> krb5_error_code>;
    #[c2rust::src_loc = "177:1"]
    pub type authdata_client_internalize_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_octet, _: *mut size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "169:1"]
    pub type authdata_client_externalize_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_octet, _: *mut size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "162:1"]
    pub type authdata_client_size_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: *mut size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "153:1"]
    pub type authdata_client_verify_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *const krb5_auth_context,
                                    _: *const krb5_keyblock,
                                    _: *const krb5_ap_req)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "146:1"]
    pub type authdata_client_free_internal_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "138:1"]
    pub type authdata_client_export_internal_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: krb5_boolean,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "86:1"]
    pub type authdata_client_import_authdata_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_authdata,
                                    _: krb5_boolean, _: krb5_const_principal)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "95:1"]
    pub type authdata_client_export_authdata_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: krb5_flags,
                                    _: *mut *mut *mut krb5_authdata)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "131:1"]
    pub type authdata_client_delete_attribute_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: *const krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "122:1"]
    pub type authdata_client_set_attribute_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: krb5_boolean,
                                    _: *const krb5_data, _: *const krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "110:1"]
    pub type authdata_client_get_attribute_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: *const krb5_data,
                                    _: *mut krb5_boolean,
                                    _: *mut krb5_boolean, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut libc::c_int)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "103:1"]
    pub type authdata_client_get_attribute_types_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "64:1"]
    pub type authdata_client_plugin_flags_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_authdatatype, _: *mut krb5_flags)
                   -> ()>;
    #[c2rust::src_loc = "70:1"]
    pub type authdata_client_plugin_fini_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void)
                   -> ()>;
    #[c2rust::src_loc = "50:1"]
    pub type authdata_client_plugin_init_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_authdatatype,
                        krb5_octet, krb5_auth_context, krb5_keyblock,
                        krb5_ap_req, krb5_boolean, krb5_authdata,
                        krb5_const_principal, krb5_flags, krb5_data};
    use super::k5_int_h::_krb5_authdata_context;
    use super::stddef_h::size_t;
    /* KRB5_AUTHDATA_PLUGIN_H_INCLUDED */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/rcache/memrcache.h:29"]
pub mod memrcache_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/rcache/memrcache.h - declarations for in-memory replay cache */
/*
 * Copyright (C) 2019 by the Massachusetts Institute of Technology.
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
    #[c2rust::src_loc = "36:1"]
    pub type k5_memrcache = *mut k5_memrcache_st;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type k5_memrcache_st;
    }
    /* MEMRCACHE_H */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_ap_req, _krb5_ap_req,
                       krb5_enc_data, _krb5_enc_data, krb5_ticket,
                       _krb5_ticket, krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_authdata, _krb5_authdata, krb5_ticket_times,
                       _krb5_ticket_times, krb5_transited, _krb5_transited,
                       krb5_keyblock, _krb5_keyblock, krb5_auth_context,
                       krb5_mk_req_checksum_func, krb5_rcache,
                       krb5_authenticator, _krb5_authenticator, krb5_checksum,
                       _krb5_checksum, krb5_key, C2RustUnnamed,
                       KRB5_C_RANDSOURCE_MAX,
                       KRB5_C_RANDSOURCE_EXTERNAL_PROTOCOL,
                       KRB5_C_RANDSOURCE_TIMING,
                       KRB5_C_RANDSOURCE_TRUSTEDPARTY,
                       KRB5_C_RANDSOURCE_OSRAND, KRB5_C_RANDSOURCE_OLDAPI,
                       _profile_t, krb5_rc_st, krb5_c_random_add_entropy,
                       krb5_free_keyblock, krb5_auth_con_setsendsubkey,
                       krb5_auth_con_setrecvsubkey};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_authdata_context,
                         _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_key_st,
                         derived_key, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_crypto_us_timeofday,
                         krb5_generate_subkey_extended};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::auth_con_h::_krb5_auth_context;
pub use self::authdata_plugin_h::{authdata_client_request_fini_proc,
                                  authdata_client_request_init_proc,
                                  krb5plugin_authdata_client_ftable_v0,
                                  authdata_client_copy_proc,
                                  authdata_client_internalize_proc,
                                  authdata_client_externalize_proc,
                                  authdata_client_size_proc,
                                  authdata_client_verify_proc,
                                  authdata_client_free_internal_proc,
                                  authdata_client_export_internal_proc,
                                  authdata_client_import_authdata_proc,
                                  authdata_client_export_authdata_proc,
                                  authdata_client_delete_attribute_proc,
                                  authdata_client_set_attribute_proc,
                                  authdata_client_get_attribute_proc,
                                  authdata_client_get_attribute_types_proc,
                                  authdata_client_plugin_flags_proc,
                                  authdata_client_plugin_fini_proc,
                                  authdata_client_plugin_init_proc};
pub use self::memrcache_h::{k5_memrcache, k5_memrcache_st};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:5"]
pub struct C2RustUnnamed_0 {
    pub sec: krb5_timestamp,
    pub usec: krb5_int32,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/gen_save_subkey.c */
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
#[no_mangle]
#[c2rust::src_loc = "31:1"]
pub unsafe extern "C" fn k5_generate_and_save_subkey(mut context:
                                                         krb5_context,
                                                     mut auth_context:
                                                         krb5_auth_context,
                                                     mut keyblock:
                                                         *mut krb5_keyblock,
                                                     mut enctype:
                                                         krb5_enctype)
 -> krb5_error_code {
    /* Provide some more fodder for random number code.
       This isn't strong cryptographically; the point here is not
       to guarantee randomness, but to make it less likely that multiple
       sessions could pick the same subkey.  */
    let mut rnd_data: C2RustUnnamed_0 = C2RustUnnamed_0{sec: 0, usec: 0,};
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut retval: krb5_error_code = 0;
    let mut kb: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    if krb5_crypto_us_timeofday(&mut rnd_data.sec, &mut rnd_data.usec) ==
           0 as libc::c_int {
        d.length =
            ::std::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong as
                libc::c_uint;
        d.data = &mut rnd_data as *mut C2RustUnnamed_0 as *mut libc::c_char;
        krb5_c_random_add_entropy(context,
                                  KRB5_C_RANDSOURCE_TIMING as libc::c_int as
                                      libc::c_uint, &mut d);
    }
    retval =
        krb5_generate_subkey_extended(context, keyblock, enctype, &mut kb);
    if retval != 0 { return retval }
    retval = krb5_auth_con_setsendsubkey(context, auth_context, kb);
    if !(retval != 0) {
        retval = krb5_auth_con_setrecvsubkey(context, auth_context, kb);
        (retval) != 0;
    }
    if retval != 0 {
        krb5_auth_con_setsendsubkey(context, auth_context,
                                    0 as *mut krb5_keyblock);
        krb5_auth_con_setrecvsubkey(context, auth_context,
                                    0 as *mut krb5_keyblock);
    }
    krb5_free_keyblock(context, kb);
    return retval;
}
