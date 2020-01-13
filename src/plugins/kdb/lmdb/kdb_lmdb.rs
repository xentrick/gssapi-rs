use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:79"]
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
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:79"]
pub mod sys_types_h {
    #[c2rust::src_loc = "69:1"]
    pub type mode_t = __mode_t;
    #[c2rust::src_loc = "85:1"]
    pub type off_t = __off_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::{__mode_t, __off_t, __ssize_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:79"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:79"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:79"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:79"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:79"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/stat.h:79"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:79"]
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
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
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
 * Add a buffer to a PAC handle.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] type             Buffer type
 * @param [in] data             contents
 *
 * This function adds a buffer of type @a type and contents @a data to @a pac
 * if there isn't already a buffer of this type present.
 *
 * The valid values of @a type is one of the following:
 * @li #KRB5_PAC_LOGON_INFO         -  Logon information
 * @li #KRB5_PAC_CREDENTIALS_INFO   -  Credentials information
 * @li #KRB5_PAC_SERVER_CHECKSUM    -  Server checksum
 * @li #KRB5_PAC_PRIVSVR_CHECKSUM   -  KDC checksum
 * @li #KRB5_PAC_CLIENT_INFO        -  Client name and ticket information
 * @li #KRB5_PAC_DELEGATION_INFO    -  Constrained delegation information
 * @li #KRB5_PAC_UPN_DNS_INFO       -  User principal name and DNS information
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a PAC handle.
 *
 * @param [in] context         Library context
 * @param [in] pac             PAC to be freed
 *
 * This function frees the contents of @a pac and the structure itself.
 */
    /* *
 * Retrieve a buffer value from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  type            Type of buffer to retrieve
 * @param [out] data            Buffer value
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return an array of buffer types in a PAC handle.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] len             Number of entries in @a types
 * @param [out] types           Array of buffer types
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create an empty Privilege Attribute Certificate (PAC) handle.
 *
 * @param [in]  context         Library context
 * @param [out] pac             New PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Unparse an encoded PAC into a new handle.
 *
 * @param [in]  context         Library context
 * @param [in]  ptr             PAC buffer
 * @param [in]  len             Length of @a ptr
 * @param [out] pac             PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 *
 * This function validates @a pac against the supplied @a server, @a privsvr,
 * @a principal and @a authtime.  If @a principal is NULL, the principal and
 * authtime are not verified.  If @a server or @a privsvr is NULL, the
 * corresponding checksum is not verified.
 *
 * If successful, @a pac is marked as verified.
 *
 * @note A checksum mismatch can occur if the PAC was copied from a cross-realm
 * TGT by an ignorant KDC; also macOS Server Open Directory (as of 10.6)
 * generates PACs with no server checksum at all.  One should consider not
 * failing the whole authentication because of this reason, but, instead,
 * treating the ticket as if it did not contain a PAC or marking the PAC
 * information as non-verified.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC, possibly from a specified realm.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 * @param [in] with_realm       If true, expect the realm of @a principal
 *
 * This function is similar to krb5_pac_verify(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field is
 * expected to include the realm of @a principal as well as the name.  This
 * flag is necessary to verify PACs in cross-realm S4U2Self referral TGTs.
 *
 * @version New in 1.17
 */
    /* *
 * Sign a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Expected principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [out] data            Signed PAC encoding
 *
 * This function signs @a pac using the keys @a server_key and @a privsvr_key
 * and returns the signed encoding in @a data.  @a pac is modified to include
 * the server and KDC checksum buffers.  Use krb5_free_data_contents() to free
 * @a data when it is no longer needed.
 *
 * @version New in 1.10
 */
    /* *
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
    /*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
    /* *
 * Allow the appplication to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    /* *
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
    /* *
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
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
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
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
    /* *< Transited encoding type */
    /* *< Contents */
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
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
        /* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        /* *
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:79"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb5_h::_kdb5_dal_handle;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
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
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/kdb5.h:81"]
pub mod kdb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct _kdb5_dal_handle {
        pub db_context: *mut libc::c_void,
        pub lib_handle: db_library,
        pub master_keylist: *mut krb5_keylist_node,
        pub master_princ: krb5_principal,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[c2rust::src_loc = "19:1"]
    pub type db_library = *mut _db_library;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "19:16"]
    pub struct _db_library {
        pub name: [libc::c_char; 128],
        pub reference_cnt: libc::c_int,
        pub dl_dir_handle: plugin_dir_handle,
        pub vftabl: kdb_vftabl,
        pub next: *mut _db_library,
        pub prev: *mut _db_library,
    }
    use super::kdb_h::{krb5_keylist_node, kdb_vftabl};
    use super::krb5_h::krb5_principal;
    use super::k5_plugin_h::plugin_dir_handle;
    /* end of _KRB5_KDB5_H_ */
    /* typedef kdb5_dal_handle is in k5-int.h now */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:80"]
pub mod kdb_h {
    #[c2rust::src_loc = "294:1"]
    pub type krb5_keylist_node = _krb5_keylist_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:16"]
    pub struct _krb5_keylist_node {
        pub keyblock: krb5_keyblock,
        pub kvno: krb5_kvno,
        pub next: *mut _krb5_keylist_node,
    }
    /*
 * A krb5_context can hold one database object.  Modules should use
 * krb5_db_set_context and krb5_db_get_context to store state associated with
 * the database object.
 *
 * Some module functions are mandatory for KDC operation; others are optional
 * or apply only to administrative operations.  If a function is optional, a
 * module can leave the function pointer as NULL.  Alternatively, modules can
 * return KRB5_PLUGIN_OP_NOTSUPP when asked to perform an inapplicable action.
 *
 * Some module functions have default implementations which will call back into
 * the vtable interface.  Leave these functions as NULL to use the default
 * implementations.
 *
 * The documentation in these comments describes the DAL as it is currently
 * implemented and used, not as it should be.  So if anything seems off, that
 * probably means the current state of things is off.
 *
 * Modules must allocate memory for principal entries, policy entries, and
 * other structures using an allocator compatible with malloc() as seen by
 * libkdb5 and libkrb5.  Modules may link against libkdb5 and call
 * krb5_db_alloc() to be certain that the same malloc implementation is used.
 */
    #[c2rust::src_loc = "923:1"]
    pub type kdb_vftabl = _kdb_vftabl;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "923:16"]
    pub struct _kdb_vftabl {
        pub maj_ver: libc::c_short,
        pub min_ver: libc::c_short,
        pub init_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub fini_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub init_module: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         *mut *mut libc::c_char,
                                                     _: libc::c_int)
                                    -> krb5_error_code>,
        pub fini_module: Option<unsafe extern "C" fn(_: krb5_context)
                                    -> krb5_error_code>,
        pub create: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut libc::c_char,
                                                _: *mut *mut libc::c_char)
                               -> krb5_error_code>,
        pub destroy: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut *mut libc::c_char)
                                -> krb5_error_code>,
        pub get_age: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut time_t)
                                -> krb5_error_code>,
        pub lock: Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int)
                             -> krb5_error_code>,
        pub unlock: Option<unsafe extern "C" fn(_: krb5_context)
                               -> krb5_error_code>,
        pub get_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_const_principal,
                                                       _: libc::c_uint,
                                                       _:
                                                           *mut *mut krb5_db_entry)
                                      -> krb5_error_code>,
        pub put_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut krb5_db_entry,
                                                       _:
                                                           *mut *mut libc::c_char)
                                      -> krb5_error_code>,
        pub delete_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub rename_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub iterate: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _:
                                                     Option<unsafe extern "C" fn(_:
                                                                                     krb5_pointer,
                                                                                 _:
                                                                                     *mut krb5_db_entry)
                                                                ->
                                                                    libc::c_int>,
                                                 _: krb5_pointer,
                                                 _: krb5_flags)
                                -> krb5_error_code>,
        pub create_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: osa_policy_ent_t)
                                      -> krb5_error_code>,
        pub get_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub put_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub iter_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         osa_adb_iter_policy_func,
                                                     _: *mut libc::c_void)
                                    -> krb5_error_code>,
        pub delete_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut libc::c_char)
                                      -> krb5_error_code>,
        pub fetch_master_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _: krb5_principal,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _: *mut krb5_kvno,
                                                          _:
                                                              *mut libc::c_char)
                                         -> krb5_error_code>,
        pub fetch_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *const krb5_keyblock,
                                                               _:
                                                                   *mut *mut krb5_keylist_node)
                                              -> krb5_error_code>,
        pub store_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *mut krb5_keylist_node,
                                                               _:
                                                                   *mut libc::c_char)
                                              -> krb5_error_code>,
        pub dbe_search_enctype: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut krb5_db_entry,
                                                            _:
                                                                *mut krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _:
                                                                *mut *mut krb5_key_data)
                                           -> krb5_error_code>,
        pub change_pwd: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut krb5_keyblock,
                                                    _:
                                                        *mut krb5_key_salt_tuple,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_char,
                                                    _: libc::c_int,
                                                    _: krb5_boolean,
                                                    _: *mut krb5_db_entry)
                                   -> krb5_error_code>,
        pub promote_db: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut *mut libc::c_char)
                                   -> krb5_error_code>,
        pub decrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_key_data,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _:
                                                              *mut krb5_keysalt)
                                         -> krb5_error_code>,
        pub encrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keysalt,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut krb5_key_data)
                                         -> krb5_error_code>,
        pub sign_authdata: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: libc::c_uint,
                                                       _:
                                                           krb5_const_principal,
                                                       _:
                                                           krb5_const_principal,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: krb5_timestamp,
                                                       _:
                                                           *mut *mut krb5_authdata,
                                                       _: *mut libc::c_void,
                                                       _:
                                                           *mut *mut *mut krb5_data,
                                                       _:
                                                           *mut *mut *mut krb5_authdata)
                                      -> krb5_error_code>,
        pub check_transited_realms: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data)
                                               -> krb5_error_code>,
        pub check_policy_as: Option<unsafe extern "C" fn(_: krb5_context,
                                                         _: *mut krb5_kdc_req,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _: krb5_timestamp,
                                                         _:
                                                             *mut *const libc::c_char,
                                                         _:
                                                             *mut *mut *mut krb5_pa_data)
                                        -> krb5_error_code>,
        pub check_policy_tgs: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *mut krb5_kdc_req,
                                                          _:
                                                              *mut krb5_db_entry,
                                                          _: *mut krb5_ticket,
                                                          _:
                                                              *mut *const libc::c_char,
                                                          _:
                                                              *mut *mut *mut krb5_pa_data)
                                         -> krb5_error_code>,
        pub audit_as_req: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: *mut krb5_kdc_req,
                                                      _: *const krb5_address,
                                                      _: *const krb5_address,
                                                      _: *mut krb5_db_entry,
                                                      _: *mut krb5_db_entry,
                                                      _: krb5_timestamp,
                                                      _: krb5_error_code)
                                     -> ()>,
        pub refresh_config: Option<unsafe extern "C" fn(_: krb5_context)
                                       -> ()>,
        pub check_allowed_to_delegate: Option<unsafe extern "C" fn(_:
                                                                       krb5_context,
                                                                   _:
                                                                       krb5_const_principal,
                                                                   _:
                                                                       *const krb5_db_entry,
                                                                   _:
                                                                       krb5_const_principal)
                                                  -> krb5_error_code>,
        pub free_principal_e_data: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_octet)
                                              -> ()>,
        pub get_s4u_x509_principal: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    krb5_const_principal,
                                                                _:
                                                                    libc::c_uint,
                                                                _:
                                                                    *mut *mut krb5_db_entry)
                                               -> krb5_error_code>,
        pub allowed_to_delegate_from: Option<unsafe extern "C" fn(_:
                                                                      krb5_context,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const krb5_db_entry)
                                                 -> krb5_error_code>,
        pub get_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                           _: libc::c_uint,
                                                           _:
                                                               *mut *mut krb5_authdata,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_db_entry,
                                                           _: krb5_timestamp,
                                                           _:
                                                               *mut *mut libc::c_void,
                                                           _:
                                                               *mut krb5_principal)
                                          -> krb5_error_code>,
        pub free_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    }
    /*
     * Mandatory: Invoked after the module library is loaded, when the first DB
     * using the module is opened, across all contexts.
     */
    /*
     * Mandatory: Invoked before the module library is unloaded, after the last
     * DB using the module is closed, across all contexts.
     */
    /*
     * Mandatory: Initialize a database object.  Profile settings should be
     * read from conf_section inside KDB_MODULE_SECTION.  db_args communicates
     * command-line arguments for module-specific flags.  mode will be one of
     * KRB5_KDB_OPEN_{RW,RO} or'd with one of
     * KRB5_KDB_SRV_TYPE_{KDC,ADMIN,PASSWD,OTHER}.
     */
    /*
     * Mandatory: Finalize the database object contained in a context.  Free
     * any state contained in the db_context pointer and null it out.
     */
    /*
     * Optional: Initialize a database object while creating the underlying
     * database.  conf_section and db_args have the same meaning as in
     * init_module.  This function may return an error if the database already
     * exists.  Used by kdb5_util create.
     *
     * If db_args contains the value "temporary", the module should create an
     * exclusively locked side copy of the database suitable for loading in a
     * propagation from master to replica.  This side copy will later be
     * promoted with promote_db, allowing complete updates of the DB with no
     * loss in read availability.  If the module cannot comply with this
     * architecture, it should return an error.
     */
    /*
     * Optional: Destroy a database.  conf_section and db_args have the same
     * meaning as in init_module.  Used by kdb5_util destroy.  In current
     * usage, the database is destroyed while open, so the module should handle
     * that.
     */
    /*
     * Deprecated: No longer used as of krb5 1.10; can be removed in the next
     * DAL revision.  Modules should leave as NULL.
     */
    /*
     * Optional: Lock the database, with semantics depending on the mode
     * argument:
     *
     * KRB5_DB_LOCKMODE_SHARED: Lock may coexist with other shared locks.
     * KRB5_DB_LOCKMODE_EXCLUSIVE: Lock may not coexist with other locks.
     * KRB5_DB_LOCKMODE_PERMANENT: Exclusive lock surviving process exit.
     *
     * Used by the "kadmin lock" command, incremental propagation, and
     * kdb5_util dump.  Incremental propagation support requires shared locks
     * to operate.  kdb5_util dump will continue unlocked if the module returns
     * KRB5_PLUGIN_OP_NOTSUPP.
     */
    /* Optional: Release a lock created with db_lock. */
    /*
     * Mandatory: Set *entry to an allocated entry for the principal
     * search_for.  If the principal is not found, return KRB5_KDB_NOENTRY.
     *
     * The meaning of flags are as follows:
     *
     * KRB5_KDB_FLAG_CANONICALIZE: Set by the KDC when looking up entries for
     *     an AS or TGS request with canonicalization requested.  Determines
     *     whether the module should return out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_INCLUDE_PAC: Set by the KDC during an AS request when the
     *     client requested PAC information during padata, and during most TGS
     *     requests.  Indicates that the module should include PAC information
     *     when its sign_authdata method is invoked.
     *
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY: Set by the KDC when looking up the
     *     client entry in an AS request.  Affects how the module should return
     *     out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_MAP_PRINCIPALS: Set by the KDC when looking up the client
     *     entry during TGS requests, except for S4U TGS requests and requests
     *     where the server entry has the KRB5_KDB_NO_AUTH_DATA_REQUIRED
     *     attribute.  Indicates that the module should map foreign principals
     *     to local principals if it supports doing so.
     *
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION: Set by the KDC when looking up the
     *     client entry during an S4U2Self TGS request.  This affects the PAC
     *     information which should be included when authorization data is
     *     generated; see the Microsoft S4U specification for details.
     *
     * KRB5_KDB_FLAG_CONSTRAINED_DELEGATION: Set by the KDC when looking up the
     *     client entry during an S4U2Proxy TGS request.  Also affects PAC
     *     generation.
     *
     * KRB5_KDB_FLAG_CROSS_REALM: Set by the KDC after looking up a server
     *     entry during a TGS request, if the header ticket was issued by a
     *     different realm.
     *
     * KRB5_KDB_FLAG_ISSUING_REFERRAL: Set by the KDC after looking up a server
     *     entry during a TGS request, if the requested server principal is not
     *     part of the realm being served, and a referral or alternate TGT will
     *     be issued instead.
     *
     * A module may return an in-realm alias by setting (*entry)->princ to the
     * canonical name.  The KDC will decide based on the request whether to use
     * the requested name or the canonical name in the issued ticket.
     *
     * A module can return a referral to another realm if
     * KRB5_KDB_FLAG_CANONICALIZE is set, or if
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is set and search_for->type is
     * KRB5_NT_ENTERPRISE_PRINCIPAL.  If KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is
     * set, the module should return a referral by simply filling in an
     * out-of-realm name in (*entry)->princ and setting all other fields to
     * NULL.  Otherwise, the module should return the entry for the cross-realm
     * TGS of the referred-to realm.  For TGS referals, the module can also
     * include tl-data of type KRB5_TL_SERVER_REFERRAL containing ASN.1-encoded
     * Windows referral data as documented in
     * draft-ietf-krb-wg-kerberos-referrals-11 appendix A; this will be
     * returned to the client as encrypted padata.
     */
    /*
     * Optional: Create or modify a principal entry.  db_args communicates
     * command-line arguments for module-specific flags.
     *
     * The mask field of an entry indicates the changed fields.  Mask values
     * are defined in kadmin's admin.h header.  If KADM5_PRINCIPAL is set in
     * the mask, the entry is new; otherwise it already exists.  All fields of
     * an entry are expected to contain correct values, regardless of whether
     * they are specified in the mask, so it is acceptable for a module to
     * ignore the mask and update the entire entry.
     */
    /*
     * Optional: Delete the entry for the principal search_for.  If the
     * principal did not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional with default: Rename a principal.  If the source principal does
     * not exist, return KRB5_KDB_NOENTRY.  If the target exists, return an
     * error.
     *
     * NOTE: If the module chooses to implement a custom function for renaming
     * a principal instead of using the default, then rename operations will
     * fail if iprop logging is enabled.
     */
    /*
     * Optional: For each principal entry in the database, invoke func with the
     * argments func_arg and the entry data.  If match_entry is specified, the
     * module may narrow the iteration to principal names matching that regular
     * expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Create a password policy entry.  Return an error if the policy
     * already exists.
     */
    /*
     * Optional: Set *policy to the policy entry of the specified name.  If the
     * entry does not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional: Modify an existing password policy entry to match the values
     * in policy.  Return an error if the policy does not already exist.
     */
    /*
     * Optional: For each password policy entry in the database, invoke func
     * with the argments data and the entry data.  If match_entry is specified,
     * the module may narrow the iteration to policy names matching that
     * regular expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Delete the password policy entry with the name policy.  Return
     * an error if the entry does not exist.
     */
    /*
     * Optional with default: Retrieve a master keyblock from the stash file
     * db_args, filling in *key and *kvno.  mname is the name of the master
     * principal for the realm.
     *
     * The default implementation reads the master keyblock from a keytab or
     * old-format stash file.
     */
    /*
     * Optional with default: Given a keyblock for some version of the
     * database's master key, fetch the decrypted master key values from the
     * database and store the list into *mkeys_list.  The caller will free
     * *mkeys_list using a libkdb5 function which uses the standard free()
     * function, so the module must not use a custom allocator.
     *
     * The caller may not know the version number of the master key it has, in
     * which case it will pass IGNORE_VNO.
     *
     * The default implementation ignores kvno and tries the key against the
     * current master key data and all KRB5_TL_MKEY_AUX values, which contain
     * copies of the master keys encrypted with old master keys.
     */
    /*
     * Optional with default: Save a list of master keyblocks, obtained from
     * fetch_master_key_list, into the stash file db_arg.  The caller will set
     * master_pwd to NULL, so the module should just ignore it.  mname is the
     * name of the master principal for the realm.
     *
     * The default implementation saves the list of master keys in a
     * keytab-format file.
     */
    /*
     * Optional with default: Starting at position *start, scan the key data of
     * a database entry for a key matching the enctype ktype, the salt type
     * stype, and the version kvno.  Store the resulting key into *kdatap and
     * set *start to the position after the key found.  If ktype is negative,
     * match any enctype.  If stype is negative, match any salt type.  If kvno
     * is zero or negative, find the most recent key version satisfying the
     * other constraints.
     */
    /*
     * Optional with default: Change the key data for db_entry to include keys
     * derived from the password passwd in each of the specified key-salt
     * types, at version new_kvno.  Discard the old key data if keepold is not
     * set.
     *
     * The default implementation uses the keyblock master_key to encrypt each
     * new key, via the function encrypt_key_data.
     */
    /*
     * Optional: Promote a temporary database to be the live one.  context must
     * be initialized with an exclusively locked database created with the
     * "temporary" db_arg.  On success, the database object contained in
     * context will be finalized.
     *
     * This method is used by kdb5_util load to replace the live database with
     * minimal loss of read availability.
     */
    /*
     * Optional with default: Decrypt the key in key_data with master keyblock
     * mkey, placing the result into dbkey.  Copy the salt from key_data, if
     * any, into keysalt.  Either dbkey or keysalt may be left unmodified on
     * successful return if key_data does not contain key or salt information.
     *
     * The default implementation expects the encrypted key (in krb5_c_encrypt
     * format) to be stored in key_data_contents[0], with length given by
     * key_data_length[0].  If key_data_ver is 2, it expects the salt to be
     * stored, unencrypted, in key_data_contents[1], with length given by
     * key_data_length[1].
     */
    /*
     * Optional with default: Encrypt dbkey with master keyblock mkey, placing
     * the result into key_data along with keysalt.
     *
     * The default implementation stores the encrypted key (in krb5_c_encrypt
     * format) in key_data_contents[0] and the length in key_data_length[0].
     * If keysalt is specified, it sets key_data_ver to 2, and stores the salt
     * in key_data_contents[1] and its length in key_data_length[1].  If
     * keysalt is not specified, key_data_ver is set to 1.
     */
    /*
     * Optional: Generate signed authorization data, such as a Windows PAC, for
     * the ticket to be returned to the client.  Place the signed authorization
     * data, if any, in *signed_auth_data.  This function will be invoked for
     * an AS request if the client included padata requesting a PAC.  This
     * function will be invoked for a TGS request if there is authorization
     * data in the TGT, if the client is from another realm, or if the TGS
     * request is an S4U2Self or S4U2Proxy request.  This function will not be
     * invoked during TGS requests if the server principal has the
     * no_auth_data_required attribute set.  Input parameters are:
     *
     *   flags: The flags used to look up the client principal.
     *
     *   client_princ: For S4U2Self and S4U2Proxy TGS requests, the client
     *     principal requested by the service; for regular TGS requests, the
     *     possibly-canonicalized client principal.
     *
     *   server_princ: The server principal in the request.
     *
     *   client: The DB entry of the client if it is in the local realm, NULL
     *     if not.  For S4U2Self and S4U2Proxy TGS requests, this is the DB
     *     entry for the client principal requested by the service.
     *
     *   server: The DB entry of the service principal, or of a cross-realm
     *     krbtgt principal in case of referral.
     *
     *   header_server: For S4U2Proxy requests, the DB entry of the second
     *     ticket server.  For other TGS requests, the DB entry of the header
     *     ticket server.  For AS requests, NULL.
     *
     *   local_tgt: the DB entry of the local krbtgt principal.
     *
     *   client_key: The reply key for the KDC request, before any FAST armor
     *     is applied.  For AS requests, this may be the client's long-term key
     *     or a key chosen by a preauth mechanism.  For TGS requests, this may
     *     be the subkey found in the AP-REQ or the session key of the TGT.
     *
     *   server_key: The server key used to encrypt the returned ticket.
     *
     *   header_key: For S4U2Proxy requests, the key used to decrypt the second
     *     ticket.  For TGS requests, the key used to decrypt the header
     *     ticket.  For AS requests, NULL.
     *
     *   local_tgt_key: The decrypted first key of local_tgt.
     *
     *   session_key: The session key of the ticket being granted to the
     *     requestor.
     *
     *   authtime: The timestamp of the original client authentication time.
     *     For AS requests, this is the current time.  For TGS requests, this
     *     is the authtime of the subject ticket (TGT or S4U2Proxy evidence
     *     ticket).
     *
     *   tgt_auth_data: For TGS requests, the authorization data present in the
     *     subject ticket.  For AS requests, NULL.
     *
     *   ad_info: For TGS requests, the parsed authorization data if obtained
     *     by get_authdata_info method from the authorization data present in
     *     the subject ticket.  Otherwise NULL.
     *
     *   auth_indicators: Points to NULL or a null-terminated list of krb5_data
     *     pointers, each containing an authentication indicator (RFC 8129).
     *     The method may modify this list, or free it and replace
     *     *auth_indicators with NULL, to change which auth indicators will be
     *     included in the ticket.
     */
    /*
     * Optional: Perform a policy check on a cross-realm ticket's transited
     * field.  Return 0 if the check authoritatively succeeds,
     * KRB5_PLUGIN_NO_HANDLE to use the core transited-checking mechanisms, or
     * another error (other than KRB5_PLUGIN_OP_NOTSUPP) if the check fails.
     */
    /*
     * Optional: Perform a policy check on an AS request, in addition to the
     * standard policy checks.  Return 0 if the AS request is allowed.  If the
     * AS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     */
    /*
     * Optional: Perform a policy check on a TGS request, in addition to the
     * standard policy checks.  Return 0 if the TGS request is allowed.  If the
     * TGS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     * The input parameter ticket contains the TGT used in the TGS request.
     */
    /*
     * Optional: This method informs the module of a successful or unsuccessful
     * AS request.
     */
    /* Note: there is currently no method for auditing TGS requests. */
    /*
     * Optional: This method informs the module of a request to reload
     * configuration or other state (that is, the KDC received a SIGHUP).
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  (Note that proxy is the target of the
     * delegation, not the delegating service; the term "proxy" is from the
     * viewpoint of the delegating service asking another service to perform
     * some of its work in the authentication context of the client.  This
     * terminology comes from the Microsoft S4U protocol documentation.)
     * Return 0 if policy allows it, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.  If this method is not implemented, all
     * S4U2Proxy delegation requests will be rejected.
     */
    /*
     * Optional: Free the e_data pointer of a database entry.  If this method
     * is not implemented, the e_data pointer in principal entries will be
     * freed with free() as seen by libkdb5.
     */
    /*
     * Optional: get a principal entry for S4U2Self based on X509 certificate.
     *
     * If flags include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ->realm
     * indicates the request realm, but the data components should be ignored.
     * The module can return an out-of-realm client referral as it would for
     * get_principal().
     *
     * If flags does not include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ is
     * from PA-S4U-X509-USER.  If it contains data components (and not just a
     * realm), the module should verify that it is the same as the lookup
     * result for client_cert.  The module should not return a referral.
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  This method is similar to
     * check_allowed_to_delegate, but it operates on the target server DB entry
     * (called "proxy" here as in Microsoft's protocol documentation) rather
     * than the intermediate server entry.  server_ad_info represents the
     * authdata of the intermediate server, as returned by the
     * get_authdata_info method on the header ticket.  Return 0 if policy
     * allows the delegation, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.
     *
     * This method is called for S4U2Proxy requests and implements the
     * resource-based constrained delegation variant, which can support
     * cross-realm delegation.  If this method is not implemented or if it
     * returns a policy error, the KDC will fall back to
     * check_allowed_to_delegate if the intermediate and target servers are in
     * the same realm and the evidence ticket is forwardable.
     */
    /*
     * Optional: Perform verification and policy checks on authorization data,
     * such as a Windows PAC, based on the request client lookup flags.  Return
     * 0 if all checks have passed.  Optionally return a representation of the
     * authdata in *ad_info_out, to be consumed by allowed_to_delegate_from and
     * sign_authdata.  If client_out is not NULL, set *client_out to the client
     * name in the PAC; this indicates the requested client principal for a
     * cross-realm S4U2Proxy request.
     *
     * This method is called for TGS requests on the authorization data from
     * the header ticket.  For S4U2Proxy requests it is also called on the
     * authorization data from the evidence ticket.  If the
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION bit is set in flags, the authdata is
     * from the header ticket of an S4U2Self referral request, and the supplied
     * client_princ is the requested client.
     */
    /* End of minor version 0 for major version 8. */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* NOT saved */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "237:1"]
    pub type osa_adb_iter_policy_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: osa_policy_ent_t)
                   -> ()>;
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_rec = _osa_policy_ent_t;
    use super::krb5_h::{krb5_keyblock, krb5_kvno, krb5_error_code,
                        krb5_context, krb5_const_principal, krb5_pointer,
                        krb5_flags, krb5_principal, krb5_int32, krb5_boolean,
                        krb5_timestamp, krb5_authdata, krb5_data,
                        krb5_kdc_req, krb5_pa_data, krb5_ticket, krb5_address,
                        krb5_octet, krb5_magic, krb5_ui_2, krb5_ui_4,
                        krb5_deltat, krb5_int16, krb5_enctype};
    use super::time_t_h::time_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "837:1"]
        pub fn krb5_db_free_policy(kcontext: krb5_context,
                                   policy: osa_policy_ent_t);
    }
    /* Length, data */
    /* no longer used */
    /* Only valid if version > 1 */
    /* pwdMaxFailure */
    /* pwdFailureCountInterval */
    /* pwdLockoutDuration */
    /* Only valid if version > 2 */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:79"]
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
        #[c2rust::src_loc = "84:1"]
        pub fn profile_get_integer(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn profile_get_boolean(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn profile_release_string(str: *mut libc::c_char);
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/lmdb.h:83"]
pub mod lmdb_h {
    #[c2rust::src_loc = "241:1"]
    pub type MDB_dbi = libc::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "257:16"]
    pub struct MDB_val {
        pub mv_size: size_t,
        pub mv_data: *mut libc::c_void,
    }
    #[c2rust::src_loc = "178:1"]
    pub type mdb_mode_t = mode_t;
    #[c2rust::src_loc = "366:9"]
    pub type MDB_cursor_op = libc::c_uint;
    #[c2rust::src_loc = "393:2"]
    pub const MDB_PREV_MULTIPLE: MDB_cursor_op = 18;
    #[c2rust::src_loc = "392:2"]
    pub const MDB_SET_RANGE: MDB_cursor_op = 17;
    #[c2rust::src_loc = "391:2"]
    pub const MDB_SET_KEY: MDB_cursor_op = 16;
    #[c2rust::src_loc = "390:2"]
    pub const MDB_SET: MDB_cursor_op = 15;
    #[c2rust::src_loc = "389:2"]
    pub const MDB_PREV_NODUP: MDB_cursor_op = 14;
    #[c2rust::src_loc = "387:2"]
    pub const MDB_PREV_DUP: MDB_cursor_op = 13;
    #[c2rust::src_loc = "386:2"]
    pub const MDB_PREV: MDB_cursor_op = 12;
    #[c2rust::src_loc = "385:2"]
    pub const MDB_NEXT_NODUP: MDB_cursor_op = 11;
    #[c2rust::src_loc = "382:2"]
    pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = 10;
    #[c2rust::src_loc = "380:2"]
    pub const MDB_NEXT_DUP: MDB_cursor_op = 9;
    #[c2rust::src_loc = "379:2"]
    pub const MDB_NEXT: MDB_cursor_op = 8;
    #[c2rust::src_loc = "377:2"]
    pub const MDB_LAST_DUP: MDB_cursor_op = 7;
    #[c2rust::src_loc = "376:2"]
    pub const MDB_LAST: MDB_cursor_op = 6;
    #[c2rust::src_loc = "373:2"]
    pub const MDB_GET_MULTIPLE: MDB_cursor_op = 5;
    #[c2rust::src_loc = "372:2"]
    pub const MDB_GET_CURRENT: MDB_cursor_op = 4;
    #[c2rust::src_loc = "371:2"]
    pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = 3;
    #[c2rust::src_loc = "370:2"]
    pub const MDB_GET_BOTH: MDB_cursor_op = 2;
    #[c2rust::src_loc = "368:2"]
    pub const MDB_FIRST_DUP: MDB_cursor_op = 1;
    #[c2rust::src_loc = "367:2"]
    pub const MDB_FIRST: MDB_cursor_op = 0;
    use super::stddef_h::size_t;
    use super::sys_types_h::mode_t;
    extern "C" {
        #[c2rust::src_loc = "238:16"]
        pub type MDB_txn;
        #[c2rust::src_loc = "231:16"]
        pub type MDB_env;
        #[c2rust::src_loc = "244:16"]
        pub type MDB_cursor;
        #[no_mangle]
        #[c2rust::src_loc = "495:1"]
        pub fn mdb_strerror(err: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "508:1"]
        pub fn mdb_env_create(env: *mut *mut MDB_env) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "631:1"]
        pub fn mdb_env_open(env: *mut MDB_env, path: *const libc::c_char,
                            flags: libc::c_uint, mode: mdb_mode_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "753:1"]
        pub fn mdb_env_close(env: *mut MDB_env);
        #[no_mangle]
        #[c2rust::src_loc = "845:1"]
        pub fn mdb_env_set_mapsize(env: *mut MDB_env, size: size_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "864:1"]
        pub fn mdb_env_set_maxreaders(env: *mut MDB_env,
                                      readers: libc::c_uint) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "896:1"]
        pub fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "973:1"]
        pub fn mdb_txn_begin(env: *mut MDB_env, parent: *mut MDB_txn,
                             flags: libc::c_uint, txn: *mut *mut MDB_txn)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1008:1"]
        pub fn mdb_txn_commit(txn: *mut MDB_txn) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1018:1"]
        pub fn mdb_txn_abort(txn: *mut MDB_txn);
        #[no_mangle]
        #[c2rust::src_loc = "1037:1"]
        pub fn mdb_txn_reset(txn: *mut MDB_txn);
        #[no_mangle]
        #[c2rust::src_loc = "1053:1"]
        pub fn mdb_txn_renew(txn: *mut MDB_txn) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1128:1"]
        pub fn mdb_dbi_open(txn: *mut MDB_txn, name: *const libc::c_char,
                            flags: libc::c_uint, dbi: *mut MDB_dbi)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1180:1"]
        pub fn mdb_drop(txn: *mut MDB_txn, dbi: MDB_dbi, del_0: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1288:1"]
        pub fn mdb_get(txn: *mut MDB_txn, dbi: MDB_dbi, key: *mut MDB_val,
                       data: *mut MDB_val) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1337:1"]
        pub fn mdb_put(txn: *mut MDB_txn, dbi: MDB_dbi, key: *mut MDB_val,
                       data: *mut MDB_val, flags: libc::c_uint)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1362:1"]
        pub fn mdb_del(txn: *mut MDB_txn, dbi: MDB_dbi, key: *mut MDB_val,
                       data: *mut MDB_val) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1386:1"]
        pub fn mdb_cursor_open(txn: *mut MDB_txn, dbi: MDB_dbi,
                               cursor: *mut *mut MDB_cursor) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "1394:1"]
        pub fn mdb_cursor_close(cursor: *mut MDB_cursor);
        #[no_mangle]
        #[c2rust::src_loc = "1445:1"]
        pub fn mdb_cursor_get(cursor: *mut MDB_cursor, key: *mut MDB_val,
                              data: *mut MDB_val, op: MDB_cursor_op)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:79"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:79"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:79"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:79"]
pub mod unistd_h {
    use super::types_h::__off_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "334:1"]
        pub fn lseek(__fd: libc::c_int, __offset: __off_t,
                     __whence: libc::c_int) -> __off_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:79"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:79"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:79"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:79"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/kdb/lmdb/klmdb-int.h:82"]
pub mod klmdb_int_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_timestamp,
                        krb5_boolean};
    use super::kdb_h::{krb5_db_entry, osa_policy_ent_rec, osa_policy_ent_t};
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "39:1"]
        pub fn klmdb_encode_princ(context: krb5_context,
                                  entry: *const krb5_db_entry,
                                  enc_out: *mut *mut uint8_t,
                                  len_out: *mut size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn klmdb_encode_princ_lockout(context: krb5_context,
                                          entry: *const krb5_db_entry,
                                          buf: *mut uint8_t);
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn klmdb_encode_policy(context: krb5_context,
                                   pol: *const osa_policy_ent_rec,
                                   enc_out: *mut *mut uint8_t,
                                   len_out: *mut size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn klmdb_decode_princ(context: krb5_context,
                                  key: *const libc::c_void, key_len: size_t,
                                  enc: *const libc::c_void, enc_len: size_t,
                                  entry_out: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn klmdb_decode_princ_lockout(context: krb5_context,
                                          entry: *mut krb5_db_entry,
                                          buf: *const uint8_t);
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn klmdb_decode_policy(context: krb5_context,
                                   key: *const libc::c_void, key_len: size_t,
                                   enc: *const libc::c_void, enc_len: size_t,
                                   pol_out: *mut osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn klmdb_lockout_check_policy(context: krb5_context,
                                          entry: *mut krb5_db_entry,
                                          stamp: krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn klmdb_lockout_audit(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   stamp: krb5_timestamp,
                                   status: krb5_error_code,
                                   disable_last_success: krb5_boolean,
                                   disable_lockout: krb5_boolean)
         -> krb5_error_code;
    }
    /* LMDB_INT_H */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __time_t, __blksize_t,
                        __blkcnt_t, __ssize_t, __syscall_slong_t};
pub use self::sys_types_h::{mode_t, off_t, ssize_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timespec_h::timespec;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::stat_h::stat;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_keyblock,
                       _krb5_keyblock, krb5_authdata, _krb5_authdata,
                       krb5_kdc_req, _krb5_kdc_req, krb5_ticket, _krb5_ticket,
                       krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_ticket_times, _krb5_ticket_times, krb5_transited,
                       _krb5_transited, krb5_enc_data, _krb5_enc_data,
                       krb5_pa_data, _krb5_pa_data, _profile_t,
                       krb5_unparse_name, krb5_free_unparsed_name,
                       krb5_set_error_message, krb5_prepend_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5alloc, k5calloc, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::kdb5_h::{_kdb5_dal_handle, db_library, _db_library};
pub use self::kdb_h::{krb5_keylist_node, _krb5_keylist_node, kdb_vftabl,
                      _kdb_vftabl, krb5_db_entry, _krb5_db_entry_new,
                      krb5_key_data, _krb5_key_data, krb5_tl_data,
                      _krb5_tl_data, krb5_keysalt, _krb5_keysalt,
                      krb5_key_salt_tuple, __krb5_key_salt_tuple,
                      osa_adb_iter_policy_func, osa_policy_ent_t,
                      _osa_policy_ent_t, osa_policy_ent_rec,
                      krb5_db_free_principal, krb5_db_free_policy};
pub use self::profile_h::{profile_t, profile_get_string, profile_get_integer,
                          profile_get_boolean, profile_release_string};
pub use self::lmdb_h::{MDB_dbi, MDB_val, mdb_mode_t, MDB_cursor_op,
                       MDB_PREV_MULTIPLE, MDB_SET_RANGE, MDB_SET_KEY, MDB_SET,
                       MDB_PREV_NODUP, MDB_PREV_DUP, MDB_PREV, MDB_NEXT_NODUP,
                       MDB_NEXT_MULTIPLE, MDB_NEXT_DUP, MDB_NEXT,
                       MDB_LAST_DUP, MDB_LAST, MDB_GET_MULTIPLE,
                       MDB_GET_CURRENT, MDB_GET_BOTH_RANGE, MDB_GET_BOTH,
                       MDB_FIRST_DUP, MDB_FIRST, MDB_txn, MDB_env, MDB_cursor,
                       mdb_strerror, mdb_env_create, mdb_env_open,
                       mdb_env_close, mdb_env_set_mapsize,
                       mdb_env_set_maxreaders, mdb_env_set_maxdbs,
                       mdb_txn_begin, mdb_txn_commit, mdb_txn_abort,
                       mdb_txn_reset, mdb_txn_renew, mdb_dbi_open, mdb_drop,
                       mdb_get, mdb_put, mdb_del, mdb_cursor_open,
                       mdb_cursor_close, mdb_cursor_get};
use self::stdio_h::asprintf;
use self::fcntl_h::{fcntl, open};
use self::errno_h::__errno_location;
use self::unistd_h::{lseek, close, read, write, unlink};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, calloc};
use self::string_h::{strlen, strncmp, strcmp, memcmp, memset};
use self::sys_stat_h::{stat, fstat};
use self::klmdb_int_h::{klmdb_encode_princ, klmdb_encode_princ_lockout,
                        klmdb_encode_policy, klmdb_decode_princ,
                        klmdb_decode_princ_lockout, klmdb_decode_policy,
                        klmdb_lockout_check_policy, klmdb_lockout_audit};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "97:9"]
pub struct klmdb_context {
    pub path: *mut libc::c_char,
    pub lockout_path: *mut libc::c_char,
    pub temporary: krb5_boolean,
    pub merge_nra: krb5_boolean,
    pub disable_last_success: krb5_boolean,
    pub disable_lockout: krb5_boolean,
    pub nosync: krb5_boolean,
    pub mapsize: size_t,
    pub maxreaders: libc::c_uint,
    pub env: *mut MDB_env,
    pub lockout_env: *mut MDB_env,
    pub princ_db: MDB_dbi,
    pub policy_db: MDB_dbi,
    pub lockout_db: MDB_dbi,
    pub read_txn: *mut MDB_txn,
    pub load_txn: *mut MDB_txn,
}
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn klerr(mut context: krb5_context, mut err: libc::c_int,
                           mut msg: *const libc::c_char) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    /* Pass through system errors; map MDB errors to a com_err code. */
    ret =
        if err > 0 as libc::c_int {
            err as libc::c_long
        } else { -(1780008412 as libc::c_long) } as krb5_error_code;
    krb5_set_error_message(context, ret,
                           dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"%s (path: %s): %s\x00" as *const u8 as
                                        *const libc::c_char), msg,
                           (*dbc).path, mdb_strerror(err));
    return ret;
}
/* Using db_args and the profile, create a DB context inside context and
 * initialize its configurable parameters. */
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn configure_context(mut context: krb5_context,
                                       mut conf_section: *const libc::c_char,
                                       mut db_args: *const *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context = 0 as *mut klmdb_context;
    let mut pval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut profile: profile_t = (*context).profile;
    let mut i: libc::c_int = 0;
    let mut bval: libc::c_int = 0;
    let mut ival: libc::c_int = 0;
    dbc =
        k5alloc(::std::mem::size_of::<klmdb_context>() as libc::c_ulong,
                &mut ret) as *mut klmdb_context;
    if dbc.is_null() { return ret }
    (*(*context).dal_handle).db_context = dbc as *mut libc::c_void;
    i = 0 as libc::c_int;
    loop  {
        if !(!db_args.is_null() && !(*db_args.offset(i as isize)).is_null()) {
            current_block = 2838571290723028321;
            break ;
        }
        if strcmp(*db_args.offset(i as isize),
                  b"temporary\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*dbc).temporary = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*db_args.offset(i as isize),
                         b"merge_nra\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            (*dbc).merge_nra = 1 as libc::c_int as krb5_boolean
        } else if strncmp(*db_args.offset(i as isize),
                          b"dbname=\x00" as *const u8 as *const libc::c_char,
                          7 as libc::c_int as libc::c_ulong) ==
                      0 as libc::c_int {
            path =
                (*db_args.offset(i as
                                     isize)).offset(7 as libc::c_int as isize)
        } else {
            ret = 22 as libc::c_int;
            krb5_set_error_message(context, ret,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Unsupported argument \"%s\" for LMDB\x00"
                                                as *const u8 as
                                                *const libc::c_char),
                                   *db_args.offset(i as isize));
            current_block = 7054059323154606855;
            break ;
        }
        i += 1
    }
    match current_block {
        2838571290723028321 => {
            if path.is_null() {
                /* Check for database_name in the db_module section. */
                ret =
                    profile_get_string(profile,
                                       b"dbmodules\x00" as *const u8 as
                                           *const libc::c_char, conf_section,
                                       b"database_name\x00" as *const u8 as
                                           *const libc::c_char,
                                       0 as *const libc::c_char, &mut pval) as
                        krb5_error_code;
                if ret == 0 && pval.is_null() {
                    /* For compatibility, check for database_name in the realm. */
                    ret =
                        profile_get_string(profile,
                                           b"realms\x00" as *const u8 as
                                               *const libc::c_char,
                                           (*context).default_realm,
                                           b"database_name\x00" as *const u8
                                               as *const libc::c_char,
                                           b"/usr/local/var/krb5kdc/principal\x00"
                                               as *const u8 as
                                               *const libc::c_char, &mut pval)
                            as krb5_error_code
                }
                if ret != 0 {
                    current_block = 7054059323154606855;
                } else { path = pval; current_block = 15125582407903384992; }
            } else { current_block = 15125582407903384992; }
            match current_block {
                7054059323154606855 => { }
                _ => {
                    if asprintf(&mut (*dbc).path as *mut *mut libc::c_char,
                                b"%s.mdb\x00" as *const u8 as
                                    *const libc::c_char, path) <
                           0 as libc::c_int {
                        (*dbc).path = 0 as *mut libc::c_char;
                        ret = 12 as libc::c_int
                    } else if asprintf(&mut (*dbc).lockout_path as
                                           *mut *mut libc::c_char,
                                       b"%s.lockout.mdb\x00" as *const u8 as
                                           *const libc::c_char, path) <
                                  0 as libc::c_int {
                        (*dbc).lockout_path = 0 as *mut libc::c_char;
                        ret = 12 as libc::c_int
                    } else {
                        ret =
                            profile_get_boolean(profile,
                                                b"dbmodules\x00" as *const u8
                                                    as *const libc::c_char,
                                                conf_section,
                                                b"disable_last_success\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                0 as libc::c_int, &mut bval)
                                as krb5_error_code;
                        if !(ret != 0) {
                            (*dbc).disable_last_success =
                                bval as krb5_boolean;
                            ret =
                                profile_get_boolean(profile,
                                                    b"dbmodules\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    conf_section,
                                                    b"disable_lockout\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    0 as libc::c_int,
                                                    &mut bval) as
                                    krb5_error_code;
                            if !(ret != 0) {
                                (*dbc).disable_lockout = bval as krb5_boolean;
                                ret =
                                    profile_get_integer(profile,
                                                        b"dbmodules\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        conf_section,
                                                        b"mapsize\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        128 as libc::c_int,
                                                        &mut ival) as
                                        krb5_error_code;
                                if !(ret != 0) {
                                    (*dbc).mapsize =
                                        (ival as
                                             size_t).wrapping_mul(1024 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_mul(1024
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong);
                                    ret =
                                        profile_get_integer(profile,
                                                            b"dbmodules\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            conf_section,
                                                            b"max_readers\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char,
                                                            0 as libc::c_int,
                                                            &mut ival) as
                                            krb5_error_code;
                                    if !(ret != 0) {
                                        (*dbc).maxreaders =
                                            ival as libc::c_uint;
                                        ret =
                                            profile_get_boolean(profile,
                                                                b"dbmodules\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                conf_section,
                                                                b"nosync\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char,
                                                                0 as
                                                                    libc::c_int,
                                                                &mut bval) as
                                                krb5_error_code;
                                        if !(ret != 0) {
                                            (*dbc).nosync =
                                                bval as krb5_boolean
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    profile_release_string(pval);
    return ret;
}
#[c2rust::src_loc = "232:1"]
unsafe extern "C" fn open_lmdb_env(mut context: krb5_context,
                                   mut dbc: *mut klmdb_context,
                                   mut is_lockout: krb5_boolean,
                                   mut readonly: krb5_boolean,
                                   mut env_out: *mut *mut MDB_env)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut path: *const libc::c_char =
        if is_lockout != 0 { (*dbc).lockout_path } else { (*dbc).path };
    let mut flags: libc::c_uint = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut err: libc::c_int = 0;
    *env_out = 0 as *mut MDB_env;
    err = mdb_env_create(&mut env);
    if !(err != 0) {
        /* Use a pair of files instead of a subdirectory. */
        flags = 0x4000 as libc::c_int as libc::c_uint;
        /*
     * For the primary database, tie read transaction locktable slots to the
     * transaction and not the thread, so read transactions for iteration
     * cursors can coexist with short-lived transactions for operations invoked
     * by the iteration callback..
     */
        if is_lockout == 0 {
            flags |= 0x200000 as libc::c_int as libc::c_uint
        }
        if readonly != 0 { flags |= 0x20000 as libc::c_int as libc::c_uint }
        /* Durability for lockout records is never worth the performance penalty.
     * For the primary environment it might be, so we make it configurable. */
        if is_lockout != 0 || (*dbc).nosync != 0 {
            flags |= 0x10000 as libc::c_int as libc::c_uint
        }
        /* We use one database in the lockout env, two in the primary env. */
        err =
            mdb_env_set_maxdbs(env,
                               if is_lockout != 0 {
                                   1 as libc::c_int
                               } else { 2 as libc::c_int } as MDB_dbi);
        if !(err != 0) {
            if (*dbc).mapsize != 0 {
                err = mdb_env_set_mapsize(env, (*dbc).mapsize);
                if err != 0 {
                    current_block = 4238956734571425678;
                } else { current_block = 15652330335145281839; }
            } else { current_block = 15652330335145281839; }
            match current_block {
                4238956734571425678 => { }
                _ => {
                    if (*dbc).maxreaders != 0 {
                        err = mdb_env_set_maxreaders(env, (*dbc).maxreaders);
                        if err != 0 {
                            current_block = 4238956734571425678;
                        } else { current_block = 224731115979188411; }
                    } else { current_block = 224731115979188411; }
                    match current_block {
                        4238956734571425678 => { }
                        _ => {
                            err =
                                mdb_env_open(env, path, flags,
                                             (0o400 as libc::c_int |
                                                  0o200 as libc::c_int) as
                                                 mdb_mode_t);
                            if !(err != 0) {
                                *env_out = env;
                                return 0 as libc::c_int
                            }
                        }
                    }
                }
            }
        }
    }
    ret =
        klerr(context, err,
              dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"LMDB environment open failure\x00" as *const u8 as
                           *const libc::c_char));
    mdb_env_close(env);
    return ret;
}
/* Read a key from the primary environment, using a saved read transaction from
 * the database context.  Return KRB5_KDB_NOENTRY if the key is not found. */
#[c2rust::src_loc = "301:1"]
unsafe extern "C" fn fetch(mut context: krb5_context, mut db: MDB_dbi,
                           mut key: *mut MDB_val, mut val_out: *mut MDB_val)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut err: libc::c_int = 0;
    if (*dbc).read_txn.is_null() {
        err =
            mdb_txn_begin((*dbc).env, 0 as *mut MDB_txn,
                          0x20000 as libc::c_int as libc::c_uint,
                          &mut (*dbc).read_txn)
    } else { err = mdb_txn_renew((*dbc).read_txn) }
    if err == 0 { err = mdb_get((*dbc).read_txn, db, key, val_out) }
    if err == -(30798 as libc::c_int) {
        ret = -(1780008443 as libc::c_long) as krb5_error_code
    } else if err != 0 {
        ret =
            klerr(context, err,
                  dgettext(b"mit-krb5\x00" as *const u8 as
                               *const libc::c_char,
                           b"LMDB read failure\x00" as *const u8 as
                               *const libc::c_char))
    }
    mdb_txn_reset((*dbc).read_txn);
    return ret;
}
/* If we are using a lockout database, try to fetch the lockout attributes for
 * key and set them in entry. */
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn fetch_lockout(mut context: krb5_context,
                                   mut key: *mut MDB_val,
                                   mut entry: *mut krb5_db_entry) {
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut err: libc::c_int = 0;
    if (*dbc).lockout_env.is_null() { return }
    err =
        mdb_txn_begin((*dbc).lockout_env, 0 as *mut MDB_txn,
                      0x20000 as libc::c_int as libc::c_uint, &mut txn);
    if err == 0 { err = mdb_get(txn, (*dbc).lockout_db, key, &mut val) }
    if err == 0 && val.mv_size >= 12 as libc::c_int as libc::c_ulong {
        klmdb_decode_princ_lockout(context, entry,
                                   val.mv_data as *const uint8_t);
    }
    mdb_txn_abort(txn);
}
/*
 * Store a value for key in the specified database within the primary
 * environment.  Use the saved load transaction if one is present, or a
 * temporary write transaction if not.  If no_overwrite is true and the key
 * already exists, return KRB5_KDB_INUSE.  If must_overwrite is true and the
 * key does not already exist, return KRB5_KDB_NOENTRY.
 */
#[c2rust::src_loc = "352:1"]
unsafe extern "C" fn put(mut context: krb5_context, mut db: MDB_dbi,
                         mut keystr: *mut libc::c_char,
                         mut bytes: *mut uint8_t, mut len: size_t,
                         mut no_overwrite: krb5_boolean,
                         mut must_overwrite: krb5_boolean)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut putflags: libc::c_uint =
        if no_overwrite != 0 { 0x10 as libc::c_int } else { 0 as libc::c_int }
            as libc::c_uint;
    let mut temp_txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut key: MDB_val =
        {
            let mut init =
                MDB_val{mv_size: strlen(keystr),
                        mv_data: keystr as *mut libc::c_void,};
            init
        };
    let mut val: MDB_val =
        {
            let mut init =
                MDB_val{mv_size: len, mv_data: bytes as *mut libc::c_void,};
            init
        };
    let mut dummy: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut err: libc::c_int = 0;
    if !(*dbc).load_txn.is_null() {
        txn = (*dbc).load_txn;
        current_block = 5720623009719927633;
    } else {
        err =
            mdb_txn_begin((*dbc).env, 0 as *mut MDB_txn,
                          0 as libc::c_int as libc::c_uint, &mut temp_txn);
        if err != 0 {
            current_block = 2554982661806928548;
        } else { txn = temp_txn; current_block = 5720623009719927633; }
    }
    match current_block {
        5720623009719927633 => {
            if must_overwrite != 0 &&
                   mdb_get(txn, db, &mut key, &mut dummy) ==
                       -(30798 as libc::c_int) {
                mdb_txn_abort(temp_txn);
                return -(1780008443 as libc::c_long) as krb5_error_code
            }
            err = mdb_put(txn, db, &mut key, &mut val, putflags);
            if !(err != 0) {
                if !temp_txn.is_null() {
                    err = mdb_txn_commit(temp_txn);
                    temp_txn = 0 as *mut MDB_txn;
                    if err != 0 {
                        current_block = 2554982661806928548;
                    } else { current_block = 15904375183555213903; }
                } else { current_block = 15904375183555213903; }
                match current_block {
                    2554982661806928548 => { }
                    _ => { return 0 as libc::c_int }
                }
            }
        }
        _ => { }
    }
    mdb_txn_abort(temp_txn);
    if err == -(30799 as libc::c_int) {
        return -(1780008447 as libc::c_long) as krb5_error_code
    } else {
        return klerr(context, err,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"LMDB write failure\x00" as *const u8 as
                                  *const libc::c_char))
    };
}
/* Delete an entry from the specified env and database, using a temporary write
 * transaction.  Return KRB5_KDB_NOENTRY if the key does not exist. */
#[c2rust::src_loc = "399:1"]
unsafe extern "C" fn del(mut context: krb5_context, mut env: *mut MDB_env,
                         mut db: MDB_dbi, mut keystr: *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut key: MDB_val =
        {
            let mut init =
                MDB_val{mv_size: strlen(keystr),
                        mv_data: keystr as *mut libc::c_void,};
            init
        };
    let mut err: libc::c_int = 0;
    err =
        mdb_txn_begin(env, 0 as *mut MDB_txn,
                      0 as libc::c_int as libc::c_uint, &mut txn);
    if err == 0 { err = mdb_del(txn, db, &mut key, 0 as *mut MDB_val) }
    if err == 0 { err = mdb_txn_commit(txn); txn = 0 as *mut MDB_txn }
    if err == -(30798 as libc::c_int) {
        ret = -(1780008443 as libc::c_long) as krb5_error_code
    } else if err != 0 {
        ret =
            klerr(context, err,
                  dgettext(b"mit-krb5\x00" as *const u8 as
                               *const libc::c_char,
                           b"LMDB delete failure\x00" as *const u8 as
                               *const libc::c_char))
    }
    mdb_txn_abort(txn);
    return ret;
}
/* Zero out and unlink filename. */
#[c2rust::src_loc = "425:1"]
unsafe extern "C" fn destroy_file(mut filename: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut len: ssize_t = 0;
    let mut pos: off_t = 0;
    let mut buf: [uint8_t; 8192] = [0; 8192];
    let mut zbuf: [uint8_t; 8192] =
        [0 as libc::c_int as uint8_t, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut fd: libc::c_int = 0;
    fd =
        open(filename, 0o2 as libc::c_int | 0o2000000 as libc::c_int,
             0 as libc::c_int);
    if fd < 0 as libc::c_int { return *__errno_location() }
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
    if !(fstat(fd, &mut st) == -(1 as libc::c_int)) {
        memset(zbuf.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               8192 as libc::c_int as libc::c_ulong);
        pos = 0 as libc::c_int as off_t;
        loop  {
            if !(pos < st.st_size) {
                current_block = 5948590327928692120;
                break ;
            }
            len =
                read(fd, buf.as_mut_ptr() as *mut libc::c_void,
                     8192 as libc::c_int as size_t);
            if len < 0 as libc::c_int as libc::c_long {
                current_block = 17076541844165937936;
                break ;
            }
            /* Only rewrite the block if it's not already zeroed, in case the file
         * is sparse. */
            if memcmp(buf.as_mut_ptr() as *const libc::c_void,
                      zbuf.as_mut_ptr() as *const libc::c_void,
                      len as libc::c_ulong) != 0 as libc::c_int {
                lseek(fd, pos, 0 as libc::c_int);
                len =
                    write(fd, zbuf.as_mut_ptr() as *const libc::c_void,
                          len as size_t);
                if len < 0 as libc::c_int as libc::c_long {
                    current_block = 17076541844165937936;
                    break ;
                }
            }
            pos += len
        }
        match current_block {
            17076541844165937936 => { }
            _ => {
                close(fd);
                if unlink(filename) != 0 as libc::c_int {
                    return *__errno_location()
                }
                return 0 as libc::c_int
            }
        }
    }
    ret = *__errno_location();
    close(fd);
    return ret;
}
#[c2rust::src_loc = "470:1"]
unsafe extern "C" fn klmdb_lib_init() -> krb5_error_code {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "476:1"]
unsafe extern "C" fn klmdb_lib_cleanup() -> krb5_error_code {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "482:1"]
unsafe extern "C" fn klmdb_fini(mut context: krb5_context)
 -> krb5_error_code {
    let mut dbc: *mut klmdb_context = 0 as *mut klmdb_context;
    dbc = (*(*context).dal_handle).db_context as *mut klmdb_context;
    if dbc.is_null() { return 0 as libc::c_int }
    mdb_txn_abort((*dbc).read_txn);
    mdb_txn_abort((*dbc).load_txn);
    mdb_env_close((*dbc).env);
    mdb_env_close((*dbc).lockout_env);
    free((*dbc).path as *mut libc::c_void);
    free((*dbc).lockout_path as *mut libc::c_void);
    free(dbc as *mut libc::c_void);
    (*(*context).dal_handle).db_context = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "501:1"]
unsafe extern "C" fn klmdb_open(mut context: krb5_context,
                                mut conf_section: *mut libc::c_char,
                                mut db_args: *mut *mut libc::c_char,
                                mut mode: libc::c_int) -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context = 0 as *mut klmdb_context;
    let mut readonly: krb5_boolean = 0;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut err: libc::c_int = 0;
    if !(*(*context).dal_handle).db_context.is_null() {
        return 0 as libc::c_int
    }
    ret = configure_context(context, conf_section, db_args);
    if ret != 0 { return ret }
    dbc = (*(*context).dal_handle).db_context as *mut klmdb_context;
    if stat((*dbc).path, &mut st) != 0 as libc::c_int {
        ret = 2 as libc::c_int;
        krb5_set_error_message(context, ret,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"LMDB file %s does not exist\x00" as
                                            *const u8 as *const libc::c_char),
                               (*dbc).path);
    } else {
        /* Open the primary environment and databases.  The KDC can open this
     * environment read-only. */
        readonly =
            (mode & 1 as libc::c_int != 0 || mode & 0x100 as libc::c_int != 0)
                as libc::c_int as krb5_boolean;
        ret =
            open_lmdb_env(context, dbc, 0 as libc::c_int as krb5_boolean,
                          readonly, &mut (*dbc).env);
        if !(ret != 0) {
            err =
                mdb_txn_begin((*dbc).env, 0 as *mut MDB_txn,
                              0x20000 as libc::c_int as libc::c_uint,
                              &mut txn);
            if err != 0 {
                current_block = 10454007093657292189;
            } else {
                err =
                    mdb_dbi_open(txn,
                                 b"principal\x00" as *const u8 as
                                     *const libc::c_char,
                                 0 as libc::c_int as libc::c_uint,
                                 &mut (*dbc).princ_db);
                if err != 0 {
                    current_block = 10454007093657292189;
                } else {
                    err =
                        mdb_dbi_open(txn,
                                     b"policy\x00" as *const u8 as
                                         *const libc::c_char,
                                     0 as libc::c_int as libc::c_uint,
                                     &mut (*dbc).policy_db);
                    if err != 0 {
                        current_block = 10454007093657292189;
                    } else {
                        err = mdb_txn_commit(txn);
                        txn = 0 as *mut MDB_txn;
                        if err != 0 {
                            current_block = 10454007093657292189;
                        } else {
                            /* Open the lockout environment and database if we will need it. */
                            if (*dbc).disable_last_success == 0 ||
                                   (*dbc).disable_lockout == 0 {
                                readonly =
                                    (mode & 1 as libc::c_int != 0) as
                                        libc::c_int as krb5_boolean;
                                ret =
                                    open_lmdb_env(context, dbc,
                                                  1 as libc::c_int as
                                                      krb5_boolean, readonly,
                                                  &mut (*dbc).lockout_env);
                                if ret != 0 {
                                    current_block = 9880701465842516334;
                                } else {
                                    err =
                                        mdb_txn_begin((*dbc).lockout_env,
                                                      0 as *mut MDB_txn,
                                                      0x20000 as libc::c_int
                                                          as libc::c_uint,
                                                      &mut txn);
                                    if err != 0 {
                                        current_block = 10454007093657292189;
                                    } else {
                                        err =
                                            mdb_dbi_open(txn,
                                                         b"lockout\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         0 as libc::c_int as
                                                             libc::c_uint,
                                                         &mut (*dbc).lockout_db);
                                        if err != 0 {
                                            current_block =
                                                10454007093657292189;
                                        } else {
                                            err = mdb_txn_commit(txn);
                                            txn = 0 as *mut MDB_txn;
                                            if err != 0 {
                                                current_block =
                                                    10454007093657292189;
                                            } else {
                                                current_block =
                                                    9853141518545631134;
                                            }
                                        }
                                    }
                                }
                            } else { current_block = 9853141518545631134; }
                            match current_block {
                                10454007093657292189 => { }
                                9880701465842516334 => { }
                                _ => { return 0 as libc::c_int }
                            }
                        }
                    }
                }
            }
            match current_block {
                9880701465842516334 => { }
                _ => {
                    ret =
                        klerr(context, err,
                              dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"LMDB open failure\x00" as *const u8
                                           as *const libc::c_char))
                }
            }
        }
    }
    mdb_txn_abort(txn);
    klmdb_fini(context);
    return ret;
}
#[c2rust::src_loc = "573:1"]
unsafe extern "C" fn klmdb_create(mut context: krb5_context,
                                  mut conf_section: *mut libc::c_char,
                                  mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context = 0 as *mut klmdb_context;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut st: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut err: libc::c_int = 0;
    if !(*(*context).dal_handle).db_context.is_null() {
        return 0 as libc::c_int
    }
    ret = configure_context(context, conf_section, db_args);
    if ret != 0 { return ret }
    dbc = (*(*context).dal_handle).db_context as *mut klmdb_context;
    if (*dbc).temporary == 0 {
        if stat((*dbc).path, &mut st) == 0 as libc::c_int {
            ret = 2 as libc::c_int;
            krb5_set_error_message(context, ret,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"LMDB file %s already exists\x00"
                                                as *const u8 as
                                                *const libc::c_char),
                                   (*dbc).path);
            current_block = 9232649119414585772;
        } else { current_block = 1856101646708284338; }
    } else { current_block = 1856101646708284338; }
    match current_block {
        1856101646708284338 => {
            /* Open (and create if necessary) the LMDB environments. */
            ret =
                open_lmdb_env(context, dbc, 0 as libc::c_int as krb5_boolean,
                              0 as libc::c_int as krb5_boolean,
                              &mut (*dbc).env);
            if !(ret != 0) {
                ret =
                    open_lmdb_env(context, dbc,
                                  1 as libc::c_int as krb5_boolean,
                                  0 as libc::c_int as krb5_boolean,
                                  &mut (*dbc).lockout_env);
                if !(ret != 0) {
                    /* Open the primary databases, creating them if they don't exist. */
                    err =
                        mdb_txn_begin((*dbc).env, 0 as *mut MDB_txn,
                                      0 as libc::c_int as libc::c_uint,
                                      &mut txn);
                    if !(err != 0) {
                        err =
                            mdb_dbi_open(txn,
                                         b"principal\x00" as *const u8 as
                                             *const libc::c_char,
                                         0x40000 as libc::c_int as
                                             libc::c_uint,
                                         &mut (*dbc).princ_db);
                        if !(err != 0) {
                            err =
                                mdb_dbi_open(txn,
                                             b"policy\x00" as *const u8 as
                                                 *const libc::c_char,
                                             0x40000 as libc::c_int as
                                                 libc::c_uint,
                                             &mut (*dbc).policy_db);
                            if !(err != 0) {
                                err = mdb_txn_commit(txn);
                                txn = 0 as *mut MDB_txn;
                                if !(err != 0) {
                                    /* Create the lockout database if it doesn't exist. */
                                    err =
                                        mdb_txn_begin((*dbc).lockout_env,
                                                      0 as *mut MDB_txn,
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                      &mut txn);
                                    if !(err != 0) {
                                        err =
                                            mdb_dbi_open(txn,
                                                         b"lockout\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         0x40000 as
                                                             libc::c_int as
                                                             libc::c_uint,
                                                         &mut (*dbc).lockout_db);
                                        if !(err != 0) {
                                            err = mdb_txn_commit(txn);
                                            txn = 0 as *mut MDB_txn;
                                            if !(err != 0) {
                                                if (*dbc).temporary != 0 {
                                                    /* Create a load transaction and empty the primary databases within
         * it. */
                                                    err =
                                                        mdb_txn_begin((*dbc).env,
                                                                      0 as
                                                                          *mut MDB_txn,
                                                                      0 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint,
                                                                      &mut (*dbc).load_txn);
                                                    if err != 0 {
                                                        current_block =
                                                            13476053418519233588;
                                                    } else {
                                                        err =
                                                            mdb_drop((*dbc).load_txn,
                                                                     (*dbc).princ_db,
                                                                     0 as
                                                                         libc::c_int);
                                                        if err != 0 {
                                                            current_block =
                                                                13476053418519233588;
                                                        } else {
                                                            err =
                                                                mdb_drop((*dbc).load_txn,
                                                                         (*dbc).policy_db,
                                                                         0 as
                                                                             libc::c_int);
                                                            if err != 0 {
                                                                current_block
                                                                    =
                                                                    13476053418519233588;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    11743904203796629665;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    current_block =
                                                        11743904203796629665;
                                                }
                                                match current_block {
                                                    13476053418519233588 => {
                                                    }
                                                    _ => {
                                                        /* Close the lockout environment if we won't need it. */
                                                        if (*dbc).disable_last_success
                                                               != 0 &&
                                                               (*dbc).disable_lockout
                                                                   != 0 {
                                                            mdb_env_close((*dbc).lockout_env);
                                                            (*dbc).lockout_env
                                                                =
                                                                0 as
                                                                    *mut MDB_env;
                                                            (*dbc).lockout_db
                                                                =
                                                                0 as
                                                                    libc::c_int
                                                                    as MDB_dbi
                                                        }
                                                        return 0 as
                                                                   libc::c_int
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ret =
                        klerr(context, err,
                              dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"LMDB create error\x00" as *const u8
                                           as *const libc::c_char))
                }
            }
        }
        _ => { }
    }
    mdb_txn_abort(txn);
    klmdb_fini(context);
    return ret;
}
/* Unlink the "-lock" extension of path. */
#[c2rust::src_loc = "666:1"]
unsafe extern "C" fn unlink_lock_file(mut context: krb5_context,
                                      mut path: *const libc::c_char)
 -> krb5_error_code {
    let mut lock_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut st: libc::c_int = 0;
    if asprintf(&mut lock_path as *mut *mut libc::c_char,
                b"%s-lock\x00" as *const u8 as *const libc::c_char, path) <
           0 as libc::c_int {
        return 12 as libc::c_int
    }
    st = unlink(lock_path);
    if st != 0 {
        krb5_prepend_error_message(context, st,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"Could not unlink %s\x00" as
                                                *const u8 as
                                                *const libc::c_char),
                                   lock_path);
    }
    free(lock_path as *mut libc::c_void);
    return st;
}
#[c2rust::src_loc = "681:1"]
unsafe extern "C" fn klmdb_destroy(mut context: krb5_context,
                                   mut conf_section: *mut libc::c_char,
                                   mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context = 0 as *mut klmdb_context;
    if !(*(*context).dal_handle).db_context.is_null() { klmdb_fini(context); }
    ret = configure_context(context, conf_section, db_args);
    if !(ret != 0) {
        dbc = (*(*context).dal_handle).db_context as *mut klmdb_context;
        ret = destroy_file((*dbc).path);
        if !(ret != 0) {
            ret = unlink_lock_file(context, (*dbc).path);
            if !(ret != 0) {
                ret = destroy_file((*dbc).lockout_path);
                if !(ret != 0) {
                    ret = unlink_lock_file(context, (*dbc).lockout_path)
                }
            }
        }
    }
    klmdb_fini(context);
    return ret;
}
#[c2rust::src_loc = "711:1"]
unsafe extern "C" fn klmdb_get_principal(mut context: krb5_context,
                                         mut searchfor: krb5_const_principal,
                                         mut flags: libc::c_uint,
                                         mut entry_out:
                                             *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut key: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    *entry_out = 0 as *mut krb5_db_entry;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    ret = krb5_unparse_name(context, searchfor, &mut name);
    if !(ret != 0) {
        key.mv_data = name as *mut libc::c_void;
        key.mv_size = strlen(name);
        ret = fetch(context, (*dbc).princ_db, &mut key, &mut val);
        if !(ret != 0) {
            ret =
                klmdb_decode_princ(context, name as *const libc::c_void,
                                   strlen(name), val.mv_data, val.mv_size,
                                   entry_out);
            if !(ret != 0) { fetch_lockout(context, &mut key, *entry_out); }
        }
    }
    krb5_free_unparsed_name(context, name);
    return ret;
}
#[c2rust::src_loc = "746:1"]
unsafe extern "C" fn klmdb_put_principal(mut context: krb5_context,
                                         mut entry: *mut krb5_db_entry,
                                         mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut key: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut dummy: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut lockout: [uint8_t; 12] = [0; 12];
    let mut enc: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: libc::c_int = 0;
    if !db_args.is_null() {
        /* This module does not support DB arguments for put_principal. */
        krb5_set_error_message(context, 22 as libc::c_int,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Unsupported argument \"%s\" for lmdb\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               *db_args.offset(0 as libc::c_int as isize));
        return 22 as libc::c_int
    }
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    ret =
        krb5_unparse_name(context, (*entry).princ as krb5_const_principal,
                          &mut name);
    if !(ret != 0) {
        ret = klmdb_encode_princ(context, entry, &mut enc, &mut len);
        if !(ret != 0) {
            ret =
                put(context, (*dbc).princ_db, name, enc, len,
                    0 as libc::c_int as krb5_boolean,
                    0 as libc::c_int as krb5_boolean);
            free(enc as *mut libc::c_void);
            if !(ret != 0) {
                /*
     * Write the lockout attributes to the lockout database if we are using
     * one.  During a load operation, changes to lockout attributes will become
     * visible before the load is finished, which is an acceptable compromise
     * on load atomicity.
     */
                if !(*dbc).lockout_env.is_null() &&
                       (*entry).mask &
                           (0x4000 as libc::c_int | 0x8000 as libc::c_int |
                                0x10000 as libc::c_int | 0x1 as libc::c_int)
                               as libc::c_uint != 0 {
                    key.mv_data = name as *mut libc::c_void;
                    key.mv_size = strlen(name);
                    klmdb_encode_princ_lockout(context, entry,
                                               lockout.as_mut_ptr());
                    val.mv_data = lockout.as_mut_ptr() as *mut libc::c_void;
                    val.mv_size =
                        ::std::mem::size_of::<[uint8_t; 12]>() as
                            libc::c_ulong;
                    err =
                        mdb_txn_begin((*dbc).lockout_env, 0 as *mut MDB_txn,
                                      0 as libc::c_int as libc::c_uint,
                                      &mut txn);
                    if err == 0 && (*dbc).merge_nra != 0 {
                        /* During an iprop load, do not change existing lockout entries. */
                        if mdb_get(txn, (*dbc).lockout_db, &mut key,
                                   &mut dummy) == 0 as libc::c_int {
                            current_block = 2385349426156589262;
                        } else { current_block = 17478428563724192186; }
                    } else { current_block = 17478428563724192186; }
                    match current_block {
                        2385349426156589262 => { }
                        _ => {
                            if err == 0 {
                                err =
                                    mdb_put(txn, (*dbc).lockout_db, &mut key,
                                            &mut val,
                                            0 as libc::c_int as libc::c_uint)
                            }
                            if err == 0 {
                                err = mdb_txn_commit(txn);
                                txn = 0 as *mut MDB_txn
                            }
                            if err != 0 {
                                ret =
                                    klerr(context, err,
                                          dgettext(b"mit-krb5\x00" as
                                                       *const u8 as
                                                       *const libc::c_char,
                                                   b"LMDB lockout write failure\x00"
                                                       as *const u8 as
                                                       *const libc::c_char))
                            }
                        }
                    }
                }
            }
        }
    }
    mdb_txn_abort(txn);
    krb5_free_unparsed_name(context, name);
    return ret;
}
#[c2rust::src_loc = "817:1"]
unsafe extern "C" fn klmdb_delete_principal(mut context: krb5_context,
                                            mut searchfor:
                                                krb5_const_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    ret = krb5_unparse_name(context, searchfor, &mut name);
    if ret != 0 { return ret }
    ret = del(context, (*dbc).env, (*dbc).princ_db, name);
    if ret == 0 && !(*dbc).lockout_env.is_null() {
        del(context, (*dbc).lockout_env, (*dbc).lockout_db, name);
    }
    krb5_free_unparsed_name(context, name);
    return ret;
}
#[c2rust::src_loc = "839:1"]
unsafe extern "C" fn klmdb_iterate(mut context: krb5_context,
                                   mut match_expr: *mut libc::c_char,
                                   mut func:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut libc::c_void,
                                                                   _:
                                                                       *mut krb5_db_entry)
                                                  -> krb5_error_code>,
                                   mut arg: *mut libc::c_void,
                                   mut iterflags: krb5_flags)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut cursor: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut key: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut op: MDB_cursor_op =
        if iterflags & 0x2 as libc::c_int != 0 {
            MDB_PREV as libc::c_int
        } else { MDB_NEXT as libc::c_int } as MDB_cursor_op;
    let mut err: libc::c_int = 0;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    err =
        mdb_txn_begin((*dbc).env, 0 as *mut MDB_txn,
                      0x20000 as libc::c_int as libc::c_uint, &mut txn);
    if err != 0 {
        current_block = 2162225893322838330;
    } else {
        err = mdb_cursor_open(txn, (*dbc).princ_db, &mut cursor);
        if err != 0 {
            current_block = 2162225893322838330;
        } else {
            loop  {
                err = mdb_cursor_get(cursor, &mut key, &mut val, op);
                if err == -(30798 as libc::c_int) {
                    current_block = 5601891728916014340;
                    break ;
                }
                if err != 0 { current_block = 2162225893322838330; break ; }
                ret =
                    klmdb_decode_princ(context, key.mv_data, key.mv_size,
                                       val.mv_data, val.mv_size, &mut entry);
                if ret != 0 { current_block = 17160568077609190382; break ; }
                fetch_lockout(context, &mut key, entry);
                ret =
                    Some(func.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                                       entry);
                krb5_db_free_principal(context, entry);
                if ret != 0 { current_block = 17160568077609190382; break ; }
            }
            match current_block {
                17160568077609190382 => { }
                2162225893322838330 => { }
                _ => {
                    ret = 0 as libc::c_int;
                    current_block = 17160568077609190382;
                }
            }
        }
    }
    match current_block {
        2162225893322838330 => {
            ret =
                klerr(context, err,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"LMDB principal iteration failure\x00" as
                                   *const u8 as *const libc::c_char))
        }
        _ => { }
    }
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "889:1"]
pub unsafe extern "C" fn klmdb_get_policy(mut context: krb5_context,
                                          mut name: *mut libc::c_char,
                                          mut policy: *mut osa_policy_ent_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut key: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    *policy = 0 as osa_policy_ent_t;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    key.mv_data = name as *mut libc::c_void;
    key.mv_size = strlen(name);
    ret = fetch(context, (*dbc).policy_db, &mut key, &mut val);
    if ret != 0 { return ret }
    return klmdb_decode_policy(context, name as *const libc::c_void,
                               strlen(name), val.mv_data, val.mv_size,
                               policy);
}
#[c2rust::src_loc = "909:1"]
unsafe extern "C" fn klmdb_create_policy(mut context: krb5_context,
                                         mut policy: osa_policy_ent_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut enc: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    ret =
        klmdb_encode_policy(context, policy as *const osa_policy_ent_rec,
                            &mut enc, &mut len);
    if ret != 0 { return ret }
    ret =
        put(context, (*dbc).policy_db, (*policy).name, enc, len,
            1 as libc::c_int as krb5_boolean,
            0 as libc::c_int as krb5_boolean);
    free(enc as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "928:1"]
unsafe extern "C" fn klmdb_put_policy(mut context: krb5_context,
                                      mut policy: osa_policy_ent_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut enc: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    ret =
        klmdb_encode_policy(context, policy as *const osa_policy_ent_rec,
                            &mut enc, &mut len);
    if ret != 0 { return ret }
    ret =
        put(context, (*dbc).policy_db, (*policy).name, enc, len,
            0 as libc::c_int as krb5_boolean,
            1 as libc::c_int as krb5_boolean);
    free(enc as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "947:1"]
unsafe extern "C" fn klmdb_iter_policy(mut context: krb5_context,
                                       mut match_entry: *mut libc::c_char,
                                       mut func: osa_adb_iter_policy_func,
                                       mut arg: *mut libc::c_void)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut pol: osa_policy_ent_t = 0 as *mut _osa_policy_ent_t;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut cursor: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut key: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut err: libc::c_int = 0;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    err =
        mdb_txn_begin((*dbc).env, 0 as *mut MDB_txn,
                      0x20000 as libc::c_int as libc::c_uint, &mut txn);
    if err != 0 {
        current_block = 4837026447722880994;
    } else {
        err = mdb_cursor_open(txn, (*dbc).policy_db, &mut cursor);
        if err != 0 {
            current_block = 4837026447722880994;
        } else {
            loop  {
                err = mdb_cursor_get(cursor, &mut key, &mut val, MDB_NEXT);
                if err == -(30798 as libc::c_int) {
                    current_block = 15904375183555213903;
                    break ;
                }
                if err != 0 { current_block = 4837026447722880994; break ; }
                ret =
                    klmdb_decode_policy(context, key.mv_data, key.mv_size,
                                        val.mv_data, val.mv_size, &mut pol);
                if ret != 0 { current_block = 8618916773443744716; break ; }
                Some(func.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                                   pol);
                krb5_db_free_policy(context, pol);
            }
            match current_block {
                8618916773443744716 => { }
                4837026447722880994 => { }
                _ => {
                    ret = 0 as libc::c_int;
                    current_block = 8618916773443744716;
                }
            }
        }
    }
    match current_block {
        4837026447722880994 => {
            ret =
                klerr(context, err,
                      dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"LMDB policy iteration failure\x00" as
                                   *const u8 as *const libc::c_char))
        }
        _ => { }
    }
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    return ret;
}
#[c2rust::src_loc = "992:1"]
unsafe extern "C" fn klmdb_delete_policy(mut context: krb5_context,
                                         mut policy: *mut libc::c_char)
 -> krb5_error_code {
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    return del(context, (*dbc).env, (*dbc).policy_db, policy);
}
#[c2rust::src_loc = "1002:1"]
unsafe extern "C" fn klmdb_promote_db(mut context: krb5_context,
                                      mut conf_section: *mut libc::c_char,
                                      mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut err: libc::c_int = 0;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    if (*dbc).load_txn.is_null() { return 22 as libc::c_int }
    err = mdb_txn_commit((*dbc).load_txn);
    (*dbc).load_txn = 0 as *mut MDB_txn;
    if err != 0 {
        ret =
            klerr(context, err,
                  dgettext(b"mit-krb5\x00" as *const u8 as
                               *const libc::c_char,
                           b"LMDB transaction commit failure\x00" as *const u8
                               as *const libc::c_char))
    }
    klmdb_fini(context);
    return ret;
}
#[c2rust::src_loc = "1021:1"]
unsafe extern "C" fn klmdb_check_policy_as(mut context: krb5_context,
                                           mut request: *mut krb5_kdc_req,
                                           mut client: *mut krb5_db_entry,
                                           mut server: *mut krb5_db_entry,
                                           mut kdc_time: krb5_timestamp,
                                           mut status:
                                               *mut *const libc::c_char,
                                           mut e_data:
                                               *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    if (*dbc).disable_lockout != 0 { return 0 as libc::c_int }
    ret = klmdb_lockout_check_policy(context, client, kdc_time);
    if ret as libc::c_long == -(1765328366 as libc::c_long) {
        *status = b"LOCKED_OUT\x00" as *const u8 as *const libc::c_char
    }
    return ret;
}
#[c2rust::src_loc = "1039:1"]
unsafe extern "C" fn klmdb_audit_as_req(mut context: krb5_context,
                                        mut request: *mut krb5_kdc_req,
                                        mut local_addr: *const krb5_address,
                                        mut remote_addr: *const krb5_address,
                                        mut client: *mut krb5_db_entry,
                                        mut server: *mut krb5_db_entry,
                                        mut authtime: krb5_timestamp,
                                        mut status: krb5_error_code) {
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    klmdb_lockout_audit(context, client, authtime, status,
                        (*dbc).disable_last_success, (*dbc).disable_lockout);
}
#[no_mangle]
#[c2rust::src_loc = "1052:1"]
pub unsafe extern "C" fn klmdb_update_lockout(mut context: krb5_context,
                                              mut entry: *mut krb5_db_entry,
                                              mut stamp: krb5_timestamp,
                                              mut zero_fail_count:
                                                  krb5_boolean,
                                              mut set_last_success:
                                                  krb5_boolean,
                                              mut set_last_failure:
                                                  krb5_boolean)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut dbc: *mut klmdb_context =
        (*(*context).dal_handle).db_context as *mut klmdb_context;
    let mut dummy: krb5_db_entry =
        {
            let mut init =
                _krb5_db_entry_new{magic: 0 as libc::c_int,
                                   len: 0,
                                   mask: 0,
                                   attributes: 0,
                                   max_life: 0,
                                   max_renewable_life: 0,
                                   expiration: 0,
                                   pw_expiration: 0,
                                   last_success: 0,
                                   last_failed: 0,
                                   fail_auth_count: 0,
                                   n_tl_data: 0,
                                   n_key_data: 0,
                                   e_length: 0,
                                   e_data: 0 as *mut krb5_octet,
                                   princ: 0 as *mut krb5_principal_data,
                                   tl_data: 0 as *mut krb5_tl_data,
                                   key_data: 0 as *mut krb5_key_data,};
            init
        };
    let mut lockout: [uint8_t; 12] = [0; 12];
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut key: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut val: MDB_val =
        MDB_val{mv_size: 0, mv_data: 0 as *mut libc::c_void,};
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: libc::c_int = 0;
    if dbc.is_null() {
        return -(1780008435 as libc::c_long) as krb5_error_code
    }
    if (*dbc).lockout_env.is_null() { return 0 as libc::c_int }
    if zero_fail_count == 0 && set_last_success == 0 && set_last_failure == 0
       {
        return 0 as libc::c_int
    }
    ret =
        krb5_unparse_name(context, (*entry).princ as krb5_const_principal,
                          &mut name);
    if !(ret != 0) {
        key.mv_data = name as *mut libc::c_void;
        key.mv_size = strlen(name);
        err =
            mdb_txn_begin((*dbc).lockout_env, 0 as *mut MDB_txn,
                          0 as libc::c_int as libc::c_uint, &mut txn);
        if err != 0 {
            current_block = 6850170778190853433;
        } else {
            /* Fetch base lockout info within txn so we update transactionally. */
            err = mdb_get(txn, (*dbc).lockout_db, &mut key, &mut val);
            if err == 0 && val.mv_size >= 12 as libc::c_int as libc::c_ulong {
                klmdb_decode_princ_lockout(context, &mut dummy,
                                           val.mv_data as *const uint8_t);
            } else {
                dummy.last_success = (*entry).last_success;
                dummy.last_failed = (*entry).last_failed;
                dummy.fail_auth_count = (*entry).fail_auth_count
            }
            if zero_fail_count != 0 {
                dummy.fail_auth_count = 0 as libc::c_int as krb5_kvno
            }
            if set_last_success != 0 { dummy.last_success = stamp }
            if set_last_failure != 0 {
                dummy.last_failed = stamp;
                dummy.fail_auth_count = dummy.fail_auth_count.wrapping_add(1)
            }
            klmdb_encode_princ_lockout(context, &mut dummy,
                                       lockout.as_mut_ptr());
            val.mv_data = lockout.as_mut_ptr() as *mut libc::c_void;
            val.mv_size =
                ::std::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong;
            err =
                mdb_put(txn, (*dbc).lockout_db, &mut key, &mut val,
                        0 as libc::c_int as libc::c_uint);
            if err != 0 {
                current_block = 6850170778190853433;
            } else {
                err = mdb_txn_commit(txn);
                txn = 0 as *mut MDB_txn;
                if err != 0 {
                    current_block = 6850170778190853433;
                } else { current_block = 12512295087047028901; }
            }
        }
        match current_block {
            12512295087047028901 => { }
            _ => {
                ret =
                    klerr(context, err,
                          dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"LMDB lockout update failure\x00" as
                                       *const u8 as *const libc::c_char))
            }
        }
    }
    krb5_free_unparsed_name(context, name);
    mdb_txn_abort(txn);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1122:42"]
pub static mut kdb_function_table: kdb_vftabl =
    unsafe {
        {
            let mut init =
                _kdb_vftabl{maj_ver: 8 as libc::c_int as libc::c_short,
                            min_ver: 0 as libc::c_int as libc::c_short,
                            init_library:
                                ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                   ->
                                                                       krb5_error_code>,
                                                        Option<unsafe extern "C" fn()
                                                                   ->
                                                                       krb5_error_code>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                          ->
                                                                                                                              krb5_error_code,
                                                                                                                      unsafe extern "C" fn()
                                                                                                                          ->
                                                                                                                              krb5_error_code>(klmdb_lib_init))),
                            fini_library:
                                ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                   ->
                                                                       krb5_error_code>,
                                                        Option<unsafe extern "C" fn()
                                                                   ->
                                                                       krb5_error_code>>(Some(::std::mem::transmute::<unsafe extern "C" fn()
                                                                                                                          ->
                                                                                                                              krb5_error_code,
                                                                                                                      unsafe extern "C" fn()
                                                                                                                          ->
                                                                                                                              krb5_error_code>(klmdb_lib_cleanup))),
                            init_module:
                                Some(klmdb_open as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut *mut libc::c_char,
                                                              _: libc::c_int)
                                             -> krb5_error_code),
                            fini_module:
                                Some(klmdb_fini as
                                         unsafe extern "C" fn(_: krb5_context)
                                             -> krb5_error_code),
                            create:
                                Some(klmdb_create as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> krb5_error_code),
                            destroy:
                                Some(klmdb_destroy as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> krb5_error_code),
                            get_age: None,
                            lock: None,
                            unlock: None,
                            get_principal:
                                Some(klmdb_get_principal as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_const_principal,
                                                              _: libc::c_uint,
                                                              _:
                                                                  *mut *mut krb5_db_entry)
                                             -> krb5_error_code),
                            put_principal:
                                Some(klmdb_put_principal as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> krb5_error_code),
                            delete_principal:
                                Some(klmdb_delete_principal as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_const_principal)
                                             -> krb5_error_code),
                            rename_principal: None,
                            iterate:
                                Some(klmdb_iterate as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  Option<unsafe extern "C" fn(_:
                                                                                                  *mut libc::c_void,
                                                                                              _:
                                                                                                  *mut krb5_db_entry)
                                                                             ->
                                                                                 krb5_error_code>,
                                                              _:
                                                                  *mut libc::c_void,
                                                              _: krb5_flags)
                                             -> krb5_error_code),
                            create_policy:
                                Some(klmdb_create_policy as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  osa_policy_ent_t)
                                             -> krb5_error_code),
                            get_policy:
                                Some(klmdb_get_policy as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut osa_policy_ent_t)
                                             -> krb5_error_code),
                            put_policy:
                                Some(klmdb_put_policy as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  osa_policy_ent_t)
                                             -> krb5_error_code),
                            iter_policy:
                                Some(klmdb_iter_policy as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  osa_adb_iter_policy_func,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> krb5_error_code),
                            delete_policy:
                                Some(klmdb_delete_policy as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char)
                                             -> krb5_error_code),
                            fetch_master_key: None,
                            fetch_master_key_list: None,
                            store_master_key_list: None,
                            dbe_search_enctype: None,
                            change_pwd: None,
                            promote_db:
                                Some(klmdb_promote_db as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut *mut libc::c_char)
                                             -> krb5_error_code),
                            decrypt_key_data: None,
                            encrypt_key_data: None,
                            sign_authdata: None,
                            check_transited_realms: None,
                            check_policy_as:
                                Some(klmdb_check_policy_as as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut krb5_kdc_req,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  krb5_timestamp,
                                                              _:
                                                                  *mut *const libc::c_char,
                                                              _:
                                                                  *mut *mut *mut krb5_pa_data)
                                             -> krb5_error_code),
                            check_policy_tgs: None,
                            audit_as_req:
                                Some(klmdb_audit_as_req as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut krb5_kdc_req,
                                                              _:
                                                                  *const krb5_address,
                                                              _:
                                                                  *const krb5_address,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  krb5_timestamp,
                                                              _:
                                                                  krb5_error_code)
                                             -> ()),
                            refresh_config: None,
                            check_allowed_to_delegate: None,
                            free_principal_e_data: None,
                            get_s4u_x509_principal: None,
                            allowed_to_delegate_from: None,
                            get_authdata_info: None,
                            free_authdata_info: None,};
            init
        }
    };
