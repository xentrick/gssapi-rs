use ::libc;
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/spnego/gssapiP_spnego.h:32"]
pub mod gssapiP_spnego_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:8"]
    pub struct spnego_ctx_st {
        pub magic_num: OM_uint32,
        pub DER_mechTypes: gss_buffer_desc,
        pub mech_set: gss_OID_set,
        pub internal_mech: gss_OID,
        pub ctx_handle: gss_ctx_id_t,
        pub mic_reqd: libc::c_int,
        pub mic_sent: libc::c_int,
        pub mic_rcvd: libc::c_int,
        pub firstpass: libc::c_int,
        pub mech_complete: libc::c_int,
        pub nego_done: libc::c_int,
        pub initiate: libc::c_int,
        pub opened: libc::c_int,
        pub ctx_flags: OM_uint32,
        pub internal_name: gss_name_t,
        pub actual_mech: gss_OID,
        pub deleg_cred: gss_cred_id_t,
        pub negoex_step: libc::c_int,
        pub negoex_transcript: k5buf,
        pub negoex_seqnum: uint32_t,
        pub negoex_conv_id: conversation_id,
        pub negoex_mechs: negoex_mech_list,
        pub kctx: krb5_context,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:16"]
    pub struct negoex_mech_list {
        pub tqh_first: *mut negoex_auth_mech,
        pub tqh_last: *mut *mut negoex_auth_mech,
    }
    /*
 * Copyright 2003 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident	"@(#)gssapiP_spnego.h	1.3	03/09/18 SMI" */
    #[c2rust::src_loc = "15:1"]
    pub type spnego_gss_ctx_id_t = *mut spnego_ctx_st;
    use super::gssapi_h::{OM_uint32, gss_buffer_desc, gss_OID_set, gss_OID,
                          gss_ctx_id_t, gss_name_t, gss_cred_id_t};
    use super::k5_buf_h::k5buf;
    use super::stdint_uintn_h::uint32_t;
    use super::gssapiP_negoex_h::{conversation_id, negoex_auth_mech};
    use super::krb5_h::krb5_context;
    /* _GSSAPIP_SPNEGO_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
pub mod krb5_h {
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    /* *
 * Get prompt types array from a context.
 *
 * @param [in] context          Library context
 *
 * @return
 * Pointer to an array of prompt types corresponding to the prompter's @a
 * prompts arguments.  Each type has one of the following values:
 *  @li #KRB5_PROMPT_TYPE_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN
 *  @li #KRB5_PROMPT_TYPE_PREAUTH
 */
    /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
    /* *
 * Set an extended error message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] args             List of vprintf(3) style arguments
 */
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
    /* *
 * Add a prefix to the message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_prepend_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Add a prefix to a different error code's message.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the message for @a old_code.  The prefix
 * will be separated from the old message with a colon and space.  Set the
 * resulting message as the extended error message for @a code.
 */
    /* *
 * Add a prefix to a different error code's message using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_wrap_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Copy the most recent extended error message from one context to another.
 *
 * @param [in] dest_ctx         Library context to copy message to
 * @param [in] src_ctx          Library context with current message
 */
    /* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
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
    /* -1 when not specified. */
    /* -1 when not specified. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_OTP to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_OTP
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to interact with OTP tokens without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_otp_challenge_free() to
 * be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl             Challenge structure
 *
 * @version New in 1.11
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_OTP question.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] ti               The index of the tokeninfo selected
 * @param [in] value            The value to set, or NULL for none
 * @param [in] pin              The pin to set, or NULL for none
 *
 * @version New in 1.11
 */
    /* *
 * Free the value returned by krb5_responder_otp_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.11
 */
    /* 0 when not specified or not applicable. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_PKINIT to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_PKINIT
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to read the challenge without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_pkinit_challenge_free()
 * to be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl_out         Challenge structure
 *
 * @version New in 1.12
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_PKINIT question for one identity.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] identity         The identity for which a PIN is being supplied
 * @param [in] pin              The provided PIN, or NULL for none
 *
 * @version New in 1.12
 */
    /* *
 * Free the value returned by krb5_responder_pkinit_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.12
 */
    /* * Store options for @c _krb5_get_init_creds */
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
    /* *
 * Free initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure to free
 *
 * @sa krb5_get_init_creds_opt_alloc()
 */
    /* * @deprecated Use krb5_get_init_creds_opt_alloc() instead. */
    /* *
 * Set the ticket lifetime in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] tkt_life         Ticket lifetime
 */
    /* *
 * Set the ticket renewal lifetime in initial credential options.
 *
 * @param [in] opt              Pointer to @a options field
 * @param [in] renew_life       Ticket renewal lifetime
 */
    /* *
 * Set or unset the forwardable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] forwardable      Whether credentials should be forwardable
 */
    /* *
 * Set or unset the proxiable flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] proxiable        Whether credentials should be proxiable
 */
    /* *
 * Set or unset the canonicalize flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] canonicalize     Whether to canonicalize client principal
 */
    /* *
 * Set or unset the anonymous flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] anonymous        Whether to make an anonymous request
 *
 * This function may be used to request anonymous credentials from the KDC by
 * setting @a anonymous to non-zero.  Note that anonymous credentials are only
 * a request; clients must verify that credentials are anonymous if that is a
 * requirement.
 */
    /* *
 * Set allowable encryption types in initial credential options.
 *
 * @param [in] opt               Options structure
 * @param [in] etype_list        Array of encryption types
 * @param [in] etype_list_length Length of @a etype_list
 */
    /* *
 * Set address restrictions in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] addresses        Null-terminated array of addresses
 */
    /* *
 * Set preauthentication types in initial credential options.
 *
 * @param [in] opt                 Options structure
 * @param [in] preauth_list        Array of preauthentication types
 * @param [in] preauth_list_length Length of @a preauth_list
 *
 * This function can be used to perform optimistic preauthentication when
 * getting initial credentials, in combination with
 * krb5_get_init_creds_opt_set_salt() and krb5_get_init_creds_opt_set_pa().
 */
    /* *
 * Set salt for optimistic preauthentication in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] salt             Salt data
 *
 * When getting initial credentials with a password, a salt string it used to
 * convert the password to a key.  Normally this salt is obtained from the
 * first KDC reply, but when performing optimistic preauthentication, the
 * client may need to supply the salt string with this function.
 */
    /* *
 * Set or unset change-password-prompt flag in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] prompt           Whether to prompt to change password
 *
 * This flag is on by default.  It controls whether
 * krb5_get_init_creds_password() will react to an expired-password error by
 * prompting for a new password and attempting to change the old one.
 */
    /* * Generic preauth option attribute/value pairs */
    /* *
 * Supply options for preauthentication in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] attr             Preauthentication option name
 * @param [in] value            Preauthentication option value
 *
 * This function allows the caller to supply options for preauthentication.
 * The values of @a attr and @a value are supplied to each preauthentication
 * module available within @a context.
 */
    /* *
 * Set location of FAST armor ccache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] fast_ccache_name Credential cache name
 *
 * Sets the location of a credential cache containing an armor ticket to
 * protect an initial credential exchange using the FAST protocol extension.
 *
 * In version 1.7, setting an armor ccache requires that FAST be used for the
 * exchange.  In version 1.8 or later, setting the armor ccache causes FAST to
 * be used if the KDC supports it; krb5_get_init_creds_opt_set_fast_flags()
 * must be used to require that FAST be used.
 */
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
    /* *
 * Set an input credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an input credential cache is set, then the krb5_get_init_creds family of
 * APIs will read settings from it.  Setting an input ccache is desirable when
 * the application wishes to perform authentication in the same way (using the
 * same preauthentication mechanisms, and making the same non-security-
 * sensitive choices) as the previous authentication attempt, which stored
 * information in the passed-in ccache.
 *
 * @version New in 1.11
 */
    /* *
 * Set an output credential cache in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] ccache           Credential cache handle
 *
 * If an output credential cache is set, then the krb5_get_init_creds family of
 * APIs will write credentials to it.  Setting an output ccache is desirable
 * both because it simplifies calling code and because it permits the
 * krb5_get_init_creds APIs to write out configuration information about the
 * realm to the ccache.
 */
    /* *
 * @brief Ask the KDC to include or not include a PAC in the ticket
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] req_pac          Whether to request a PAC or not
 *
 * If this option is set, the AS request will include a PAC-REQUEST pa-data
 * item explicitly asking the KDC to either include or not include a privilege
 * attribute certificate in the ticket authorization data.  By default, no
 * request is made; typically the KDC will default to including a PAC if it
 * supports them.
 *
 * @version New in 1.15
 */
    /* *
 * Set FAST flags in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options
 * @param [in] flags            FAST flags
 *
 * The following flag values are valid:
 * @li #KRB5_FAST_REQUIRED - Require FAST to be used
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
    /* *
 * Retrieve FAST flags from initial credential options.
 *
 * @param [in]  context         Library context
 * @param [in]  opt             Options
 * @param [out] out_flags       FAST flags
 *
 * @retval
 * 0 - Success; Kerberos errors otherwise.
 */
    /* Fast flags*/
    /* *< Require KDC to support FAST*/
    /* *
 * Set an expiration callback in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] cb               Callback function
 * @param [in] data             Callback argument
 *
 * Set a callback to receive password and account expiration times.
 *
 * This option only applies to krb5_get_init_creds_password().  @a cb will be
 * invoked if and only if credentials are successfully acquired.  The callback
 * will receive the @a context from the krb5_get_init_creds_password() call and
 * the @a data argument supplied with this API.  The remaining arguments should
 * be interpreted as follows:
 *
 * If @a is_last_req is true, then the KDC reply contained last-req entries
 * which unambiguously indicated the password expiration, account expiration,
 * or both.  (If either value was not present, the corresponding argument will
 * be 0.)  Furthermore, a non-zero @a password_expiration should be taken as a
 * suggestion from the KDC that a warning be displayed.
 *
 * If @a is_last_req is false, then @a account_expiration will be 0 and @a
 * password_expiration will contain the expiration time of either the password
 * or account, or 0 if no expiration time was indicated in the KDC reply.  The
 * callback should independently decide whether to display a password
 * expiration warning.
 *
 * Note that @a cb may be invoked even if credentials are being acquired for
 * the kadmin/changepw service in order to change the password.  It is the
 * caller's responsibility to avoid displaying a password expiry warning in
 * this case.
 *
 * @warning Setting an expire callback with this API will cause
 * krb5_get_init_creds_password() not to send password expiry warnings to the
 * prompter, as it ordinarily may.
 *
 * @version New in 1.9
 */
    /* *
 * Set the responder function in initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure
 * @param [in] responder        Responder function
 * @param [in] data             Responder data argument
 *
 * @version New in 1.11
 */
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
    /* *
 * Retrieve enctype, salt and s2kparams from KDC
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal whose information is requested
 * @param [in]  opt             Initial credential options
 * @param [out] enctype_out     The enctype chosen by KDC
 * @param [out] salt_out        Salt returned from KDC
 * @param [out] s2kparams_out   String-to-key parameters returned from KDC
 *
 * Send an initial ticket request for @a principal and extract the encryption
 * type, salt type, and string-to-key parameters from the KDC response.  If the
 * KDC provides no etype-info, set @a enctype_out to @c ENCTYPE_NULL and set @a
 * salt_out and @a s2kparams_out to empty.  If the KDC etype-info provides no
 * salt, compute the default salt and place it in @a salt_out.  If the KDC
 * etype-info provides no string-to-key parameters, set @a s2kparams_out to
 * empty.
 *
 * @a opt may be used to specify options which affect the initial request, such
 * as request encryption types or a FAST armor cache (see
 * krb5_get_init_creds_opt_set_etype_list() and
 * krb5_get_init_creds_opt_set_fast_ccache_name()).
 *
 * Use krb5_free_data_contents() to free @a salt_out and @a s2kparams_out when
 * they are no longer needed.
 *
 * @version New in 1.17
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *< More responses needed */
    /* *
 * Free an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 */
    /* *
 * Acquire credentials using an initial credentials context.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_init_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_init_creds_get_creds().
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] creds           Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_init_creds_get() or
 * krb5_init_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the last error from KDC from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] error           Error from KDC, or NULL if none was received
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal to get initial creds for
 * @param [in]  prompter        Prompter callback
 * @param [in]  data            Prompter callback argument
 * @param [in]  start_time      Time when credentials become valid (0 for now)
 * @param [in]  options         Options structure (NULL for default)
 * @param [out] ctx             New initial credentials context
 *
 * This function creates a new context for acquiring initial credentials.  Use
 * krb5_init_creds_free() to free @a ctx when it is no longer needed.
 *
 * Any subsequent calls to krb5_init_creds_step(), krb5_init_creds_get(), or
 * krb5_init_creds_free() for this initial credentials context must use the
 * same @a context argument as the one passed to this function.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a keytab to use for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] keytab           Key table handle
 *
 * This function supplies a keytab containing the client key for an initial
 * credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the next KDC request for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [in]  in              KDC response (empty on the first call)
 * @param [out] out             Next KDC request
 * @param [out] realm           Realm for next KDC request
 * @param [out] flags           Output flags
 *
 * This function constructs the next KDC request in an initial credential
 * exchange, allowing the caller to control the transport of KDC requests and
 * replies.  On the first call, @a in should be set to an empty buffer; on
 * subsequent calls, it should be set to the KDC's reply to the previous
 * request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in
 * @a out.  If no more requests are needed, @a flags will not contain
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the initial credential exchange has failed.
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] password         Password
 *
 * This function supplies a password to be used to construct the client key for
 * an initial credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a service principal for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] service          Service principal string
 *
 * This function supplies a service principal string to acquire initial
 * credentials for instead of the default krbtgt service.  @a service is parsed
 * as a principal name; any realm part is ignored.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] times           Ticket times for acquired credentials
 *
 * The initial credentials context must have completed obtaining credentials
 * via either krb5_init_creds_get() or krb5_init_creds_step().
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context to get credentials from a KDC's Ticket Granting Service.
 *
 * @param[in]  context          Library context
 * @param[in]  ccache           Credential cache handle
 * @param[in]  creds            Input credentials
 * @param[in]  options          @ref KRB5_GC options for this request.
 * @param[out] ctx              New TGS request context
 *
 * This function prepares to obtain credentials matching @a creds, either by
 * retrieving them from @a ccache or by making requests to ticket-granting
 * services beginning with a ticket-granting ticket for the client principal's
 * realm.
 *
 * The resulting TGS acquisition context can be used asynchronously with
 * krb5_tkt_creds_step() or synchronously with krb5_tkt_creds_get().  See also
 * krb5_get_credentials() for synchronous use.
 *
 * Use krb5_tkt_creds_free() to free @a ctx when it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Synchronously obtain credentials using a TGS request context.
 *
 * @param[in] context           Library context
 * @param[in] ctx               TGS request context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_tkt_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_tkt_creds_get_creds().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] creds            Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_tkt_creds_get() or
 * krb5_tkt_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
    /* *< More responses needed */
    /* *
 * Get the next KDC request in a TGS exchange.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[in]  in               KDC response (empty on the first call)
 * @param[out] out              Next KDC request
 * @param[out] realm            Realm for next KDC request
 * @param[out] flags            Output flags
 *
 * This function constructs the next KDC request for a TGS exchange, allowing
 * the caller to control the transport of KDC requests and replies.  On the
 * first call, @a in should be set to an empty buffer; on subsequent calls, it
 * should be set to the KDC's reply to the previous request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in @a
 * out.  If no more requests are needed, @a flags will not contain
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the TGS exchange has failed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] times            Ticket times for acquired credentials
 *
 * The TGS request context must have completed obtaining credentials via either
 * krb5_tkt_creds_get() or krb5_tkt_creds_step().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get initial credentials using a key table.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  arg_keytab      Key table handle
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using a
 * client key stored in @a arg_keytab.  If @a in_tkt_service is specified, it
 * is parsed as a principal name (with the realm ignored) and used as the
 * service principal for the request; otherwise the ticket-granting service is
 * used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *< boolean */
    /* *
 * Initialize a credential verification options structure.
 *
 * @param [in] k5_vic_options   Verification options structure
 */
    /* *
 * Set whether credential verification is required.
 *
 * @param [in] k5_vic_options   Verification options structure
 * @param [in] ap_req_nofail    Whether to require successful verification
 *
 * This function determines how krb5_verify_init_creds() behaves if no keytab
 * information is available.  If @a ap_req_nofail is @c FALSE, verification
 * will be skipped in this case and krb5_verify_init_creds() will return
 * successfully.  If @a ap_req_nofail is @c TRUE, krb5_verify_init_creds() will
 * not return successfully unless verification can be performed.
 *
 * If this function is not used, the behavior of krb5_verify_init_creds() is
 * determined through configuration.
 */
    /* *
 * Verify initial credentials against a keytab.
 *
 * @param [in] context          Library context
 * @param [in] creds            Initial credentials to be verified
 * @param [in] server           Server principal (or NULL)
 * @param [in] keytab           Key table (NULL to use default keytab)
 * @param [in] ccache           Credential cache for fetched creds (or NULL)
 * @param [in] options          Verification options (NULL for default options)
 *
 * This function attempts to verify that @a creds were obtained from a KDC with
 * knowledge of a key in @a keytab, or the default keytab if @a keytab is NULL.
 * If @a server is provided, the highest-kvno key entry for that principal name
 * is used to verify the credentials; otherwise, all unique "host" service
 * principals in the keytab are tried.
 *
 * If the specified keytab does not exist, or is empty, or cannot be read, or
 * does not contain an entry for @a server, then credential verification may be
 * skipped unless configuration demands that it succeed.  The caller can
 * control this behavior by providing a verification options structure; see
 * krb5_verify_init_creds_opt_init() and
 * krb5_verify_init_creds_opt_set_ap_req_nofail().
 *
 * If @a ccache is NULL, any additional credentials fetched during the
 * verification process will be destroyed.  If @a ccache points to NULL, a
 * memory ccache will be created for the additional credentials and returned in
 * @a ccache.  If @a ccache points to a valid credential cache handle, the
 * additional credentials will be stored in that cache.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get validated credentials from the KDC.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Validated credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a validated credential using a postdated credential from
 * @a ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential;
 * otherwise, the ticket-granting service is used.
 *
 * If successful, the validated credential is placed in @a creds.
 *
 * @sa krb5_get_renewed_creds()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_NO_2ND_TKT Request missing second ticket
 * @retval
 * KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 * KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 * KRB5_KDCREP_MODIFIED KDC reply did not match expectations
 * @retval
 * KRB5_KDCREP_SKEW Clock skew too great in KDC reply
 * @return
 * Kerberos error codes
 */
    /* *
 * Get renewed credential from KDC using an existing credential.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Renewed credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a renewed credential using an existing one from @a
 * ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential; otherwise,
 * the ticket-granting service is used.
 *
 * If successful, the renewed credential is placed in @a creds.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Decode an ASN.1-formatted ticket.
 *
 * @param [in]  code            ASN.1-formatted ticket
 * @param [out] rep             Decoded ticket information
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a string value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       String value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_boolean()
 */
    /* *
 * Retrieve a boolean value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       Boolean value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_string()
 */
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
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
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    use super::k5_int_h::_krb5_context;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint8_t;
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
        /* *
 * Generate pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [out] data            Random data
 *
 * Fills in @a data with bytes from the PRNG used by krb5 crypto operations.
 * The caller must preinitialize @a data and allocate the desired amount of
 * space.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn krb5_c_random_make_octets(context: krb5_context,
                                         data: *mut krb5_data)
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
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:32"]
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
                        krb5_data};
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
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:32"]
pub mod k5_plugin_h {
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
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
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:32"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/spnego/gssapiP_negoex.h:32"]
pub mod gssapiP_negoex_h {
    /* NEGO_MESSAGE */
    /* NEGO_MESSAGE */
    /* EXCHANGE_MESSAGE */
    /* EXCHANGE_MESSAGE */
    /* EXCHANGE_MESSAGE */
    /* EXCHANGE_MESSAGE */
    /* VERIFY_MESSAGE */
    /* ALERT */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:8"]
    pub struct negoex_auth_mech {
        pub links: C2RustUnnamed,
        pub oid: gss_OID,
        pub scheme: auth_scheme,
        pub mech_context: gss_ctx_id_t,
        pub metadata: gss_buffer_desc,
        pub key: krb5_keyblock,
        pub verify_key: krb5_keyblock,
        pub complete: libc::c_int,
        pub sent_checksum: libc::c_int,
        pub verified_checksum: libc::c_int,
    }
    #[c2rust::src_loc = "56:1"]
    pub type auth_scheme = [uint8_t; 16];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:5"]
    pub struct C2RustUnnamed {
        pub tqe_next: *mut negoex_auth_mech,
        pub tqe_prev: *mut *mut negoex_auth_mech,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2011-2018 PADL Software Pty Ltd.
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
 * { iso(1) identified-organization(3) dod(6) internet(1) private(4)
 *   enterprise(1) microsoft (311) security(2) mechanisms(2) negoex(30) }
 */
    #[c2rust::src_loc = "57:1"]
    pub type conversation_id = [uint8_t; 16];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:8"]
    pub struct exchange_message {
        pub scheme: auth_scheme,
        pub token: gss_buffer_desc,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct verify_message {
        pub scheme: auth_scheme,
        pub cksum_type: uint32_t,
        pub cksum: *const uint8_t,
        pub cksum_len: size_t,
        pub offset_in_token: size_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:8"]
    pub struct nego_message {
        pub random: [uint8_t; 32],
        pub schemes: *const uint8_t,
        pub nschemes: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "105:8"]
    pub struct negoex_message {
        pub type_0: uint32_t,
        pub u: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "107:5"]
    pub union C2RustUnnamed_6 {
        pub n: nego_message,
        pub e: exchange_message,
        pub v: verify_message,
        pub a: alert_message,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:8"]
    pub struct alert_message {
        pub scheme: auth_scheme,
        pub verify_no_key: libc::c_int,
    }
    #[c2rust::src_loc = "70:1"]
    pub type message_type = libc::c_uint;
    #[c2rust::src_loc = "78:5"]
    pub const ALERT: message_type = 7;
    #[c2rust::src_loc = "77:5"]
    pub const VERIFY: message_type = 6;
    #[c2rust::src_loc = "76:5"]
    pub const AP_REQUEST: message_type = 5;
    #[c2rust::src_loc = "75:5"]
    pub const CHALLENGE: message_type = 4;
    #[c2rust::src_loc = "74:5"]
    pub const ACCEPTOR_META_DATA: message_type = 3;
    #[c2rust::src_loc = "73:5"]
    pub const INITIATOR_META_DATA: message_type = 2;
    #[c2rust::src_loc = "72:5"]
    pub const ACCEPTOR_NEGO: message_type = 1;
    #[c2rust::src_loc = "71:5"]
    pub const INITIATOR_NEGO: message_type = 0;
    use super::gssapi_h::{gss_OID, gss_ctx_id_t, gss_buffer_desc};
    use super::krb5_h::krb5_keyblock;
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint16_t};
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint32_t, __uint16_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:32"]
pub mod gssapi_h {
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
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
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
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
        /* OM_STRING */
        /*
 * For now, define a QOP-type as an OM_uint32 (pending resolution of ongoing
 * discussions).
 */
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
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x02"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) machine_uid_name(2)}.
 * The constant GSS_C_NT_MACHINE_UID_NAME should be
 * initialized to point to that gss_OID_desc.
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x03"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) string_uid_name(3)}.
 * The constant GSS_C_NT_STRING_UID_NAME should be
 * initialized to point to that gss_OID_desc.
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x02"},
 * corresponding to an object-identifier value of
 * {iso(1) org(3) dod(6) internet(1) security(5)
 * nametypes(6) gss-host-based-services(2)).  The constant
 * GSS_C_NT_HOSTBASED_SERVICE_X should be initialized to point
 * to that gss_OID_desc.  This is a deprecated OID value, and
 * implementations wishing to support hostbased-service names
 * should instead use the GSS_C_NT_HOSTBASED_SERVICE OID,
 * defined below, to identify such names;
 * GSS_C_NT_HOSTBASED_SERVICE_X should be accepted a synonym
 * for GSS_C_NT_HOSTBASED_SERVICE when presented as an input
 * parameter, but should not be emitted by GSS-API
 * implementations
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12"
 *              "\x01\x02\x01\x04"}, corresponding to an
 * object-identifier value of {iso(1) member-body(2)
 * Unites States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The constant
 * GSS_C_NT_HOSTBASED_SERVICE should be initialized
 * to point to that gss_OID_desc.
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\01\x05\x06\x03"},
 * corresponding to an object identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 3(gss-anonymous-name)}.  The constant
 * and GSS_C_NT_ANONYMOUS should be initialized to point
 * to that gss_OID_desc.
 */
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {6, (void *)"\x2b\x06\x01\x05\x06\x04"},
 * corresponding to an object-identifier value of
 * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
 * 6(nametypes), 4(gss-api-exported-name)}.  The constant
 * GSS_C_NT_EXPORT_NAME should be initialized to point
 * to that gss_OID_desc.
 */
        /* Function Prototypes */
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
        /* mech_type (used to be const) */
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
        /* acceptor_cred_handle */
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
        /* New for V2 */
        /* minor_status */
        /* context_handle */
        /* qop_req */
        /* message_buffer */
        /* message_token */
        /* New for V2 */
        /* minor_status */
        /* context_handle */
        /* message_buffer */
        /* message_token */
        /* qop_state */
        /* New for V2 */
        /* minor_status */
        /* context_handle */
        /* conf_req_flag */
        /* qop_req */
        /* input_message_buffer */
        /* conf_state */
        /* output_message_buffer */
        /* New for V2 */
        /* minor_status */
        /* context_handle */
        /* input_message_buffer */
        /* output_message_buffer */
        /* conf_state */
        /* qop_state */
        /* minor_status */
        /* status_value */
        /* status_type */
        /* mech_type (used to be const) */
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
        /* input_name_type(used to be const) */
        /* output_name */
        /* minor_status */
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:32"]
pub mod k5_buf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[inline]
    #[c2rust::src_loc = "120:1"]
    pub unsafe extern "C" fn k5_buf_add_uint16_le(mut buf: *mut k5buf,
                                                  mut val: uint16_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 2 as libc::c_int as size_t);
        if !p.is_null() { store_16_le(val as libc::c_uint, p); };
    }
    #[inline]
    #[c2rust::src_loc = "138:1"]
    pub unsafe extern "C" fn k5_buf_add_uint32_le(mut buf: *mut k5buf,
                                                  mut val: uint32_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 4 as libc::c_int as size_t);
        if !p.is_null() { store_32_le(val, p); };
    }
    #[inline]
    #[c2rust::src_loc = "156:1"]
    pub unsafe extern "C" fn k5_buf_add_uint64_le(mut buf: *mut k5buf,
                                                  mut val: uint64_t) {
        let mut p: *mut libc::c_void =
            k5_buf_get_space(buf, 8 as libc::c_int as size_t);
        if !p.is_null() { store_64_le(val, p); };
    }
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t};
    use super::k5_platform_h::{store_16_le, store_32_le, store_64_le};
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a C string to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const libc::c_char);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
        /* Extend the length of buf by len and return a pointer to the reserved space,
 * to be filled in by the caller.  Return NULL on error. */
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn k5_buf_get_space(buf: *mut k5buf, len: size_t)
         -> *mut libc::c_void;
        /* Truncate BUF.  LEN must be between 0 and the existing buffer
 * length, or an assertion failure will result. */
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_buf_truncate(buf: *mut k5buf, len: size_t);
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:32"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "644:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed_1 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "672:5"]
    pub struct C2RustUnnamed_2 {
        pub i: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "691:12"]
    pub struct C2RustUnnamed_3 {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "703:12"]
    pub struct C2RustUnnamed_4 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "715:12"]
    pub struct C2RustUnnamed_5 {
        pub i: uint64_t,
    }
    #[inline]
    #[c2rust::src_loc = "639:1"]
    pub unsafe extern "C" fn store_16_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = val as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_1)).i = val;
    }
    #[inline]
    #[c2rust::src_loc = "667:1"]
    pub unsafe extern "C" fn store_64_le(mut val: uint64_t,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_2)).i = val;
    }
    #[inline]
    #[c2rust::src_loc = "686:1"]
    pub unsafe extern "C" fn load_16_le(mut cvp: *const libc::c_void)
     -> libc::c_ushort {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_3)).i;
    }
    #[inline]
    #[c2rust::src_loc = "698:1"]
    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_4)).i;
    }
    #[inline]
    #[c2rust::src_loc = "710:1"]
    pub unsafe extern "C" fn load_64_le(mut cvp: *const libc::c_void)
     -> uint64_t {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_5)).i;
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t};
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-input.h:34"]
pub mod k5_input_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-input.h - k5input helper functions */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
 * The k5input module defines helpers for safely consuming a fixed-sized block
 * of memory.  If an overrun or allocation failure occurs at any step,
 * subsequent functions will return default values until the error is detected
 * by looking at the status field.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct k5input {
        pub ptr: *const libc::c_uchar,
        pub len: size_t,
        pub status: int32_t,
    }
    #[inline]
    #[c2rust::src_loc = "51:1"]
    pub unsafe extern "C" fn k5_input_init(mut in_0: *mut k5input,
                                           mut ptr: *const libc::c_void,
                                           mut len: size_t) {
        (*in_0).ptr = ptr as *const libc::c_uchar;
        (*in_0).len = len;
        (*in_0).status = 0 as libc::c_int;
    }
    /* Only set the status value of in if it hasn't already been set, so status
 * reflects the first thing to go wrong. */
    #[inline]
    #[c2rust::src_loc = "61:1"]
    pub unsafe extern "C" fn k5_input_set_status(mut in_0: *mut k5input,
                                                 mut status: int32_t) {
        if (*in_0).status == 0 { (*in_0).status = status };
    }
    #[inline]
    #[c2rust::src_loc = "68:1"]
    pub unsafe extern "C" fn k5_input_get_bytes(mut in_0: *mut k5input,
                                                mut len: size_t)
     -> *const libc::c_uchar {
        if (*in_0).len < len { k5_input_set_status(in_0, 22 as libc::c_int); }
        if (*in_0).status != 0 { return 0 as *const libc::c_uchar }
        (*in_0).len =
            ((*in_0).len as libc::c_ulong).wrapping_sub(len) as size_t as
                size_t;
        (*in_0).ptr = (*in_0).ptr.offset(len as isize);
        return (*in_0).ptr.offset(-(len as isize));
    }
    #[inline]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn k5_input_get_uint16_le(mut in_0: *mut k5input)
     -> uint16_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 2 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int
               } else {
                   load_16_le(ptr as *const libc::c_void) as libc::c_int
               } as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "115:1"]
    pub unsafe extern "C" fn k5_input_get_uint32_le(mut in_0: *mut k5input)
     -> uint32_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 4 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int as libc::c_uint
               } else { load_32_le(ptr as *const libc::c_void) };
    }
    #[inline]
    #[c2rust::src_loc = "136:1"]
    pub unsafe extern "C" fn k5_input_get_uint64_le(mut in_0: *mut k5input)
     -> uint64_t {
        let mut ptr: *const libc::c_uchar =
            k5_input_get_bytes(in_0, 8 as libc::c_int as size_t);
        return if ptr.is_null() {
                   0 as libc::c_int as libc::c_ulong
               } else { load_64_le(ptr as *const libc::c_void) };
    }
    use super::stddef_h::size_t;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t};
    use super::k5_platform_h::{load_16_le, load_32_le, load_64_le};
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/assert.h:32"]
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
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:32"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:33"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID, gss_OID_desc};
    extern "C" {
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn generic_gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn generic_gss_copy_oid(_: *mut OM_uint32, _: *const gss_OID_desc,
                                    _: *mut gss_OID) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
pub use self::gssapiP_spnego_h::{spnego_ctx_st, negoex_mech_list,
                                 spnego_gss_ctx_id_t};
pub use self::krb5_h::{krb5_context, krb5_boolean, krb5_post_recv_fn,
                       krb5_data, _krb5_data, krb5_magic, krb5_error_code,
                       krb5_int32, krb5_pre_send_fn, krb5_trace_callback,
                       krb5_trace_info, _krb5_trace_info, krb5_prompt_type,
                       krb5_flags, krb5_deltat, krb5_enctype, krb5_keyblock,
                       _krb5_keyblock, krb5_octet, _profile_t,
                       krb5_c_random_make_octets, krb5_init_context,
                       krb5_free_context, krb5_free_keyblock_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, make_data, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::stdint_intn_h::int32_t;
pub use self::types_h::{__int32_t, __uint8_t, __uint32_t, __uint16_t,
                        __uint64_t};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::gssapiP_negoex_h::{negoex_auth_mech, auth_scheme, C2RustUnnamed,
                                 conversation_id, exchange_message,
                                 verify_message, nego_message, negoex_message,
                                 C2RustUnnamed_6, alert_message, message_type,
                                 ALERT, VERIFY, AP_REQUEST, CHALLENGE,
                                 ACCEPTOR_META_DATA, INITIATOR_META_DATA,
                                 ACCEPTOR_NEGO, INITIATOR_NEGO};
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint16_t, uint64_t};
pub use self::gssapi_h::{gss_buffer_desc, gss_buffer_desc_struct,
                         gss_ctx_id_t, gss_OID, gss_OID_desc_struct,
                         OM_uint32, gss_uint32, gss_cred_id_t, gss_name_t,
                         gss_OID_set, gss_OID_set_desc_struct, gss_OID_desc,
                         gss_buffer_t, gss_const_buffer_t, gss_const_OID,
                         gss_ctx_id_struct, gss_cred_id_struct,
                         gss_name_struct, gss_release_buffer,
                         gss_delete_sec_context};
pub use self::stddef_h::size_t;
pub use self::k5_buf_h::{k5buf, k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5_buf_add_uint16_le,
                         k5_buf_add_uint32_le, k5_buf_add_uint64_le,
                         k5_buf_init_dynamic, k5_buf_add, k5_buf_add_len,
                         k5_buf_add_fmt, k5_buf_get_space, k5_buf_truncate,
                         k5_buf_free};
pub use self::k5_platform_h::{C2RustUnnamed_0, C2RustUnnamed_1,
                              C2RustUnnamed_2, C2RustUnnamed_3,
                              C2RustUnnamed_4, C2RustUnnamed_5, store_16_le,
                              store_32_le, store_64_le, load_16_le,
                              load_32_le, load_64_le};
pub use self::k5_input_h::{k5input, k5_input_init, k5_input_set_status,
                           k5_input_get_bytes, k5_input_get_uint16_le,
                           k5_input_get_uint32_le, k5_input_get_uint64_le};
use self::assert_h::__assert_fail;
use self::string_h::{memcpy, memcmp};
use self::stdlib_h::{calloc, realloc, free, abort};
use self::k5_trace_h::krb5int_trace;
use self::gssapiP_generic_h::{generic_gss_release_oid, generic_gss_copy_oid};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn negoex_random(mut minor: *mut OM_uint32,
                                       mut ctx: spnego_gss_ctx_id_t,
                                       mut data: *mut uint8_t,
                                       mut length: size_t) -> OM_uint32 {
    let mut d: krb5_data =
        make_data(data as *mut libc::c_void, length as libc::c_uint);
    *minor = krb5_c_random_make_octets((*ctx).kctx, &mut d) as OM_uint32;
    return if *minor != 0 {
               ((13 as libc::c_ulong as OM_uint32)) << 16 as libc::c_int
           } else { 0 as libc::c_int as libc::c_uint };
}
/*
 * SPNEGO functions expect to find the active mech context in ctx->ctx_handle,
 * but the metadata exchange APIs force us to have one mech context per mech
 * entry.  To address this mismatch, move the active mech context (if we have
 * one) to ctx->ctx_handle at the end of NegoEx processing.
 */
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn negoex_prep_context_for_spnego(mut ctx:
                                                            spnego_gss_ctx_id_t) {
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    mech = (*ctx).negoex_mechs.tqh_first;
    if mech.is_null() || (*mech).mech_context.is_null() { return }
    if (*ctx).ctx_handle.is_null() {
    } else {
        __assert_fail(b"ctx->ctx_handle == GSS_C_NO_CONTEXT\x00" as *const u8
                          as *const libc::c_char,
                      b"negoex_util.c\x00" as *const u8 as
                          *const libc::c_char,
                      64 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"void negoex_prep_context_for_spnego(spnego_gss_ctx_id_t)\x00")).as_ptr());
    }
    (*ctx).ctx_handle = (*mech).mech_context;
    (*mech).mech_context = 0 as gss_ctx_id_t;
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn negoex_prep_context_for_negoex(mut minor:
                                                            *mut OM_uint32,
                                                        mut ctx:
                                                            spnego_gss_ctx_id_t)
 -> OM_uint32 {
    let mut ret: krb5_error_code = 0;
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    if !(*ctx).kctx.is_null() {
        /* The context is already initialized for NegoEx.  Undo what
         * negoex_prep_for_spnego() did, if applicable. */
        if !(*ctx).ctx_handle.is_null() {
            mech = (*ctx).negoex_mechs.tqh_first;
            if !mech.is_null() && (*mech).mech_context.is_null() {
            } else {
                __assert_fail(b"mech != NULL && mech->mech_context == GSS_C_NO_CONTEXT\x00"
                                  as *const u8 as *const libc::c_char,
                              b"negoex_util.c\x00" as *const u8 as
                                  *const libc::c_char,
                              80 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 75],
                                                        &[libc::c_char; 75]>(b"OM_uint32 negoex_prep_context_for_negoex(OM_uint32 *, spnego_gss_ctx_id_t)\x00")).as_ptr());
            }
            (*mech).mech_context = (*ctx).ctx_handle;
            (*ctx).ctx_handle = 0 as gss_ctx_id_t
        }
        return 0 as libc::c_int as OM_uint32
    }
    /* Initialize the NegoEX context fields.  (negoex_mechs is already set up
     * by SPNEGO.) */
    ret = krb5_init_context(&mut (*ctx).kctx);
    if ret != 0 {
        *minor = ret as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    k5_buf_init_dynamic(&mut (*ctx).negoex_transcript);
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn release_all_mechs(mut ctx: spnego_gss_ctx_id_t) {
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    let mut next: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() &&
              { next = (*mech).links.tqe_next; (1 as libc::c_int) != 0 } {
        release_auth_mech(mech);
        mech = next
    }
    (*ctx).negoex_mechs.tqh_first = 0 as *mut negoex_auth_mech;
    (*ctx).negoex_mechs.tqh_last = &mut (*ctx).negoex_mechs.tqh_first;
}
#[no_mangle]
#[c2rust::src_loc = "110:1"]
pub unsafe extern "C" fn negoex_release_context(mut ctx:
                                                    spnego_gss_ctx_id_t) {
    k5_buf_free(&mut (*ctx).negoex_transcript);
    release_all_mechs(ctx);
    krb5_free_context((*ctx).kctx);
    (*ctx).kctx = 0 as krb5_context;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn typestr(mut type_0: message_type)
 -> *const libc::c_char {
    if type_0 as libc::c_uint == INITIATOR_NEGO as libc::c_int as libc::c_uint
       {
        return b"INITIATOR_NEGO\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  ACCEPTOR_NEGO as libc::c_int as libc::c_uint {
        return b"ACCEPTOR_NEGO\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  INITIATOR_META_DATA as libc::c_int as libc::c_uint {
        return b"INITIATOR_META_DATA\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  ACCEPTOR_META_DATA as libc::c_int as libc::c_uint {
        return b"ACCEPTOR_META_DATA\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  CHALLENGE as libc::c_int as libc::c_uint {
        return b"CHALLENGE\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint ==
                  AP_REQUEST as libc::c_int as libc::c_uint {
        return b"AP_REQUEST\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint == VERIFY as libc::c_int as libc::c_uint
     {
        return b"VERIFY\x00" as *const u8 as *const libc::c_char
    } else if type_0 as libc::c_uint == ALERT as libc::c_int as libc::c_uint {
        return b"ALERT\x00" as *const u8 as *const libc::c_char
    } else { return b"UNKNOWN\x00" as *const u8 as *const libc::c_char };
}
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn add_guid(mut buf: *mut k5buf, mut guid: *const uint8_t) {
    let mut data1: uint32_t = load_32_le(guid as *const libc::c_void);
    let mut data2: uint16_t =
        load_16_le(guid.offset(4 as libc::c_int as isize) as
                       *const libc::c_void);
    let mut data3: uint16_t =
        load_16_le(guid.offset(6 as libc::c_int as isize) as
                       *const libc::c_void);
    k5_buf_add_fmt(buf,
                   b"%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x\x00" as
                       *const u8 as *const libc::c_char, data1,
                   data2 as libc::c_int, data3 as libc::c_int,
                   *guid.offset(8 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(9 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(10 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(11 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(12 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(13 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(14 as libc::c_int as isize) as libc::c_int,
                   *guid.offset(15 as libc::c_int as isize) as libc::c_int);
}
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn guid_to_string(mut guid: *const uint8_t)
 -> *mut libc::c_char {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    k5_buf_init_dynamic(&mut buf);
    add_guid(&mut buf, guid);
    return buf.data as *mut libc::c_char;
}
/* Check that the described vector lies within the message, and return a
 * pointer to its first element. */
#[inline]
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn vector_base(mut offset: size_t, mut count: size_t,
                                 mut width: size_t,
                                 mut msg_base: *const uint8_t,
                                 mut msg_len: size_t) -> *const uint8_t {
    if offset > msg_len ||
           count > msg_len.wrapping_sub(offset).wrapping_div(width) {
        return 0 as *const uint8_t
    }
    return msg_base.offset(offset as isize);
}
/* Trace a received message.  Call after the context sequence number is
 * incremented. */
#[c2rust::src_loc = "176:1"]
unsafe extern "C" fn trace_received_message(mut ctx: spnego_gss_ctx_id_t,
                                            mut msg: *const negoex_message) {
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut i: uint16_t = 0;
    let mut info: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*msg).type_0 == INITIATOR_NEGO as libc::c_int as libc::c_uint ||
           (*msg).type_0 == ACCEPTOR_NEGO as libc::c_int as libc::c_uint {
        k5_buf_init_dynamic(&mut buf);
        i = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < (*msg).u.n.nschemes as libc::c_int {
            add_guid(&mut buf,
                     (*msg).u.n.schemes.offset((i as libc::c_int *
                                                    16 as libc::c_int) as
                                                   isize));
            if (i as libc::c_int + 1 as libc::c_int) <
                   (*msg).u.n.nschemes as libc::c_int {
                k5_buf_add(&mut buf,
                           b" \x00" as *const u8 as *const libc::c_char);
            }
            i = i.wrapping_add(1)
        }
        info = buf.data as *mut libc::c_char
    } else if (*msg).type_0 ==
                  INITIATOR_META_DATA as libc::c_int as libc::c_uint ||
                  (*msg).type_0 ==
                      ACCEPTOR_META_DATA as libc::c_int as libc::c_uint ||
                  (*msg).type_0 == CHALLENGE as libc::c_int as libc::c_uint ||
                  (*msg).type_0 == AP_REQUEST as libc::c_int as libc::c_uint {
        info = guid_to_string((*msg).u.e.scheme.as_ptr())
    } else if (*msg).type_0 == VERIFY as libc::c_int as libc::c_uint {
        info = guid_to_string((*msg).u.v.scheme.as_ptr())
    } else if (*msg).type_0 == ALERT as libc::c_int as libc::c_uint {
        info = guid_to_string((*msg).u.a.scheme.as_ptr())
    }
    if info.is_null() { return }
    if (*(*ctx).kctx).trace_callback.is_some() {
        krb5int_trace((*ctx).kctx,
                      b"NegoEx received [{int}]{str}: {str}\x00" as *const u8
                          as *const libc::c_char,
                      (*ctx).negoex_seqnum as libc::c_int - 1 as libc::c_int,
                      typestr((*msg).type_0 as message_type), info);
    }
    free(info as *mut libc::c_void);
}
/* Trace an outgoing message with a GUID info string.  Call after the context
 * sequence number is incremented. */
#[c2rust::src_loc = "212:1"]
unsafe extern "C" fn trace_outgoing_message(mut ctx: spnego_gss_ctx_id_t,
                                            mut type_0: message_type,
                                            mut guid: *const uint8_t) {
    let mut info: *mut libc::c_char =
        guid_to_string(guid); /* skip over ErrorCode */
    if info.is_null() { return }
    if (*(*ctx).kctx).trace_callback.is_some() {
        krb5int_trace((*ctx).kctx,
                      b"NegoEx sending [{int}]{str}: {str}\x00" as *const u8
                          as *const libc::c_char,
                      (*ctx).negoex_seqnum as libc::c_int - 1 as libc::c_int,
                      typestr(type_0), info);
    }
    free(info as *mut libc::c_void);
}
#[c2rust::src_loc = "225:1"]
unsafe extern "C" fn parse_nego_message(mut minor: *mut OM_uint32,
                                        mut in_0: *mut k5input,
                                        mut msg_base: *const uint8_t,
                                        mut msg_len: size_t,
                                        mut msg: *mut nego_message)
 -> OM_uint32 {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut protocol_version: uint64_t = 0;
    let mut extension_type: uint32_t = 0;
    let mut offset: size_t = 0;
    let mut count: size_t = 0;
    let mut i: size_t = 0;
    p =
        k5_input_get_bytes(in_0,
                           ::std::mem::size_of::<[uint8_t; 32]>() as
                               libc::c_ulong);
    if !p.is_null() {
        memcpy((*msg).random.as_mut_ptr() as *mut libc::c_void,
               p as *const libc::c_void,
               ::std::mem::size_of::<[uint8_t; 32]>() as libc::c_ulong);
    }
    protocol_version = k5_input_get_uint64_le(in_0);
    if protocol_version != 0 as libc::c_int as libc::c_ulong {
        *minor = 0x20000018 as libc::c_int as OM_uint32;
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    offset = k5_input_get_uint32_le(in_0) as size_t;
    count = k5_input_get_uint16_le(in_0) as size_t;
    (*msg).schemes =
        vector_base(offset, count, 16 as libc::c_int as size_t, msg_base,
                    msg_len);
    (*msg).nschemes = count as uint16_t;
    if (*msg).schemes.is_null() {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    offset = k5_input_get_uint32_le(in_0) as size_t;
    count = k5_input_get_uint16_le(in_0) as size_t;
    p =
        vector_base(offset, count, 12 as libc::c_int as size_t, msg_base,
                    msg_len);
    i = 0 as libc::c_int as size_t;
    while i < count {
        extension_type =
            load_32_le(p.offset(i.wrapping_mul(12 as libc::c_int as
                                                   libc::c_ulong) as isize) as
                           *const libc::c_void);
        if extension_type & 0x80000000 as libc::c_uint != 0 {
            *minor = 0x20000017 as libc::c_int as OM_uint32;
            return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "267:1"]
unsafe extern "C" fn parse_exchange_message(mut minor: *mut OM_uint32,
                                            mut in_0: *mut k5input,
                                            mut msg_base: *const uint8_t,
                                            mut msg_len: size_t,
                                            mut msg: *mut exchange_message)
 -> OM_uint32 {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut offset: size_t = 0;
    let mut len: size_t = 0;
    p = k5_input_get_bytes(in_0, 16 as libc::c_int as size_t);
    if !p.is_null() {
        memcpy((*msg).scheme.as_mut_ptr() as *mut libc::c_void,
               p as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    }
    offset = k5_input_get_uint32_le(in_0) as size_t;
    len = k5_input_get_uint32_le(in_0) as size_t;
    p =
        vector_base(offset, len, 1 as libc::c_int as size_t, msg_base,
                    msg_len);
    if p.is_null() {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*msg).token.value = p as *mut libc::c_void;
    (*msg).token.length = len;
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "292:1"]
unsafe extern "C" fn parse_verify_message(mut minor: *mut OM_uint32,
                                          mut in_0: *mut k5input,
                                          mut msg_base: *const uint8_t,
                                          mut msg_len: size_t,
                                          mut token_offset: size_t,
                                          mut msg: *mut verify_message)
 -> OM_uint32 {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut offset: size_t = 0;
    let mut len: size_t = 0;
    let mut hdrlen: uint32_t = 0;
    let mut cksum_scheme: uint32_t = 0;
    p = k5_input_get_bytes(in_0, 16 as libc::c_int as size_t);
    if !p.is_null() {
        memcpy((*msg).scheme.as_mut_ptr() as *mut libc::c_void,
               p as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    }
    hdrlen = k5_input_get_uint32_le(in_0);
    if hdrlen != 20 as libc::c_int as libc::c_uint {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    cksum_scheme = k5_input_get_uint32_le(in_0);
    if cksum_scheme != 1 as libc::c_int as libc::c_uint {
        *minor = 0x20000015 as libc::c_int as OM_uint32;
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*msg).cksum_type = k5_input_get_uint32_le(in_0);
    offset = k5_input_get_uint32_le(in_0) as size_t;
    len = k5_input_get_uint32_le(in_0) as size_t;
    (*msg).cksum =
        vector_base(offset, len, 1 as libc::c_int as size_t, msg_base,
                    msg_len);
    (*msg).cksum_len = len;
    if (*msg).cksum.is_null() {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*msg).offset_in_token = token_offset;
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "330:1"]
unsafe extern "C" fn parse_alert_message(mut minor: *mut OM_uint32,
                                         mut in_0: *mut k5input,
                                         mut msg_base: *const uint8_t,
                                         mut msg_len: size_t,
                                         mut msg: *mut alert_message)
 -> OM_uint32 {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut atype: uint32_t = 0;
    let mut reason: uint32_t = 0;
    let mut alerts_offset: size_t = 0;
    let mut nalerts: size_t = 0;
    let mut value_offset: size_t = 0;
    let mut value_len: size_t = 0;
    let mut i: size_t = 0;
    let mut alerts_in: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut pulse_in: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    p = k5_input_get_bytes(in_0, 16 as libc::c_int as size_t);
    if !p.is_null() {
        memcpy((*msg).scheme.as_mut_ptr() as *mut libc::c_void,
               p as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    }
    k5_input_get_uint32_le(in_0);
    alerts_offset = k5_input_get_uint32_le(in_0) as size_t;
    nalerts = k5_input_get_uint32_le(in_0) as size_t;
    p =
        vector_base(alerts_offset, nalerts, 12 as libc::c_int as size_t,
                    msg_base, msg_len);
    if p.is_null() {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* Look for a VERIFY_NO_KEY pulse alert in the alerts vector. */
    (*msg).verify_no_key = 0 as libc::c_int; /* skip header length */
    k5_input_init(&mut alerts_in, p as *const libc::c_void,
                  nalerts.wrapping_mul(12 as libc::c_int as libc::c_ulong));
    i = 0 as libc::c_int as size_t;
    while i < nalerts {
        atype = k5_input_get_uint32_le(&mut alerts_in);
        value_offset = k5_input_get_uint32_le(&mut alerts_in) as size_t;
        value_len = k5_input_get_uint32_le(&mut alerts_in) as size_t;
        p =
            vector_base(value_offset, value_len, 1 as libc::c_int as size_t,
                        msg_base, msg_len);
        if p.is_null() {
            *minor = 0x20000008 as libc::c_int as OM_uint32;
            return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if atype == 1 as libc::c_int as libc::c_uint &&
               value_len >= 8 as libc::c_int as libc::c_ulong {
            k5_input_init(&mut pulse_in, p as *const libc::c_void, value_len);
            k5_input_get_uint32_le(&mut pulse_in);
            reason = k5_input_get_uint32_le(&mut pulse_in);
            if reason == 1 as libc::c_int as libc::c_uint {
                (*msg).verify_no_key = 1 as libc::c_int
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "377:1"]
unsafe extern "C" fn parse_message(mut minor: *mut OM_uint32,
                                   mut ctx: spnego_gss_ctx_id_t,
                                   mut in_0: *mut k5input,
                                   mut token_base: *const uint8_t,
                                   mut msg: *mut negoex_message)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut msg_base: *const uint8_t = (*in_0).ptr;
    let mut conv_id: *const uint8_t = 0 as *const uint8_t;
    let mut token_remaining: size_t = (*in_0).len;
    let mut header_len: size_t = 0;
    let mut msg_len: size_t = 0;
    let mut signature: uint64_t = 0;
    let mut type_0: uint32_t = 0;
    let mut seqnum: uint32_t = 0;
    signature = k5_input_get_uint64_le(in_0);
    type_0 = k5_input_get_uint32_le(in_0);
    seqnum = k5_input_get_uint32_le(in_0);
    header_len = k5_input_get_uint32_le(in_0) as size_t;
    msg_len = k5_input_get_uint32_le(in_0) as size_t;
    conv_id = k5_input_get_bytes(in_0, 16 as libc::c_int as size_t);
    if (*in_0).status != 0 || msg_len > token_remaining ||
           header_len > msg_len {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if signature as libc::c_ulonglong !=
           0x535458454f47454e as libc::c_ulonglong {
        *minor = 0x20000006 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if seqnum != (*ctx).negoex_seqnum {
        *minor = 0x20000019 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if seqnum == 0 as libc::c_int as libc::c_uint {
        memcpy((*ctx).negoex_conv_id.as_mut_ptr() as *mut libc::c_void,
               conv_id as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    } else if !(memcmp(conv_id as *const libc::c_void,
                       (*ctx).negoex_conv_id.as_mut_ptr() as
                           *const libc::c_void,
                       16 as libc::c_int as libc::c_ulong) ==
                    0 as libc::c_int) {
        *minor = 0x20000009 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* Restrict the input region to the header. */
    (*in_0).len =
        header_len.wrapping_sub((*in_0).ptr.wrapping_offset_from(msg_base) as
                                    libc::c_long as libc::c_ulong);
    (*msg).type_0 = type_0;
    if type_0 == INITIATOR_NEGO as libc::c_int as libc::c_uint ||
           type_0 == ACCEPTOR_NEGO as libc::c_int as libc::c_uint {
        major =
            parse_nego_message(minor, in_0, msg_base, msg_len,
                               &mut (*msg).u.n)
    } else if type_0 == INITIATOR_META_DATA as libc::c_int as libc::c_uint ||
                  type_0 == ACCEPTOR_META_DATA as libc::c_int as libc::c_uint
                  || type_0 == CHALLENGE as libc::c_int as libc::c_uint ||
                  type_0 == AP_REQUEST as libc::c_int as libc::c_uint {
        major =
            parse_exchange_message(minor, in_0, msg_base, msg_len,
                                   &mut (*msg).u.e)
    } else if type_0 == VERIFY as libc::c_int as libc::c_uint {
        major =
            parse_verify_message(minor, in_0, msg_base, msg_len,
                                 msg_base.wrapping_offset_from(token_base) as
                                     libc::c_long as size_t, &mut (*msg).u.v)
    } else if type_0 == ALERT as libc::c_int as libc::c_uint {
        major =
            parse_alert_message(minor, in_0, msg_base, msg_len,
                                &mut (*msg).u.a)
    } else {
        *minor = 0x20000007 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if major != 0 as libc::c_int as libc::c_uint { return major }
    /* Reset the input buffer to the remainder of the token. */
    if (*in_0).status == 0 {
        k5_input_init(in_0,
                      msg_base.offset(msg_len as isize) as
                          *const libc::c_void,
                      token_remaining.wrapping_sub(msg_len));
    }
    (*ctx).negoex_seqnum = (*ctx).negoex_seqnum.wrapping_add(1);
    trace_received_message(ctx, msg);
    return 0 as libc::c_int as OM_uint32;
}
/*
 * Parse token into an array of negoex_message structures.  All pointer fields
 * within the parsed messages are aliases into token, so the result can be
 * freed with free().  An unknown protocol version, a critical extension, or an
 * unknown checksum scheme will cause a parsing failure.  Increment the
 * sequence number in ctx for each message, and record and check the
 * conversation ID in ctx as appropriate.
 */
#[no_mangle]
#[c2rust::src_loc = "452:1"]
pub unsafe extern "C" fn negoex_parse_token(mut minor: *mut OM_uint32,
                                            mut ctx: spnego_gss_ctx_id_t,
                                            mut token: gss_const_buffer_t,
                                            mut messages_out:
                                                *mut *mut negoex_message,
                                            mut count_out: *mut size_t)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut in_0: k5input =
        k5input{ptr: 0 as *const libc::c_uchar, len: 0, status: 0,};
    let mut messages: *mut negoex_message = 0 as *mut negoex_message;
    let mut newptr: *mut negoex_message = 0 as *mut negoex_message;
    *messages_out = 0 as *mut negoex_message;
    *count_out = 0 as libc::c_int as size_t;
    if !token.is_null() {
    } else {
        __assert_fail(b"token != GSS_C_NO_BUFFER\x00" as *const u8 as
                          *const libc::c_char,
                      b"negoex_util.c\x00" as *const u8 as
                          *const libc::c_char,
                      464 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 119],
                                                &[libc::c_char; 119]>(b"OM_uint32 negoex_parse_token(OM_uint32 *, spnego_gss_ctx_id_t, gss_const_buffer_t, struct negoex_message **, size_t *)\x00")).as_ptr());
    }
    k5_input_init(&mut in_0, (*token).value, (*token).length);
    while in_0.status == 0 as libc::c_int &&
              in_0.len > 0 as libc::c_int as libc::c_ulong {
        newptr =
            realloc(messages as *mut libc::c_void,
                    count.wrapping_add(1 as libc::c_int as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<negoex_message>()
                                                                           as
                                                                           libc::c_ulong))
                as *mut negoex_message;
        if newptr.is_null() {
            free(messages as *mut libc::c_void);
            *minor = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        messages = newptr;
        major =
            parse_message(minor, ctx, &mut in_0,
                          (*token).value as *const uint8_t,
                          &mut *messages.offset(count as isize));
        if major != 0 as libc::c_int as libc::c_uint { break ; }
        count = count.wrapping_add(1)
    }
    if in_0.status != 0 {
        *minor = 0x20000008 as libc::c_int as OM_uint32;
        major = (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if major != 0 as libc::c_int as libc::c_uint {
        free(messages as *mut libc::c_void);
        return major
    }
    *messages_out = messages;
    *count_out = count;
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "497:1"]
unsafe extern "C" fn locate_message(mut messages: *mut negoex_message,
                                    mut nmessages: size_t,
                                    mut type_0: message_type)
 -> *mut negoex_message {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while (i as libc::c_ulong) < nmessages {
        if (*messages.offset(i as isize)).type_0 == type_0 as libc::c_uint {
            return &mut *messages.offset(i as isize) as *mut negoex_message
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut negoex_message;
}
#[no_mangle]
#[c2rust::src_loc = "511:1"]
pub unsafe extern "C" fn negoex_locate_nego_message(mut messages:
                                                        *mut negoex_message,
                                                    mut nmessages: size_t,
                                                    mut type_0: message_type)
 -> *mut nego_message {
    let mut msg: *mut negoex_message =
        locate_message(messages, nmessages, type_0);
    return if msg.is_null() {
               0 as *mut nego_message
           } else { &mut (*msg).u.n };
}
#[no_mangle]
#[c2rust::src_loc = "520:1"]
pub unsafe extern "C" fn negoex_locate_exchange_message(mut messages:
                                                            *mut negoex_message,
                                                        mut nmessages: size_t,
                                                        mut type_0:
                                                            message_type)
 -> *mut exchange_message {
    let mut msg: *mut negoex_message =
        locate_message(messages, nmessages, type_0);
    return if msg.is_null() {
               0 as *mut exchange_message
           } else { &mut (*msg).u.e };
}
#[no_mangle]
#[c2rust::src_loc = "529:1"]
pub unsafe extern "C" fn negoex_locate_verify_message(mut messages:
                                                          *mut negoex_message,
                                                      mut nmessages: size_t)
 -> *mut verify_message {
    let mut msg: *mut negoex_message =
        locate_message(messages, nmessages, VERIFY);
    return if msg.is_null() {
               0 as *mut verify_message
           } else { &mut (*msg).u.v };
}
#[no_mangle]
#[c2rust::src_loc = "538:1"]
pub unsafe extern "C" fn negoex_locate_alert_message(mut messages:
                                                         *mut negoex_message,
                                                     mut nmessages: size_t)
 -> *mut alert_message {
    let mut msg: *mut negoex_message =
        locate_message(messages, nmessages, ALERT);
    return if msg.is_null() {
               0 as *mut alert_message
           } else { &mut (*msg).u.a };
}
/*
 * Add the encoding of a MESSAGE_HEADER structure to buf, given the number of
 * bytes of the payload following the full header.  Increment the sequence
 * number in ctx.  Set *payload_start_out to the position of the payload within
 * the message.
 */
#[c2rust::src_loc = "552:1"]
unsafe extern "C" fn put_message_header(mut ctx: spnego_gss_ctx_id_t,
                                        mut type_0: message_type,
                                        mut payload_len: uint32_t,
                                        mut payload_start_out:
                                            *mut uint32_t) {
    let mut header_len: size_t = 0;
    if type_0 as libc::c_uint == INITIATOR_NEGO as libc::c_int as libc::c_uint
           ||
           type_0 as libc::c_uint ==
               ACCEPTOR_NEGO as libc::c_int as libc::c_uint {
        header_len = 96 as libc::c_int as size_t
    } else if type_0 as libc::c_uint ==
                  INITIATOR_META_DATA as libc::c_int as libc::c_uint ||
                  type_0 as libc::c_uint ==
                      ACCEPTOR_META_DATA as libc::c_int as libc::c_uint ||
                  type_0 as libc::c_uint ==
                      CHALLENGE as libc::c_int as libc::c_uint ||
                  type_0 as libc::c_uint ==
                      AP_REQUEST as libc::c_int as libc::c_uint {
        header_len = 64 as libc::c_int as size_t
    } else if type_0 as libc::c_uint == VERIFY as libc::c_int as libc::c_uint
     {
        header_len = 80 as libc::c_int as size_t
    } else if type_0 as libc::c_uint == ALERT as libc::c_int as libc::c_uint {
        header_len = 72 as libc::c_int as size_t
    } else { abort(); }
    k5_buf_add_uint64_le(&mut (*ctx).negoex_transcript,
                         0x535458454f47454e as libc::c_ulonglong as uint64_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, type_0 as uint32_t);
    let fresh0 = (*ctx).negoex_seqnum;
    (*ctx).negoex_seqnum = (*ctx).negoex_seqnum.wrapping_add(1);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, fresh0);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         header_len as uint32_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         header_len.wrapping_add(payload_len as libc::c_ulong)
                             as uint32_t);
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   (*ctx).negoex_conv_id.as_mut_ptr() as *const libc::c_void,
                   16 as libc::c_int as size_t);
    *payload_start_out = header_len as uint32_t;
}
#[no_mangle]
#[c2rust::src_loc = "580:1"]
pub unsafe extern "C" fn negoex_add_nego_message(mut ctx: spnego_gss_ctx_id_t,
                                                 mut type_0: message_type,
                                                 mut random: *mut uint8_t) {
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    let mut payload_start: uint32_t = 0;
    let mut seqnum: uint32_t = (*ctx).negoex_seqnum;
    let mut nschemes: uint16_t = 0;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    nschemes = 0 as libc::c_int as uint16_t;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() {
        nschemes = nschemes.wrapping_add(1);
        mech = (*mech).links.tqe_next
    }
    put_message_header(ctx, type_0,
                       (nschemes as libc::c_int * 16 as libc::c_int) as
                           uint32_t, &mut payload_start);
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   random as *const libc::c_void,
                   32 as libc::c_int as size_t);
    /* ProtocolVersion */
    k5_buf_add_uint64_le(&mut (*ctx).negoex_transcript,
                         0 as libc::c_int as uint64_t);
    /* AuthSchemes vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint16_le(&mut (*ctx).negoex_transcript, nschemes);
    /* Extensions vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint16_le(&mut (*ctx).negoex_transcript,
                         0 as libc::c_int as uint16_t);
    /* Four bytes of padding to reach a multiple of 8 bytes. */
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   b"\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_char
                       as *const libc::c_void, 4 as libc::c_int as size_t);
    /* Payload (auth schemes); also build guid string for tracing. */
    k5_buf_init_dynamic(&mut buf);
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() {
        k5_buf_add_len(&mut (*ctx).negoex_transcript,
                       (*mech).scheme.as_mut_ptr() as *const libc::c_void,
                       16 as libc::c_int as size_t);
        add_guid(&mut buf, (*mech).scheme.as_mut_ptr() as *const uint8_t);
        k5_buf_add(&mut buf, b" \x00" as *const u8 as *const libc::c_char);
        mech = (*mech).links.tqe_next
    }
    if buf.len > 0 as libc::c_int as libc::c_ulong {
        k5_buf_truncate(&mut buf,
                        buf.len.wrapping_sub(1 as libc::c_int as
                                                 libc::c_ulong));
        if (*(*ctx).kctx).trace_callback.is_some() {
            krb5int_trace((*ctx).kctx,
                          b"NegoEx sending [{int}]{str}: {str}\x00" as
                              *const u8 as *const libc::c_char,
                          seqnum as libc::c_int, typestr(type_0), buf.data);
        }
        k5_buf_free(&mut buf);
    };
}
#[no_mangle]
#[c2rust::src_loc = "621:1"]
pub unsafe extern "C" fn negoex_add_exchange_message(mut ctx:
                                                         spnego_gss_ctx_id_t,
                                                     mut type_0: message_type,
                                                     mut scheme:
                                                         *const uint8_t,
                                                     mut token:
                                                         gss_buffer_t) {
    let mut payload_start: uint32_t = 0;
    put_message_header(ctx, type_0, (*token).length as uint32_t,
                       &mut payload_start);
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   scheme as *const libc::c_void,
                   16 as libc::c_int as size_t);
    /* Exchange byte vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         (*token).length as uint32_t);
    /* Payload (token) */
    k5_buf_add_len(&mut (*ctx).negoex_transcript, (*token).value,
                   (*token).length);
    trace_outgoing_message(ctx, type_0, scheme);
}
#[no_mangle]
#[c2rust::src_loc = "638:1"]
pub unsafe extern "C" fn negoex_add_verify_message(mut ctx:
                                                       spnego_gss_ctx_id_t,
                                                   mut scheme: *const uint8_t,
                                                   mut cksum_type: uint32_t,
                                                   mut cksum: *const uint8_t,
                                                   mut cksum_len: uint32_t) {
    let mut payload_start: uint32_t = 0;
    put_message_header(ctx, VERIFY, cksum_len, &mut payload_start);
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   scheme as *const libc::c_void,
                   16 as libc::c_int as size_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         20 as libc::c_int as uint32_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         1 as libc::c_int as uint32_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, cksum_type);
    /* ChecksumValue vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, cksum_len);
    /* Four bytes of padding to reach a multiple of 8 bytes. */
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   b"\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_char
                       as *const libc::c_void, 4 as libc::c_int as size_t);
    /* Payload (checksum contents) */
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   cksum as *const libc::c_void, cksum_len as size_t);
    trace_outgoing_message(ctx, VERIFY, scheme);
}
/* Add an ALERT_MESSAGE containing a single ALERT_TYPE_PULSE alert with the
 * reason ALERT_VERIFY_NO_KEY. */
#[no_mangle]
#[c2rust::src_loc = "663:1"]
pub unsafe extern "C" fn negoex_add_verify_no_key_alert(mut ctx:
                                                            spnego_gss_ctx_id_t,
                                                        mut scheme:
                                                            *const uint8_t) {
    let mut payload_start: uint32_t = 0;
    put_message_header(ctx, ALERT,
                       (12 as libc::c_int + 8 as libc::c_int) as uint32_t,
                       &mut payload_start);
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   scheme as *const libc::c_void,
                   16 as libc::c_int as size_t);
    /* ErrorCode */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         0 as libc::c_int as uint32_t);
    /* Alerts vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint16_le(&mut (*ctx).negoex_transcript,
                         1 as libc::c_int as uint16_t);
    /* Six bytes of padding to reach a multiple of 8 bytes. */
    k5_buf_add_len(&mut (*ctx).negoex_transcript,
                   b"\x00\x00\x00\x00\x00\x00\x00" as *const u8 as
                       *const libc::c_char as *const libc::c_void,
                   6 as libc::c_int as size_t);
    /* Payload part 1: a single ALERT element */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         1 as libc::c_int as uint32_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         payload_start.wrapping_add(12 as libc::c_int as
                                                        libc::c_uint));
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         8 as libc::c_int as uint32_t);
    /* Payload part 2: ALERT_PULSE */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         8 as libc::c_int as uint32_t);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript,
                         1 as libc::c_int as uint32_t);
    trace_outgoing_message(ctx, ALERT, scheme);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2011-2018 PADL Software Pty Ltd.
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
#[c2rust::src_loc = "691:1"]
unsafe extern "C" fn release_auth_mech(mut mech: *mut negoex_auth_mech) {
    let mut tmpmin: OM_uint32 = 0;
    if mech.is_null() { return }
    gss_delete_sec_context(&mut tmpmin, &mut (*mech).mech_context,
                           0 as gss_buffer_t);
    generic_gss_release_oid(&mut tmpmin, &mut (*mech).oid);
    gss_release_buffer(&mut tmpmin, &mut (*mech).metadata);
    krb5_free_keyblock_contents(0 as krb5_context, &mut (*mech).key);
    krb5_free_keyblock_contents(0 as krb5_context, &mut (*mech).verify_key);
    free(mech as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "708:1"]
pub unsafe extern "C" fn negoex_delete_auth_mech(mut ctx: spnego_gss_ctx_id_t,
                                                 mut mech:
                                                     *mut negoex_auth_mech) {
    if !(*mech).links.tqe_next.is_null() {
        (*(*mech).links.tqe_next).links.tqe_prev = (*mech).links.tqe_prev
    } else { (*ctx).negoex_mechs.tqh_last = (*mech).links.tqe_prev }
    *(*mech).links.tqe_prev = (*mech).links.tqe_next;
    release_auth_mech(mech);
}
/* Remove all auth mech entries except for mech from ctx->mechs. */
#[no_mangle]
#[c2rust::src_loc = "717:1"]
pub unsafe extern "C" fn negoex_select_auth_mech(mut ctx: spnego_gss_ctx_id_t,
                                                 mut mech:
                                                     *mut negoex_auth_mech) {
    if !mech.is_null() {
    } else {
        __assert_fail(b"mech != NULL\x00" as *const u8 as *const libc::c_char,
                      b"negoex_util.c\x00" as *const u8 as
                          *const libc::c_char,
                      721 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 77],
                                                &[libc::c_char; 77]>(b"void negoex_select_auth_mech(spnego_gss_ctx_id_t, struct negoex_auth_mech *)\x00")).as_ptr());
    }
    if !(*mech).links.tqe_next.is_null() {
        (*(*mech).links.tqe_next).links.tqe_prev = (*mech).links.tqe_prev
    } else { (*ctx).negoex_mechs.tqh_last = (*mech).links.tqe_prev }
    *(*mech).links.tqe_prev = (*mech).links.tqe_next;
    release_all_mechs(ctx);
    (*mech).links.tqe_next = (*ctx).negoex_mechs.tqh_first;
    if !(*mech).links.tqe_next.is_null() {
        (*(*ctx).negoex_mechs.tqh_first).links.tqe_prev =
            &mut (*mech).links.tqe_next
    } else { (*ctx).negoex_mechs.tqh_last = &mut (*mech).links.tqe_next }
    (*ctx).negoex_mechs.tqh_first = mech;
    (*mech).links.tqe_prev = &mut (*ctx).negoex_mechs.tqh_first;
}
#[no_mangle]
#[c2rust::src_loc = "727:1"]
pub unsafe extern "C" fn negoex_add_auth_mech(mut minor: *mut OM_uint32,
                                              mut ctx: spnego_gss_ctx_id_t,
                                              mut oid: gss_const_OID,
                                              mut scheme: *mut uint8_t)
 -> OM_uint32 {
    let mut major: OM_uint32 = 0;
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    mech =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<negoex_auth_mech>() as libc::c_ulong) as
            *mut negoex_auth_mech;
    if mech.is_null() {
        *minor = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    major =
        generic_gss_copy_oid(minor, oid as gss_OID as *const gss_OID_desc,
                             &mut (*mech).oid);
    if major != 0 as libc::c_int as libc::c_uint {
        free(mech as *mut libc::c_void);
        return major
    }
    memcpy((*mech).scheme.as_mut_ptr() as *mut libc::c_void,
           scheme as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    (*mech).links.tqe_next = 0 as *mut negoex_auth_mech;
    (*mech).links.tqe_prev = (*ctx).negoex_mechs.tqh_last;
    *(*ctx).negoex_mechs.tqh_last = mech;
    (*ctx).negoex_mechs.tqh_last = &mut (*mech).links.tqe_next;
    *minor = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "754:1"]
pub unsafe extern "C" fn negoex_locate_auth_scheme(mut ctx:
                                                       spnego_gss_ctx_id_t,
                                                   mut scheme: *const uint8_t)
 -> *mut negoex_auth_mech {
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() {
        if memcmp((*mech).scheme.as_mut_ptr() as *const libc::c_void,
                  scheme as *const libc::c_void,
                  16 as libc::c_int as libc::c_ulong) == 0 as libc::c_int {
            return mech
        }
        mech = (*mech).links.tqe_next
    }
    return 0 as *mut negoex_auth_mech;
}
/* Prune ctx->mechs to the schemes present in schemes, and reorder them to
 * match its order. */
#[no_mangle]
#[c2rust::src_loc = "769:1"]
pub unsafe extern "C" fn negoex_common_auth_schemes(mut ctx:
                                                        spnego_gss_ctx_id_t,
                                                    mut schemes:
                                                        *const uint8_t,
                                                    mut nschemes: uint16_t) {
    let mut list: negoex_mech_list =
        negoex_mech_list{tqh_first: 0 as *mut negoex_auth_mech,
                         tqh_last: 0 as *mut *mut negoex_auth_mech,};
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    let mut i: uint16_t = 0;
    /* Construct a new list in the order of schemes. */
    list.tqh_first = 0 as *mut negoex_auth_mech;
    list.tqh_last = &mut list.tqh_first;
    i = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < nschemes as libc::c_int {
        mech =
            negoex_locate_auth_scheme(ctx,
                                      schemes.offset((i as libc::c_int *
                                                          16 as libc::c_int)
                                                         as isize));
        if !mech.is_null() {
            if !(*mech).links.tqe_next.is_null() {
                (*(*mech).links.tqe_next).links.tqe_prev =
                    (*mech).links.tqe_prev
            } else { (*ctx).negoex_mechs.tqh_last = (*mech).links.tqe_prev }
            *(*mech).links.tqe_prev = (*mech).links.tqe_next;
            (*mech).links.tqe_next = 0 as *mut negoex_auth_mech;
            (*mech).links.tqe_prev = list.tqh_last;
            *list.tqh_last = mech;
            list.tqh_last = &mut (*mech).links.tqe_next
        }
        i = i.wrapping_add(1)
    }
    /* Release any leftover entries and replace the context list. */
    release_all_mechs(ctx);
    if !list.tqh_first.is_null() {
        *(*ctx).negoex_mechs.tqh_last = list.tqh_first;
        (*list.tqh_first).links.tqe_prev = (*ctx).negoex_mechs.tqh_last;
        (*ctx).negoex_mechs.tqh_last = list.tqh_last;
        list.tqh_first = 0 as *mut negoex_auth_mech;
        list.tqh_last = &mut list.tqh_first
    };
}
/* negoex_util.c */
/* Prune ctx->mechs to the schemes present in schemes, but do not change
 * their order. */
#[no_mangle]
#[c2rust::src_loc = "794:1"]
pub unsafe extern "C" fn negoex_restrict_auth_schemes(mut ctx:
                                                          spnego_gss_ctx_id_t,
                                                      mut schemes:
                                                          *const uint8_t,
                                                      mut nschemes:
                                                          uint16_t) {
    let mut mech: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    let mut next: *mut negoex_auth_mech = 0 as *mut negoex_auth_mech;
    let mut i: uint16_t = 0;
    let mut found: libc::c_int = 0;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() &&
              { next = (*mech).links.tqe_next; (1 as libc::c_int) != 0 } {
        found = 0 as libc::c_int;
        i = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < nschemes as libc::c_int && found == 0 {
            if memcmp((*mech).scheme.as_mut_ptr() as *const libc::c_void,
                      schemes.offset((i as libc::c_int * 16 as libc::c_int) as
                                         isize) as *const libc::c_void,
                      16 as libc::c_int as libc::c_ulong) == 0 as libc::c_int
               {
                found = 1 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
        if found == 0 { negoex_delete_auth_mech(ctx, mech); }
        mech = next
    };
}
