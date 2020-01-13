use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:34"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:34"]
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
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
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
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
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* *< Client time, seconds portion */
        /* *< Client time, microseconds portion */
        /* *< Subkey (optional) */
        /* *< Sequence number */
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
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
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4732:1"]
        pub fn krb5_free_ap_rep_enc_part(context: krb5_context,
                                         val: *mut krb5_ap_rep_enc_part);
        #[no_mangle]
        #[c2rust::src_loc = "4961:1"]
        pub fn krb5_sname_to_principal(context: krb5_context,
                                       hostname: *const libc::c_char,
                                       sname: *const libc::c_char,
                                       type_0: krb5_int32,
                                       ret_princ: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5391:1"]
        pub fn krb5_sendauth(context: krb5_context,
                             auth_context: *mut krb5_auth_context,
                             fd: krb5_pointer,
                             appl_version: *mut libc::c_char,
                             client: krb5_principal, server: krb5_principal,
                             ap_req_options: krb5_flags,
                             in_data: *mut krb5_data,
                             in_creds: *mut krb5_creds, ccache: krb5_ccache,
                             error: *mut *mut krb5_error,
                             rep_result: *mut *mut krb5_ap_rep_enc_part,
                             out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:34"]
pub mod com_err_h {
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
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
#[c2rust::header_src = "/usr/include/bits/socket.h:43"]
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
#[c2rust::header_src = "/usr/include/bits/socket_type.h:43"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:43"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/netdb.h:45"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "565:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    use super::socket_h::{socklen_t, sockaddr};
}
#[c2rust::header_src = "/usr/include/signal.h:46"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn signal(__sig: libc::c_int, __handler: __sighandler_t)
         -> __sighandler_t;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    }
}
#[c2rust::header_src = "/usr/include/string.h:38"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:40"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/socket.h:43"]
pub mod sys_socket_h {
    use super::socket_h::{sockaddr, socklen_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: *const sockaddr,
                       __len: socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:44"]
pub mod in_h {
    use super::stdint_uintn_h::uint16_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:46"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:46"]
pub mod fake_addrinfo_h {
    use super::socket_h::{sockaddr, socklen_t};
    use super::stddef_h::size_t;
    use super::netdb_h::addrinfo;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn krb5int_getnameinfo(sa: *const sockaddr, salen: socklen_t,
                                   hbuf: *mut libc::c_char, hbuflen: size_t,
                                   sbuf: *mut libc::c_char, sbuflen: size_t,
                                   flags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn krb5int_gai_strerror(err: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn krb5int_freeaddrinfo(ai: *mut addrinfo);
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn krb5int_getaddrinfo(node: *const libc::c_char,
                                   service: *const libc::c_char,
                                   hints: *const addrinfo,
                                   aip: *mut *mut addrinfo) -> libc::c_int;
    }
    /* FAI_DEFINED */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t, __off_t,
                        __off64_t, __ssize_t, __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_flags, krb5_timestamp, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_context, krb5_auth_context,
                       _krb5_keyblock, krb5_keyblock, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_creds, krb5_creds, _krb5_error, krb5_error,
                       _krb5_ap_rep_enc_part, krb5_ap_rep_enc_part,
                       krb5_ccache, _krb5_context, _krb5_auth_context,
                       _krb5_ccache, krb5_cc_close, krb5_cc_get_principal,
                       krb5_init_context, krb5_free_context, krb5_cc_default,
                       krb5_free_principal, krb5_free_ap_rep_enc_part,
                       krb5_sname_to_principal, krb5_sendauth,
                       krb5_auth_con_free};
pub use self::com_err_h::{errcode_t, com_err};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::socket_h::{socklen_t, sockaddr};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::netdb_h::addrinfo;
pub use self::signal_h::{__sighandler_t, signal};
use self::stdlib_h::{exit, free, malloc};
use self::stdio_h::{stderr, fprintf, printf};
use self::string_h::{memset, strncpy, strncat, strlen, strerror};
use self::errno_h::__errno_location;
use self::sys_socket_h::{socket, connect};
use self::in_h::ntohs;
use self::unistd_h::{close, read};
use self::fake_addrinfo_h::{krb5int_getnameinfo, krb5int_gai_strerror,
                            krb5int_freeaddrinfo, krb5int_getaddrinfo};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* appl/sample/sclient/sclient.c */
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
/*
 *
 * Sample Kerberos v5 client.
 *
 * Usage: sample_client hostname
 */
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn net_read(mut fd: libc::c_int, mut buf: *mut libc::c_char,
                              mut len: libc::c_int) -> libc::c_int {
    let mut cc: libc::c_int = 0;
    let mut len2: libc::c_int = 0 as libc::c_int;
    loop  {
        cc = read(fd, buf as *mut libc::c_void, len as size_t) as libc::c_int;
        if cc < 0 as libc::c_int {
            if !(*__errno_location() == 4 as libc::c_int) {
                /* XXX this interface sucks! */
                *__errno_location() = *__errno_location();
                return cc
            }
            /* errno is already set */
        } else if cc == 0 as libc::c_int {
            return len2
        } else { buf = buf.offset(cc as isize); len2 += cc; len -= cc }
        if !(len > 0 as libc::c_int) { break ; }
    }
    return len2;
}
#[c2rust::src_loc = "89:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ap: *mut addrinfo = 0 as *mut addrinfo;
    let mut aihints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut apstart: *mut addrinfo = 0 as *mut addrinfo;
    let mut aierr: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut recv_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut cksum_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut retval: krb5_error_code = 0;
    let mut ccdef: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut client: krb5_principal = 0 as *mut krb5_principal_data;
    let mut server: krb5_principal = 0 as *mut krb5_principal_data;
    let mut err_ret: *mut krb5_error = 0 as *mut krb5_error;
    let mut rep_ret: *mut krb5_ap_rep_enc_part =
        0 as *mut krb5_ap_rep_enc_part;
    let mut auth_context: krb5_auth_context = 0 as krb5_auth_context;
    let mut xmitlen: libc::c_short = 0;
    let mut portstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut service: *mut libc::c_char =
        b"sample\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    if argc != 2 as libc::c_int && argc != 3 as libc::c_int &&
           argc != 4 as libc::c_int {
        fprintf(stderr,
                b"usage: %s <hostname> [port] [service]\n\x00" as *const u8 as
                    *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize));
        exit(1 as libc::c_int);
    }
    retval = krb5_init_context(&mut context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while initializing krb5\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    signal(13 as libc::c_int,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1 as libc::c_int as
                                                       libc::intptr_t));
    if argc > 2 as libc::c_int {
        portstr = *argv.offset(2 as libc::c_int as isize)
    } else {
        portstr =
            b"sample\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    memset(&mut aihints as *mut addrinfo as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    aihints.ai_socktype = SOCK_STREAM as libc::c_int;
    aihints.ai_flags = 0x20 as libc::c_int;
    aierr =
        krb5int_getaddrinfo(*argv.offset(1 as libc::c_int as isize), portstr,
                            &mut aihints, &mut ap);
    if aierr != 0 {
        fprintf(stderr,
                b"%s: error looking up host \'%s\' port \'%s\'/tcp: %s\n\x00"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
                *argv.offset(1 as libc::c_int as isize), portstr,
                krb5int_gai_strerror(aierr));
        exit(1 as libc::c_int);
    }
    if ap.is_null() {
        /* Should never happen.  */
        fprintf(stderr,
                b"%s: error looking up host \'%s\' port \'%s\'/tcp: no addresses returned?\n\x00"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
                *argv.offset(1 as libc::c_int as isize), portstr);
        exit(1 as libc::c_int);
    }
    if argc > 3 as libc::c_int {
        service = *argv.offset(3 as libc::c_int as isize)
    }
    retval =
        krb5_sname_to_principal(context,
                                *argv.offset(1 as libc::c_int as isize),
                                service, 3 as libc::c_int, &mut server);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while creating server name for host %s service %s\x00" as
                    *const u8 as *const libc::c_char,
                *argv.offset(1 as libc::c_int as isize), service);
        exit(1 as libc::c_int);
    }
    /* set up the address of the foreign socket for connect() */
    apstart = ap; /* For freeing later */
    sock = -(1 as libc::c_int);
    while !ap.is_null() && sock == -(1 as libc::c_int) {
        let mut abuf: [libc::c_char; 1025] = [0; 1025];
        let mut pbuf: [libc::c_char; 32] = [0; 32];
        let mut mbuf: [libc::c_char; 1121] = [0; 1121];
        if krb5int_getnameinfo((*ap).ai_addr, (*ap).ai_addrlen,
                               abuf.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 1025]>()
                                   as libc::c_ulong, pbuf.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 32]>() as
                                   libc::c_ulong,
                               1 as libc::c_int | 2 as libc::c_int) != 0 {
            memset(abuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<[libc::c_char; 1025]>() as
                       libc::c_ulong);
            memset(pbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<[libc::c_char; 32]>() as
                       libc::c_ulong);
            strncpy(abuf.as_mut_ptr(),
                    b"[error, cannot print address?]\x00" as *const u8 as
                        *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 1025]>() as
                         libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong));
            strncpy(pbuf.as_mut_ptr(),
                    b"[?]\x00" as *const u8 as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 32]>() as
                         libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong));
        }
        memset(mbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[libc::c_char; 1121]>() as
                   libc::c_ulong);
        strncpy(mbuf.as_mut_ptr(),
                b"error contacting \x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 1121]>() as
                     libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                     libc::c_ulong));
        strncat(mbuf.as_mut_ptr(), abuf.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 1121]>() as
                     libc::c_ulong).wrapping_sub(strlen(mbuf.as_mut_ptr())).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong));
        strncat(mbuf.as_mut_ptr(),
                b" port \x00" as *const u8 as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 1121]>() as
                     libc::c_ulong).wrapping_sub(strlen(mbuf.as_mut_ptr())).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong));
        strncat(mbuf.as_mut_ptr(), pbuf.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 1121]>() as
                     libc::c_ulong).wrapping_sub(strlen(mbuf.as_mut_ptr())).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong));
        sock =
            socket((*ap).ai_family, SOCK_STREAM as libc::c_int,
                   0 as libc::c_int);
        if sock < 0 as libc::c_int {
            fprintf(stderr,
                    b"%s: socket: %s\n\x00" as *const u8 as
                        *const libc::c_char, mbuf.as_mut_ptr(),
                    strerror(*__errno_location()));
        } else if connect(sock, (*ap).ai_addr, (*ap).ai_addrlen) <
                      0 as libc::c_int {
            fprintf(stderr,
                    b"%s: connect: %s\n\x00" as *const u8 as
                        *const libc::c_char, mbuf.as_mut_ptr(),
                    strerror(*__errno_location()));
            close(sock);
            sock = -(1 as libc::c_int)
        }
        ap = (*ap).ai_next
        /* connected, yay! */
    }
    if sock == -(1 as libc::c_int) {
        /* Already printed error message above.  */
        exit(1 as libc::c_int); /* finished using it */
    }
    printf(b"connected\n\x00" as *const u8 as *const libc::c_char);
    cksum_data.data = *argv.offset(1 as libc::c_int as isize);
    cksum_data.length =
        strlen(*argv.offset(1 as libc::c_int as isize)) as libc::c_uint;
    retval = krb5_cc_default(context, &mut ccdef);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while getting default ccache\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval = krb5_cc_get_principal(context, ccdef, &mut client);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while getting client principal name\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        krb5_sendauth(context, &mut auth_context,
                      &mut sock as *mut libc::c_int as krb5_pointer,
                      b"KRB5_sample_protocol_v1.0\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, client,
                      server, 0x20000000 as libc::c_int, &mut cksum_data,
                      0 as *mut krb5_creds, ccdef, &mut err_ret, &mut rep_ret,
                      0 as *mut *mut krb5_creds);
    krb5_free_principal(context, server);
    krb5_free_principal(context, client);
    krb5_cc_close(context, ccdef);
    if !auth_context.is_null() { krb5_auth_con_free(context, auth_context); }
    if retval != 0 && retval as libc::c_long != -(1765328177 as libc::c_long)
       {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                b"while using sendauth\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if retval as libc::c_long == -(1765328177 as libc::c_long) {
        /* got an error */
        printf(b"sendauth rejected, error reply is:\n\t\"%*s\"\n\x00" as
                   *const u8 as *const libc::c_char, (*err_ret).text.length,
               (*err_ret).text.data);
    } else if !rep_ret.is_null() {
        /* got a reply */
        krb5_free_ap_rep_enc_part(context, rep_ret);
        printf(b"sendauth succeeded, reply is:\n\x00" as *const u8 as
                   *const libc::c_char);
        retval =
            net_read(sock,
                     &mut xmitlen as *mut libc::c_short as *mut libc::c_char,
                     ::std::mem::size_of::<libc::c_short>() as libc::c_ulong
                         as libc::c_int);
        if retval <= 0 as libc::c_int {
            if retval == 0 as libc::c_int {
                *__errno_location() = 103 as libc::c_int
            }
            com_err(*argv.offset(0 as libc::c_int as isize),
                    *__errno_location() as errcode_t,
                    b"while reading data from server\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        recv_data.length = ntohs(xmitlen as uint16_t) as libc::c_uint;
        recv_data.data =
            malloc((recv_data.length as
                        size_t).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong)) as
                *mut libc::c_char;
        if recv_data.data.is_null() {
            com_err(*argv.offset(0 as libc::c_int as isize),
                    12 as libc::c_int as errcode_t,
                    b"while allocating buffer to read from server\x00" as
                        *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        retval =
            net_read(sock, recv_data.data, recv_data.length as libc::c_int);
        if retval <= 0 as libc::c_int {
            if retval == 0 as libc::c_int {
                *__errno_location() = 103 as libc::c_int
            }
            com_err(*argv.offset(0 as libc::c_int as isize),
                    *__errno_location() as errcode_t,
                    b"while reading data from server\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        *recv_data.data.offset(recv_data.length as isize) =
            '\u{0}' as i32 as libc::c_char;
        printf(b"reply len %d, contents:\n%s\n\x00" as *const u8 as
                   *const libc::c_char, recv_data.length, recv_data.data);
        free(recv_data.data as *mut libc::c_void);
    } else {
        com_err(*argv.offset(0 as libc::c_int as isize),
                0 as libc::c_int as errcode_t,
                b"no error or reply from sendauth!\x00" as *const u8 as
                    *const libc::c_char);
        exit(1 as libc::c_int);
    }
    krb5int_freeaddrinfo(apstart);
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
