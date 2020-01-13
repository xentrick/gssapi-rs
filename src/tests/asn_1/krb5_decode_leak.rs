use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:38"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:38"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:38"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:38"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:38"]
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
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Tickets */
        /* *< Encrypted part */
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
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
         -> krb5_error_code;
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        /* *
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4607:1"]
        pub fn krb5_free_authenticator(context: krb5_context,
                                       val: *mut krb5_authenticator);
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
        #[no_mangle]
        #[c2rust::src_loc = "4633:1"]
        pub fn krb5_free_authdata(context: krb5_context,
                                  val: *mut *mut krb5_authdata);
        /* *
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context: krb5_context, val: *mut krb5_ticket);
        /* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
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
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4732:1"]
        pub fn krb5_free_ap_rep_enc_part(context: krb5_context,
                                         val: *mut krb5_ap_rep_enc_part);
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:38"]
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
    #[c2rust::src_loc = "420:1"]
    pub type krb5_etype_info_entry = _krb5_etype_info_entry;
    #[c2rust::src_loc = "436:1"]
    pub type krb5_etype_info = *mut *mut krb5_etype_info_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "460:16"]
    pub struct _krb5_sam_challenge_2 {
        pub sam_challenge_2_body: krb5_data,
        pub sam_cksum: *mut *mut krb5_checksum,
    }
    #[c2rust::src_loc = "460:1"]
    pub type krb5_sam_challenge_2 = _krb5_sam_challenge_2;
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
    #[c2rust::src_loc = "465:1"]
    pub type krb5_sam_challenge_2_body = _krb5_sam_challenge_2_body;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "479:16"]
    pub struct _krb5_sam_response_2 {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_track_id: krb5_data,
        pub sam_enc_nonce_or_sad: krb5_enc_data,
        pub sam_nonce: krb5_int32,
    }
    #[c2rust::src_loc = "479:1"]
    pub type krb5_sam_response_2 = _krb5_sam_response_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "488:16"]
    pub struct _krb5_enc_sam_response_enc_2 {
        pub magic: krb5_magic,
        pub sam_nonce: krb5_int32,
        pub sam_sad: krb5_data,
    }
    #[c2rust::src_loc = "488:1"]
    pub type krb5_enc_sam_response_enc_2 = _krb5_enc_sam_response_enc_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "746:16"]
    pub struct _krb5_pa_enc_ts {
        pub patimestamp: krb5_timestamp,
        pub pausec: krb5_int32,
    }
    #[c2rust::src_loc = "746:1"]
    pub type krb5_pa_enc_ts = _krb5_pa_enc_ts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "751:16"]
    pub struct _krb5_pa_for_user {
        pub user: krb5_principal,
        pub cksum: krb5_checksum,
        pub auth_package: krb5_data,
    }
    #[c2rust::src_loc = "751:1"]
    pub type krb5_pa_for_user = _krb5_pa_for_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "798:16"]
    pub struct _krb5_fast_finished {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub client: krb5_principal,
        pub ticket_checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "798:1"]
    pub type krb5_fast_finished = _krb5_fast_finished;
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
    #[c2rust::src_loc = "805:1"]
    pub type krb5_fast_response = _krb5_fast_response;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "813:16"]
    pub struct _krb5_ad_kdcissued {
        pub ad_checksum: krb5_checksum,
        pub i_principal: krb5_principal,
        pub elements: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "813:1"]
    pub type krb5_ad_kdcissued = _krb5_ad_kdcissued;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "827:16"]
    pub struct _krb5_ad_signedpath {
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "827:1"]
    pub type krb5_ad_signedpath = _krb5_ad_signedpath;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "834:16"]
    pub struct _krb5_iakerb_header {
        pub target_realm: krb5_data,
        pub cookie: *mut krb5_data,
    }
    #[c2rust::src_loc = "834:1"]
    pub type krb5_iakerb_header = _krb5_iakerb_header;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "839:16"]
    pub struct _krb5_iakerb_finished {
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "839:1"]
    pub type krb5_iakerb_finished = _krb5_iakerb_finished;
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
    #[c2rust::src_loc = "1281:1"]
    pub type krb5_safe = _krb5_safe;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1293:16"]
    pub struct _krb5_priv {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    #[c2rust::src_loc = "1293:1"]
    pub type krb5_priv = _krb5_priv;
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
    #[c2rust::src_loc = "1298:1"]
    pub type krb5_priv_enc_part = _krb5_priv_enc_part;
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_octet, krb5_data, krb5_checksum, krb5_enc_data,
                        krb5_timestamp, krb5_principal, krb5_pa_data,
                        krb5_keyblock, krb5_authdata, krb5_ui_4, krb5_address,
                        krb5_context, krb5_authenticator, krb5_error_code,
                        krb5_ticket, krb5_enc_tkt_part, krb5_enc_kdc_rep_part,
                        krb5_kdc_rep, krb5_ap_req, krb5_ap_rep,
                        krb5_ap_rep_enc_part, krb5_kdc_req, krb5_cred,
                        krb5_cred_enc_part, krb5_error};
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
        #[c2rust::src_loc = "863:1"]
        pub fn krb5_free_etype_info(_: krb5_context, _: krb5_etype_info);
        #[no_mangle]
        #[c2rust::src_loc = "904:1"]
        pub fn krb5_free_sam_challenge_2(_: krb5_context,
                                         _: *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "907:1"]
        pub fn krb5_free_sam_challenge_2_body(_: krb5_context,
                                              _:
                                                  *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "910:1"]
        pub fn krb5_free_sam_response_2(_: krb5_context,
                                        _: *mut krb5_sam_response_2);
        #[no_mangle]
        #[c2rust::src_loc = "913:1"]
        pub fn krb5_free_enc_sam_response_enc_2(_: krb5_context,
                                                _:
                                                    *mut krb5_enc_sam_response_enc_2);
        #[no_mangle]
        #[c2rust::src_loc = "930:1"]
        pub fn krb5_free_pa_enc_ts(_: krb5_context, _: *mut krb5_pa_enc_ts);
        #[no_mangle]
        #[c2rust::src_loc = "933:1"]
        pub fn krb5_free_pa_for_user(_: krb5_context,
                                     _: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_free_pa_s4u_x509_user(_: krb5_context,
                                          _: *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "950:1"]
        pub fn krb5_free_fast_response(_: krb5_context,
                                       _: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "951:1"]
        pub fn krb5_free_ad_kdcissued(_: krb5_context,
                                      _: *mut krb5_ad_kdcissued);
        #[no_mangle]
        #[c2rust::src_loc = "952:1"]
        pub fn krb5_free_ad_signedpath(_: krb5_context,
                                       _: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "953:1"]
        pub fn krb5_free_iakerb_header(_: krb5_context,
                                       _: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "954:1"]
        pub fn krb5_free_iakerb_finished(_: krb5_context,
                                         _: *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "1308:1"]
        pub fn krb5_free_safe(_: krb5_context, _: *mut krb5_safe);
        #[no_mangle]
        #[c2rust::src_loc = "1309:1"]
        pub fn krb5_free_priv(_: krb5_context, _: *mut krb5_priv);
        #[no_mangle]
        #[c2rust::src_loc = "1310:1"]
        pub fn krb5_free_priv_enc_part(_: krb5_context,
                                       _: *mut krb5_priv_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "1375:1"]
        pub fn encode_krb5_authenticator(rep: *const krb5_authenticator,
                                         code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1378:1"]
        pub fn encode_krb5_ticket(rep: *const krb5_ticket,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1381:1"]
        pub fn encode_krb5_enc_tkt_part(rep: *const krb5_enc_tkt_part,
                                        code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1384:1"]
        pub fn encode_krb5_enc_kdc_rep_part(rep: *const krb5_enc_kdc_rep_part,
                                            code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1389:1"]
        pub fn encode_krb5_as_rep(rep: *const krb5_kdc_rep,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1393:1"]
        pub fn encode_krb5_tgs_rep(rep: *const krb5_kdc_rep,
                                   code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1396:1"]
        pub fn encode_krb5_ap_req(rep: *const krb5_ap_req,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1399:1"]
        pub fn encode_krb5_ap_rep(rep: *const krb5_ap_rep,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1402:1"]
        pub fn encode_krb5_ap_rep_enc_part(rep: *const krb5_ap_rep_enc_part,
                                           code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1405:1"]
        pub fn encode_krb5_as_req(rep: *const krb5_kdc_req,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1408:1"]
        pub fn encode_krb5_tgs_req(rep: *const krb5_kdc_req,
                                   code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1411:1"]
        pub fn encode_krb5_kdc_req_body(rep: *const krb5_kdc_req,
                                        code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1414:1"]
        pub fn encode_krb5_safe(rep: *const krb5_safe,
                                code: *mut *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1425:1"]
        pub fn encode_krb5_priv(rep: *const krb5_priv,
                                code: *mut *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1428:1"]
        pub fn encode_krb5_enc_priv_part(rep: *const krb5_priv_enc_part,
                                         code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1431:1"]
        pub fn encode_krb5_cred(rep: *const krb5_cred,
                                code: *mut *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1436:1"]
        pub fn encode_krb5_enc_cred_part(rep: *const krb5_cred_enc_part,
                                         code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1439:1"]
        pub fn encode_krb5_error(rep: *const krb5_error,
                                 code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1442:1"]
        pub fn encode_krb5_authdata(rep: *const *mut krb5_authdata,
                                    code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1445:1"]
        pub fn encode_krb5_padata_sequence(rep: *const *mut krb5_pa_data,
                                           code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1448:1"]
        pub fn encode_krb5_typed_data(rep: *const *mut krb5_pa_data,
                                      code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1451:1"]
        pub fn encode_krb5_etype_info(_: *const *mut krb5_etype_info_entry,
                                      code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1454:1"]
        pub fn encode_krb5_etype_info2(_: *const *mut krb5_etype_info_entry,
                                       code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1457:1"]
        pub fn encode_krb5_pa_enc_ts(_: *const krb5_pa_enc_ts,
                                     _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1460:1"]
        pub fn encode_krb5_sam_challenge_2(_: *const krb5_sam_challenge_2,
                                           _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1463:1"]
        pub fn encode_krb5_sam_challenge_2_body(_:
                                                    *const krb5_sam_challenge_2_body,
                                                _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1467:1"]
        pub fn encode_krb5_enc_sam_response_enc_2(_:
                                                      *const krb5_enc_sam_response_enc_2,
                                                  _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1471:1"]
        pub fn encode_krb5_sam_response_2(_: *const krb5_sam_response_2,
                                          _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1481:1"]
        pub fn encode_krb5_pa_for_user(_: *const krb5_pa_for_user,
                                       _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1487:1"]
        pub fn encode_krb5_pa_s4u_x509_user(_: *const krb5_pa_s4u_x509_user,
                                            _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1502:1"]
        pub fn encode_krb5_pa_fx_fast_reply(_: *const krb5_enc_data,
                                            _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1505:1"]
        pub fn encode_krb5_iakerb_header(_: *const krb5_iakerb_header,
                                         _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1508:1"]
        pub fn encode_krb5_iakerb_finished(_: *const krb5_iakerb_finished,
                                           _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1511:1"]
        pub fn encode_krb5_fast_response(_: *const krb5_fast_response,
                                         _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1514:1"]
        pub fn encode_krb5_ad_kdcissued(_: *const krb5_ad_kdcissued,
                                        _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1517:1"]
        pub fn encode_krb5_ad_signedpath(_: *const krb5_ad_signedpath,
                                         _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1554:1"]
        pub fn decode_krb5_sam_challenge_2(_: *const krb5_data,
                                           _: *mut *mut krb5_sam_challenge_2)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1557:1"]
        pub fn decode_krb5_sam_challenge_2_body(_: *const krb5_data,
                                                _:
                                                    *mut *mut krb5_sam_challenge_2_body)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1561:1"]
        pub fn decode_krb5_enc_sam_response_enc_2(_: *const krb5_data,
                                                  _:
                                                      *mut *mut krb5_enc_sam_response_enc_2)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1565:1"]
        pub fn decode_krb5_sam_response_2(_: *const krb5_data,
                                          _: *mut *mut krb5_sam_response_2)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1583:1"]
        pub fn decode_krb5_authenticator(code: *const krb5_data,
                                         rep: *mut *mut krb5_authenticator)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1586:1"]
        pub fn decode_krb5_ticket(code: *const krb5_data,
                                  rep: *mut *mut krb5_ticket)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1589:1"]
        pub fn decode_krb5_encryption_key(output: *const krb5_data,
                                          rep: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1592:1"]
        pub fn decode_krb5_enc_tkt_part(output: *const krb5_data,
                                        rep: *mut *mut krb5_enc_tkt_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1595:1"]
        pub fn decode_krb5_enc_kdc_rep_part(output: *const krb5_data,
                                            rep:
                                                *mut *mut krb5_enc_kdc_rep_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1599:1"]
        pub fn decode_krb5_as_rep(output: *const krb5_data,
                                  rep: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1602:1"]
        pub fn decode_krb5_tgs_rep(output: *const krb5_data,
                                   rep: *mut *mut krb5_kdc_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1605:1"]
        pub fn decode_krb5_ap_req(output: *const krb5_data,
                                  rep: *mut *mut krb5_ap_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1608:1"]
        pub fn decode_krb5_ap_rep(output: *const krb5_data,
                                  rep: *mut *mut krb5_ap_rep)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1611:1"]
        pub fn decode_krb5_ap_rep_enc_part(output: *const krb5_data,
                                           rep:
                                               *mut *mut krb5_ap_rep_enc_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1615:1"]
        pub fn decode_krb5_as_req(output: *const krb5_data,
                                  rep: *mut *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1618:1"]
        pub fn decode_krb5_tgs_req(output: *const krb5_data,
                                   rep: *mut *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1621:1"]
        pub fn decode_krb5_kdc_req_body(output: *const krb5_data,
                                        rep: *mut *mut krb5_kdc_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1624:1"]
        pub fn decode_krb5_safe(output: *const krb5_data,
                                rep: *mut *mut krb5_safe) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1631:1"]
        pub fn decode_krb5_priv(output: *const krb5_data,
                                rep: *mut *mut krb5_priv) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1634:1"]
        pub fn decode_krb5_enc_priv_part(output: *const krb5_data,
                                         rep: *mut *mut krb5_priv_enc_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1639:1"]
        pub fn decode_krb5_cred(output: *const krb5_data,
                                rep: *mut *mut krb5_cred) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1642:1"]
        pub fn decode_krb5_enc_cred_part(output: *const krb5_data,
                                         rep: *mut *mut krb5_cred_enc_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1648:1"]
        pub fn decode_krb5_authdata(output: *const krb5_data,
                                    rep: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1651:1"]
        pub fn decode_krb5_padata_sequence(output: *const krb5_data,
                                           rep: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1654:1"]
        pub fn decode_krb5_typed_data(_: *const krb5_data,
                                      _: *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1657:1"]
        pub fn decode_krb5_etype_info(output: *const krb5_data,
                                      rep:
                                          *mut *mut *mut krb5_etype_info_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1660:1"]
        pub fn decode_krb5_etype_info2(output: *const krb5_data,
                                       rep:
                                           *mut *mut *mut krb5_etype_info_entry)
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
        #[c2rust::src_loc = "1672:1"]
        pub fn decode_krb5_pa_for_user(_: *const krb5_data,
                                       _: *mut *mut krb5_pa_for_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1675:1"]
        pub fn decode_krb5_pa_s4u_x509_user(_: *const krb5_data,
                                            _:
                                                *mut *mut krb5_pa_s4u_x509_user)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1690:1"]
        pub fn decode_krb5_pa_fx_fast_reply(_: *const krb5_data,
                                            _: *mut *mut krb5_enc_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1693:1"]
        pub fn decode_krb5_fast_response(_: *const krb5_data,
                                         _: *mut *mut krb5_fast_response)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1696:1"]
        pub fn decode_krb5_ad_kdcissued(_: *const krb5_data,
                                        _: *mut *mut krb5_ad_kdcissued)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1699:1"]
        pub fn decode_krb5_ad_signedpath(_: *const krb5_data,
                                         _: *mut *mut krb5_ad_signedpath)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1702:1"]
        pub fn decode_krb5_iakerb_header(_: *const krb5_data,
                                         _: *mut *mut krb5_iakerb_header)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1705:1"]
        pub fn decode_krb5_iakerb_finished(_: *const krb5_data,
                                           _: *mut *mut krb5_iakerb_finished)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2147:1"]
        pub fn krb5_free_enc_tkt_part(_: krb5_context,
                                      _: *mut krb5_enc_tkt_part);
        #[no_mangle]
        #[c2rust::src_loc = "2150:1"]
        pub fn krb5_free_kdc_req(_: krb5_context, _: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "2151:1"]
        pub fn krb5_free_kdc_rep(_: krb5_context, _: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "2153:1"]
        pub fn krb5_free_enc_kdc_rep_part(_: krb5_context,
                                          _: *mut krb5_enc_kdc_rep_part);
        #[no_mangle]
        #[c2rust::src_loc = "2155:1"]
        pub fn krb5_free_ap_req(_: krb5_context, _: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "2156:1"]
        pub fn krb5_free_ap_rep(_: krb5_context, _: *mut krb5_ap_rep);
        #[no_mangle]
        #[c2rust::src_loc = "2157:1"]
        pub fn krb5_free_cred(_: krb5_context, _: *mut krb5_cred);
        #[no_mangle]
        #[c2rust::src_loc = "2158:1"]
        pub fn krb5_free_cred_enc_part(_: krb5_context,
                                       _: *mut krb5_cred_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2161:1"]
        pub fn krb5_free_enc_data(_: krb5_context, _: *mut krb5_enc_data);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:38"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:38"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:38"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:38"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:38"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:38"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:38"]
pub mod k5_int_pkinit_h {
    use super::krb5_h::{krb5_enc_data, krb5_data, krb5_error_code,
                        krb5_keyblock};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn encode_krb5_enc_data(_: *const krb5_enc_data,
                                    _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn encode_krb5_encryption_key(rep: *const krb5_keyblock,
                                          code: *mut *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/utility.h:40"]
pub mod utility_h {
    use super::stddef_h::size_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/utility.h */
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
        /* Aborts on failure.  ealloc returns zero-filled memory. */
        #[no_mangle]
        #[c2rust::src_loc = "33:1"]
        pub fn ealloc(size: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn init_access(progname: *const libc::c_char);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/ktest.h:41"]
pub mod ktest_h {
    use super::krb5_h::{krb5_keyblock, krb5_principal, krb5_authenticator,
                        krb5_ticket, krb5_enc_data, krb5_enc_tkt_part,
                        krb5_authdata, krb5_enc_kdc_rep_part, krb5_kdc_req,
                        krb5_kdc_rep, krb5_pa_data, krb5_ap_req, krb5_ap_rep,
                        krb5_ap_rep_enc_part, krb5_cred, krb5_cred_enc_part,
                        krb5_error, krb5_address, krb5_data, krb5_checksum};
    use super::k5_int_h::{krb5_safe, krb5_priv, krb5_priv_enc_part,
                          krb5_etype_info_entry, krb5_pa_enc_ts,
                          krb5_sam_challenge_2, krb5_sam_challenge_2_body,
                          krb5_sam_response_2, krb5_enc_sam_response_enc_2,
                          krb5_pa_for_user, krb5_pa_s4u_x509_user,
                          krb5_ad_kdcissued, krb5_ad_signedpath,
                          krb5_iakerb_header, krb5_iakerb_finished,
                          krb5_fast_response};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn ktest_make_sample_keyblock(kb: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn ktest_make_sample_principal(p: *mut krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn ktest_make_sample_authenticator(a: *mut krb5_authenticator);
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn ktest_make_sample_ticket(tkt: *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub fn ktest_make_sample_enc_data(ed: *mut krb5_enc_data);
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn ktest_make_sample_enc_tkt_part(etp: *mut krb5_enc_tkt_part);
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn ktest_make_sample_authorization_data(ad:
                                                        *mut *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn ktest_make_sample_enc_kdc_rep_part(ekr:
                                                      *mut krb5_enc_kdc_rep_part);
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn ktest_make_sample_kdc_req(kr: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn ktest_make_sample_kdc_rep(kdcr: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ktest_make_sample_pa_data_array(pad:
                                                   *mut *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "62:1"]
        pub fn ktest_make_sample_empty_pa_data_array(pad:
                                                         *mut *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn ktest_make_sample_ap_req(ar: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn ktest_make_sample_ap_rep(ar: *mut krb5_ap_rep);
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn ktest_make_sample_ap_rep_enc_part(arep:
                                                     *mut krb5_ap_rep_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn ktest_make_sample_kdc_req_body(krb: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn ktest_make_sample_safe(s: *mut krb5_safe);
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn ktest_make_sample_priv(p: *mut krb5_priv);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn ktest_make_sample_priv_enc_part(pep: *mut krb5_priv_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn ktest_make_sample_cred(c: *mut krb5_cred);
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn ktest_make_sample_cred_enc_part(cep: *mut krb5_cred_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn ktest_make_sample_error(kerr: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn ktest_make_sample_etype_info(p:
                                                *mut *mut *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn ktest_make_sample_etype_info2(p:
                                                 *mut *mut *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn ktest_make_sample_pa_enc_ts(am: *mut krb5_pa_enc_ts);
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn ktest_make_sample_sam_challenge_2(p:
                                                     *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn ktest_make_sample_sam_challenge_2_body(p:
                                                          *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "83:1"]
        pub fn ktest_make_sample_sam_response_2(p: *mut krb5_sam_response_2);
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn ktest_make_sample_enc_sam_response_enc_2(p:
                                                            *mut krb5_enc_sam_response_enc_2);
        #[no_mangle]
        #[c2rust::src_loc = "85:1"]
        pub fn ktest_make_sample_pa_for_user(p: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "86:1"]
        pub fn ktest_make_sample_pa_s4u_x509_user(p:
                                                      *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "87:1"]
        pub fn ktest_make_sample_ad_kdcissued(p: *mut krb5_ad_kdcissued);
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn ktest_make_sample_ad_signedpath(p: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn ktest_make_sample_iakerb_header(p: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn ktest_make_sample_iakerb_finished(p:
                                                     *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn ktest_make_sample_fast_response(p: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn ktest_empty_authorization_data(ad: *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "131:1"]
        pub fn ktest_destroy_authorization_data(ad:
                                                    *mut *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn ktest_destroy_addresses(a: *mut *mut *mut krb5_address);
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn ktest_destroy_address(a: *mut *mut krb5_address);
        #[no_mangle]
        #[c2rust::src_loc = "137:1"]
        pub fn ktest_destroy_pa_data_array(pad: *mut *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "141:1"]
        pub fn ktest_empty_data(d: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "142:1"]
        pub fn ktest_destroy_principal(p: *mut krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "143:1"]
        pub fn ktest_destroy_checksum(cs: *mut *mut krb5_checksum);
        #[no_mangle]
        #[c2rust::src_loc = "144:1"]
        pub fn ktest_empty_keyblock(kb: *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn ktest_destroy_keyblock(kb: *mut *mut krb5_keyblock);
        #[no_mangle]
        #[c2rust::src_loc = "146:1"]
        pub fn ktest_destroy_authdata(ad: *mut *mut krb5_authdata);
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn ktest_destroy_sequence_of_ticket(sot:
                                                    *mut *mut *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn ktest_empty_ticket(tkt: *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "151:1"]
        pub fn ktest_destroy_enc_data(ed: *mut krb5_enc_data);
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn ktest_empty_error(kerr: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "153:1"]
        pub fn ktest_destroy_etype_info_entry(i: *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn ktest_destroy_etype_info(info:
                                            *mut *mut krb5_etype_info_entry);
        #[no_mangle]
        #[c2rust::src_loc = "156:1"]
        pub fn ktest_empty_kdc_req(kr: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "157:1"]
        pub fn ktest_empty_kdc_rep(kr: *mut krb5_kdc_rep);
        #[no_mangle]
        #[c2rust::src_loc = "159:1"]
        pub fn ktest_empty_authenticator(a: *mut krb5_authenticator);
        #[no_mangle]
        #[c2rust::src_loc = "171:1"]
        pub fn ktest_empty_priv_enc_part(pep: *mut krb5_priv_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "172:1"]
        pub fn ktest_empty_cred(c: *mut krb5_cred);
        #[no_mangle]
        #[c2rust::src_loc = "174:1"]
        pub fn ktest_empty_ap_rep_enc_part(arep: *mut krb5_ap_rep_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn ktest_empty_sam_challenge_2(p: *mut krb5_sam_challenge_2);
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn ktest_empty_sam_challenge_2_body(p:
                                                    *mut krb5_sam_challenge_2_body);
        #[no_mangle]
        #[c2rust::src_loc = "177:1"]
        pub fn ktest_empty_sam_response_2(p: *mut krb5_sam_response_2);
        #[no_mangle]
        #[c2rust::src_loc = "178:1"]
        pub fn ktest_empty_enc_sam_response_enc_2(p:
                                                      *mut krb5_enc_sam_response_enc_2);
        #[no_mangle]
        #[c2rust::src_loc = "179:1"]
        pub fn ktest_empty_pa_for_user(p: *mut krb5_pa_for_user);
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn ktest_empty_pa_s4u_x509_user(p: *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "181:1"]
        pub fn ktest_empty_ad_kdcissued(p: *mut krb5_ad_kdcissued);
        #[no_mangle]
        #[c2rust::src_loc = "183:1"]
        pub fn ktest_empty_ad_signedpath(p: *mut krb5_ad_signedpath);
        #[no_mangle]
        #[c2rust::src_loc = "184:1"]
        pub fn ktest_empty_iakerb_header(p: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "185:1"]
        pub fn ktest_empty_iakerb_finished(p: *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "186:1"]
        pub fn ktest_empty_fast_response(p: *mut krb5_fast_response);
        #[no_mangle]
        #[c2rust::src_loc = "170:1"]
        pub fn ktest_empty_priv(p: *mut krb5_priv);
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn ktest_empty_ap_rep(ar: *mut krb5_ap_rep);
        #[no_mangle]
        #[c2rust::src_loc = "165:1"]
        pub fn ktest_empty_ap_req(ar: *mut krb5_ap_req);
        #[no_mangle]
        #[c2rust::src_loc = "166:1"]
        pub fn ktest_empty_cred_enc_part(cep: *mut krb5_cred_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "169:1"]
        pub fn ktest_empty_safe(s: *mut krb5_safe);
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_cksumtype, krb5_authdatatype, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_authenticator, krb5_authenticator,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_ap_req, krb5_ap_req,
                       _krb5_ap_rep, krb5_ap_rep, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, _krb5_cred_info, krb5_cred_info,
                       _krb5_cred_enc_part, krb5_cred_enc_part, _krb5_cred,
                       krb5_cred, _profile_t, krb5_init_context,
                       krb5_free_context, krb5_free_authenticator,
                       krb5_free_authdata, krb5_free_ticket, krb5_free_error,
                       krb5_free_keyblock, krb5_free_ap_rep_enc_part,
                       krb5_free_data};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_etype_info_entry,
                         krb5_etype_info_entry, krb5_etype_info,
                         _krb5_sam_challenge_2, krb5_sam_challenge_2,
                         _krb5_sam_challenge_2_body,
                         krb5_sam_challenge_2_body, _krb5_sam_response_2,
                         krb5_sam_response_2, _krb5_enc_sam_response_enc_2,
                         krb5_enc_sam_response_enc_2, _krb5_pa_enc_ts,
                         krb5_pa_enc_ts, _krb5_pa_for_user, krb5_pa_for_user,
                         _krb5_s4u_userid, krb5_s4u_userid,
                         _krb5_pa_s4u_x509_user, krb5_pa_s4u_x509_user,
                         _krb5_fast_finished, krb5_fast_finished,
                         _krb5_fast_response, krb5_fast_response,
                         _krb5_ad_kdcissued, krb5_ad_kdcissued,
                         _krb5_ad_signedpath, krb5_ad_signedpath,
                         _krb5_iakerb_header, krb5_iakerb_header,
                         _krb5_iakerb_finished, krb5_iakerb_finished,
                         _krb5_safe, krb5_safe, _krb5_priv, krb5_priv,
                         _krb5_priv_enc_part, krb5_priv_enc_part,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_free_etype_info,
                         krb5_free_sam_challenge_2,
                         krb5_free_sam_challenge_2_body,
                         krb5_free_sam_response_2,
                         krb5_free_enc_sam_response_enc_2,
                         krb5_free_pa_enc_ts, krb5_free_pa_for_user,
                         krb5_free_pa_s4u_x509_user, krb5_free_fast_response,
                         krb5_free_ad_kdcissued, krb5_free_ad_signedpath,
                         krb5_free_iakerb_header, krb5_free_iakerb_finished,
                         krb5_free_safe, krb5_free_priv,
                         krb5_free_priv_enc_part, encode_krb5_authenticator,
                         encode_krb5_ticket, encode_krb5_enc_tkt_part,
                         encode_krb5_enc_kdc_rep_part, encode_krb5_as_rep,
                         encode_krb5_tgs_rep, encode_krb5_ap_req,
                         encode_krb5_ap_rep, encode_krb5_ap_rep_enc_part,
                         encode_krb5_as_req, encode_krb5_tgs_req,
                         encode_krb5_kdc_req_body, encode_krb5_safe,
                         encode_krb5_priv, encode_krb5_enc_priv_part,
                         encode_krb5_cred, encode_krb5_enc_cred_part,
                         encode_krb5_error, encode_krb5_authdata,
                         encode_krb5_padata_sequence, encode_krb5_typed_data,
                         encode_krb5_etype_info, encode_krb5_etype_info2,
                         encode_krb5_pa_enc_ts, encode_krb5_sam_challenge_2,
                         encode_krb5_sam_challenge_2_body,
                         encode_krb5_enc_sam_response_enc_2,
                         encode_krb5_sam_response_2, encode_krb5_pa_for_user,
                         encode_krb5_pa_s4u_x509_user,
                         encode_krb5_pa_fx_fast_reply,
                         encode_krb5_iakerb_header,
                         encode_krb5_iakerb_finished,
                         encode_krb5_fast_response, encode_krb5_ad_kdcissued,
                         encode_krb5_ad_signedpath,
                         decode_krb5_sam_challenge_2,
                         decode_krb5_sam_challenge_2_body,
                         decode_krb5_enc_sam_response_enc_2,
                         decode_krb5_sam_response_2,
                         decode_krb5_authenticator, decode_krb5_ticket,
                         decode_krb5_encryption_key, decode_krb5_enc_tkt_part,
                         decode_krb5_enc_kdc_rep_part, decode_krb5_as_rep,
                         decode_krb5_tgs_rep, decode_krb5_ap_req,
                         decode_krb5_ap_rep, decode_krb5_ap_rep_enc_part,
                         decode_krb5_as_req, decode_krb5_tgs_req,
                         decode_krb5_kdc_req_body, decode_krb5_safe,
                         decode_krb5_priv, decode_krb5_enc_priv_part,
                         decode_krb5_cred, decode_krb5_enc_cred_part,
                         decode_krb5_error, decode_krb5_authdata,
                         decode_krb5_padata_sequence, decode_krb5_typed_data,
                         decode_krb5_etype_info, decode_krb5_etype_info2,
                         decode_krb5_enc_data, decode_krb5_pa_enc_ts,
                         decode_krb5_pa_for_user,
                         decode_krb5_pa_s4u_x509_user,
                         decode_krb5_pa_fx_fast_reply,
                         decode_krb5_fast_response, decode_krb5_ad_kdcissued,
                         decode_krb5_ad_signedpath, decode_krb5_iakerb_header,
                         decode_krb5_iakerb_finished, krb5_free_enc_tkt_part,
                         krb5_free_kdc_req, krb5_free_kdc_rep,
                         krb5_free_enc_kdc_rep_part, krb5_free_ap_req,
                         krb5_free_ap_rep, krb5_free_cred,
                         krb5_free_cred_enc_part, krb5_free_pa_data,
                         krb5_free_enc_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
use self::stdlib_h::{free, exit};
use self::string_h::memset;
use self::k5_int_pkinit_h::{encode_krb5_enc_data, encode_krb5_encryption_key};
use self::utility_h::{ealloc, init_access};
use self::ktest_h::{ktest_make_sample_keyblock, ktest_make_sample_principal,
                    ktest_make_sample_authenticator, ktest_make_sample_ticket,
                    ktest_make_sample_enc_data,
                    ktest_make_sample_enc_tkt_part,
                    ktest_make_sample_authorization_data,
                    ktest_make_sample_enc_kdc_rep_part,
                    ktest_make_sample_kdc_req, ktest_make_sample_kdc_rep,
                    ktest_make_sample_pa_data_array,
                    ktest_make_sample_empty_pa_data_array,
                    ktest_make_sample_ap_req, ktest_make_sample_ap_rep,
                    ktest_make_sample_ap_rep_enc_part,
                    ktest_make_sample_kdc_req_body, ktest_make_sample_safe,
                    ktest_make_sample_priv, ktest_make_sample_priv_enc_part,
                    ktest_make_sample_cred, ktest_make_sample_cred_enc_part,
                    ktest_make_sample_error, ktest_make_sample_etype_info,
                    ktest_make_sample_etype_info2,
                    ktest_make_sample_pa_enc_ts,
                    ktest_make_sample_sam_challenge_2,
                    ktest_make_sample_sam_challenge_2_body,
                    ktest_make_sample_sam_response_2,
                    ktest_make_sample_enc_sam_response_enc_2,
                    ktest_make_sample_pa_for_user,
                    ktest_make_sample_pa_s4u_x509_user,
                    ktest_make_sample_ad_kdcissued,
                    ktest_make_sample_ad_signedpath,
                    ktest_make_sample_iakerb_header,
                    ktest_make_sample_iakerb_finished,
                    ktest_make_sample_fast_response,
                    ktest_empty_authorization_data,
                    ktest_destroy_authorization_data, ktest_destroy_addresses,
                    ktest_destroy_address, ktest_destroy_pa_data_array,
                    ktest_empty_data, ktest_destroy_principal,
                    ktest_destroy_checksum, ktest_empty_keyblock,
                    ktest_destroy_keyblock, ktest_destroy_authdata,
                    ktest_destroy_sequence_of_ticket, ktest_empty_ticket,
                    ktest_destroy_enc_data, ktest_empty_error,
                    ktest_destroy_etype_info_entry, ktest_destroy_etype_info,
                    ktest_empty_kdc_req, ktest_empty_kdc_rep,
                    ktest_empty_authenticator, ktest_empty_priv_enc_part,
                    ktest_empty_cred, ktest_empty_ap_rep_enc_part,
                    ktest_empty_sam_challenge_2,
                    ktest_empty_sam_challenge_2_body,
                    ktest_empty_sam_response_2,
                    ktest_empty_enc_sam_response_enc_2,
                    ktest_empty_pa_for_user, ktest_empty_pa_s4u_x509_user,
                    ktest_empty_ad_kdcissued, ktest_empty_ad_signedpath,
                    ktest_empty_iakerb_header, ktest_empty_iakerb_finished,
                    ktest_empty_fast_response, ktest_empty_priv,
                    ktest_empty_ap_rep, ktest_empty_ap_req,
                    ktest_empty_cred_enc_part, ktest_empty_safe};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/krb5_decode_leak.c */
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
 * This program is intended to help detect memory leaks in the ASN.1
 * decoder functions by exercising their failure paths.  The setup
 * code for the test cases is copied from krb5_encode_test.c.
 *
 * This code does not actually detect leaks by itself; it must be run
 * through a leak-detection tool such as valgrind to do so.  Simply
 * running the program will exercise a bunch of ASN.1 encoder and
 * decoder code paths but won't validate the results.
 */
#[no_mangle]
#[c2rust::src_loc = "44:14"]
pub static mut test_context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
/*
 * Contrary to our usual convention, krb5_free_cred_enc_part is a
 * contents-only free function (and is assumed to be by mk_cred and
 * rd_cred) and we have no whole-structure free function for that data
 * type.  So create one here.
 */
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn free_cred_enc_part_whole(mut ctx: krb5_context,
                                              mut val:
                                                  *mut krb5_cred_enc_part) {
    krb5_free_cred_enc_part(ctx, val);
    free(val as *mut libc::c_void);
}
#[c2rust::src_loc = "60:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut code: *mut krb5_data = 0 as *mut krb5_data;
    let mut retval: krb5_error_code = 0;
    let mut i: libc::c_uint = 0;
    retval = krb5_init_context(&mut test_context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while initializing krb5\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    init_access(*argv.offset(0 as libc::c_int as isize));
    /*
     * Encode a value.  Then attempt to trigger most failure paths of
     * the decoder function by passing in corrupt encodings, which we
     * generate by perturbing each byte of the encoding in turn.  Some
     * of the perturbed encodings are expected to decode successfully,
     * so we need a free function to discard successful results.  Make
     * sure to define a pointer named "tmp" of the correct type in the
     * enclosing block.
     */
    /* ***************************************************************/
    /* encode_krb5_authenticator */
    let mut authent: krb5_authenticator =
        krb5_authenticator{magic: 0,
                           client: 0 as *mut krb5_principal_data,
                           checksum: 0 as *mut krb5_checksum,
                           cusec: 0,
                           ctime: 0,
                           subkey: 0 as *mut krb5_keyblock,
                           seq_number: 0,
                           authorization_data: 0 as *mut *mut krb5_authdata,};
    let mut tmp: *mut krb5_authenticator = 0 as *mut krb5_authenticator;
    ktest_make_sample_authenticator(&mut authent);
    retval = encode_krb5_authenticator(&mut authent, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_authenticator(code, &mut tmp);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_authenticator(test_context, tmp);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_checksum(&mut authent.checksum);
    ktest_destroy_keyblock(&mut authent.subkey);
    authent.seq_number = 0 as libc::c_int as krb5_ui_4;
    ktest_empty_authorization_data(authent.authorization_data);
    retval = encode_krb5_authenticator(&mut authent, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_authenticator(code, &mut tmp);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_authenticator(test_context, tmp);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_authorization_data(&mut authent.authorization_data);
    retval = encode_krb5_authenticator(&mut authent, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_authenticator(code, &mut tmp);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_authenticator(test_context, tmp);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_authenticator(&mut authent);
    /* ***************************************************************/
    /* encode_krb5_ticket */
    let mut tkt: krb5_ticket =
        krb5_ticket{magic: 0,
                    server: 0 as *mut krb5_principal_data,
                    enc_part:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},
                    enc_part2: 0 as *mut krb5_enc_tkt_part,};
    let mut tmp_0: *mut krb5_ticket = 0 as *mut krb5_ticket;
    ktest_make_sample_ticket(&mut tkt);
    retval = encode_krb5_ticket(&mut tkt, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ticket(code, &mut tmp_0);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ticket(test_context, tmp_0);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ticket(&mut tkt);
    /* ***************************************************************/
    /* encode_krb5_encryption_key */
    let mut keyblk: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut tmp_1: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    ktest_make_sample_keyblock(&mut keyblk);
    retval = encode_krb5_encryption_key(&mut keyblk, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_encryption_key(code, &mut tmp_1);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_keyblock(test_context, tmp_1);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_keyblock(&mut keyblk);
    /* ***************************************************************/
    /* encode_krb5_enc_tkt_part */
    let mut tkt_0: krb5_ticket =
        krb5_ticket{magic: 0,
                    server: 0 as *mut krb5_principal_data,
                    enc_part:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},
                    enc_part2: 0 as *mut krb5_enc_tkt_part,};
    let mut tmp_2: *mut krb5_enc_tkt_part = 0 as *mut krb5_enc_tkt_part;
    memset(&mut tkt_0 as *mut krb5_ticket as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_ticket>() as libc::c_ulong);
    tkt_0.enc_part2 =
        ealloc(::std::mem::size_of::<krb5_enc_tkt_part>() as libc::c_ulong) as
            *mut krb5_enc_tkt_part;
    ktest_make_sample_enc_tkt_part(tkt_0.enc_part2);
    retval = encode_krb5_enc_tkt_part(tkt_0.enc_part2, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_tkt_part(code, &mut tmp_2);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_tkt_part(test_context, tmp_2);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    (*tkt_0.enc_part2).times.starttime = 0 as libc::c_int;
    (*tkt_0.enc_part2).times.renew_till = 0 as libc::c_int;
    ktest_destroy_address(&mut *(*tkt_0.enc_part2).caddrs.offset(1 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
    ktest_destroy_address(&mut *(*tkt_0.enc_part2).caddrs.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize));
    ktest_destroy_authdata(&mut *(*tkt_0.enc_part2).authorization_data.offset(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
    ktest_destroy_authdata(&mut *(*tkt_0.enc_part2).authorization_data.offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize));
    /* ISODE version fails on the empty caddrs field */
    ktest_destroy_addresses(&mut (*tkt_0.enc_part2).caddrs);
    ktest_destroy_authorization_data(&mut (*tkt_0.enc_part2).authorization_data);
    retval = encode_krb5_enc_tkt_part(tkt_0.enc_part2, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_tkt_part(code, &mut tmp_2);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_tkt_part(test_context, tmp_2);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ticket(&mut tkt_0);
    /* ***************************************************************/
    /* encode_krb5_enc_kdc_rep_part */
    let mut kdcr: krb5_kdc_rep =
        krb5_kdc_rep{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     client: 0 as *mut krb5_principal_data,
                     ticket: 0 as *mut krb5_ticket,
                     enc_part:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     enc_part2: 0 as *mut krb5_enc_kdc_rep_part,};
    let mut tmp_3: *mut krb5_enc_kdc_rep_part =
        0 as *mut krb5_enc_kdc_rep_part;
    memset(&mut kdcr as *mut krb5_kdc_rep as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_kdc_rep>() as libc::c_ulong);
    kdcr.enc_part2 =
        ealloc(::std::mem::size_of::<krb5_enc_kdc_rep_part>() as
                   libc::c_ulong) as *mut krb5_enc_kdc_rep_part;
    ktest_make_sample_enc_kdc_rep_part(kdcr.enc_part2);
    retval = encode_krb5_enc_kdc_rep_part(kdcr.enc_part2, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_kdc_rep_part(code, &mut tmp_3);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_kdc_rep_part(test_context, tmp_3);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    (*kdcr.enc_part2).key_exp = 0 as libc::c_int;
    (*kdcr.enc_part2).times.starttime = 0 as libc::c_int;
    (*kdcr.enc_part2).flags &= !(0x800000 as libc::c_int);
    ktest_destroy_addresses(&mut (*kdcr.enc_part2).caddrs);
    retval = encode_krb5_enc_kdc_rep_part(kdcr.enc_part2, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_kdc_rep_part(code, &mut tmp_3);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_kdc_rep_part(test_context, tmp_3);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_kdc_rep(&mut kdcr);
    /* ***************************************************************/
    /* encode_krb5_as_rep */
    let mut kdcr_0: krb5_kdc_rep =
        krb5_kdc_rep{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     client: 0 as *mut krb5_principal_data,
                     ticket: 0 as *mut krb5_ticket,
                     enc_part:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     enc_part2: 0 as *mut krb5_enc_kdc_rep_part,};
    let mut tmp_4: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    ktest_make_sample_kdc_rep(&mut kdcr_0);
    kdcr_0.msg_type = 11 as libc::c_int as krb5_msgtype;
    retval = encode_krb5_as_rep(&mut kdcr_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_as_rep(code, &mut tmp_4);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_rep(test_context, tmp_4);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_pa_data_array(&mut kdcr_0.padata);
    retval = encode_krb5_as_rep(&mut kdcr_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_as_rep(code, &mut tmp_4);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_rep(test_context, tmp_4);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_kdc_rep(&mut kdcr_0);
    /* ***************************************************************/
    /* encode_krb5_tgs_rep */
    let mut kdcr_1: krb5_kdc_rep =
        krb5_kdc_rep{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     client: 0 as *mut krb5_principal_data,
                     ticket: 0 as *mut krb5_ticket,
                     enc_part:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     enc_part2: 0 as *mut krb5_enc_kdc_rep_part,};
    let mut tmp_5: *mut krb5_kdc_rep = 0 as *mut krb5_kdc_rep;
    ktest_make_sample_kdc_rep(&mut kdcr_1);
    kdcr_1.msg_type = 13 as libc::c_int as krb5_msgtype;
    retval = encode_krb5_tgs_rep(&mut kdcr_1, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_tgs_rep(code, &mut tmp_5);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_rep(test_context, tmp_5);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_pa_data_array(&mut kdcr_1.padata);
    retval = encode_krb5_tgs_rep(&mut kdcr_1, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_tgs_rep(code, &mut tmp_5);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_rep(test_context, tmp_5);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_kdc_rep(&mut kdcr_1);
    /* ***************************************************************/
    /* encode_krb5_ap_req */
    let mut apreq: krb5_ap_req =
        krb5_ap_req{magic: 0,
                    ap_options: 0,
                    ticket: 0 as *mut krb5_ticket,
                    authenticator:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},};
    let mut tmp_6: *mut krb5_ap_req = 0 as *mut krb5_ap_req;
    ktest_make_sample_ap_req(&mut apreq);
    retval = encode_krb5_ap_req(&mut apreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ap_req(code, &mut tmp_6);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ap_req(test_context, tmp_6);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ap_req(&mut apreq);
    /* ***************************************************************/
    /* encode_krb5_ap_rep */
    let mut aprep: krb5_ap_rep =
        krb5_ap_rep{magic: 0,
                    enc_part:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},};
    let mut tmp_7: *mut krb5_ap_rep = 0 as *mut krb5_ap_rep;
    ktest_make_sample_ap_rep(&mut aprep);
    retval = encode_krb5_ap_rep(&mut aprep, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ap_rep(code, &mut tmp_7);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ap_rep(test_context, tmp_7);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ap_rep(&mut aprep);
    /* ***************************************************************/
    /* encode_krb5_ap_rep_enc_part */
    let mut apenc: krb5_ap_rep_enc_part =
        krb5_ap_rep_enc_part{magic: 0,
                             ctime: 0,
                             cusec: 0,
                             subkey: 0 as *mut krb5_keyblock,
                             seq_number: 0,};
    let mut tmp_8: *mut krb5_ap_rep_enc_part = 0 as *mut krb5_ap_rep_enc_part;
    ktest_make_sample_ap_rep_enc_part(&mut apenc);
    retval = encode_krb5_ap_rep_enc_part(&mut apenc, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ap_rep_enc_part(code, &mut tmp_8);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ap_rep_enc_part(test_context, tmp_8);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_keyblock(&mut apenc.subkey);
    apenc.seq_number = 0 as libc::c_int as krb5_ui_4;
    retval = encode_krb5_ap_rep_enc_part(&mut apenc, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ap_rep_enc_part(code, &mut tmp_8);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ap_rep_enc_part(test_context, tmp_8);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ap_rep_enc_part(&mut apenc);
    /* ***************************************************************/
    /* encode_krb5_as_req */
    let mut asreq: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut tmp_9: *mut krb5_kdc_req = 0 as *mut krb5_kdc_req;
    ktest_make_sample_kdc_req(&mut asreq);
    asreq.msg_type = 10 as libc::c_int as krb5_msgtype;
    asreq.kdc_options &= !(0x8 as libc::c_int);
    retval = encode_krb5_as_req(&mut asreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_as_req(code, &mut tmp_9);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_9);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_pa_data_array(&mut asreq.padata);
    ktest_destroy_principal(&mut asreq.client);
    ktest_destroy_principal(&mut asreq.server);
    asreq.kdc_options |= 0x8 as libc::c_int;
    asreq.from = 0 as libc::c_int;
    asreq.rtime = 0 as libc::c_int;
    ktest_destroy_addresses(&mut asreq.addresses);
    ktest_destroy_enc_data(&mut asreq.authorization_data);
    retval = encode_krb5_as_req(&mut asreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_as_req(code, &mut tmp_9);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_9);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_sequence_of_ticket(&mut asreq.second_ticket);
    ktest_make_sample_principal(&mut asreq.server);
    asreq.kdc_options &= !(0x8 as libc::c_int);
    retval = encode_krb5_as_req(&mut asreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_as_req(code, &mut tmp_9);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_9);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_kdc_req(&mut asreq);
    /* ***************************************************************/
    /* encode_krb5_tgs_req */
    let mut tgsreq: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut tmp_10: *mut krb5_kdc_req = 0 as *mut krb5_kdc_req;
    ktest_make_sample_kdc_req(&mut tgsreq);
    tgsreq.msg_type = 12 as libc::c_int as krb5_msgtype;
    tgsreq.kdc_options &= !(0x8 as libc::c_int);
    retval = encode_krb5_tgs_req(&mut tgsreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_tgs_req(code, &mut tmp_10);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_10);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_pa_data_array(&mut tgsreq.padata);
    ktest_destroy_principal(&mut tgsreq.client);
    ktest_destroy_principal(&mut tgsreq.server);
    tgsreq.kdc_options |= 0x8 as libc::c_int;
    tgsreq.from = 0 as libc::c_int;
    tgsreq.rtime = 0 as libc::c_int;
    ktest_destroy_addresses(&mut tgsreq.addresses);
    ktest_destroy_enc_data(&mut tgsreq.authorization_data);
    retval = encode_krb5_tgs_req(&mut tgsreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_tgs_req(code, &mut tmp_10);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_10);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_sequence_of_ticket(&mut tgsreq.second_ticket);
    ktest_make_sample_principal(&mut tgsreq.server);
    tgsreq.kdc_options &= !(0x8 as libc::c_int);
    retval = encode_krb5_tgs_req(&mut tgsreq, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_tgs_req(code, &mut tmp_10);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_10);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_kdc_req(&mut tgsreq);
    /* ***************************************************************/
    /* encode_krb5_kdc_req_body */
    let mut kdcrb: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut tmp_11: *mut krb5_kdc_req = 0 as *mut krb5_kdc_req;
    memset(&mut kdcrb as *mut krb5_kdc_req as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_kdc_req>() as libc::c_ulong);
    ktest_make_sample_kdc_req_body(&mut kdcrb);
    kdcrb.kdc_options &= !(0x8 as libc::c_int);
    retval = encode_krb5_kdc_req_body(&mut kdcrb, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_kdc_req_body(code, &mut tmp_11);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_11);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_principal(&mut kdcrb.client);
    ktest_destroy_principal(&mut kdcrb.server);
    kdcrb.kdc_options |= 0x8 as libc::c_int;
    kdcrb.from = 0 as libc::c_int;
    kdcrb.rtime = 0 as libc::c_int;
    ktest_destroy_addresses(&mut kdcrb.addresses);
    ktest_destroy_enc_data(&mut kdcrb.authorization_data);
    retval = encode_krb5_kdc_req_body(&mut kdcrb, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_kdc_req_body(code, &mut tmp_11);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_11);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_sequence_of_ticket(&mut kdcrb.second_ticket);
    ktest_make_sample_principal(&mut kdcrb.server);
    kdcrb.kdc_options &= !(0x8 as libc::c_int);
    retval = encode_krb5_kdc_req_body(&mut kdcrb, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_kdc_req_body(code, &mut tmp_11);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_kdc_req(test_context, tmp_11);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_kdc_req(&mut kdcrb);
    /* ***************************************************************/
    /* encode_krb5_safe */
    let mut s: krb5_safe =
        krb5_safe{magic: 0,
                  user_data:
                      krb5_data{magic: 0,
                                length: 0,
                                data: 0 as *mut libc::c_char,},
                  timestamp: 0,
                  usec: 0,
                  seq_number: 0,
                  s_address: 0 as *mut krb5_address,
                  r_address: 0 as *mut krb5_address,
                  checksum: 0 as *mut krb5_checksum,};
    let mut tmp_12: *mut krb5_safe = 0 as *mut krb5_safe;
    ktest_make_sample_safe(&mut s);
    retval = encode_krb5_safe(&mut s, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_safe(code, &mut tmp_12);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_safe(test_context, tmp_12);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    s.timestamp = 0 as libc::c_int;
    /* s.usec should be opted out by the timestamp */
    s.seq_number = 0 as libc::c_int as krb5_ui_4;
    ktest_destroy_address(&mut s.r_address);
    retval = encode_krb5_safe(&mut s, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_safe(code, &mut tmp_12);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_safe(test_context, tmp_12);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_safe(&mut s);
    /* ***************************************************************/
    /* encode_krb5_priv */
    let mut p: krb5_priv =
        krb5_priv{magic: 0,
                  enc_part:
                      krb5_enc_data{magic: 0,
                                    enctype: 0,
                                    kvno: 0,
                                    ciphertext:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},},};
    let mut tmp_13: *mut krb5_priv = 0 as *mut krb5_priv;
    ktest_make_sample_priv(&mut p);
    retval = encode_krb5_priv(&mut p, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_priv(code, &mut tmp_13);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_priv(test_context, tmp_13);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_priv(&mut p);
    /* ***************************************************************/
    /* encode_krb5_enc_priv_part */
    let mut ep: krb5_priv_enc_part =
        krb5_priv_enc_part{magic: 0,
                           user_data:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           timestamp: 0,
                           usec: 0,
                           seq_number: 0,
                           s_address: 0 as *mut krb5_address,
                           r_address: 0 as *mut krb5_address,};
    let mut tmp_14: *mut krb5_priv_enc_part = 0 as *mut krb5_priv_enc_part;
    ktest_make_sample_priv_enc_part(&mut ep);
    retval = encode_krb5_enc_priv_part(&mut ep, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_priv_part(code, &mut tmp_14);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_priv_enc_part(test_context, tmp_14);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ep.timestamp = 0 as libc::c_int;
    /* ep.usec should be opted out along with timestamp */
    ep.seq_number = 0 as libc::c_int as krb5_ui_4;
    ktest_destroy_address(&mut ep.r_address);
    retval = encode_krb5_enc_priv_part(&mut ep, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_priv_part(code, &mut tmp_14);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_priv_enc_part(test_context, tmp_14);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_priv_enc_part(&mut ep);
    /* ***************************************************************/
    /* encode_krb5_cred */
    let mut c: krb5_cred =
        krb5_cred{magic: 0,
                  tickets: 0 as *mut *mut krb5_ticket,
                  enc_part:
                      krb5_enc_data{magic: 0,
                                    enctype: 0,
                                    kvno: 0,
                                    ciphertext:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},},
                  enc_part2: 0 as *mut krb5_cred_enc_part,};
    let mut tmp_15: *mut krb5_cred = 0 as *mut krb5_cred;
    ktest_make_sample_cred(&mut c);
    retval = encode_krb5_cred(&mut c, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_cred(code, &mut tmp_15);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_cred(test_context, tmp_15);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_cred(&mut c);
    /* ***************************************************************/
    /* encode_krb5_enc_cred_part */
    let mut cep: krb5_cred_enc_part =
        krb5_cred_enc_part{magic: 0,
                           nonce: 0,
                           timestamp: 0,
                           usec: 0,
                           s_address: 0 as *mut krb5_address,
                           r_address: 0 as *mut krb5_address,
                           ticket_info: 0 as *mut *mut krb5_cred_info,};
    let mut tmp_16: *mut krb5_cred_enc_part = 0 as *mut krb5_cred_enc_part;
    ktest_make_sample_cred_enc_part(&mut cep);
    retval = encode_krb5_enc_cred_part(&mut cep, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_cred_part(code, &mut tmp_16);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            free_cred_enc_part_whole(test_context, tmp_16);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_principal(&mut (**cep.ticket_info.offset(0 as libc::c_int as
                                                               isize)).client);
    ktest_destroy_principal(&mut (**cep.ticket_info.offset(0 as libc::c_int as
                                                               isize)).server);
    (**cep.ticket_info.offset(0 as libc::c_int as isize)).flags =
        0 as libc::c_int;
    (**cep.ticket_info.offset(0 as libc::c_int as isize)).times.authtime =
        0 as libc::c_int;
    (**cep.ticket_info.offset(0 as libc::c_int as isize)).times.starttime =
        0 as libc::c_int;
    (**cep.ticket_info.offset(0 as libc::c_int as isize)).times.endtime =
        0 as libc::c_int;
    (**cep.ticket_info.offset(0 as libc::c_int as isize)).times.renew_till =
        0 as libc::c_int;
    ktest_destroy_addresses(&mut (**cep.ticket_info.offset(0 as libc::c_int as
                                                               isize)).caddrs);
    cep.nonce = 0 as libc::c_int;
    cep.timestamp = 0 as libc::c_int;
    ktest_destroy_address(&mut cep.s_address);
    ktest_destroy_address(&mut cep.r_address);
    retval = encode_krb5_enc_cred_part(&mut cep, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_cred_part(code, &mut tmp_16);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            free_cred_enc_part_whole(test_context, tmp_16);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_cred_enc_part(&mut cep);
    /* ***************************************************************/
    /* encode_krb5_error */
    let mut kerr: krb5_error =
        krb5_error{magic: 0,
                   ctime: 0,
                   cusec: 0,
                   susec: 0,
                   stime: 0,
                   error: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   text:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},};
    let mut tmp_17: *mut krb5_error = 0 as *mut krb5_error;
    ktest_make_sample_error(&mut kerr);
    retval = encode_krb5_error(&mut kerr, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_error(code, &mut tmp_17);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_error(test_context, tmp_17);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    kerr.ctime = 0 as libc::c_int;
    ktest_destroy_principal(&mut kerr.client);
    ktest_empty_data(&mut kerr.text);
    ktest_empty_data(&mut kerr.e_data);
    retval = encode_krb5_error(&mut kerr, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_error(code, &mut tmp_17);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_error(test_context, tmp_17);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_error(&mut kerr);
    /* ***************************************************************/
    /* encode_krb5_authdata */
    let mut ad: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut tmp_18: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    ktest_make_sample_authorization_data(&mut ad);
    retval = encode_krb5_authdata(ad, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_authdata(code, &mut tmp_18);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_authdata(test_context, tmp_18);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_authorization_data(&mut ad);
    /* ***************************************************************/
    /* encode_padata_sequence and encode_typed_data */
    let mut pa: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut tmp_19: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    ktest_make_sample_pa_data_array(&mut pa);
    retval = encode_krb5_padata_sequence(pa, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_padata_sequence(code, &mut tmp_19);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_data(test_context, tmp_19);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    retval = encode_krb5_typed_data(pa, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_typed_data(code, &mut tmp_19);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_data(test_context, tmp_19);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_pa_data_array(&mut pa);
    /* ***************************************************************/
    /* encode_padata_sequence (empty) */
    let mut pa_0: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut tmp_20: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    ktest_make_sample_empty_pa_data_array(&mut pa_0);
    retval = encode_krb5_padata_sequence(pa_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_padata_sequence(code, &mut tmp_20);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_data(test_context, tmp_20);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_pa_data_array(&mut pa_0);
    /* ***************************************************************/
    /* encode_etype_info */
    let mut info: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    let mut tmp_21: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    ktest_make_sample_etype_info(&mut info);
    retval = encode_krb5_etype_info(info, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_etype_info(code, &mut tmp_21);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_etype_info(test_context, tmp_21);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_etype_info_entry(*info.offset(2 as libc::c_int as isize));
    let ref mut fresh0 = *info.offset(2 as libc::c_int as isize);
    *fresh0 = 0 as *mut krb5_etype_info_entry;
    ktest_destroy_etype_info_entry(*info.offset(1 as libc::c_int as isize));
    let ref mut fresh1 = *info.offset(1 as libc::c_int as isize);
    *fresh1 = 0 as *mut krb5_etype_info_entry;
    retval = encode_krb5_etype_info(info, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_etype_info(code, &mut tmp_21);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_etype_info(test_context, tmp_21);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_etype_info_entry(*info.offset(0 as libc::c_int as isize));
    let ref mut fresh2 = *info.offset(0 as libc::c_int as isize);
    *fresh2 = 0 as *mut krb5_etype_info_entry;
    retval = encode_krb5_etype_info(info, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_etype_info(code, &mut tmp_21);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_etype_info(test_context, tmp_21);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_etype_info(info);
    /* encode_etype_info 2*/
    let mut info_0: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    let mut tmp_22: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    ktest_make_sample_etype_info2(&mut info_0);
    retval = encode_krb5_etype_info2(info_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_etype_info2(code, &mut tmp_22);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_etype_info(test_context, tmp_22);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_etype_info_entry(*info_0.offset(2 as libc::c_int as isize));
    let ref mut fresh3 = *info_0.offset(2 as libc::c_int as isize);
    *fresh3 = 0 as *mut krb5_etype_info_entry;
    ktest_destroy_etype_info_entry(*info_0.offset(1 as libc::c_int as isize));
    let ref mut fresh4 = *info_0.offset(1 as libc::c_int as isize);
    *fresh4 = 0 as *mut krb5_etype_info_entry;
    retval = encode_krb5_etype_info2(info_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_etype_info2(code, &mut tmp_22);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_etype_info(test_context, tmp_22);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_etype_info(info_0);
    /* ***************************************************************/
    /* encode_pa_enc_ts */
    let mut pa_enc: krb5_pa_enc_ts =
        krb5_pa_enc_ts{patimestamp: 0, pausec: 0,};
    let mut tmp_23: *mut krb5_pa_enc_ts = 0 as *mut krb5_pa_enc_ts;
    ktest_make_sample_pa_enc_ts(&mut pa_enc);
    retval = encode_krb5_pa_enc_ts(&mut pa_enc, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_pa_enc_ts(code, &mut tmp_23);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_enc_ts(test_context, tmp_23);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    pa_enc.pausec = 0 as libc::c_int;
    retval = encode_krb5_pa_enc_ts(&mut pa_enc, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_pa_enc_ts(code, &mut tmp_23);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_enc_ts(test_context, tmp_23);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    /* ***************************************************************/
    /* encode_enc_data */
    let mut enc_data: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut tmp_24: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    ktest_make_sample_enc_data(&mut enc_data);
    retval = encode_krb5_enc_data(&mut enc_data, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_data(code, &mut tmp_24);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_data(test_context, tmp_24);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_enc_data(&mut enc_data);
    /* ***************************************************************/
    /* encode_krb5_sam_challenge_2 */
    let mut sam_ch2: krb5_sam_challenge_2 =
        krb5_sam_challenge_2{sam_challenge_2_body:
                                 krb5_data{magic: 0,
                                           length: 0,
                                           data: 0 as *mut libc::c_char,},
                             sam_cksum: 0 as *mut *mut krb5_checksum,};
    let mut tmp_25: *mut krb5_sam_challenge_2 =
        0 as *mut krb5_sam_challenge_2;
    ktest_make_sample_sam_challenge_2(&mut sam_ch2);
    retval = encode_krb5_sam_challenge_2(&mut sam_ch2, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_sam_challenge_2(code, &mut tmp_25);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_sam_challenge_2(test_context, tmp_25);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_sam_challenge_2(&mut sam_ch2);
    /* ***************************************************************/
    /* encode_krb5_sam_challenge_2 */
    let mut body: krb5_sam_challenge_2_body =
        krb5_sam_challenge_2_body{magic: 0,
                                  sam_type: 0,
                                  sam_flags: 0,
                                  sam_type_name:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_track_id:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_challenge_label:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_challenge:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_response_prompt:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_pk_for_sad:
                                      krb5_data{magic: 0,
                                                length: 0,
                                                data:
                                                    0 as *mut libc::c_char,},
                                  sam_nonce: 0,
                                  sam_etype: 0,};
    let mut tmp_26: *mut krb5_sam_challenge_2_body =
        0 as *mut krb5_sam_challenge_2_body;
    ktest_make_sample_sam_challenge_2_body(&mut body);
    retval = encode_krb5_sam_challenge_2_body(&mut body, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_sam_challenge_2_body(code, &mut tmp_26);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_sam_challenge_2_body(test_context, tmp_26);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_sam_challenge_2_body(&mut body);
    /* ***************************************************************/
    /* encode_krb5_sam_response_2 */
    let mut sam_ch2_0: krb5_sam_response_2 =
        krb5_sam_response_2{magic: 0,
                            sam_type: 0,
                            sam_flags: 0,
                            sam_track_id:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            sam_enc_nonce_or_sad:
                                krb5_enc_data{magic: 0,
                                              enctype: 0,
                                              kvno: 0,
                                              ciphertext:
                                                  krb5_data{magic: 0,
                                                            length: 0,
                                                            data:
                                                                0 as
                                                                    *mut libc::c_char,},},
                            sam_nonce: 0,};
    let mut tmp_27: *mut krb5_sam_response_2 = 0 as *mut krb5_sam_response_2;
    ktest_make_sample_sam_response_2(&mut sam_ch2_0);
    retval = encode_krb5_sam_response_2(&mut sam_ch2_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_sam_response_2(code, &mut tmp_27);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_sam_response_2(test_context, tmp_27);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_sam_response_2(&mut sam_ch2_0);
    /* ***************************************************************/
    /* encode_krb5_sam_response_enc_2 */
    let mut sam_ch2_1: krb5_enc_sam_response_enc_2 =
        krb5_enc_sam_response_enc_2{magic: 0,
                                    sam_nonce: 0,
                                    sam_sad:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},};
    let mut tmp_28: *mut krb5_enc_sam_response_enc_2 =
        0 as *mut krb5_enc_sam_response_enc_2;
    ktest_make_sample_enc_sam_response_enc_2(&mut sam_ch2_1);
    retval = encode_krb5_enc_sam_response_enc_2(&mut sam_ch2_1, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_enc_sam_response_enc_2(code, &mut tmp_28);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_sam_response_enc_2(test_context, tmp_28);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_enc_sam_response_enc_2(&mut sam_ch2_1);
    /* ***************************************************************/
    /* encode_krb5_pa_for_user */
    let mut foru: krb5_pa_for_user =
        krb5_pa_for_user{user: 0 as *mut krb5_principal_data,
                         cksum:
                             krb5_checksum{magic: 0,
                                           checksum_type: 0,
                                           length: 0,
                                           contents: 0 as *mut krb5_octet,},
                         auth_package:
                             krb5_data{magic: 0,
                                       length: 0,
                                       data: 0 as *mut libc::c_char,},};
    let mut tmp_29: *mut krb5_pa_for_user = 0 as *mut krb5_pa_for_user;
    ktest_make_sample_pa_for_user(&mut foru);
    retval = encode_krb5_pa_for_user(&mut foru, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_pa_for_user(code, &mut tmp_29);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_for_user(test_context, tmp_29);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_pa_for_user(&mut foru);
    /* ***************************************************************/
    /* encode_krb5_pa_s4u_x509_user */
    let mut s4u: krb5_pa_s4u_x509_user =
        krb5_pa_s4u_x509_user{user_id:
                                  krb5_s4u_userid{nonce: 0,
                                                  user:
                                                      0 as
                                                          *mut krb5_principal_data,
                                                  subject_cert:
                                                      krb5_data{magic: 0,
                                                                length: 0,
                                                                data:
                                                                    0 as
                                                                        *mut libc::c_char,},
                                                  options: 0,},
                              cksum:
                                  krb5_checksum{magic: 0,
                                                checksum_type: 0,
                                                length: 0,
                                                contents:
                                                    0 as *mut krb5_octet,},};
    let mut tmp_30: *mut krb5_pa_s4u_x509_user =
        0 as *mut krb5_pa_s4u_x509_user;
    ktest_make_sample_pa_s4u_x509_user(&mut s4u);
    retval = encode_krb5_pa_s4u_x509_user(&mut s4u, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_pa_s4u_x509_user(code, &mut tmp_30);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_pa_s4u_x509_user(test_context, tmp_30);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_pa_s4u_x509_user(&mut s4u);
    /* ***************************************************************/
    /* encode_krb5_ad_kdcissued */
    let mut kdci: krb5_ad_kdcissued =
        krb5_ad_kdcissued{ad_checksum:
                              krb5_checksum{magic: 0,
                                            checksum_type: 0,
                                            length: 0,
                                            contents: 0 as *mut krb5_octet,},
                          i_principal: 0 as *mut krb5_principal_data,
                          elements: 0 as *mut *mut krb5_authdata,};
    let mut tmp_31: *mut krb5_ad_kdcissued = 0 as *mut krb5_ad_kdcissued;
    ktest_make_sample_ad_kdcissued(&mut kdci);
    retval = encode_krb5_ad_kdcissued(&mut kdci, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ad_kdcissued(code, &mut tmp_31);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ad_kdcissued(test_context, tmp_31);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ad_kdcissued(&mut kdci);
    /* ***************************************************************/
    /* encode_krb5_ad_signedpath */
    let mut sp: krb5_ad_signedpath =
        krb5_ad_signedpath{enctype: 0,
                           checksum:
                               krb5_checksum{magic: 0,
                                             checksum_type: 0,
                                             length: 0,
                                             contents: 0 as *mut krb5_octet,},
                           delegated: 0 as *mut krb5_principal,
                           method_data: 0 as *mut *mut krb5_pa_data,};
    let mut tmp_32: *mut krb5_ad_signedpath = 0 as *mut krb5_ad_signedpath;
    ktest_make_sample_ad_signedpath(&mut sp);
    retval = encode_krb5_ad_signedpath(&mut sp, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_ad_signedpath(code, &mut tmp_32);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_ad_signedpath(test_context, tmp_32);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_ad_signedpath(&mut sp);
    /* ***************************************************************/
    /* encode_krb5_iakerb_header */
    let mut ih: krb5_iakerb_header =
        krb5_iakerb_header{target_realm:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           cookie: 0 as *mut krb5_data,};
    let mut tmp_33: *mut krb5_iakerb_header = 0 as *mut krb5_iakerb_header;
    ktest_make_sample_iakerb_header(&mut ih);
    retval = encode_krb5_iakerb_header(&mut ih, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_iakerb_header(code, &mut tmp_33);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_iakerb_header(test_context, tmp_33);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_iakerb_header(&mut ih);
    /* ***************************************************************/
    /* encode_krb5_iakerb_finished */
    let mut ih_0: krb5_iakerb_finished =
        krb5_iakerb_finished{checksum:
                                 krb5_checksum{magic: 0,
                                               checksum_type: 0,
                                               length: 0,
                                               contents:
                                                   0 as *mut krb5_octet,},};
    let mut tmp_34: *mut krb5_iakerb_finished =
        0 as *mut krb5_iakerb_finished;
    ktest_make_sample_iakerb_finished(&mut ih_0);
    retval = encode_krb5_iakerb_finished(&mut ih_0, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_iakerb_finished(code, &mut tmp_34);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_iakerb_finished(test_context, tmp_34);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_iakerb_finished(&mut ih_0);
    /* ***************************************************************/
    /* encode_krb5_fast_response */
    let mut fr: krb5_fast_response =
        krb5_fast_response{magic: 0,
                           padata: 0 as *mut *mut krb5_pa_data,
                           strengthen_key: 0 as *mut krb5_keyblock,
                           finished: 0 as *mut krb5_fast_finished,
                           nonce: 0,};
    let mut tmp_35: *mut krb5_fast_response = 0 as *mut krb5_fast_response;
    ktest_make_sample_fast_response(&mut fr);
    retval = encode_krb5_fast_response(&mut fr, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_fast_response(code, &mut tmp_35);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_fast_response(test_context, tmp_35);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_empty_fast_response(&mut fr);
    /* ***************************************************************/
    /* encode_krb5_pa_fx_fast_reply */
    let mut enc: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut tmp_36: *mut krb5_enc_data = 0 as *mut krb5_enc_data;
    ktest_make_sample_enc_data(&mut enc);
    retval = encode_krb5_pa_fx_fast_reply(&mut enc, &mut code);
    if retval != 0 {
        com_err(b"krb5_decode_leak\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                b"while encoding\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*code).length {
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        retval = decode_krb5_pa_fx_fast_reply(code, &mut tmp_36);
        *(*code).data.offset(i as isize) =
            !(*(*code).data.offset(i as isize) as libc::c_uchar as
                  libc::c_int) as libc::c_char;
        if retval == 0 as libc::c_int {
            krb5_free_enc_data(test_context, tmp_36);
        }
        i = i.wrapping_add(1)
    }
    krb5_free_data(test_context, code);
    ktest_destroy_enc_data(&mut enc);
    krb5_free_context(test_context);
    return 0 as libc::c_int;
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
