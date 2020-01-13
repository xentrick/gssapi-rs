use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:27"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:29"]
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
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
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
    /* checksum type */
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
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
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
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
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
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2134:16"]
    pub struct _krb5_ap_rep {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
    /* *
 * C representaton of AP-REP message.
 *
 * The server's response to a client's request for mutual authentication.
 */
    #[c2rust::src_loc = "2134:1"]
    pub type krb5_ap_rep = _krb5_ap_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2140:16"]
    pub struct _krb5_ap_rep_enc_part {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
    }
    /* *< Ciphertext of ApRepEncPart */
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2158:16"]
    pub struct _krb5_cred_info {
        pub magic: krb5_magic,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
    }
    /* *< Client time, seconds portion */
    /* *< Client time, microseconds portion */
    /* *< Subkey (optional) */
    /* *< Sequence number */
    /* * Credentials information inserted into @c EncKrbCredPart. */
    #[c2rust::src_loc = "2158:1"]
    pub type krb5_cred_info = _krb5_cred_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2169:16"]
    pub struct _krb5_cred_enc_part {
        pub magic: krb5_magic,
        pub nonce: krb5_int32,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub ticket_info: *mut *mut krb5_cred_info,
    }
    /* *< Session key used to encrypt ticket */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Ticket flags */
    /* *< Auth, start, end, renew_till */
    /* *< Array of pointers to addrs (optional) */
    /* * Cleartext credentials information.  */
    #[c2rust::src_loc = "2169:1"]
    pub type krb5_cred_enc_part = _krb5_cred_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2180:16"]
    pub struct _krb5_cred {
        pub magic: krb5_magic,
        pub tickets: *mut *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_cred_enc_part,
    }
    /* *< Nonce (optional) */
    /* *< Generation time, seconds portion */
    /* *< Generation time, microseconds portion */
    /* *< Sender address (optional) */
    /* *< Recipient address (optional) */
    /* * Credentials data structure.*/
    #[c2rust::src_loc = "2180:1"]
    pub type krb5_cred = _krb5_cred;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    /* *< Tickets */
    /* *< Encrypted part */
    /* *< Unencrypted version, if available */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:29"]
pub mod k5_int_h {
    #[c2rust::src_loc = "1298:1"]
    pub type krb5_priv_enc_part = _krb5_priv_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1298:16"]
    pub struct _krb5_priv_enc_part {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
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
    #[c2rust::src_loc = "420:1"]
    pub type krb5_etype_info_entry = _krb5_etype_info_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "420:16"]
    pub struct _krb5_etype_info_entry {
        pub magic: krb5_magic,
        pub etype: krb5_enctype,
        pub length: libc::c_uint,
        pub salt: *mut krb5_octet,
        pub s2kparams: krb5_data,
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
    #[c2rust::src_loc = "798:1"]
    pub type krb5_fast_finished = _krb5_fast_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "798:16"]
    pub struct _krb5_fast_finished {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub client: krb5_principal,
        pub ticket_checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "843:1"]
    pub type krb5_verifier_mac = _krb5_verifier_mac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "843:16"]
    pub struct _krb5_verifier_mac {
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "1281:1"]
    pub type krb5_safe = _krb5_safe;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1281:16"]
    pub struct _krb5_safe {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub checksum: *mut krb5_checksum,
    }
    #[c2rust::src_loc = "827:1"]
    pub type krb5_ad_signedpath = _krb5_ad_signedpath;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "827:16"]
    pub struct _krb5_ad_signedpath {
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "751:1"]
    pub type krb5_pa_for_user = _krb5_pa_for_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "751:16"]
    pub struct _krb5_pa_for_user {
        pub user: krb5_principal,
        pub cksum: krb5_checksum,
        pub auth_package: krb5_data,
    }
    #[c2rust::src_loc = "813:1"]
    pub type krb5_ad_kdcissued = _krb5_ad_kdcissued;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "813:16"]
    pub struct _krb5_ad_kdcissued {
        pub ad_checksum: krb5_checksum,
        pub i_principal: krb5_principal,
        pub elements: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "552:1"]
    pub type krb5_kkdcp_message = _krb5_kkdcp_message;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "552:16"]
    pub struct _krb5_kkdcp_message {
        pub kerb_message: krb5_data,
        pub target_domain: krb5_data,
        pub dclocator_hint: krb5_int32,
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
    #[c2rust::src_loc = "839:1"]
    pub type krb5_iakerb_finished = _krb5_iakerb_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "839:16"]
    pub struct _krb5_iakerb_finished {
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "559:1"]
    pub type krb5_secure_cookie = _krb5_secure_cookie;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "559:16"]
    pub struct _krb5_secure_cookie {
        pub time: time_t,
        pub data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "460:1"]
    pub type krb5_sam_challenge_2 = _krb5_sam_challenge_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "460:16"]
    pub struct _krb5_sam_challenge_2 {
        pub sam_challenge_2_body: krb5_data,
        pub sam_cksum: *mut *mut krb5_checksum,
    }
    #[c2rust::src_loc = "465:1"]
    pub type krb5_sam_challenge_2_body = _krb5_sam_challenge_2_body;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "465:16"]
    pub struct _krb5_sam_challenge_2_body {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_type_name: krb5_data,
        pub sam_track_id: krb5_data,
        pub sam_challenge_label: krb5_data,
        pub sam_challenge: krb5_data,
        pub sam_response_prompt: krb5_data,
        pub sam_pk_for_sad: krb5_data,
        pub sam_nonce: krb5_int32,
        pub sam_etype: krb5_enctype,
    }
    #[c2rust::src_loc = "819:1"]
    pub type krb5_ad_signedpath_data = _krb5_ad_signedpath_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "819:16"]
    pub struct _krb5_ad_signedpath_data {
        pub client: krb5_principal,
        pub authtime: krb5_timestamp,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "805:1"]
    pub type krb5_fast_response = _krb5_fast_response;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "805:16"]
    pub struct _krb5_fast_response {
        pub magic: krb5_magic,
        pub padata: *mut *mut krb5_pa_data,
        pub strengthen_key: *mut krb5_keyblock,
        pub finished: *mut krb5_fast_finished,
        pub nonce: krb5_int32,
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
    #[c2rust::src_loc = "856:1"]
    pub type krb5_cammac = _krb5_cammac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "856:16"]
    pub struct _krb5_cammac {
        pub elements: *mut *mut krb5_authdata,
        pub kdc_verifier: *mut krb5_verifier_mac,
        pub svc_verifier: *mut krb5_verifier_mac,
        pub other_verifiers: *mut *mut krb5_verifier_mac,
    }
    #[c2rust::src_loc = "1293:1"]
    pub type krb5_priv = _krb5_priv;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1293:16"]
    pub struct _krb5_priv {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "834:1"]
    pub type krb5_iakerb_header = _krb5_iakerb_header;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "834:16"]
    pub struct _krb5_iakerb_header {
        pub target_realm: krb5_data,
        pub cookie: *mut krb5_data,
    }
    /* Some data comparison and conversion functions.  */
    #[inline]
    #[c2rust::src_loc = "2237:1"]
    pub unsafe extern "C" fn data_eq(mut d1: krb5_data, mut d2: krb5_data)
     -> libc::c_int {
        return (d1.length == d2.length &&
                    (d1.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d1.data as *const libc::c_void,
                                d2.data as *const libc::c_void,
                                d1.length as libc::c_ulong) == 0)) as
                   libc::c_int;
    }
    use super::krb5_h::{krb5_magic, krb5_data, krb5_timestamp, krb5_int32,
                        krb5_ui_4, krb5_address, krb5_flags, krb5_enctype,
                        krb5_octet, krb5_principal, krb5_checksum, krb5_kvno,
                        krb5_pa_data, krb5_authdata, krb5_enc_data,
                        krb5_keyblock};
    use super::k5_int_pkinit_h::krb5_algorithm_identifier;
    use super::time_t_h::time_t;
    use super::string_h::memcmp;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:29"]
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
    /* OID */
    /* Optional */
    /* SubjectPublicKeyInfo */
    /* BIT STRING */
    /* * AuthPack from RFC 4556*/
    /* Optional */
    /* Optional */
    /* Optional */
    /* OIDs of KDFs; OPTIONAL */
    /* ExternalPrincipalIdentifier */
    /* Optional */
    /* Optional */
    /* Optional */
    /* PA-PK-AS-REQ (rfc4556 -- PA TYPE 16) */
    /* Optional array */
    /* Optional */
    /* * Pkinit DHRepInfo */
    /* Optional */
    /* OID of selected KDF OPTIONAL */
    /* KDCDHKeyInfo */
    /* BIT STRING */
    /* (0..4294967295) */
    /* Optional */
    /* ReplyKeyPack */
    /* PA-PK-AS-REP (rfc4556 -- PA TYPE 17) */
    #[c2rust::src_loc = "104:1"]
    pub type krb5_pa_pk_as_rep = _krb5_pa_pk_as_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:16"]
    pub struct _krb5_pa_pk_as_rep {
        pub choice: krb5_pa_pk_as_rep_selection,
        pub u: krb5_pa_pk_as_rep_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:11"]
    pub union krb5_pa_pk_as_rep_choices {
        pub dh_Info: krb5_dh_rep_info,
        pub encKeyPack: krb5_data,
    }
    #[c2rust::src_loc = "84:1"]
    pub type krb5_dh_rep_info = _krb5_dh_rep_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:16"]
    pub struct _krb5_dh_rep_info {
        pub dhSignedData: krb5_data,
        pub serverDHNonce: krb5_data,
        pub kdfID: *mut krb5_data,
    }
    #[c2rust::src_loc = "105:5"]
    pub type krb5_pa_pk_as_rep_selection = libc::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const choice_pa_pk_as_rep_encKeyPack: krb5_pa_pk_as_rep_selection = 1;
    #[c2rust::src_loc = "107:9"]
    pub const choice_pa_pk_as_rep_dhInfo: krb5_pa_pk_as_rep_selection = 0;
    #[c2rust::src_loc = "106:9"]
    pub const choice_pa_pk_as_rep_UNKNOWN: krb5_pa_pk_as_rep_selection = -1;
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
    #[c2rust::src_loc = "61:1"]
    pub type krb5_auth_pack = _krb5_auth_pack;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct _krb5_auth_pack {
        pub pkAuthenticator: krb5_pk_authenticator,
        pub clientPublicValue: *mut krb5_subject_pk_info,
        pub supportedCMSTypes: *mut *mut krb5_algorithm_identifier,
        pub clientDHNonce: krb5_data,
        pub supportedKDFs: *mut *mut krb5_data,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_subject_pk_info = _krb5_subject_pk_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct _krb5_subject_pk_info {
        pub algorithm: krb5_algorithm_identifier,
        pub subjectPublicKey: krb5_data,
    }
    #[c2rust::src_loc = "40:1"]
    pub type krb5_pk_authenticator = _krb5_pk_authenticator;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct _krb5_pk_authenticator {
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub paChecksum: krb5_checksum,
        pub freshnessToken: *mut krb5_data,
    }
    #[c2rust::src_loc = "77:1"]
    pub type krb5_pa_pk_as_req = _krb5_pa_pk_as_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:16"]
    pub struct _krb5_pa_pk_as_req {
        pub signedAuthPack: krb5_data,
        pub trustedCertifiers: *mut *mut krb5_external_principal_identifier,
        pub kdcPkId: krb5_data,
    }
    #[c2rust::src_loc = "91:1"]
    pub type krb5_kdc_dh_key_info = _krb5_kdc_dh_key_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:16"]
    pub struct _krb5_kdc_dh_key_info {
        pub subjectPublicKey: krb5_data,
        pub nonce: krb5_int32,
        pub dhKeyExpiration: krb5_timestamp,
    }
    #[c2rust::src_loc = "98:1"]
    pub type krb5_reply_key_pack = _krb5_reply_key_pack;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct _krb5_reply_key_pack {
        pub replyKey: krb5_keyblock,
        pub asChecksum: krb5_checksum,
    }
    use super::krb5_h::{krb5_data, krb5_int32, krb5_timestamp, krb5_checksum,
                        krb5_keyblock};
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-spake.h:29"]
pub mod k5_spake_h {
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
    #[c2rust::src_loc = "48:16"]
    pub struct krb5_spake_factor_st {
        pub type_0: int32_t,
        pub data: *mut krb5_data,
    }
    /* SPAKESupport is sent from the client to the KDC to indicate which group the
 * client supports. */
    /* SPAKEChallenge is sent from the KDC to the client to communicate its group
 * selection, public value, and second-factor challenge options. */
    /* SPAKEResponse is sent from the client to the KDC to communicate its public
 * value and encrypted second-factor response. */
    /* PA-SPAKE is a choice among the message types which can appear in a PA-SPAKE
 * padata element. */
    #[c2rust::src_loc = "85:1"]
    pub type krb5_pa_spake = krb5_pa_spake_st;
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
    #[c2rust::src_loc = "70:1"]
    pub type krb5_spake_response = krb5_spake_response_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct krb5_spake_response_st {
        pub pubkey: krb5_data,
        pub factor: krb5_enc_data,
    }
    #[c2rust::src_loc = "62:1"]
    pub type krb5_spake_challenge = krb5_spake_challenge_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct krb5_spake_challenge_st {
        pub group: int32_t,
        pub pubkey: krb5_data,
        pub factors: *mut *mut krb5_spake_factor,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_spake_support = krb5_spake_support_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct krb5_spake_support_st {
        pub ngroups: int32_t,
        pub groups: *mut int32_t,
    }
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
    use super::stdint_intn_h::int32_t;
    use super::krb5_h::{krb5_data, krb5_enc_data};
    /* K5_SPAKE_H */
}
#[c2rust::header_src = "/usr/include/string.h:29"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_msgtype,
                       krb5_kvno, krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, _krb5_address, krb5_address,
                       _krb5_keyblock, krb5_keyblock, _krb5_checksum,
                       krb5_checksum, _krb5_enc_data, krb5_enc_data,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _krb5_ticket,
                       krb5_ticket, _krb5_authenticator, krb5_authenticator,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_ap_req, krb5_ap_req,
                       _krb5_ap_rep, krb5_ap_rep, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, _krb5_cred_info, krb5_cred_info,
                       _krb5_cred_enc_part, krb5_cred_enc_part, _krb5_cred,
                       krb5_cred};
pub use self::k5_int_h::{krb5_priv_enc_part, _krb5_priv_enc_part,
                         krb5_otp_tokeninfo, _krb5_otp_tokeninfo,
                         krb5_etype_info_entry, _krb5_etype_info_entry,
                         krb5_pa_otp_challenge, _krb5_pa_otp_challenge,
                         krb5_fast_finished, _krb5_fast_finished,
                         krb5_verifier_mac, _krb5_verifier_mac, krb5_safe,
                         _krb5_safe, krb5_ad_signedpath, _krb5_ad_signedpath,
                         krb5_pa_for_user, _krb5_pa_for_user,
                         krb5_ad_kdcissued, _krb5_ad_kdcissued,
                         krb5_kkdcp_message, _krb5_kkdcp_message,
                         krb5_pa_otp_req, _krb5_pa_otp_req,
                         krb5_iakerb_finished, _krb5_iakerb_finished,
                         krb5_secure_cookie, _krb5_secure_cookie,
                         krb5_sam_challenge_2, _krb5_sam_challenge_2,
                         krb5_sam_challenge_2_body,
                         _krb5_sam_challenge_2_body, krb5_ad_signedpath_data,
                         _krb5_ad_signedpath_data, krb5_fast_response,
                         _krb5_fast_response, krb5_pa_enc_ts, _krb5_pa_enc_ts,
                         krb5_cammac, _krb5_cammac, krb5_priv, _krb5_priv,
                         krb5_pa_s4u_x509_user, _krb5_pa_s4u_x509_user,
                         krb5_s4u_userid, _krb5_s4u_userid,
                         krb5_iakerb_header, _krb5_iakerb_header, data_eq};
pub use self::k5_int_pkinit_h::{krb5_pa_pk_as_rep, _krb5_pa_pk_as_rep,
                                krb5_pa_pk_as_rep_choices, krb5_dh_rep_info,
                                _krb5_dh_rep_info,
                                krb5_pa_pk_as_rep_selection,
                                choice_pa_pk_as_rep_encKeyPack,
                                choice_pa_pk_as_rep_dhInfo,
                                choice_pa_pk_as_rep_UNKNOWN,
                                krb5_algorithm_identifier,
                                _krb5_algorithm_identifier,
                                krb5_external_principal_identifier,
                                _krb5_external_principal_identifier,
                                krb5_auth_pack, _krb5_auth_pack,
                                krb5_subject_pk_info, _krb5_subject_pk_info,
                                krb5_pk_authenticator, _krb5_pk_authenticator,
                                krb5_pa_pk_as_req, _krb5_pa_pk_as_req,
                                krb5_kdc_dh_key_info, _krb5_kdc_dh_key_info,
                                krb5_reply_key_pack, _krb5_reply_key_pack};
pub use self::k5_spake_h::{krb5_spake_factor, krb5_spake_factor_st,
                           krb5_pa_spake, krb5_pa_spake_st,
                           krb5_spake_message_choices, krb5_spake_response,
                           krb5_spake_response_st, krb5_spake_challenge,
                           krb5_spake_challenge_st, krb5_spake_support,
                           krb5_spake_support_st, krb5_spake_msgtype,
                           SPAKE_MSGTYPE_ENCDATA, SPAKE_MSGTYPE_RESPONSE,
                           SPAKE_MSGTYPE_CHALLENGE, SPAKE_MSGTYPE_SUPPORT,
                           SPAKE_MSGTYPE_UNKNOWN};
use self::string_h::memcmp;
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn ktest_equal_authenticator(mut ref_0:
                                                       *mut krb5_authenticator,
                                                   mut var:
                                                       *mut krb5_authenticator)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum((*ref_0).checksum, (*var).checksum) != 0) as
            libc::c_int;
    p = (p != 0 && (*ref_0).cusec == (*var).cusec) as libc::c_int;
    p = (p != 0 && (*ref_0).ctime == (*var).ctime) as libc::c_int;
    p =
        (p != 0 && ktest_equal_keyblock((*ref_0).subkey, (*var).subkey) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).seq_number == (*var).seq_number) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_authorization_data((*ref_0).authorization_data,
                                            (*var).authorization_data) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "64:1"]
pub unsafe extern "C" fn ktest_equal_principal_data(mut ref_0:
                                                        *mut krb5_principal_data,
                                                    mut var:
                                                        *mut krb5_principal_data)
 -> libc::c_int {
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    return (ktest_equal_data(&mut (*ref_0).realm, &mut (*var).realm) != 0 &&
                ((*ref_0).length == (*var).length &&
                     ktest_equal_array_of_data((*ref_0).length, (*ref_0).data,
                                               (*var).data) != 0) &&
                (*ref_0).type_0 == (*var).type_0) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "74:1"]
pub unsafe extern "C" fn ktest_equal_authdata(mut ref_0: *mut krb5_authdata,
                                              mut var: *mut krb5_authdata)
 -> libc::c_int {
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    return ((*ref_0).ad_type == (*var).ad_type &&
                ((*ref_0).length == (*var).length &&
                     ktest_equal_array_of_octet((*ref_0).length,
                                                (*ref_0).contents,
                                                (*var).contents) != 0)) as
               libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "83:1"]
pub unsafe extern "C" fn ktest_equal_checksum(mut ref_0: *mut krb5_checksum,
                                              mut var: *mut krb5_checksum)
 -> libc::c_int {
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    return ((*ref_0).checksum_type == (*var).checksum_type &&
                ((*ref_0).length == (*var).length &&
                     ktest_equal_array_of_octet((*ref_0).length,
                                                (*ref_0).contents,
                                                (*var).contents) != 0)) as
               libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "91:1"]
pub unsafe extern "C" fn ktest_equal_keyblock(mut ref_0: *mut krb5_keyblock,
                                              mut var: *mut krb5_keyblock)
 -> libc::c_int {
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    return ((*ref_0).enctype == (*var).enctype &&
                ((*ref_0).length == (*var).length &&
                     ktest_equal_array_of_octet((*ref_0).length,
                                                (*ref_0).contents,
                                                (*var).contents) != 0)) as
               libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "99:1"]
pub unsafe extern "C" fn ktest_equal_data(mut ref_0: *mut krb5_data,
                                          mut var: *mut krb5_data)
 -> libc::c_int {
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    return ((*ref_0).length == (*var).length &&
                ktest_equal_array_of_char((*ref_0).length, (*ref_0).data,
                                          (*var).data) != 0) as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn ktest_equal_ticket(mut ref_0: *mut krb5_ticket,
                                            mut var: *mut krb5_ticket)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).server, (*var).server) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).enc_part,
                                  &mut (*var).enc_part) != 0) as libc::c_int;
    /* enc_part2 is irrelevant, as far as the ASN.1 code is concerned */
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn ktest_equal_enc_data(mut ref_0: *mut krb5_enc_data,
                                              mut var: *mut krb5_enc_data)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).enctype == (*var).enctype) as libc::c_int;
    p = (p != 0 && (*ref_0).kvno == (*var).kvno) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).ciphertext,
                              &mut (*var).ciphertext) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "131:1"]
pub unsafe extern "C" fn ktest_equal_encryption_key(mut ref_0:
                                                        *mut krb5_keyblock,
                                                    mut var:
                                                        *mut krb5_keyblock)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).enctype == (*var).enctype) as libc::c_int;
    p =
        (p != 0 &&
             ((*ref_0).length == (*var).length &&
                  ktest_equal_array_of_octet((*ref_0).length,
                                             (*ref_0).contents,
                                             (*var).contents) != 0)) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn ktest_equal_enc_tkt_part(mut ref_0:
                                                      *mut krb5_enc_tkt_part,
                                                  mut var:
                                                      *mut krb5_enc_tkt_part)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).flags == (*var).flags) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_encryption_key((*ref_0).session, (*var).session) !=
                 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_transited(&mut (*ref_0).transited,
                                   &mut (*var).transited) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_ticket_times(&mut (*ref_0).times, &mut (*var).times)
                 != 0) as libc::c_int;
    p =
        (p != 0 && ktest_equal_addresses((*ref_0).caddrs, (*var).caddrs) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_authorization_data((*ref_0).authorization_data,
                                            (*var).authorization_data) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn ktest_equal_transited(mut ref_0: *mut krb5_transited,
                                               mut var: *mut krb5_transited)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             (*ref_0).tr_type as libc::c_int == (*var).tr_type as libc::c_int)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).tr_contents,
                              &mut (*var).tr_contents) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub unsafe extern "C" fn ktest_equal_ticket_times(mut ref_0:
                                                      *mut krb5_ticket_times,
                                                  mut var:
                                                      *mut krb5_ticket_times)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).authtime == (*var).authtime) as libc::c_int;
    p = (p != 0 && (*ref_0).starttime == (*var).starttime) as libc::c_int;
    p = (p != 0 && (*ref_0).endtime == (*var).endtime) as libc::c_int;
    p = (p != 0 && (*ref_0).renew_till == (*var).renew_till) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn ktest_equal_address(mut ref_0: *mut krb5_address,
                                             mut var: *mut krb5_address)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).addrtype == (*var).addrtype) as libc::c_int;
    p =
        (p != 0 &&
             ((*ref_0).length == (*var).length &&
                  ktest_equal_array_of_octet((*ref_0).length,
                                             (*ref_0).contents,
                                             (*var).contents) != 0)) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn ktest_equal_enc_kdc_rep_part(mut ref_0:
                                                          *mut krb5_enc_kdc_rep_part,
                                                      mut var:
                                                          *mut krb5_enc_kdc_rep_part)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_keyblock((*ref_0).session, (*var).session) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_last_req((*ref_0).last_req, (*var).last_req) != 0) as
            libc::c_int;
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    p = (p != 0 && (*ref_0).key_exp == (*var).key_exp) as libc::c_int;
    p = (p != 0 && (*ref_0).flags == (*var).flags) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_ticket_times(&mut (*ref_0).times, &mut (*var).times)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).server, (*var).server) != 0)
            as libc::c_int;
    p =
        (p != 0 && ktest_equal_addresses((*ref_0).caddrs, (*var).caddrs) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "211:1"]
pub unsafe extern "C" fn ktest_equal_priv(mut ref_0: *mut krb5_priv,
                                          mut var: *mut krb5_priv)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).enc_part,
                                  &mut (*var).enc_part) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "221:1"]
pub unsafe extern "C" fn ktest_equal_cred(mut ref_0: *mut krb5_cred,
                                          mut var: *mut krb5_cred)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_sequence_of_ticket((*ref_0).tickets, (*var).tickets)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).enc_part,
                                  &mut (*var).enc_part) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "232:1"]
pub unsafe extern "C" fn ktest_equal_error(mut ref_0: *mut krb5_error,
                                           mut var: *mut krb5_error)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).ctime == (*var).ctime) as libc::c_int;
    p = (p != 0 && (*ref_0).cusec == (*var).cusec) as libc::c_int;
    p = (p != 0 && (*ref_0).susec == (*var).susec) as libc::c_int;
    p = (p != 0 && (*ref_0).stime == (*var).stime) as libc::c_int;
    p = (p != 0 && (*ref_0).error == (*var).error) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).server, (*var).server) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).text, &mut (*var).text) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).e_data, &mut (*var).e_data) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "250:1"]
pub unsafe extern "C" fn ktest_equal_ap_req(mut ref_0: *mut krb5_ap_req,
                                            mut var: *mut krb5_ap_req)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).ap_options == (*var).ap_options) as libc::c_int;
    p =
        (p != 0 && ktest_equal_ticket((*ref_0).ticket, (*var).ticket) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).authenticator,
                                  &mut (*var).authenticator) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "262:1"]
pub unsafe extern "C" fn ktest_equal_ap_rep(mut ref_0: *mut krb5_ap_rep,
                                            mut var: *mut krb5_ap_rep)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).enc_part,
                                  &mut (*var).enc_part) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "272:1"]
pub unsafe extern "C" fn ktest_equal_ap_rep_enc_part(mut ref_0:
                                                         *mut krb5_ap_rep_enc_part,
                                                     mut var:
                                                         *mut krb5_ap_rep_enc_part)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).ctime == (*var).ctime) as libc::c_int;
    p = (p != 0 && (*ref_0).cusec == (*var).cusec) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_encryption_key((*ref_0).subkey, (*var).subkey) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).seq_number == (*var).seq_number) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "286:1"]
pub unsafe extern "C" fn ktest_equal_safe(mut ref_0: *mut krb5_safe,
                                          mut var: *mut krb5_safe)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).user_data, &mut (*var).user_data)
                 != 0) as libc::c_int;
    p = (p != 0 && (*ref_0).timestamp == (*var).timestamp) as libc::c_int;
    p = (p != 0 && (*ref_0).usec == (*var).usec) as libc::c_int;
    p = (p != 0 && (*ref_0).seq_number == (*var).seq_number) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_address((*ref_0).s_address, (*var).s_address) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_address((*ref_0).r_address, (*var).r_address) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum((*ref_0).checksum, (*var).checksum) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "303:1"]
pub unsafe extern "C" fn ktest_equal_enc_cred_part(mut ref_0:
                                                       *mut krb5_cred_enc_part,
                                                   mut var:
                                                       *mut krb5_cred_enc_part)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    p = (p != 0 && (*ref_0).timestamp == (*var).timestamp) as libc::c_int;
    p = (p != 0 && (*ref_0).usec == (*var).usec) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_address((*ref_0).s_address, (*var).s_address) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_address((*ref_0).r_address, (*var).r_address) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_cred_info((*ref_0).ticket_info,
                                               (*var).ticket_info) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "318:1"]
pub unsafe extern "C" fn ktest_equal_enc_priv_part(mut ref_0:
                                                       *mut krb5_priv_enc_part,
                                                   mut var:
                                                       *mut krb5_priv_enc_part)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).user_data, &mut (*var).user_data)
                 != 0) as libc::c_int;
    p = (p != 0 && (*ref_0).timestamp == (*var).timestamp) as libc::c_int;
    p = (p != 0 && (*ref_0).usec == (*var).usec) as libc::c_int;
    p = (p != 0 && (*ref_0).seq_number == (*var).seq_number) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_address((*ref_0).s_address, (*var).s_address) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_address((*ref_0).r_address, (*var).r_address) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "333:1"]
pub unsafe extern "C" fn ktest_equal_as_rep(mut ref_0: *mut krb5_kdc_rep,
                                            mut var: *mut krb5_kdc_rep)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).msg_type == (*var).msg_type) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_pa_data((*ref_0).padata, (*var).padata)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 && ktest_equal_ticket((*ref_0).ticket, (*var).ticket) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).enc_part,
                                  &mut (*var).enc_part) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_kdc_rep_part((*ref_0).enc_part2,
                                          (*var).enc_part2) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "348:1"]
pub unsafe extern "C" fn ktest_equal_tgs_rep(mut ref_0: *mut krb5_kdc_rep,
                                             mut var: *mut krb5_kdc_rep)
 -> libc::c_int {
    return ktest_equal_as_rep(ref_0, var);
}
#[no_mangle]
#[c2rust::src_loc = "354:1"]
pub unsafe extern "C" fn ktest_equal_as_req(mut ref_0: *mut krb5_kdc_req,
                                            mut var: *mut krb5_kdc_req)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).msg_type == (*var).msg_type) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_pa_data((*ref_0).padata, (*var).padata)
                 != 0) as libc::c_int;
    p = (p != 0 && (*ref_0).kdc_options == (*var).kdc_options) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).server, (*var).server) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).from == (*var).from) as libc::c_int;
    p = (p != 0 && (*ref_0).till == (*var).till) as libc::c_int;
    p = (p != 0 && (*ref_0).rtime == (*var).rtime) as libc::c_int;
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    p =
        (p != 0 &&
             ((*ref_0).nktypes == (*var).nktypes &&
                  ktest_equal_array_of_enctype((*ref_0).nktypes,
                                               (*ref_0).ktype, (*var).ktype)
                      != 0)) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_addresses((*ref_0).addresses, (*var).addresses) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).authorization_data,
                                  &mut (*var).authorization_data) != 0) as
            libc::c_int;
    /* This field isn't actually in the ASN.1 encoding. */
/* p = p && ptr_equal(unenc_authdata,ktest_equal_authorization_data); */
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "377:1"]
pub unsafe extern "C" fn ktest_equal_tgs_req(mut ref_0: *mut krb5_kdc_req,
                                             mut var: *mut krb5_kdc_req)
 -> libc::c_int {
    return ktest_equal_as_req(ref_0, var);
}
#[no_mangle]
#[c2rust::src_loc = "383:1"]
pub unsafe extern "C" fn ktest_equal_kdc_req_body(mut ref_0:
                                                      *mut krb5_kdc_req,
                                                  mut var: *mut krb5_kdc_req)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).kdc_options == (*var).kdc_options) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).server, (*var).server) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).from == (*var).from) as libc::c_int;
    p = (p != 0 && (*ref_0).till == (*var).till) as libc::c_int;
    p = (p != 0 && (*ref_0).rtime == (*var).rtime) as libc::c_int;
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    p =
        (p != 0 &&
             ((*ref_0).nktypes == (*var).nktypes &&
                  ktest_equal_array_of_enctype((*ref_0).nktypes,
                                               (*ref_0).ktype, (*var).ktype)
                      != 0)) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_addresses((*ref_0).addresses, (*var).addresses) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).authorization_data,
                                  &mut (*var).authorization_data) != 0) as
            libc::c_int;
    /* This isn't part of the ASN.1 encoding. */
    /* p = p && ptr_equal(unenc_authdata,ktest_equal_authorization_data); */
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "404:1"]
pub unsafe extern "C" fn ktest_equal_last_req_entry(mut ref_0:
                                                        *mut krb5_last_req_entry,
                                                    mut var:
                                                        *mut krb5_last_req_entry)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).lr_type == (*var).lr_type) as libc::c_int;
    p = (p != 0 && (*ref_0).value == (*var).value) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "415:1"]
pub unsafe extern "C" fn ktest_equal_pa_data(mut ref_0: *mut krb5_pa_data,
                                             mut var: *mut krb5_pa_data)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).pa_type == (*var).pa_type) as libc::c_int;
    p =
        (p != 0 &&
             ((*ref_0).length == (*var).length &&
                  ktest_equal_array_of_octet((*ref_0).length,
                                             (*ref_0).contents,
                                             (*var).contents) != 0)) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "426:1"]
pub unsafe extern "C" fn ktest_equal_cred_info(mut ref_0: *mut krb5_cred_info,
                                               mut var: *mut krb5_cred_info)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_keyblock((*ref_0).session, (*var).session) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).server, (*var).server) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).flags == (*var).flags) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_ticket_times(&mut (*ref_0).times, &mut (*var).times)
                 != 0) as libc::c_int;
    p =
        (p != 0 && ktest_equal_addresses((*ref_0).caddrs, (*var).caddrs) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "442:1"]
pub unsafe extern "C" fn ktest_equal_krb5_etype_info_entry(mut ref_0:
                                                               *mut krb5_etype_info_entry,
                                                           mut var:
                                                               *mut krb5_etype_info_entry)
 -> libc::c_int {
    if (*ref_0).etype != (*var).etype { return 0 as libc::c_int }
    if (*ref_0).length != (*var).length { return 0 as libc::c_int }
    if (*ref_0).length > 0 as libc::c_int as libc::c_uint &&
           (*ref_0).length !=
               (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
       {
        if memcmp((*ref_0).salt as *const libc::c_void,
                  (*var).salt as *const libc::c_void,
                  (*ref_0).length as libc::c_ulong) != 0 as libc::c_int {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "456:1"]
pub unsafe extern "C" fn ktest_equal_krb5_pa_enc_ts(mut ref_0:
                                                        *mut krb5_pa_enc_ts,
                                                    mut var:
                                                        *mut krb5_pa_enc_ts)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).patimestamp == (*var).patimestamp) as libc::c_int;
    p = (p != 0 && (*ref_0).pausec == (*var).pausec) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "469:1"]
pub unsafe extern "C" fn ktest_equal_sam_challenge_2_body(mut ref_0:
                                                              *mut krb5_sam_challenge_2_body,
                                                          mut var:
                                                              *mut krb5_sam_challenge_2_body)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).sam_type == (*var).sam_type) as libc::c_int;
    p = (p != 0 && (*ref_0).sam_flags == (*var).sam_flags) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_type_name,
                              &mut (*var).sam_type_name) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_track_id,
                              &mut (*var).sam_track_id) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_challenge_label,
                              &mut (*var).sam_challenge_label) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_challenge,
                              &mut (*var).sam_challenge) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_response_prompt,
                              &mut (*var).sam_response_prompt) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_pk_for_sad,
                              &mut (*var).sam_pk_for_sad) != 0) as
            libc::c_int;
    p = (p != 0 && (*ref_0).sam_nonce == (*var).sam_nonce) as libc::c_int;
    p = (p != 0 && (*ref_0).sam_etype == (*var).sam_etype) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "489:1"]
pub unsafe extern "C" fn ktest_equal_sam_challenge_2(mut ref_0:
                                                         *mut krb5_sam_challenge_2,
                                                     mut var:
                                                         *mut krb5_sam_challenge_2)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).sam_challenge_2_body,
                              &mut (*var).sam_challenge_2_body) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_checksum((*ref_0).sam_cksum,
                                              (*var).sam_cksum) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "501:1"]
pub unsafe extern "C" fn ktest_equal_pa_for_user(mut ref_0:
                                                     *mut krb5_pa_for_user,
                                                 mut var:
                                                     *mut krb5_pa_for_user)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).user, (*var).user) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).cksum, &mut (*var).cksum) !=
                 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).auth_package,
                              &mut (*var).auth_package) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "513:1"]
pub unsafe extern "C" fn ktest_equal_pa_s4u_x509_user(mut ref_0:
                                                          *mut krb5_pa_s4u_x509_user,
                                                      mut var:
                                                          *mut krb5_pa_s4u_x509_user)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 && (*ref_0).user_id.nonce == (*var).user_id.nonce) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).user_id.user,
                                        (*var).user_id.user) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).user_id.subject_cert,
                              &mut (*var).user_id.subject_cert) != 0) as
            libc::c_int;
    p =
        (p != 0 && (*ref_0).user_id.options == (*var).user_id.options) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).cksum, &mut (*var).cksum) !=
                 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "528:1"]
pub unsafe extern "C" fn ktest_equal_ad_kdcissued(mut ref_0:
                                                      *mut krb5_ad_kdcissued,
                                                  mut var:
                                                      *mut krb5_ad_kdcissued)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).ad_checksum,
                                  &mut (*var).ad_checksum) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).i_principal,
                                        (*var).i_principal) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_authorization_data((*ref_0).elements,
                                            (*var).elements) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "540:1"]
pub unsafe extern "C" fn ktest_equal_ad_signedpath_data(mut ref_0:
                                                            *mut krb5_ad_signedpath_data,
                                                        mut var:
                                                            *mut krb5_ad_signedpath_data)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).authtime == (*var).authtime) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_principal((*ref_0).delegated,
                                               (*var).delegated) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_pa_data((*ref_0).method_data,
                                             (*var).method_data) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_authorization_data((*ref_0).authorization_data,
                                            (*var).authorization_data) != 0)
            as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "555:1"]
pub unsafe extern "C" fn ktest_equal_ad_signedpath(mut ref_0:
                                                       *mut krb5_ad_signedpath,
                                                   mut var:
                                                       *mut krb5_ad_signedpath)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).enctype == (*var).enctype) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).checksum,
                                  &mut (*var).checksum) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_principal((*ref_0).delegated,
                                               (*var).delegated) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_pa_data((*ref_0).method_data,
                                             (*var).method_data) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "568:1"]
pub unsafe extern "C" fn ktest_equal_iakerb_header(mut ref_0:
                                                       *mut krb5_iakerb_header,
                                                   mut var:
                                                       *mut krb5_iakerb_header)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).target_realm,
                              &mut (*var).target_realm) != 0) as libc::c_int;
    p =
        (p != 0 && ktest_equal_data((*ref_0).cookie, (*var).cookie) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "579:1"]
pub unsafe extern "C" fn ktest_equal_iakerb_finished(mut ref_0:
                                                         *mut krb5_iakerb_finished,
                                                     mut var:
                                                         *mut krb5_iakerb_finished)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).checksum,
                                  &mut (*var).checksum) != 0) as libc::c_int;
    return p;
}
#[c2rust::src_loc = "590:1"]
unsafe extern "C" fn ktest_equal_fast_finished(mut ref_0:
                                                   *mut krb5_fast_finished,
                                               mut var:
                                                   *mut krb5_fast_finished)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).timestamp == (*var).timestamp) as libc::c_int;
    p = (p != 0 && (*ref_0).usec == (*var).usec) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).client, (*var).client) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).ticket_checksum,
                                  &mut (*var).ticket_checksum) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "603:1"]
pub unsafe extern "C" fn ktest_equal_fast_response(mut ref_0:
                                                       *mut krb5_fast_response,
                                                   mut var:
                                                       *mut krb5_fast_response)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_sequence_of_pa_data((*ref_0).padata, (*var).padata)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_keyblock((*ref_0).strengthen_key,
                                  (*var).strengthen_key) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_fast_finished((*ref_0).finished, (*var).finished) !=
                 0) as libc::c_int;
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    return p;
}
#[c2rust::src_loc = "616:1"]
unsafe extern "C" fn ktest_equal_algorithm_identifier(mut ref_0:
                                                          *mut krb5_algorithm_identifier,
                                                      mut var:
                                                          *mut krb5_algorithm_identifier)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).algorithm, &mut (*var).algorithm)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).parameters,
                              &mut (*var).parameters) != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "628:1"]
pub unsafe extern "C" fn ktest_equal_otp_tokeninfo(mut ref_0:
                                                       *mut krb5_otp_tokeninfo,
                                                   mut var:
                                                       *mut krb5_otp_tokeninfo)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).flags == (*var).flags) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).vendor, &mut (*var).vendor) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).challenge, &mut (*var).challenge)
                 != 0) as libc::c_int;
    p = (p != 0 && (*ref_0).length == (*var).length) as libc::c_int;
    p = (p != 0 && (*ref_0).format == (*var).format) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).token_id, &mut (*var).token_id) !=
                 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).alg_id, &mut (*var).alg_id) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_algorithm_identifier((*ref_0).supported_hash_alg,
                                                          (*var).supported_hash_alg)
                 != 0) as libc::c_int;
    p =
        (p != 0 && (*ref_0).iteration_count == (*var).iteration_count) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "647:1"]
pub unsafe extern "C" fn ktest_equal_pa_otp_challenge(mut ref_0:
                                                          *mut krb5_pa_otp_challenge,
                                                      mut var:
                                                          *mut krb5_pa_otp_challenge)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).nonce, &mut (*var).nonce) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).service, &mut (*var).service) !=
                 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_otp_tokeninfo((*ref_0).tokeninfo,
                                                   (*var).tokeninfo) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).salt, &mut (*var).salt) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).s2kparams, &mut (*var).s2kparams)
                 != 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "662:1"]
pub unsafe extern "C" fn ktest_equal_pa_otp_req(mut ref_0:
                                                    *mut krb5_pa_otp_req,
                                                mut var: *mut krb5_pa_otp_req)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).flags == (*var).flags) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).nonce, &mut (*var).nonce) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_enc_data(&mut (*ref_0).enc_data,
                                  &mut (*var).enc_data) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_algorithm_identifier((*ref_0).hash_alg,
                                              (*var).hash_alg) != 0) as
            libc::c_int;
    p =
        (p != 0 && (*ref_0).iteration_count == (*var).iteration_count) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).otp_value, &mut (*var).otp_value)
                 != 0) as libc::c_int;
    p =
        (p != 0 && ktest_equal_data(&mut (*ref_0).pin, &mut (*var).pin) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).challenge, &mut (*var).challenge)
                 != 0) as libc::c_int;
    p = (p != 0 && (*ref_0).time == (*var).time) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).counter, &mut (*var).counter) !=
                 0) as libc::c_int;
    p = (p != 0 && (*ref_0).format == (*var).format) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).token_id, &mut (*var).token_id) !=
                 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).alg_id, &mut (*var).alg_id) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).vendor, &mut (*var).vendor) != 0)
            as libc::c_int;
    return p;
}
/* *** arrays ****************************************************************/
#[no_mangle]
#[c2rust::src_loc = "727:1"]
pub unsafe extern "C" fn ktest_equal_array_of_data(mut length: libc::c_int,
                                                   mut ref_0: *mut krb5_data,
                                                   mut var: *mut krb5_data)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if length == 0 as libc::c_int || ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    i = 0 as libc::c_int;
    while i < length {
        p =
            (p != 0 &&
                 ktest_equal_data(&mut *ref_0.offset(i as isize),
                                  &mut *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "740:1"]
pub unsafe extern "C" fn ktest_equal_array_of_octet(mut length: libc::c_uint,
                                                    mut ref_0:
                                                        *mut krb5_octet,
                                                    mut var: *mut krb5_octet)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut p: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    if length == 0 as libc::c_int as libc::c_uint || ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        p =
            (p != 0 &&
                 *ref_0.offset(i as isize) as libc::c_int ==
                     *var.offset(i as isize) as libc::c_int) as libc::c_int as
                libc::c_uint;
        i = i.wrapping_add(1)
    }
    return p as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "753:1"]
pub unsafe extern "C" fn ktest_equal_array_of_char(mut length: libc::c_uint,
                                                   mut ref_0:
                                                       *mut libc::c_char,
                                                   mut var: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut p: libc::c_uint = 1 as libc::c_int as libc::c_uint;
    if length == 0 as libc::c_int as libc::c_uint || ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    i = 0 as libc::c_int as libc::c_uint;
    while i < length {
        p =
            (p != 0 &&
                 *ref_0.offset(i as isize) as libc::c_int ==
                     *var.offset(i as isize) as libc::c_int) as libc::c_int as
                libc::c_uint;
        i = i.wrapping_add(1)
    }
    return p as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "765:1"]
pub unsafe extern "C" fn ktest_equal_array_of_enctype(mut length: libc::c_int,
                                                      mut ref_0:
                                                          *mut krb5_enctype,
                                                      mut var:
                                                          *mut krb5_enctype)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if length == 0 as libc::c_int || ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    i = 0 as libc::c_int;
    while i < length {
        p =
            (p != 0 && *ref_0.offset(i as isize) == *var.offset(i as isize))
                as libc::c_int;
        i += 1
    }
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "788:1"]
pub unsafe extern "C" fn ktest_equal_authorization_data(mut ref_0:
                                                            *mut *mut krb5_authdata,
                                                        mut var:
                                                            *mut *mut krb5_authdata)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_authdata(*ref_0.offset(i as isize),
                                      *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "794:1"]
pub unsafe extern "C" fn ktest_equal_addresses(mut ref_0:
                                                   *mut *mut krb5_address,
                                               mut var:
                                                   *mut *mut krb5_address)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_address(*ref_0.offset(i as isize),
                                     *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "800:1"]
pub unsafe extern "C" fn ktest_equal_last_req(mut ref_0:
                                                  *mut *mut krb5_last_req_entry,
                                              mut var:
                                                  *mut *mut krb5_last_req_entry)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_last_req_entry(*ref_0.offset(i as isize),
                                            *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "806:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_ticket(mut ref_0:
                                                            *mut *mut krb5_ticket,
                                                        mut var:
                                                            *mut *mut krb5_ticket)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_ticket(*ref_0.offset(i as isize),
                                    *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "812:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_pa_data(mut ref_0:
                                                             *mut *mut krb5_pa_data,
                                                         mut var:
                                                             *mut *mut krb5_pa_data)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_pa_data(*ref_0.offset(i as isize),
                                     *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "818:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_cred_info(mut ref_0:
                                                               *mut *mut krb5_cred_info,
                                                           mut var:
                                                               *mut *mut krb5_cred_info)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_cred_info(*ref_0.offset(i as isize),
                                       *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "824:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_principal(mut ref_0:
                                                               *mut krb5_principal,
                                                           mut var:
                                                               *mut krb5_principal)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_principal_data(*ref_0.offset(i as isize),
                                            *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "830:1"]
pub unsafe extern "C" fn ktest_equal_etype_info(mut ref_0:
                                                    *mut *mut krb5_etype_info_entry,
                                                mut var:
                                                    *mut *mut krb5_etype_info_entry)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_krb5_etype_info_entry(*ref_0.offset(i as isize),
                                                   *var.offset(i as isize)) !=
                     0) as libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "836:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_checksum(mut ref_0:
                                                              *mut *mut krb5_checksum,
                                                          mut var:
                                                              *mut *mut krb5_checksum)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_checksum(*ref_0.offset(i as isize),
                                      *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "842:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_algorithm_identifier(mut ref_0:
                                                                          *mut *mut krb5_algorithm_identifier,
                                                                      mut var:
                                                                          *mut *mut krb5_algorithm_identifier)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_algorithm_identifier(*ref_0.offset(i as isize),
                                                  *var.offset(i as isize)) !=
                     0) as libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "849:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_otp_tokeninfo(mut ref_0:
                                                                   *mut *mut krb5_otp_tokeninfo,
                                                               mut var:
                                                                   *mut *mut krb5_otp_tokeninfo)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_otp_tokeninfo(*ref_0.offset(i as isize),
                                           *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "856:1"]
pub unsafe extern "C" fn ktest_equal_sequence_of_spake_factor(mut ref_0:
                                                                  *mut *mut krb5_spake_factor,
                                                              mut var:
                                                                  *mut *mut krb5_spake_factor)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_spake_factor(*ref_0.offset(i as isize),
                                          *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[c2rust::src_loc = "865:1"]
unsafe extern "C" fn ktest_equal_pk_authenticator(mut ref_0:
                                                      *mut krb5_pk_authenticator,
                                                  mut var:
                                                      *mut krb5_pk_authenticator)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).cusec == (*var).cusec) as libc::c_int;
    p = (p != 0 && (*ref_0).ctime == (*var).ctime) as libc::c_int;
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).paChecksum,
                                  &mut (*var).paChecksum) != 0) as
            libc::c_int;
    return p;
}
#[c2rust::src_loc = "879:1"]
unsafe extern "C" fn ktest_equal_subject_pk_info(mut ref_0:
                                                     *mut krb5_subject_pk_info,
                                                 mut var:
                                                     *mut krb5_subject_pk_info)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_algorithm_identifier(&mut (*ref_0).algorithm,
                                              &mut (*var).algorithm) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).subjectPublicKey,
                              &mut (*var).subjectPublicKey) != 0) as
            libc::c_int;
    return p;
}
#[c2rust::src_loc = "891:1"]
unsafe extern "C" fn ktest_equal_external_principal_identifier(mut ref_0:
                                                                   *mut krb5_external_principal_identifier,
                                                               mut var:
                                                                   *mut krb5_external_principal_identifier)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).subjectName,
                              &mut (*var).subjectName) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).issuerAndSerialNumber,
                              &mut (*var).issuerAndSerialNumber) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).subjectKeyIdentifier,
                              &mut (*var).subjectKeyIdentifier) != 0) as
            libc::c_int;
    return p;
}
#[c2rust::src_loc = "905:1"]
unsafe extern "C" fn ktest_equal_sequence_of_external_principal_identifier(mut ref_0:
                                                                               *mut *mut krb5_external_principal_identifier,
                                                                           mut var:
                                                                               *mut *mut krb5_external_principal_identifier)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_external_principal_identifier(*ref_0.offset(i as
                                                                             isize),
                                                           *var.offset(i as
                                                                           isize))
                     != 0) as libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "913:1"]
pub unsafe extern "C" fn ktest_equal_pa_pk_as_req(mut ref_0:
                                                      *mut krb5_pa_pk_as_req,
                                                  mut var:
                                                      *mut krb5_pa_pk_as_req)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).signedAuthPack,
                              &mut (*var).signedAuthPack) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_external_principal_identifier((*ref_0).trustedCertifiers,
                                                                   (*var).trustedCertifiers)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).kdcPkId, &mut (*var).kdcPkId) !=
                 0) as libc::c_int;
    return p;
}
#[c2rust::src_loc = "926:1"]
unsafe extern "C" fn ktest_equal_dh_rep_info(mut ref_0: *mut krb5_dh_rep_info,
                                             mut var: *mut krb5_dh_rep_info)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).dhSignedData,
                              &mut (*var).dhSignedData) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).serverDHNonce,
                              &mut (*var).serverDHNonce) != 0) as libc::c_int;
    p =
        (p != 0 && ktest_equal_data((*ref_0).kdfID, (*var).kdfID) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "938:1"]
pub unsafe extern "C" fn ktest_equal_pa_pk_as_rep(mut ref_0:
                                                      *mut krb5_pa_pk_as_rep,
                                                  mut var:
                                                      *mut krb5_pa_pk_as_rep)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    if (*ref_0).choice as libc::c_int != (*var).choice as libc::c_int {
        return 0 as libc::c_int
    }
    if (*ref_0).choice as libc::c_int ==
           choice_pa_pk_as_rep_dhInfo as libc::c_int {
        p =
            (p != 0 &&
                 ktest_equal_dh_rep_info(&mut (*ref_0).u.dh_Info,
                                         &mut (*var).u.dh_Info) != 0) as
                libc::c_int
    } else if (*ref_0).choice as libc::c_int ==
                  choice_pa_pk_as_rep_encKeyPack as libc::c_int {
        p =
            (p != 0 &&
                 ktest_equal_data(&mut (*ref_0).u.encKeyPack,
                                  &mut (*var).u.encKeyPack) != 0) as
                libc::c_int
    }
    return p;
}
#[c2rust::src_loc = "952:1"]
unsafe extern "C" fn ktest_equal_sequence_of_data(mut ref_0:
                                                      *mut *mut krb5_data,
                                                  mut var:
                                                      *mut *mut krb5_data)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 ktest_equal_data(*ref_0.offset(i as isize),
                                  *var.offset(i as isize)) != 0) as
                libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "958:1"]
pub unsafe extern "C" fn ktest_equal_auth_pack(mut ref_0: *mut krb5_auth_pack,
                                               mut var: *mut krb5_auth_pack)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_pk_authenticator(&mut (*ref_0).pkAuthenticator,
                                          &mut (*var).pkAuthenticator) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_subject_pk_info((*ref_0).clientPublicValue,
                                         (*var).clientPublicValue) != 0) as
            libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_algorithm_identifier((*ref_0).supportedCMSTypes,
                                                          (*var).supportedCMSTypes)
                 != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).clientDHNonce,
                              &mut (*var).clientDHNonce) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_sequence_of_data((*ref_0).supportedKDFs,
                                          (*var).supportedKDFs) != 0) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "973:1"]
pub unsafe extern "C" fn ktest_equal_kdc_dh_key_info(mut ref_0:
                                                         *mut krb5_kdc_dh_key_info,
                                                     mut var:
                                                         *mut krb5_kdc_dh_key_info)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_data(&mut (*ref_0).subjectPublicKey,
                              &mut (*var).subjectPublicKey) != 0) as
            libc::c_int;
    p = (p != 0 && (*ref_0).nonce == (*var).nonce) as libc::c_int;
    p =
        (p != 0 && (*ref_0).dhKeyExpiration == (*var).dhKeyExpiration) as
            libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "986:1"]
pub unsafe extern "C" fn ktest_equal_reply_key_pack(mut ref_0:
                                                        *mut krb5_reply_key_pack,
                                                    mut var:
                                                        *mut krb5_reply_key_pack)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_keyblock(&mut (*ref_0).replyKey,
                                  &mut (*var).replyKey) != 0) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).asChecksum,
                                  &mut (*var).asChecksum) != 0) as
            libc::c_int;
    return p;
}
/* not DISABLE_PKINIT */
#[no_mangle]
#[c2rust::src_loc = "999:1"]
pub unsafe extern "C" fn ktest_equal_kkdcp_message(mut ref_0:
                                                       *mut krb5_kkdcp_message,
                                                   mut var:
                                                       *mut krb5_kkdcp_message)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 && data_eq((*ref_0).kerb_message, (*var).kerb_message) != 0)
            as libc::c_int;
    p =
        (p != 0 && data_eq((*ref_0).target_domain, (*var).target_domain) != 0)
            as libc::c_int;
    p =
        (p != 0 && (*ref_0).dclocator_hint == (*var).dclocator_hint) as
            libc::c_int;
    return p;
}
#[c2rust::src_loc = "1011:1"]
unsafe extern "C" fn vmac_eq(mut ref_0: *mut krb5_verifier_mac,
                             mut var: *mut krb5_verifier_mac) -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_principal_data((*ref_0).princ, (*var).princ) != 0) as
            libc::c_int;
    p = (p != 0 && (*ref_0).kvno == (*var).kvno) as libc::c_int;
    p = (p != 0 && (*ref_0).enctype == (*var).enctype) as libc::c_int;
    p =
        (p != 0 &&
             ktest_equal_checksum(&mut (*ref_0).checksum,
                                  &mut (*var).checksum) != 0) as libc::c_int;
    return p;
}
#[c2rust::src_loc = "1024:1"]
unsafe extern "C" fn vmac_list_eq(mut ref_0: *mut *mut krb5_verifier_mac,
                                  mut var: *mut *mut krb5_verifier_mac)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var { return 1 as libc::c_int }
    if ref_0.is_null() || (*ref_0.offset(0 as libc::c_int as isize)).is_null()
       {
        return (var.is_null() ||
                    (*var.offset(0 as libc::c_int as isize)).is_null()) as
                   libc::c_int
    }
    if var.is_null() || (*var.offset(0 as libc::c_int as isize)).is_null() {
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while !(*ref_0.offset(i as isize)).is_null() &&
              !(*var.offset(i as isize)).is_null() {
        p =
            (p != 0 &&
                 vmac_eq(*ref_0.offset(i as isize), *var.offset(i as isize))
                     != 0) as libc::c_int;
        i += 1
    }
    if (*ref_0.offset(i as isize)).is_null() &&
           (*var.offset(i as isize)).is_null() {
        return p
    } else { return 0 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "1030:1"]
pub unsafe extern "C" fn ktest_equal_cammac(mut ref_0: *mut krb5_cammac,
                                            mut var: *mut krb5_cammac)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_authorization_data((*ref_0).elements,
                                            (*var).elements) != 0) as
            libc::c_int;
    p =
        (p != 0 && vmac_eq((*ref_0).kdc_verifier, (*var).kdc_verifier) != 0)
            as libc::c_int;
    p =
        (p != 0 && vmac_eq((*ref_0).svc_verifier, (*var).svc_verifier) != 0)
            as libc::c_int;
    p =
        (p != 0 &&
             vmac_list_eq((*ref_0).other_verifiers, (*var).other_verifiers) !=
                 0) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "1043:1"]
pub unsafe extern "C" fn ktest_equal_secure_cookie(mut ref_0:
                                                       *mut krb5_secure_cookie,
                                                   mut var:
                                                       *mut krb5_secure_cookie)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p =
        (p != 0 &&
             ktest_equal_sequence_of_pa_data((*ref_0).data, (*var).data) != 0)
            as libc::c_int;
    p = (p != 0 && (*ref_0).time == (*var).time) as libc::c_int;
    return p;
}
#[no_mangle]
#[c2rust::src_loc = "1054:1"]
pub unsafe extern "C" fn ktest_equal_spake_factor(mut ref_0:
                                                      *mut krb5_spake_factor,
                                                  mut var:
                                                      *mut krb5_spake_factor)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else { if ref_0.is_null() || var.is_null() { return 0 as libc::c_int } }
    p = (p != 0 && (*ref_0).type_0 == (*var).type_0) as libc::c_int;
    p =
        (p != 0 && ktest_equal_data((*ref_0).data, (*var).data) != 0) as
            libc::c_int;
    return p;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/ktest_equal.h */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
/* int ktest_equal_structure(krb5_structure *ref, *var) */
/* effects  Returns true (non-zero) if ref and var are
             semantically equivalent (i.e. have the same values,
             but aren't necessarily the same object).
            Returns false (zero) if ref and var differ. */
/* not DISABLE_PKINIT */
#[no_mangle]
#[c2rust::src_loc = "1065:1"]
pub unsafe extern "C" fn ktest_equal_pa_spake(mut ref_0: *mut krb5_pa_spake,
                                              mut var: *mut krb5_pa_spake)
 -> libc::c_int {
    let mut p: libc::c_int = 1 as libc::c_int;
    if ref_0 == var {
        return 1 as libc::c_int
    } else {
        if ref_0.is_null() || var.is_null() {
            return 0 as libc::c_int
        } else {
            if (*ref_0).choice as libc::c_int != (*var).choice as libc::c_int
               {
                return 0 as libc::c_int
            }
        }
    }
    match (*ref_0).choice as libc::c_int {
        0 => {
            p =
                (p != 0 &&
                     (*ref_0).u.support.ngroups == (*var).u.support.ngroups)
                    as libc::c_int;
            p =
                (p != 0 &&
                     memcmp((*ref_0).u.support.groups as *const libc::c_void,
                            (*var).u.support.groups as *const libc::c_void,
                            ((*ref_0).u.support.ngroups as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>()
                                                                 as
                                                                 libc::c_ulong))
                         == 0 as libc::c_int) as libc::c_int
        }
        1 => {
            p =
                (p != 0 &&
                     ktest_equal_data(&mut (*ref_0).u.challenge.pubkey,
                                      &mut (*var).u.challenge.pubkey) != 0) as
                    libc::c_int;
            p =
                (p != 0 &&
                     ktest_equal_sequence_of_spake_factor((*ref_0).u.challenge.factors,
                                                          (*var).u.challenge.factors)
                         != 0) as libc::c_int
        }
        2 => {
            p =
                (p != 0 &&
                     ktest_equal_data(&mut (*ref_0).u.response.pubkey,
                                      &mut (*var).u.response.pubkey) != 0) as
                    libc::c_int;
            p =
                (p != 0 &&
                     ktest_equal_enc_data(&mut (*ref_0).u.response.factor,
                                          &mut (*var).u.response.factor) != 0)
                    as libc::c_int
        }
        3 => {
            p =
                (p != 0 &&
                     ktest_equal_enc_data(&mut (*ref_0).u.encdata,
                                          &mut (*var).u.encdata) != 0) as
                    libc::c_int
        }
        _ => { }
    }
    return p;
}
