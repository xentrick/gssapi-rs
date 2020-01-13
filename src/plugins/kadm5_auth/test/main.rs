use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:43"]
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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:43"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:43"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:43"]
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
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:43"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32};
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:43"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:43"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:43"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:43"]
pub mod plugin_h {
    #[c2rust::src_loc = "34:1"]
    pub type krb5_plugin_vtable = *mut krb5_plugin_vtable_st;
    extern "C" {
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
        #[c2rust::src_loc = "34:16"]
        pub type krb5_plugin_vtable_st;
    }
    /* KRB5_PLUGIN_H */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:44"]
pub mod kdb_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    /* NOT saved */
    /* String attributes (currently stored inside tl-data) map C string keys to
 * values.  They can be set via kadmin and consumed by KDC plugins. */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    /* Array of pointers */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
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
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_magic,
                        krb5_ui_4, krb5_flags, krb5_deltat, krb5_timestamp,
                        krb5_kvno, krb5_principal, krb5_context,
                        krb5_principal_data, krb5_const_principal,
                        krb5_error_code};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn krb5_db_get_principal(kcontext: krb5_context,
                                     search_for: krb5_const_principal,
                                     flags: libc::c_uint,
                                     entry: *mut *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn krb5_db_put_principal(kcontext: krb5_context,
                                     entry: *mut krb5_db_entry)
         -> krb5_error_code;
        /* Array */
        /* Retrieve a single string attribute from entry, or NULL if there is no
 * attribute for key.  Free *value_out with krb5_dbe_free_string when done. */
        #[no_mangle]
        #[c2rust::src_loc = "582:1"]
        pub fn krb5_dbe_get_string(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   key: *const libc::c_char,
                                   value_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* Change or add a string attribute in entry, or delete it if value is NULL. */
        #[no_mangle]
        #[c2rust::src_loc = "587:1"]
        pub fn krb5_dbe_set_string(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   key: *const libc::c_char,
                                   value: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "866:1"]
        pub fn krb5_dbe_free_string(_: krb5_context, _: *mut libc::c_char);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:44"]
pub mod admin_h {
    /* Novell */
    /* all but KEY_DATA, TL_DATA, LOAD */
    /* kadm5_policy_ent_t */
    /* kadm5_config_params */
    /*#define KADM5_CONFIG_ADMIN_KEYTAB       0x00000080*/
    /*
 * permission bits
 */
    /*
 * API versioning constants
 */
    /*
 * A module can optionally include <kadm5/admin.h> to inspect principal or
 * policy records from requests that add or modify principals or policies.
 * Note that fields of principal and policy structures are only valid if the
 * corresponding bit is set in the accompanying mask parameter.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16};
    use super::kdb_h::{krb5_tl_data, krb5_key_data};
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kadm5_auth_plugin.h:45"]
pub mod kadm5_auth_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2017 by the Massachusetts Institute of Technology.
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
 * Declarations for kadm5_auth plugin module implementors.
 *
 * The kadm5_auth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * kadm5_auth plugin modules should define a function named
 * kadm5_auth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   kadm5_auth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                             krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_kadm5_auth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* An abstract type for kadm5_auth module data. */
    #[c2rust::src_loc = "67:1"]
    pub type kadm5_auth_moddata = *mut kadm5_auth_moddata_st;
    /*
 * A module can optionally generate restrictions when checking permissions for
 * adding or modifying a principal entry.  Restriction fields will only be
 * honored if the corresponding mask bit is set.  The operable mask bits are
 * defined in <kadmin/admin.h> and are:
 *
 * - KADM5_ATTRIBUTES for require_attrs, forbid_attrs
 * - KADM5_POLICY for policy
 * - KADM5_POLICY_CLR to require that policy be unset
 * - KADM5_PRINC_EXPIRE_TIME for princ_lifetime
 * - KADM5_PW_EXPIRATION for pw_lifetime
 * - KADM5_MAX_LIFE for max_life
 * - KADM5_MAX_RLIFE for max_renewable_life
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:8"]
    pub struct kadm5_auth_restrictions {
        pub mask: libc::c_long,
        pub require_attrs: krb5_flags,
        pub forbid_attrs: krb5_flags,
        pub princ_lifetime: krb5_deltat,
        pub pw_lifetime: krb5_deltat,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub policy: *mut libc::c_char,
    }
    /* ** Method type declarations ***/
    /*
 * Optional: Initialize module data.  acl_file is the realm's configured ACL
 * file, or NULL if none was configured.  Return 0 on success,
 * KRB5_PLUGIN_NO_HANDLE if the module is inoperable (due to configuration, for
 * example), and any other error code to abort kadmind startup.  Optionally set
 * *data_out to a module data object to be passed to future calls.
 */
    #[c2rust::src_loc = "112:1"]
    pub type kadm5_auth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                    _: *mut kadm5_auth_moddata)
                   -> krb5_error_code>;
    /* Optional: Release resources used by module data. */
    #[c2rust::src_loc = "117:1"]
    pub type kadm5_auth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata)
                   -> ()>;
    /*
 * Each check method below should return 0 to explicitly authorize the request,
 * KRB5_PLUGIN_NO_HANDLE to neither authorize nor deny the request, and any
 * other error code (such as EPERM) to explicitly deny the request.  If a check
 * method is not defined, the module will neither authorize nor deny the
 * request.  A request succeeds if at least one kadm5_auth module explicitly
 * authorizes the request and none of the modules explicitly deny it.
 */
    /* Optional: authorize an add-principal operation, and optionally generate
 * restrictions. */
    #[c2rust::src_loc = "131:1"]
    pub type kadm5_auth_addprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: *const _kadm5_principal_ent_t,
                                    _: libc::c_long,
                                    _: *mut *mut kadm5_auth_restrictions)
                   -> krb5_error_code>;
    /* Optional: authorize a modify-principal operation, and optionally generate
 * restrictions. */
    #[c2rust::src_loc = "140:1"]
    pub type kadm5_auth_modprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: *const _kadm5_principal_ent_t,
                                    _: libc::c_long,
                                    _: *mut *mut kadm5_auth_restrictions)
                   -> krb5_error_code>;
    /* Optional: authorize a set-string operation. */
    #[c2rust::src_loc = "148:1"]
    pub type kadm5_auth_setstr_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: authorize a change-password operation. */
    #[c2rust::src_loc = "155:1"]
    pub type kadm5_auth_cpw_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a randomize-keys operation. */
    #[c2rust::src_loc = "160:1"]
    pub type kadm5_auth_chrand_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a set-key operation. */
    #[c2rust::src_loc = "166:1"]
    pub type kadm5_auth_setkey_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a purgekeys operation. */
    #[c2rust::src_loc = "172:1"]
    pub type kadm5_auth_purgekeys_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a delete-principal operation. */
    #[c2rust::src_loc = "178:1"]
    pub type kadm5_auth_delprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a rename-principal operation. */
    #[c2rust::src_loc = "184:1"]
    pub type kadm5_auth_renprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a get-principal operation. */
    #[c2rust::src_loc = "191:1"]
    pub type kadm5_auth_getprinc_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a get-strings operation. */
    #[c2rust::src_loc = "197:1"]
    pub type kadm5_auth_getstrs_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize an extract-keys operation. */
    #[c2rust::src_loc = "203:1"]
    pub type kadm5_auth_extract_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize a list-principals operation. */
    #[c2rust::src_loc = "209:1"]
    pub type kadm5_auth_listprincs_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize an add-policy operation. */
    #[c2rust::src_loc = "214:1"]
    pub type kadm5_auth_addpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const _kadm5_policy_ent_t,
                                    _: libc::c_long) -> krb5_error_code>;
    /* Optional: authorize a modify-policy operation. */
    #[c2rust::src_loc = "220:1"]
    pub type kadm5_auth_modpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const _kadm5_policy_ent_t,
                                    _: libc::c_long) -> krb5_error_code>;
    /* Optional: authorize a delete-policy operation. */
    #[c2rust::src_loc = "226:1"]
    pub type kadm5_auth_delpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: authorize a get-policy operation.  client_policy is the client
 * principal's policy name, or NULL if it does not have one. */
    #[c2rust::src_loc = "232:1"]
    pub type kadm5_auth_getpol_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: authorize a list-policies operation. */
    #[c2rust::src_loc = "238:1"]
    pub type kadm5_auth_listpols_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /* Optional: authorize an iprop operation. */
    #[c2rust::src_loc = "243:1"]
    pub type kadm5_auth_iprop_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: krb5_const_principal)
                   -> krb5_error_code>;
    /*
 * Optional: receive a notification that the most recent authorized operation
 * has ended.  If a kadm5_auth module is also a KDB module, it can assume that
 * all KDB methods invoked between a kadm5_auth authorization method invocation
 * and a kadm5_auth end invocation are performed as part of the authorized
 * operation.
 *
 * The end method may be invoked without a preceding authorization method in
 * some cases; the module must be prepared to ignore such calls.
 */
    #[c2rust::src_loc = "257:1"]
    pub type kadm5_auth_end_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata)
                   -> ()>;
    /*
 * Optional: free a restrictions object.  This method does not need to be
 * defined if the module does not generate restrictions objects, or if it
 * returns aliases to restrictions objects contained from within the module
 * data.
 */
    #[c2rust::src_loc = "266:1"]
    pub type kadm5_auth_free_restrictions_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                    _: *mut kadm5_auth_restrictions) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "272:16"]
    pub struct kadm5_auth_vtable_st {
        pub name: *const libc::c_char,
        pub init: kadm5_auth_init_fn,
        pub fini: kadm5_auth_fini_fn,
        pub addprinc: kadm5_auth_addprinc_fn,
        pub modprinc: kadm5_auth_modprinc_fn,
        pub setstr: kadm5_auth_setstr_fn,
        pub cpw: kadm5_auth_cpw_fn,
        pub chrand: kadm5_auth_chrand_fn,
        pub setkey: kadm5_auth_setkey_fn,
        pub purgekeys: kadm5_auth_purgekeys_fn,
        pub delprinc: kadm5_auth_delprinc_fn,
        pub renprinc: kadm5_auth_renprinc_fn,
        pub getprinc: kadm5_auth_getprinc_fn,
        pub getstrs: kadm5_auth_getstrs_fn,
        pub extract: kadm5_auth_extract_fn,
        pub listprincs: kadm5_auth_listprincs_fn,
        pub addpol: kadm5_auth_addpol_fn,
        pub modpol: kadm5_auth_modpol_fn,
        pub delpol: kadm5_auth_delpol_fn,
        pub getpol: kadm5_auth_getpol_fn,
        pub listpols: kadm5_auth_listpols_fn,
        pub iprop: kadm5_auth_iprop_fn,
        pub end: kadm5_auth_end_fn,
        pub free_restrictions: kadm5_auth_free_restrictions_fn,
    }
    /* kadm5_auth vtable for major version 1. */
    #[c2rust::src_loc = "272:1"]
    pub type kadm5_auth_vtable = *mut kadm5_auth_vtable_st;
    use super::krb5_h::{krb5_flags, krb5_deltat, krb5_error_code,
                        krb5_context, krb5_const_principal};
    use super::admin_h::{_kadm5_principal_ent_t, _kadm5_policy_ent_t};
    extern "C" {
        #[c2rust::src_loc = "67:16"]
        pub type kadm5_auth_moddata_st;
    }
    /* Mandatory: name of module. */
    /* Minor version 1 ends here. */
    /* KRB5_KADM5_AUTH_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:43"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:43"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:43"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t,
                       krb5_parse_name, krb5_free_principal};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data, _krb5_db_entry_new, krb5_db_entry,
                      krb5_db_get_principal, krb5_db_free_principal,
                      krb5_db_put_principal, krb5_dbe_get_string,
                      krb5_dbe_set_string, krb5_dbe_free_string};
pub use self::admin_h::{_kadm5_principal_ent_t, _kadm5_policy_ent_t};
pub use self::kadm5_auth_plugin_h::{kadm5_auth_moddata,
                                    kadm5_auth_restrictions,
                                    kadm5_auth_init_fn, kadm5_auth_fini_fn,
                                    kadm5_auth_addprinc_fn,
                                    kadm5_auth_modprinc_fn,
                                    kadm5_auth_setstr_fn, kadm5_auth_cpw_fn,
                                    kadm5_auth_chrand_fn,
                                    kadm5_auth_setkey_fn,
                                    kadm5_auth_purgekeys_fn,
                                    kadm5_auth_delprinc_fn,
                                    kadm5_auth_renprinc_fn,
                                    kadm5_auth_getprinc_fn,
                                    kadm5_auth_getstrs_fn,
                                    kadm5_auth_extract_fn,
                                    kadm5_auth_listprincs_fn,
                                    kadm5_auth_addpol_fn,
                                    kadm5_auth_modpol_fn,
                                    kadm5_auth_delpol_fn,
                                    kadm5_auth_getpol_fn,
                                    kadm5_auth_listpols_fn,
                                    kadm5_auth_iprop_fn, kadm5_auth_end_fn,
                                    kadm5_auth_free_restrictions_fn,
                                    kadm5_auth_vtable_st, kadm5_auth_vtable,
                                    kadm5_auth_moddata_st};
use self::stdio_h::snprintf;
use self::stdlib_h::atoi;
use self::string_h::{strlen, strcmp};
/* The welcomer authorizes all getprinc operations, since kadmin uses them as a
 * precursor to modprinc. */
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn welcomer_getprinc(mut context: krb5_context,
                                       mut data: kadm5_auth_moddata,
                                       mut client: krb5_const_principal,
                                       mut target: krb5_const_principal)
 -> krb5_error_code {
    return 0 as libc::c_int;
}
/* The welcomer authorizes addprinc operations which set a policy "VIP". */
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn welcomer_addprinc(mut context: krb5_context,
                                       mut data: kadm5_auth_moddata,
                                       mut client: krb5_const_principal,
                                       mut target: krb5_const_principal,
                                       mut ent: *const _kadm5_principal_ent_t,
                                       mut mask: libc::c_long,
                                       mut rs_out:
                                           *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    if mask & 0x800 as libc::c_int as libc::c_long != 0 &&
           strcmp((*ent).policy,
                  b"VIP\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        return 0 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The bouncer denies addprinc operations which include a maximum lifetime. */
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn bouncer_addprinc(mut context: krb5_context,
                                      mut data: kadm5_auth_moddata,
                                      mut client: krb5_const_principal,
                                      mut target: krb5_const_principal,
                                      mut ent: *const _kadm5_principal_ent_t,
                                      mut mask: libc::c_long,
                                      mut rs_out:
                                          *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    return if mask & 0x20 as libc::c_int as libc::c_long != 0 {
               1 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The welcomer authorizes modprinc operations which only set maxrenewlife. */
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn welcomer_modprinc(mut context: krb5_context,
                                       mut data: kadm5_auth_moddata,
                                       mut client: krb5_const_principal,
                                       mut target: krb5_const_principal,
                                       mut ent: *const _kadm5_principal_ent_t,
                                       mut mask: libc::c_long,
                                       mut rs_out:
                                           *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    return if mask == 0x2000 as libc::c_int as libc::c_long {
               0 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The bouncer denies modprinc operations if the target principal has an even
 * number of components. */
#[c2rust::src_loc = "97:1"]
unsafe extern "C" fn bouncer_modprinc(mut context: krb5_context,
                                      mut data: kadm5_auth_moddata,
                                      mut client: krb5_const_principal,
                                      mut target: krb5_const_principal,
                                      mut ent: *const _kadm5_principal_ent_t,
                                      mut mask: libc::c_long,
                                      mut rs_out:
                                          *mut *mut kadm5_auth_restrictions)
 -> krb5_error_code {
    return if (*target).length % 2 as libc::c_int == 0 as libc::c_int {
               1 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The welcomer authorizes setstr operations for the attribute "note". */
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn welcomer_setstr(mut context: krb5_context,
                                     mut data: kadm5_auth_moddata,
                                     mut client: krb5_const_principal,
                                     mut target: krb5_const_principal,
                                     mut key: *const libc::c_char,
                                     mut value: *const libc::c_char)
 -> krb5_error_code {
    return if strcmp(key, b"note\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
               0 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The bouncer denies setstr operations if the value is more than 10 bytes. */
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn bouncer_setstr(mut context: krb5_context,
                                    mut data: kadm5_auth_moddata,
                                    mut client: krb5_const_principal,
                                    mut target: krb5_const_principal,
                                    mut key: *const libc::c_char,
                                    mut value: *const libc::c_char)
 -> krb5_error_code {
    return if strlen(value) > 10 as libc::c_int as libc::c_ulong {
               1 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The welcomer authorizes delprinc operations if the target principal starts
 * with "d". */
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn welcomer_delprinc(mut context: krb5_context,
                                       mut data: kadm5_auth_moddata,
                                       mut client: krb5_const_principal,
                                       mut target: krb5_const_principal)
 -> krb5_error_code {
    if (*target).length > 0 as libc::c_int &&
           (*(*target).data.offset(0 as libc::c_int as isize)).length >
               0 as libc::c_int as libc::c_uint &&
           *(*(*target).data.offset(0 as libc::c_int as isize)).data as
               libc::c_int == 'd' as i32 {
        return 0 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The bouncer denies delprinc operations if the target principal has the
 * "nodelete" string attribute. */
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn bouncer_delprinc(mut context: krb5_context,
                                      mut data: kadm5_auth_moddata,
                                      mut client: krb5_const_principal,
                                      mut target: krb5_const_principal)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut ent: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    if krb5_db_get_principal(context, target,
                             0 as libc::c_int as libc::c_uint, &mut ent) !=
           0 as libc::c_int {
        return 1 as libc::c_int
    }
    ret =
        krb5_dbe_get_string(context, ent,
                            b"nodelete\x00" as *const u8 as
                                *const libc::c_char, &mut val);
    krb5_db_free_principal(context, ent);
    ret =
        if ret != 0 as libc::c_int || !val.is_null() {
            1 as libc::c_int as libc::c_long
        } else { -(1765328135 as libc::c_long) } as krb5_error_code;
    krb5_dbe_free_string(context, val);
    return ret;
}
/* The welcomer authorizes rename operations if the first components of the
 * principals have the same length. */
#[c2rust::src_loc = "157:1"]
unsafe extern "C" fn welcomer_renprinc(mut context: krb5_context,
                                       mut data: kadm5_auth_moddata,
                                       mut client: krb5_const_principal,
                                       mut src: krb5_const_principal,
                                       mut dest: krb5_const_principal)
 -> krb5_error_code {
    if (*src).length > 0 as libc::c_int && (*dest).length > 0 as libc::c_int
           &&
           (*(*src).data.offset(0 as libc::c_int as isize)).length ==
               (*(*dest).data.offset(0 as libc::c_int as isize)).length {
        return 0 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The bouncer denies rename operations if the source principal starts with
 * "a". */
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn bouncer_renprinc(mut context: krb5_context,
                                      mut data: kadm5_auth_moddata,
                                      mut client: krb5_const_principal,
                                      mut src: krb5_const_principal,
                                      mut dest: krb5_const_principal)
 -> krb5_error_code {
    if (*src).length > 0 as libc::c_int &&
           (*(*src).data.offset(0 as libc::c_int as isize)).length >
               0 as libc::c_int as libc::c_uint &&
           *(*(*src).data.offset(0 as libc::c_int as isize)).data as
               libc::c_int == 'a' as i32 {
        return 1 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The welcomer authorizes addpol operations which set a minlength of 3. */
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn welcomer_addpol(mut context: krb5_context,
                                     mut data: kadm5_auth_moddata,
                                     mut client: krb5_const_principal,
                                     mut policy: *const libc::c_char,
                                     mut ent: *const _kadm5_policy_ent_t,
                                     mut mask: libc::c_long)
 -> krb5_error_code {
    if mask & 0x10000 as libc::c_int as libc::c_long != 0 &&
           (*ent).pw_min_length == 3 as libc::c_int as libc::c_long {
        return 0 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The bouncer denies addpol operations if the name is 3 bytes or less. */
#[c2rust::src_loc = "193:1"]
unsafe extern "C" fn bouncer_addpol(mut context: krb5_context,
                                    mut data: kadm5_auth_moddata,
                                    mut client: krb5_const_principal,
                                    mut policy: *const libc::c_char,
                                    mut ent: *const _kadm5_policy_ent_t,
                                    mut mask: libc::c_long)
 -> krb5_error_code {
    return if strlen(policy) <= 3 as libc::c_int as libc::c_ulong {
               1 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The welcomer authorizes modpol operations which only change min_life. */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn welcomer_modpol(mut context: krb5_context,
                                     mut data: kadm5_auth_moddata,
                                     mut client: krb5_const_principal,
                                     mut policy: *const libc::c_char,
                                     mut ent: *const _kadm5_policy_ent_t,
                                     mut mask: libc::c_long)
 -> krb5_error_code {
    return if mask == 0x8000 as libc::c_int as libc::c_long {
               0 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The bouncer denies modpol operations which set pw_min_life above 10. */
#[c2rust::src_loc = "211:1"]
unsafe extern "C" fn bouncer_modpol(mut context: krb5_context,
                                    mut data: kadm5_auth_moddata,
                                    mut client: krb5_const_principal,
                                    mut policy: *const libc::c_char,
                                    mut ent: *const _kadm5_policy_ent_t,
                                    mut mask: libc::c_long)
 -> krb5_error_code {
    if mask & 0x8000 as libc::c_int as libc::c_long != 0 &&
           (*ent).pw_min_life > 10 as libc::c_int as libc::c_long {
        return 1 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The welcomer authorizes getpol operations if the policy and client principal
 * policy have the same length. */
#[c2rust::src_loc = "223:1"]
unsafe extern "C" fn welcomer_getpol(mut context: krb5_context,
                                     mut data: kadm5_auth_moddata,
                                     mut client: krb5_const_principal,
                                     mut policy: *const libc::c_char,
                                     mut client_policy: *const libc::c_char)
 -> krb5_error_code {
    if !client_policy.is_null() && strlen(policy) == strlen(client_policy) {
        return 0 as libc::c_int
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
/* The bouncer denies getpol operations if the policy name begins with 'x'. */
#[c2rust::src_loc = "234:1"]
unsafe extern "C" fn bouncer_getpol(mut context: krb5_context,
                                    mut data: kadm5_auth_moddata,
                                    mut client: krb5_const_principal,
                                    mut policy: *const libc::c_char,
                                    mut client_policy: *const libc::c_char)
 -> krb5_error_code {
    return if *policy as libc::c_int == 'x' as i32 {
               1 as libc::c_int as libc::c_long
           } else { -(1765328135 as libc::c_long) } as krb5_error_code;
}
/* The welcomer counts end calls by incrementing the "ends" string attribute on
 * the "opcount" principal, if it exists. */
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn welcomer_end(mut context: krb5_context,
                                  mut data: kadm5_auth_moddata) {
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut ent: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 10] = [0; 10];
    if !(krb5_parse_name(context,
                         b"opcount\x00" as *const u8 as *const libc::c_char,
                         &mut princ) != 0 as libc::c_int) {
        if !(krb5_db_get_principal(context, princ as krb5_const_principal,
                                   0 as libc::c_int as libc::c_uint, &mut ent)
                 != 0 as libc::c_int) {
            if !(krb5_dbe_get_string(context, ent,
                                     b"ends\x00" as *const u8 as
                                         *const libc::c_char, &mut val) !=
                     0 as libc::c_int || val.is_null()) {
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 10]>() as
                             libc::c_ulong,
                         b"%d\x00" as *const u8 as *const libc::c_char,
                         atoi(val) + 1 as libc::c_int);
                if !(krb5_dbe_set_string(context, ent,
                                         b"ends\x00" as *const u8 as
                                             *const libc::c_char,
                                         buf.as_mut_ptr()) !=
                         0 as libc::c_int) {
                    (*ent).mask = 0x40000 as libc::c_int as krb5_ui_4;
                    krb5_db_put_principal(context, ent);
                }
            }
        }
    }
    krb5_dbe_free_string(context, val);
    krb5_db_free_principal(context, ent);
    krb5_free_principal(context, princ);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* plugins/kadm5_auth/test/main.c - test modules for kadm5_auth interface */
/*
 * Copyright (C) 2017 by the Massachusetts Institute of Technology.
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
 * This file implements two testing kadm5_auth modules, the welcomer and the
 * bouncer.  The welcomer implements permissive behavior, while the bouncer
 * implements restrictive behavior.
 *
 * Module data objects and restrictions are adequately tested by the acl
 * module, so we do not test them here.  Focus instead on the ability to
 * examine principal and policy objects and to perform DB operations.
 */
#[no_mangle]
#[c2rust::src_loc = "269:1"]
pub unsafe extern "C" fn kadm5_auth_welcomer_initvt(mut context: krb5_context,
                                                    mut maj_ver: libc::c_int,
                                                    mut min_ver: libc::c_int,
                                                    mut vtable:
                                                        krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: kadm5_auth_vtable = vtable as kadm5_auth_vtable;
    (*vt).name = b"welcomer\x00" as *const u8 as *const libc::c_char;
    (*vt).addprinc =
        Some(welcomer_addprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const _kadm5_principal_ent_t,
                                      _: libc::c_long,
                                      _: *mut *mut kadm5_auth_restrictions)
                     -> krb5_error_code);
    (*vt).modprinc =
        Some(welcomer_modprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const _kadm5_principal_ent_t,
                                      _: libc::c_long,
                                      _: *mut *mut kadm5_auth_restrictions)
                     -> krb5_error_code);
    (*vt).setstr =
        Some(welcomer_setstr as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    (*vt).delprinc =
        Some(welcomer_delprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).renprinc =
        Some(welcomer_renprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).getprinc =
        Some(welcomer_getprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).addpol =
        Some(welcomer_addpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const _kadm5_policy_ent_t,
                                      _: libc::c_long) -> krb5_error_code);
    (*vt).modpol =
        Some(welcomer_modpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const _kadm5_policy_ent_t,
                                      _: libc::c_long) -> krb5_error_code);
    (*vt).getpol =
        Some(welcomer_getpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    (*vt).end =
        Some(welcomer_end as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata)
                     -> ());
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "289:1"]
pub unsafe extern "C" fn kadm5_auth_bouncer_initvt(mut context: krb5_context,
                                                   mut maj_ver: libc::c_int,
                                                   mut min_ver: libc::c_int,
                                                   mut vtable:
                                                       krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: kadm5_auth_vtable = vtable as kadm5_auth_vtable;
    (*vt).name = b"bouncer\x00" as *const u8 as *const libc::c_char;
    (*vt).addprinc =
        Some(bouncer_addprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const _kadm5_principal_ent_t,
                                      _: libc::c_long,
                                      _: *mut *mut kadm5_auth_restrictions)
                     -> krb5_error_code);
    (*vt).modprinc =
        Some(bouncer_modprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const _kadm5_principal_ent_t,
                                      _: libc::c_long,
                                      _: *mut *mut kadm5_auth_restrictions)
                     -> krb5_error_code);
    (*vt).setstr =
        Some(bouncer_setstr as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    (*vt).delprinc =
        Some(bouncer_delprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).renprinc =
        Some(bouncer_renprinc as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal,
                                      _: krb5_const_principal)
                     -> krb5_error_code);
    (*vt).addpol =
        Some(bouncer_addpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const _kadm5_policy_ent_t,
                                      _: libc::c_long) -> krb5_error_code);
    (*vt).modpol =
        Some(bouncer_modpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const _kadm5_policy_ent_t,
                                      _: libc::c_long) -> krb5_error_code);
    (*vt).getpol =
        Some(bouncer_getpol as
                 unsafe extern "C" fn(_: krb5_context, _: kadm5_auth_moddata,
                                      _: krb5_const_principal,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
