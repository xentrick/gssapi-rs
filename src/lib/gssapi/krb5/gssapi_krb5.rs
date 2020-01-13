use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:82"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:82"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:82"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:82"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:82"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:82"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:82"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:82"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    /* Thread-specific data; implemented in a support file, because we'll
   need to keep track of some global data for cleanup purposes.

   Note that the callback function type is such that the C library
   routine free() is a valid callback.  */
    #[c2rust::src_loc = "401:9"]
    pub type k5_key_t = libc::c_uint;
    #[c2rust::src_loc = "410:5"]
    pub const K5_KEY_MAX: k5_key_t = 5;
    #[c2rust::src_loc = "406:5"]
    pub const K5_KEY_GSS_SPNEGO_STATUS: k5_key_t = 4;
    #[c2rust::src_loc = "405:5"]
    pub const K5_KEY_GSS_KRB5_ERROR_MESSAGE: k5_key_t = 3;
    #[c2rust::src_loc = "404:5"]
    pub const K5_KEY_GSS_KRB5_CCACHE_NAME: k5_key_t = 2;
    #[c2rust::src_loc = "403:5"]
    pub const K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME: k5_key_t = 1;
    #[c2rust::src_loc = "402:5"]
    pub const K5_KEY_COM_ERR: k5_key_t = 0;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
        /* rename shorthand symbols for export */
        #[no_mangle]
        #[c2rust::src_loc = "417:1"]
        pub fn krb5int_key_register(_: k5_key_t,
                                    _:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut libc::c_void)
                                                   -> ()>) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5int_getspecific(_: k5_key_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "419:1"]
        pub fn krb5int_setspecific(_: k5_key_t, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "420:1"]
        pub fn krb5int_key_delete(_: k5_key_t) -> libc::c_int;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:82"]
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
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
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
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
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
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
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
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2710:1"]
    pub type krb5_rcache = *mut krb5_rc_st;
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
    /*
 * end "rcache.h"
 */
    /*
 * begin "keytab.h"
 */
    /* XXX */
    /* *< Long enough for MAXPATHLEN + some extra */
    #[c2rust::src_loc = "2724:1"]
    pub type krb5_kt_cursor = krb5_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2727:16"]
    pub struct krb5_keytab_entry_st {
        pub magic: krb5_magic,
        pub principal: krb5_principal,
        pub timestamp: krb5_timestamp,
        pub vno: krb5_kvno,
        pub key: krb5_keyblock,
    }
    /* * A key table entry. */
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, krb5_key_st, _krb5_kt};
    extern "C" {
        /* *< Principal of this key */
        /* *< Time entry written to keytable */
        /* *< Key version number */
        /* *< The secret key */
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
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        /*
 * end "ccache.h"
 */
        /*
 * begin "rcache.h"
 */
        #[c2rust::src_loc = "2709:8"]
        pub type krb5_rc_st;
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
        /* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4403:1"]
        pub fn krb5_cc_set_default_name(context: krb5_context,
                                        name: *const libc::c_char)
         -> krb5_error_code;
        /* *
 * Convert a principal name to a local name.
 *
 * @param [in]  context         Library context
 * @param [in]  aname           Principal name
 * @param [in]  lnsize_in       Space available in @a lname
 * @param [out] lname           Local name buffer to be filled in
 *
 * If @a aname does not correspond to any local account, KRB5_LNAME_NOTRANS is
 * returned.  If @a lnsize_in is too small for the local name,
 * KRB5_CONFIG_NOTENUFSPACE is returned.
 *
 * Local names, rather than principal names, can be used by programs that
 * translate to an environment-specific name (for example, a user account
 * name).
 *
 * @retval
 * 0  Success
 * @retval
 *  System errors
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6124:1"]
        pub fn krb5_aname_to_localname(context: krb5_context,
                                       aname: krb5_const_principal,
                                       lnsize_in: libc::c_int,
                                       lname: *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Determine if a principal is authorized to log in as a local user.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] luser            Local username
 *
 * Determine whether @a principal is authorized to log in as a local user @a
 * luser.
 *
 * @retval
 * TRUE Principal is authorized to log in as user; FALSE otherwise.
 */
        #[no_mangle]
        #[c2rust::src_loc = "6200:1"]
        pub fn krb5_kuserok(context: krb5_context, principal: krb5_principal,
                            luser: *const libc::c_char) -> krb5_boolean;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:82"]
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
    /* could be used in a table to find an etype and initialize a block */
    /* internal message representations */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time,
                                           optional */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* data integrity checksum */
    /* encrypted part */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time, opt. */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /*
 * Begin "asn1.h"
 */
    /* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
/* here we use some knowledge of ASN.1 encodings */
/*
  Ticket is APPLICATION 1.
  Authenticator is APPLICATION 2.
  AS_REQ is APPLICATION 10.
  AS_REP is APPLICATION 11.
  TGS_REQ is APPLICATION 12.
  TGS_REP is APPLICATION 13.
  AP_REQ is APPLICATION 14.
  AP_REP is APPLICATION 15.
  KRB_SAFE is APPLICATION 20.
  KRB_PRIV is APPLICATION 21.
  KRB_CRED is APPLICATION 22.
  EncASRepPart is APPLICATION 25.
  EncTGSRepPart is APPLICATION 26.
  EncAPRepPart is APPLICATION 27.
  EncKrbPrivPart is APPLICATION 28.
  EncKrbCredPart is APPLICATION 29.
  KRB_ERROR is APPLICATION 30.
*/
/* allow either constructed or primitive encoding, so check for bit 6
   set or reset */
    /* ************************************************************************
 * Prototypes for krb5_encode.c
 *************************************************************************/
    /*
  krb5_error_code encode_krb5_structure(const krb5_structure *rep,
  krb5_data **code);
  modifies  *code
  effects   Returns the ASN.1 encoding of *rep in **code.
  Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
  Returns ENOMEM if memory runs out.
*/
    /* yes, the translation is identical to that used for KDC__REP */
    /* yes, the translation is identical to that used for KDC__REP */
    /* ************************************************************************
 * End of prototypes for krb5_encode.c
 *************************************************************************/
    /* ************************************************************************
 * Prototypes for krb5_decode.c
 *************************************************************************/
/*
  krb5_error_code decode_krb5_structure(const krb5_data *code,
  krb5_structure **rep);

  requires  Expects **rep to not have been allocated;
  a new *rep is allocated regardless of the old value.
  effects   Decodes *code into **rep.
  Returns ENOMEM if memory is exhausted.
  Returns asn1 and krb5 errors.
*/
    /* kdb.h */
    /* Master key version number */
    /* kvno of key_data elements (all the same) */
    /* ************************************************************************
 * End of prototypes for krb5_decode.c
 *************************************************************************/
    /* KRB5_ASN1__ */
    /*
 * End "asn1.h"
 */
    /*
 * Internal krb5 library routines
 */
    /* Return true if s is non-empty and composed solely of digits. */
    /*
 * Initialization routines.
 */
    /* [De]serialize 4-byte integer */
    /* [De]serialize 8-byte integer */
    /* [De]serialize byte string */
    /* Fill in the buffer with random alpha-numeric data. */
    /* value to use when requesting a keytab entry and KVNO doesn't matter */
    /* value to use when requesting a keytab entry and enctype doesn't matter */
    /* To keep happy libraries which are (for now) accessing internal stuff */
    /* Make sure to increment by one when changing the struct */
    /* Used for KDB LDAP back end.  */
    /*
     * pkinit asn.1 encode/decode functions
     */
    /* Set *tag_out to the integrity tag of *enc.  (Does not allocate memory;
 * returned buffer is a subrange of *ctext.) */
    /*
 * This structure was exposed and used in macros in krb5 1.2, so do not
 * change its ABI.
 */
    /* routines always present */
    /* routines to be included on extended version (write routines) */
    /* Not sure it's ready for exposure just yet.  */
    /*
 * Referral definitions and subfunctions.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2076:8"]
    pub struct _krb5_kt {
        pub magic: krb5_magic,
        pub ops: *const _krb5_kt_ops,
        pub data: krb5_pointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2040:16"]
    pub struct _krb5_kt_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *const libc::c_char,
                                                 _: *mut krb5_keytab)
                                -> krb5_error_code>,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_uint)
                                 -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_keytab)
                              -> krb5_error_code>,
        pub get: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: krb5_const_principal,
                                             _: krb5_kvno, _: krb5_enctype,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub start_seq_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_keytab,
                                                       _: *mut krb5_kt_cursor)
                                      -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut krb5_keytab_entry,
                                                  _: *mut krb5_kt_cursor)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_keytab,
                                                 _: *mut krb5_kt_cursor)
                                -> krb5_error_code>,
        pub add: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub remove: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_keytab,
                                                _: *mut krb5_keytab_entry)
                               -> krb5_error_code>,
    }
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
    #[c2rust::src_loc = "996:1"]
    pub type krb5_authdata_context = *mut _krb5_authdata_context;
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
                        krb5_keyblock, krb5_data, krb5_key, krb5_pointer,
                        krb5_error_code, krb5_context, krb5_keytab,
                        krb5_const_principal, krb5_kvno, krb5_keytab_entry,
                        krb5_kt_cursor, krb5_authdatatype};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:82"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:82"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:82"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:82"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct error_table {
        pub msgs: *const *const libc::c_char,
        pub base: libc::c_long,
        pub n_msgs: libc::c_uint,
    }
    extern "C" {
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn add_error_table(_: *const error_table) -> errcode_t;
        /*@modifies internalState@*/
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn remove_error_table(_: *const error_table) -> errcode_t;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:82"]
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
    #[c2rust::src_loc = "50:1"]
    pub type authdata_client_plugin_init_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
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
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_authdatatype,
                        krb5_octet, krb5_auth_context, krb5_keyblock,
                        krb5_ap_req, krb5_boolean, krb5_authdata,
                        krb5_const_principal, krb5_flags, krb5_data};
    use super::k5_int_h::_krb5_authdata_context;
    use super::stddef_h::size_t;
    /* KRB5_AUTHDATA_PLUGIN_H_INCLUDED */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:82"]
pub mod gssapi_h {
    /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
    /*
 * Determine platform-dependent configuration.
 */
    /* __cplusplus */
    /*
 * First, include stddef.h to get size_t defined.
 */
    /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
    /*
 * $Id$
 */
    /*
 * First, define the three platform-dependent pointer types.
 */
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    /* OM_STRING */
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    /*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    /* mech_set */
    /* XXXX these are not part of the GSSAPI C bindings!  (but should be) */
    /* XXXX This is a necessary evil until the spec is fixed */
    /*
 * RFC 5587
 */
    #[c2rust::src_loc = "845:1"]
    pub type gss_const_buffer_t = *const gss_buffer_desc;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /*
 * Flag bits for context-level services.
 */
        /*
 * Credential usage options
 */
        /*
 * Status code types for gss_display_status
 */
        /*
 * The constant definitions for channel-bindings address families
 */
        /*
 * Various Null values.
 */
        /*
 * Some alternate names for a couple of the above values.  These are defined
 * for V1 compatibility.
 */
        /*
 * Define the default Quality of Protection for per-message services.  Note
 * that an implementation that offers multiple levels of QOP may either reserve
 * a value (for example zero, as assumed here) to mean "default protection", or
 * alternatively may simply equate GSS_C_QOP_DEFAULT to a specific explicit
 * QOP value.  However a value of 0 should always be interpreted by a GSSAPI
 * implementation as a request for the default protection level.
 */
        /*
 * Expiration time of 2^32-1 seconds means infinite lifetime for a
 * credential or security context
 */
        /* Major status codes */
        /*
 * Some "helper" definitions to make the status code macros obvious.
 */
        /*
 * The macros that test status codes for error conditions.  Note that the
 * GSS_ERROR() macro has changed slightly from the V1 GSSAPI so that it now
 * evaluates its argument only once.
 */
        /*
 * Now the actual status code definitions
 */
        /*
 * Calling errors:
 */
        /*
 * Routine errors:
 */
        /*
 * Supplementary info bits:
 */
        /*
 * Finally, function prototypes for the GSSAPI routines.
 */
        /* Reserved static storage for GSS_oids.  Comments are quotes from RFC 2744.
 *
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x01"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) user_name(1)}.  The constant
 * GSS_C_NT_USER_NAME should be initialized to point
 * to that gss_OID_desc.
 */
        #[no_mangle]
        #[c2rust::src_loc = "336:27"]
        pub static mut GSS_C_NT_USER_NAME: gss_OID;
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "679:1"]
        pub fn gss_create_empty_oid_set(_: *mut OM_uint32,
                                        _: *mut gss_OID_set) -> OM_uint32;
        /* oid_set */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "685:1"]
        pub fn gss_add_oid_set_member(_: *mut OM_uint32, _: gss_OID,
                                      _: *mut gss_OID_set) -> OM_uint32;
        /* long_desc */
        #[no_mangle]
        #[c2rust::src_loc = "876:33"]
        pub static mut GSS_C_MA_MECH_CONCRETE: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "882:33"]
        pub static mut GSS_C_MA_DEPRECATED: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "883:33"]
        pub static mut GSS_C_MA_NOT_DFLT_MECH: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "884:33"]
        pub static mut GSS_C_MA_ITOK_FRAMED: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "885:33"]
        pub static mut GSS_C_MA_AUTH_INIT: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "886:33"]
        pub static mut GSS_C_MA_AUTH_TARG: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "887:33"]
        pub static mut GSS_C_MA_AUTH_INIT_INIT: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "891:33"]
        pub static mut GSS_C_MA_DELEG_CRED: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "892:33"]
        pub static mut GSS_C_MA_INTEG_PROT: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "893:33"]
        pub static mut GSS_C_MA_CONF_PROT: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "894:33"]
        pub static mut GSS_C_MA_MIC: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "895:33"]
        pub static mut GSS_C_MA_WRAP: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "896:33"]
        pub static mut GSS_C_MA_PROT_READY: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "897:33"]
        pub static mut GSS_C_MA_REPLAY_DET: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "898:33"]
        pub static mut GSS_C_MA_OOS_DET: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "899:33"]
        pub static mut GSS_C_MA_CBINDINGS: gss_const_OID;
        #[no_mangle]
        #[c2rust::src_loc = "902:33"]
        pub static mut GSS_C_MA_CTX_TRANS: gss_const_OID;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:83"]
pub mod mglueP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct gss_name_struct {
        pub loopback: *mut gss_name_struct,
        pub name_type: gss_OID,
        pub external_name: gss_buffer_t,
        pub mech_type: gss_OID,
        pub mech_name: gss_name_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:16"]
    pub struct gss_cred_id_struct {
        pub loopback: *mut gss_cred_id_struct,
        pub count: libc::c_int,
        pub mechs_array: gss_OID,
        pub cred_array: *mut gss_cred_id_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "745:16"]
    pub struct gss_mech_config {
        pub kmodName: *mut libc::c_char,
        pub uLibName: *mut libc::c_char,
        pub mechNameStr: *mut libc::c_char,
        pub optionStr: *mut libc::c_char,
        pub dl_handle: *mut libc::c_void,
        pub mech_type: gss_OID,
        pub mech: gss_mechanism,
        pub priority: libc::c_int,
        pub freeMech: libc::c_int,
        pub is_interposer: libc::c_int,
        pub int_mech_type: gss_OID,
        pub int_mech: gss_mechanism,
        pub next: *mut gss_mech_config,
    }
    /*
 * This is the definition of the mechs_array struct, which is used to
 * define the mechs array table. This table is used to indirectly
 * access mechanism specific versions of the gssapi routines through
 * the routines in the glue module (gssd_mech_glue.c)
 *
 * This contants all of the functions defined in gssapi.h except for
 * gss_release_buffer() and gss_release_oid_set(), which I am
 * assuming, for now, to be equal across mechanisms.
 */
    #[c2rust::src_loc = "95:1"]
    pub type gss_mechanism = *mut gss_config;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:16"]
    pub struct gss_config {
        pub mech_type: gss_OID_desc,
        pub context: *mut libc::c_void,
        pub gss_acquire_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: OM_uint32,
                                                          _: gss_OID_set,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut gss_cred_id_t,
                                                          _: *mut gss_OID_set,
                                                          _: *mut OM_uint32)
                                         -> OM_uint32>,
        pub gss_release_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _:
                                                              *mut gss_cred_id_t)
                                         -> OM_uint32>,
        pub gss_init_sec_context: Option<unsafe extern "C" fn(_:
                                                                  *mut OM_uint32,
                                                              _:
                                                                  gss_cred_id_t,
                                                              _:
                                                                  *mut gss_ctx_id_t,
                                                              _: gss_name_t,
                                                              _: gss_OID,
                                                              _: OM_uint32,
                                                              _: OM_uint32,
                                                              _:
                                                                  gss_channel_bindings_t,
                                                              _: gss_buffer_t,
                                                              _: *mut gss_OID,
                                                              _: gss_buffer_t,
                                                              _:
                                                                  *mut OM_uint32,
                                                              _:
                                                                  *mut OM_uint32)
                                             -> OM_uint32>,
        pub gss_accept_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_cred_id_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_channel_bindings_t,
                                                                _:
                                                                    *mut gss_name_t,
                                                                _:
                                                                    *mut gss_OID,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_cred_id_t)
                                               -> OM_uint32>,
        pub gss_process_context_token: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_ctx_id_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_delete_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_context_time: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_ctx_id_t,
                                                          _: *mut OM_uint32)
                                         -> OM_uint32>,
        pub gss_get_mic: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                     _: gss_ctx_id_t,
                                                     _: gss_qop_t,
                                                     _: gss_buffer_t,
                                                     _: gss_buffer_t)
                                    -> OM_uint32>,
        pub gss_verify_mic: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_ctx_id_t,
                                                        _: gss_buffer_t,
                                                        _: gss_buffer_t,
                                                        _: *mut gss_qop_t)
                                       -> OM_uint32>,
        pub gss_wrap: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                  _: gss_ctx_id_t,
                                                  _: libc::c_int,
                                                  _: gss_qop_t,
                                                  _: gss_buffer_t,
                                                  _: *mut libc::c_int,
                                                  _: gss_buffer_t)
                                 -> OM_uint32>,
        pub gss_unwrap: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                    _: gss_ctx_id_t,
                                                    _: gss_buffer_t,
                                                    _: gss_buffer_t,
                                                    _: *mut libc::c_int,
                                                    _: *mut gss_qop_t)
                                   -> OM_uint32>,
        pub gss_display_status: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: OM_uint32,
                                                            _: libc::c_int,
                                                            _: gss_OID,
                                                            _: *mut OM_uint32,
                                                            _: gss_buffer_t)
                                           -> OM_uint32>,
        pub gss_indicate_mechs: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _:
                                                                *mut gss_OID_set)
                                           -> OM_uint32>,
        pub gss_compare_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: gss_name_t,
                                                          _: *mut libc::c_int)
                                         -> OM_uint32>,
        pub gss_display_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: gss_buffer_t,
                                                          _: *mut gss_OID)
                                         -> OM_uint32>,
        pub gss_import_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_buffer_t,
                                                         _: gss_OID,
                                                         _: *mut gss_name_t)
                                        -> OM_uint32>,
        pub gss_release_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: *mut gss_name_t)
                                         -> OM_uint32>,
        pub gss_inquire_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_cred_id_t,
                                                          _: *mut gss_name_t,
                                                          _: *mut OM_uint32,
                                                          _: *mut libc::c_int,
                                                          _: *mut gss_OID_set)
                                         -> OM_uint32>,
        pub gss_add_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                      _: gss_cred_id_t,
                                                      _: gss_name_t,
                                                      _: gss_OID,
                                                      _: gss_cred_usage_t,
                                                      _: OM_uint32,
                                                      _: OM_uint32,
                                                      _: *mut gss_cred_id_t,
                                                      _: *mut gss_OID_set,
                                                      _: *mut OM_uint32,
                                                      _: *mut OM_uint32)
                                     -> OM_uint32>,
        pub gss_export_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_import_sec_context: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut gss_ctx_id_t)
                                               -> OM_uint32>,
        pub gss_inquire_cred_by_mech: Option<unsafe extern "C" fn(_:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      gss_cred_id_t,
                                                                  _: gss_OID,
                                                                  _:
                                                                      *mut gss_name_t,
                                                                  _:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut gss_cred_usage_t)
                                                 -> OM_uint32>,
        pub gss_inquire_names_for_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32>,
        pub gss_inquire_context: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32>,
        pub gss_internal_release_oid: Option<unsafe extern "C" fn(_:
                                                                      *mut OM_uint32,
                                                                  _:
                                                                      *mut gss_OID)
                                                 -> OM_uint32>,
        pub gss_wrap_size_limit: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32>,
        pub gss_localname: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                       _: gss_name_t,
                                                       _: gss_const_OID,
                                                       _: gss_buffer_t)
                                      -> OM_uint32>,
        pub gssspi_authorize_localname: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_name_t,
                                                                    _:
                                                                        gss_const_buffer_t,
                                                                    _:
                                                                        gss_const_OID)
                                                   -> OM_uint32>,
        pub gss_export_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_name_t,
                                                         _: gss_buffer_t)
                                        -> OM_uint32>,
        pub gss_duplicate_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_name_t,
                                                            _:
                                                                *mut gss_name_t)
                                           -> OM_uint32>,
        pub gss_store_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_cred_id_t,
                                                        _: gss_cred_usage_t,
                                                        _: gss_OID,
                                                        _: OM_uint32,
                                                        _: OM_uint32,
                                                        _: *mut gss_OID_set,
                                                        _:
                                                            *mut gss_cred_usage_t)
                                       -> OM_uint32>,
        pub gss_inquire_sec_context_by_oid: Option<unsafe extern "C" fn(_:
                                                                            *mut OM_uint32,
                                                                        _:
                                                                            gss_ctx_id_t,
                                                                        _:
                                                                            gss_OID,
                                                                        _:
                                                                            *mut gss_buffer_set_t)
                                                       -> OM_uint32>,
        pub gss_inquire_cred_by_oid: Option<unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_cred_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32>,
        pub gss_set_sec_context_option: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        *mut gss_ctx_id_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t)
                                                   -> OM_uint32>,
        pub gssspi_set_cred_option: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    *mut gss_cred_id_t,
                                                                _: gss_OID,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gssspi_mech_invoke: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_OID,
                                                            _: gss_OID,
                                                            _: gss_buffer_t)
                                           -> OM_uint32>,
        pub gss_wrap_aead: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                       _: gss_ctx_id_t,
                                                       _: libc::c_int,
                                                       _: gss_qop_t,
                                                       _: gss_buffer_t,
                                                       _: gss_buffer_t,
                                                       _: *mut libc::c_int,
                                                       _: gss_buffer_t)
                                      -> OM_uint32>,
        pub gss_unwrap_aead: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_ctx_id_t,
                                                         _: gss_buffer_t,
                                                         _: gss_buffer_t,
                                                         _: gss_buffer_t,
                                                         _: *mut libc::c_int,
                                                         _: *mut gss_qop_t)
                                        -> OM_uint32>,
        pub gss_wrap_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                      _: gss_ctx_id_t,
                                                      _: libc::c_int,
                                                      _: gss_qop_t,
                                                      _: *mut libc::c_int,
                                                      _:
                                                          *mut gss_iov_buffer_desc,
                                                      _: libc::c_int)
                                     -> OM_uint32>,
        pub gss_unwrap_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                        _: gss_ctx_id_t,
                                                        _: *mut libc::c_int,
                                                        _: *mut gss_qop_t,
                                                        _:
                                                            *mut gss_iov_buffer_desc,
                                                        _: libc::c_int)
                                       -> OM_uint32>,
        pub gss_wrap_iov_length: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32>,
        pub gss_complete_auth_token: Option<unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _:
                                                                     gss_buffer_t)
                                                -> OM_uint32>,
        pub gss_acquire_cred_impersonate_name: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_cred_id_t,
                                                                           _:
                                                                               gss_name_t,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               gss_OID_set,
                                                                           _:
                                                                               gss_cred_usage_t,
                                                                           _:
                                                                               *mut gss_cred_id_t,
                                                                           _:
                                                                               *mut gss_OID_set,
                                                                           _:
                                                                               *mut OM_uint32)
                                                          -> OM_uint32>,
        pub gss_add_cred_impersonate_name: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_cred_id_t,
                                                                       _:
                                                                           gss_cred_id_t,
                                                                       _:
                                                                           gss_name_t,
                                                                       _:
                                                                           gss_OID,
                                                                       _:
                                                                           gss_cred_usage_t,
                                                                       _:
                                                                           OM_uint32,
                                                                       _:
                                                                           OM_uint32,
                                                                       _:
                                                                           *mut gss_cred_id_t,
                                                                       _:
                                                                           *mut gss_OID_set,
                                                                       _:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           *mut OM_uint32)
                                                      -> OM_uint32>,
        pub gss_display_name_ext: Option<unsafe extern "C" fn(_:
                                                                  *mut OM_uint32,
                                                              _: gss_name_t,
                                                              _: gss_OID,
                                                              _: gss_buffer_t)
                                             -> OM_uint32>,
        pub gss_inquire_name: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                          _: gss_name_t,
                                                          _: *mut libc::c_int,
                                                          _: *mut gss_OID,
                                                          _:
                                                              *mut gss_buffer_set_t)
                                         -> OM_uint32>,
        pub gss_get_name_attribute: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _: gss_name_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    *mut libc::c_int,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> OM_uint32>,
        pub gss_set_name_attribute: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _: gss_name_t,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    gss_buffer_t,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gss_delete_name_attribute: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_export_name_composite: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32>,
        pub gss_map_name_to_any: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32>,
        pub gss_release_any_name_mapping: Option<unsafe extern "C" fn(_:
                                                                          *mut OM_uint32,
                                                                      _:
                                                                          gss_name_t,
                                                                      _:
                                                                          gss_buffer_t,
                                                                      _:
                                                                          *mut gss_any_t)
                                                     -> OM_uint32>,
        pub gss_pseudo_random: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                           _: gss_ctx_id_t,
                                                           _: libc::c_int,
                                                           _: gss_buffer_t,
                                                           _: ssize_t,
                                                           _: gss_buffer_t)
                                          -> OM_uint32>,
        pub gss_set_neg_mechs: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                           _: gss_cred_id_t,
                                                           _: gss_OID_set)
                                          -> OM_uint32>,
        pub gss_inquire_saslname_for_mech: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_OID,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           gss_buffer_t)
                                                      -> OM_uint32>,
        pub gss_inquire_mech_for_saslname: Option<unsafe extern "C" fn(_:
                                                                           *mut OM_uint32,
                                                                       _:
                                                                           gss_buffer_t,
                                                                       _:
                                                                           *mut gss_OID)
                                                      -> OM_uint32>,
        pub gss_inquire_attrs_for_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_const_OID,
                                                                    _:
                                                                        *mut gss_OID_set,
                                                                    _:
                                                                        *mut gss_OID_set)
                                                   -> OM_uint32>,
        pub gss_acquire_cred_from: Option<unsafe extern "C" fn(_:
                                                                   *mut OM_uint32,
                                                               _: gss_name_t,
                                                               _: OM_uint32,
                                                               _: gss_OID_set,
                                                               _:
                                                                   gss_cred_usage_t,
                                                               _:
                                                                   gss_const_key_value_set_t,
                                                               _:
                                                                   *mut gss_cred_id_t,
                                                               _:
                                                                   *mut gss_OID_set,
                                                               _:
                                                                   *mut OM_uint32)
                                              -> OM_uint32>,
        pub gss_store_cred_into: Option<unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32>,
        pub gssspi_acquire_cred_with_password: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_name_t,
                                                                           _:
                                                                               gss_buffer_t,
                                                                           _:
                                                                               OM_uint32,
                                                                           _:
                                                                               gss_OID_set,
                                                                           _:
                                                                               libc::c_int,
                                                                           _:
                                                                               *mut gss_cred_id_t,
                                                                           _:
                                                                               *mut gss_OID_set,
                                                                           _:
                                                                               *mut OM_uint32)
                                                          -> OM_uint32>,
        pub gss_export_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_cred_id_t,
                                                         _: gss_buffer_t)
                                        -> OM_uint32>,
        pub gss_import_cred: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_buffer_t,
                                                         _:
                                                             *mut gss_cred_id_t)
                                        -> OM_uint32>,
        pub gssspi_import_sec_context_by_mech: Option<unsafe extern "C" fn(_:
                                                                               *mut OM_uint32,
                                                                           _:
                                                                               gss_OID,
                                                                           _:
                                                                               gss_buffer_t,
                                                                           _:
                                                                               *mut gss_ctx_id_t)
                                                          -> OM_uint32>,
        pub gssspi_import_name_by_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        *mut gss_name_t)
                                                   -> OM_uint32>,
        pub gssspi_import_cred_by_mech: Option<unsafe extern "C" fn(_:
                                                                        *mut OM_uint32,
                                                                    _:
                                                                        gss_OID,
                                                                    _:
                                                                        gss_buffer_t,
                                                                    _:
                                                                        *mut gss_cred_id_t)
                                                   -> OM_uint32>,
        pub gss_get_mic_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                         _: gss_ctx_id_t,
                                                         _: gss_qop_t,
                                                         _:
                                                             *mut gss_iov_buffer_desc,
                                                         _: libc::c_int)
                                        -> OM_uint32>,
        pub gss_verify_mic_iov: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                                            _: gss_ctx_id_t,
                                                            _: *mut gss_qop_t,
                                                            _:
                                                                *mut gss_iov_buffer_desc,
                                                            _: libc::c_int)
                                           -> OM_uint32>,
        pub gss_get_mic_iov_length: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_ctx_id_t,
                                                                _: gss_qop_t,
                                                                _:
                                                                    *mut gss_iov_buffer_desc,
                                                                _:
                                                                    libc::c_int)
                                               -> OM_uint32>,
        pub gssspi_query_meta_data: Option<unsafe extern "C" fn(_:
                                                                    *mut OM_uint32,
                                                                _:
                                                                    gss_const_OID,
                                                                _:
                                                                    gss_cred_id_t,
                                                                _:
                                                                    *mut gss_ctx_id_t,
                                                                _: gss_name_t,
                                                                _: OM_uint32,
                                                                _:
                                                                    gss_buffer_t)
                                               -> OM_uint32>,
        pub gssspi_exchange_meta_data: Option<unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_const_OID,
                                                                   _:
                                                                       gss_cred_id_t,
                                                                   _:
                                                                       *mut gss_ctx_id_t,
                                                                   _:
                                                                       gss_name_t,
                                                                   _:
                                                                       OM_uint32,
                                                                   _:
                                                                       gss_const_buffer_t)
                                                  -> OM_uint32>,
        pub gssspi_query_mechanism_info: Option<unsafe extern "C" fn(_:
                                                                         *mut OM_uint32,
                                                                     _:
                                                                         gss_const_OID,
                                                                     _:
                                                                         *mut libc::c_uchar)
                                                    -> OM_uint32>,
    }
    /* minor_status */
    /* desired_name */
    /* time_req */
    /* desired_mechs */
    /* cred_usage */
    /* output_cred_handle */
    /* actual_mechs */
    /* time_rec */
    /* minor_status */
    /* cred_handle */
    /* minor_status */
    /* claimant_cred_handle */
    /* context_handle */
    /* target_name */
    /* mech_type */
    /* req_flags */
    /* time_req */
    /* input_chan_bindings */
    /* input_token */
    /* actual_mech_type */
    /* output_token */
    /* ret_flags */
    /* time_rec */
    /* minor_status */
    /* context_handle */
    /* verifier_cred_handle */
    /* input_token_buffer */
    /* input_chan_bindings */
    /* src_name */
    /* mech_type */
    /* output_token */
    /* ret_flags */
    /* time_rec */
    /* delegated_cred_handle */
    /* minor_status */
    /* context_handle */
    /* token_buffer */
    /* minor_status */
    /* context_handle */
    /* output_token */
    /* minor_status */
    /* context_handle */
    /* time_rec */
    /* minor_status */
    /* context_handle */
    /* qop_req */
    /* message_buffer */
    /* message_token */
    /* minor_status */
    /* context_handle */
    /* message_buffer */
    /* token_buffer */
    /* qop_state */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag */
    /* qop_req */
    /* input_message_buffer */
    /* conf_state */
    /* output_message_buffer */
    /* minor_status */
    /* context_handle */
    /* input_message_buffer */
    /* output_message_buffer */
    /* conf_state */
    /* qop_state */
    /* minor_status */
    /* status_value */
    /* status_type */
    /* mech_type */
    /* message_context */
    /* status_string */
    /* minor_status */
    /* mech_set */
    /* minor_status */
    /* name1 */
    /* name2 */
    /* name_equal */
    /* minor_status */
    /* input_name */
    /* output_name_buffer */
    /* output_name_type */
    /* minor_status */
    /* input_name_buffer */
    /* input_name_type */
    /* output_name */
    /* minor_status */
    /* input_name */
    /* minor_status */
    /* cred_handle */
    /* name */
    /* lifetime */
    /* cred_usage */
    /* mechanisms */
    /* minor_status */
    /* input_cred_handle */
    /* desired_name */
    /* desired_mech */
    /* cred_usage */
    /* initiator_time_req */
    /* acceptor_time_req */
    /* output_cred_handle */
    /* actual_mechs */
    /* initiator_time_rec */
    /* acceptor_time_rec */
    /* minor_status */
    /* context_handle */
    /* interprocess_token */
    /* minor_status */
    /* interprocess_token */
    /* context_handle */
    /* minor_status */
    /* cred_handle */
    /* mech_type */
    /* name */
    /* initiator_lifetime */
    /* acceptor_lifetime */
    /* cred_usage */
    /* minor_status */
    /* mechanism */
    /* name_types */
    /* minor_status */
    /* context_handle */
    /* src_name */
    /* targ_name */
    /* lifetime_rec */
    /* mech_type */
    /* ctx_flags */
    /* locally_initiated */
    /* open */
    /* minor_status */
    /* OID */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag */
    /* qop_req */
    /* req_output_size */
    /* max_input_size */
    /* minor */
    /* name */
    /* mech_type */
    /* localname */
    /* minor_status */
    /* pname */
    /* local user */
    /* local nametype */
    /* */
    /* minor_status */
    /* input_name */
    /* exported_name */
    /* */
    /* minor_status */
    /* input_name */
    /* output_name */
    /* */
    /* minor_status */
    /* input_cred */
    /* cred_usage */
    /* desired_mech */
    /* overwrite_cred */
    /* default_cred */
    /* elements_stored */
    /* cred_usage_stored */
    /* */
    /* GGF extensions */
    /* minor_status */
    /* context_handle */
    /* OID */
    /* data_set */
    /* minor_status */
    /* cred_handle */
    /* OID */
    /* data_set */
    /* minor_status */
    /* context_handle */
    /* OID */
    /* value */
    /* minor_status */
    /* cred_handle */
    /* OID */
    /* value */
    /* minor_status */
    /* mech OID */
    /* OID */
    /* value */
    /* AEAD extensions */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag */
    /* qop_req */
    /* input_assoc_buffer */
    /* input_payload_buffer */
    /* conf_state */
    /* output_message_buffer */
    /* */
    /* minor_status */
    /* context_handle */
    /* input_message_buffer */
    /* input_assoc_buffer */
    /* output_payload_buffer */
    /* conf_state */
    /* qop_state */
    /* */
    /* SSPI extensions */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag */
    /* qop_req */
    /* conf_state */
    /* iov */
    /* iov_count */
    /* */
    /* minor_status */
    /* context_handle */
    /* conf_state */
    /* qop_state */
    /* iov */
    /* iov_count */
    /* */
    /* minor_status */
    /* context_handle */
    /* conf_req_flag*/
    /* qop_req */
    /* conf_state */
    /* iov */
    /* iov_count */
    /* */
    /* minor_status */
    /* context_handle */
    /* input_message_buffer */
    /* New for 1.8 */
    /* minor_status */
    /* impersonator_cred_handle */
    /* desired_name */
    /* time_req */
    /* desired_mechs */
    /* cred_usage */
    /* output_cred_handle */
    /* actual_mechs */
    /* time_rec */
    /* */
    /* minor_status */
    /* input_cred_handle */
    /* impersonator_cred_handle */
    /* desired_name */
    /* desired_mech */
    /* cred_usage */
    /* initiator_time_req */
    /* acceptor_time_req */
    /* output_cred_handle */
    /* actual_mechs */
    /* initiator_time_rec */
    /* acceptor_time_rec */
    /* */
    /* minor_status */
    /* name */
    /* display_as_name_type */
    /* display_name */
    /* */
    /* minor_status */
    /* name */
    /* name_is_MN */
    /* MN_mech */
    /* attrs */
    /* */
    /* minor_status */
    /* name */
    /* attr */
    /* authenticated */
    /* complete */
    /* value */
    /* display_value */
    /* more */
    /* */
    /* minor_status */
    /* name */
    /* complete */
    /* attr */
    /* value */
    /* */
    /* minor_status */
    /* name */
    /* attr */
    /* */
    /* minor_status */
    /* name */
    /* exp_composite_name */
    /* */
    /* minor_status */
    /* name */
    /* authenticated */
    /* type_id */
    /* output */
    /* */
    /* minor_status */
    /* name */
    /* type_id */
    /* input */
    /* */
    /* minor_status */
    /* context */
    /* prf_key */
    /* prf_in */
    /* desired_output_len */
    /* prf_out */
    /* */
    /* minor_status */
    /* cred_handle */
    /* mech_set */
    /* */
    /* minor_status */
    /* desired_mech */
    /* sasl_mech_name */
    /* mech_name */
    /* mech_description */
    /* */
    /* minor_status */
    /* sasl_mech_name */
    /* mech_type */
    /* */
    /* minor_status */
    /* mech */
    /* mech_attrs */
    /* known_mech_attrs */
    /* */
    /* Credential store extensions */
    /* minor_status */
    /* desired_name */
    /* time_req */
    /* desired_mechs */
    /* cred_usage */
    /* cred_store */
    /* output_cred_handle */
    /* actual_mechs */
    /* time_rec */
    /* */
    /* minor_status */
    /* input_cred_handle */
    /* input_usage */
    /* desired_mech */
    /* overwrite_cred */
    /* default_cred */
    /* cred_store */
    /* elements_stored */
    /* cred_usage_stored */
    /* */
    /* minor_status */
    /* desired_name */
    /* password */
    /* time_req */
    /* desired_mechs */
    /* cred_usage */
    /* output_cred_handle */
    /* actual_mechs */
    /* time_rec */
    /* */
    /* minor_status */
    /* cred_handle */
    /* token */
    /* */
    /* minor_status */
    /* token */
    /* cred_handle */
    /* */
    /* minor_status */
    /* desired_mech */
    /* interprocess_token */
    /* context_handle */
    /* */
    /* minor_status */
    /* mech_type */
    /* input_name_buffer */
    /* input_name_type */
    /* output_name */
    /* */
    /* minor_status */
    /* mech_type */
    /* token */
    /* cred_handle */
    /* */
    /* get_mic_iov extensions, added in 1.12 */
    /* minor_status */
    /* context_handle */
    /* qop_req */
    /* iov */
    /* iov_count */
    /* minor_status */
    /* context_handle */
    /* qop_state */
    /* iov */
    /* iov_count */
    /* minor_status */
    /* context_handle */
    /* qop_req */
    /* iov */
    /* iov_count */
    /* NegoEx extensions added in 1.18 */
    /* minor_status */
    /* mech_oid */
    /* cred_handle */
    /* context_handle */
    /* targ_name */
    /* req_flags */
    /* meta_data */
    /* */
    /* minor_status */
    /* mech_oid */
    /* cred_handle */
    /* context_handle */
    /* targ_name */
    /* req_flags */
    /* meta_data */
    /* */
    /* minor_status */
    /* mech_oid */
    /* auth_scheme */
    /* */
    /*
 * In the user space we use a wrapper structure to encompass the
 * mechanism entry points.  The wrapper contain the mechanism
 * entry points and other data which is only relevant to the gss-api
 * layer.  In the kernel we use only the gss_config strucutre because
 * the kernal does not cantain any of the extra gss-api specific data.
 */
    #[c2rust::src_loc = "745:1"]
    pub type gss_mech_info = *mut gss_mech_config;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_OID_desc, OM_uint32, gss_OID_set, gss_ctx_id_t,
                          gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                          gss_const_OID, gss_const_buffer_t};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_const_key_value_set_t};
    use super::sys_types_h::ssize_t;
    extern "C" {
        /* kernel module name */
        /* user library name */
        /* mechanism string name */
        /* optional mech parameters */
        /* RTLD object handle for the mech */
        /* mechanism oid */
        /* mechanism initialization struct */
        /* mechanism preference order */
        /* free mech table */
        /* interposer mechanism flag */
        /* points to the interposer OID */
        /* points to the interposer mech */
        /* next element in the list */
        /*
 * Rudimentary pointer validation macro to check whether the
 * "loopback" field of an opaque struct points back to itself.  This
 * field also catches some programming errors where an opaque pointer
 * is passed to a function expecting the address of the opaque
 * pointer.
 */
        /* *******************************************************/
/* The Mechanism Dispatch Table -- a mechanism needs to */
/* define one of these and provide a function to return */
/* it to initialize the GSSAPI library		  */
        #[no_mangle]
        #[c2rust::src_loc = "80:1"]
        pub fn gssint_mechglue_initialize_library() -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "783:1"]
        pub fn gssint_register_mechinfo(template: gss_mech_info)
         -> libc::c_int;
    }
    /* _GSS_MECHGLUEP_H */
    /* Use this to map an errno value or com_err error code being
   generated within the mechglue code (e.g., by calling generic oid
   ops).  Any errno or com_err values produced by mech operations
   should be processed with map_error.  This means they'll be stored
   separately even if the mech uses com_err, because we can't assume
   that it will use com_err.  */
    /* Use this to map an error code that was returned from a mech
   operation; the mech will be asked to produce the associated error
   messages.

   Remember that if the minor status code cannot be returned to the
   caller (e.g., if it's stuffed in an automatic variable and then
   ignored), then we don't care about producing a mapping.  */
    /* qop_state */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:82"]
pub mod gssapi_ext_h {
    /* acceptor_time_rec */
    /*
 * GGF extensions
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    #[c2rust::src_loc = "488:1"]
    pub type gss_any_t = *mut gss_any;
    /* Credential store extensions */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "530:8"]
    pub struct gss_key_value_element_struct {
        pub key: *const libc::c_char,
        pub value: *const libc::c_char,
    }
    #[c2rust::src_loc = "534:1"]
    pub type gss_key_value_element_desc = gss_key_value_element_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:8"]
    pub struct gss_key_value_set_struct {
        pub count: OM_uint32,
        pub elements: *mut gss_key_value_element_desc,
    }
    #[c2rust::src_loc = "540:1"]
    pub type gss_key_value_set_desc = gss_key_value_set_struct;
    #[c2rust::src_loc = "541:1"]
    pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32};
    extern "C" {
        #[c2rust::src_loc = "488:16"]
        pub type gss_any;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:82"]
pub mod gssapiP_generic_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:9"]
    pub struct g_set {
        pub mutex: k5_mutex_t,
        pub data: *mut libc::c_void,
    }
    #[c2rust::src_loc = "129:1"]
    pub type g_seqnum_state = *mut g_seqnum_state_st;
    use super::k5_thread_h::k5_mutex_t;
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t};
    extern "C" {
        #[c2rust::src_loc = "129:16"]
        pub type g_seqnum_state_st;
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn gssint_g_make_string_buffer(str: *const libc::c_char,
                                           buffer: gss_buffer_t)
         -> libc::c_int;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:82"]
pub mod gssapiP_krb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "162:16"]
    pub struct _krb5_gss_name_rec {
        pub princ: krb5_principal,
        pub service: *mut libc::c_char,
        pub host: *mut libc::c_char,
        pub lock: k5_mutex_t,
        pub ad_context: krb5_authdata_context,
    }
    #[c2rust::src_loc = "162:1"]
    pub type krb5_gss_name_t = *mut _krb5_gss_name_rec;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "170:16"]
    pub struct _krb5_gss_cred_id_rec {
        pub lock: k5_mutex_t,
        pub usage: gss_cred_usage_t,
        pub name: krb5_gss_name_t,
        pub impersonator: krb5_principal,
        #[bitfield(name = "default_identity", ty = "libc::c_uint", bits =
                   "0..=0")]
        #[bitfield(name = "iakerb_mech", ty = "libc::c_uint", bits = "1..=1")]
        #[bitfield(name = "destroy_ccache", ty = "libc::c_uint", bits =
                   "2..=2")]
        #[bitfield(name = "suppress_ci_flags", ty = "libc::c_uint", bits =
                   "3..=3")]
        pub default_identity_iakerb_mech_destroy_ccache_suppress_ci_flags: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub keytab: krb5_keytab,
        pub rcache: krb5_rcache,
        pub ccache: krb5_ccache,
        pub client_keytab: krb5_keytab,
        pub have_tgt: krb5_boolean,
        pub expire: krb5_timestamp,
        pub refresh_time: krb5_timestamp,
        pub req_enctypes: *mut krb5_enctype,
        pub password: *mut libc::c_char,
    }
    #[c2rust::src_loc = "170:1"]
    pub type krb5_gss_cred_id_t = *mut _krb5_gss_cred_id_rec;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "204:16"]
    pub struct _krb5_gss_ctx_id_rec {
        pub magic: krb5_magic,
        #[bitfield(name = "initiate", ty = "libc::c_uint", bits = "0..=0")]
        #[bitfield(name = "established", ty = "libc::c_uint", bits = "1..=1")]
        #[bitfield(name = "have_acceptor_subkey", ty = "libc::c_uint", bits =
                   "2..=2")]
        #[bitfield(name = "seed_init", ty = "libc::c_uint", bits = "3..=3")]
        #[bitfield(name = "terminated", ty = "libc::c_uint", bits = "4..=4")]
        pub initiate_established_have_acceptor_subkey_seed_init_terminated: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
        pub gss_flags: OM_uint32,
        pub seed: [libc::c_uchar; 16],
        pub here: krb5_gss_name_t,
        pub there: krb5_gss_name_t,
        pub subkey: krb5_key,
        pub signalg: libc::c_int,
        pub cksum_size: size_t,
        pub sealalg: libc::c_int,
        pub enc: krb5_key,
        pub seq: krb5_key,
        pub krb_times: krb5_ticket_times,
        pub krb_flags: krb5_flags,
        pub seq_send: uint64_t,
        pub seq_recv: uint64_t,
        pub seqstate: g_seqnum_state,
        pub k5_context: krb5_context,
        pub auth_context: krb5_auth_context,
        pub mech_used: *mut gss_OID_desc,
        pub proto: libc::c_int,
        pub cksumtype: krb5_cksumtype,
        pub acceptor_subkey: krb5_key,
        pub acceptor_subkey_cksumtype: krb5_cksumtype,
        pub cred_rcache: libc::c_int,
        pub authdata: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "204:1"]
    pub type krb5_gss_ctx_id_rec = _krb5_gss_ctx_id_rec;
    use super::krb5_h::{krb5_principal, krb5_keytab, krb5_rcache, krb5_ccache,
                        krb5_boolean, krb5_timestamp, krb5_enctype,
                        krb5_magic, krb5_key, krb5_ticket_times, krb5_flags,
                        krb5_context, krb5_auth_context, krb5_cksumtype,
                        krb5_authdata, krb5_error_code};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::{krb5_authdata_context, _krb5_context};
    use super::gssapi_h::{gss_cred_usage_t, OM_uint32, gss_OID_desc,
                          gss_cred_id_t, gss_ctx_id_t, gss_name_t,
                          gss_OID_desc_struct, gss_OID,
                          gss_channel_bindings_struct, gss_channel_bindings_t,
                          gss_buffer_desc_struct, gss_buffer_t,
                          gss_ctx_id_struct, gss_qop_t,
                          gss_OID_set_desc_struct, gss_OID_set};
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::gssapiP_generic_h::g_seqnum_state;
    use super::mglueP_h::{gss_cred_id_struct, gss_name_struct};
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc,
                              gss_any_t, gss_key_value_set_desc,
                              gss_const_key_value_set_t};
    use super::thread_shared_types_h::{__pthread_mutex_s, __pthread_list_t,
                                       __pthread_internal_list};
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "1197:1"]
        pub fn krb5_gss_delete_error_info(p: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "1205:1"]
        pub fn iakerb_gss_init_sec_context(minor_status: *mut OM_uint32,
                                           claimant_cred_handle:
                                               gss_cred_id_t,
                                           context_handle: *mut gss_ctx_id_t,
                                           target_name: gss_name_t,
                                           mech_type: gss_OID,
                                           req_flags: OM_uint32,
                                           time_req: OM_uint32,
                                           input_chan_bindings:
                                               gss_channel_bindings_t,
                                           input_token: gss_buffer_t,
                                           actual_mech_type: *mut gss_OID,
                                           output_token: gss_buffer_t,
                                           ret_flags: *mut OM_uint32,
                                           time_rec: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1220:1"]
        pub fn iakerb_gss_accept_sec_context(minor_status: *mut OM_uint32,
                                             context_handler:
                                                 *mut gss_ctx_id_t,
                                             verifier_cred_handle:
                                                 gss_cred_id_t,
                                             input_token: gss_buffer_t,
                                             input_chan_bindings:
                                                 gss_channel_bindings_t,
                                             src_name: *mut gss_name_t,
                                             mech_type: *mut gss_OID,
                                             output_token: gss_buffer_t,
                                             ret_flags: *mut OM_uint32,
                                             time_rec: *mut OM_uint32,
                                             delegated_cred_handle:
                                                 *mut gss_cred_id_t)
         -> OM_uint32;
        /* minor_status */
        /* desired_name */
        /* time_req */
        /* desired_mechs */
        /* cred_usage */
        /* cred_store */
        /* output_cred_handle */
        /* actual_mechs */
        /* time_rec */
        /* minor_status */
        /* input_cred_handle */
        /* input_usage */
        /* desired_mech */
        /* overwrite_cred */
        /* default_cred */
        /* cred_store */
        /* elements_stored */
        /* cred_usage_stored */
        #[no_mangle]
        #[c2rust::src_loc = "1324:1"]
        pub fn iakerb_gss_process_context_token(minor_status: *mut OM_uint32,
                                                context_handle: gss_ctx_id_t,
                                                token_buffer: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1233:1"]
        pub fn iakerb_gss_delete_sec_context(minor_status: *mut OM_uint32,
                                             context_handle:
                                                 *mut gss_ctx_id_t,
                                             output_token: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1329:1"]
        pub fn iakerb_gss_context_time(minor_status: *mut OM_uint32,
                                       context_handle: gss_ctx_id_t,
                                       time_rec: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1340:1"]
        pub fn iakerb_gss_get_mic(minor_status: *mut OM_uint32,
                                  context_handle: gss_ctx_id_t,
                                  qop_req: gss_qop_t,
                                  message_buffer: gss_buffer_t,
                                  message_token: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1355:1"]
        pub fn iakerb_gss_verify_mic(minor_status: *mut OM_uint32,
                                     context_handle: gss_ctx_id_t,
                                     msg_buffer: gss_buffer_t,
                                     token_buffer: gss_buffer_t,
                                     qop_state: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1365:1"]
        pub fn iakerb_gss_wrap(minor_status: *mut OM_uint32,
                               context_handle: gss_ctx_id_t,
                               conf_req_flag: libc::c_int, qop_req: gss_qop_t,
                               input_message_buffer: gss_buffer_t,
                               conf_state: *mut libc::c_int,
                               output_message_buffer: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1382:1"]
        pub fn iakerb_gss_unwrap(minor_status: *mut OM_uint32,
                                 context_handle: gss_ctx_id_t,
                                 input_message_buffer: gss_buffer_t,
                                 output_message_buffer: gss_buffer_t,
                                 conf_state: *mut libc::c_int,
                                 qop_state: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1400:1"]
        pub fn iakerb_gss_export_sec_context(minor_status: *mut OM_uint32,
                                             context_handle:
                                                 *mut gss_ctx_id_t,
                                             interprocess_token: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1405:1"]
        pub fn iakerb_gss_import_sec_context(minor_status: *mut OM_uint32,
                                             interprocess_token: gss_buffer_t,
                                             context_handle:
                                                 *mut gss_ctx_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1333:1"]
        pub fn iakerb_gss_inquire_context(minor_status: *mut OM_uint32,
                                          context_handle: gss_ctx_id_t,
                                          src_name: *mut gss_name_t,
                                          targ_name: *mut gss_name_t,
                                          lifetime_rec: *mut OM_uint32,
                                          mech_type: *mut gss_OID,
                                          ctx_flags: *mut OM_uint32,
                                          locally_initiated: *mut libc::c_int,
                                          opened: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1393:1"]
        pub fn iakerb_gss_wrap_size_limit(minor_status: *mut OM_uint32,
                                          context_handle: gss_ctx_id_t,
                                          conf_req_flag: libc::c_int,
                                          qop_req: gss_qop_t,
                                          req_output_size: OM_uint32,
                                          max_input_size: *mut OM_uint32)
         -> OM_uint32;
        /* LEAN_CLIENT */
        #[no_mangle]
        #[c2rust::src_loc = "1411:1"]
        pub fn iakerb_gss_inquire_sec_context_by_oid(minor_status:
                                                         *mut OM_uint32,
                                                     context_handle:
                                                         gss_ctx_id_t,
                                                     desired_object: gss_OID,
                                                     data_set:
                                                         *mut gss_buffer_set_t)
         -> OM_uint32;
        /* Magic string to identify exported krb5 GSS credentials.  Increment this if
 * the format changes. */
        #[no_mangle]
        #[c2rust::src_loc = "1432:1"]
        pub fn gss_krb5int_get_cred_impersonator(minor_status: *mut OM_uint32,
                                                 cred_handle: gss_cred_id_t,
                                                 desired_object: gss_OID,
                                                 data_set:
                                                     *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "251:19"]
        pub static mut gssint_krb5_keytab_lock: k5_mutex_t;
        #[no_mangle]
        #[c2rust::src_loc = "1417:1"]
        pub fn iakerb_gss_set_sec_context_option(minor_status: *mut OM_uint32,
                                                 context_handle:
                                                     *mut gss_ctx_id_t,
                                                 desired_object: gss_OID,
                                                 value: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "505:1"]
        pub fn krb5_gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                     _: OM_uint32, _: gss_OID_set,
                                     _: gss_cred_usage_t,
                                     _: *mut gss_cred_id_t,
                                     _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1186:5"]
        pub fn krb5_gss_save_error_info(minor_code: OM_uint32,
                                        ctx: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "1105:1"]
        pub fn krb5_gss_init_context(ctxp: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "516:1"]
        pub fn iakerb_gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                       _: OM_uint32, _: gss_OID_set,
                                       _: gss_cred_usage_t,
                                       _: *mut gss_cred_id_t,
                                       _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "527:1"]
        pub fn krb5_gss_acquire_cred_with_password(minor_status:
                                                       *mut OM_uint32,
                                                   desired_name: gss_name_t,
                                                   password: gss_buffer_t,
                                                   time_req: OM_uint32,
                                                   desired_mechs: gss_OID_set,
                                                   cred_usage: libc::c_int,
                                                   output_cred_handle:
                                                       *mut gss_cred_id_t,
                                                   actual_mechs:
                                                       *mut gss_OID_set,
                                                   time_rec: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "539:1"]
        pub fn iakerb_gss_acquire_cred_with_password(minor_status:
                                                         *mut OM_uint32,
                                                     desired_name: gss_name_t,
                                                     password: gss_buffer_t,
                                                     time_req: OM_uint32,
                                                     desired_mechs:
                                                         gss_OID_set,
                                                     cred_usage: libc::c_int,
                                                     output_cred_handle:
                                                         *mut gss_cred_id_t,
                                                     actual_mechs:
                                                         *mut gss_OID_set,
                                                     time_rec: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn krb5_gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "556:1"]
        pub fn krb5_gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                         _: *mut gss_ctx_id_t, _: gss_name_t,
                                         _: gss_OID, _: OM_uint32,
                                         _: OM_uint32,
                                         _: gss_channel_bindings_t,
                                         _: gss_buffer_t, _: *mut gss_OID,
                                         _: gss_buffer_t, _: *mut OM_uint32,
                                         _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "592:1"]
        pub fn krb5_gss_accept_sec_context(_: *mut OM_uint32,
                                           _: *mut gss_ctx_id_t,
                                           _: gss_cred_id_t, _: gss_buffer_t,
                                           _: gss_channel_bindings_t,
                                           _: *mut gss_name_t,
                                           _: *mut gss_OID, _: gss_buffer_t,
                                           _: *mut OM_uint32,
                                           _: *mut OM_uint32,
                                           _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1147:1"]
        pub fn gss_krb5int_sec_context_sasl_ssf(_: *mut OM_uint32,
                                                _: gss_ctx_id_t, _: gss_OID,
                                                _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1139:1"]
        pub fn gss_krb5int_extract_authtime_from_sec_context(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1091:1"]
        pub fn gss_krb5int_export_lucid_sec_context(minor_status:
                                                        *mut OM_uint32,
                                                    context_handle:
                                                        gss_ctx_id_t,
                                                    desired_object: gss_OID,
                                                    data_set:
                                                        *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1071:1"]
        pub fn gss_krb5int_inq_session_key(_: *mut OM_uint32, _: gss_ctx_id_t,
                                           _: gss_OID,
                                           _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1124:1"]
        pub fn gss_krb5int_extract_authz_data_from_sec_context(minor_status:
                                                                   *mut OM_uint32,
                                                               context_handle:
                                                                   gss_ctx_id_t,
                                                               desired_object:
                                                                   gss_OID,
                                                               ad_data:
                                                                   *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1041:1"]
        pub fn gss_krb5int_get_tkt_flags(minor_status: *mut OM_uint32,
                                         context_handle: gss_ctx_id_t,
                                         desired_object: gss_OID,
                                         data_set: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn krb5_gss_process_context_token(_: *mut OM_uint32,
                                              _: gss_ctx_id_t,
                                              _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "645:1"]
        pub fn krb5_gss_delete_sec_context(_: *mut OM_uint32,
                                           _: *mut gss_ctx_id_t,
                                           _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "651:1"]
        pub fn krb5_gss_context_time(_: *mut OM_uint32, _: gss_ctx_id_t,
                                     _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "657:1"]
        pub fn krb5_gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                       _: libc::c_int, _: gss_OID,
                                       _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "666:1"]
        pub fn krb5_gss_indicate_mechs(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "671:1"]
        pub fn krb5_gss_compare_name(_: *mut OM_uint32, _: gss_name_t,
                                     _: gss_name_t, _: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn krb5_gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                     _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "686:1"]
        pub fn krb5_gss_import_name(_: *mut OM_uint32, _: gss_buffer_t,
                                    _: gss_OID, _: *mut gss_name_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "693:1"]
        pub fn krb5_gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "698:1"]
        pub fn krb5_gss_inquire_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                                     _: *mut gss_name_t, _: *mut OM_uint32,
                                     _: *mut gss_cred_usage_t,
                                     _: *mut gss_OID_set) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn krb5_gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                        _: *mut gss_name_t,
                                        _: *mut gss_name_t, _: *mut OM_uint32,
                                        _: *mut gss_OID, _: *mut OM_uint32,
                                        _: *mut libc::c_int,
                                        _: *mut libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "720:1"]
        pub fn krb5_gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                                _: gss_qop_t, _: gss_buffer_t,
                                _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "728:1"]
        pub fn krb5_gss_get_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                    _: gss_qop_t, _: *mut gss_iov_buffer_desc,
                                    _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "736:1"]
        pub fn krb5_gss_get_mic_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                           _: gss_qop_t,
                                           _: *mut gss_iov_buffer_desc,
                                           _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "744:1"]
        pub fn krb5_gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: gss_buffer_t, _: gss_buffer_t,
                                   _: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "752:1"]
        pub fn krb5_gss_verify_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                       _: *mut gss_qop_t,
                                       _: *mut gss_iov_buffer_desc,
                                       _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "760:1"]
        pub fn krb5_gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t,
                             _: libc::c_int, _: gss_qop_t, _: gss_buffer_t,
                             _: *mut libc::c_int, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn krb5_gss_wrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                 _: libc::c_int, _: gss_qop_t,
                                 _: *mut libc::c_int,
                                 _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "780:1"]
        pub fn krb5_gss_wrap_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                        _: libc::c_int, _: gss_qop_t,
                                        _: *mut libc::c_int,
                                        _: *mut gss_iov_buffer_desc,
                                        _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "790:1"]
        pub fn krb5_gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_buffer_t, _: gss_buffer_t,
                               _: *mut libc::c_int, _: *mut gss_qop_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "799:1"]
        pub fn krb5_gss_unwrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut libc::c_int, _: *mut gss_qop_t,
                                   _: *mut gss_iov_buffer_desc,
                                   _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "808:1"]
        pub fn krb5_gss_wrap_size_limit(_: *mut OM_uint32, _: gss_ctx_id_t,
                                        _: libc::c_int, _: gss_qop_t,
                                        _: OM_uint32, _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "831:1"]
        pub fn krb5_gss_inquire_cred_by_mech(_: *mut OM_uint32,
                                             _: gss_cred_id_t, _: gss_OID,
                                             _: *mut gss_name_t,
                                             _: *mut OM_uint32,
                                             _: *mut OM_uint32,
                                             _: *mut gss_cred_usage_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "841:1"]
        pub fn krb5_gss_export_sec_context(_: *mut OM_uint32,
                                           _: *mut gss_ctx_id_t,
                                           _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "847:1"]
        pub fn krb5_gss_import_sec_context(_: *mut OM_uint32, _: gss_buffer_t,
                                           _: *mut gss_ctx_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "859:1"]
        pub fn krb5_gss_internal_release_oid(_: *mut OM_uint32,
                                             _: *mut gss_OID) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "864:1"]
        pub fn krb5_gss_inquire_names_for_mech(_: *mut OM_uint32, _: gss_OID,
                                               _: *mut gss_OID_set)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "877:1"]
        pub fn krb5_gss_export_name(_: *mut OM_uint32, _: gss_name_t,
                                    _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "883:1"]
        pub fn krb5_gss_duplicate_name(_: *mut OM_uint32, _: gss_name_t,
                                       _: *mut gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "889:1"]
        pub fn krb5_gss_validate_cred(_: *mut OM_uint32, _: gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "894:1"]
        pub fn krb5_gss_acquire_cred_impersonate_name(_: *mut OM_uint32,
                                                      _: gss_cred_id_t,
                                                      _: gss_name_t,
                                                      _: OM_uint32,
                                                      _: gss_OID_set,
                                                      _: gss_cred_usage_t,
                                                      _: *mut gss_cred_id_t,
                                                      _: *mut gss_OID_set,
                                                      _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "959:1"]
        pub fn krb5_gss_inquire_name(minor_status: *mut OM_uint32,
                                     name: gss_name_t,
                                     name_is_MN: *mut libc::c_int,
                                     MN_mech: *mut gss_OID,
                                     attrs: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "966:1"]
        pub fn krb5_gss_get_name_attribute(minor_status: *mut OM_uint32,
                                           name: gss_name_t,
                                           attr: gss_buffer_t,
                                           authenticated: *mut libc::c_int,
                                           complete: *mut libc::c_int,
                                           value: gss_buffer_t,
                                           display_value: gss_buffer_t,
                                           more: *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "976:1"]
        pub fn krb5_gss_set_name_attribute(minor_status: *mut OM_uint32,
                                           name: gss_name_t,
                                           complete: libc::c_int,
                                           attr: gss_buffer_t,
                                           value: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "983:1"]
        pub fn krb5_gss_delete_name_attribute(minor_status: *mut OM_uint32,
                                              name: gss_name_t,
                                              attr: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "988:1"]
        pub fn krb5_gss_export_name_composite(minor_status: *mut OM_uint32,
                                              name: gss_name_t,
                                              exp_composite_name:
                                                  gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "993:1"]
        pub fn krb5_gss_map_name_to_any(minor_status: *mut OM_uint32,
                                        name: gss_name_t,
                                        authenticated: libc::c_int,
                                        type_id: gss_buffer_t,
                                        output: *mut gss_any_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1000:1"]
        pub fn krb5_gss_release_any_name_mapping(minor_status: *mut OM_uint32,
                                                 name: gss_name_t,
                                                 type_id: gss_buffer_t,
                                                 input: *mut gss_any_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1006:1"]
        pub fn krb5_gss_pseudo_random(minor_status: *mut OM_uint32,
                                      context: gss_ctx_id_t,
                                      prf_key: libc::c_int,
                                      prf_in: gss_buffer_t,
                                      desired_output_len: ssize_t,
                                      prf_out: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1014:1"]
        pub fn krb5_gss_store_cred(minor_status: *mut OM_uint32,
                                   input_cred_handle: gss_cred_id_t,
                                   cred_usage: gss_cred_usage_t,
                                   desired_mech: gss_OID,
                                   overwrite_cred: OM_uint32,
                                   default_cred: OM_uint32,
                                   elements_stored: *mut gss_OID_set,
                                   cred_usage_stored: *mut gss_cred_usage_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1050:1"]
        pub fn gss_krb5int_copy_ccache(minor_status: *mut OM_uint32,
                                       cred_handle: *mut gss_cred_id_t,
                                       desired_oid: gss_OID,
                                       value: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1064:1"]
        pub fn gss_krb5int_ccache_name(minor_status: *mut OM_uint32,
                                       _: gss_OID, _: gss_OID,
                                       _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1082:1"]
        pub fn gss_krb5int_set_allowable_enctypes(minor_status:
                                                      *mut OM_uint32,
                                                  cred: *mut gss_cred_id_t,
                                                  desired_oid: gss_OID,
                                                  value: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1100:1"]
        pub fn gss_krb5int_free_lucid_sec_context(_: *mut OM_uint32,
                                                  _: gss_OID, _: gss_OID,
                                                  _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1104:19"]
        pub static mut kg_kdc_flag_mutex: k5_mutex_t;
        #[no_mangle]
        #[c2rust::src_loc = "1110:1"]
        pub fn krb5int_gss_use_kdc_context(_: *mut OM_uint32, _: gss_OID,
                                           _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1118:1"]
        pub fn gss_krb5int_register_acceptor_identity(_: *mut OM_uint32,
                                                      _: gss_OID, _: gss_OID,
                                                      _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1133:1"]
        pub fn gss_krb5int_set_cred_rcache(_: *mut OM_uint32,
                                           _: *mut gss_cred_id_t, _: gss_OID,
                                           _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1160:1"]
        pub fn gss_krb5int_import_cred(minor_status: *mut OM_uint32,
                                       cred: *mut gss_cred_id_t,
                                       desired_oid: gss_OID,
                                       value: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1350:1"]
        pub fn iakerb_gss_get_mic_iov_length(minor_status: *mut OM_uint32,
                                             context_handle: gss_ctx_id_t,
                                             qop_req: gss_qop_t,
                                             iov: *mut gss_iov_buffer_desc,
                                             iov_count: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1360:1"]
        pub fn iakerb_gss_verify_mic_iov(minor_status: *mut OM_uint32,
                                         context_handle: gss_ctx_id_t,
                                         qop_state: *mut gss_qop_t,
                                         iov: *mut gss_iov_buffer_desc,
                                         iov_count: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1345:1"]
        pub fn iakerb_gss_get_mic_iov(minor_status: *mut OM_uint32,
                                      context_handle: gss_ctx_id_t,
                                      qop_req: gss_qop_t,
                                      iov: *mut gss_iov_buffer_desc,
                                      iov_count: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1320:1"]
        pub fn krb5_gss_import_cred(minor_status: *mut OM_uint32,
                                    token: gss_buffer_t,
                                    cred_handle: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1316:1"]
        pub fn krb5_gss_export_cred(minor_status: *mut OM_uint32,
                                    cred_handle: gss_cred_id_t,
                                    token: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1304:1"]
        pub fn krb5_gss_store_cred_into(_: *mut OM_uint32, _: gss_cred_id_t,
                                        _: gss_cred_usage_t, _: gss_OID,
                                        _: OM_uint32, _: OM_uint32,
                                        _: gss_const_key_value_set_t,
                                        _: *mut gss_OID_set,
                                        _: *mut gss_cred_usage_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1292:1"]
        pub fn krb5_gss_acquire_cred_from(_: *mut OM_uint32, _: gss_name_t,
                                          _: OM_uint32, _: gss_OID_set,
                                          _: gss_cred_usage_t,
                                          _: gss_const_key_value_set_t,
                                          _: *mut gss_cred_id_t,
                                          _: *mut gss_OID_set,
                                          _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1371:1"]
        pub fn iakerb_gss_wrap_iov(minor_status: *mut OM_uint32,
                                   context_handle: gss_ctx_id_t,
                                   conf_req_flag: libc::c_int,
                                   qop_req: gss_qop_t,
                                   conf_state: *mut libc::c_int,
                                   iov: *mut gss_iov_buffer_desc,
                                   iov_count: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1388:1"]
        pub fn iakerb_gss_unwrap_iov(minor_status: *mut OM_uint32,
                                     context_handle: gss_ctx_id_t,
                                     conf_state: *mut libc::c_int,
                                     qop_state: *mut gss_qop_t,
                                     iov: *mut gss_iov_buffer_desc,
                                     iov_count: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1376:1"]
        pub fn iakerb_gss_wrap_iov_length(minor_status: *mut OM_uint32,
                                          context_handle: gss_ctx_id_t,
                                          conf_req_flag: libc::c_int,
                                          qop_req: gss_qop_t,
                                          conf_state: *mut libc::c_int,
                                          iov: *mut gss_iov_buffer_desc,
                                          iov_count: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1423:1"]
        pub fn iakerb_gss_pseudo_random(minor_status: *mut OM_uint32,
                                        context_handle: gss_ctx_id_t,
                                        prf_key: libc::c_int,
                                        prf_in: gss_buffer_t,
                                        desired_output_len: ssize_t,
                                        prf_out: gss_buffer_t) -> OM_uint32;
    }
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:82"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:82"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:82"]
pub mod gssapi_alloc_h {
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]
    #[c2rust::src_loc = "119:1"]
    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const libc::c_char)
     -> *mut libc::c_char {
        let mut size: size_t =
            strlen(str).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut copy: *mut libc::c_char =
            gssalloc_malloc(size) as *mut libc::c_char;
        if !copy.is_null() {
            memcpy(copy as *mut libc::c_void, str as *const libc::c_void,
                   size);
            *copy.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                             as isize) = '\u{0}' as i32 as libc::c_char
        }
        return copy;
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::malloc;
    use super::string_h::{strlen, memcpy};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapi_err_krb5.h:82"]
pub mod gssapi_err_krb5_h {
    use super::com_err_h::error_table;
    extern "C" {
        /*
 * et-h-gssapi_err_krb5.h:
 * This file is automatically generated; please do not edit it.
 */
        #[no_mangle]
        #[c2rust::src_loc = "27:33"]
        pub static et_k5g_error_table: error_table;
    }
    /*@modifies internalState@*/
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __uint64_t,
                        __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_key_t, K5_KEY_MAX,
                            K5_KEY_GSS_SPNEGO_STATUS,
                            K5_KEY_GSS_KRB5_ERROR_MESSAGE,
                            K5_KEY_GSS_KRB5_CCACHE_NAME,
                            K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                            K5_KEY_COM_ERR, k5_mutex_finish_init,
                            k5_os_mutex_destroy, krb5int_key_register,
                            krb5int_getspecific, krb5int_setspecific,
                            krb5int_key_delete};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_auth_context,
                       _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_ap_req, krb5_ap_req, krb5_ccache, krb5_rcache,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       _krb5_auth_context, _krb5_ccache, krb5_rc_st,
                       krb5_free_context, krb5_cc_default_name,
                       krb5_cc_set_default_name, krb5_aname_to_localname,
                       krb5_kuserok};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_key_st, derived_key, _krb5_kt,
                         _krb5_kt_ops, _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_authdata_context,
                         k5calloc, k5alloc, k5memdup0, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, error_table, add_error_table,
                          remove_error_table};
pub use self::authdata_plugin_h::{authdata_client_plugin_init_proc,
                                  authdata_client_request_fini_proc,
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
                                  authdata_client_plugin_fini_proc};
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_name_t, gss_OID,
                         gss_OID_desc_struct, gss_buffer_t,
                         gss_buffer_desc_struct, gss_cred_id_t, gss_ctx_id_t,
                         gss_OID_desc, gss_OID_set_desc_struct,
                         gss_OID_set_desc, gss_OID_set, gss_buffer_desc,
                         gss_channel_bindings_struct, gss_channel_bindings_t,
                         gss_qop_t, gss_cred_usage_t, gss_const_buffer_t,
                         gss_const_OID, gss_ctx_id_struct, GSS_C_NT_USER_NAME,
                         gss_release_oid_set, gss_create_empty_oid_set,
                         gss_add_oid_set_member, GSS_C_MA_MECH_CONCRETE,
                         GSS_C_MA_DEPRECATED, GSS_C_MA_NOT_DFLT_MECH,
                         GSS_C_MA_ITOK_FRAMED, GSS_C_MA_AUTH_INIT,
                         GSS_C_MA_AUTH_TARG, GSS_C_MA_AUTH_INIT_INIT,
                         GSS_C_MA_DELEG_CRED, GSS_C_MA_INTEG_PROT,
                         GSS_C_MA_CONF_PROT, GSS_C_MA_MIC, GSS_C_MA_WRAP,
                         GSS_C_MA_PROT_READY, GSS_C_MA_REPLAY_DET,
                         GSS_C_MA_OOS_DET, GSS_C_MA_CBINDINGS,
                         GSS_C_MA_CTX_TRANS};
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct, gss_mech_config,
                         gss_mechanism, gss_config, gss_mech_info,
                         gssint_mechglue_initialize_library,
                         gssint_register_mechinfo};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_iov_buffer_desc_struct, gss_iov_buffer_desc,
                             gss_any_t, gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t, gss_any};
pub use self::gssapiP_generic_h::{g_set, g_seqnum_state, g_seqnum_state_st,
                                  gssint_g_make_string_buffer};
pub use self::gssapiP_krb5_h::{_krb5_gss_name_rec, krb5_gss_name_t,
                               _krb5_gss_cred_id_rec, krb5_gss_cred_id_t,
                               _krb5_gss_ctx_id_rec, krb5_gss_ctx_id_rec,
                               krb5_gss_delete_error_info,
                               iakerb_gss_init_sec_context,
                               iakerb_gss_accept_sec_context,
                               iakerb_gss_process_context_token,
                               iakerb_gss_delete_sec_context,
                               iakerb_gss_context_time, iakerb_gss_get_mic,
                               iakerb_gss_verify_mic, iakerb_gss_wrap,
                               iakerb_gss_unwrap,
                               iakerb_gss_export_sec_context,
                               iakerb_gss_import_sec_context,
                               iakerb_gss_inquire_context,
                               iakerb_gss_wrap_size_limit,
                               iakerb_gss_inquire_sec_context_by_oid,
                               gss_krb5int_get_cred_impersonator,
                               gssint_krb5_keytab_lock,
                               iakerb_gss_set_sec_context_option,
                               krb5_gss_acquire_cred,
                               krb5_gss_save_error_info,
                               krb5_gss_init_context, iakerb_gss_acquire_cred,
                               krb5_gss_acquire_cred_with_password,
                               iakerb_gss_acquire_cred_with_password,
                               krb5_gss_release_cred,
                               krb5_gss_init_sec_context,
                               krb5_gss_accept_sec_context,
                               gss_krb5int_sec_context_sasl_ssf,
                               gss_krb5int_extract_authtime_from_sec_context,
                               gss_krb5int_export_lucid_sec_context,
                               gss_krb5int_inq_session_key,
                               gss_krb5int_extract_authz_data_from_sec_context,
                               gss_krb5int_get_tkt_flags,
                               krb5_gss_process_context_token,
                               krb5_gss_delete_sec_context,
                               krb5_gss_context_time, krb5_gss_display_status,
                               krb5_gss_indicate_mechs, krb5_gss_compare_name,
                               krb5_gss_display_name, krb5_gss_import_name,
                               krb5_gss_release_name, krb5_gss_inquire_cred,
                               krb5_gss_inquire_context, krb5_gss_get_mic,
                               krb5_gss_get_mic_iov,
                               krb5_gss_get_mic_iov_length,
                               krb5_gss_verify_mic, krb5_gss_verify_mic_iov,
                               krb5_gss_wrap, krb5_gss_wrap_iov,
                               krb5_gss_wrap_iov_length, krb5_gss_unwrap,
                               krb5_gss_unwrap_iov, krb5_gss_wrap_size_limit,
                               krb5_gss_inquire_cred_by_mech,
                               krb5_gss_export_sec_context,
                               krb5_gss_import_sec_context,
                               krb5_gss_internal_release_oid,
                               krb5_gss_inquire_names_for_mech,
                               krb5_gss_export_name, krb5_gss_duplicate_name,
                               krb5_gss_validate_cred,
                               krb5_gss_acquire_cred_impersonate_name,
                               krb5_gss_inquire_name,
                               krb5_gss_get_name_attribute,
                               krb5_gss_set_name_attribute,
                               krb5_gss_delete_name_attribute,
                               krb5_gss_export_name_composite,
                               krb5_gss_map_name_to_any,
                               krb5_gss_release_any_name_mapping,
                               krb5_gss_pseudo_random, krb5_gss_store_cred,
                               gss_krb5int_copy_ccache,
                               gss_krb5int_ccache_name,
                               gss_krb5int_set_allowable_enctypes,
                               gss_krb5int_free_lucid_sec_context,
                               kg_kdc_flag_mutex, krb5int_gss_use_kdc_context,
                               gss_krb5int_register_acceptor_identity,
                               gss_krb5int_set_cred_rcache,
                               gss_krb5int_import_cred,
                               iakerb_gss_get_mic_iov_length,
                               iakerb_gss_verify_mic_iov,
                               iakerb_gss_get_mic_iov, krb5_gss_import_cred,
                               krb5_gss_export_cred, krb5_gss_store_cred_into,
                               krb5_gss_acquire_cred_from,
                               iakerb_gss_wrap_iov, iakerb_gss_unwrap_iov,
                               iakerb_gss_wrap_iov_length,
                               iakerb_gss_pseudo_random};
use self::stdlib_h::{free, calloc, malloc};
use self::string_h::{strlen, strdup, memcmp, memset, memcpy};
pub use self::gssapi_alloc_h::{gssalloc_malloc, gssalloc_strdup};
use self::gssapi_err_krb5_h::et_k5g_error_table;
/*
 * gss_inquire_sec_context_by_oid() methods
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "338:8"]
pub struct C2RustUnnamed {
    pub oid: gss_OID_desc,
    pub func: Option<unsafe extern "C" fn(_: *mut OM_uint32, _: gss_ctx_id_t,
                                          _: gss_OID,
                                          _: *mut gss_buffer_set_t)
                         -> OM_uint32>,
}
/*
 * gssspi_mech_invoke() methods
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "578:8"]
pub struct C2RustUnnamed_0 {
    pub oid: gss_OID_desc,
    pub func: Option<unsafe extern "C" fn(_: *mut OM_uint32, _: gss_OID,
                                          _: gss_OID, _: gss_buffer_t)
                         -> OM_uint32>,
}
/*
 * gssspi_set_cred_option() methods
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "508:8"]
pub struct C2RustUnnamed_1 {
    pub oid: gss_OID_desc,
    pub func: Option<unsafe extern "C" fn(_: *mut OM_uint32,
                                          _: *mut gss_cred_id_t, _: gss_OID,
                                          _: gss_buffer_t) -> OM_uint32>,
}
/*
 * gss_inquire_cred_by_oid() methods
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "414:8"]
pub struct C2RustUnnamed_2 {
    pub oid: gss_OID_desc,
    pub func: Option<unsafe extern "C" fn(_: *mut OM_uint32, _: gss_cred_id_t,
                                          _: gss_OID,
                                          _: *mut gss_buffer_set_t)
                         -> OM_uint32>,
}
#[no_mangle]
#[c2rust::src_loc = "130:20"]
pub static mut krb5_gss_oid_array: [gss_OID_desc; 11] =
    [{
         let mut init =
             gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 5 as libc::c_int as OM_uint32,
                                 elements:
                                     b"+\x05\x01\x05\x02\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x82\xf7\x12\x01\x02\x02\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x02\x05\x00" as *const u8
                                         as *const libc::c_char as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x03\x00" as
                                         *const u8 as *const libc::c_char as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 10 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x01\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 10 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x02\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x85p+\r\x1d\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0e\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 10 as libc::c_int as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x06\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 0 as libc::c_int as OM_uint32,
                                 elements:
                                     0 as *const libc::c_void as
                                         *mut libc::c_void,};
         init
     }];
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "160:15"]
pub static mut gss_mech_krb5: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "161:15"]
pub static mut gss_mech_krb5_old: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "162:15"]
pub static mut gss_mech_krb5_wrong: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "163:15"]
pub static mut gss_mech_iakerb: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "166:15"]
pub static mut gss_nt_krb5_name: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "167:15"]
pub static mut gss_nt_krb5_principal: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "168:15"]
pub static mut GSS_KRB5_NT_PRINCIPAL_NAME: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "170:15"]
pub static mut GSS_KRB5_CRED_NO_CI_FLAGS_X: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "171:15"]
pub static mut GSS_KRB5_GET_CRED_IMPERSONATOR: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "172:15"]
pub static mut GSS_KRB5_NT_ENTERPRISE_NAME: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[c2rust::src_loc = "174:31"]
static mut oidsets: [gss_OID_set_desc; 4] =
    [gss_OID_set_desc{count: 0, elements: 0 as *mut gss_OID_desc_struct,}; 4];
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "183:19"]
pub static mut gss_mech_set_krb5: gss_OID_set =
    0 as *const gss_OID_set_desc_struct as *mut gss_OID_set_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "184:19"]
pub static mut gss_mech_set_krb5_old: gss_OID_set =
    0 as *const gss_OID_set_desc_struct as *mut gss_OID_set_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "185:19"]
pub static mut gss_mech_set_krb5_both: gss_OID_set =
    0 as *const gss_OID_set_desc_struct as *mut gss_OID_set_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "186:19"]
pub static mut kg_all_mechs: gss_OID_set =
    0 as *const gss_OID_set_desc_struct as *mut gss_OID_set_desc_struct;
#[no_mangle]
#[c2rust::src_loc = "188:7"]
pub static mut kg_vdb: g_set =
    {
        let mut init =
            g_set{mutex:
                      pthread_mutex_t{__data:
                                          {
                                              let mut init =
                                                  __pthread_mutex_s{__lock:
                                                                        0 as
                                                                            libc::c_int,
                                                                    __count:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint,
                                                                    __owner:
                                                                        0 as
                                                                            libc::c_int,
                                                                    __nusers:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint,
                                                                    __kind:
                                                                        0 as
                                                                            libc::c_int,
                                                                    __spins:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_short,
                                                                    __elision:
                                                                        0 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_short,
                                                                    __list:
                                                                        {
                                                                            let mut init =
                                                                                __pthread_internal_list{__prev:
                                                                                                            0
                                                                                                                as
                                                                                                                *const __pthread_internal_list
                                                                                                                as
                                                                                                                *mut __pthread_internal_list,
                                                                                                        __next:
                                                                                                            0
                                                                                                                as
                                                                                                                *const __pthread_internal_list
                                                                                                                as
                                                                                                                *mut __pthread_internal_list,};
                                                                            init
                                                                        },};
                                              init
                                          },},
                  data: 0 as *const libc::c_void as *mut libc::c_void,};
        init
    };
/* * default credential support */
/*
 * init_sec_context() will explicitly re-acquire default credentials,
 * so handling the expiration/invalidation condition here isn't needed.
 */
#[no_mangle]
#[c2rust::src_loc = "196:1"]
pub unsafe extern "C" fn kg_get_defcred(mut minor_status: *mut OM_uint32,
                                        mut cred: *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    major =
        krb5_gss_acquire_cred(minor_status,
                              0 as *mut libc::c_void as gss_name_t,
                              0xffffffff as libc::c_ulong as OM_uint32,
                              0 as gss_OID_set, 1 as libc::c_int, cred,
                              0 as *mut gss_OID_set, 0 as *mut OM_uint32);
    if major != 0 &&
           major &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
        return major
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "213:1"]
pub unsafe extern "C" fn kg_sync_ccache_name(mut context: krb5_context,
                                             mut minor_status: *mut OM_uint32)
 -> OM_uint32 {
    let mut err: OM_uint32 = 0 as libc::c_int as OM_uint32;
    /*
     * Sync up the context ccache name with the GSSAPI ccache name.
     * If kg_ccache_name is NULL -- normal unless someone has called
     * gss_krb5_ccache_name() -- then the system default ccache will
     * be picked up and used by resetting the context default ccache.
     * This is needed for platforms which support multiple ccaches.
     */
    if err == 0 {
        /* if NULL, resets the context default ccache */
        err =
            krb5_cc_set_default_name(context,
                                     krb5int_getspecific(K5_KEY_GSS_KRB5_CCACHE_NAME)
                                         as *mut libc::c_char) as OM_uint32
    }
    *minor_status = err;
    return if *minor_status == 0 as libc::c_int as libc::c_uint {
               0 as libc::c_int as libc::c_uint
           } else {
               ((13 as libc::c_ulong as OM_uint32)) << 16 as libc::c_int
           };
}
/* This function returns whether or not the caller set a cccache name.  Used by
 * gss_acquire_cred to figure out if the caller wants to only look at this
 * ccache or search the cache collection for the desired name */
#[no_mangle]
#[c2rust::src_loc = "239:1"]
pub unsafe extern "C" fn kg_caller_provided_ccache_name(mut minor_status:
                                                            *mut OM_uint32,
                                                        mut out_caller_provided_name:
                                                            *mut libc::c_int)
 -> OM_uint32 {
    if !out_caller_provided_name.is_null() {
        *out_caller_provided_name =
            (krb5int_getspecific(K5_KEY_GSS_KRB5_CCACHE_NAME) !=
                 0 as *mut libc::c_void) as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn kg_get_ccache_name(mut minor_status: *mut OM_uint32,
                                            mut out_name:
                                                *mut *const libc::c_char)
 -> OM_uint32 {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut err: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut kg_ccache_name: *mut libc::c_char = 0 as *mut libc::c_char;
    kg_ccache_name =
        krb5int_getspecific(K5_KEY_GSS_KRB5_CCACHE_NAME) as *mut libc::c_char;
    if !kg_ccache_name.is_null() {
        name = strdup(kg_ccache_name);
        if name.is_null() { err = 12 as libc::c_int as OM_uint32 }
    } else {
        let mut context: krb5_context = 0 as krb5_context;
        /* Reset the context default ccache (see text above), and then
           retrieve it.  */
        err = krb5_gss_init_context(&mut context) as OM_uint32;
        if err == 0 {
            err =
                krb5_cc_set_default_name(context, 0 as *const libc::c_char) as
                    OM_uint32
        }
        if err == 0 {
            name = krb5_cc_default_name(context);
            if !name.is_null() {
                name = strdup(name);
                if name.is_null() { err = 12 as libc::c_int as OM_uint32 }
            }
        }
        if err != 0 && !context.is_null() {
            krb5_gss_save_error_info(err, context);
        }
        if !context.is_null() { krb5_free_context(context); }
    }
    if err == 0 { if !out_name.is_null() { *out_name = name } }
    *minor_status = err;
    return if *minor_status == 0 as libc::c_int as libc::c_uint {
               0 as libc::c_int as libc::c_uint
           } else {
               ((13 as libc::c_ulong as OM_uint32)) << 16 as libc::c_int
           };
}
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn kg_set_ccache_name(mut minor_status: *mut OM_uint32,
                                            mut name: *const libc::c_char)
 -> OM_uint32 {
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut swap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kg_ccache_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kerr: krb5_error_code = 0;
    if !name.is_null() {
        new_name = strdup(name);
        if new_name.is_null() {
            *minor_status = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    kg_ccache_name =
        krb5int_getspecific(K5_KEY_GSS_KRB5_CCACHE_NAME) as *mut libc::c_char;
    swap = kg_ccache_name;
    kg_ccache_name = new_name;
    new_name = swap;
    kerr =
        krb5int_setspecific(K5_KEY_GSS_KRB5_CCACHE_NAME,
                            kg_ccache_name as *mut libc::c_void);
    if kerr != 0 as libc::c_int {
        /* Can't store, so free up the storage.  */
        free(kg_ccache_name as *mut libc::c_void);
        /* ??? free(new_name); */
        *minor_status = kerr as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    free(new_name as *mut libc::c_void);
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "341:3"]
static mut krb5_gss_inquire_sec_context_by_oid_ops: [C2RustUnnamed; 6] =
    unsafe {
        [{
             let mut init =
                 C2RustUnnamed{oid:
                                   {
                                       let mut init =
                                           gss_OID_desc_struct{length:
                                                                   11 as
                                                                       libc::c_int
                                                                       as
                                                                       OM_uint32,
                                                               elements:
                                                                   b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x01\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                                       as
                                                                       *mut libc::c_void,};
                                       init
                                   },
                               func:
                                   Some(gss_krb5int_get_tkt_flags as
                                            unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{oid:
                                   {
                                       let mut init =
                                           gss_OID_desc_struct{length:
                                                                   11 as
                                                                       libc::c_int
                                                                       as
                                                                       OM_uint32,
                                                               elements:
                                                                   b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\n\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                                       as
                                                                       *mut libc::c_void,};
                                       init
                                   },
                               func:
                                   Some(gss_krb5int_extract_authz_data_from_sec_context
                                            as
                                            unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{oid:
                                   {
                                       let mut init =
                                           gss_OID_desc_struct{length:
                                                                   11 as
                                                                       libc::c_int
                                                                       as
                                                                       OM_uint32,
                                                               elements:
                                                                   b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x05\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                                       as
                                                                       *mut libc::c_void,};
                                       init
                                   },
                               func:
                                   Some(gss_krb5int_inq_session_key as
                                            unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{oid:
                                   {
                                       let mut init =
                                           gss_OID_desc_struct{length:
                                                                   11 as
                                                                       libc::c_int
                                                                       as
                                                                       OM_uint32,
                                                               elements:
                                                                   b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x06\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                                       as
                                                                       *mut libc::c_void,};
                                       init
                                   },
                               func:
                                   Some(gss_krb5int_export_lucid_sec_context
                                            as
                                            unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{oid:
                                   {
                                       let mut init =
                                           gss_OID_desc_struct{length:
                                                                   11 as
                                                                       libc::c_int
                                                                       as
                                                                       OM_uint32,
                                                               elements:
                                                                   b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0c\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                                       as
                                                                       *mut libc::c_void,};
                                       init
                                   },
                               func:
                                   Some(gss_krb5int_extract_authtime_from_sec_context
                                            as
                                            unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{oid:
                                   {
                                       let mut init =
                                           gss_OID_desc_struct{length:
                                                                   11 as
                                                                       libc::c_int
                                                                       as
                                                                       OM_uint32,
                                                               elements:
                                                                   b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0f\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char
                                                                       as
                                                                       *mut libc::c_void,};
                                       init
                                   },
                               func:
                                   Some(gss_krb5int_sec_context_sasl_ssf as
                                            unsafe extern "C" fn(_:
                                                                     *mut OM_uint32,
                                                                 _:
                                                                     gss_ctx_id_t,
                                                                 _: gss_OID,
                                                                 _:
                                                                     *mut gss_buffer_set_t)
                                                -> OM_uint32),};
             init
         }]
    };
#[no_mangle]
#[c2rust::src_loc = "368:1"]
pub unsafe extern "C" fn krb5_gss_inquire_sec_context_by_oid(mut minor_status:
                                                                 *mut OM_uint32,
                                                             context_handle:
                                                                 gss_ctx_id_t,
                                                             desired_object:
                                                                 gss_OID,
                                                             mut data_set:
                                                                 *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut ctx: *mut krb5_gss_ctx_id_rec = 0 as *mut krb5_gss_ctx_id_rec;
    let mut i: size_t = 0;
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    if desired_object.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if data_set.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *data_set = 0 as gss_buffer_set_t;
    ctx = context_handle as *mut krb5_gss_ctx_id_rec;
    if (*ctx).terminated() as libc::c_int != 0 || (*ctx).established() == 0 {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed; 6]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed>()
                                                   as libc::c_ulong) {
        if (*desired_object).length >=
               krb5_gss_inquire_sec_context_by_oid_ops[i as usize].oid.length
               &&
               memcmp((*desired_object).elements,
                      krb5_gss_inquire_sec_context_by_oid_ops[i as
                                                                  usize].oid.elements,
                      krb5_gss_inquire_sec_context_by_oid_ops[i as
                                                                  usize].oid.length
                          as libc::c_ulong) == 0 as libc::c_int {
            return Some((*krb5_gss_inquire_sec_context_by_oid_ops.as_mut_ptr().offset(i
                                                                                          as
                                                                                          isize)).func.expect("non-null function pointer")).expect("non-null function pointer")(minor_status,
                                                                                                                                                                                context_handle,
                                                                                                                                                                                desired_object,
                                                                                                                                                                                data_set)
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22 as libc::c_int as OM_uint32;
    return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[c2rust::src_loc = "417:3"]
static mut krb5_gss_inquire_cred_by_oid_ops: [C2RustUnnamed_2; 1] =
    unsafe {
        [{
             let mut init =
                 C2RustUnnamed_2{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0e\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_get_cred_impersonator as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       gss_cred_id_t,
                                                                   _: gss_OID,
                                                                   _:
                                                                       *mut gss_buffer_set_t)
                                                  -> OM_uint32),};
             init
         }]
    };
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn krb5_gss_inquire_cred_by_oid(mut minor_status:
                                                      *mut OM_uint32,
                                                  cred_handle: gss_cred_id_t,
                                                  desired_object: gss_OID,
                                                  mut data_set:
                                                      *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut major_status: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut i: size_t = 0;
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    if desired_object.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if data_set.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *data_set = 0 as gss_buffer_set_t;
    if cred_handle.is_null() {
        *minor_status = -(1765328181 as libc::c_long) as OM_uint32;
        return (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    major_status = krb5_gss_validate_cred(minor_status, cred_handle);
    if major_status &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return major_status
    }
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed_2; 1]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_2>()
                                                   as libc::c_ulong) {
        if (*desired_object).length >=
               krb5_gss_inquire_cred_by_oid_ops[i as usize].oid.length &&
               memcmp((*desired_object).elements,
                      krb5_gss_inquire_cred_by_oid_ops[i as
                                                           usize].oid.elements,
                      krb5_gss_inquire_cred_by_oid_ops[i as usize].oid.length
                          as libc::c_ulong) == 0 as libc::c_int {
            return Some((*krb5_gss_inquire_cred_by_oid_ops.as_mut_ptr().offset(i
                                                                                   as
                                                                                   isize)).func.expect("non-null function pointer")).expect("non-null function pointer")(minor_status,
                                                                                                                                                                         cred_handle,
                                                                                                                                                                         desired_object,
                                                                                                                                                                         data_set)
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22 as libc::c_int as OM_uint32;
    return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "469:1"]
pub unsafe extern "C" fn krb5_gss_set_sec_context_option(mut minor_status:
                                                             *mut OM_uint32,
                                                         mut context_handle:
                                                             *mut gss_ctx_id_t,
                                                         desired_object:
                                                             gss_OID,
                                                         value: gss_buffer_t)
 -> OM_uint32 {
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    if context_handle.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if desired_object.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 22 as libc::c_int as OM_uint32;
    return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[c2rust::src_loc = "491:1"]
unsafe extern "C" fn no_ci_flags(mut minor_status: *mut OM_uint32,
                                 mut cred_handle: *mut gss_cred_id_t,
                                 desired_oid: gss_OID, value: gss_buffer_t)
 -> OM_uint32 {
    let mut cred: krb5_gss_cred_id_t = 0 as *mut _krb5_gss_cred_id_rec;
    cred = *cred_handle as krb5_gss_cred_id_t;
    (*cred).set_suppress_ci_flags(1 as libc::c_int as libc::c_uint);
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "511:3"]
static mut krb5_gssspi_set_cred_option_ops: [C2RustUnnamed_1; 5] =
    unsafe {
        [{
             let mut init =
                 C2RustUnnamed_1{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x02\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_copy_ccache as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       *mut gss_cred_id_t,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_1{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x04\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_set_allowable_enctypes
                                              as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       *mut gss_cred_id_t,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_1{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0b\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_set_cred_rcache as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       *mut gss_cred_id_t,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_1{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\r\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_import_cred as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       *mut gss_cred_id_t,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_1{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     6 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x85p+\r\x1d\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(no_ci_flags as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _:
                                                                       *mut gss_cred_id_t,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         }]
    };
#[c2rust::src_loc = "534:1"]
unsafe extern "C" fn krb5_gssspi_set_cred_option(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut cred_handle:
                                                     *mut gss_cred_id_t,
                                                 desired_object: gss_OID,
                                                 value: gss_buffer_t)
 -> OM_uint32 {
    let mut major_status: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut i: size_t = 0;
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if cred_handle.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    if desired_object.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    if !(*cred_handle).is_null() {
        major_status = krb5_gss_validate_cred(minor_status, *cred_handle);
        if major_status &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
            return major_status
        }
    }
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed_1; 5]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_1>()
                                                   as libc::c_ulong) {
        if (*desired_object).length >=
               krb5_gssspi_set_cred_option_ops[i as usize].oid.length &&
               memcmp((*desired_object).elements,
                      krb5_gssspi_set_cred_option_ops[i as
                                                          usize].oid.elements,
                      krb5_gssspi_set_cred_option_ops[i as usize].oid.length
                          as libc::c_ulong) == 0 as libc::c_int {
            return Some((*krb5_gssspi_set_cred_option_ops.as_mut_ptr().offset(i
                                                                                  as
                                                                                  isize)).func.expect("non-null function pointer")).expect("non-null function pointer")(minor_status,
                                                                                                                                                                        cred_handle,
                                                                                                                                                                        desired_object,
                                                                                                                                                                        value)
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22 as libc::c_int as OM_uint32;
    return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[c2rust::src_loc = "581:3"]
static mut krb5_gssspi_mech_invoke_ops: [C2RustUnnamed_0; 4] =
    unsafe {
        [{
             let mut init =
                 C2RustUnnamed_0{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\t\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_register_acceptor_identity
                                              as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _: gss_OID,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x03\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_ccache_name as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _: gss_OID,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x07\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(gss_krb5int_free_lucid_sec_context
                                              as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _: gss_OID,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         },
         {
             let mut init =
                 C2RustUnnamed_0{oid:
                                     {
                                         let mut init =
                                             gss_OID_desc_struct{length:
                                                                     11 as
                                                                         libc::c_int
                                                                         as
                                                                         OM_uint32,
                                                                 elements:
                                                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x08\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char
                                                                         as
                                                                         *mut libc::c_void,};
                                         init
                                     },
                                 func:
                                     Some(krb5int_gss_use_kdc_context as
                                              unsafe extern "C" fn(_:
                                                                       *mut OM_uint32,
                                                                   _: gss_OID,
                                                                   _: gss_OID,
                                                                   _:
                                                                       gss_buffer_t)
                                                  -> OM_uint32),};
             init
         }]
    };
#[c2rust::src_loc = "602:1"]
unsafe extern "C" fn krb5_gssspi_mech_invoke(mut minor_status: *mut OM_uint32,
                                             desired_mech: gss_OID,
                                             desired_object: gss_OID,
                                             mut value: gss_buffer_t)
 -> OM_uint32 {
    let mut i: size_t = 0;
    if minor_status.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    if desired_mech.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if desired_object.is_null() {
        return (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed_0; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_0>()
                                                   as libc::c_ulong) {
        if (*desired_object).length >=
               krb5_gssspi_mech_invoke_ops[i as usize].oid.length &&
               memcmp((*desired_object).elements,
                      krb5_gssspi_mech_invoke_ops[i as usize].oid.elements,
                      krb5_gssspi_mech_invoke_ops[i as usize].oid.length as
                          libc::c_ulong) == 0 as libc::c_int {
            return Some((*krb5_gssspi_mech_invoke_ops.as_mut_ptr().offset(i as
                                                                              isize)).func.expect("non-null function pointer")).expect("non-null function pointer")(minor_status,
                                                                                                                                                                    desired_mech,
                                                                                                                                                                    desired_object,
                                                                                                                                                                    value)
        }
        i = i.wrapping_add(1)
    }
    *minor_status = 22 as libc::c_int as OM_uint32;
    return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[c2rust::src_loc = "642:1"]
unsafe extern "C" fn krb5_gss_inquire_mech_for_saslname(mut minor_status:
                                                            *mut OM_uint32,
                                                        sasl_mech_name:
                                                            gss_buffer_t,
                                                        mut mech_type:
                                                            *mut gss_OID)
 -> OM_uint32 {
    *minor_status = 0 as libc::c_int as OM_uint32;
    if (*sasl_mech_name).length ==
           (::std::mem::size_of::<[libc::c_char; 9]>() as
                libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
           &&
           memcmp((*sasl_mech_name).value,
                  b"GS2-KRB5\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void,
                  (::std::mem::size_of::<[libc::c_char; 9]>() as
                       libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong)) ==
               0 as libc::c_int {
        if !mech_type.is_null() { *mech_type = gss_mech_krb5 }
        return 0 as libc::c_int as OM_uint32
    } else {
        if (*sasl_mech_name).length ==
               (::std::mem::size_of::<[libc::c_char; 11]>() as
                    libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulong) &&
               memcmp((*sasl_mech_name).value,
                      b"GS2-IAKERB\x00" as *const u8 as *const libc::c_char as
                          *const libc::c_void,
                      (::std::mem::size_of::<[libc::c_char; 11]>() as
                           libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong)) ==
                   0 as libc::c_int {
            if !mech_type.is_null() { *mech_type = gss_mech_iakerb }
            return 0 as libc::c_int as OM_uint32
        }
    }
    return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[c2rust::src_loc = "666:1"]
unsafe extern "C" fn krb5_gss_inquire_saslname_for_mech(mut minor_status:
                                                            *mut OM_uint32,
                                                        desired_mech: gss_OID,
                                                        mut sasl_mech_name:
                                                            gss_buffer_t,
                                                        mut mech_name:
                                                            gss_buffer_t,
                                                        mut mech_description:
                                                            gss_buffer_t)
 -> OM_uint32 {
    let mut current_block: u64;
    if (*desired_mech).length == (*gss_mech_iakerb).length &&
           memcmp((*desired_mech).elements, (*gss_mech_iakerb).elements,
                  (*desired_mech).length as libc::c_ulong) == 0 as libc::c_int
       {
        if gssint_g_make_string_buffer(b"GS2-IAKERB\x00" as *const u8 as
                                           *const libc::c_char,
                                       sasl_mech_name) == 0 ||
               gssint_g_make_string_buffer(b"iakerb\x00" as *const u8 as
                                               *const libc::c_char, mech_name)
                   == 0 ||
               gssint_g_make_string_buffer(b"Initial and Pass Through Authentication Kerberos Mechanism (IAKERB)\x00"
                                               as *const u8 as
                                               *const libc::c_char,
                                           mech_description) == 0 {
            current_block = 2215464789205338958;
        } else { current_block = 735147466149431745; }
    } else if gssint_g_make_string_buffer(b"GS2-KRB5\x00" as *const u8 as
                                              *const libc::c_char,
                                          sasl_mech_name) == 0 ||
                  gssint_g_make_string_buffer(b"krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              mech_name) == 0 ||
                  gssint_g_make_string_buffer(b"Kerberos 5 GSS-API Mechanism\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              mech_description) == 0 {
        current_block = 2215464789205338958;
    } else { current_block = 735147466149431745; }
    match current_block {
        735147466149431745 => {
            *minor_status = 0 as libc::c_int as OM_uint32;
            return 0 as libc::c_int as OM_uint32
        }
        _ => {
            *minor_status = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    };
}
#[c2rust::src_loc = "696:1"]
unsafe extern "C" fn krb5_gss_inquire_attrs_for_mech(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut mech: gss_const_OID,
                                                     mut mech_attrs:
                                                         *mut gss_OID_set,
                                                     mut known_mech_attrs:
                                                         *mut gss_OID_set)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut tmpMinor: OM_uint32 = 0;
    if mech_attrs.is_null() {
        *minor_status = 0 as libc::c_int as OM_uint32;
        return 0 as libc::c_int as OM_uint32
    }
    major = gss_create_empty_oid_set(minor_status, mech_attrs);
    if !(major &
             ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                  (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int)
             != 0) {
        major =
            gss_add_oid_set_member(minor_status,
                                   GSS_C_MA_MECH_CONCRETE as gss_OID,
                                   mech_attrs);
        if !(major &
                 ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                      (0o377 as libc::c_ulong as OM_uint32) <<
                          16 as libc::c_int) != 0) {
            major =
                gss_add_oid_set_member(minor_status,
                                       GSS_C_MA_ITOK_FRAMED as gss_OID,
                                       mech_attrs);
            if !(major &
                     ((0o377 as libc::c_ulong as OM_uint32) <<
                          24 as libc::c_int |
                          (0o377 as libc::c_ulong as OM_uint32) <<
                              16 as libc::c_int) != 0) {
                major =
                    gss_add_oid_set_member(minor_status,
                                           GSS_C_MA_AUTH_INIT as gss_OID,
                                           mech_attrs);
                if !(major &
                         ((0o377 as libc::c_ulong as OM_uint32) <<
                              24 as libc::c_int |
                              (0o377 as libc::c_ulong as OM_uint32) <<
                                  16 as libc::c_int) != 0) {
                    major =
                        gss_add_oid_set_member(minor_status,
                                               GSS_C_MA_AUTH_TARG as gss_OID,
                                               mech_attrs);
                    if !(major &
                             ((0o377 as libc::c_ulong as OM_uint32) <<
                                  24 as libc::c_int |
                                  (0o377 as libc::c_ulong as OM_uint32) <<
                                      16 as libc::c_int) != 0) {
                        major =
                            gss_add_oid_set_member(minor_status,
                                                   GSS_C_MA_DELEG_CRED as
                                                       gss_OID, mech_attrs);
                        if !(major &
                                 ((0o377 as libc::c_ulong as OM_uint32) <<
                                      24 as libc::c_int |
                                      (0o377 as libc::c_ulong as OM_uint32) <<
                                          16 as libc::c_int) != 0) {
                            major =
                                gss_add_oid_set_member(minor_status,
                                                       GSS_C_MA_INTEG_PROT as
                                                           gss_OID,
                                                       mech_attrs);
                            if !(major &
                                     ((0o377 as libc::c_ulong as OM_uint32) <<
                                          24 as libc::c_int |
                                          (0o377 as libc::c_ulong as
                                               OM_uint32) <<
                                              16 as libc::c_int) != 0) {
                                major =
                                    gss_add_oid_set_member(minor_status,
                                                           GSS_C_MA_CONF_PROT
                                                               as gss_OID,
                                                           mech_attrs);
                                if !(major &
                                         ((0o377 as libc::c_ulong as
                                               OM_uint32) << 24 as libc::c_int
                                              |
                                              (0o377 as libc::c_ulong as
                                                   OM_uint32) <<
                                                  16 as libc::c_int) != 0) {
                                    major =
                                        gss_add_oid_set_member(minor_status,
                                                               GSS_C_MA_MIC as
                                                                   gss_OID,
                                                               mech_attrs);
                                    if !(major &
                                             ((0o377 as libc::c_ulong as
                                                   OM_uint32) <<
                                                  24 as libc::c_int |
                                                  (0o377 as libc::c_ulong as
                                                       OM_uint32) <<
                                                      16 as libc::c_int) != 0)
                                       {
                                        major =
                                            gss_add_oid_set_member(minor_status,
                                                                   GSS_C_MA_WRAP
                                                                       as
                                                                       gss_OID,
                                                                   mech_attrs);
                                        if !(major &
                                                 ((0o377 as libc::c_ulong as
                                                       OM_uint32) <<
                                                      24 as libc::c_int |
                                                      (0o377 as libc::c_ulong
                                                           as OM_uint32) <<
                                                          16 as libc::c_int)
                                                 != 0) {
                                            major =
                                                gss_add_oid_set_member(minor_status,
                                                                       GSS_C_MA_PROT_READY
                                                                           as
                                                                           gss_OID,
                                                                       mech_attrs);
                                            if !(major &
                                                     ((0o377 as libc::c_ulong
                                                           as OM_uint32) <<
                                                          24 as libc::c_int |
                                                          (0o377 as
                                                               libc::c_ulong
                                                               as OM_uint32)
                                                              <<
                                                              16 as
                                                                  libc::c_int)
                                                     != 0) {
                                                major =
                                                    gss_add_oid_set_member(minor_status,
                                                                           GSS_C_MA_REPLAY_DET
                                                                               as
                                                                               gss_OID,
                                                                           mech_attrs);
                                                if !(major &
                                                         ((0o377 as
                                                               libc::c_ulong
                                                               as OM_uint32)
                                                              <<
                                                              24 as
                                                                  libc::c_int
                                                              |
                                                              (0o377 as
                                                                   libc::c_ulong
                                                                   as
                                                                   OM_uint32)
                                                                  <<
                                                                  16 as
                                                                      libc::c_int)
                                                         != 0) {
                                                    major =
                                                        gss_add_oid_set_member(minor_status,
                                                                               GSS_C_MA_OOS_DET
                                                                                   as
                                                                                   gss_OID,
                                                                               mech_attrs);
                                                    if !(major &
                                                             ((0o377 as
                                                                   libc::c_ulong
                                                                   as
                                                                   OM_uint32)
                                                                  <<
                                                                  24 as
                                                                      libc::c_int
                                                                  |
                                                                  (0o377 as
                                                                       libc::c_ulong
                                                                       as
                                                                       OM_uint32)
                                                                      <<
                                                                      16 as
                                                                          libc::c_int)
                                                             != 0) {
                                                        major =
                                                            gss_add_oid_set_member(minor_status,
                                                                                   GSS_C_MA_CBINDINGS
                                                                                       as
                                                                                       gss_OID,
                                                                                   mech_attrs);
                                                        if !(major &
                                                                 ((0o377 as
                                                                       libc::c_ulong
                                                                       as
                                                                       OM_uint32)
                                                                      <<
                                                                      24 as
                                                                          libc::c_int
                                                                      |
                                                                      (0o377
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           OM_uint32)
                                                                          <<
                                                                          16
                                                                              as
                                                                              libc::c_int)
                                                                 != 0) {
                                                            major =
                                                                gss_add_oid_set_member(minor_status,
                                                                                       GSS_C_MA_CTX_TRANS
                                                                                           as
                                                                                           gss_OID,
                                                                                       mech_attrs);
                                                            if !(major &
                                                                     ((0o377
                                                                           as
                                                                           libc::c_ulong
                                                                           as
                                                                           OM_uint32)
                                                                          <<
                                                                          24
                                                                              as
                                                                              libc::c_int
                                                                          |
                                                                          (0o377
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               OM_uint32)
                                                                              <<
                                                                              16
                                                                                  as
                                                                                  libc::c_int)
                                                                     != 0) {
                                                                if (*mech).length
                                                                       ==
                                                                       (*gss_mech_iakerb).length
                                                                       &&
                                                                       memcmp((*mech).elements,
                                                                              (*gss_mech_iakerb).elements,
                                                                              (*mech).length
                                                                                  as
                                                                                  libc::c_ulong)
                                                                           ==
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                   {
                                                                    major =
                                                                        gss_add_oid_set_member(minor_status,
                                                                                               GSS_C_MA_AUTH_INIT_INIT
                                                                                                   as
                                                                                                   gss_OID,
                                                                                               mech_attrs);
                                                                    if !(major
                                                                             &
                                                                             ((0o377
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   OM_uint32)
                                                                                  <<
                                                                                  24
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  (0o377
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       OM_uint32)
                                                                                      <<
                                                                                      16
                                                                                          as
                                                                                          libc::c_int)
                                                                             !=
                                                                             0)
                                                                       {
                                                                        major
                                                                            =
                                                                            gss_add_oid_set_member(minor_status,
                                                                                                   GSS_C_MA_NOT_DFLT_MECH
                                                                                                       as
                                                                                                       gss_OID,
                                                                                                   mech_attrs);
                                                                        (major
                                                                             &
                                                                             ((0o377
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   OM_uint32)
                                                                                  <<
                                                                                  24
                                                                                      as
                                                                                      libc::c_int
                                                                                  |
                                                                                  (0o377
                                                                                       as
                                                                                       libc::c_ulong
                                                                                       as
                                                                                       OM_uint32)
                                                                                      <<
                                                                                      16
                                                                                          as
                                                                                          libc::c_int))
                                                                            !=
                                                                            0;
                                                                    }
                                                                } else if !((*mech).length
                                                                                ==
                                                                                (*gss_mech_krb5).length
                                                                                &&
                                                                                memcmp((*mech).elements,
                                                                                       (*gss_mech_krb5).elements,
                                                                                       (*mech).length
                                                                                           as
                                                                                           libc::c_ulong)
                                                                                    ==
                                                                                    0
                                                                                        as
                                                                                        libc::c_int)
                                                                 {
                                                                    major =
                                                                        gss_add_oid_set_member(minor_status,
                                                                                               GSS_C_MA_DEPRECATED
                                                                                                   as
                                                                                                   gss_OID,
                                                                                               mech_attrs);
                                                                    (major &
                                                                         ((0o377
                                                                               as
                                                                               libc::c_ulong
                                                                               as
                                                                               OM_uint32)
                                                                              <<
                                                                              24
                                                                                  as
                                                                                  libc::c_int
                                                                              |
                                                                              (0o377
                                                                                   as
                                                                                   libc::c_ulong
                                                                                   as
                                                                                   OM_uint32)
                                                                                  <<
                                                                                  16
                                                                                      as
                                                                                      libc::c_int))
                                                                        != 0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if major &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        gss_release_oid_set(&mut tmpMinor, mech_attrs);
    }
    return major;
}
#[c2rust::src_loc = "749:1"]
unsafe extern "C" fn krb5_gss_localname(mut minor: *mut OM_uint32,
                                        pname: gss_name_t,
                                        mech_type: gss_const_OID,
                                        mut localname: gss_buffer_t)
 -> OM_uint32 {
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut code: krb5_error_code = 0;
    let mut kname: krb5_gss_name_t = 0 as *mut _krb5_gss_name_rec;
    let mut lname: [libc::c_char; 8192] = [0; 8192];
    code = krb5_gss_init_context(&mut context);
    if code != 0 as libc::c_int {
        *minor = code as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    kname = pname as krb5_gss_name_t;
    code =
        krb5_aname_to_localname(context,
                                (*kname).princ as krb5_const_principal,
                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                    as libc::c_ulong as libc::c_int,
                                lname.as_mut_ptr());
    if code != 0 as libc::c_int {
        *minor = -(1765328227 as libc::c_long) as OM_uint32;
        krb5_free_context(context);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    krb5_free_context(context);
    (*localname).value =
        gssalloc_strdup(lname.as_mut_ptr()) as *mut libc::c_void;
    (*localname).length = strlen(lname.as_mut_ptr());
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "785:1"]
unsafe extern "C" fn krb5_gss_authorize_localname(mut minor: *mut OM_uint32,
                                                  pname: gss_name_t,
                                                  mut local_user:
                                                      gss_const_buffer_t,
                                                  mut name_type:
                                                      gss_const_OID)
 -> OM_uint32 {
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut code: krb5_error_code = 0;
    let mut kname: krb5_gss_name_t = 0 as *mut _krb5_gss_name_rec;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut user_ok: libc::c_int = 0;
    if !name_type.is_null() &&
           !((*name_type).length == (*GSS_C_NT_USER_NAME).length &&
                 memcmp((*name_type).elements, (*GSS_C_NT_USER_NAME).elements,
                        (*name_type).length as libc::c_ulong) ==
                     0 as libc::c_int) {
        return (3 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    kname = pname as krb5_gss_name_t;
    code = krb5_gss_init_context(&mut context);
    if code != 0 as libc::c_int {
        *minor = code as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    user =
        k5memdup0((*local_user).value, (*local_user).length, &mut code) as
            *mut libc::c_char;
    if user.is_null() {
        *minor = code as OM_uint32;
        krb5_free_context(context);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    user_ok = krb5_kuserok(context, (*kname).princ, user) as libc::c_int;
    free(user as *mut libc::c_void);
    krb5_free_context(context);
    *minor = 0 as libc::c_int as OM_uint32;
    return if user_ok != 0 {
               0 as libc::c_int as libc::c_uint
           } else {
               ((15 as libc::c_ulong as OM_uint32)) << 16 as libc::c_int
           };
}
#[c2rust::src_loc = "826:26"]
static mut krb5_mechanism: gss_config =
    unsafe {
        {
            let mut init =
                gss_config{mech_type:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               9 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },
                           context:
                               0 as *const libc::c_void as *mut libc::c_void,
                           gss_acquire_cred:
                               Some(krb5_gss_acquire_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_release_cred:
                               Some(krb5_gss_release_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gss_init_sec_context:
                               Some(krb5_gss_init_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_name_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_channel_bindings_t,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_accept_sec_context:
                               Some(krb5_gss_accept_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_cred_id_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 gss_channel_bindings_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _: *mut gss_OID,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gss_process_context_token:
                               Some(krb5_gss_process_context_token as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_delete_sec_context:
                               Some(krb5_gss_delete_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_context_time:
                               Some(krb5_gss_context_time as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_get_mic:
                               Some(krb5_gss_get_mic as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_verify_mic:
                               Some(krb5_gss_verify_mic as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_wrap:
                               Some(krb5_gss_wrap as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_unwrap:
                               Some(krb5_gss_unwrap as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_display_status:
                               Some(krb5_gss_display_status as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: OM_uint32,
                                                             _: libc::c_int,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_indicate_mechs:
                               Some(krb5_gss_indicate_mechs as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_compare_name:
                               Some(krb5_gss_compare_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_display_name:
                               Some(krb5_gss_display_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_import_name:
                               Some(krb5_gss_import_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_release_name:
                               Some(krb5_gss_release_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_inquire_cred:
                               Some(krb5_gss_inquire_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_add_cred: None,
                           gss_export_sec_context:
                               Some(krb5_gss_export_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_import_sec_context:
                               Some(krb5_gss_import_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_ctx_id_t)
                                            -> OM_uint32),
                           gss_inquire_cred_by_mech:
                               Some(krb5_gss_inquire_cred_by_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32),
                           gss_inquire_names_for_mech:
                               Some(krb5_gss_inquire_names_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_inquire_context:
                               Some(krb5_gss_inquire_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_internal_release_oid:
                               Some(krb5_gss_internal_release_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_wrap_size_limit:
                               Some(krb5_gss_wrap_size_limit as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_localname:
                               Some(krb5_gss_localname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_const_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_authorize_localname:
                               Some(krb5_gss_authorize_localname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 gss_const_buffer_t,
                                                             _: gss_const_OID)
                                            -> OM_uint32),
                           gss_export_name:
                               Some(krb5_gss_export_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_duplicate_name:
                               Some(krb5_gss_duplicate_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_store_cred:
                               Some(krb5_gss_store_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32),
                           gss_inquire_sec_context_by_oid:
                               Some(krb5_gss_inquire_sec_context_by_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_inquire_cred_by_oid:
                               Some(krb5_gss_inquire_cred_by_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_set_sec_context_option:
                               Some(krb5_gss_set_sec_context_option as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_set_cred_option:
                               Some(krb5_gssspi_set_cred_option as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_mech_invoke:
                               Some(krb5_gssspi_mech_invoke as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_wrap_aead: None,
                           gss_unwrap_aead: None,
                           gss_wrap_iov:
                               Some(krb5_gss_wrap_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_unwrap_iov:
                               Some(krb5_gss_unwrap_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_wrap_iov_length:
                               Some(krb5_gss_wrap_iov_length as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_complete_auth_token: None,
                           gss_acquire_cred_impersonate_name:
                               Some(krb5_gss_acquire_cred_impersonate_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_add_cred_impersonate_name: None,
                           gss_display_name_ext: None,
                           gss_inquire_name:
                               Some(krb5_gss_inquire_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_get_name_attribute:
                               Some(krb5_gss_get_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_set_name_attribute:
                               Some(krb5_gss_set_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_delete_name_attribute:
                               Some(krb5_gss_delete_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_export_name_composite:
                               Some(krb5_gss_export_name_composite as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_map_name_to_any:
                               Some(krb5_gss_map_name_to_any as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32),
                           gss_release_any_name_mapping:
                               Some(krb5_gss_release_any_name_mapping as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32),
                           gss_pseudo_random:
                               Some(krb5_gss_pseudo_random as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: ssize_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_set_neg_mechs: None,
                           gss_inquire_saslname_for_mech:
                               Some(krb5_gss_inquire_saslname_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_inquire_mech_for_saslname:
                               Some(krb5_gss_inquire_mech_for_saslname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_inquire_attrs_for_mech:
                               Some(krb5_gss_inquire_attrs_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_const_OID,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_acquire_cred_from:
                               Some(krb5_gss_acquire_cred_from as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_store_cred_into:
                               Some(krb5_gss_store_cred_into as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32),
                           gssspi_acquire_cred_with_password:
                               Some(krb5_gss_acquire_cred_with_password as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_export_cred:
                               Some(krb5_gss_export_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_import_cred:
                               Some(krb5_gss_import_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gssspi_import_sec_context_by_mech: None,
                           gssspi_import_name_by_mech: None,
                           gssspi_import_cred_by_mech: None,
                           gss_get_mic_iov:
                               Some(krb5_gss_get_mic_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_verify_mic_iov:
                               Some(krb5_gss_verify_mic_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_get_mic_iov_length:
                               Some(krb5_gss_get_mic_iov_length as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gssspi_query_meta_data: None,
                           gssspi_exchange_meta_data: None,
                           gssspi_query_mechanism_info: None,};
            init
        }
    };
/* Functions which use security contexts or acquire creds are IAKERB-specific;
 * other functions can borrow from the krb5 mech. */
#[c2rust::src_loc = "919:26"]
static mut iakerb_mechanism: gss_config =
    unsafe {
        {
            let mut init =
                gss_config{mech_type:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               9 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },
                           context:
                               0 as *const libc::c_void as *mut libc::c_void,
                           gss_acquire_cred:
                               Some(iakerb_gss_acquire_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_release_cred:
                               Some(krb5_gss_release_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gss_init_sec_context:
                               Some(iakerb_gss_init_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_name_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_channel_bindings_t,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_accept_sec_context:
                               Some(iakerb_gss_accept_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_cred_id_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 gss_channel_bindings_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _: *mut gss_OID,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gss_process_context_token:
                               Some(iakerb_gss_process_context_token as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_delete_sec_context:
                               Some(iakerb_gss_delete_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_context_time:
                               Some(iakerb_gss_context_time as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_get_mic:
                               Some(iakerb_gss_get_mic as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_verify_mic:
                               Some(iakerb_gss_verify_mic as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_wrap:
                               Some(iakerb_gss_wrap as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_unwrap:
                               Some(iakerb_gss_unwrap as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t)
                                            -> OM_uint32),
                           gss_display_status:
                               Some(krb5_gss_display_status as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: OM_uint32,
                                                             _: libc::c_int,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_indicate_mechs:
                               Some(krb5_gss_indicate_mechs as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_compare_name:
                               Some(krb5_gss_compare_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_display_name:
                               Some(krb5_gss_display_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_import_name:
                               Some(krb5_gss_import_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_release_name:
                               Some(krb5_gss_release_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_inquire_cred:
                               Some(krb5_gss_inquire_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_usage_t,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_add_cred: None,
                           gss_export_sec_context:
                               Some(iakerb_gss_export_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_import_sec_context:
                               Some(iakerb_gss_import_sec_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_ctx_id_t)
                                            -> OM_uint32),
                           gss_inquire_cred_by_mech:
                               Some(krb5_gss_inquire_cred_by_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32),
                           gss_inquire_names_for_mech:
                               Some(krb5_gss_inquire_names_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_inquire_context:
                               Some(iakerb_gss_inquire_context as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut gss_name_t,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_internal_release_oid:
                               Some(krb5_gss_internal_release_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_wrap_size_limit:
                               Some(iakerb_gss_wrap_size_limit as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_localname:
                               Some(krb5_gss_localname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_const_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_authorize_localname:
                               Some(krb5_gss_authorize_localname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 gss_const_buffer_t,
                                                             _: gss_const_OID)
                                            -> OM_uint32),
                           gss_export_name:
                               Some(krb5_gss_export_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_duplicate_name:
                               Some(krb5_gss_duplicate_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut gss_name_t)
                                            -> OM_uint32),
                           gss_store_cred:
                               Some(krb5_gss_store_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32),
                           gss_inquire_sec_context_by_oid:
                               Some(iakerb_gss_inquire_sec_context_by_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_inquire_cred_by_oid:
                               Some(krb5_gss_inquire_cred_by_oid as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_set_sec_context_option:
                               Some(iakerb_gss_set_sec_context_option as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_ctx_id_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_set_cred_option:
                               Some(krb5_gssspi_set_cred_option as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gssspi_mech_invoke:
                               Some(krb5_gssspi_mech_invoke as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _: gss_OID,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_wrap_aead: None,
                           gss_unwrap_aead: None,
                           gss_wrap_iov:
                               Some(iakerb_gss_wrap_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_unwrap_iov:
                               Some(iakerb_gss_unwrap_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_wrap_iov_length:
                               Some(iakerb_gss_wrap_iov_length as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_complete_auth_token: None,
                           gss_acquire_cred_impersonate_name: None,
                           gss_add_cred_impersonate_name: None,
                           gss_display_name_ext: None,
                           gss_inquire_name:
                               Some(krb5_gss_inquire_name as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: *mut gss_OID,
                                                             _:
                                                                 *mut gss_buffer_set_t)
                                            -> OM_uint32),
                           gss_get_name_attribute:
                               Some(krb5_gss_get_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _:
                                                                 *mut libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut libc::c_int)
                                            -> OM_uint32),
                           gss_set_name_attribute:
                               Some(krb5_gss_set_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_delete_name_attribute:
                               Some(krb5_gss_delete_name_attribute as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_export_name_composite:
                               Some(krb5_gss_export_name_composite as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_map_name_to_any:
                               Some(krb5_gss_map_name_to_any as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32),
                           gss_release_any_name_mapping:
                               Some(krb5_gss_release_any_name_mapping as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_any_t)
                                            -> OM_uint32),
                           gss_pseudo_random:
                               Some(iakerb_gss_pseudo_random as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: libc::c_int,
                                                             _: gss_buffer_t,
                                                             _: ssize_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_set_neg_mechs: None,
                           gss_inquire_saslname_for_mech:
                               Some(krb5_gss_inquire_saslname_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_OID,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_inquire_mech_for_saslname:
                               Some(krb5_gss_inquire_mech_for_saslname as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _: *mut gss_OID)
                                            -> OM_uint32),
                           gss_inquire_attrs_for_mech:
                               Some(krb5_gss_inquire_attrs_for_mech as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_const_OID,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_OID_set)
                                            -> OM_uint32),
                           gss_acquire_cred_from:
                               Some(krb5_gss_acquire_cred_from as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_store_cred_into:
                               Some(krb5_gss_store_cred_into as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _:
                                                                 gss_cred_usage_t,
                                                             _: gss_OID,
                                                             _: OM_uint32,
                                                             _: OM_uint32,
                                                             _:
                                                                 gss_const_key_value_set_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut gss_cred_usage_t)
                                            -> OM_uint32),
                           gssspi_acquire_cred_with_password:
                               Some(iakerb_gss_acquire_cred_with_password as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_name_t,
                                                             _: gss_buffer_t,
                                                             _: OM_uint32,
                                                             _: gss_OID_set,
                                                             _: libc::c_int,
                                                             _:
                                                                 *mut gss_cred_id_t,
                                                             _:
                                                                 *mut gss_OID_set,
                                                             _:
                                                                 *mut OM_uint32)
                                            -> OM_uint32),
                           gss_export_cred:
                               Some(krb5_gss_export_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_cred_id_t,
                                                             _: gss_buffer_t)
                                            -> OM_uint32),
                           gss_import_cred:
                               Some(krb5_gss_import_cred as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_buffer_t,
                                                             _:
                                                                 *mut gss_cred_id_t)
                                            -> OM_uint32),
                           gssspi_import_sec_context_by_mech: None,
                           gssspi_import_name_by_mech: None,
                           gssspi_import_cred_by_mech: None,
                           gss_get_mic_iov:
                               Some(iakerb_gss_get_mic_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_verify_mic_iov:
                               Some(iakerb_gss_verify_mic_iov as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _:
                                                                 *mut gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gss_get_mic_iov_length:
                               Some(iakerb_gss_get_mic_iov_length as
                                        unsafe extern "C" fn(_:
                                                                 *mut OM_uint32,
                                                             _: gss_ctx_id_t,
                                                             _: gss_qop_t,
                                                             _:
                                                                 *mut gss_iov_buffer_desc,
                                                             _: libc::c_int)
                                            -> OM_uint32),
                           gssspi_query_meta_data: None,
                           gssspi_exchange_meta_data: None,
                           gssspi_query_mechanism_info: None,};
            init
        }
    };
#[c2rust::src_loc = "1011:1"]
unsafe extern "C" fn gss_iakerbmechglue_init() -> libc::c_int {
    let mut mech_iakerb: gss_mech_config =
        gss_mech_config{kmodName: 0 as *mut libc::c_char,
                        uLibName: 0 as *mut libc::c_char,
                        mechNameStr: 0 as *mut libc::c_char,
                        optionStr: 0 as *mut libc::c_char,
                        dl_handle: 0 as *mut libc::c_void,
                        mech_type: 0 as *mut gss_OID_desc_struct,
                        mech: 0 as *mut gss_config,
                        priority: 0,
                        freeMech: 0,
                        is_interposer: 0,
                        int_mech_type: 0 as *mut gss_OID_desc_struct,
                        int_mech: 0 as *mut gss_config,
                        next: 0 as *mut gss_mech_config,};
    memset(&mut mech_iakerb as *mut gss_mech_config as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gss_mech_config>() as libc::c_ulong);
    mech_iakerb.mech = &mut iakerb_mechanism;
    mech_iakerb.mechNameStr =
        b"iakerb\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    mech_iakerb.mech_type = gss_mech_iakerb;
    gssint_register_mechinfo(&mut mech_iakerb);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1025:1"]
unsafe extern "C" fn gss_krb5mechglue_init() -> libc::c_int {
    let mut mech_krb5: gss_mech_config =
        gss_mech_config{kmodName: 0 as *mut libc::c_char,
                        uLibName: 0 as *mut libc::c_char,
                        mechNameStr: 0 as *mut libc::c_char,
                        optionStr: 0 as *mut libc::c_char,
                        dl_handle: 0 as *mut libc::c_void,
                        mech_type: 0 as *mut gss_OID_desc_struct,
                        mech: 0 as *mut gss_config,
                        priority: 0,
                        freeMech: 0,
                        is_interposer: 0,
                        int_mech_type: 0 as *mut gss_OID_desc_struct,
                        int_mech: 0 as *mut gss_config,
                        next: 0 as *mut gss_mech_config,};
    memset(&mut mech_krb5 as *mut gss_mech_config as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<gss_mech_config>() as libc::c_ulong);
    mech_krb5.mech = &mut krb5_mechanism;
    mech_krb5.mechNameStr =
        b"kerberos_v5\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    mech_krb5.mech_type = gss_mech_krb5;
    gssint_register_mechinfo(&mut mech_krb5);
    mech_krb5.mechNameStr =
        b"kerberos_v5_old\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    mech_krb5.mech_type = gss_mech_krb5_old;
    gssint_register_mechinfo(&mut mech_krb5);
    mech_krb5.mechNameStr =
        b"mskrb\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    mech_krb5.mech_type = gss_mech_krb5_wrong;
    gssint_register_mechinfo(&mut mech_krb5);
    return 0 as libc::c_int;
}
/* _GSS_STATIC_LINK */
#[no_mangle]
#[c2rust::src_loc = "1057:1"]
pub unsafe extern "C" fn gss_krb5int_lib_init() -> libc::c_int {
    let mut err: libc::c_int = 0;
    add_error_table(&et_k5g_error_table);
    err = k5_mutex_finish_init(&mut gssint_krb5_keytab_lock);
    if err != 0 { return err }
    /* LEAN_CLIENT */
    err =
        krb5int_key_register(K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME,
                             Some(free as
                                      unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> ()));
    if err != 0 { return err }
    err =
        krb5int_key_register(K5_KEY_GSS_KRB5_CCACHE_NAME,
                             Some(free as
                                      unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> ()));
    if err != 0 { return err }
    err =
        krb5int_key_register(K5_KEY_GSS_KRB5_ERROR_MESSAGE,
                             Some(krb5_gss_delete_error_info as
                                      unsafe extern "C" fn(_:
                                                               *mut libc::c_void)
                                          -> ()));
    if err != 0 { return err }
    err = k5_mutex_finish_init(&mut kg_kdc_flag_mutex);
    if err != 0 { return err }
    err = k5_mutex_finish_init(&mut kg_vdb.mutex);
    if err != 0 { return err }
    err = gss_krb5mechglue_init();
    if err != 0 { return err }
    err = gss_iakerbmechglue_init();
    if err != 0 { return err }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1102:1"]
pub unsafe extern "C" fn gss_krb5int_lib_fini() {
    remove_error_table(&et_k5g_error_table);
    krb5int_key_delete(K5_KEY_GSS_KRB5_SET_CCACHE_OLD_NAME);
    krb5int_key_delete(K5_KEY_GSS_KRB5_CCACHE_NAME);
    krb5int_key_delete(K5_KEY_GSS_KRB5_ERROR_MESSAGE);
    k5_os_mutex_destroy(&mut kg_vdb.mutex);
    k5_os_mutex_destroy(&mut kg_kdc_flag_mutex);
    k5_os_mutex_destroy(&mut gssint_krb5_keytab_lock);
    /* LEAN_CLIENT */
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2008 by the Massachusetts Institute of Technology.
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
 *
 */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/* work around sunos braindamage */
/* The include of gssapi_krb5.h will dtrt with the above #defines in
 * effect.
 */
/* for debugging */
/* * constants **/
/* Incorrect krb5 mech OID emitted by MS. */
/* IAKERB variant */
/* * CFX flags **/
/* These are to be stored in little-endian order, i.e., des-mac is
   stored as 02 00.  */
/* SGN_ALG_DES_MAC_MD5           = 0x0000, */
    /* SGN_ALG_MD2_5                 = 0x0001, */
    /* SGN_ALG_DES_MAC               = 0x0002, */
    /* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
/* microsoft w2k;  */
/* SEAL_ALG_DES             = 0x0000, */
    /* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
/* microsoft w2k;  */
/* for 3DES */
/* for draft-ietf-krb-wg-gssapi-cfx-01 */
/* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
/* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
/* * internal types **/
/* immutable */
/* immutable */
/* immutable */
/* protects ad_context only for now */
/* protect against simultaneous accesses */
/* name/type of credential */
/* keytab (accept) data */
/* ccache (init) data */
/* limit negotiated enctypes to this list */
/* nonzero if initiating, zero if accepting */
/* XXX tested but never actually set */
/* One of two potential keys to use with RFC 4121
                      * packets; this key must always be set. */
/* RFC 1964 encryption key; seq xored with a constant
                   * for DES, seq for other RFC 1964 enctypes  */
/* RFC 1964 sequencing key */
/* XXX these used to be signed.  the old spec is inspecific, and
       the new spec specifies unsigned.  I don't believe that the change
       affects the wire encoding. */
/* Protocol spec revision for sending packets
       0 => RFC 1964 with 3DES and RC4 enhancements
       1 => RFC 4121
       No others defined so far.  It is always permitted to receive
       tokens in RFC 4121 format.  If enc is non-null, receiving RFC
       1964 tokens is permitted.*/
/* for "main" subkey */
/* CFX only */
/* did we get rcache from creds? */
/* LEAN_CLIENT */
/* * helper functions **/
/* Encrypt length bytes at ptr in place, with the given key and usage.  If
 * iv is not NULL, use it as the cipher state. */
/* AEAD */
/* for conf len */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* exts */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/*exts */
/* LEAN_CLIENT */
/* minor_status */
/* context_handle */
/* desired_object */
/* data_set */
/* minor_status */
/* context_handle */
/* desired_object */
/* value */
/* minor_status */
/* context_handle */
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type */
/* message_context */
/* status_string */
/* minor_status */
/* mech_set */
/* minor_status */
/* name1 */
/* name2 */
/* name_equal */
/* minor_status */
/* input_name */
/* output_name_buffer */
/* output_name_type */
/* minor_status */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* minor_status */
/* context_handle */
/* initiator_name */
/* acceptor_name */
/* lifetime_rec */
/* mech_type */
/* ret_flags */
/* locally_initiated */
/* open */
/* New V2 entry points */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* LEAN_CLIENT */
/* minor_status */
/* oid */
/* minor_status */
/* oid */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* input_name */
/* mech_type */
/* output_name */
/* minor_status */
/* input_name */
/* exported_name */
/* minor_status */
/* input_name */
/* dest_name */
/* minor_status */
/* cred */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* context */
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* _GSS_STATIC_LINK */
#[no_mangle]
#[c2rust::src_loc = "1133:1"]
pub unsafe extern "C" fn gss_krb5int_initialize_library() -> OM_uint32 {
    return gssint_mechglue_initialize_library() as OM_uint32;
}
unsafe extern "C" fn run_static_initializers() {
    gss_mech_krb5 =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(0 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    gss_mech_krb5_old =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(1 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    gss_mech_krb5_wrong =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(2 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    gss_mech_iakerb =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(3 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    gss_nt_krb5_name =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(5 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    gss_nt_krb5_principal =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(6 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    GSS_KRB5_NT_PRINCIPAL_NAME =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(5 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    GSS_KRB5_CRED_NO_CI_FLAGS_X =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(7 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    GSS_KRB5_GET_CRED_IMPERSONATOR =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(8 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    GSS_KRB5_NT_ENTERPRISE_NAME =
        &mut *(krb5_gss_oid_array.as_ptr() as
                   gss_OID).offset(9 as libc::c_int as isize) as
            *mut gss_OID_desc_struct;
    oidsets =
        [{
             let mut init =
                 gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                         elements:
                                             &mut *(krb5_gss_oid_array.as_ptr()
                                                        as
                                                        gss_OID).offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                 as
                                                 *mut gss_OID_desc_struct,};
             init
         },
         {
             let mut init =
                 gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                         elements:
                                             &mut *(krb5_gss_oid_array.as_ptr()
                                                        as
                                                        gss_OID).offset(1 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                 as
                                                 *mut gss_OID_desc_struct,};
             init
         },
         {
             let mut init =
                 gss_OID_set_desc_struct{count: 3 as libc::c_int as size_t,
                                         elements:
                                             &mut *(krb5_gss_oid_array.as_ptr()
                                                        as
                                                        gss_OID).offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                 as
                                                 *mut gss_OID_desc_struct,};
             init
         },
         {
             let mut init =
                 gss_OID_set_desc_struct{count: 4 as libc::c_int as size_t,
                                         elements:
                                             &mut *(krb5_gss_oid_array.as_ptr()
                                                        as
                                                        gss_OID).offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize)
                                                 as
                                                 *mut gss_OID_desc_struct,};
             init
         }];
    gss_mech_set_krb5 =
        &mut *(oidsets.as_ptr() as
                   gss_OID_set).offset(0 as libc::c_int as isize) as
            *mut gss_OID_set_desc_struct;
    gss_mech_set_krb5_old =
        &mut *(oidsets.as_ptr() as
                   gss_OID_set).offset(1 as libc::c_int as isize) as
            *mut gss_OID_set_desc_struct;
    gss_mech_set_krb5_both =
        &mut *(oidsets.as_ptr() as
                   gss_OID_set).offset(2 as libc::c_int as isize) as
            *mut gss_OID_set_desc_struct;
    kg_all_mechs =
        &mut *(oidsets.as_ptr() as
                   gss_OID_set).offset(3 as libc::c_int as isize) as
            *mut gss_OID_set_desc_struct
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
