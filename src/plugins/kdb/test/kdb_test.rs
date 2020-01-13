use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:84"]
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
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:84"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:84"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:84"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:84"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:84"]
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
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
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
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
    /* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
    /* *
 * Unwrap authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  container       Authorization data to be decoded
 * @param [out] authdata        List of decoded authorization data
 *
 * @sa krb5_encode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Wrap authorization data in a container.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  authdata        List of authorization data to be encoded
 * @param [out] container       List of encoded authorization data
 *
 * The result is returned in @a container as a single-element list.
 *
 * @sa krb5_decode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /*
 * AD-KDCIssued
 */
/* *
 * Encode and sign AD-KDCIssued authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Session key
 * @param [in]  issuer          The name of the issuing principal
 * @param [in]  authdata        List of authorization data to be signed
 * @param [out] ad_kdcissued    List containing AD-KDCIssued authdata
 *
 * This function wraps a list of authorization data entries @a authdata in an
 * AD-KDCIssued container (see RFC 4120 section 5.2.6.2) signed with @a key.
 * The result is returned in @a ad_kdcissued as a single-element list.
 */
    /* *
 * Unwrap and verify AD-KDCIssued authorization data.
 *
 * @param [in] context          Library context
 * @param [in] key              Session key
 * @param [in] ad_kdcissued     AD-KDCIssued authorization data to be unwrapped
 * @param [out] issuer          Name of issuing principal (or NULL)
 * @param [out] authdata        Unwrapped list of authorization data
 *
 * This function unwraps an AD-KDCIssued authdatum (see RFC 4120 section
 * 5.2.6.2) and verifies its signature against @a key.  The issuer field of the
 * authdatum element is returned in @a issuer, and the unwrapped list of
 * authdata is returned in @a authdata.
 */
    /*
 * Windows PAC
 */
    /* Microsoft defined types of data */
    /* *< Logon information */
    /* *< Credentials information */
    /* *< Server checksum */
    /* *< KDC checksum */
    /* *< Client name and ticket info */
    /* *< Constrained delegation info */
    /* *< User principal name and DNS info */
    /* * PAC data structure to convey authorization information */
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
    #[c2rust::src_loc = "8142:1"]
    pub type krb5_pac = *mut krb5_pac_data;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    extern "C" {
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
        /* checksum type */
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
        #[c2rust::src_loc = "8140:8"]
        pub type krb5_pac_data;
        /* *
 * Return length of the specified key in bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] keybytes        Number of bytes required to make a key
 * @param [out] keylength       Length of final key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "605:1"]
        pub fn krb5_c_keylengths(context: krb5_context, enctype: krb5_enctype,
                                 keybytes: *mut size_t,
                                 keylength: *mut size_t) -> krb5_error_code;
        /* *
 * Generate an enctype-specific key from random data.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  random_data     Random input data
 * @param [out] k5_random_key   Resulting key
 *
 * This function takes random input data @a random_data and produces a valid
 * key @a k5_random_key for a given @a enctype.
 *
 * @note It is assumed that @a k5_random_key has already been initialized and
 * @a k5_random_key->contents has been allocated with the correct length.
 *
 * @sa krb5_c_keylengths()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "767:1"]
        pub fn krb5_c_random_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    random_data: *mut krb5_data,
                                    k5_random_key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Compute a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type (0 for mandatory type)
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]  input           Input data
 * @param [out] cksum           Generated checksum
 *
 * This function computes a checksum of type @a cksumtype over @a input, using
 * @a key if the checksum type is a keyed checksum.  If @a cksumtype is 0 and
 * @a key is non-null, the checksum type will be the mandatory-to-implement
 * checksum type for the key's encryption type.  The actual checksum key will
 * be derived from @a key and @a usage if key derivation is specified for the
 * checksum type.  The newly created @a cksum must be released by calling
 * krb5_free_checksum_contents() when it is no longer needed.
 *
 * @note This function is similar to krb5_k_make_checksum(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "911:1"]
        pub fn krb5_c_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype,
                                    key: *const krb5_keyblock,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
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
        /* *< Create single-component
                                                  enterprise principle */
        /* *< Ignore realm if present */
        /* *
 * Convert a string principal name to a krb5_principal with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [in]  flags           Flag
 * @param [out] principal_out   New principal
 *
 * Similar to krb5_parse_name(), this function converts a single-string
 * representation of a principal name to a krb5_principal structure.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_PARSE_NO_REALM - no realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_REQUIRE_REALM - realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_ENTERPRISE - create single-component enterprise
 *                                        principal
 * @li #KRB5_PRINCIPAL_PARSE_IGNORE_REALM - ignore realm if present in @a name
 *
 * If @c KRB5_PRINCIPAL_PARSE_NO_REALM or @c KRB5_PRINCIPAL_PARSE_IGNORE_REALM
 * is specified in @a flags, the realm of the new principal will be empty.
 * Otherwise, the default realm for @a context will be used if @a name does not
 * specify a realm.
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
        #[c2rust::src_loc = "3468:1"]
        pub fn krb5_parse_name_flags(context: krb5_context,
                                     name: *const libc::c_char,
                                     flags: libc::c_int,
                                     principal_out: *mut krb5_principal)
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
        /* *< Omit realm always */
        /* *< Don't escape special characters */
        /* *
 * Convert krb5_principal structure to a string with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            String representation of principal name
 *
 * Similar to krb5_unparse_name(), this function converts a krb5_principal
 * structure to a string representation.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_UNPARSE_SHORT - omit realm if it is the local realm
 * @li #KRB5_PRINCIPAL_UNPARSE_NO_REALM - omit realm
 * @li #KRB5_PRINCIPAL_UNPARSE_DISPLAY - do not quote special characters
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
        #[no_mangle]
        #[c2rust::src_loc = "3547:1"]
        pub fn krb5_unparse_name_flags(context: krb5_context,
                                       principal: krb5_const_principal,
                                       flags: libc::c_int,
                                       name: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Compare the realms of two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the realm names are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3650:1"]
        pub fn krb5_realm_compare(context: krb5_context,
                                  princ1: krb5_const_principal,
                                  princ2: krb5_const_principal)
         -> krb5_boolean;
        /* *
 * Compare two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        /* *
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3797:1"]
        pub fn krb5_copy_data(context: krb5_context, indata: *const krb5_data,
                              outdata: *mut *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "3885:1"]
        pub fn krb5_find_authdata(context: krb5_context,
                                  ticket_authdata: *const *mut krb5_authdata,
                                  ap_req_authdata: *const *mut krb5_authdata,
                                  ad_type: krb5_authdatatype,
                                  results: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        /* *
 * Build a principal name using length-counted strings.
 *
 * @param [in]  context  Library context
 * @param [out] princ    Principal name
 * @param [in]  rlen     Realm name length
 * @param [in]  realm    Realm name
 * @param [in]  ...      List of unsigned int/char * components, followed by 0
 *
 * This function creates a principal from a length-counted string and a
 * variable-length list of length-counted components.  The list of components
 * ends with the first 0 length argument (so it is not possible to specify an
 * empty component with this function).  Call krb5_free_principal() to free
 * allocated memory for principal when it is no longer needed.
 *
 * @code
 * Example of how to build principal WELLKNOWN/ANONYMOUS@R
 *     krb5_build_principal_ext(context, &principal, strlen("R"), "R",
 *         (unsigned int)strlen(KRB5_WELLKNOWN_NAMESTR),
 *         KRB5_WELLKNOWN_NAMESTR,
 *         (unsigned int)strlen(KRB5_ANONYMOUS_PRINCSTR),
 *         KRB5_ANONYMOUS_PRINCSTR, 0);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3995:1"]
        pub fn krb5_build_principal_ext(context: krb5_context,
                                        princ: *mut krb5_principal,
                                        rlen: libc::c_uint,
                                        realm: *const libc::c_char, _: ...)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8077:1"]
        pub fn krb5_encode_authdata_container(context: krb5_context,
                                              type_0: krb5_authdatatype,
                                              authdata:
                                                  *const *mut krb5_authdata,
                                              container:
                                                  *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8166:1"]
        pub fn krb5_pac_add_buffer(context: krb5_context, pac: krb5_pac,
                                   type_0: krb5_ui_4, data: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8178:1"]
        pub fn krb5_pac_free(context: krb5_context, pac: krb5_pac);
        #[no_mangle]
        #[c2rust::src_loc = "8193:1"]
        pub fn krb5_pac_get_buffer(context: krb5_context, pac: krb5_pac,
                                   type_0: krb5_ui_4, data: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8221:1"]
        pub fn krb5_pac_init(context: krb5_context, pac: *mut krb5_pac)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8236:1"]
        pub fn krb5_pac_parse(context: krb5_context, ptr: *const libc::c_void,
                              len: size_t, pac: *mut krb5_pac)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8266:1"]
        pub fn krb5_pac_verify(context: krb5_context, pac: krb5_pac,
                               authtime: krb5_timestamp,
                               principal: krb5_const_principal,
                               server: *const krb5_keyblock,
                               privsvr: *const krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8289:1"]
        pub fn krb5_pac_verify_ext(context: krb5_context, pac: krb5_pac,
                                   authtime: krb5_timestamp,
                                   principal: krb5_const_principal,
                                   server: *const krb5_keyblock,
                                   privsvr: *const krb5_keyblock,
                                   with_realm: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8337:1"]
        pub fn krb5_pac_sign_ext(context: krb5_context, pac: krb5_pac,
                                 authtime: krb5_timestamp,
                                 principal: krb5_const_principal,
                                 server_key: *const krb5_keyblock,
                                 privsvr_key: *const krb5_keyblock,
                                 with_realm: krb5_boolean,
                                 data: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8362:1"]
        pub fn krb5_pac_get_client_info(context: krb5_context, pac: krb5_pac,
                                        authtime_out: *mut krb5_timestamp,
                                        princname_out: *mut *mut libc::c_char)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:84"]
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
    #[c2rust::src_loc = "2244:1"]
    pub unsafe extern "C" fn data_eq_string(mut d: krb5_data,
                                            mut s: *const libc::c_char)
     -> libc::c_int {
        return (d.length as libc::c_ulong == strlen(s) &&
                    (d.length == 0 as libc::c_int as libc::c_uint ||
                         memcmp(d.data as *const libc::c_void,
                                s as *const libc::c_void,
                                d.length as libc::c_ulong) == 0)) as
                   libc::c_int;
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
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
    }
    #[inline]
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
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
    use super::kdb5_h::_kdb5_dal_handle;
    use super::string_h::{strlen, memcmp, memcpy};
    use super::stdlib_h::calloc;
    use super::stddef_h::size_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "998:1"]
        pub fn k5_free_data_ptr_list(list: *mut *mut krb5_data);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:84"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:84"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/kdb5.h:85"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:85"]
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
    /* KDB iteration flags */
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
    use super::krb5_h::{krb5_keyblock, krb5_kvno, krb5_error_code,
                        krb5_context, krb5_const_principal, krb5_pointer,
                        krb5_flags, krb5_principal, krb5_int32, krb5_boolean,
                        krb5_timestamp, krb5_authdata, krb5_data,
                        krb5_kdc_req, krb5_pa_data, krb5_ticket, krb5_address,
                        krb5_octet, krb5_magic, krb5_ui_2, krb5_ui_4,
                        krb5_deltat, krb5_int16, krb5_enctype,
                        krb5_principal_data};
    use super::time_t_h::time_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn krb5_dbe_find_enctype(kcontext: krb5_context,
                                     dbentp: *mut krb5_db_entry,
                                     ktype: krb5_int32, stype: krb5_int32,
                                     kvno: krb5_int32,
                                     kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
        /* Length, data */
        /* *
 * Decrypts the key given in @@a key_data. If @a mkey is specified, that
 * master key is used. If @a mkey is NULL, then all master keys are tried.
 */
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "542:1"]
        pub fn krb5_dbe_update_mod_princ_data(context: krb5_context,
                                              entry: *mut krb5_db_entry,
                                              mod_date: krb5_timestamp,
                                              mod_princ: krb5_const_principal)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:84"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn profile_get_relation_names(profile: profile_t,
                                          names: *mut *const libc::c_char,
                                          ret_names:
                                              *mut *mut *mut libc::c_char)
         -> libc::c_long;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/ctype.h:87"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_0 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_0 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_0 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_0 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_0 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_0 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_0 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_0 = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:84"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:84"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:84"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:84"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "90:14"]
        pub fn memchr(_: *const libc::c_void, _: libc::c_int,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
#[c2rust::header_src = "/usr/include/assert.h:84"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:86"]
pub mod adm_proto_h {
    use super::krb5_h::{krb5_flags, krb5_error_code, krb5_boolean,
                        krb5_int32};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        /* str_conv.c */
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn krb5_flagspec_to_mask(_: *const libc::c_char,
                                     _: *mut krb5_flags, _: *mut krb5_flags)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn krb5_string_to_keysalts(_: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: krb5_boolean,
                                       _: *mut *mut krb5_key_salt_tuple,
                                       _: *mut krb5_int32) -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_keyblock, _krb5_keyblock,
                       krb5_authdata, _krb5_authdata, krb5_kdc_req,
                       _krb5_kdc_req, krb5_ticket, _krb5_ticket,
                       krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_ticket_times, _krb5_ticket_times, krb5_transited,
                       _krb5_transited, krb5_enc_data, _krb5_enc_data,
                       krb5_pa_data, _krb5_pa_data, _krb5_checksum,
                       krb5_checksum, krb5_pac, _profile_t, krb5_pac_data,
                       krb5_c_keylengths, krb5_c_random_to_key,
                       krb5_c_make_checksum, krb5_parse_name,
                       krb5_parse_name_flags, krb5_unparse_name,
                       krb5_unparse_name_flags, krb5_realm_compare,
                       krb5_principal_compare, krb5_copy_data,
                       krb5_copy_principal, krb5_find_authdata,
                       krb5_build_principal_ext, krb5_free_principal,
                       krb5_free_authdata, krb5_free_keyblock_contents,
                       krb5_free_data_contents, krb5_free_unparsed_name,
                       krb5_string_to_deltat, krb5_encode_authdata_container,
                       krb5_pac_add_buffer, krb5_pac_free,
                       krb5_pac_get_buffer, krb5_pac_init, krb5_pac_parse,
                       krb5_pac_verify, krb5_pac_verify_ext,
                       krb5_pac_sign_ext, krb5_pac_get_client_info};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, data_eq_string, make_data,
                         empty_data, string2data, alloc_data, k5calloc,
                         k5alloc, k5memdup0, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, k5_free_data_ptr_list};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::kdb5_h::{_kdb5_dal_handle, db_library, _db_library};
pub use self::kdb_h::{krb5_keylist_node, _krb5_keylist_node, kdb_vftabl,
                      _kdb_vftabl, krb5_db_entry, _krb5_db_entry_new,
                      krb5_key_data, _krb5_key_data, krb5_tl_data,
                      _krb5_tl_data, krb5_keysalt, _krb5_keysalt,
                      krb5_key_salt_tuple, __krb5_key_salt_tuple,
                      osa_adb_iter_policy_func, osa_policy_ent_t,
                      _osa_policy_ent_t, krb5_dbe_find_enctype,
                      krb5_dbe_decrypt_key_data,
                      krb5_dbe_update_mod_princ_data};
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list,
                          profile_get_relation_names};
pub use self::ctype_h::{C2RustUnnamed_0, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper,
                        __ctype_b_loc};
use self::stdlib_h::{calloc, free, abort, strtol};
use self::stdio_h::asprintf;
use self::time_h::time;
use self::string_h::{strlen, strdup, strncmp, strcmp, memchr, memcmp, memset,
                     memcpy};
use self::assert_h::__assert_fail;
use self::adm_proto_h::{krb5_flagspec_to_mask, krb5_string_to_keysalts};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "553:9"]
pub struct pac_info {
    pub pac_princ: *mut libc::c_char,
    pub deleg_info: C2RustUnnamed,
    pub not_delegated: krb5_boolean,
    pub pac: krb5_pac,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "555:5"]
pub struct C2RustUnnamed {
    pub proxy_target: *mut libc::c_char,
    pub impersonator: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "94:9"]
pub struct C2RustUnnamed_1 {
    pub profile: *mut libc::c_void,
    pub section: *mut libc::c_char,
    pub names: [*const libc::c_char; 6],
}
#[c2rust::src_loc = "94:1"]
pub type testhandle = *mut C2RustUnnamed_1;
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn ealloc(mut sz: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void =
        calloc(sz, 1 as libc::c_int as libc::c_ulong);
    if p.is_null() { abort(); }
    return p;
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn estrdup(mut s: *const libc::c_char)
 -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = strdup(s);
    if copy.is_null() { abort(); }
    return copy;
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn check(mut code: krb5_error_code) {
    if code != 0 as libc::c_int { abort(); };
}
/* Set up for a profile query using h->names.  Look up s1 -> s2 -> s3 (some of
 * which may be NULL) within this database's dbmodules section. */
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn set_names(mut h: testhandle, mut s1: *const libc::c_char,
                               mut s2: *const libc::c_char,
                               mut s3: *const libc::c_char) {
    (*h).names[0 as libc::c_int as usize] =
        b"dbmodules\x00" as *const u8 as *const libc::c_char;
    (*h).names[1 as libc::c_int as usize] = (*h).section;
    (*h).names[2 as libc::c_int as usize] = s1;
    (*h).names[3 as libc::c_int as usize] = s2;
    (*h).names[4 as libc::c_int as usize] = s3;
    (*h).names[5 as libc::c_int as usize] = 0 as *const libc::c_char;
}
/* Look up a string within this database's dbmodules section. */
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn get_string(mut h: testhandle,
                                mut s1: *const libc::c_char,
                                mut s2: *const libc::c_char,
                                mut s3: *const libc::c_char)
 -> *mut libc::c_char {
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    set_names(h, s1, s2, s3);
    ret =
        profile_get_values((*h).profile as profile_t, (*h).names.as_mut_ptr(),
                           &mut values) as krb5_error_code;
    if ret as libc::c_long == -(1429577725 as libc::c_long) {
        return 0 as *mut libc::c_char
    }
    if ret != 0 { abort(); }
    val = estrdup(*values.offset(0 as libc::c_int as isize));
    profile_free_list(values);
    return val;
}
/* Look up a duration within this database's dbmodules section. */
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn get_duration(mut h: testhandle,
                                  mut s1: *const libc::c_char,
                                  mut s2: *const libc::c_char,
                                  mut s3: *const libc::c_char)
 -> krb5_deltat {
    let mut strval: *mut libc::c_char = get_string(h, s1, s2, s3);
    let mut val: krb5_deltat = 0;
    if strval.is_null() { return 0 as libc::c_int }
    check(krb5_string_to_deltat(strval, &mut val));
    free(strval as *mut libc::c_void);
    return val;
}
/* Look up an absolute time within this database's dbmodules section.  The time
 * is expressed in the profile as an interval relative to the current time. */
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn get_time(mut h: testhandle, mut s1: *const libc::c_char,
                              mut s2: *const libc::c_char,
                              mut s3: *const libc::c_char) -> krb5_timestamp {
    let mut strval: *mut libc::c_char = get_string(h, s1, s2, s3);
    let mut val: krb5_deltat = 0;
    if strval.is_null() { return 0 as libc::c_int }
    check(krb5_string_to_deltat(strval, &mut val));
    free(strval as *mut libc::c_void);
    return (val as libc::c_long + time(0 as *mut time_t)) as krb5_timestamp;
}
/* Initialize kb_out with a key of type etype, using a hash of kvno, etype,
 * salttype, and princstr for the key bytes. */
#[c2rust::src_loc = "189:1"]
unsafe extern "C" fn make_keyblock(mut kvno: krb5_kvno,
                                   mut etype: krb5_enctype,
                                   mut salttype: int32_t,
                                   mut princstr: *const libc::c_char,
                                   mut realm: *const krb5_data,
                                   mut kb_out: *mut krb5_keyblock) {
    let mut keybytes: size_t = 0;
    let mut keylength: size_t = 0;
    let mut pos: size_t = 0;
    let mut n: size_t = 0;
    let mut hashstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut rndin: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    check(krb5_c_keylengths(0 as krb5_context, etype, &mut keybytes,
                            &mut keylength));
    alloc_data(&mut rndin, keybytes as libc::c_uint);
    /* Hash the kvno, enctype, salt type, and principal name together. */
    if asprintf(&mut hashstr as *mut *mut libc::c_char,
                b"%d %d %d %s %.*s\x00" as *const u8 as *const libc::c_char,
                kvno as libc::c_int, etype, salttype, princstr,
                (*realm).length as libc::c_int, (*realm).data) <
           0 as libc::c_int {
        abort();
    }
    d = string2data(hashstr);
    check(krb5_c_make_checksum(0 as krb5_context, 0x9 as libc::c_int,
                               0 as *const krb5_keyblock, 0 as libc::c_int,
                               &mut d, &mut cksum));
    /* Make the appropriate number of input bytes from the hash result. */
    pos = 0 as libc::c_int as size_t;
    while pos < keybytes {
        n =
            if (cksum.length as libc::c_ulong) < keybytes.wrapping_sub(pos) {
                cksum.length as libc::c_ulong
            } else { keybytes.wrapping_sub(pos) };
        memcpy(rndin.data.offset(pos as isize) as *mut libc::c_void,
               cksum.contents as *const libc::c_void, n);
        pos = (pos as libc::c_ulong).wrapping_add(n) as size_t as size_t
    }
    (*kb_out).enctype = etype;
    (*kb_out).length = keylength as libc::c_uint;
    (*kb_out).contents = ealloc(keylength) as *mut krb5_octet;
    check(krb5_c_random_to_key(0 as krb5_context, etype, &mut rndin, kb_out));
    free(cksum.contents as *mut libc::c_void);
    free(rndin.data as *mut libc::c_void);
    free(hashstr as *mut libc::c_void);
}
/* Return key data for the given key/salt tuple strings, using hashes of the
 * enctypes, salts, and princstr for the key contents. */
#[c2rust::src_loc = "226:1"]
unsafe extern "C" fn make_keys(mut strings: *mut *mut libc::c_char,
                               mut princstr: *const libc::c_char,
                               mut realm: *const krb5_data,
                               mut ent: *mut krb5_db_entry) {
    let mut key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut kb: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut ks_list_sizes: *mut int32_t = 0 as *mut int32_t;
    let mut nstrings: int32_t = 0;
    let mut nkeys: int32_t = 0;
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    let mut ks_lists: *mut *mut krb5_key_salt_tuple =
        0 as *mut *mut krb5_key_salt_tuple;
    let mut ks: *mut krb5_key_salt_tuple = 0 as *mut krb5_key_salt_tuple;
    let mut kvnos: *mut krb5_kvno = 0 as *mut krb5_kvno;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    nstrings = 0 as libc::c_int;
    while !(*strings.offset(nstrings as isize)).is_null() { nstrings += 1 }
    ks_lists =
        ealloc((nstrings as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_key_salt_tuple>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_key_salt_tuple;
    ks_list_sizes =
        ealloc((nstrings as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>()
                                                    as libc::c_ulong)) as
            *mut int32_t;
    kvnos =
        ealloc((nstrings as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_kvno>()
                                                    as libc::c_ulong)) as
            *mut krb5_kvno;
    /* Convert each string into a key/salt tuple list and count the total
     * number of key data structures needed. */
    nkeys = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nstrings {
        s = *strings.offset(i as isize);
        /* Read a leading kvno if present; otherwise assume kvno 1. */
        if *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
               libc::c_int &
               _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 {
            *kvnos.offset(i as isize) =
                strtol(s, &mut s, 10 as libc::c_int) as krb5_kvno;
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
                s = s.offset(1)
            }
        } else { *kvnos.offset(i as isize) = 1 as libc::c_int as krb5_kvno }
        check(krb5_string_to_keysalts(s, 0 as *const libc::c_char,
                                      0 as *const libc::c_char,
                                      0 as libc::c_int as krb5_boolean,
                                      &mut *ks_lists.offset(i as isize),
                                      &mut *ks_list_sizes.offset(i as
                                                                     isize)));
        nkeys += *ks_list_sizes.offset(i as isize);
        i += 1
    }
    /* Turn each key/salt tuple into a key data entry. */
    key_data =
        ealloc((nkeys as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_data>()
                                                    as libc::c_ulong)) as
            *mut krb5_key_data;
    kd = key_data;
    i = 0 as libc::c_int;
    while i < nstrings {
        ks = *ks_lists.offset(i as isize);
        j = 0 as libc::c_int;
        while j < *ks_list_sizes.offset(i as isize) {
            make_keyblock(*kvnos.offset(i as isize),
                          (*ks.offset(j as isize)).ks_enctype,
                          (*ks.offset(j as isize)).ks_salttype, princstr,
                          realm, &mut kb);
            (*kd).key_data_ver = 2 as libc::c_int as krb5_int16;
            (*kd).key_data_kvno = *kvnos.offset(i as isize) as krb5_ui_2;
            (*kd).key_data_type[0 as libc::c_int as usize] =
                (*ks.offset(j as isize)).ks_enctype as krb5_int16;
            (*kd).key_data_length[0 as libc::c_int as usize] =
                kb.length as krb5_ui_2;
            (*kd).key_data_contents[0 as libc::c_int as usize] = kb.contents;
            (*kd).key_data_type[1 as libc::c_int as usize] =
                (*ks.offset(j as isize)).ks_salttype as krb5_int16;
            kd = kd.offset(1);
            j += 1
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < nstrings {
        free(*ks_lists.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(ks_lists as *mut libc::c_void);
    free(ks_list_sizes as *mut libc::c_void);
    free(kvnos as *mut libc::c_void);
    (*ent).key_data = key_data;
    (*ent).n_key_data = nkeys as krb5_int16;
}
#[c2rust::src_loc = "286:1"]
unsafe extern "C" fn test_init() -> krb5_error_code {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "292:1"]
unsafe extern "C" fn test_cleanup() -> krb5_error_code {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "298:1"]
unsafe extern "C" fn test_open(mut context: krb5_context,
                               mut conf_section: *mut libc::c_char,
                               mut db_args: *mut *mut libc::c_char,
                               mut mode: libc::c_int) -> krb5_error_code {
    let mut h: testhandle = 0 as *mut C2RustUnnamed_1;
    h =
        ealloc(::std::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong) as
            testhandle;
    (*h).profile = (*context).profile as *mut libc::c_void;
    (*h).section = estrdup(conf_section);
    (*(*context).dal_handle).db_context = h as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "310:1"]
unsafe extern "C" fn test_close(mut context: krb5_context)
 -> krb5_error_code {
    let mut h: testhandle = (*(*context).dal_handle).db_context as testhandle;
    free((*h).section as *mut libc::c_void);
    free(h as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* Return the principal name krbtgt/tgs_realm@our_realm. */
#[c2rust::src_loc = "321:1"]
unsafe extern "C" fn tgtname(mut context: krb5_context,
                             mut tgs_realm: *const krb5_data,
                             mut our_realm: *const krb5_data)
 -> krb5_principal {
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    check(krb5_build_principal_ext(context, &mut princ as *mut krb5_principal,
                                   (*our_realm).length, (*our_realm).data,
                                   6 as libc::c_int,
                                   b"krbtgt\x00" as *const u8 as
                                       *const libc::c_char,
                                   (*tgs_realm).length, (*tgs_realm).data,
                                   0 as libc::c_int));
    (*princ).type_0 = 2 as libc::c_int;
    return princ;
}
#[c2rust::src_loc = "335:1"]
unsafe extern "C" fn test_get_principal(mut context: krb5_context,
                                        mut search_for: krb5_const_principal,
                                        mut flags: libc::c_uint,
                                        mut entry: *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut tgtprinc: krb5_principal = 0 as *mut krb5_principal_data;
    let mut empty_princ: krb5_principal_data =
        {
            let mut init =
                krb5_principal_data{magic:
                                        -(1760647423 as libc::c_long) as
                                            krb5_magic,
                                    realm:
                                        krb5_data{magic: 0,
                                                  length: 0,
                                                  data:
                                                      0 as
                                                          *mut libc::c_char,},
                                    data: 0 as *mut krb5_data,
                                    length: 0,
                                    type_0: 0,};
            init
        };
    let mut h: testhandle = (*(*context).dal_handle).db_context as testhandle;
    let mut search_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flagstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut key_strings: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ename: *const libc::c_char = 0 as *const libc::c_char;
    let mut ent: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    *entry = 0 as *mut krb5_db_entry;
    check(krb5_unparse_name_flags(context, search_for, 0x2 as libc::c_int,
                                  &mut search_name));
    canon =
        get_string(h, b"alias\x00" as *const u8 as *const libc::c_char,
                   search_name, 0 as *const libc::c_char);
    if !canon.is_null() {
        check(krb5_parse_name(context, canon, &mut princ));
        if krb5_realm_compare(context, search_for,
                              princ as krb5_const_principal) == 0 {
            /* Out of realm */
            if flags & 0x40 as libc::c_int as libc::c_uint != 0 &&
                   (flags & 0x10 as libc::c_int as libc::c_uint != 0 ||
                        (*search_for).type_0 == 10 as libc::c_int) {
                /* Return a client referral by creating an entry with only the
                 * principal set. */
                *entry =
                    ealloc(::std::mem::size_of::<krb5_db_entry>() as
                               libc::c_ulong) as *mut krb5_db_entry;
                (**entry).princ = princ;
                princ = 0 as krb5_principal;
                ret = 0 as libc::c_int;
                current_block = 5244350891429070227;
            } else if flags & 0x10 as libc::c_int as libc::c_uint != 0 {
                /* Generate a server referral by looking up the TGT for the
                 * canonical name's realm. */
                tgtprinc =
                    tgtname(context, &mut (*princ).realm,
                            &(*search_for).realm);
                krb5_free_principal(context, princ);
                princ = tgtprinc;
                krb5_free_unparsed_name(context, search_name);
                check(krb5_unparse_name_flags(context,
                                              princ as krb5_const_principal,
                                              0x2 as libc::c_int,
                                              &mut search_name));
                ename = search_name;
                current_block = 14648156034262866959;
            } else {
                ret = -(1780008443 as libc::c_long) as krb5_error_code;
                current_block = 5244350891429070227;
            }
        } else { ename = canon; current_block = 14648156034262866959; }
    } else {
        check(krb5_copy_principal(context, search_for, &mut princ));
        ename = search_name;
        current_block = 14648156034262866959;
    }
    match current_block {
        14648156034262866959 => {
            /* Check that the entry exists. */
            set_names(h, b"princs\x00" as *const u8 as *const libc::c_char,
                      ename, 0 as *const libc::c_char);
            ret =
                profile_get_relation_names((*h).profile as profile_t,
                                           (*h).names.as_mut_ptr(),
                                           &mut names) as krb5_error_code;
            if ret as libc::c_long == -(1429577725 as libc::c_long) {
                ret = -(1780008443 as libc::c_long) as krb5_error_code
            } else {
                profile_free_list(names);
                /* No error exits after this point. */
                ent =
                    ealloc(::std::mem::size_of::<krb5_db_entry>() as
                               libc::c_ulong) as *mut krb5_db_entry;
                (*ent).princ = princ;
                princ = 0 as krb5_principal;
                flagstr =
                    get_string(h,
                               b"princs\x00" as *const u8 as
                                   *const libc::c_char, ename,
                               b"flags\x00" as *const u8 as
                                   *const libc::c_char);
                if !flagstr.is_null() {
                    check(krb5_flagspec_to_mask(flagstr,
                                                &mut (*ent).attributes,
                                                &mut (*ent).attributes));
                }
                free(flagstr as *mut libc::c_void);
                (*ent).max_life =
                    get_duration(h,
                                 b"princs\x00" as *const u8 as
                                     *const libc::c_char, ename,
                                 b"maxlife\x00" as *const u8 as
                                     *const libc::c_char);
                (*ent).max_renewable_life =
                    get_duration(h,
                                 b"princs\x00" as *const u8 as
                                     *const libc::c_char, ename,
                                 b"maxrenewlife\x00" as *const u8 as
                                     *const libc::c_char);
                (*ent).expiration =
                    get_time(h,
                             b"princs\x00" as *const u8 as
                                 *const libc::c_char, ename,
                             b"expiration\x00" as *const u8 as
                                 *const libc::c_char);
                (*ent).pw_expiration =
                    get_time(h,
                             b"princs\x00" as *const u8 as
                                 *const libc::c_char, ename,
                             b"pwexpiration\x00" as *const u8 as
                                 *const libc::c_char);
                /* Leave last_success, last_failed, fail_auth_count zeroed. */
    /* Leave tl_data and e_data empty. */
                set_names(h,
                          b"princs\x00" as *const u8 as *const libc::c_char,
                          ename,
                          b"keys\x00" as *const u8 as *const libc::c_char);
                ret =
                    profile_get_values((*h).profile as profile_t,
                                       (*h).names.as_mut_ptr(),
                                       &mut key_strings) as krb5_error_code;
                if ret as libc::c_long != -(1429577725 as libc::c_long) {
                    make_keys(key_strings, ename, &(*search_for).realm, ent);
                    profile_free_list(key_strings);
                }
                /* We must include mod-princ data or kadm5_get_principal() won't work and
     * we can't extract keys with kadmin.local. */
                check(krb5_dbe_update_mod_princ_data(context, ent,
                                                     0 as libc::c_int,
                                                     &mut empty_princ as
                                                         *mut krb5_principal_data
                                                         as
                                                         krb5_const_principal));
                *entry = ent
            }
        }
        _ => { }
    }
    krb5_free_unparsed_name(context, search_name);
    krb5_free_principal(context, princ);
    free(canon as *mut libc::c_void);
    return ret;
}
#[c2rust::src_loc = "441:1"]
unsafe extern "C" fn lookup_princ_by_cert(mut context: krb5_context,
                                          mut client_cert: *const krb5_data,
                                          mut princ: *mut krb5_principal) {
    let mut ret: krb5_error_code = 0;
    let mut cert_princ_name: *mut libc::c_char = 0 as *mut libc::c_char;
    /* The test client sends a principal string instead of a cert. */
    cert_princ_name =
        k5memdup0((*client_cert).data as *const libc::c_void,
                  (*client_cert).length as size_t, &mut ret) as
            *mut libc::c_char;
    check(ret);
    check(krb5_parse_name(context, cert_princ_name, princ));
    free(cert_princ_name as *mut libc::c_void);
}
#[c2rust::src_loc = "456:1"]
unsafe extern "C" fn test_get_s4u_x509_principal(mut context: krb5_context,
                                                 mut client_cert:
                                                     *const krb5_data,
                                                 mut princ:
                                                     krb5_const_principal,
                                                 mut flags: libc::c_uint,
                                                 mut entry:
                                                     *mut *mut krb5_db_entry)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut cert_princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut canon_princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut h: testhandle = (*(*context).dal_handle).db_context as testhandle;
    let mut match_0: krb5_boolean = 0;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ_name: *mut libc::c_char = 0 as *mut libc::c_char;
    lookup_princ_by_cert(context, client_cert, &mut cert_princ);
    ret =
        test_get_principal(context, cert_princ as krb5_const_principal, flags,
                           entry);
    krb5_free_principal(context, cert_princ);
    if ret != 0 || flags & 0x40 as libc::c_int as libc::c_uint != 0 {
        return ret
    }
    if krb5_realm_compare(context, princ,
                          (**entry).princ as krb5_const_principal) == 0 {
        abort();
    }
    if (*princ).length == 0 as libc::c_int ||
           krb5_principal_compare(context, princ,
                                  (**entry).princ as krb5_const_principal) !=
               0 {
        return 0 as libc::c_int
    }
    match_0 = 0 as libc::c_int as krb5_boolean;
    check(krb5_unparse_name_flags(context, princ, 0x2 as libc::c_int,
                                  &mut princ_name));
    canon =
        get_string(h, b"alias\x00" as *const u8 as *const libc::c_char,
                   princ_name, 0 as *const libc::c_char);
    krb5_free_unparsed_name(context, princ_name);
    if !canon.is_null() {
        check(krb5_parse_name(context, canon, &mut canon_princ));
        match_0 =
            krb5_principal_compare(context,
                                   canon_princ as krb5_const_principal,
                                   (**entry).princ as krb5_const_principal);
        krb5_free_principal(context, canon_princ);
    }
    free(canon as *mut libc::c_void);
    return if match_0 != 0 {
               0 as libc::c_int as libc::c_long
           } else { -(1765328309 as libc::c_long) } as krb5_error_code;
}
#[c2rust::src_loc = "497:1"]
unsafe extern "C" fn test_fetch_master_key(mut context: krb5_context,
                                           mut mname: krb5_principal,
                                           mut key_out: *mut krb5_keyblock,
                                           mut kvno_out: *mut krb5_kvno,
                                           mut db_args: *mut libc::c_char)
 -> krb5_error_code {
    memset(key_out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    *kvno_out = 0 as libc::c_int as krb5_kvno;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "507:1"]
unsafe extern "C" fn test_fetch_master_key_list(mut context: krb5_context,
                                                mut mname: krb5_principal,
                                                mut key: *const krb5_keyblock,
                                                mut mkeys_out:
                                                    *mut *mut krb5_keylist_node)
 -> krb5_error_code {
    /* krb5_dbe_get_mkvno() returns an error if we produce NULL, so return an
     * empty node to make kadm5_get_principal() work. */
    *mkeys_out =
        ealloc(::std::mem::size_of::<krb5_keylist_node>() as libc::c_ulong) as
            *mut krb5_keylist_node;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "518:1"]
unsafe extern "C" fn test_decrypt_key_data(mut context: krb5_context,
                                           mut mkey: *const krb5_keyblock,
                                           mut kd: *const krb5_key_data,
                                           mut key_out: *mut krb5_keyblock,
                                           mut salt_out: *mut krb5_keysalt)
 -> krb5_error_code {
    (*key_out).magic = -(1760647421 as libc::c_long) as krb5_magic;
    (*key_out).enctype =
        (*kd).key_data_type[0 as libc::c_int as usize] as krb5_enctype;
    (*key_out).length =
        (*kd).key_data_length[0 as libc::c_int as usize] as libc::c_uint;
    (*key_out).contents =
        ealloc((*key_out).length as size_t) as *mut krb5_octet;
    memcpy((*key_out).contents as *mut libc::c_void,
           (*kd).key_data_contents[0 as libc::c_int as usize] as
               *const libc::c_void, (*key_out).length as libc::c_ulong);
    if !salt_out.is_null() {
        (*salt_out).type_0 =
            if (*kd).key_data_ver as libc::c_int > 1 as libc::c_int {
                (*kd).key_data_type[1 as libc::c_int as usize] as libc::c_int
            } else { 0 as libc::c_int } as krb5_int16;
        (*salt_out).data = empty_data()
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "536:1"]
unsafe extern "C" fn test_encrypt_key_data(mut context: krb5_context,
                                           mut mkey: *const krb5_keyblock,
                                           mut key: *const krb5_keyblock,
                                           mut salt: *const krb5_keysalt,
                                           mut kvno: libc::c_int,
                                           mut kd_out: *mut krb5_key_data)
 -> krb5_error_code {
    memset(kd_out as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
    (*kd_out).key_data_ver = 2 as libc::c_int as krb5_int16;
    (*kd_out).key_data_kvno = kvno as krb5_ui_2;
    (*kd_out).key_data_type[0 as libc::c_int as usize] =
        (*key).enctype as krb5_int16;
    (*kd_out).key_data_length[0 as libc::c_int as usize] =
        (*key).length as krb5_ui_2;
    (*kd_out).key_data_contents[0 as libc::c_int as usize] =
        ealloc((*key).length as size_t) as *mut krb5_octet;
    memcpy((*kd_out).key_data_contents[0 as libc::c_int as usize] as
               *mut libc::c_void, (*key).contents as *const libc::c_void,
           (*key).length as libc::c_ulong);
    (*kd_out).key_data_type[1 as libc::c_int as usize] =
        if !salt.is_null() {
            (*salt).type_0 as libc::c_int
        } else { 0 as libc::c_int } as krb5_int16;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "563:1"]
unsafe extern "C" fn free_pac_info(mut context: krb5_context,
                                   mut info: *mut pac_info) {
    if info.is_null() { return }
    free((*info).pac_princ as *mut libc::c_void);
    free((*info).deleg_info.proxy_target as *mut libc::c_void);
    free((*info).deleg_info.impersonator as *mut libc::c_void);
    krb5_pac_free(context, (*info).pac);
    free(info as *mut libc::c_void);
}
/*
 * Create a PAC object with a fake logon-info blob.  Instead of a real
 * KERB_VALIDATION_INFO structure, store a byte indicating whether the
 * USER_NOT_DELEGATED bit is set.
 */
#[c2rust::src_loc = "581:1"]
unsafe extern "C" fn create_pac(mut context: krb5_context,
                                mut not_delegated: krb5_boolean,
                                mut pac_out: *mut krb5_pac)
 -> krb5_error_code {
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pac: krb5_pac = 0 as *mut krb5_pac_data;
    let mut nd: libc::c_char = 0;
    nd =
        if not_delegated != 0 { 1 as libc::c_int } else { 0 as libc::c_int }
            as libc::c_char;
    data =
        make_data(&mut nd as *mut libc::c_char as *mut libc::c_void,
                  1 as libc::c_int as libc::c_uint);
    check(krb5_pac_init(context, &mut pac));
    check(krb5_pac_add_buffer(context, pac, 1 as libc::c_int as krb5_ui_4,
                              &mut data));
    *pac_out = pac;
    return 0 as libc::c_int;
}
/* Create a fake PAC, setting the USER_NOT_DELEGATED bit if the client DB entry
 * disallows forwardable tickets. */
#[c2rust::src_loc = "599:1"]
unsafe extern "C" fn create_pac_db(mut context: krb5_context,
                                   mut client: *mut krb5_db_entry,
                                   mut pac_out: *mut krb5_pac)
 -> krb5_error_code {
    let mut not_delegated: krb5_boolean = 0;
    /* Use disallow_forwardable as delegation_not_allowed attribute */
    not_delegated =
        ((*client).attributes & 0x2 as libc::c_int) as krb5_boolean;
    return create_pac(context, not_delegated, pac_out);
}
/* Locate the PAC in tgt_authdata and set *pac_out to its PAC object
 * representation.  Set it to NULL if no PAC is present. */
#[c2rust::src_loc = "610:1"]
unsafe extern "C" fn parse_ticket_pac(mut context: krb5_context,
                                      mut tgt_auth_data:
                                          *mut *mut krb5_authdata,
                                      mut pac_out: *mut krb5_pac) {
    let mut authdata: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    *pac_out = 0 as krb5_pac;
    check(krb5_find_authdata(context, tgt_auth_data,
                             0 as *const *mut krb5_authdata,
                             128 as libc::c_int, &mut authdata));
    if authdata.is_null() { return }
    if (*authdata.offset(1 as libc::c_int as isize)).is_null() {
    } else {
        __assert_fail(b"authdata[1] == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_test.c\x00" as *const u8 as *const libc::c_char,
                      622 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"void parse_ticket_pac(krb5_context, krb5_authdata **, krb5_pac *)\x00")).as_ptr());
    }
    check(krb5_pac_parse(context,
                         (**authdata.offset(0 as libc::c_int as
                                                isize)).contents as
                             *const libc::c_void,
                         (**authdata.offset(0 as libc::c_int as isize)).length
                             as size_t, pac_out));
    krb5_free_authdata(context, authdata);
}
/* Verify the KDC signature against the local TGT key.  tgt_key must be the
 * decrypted first key data entry of tgt. */
#[c2rust::src_loc = "630:1"]
unsafe extern "C" fn verify_kdc_signature(mut context: krb5_context,
                                          mut pac: krb5_pac,
                                          mut tgt_key: *mut krb5_keyblock,
                                          mut tgt: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut old_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut kvno: krb5_kvno = 0;
    let mut tries: libc::c_int = 0;
    ret =
        krb5_pac_verify(context, pac, 0 as libc::c_int,
                        0 as krb5_const_principal, 0 as *const krb5_keyblock,
                        tgt_key);
    if ret as libc::c_long != -(1765328353 as libc::c_long) { return ret }
    kvno =
        ((*(*tgt).key_data.offset(0 as libc::c_int as isize)).key_data_kvno as
             libc::c_int - 1 as libc::c_int) as krb5_kvno;
    /* There is no kvno in PAC signatures, so try two previous versions. */
    tries = 2 as libc::c_int;
    while tries > 0 as libc::c_int && kvno > 0 as libc::c_int as libc::c_uint
          {
        ret =
            krb5_dbe_find_enctype(context, tgt, -(1 as libc::c_int),
                                  -(1 as libc::c_int), kvno as krb5_int32,
                                  &mut kd);
        if ret != 0 {
            return -(1765328353 as libc::c_long) as krb5_error_code
        }
        ret =
            krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock, kd,
                                      &mut old_key, 0 as *mut krb5_keysalt);
        if ret != 0 { return ret }
        ret =
            krb5_pac_verify(context, pac, 0 as libc::c_int,
                            0 as krb5_const_principal,
                            0 as *const krb5_keyblock, &mut old_key);
        krb5_free_keyblock_contents(context, &mut old_key);
        if ret == 0 { return 0 as libc::c_int }
        /* Try the next lower kvno on the next iteration. */
        kvno =
            ((*kd).key_data_kvno as libc::c_int - 1 as libc::c_int) as
                krb5_kvno;
        tries -= 1;
        kvno = kvno.wrapping_sub(1)
    }
    return -(1765328353 as libc::c_long) as krb5_error_code;
}
#[c2rust::src_loc = "666:1"]
unsafe extern "C" fn verify_ticket_pac(mut context: krb5_context,
                                       mut pac: krb5_pac,
                                       mut flags: libc::c_uint,
                                       mut client_princ: krb5_const_principal,
                                       mut check_realm: krb5_boolean,
                                       mut server_key: *mut krb5_keyblock,
                                       mut local_tgt_key: *mut krb5_keyblock,
                                       mut local_tgt: *mut krb5_db_entry,
                                       mut authtime: krb5_timestamp)
 -> krb5_error_code {
    check(krb5_pac_verify_ext(context, pac, authtime, client_princ,
                              server_key, 0 as *const krb5_keyblock,
                              check_realm));
    if flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
        return 0 as libc::c_int
    }
    return verify_kdc_signature(context, pac, local_tgt_key, local_tgt);
}
#[c2rust::src_loc = "679:1"]
unsafe extern "C" fn get_pac_info(mut context: krb5_context,
                                  mut in_authdata: *mut *mut krb5_authdata,
                                  mut info_out: *mut *mut pac_info) {
    let mut ret: krb5_error_code = 0;
    let mut pac: krb5_pac = 0 as krb5_pac;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: *mut pac_info = 0 as *mut pac_info;
    *info_out = 0 as *mut pac_info;
    parse_ticket_pac(context, in_authdata, &mut pac);
    if pac.is_null() { return }
    info =
        ealloc(::std::mem::size_of::<pac_info>() as libc::c_ulong) as
            *mut pac_info;
    /* Read the fake logon-info buffer from the PAC and set not_delegated
     * according to the byte value. */
    check(krb5_pac_get_client_info(context, pac, 0 as *mut krb5_timestamp,
                                   &mut (*info).pac_princ));
    check(krb5_pac_get_buffer(context, pac, 1 as libc::c_int as krb5_ui_4,
                              &mut data));
    if data.length == 1 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"data.length == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_test.c\x00" as *const u8 as *const libc::c_char,
                      701 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 63],
                                                &[libc::c_char; 63]>(b"void get_pac_info(krb5_context, krb5_authdata **, pac_info **)\x00")).as_ptr());
    }
    (*info).not_delegated = *data.data as krb5_boolean;
    krb5_free_data_contents(context, &mut data);
    ret =
        krb5_pac_get_buffer(context, pac, 11 as libc::c_int as krb5_ui_4,
                            &mut data);
    if ret != 0 && ret != 2 as libc::c_int { abort(); }
    if ret == 0 {
        sep =
            memchr(data.data as *const libc::c_void, ':' as i32,
                   data.length as libc::c_ulong) as *mut libc::c_char;
        if !sep.is_null() {
        } else {
            __assert_fail(b"sep != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"kdb_test.c\x00" as *const u8 as
                              *const libc::c_char,
                          710 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 63],
                                                    &[libc::c_char; 63]>(b"void get_pac_info(krb5_context, krb5_authdata **, pac_info **)\x00")).as_ptr());
        }
        (*info).deleg_info.proxy_target =
            k5memdup0(data.data as *const libc::c_void,
                      sep.wrapping_offset_from(data.data) as libc::c_long as
                          size_t, &mut ret) as *mut libc::c_char;
        check(ret);
        (*info).deleg_info.impersonator =
            k5memdup0(sep.offset(1 as libc::c_int as isize) as
                          *const libc::c_void,
                      (data.length.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) as
                           libc::c_long -
                           sep.wrapping_offset_from(data.data) as
                               libc::c_long) as size_t, &mut ret) as
                *mut libc::c_char;
        check(ret);
        krb5_free_data_contents(context, &mut data);
    }
    (*info).pac = pac;
    *info_out = info;
}
/* Add a fake delegation-info buffer to pac containing the proxy target and
 * impersonator from info. */
#[c2rust::src_loc = "726:1"]
unsafe extern "C" fn add_delegation_info(mut context: krb5_context,
                                         mut pac: krb5_pac,
                                         mut info: *mut pac_info) {
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*info).deleg_info.proxy_target.is_null() { return }
    if asprintf(&mut str as *mut *mut libc::c_char,
                b"%s:%s\x00" as *const u8 as *const libc::c_char,
                (*info).deleg_info.proxy_target,
                (*info).deleg_info.impersonator) < 0 as libc::c_int {
        abort();
    }
    data = string2data(str);
    check(krb5_pac_add_buffer(context, pac, 11 as libc::c_int as krb5_ui_4,
                              &mut data));
    free(str as *mut libc::c_void);
}
/* Set *out to an AD-IF-RELEVANT authdata element containing a PAC authdata
 * element with contents pac_data. */
#[c2rust::src_loc = "745:1"]
unsafe extern "C" fn encode_pac_ad(mut context: krb5_context,
                                   mut pac_data: *mut krb5_data,
                                   mut out: *mut *mut krb5_authdata) {
    let mut pac_ad: krb5_authdata =
        krb5_authdata{magic: 0,
                      ad_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut list: [*mut krb5_authdata; 2] = [0 as *mut krb5_authdata; 2];
    let mut ifrel: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    pac_ad.magic = -(1760647414 as libc::c_long) as krb5_magic;
    pac_ad.ad_type = 128 as libc::c_int;
    pac_ad.contents = (*pac_data).data as *mut krb5_octet;
    pac_ad.length = (*pac_data).length;
    list[0 as libc::c_int as usize] = &mut pac_ad;
    list[1 as libc::c_int as usize] = 0 as *mut krb5_authdata;
    check(krb5_encode_authdata_container(context, 1 as libc::c_int,
                                         list.as_mut_ptr(), &mut ifrel));
    if (*ifrel.offset(1 as libc::c_int as isize)).is_null() {
    } else {
        __assert_fail(b"ifrel[1] == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_test.c\x00" as *const u8 as *const libc::c_char,
                      759 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"void encode_pac_ad(krb5_context, krb5_data *, krb5_authdata **)\x00")).as_ptr());
    }
    *out = *ifrel.offset(0 as libc::c_int as isize);
    free(ifrel as *mut libc::c_void);
}
/* Parse a PAC client-info string into a principal name.  If xrealm_s4u is
 * true, expect a realm in the string. */
#[c2rust::src_loc = "766:1"]
unsafe extern "C" fn parse_pac_princ(mut context: krb5_context,
                                     mut xrealm_s4u: krb5_boolean,
                                     mut pac_princ: *mut libc::c_char,
                                     mut client_out: *mut krb5_principal)
 -> krb5_error_code {
    let mut n_atsigns: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = pac_princ;
    loop  {
        let fresh0 = p;
        p = p.offset(1);
        if !(*fresh0 != 0) { break ; }
        if *p as libc::c_int == '@' as i32 { n_atsigns += 1 }
    }
    if xrealm_s4u != 0 {
        flags |= 0x2 as libc::c_int;
        n_atsigns -= 1
    } else { flags |= 0x1 as libc::c_int }
    if n_atsigns == 0 as libc::c_int || n_atsigns == 1 as libc::c_int {
    } else {
        __assert_fail(b"n_atsigns == 0 || n_atsigns == 1\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_test.c\x00" as *const u8 as *const libc::c_char,
                      783 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"krb5_error_code parse_pac_princ(krb5_context, krb5_boolean, char *, krb5_principal *)\x00")).as_ptr());
    }
    if n_atsigns == 1 as libc::c_int { flags |= 0x4 as libc::c_int }
    check(krb5_parse_name_flags(context, pac_princ, flags, client_out));
    (**client_out).type_0 = -(128 as libc::c_int);
    return 0 as libc::c_int;
}
/* Set *ad_out to a fake PAC for testing, or to NULL if it doesn't make sense
 * to generate a PAC for the request. */
#[c2rust::src_loc = "793:1"]
unsafe extern "C" fn generate_pac(mut context: krb5_context,
                                  mut flags: libc::c_uint,
                                  mut client_princ: krb5_const_principal,
                                  mut server_princ: krb5_const_principal,
                                  mut client: *mut krb5_db_entry,
                                  mut header_server: *mut krb5_db_entry,
                                  mut local_tgt: *mut krb5_db_entry,
                                  mut server_key: *mut krb5_keyblock,
                                  mut header_key: *mut krb5_keyblock,
                                  mut local_tgt_key: *mut krb5_keyblock,
                                  mut authtime: krb5_timestamp,
                                  mut info: *mut pac_info,
                                  mut ad_out: *mut *mut krb5_authdata) {
    let mut sign_realm: krb5_boolean = 0;
    let mut check_realm: krb5_boolean = 0;
    let mut pac_data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pac: krb5_pac = 0 as krb5_pac;
    let mut pac_princ: krb5_principal = 0 as krb5_principal;
    *ad_out = 0 as *mut krb5_authdata;
    check_realm =
        (flags & (0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint
             != 0 && flags & 0x1000 as libc::c_int as libc::c_uint != 0) as
            libc::c_int as krb5_boolean;
    sign_realm =
        (flags & (0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint
             != 0 && flags & 0x4000 as libc::c_int as libc::c_uint != 0) as
            libc::c_int as krb5_boolean;
    if !client.is_null() &&
           (flags & 0x40 as libc::c_int as libc::c_uint != 0 ||
                flags & 0x100 as libc::c_int as libc::c_uint != 0) {
        /* For AS or local-realm S4U2Self, generate an initial PAC. */
        check(create_pac_db(context, client, &mut pac));
    } else if info.is_null() {
        /* If there is no input PAC, do not generate one. */
        if flags &
               (0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint
               == 0 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"(flags & KRB5_KDB_FLAGS_S4U) == 0\x00" as
                              *const u8 as *const libc::c_char,
                          b"kdb_test.c\x00" as *const u8 as
                              *const libc::c_char,
                          821 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 238],
                                                    &[libc::c_char; 238]>(b"void generate_pac(krb5_context, unsigned int, krb5_const_principal, krb5_const_principal, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_timestamp, pac_info *, krb5_authdata **)\x00")).as_ptr());
        }
        return
    } else {
        if (*server_princ).length == 2 as libc::c_int &&
               data_eq_string(*(*server_princ).data.offset(0 as libc::c_int as
                                                               isize),
                              b"krbtgt\x00" as *const u8 as
                                  *const libc::c_char) != 0 &&
               !(*info).deleg_info.proxy_target.is_null() {
            /* RBCD transitive trust. */
            if flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
            } else {
                __assert_fail(b"flags & KRB5_KDB_FLAG_CROSS_REALM\x00" as
                                  *const u8 as *const libc::c_char,
                              b"kdb_test.c\x00" as *const u8 as
                                  *const libc::c_char,
                              827 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 238],
                                                        &[libc::c_char; 238]>(b"void generate_pac(krb5_context, unsigned int, krb5_const_principal, krb5_const_principal, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_timestamp, pac_info *, krb5_authdata **)\x00")).as_ptr());
            }
            if flags & 0x200 as libc::c_int as libc::c_uint == 0 {
            } else {
                __assert_fail(b"!(flags & KRB5_KDB_FLAG_CONSTRAINED_DELEGATION)\x00"
                                  as *const u8 as *const libc::c_char,
                              b"kdb_test.c\x00" as *const u8 as
                                  *const libc::c_char,
                              828 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 238],
                                                        &[libc::c_char; 238]>(b"void generate_pac(krb5_context, unsigned int, krb5_const_principal, krb5_const_principal, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_timestamp, pac_info *, krb5_authdata **)\x00")).as_ptr());
            }
            check(parse_pac_princ(context, 1 as libc::c_int as krb5_boolean,
                                  (*info).pac_princ, &mut pac_princ));
            client_princ = pac_princ as krb5_const_principal;
            check_realm = 1 as libc::c_int as krb5_boolean;
            sign_realm = 1 as libc::c_int as krb5_boolean
        } else if flags & 0x200 as libc::c_int as libc::c_uint != 0 &&
                      flags & 0x1000 as libc::c_int as libc::c_uint == 0 {
            /*
             * Initial RBCD and old constrained delegation requests to
             * impersonator realm; create delegation info blob.  We cannot
             * assume that proxy_target is NULL as the evidence ticket could
             * have been acquired via constrained delegation.
             */
            free((*info).deleg_info.proxy_target as *mut libc::c_void);
            check(krb5_unparse_name_flags(context, server_princ,
                                          0x2 as libc::c_int,
                                          &mut (*info).deleg_info.proxy_target));
            /* This is supposed to be a list of impersonators, but we currently
             * only deal with one. */
            free((*info).deleg_info.impersonator as *mut libc::c_void);
            check(krb5_unparse_name(context,
                                    (*header_server).princ as
                                        krb5_const_principal,
                                    &mut (*info).deleg_info.impersonator));
        } else if flags & 0x200 as libc::c_int as libc::c_uint != 0 {
            /* Last cross realm RBCD request to proxy realm. */
            if !(*info).deleg_info.proxy_target.is_null() {
            } else {
                __assert_fail(b"info->deleg_info.proxy_target != NULL\x00" as
                                  *const u8 as *const libc::c_char,
                              b"kdb_test.c\x00" as *const u8 as
                                  *const libc::c_char,
                              852 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 238],
                                                        &[libc::c_char; 238]>(b"void generate_pac(krb5_context, unsigned int, krb5_const_principal, krb5_const_principal, krb5_db_entry *, krb5_db_entry *, krb5_db_entry *, krb5_keyblock *, krb5_keyblock *, krb5_keyblock *, krb5_timestamp, pac_info *, krb5_authdata **)\x00")).as_ptr());
            }
        }
        /* We have already verified the PAC in get_authdata_info, but we should
         * be able to verify the signatures here as well. */
        check(verify_ticket_pac(context, (*info).pac, flags, client_princ,
                                check_realm, header_key, local_tgt_key,
                                local_tgt, authtime));
        /* Create a new pac as we may be altering pac principal's realm */
        check(create_pac(context, (*info).not_delegated, &mut pac));
        add_delegation_info(context, pac, info);
    }
    check(krb5_pac_sign_ext(context, pac, authtime, client_princ, server_key,
                            local_tgt_key, sign_realm, &mut pac_data));
    krb5_pac_free(context, pac);
    krb5_free_principal(context, pac_princ);
    encode_pac_ad(context, &mut pac_data, ad_out);
    krb5_free_data_contents(context, &mut pac_data);
}
#[c2rust::src_loc = "873:1"]
unsafe extern "C" fn test_sign_authdata(mut context: krb5_context,
                                        mut flags: libc::c_uint,
                                        mut client_princ:
                                            krb5_const_principal,
                                        mut server_princ:
                                            krb5_const_principal,
                                        mut client: *mut krb5_db_entry,
                                        mut server: *mut krb5_db_entry,
                                        mut header_server: *mut krb5_db_entry,
                                        mut local_tgt: *mut krb5_db_entry,
                                        mut client_key: *mut krb5_keyblock,
                                        mut server_key: *mut krb5_keyblock,
                                        mut header_key: *mut krb5_keyblock,
                                        mut local_tgt_key: *mut krb5_keyblock,
                                        mut session_key: *mut krb5_keyblock,
                                        mut authtime: krb5_timestamp,
                                        mut tgt_auth_data:
                                            *mut *mut krb5_authdata,
                                        mut ad_info: *mut libc::c_void,
                                        mut auth_indicators:
                                            *mut *mut *mut krb5_data,
                                        mut signed_auth_data:
                                            *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut h: testhandle = (*(*context).dal_handle).db_context as testhandle;
    let mut pac_ad: *mut krb5_authdata = 0 as *mut krb5_authdata;
    let mut test_ad: *mut krb5_authdata = 0 as *mut krb5_authdata;
    let mut list: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut inds: *mut *mut krb5_data = 0 as *mut *mut krb5_data;
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut ad_type: *mut libc::c_char = 0 as *mut libc::c_char;
    generate_pac(context, flags, client_princ, server_princ, client,
                 header_server, local_tgt, server_key, header_key,
                 local_tgt_key, authtime, ad_info as *mut pac_info,
                 &mut pac_ad);
    /*
     * Omit test_ad if ad_type is mspac (only), as handle_signticket() fails in
     * constrained delegation if the PAC is not the only authorization data
     * element.
     */
    ad_type =
        get_string(h, b"ad_type\x00" as *const u8 as *const libc::c_char,
                   0 as *const libc::c_char, 0 as *const libc::c_char);
    if ad_type.is_null() ||
           strcmp(ad_type, b"mspac\x00" as *const u8 as *const libc::c_char)
               != 0 as libc::c_int {
        test_ad =
            ealloc(::std::mem::size_of::<krb5_authdata>() as libc::c_ulong) as
                *mut krb5_authdata;
        (*test_ad).magic = -(1760647414 as libc::c_long) as krb5_magic;
        (*test_ad).ad_type = -(456 as libc::c_int);
        (*test_ad).contents =
            estrdup(b"db-authdata-test\x00" as *const u8 as
                        *const libc::c_char) as *mut uint8_t;
        (*test_ad).length =
            strlen((*test_ad).contents as *mut libc::c_char) as libc::c_uint
    }
    free(ad_type as *mut libc::c_void);
    list =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_authdata;
    let ref mut fresh1 = *list.offset(0 as libc::c_int as isize);
    *fresh1 = if !test_ad.is_null() { test_ad } else { pac_ad };
    let ref mut fresh2 = *list.offset(1 as libc::c_int as isize);
    *fresh2 =
        if !test_ad.is_null() { pac_ad } else { 0 as *mut krb5_authdata };
    let ref mut fresh3 = *list.offset(2 as libc::c_int as isize);
    *fresh3 = 0 as *mut krb5_authdata;
    *signed_auth_data = list;
    /* If we see an auth indicator "dbincrX", replace the whole indicator list
     * with "dbincr{X+1}". */
    inds = *auth_indicators;
    i = 0 as libc::c_int;
    while !inds.is_null() && !(*inds.offset(i as isize)).is_null() {
        if (**inds.offset(i as isize)).length ==
               7 as libc::c_int as libc::c_uint &&
               memcmp((**inds.offset(i as isize)).data as *const libc::c_void,
                      b"dbincr\x00" as *const u8 as *const libc::c_char as
                          *const libc::c_void,
                      6 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
            val =
                *(**inds.offset(i as
                                    isize)).data.offset(6 as libc::c_int as
                                                            isize) as
                    libc::c_int;
            k5_free_data_ptr_list(inds);
            inds =
                ealloc((2 as libc::c_int as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_data>()
                                                            as libc::c_ulong))
                    as *mut *mut krb5_data;
            d =
                string2data(b"dbincr0\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char);
            check(krb5_copy_data(context, &mut d,
                                 &mut *inds.offset(0 as libc::c_int as
                                                       isize)));
            *(**inds.offset(0 as libc::c_int as
                                isize)).data.offset(6 as libc::c_int as isize)
                = (val + 1 as libc::c_int) as libc::c_char;
            let ref mut fresh4 = *inds.offset(1 as libc::c_int as isize);
            *fresh4 = 0 as *mut krb5_data;
            *auth_indicators = inds;
            break ;
        } else { i += 1 }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "936:1"]
unsafe extern "C" fn match_in_table(mut context: krb5_context,
                                    mut table: *const libc::c_char,
                                    mut sprinc: *const libc::c_char,
                                    mut tprinc: *const libc::c_char)
 -> krb5_boolean {
    let mut h: testhandle = (*(*context).dal_handle).db_context as testhandle;
    let mut ret: krb5_error_code = 0;
    let mut values: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut v: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut found: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    set_names(h, table, sprinc, 0 as *const libc::c_char);
    ret =
        profile_get_values((*h).profile as profile_t, (*h).names.as_mut_ptr(),
                           &mut values) as krb5_error_code;
    if ret == 0 as libc::c_int ||
           ret as libc::c_long == -(1429577725 as libc::c_long) {
    } else {
        __assert_fail(b"ret == 0 || ret == PROF_NO_RELATION\x00" as *const u8
                          as *const libc::c_char,
                      b"kdb_test.c\x00" as *const u8 as *const libc::c_char,
                      947 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"krb5_boolean match_in_table(krb5_context, const char *, const char *, const char *)\x00")).as_ptr());
    }
    if ret != 0 { return 0 as libc::c_int as krb5_boolean }
    v = values;
    while !(*v).is_null() {
        if strcmp(*v, tprinc) == 0 as libc::c_int {
            found = 1 as libc::c_int as krb5_boolean;
            break ;
        } else { v = v.offset(1) }
    }
    profile_free_list(values);
    return found;
}
#[c2rust::src_loc = "960:1"]
unsafe extern "C" fn test_check_allowed_to_delegate(mut context: krb5_context,
                                                    mut client:
                                                        krb5_const_principal,
                                                    mut server:
                                                        *const krb5_db_entry,
                                                    mut proxy:
                                                        krb5_const_principal)
 -> krb5_error_code {
    let mut sprinc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tprinc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    check(krb5_unparse_name_flags(context,
                                  (*server).princ as krb5_const_principal,
                                  0x2 as libc::c_int, &mut sprinc));
    check(krb5_unparse_name_flags(context, proxy, 0x2 as libc::c_int,
                                  &mut tprinc));
    found =
        match_in_table(context,
                       b"delegation\x00" as *const u8 as *const libc::c_char,
                       sprinc, tprinc);
    krb5_free_unparsed_name(context, sprinc);
    krb5_free_unparsed_name(context, tprinc);
    return if found != 0 {
               0 as libc::c_int as libc::c_long
           } else { -(1765328372 as libc::c_long) } as krb5_error_code;
}
#[c2rust::src_loc = "979:1"]
unsafe extern "C" fn test_allowed_to_delegate_from(mut context: krb5_context,
                                                   mut client:
                                                       krb5_const_principal,
                                                   mut server:
                                                       krb5_const_principal,
                                                   mut server_ad_info:
                                                       *mut libc::c_void,
                                                   mut proxy:
                                                       *const krb5_db_entry)
 -> krb5_error_code {
    let mut sprinc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tprinc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info: *mut pac_info = server_ad_info as *mut pac_info;
    let mut found: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    check(krb5_unparse_name(context, (*proxy).princ as krb5_const_principal,
                            &mut sprinc));
    check(krb5_unparse_name(context, server, &mut tprinc));
    if strncmp((*info).pac_princ, tprinc, strlen((*info).pac_princ)) ==
           0 as libc::c_int {
    } else {
        __assert_fail(b"strncmp(info->pac_princ, tprinc, strlen(info->pac_princ)) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"kdb_test.c\x00" as *const u8 as *const libc::c_char,
                      991 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 135],
                                                &[libc::c_char; 135]>(b"krb5_error_code test_allowed_to_delegate_from(krb5_context, krb5_const_principal, krb5_const_principal, void *, const krb5_db_entry *)\x00")).as_ptr());
    }
    found =
        match_in_table(context,
                       b"rbcd\x00" as *const u8 as *const libc::c_char,
                       sprinc, tprinc);
    krb5_free_unparsed_name(context, sprinc);
    krb5_free_unparsed_name(context, tprinc);
    return if found != 0 {
               0 as libc::c_int as libc::c_long
           } else { -(1765328372 as libc::c_long) } as krb5_error_code;
}
#[c2rust::src_loc = "998:1"]
unsafe extern "C" fn test_get_authdata_info(mut context: krb5_context,
                                            mut flags: libc::c_uint,
                                            mut in_authdata:
                                                *mut *mut krb5_authdata,
                                            mut client_princ:
                                                krb5_const_principal,
                                            mut server_princ:
                                                krb5_const_principal,
                                            mut server_key:
                                                *mut krb5_keyblock,
                                            mut krbtgt_key:
                                                *mut krb5_keyblock,
                                            mut krbtgt: *mut krb5_db_entry,
                                            mut authtime: krb5_timestamp,
                                            mut ad_info_out:
                                                *mut *mut libc::c_void,
                                            mut client_out:
                                                *mut krb5_principal)
 -> krb5_error_code {
    let mut info: *mut pac_info = 0 as *mut pac_info;
    let mut rbcd_transitive: krb5_boolean = 0;
    let mut xrealm_s4u: krb5_boolean = 0;
    let mut pac_princ: krb5_principal = 0 as krb5_principal;
    let mut proxy_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut impersonator_name: *mut libc::c_char = 0 as *mut libc::c_char;
    get_pac_info(context, in_authdata, &mut info);
    if info.is_null() { return 0 as libc::c_int }
    /* Transitive RBCD requests are not flagged as constrained delegation */
    if (*info).not_delegated != 0 &&
           (!(*info).deleg_info.proxy_target.is_null() ||
                flags & 0x200 as libc::c_int as libc::c_uint != 0) {
        free_pac_info(context, info);
        return -(1765328371 as libc::c_long) as krb5_error_code
    }
    rbcd_transitive =
        ((*server_princ).length == 2 as libc::c_int &&
             data_eq_string(*(*server_princ).data.offset(0 as libc::c_int as
                                                             isize),
                            b"krbtgt\x00" as *const u8 as *const libc::c_char)
                 != 0 && flags & 0x1000 as libc::c_int as libc::c_uint != 0 &&
             !(*info).deleg_info.proxy_target.is_null() &&
             flags & 0x200 as libc::c_int as libc::c_uint == 0) as libc::c_int
            as krb5_boolean;
    xrealm_s4u =
        (rbcd_transitive != 0 ||
             flags & 0x1000 as libc::c_int as libc::c_uint != 0 &&
                 flags &
                     (0x100 as libc::c_int | 0x200 as libc::c_int) as
                         libc::c_uint != 0) as libc::c_int as krb5_boolean;
    check(parse_pac_princ(context, xrealm_s4u, (*info).pac_princ,
                          &mut pac_princ));
    /* Cross-realm and transitive trust RBCD requests */
    if rbcd_transitive != 0 ||
           flags & 0x1000 as libc::c_int as libc::c_uint != 0 &&
               flags & 0x200 as libc::c_int as libc::c_uint != 0 {
        if !(*info).deleg_info.proxy_target.is_null() {
        } else {
            __assert_fail(b"info->deleg_info.proxy_target != NULL\x00" as
                              *const u8 as *const libc::c_char,
                          b"kdb_test.c\x00" as *const u8 as
                              *const libc::c_char,
                          1036 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 223],
                                                    &[libc::c_char; 223]>(b"krb5_error_code test_get_authdata_info(krb5_context, unsigned int, krb5_authdata **, krb5_const_principal, krb5_const_principal, krb5_keyblock *, krb5_keyblock *, krb5_db_entry *, krb5_timestamp, void **, krb5_principal *)\x00")).as_ptr());
        }
        if !(*info).deleg_info.impersonator.is_null() {
        } else {
            __assert_fail(b"info->deleg_info.impersonator != NULL\x00" as
                              *const u8 as *const libc::c_char,
                          b"kdb_test.c\x00" as *const u8 as
                              *const libc::c_char,
                          1037 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 223],
                                                    &[libc::c_char; 223]>(b"krb5_error_code test_get_authdata_info(krb5_context, unsigned int, krb5_authdata **, krb5_const_principal, krb5_const_principal, krb5_keyblock *, krb5_keyblock *, krb5_db_entry *, krb5_timestamp, void **, krb5_principal *)\x00")).as_ptr());
        }
        /* We must be able to find the impersonator in the delegation info. */
        if krb5_principal_compare(context, client_princ,
                                  pac_princ as krb5_const_principal) == 0 {
        } else {
            __assert_fail(b"!krb5_principal_compare(context, client_princ, pac_princ)\x00"
                              as *const u8 as *const libc::c_char,
                          b"kdb_test.c\x00" as *const u8 as
                              *const libc::c_char,
                          1039 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 223],
                                                    &[libc::c_char; 223]>(b"krb5_error_code test_get_authdata_info(krb5_context, unsigned int, krb5_authdata **, krb5_const_principal, krb5_const_principal, krb5_keyblock *, krb5_keyblock *, krb5_db_entry *, krb5_timestamp, void **, krb5_principal *)\x00")).as_ptr());
        }
        check(krb5_unparse_name(context, client_princ,
                                &mut impersonator_name));
        if strcmp((*info).deleg_info.impersonator, impersonator_name) ==
               0 as libc::c_int {
        } else {
            __assert_fail(b"strcmp(info->deleg_info.impersonator, impersonator_name) == 0\x00"
                              as *const u8 as *const libc::c_char,
                          b"kdb_test.c\x00" as *const u8 as
                              *const libc::c_char,
                          1041 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 223],
                                                    &[libc::c_char; 223]>(b"krb5_error_code test_get_authdata_info(krb5_context, unsigned int, krb5_authdata **, krb5_const_principal, krb5_const_principal, krb5_keyblock *, krb5_keyblock *, krb5_db_entry *, krb5_timestamp, void **, krb5_principal *)\x00")).as_ptr());
        }
        krb5_free_unparsed_name(context, impersonator_name);
        client_princ = pac_princ as krb5_const_principal;
        /* In the non-transitive case we can match the proxy too. */
        if rbcd_transitive == 0 {
            check(krb5_unparse_name_flags(context, server_princ,
                                          0x2 as libc::c_int,
                                          &mut proxy_name));
            if !(*info).deleg_info.proxy_target.is_null() {
            } else {
                __assert_fail(b"info->deleg_info.proxy_target != NULL\x00" as
                                  *const u8 as *const libc::c_char,
                              b"kdb_test.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1049 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 223],
                                                        &[libc::c_char; 223]>(b"krb5_error_code test_get_authdata_info(krb5_context, unsigned int, krb5_authdata **, krb5_const_principal, krb5_const_principal, krb5_keyblock *, krb5_keyblock *, krb5_db_entry *, krb5_timestamp, void **, krb5_principal *)\x00")).as_ptr());
            }
            if strcmp((*info).deleg_info.proxy_target, proxy_name) ==
                   0 as libc::c_int {
            } else {
                __assert_fail(b"strcmp(info->deleg_info.proxy_target, proxy_name) == 0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"kdb_test.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1050 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 223],
                                                        &[libc::c_char; 223]>(b"krb5_error_code test_get_authdata_info(krb5_context, unsigned int, krb5_authdata **, krb5_const_principal, krb5_const_principal, krb5_keyblock *, krb5_keyblock *, krb5_db_entry *, krb5_timestamp, void **, krb5_principal *)\x00")).as_ptr());
            }
            krb5_free_unparsed_name(context, proxy_name);
        }
    }
    check(verify_ticket_pac(context, (*info).pac, flags, client_princ,
                            xrealm_s4u, server_key, krbtgt_key, krbtgt,
                            authtime));
    *ad_info_out = info as *mut libc::c_void;
    if !client_out.is_null() {
        *client_out = pac_princ
    } else { krb5_free_principal(context, pac_princ); }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1068:1"]
unsafe extern "C" fn test_free_authdata_info(mut context: krb5_context,
                                             mut ad_info: *mut libc::c_void) {
    let mut info: *mut pac_info = ad_info as *mut pac_info;
    free_pac_info(context, info);
}
#[no_mangle]
#[c2rust::src_loc = "1076:42"]
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
                                                                                                                              krb5_error_code>(test_init))),
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
                                                                                                                              krb5_error_code>(test_cleanup))),
                            init_module:
                                Some(test_open as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut *mut libc::c_char,
                                                              _: libc::c_int)
                                             -> krb5_error_code),
                            fini_module:
                                Some(test_close as
                                         unsafe extern "C" fn(_: krb5_context)
                                             -> krb5_error_code),
                            create: None,
                            destroy: None,
                            get_age: None,
                            lock: None,
                            unlock: None,
                            get_principal:
                                Some(test_get_principal as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_const_principal,
                                                              _: libc::c_uint,
                                                              _:
                                                                  *mut *mut krb5_db_entry)
                                             -> krb5_error_code),
                            put_principal: None,
                            delete_principal: None,
                            rename_principal: None,
                            iterate: None,
                            create_policy: None,
                            get_policy: None,
                            put_policy: None,
                            iter_policy: None,
                            delete_policy: None,
                            fetch_master_key:
                                Some(test_fetch_master_key as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_principal,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  *mut krb5_kvno,
                                                              _:
                                                                  *mut libc::c_char)
                                             -> krb5_error_code),
                            fetch_master_key_list:
                                Some(test_fetch_master_key_list as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_principal,
                                                              _:
                                                                  *const krb5_keyblock,
                                                              _:
                                                                  *mut *mut krb5_keylist_node)
                                             -> krb5_error_code),
                            store_master_key_list: None,
                            dbe_search_enctype: None,
                            change_pwd: None,
                            promote_db: None,
                            decrypt_key_data:
                                Some(test_decrypt_key_data as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *const krb5_keyblock,
                                                              _:
                                                                  *const krb5_key_data,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  *mut krb5_keysalt)
                                             -> krb5_error_code),
                            encrypt_key_data:
                                Some(test_encrypt_key_data as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *const krb5_keyblock,
                                                              _:
                                                                  *const krb5_keyblock,
                                                              _:
                                                                  *const krb5_keysalt,
                                                              _: libc::c_int,
                                                              _:
                                                                  *mut krb5_key_data)
                                             -> krb5_error_code),
                            sign_authdata:
                                Some(test_sign_authdata as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _: libc::c_uint,
                                                              _:
                                                                  krb5_const_principal,
                                                              _:
                                                                  krb5_const_principal,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut krb5_db_entry,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  *mut krb5_keyblock,
                                                              _:
                                                                  krb5_timestamp,
                                                              _:
                                                                  *mut *mut krb5_authdata,
                                                              _:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  *mut *mut *mut krb5_data,
                                                              _:
                                                                  *mut *mut *mut krb5_authdata)
                                             -> krb5_error_code),
                            check_transited_realms: None,
                            check_policy_as: None,
                            check_policy_tgs: None,
                            audit_as_req: None,
                            refresh_config: None,
                            check_allowed_to_delegate:
                                Some(test_check_allowed_to_delegate as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_const_principal,
                                                              _:
                                                                  *const krb5_db_entry,
                                                              _:
                                                                  krb5_const_principal)
                                             -> krb5_error_code),
                            free_principal_e_data: None,
                            get_s4u_x509_principal:
                                Some(test_get_s4u_x509_principal as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *const krb5_data,
                                                              _:
                                                                  krb5_const_principal,
                                                              _: libc::c_uint,
                                                              _:
                                                                  *mut *mut krb5_db_entry)
                                             -> krb5_error_code),
                            allowed_to_delegate_from:
                                Some(test_allowed_to_delegate_from as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_const_principal,
                                                              _:
                                                                  krb5_const_principal,
                                                              _:
                                                                  *mut libc::c_void,
                                                              _:
                                                                  *const krb5_db_entry)
                                             -> krb5_error_code),
                            get_authdata_info:
                                Some(test_get_authdata_info as
                                         unsafe extern "C" fn(_: krb5_context,
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
                                                              _:
                                                                  krb5_timestamp,
                                                              _:
                                                                  *mut *mut libc::c_void,
                                                              _:
                                                                  *mut krb5_principal)
                                             -> krb5_error_code),
                            free_authdata_info:
                                Some(test_free_authdata_info as
                                         unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  *mut libc::c_void)
                                             -> ()),};
            init
        }
    };
