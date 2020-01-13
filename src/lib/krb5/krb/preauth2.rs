use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
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
    /* * Store options for @c _krb5_get_init_creds */
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
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
    /* * Pointer to a prompter callback function. */
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
    /* The name of the Kerberos ticket granting service... and its size */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
    }
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
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
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
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
    /* * Credentials structure including ticket, session key, and lifetime info. */
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
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
    /* * Last request entry */
    #[c2rust::src_loc = "2031:1"]
    pub type krb5_last_req_entry = _krb5_last_req_entry;
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
    /* *< LR type */
    /* *< Timestamp */
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
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* *
 * OTP responder question
 *
 * The OTP responder question is asked when the KDC indicates that an OTP
 * value is required in order to complete the authentication.  The JSON format
 * of the challenge is:
 *
 *  @n {
 *  @n   "service": <string (optional)>,
 *  @n   "tokenInfo": [
 *  @n      {
 *  @n        "flags":     <number>,
 *  @n        "vendor":    <string (optional)>,
 *  @n        "challenge": <string (optional)>,
 *  @n        "length":    <number (optional)>,
 *  @n        "format":    <number (optional)>,
 *  @n        "tokenID":   <string (optional)>,
 *  @n        "algID":     <string (optional)>,
 *  @n      },
 *  @n      ...
 *  @n    ]
 *  @n  }
 *
 * The answer to the question MUST be JSON formatted:
 *
 * @n  {
 * @n    "tokeninfo": <number>,
 * @n    "value":     <string (optional)>,
 * @n    "pin":       <string (optional)>,
 * @n  }
 *
 * For more detail, please see RFC 6560.
 *
 * @version New in 1.11
 */
    /* *
 * These format constants identify the format of the token value.
 */
    /* *
 * This flag indicates that the token value MUST be collected.
 */
    /* *
 * This flag indicates that the PIN value MUST be collected.
 */
    /* *
 * This flag indicates that the token is now in re-synchronization mode with
 * the server.  The user is expected to reply with the next code displayed on
 * the token.
 */
    /* *
 * This flag indicates that the PIN MUST be returned as a separate item. This
 * flag only takes effect if KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN is set. If
 * this flag is not set, the responder may either concatenate PIN + token value
 * and store it as "value" in the answer or it may return them separately. If
 * they are returned separately, they will be concatenated internally.
 */
    /* *
 * PKINIT responder question
 *
 * The PKINIT responder question is asked when the client needs a password
 * that's being used to protect key information, and is formatted as a JSON
 * object.  A specific identity's flags value, if not zero, is the bitwise-OR
 * of one or more of the KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_* flags defined
 * below, and possibly other flags to be added later.  Any resemblance to
 * similarly-named CKF_* values in the PKCS#11 API should not be depended on.
 *
 *  @n {
 *  @n     identity <string> : flags <number>,
 *  @n     ...
 *  @n }
 *
 * The answer to the question MUST be JSON formatted:
 *
 *  @n {
 *  @n     identity <string> : password <string>,
 *  @n     ...
 *  @n }
 *
 * @version New in 1.12
 */
    /* *
 * This flag indicates that an incorrect PIN was supplied at least once since
 * the last time the correct PIN was supplied.
 */
    /* *
 * This flag indicates that supplying an incorrect PIN will cause the token to
 * lock itself.
 */
    /* *
 * This flag indicates that the user PIN is locked, and you can't log in to the
 * token with it.
 */
    /* *
 * A container for a set of preauthentication questions and answers
 *
 * A responder context is supplied by the krb5 authentication system to a @ref
 * krb5_responder_fn callback.  It contains a list of questions and can receive
 * answers.  Questions contained in a responder context can be listed using
 * krb5_responder_list_questions(), retrieved using
 * krb5_responder_get_challenge(), or answered using
 * krb5_responder_set_answer().  The form of a question's challenge and
 * answer depend on the question name.
 *
 * @version New in 1.11
 */
    #[c2rust::src_loc = "6603:1"]
    pub type krb5_responder_context = *mut krb5_responder_context_st;
    /* *
 * Responder function for an initial credential exchange.
 *
 * @param [in] ctx              Library context
 * @param [in] data             Callback data
 * @param [in] rctx             Responder context
 *
 * A responder function is like a prompter function, but is used for handling
 * questions and answers as potentially complex data types.  Client
 * preauthentication modules will insert a set of named "questions" into
 * the responder context.  Each question may optionally contain a challenge.
 * This challenge is printable UTF-8, but may be an encoded value.  The
 * precise encoding and contents of the challenge are specific to the question
 * asked.  When the responder is called, it should answer all the questions it
 * understands.  Like the challenge, the answer MUST be printable UTF-8, but
 * may contain structured/encoded data formatted to the expected answer format
 * of the question.
 *
 * If a required question is unanswered, the prompter may be called.
 */
    #[c2rust::src_loc = "6681:1"]
    pub type krb5_responder_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_responder_context)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "7313:1"]
    pub type krb5_init_creds_context = *mut _krb5_init_creds_context;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::init_creds_ctx_h::{krb5_responder_context_st,
                                  _krb5_init_creds_context};
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
        /* * @defgroup KRB5_KEYUSAGE KRB5_KEYUSAGE
 * @{
 */
        /* XXX need to register these */
        /* Defined in Integrating SAM Mechanisms with Kerberos draft */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY */
        /* Defined in [MS-SFU] */
/* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_RESPONSE */
        /* unused */
        /* *< See RFC 6560 section 4.2 */
        /* define in draft-ietf-krb-wg-preauth-framework*/
        /* Key usage values 512-1023 are reserved for uses internal to a Kerberos
 * implementation. */
        /* *< Used for encrypted FAST cookies */
        /* *< Used for freshness tokens */
        /* * @} */
        /* end of KRB5_KEYUSAGE group */
        /* *
 * Verify that a specified encryption type is a valid Kerberos encryption type.
 *
 * @param [in] ktype            Encryption type
 *
 * @return @c TRUE if @a ktype is valid, @c FALSE if not
 */
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
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
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
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
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "7966:1"]
        pub fn krb5_wrap_error_message(ctx: krb5_context,
                                       old_code: krb5_error_code,
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
    #[c2rust::src_loc = "436:1"]
    pub type krb5_etype_info = *mut *mut krb5_etype_info_entry;
    #[c2rust::src_loc = "776:1"]
    pub type krb5_fast_armor = _krb5_fast_armor;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "776:16"]
    pub struct _krb5_fast_armor {
        pub armor_type: krb5_int32,
        pub armor_value: krb5_data,
    }
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
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
                        krb5_octet, krb5_data, krb5_error_code, krb5_context,
                        krb5_pa_data, krb5_preauthtype};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::krb5_preauth_context_st;
    use super::stddef_h::size_t;
    use super::string_h::memcpy;
    use super::stdlib_h::calloc;
    use super::plugin_h::krb5_plugin_initvt_fn;
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
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
        #[no_mangle]
        #[c2rust::src_loc = "863:1"]
        pub fn krb5_free_etype_info(_: krb5_context, _: krb5_etype_info);
        #[no_mangle]
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "898:1"]
        pub fn krb5int_copy_data_contents(_: krb5_context,
                                          _: *const krb5_data,
                                          _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1168:1"]
        pub fn k5_plugin_load_all(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modules: *mut *mut krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1173:1"]
        pub fn k5_plugin_free_modules(context: krb5_context,
                                      modules: *mut krb5_plugin_initvt_fn);
        #[no_mangle]
        #[c2rust::src_loc = "1177:1"]
        pub fn k5_plugin_register(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modname: *const libc::c_char,
                                  module: krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1186:1"]
        pub fn k5_plugin_register_dyn(context: krb5_context,
                                      interface_id: libc::c_int,
                                      modname: *const libc::c_char,
                                      modsubdir: *const libc::c_char)
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
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn k5_clear_error(ep: *mut errinfo);
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:36"]
pub mod clpreauth_plugin_h {
    /* Abstract types for module data and per-request module data. */
    #[c2rust::src_loc = "78:1"]
    pub type krb5_clpreauth_moddata = *mut krb5_clpreauth_moddata_st;
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
    /* Doesn't provide a real answer, but must be given a chance to run before any
 * REAL mechanism callbacks. */
    /* Abstract type for a client request information handle. */
    #[c2rust::src_loc = "75:1"]
    pub type krb5_clpreauth_rock = *mut krb5_clpreauth_rock_st;
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    #[c2rust::src_loc = "83:1"]
    pub type krb5_clpreauth_callbacks = *mut krb5_clpreauth_callbacks_st;
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
    #[c2rust::src_loc = "79:1"]
    pub type krb5_clpreauth_modreq = *mut krb5_clpreauth_modreq_st;
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
    #[c2rust::src_loc = "214:1"]
    pub type krb5_clpreauth_request_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq) -> ()>;
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
    #[c2rust::src_loc = "189:1"]
    pub type krb5_clpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata) -> ()>;
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
    use super::krb5_h::{krb5_preauthtype, krb5_enctype, krb5_error_code,
                        krb5_context, krb5_get_init_creds_opt, krb5_kdc_req,
                        krb5_data, krb5_pa_data, krb5_keyblock, krb5_boolean,
                        krb5_timestamp, krb5_int32, krb5_error,
                        krb5_prompter_fct};
    extern "C" {
        #[c2rust::src_loc = "78:16"]
        pub type krb5_clpreauth_moddata_st;
        #[c2rust::src_loc = "75:16"]
        pub type krb5_clpreauth_rock_st;
        #[c2rust::src_loc = "79:16"]
        pub type krb5_clpreauth_modreq_st;
    }
    /* KRB5_CLPREAUTH_PLUGIN_H */
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
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/init_creds_ctx.h:40"]
pub mod init_creds_ctx_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct krb5_responder_context_st {
        pub items: *mut k5_response_items,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[c2rust::src_loc = "9:1"]
    pub type krb5_preauth_req_context = *mut krb5_preauth_req_context_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "20:8"]
    pub struct _krb5_init_creds_context {
        pub opt: *mut krb5_get_init_creds_opt,
        pub opt_storage: krb5_get_init_creds_opt,
        pub identify_realm: krb5_boolean,
        pub subject_cert: *const krb5_data,
        pub in_tkt_service: *mut libc::c_char,
        pub prompter: krb5_prompter_fct,
        pub prompter_data: *mut libc::c_void,
        pub gak_fct: get_as_key_fn,
        pub gak_data: *mut libc::c_void,
        pub request_time: krb5_timestamp,
        pub start_time: krb5_deltat,
        pub tkt_life: krb5_deltat,
        pub renew_life: krb5_deltat,
        pub complete: krb5_boolean,
        pub loopcount: libc::c_uint,
        pub gakpw: gak_password,
        pub err_reply: *mut krb5_error,
        pub err_padata: *mut *mut krb5_pa_data,
        pub cred: krb5_creds,
        pub request: *mut krb5_kdc_req,
        pub reply: *mut krb5_kdc_rep,
        pub outer_request_body: *mut krb5_data,
        pub inner_request_body: *mut krb5_data,
        pub encoded_previous_request: *mut krb5_data,
        pub fast_state: *mut krb5int_fast_request_state,
        pub optimistic_padata: *mut *mut krb5_pa_data,
        pub method_padata: *mut *mut krb5_pa_data,
        pub more_padata: *mut *mut krb5_pa_data,
        pub default_salt: krb5_boolean,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
        pub as_key: krb5_keyblock,
        pub etype: krb5_enctype,
        pub info_pa_permitted: krb5_boolean,
        pub restarted: krb5_boolean,
        pub fallback_disabled: krb5_boolean,
        pub encts_disabled: krb5_boolean,
        pub rctx: krb5_responder_context_st,
        pub selected_preauth_type: krb5_preauthtype,
        pub allowed_preauth_type: krb5_preauthtype,
        pub cc_config_in: k5_json_object,
        pub cc_config_out: k5_json_object,
        pub pa_offset: krb5_timestamp,
        pub pa_offset_usec: krb5_int32,
        pub pa_offset_state: C2RustUnnamed_0,
        pub preauth_reqctx: krb5_preauth_req_context,
    }
    #[c2rust::src_loc = "75:5"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "75:42"]
    pub const AUTH_OFFSET: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "75:27"]
    pub const UNAUTH_OFFSET: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "75:12"]
    pub const NO_OFFSET: C2RustUnnamed_0 = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "15:8"]
    pub struct gak_password {
        pub storage: krb5_data,
        pub password: *const krb5_data,
    }
    use super::int_proto_h::{k5_response_items, get_as_key_fn};
    use super::krb5_preauth_req_context_st;
    use super::krb5_h::{krb5_get_init_creds_opt, krb5_boolean, krb5_data,
                        krb5_prompter_fct, krb5_timestamp, krb5_deltat,
                        krb5_error, krb5_pa_data, krb5_creds, krb5_kdc_req,
                        krb5_kdc_rep, krb5_keyblock, krb5_enctype,
                        krb5_preauthtype, krb5_int32};
    use super::fast_h::krb5int_fast_request_state;
    use super::k5_json_h::k5_json_object;
    /* !KRB5_INIT_CREDS_CONTEXT */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:37"]
pub mod int_proto_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/int-proto.h - Prototypes for libkrb5 internal functions */
/*
 * Copyright 1990,1991 the Massachusetts Institute of Technology.
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
    #[c2rust::src_loc = "32:1"]
    pub type k5_response_items = k5_response_items_st;
    #[c2rust::src_loc = "34:1"]
    pub type get_as_key_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_principal,
                                    _: krb5_enctype, _: krb5_prompter_fct,
                                    _: *mut libc::c_void, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut krb5_keyblock,
                                    _: *mut libc::c_void,
                                    _: *mut k5_response_items)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_principal,
                        krb5_enctype, krb5_prompter_fct, krb5_data,
                        krb5_keyblock, krb5_init_creds_context, krb5_boolean,
                        krb5_timestamp, krb5_int32, krb5_get_init_creds_opt,
                        krb5_responder_fn};
    use super::k5_int_h::_krb5_context;
    use super::plugin_h::{krb5_plugin_vtable_st, krb5_plugin_vtable};
    use super::init_creds_ctx_h::_krb5_init_creds_context;
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "32:16"]
        pub type k5_response_items_st;
        #[no_mangle]
        #[c2rust::src_loc = "65:1"]
        pub fn clpreauth_encrypted_challenge_initvt(context: krb5_context,
                                                    maj_ver: libc::c_int,
                                                    min_ver: libc::c_int,
                                                    vtable:
                                                        krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn clpreauth_encrypted_timestamp_initvt(context: krb5_context,
                                                    maj_ver: libc::c_int,
                                                    min_ver: libc::c_int,
                                                    vtable:
                                                        krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn clpreauth_sam2_initvt(context: krb5_context,
                                     maj_ver: libc::c_int,
                                     min_ver: libc::c_int,
                                     vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn clpreauth_otp_initvt(context: krb5_context,
                                    maj_ver: libc::c_int,
                                    min_ver: libc::c_int,
                                    vtable: krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn k5_init_creds_current_time(context: krb5_context,
                                          ctx: krb5_init_creds_context,
                                          allow_unauth: krb5_boolean,
                                          time_out: *mut krb5_timestamp,
                                          usec_out: *mut krb5_int32)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn k5_response_items_reset(ri: *mut k5_response_items);
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn k5_response_items_empty(ri: *const k5_response_items)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "257:1"]
        pub fn k5_response_items_list_questions(ri: *const k5_response_items)
         -> *const *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn k5_response_items_ask_question(ri: *mut k5_response_items,
                                              question: *const libc::c_char,
                                              challenge: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "264:1"]
        pub fn k5_response_items_get_challenge(ri: *const k5_response_items,
                                               question: *const libc::c_char)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "268:1"]
        pub fn k5_response_items_set_answer(ri: *mut k5_response_items,
                                            question: *const libc::c_char,
                                            answer: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "272:1"]
        pub fn k5_response_items_get_answer(ri: *const k5_response_items,
                                            question: *const libc::c_char)
         -> *const libc::c_char;
        /* Save code and its extended message (if any) in out. */
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn k5_save_ctx_error(ctx: krb5_context, code: krb5_error_code,
                                 out: *mut errinfo);
        /* Return the code from in and restore its extended message (if any). */
        #[no_mangle]
        #[c2rust::src_loc = "281:1"]
        pub fn k5_restore_ctx_error(ctx: krb5_context, in_0: *mut errinfo)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn k5_gic_opt_get_responder(opt: *mut krb5_get_init_creds_opt,
                                        responder_out: *mut krb5_responder_fn,
                                        data_out: *mut *mut libc::c_void);
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:34"]
pub mod k5_json_h {
    /*
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    /*
 * The k5_json_value C type can represent any kind of JSON value.  It has no
 * static type safety since it is represented using a void pointer, so be
 * careful with it.  Its type can be checked dynamically with k5_json_get_tid()
 * and the above constants.
 */
    #[c2rust::src_loc = "86:1"]
    pub type k5_json_value = *mut libc::c_void;
    #[c2rust::src_loc = "87:1"]
    pub type k5_json_tid = libc::c_uint;
    /*
 * String
 */
    #[c2rust::src_loc = "186:1"]
    pub type k5_json_string = *mut k5_json_string_st;
    extern "C" {
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
        #[c2rust::src_loc = "186:16"]
        pub type k5_json_string_st;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn k5_json_get_tid(val: k5_json_value) -> k5_json_tid;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_json_release(val: k5_json_value);
        /*
 * Store val into object at key, incrementing val's reference count and
 * releasing any previous value at key.  If val is NULL, key is removed from
 * obj if it exists, and obj remains unchanged if it does not.
 */
        #[no_mangle]
        #[c2rust::src_loc = "176:1"]
        pub fn k5_json_object_set(obj: k5_json_object,
                                  key: *const libc::c_char,
                                  val: k5_json_value) -> libc::c_int;
        /* Get an alias to the object's value for key, without incrementing the
 * reference count.  Returns NULL if there is no value for key. */
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn k5_json_object_get(obj: k5_json_object,
                                  key: *const libc::c_char) -> k5_json_value;
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn k5_json_string_create(cstring: *const libc::c_char,
                                     val_out: *mut k5_json_string)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn k5_json_string_utf8(string: k5_json_string)
         -> *const libc::c_char;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:33"]
pub mod plugin_h {
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
    /*
 * krb5_plugin_initvt_fn is the type of all module initvt functions.  Based on
 * the maj_ver argument, the initvt function should cast vtable to the
 * appropriate type and then fill it in.  If a vtable has been expanded,
 * min_ver indicates which version of the vtable is being filled in.
 */
    #[c2rust::src_loc = "42:1"]
    pub type krb5_plugin_initvt_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int,
                                    _: libc::c_int, _: krb5_plugin_vtable)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    use super::krb5_h::{krb5_error_code, krb5_context};
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/fast.h:39"]
pub mod fast_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/fast.h */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:8"]
    pub struct krb5int_fast_request_state {
        pub fast_outer_request: krb5_kdc_req,
        pub armor_key: *mut krb5_keyblock,
        pub armor: *mut krb5_fast_armor,
        pub fast_state_flags: krb5_ui_4,
        pub fast_options: krb5_ui_4,
        pub nonce: krb5_int32,
    }
    use super::krb5_h::{krb5_kdc_req, krb5_keyblock, krb5_ui_4, krb5_int32};
    use super::k5_int_h::krb5_fast_armor;
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
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
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
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_pa_data, _krb5_pa_data, krb5_kdc_req,
                       _krb5_kdc_req, krb5_ticket, _krb5_ticket,
                       krb5_enc_tkt_part, _krb5_enc_tkt_part, krb5_authdata,
                       _krb5_authdata, krb5_ticket_times, _krb5_ticket_times,
                       krb5_transited, _krb5_transited, krb5_keyblock,
                       _krb5_keyblock, krb5_enc_data, _krb5_enc_data,
                       krb5_get_init_creds_opt, _krb5_get_init_creds_opt,
                       krb5_prompter_fct, krb5_prompt, _krb5_prompt,
                       krb5_error, _krb5_error, krb5_prompt_type, _krb5_creds,
                       krb5_creds, _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_enc_kdc_rep_part, krb5_enc_kdc_rep_part,
                       _krb5_kdc_rep, krb5_kdc_rep, krb5_responder_context,
                       krb5_responder_fn, krb5_init_creds_context, _profile_t,
                       krb5_c_valid_enctype, krb5_copy_keyblock_contents,
                       krb5_free_keyblock_contents, krb5_free_data_contents,
                       krb5_set_error_message, krb5_prepend_error_message,
                       krb5_wrap_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_etype_info_entry,
                         _krb5_etype_info_entry, krb5_etype_info,
                         krb5_fast_armor, _krb5_fast_armor, k5memdup, k5alloc,
                         k5calloc, alloc_data, empty_data, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, _kdb5_dal_handle,
                         krb5_free_etype_info, krb5int_find_pa_data,
                         krb5int_copy_data_contents, k5_plugin_load_all,
                         k5_plugin_free_modules, k5_plugin_register,
                         k5_plugin_register_dyn, decode_krb5_etype_info,
                         decode_krb5_etype_info2, krb5_free_pa_data};
pub use self::k5_err_h::{errinfo, k5_clear_error};
pub use self::clpreauth_plugin_h::{krb5_clpreauth_moddata,
                                   krb5_clpreauth_vtable_st,
                                   krb5_clpreauth_prep_questions_fn,
                                   krb5_clpreauth_rock,
                                   krb5_clpreauth_callbacks,
                                   krb5_clpreauth_callbacks_st,
                                   krb5_clpreauth_modreq,
                                   krb5_clpreauth_supply_gic_opts_fn,
                                   krb5_clpreauth_tryagain_fn,
                                   krb5_clpreauth_process_fn,
                                   krb5_clpreauth_request_fini_fn,
                                   krb5_clpreauth_request_init_fn,
                                   krb5_clpreauth_get_flags_fn,
                                   krb5_clpreauth_fini_fn,
                                   krb5_clpreauth_init_fn,
                                   krb5_clpreauth_moddata_st,
                                   krb5_clpreauth_rock_st,
                                   krb5_clpreauth_modreq_st};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::init_creds_ctx_h::{krb5_responder_context_st,
                                 krb5_preauth_req_context,
                                 _krb5_init_creds_context, C2RustUnnamed_0,
                                 AUTH_OFFSET, UNAUTH_OFFSET, NO_OFFSET,
                                 gak_password};
pub use self::int_proto_h::{k5_response_items, get_as_key_fn,
                            k5_response_items_st,
                            clpreauth_encrypted_challenge_initvt,
                            clpreauth_encrypted_timestamp_initvt,
                            clpreauth_sam2_initvt, clpreauth_otp_initvt,
                            k5_init_creds_current_time,
                            k5_response_items_reset, k5_response_items_empty,
                            k5_response_items_list_questions,
                            k5_response_items_ask_question,
                            k5_response_items_get_challenge,
                            k5_response_items_set_answer,
                            k5_response_items_get_answer, k5_save_ctx_error,
                            k5_restore_ctx_error, k5_gic_opt_get_responder};
pub use self::k5_json_h::{k5_json_object, k5_json_value, k5_json_tid,
                          k5_json_string, k5_json_object_st,
                          k5_json_string_st, k5_json_get_tid, k5_json_release,
                          k5_json_object_set, k5_json_object_get,
                          k5_json_string_create, k5_json_string_utf8};
pub use self::plugin_h::{krb5_plugin_initvt_fn, krb5_plugin_vtable,
                         krb5_plugin_vtable_st};
pub use self::fast_h::krb5int_fast_request_state;
use self::libintl_h::dgettext;
use self::string_h::{strcmp, memchr, memcpy};
use self::stdlib_h::{free, realloc, calloc, malloc};
use self::k5_trace_h::krb5int_trace;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "51:8"]
pub struct krb5_preauth_context_st {
    pub handles: *mut clpreauth_handle,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1995, 2003, 2008, 2012 by the Massachusetts Institute of Technology.  All
 * Rights Reserved.
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
 *
 */
/*
 * This file contains routines for establishing, verifying, and any other
 * necessary functions, for utilizing the pre-authentication field of the
 * kerberos kdc request, with various hardware/software verification devices.
 */
#[c2rust::src_loc = "46:1"]
pub type clpreauth_handle = *mut C2RustUnnamed;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "46:9"]
pub struct C2RustUnnamed {
    pub vt: krb5_clpreauth_vtable_st,
    pub data: krb5_clpreauth_moddata,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "55:8"]
pub struct krb5_preauth_req_context_st {
    pub orig_context: krb5_context,
    pub failed: *mut krb5_preauthtype,
    pub modreqs: *mut krb5_clpreauth_modreq,
}
/* Release the memory used by a list of handles. */
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn free_handles(mut context: krb5_context,
                                  mut handles: *mut clpreauth_handle) {
    let mut hp: *mut clpreauth_handle = 0 as *mut clpreauth_handle;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    if handles.is_null() { return }
    hp = handles;
    while !(*hp).is_null() {
        h = *hp;
        if (*h).vt.fini.is_some() {
            (*h).vt.fini.expect("non-null function pointer")(context,
                                                             (*h).data);
        }
        free(h as *mut libc::c_void);
        hp = hp.offset(1)
    }
    free(handles as *mut libc::c_void);
}
/* Return an index into handles which can process pa_type, or -1 if none is
 * found found. */
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn search_module_list(mut handles: *mut clpreauth_handle,
                                        mut pa_type: krb5_preauthtype)
 -> libc::c_int {
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*handles.offset(i as isize)).is_null() {
        h = *handles.offset(i as isize);
        j = 0 as libc::c_int;
        while *(*h).vt.pa_type_list.offset(j as isize) != 0 as libc::c_int {
            if *(*h).vt.pa_type_list.offset(j as isize) == pa_type {
                return i
            }
            j += 1
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
/* Find the handle which can process pa_type, or NULL if none is found.  On
 * success, set *modreq_out to the corresponding per-request module data. */
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn find_module(mut context: krb5_context,
                                 mut ctx: krb5_init_creds_context,
                                 mut pa_type: krb5_preauthtype,
                                 mut modreq_out: *mut krb5_clpreauth_modreq)
 -> clpreauth_handle {
    let mut pctx: krb5_preauth_context = (*context).preauth_context;
    let mut reqctx: krb5_preauth_req_context = (*ctx).preauth_reqctx;
    let mut i: libc::c_int = 0;
    *modreq_out = 0 as krb5_clpreauth_modreq;
    if pctx.is_null() || reqctx.is_null() { return 0 as clpreauth_handle }
    i = search_module_list((*pctx).handles, pa_type);
    if i == -(1 as libc::c_int) { return 0 as clpreauth_handle }
    *modreq_out = *(*reqctx).modreqs.offset(i as isize);
    return *(*pctx).handles.offset(i as isize);
}
/* Initialize the preauth state for a krb5 context. */
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn k5_init_preauth_context(mut context: krb5_context) {
    let mut current_block: u64;
    let mut modules: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut mod_0: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut list: *mut clpreauth_handle = 0 as *mut clpreauth_handle;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut i: libc::c_int = 0;
    let mut count: size_t = 0;
    let mut tp: *mut krb5_preauthtype = 0 as *mut krb5_preauthtype;
    /* Only do this once for each krb5_context */
    if !(*context).preauth_context.is_null() { return }
    /* Auto-register built-in modules. */
    k5_plugin_register_dyn(context, 2 as libc::c_int,
                           b"pkinit\x00" as *const u8 as *const libc::c_char,
                           b"preauth\x00" as *const u8 as
                               *const libc::c_char);
    k5_plugin_register_dyn(context, 2 as libc::c_int,
                           b"spake\x00" as *const u8 as *const libc::c_char,
                           b"preauth\x00" as *const u8 as
                               *const libc::c_char);
    k5_plugin_register(context, 2 as libc::c_int,
                       b"encrypted_challenge\x00" as *const u8 as
                           *const libc::c_char,
                       Some(clpreauth_encrypted_challenge_initvt as
                                unsafe extern "C" fn(_: krb5_context,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: krb5_plugin_vtable)
                                    -> krb5_error_code));
    k5_plugin_register(context, 2 as libc::c_int,
                       b"encrypted_timestamp\x00" as *const u8 as
                           *const libc::c_char,
                       Some(clpreauth_encrypted_timestamp_initvt as
                                unsafe extern "C" fn(_: krb5_context,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: krb5_plugin_vtable)
                                    -> krb5_error_code));
    k5_plugin_register(context, 2 as libc::c_int,
                       b"sam2\x00" as *const u8 as *const libc::c_char,
                       Some(clpreauth_sam2_initvt as
                                unsafe extern "C" fn(_: krb5_context,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: krb5_plugin_vtable)
                                    -> krb5_error_code));
    k5_plugin_register(context, 2 as libc::c_int,
                       b"otp\x00" as *const u8 as *const libc::c_char,
                       Some(clpreauth_otp_initvt as
                                unsafe extern "C" fn(_: krb5_context,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: krb5_plugin_vtable)
                                    -> krb5_error_code));
    /* Get all available clpreauth vtables. */
    if k5_plugin_load_all(context, 2 as libc::c_int, &mut modules) != 0 {
        return
    }
    /* Allocate a large enough list of handles. */
    count = 0 as libc::c_int as size_t;
    while (*modules.offset(count as isize)).is_some() {
        count = count.wrapping_add(1)
    }
    list =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<clpreauth_handle>() as libc::c_ulong) as
            *mut clpreauth_handle;
    if !list.is_null() {
        /* Create a handle for each module we can successfully initialize. */
        count = 0 as libc::c_int as size_t;
        mod_0 = modules;
        loop  {
            if !(*mod_0).is_some() {
                current_block = 11743904203796629665;
                break ;
            }
            h =
                calloc(1 as libc::c_int as libc::c_ulong,
                       ::std::mem::size_of::<C2RustUnnamed>() as
                           libc::c_ulong) as clpreauth_handle;
            if h.is_null() { current_block = 8556093922708094841; break ; }
            /* Initialize the handle vtable. */
            if (*mod_0).expect("non-null function pointer")(context,
                                                            1 as libc::c_int,
                                                            1 as libc::c_int,
                                                            &mut (*h).vt as
                                                                *mut krb5_clpreauth_vtable_st
                                                                as
                                                                krb5_plugin_vtable)
                   != 0 as libc::c_int {
                free(h as *mut libc::c_void);
            } else {
                /* Check for a preauth type conflict with an existing module. */
                tp = (*h).vt.pa_type_list;
                while *tp != 0 as libc::c_int {
                    i = search_module_list(list, *tp);
                    if i != -(1 as libc::c_int) {
                        if (*context).trace_callback.is_some() {
                            krb5int_trace(context,
                                          b"Preauth module {str} conflicts with module {str} for pa type {patype}\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          (*h).vt.name,
                                          (**list.offset(i as isize)).vt.name,
                                          *tp);
                        }
                        break ;
                    } else { tp = tp.offset(1) }
                }
                if !(*tp != 0 as libc::c_int) {
                    /* Initialize the module data. */
                    (*h).data = 0 as krb5_clpreauth_moddata;
                    if (*h).vt.init.is_some() &&
                           (*h).vt.init.expect("non-null function pointer")(context,
                                                                            &mut (*h).data)
                               != 0 as libc::c_int {
                        free(h as *mut libc::c_void);
                    } else {
                        let fresh0 = count;
                        count = count.wrapping_add(1);
                        let ref mut fresh1 = *list.offset(fresh0 as isize);
                        *fresh1 = h;
                        let ref mut fresh2 = *list.offset(count as isize);
                        *fresh2 = 0 as clpreauth_handle
                    }
                }
            }
            mod_0 = mod_0.offset(1)
        }
        match current_block {
            8556093922708094841 => { }
            _ => {
                let ref mut fresh3 = *list.offset(count as isize);
                *fresh3 = 0 as clpreauth_handle;
                /* Place the constructed preauth context into the krb5 context. */
                (*context).preauth_context =
                    malloc(::std::mem::size_of::<krb5_preauth_context_st>() as
                               libc::c_ulong) as krb5_preauth_context;
                if !(*context).preauth_context.is_null() {
                    (*(*context).preauth_context).handles = list;
                    list = 0 as *mut clpreauth_handle
                }
            }
        }
    }
    k5_plugin_free_modules(context, modules);
    free_handles(context, list);
}
/* Add pa_type to the list of types which has previously failed. */
#[no_mangle]
#[c2rust::src_loc = "207:1"]
pub unsafe extern "C" fn k5_preauth_note_failed(mut ctx:
                                                    krb5_init_creds_context,
                                                mut pa_type: krb5_preauthtype)
 -> krb5_error_code {
    let mut reqctx: krb5_preauth_req_context = (*ctx).preauth_reqctx;
    let mut newptr: *mut krb5_preauthtype = 0 as *mut krb5_preauthtype;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !(*reqctx).failed.is_null() &&
              *(*reqctx).failed.offset(i as isize) != 0 as libc::c_int {
        i = i.wrapping_add(1)
    }
    newptr =
        realloc((*reqctx).failed as *mut libc::c_void,
                i.wrapping_add(2 as libc::c_int as
                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_preauthtype>()
                                                                   as
                                                                   libc::c_ulong))
            as *mut krb5_preauthtype;
    if newptr.is_null() { return 12 as libc::c_int }
    (*reqctx).failed = newptr;
    *(*reqctx).failed.offset(i as isize) = pa_type;
    *(*reqctx).failed.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                 as isize) = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/* Free the per-krb5_context preauth_context. This means clearing any
 * plugin-specific context which may have been created, and then
 * freeing the context itself. */
#[no_mangle]
#[c2rust::src_loc = "227:1"]
pub unsafe extern "C" fn k5_free_preauth_context(mut context: krb5_context) {
    let mut pctx: krb5_preauth_context = (*context).preauth_context;
    if pctx.is_null() { return }
    free_handles(context, (*pctx).handles);
    free(pctx as *mut libc::c_void);
    (*context).preauth_context = 0 as krb5_preauth_context;
}
/* Initialize the per-AS-REQ context. This means calling the client_req_init
 * function to give the plugin a chance to allocate a per-request context. */
#[no_mangle]
#[c2rust::src_loc = "241:1"]
pub unsafe extern "C" fn k5_preauth_request_context_init(mut context:
                                                             krb5_context,
                                                         mut ctx:
                                                             krb5_init_creds_context) {
    let mut pctx: krb5_preauth_context = (*context).preauth_context;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut reqctx: krb5_preauth_req_context =
        0 as *mut krb5_preauth_req_context_st;
    let mut count: size_t = 0;
    let mut i: size_t = 0;
    if pctx.is_null() {
        k5_init_preauth_context(context);
        pctx = (*context).preauth_context;
        if pctx.is_null() { return }
    }
    reqctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_preauth_req_context_st>() as
                   libc::c_ulong) as krb5_preauth_req_context;
    if reqctx.is_null() { return }
    (*reqctx).orig_context = context;
    /* Create an array of per-request module data objects corresponding to the
     * preauth context's array of handles. */
    count = 0 as libc::c_int as size_t;
    while !(*(*pctx).handles.offset(count as isize)).is_null() {
        count = count.wrapping_add(1)
    }
    (*reqctx).modreqs =
        calloc(count,
               ::std::mem::size_of::<krb5_clpreauth_modreq>() as
                   libc::c_ulong) as *mut krb5_clpreauth_modreq;
    i = 0 as libc::c_int as size_t;
    while i < count {
        h = *(*pctx).handles.offset(i as isize);
        if (*h).vt.request_init.is_some() {
            (*h).vt.request_init.expect("non-null function pointer")(context,
                                                                     (*h).data,
                                                                     &mut *(*reqctx).modreqs.offset(i
                                                                                                        as
                                                                                                        isize));
        }
        i = i.wrapping_add(1)
    }
    (*ctx).preauth_reqctx = reqctx;
}
/* Free the per-AS-REQ context. This means clearing any request-specific
 * context which the plugin may have created. */
#[no_mangle]
#[c2rust::src_loc = "276:1"]
pub unsafe extern "C" fn k5_preauth_request_context_fini(mut context:
                                                             krb5_context,
                                                         mut ctx:
                                                             krb5_init_creds_context) {
    let mut pctx: krb5_preauth_context = (*context).preauth_context;
    let mut reqctx: krb5_preauth_req_context = (*ctx).preauth_reqctx;
    let mut i: size_t = 0;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    if reqctx.is_null() { return }
    if (*reqctx).orig_context == context && !pctx.is_null() {
        i = 0 as libc::c_int as size_t;
        while !(*(*pctx).handles.offset(i as isize)).is_null() {
            h = *(*pctx).handles.offset(i as isize);
            if !(*(*reqctx).modreqs.offset(i as isize)).is_null() &&
                   (*h).vt.request_fini.is_some() {
                (*h).vt.request_fini.expect("non-null function pointer")(context,
                                                                         (*h).data,
                                                                         *(*reqctx).modreqs.offset(i
                                                                                                       as
                                                                                                       isize));
            }
            i = i.wrapping_add(1)
        }
    } else if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Wrong context passed to krb5_init_creds_free(); leaking modreq objects\x00"
                          as *const u8 as *const libc::c_char);
    }
    free((*reqctx).modreqs as *mut libc::c_void);
    free((*reqctx).failed as *mut libc::c_void);
    free(reqctx as *mut libc::c_void);
    (*ctx).preauth_reqctx = 0 as krb5_preauth_req_context;
}
#[no_mangle]
#[c2rust::src_loc = "302:1"]
pub unsafe extern "C" fn k5_preauth_check_context(mut context: krb5_context,
                                                  mut ctx:
                                                      krb5_init_creds_context)
 -> krb5_error_code {
    let mut reqctx: krb5_preauth_req_context = (*ctx).preauth_reqctx;
    if !reqctx.is_null() && (*reqctx).orig_context != context {
        krb5_set_error_message(context, 22 as libc::c_int,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"krb5_init_creds calls must use same library context\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        return 22 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Return 1 if pa_type is a real preauthentication mechanism according to the
 * module h.  Return 0 if it is not. */
#[c2rust::src_loc = "317:1"]
unsafe extern "C" fn clpreauth_is_real(mut context: krb5_context,
                                       mut h: clpreauth_handle,
                                       mut pa_type: krb5_preauthtype)
 -> libc::c_int {
    if (*h).vt.flags.is_none() { return 1 as libc::c_int }
    return ((*h).vt.flags.expect("non-null function pointer")(context,
                                                              pa_type) &
                0x1 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[c2rust::src_loc = "326:1"]
unsafe extern "C" fn clpreauth_prep_questions(mut context: krb5_context,
                                              mut h: clpreauth_handle,
                                              mut modreq:
                                                  krb5_clpreauth_modreq,
                                              mut opt:
                                                  *mut krb5_get_init_creds_opt,
                                              mut cb:
                                                  krb5_clpreauth_callbacks,
                                              mut rock: krb5_clpreauth_rock,
                                              mut req: *mut krb5_kdc_req,
                                              mut req_body: *mut krb5_data,
                                              mut prev_req: *mut krb5_data,
                                              mut pa_data: *mut krb5_pa_data)
 -> krb5_error_code {
    if (*h).vt.prep_questions.is_none() { return 0 as libc::c_int }
    return (*h).vt.prep_questions.expect("non-null function pointer")(context,
                                                                      (*h).data,
                                                                      modreq,
                                                                      opt, cb,
                                                                      rock,
                                                                      req,
                                                                      req_body,
                                                                      prev_req,
                                                                      pa_data);
}
#[c2rust::src_loc = "340:1"]
unsafe extern "C" fn clpreauth_process(mut context: krb5_context,
                                       mut h: clpreauth_handle,
                                       mut modreq: krb5_clpreauth_modreq,
                                       mut opt: *mut krb5_get_init_creds_opt,
                                       mut cb: krb5_clpreauth_callbacks,
                                       mut rock: krb5_clpreauth_rock,
                                       mut req: *mut krb5_kdc_req,
                                       mut req_body: *mut krb5_data,
                                       mut prev_req: *mut krb5_data,
                                       mut pa_data: *mut krb5_pa_data,
                                       mut prompter: krb5_prompter_fct,
                                       mut prompter_data: *mut libc::c_void,
                                       mut pa_data_out:
                                           *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    return (*h).vt.process.expect("non-null function pointer")(context,
                                                               (*h).data,
                                                               modreq, opt,
                                                               cb, rock, req,
                                                               req_body,
                                                               prev_req,
                                                               pa_data,
                                                               prompter,
                                                               prompter_data,
                                                               pa_data_out);
}
#[c2rust::src_loc = "353:1"]
unsafe extern "C" fn clpreauth_tryagain(mut context: krb5_context,
                                        mut h: clpreauth_handle,
                                        mut modreq: krb5_clpreauth_modreq,
                                        mut opt: *mut krb5_get_init_creds_opt,
                                        mut cb: krb5_clpreauth_callbacks,
                                        mut rock: krb5_clpreauth_rock,
                                        mut req: *mut krb5_kdc_req,
                                        mut req_body: *mut krb5_data,
                                        mut prev_req: *mut krb5_data,
                                        mut pa_type: krb5_preauthtype,
                                        mut error: *mut krb5_error,
                                        mut error_padata:
                                            *mut *mut krb5_pa_data,
                                        mut prompter: krb5_prompter_fct,
                                        mut prompter_data: *mut libc::c_void,
                                        mut pa_data_out:
                                            *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    if (*h).vt.tryagain.is_none() { return 0 as libc::c_int }
    return (*h).vt.tryagain.expect("non-null function pointer")(context,
                                                                (*h).data,
                                                                modreq, opt,
                                                                cb, rock, req,
                                                                req_body,
                                                                prev_req,
                                                                pa_type,
                                                                error,
                                                                error_padata,
                                                                prompter,
                                                                prompter_data,
                                                                pa_data_out);
}
#[c2rust::src_loc = "369:1"]
unsafe extern "C" fn clpreauth_gic_opts(mut context: krb5_context,
                                        mut h: clpreauth_handle,
                                        mut opt: *mut krb5_get_init_creds_opt,
                                        mut attr: *const libc::c_char,
                                        mut value: *const libc::c_char)
 -> krb5_error_code {
    if (*h).vt.gic_opts.is_none() { return 0 as libc::c_int }
    return (*h).vt.gic_opts.expect("non-null function pointer")(context,
                                                                (*h).data,
                                                                opt, attr,
                                                                value);
}
/* Add the named encryption type to the existing list of ktypes. */
#[c2rust::src_loc = "380:1"]
unsafe extern "C" fn grow_ktypes(mut out_ktypes: *mut *mut krb5_enctype,
                                 mut out_nktypes: *mut libc::c_int,
                                 mut ktype: krb5_enctype) {
    let mut i: libc::c_int = 0;
    let mut ktypes: *mut krb5_enctype = 0 as *mut krb5_enctype;
    i = 0 as libc::c_int;
    while i < *out_nktypes {
        if *(*out_ktypes).offset(i as isize) == ktype { return }
        i += 1
    }
    ktypes =
        realloc(*out_ktypes as *mut libc::c_void,
                ((*out_nktypes + 2 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_enctype>()
                                                     as libc::c_ulong)) as
            *mut krb5_enctype;
    if !ktypes.is_null() {
        *out_ktypes = ktypes;
        let fresh4 = *out_nktypes;
        *out_nktypes = *out_nktypes + 1;
        *ktypes.offset(fresh4 as isize) = ktype;
        *ktypes.offset(*out_nktypes as isize) = 0 as libc::c_int
    };
}
/* Add a list of new pa_data items to an existing list. */
#[c2rust::src_loc = "399:1"]
unsafe extern "C" fn grow_pa_list(mut out_pa_list:
                                      *mut *mut *mut krb5_pa_data,
                                  mut out_pa_list_size: *mut libc::c_int,
                                  mut addition: *mut *mut krb5_pa_data,
                                  mut num_addition: libc::c_int)
 -> libc::c_int {
    let mut pa_list: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut i: libc::c_int = 0;
    /* Allocate space for new entries and a null terminator. */
    pa_list =
        realloc(*out_pa_list as *mut libc::c_void,
                ((*out_pa_list_size + num_addition + 1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_pa_data>()
                                                     as libc::c_ulong)) as
            *mut *mut krb5_pa_data;
    if pa_list.is_null() { return 12 as libc::c_int }
    *out_pa_list = pa_list;
    i = 0 as libc::c_int;
    while i < num_addition {
        let fresh5 = *out_pa_list_size;
        *out_pa_list_size = *out_pa_list_size + 1;
        let ref mut fresh6 = *pa_list.offset(fresh5 as isize);
        *fresh6 = *addition.offset(i as isize);
        i += 1
    }
    let ref mut fresh7 = *pa_list.offset(*out_pa_list_size as isize);
    *fresh7 = 0 as *mut krb5_pa_data;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "418:1"]
unsafe extern "C" fn get_etype(mut context: krb5_context,
                               mut rock: krb5_clpreauth_rock)
 -> krb5_enctype {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    if !(*ctx).reply.is_null() { return (*(*ctx).reply).enc_part.enctype }
    return (*ctx).etype;
}
#[c2rust::src_loc = "428:1"]
unsafe extern "C" fn fast_armor(mut context: krb5_context,
                                mut rock: krb5_clpreauth_rock)
 -> *mut krb5_keyblock {
    return (*(*(rock as krb5_init_creds_context)).fast_state).armor_key;
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn get_as_key(mut context: krb5_context,
                                mut rock: krb5_clpreauth_rock,
                                mut keyblock: *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    let mut ret: krb5_error_code = 0;
    let mut salt: *mut krb5_data = 0 as *mut krb5_data;
    if (*ctx).as_key.length == 0 as libc::c_int as libc::c_uint {
        salt =
            if (*ctx).default_salt != 0 {
                0 as *mut krb5_data
            } else { &mut (*ctx).salt };
        ret =
            (*ctx).gak_fct.expect("non-null function pointer")(context,
                                                               (*(*ctx).request).client,
                                                               (*ctx).etype,
                                                               (*ctx).prompter,
                                                               (*ctx).prompter_data,
                                                               salt,
                                                               &mut (*ctx).s2kparams,
                                                               &mut (*ctx).as_key,
                                                               (*ctx).gak_data,
                                                               (*ctx).rctx.items);
        if ret != 0 { return ret }
    }
    *keyblock = &mut (*ctx).as_key;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "455:1"]
unsafe extern "C" fn set_as_key(mut context: krb5_context,
                                mut rock: krb5_clpreauth_rock,
                                mut keyblock: *const krb5_keyblock)
 -> krb5_error_code {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    krb5_free_keyblock_contents(context, &mut (*ctx).as_key);
    return krb5_copy_keyblock_contents(context, keyblock, &mut (*ctx).as_key);
}
#[c2rust::src_loc = "465:1"]
unsafe extern "C" fn get_preauth_time(mut context: krb5_context,
                                      mut rock: krb5_clpreauth_rock,
                                      mut allow_unauth_time: krb5_boolean,
                                      mut time_out: *mut krb5_timestamp,
                                      mut usec_out: *mut krb5_int32)
 -> krb5_error_code {
    return k5_init_creds_current_time(context,
                                      rock as krb5_init_creds_context,
                                      allow_unauth_time, time_out, usec_out);
}
#[c2rust::src_loc = "474:1"]
unsafe extern "C" fn responder_ask_question(mut context: krb5_context,
                                            mut rock: krb5_clpreauth_rock,
                                            mut question: *const libc::c_char,
                                            mut challenge:
                                                *const libc::c_char)
 -> krb5_error_code {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    /* Force plugins to use need_as_key(). */
    if strcmp(b"password\x00" as *const u8 as *const libc::c_char, question)
           == 0 as libc::c_int {
        return 22 as libc::c_int
    }
    return k5_response_items_ask_question((*ctx).rctx.items, question,
                                          challenge);
}
#[c2rust::src_loc = "487:1"]
unsafe extern "C" fn responder_get_answer(mut context: krb5_context,
                                          mut rock: krb5_clpreauth_rock,
                                          mut question: *const libc::c_char)
 -> *const libc::c_char {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    /* Don't let plugins get the raw password. */
    if strcmp(b"password\x00" as *const u8 as *const libc::c_char, question)
           == 0 as libc::c_int {
        return 0 as *const libc::c_char
    }
    return k5_response_items_get_answer((*ctx).rctx.items, question);
}
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn need_as_key(mut context: krb5_context,
                                 mut rock: krb5_clpreauth_rock) {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    /* Calling gac_fct() with NULL as_key indicates desire for the AS key. */
    (*ctx).gak_fct.expect("non-null function pointer")(context,
                                                       (*(*ctx).request).client,
                                                       (*ctx).etype, None,
                                                       0 as *mut libc::c_void,
                                                       0 as *mut krb5_data,
                                                       0 as *mut krb5_data,
                                                       0 as
                                                           *mut krb5_keyblock,
                                                       (*ctx).gak_data,
                                                       (*ctx).rctx.items);
}
#[c2rust::src_loc = "509:1"]
unsafe extern "C" fn get_cc_config(mut context: krb5_context,
                                   mut rock: krb5_clpreauth_rock,
                                   mut key: *const libc::c_char)
 -> *const libc::c_char {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    let mut value: k5_json_value = 0 as *mut libc::c_void;
    if (*ctx).cc_config_in.is_null() { return 0 as *const libc::c_char }
    value = k5_json_object_get((*ctx).cc_config_in, key);
    if value.is_null() { return 0 as *const libc::c_char }
    if k5_json_get_tid(value) != 131 as libc::c_int as libc::c_uint {
        return 0 as *const libc::c_char
    }
    return k5_json_string_utf8(value as k5_json_string);
}
#[c2rust::src_loc = "528:1"]
unsafe extern "C" fn set_cc_config(mut context: krb5_context,
                                   mut rock: krb5_clpreauth_rock,
                                   mut key: *const libc::c_char,
                                   mut data: *const libc::c_char)
 -> krb5_error_code {
    let mut ctx: krb5_init_creds_context = rock as krb5_init_creds_context;
    let mut ret: krb5_error_code = 0;
    let mut str: k5_json_string = 0 as *mut k5_json_string_st;
    if (*ctx).cc_config_out.is_null() { return 2 as libc::c_int }
    ret = k5_json_string_create(data, &mut str);
    if ret != 0 { return ret }
    ret = k5_json_object_set((*ctx).cc_config_out, key, str as k5_json_value);
    k5_json_release(str as k5_json_value);
    return ret;
}
#[c2rust::src_loc = "548:1"]
unsafe extern "C" fn disable_fallback(mut context: krb5_context,
                                      mut rock: krb5_clpreauth_rock) {
    (*(rock as krb5_init_creds_context)).fallback_disabled =
        1 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "554:43"]
static mut callbacks: krb5_clpreauth_callbacks_st =
    unsafe {
        {
            let mut init =
                krb5_clpreauth_callbacks_st{vers: 3 as libc::c_int,
                                            get_etype:
                                                Some(get_etype as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock)
                                                             -> krb5_enctype),
                                            fast_armor:
                                                Some(fast_armor as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock)
                                                             ->
                                                                 *mut krb5_keyblock),
                                            get_as_key:
                                                Some(get_as_key as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  *mut *mut krb5_keyblock)
                                                             ->
                                                                 krb5_error_code),
                                            set_as_key:
                                                Some(set_as_key as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  *const krb5_keyblock)
                                                             ->
                                                                 krb5_error_code),
                                            get_preauth_time:
                                                Some(get_preauth_time as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  krb5_boolean,
                                                                              _:
                                                                                  *mut krb5_timestamp,
                                                                              _:
                                                                                  *mut krb5_int32)
                                                             ->
                                                                 krb5_error_code),
                                            ask_responder_question:
                                                Some(responder_ask_question as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  *const libc::c_char,
                                                                              _:
                                                                                  *const libc::c_char)
                                                             ->
                                                                 krb5_error_code),
                                            get_responder_answer:
                                                Some(responder_get_answer as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  *const libc::c_char)
                                                             ->
                                                                 *const libc::c_char),
                                            need_as_key:
                                                Some(need_as_key as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock)
                                                             -> ()),
                                            get_cc_config:
                                                Some(get_cc_config as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  *const libc::c_char)
                                                             ->
                                                                 *const libc::c_char),
                                            set_cc_config:
                                                Some(set_cc_config as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock,
                                                                              _:
                                                                                  *const libc::c_char,
                                                                              _:
                                                                                  *const libc::c_char)
                                                             ->
                                                                 krb5_error_code),
                                            disable_fallback:
                                                Some(disable_fallback as
                                                         unsafe extern "C" fn(_:
                                                                                  krb5_context,
                                                                              _:
                                                                                  krb5_clpreauth_rock)
                                                             -> ()),};
            init
        }
    };
/* Tweak the request body, for now adding any enctypes which the module claims
 * to add support for to the list, but in the future perhaps doing more
 * involved things. */
#[no_mangle]
#[c2rust::src_loc = "572:1"]
pub unsafe extern "C" fn k5_preauth_prepare_request(mut context: krb5_context,
                                                    mut opt:
                                                        *mut krb5_get_init_creds_opt,
                                                    mut req:
                                                        *mut krb5_kdc_req) {
    let mut pctx: krb5_preauth_context = (*context).preauth_context;
    let mut hp: *mut clpreauth_handle = 0 as *mut clpreauth_handle;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut ep: *mut krb5_enctype = 0 as *mut krb5_enctype;
    if pctx.is_null() { return }
    /* Don't modify the enctype list if it's specified in the gic opts. */
    if !opt.is_null() && (*opt).flags & 0x10 as libc::c_int != 0 { return }
    hp = (*pctx).handles;
    while !(*hp).is_null() {
        h = *hp;
        if !(*h).vt.enctype_list.is_null() {
            ep = (*h).vt.enctype_list;
            while *ep != 0 as libc::c_int {
                grow_ktypes(&mut (*req).ktype, &mut (*req).nktypes, *ep);
                ep = ep.offset(1)
            }
        }
        hp = hp.offset(1)
    };
}
/* *
 * List the question names contained in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 *
 * Return a pointer to a null-terminated list of question names which are
 * present in @a rctx.  The pointer is an alias, valid only as long as the
 * lifetime of @a rctx, and should not be modified or freed by the caller.  A
 * question's challenge can be retrieved using krb5_responder_get_challenge()
 * and answered using krb5_responder_set_answer().
 *
 * @version New in 1.11
 */
#[no_mangle]
#[c2rust::src_loc = "594:1"]
pub unsafe extern "C" fn krb5_responder_list_questions(mut ctx: krb5_context,
                                                       mut rctx:
                                                           krb5_responder_context)
 -> *const *const libc::c_char {
    return k5_response_items_list_questions((*rctx).items);
}
/* *
 * Retrieve the challenge data for a given question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 *
 * Return a pointer to a C string containing the challenge for @a question
 * within @a rctx, or NULL if the question is not present in @a rctx.  The
 * structure of the question depends on the question name, but will always be
 * printable UTF-8 text.  The returned pointer is an alias, valid only as long
 * as the lifetime of @a rctx, and should not be modified or freed by the
 * caller.
 *
 * @version New in 1.11
 */
#[no_mangle]
#[c2rust::src_loc = "600:1"]
pub unsafe extern "C" fn krb5_responder_get_challenge(mut ctx: krb5_context,
                                                      mut rctx:
                                                          krb5_responder_context,
                                                      mut question:
                                                          *const libc::c_char)
 -> *const libc::c_char {
    if rctx.is_null() { return 0 as *const libc::c_char }
    return k5_response_items_get_challenge((*rctx).items, question);
}
/* *
 * Answer a named question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 * @param [in] answer           The string to set (MUST be printable UTF-8)
 *
 * This function supplies an answer to @a question within @a rctx.  The
 * appropriate form of the answer depends on the question name.
 *
 * @retval EINVAL @a question is not present within @a rctx
 *
 * @version New in 1.11
 */
#[no_mangle]
#[c2rust::src_loc = "610:1"]
pub unsafe extern "C" fn krb5_responder_set_answer(mut ctx: krb5_context,
                                                   mut rctx:
                                                       krb5_responder_context,
                                                   mut question:
                                                       *const libc::c_char,
                                                   mut answer:
                                                       *const libc::c_char)
 -> krb5_error_code {
    if rctx.is_null() { return 22 as libc::c_int }
    return k5_response_items_set_answer((*rctx).items, question, answer);
}
/* Return true if pa_type matches the specific preauth type allowed for this
 * authentication, or if there is no specific allowed type. */
#[inline]
#[c2rust::src_loc = "622:1"]
unsafe extern "C" fn pa_type_allowed(mut ctx: krb5_init_creds_context,
                                     mut pa_type: krb5_preauthtype)
 -> krb5_boolean {
    return ((*ctx).allowed_preauth_type == 0 as libc::c_int ||
                pa_type == (*ctx).allowed_preauth_type) as libc::c_int as
               krb5_boolean;
}
/* Return true if pa_type previously failed during this authentication. */
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn previously_failed(mut ctx: krb5_init_creds_context,
                                       mut pa_type: krb5_preauthtype)
 -> krb5_boolean {
    let mut reqctx: krb5_preauth_req_context = (*ctx).preauth_reqctx;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while !(*reqctx).failed.is_null() &&
              *(*reqctx).failed.offset(i as isize) != 0 as libc::c_int {
        if *(*reqctx).failed.offset(i as isize) == pa_type {
            return 1 as libc::c_int as krb5_boolean
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Allow clpreauth modules to process in_pa_list and produce output padata. */
#[c2rust::src_loc = "644:1"]
unsafe extern "C" fn process_pa_data(mut context: krb5_context,
                                     mut ctx: krb5_init_creds_context,
                                     mut in_pa_list: *mut *mut krb5_pa_data,
                                     mut must_preauth: krb5_boolean,
                                     mut out_pa_list:
                                         *mut *mut *mut krb5_pa_data,
                                     mut out_pa_list_size: *mut libc::c_int,
                                     mut out_type: *mut krb5_preauthtype)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut save: errinfo =
        {
            let mut init =
                errinfo{code: 0 as libc::c_int as libc::c_long,
                        msg: 0 as *mut libc::c_char,};
            init
        };
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut pa_ptr: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut mod_pa: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut modreq: krb5_clpreauth_modreq =
        0 as *mut krb5_clpreauth_modreq_st;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut real: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /* Process all informational padata types, then the first real preauth type
     * we succeed on. */
    real = 0 as libc::c_int;
    's_21:
        loop  {
            if !(real <= 1 as libc::c_int) {
                current_block = 15090052786889560393;
                break ;
            }
            pa_ptr = in_pa_list;
            while !(*pa_ptr).is_null() {
                pa = *pa_ptr;
                /* Restrict real mechanisms to the chosen one if we have one. */
                if !(real != 0 && pa_type_allowed(ctx, (*pa).pa_type) == 0) {
                    h = find_module(context, ctx, (*pa).pa_type, &mut modreq);
                    if !h.is_null() {
                        /* Make sure this type is for the current pass. */
                        if !(clpreauth_is_real(context, h, (*pa).pa_type) !=
                                 real) {
                            /* Don't try a real mechanism again after failure. */
                            if !(real != 0 &&
                                     previously_failed(ctx, (*pa).pa_type) !=
                                         0) {
                                mod_pa = 0 as *mut *mut krb5_pa_data;
                                ret =
                                    clpreauth_process(context, h, modreq,
                                                      (*ctx).opt,
                                                      &mut callbacks,
                                                      ctx as
                                                          krb5_clpreauth_rock,
                                                      (*ctx).request,
                                                      (*ctx).inner_request_body,
                                                      (*ctx).encoded_previous_request,
                                                      pa, (*ctx).prompter,
                                                      (*ctx).prompter_data,
                                                      &mut mod_pa);
                                if (*context).trace_callback.is_some() {
                                    krb5int_trace(context,
                                                  b"Preauth module {str} ({int}) ({str}) returned: {kerr}\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  (*h).vt.name, (*pa).pa_type,
                                                  if real != 0 {
                                                      b"real\x00" as *const u8
                                                          as
                                                          *const libc::c_char
                                                  } else {
                                                      b"info\x00" as *const u8
                                                          as
                                                          *const libc::c_char
                                                  }, ret);
                                }
                                if !mod_pa.is_null() {
                                    i = 0 as libc::c_int;
                                    while !(*mod_pa.offset(i as
                                                               isize)).is_null()
                                          {
                                        i += 1
                                    }
                                    ret =
                                        grow_pa_list(out_pa_list,
                                                     out_pa_list_size, mod_pa,
                                                     i);
                                    if ret != 0 {
                                        krb5_free_pa_data(context, mod_pa);
                                        current_block = 7976727365471131389;
                                        break 's_21 ;
                                    } else {
                                        free(mod_pa as *mut libc::c_void);
                                    }
                                }
                                /* Don't continue to try mechanisms after a keyboard interrupt. */
                                if ret as libc::c_long ==
                                       -(1765328252 as libc::c_long) {
                                    current_block = 7976727365471131389;
                                    break 's_21 ;
                                }
                                if ret == 0 as libc::c_int && real != 0 {
                                    /* Stop now and record which real padata type we answered. */
                                    *out_type = (*pa).pa_type;
                                    current_block = 7976727365471131389;
                                    break 's_21 ;
                                } else {
                                    if real != 0 &&
                                           save.code ==
                                               0 as libc::c_int as
                                                   libc::c_long {
                                        /* Save the first error we get from a real preauth type. */
                                        k5_save_ctx_error(context, ret,
                                                          &mut save);
                                    }
                                    if real != 0 && ret != 0 {
                                        /* Don't try this mechanism again for this authentication. */
                                        ret =
                                            k5_preauth_note_failed(ctx,
                                                                   (*pa).pa_type);
                                        if ret != 0 {
                                            current_block =
                                                7976727365471131389;
                                            break 's_21 ;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                pa_ptr = pa_ptr.offset(1)
            }
            real += 1
        }
    match current_block {
        15090052786889560393 => {
            if must_preauth != 0 {
                /* No real preauth types succeeded and we needed to preauthenticate. */
                if save.code != 0 as libc::c_int as libc::c_long {
                    ret = k5_restore_ctx_error(context, &mut save);
                    krb5_wrap_error_message(context, ret,
                                            -(1765328174 as libc::c_long) as
                                                krb5_error_code,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"Pre-authentication failed\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                }
                ret = -(1765328174 as libc::c_long) as krb5_error_code
            }
        }
        _ => { }
    }
    k5_clear_error(&mut save);
    return ret;
}
#[inline]
#[c2rust::src_loc = "727:1"]
unsafe extern "C" fn padata2data(mut p: krb5_pa_data) -> krb5_data {
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    d.magic = -(1760647422 as libc::c_long) as krb5_magic;
    d.length = p.length;
    d.data = p.contents as *mut libc::c_char;
    return d;
}
/* Set salt in rock based on pw-salt or afs3-salt elements in padata. */
#[c2rust::src_loc = "738:1"]
unsafe extern "C" fn get_salt(mut context: krb5_context,
                              mut ctx: krb5_init_creds_context,
                              mut padata: *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* Look for a pw-salt or afs3-salt element. */
    pa = krb5int_find_pa_data(context, padata, 3 as libc::c_int);
    if pa.is_null() {
        pa = krb5int_find_pa_data(context, padata, 10 as libc::c_int)
    }
    if pa.is_null() { return 0 as libc::c_int }
    /* Set ctx->salt based on the element we found. */
    krb5_free_data_contents(context, &mut (*ctx).salt);
    d = padata2data(*pa);
    ret = krb5int_copy_data_contents(context, &mut d, &mut (*ctx).salt);
    if ret != 0 { return ret }
    /* Adjust the salt if we got it from an afs3-salt element. */
    if (*pa).pa_type == 10 as libc::c_int {
        /* Work around a (possible) old Heimdal KDC foible. */
        p =
            memchr((*ctx).salt.data as *const libc::c_void, '@' as i32,
                   (*ctx).salt.length as libc::c_ulong) as
                *const libc::c_char;
        if !p.is_null() {
            (*ctx).salt.length =
                p.wrapping_offset_from((*ctx).salt.data) as libc::c_long as
                    libc::c_uint
        }
        /* Tolerate extra null in MIT KDC afs3-salt value. */
        if (*ctx).salt.length > 0 as libc::c_int as libc::c_uint &&
               *(*ctx).salt.data.offset((*ctx).salt.length.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                            as isize) as libc::c_int ==
                   '\u{0}' as i32 {
            (*ctx).salt.length = (*ctx).salt.length.wrapping_sub(1)
        }
        /* Set an s2kparams value to indicate AFS string-to-key. */
        krb5_free_data_contents(context, &mut (*ctx).s2kparams);
        ret =
            alloc_data(&mut (*ctx).s2kparams,
                       1 as libc::c_int as libc::c_uint);
        if ret != 0 { return ret }
        *(*ctx).s2kparams.data.offset(0 as libc::c_int as isize) =
            '\u{1}' as i32 as libc::c_char
    }
    (*ctx).default_salt = 0 as libc::c_int as krb5_boolean;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Received salt \"{data}\" via padata type {patype}\x00"
                          as *const u8 as *const libc::c_char,
                      &mut (*ctx).salt as *mut krb5_data, (*pa).pa_type);
    }
    return 0 as libc::c_int;
}
/* Set etype info parameters in rock based on padata. */
#[no_mangle]
#[c2rust::src_loc = "785:1"]
pub unsafe extern "C" fn k5_get_etype_info(mut context: krb5_context,
                                           mut ctx: krb5_init_creds_context,
                                           mut padata: *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut etype_info: krb5_etype_info = 0 as krb5_etype_info;
    let mut e: krb5_etype_info = 0 as *mut *mut krb5_etype_info_entry;
    let mut entry: *mut krb5_etype_info_entry =
        0 as *mut krb5_etype_info_entry;
    let mut valid_found: krb5_boolean = 0;
    let mut i: libc::c_int = 0;
    /* Find an etype-info2 or etype-info element in padata. */
    pa = krb5int_find_pa_data(context, padata, 19 as libc::c_int);
    if !pa.is_null() {
        d = padata2data(*pa);
        decode_krb5_etype_info2(&mut d, &mut etype_info);
    } else {
        pa = krb5int_find_pa_data(context, padata, 11 as libc::c_int);
        if !pa.is_null() {
            d = padata2data(*pa);
            decode_krb5_etype_info(&mut d, &mut etype_info);
        }
    }
    /* Fall back to pw-salt/afs3-salt if no etype-info element is present. */
    if etype_info.is_null() { return get_salt(context, ctx, padata) }
    /* Search entries in order of the request's enctype preference. */
    entry = 0 as *mut krb5_etype_info_entry;
    valid_found = 0 as libc::c_int as krb5_boolean;
    i = 0 as libc::c_int;
    while i < (*(*ctx).request).nktypes && entry.is_null() {
        e = etype_info;
        while !(*e).is_null() && entry.is_null() {
            if (**e).etype == *(*(*ctx).request).ktype.offset(i as isize) {
                entry = *e
            }
            if krb5_c_valid_enctype((**e).etype) != 0 {
                valid_found = 1 as libc::c_int as krb5_boolean
            }
            e = e.offset(1)
        }
        i += 1
    }
    if entry.is_null() {
        ret =
            if valid_found != 0 {
                -(1765328147 as libc::c_long)
            } else { -(1765328234 as libc::c_long) } as krb5_error_code
    } else {
        /* Set etype/salt/s2kparams fields based on the entry we selected. */
        (*ctx).etype = (*entry).etype;
        krb5_free_data_contents(context, &mut (*ctx).salt);
        if (*entry).length !=
               (2147483647 as libc::c_int as
                    libc::c_uint).wrapping_mul(2 as
                                                   libc::c_uint).wrapping_add(1
                                                                                  as
                                                                                  libc::c_uint)
           {
            (*ctx).salt =
                make_data((*entry).salt as *mut libc::c_void,
                          (*entry).length);
            (*entry).salt = 0 as *mut krb5_octet;
            (*ctx).default_salt = 0 as libc::c_int as krb5_boolean
        } else {
            (*ctx).salt = empty_data();
            (*ctx).default_salt = 1 as libc::c_int as krb5_boolean
        }
        krb5_free_data_contents(context, &mut (*ctx).s2kparams);
        (*ctx).s2kparams = (*entry).s2kparams;
        (*entry).s2kparams = empty_data();
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Selected etype info: etype {etype}, salt \"{data}\", params \"{data}\"\x00"
                              as *const u8 as *const libc::c_char,
                          (*ctx).etype, &mut (*ctx).salt as *mut krb5_data,
                          &mut (*ctx).s2kparams as *mut krb5_data);
        }
    }
    krb5_free_etype_info(context, etype_info);
    return ret;
}
/* Look for an fx-cookie element in in_padata and add it to out_pa_list. */
#[c2rust::src_loc = "853:1"]
unsafe extern "C" fn copy_cookie(mut context: krb5_context,
                                 mut in_padata: *mut *mut krb5_pa_data,
                                 mut out_pa_list: *mut *mut *mut krb5_pa_data,
                                 mut out_pa_list_size: *mut libc::c_int)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut cookie: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    cookie = krb5int_find_pa_data(context, in_padata, 133 as libc::c_int);
    if cookie.is_null() { return 0 as libc::c_int }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Received cookie: {lenstr}\x00" as *const u8 as
                          *const libc::c_char, (*cookie).length as size_t,
                      (*cookie).contents);
    }
    pa =
        k5alloc(::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong,
                &mut ret) as *mut krb5_pa_data;
    if pa.is_null() { return ret }
    *pa = *cookie;
    (*pa).contents =
        k5memdup((*cookie).contents as *const libc::c_void,
                 (*cookie).length as size_t, &mut ret) as *mut krb5_octet;
    if !(*pa).contents.is_null() {
        ret =
            grow_pa_list(out_pa_list, out_pa_list_size, &mut pa,
                         1 as libc::c_int);
        if !(ret != 0) { return 0 as libc::c_int }
    }
    free((*pa).contents as *mut libc::c_void);
    free(pa as *mut libc::c_void);
    return 12 as libc::c_int;
}
/*
 * If the module for pa_type can adjust its AS_REQ data using the contents of
 * err and err_padata, return 0 with *padata_out set to a padata list for the
 * next request.  If it's the sort of correction which requires that we ask the
 * user another question, we let the calling application deal with it.
 */
#[no_mangle]
#[c2rust::src_loc = "888:1"]
pub unsafe extern "C" fn k5_preauth_tryagain(mut context: krb5_context,
                                             mut ctx: krb5_init_creds_context,
                                             mut pa_type: krb5_preauthtype,
                                             mut err: *mut krb5_error,
                                             mut err_padata:
                                                 *mut *mut krb5_pa_data,
                                             mut padata_out:
                                                 *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut mod_pa: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut modreq: krb5_clpreauth_modreq =
        0 as *mut krb5_clpreauth_modreq_st;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut count: libc::c_int = 0;
    *padata_out = 0 as *mut *mut krb5_pa_data;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Preauth tryagain input types ({int}): {patypes}\x00"
                          as *const u8 as *const libc::c_char, pa_type,
                      err_padata);
    }
    h = find_module(context, ctx, pa_type, &mut modreq);
    if h.is_null() { return -(1765328324 as libc::c_long) as krb5_error_code }
    mod_pa = 0 as *mut *mut krb5_pa_data;
    ret =
        clpreauth_tryagain(context, h, modreq, (*ctx).opt, &mut callbacks,
                           ctx as krb5_clpreauth_rock, (*ctx).request,
                           (*ctx).inner_request_body,
                           (*ctx).encoded_previous_request, pa_type, err,
                           err_padata, (*ctx).prompter, (*ctx).prompter_data,
                           &mut mod_pa);
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Preauth module {str} ({int}) tryagain returned: {kerr}\x00"
                          as *const u8 as *const libc::c_char, (*h).vt.name,
                      pa_type, ret);
    }
    if ret == 0 && mod_pa.is_null() {
        ret = -(1765328324 as libc::c_long) as krb5_error_code
    }
    if ret != 0 { k5_preauth_note_failed(ctx, pa_type); return ret }
    count = 0 as libc::c_int;
    while !(*mod_pa.offset(count as isize)).is_null() { count += 1 }
    ret = copy_cookie(context, err_padata, &mut mod_pa, &mut count);
    if ret != 0 { krb5_free_pa_data(context, mod_pa); return ret }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Followup preauth for next request: {patypes}\x00" as
                          *const u8 as *const libc::c_char, mod_pa);
    }
    *padata_out = mod_pa;
    return 0 as libc::c_int;
}
/* Compile the set of response items for in_padata by invoke each module's
 * prep_questions method. */
#[c2rust::src_loc = "935:1"]
unsafe extern "C" fn fill_response_items(mut context: krb5_context,
                                         mut ctx: krb5_init_creds_context,
                                         mut in_padata:
                                             *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut modreq: krb5_clpreauth_modreq =
        0 as *mut krb5_clpreauth_modreq_st;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut i: libc::c_int = 0;
    k5_response_items_reset((*ctx).rctx.items);
    i = 0 as libc::c_int;
    while !(*in_padata.offset(i as isize)).is_null() {
        pa = *in_padata.offset(i as isize);
        if !(pa_type_allowed(ctx, (*pa).pa_type) == 0) {
            h = find_module(context, ctx, (*pa).pa_type, &mut modreq);
            if !h.is_null() {
                ret =
                    clpreauth_prep_questions(context, h, modreq, (*ctx).opt,
                                             &mut callbacks,
                                             ctx as krb5_clpreauth_rock,
                                             (*ctx).request,
                                             (*ctx).inner_request_body,
                                             (*ctx).encoded_previous_request,
                                             pa);
                if ret != 0 { return ret }
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "963:1"]
pub unsafe extern "C" fn k5_preauth(mut context: krb5_context,
                                    mut ctx: krb5_init_creds_context,
                                    mut in_padata: *mut *mut krb5_pa_data,
                                    mut must_preauth: krb5_boolean,
                                    mut padata_out:
                                        *mut *mut *mut krb5_pa_data,
                                    mut pa_type_out: *mut krb5_preauthtype)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut out_pa_list_size: libc::c_int = 0 as libc::c_int;
    let mut out_pa_list: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut ret: krb5_error_code = 0;
    let mut responder: krb5_responder_fn = None;
    let mut responder_data: *mut libc::c_void = 0 as *mut libc::c_void;
    *padata_out = 0 as *mut *mut krb5_pa_data;
    *pa_type_out = 0 as libc::c_int;
    /* We should never invoke preauth modules when identifying the realm. */
    if in_padata.is_null() || (*ctx).identify_realm != 0 {
        return 0 as libc::c_int
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Processing preauth types: {patypes}\x00" as *const u8
                          as *const libc::c_char, in_padata);
    }
    /* Scan the padata list and process etype-info or salt elements. */
    ret = k5_get_etype_info(context, ctx, in_padata);
    if ret != 0 { return ret }
    /* Copy the cookie if there is one. */
    ret =
        copy_cookie(context, in_padata, &mut out_pa_list,
                    &mut out_pa_list_size);
    if !(ret != 0) {
        /* If we can't initialize the preauth context, stop with what we have. */
        k5_init_preauth_context(context);
        if (*context).preauth_context.is_null() {
            *padata_out = out_pa_list;
            out_pa_list = 0 as *mut *mut krb5_pa_data
        } else {
            /* Get a list of response items for in_padata from the preauth modules. */
            ret = fill_response_items(context, ctx, in_padata);
            if !(ret != 0) {
                /* Call the responder to answer response items. */
                k5_gic_opt_get_responder((*ctx).opt, &mut responder,
                                         &mut responder_data);
                if responder.is_some() &&
                       k5_response_items_empty((*ctx).rctx.items) == 0 {
                    ret =
                        Some(responder.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                                responder_data,
                                                                                                                &mut (*ctx).rctx);
                    if ret != 0 {
                        current_block = 13583134452241760669;
                    } else { current_block = 15768484401365413375; }
                } else { current_block = 15768484401365413375; }
                match current_block {
                    13583134452241760669 => { }
                    _ => {
                        ret =
                            process_pa_data(context, ctx, in_padata,
                                            must_preauth, &mut out_pa_list,
                                            &mut out_pa_list_size,
                                            pa_type_out);
                        if !(ret != 0) {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"Produced preauth for next request: {patypes}\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              out_pa_list);
                            }
                            *padata_out = out_pa_list;
                            return 0 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    krb5_free_pa_data(context, out_pa_list);
    return ret;
}
/*
 * Give all the preauth plugins a look at the preauth option which
 * has just been set
 */
#[no_mangle]
#[c2rust::src_loc = "1032:1"]
pub unsafe extern "C" fn krb5_preauth_supply_preauth_data(mut context:
                                                              krb5_context,
                                                          mut opt:
                                                              *mut krb5_get_init_creds_opt,
                                                          mut attr:
                                                              *const libc::c_char,
                                                          mut value:
                                                              *const libc::c_char)
 -> krb5_error_code {
    let mut pctx: krb5_preauth_context = (*context).preauth_context;
    let mut hp: *mut clpreauth_handle = 0 as *mut clpreauth_handle;
    let mut h: clpreauth_handle = 0 as *mut C2RustUnnamed;
    let mut ret: krb5_error_code = 0;
    if pctx.is_null() {
        k5_init_preauth_context(context);
        pctx = (*context).preauth_context;
        if pctx.is_null() {
            krb5_set_error_message(context, 22 as libc::c_int,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Unable to initialize preauth context\x00"
                                                as *const u8 as
                                                *const libc::c_char));
            return 22 as libc::c_int
        }
    }
    /*
     * Go down the list of preauth modules, and supply them with the
     * attribute/value pair.
     */
    hp = (*pctx).handles;
    while !(*hp).is_null() {
        h = *hp;
        ret = clpreauth_gic_opts(context, h, opt, attr, value);
        if ret != 0 {
            krb5_prepend_error_message(context, ret,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Preauth module %s\x00" as
                                                    *const u8 as
                                                    *const libc::c_char),
                                       (*h).vt.name);
            return ret
        }
        hp = hp.offset(1)
    }
    return 0 as libc::c_int;
}
