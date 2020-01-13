use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:3"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:3"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:3"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:3"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:3"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:11"]
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
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
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
    /* The name of the Kerberos ticket granting service... and its size */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Pointer to a prompter callback function. */
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
    /* * Store options for @c _krb5_get_init_creds */
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
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
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        /* *
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "4778:1"]
        pub fn krb5_free_string(context: krb5_context,
                                val: *mut libc::c_char);
        /* *
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5011:1"]
        pub fn krb5_change_password(context: krb5_context,
                                    creds: *mut krb5_creds,
                                    newpw: *const libc::c_char,
                                    result_code: *mut libc::c_int,
                                    result_code_string: *mut krb5_data,
                                    result_string: *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
        #[no_mangle]
        #[c2rust::src_loc = "5110:1"]
        pub fn krb5_chpw_message(context: krb5_context,
                                 server_string: *const krb5_data,
                                 message_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /*
 * end "func-proto.h"
 */
        /*
 * begin stuff from libos.h
 */
        /* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6096:1"]
        pub fn krb5_read_password(context: krb5_context,
                                  prompt: *const libc::c_char,
                                  prompt2: *const libc::c_char,
                                  return_pwd: *mut libc::c_char,
                                  size_return: *mut libc::c_uint)
         -> krb5_error_code;
        /* *
 * Prompt user for password.
 *
 * @param [in] context          Library context
 * @param      data             Unused (callback argument)
 * @param [in] name             Name to output during prompt
 * @param [in] banner           Banner to output during prompt
 * @param [in] num_prompts      Number of prompts in @a prompts
 * @param [in] prompts          Array of prompts and replies
 *
 * This function is intended to be used as a prompter callback for
 * krb5_get_init_creds_password() or krb5_init_creds_init().
 *
 * Writes @a name and @a banner to stdout, each followed by a newline, then
 * writes each prompt field in the @a prompts array, followed by ": ", and sets
 * the reply field of the entry to a line of input read from stdin.  If the
 * hidden flag is set for a prompt, then terminal echoing is turned off when
 * input is read.
 *
 * @retval
 *  0   Success
 * @return
 * Kerberos error codes
 *
 */
        #[no_mangle]
        #[c2rust::src_loc = "6461:1"]
        pub fn krb5_prompter_posix(context: krb5_context,
                                   data: *mut libc::c_void,
                                   name: *const libc::c_char,
                                   banner: *const libc::c_char,
                                   num_prompts: libc::c_int,
                                   prompts: *mut krb5_prompt)
         -> krb5_error_code;
        /* *
 * Allocate a new initial credential options structure.
 *
 * @param [in]  context         Library context
 * @param [out] opt             New options structure
 *
 * This function is the preferred way to create an options structure for
 * getting initial credentials, and is required to make use of certain options.
 * Use krb5_get_init_creds_opt_free() to free @a opt when it is no longer
 * needed.
 *
 * @retval 0 - Success; Kerberos errors otherwise.
 */
        #[no_mangle]
        #[c2rust::src_loc = "6851:1"]
        pub fn krb5_get_init_creds_opt_alloc(context: krb5_context,
                                             opt:
                                                 *mut *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        /* *
 * Free initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure to free
 *
 * @sa krb5_get_init_creds_opt_alloc()
 */
        #[no_mangle]
        #[c2rust::src_loc = "6863:1"]
        pub fn krb5_get_init_creds_opt_free(context: krb5_context,
                                            opt:
                                                *mut krb5_get_init_creds_opt);
        /* *
 * Set the ticket lifetime in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] tkt_life         Ticket lifetime
 */
        #[no_mangle]
        #[c2rust::src_loc = "6877:1"]
        pub fn krb5_get_init_creds_opt_set_tkt_life(opt:
                                                        *mut krb5_get_init_creds_opt,
                                                    tkt_life: krb5_deltat);
        /* *
 * Set the ticket renewal lifetime in initial credential options.
 *
 * @param [in] opt              Pointer to @a options field
 * @param [in] renew_life       Ticket renewal lifetime
 */
        #[no_mangle]
        #[c2rust::src_loc = "6887:1"]
        pub fn krb5_get_init_creds_opt_set_renew_life(opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      renew_life:
                                                          krb5_deltat);
        /* *
 * Set or unset the forwardable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] forwardable      Whether credentials should be forwardable
 */
        #[no_mangle]
        #[c2rust::src_loc = "6897:1"]
        pub fn krb5_get_init_creds_opt_set_forwardable(opt:
                                                           *mut krb5_get_init_creds_opt,
                                                       forwardable:
                                                           libc::c_int);
        /* *
 * Set or unset the proxiable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] proxiable        Whether credentials should be proxiable
 */
        #[no_mangle]
        #[c2rust::src_loc = "6907:1"]
        pub fn krb5_get_init_creds_opt_set_proxiable(opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     proxiable: libc::c_int);
        /* *
 * Set FAST armor cache in initial credential options.
 *
 * @param [in] context           Library context
 * @param [in] opt               Options
 * @param [in] ccache            Credential cache handle
 *
 * This function is similar to krb5_get_init_creds_opt_set_fast_ccache_name(),
 * but uses a credential cache handle instead of a name.
 *
 * @version New in 1.9
 */
        #[no_mangle]
        #[c2rust::src_loc = "7058:1"]
        pub fn krb5_get_init_creds_opt_set_fast_ccache(context: krb5_context,
                                                       opt:
                                                           *mut krb5_get_init_creds_opt,
                                                       ccache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Get initial credentials using a password.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  password        Password (or NULL)
 * @param [in]  prompter        Prompter function
 * @param [in]  data            Prompter callback data
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using @a
 * password.  If @a password is NULL, a password will be prompted for using @a
 * prompter if necessary.  If @a in_tkt_service is specified, it is parsed as a
 * principal name (with the realm ignored) and used as the service principal
 * for the request; otherwise the ticket-granting service is used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 *  0    Success
 * @retval
 *  EINVAL Invalid argument
 * @retval
 *  KRB5_KDC_UNREACH Cannot contact any KDC for requested realm
 * @retval
 *  KRB5_PREAUTH_FAILED Generic Pre-athentication failure
 * @retval
 *  KRB5_LIBOS_PWDINTR Password read interrupted
 * @retval
 *  KRB5_REALM_CANT_RESOLVE Cannot resolve network address for KDC in requested realm
 * @retval
 *  KRB5KDC_ERR_KEY_EXP Password has expired
 * @retval
 *  KRB5_LIBOS_BADPWDMATCH Password mismatch
 * @retval
 *  KRB5_CHPW_PWDNULL New password cannot be zero length
 * @retval
 *  KRB5_CHPW_FAIL Password change failed
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "7268:1"]
        pub fn krb5_get_init_creds_password(context: krb5_context,
                                            creds: *mut krb5_creds,
                                            client: krb5_principal,
                                            password: *const libc::c_char,
                                            prompter: krb5_prompter_fct,
                                            data: *mut libc::c_void,
                                            start_time: krb5_deltat,
                                            in_tkt_service:
                                                *const libc::c_char,
                                            k5_gic_options:
                                                *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:11"]
pub mod com_err_h {
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
#[c2rust::header_src = "/usr/include/pwd.h:17"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    use super::types_h::{__uid_t, __gid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn getpwuid(__uid: __uid_t) -> *mut passwd;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:3"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:3"]
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
#[c2rust::header_src = "/usr/include/stdio.h:3"]
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
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:3"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
    }
}
#[c2rust::header_src = "/usr/include/locale.h:4"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __int32_t, __uid_t, __gid_t, __off_t,
                        __off64_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_addrtype,
                       krb5_enctype, krb5_authdatatype, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_creds, krb5_creds,
                       krb5_ccache, _krb5_prompt, krb5_prompt,
                       krb5_prompter_fct, _krb5_get_init_creds_opt,
                       krb5_get_init_creds_opt, _krb5_context, _krb5_ccache,
                       krb5_cc_close, krb5_cc_get_principal,
                       krb5_init_context, krb5_parse_name, krb5_cc_default,
                       krb5_free_principal, krb5_free_string,
                       krb5_change_password, krb5_chpw_message,
                       krb5_read_password, krb5_prompter_posix,
                       krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_set_tkt_life,
                       krb5_get_init_creds_opt_set_renew_life,
                       krb5_get_init_creds_opt_set_forwardable,
                       krb5_get_init_creds_opt_set_proxiable,
                       krb5_get_init_creds_opt_set_fast_ccache,
                       krb5_get_init_creds_password};
pub use self::com_err_h::{errcode_t, com_err};
pub use self::pwd_h::{passwd, getpwuid};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, exit};
use self::stdio_h::{stderr, fprintf, printf};
use self::unistd_h::getuid;
use self::locale_h::setlocale;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
#[c2rust::src_loc = "19:1"]
unsafe extern "C" fn get_name_from_passwd_file(mut program_name:
                                                   *mut libc::c_char,
                                               mut context: krb5_context,
                                               mut me: *mut krb5_principal) {
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut ret: krb5_error_code = 0;
    pw = getpwuid(getuid());
    if !pw.is_null() {
        ret = krb5_parse_name(context, (*pw).pw_name, me);
        if ret != 0 {
            com_err(program_name, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"when parsing name %s\x00" as *const u8 as
                                 *const libc::c_char), (*pw).pw_name);
            exit(1 as libc::c_int);
        }
    } else {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Unable to identify user from password file\n\x00"
                             as *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    };
}
/* HAVE_PWD_H */
/* HAVE_PWD_H */
#[c2rust::src_loc = "47:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ret: krb5_error_code = 0;
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw: [libc::c_char; 1024] = [0; 1024];
    let mut ccache: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut opts: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
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
    let mut pwlen: libc::c_uint = 0;
    let mut result_code: libc::c_int = 0;
    let mut result_code_string: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut result_string: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    if argc > 2 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"usage: %s [principal]\n\x00" as *const u8 as
                             *const libc::c_char),
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    pname = *argv.offset(1 as libc::c_int as isize);
    ret = krb5_init_context(&mut context);
    if ret != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"initializing kerberos library\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    ret = krb5_get_init_creds_opt_alloc(context, &mut opts);
    if ret != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"allocating krb5_get_init_creds_opt\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /*
     * In order, use the first of:
     * - A name specified on the command line
     * - The principal name from an existing ccache
     * - The name corresponding to the ruid of the process
     *
     * Otherwise, it's an error.
     * We always attempt to open the default ccache in order to use FAST if
     * possible.
     */
    ret = krb5_cc_default(context, &mut ccache);
    if ret != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"opening default ccache\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    ret = krb5_cc_get_principal(context, ccache, &mut princ);
    if ret != 0 && ret as libc::c_long != -(1765328243 as libc::c_long) &&
           ret as libc::c_long != -(1765328189 as libc::c_long) {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"getting principal from ccache\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    } else {
        if !princ.is_null() {
            ret =
                krb5_get_init_creds_opt_set_fast_ccache(context, opts,
                                                        ccache);
            if ret != 0 {
                com_err(*argv.offset(0 as libc::c_int as isize),
                        ret as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while setting FAST ccache\x00" as *const u8
                                     as *const libc::c_char));
                exit(1 as libc::c_int);
            }
        }
    }
    ret = krb5_cc_close(context, ccache);
    if ret != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"closing ccache\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !pname.is_null() {
        krb5_free_principal(context, princ);
        princ = 0 as krb5_principal;
        ret = krb5_parse_name(context, pname, &mut princ);
        if ret != 0 {
            com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"parsing client name\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if princ.is_null() {
        get_name_from_passwd_file(*argv.offset(0 as libc::c_int as isize),
                                  context, &mut princ);
    }
    krb5_get_init_creds_opt_set_tkt_life(opts,
                                         5 as libc::c_int *
                                             60 as libc::c_int);
    krb5_get_init_creds_opt_set_renew_life(opts, 0 as libc::c_int);
    krb5_get_init_creds_opt_set_forwardable(opts, 0 as libc::c_int);
    krb5_get_init_creds_opt_set_proxiable(opts, 0 as libc::c_int);
    ret =
        krb5_get_init_creds_password(context, &mut creds, princ,
                                     0 as *const libc::c_char,
                                     Some(krb5_prompter_posix as
                                              unsafe extern "C" fn(_:
                                                                       krb5_context,
                                                                   _:
                                                                       *mut libc::c_void,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       *const libc::c_char,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut krb5_prompt)
                                                  -> krb5_error_code),
                                     0 as *mut libc::c_void, 0 as libc::c_int,
                                     b"kadmin/changepw\x00" as *const u8 as
                                         *const libc::c_char, opts);
    if ret != 0 {
        if ret as libc::c_long == -(1765328353 as libc::c_long) {
            com_err(*argv.offset(0 as libc::c_int as isize),
                    0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Password incorrect while getting initial ticket\x00"
                                 as *const u8 as *const libc::c_char));
        } else {
            com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"getting initial ticket\x00" as *const u8 as
                                 *const libc::c_char));
        }
        krb5_get_init_creds_opt_free(context, opts);
        exit(1 as libc::c_int);
    }
    pwlen =
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as
            libc::c_uint;
    ret =
        krb5_read_password(context,
                           dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Enter new password\x00" as *const u8 as
                                        *const libc::c_char),
                           dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Enter it again\x00" as *const u8 as
                                        *const libc::c_char), pw.as_mut_ptr(),
                           &mut pwlen);
    if ret != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while reading password\x00" as *const u8 as
                             *const libc::c_char));
        krb5_get_init_creds_opt_free(context, opts);
        exit(1 as libc::c_int);
    }
    ret =
        krb5_change_password(context, &mut creds, pw.as_mut_ptr(),
                             &mut result_code, &mut result_code_string,
                             &mut result_string);
    if ret != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"changing password\x00" as *const u8 as
                             *const libc::c_char));
        krb5_get_init_creds_opt_free(context, opts);
        exit(1 as libc::c_int);
    }
    if result_code != 0 {
        if krb5_chpw_message(context, &mut result_string, &mut message) !=
               0 as libc::c_int {
            message = 0 as *mut libc::c_char
        }
        printf(b"%.*s%s%s\n\x00" as *const u8 as *const libc::c_char,
               result_code_string.length as libc::c_int,
               result_code_string.data,
               if !message.is_null() {
                   b": \x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char },
               if !message.is_null() {
                   message
               } else { 0 as *mut libc::c_char });
        krb5_free_string(context, message);
        krb5_get_init_creds_opt_free(context, opts);
        exit(2 as libc::c_int);
    }
    free(result_string.data as *mut libc::c_void);
    free(result_code_string.data as *mut libc::c_void);
    krb5_get_init_creds_opt_free(context, opts);
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"Password changed.\n\x00" as *const u8 as
                        *const libc::c_char));
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
