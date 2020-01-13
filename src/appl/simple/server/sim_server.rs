use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:35"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
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
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:35"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:35"]
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
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
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
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
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
    #[c2rust::src_loc = "2246:16"]
    pub struct krb5_replay_data {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq: krb5_ui_4,
    }
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* cleartext portion */
        /* *< server name/realm */
        /* *< encryption type, kvno, encrypted encoding */
        /* *< ptr to decrypted version, if available */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[c2rust::src_loc = "2735:8"]
        pub type _krb5_kt;
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
        #[no_mangle]
        #[c2rust::src_loc = "3349:1"]
        pub fn krb5_rd_safe(context: krb5_context,
                            auth_context: krb5_auth_context,
                            inbuf: *const krb5_data,
                            userdata_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "3390:1"]
        pub fn krb5_rd_priv(context: krb5_context,
                            auth_context: krb5_auth_context,
                            inbuf: *const krb5_data,
                            userdata_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
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
        /* *
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4961:1"]
        pub fn krb5_sname_to_principal(context: krb5_context,
                                       hostname: *const libc::c_char,
                                       sname: *const libc::c_char,
                                       type_0: krb5_int32,
                                       ret_princ: *mut krb5_principal)
         -> krb5_error_code;
        /* KRB5_DEPRECATED */
        /* *
 * Parse and decrypt a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     inbuf          AP-REQ message to be parsed
 * @param [in]     server         Matching principal for server, or NULL to
 *                                allow any principal in keytab
 * @param [in]     keytab         Key table, or NULL to use the default
 * @param [out]    ap_req_options If non-null, the AP-REQ flags on output
 * @param [out]    ticket         If non-null, ticket from the AP-REQ message
 *
 * This function parses, decrypts and verifies a AP-REQ message from @a inbuf
 * and stores the authenticator in @a auth_context.
 *
 * If a keyblock was specified in @a auth_context using
 * krb5_auth_con_setuseruserkey(), that key is used to decrypt the ticket in
 * AP-REQ message and @a keytab is ignored.  In this case, @a server should be
 * specified as a complete principal name to allow for proper transited-path
 * checking and replay cache selection.
 *
 * Otherwise, the decryption key is obtained from @a keytab, or from the
 * default keytab if it is NULL.  In this case, @a server may be a complete
 * principal name, a matching principal (see krb5_sname_match()), or NULL to
 * match any principal name.  The keys tried against the encrypted part of the
 * ticket are determined as follows:
 *
 * - If @a server is a complete principal name, then its entry in @a keytab is
 *   tried.
 * - Otherwise, if @a keytab is iterable, then all entries in @a keytab which
 *   match @a server are tried.
 * - Otherwise, the server principal in the ticket must match @a server, and
 *   its entry in @a keytab is tried.
 *
 * The client specified in the decrypted authenticator must match the client
 * specified in the decrypted ticket.
 *
 * If the @a remote_addr field of @a auth_context is set, the request must come
 * from that address.
 *
 * If a replay cache handle is provided in the @a auth_context, the
 * authenticator and ticket are verified against it.  If no conflict is found,
 * the new authenticator is then stored in the replay cache of @a auth_context.
 *
 * Various other checks are performed on the decoded data, including
 * cross-realm policy, clockskew, and ticket validation times.
 *
 * On success the authenticator, subkey, and remote sequence number of the
 * request are stored in @a auth_context. If the #AP_OPTS_MUTUAL_REQUIRED
 * bit is set, the local sequence number is XORed with the remote sequence
 * number in the request.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5217:1"]
        pub fn krb5_rd_req(context: krb5_context,
                           auth_context: *mut krb5_auth_context,
                           inbuf: *const krb5_data,
                           server: krb5_const_principal, keytab: krb5_keytab,
                           ap_req_options: *mut krb5_flags,
                           ticket: *mut *mut krb5_ticket) -> krb5_error_code;
        /* *
 * Free a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context to be freed
 *
 * This function frees an auth context allocated by krb5_auth_con_init().
 *
 * @retval 0  (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
        /* *
 * Set the local and remote addresses in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_addr       Local address
 * @param [in] remote_addr      Remote address
 *
 * This function releases the storage assigned to the contents of the local and
 * remote addresses of @a auth_context and then sets them to @a local_addr and
 * @a remote_addr respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5718:1"]
        pub fn krb5_auth_con_setaddrs(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      local_addr: *mut krb5_address,
                                      remote_addr: *mut krb5_address)
         -> krb5_error_code;
        /* *
 * Set local and remote port fields in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_port       Local port
 * @param [in] remote_port      Remote port
 *
 * This function releases the storage assigned to the contents of the local and
 * remote ports of @a auth_context and then sets them to @a local_port and @a
 * remote_port respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5752:1"]
        pub fn krb5_auth_con_setports(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      local_port: *mut krb5_address,
                                      remote_port: *mut krb5_address)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:35"]
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
#[c2rust::header_src = "/usr/include/bits/socket.h:36"]
pub mod socket_h {
    #[c2rust::src_loc = "33:1"]
    pub type socklen_t = __socklen_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::types_h::__socklen_t;
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:36"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:36"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:36"]
pub mod in_h {
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
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
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::sockaddr_h::sa_family_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:36"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:37"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:37"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdlib.h:35"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/socket.h:36"]
pub mod sys_socket_h {
    use super::socket_h::{sockaddr, socklen_t};
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: *const sockaddr,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "163:1"]
        pub fn recvfrom(__fd: libc::c_int, __buf: *mut libc::c_void,
                        __n: size_t, __flags: libc::c_int,
                        __addr: *mut sockaddr, __addr_len: *mut socklen_t)
         -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:36"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:37"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:39"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t, __off_t,
                        __off64_t, __ssize_t, __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_flags, krb5_timestamp, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_context, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_enc_data, krb5_enc_data,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _krb5_ticket,
                       krb5_ticket, krb5_replay_data, krb5_keytab,
                       _krb5_context, _krb5_auth_context, _krb5_kt,
                       krb5_init_context, krb5_free_context, krb5_rd_safe,
                       krb5_rd_priv, krb5_unparse_name, krb5_kt_resolve,
                       krb5_free_data_contents, krb5_sname_to_principal,
                       krb5_rd_req, krb5_auth_con_free,
                       krb5_auth_con_setaddrs, krb5_auth_con_setports};
pub use self::com_err_h::{errcode_t, com_err};
pub use self::socket_h::{socklen_t, sockaddr};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::in_h::{in_addr_t, in_addr, in_port_t, sockaddr_in, htons};
pub use self::netdb_h::{servent, getservbyname};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
use self::stdlib_h::{exit, free, atoi};
use self::sys_socket_h::{socket, bind, recvfrom, setsockopt};
use self::string_h::memset;
use self::stdio_h::{stderr, fprintf, printf, perror};
use self::getopt_core_h::{optarg, getopt};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* appl/simple/server/sim_server.c */
/*
 * Copyright 1989,1991 by the Massachusetts Institute of Technology.
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
 * Usage:
 * sample_server servername
 *
 * Simple UDP-based server application.  For demonstration.
 * This program performs no useful function.
 */
/* for old Unixes and friends ... */
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn usage(mut name: *mut libc::c_char) {
    fprintf(stderr,
            b"usage: %s [-p port] [-s service] [-S keytab]\n\x00" as *const u8
                as *const libc::c_char, name); /* for recvfrom() */
}
#[c2rust::src_loc = "64:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut sock: libc::c_int = 0; /* server's address */
    let mut i: libc::c_int = 0; /* client's address */
    let mut len: socklen_t = 0; /* If user specifies port */
    let mut flags: libc::c_int =
        0 as libc::c_int; /* Allow specification on command line */
    let mut on: libc::c_int = 1 as libc::c_int;
    let mut serv: *mut servent = 0 as *mut servent;
    let mut s_sock: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut c_sock: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    extern "C" {
        #[link_name = "optarg"]
        pub static mut optarg_0: *mut libc::c_char;
    }
    let mut ch: libc::c_int = 0;
    let mut port: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut keytab: krb5_keytab = 0 as krb5_keytab;
    let mut service: *mut libc::c_char =
        b"sample\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut packet: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut message: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pktbuf: [libc::c_uchar; 8192] = [0; 8192];
    let mut sprinc: krb5_principal = 0 as *mut krb5_principal_data;
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut auth_context: krb5_auth_context = 0 as krb5_auth_context;
    let mut addr: krb5_address =
        krb5_address{magic: 0,
                     addrtype: 0,
                     length: 0,
                     contents: 0 as *mut krb5_octet,};
    let mut ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    retval = krb5_init_context(&mut context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while initializing krb5\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    loop 
         /*
     * Parse command line arguments
     *
     */
         {
        ch =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"p:s:S:\x00" as *const u8 as *const libc::c_char);
        if !(ch != -(1 as libc::c_int)) { break ; }
        match ch {
            112 => { port = atoi(optarg) as libc::c_short }
            115 => { service = optarg }
            83 => {
                retval = krb5_kt_resolve(context, optarg, &mut keytab);
                if retval != 0 {
                    com_err(*argv.offset(0 as libc::c_int as isize),
                            retval as errcode_t,
                            b"while resolving keytab file %s\x00" as *const u8
                                as *const libc::c_char, optarg);
                    exit(2 as libc::c_int);
                }
            }
            63 | _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
    }
    retval =
        krb5_sname_to_principal(context, 0 as *const libc::c_char, service,
                                3 as libc::c_int, &mut sprinc);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while generating service name %s\x00" as *const u8 as
                    *const libc::c_char, service);
        exit(1 as libc::c_int);
    }
    /* Set up server address */
    memset(&mut s_sock as *mut sockaddr_in as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
    s_sock.sin_family = 2 as libc::c_int as sa_family_t;
    s_sock.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    if port as libc::c_int == 0 as libc::c_int {
        /* Look up service */
        serv =
            getservbyname(b"sample\x00" as *const u8 as *const libc::c_char,
                          b"udp\x00" as *const u8 as *const libc::c_char);
        if serv.is_null() {
            fprintf(stderr,
                    b"service unknown: %s/udp\n\x00" as *const u8 as
                        *const libc::c_char,
                    b"sample\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        s_sock.sin_port = (*serv).s_port as in_port_t
    } else { s_sock.sin_port = htons(port as uint16_t) }
    /* Open socket */
    sock =
        socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if sock < 0 as libc::c_int {
        perror(b"opening datagram socket\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /* Let the socket be reused right away */
    setsockopt(sock, 1 as libc::c_int, 2 as libc::c_int,
               &mut on as *mut libc::c_int as *mut libc::c_char as
                   *const libc::c_void,
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                   socklen_t);
    /* Bind the socket */
    if bind(sock, &mut s_sock as *mut sockaddr_in as *mut sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) != 0 {
        perror(b"binding datagram socket\x00" as *const u8 as
                   *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"starting...\n\x00" as *const u8 as *const libc::c_char);
    /* GET KRB_AP_REQ MESSAGE */
    /* use "recvfrom" so we know client's address */
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    i =
        recvfrom(sock,
                 pktbuf.as_mut_ptr() as *mut libc::c_char as
                     *mut libc::c_void,
                 ::std::mem::size_of::<[libc::c_uchar; 8192]>() as
                     libc::c_ulong, flags,
                 &mut c_sock as *mut sockaddr_in as *mut sockaddr, &mut len)
            as libc::c_int;
    if i < 0 as libc::c_int {
        perror(b"receiving datagram\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"Received %d bytes\n\x00" as *const u8 as *const libc::c_char, i);
    packet.length = i as libc::c_uint;
    packet.data = pktbuf.as_mut_ptr() as krb5_pointer as *mut libc::c_char;
    /* Check authentication info */
    retval =
        krb5_rd_req(context, &mut auth_context, &mut packet,
                    sprinc as krb5_const_principal, keytab,
                    0 as *mut krb5_flags, &mut ticket);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while reading request\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        krb5_unparse_name(context,
                          (*(*ticket).enc_part2).client as
                              krb5_const_principal, &mut cp);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while unparsing client name\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"Got authentication info from %s\n\x00" as *const u8 as
               *const libc::c_char, cp);
    free(cp as *mut libc::c_void);
    /* Set foreign_addr for rd_safe() and rd_priv() */
    addr.addrtype = 0x2 as libc::c_int;
    addr.length =
        ::std::mem::size_of::<in_addr>() as libc::c_ulong as libc::c_uint;
    addr.contents = &mut c_sock.sin_addr as *mut in_addr as *mut krb5_octet;
    retval =
        krb5_auth_con_setaddrs(context, auth_context, 0 as *mut krb5_address,
                               &mut addr);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while setting foreign addr\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    addr.addrtype = 0x101 as libc::c_int;
    addr.length =
        ::std::mem::size_of::<in_port_t>() as libc::c_ulong as libc::c_uint;
    addr.contents = &mut c_sock.sin_port as *mut in_port_t as *mut krb5_octet;
    retval =
        krb5_auth_con_setports(context, auth_context, 0 as *mut krb5_address,
                               &mut addr);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while setting foreign port\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /* GET KRB_MK_SAFE MESSAGE */
    /* use "recvfrom" so we know client's address */
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    i =
        recvfrom(sock,
                 pktbuf.as_mut_ptr() as *mut libc::c_char as
                     *mut libc::c_void,
                 ::std::mem::size_of::<[libc::c_uchar; 8192]>() as
                     libc::c_ulong, flags,
                 &mut c_sock as *mut sockaddr_in as *mut sockaddr, &mut len)
            as libc::c_int;
    if i < 0 as libc::c_int {
        perror(b"receiving datagram\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"Received %d bytes\n\x00" as *const u8 as *const libc::c_char, i);
    packet.length = i as libc::c_uint;
    packet.data = pktbuf.as_mut_ptr() as krb5_pointer as *mut libc::c_char;
    retval =
        krb5_rd_safe(context, auth_context, &mut packet, &mut message,
                     0 as *mut krb5_replay_data);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while verifying SAFE message\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"Safe message is: \'%.*s\'\n\x00" as *const u8 as
               *const libc::c_char, message.length as libc::c_int,
           message.data);
    krb5_free_data_contents(context, &mut message);
    /* NOW GET ENCRYPTED MESSAGE */
    /* use "recvfrom" so we know client's address */
    len = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    i =
        recvfrom(sock,
                 pktbuf.as_mut_ptr() as *mut libc::c_char as
                     *mut libc::c_void,
                 ::std::mem::size_of::<[libc::c_uchar; 8192]>() as
                     libc::c_ulong, flags,
                 &mut c_sock as *mut sockaddr_in as *mut sockaddr, &mut len)
            as libc::c_int;
    if i < 0 as libc::c_int {
        perror(b"receiving datagram\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"Received %d bytes\n\x00" as *const u8 as *const libc::c_char, i);
    packet.length = i as libc::c_uint;
    packet.data = pktbuf.as_mut_ptr() as krb5_pointer as *mut libc::c_char;
    retval =
        krb5_rd_priv(context, auth_context, &mut packet, &mut message,
                     0 as *mut krb5_replay_data);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while verifying PRIV message\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(b"Decrypted message is: \'%.*s\'\n\x00" as *const u8 as
               *const libc::c_char, message.length as libc::c_int,
           message.data);
    krb5_auth_con_free(context, auth_context);
    krb5_free_context(context);
    exit(0 as libc::c_int);
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
