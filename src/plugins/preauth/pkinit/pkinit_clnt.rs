use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
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
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
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
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
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
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
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
 * Build an anonymous principal.
 *
 * This function returns constant storage that must not be freed.
 *
 * @sa #KRB5_ANONYMOUS_PRINCSTR
 */
        #[no_mangle]
        #[c2rust::src_loc = "309:1"]
        pub fn krb5_anonymous_principal() -> krb5_const_principal;
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
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3682:1"]
        pub fn krb5_principal_compare_any_realm(context: krb5_context,
                                                princ1: krb5_const_principal,
                                                princ2: krb5_const_principal)
         -> krb5_boolean;
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
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4699:1"]
        pub fn krb5_free_checksum_contents(context: krb5_context,
                                           val: *mut krb5_checksum);
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
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:32"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_pa_data, krb5_data,
                        krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:32"]
pub mod profile_h {
    /*
 * profile.h
 */
    /*
 * profile.h
 */
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
    }
    /*@modifies internalState@*/
    /*@modifies internalState@*/
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:32"]
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:32"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:32"]
pub mod k5_int_pkinit_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * pkinit structures
 */
    /* PKAuthenticator */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct _krb5_pk_authenticator {
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub paChecksum: krb5_checksum,
        pub freshnessToken: *mut krb5_data,
    }
    #[c2rust::src_loc = "40:1"]
    pub type krb5_pk_authenticator = _krb5_pk_authenticator;
    /* AlgorithmIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    /* Optional */
    /* SubjectPublicKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct _krb5_subject_pk_info {
        pub algorithm: krb5_algorithm_identifier,
        pub subjectPublicKey: krb5_data,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_subject_pk_info = _krb5_subject_pk_info;
    /* BIT STRING */
    /* * AuthPack from RFC 4556*/
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct _krb5_auth_pack {
        pub pkAuthenticator: krb5_pk_authenticator,
        pub clientPublicValue: *mut krb5_subject_pk_info,
        pub supportedCMSTypes: *mut *mut krb5_algorithm_identifier,
        pub clientDHNonce: krb5_data,
        pub supportedKDFs: *mut *mut krb5_data,
    }
    #[c2rust::src_loc = "61:1"]
    pub type krb5_auth_pack = _krb5_auth_pack;
    /* OIDs of KDFs; OPTIONAL */
    /* ExternalPrincipalIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    /* Optional */
    /* PA-PK-AS-REQ (rfc4556 -- PA TYPE 16) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:16"]
    pub struct _krb5_pa_pk_as_req {
        pub signedAuthPack: krb5_data,
        pub trustedCertifiers: *mut *mut krb5_external_principal_identifier,
        pub kdcPkId: krb5_data,
    }
    #[c2rust::src_loc = "77:1"]
    pub type krb5_pa_pk_as_req = _krb5_pa_pk_as_req;
    /* Optional */
    /* * Pkinit DHRepInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:16"]
    pub struct _krb5_dh_rep_info {
        pub dhSignedData: krb5_data,
        pub serverDHNonce: krb5_data,
        pub kdfID: *mut krb5_data,
    }
    #[c2rust::src_loc = "84:1"]
    pub type krb5_dh_rep_info = _krb5_dh_rep_info;
    /* OID of selected KDF OPTIONAL */
    /* KDCDHKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:16"]
    pub struct _krb5_kdc_dh_key_info {
        pub subjectPublicKey: krb5_data,
        pub nonce: krb5_int32,
        pub dhKeyExpiration: krb5_timestamp,
    }
    #[c2rust::src_loc = "91:1"]
    pub type krb5_kdc_dh_key_info = _krb5_kdc_dh_key_info;
    /* Optional */
    /* ReplyKeyPack */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct _krb5_reply_key_pack {
        pub replyKey: krb5_keyblock,
        pub asChecksum: krb5_checksum,
    }
    #[c2rust::src_loc = "98:1"]
    pub type krb5_reply_key_pack = _krb5_reply_key_pack;
    /* PA-PK-AS-REP (rfc4556 -- PA TYPE 17) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:16"]
    pub struct _krb5_pa_pk_as_rep {
        pub choice: krb5_pa_pk_as_rep_selection,
        pub u: krb5_pa_pk_as_rep_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:11"]
    pub union krb5_pa_pk_as_rep_choices {
        pub dh_Info: krb5_dh_rep_info,
        pub encKeyPack: krb5_data,
    }
    #[c2rust::src_loc = "105:5"]
    pub type krb5_pa_pk_as_rep_selection = libc::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const choice_pa_pk_as_rep_encKeyPack: krb5_pa_pk_as_rep_selection = 1;
    #[c2rust::src_loc = "107:9"]
    pub const choice_pa_pk_as_rep_dhInfo: krb5_pa_pk_as_rep_selection = 0;
    #[c2rust::src_loc = "106:9"]
    pub const choice_pa_pk_as_rep_UNKNOWN: krb5_pa_pk_as_rep_selection = -1;
    #[c2rust::src_loc = "104:1"]
    pub type krb5_pa_pk_as_rep = _krb5_pa_pk_as_rep;
    use super::krb5_h::{krb5_int32, krb5_timestamp, krb5_checksum, krb5_data,
                        krb5_keyblock};
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:33"]
pub mod clpreauth_plugin_h {
    #[c2rust::src_loc = "75:1"]
    pub type krb5_clpreauth_rock = *mut krb5_clpreauth_rock_st;
    #[c2rust::src_loc = "78:1"]
    pub type krb5_clpreauth_moddata = *mut krb5_clpreauth_moddata_st;
    #[c2rust::src_loc = "79:1"]
    pub type krb5_clpreauth_modreq = *mut krb5_clpreauth_modreq_st;
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
    #[c2rust::src_loc = "83:1"]
    pub type krb5_clpreauth_callbacks = *mut krb5_clpreauth_callbacks_st;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_clpreauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_clpreauth_moddata)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "189:1"]
    pub type krb5_clpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata) -> ()>;
    #[c2rust::src_loc = "200:1"]
    pub type krb5_clpreauth_get_flags_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                   -> libc::c_int>;
    #[c2rust::src_loc = "210:1"]
    pub type krb5_clpreauth_request_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: *mut krb5_clpreauth_modreq) -> ()>;
    #[c2rust::src_loc = "214:1"]
    pub type krb5_clpreauth_request_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_clpreauth_moddata,
                                    _: krb5_clpreauth_modreq) -> ()>;
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
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 2006 Red Hat, Inc.
 * Portions copyright (c) 2006, 2011 Massachusetts Institute of Technology
 * All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in
 *    the documentation and/or other materials provided with the
 *    distribution.
 *  * Neither the name of Red Hat, Inc., nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
 * OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    /*
 * Declarations for clpreauth plugin module implementors.
 *
 * The clpreauth interface has a single supported major version, which is
 * 1.  Major version 1 has a current minor version of 2.  clpreauth modules
 * should define a function named clpreauth_<modulename>_initvt, matching
 * the signature:
 *
 *   krb5_error_code
 *   clpreauth_modname_initvt(krb5_context context, int maj_ver,
 *                            int min_ver, krb5_plugin_vtable vtable);
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     maj_ver == 1: Cast to krb5_clpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* clpreauth mechanism property flags */
    /* Provides a real answer which we can send back to the KDC.  The client
 * assumes that one real answer will be enough. */
    /* Doesn't provide a real answer, but must be given a chance to run before any
 * REAL mechanism callbacks. */
    /* Abstract type for a client request information handle. */
    /* Abstract types for module data and per-request module data. */
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
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
 * Optional: per-plugin initialization/cleanup.  The init function is called by
 * libkrb5 when the plugin is loaded, and the fini function is called before
 * the plugin is unloaded.  These may be called multiple times in case the
 * plugin is used in multiple contexts.  The returned context lives the
 * lifetime of the krb5_context.
 */
    /*
 * Optional (mandatory before MIT krb5 1.12): pa_type will be a member of the
 * vtable's pa_type_list.  Return PA_REAL if pa_type is a real
 * preauthentication type or PA_INFO if it is an informational type.  If this
 * function is not defined in 1.12 or later, all pa_type values advertised by
 * the module will be assumed to be real.
 */
    /*
 * Optional: per-request initialization/cleanup.  The request_init function is
 * called when beginning to process a get_init_creds request and the
 * request_fini function is called when processing of the request is complete.
 * This is optional.  It may be called multiple times in the lifetime of a
 * krb5_context.
 */
    /*
 * Optional: process server-supplied data in pa_data and set responder
 * questions.
 *
 * encoded_previous_request may be NULL if there has been no previous request
 * in the AS exchange.
 */
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
    #[c2rust::src_loc = "300:1"]
    pub type krb5_clpreauth_vtable = *mut krb5_clpreauth_vtable_st;
    use super::krb5_h::{krb5_enctype, krb5_context, krb5_keyblock,
                        krb5_error_code, krb5_boolean, krb5_timestamp,
                        krb5_int32, krb5_preauthtype, krb5_get_init_creds_opt,
                        krb5_kdc_req, krb5_data, krb5_pa_data,
                        krb5_prompter_fct, krb5_error};
    extern "C" {
        #[c2rust::src_loc = "75:16"]
        pub type krb5_clpreauth_rock_st;
        #[c2rust::src_loc = "78:16"]
        pub type krb5_clpreauth_moddata_st;
        #[c2rust::src_loc = "79:16"]
        pub type krb5_clpreauth_modreq_st;
    }
    /* Mandatory: name of module. */
    /* Mandatory: pointer to zero-terminated list of pa_types which this module
     * can provide services for. */
    /* Optional: pointer to zero-terminated list of enc_types which this module
     * claims to add support for. */
    /* Minor version 1 ends here. */
    /* Minor version 2 ends here. */
    /* KRB5_CLPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkcs11.h:33"]
pub mod pkcs11_h {
    #[c2rust::src_loc = "208:1"]
    pub type CK_SLOT_ID = libc::c_ulong;
    /* PKCS11_H */
    /* System dependencies.  */
    /* CRYPTOKI_COMPAT */
    /* Delete the helper macros defined at the top of the file.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit.h:33"]
pub mod pkinit_h {
    /*
 * notes about crypto contexts:
 *
 * the basic idea is that there are crypto contexts that live at
 * both the plugin level and request level. the identity context (that
 * keeps info about your own certs and such) is separate because
 * it is needed at different levels for the kdc and and the client.
 * (the kdc's identity is at the plugin level, the client's identity
 * information could change per-request.)
 * the identity context is meant to have the entity's cert,
 * a list of trusted and intermediate cas, a list of crls, and any
 * pkcs11 information.  the req context is meant to have the
 * received certificate and the DH related information. the plugin
 * context is meant to have global crypto information, i.e., OIDs
 * and constant DH parameter information.
 */
    /*
 * plugin crypto context should keep plugin common information,
 * eg., OIDs, known DHparams
 */
    #[c2rust::src_loc = "128:1"]
    pub type pkinit_plg_crypto_context = *mut _pkinit_plg_crypto_context;
    /*
 * request crypto context should keep reqyest common information,
 * eg., received credentials, DH parameters of this request
 */
    #[c2rust::src_loc = "134:1"]
    pub type pkinit_req_crypto_context = *mut _pkinit_req_crypto_context;
    /*
 * identity context should keep information about credentials
 * for the request, eg., my credentials, trusted ca certs,
 * intermediate ca certs, crls, pkcs11 info
 */
    #[c2rust::src_loc = "141:1"]
    pub type pkinit_identity_crypto_context
        =
        *mut _pkinit_identity_crypto_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:16"]
    pub struct _pkinit_plg_opts {
        pub require_eku: libc::c_int,
        pub accept_secondary_eku: libc::c_int,
        pub allow_upn: libc::c_int,
        pub dh_or_rsa: libc::c_int,
        pub require_crl_checking: libc::c_int,
        pub require_freshness: libc::c_int,
        pub disable_freshness: libc::c_int,
        pub dh_min_bits: libc::c_int,
    }
    /*
 * this structure keeps information about the config options
 */
    #[c2rust::src_loc = "146:1"]
    pub type pkinit_plg_opts = _pkinit_plg_opts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "160:16"]
    pub struct _pkinit_req_opts {
        pub require_eku: libc::c_int,
        pub accept_secondary_eku: libc::c_int,
        pub allow_upn: libc::c_int,
        pub dh_or_rsa: libc::c_int,
        pub require_crl_checking: libc::c_int,
        pub dh_size: libc::c_int,
        pub require_hostname_match: libc::c_int,
        pub disable_freshness: libc::c_int,
    }
    /* require EKU checking (default is true) */
    /* accept secondary EKU (default is false) */
    /* allow UPN-SAN instead of pkinit-SAN */
    /* selects DH or RSA based pkinit */
    /* require CRL for a CA (default is false) */
    /* require freshness token (default is false) */
    /* disable freshness token on client for testing */
    /* minimum DH modulus size allowed */
    /*
 * this structure keeps options used for a given request
 */
    #[c2rust::src_loc = "160:1"]
    pub type pkinit_req_opts = _pkinit_req_opts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "175:16"]
    pub struct _pkinit_identity_opts {
        pub identity: *mut libc::c_char,
        pub identity_alt: *mut *mut libc::c_char,
        pub anchors: *mut *mut libc::c_char,
        pub intermediates: *mut *mut libc::c_char,
        pub crls: *mut *mut libc::c_char,
        pub idtype: libc::c_int,
        pub cert_filename: *mut libc::c_char,
        pub key_filename: *mut libc::c_char,
        pub p11_module_name: *mut libc::c_char,
        pub slotid: CK_SLOT_ID,
        pub token_label: *mut libc::c_char,
        pub cert_id_string: *mut libc::c_char,
        pub cert_label: *mut libc::c_char,
    }
    /* initial request DH modulus size (default=1024) */
    /*
 * information about identity from config file or command line
 */
    #[c2rust::src_loc = "175:1"]
    pub type pkinit_identity_opts = _pkinit_identity_opts;
    /*
 * Client's plugin context
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "197:8"]
    pub struct _pkinit_context {
        pub magic: libc::c_int,
        pub cryptoctx: pkinit_plg_crypto_context,
        pub opts: *mut pkinit_plg_opts,
        pub idopts: *mut pkinit_identity_opts,
    }
    #[c2rust::src_loc = "203:1"]
    pub type pkinit_context = *mut _pkinit_context;
    /*
 * Client's per-request context
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:8"]
    pub struct _pkinit_req_context {
        pub magic: libc::c_uint,
        pub cryptoctx: pkinit_req_crypto_context,
        pub opts: *mut pkinit_req_opts,
        pub idctx: pkinit_identity_crypto_context,
        pub idopts: *mut pkinit_identity_opts,
        pub do_identity_matching: libc::c_int,
        pub pa_type: krb5_preauthtype,
        pub rfc6112_kdc: libc::c_int,
        pub identity_initialized: libc::c_int,
        pub identity_prompted: libc::c_int,
        pub identity_prompt_retval: krb5_error_code,
        pub freshness_token: *mut krb5_data,
    }
    #[c2rust::src_loc = "222:1"]
    pub type pkinit_req_context = *mut _pkinit_req_context;
    /*
 * Client's list of identities for which it needs PINs or passwords
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "309:8"]
    pub struct _pkinit_deferred_id {
        pub magic: libc::c_int,
        pub identity: *mut libc::c_char,
        pub ck_flags: libc::c_ulong,
        pub password: *mut libc::c_char,
    }
    #[c2rust::src_loc = "315:1"]
    pub type pkinit_deferred_id = *mut _pkinit_deferred_id;
    /* Make pkiDebug(fmt,...) print, or not.  */
    /* Still evaluates for side effects.  */
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn pkiDebug(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    }
    use super::pkcs11_h::CK_SLOT_ID;
    use super::krb5_h::{krb5_preauthtype, krb5_error_code, krb5_data,
                        krb5_context, krb5_principal_data, krb5_principal};
    use super::k5_int_h::_krb5_context;
    use super::clpreauth_plugin_h::{krb5_clpreauth_callbacks_st,
                                    krb5_clpreauth_callbacks,
                                    krb5_clpreauth_rock_st,
                                    krb5_clpreauth_rock};
    use super::k5_int_pkinit_h::{krb5_pa_pk_as_req, krb5_reply_key_pack,
                                 krb5_pa_pk_as_rep,
                                 krb5_external_principal_identifier,
                                 krb5_algorithm_identifier,
                                 krb5_kdc_dh_key_info};
    extern "C" {
        #[c2rust::src_loc = "128:16"]
        pub type _pkinit_plg_crypto_context;
        #[c2rust::src_loc = "134:16"]
        pub type _pkinit_req_crypto_context;
        #[c2rust::src_loc = "141:16"]
        pub type _pkinit_identity_crypto_context;
        /* Macros to deal with converting between various data types... */
        #[no_mangle]
        #[c2rust::src_loc = "105:24"]
        pub static dh_oid: krb5_data;
        /*
 * Functions in pkinit_lib.c
 */
        #[no_mangle]
        #[c2rust::src_loc = "254:1"]
        pub fn pkinit_init_req_opts(_: *mut *mut pkinit_req_opts)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "255:1"]
        pub fn pkinit_fini_req_opts(_: *mut pkinit_req_opts);
        #[no_mangle]
        #[c2rust::src_loc = "257:1"]
        pub fn pkinit_init_plg_opts(_: *mut *mut pkinit_plg_opts)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "258:1"]
        pub fn pkinit_fini_plg_opts(_: *mut pkinit_plg_opts);
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn pkinit_init_identity_opts(idopts:
                                             *mut *mut pkinit_identity_opts)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "261:1"]
        pub fn pkinit_fini_identity_opts(idopts: *mut pkinit_identity_opts);
        #[no_mangle]
        #[c2rust::src_loc = "262:1"]
        pub fn pkinit_dup_identity_opts(src_opts: *mut pkinit_identity_opts,
                                        dest_opts:
                                            *mut *mut pkinit_identity_opts)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "271:1"]
        pub fn pkinit_identity_initialize(context: krb5_context,
                                          plg_cryptoctx:
                                              pkinit_plg_crypto_context,
                                          req_cryptoctx:
                                              pkinit_req_crypto_context,
                                          idopts: *mut pkinit_identity_opts,
                                          id_cryptoctx:
                                              pkinit_identity_crypto_context,
                                          cb: krb5_clpreauth_callbacks,
                                          rock: krb5_clpreauth_rock,
                                          princ: krb5_principal)
         -> krb5_error_code;
        /* IN (optional) */
        #[no_mangle]
        #[c2rust::src_loc = "281:1"]
        pub fn pkinit_identity_prompt(context: krb5_context,
                                      plg_cryptoctx:
                                          pkinit_plg_crypto_context,
                                      req_cryptoctx:
                                          pkinit_req_crypto_context,
                                      idopts: *mut pkinit_identity_opts,
                                      id_cryptoctx:
                                          pkinit_identity_crypto_context,
                                      cb: krb5_clpreauth_callbacks,
                                      rock: krb5_clpreauth_rock,
                                      do_matching: libc::c_int,
                                      princ: krb5_principal)
         -> krb5_error_code;
        /*
 * initialization and free functions
 */
        #[no_mangle]
        #[c2rust::src_loc = "329:1"]
        pub fn init_krb5_pa_pk_as_req(in_0: *mut *mut krb5_pa_pk_as_req);
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn free_krb5_pa_pk_as_req(in_0: *mut *mut krb5_pa_pk_as_req);
        #[no_mangle]
        #[c2rust::src_loc = "336:1"]
        pub fn free_krb5_reply_key_pack(in_0: *mut *mut krb5_reply_key_pack);
        #[no_mangle]
        #[c2rust::src_loc = "338:1"]
        pub fn free_krb5_pa_pk_as_rep(in_0: *mut *mut krb5_pa_pk_as_rep);
        #[no_mangle]
        #[c2rust::src_loc = "339:1"]
        pub fn free_krb5_external_principal_identifier(in_0:
                                                           *mut *mut *mut krb5_external_principal_identifier);
        #[no_mangle]
        #[c2rust::src_loc = "340:1"]
        pub fn free_krb5_algorithm_identifiers(in_0:
                                                   *mut *mut *mut krb5_algorithm_identifier);
        #[no_mangle]
        #[c2rust::src_loc = "342:1"]
        pub fn free_krb5_kdc_dh_key_info(in_0:
                                             *mut *mut krb5_kdc_dh_key_info);
        #[no_mangle]
        #[c2rust::src_loc = "364:1"]
        pub fn pkinit_libdefault_strings(context: krb5_context,
                                         realm: *const krb5_data,
                                         option: *const libc::c_char,
                                         ret_value:
                                             *mut *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "367:1"]
        pub fn pkinit_libdefault_string(context: krb5_context,
                                        realm: *const krb5_data,
                                        option: *const libc::c_char,
                                        ret_value: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "370:1"]
        pub fn pkinit_libdefault_boolean(context: krb5_context,
                                         realm: *const krb5_data,
                                         option: *const libc::c_char,
                                         default_value: libc::c_int,
                                         ret_value: *mut libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "373:1"]
        pub fn pkinit_libdefault_integer(context: krb5_context,
                                         realm: *const krb5_data,
                                         option: *const libc::c_char,
                                         default_value: libc::c_int,
                                         ret_value: *mut libc::c_int)
         -> krb5_error_code;
    }
    /* _PKINIT_H */
    /*
 * Now get crypto function declarations
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_crypto.h:33"]
pub mod pkinit_crypto_h {
    /*
 * COPYRIGHT (C) 2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * This header defines the cryptographic interface
 */
    /*
 * these describe the CMS message types
 */
    #[c2rust::src_loc = "47:1"]
    pub type cms_msg_types = libc::c_uint;
    #[c2rust::src_loc = "50:5"]
    pub const CMS_ENVEL_SERVER: cms_msg_types = 2;
    #[c2rust::src_loc = "49:5"]
    pub const CMS_SIGN_SERVER: cms_msg_types = 1;
    #[c2rust::src_loc = "48:5"]
    pub const CMS_SIGN_CLIENT: cms_msg_types = 0;
    use super::pkinit_h::{pkinit_plg_crypto_context,
                          _pkinit_plg_crypto_context,
                          pkinit_req_crypto_context,
                          _pkinit_req_crypto_context,
                          pkinit_identity_crypto_context,
                          _pkinit_identity_crypto_context,
                          pkinit_deferred_id};
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_preauthtype,
                        krb5_principal, krb5_enctype, krb5_keyblock,
                        krb5_principal_data, krb5_prompter_fct, krb5_data,
                        krb5_const_principal};
    use super::k5_int_h::_krb5_context;
    use super::k5_int_pkinit_h::{krb5_algorithm_identifier,
                                 krb5_external_principal_identifier};
    extern "C" {
        /*
 * Functions to initialize and cleanup crypto contexts
 */
        #[no_mangle]
        #[c2rust::src_loc = "106:1"]
        pub fn pkinit_init_plg_crypto(_: *mut pkinit_plg_crypto_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "107:1"]
        pub fn pkinit_fini_plg_crypto(_: pkinit_plg_crypto_context);
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn pkinit_init_req_crypto(_: *mut pkinit_req_crypto_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn pkinit_fini_req_crypto(_: pkinit_req_crypto_context);
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn pkinit_init_identity_crypto(_:
                                               *mut pkinit_identity_crypto_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "113:1"]
        pub fn pkinit_fini_identity_crypto(_: pkinit_identity_crypto_context);
        /* *Create a pkinit ContentInfo*/
        #[no_mangle]
        #[c2rust::src_loc = "115:1"]
        pub fn cms_contentinfo_create(context: krb5_context,
                                      plg_cryptoctx:
                                          pkinit_plg_crypto_context,
                                      req_cryptoctx:
                                          pkinit_req_crypto_context,
                                      id_cryptoctx:
                                          pkinit_identity_crypto_context,
                                      cms_msg_type: libc::c_int,
                                      in_data: *mut libc::c_uchar,
                                      in_length: libc::c_uint,
                                      out_data: *mut *mut libc::c_uchar,
                                      out_data_len: *mut libc::c_uint)
         -> krb5_error_code;
        /*
 * this function creates a CMS message where eContentType is SignedData
 */
        #[no_mangle]
        #[c2rust::src_loc = "127:1"]
        pub fn cms_signeddata_create(context: krb5_context,
                                     plg_cryptoctx: pkinit_plg_crypto_context,
                                     req_cryptoctx: pkinit_req_crypto_context,
                                     id_cryptoctx:
                                         pkinit_identity_crypto_context,
                                     cms_msg_type: libc::c_int,
                                     include_certchain: libc::c_int,
                                     auth_pack: *mut libc::c_uchar,
                                     auth_pack_len: libc::c_uint,
                                     signed_data: *mut *mut libc::c_uchar,
                                     signed_data_len: *mut libc::c_uint)
         -> krb5_error_code;
        /* OUT
		    receives length of signed_data */
        /*
 * this function verifies a CMS message where eContentType is SignedData
 */
        #[no_mangle]
        #[c2rust::src_loc = "153:1"]
        pub fn cms_signeddata_verify(context: krb5_context,
                                     plg_cryptoctx: pkinit_plg_crypto_context,
                                     req_cryptoctx: pkinit_req_crypto_context,
                                     id_cryptoctx:
                                         pkinit_identity_crypto_context,
                                     cms_msg_type: libc::c_int,
                                     require_crl_checking: libc::c_int,
                                     signed_data: *mut libc::c_uchar,
                                     signed_data_len: libc::c_uint,
                                     auth_pack: *mut *mut libc::c_uchar,
                                     auth_pack_len: *mut libc::c_uint,
                                     authz_data: *mut *mut libc::c_uchar,
                                     authz_data_len: *mut libc::c_uint,
                                     is_signed: *mut libc::c_int)
         -> krb5_error_code;
        /* OUT
		    receives length of envel_data */
        /*
 * this function creates a CMS message where eContentType is EnvelopedData
 */
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn cms_envelopeddata_verify(context: krb5_context,
                                        plg_cryptoctx:
                                            pkinit_plg_crypto_context,
                                        req_cryptoctx:
                                            pkinit_req_crypto_context,
                                        id_cryptoctx:
                                            pkinit_identity_crypto_context,
                                        pa_type: krb5_preauthtype,
                                        require_crl_checking: libc::c_int,
                                        envel_data: *mut libc::c_uchar,
                                        envel_data_len: libc::c_uint,
                                        signed_data: *mut *mut libc::c_uchar,
                                        signed_data_len: *mut libc::c_uint)
         -> krb5_error_code;
        /* OUT */
        /*
 * this function returns SAN information found in the
 * received certificate.  at least one of pkinit_sans,
 * upn_sans, or kdc_hostnames must be non-NULL.
 */
        #[no_mangle]
        #[c2rust::src_loc = "243:1"]
        pub fn crypto_retrieve_cert_sans(context: krb5_context,
                                         plg_cryptoctx:
                                             pkinit_plg_crypto_context,
                                         req_cryptoctx:
                                             pkinit_req_crypto_context,
                                         id_cryptoctx:
                                             pkinit_identity_crypto_context,
                                         pkinit_sans:
                                             *mut *mut krb5_principal,
                                         upn_sans:
                                             *mut *mut *mut libc::c_char,
                                         kdc_hostname:
                                             *mut *mut *mut libc::c_uchar)
         -> krb5_error_code;
        /* OUT
		    if non-NULL, a null-terminated array of
		    dNSName (hostname) SAN values found in the
		    certificate are returned */
        /*
 * this function checks for acceptable key usage values
 * in the received certificate.
 *
 * when checking a received kdc certificate, it looks for
 * the kpKdc key usage.  if allow_secondary_usage is
 * non-zero, it will also accept kpServerAuth.
 *
 * when checking a received user certificate, it looks for
 * kpClientAuth key usage.  if allow_secondary_usage is
 * non-zero, it will also accept id-ms-sc-logon EKU.
 *
 * this function must also assert that the digitalSignature
 * key usage is consistent.
 */
        #[no_mangle]
        #[c2rust::src_loc = "276:1"]
        pub fn crypto_check_cert_eku(context: krb5_context,
                                     plg_cryptoctx: pkinit_plg_crypto_context,
                                     req_cryptoctx: pkinit_req_crypto_context,
                                     id_cryptoctx:
                                         pkinit_identity_crypto_context,
                                     checking_kdc_cert: libc::c_int,
                                     allow_secondary_usage: libc::c_int,
                                     eku_valid: *mut libc::c_int)
         -> krb5_error_code;
        /* OUT
		    receives non-zero if an acceptable EKU was found */
        /*
 * this functions takes in generated DH secret key and converts
 * it in to a kerberos session key. it takes into the account the
 * enc type and then follows the procedure specified in the RFC p 22.
 */
        #[no_mangle]
        #[c2rust::src_loc = "296:1"]
        pub fn pkinit_octetstring2key(context: krb5_context,
                                      etype: krb5_enctype,
                                      key: *mut libc::c_uchar,
                                      key_len: libc::c_uint,
                                      krb5key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* OUT
		    receives kerberos session key */
        /*
 * this function implements clients first part of the DH protocol.
 * client selects its DH parameters and pub key
 */
        #[no_mangle]
        #[c2rust::src_loc = "311:1"]
        pub fn client_create_dh(context: krb5_context,
                                plg_cryptoctx: pkinit_plg_crypto_context,
                                req_cryptoctx: pkinit_req_crypto_context,
                                id_cryptoctx: pkinit_identity_crypto_context,
                                dh_size: libc::c_int,
                                dh_params_out: *mut *mut libc::c_uchar,
                                dh_params_len_out: *mut libc::c_uint,
                                dh_pubkey_out: *mut *mut libc::c_uchar,
                                dh_pubkey_len_out: *mut libc::c_uint)
         -> krb5_error_code;
        /* OUT
		    receives length of DH pub key */
        /*
 * this function completes client's the DH protocol. client
 * processes received DH pub key from the KDC and computes
 * the DH secret key
 */
        #[no_mangle]
        #[c2rust::src_loc = "332:1"]
        pub fn client_process_dh(context: krb5_context,
                                 plg_cryptoctx: pkinit_plg_crypto_context,
                                 req_cryptoctx: pkinit_req_crypto_context,
                                 id_cryptoctx: pkinit_identity_crypto_context,
                                 dh_pubkey: *mut libc::c_uchar,
                                 dh_pubkey_len: libc::c_uint,
                                 client_key_out: *mut *mut libc::c_uchar,
                                 client_key_len_out: *mut libc::c_uint)
         -> krb5_error_code;
        /* OUT
		    receives length of DH secret key */
        /*
 * this functions takes in crypto specific representation of
 * supportedCMSTypes and creates a list of
 * krb5_algorithm_identifier
 */
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn create_krb5_supportedCMSTypes(context: krb5_context,
                                             plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             supportedCMSTypes:
                                                 *mut *mut *mut krb5_algorithm_identifier)
         -> krb5_error_code;
        /* OUT */
        /*
 * this functions takes in crypto specific representation of
 * trustedCertifiers and creates a list of
 * krb5_external_principal_identifier
 */
        #[no_mangle]
        #[c2rust::src_loc = "400:1"]
        pub fn create_krb5_trustedCertifiers(context: krb5_context,
                                             plg_cryptoctx:
                                                 pkinit_plg_crypto_context,
                                             req_cryptoctx:
                                                 pkinit_req_crypto_context,
                                             id_cryptoctx:
                                                 pkinit_identity_crypto_context,
                                             trustedCertifiers:
                                                 *mut *mut *mut krb5_external_principal_identifier)
         -> krb5_error_code;
        /* OUT */
        /*
 * this functions takes in crypto specific representation of the
 * KDC's certificate and creates a DER encoded kdcPKId
 */
        #[no_mangle]
        #[c2rust::src_loc = "411:1"]
        pub fn create_issuerAndSerial(context: krb5_context,
                                      plg_cryptoctx:
                                          pkinit_plg_crypto_context,
                                      req_cryptoctx:
                                          pkinit_req_crypto_context,
                                      id_cryptoctx:
                                          pkinit_identity_crypto_context,
                                      kdcId_buf: *mut *mut libc::c_uchar,
                                      kdcId_len: *mut libc::c_uint)
         -> krb5_error_code;
        /* OUT
		    receives length of encoded kdcPKId */
        /*
 * These functions manipulate the deferred-identities list in the identity
 * context, which is opaque outside of the crypto-specific bits.
 */
        #[no_mangle]
        #[c2rust::src_loc = "425:1"]
        pub fn crypto_get_deferred_ids(context: krb5_context,
                                       id_cryptoctx:
                                           pkinit_identity_crypto_context)
         -> *const pkinit_deferred_id;
        #[no_mangle]
        #[c2rust::src_loc = "427:1"]
        pub fn crypto_set_deferred_id(context: krb5_context,
                                      id_cryptoctx:
                                          pkinit_identity_crypto_context,
                                      identity: *const libc::c_char,
                                      password: *const libc::c_char)
         -> krb5_error_code;
        /* IN
		    defines the location (filename, directory name, etc) */
        /*
 * on the client, obtain the kdc's certificate to include
 * in a request
 */
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn pkinit_get_kdc_cert(context: krb5_context,
                                   plg_cryptoctx: pkinit_plg_crypto_context,
                                   req_cryptoctx: pkinit_req_crypto_context,
                                   id_cryptoctx:
                                       pkinit_identity_crypto_context,
                                   princ: krb5_principal) -> krb5_error_code;
        /* OUT */
        /*
 * this function processes edata that contains TD-DH-PARAMETERS.
 * the client processes the received acceptable by KDC DH
 * parameters and picks the first acceptable to it. it matches
 * them against the known DH parameters.
 */
        #[no_mangle]
        #[c2rust::src_loc = "544:1"]
        pub fn pkinit_process_td_dh_params(context: krb5_context,
                                           plg_cryptoctx:
                                               pkinit_plg_crypto_context,
                                           req_cryptoctx:
                                               pkinit_req_crypto_context,
                                           id_cryptoctx:
                                               pkinit_identity_crypto_context,
                                           algId:
                                               *mut *mut krb5_algorithm_identifier,
                                           new_dh_size: *mut libc::c_int)
         -> krb5_error_code;
        /* OUT */
        /*
 * this function processes edata that contains either
 * TD-TRUSTED-CERTIFICATES or TD-INVALID-CERTIFICATES.
 * current implementation only decodes the received message
 * but does not act on it
 */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn pkinit_process_td_trusted_certifiers(context: krb5_context,
                                                    plg_cryptoctx:
                                                        pkinit_plg_crypto_context,
                                                    req_cryptoctx:
                                                        pkinit_req_crypto_context,
                                                    id_cryptoctx:
                                                        pkinit_identity_crypto_context,
                                                    trustedCertifiers:
                                                        *mut *mut krb5_external_principal_identifier,
                                                    td_type: libc::c_int)
         -> krb5_error_code;
        /* OUT
		    1 if kdcPKId matches, otherwise 0 */
        #[no_mangle]
        #[c2rust::src_loc = "603:1"]
        pub fn pkinit_identity_set_prompter(id_cryptoctx:
                                                pkinit_identity_crypto_context,
                                            prompter: krb5_prompter_fct,
                                            prompter_data: *mut libc::c_void)
         -> krb5_error_code;
        /* IN */
        #[no_mangle]
        #[c2rust::src_loc = "608:1"]
        pub fn pkinit_alg_agility_kdf(context: krb5_context,
                                      secret: *mut krb5_data,
                                      alg_oid: *mut krb5_data,
                                      party_u_info: krb5_const_principal,
                                      party_v_info: krb5_const_principal,
                                      enctype: krb5_enctype,
                                      as_req: *mut krb5_data,
                                      pk_as_rep: *mut krb5_data,
                                      key_block: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * An ordered set of OIDs, stored as krb5_data, of KDF algorithms
 * supported by this implementation. The order of this array controls
 * the order in which the server will pick.
 */
        #[no_mangle]
        #[c2rust::src_loc = "630:32"]
        pub static supported_kdf_alg_ids: [*const krb5_data; 0];
    }
    /* _PKINIT_CRYPTO_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-json.h:34"]
pub mod k5_json_h {
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
 * Object
 */
    #[c2rust::src_loc = "160:1"]
    pub type k5_json_object = *mut k5_json_object_st;
    #[c2rust::src_loc = "161:1"]
    pub type k5_json_object_iterator_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const libc::c_char, _: k5_json_value)
                   -> ()>;
    /*
 * String
 */
    #[c2rust::src_loc = "186:1"]
    pub type k5_json_string = *mut k5_json_string_st;
    /*
 * Number
 */
    #[c2rust::src_loc = "206:1"]
    pub type k5_json_number = *mut k5_json_number_st;
    extern "C" {
        #[c2rust::src_loc = "160:16"]
        pub type k5_json_object_st;
        #[c2rust::src_loc = "186:16"]
        pub type k5_json_string_st;
        #[c2rust::src_loc = "206:16"]
        pub type k5_json_number_st;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn k5_json_get_tid(val: k5_json_value) -> k5_json_tid;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn k5_json_release(val: k5_json_value);
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_json_object_create(val_out: *mut k5_json_object)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "165:1"]
        pub fn k5_json_object_iterate(obj: k5_json_object,
                                      func: k5_json_object_iterator_fn,
                                      arg: *mut libc::c_void);
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
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn k5_json_string_utf8(string: k5_json_string)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn k5_json_number_create(number: libc::c_longlong,
                                     val_out: *mut k5_json_number)
         -> libc::c_int;
        /*
 * JSON encoding and decoding
 */
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn k5_json_encode(val: k5_json_value,
                              json_out: *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "216:1"]
        pub fn k5_json_decode(str: *const libc::c_char,
                              val_out: *mut k5_json_value) -> libc::c_int;
    }
    /* K5_JSON_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:32"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        /*
 * Return 0 if the n-byte memory regions p1 and p2 are equal, and nonzero if
 * they are not.  The function is intended to take the same amount of time
 * regardless of how many bytes of p1 and p2 are equal.
 */
        #[no_mangle]
        #[c2rust::src_loc = "1058:1"]
        pub fn k5_bcmp(p1: *const libc::c_void, p2: *const libc::c_void,
                       n: size_t) -> libc::c_int;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
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
#[c2rust::header_src = "/usr/include/strings.h:32"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
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
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit_accessor.h:33"]
pub mod pkinit_accessor_h {
    use super::krb5_h::{krb5_error_code, krb5_data, krb5_kdc_req};
    use super::k5_int_pkinit_h::{krb5_auth_pack,
                                 krb5_external_principal_identifier,
                                 krb5_algorithm_identifier,
                                 krb5_reply_key_pack, krb5_pa_pk_as_req,
                                 krb5_pa_pk_as_rep, krb5_kdc_dh_key_info};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub static mut k5int_encode_krb5_auth_pack:
                   Option<unsafe extern "C" fn(_: *const krb5_auth_pack,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "67:26"]
        pub static mut k5int_encode_krb5_kdc_req_body:
                   Option<unsafe extern "C" fn(_: *const krb5_kdc_req,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "64:26"]
        pub static mut k5int_decode_krb5_td_trusted_certifiers:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _:
                                                   *mut *mut *mut krb5_external_principal_identifier)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "59:26"]
        pub static mut k5int_decode_krb5_td_dh_parameters:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _:
                                                   *mut *mut *mut krb5_algorithm_identifier)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub static mut k5int_decode_krb5_reply_key_pack:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _:
                                                   *mut *mut krb5_reply_key_pack)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub static mut k5int_encode_krb5_pa_pk_as_req:
                   Option<unsafe extern "C" fn(_: *const krb5_pa_pk_as_req,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub static mut k5int_decode_krb5_pa_pk_as_rep:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _: *mut *mut krb5_pa_pk_as_rep)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub static mut k5int_decode_krb5_kdc_dh_key_info:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _:
                                                   *mut *mut krb5_kdc_dh_key_info)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn pkinit_accessor_init() -> krb5_error_code;
    }
    /* _PKINIT_ACCESSOR_H */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_cksumtype, krb5_authdatatype, krb5_keyusage,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_error, krb5_error,
                       _krb5_prompt, krb5_prompt, krb5_prompter_fct,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, krb5_anonymous_principal,
                       krb5_c_make_checksum, krb5_principal_compare,
                       krb5_principal_compare_any_realm,
                       krb5_copy_keyblock_contents, krb5_copy_data,
                       krb5_build_principal_ext, krb5_free_principal,
                       krb5_free_checksum_contents,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, make_data, empty_data, k5calloc,
                         k5alloc, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5_free_pa_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_free_list};
pub use self::com_err_h::{errcode_t, error_message};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_vtable_st};
pub use self::k5_int_pkinit_h::{_krb5_pk_authenticator, krb5_pk_authenticator,
                                _krb5_algorithm_identifier,
                                krb5_algorithm_identifier,
                                _krb5_subject_pk_info, krb5_subject_pk_info,
                                _krb5_auth_pack, krb5_auth_pack,
                                _krb5_external_principal_identifier,
                                krb5_external_principal_identifier,
                                _krb5_pa_pk_as_req, krb5_pa_pk_as_req,
                                _krb5_dh_rep_info, krb5_dh_rep_info,
                                _krb5_kdc_dh_key_info, krb5_kdc_dh_key_info,
                                _krb5_reply_key_pack, krb5_reply_key_pack,
                                _krb5_pa_pk_as_rep, krb5_pa_pk_as_rep_choices,
                                krb5_pa_pk_as_rep_selection,
                                choice_pa_pk_as_rep_encKeyPack,
                                choice_pa_pk_as_rep_dhInfo,
                                choice_pa_pk_as_rep_UNKNOWN,
                                krb5_pa_pk_as_rep};
pub use self::clpreauth_plugin_h::{krb5_clpreauth_rock,
                                   krb5_clpreauth_moddata,
                                   krb5_clpreauth_modreq,
                                   krb5_clpreauth_callbacks_st,
                                   krb5_clpreauth_callbacks,
                                   krb5_clpreauth_init_fn,
                                   krb5_clpreauth_fini_fn,
                                   krb5_clpreauth_get_flags_fn,
                                   krb5_clpreauth_request_init_fn,
                                   krb5_clpreauth_request_fini_fn,
                                   krb5_clpreauth_prep_questions_fn,
                                   krb5_clpreauth_process_fn,
                                   krb5_clpreauth_tryagain_fn,
                                   krb5_clpreauth_supply_gic_opts_fn,
                                   krb5_clpreauth_vtable_st,
                                   krb5_clpreauth_vtable,
                                   krb5_clpreauth_rock_st,
                                   krb5_clpreauth_moddata_st,
                                   krb5_clpreauth_modreq_st};
pub use self::pkcs11_h::CK_SLOT_ID;
pub use self::pkinit_h::{pkinit_plg_crypto_context, pkinit_req_crypto_context,
                         pkinit_identity_crypto_context, _pkinit_plg_opts,
                         pkinit_plg_opts, _pkinit_req_opts, pkinit_req_opts,
                         _pkinit_identity_opts, pkinit_identity_opts,
                         _pkinit_context, pkinit_context, _pkinit_req_context,
                         pkinit_req_context, _pkinit_deferred_id,
                         pkinit_deferred_id, pkiDebug,
                         _pkinit_plg_crypto_context,
                         _pkinit_req_crypto_context,
                         _pkinit_identity_crypto_context, dh_oid,
                         pkinit_init_req_opts, pkinit_fini_req_opts,
                         pkinit_init_plg_opts, pkinit_fini_plg_opts,
                         pkinit_init_identity_opts, pkinit_fini_identity_opts,
                         pkinit_dup_identity_opts, pkinit_identity_initialize,
                         pkinit_identity_prompt, init_krb5_pa_pk_as_req,
                         free_krb5_pa_pk_as_req, free_krb5_reply_key_pack,
                         free_krb5_pa_pk_as_rep,
                         free_krb5_external_principal_identifier,
                         free_krb5_algorithm_identifiers,
                         free_krb5_kdc_dh_key_info, pkinit_libdefault_strings,
                         pkinit_libdefault_string, pkinit_libdefault_boolean,
                         pkinit_libdefault_integer};
pub use self::pkinit_crypto_h::{cms_msg_types, CMS_ENVEL_SERVER,
                                CMS_SIGN_SERVER, CMS_SIGN_CLIENT,
                                pkinit_init_plg_crypto,
                                pkinit_fini_plg_crypto,
                                pkinit_init_req_crypto,
                                pkinit_fini_req_crypto,
                                pkinit_init_identity_crypto,
                                pkinit_fini_identity_crypto,
                                cms_contentinfo_create, cms_signeddata_create,
                                cms_signeddata_verify,
                                cms_envelopeddata_verify,
                                crypto_retrieve_cert_sans,
                                crypto_check_cert_eku, pkinit_octetstring2key,
                                client_create_dh, client_process_dh,
                                create_krb5_supportedCMSTypes,
                                create_krb5_trustedCertifiers,
                                create_issuerAndSerial,
                                crypto_get_deferred_ids,
                                crypto_set_deferred_id, pkinit_get_kdc_cert,
                                pkinit_process_td_dh_params,
                                pkinit_process_td_trusted_certifiers,
                                pkinit_identity_set_prompter,
                                pkinit_alg_agility_kdf,
                                supported_kdf_alg_ids};
pub use self::k5_json_h::{k5_json_value, k5_json_tid, k5_json_object,
                          k5_json_object_iterator_fn, k5_json_string,
                          k5_json_number, k5_json_object_st,
                          k5_json_string_st, k5_json_number_st,
                          k5_json_get_tid, k5_json_release,
                          k5_json_object_create, k5_json_object_iterate,
                          k5_json_object_set, k5_json_string_utf8,
                          k5_json_number_create, k5_json_encode,
                          k5_json_decode};
use self::k5_platform_h::k5_bcmp;
use self::stdlib_h::{free, realloc, calloc, malloc};
use self::strings_h::strcasecmp;
use self::string_h::{strlen, strdup, strcmp, memset};
use self::assert_h::__assert_fail;
use self::k5_trace_h::krb5int_trace;
use self::pkinit_accessor_h::{k5int_encode_krb5_auth_pack,
                              k5int_encode_krb5_kdc_req_body,
                              k5int_decode_krb5_td_trusted_certifiers,
                              k5int_decode_krb5_td_dh_parameters,
                              k5int_decode_krb5_reply_key_pack,
                              k5int_encode_krb5_pa_pk_as_req,
                              k5int_decode_krb5_pa_pk_as_rep,
                              k5int_decode_krb5_kdc_dh_key_info,
                              pkinit_accessor_init};
/*
 * Parse data supplied by the application's responder callback, saving off any
 * PINs and passwords for identities which we noted needed them.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "985:8"]
pub struct save_one_password_data {
    pub context: krb5_context,
    pub modreq: krb5_clpreauth_modreq,
    pub caller: *const libc::c_char,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
/* *
 * Return true if we should use ContentInfo rather than SignedData. This
 * happens if we are talking to what might be an old (pre-6112) MIT KDC and
 * we're using anonymous.
 */
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn use_content_info(mut context: krb5_context,
                                      mut req: pkinit_req_context,
                                      mut client: krb5_principal)
 -> libc::c_int {
    if (*req).rfc6112_kdc != 0 { return 0 as libc::c_int }
    if krb5_principal_compare_any_realm(context,
                                        client as krb5_const_principal,
                                        krb5_anonymous_principal()) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn pa_pkinit_gen_req(mut context: krb5_context,
                                       mut plgctx: pkinit_context,
                                       mut reqctx: pkinit_req_context,
                                       mut cb: krb5_clpreauth_callbacks,
                                       mut rock: krb5_clpreauth_rock,
                                       mut request: *mut krb5_kdc_req,
                                       mut pa_type: krb5_preauthtype,
                                       mut out_padata:
                                           *mut *mut *mut krb5_pa_data,
                                       mut prompter: krb5_prompter_fct,
                                       mut prompter_data: *mut libc::c_void,
                                       mut gic_opt:
                                           *mut krb5_get_init_creds_opt)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut out_data: *mut krb5_data = 0 as *mut krb5_data;
    let mut ctsec: krb5_timestamp = 0 as libc::c_int;
    let mut cusec: krb5_int32 = 0 as libc::c_int;
    let mut nonce: krb5_ui_4 = 0 as libc::c_int as krb5_ui_4;
    let mut cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut der_req: *mut krb5_data = 0 as *mut krb5_data;
    let mut return_pa_data: *mut *mut krb5_pa_data =
        0 as *mut *mut krb5_pa_data;
    memset(&mut cksum as *mut krb5_checksum as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    (*reqctx).pa_type = pa_type;
    pkiDebug(b"kdc_options = 0x%x  till = %d\n\x00" as *const u8 as
                 *const libc::c_char, (*request).kdc_options,
             (*request).till);
    /* If we don't have a client, we're done */
    if (*request).client.is_null() {
        pkiDebug(b"No request->client; aborting PKINIT\n\x00" as *const u8 as
                     *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    retval =
        pkinit_get_kdc_cert(context, (*plgctx).cryptoctx, (*reqctx).cryptoctx,
                            (*reqctx).idctx, (*request).server);
    if retval != 0 {
        pkiDebug(b"pkinit_get_kdc_cert returned %d\n\x00" as *const u8 as
                     *const libc::c_char, retval);
    } else {
        /* checksum of the encoded KDC-REQ-BODY */
        retval =
            k5int_encode_krb5_kdc_req_body.expect("non-null function pointer")(request,
                                                                               &mut der_req);
        if retval != 0 {
            pkiDebug(b"encode_krb5_kdc_req_body returned %d\n\x00" as
                         *const u8 as *const libc::c_char, retval);
        } else {
            retval =
                krb5_c_make_checksum(context, 0x9 as libc::c_int,
                                     0 as *const krb5_keyblock,
                                     0 as libc::c_int, der_req, &mut cksum);
            if !(retval != 0) {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client computed kdc-req-body checksum {cksum}\x00"
                                      as *const u8 as *const libc::c_char,
                                  &mut cksum as *mut krb5_checksum);
                }
                retval =
                    (*cb).get_preauth_time.expect("non-null function pointer")(context,
                                                                               rock,
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   krb5_boolean,
                                                                               &mut ctsec,
                                                                               &mut cusec);
                if !(retval != 0) {
                    /* XXX PKINIT RFC says that nonce in PKAuthenticator doesn't have be the
     * same as in the AS_REQ. However, if we pick a different nonce, then we
     * need to remember that info when AS_REP is returned. I'm choosing to
     * reuse the AS_REQ nonce.
     */
                    nonce = (*request).nonce as krb5_ui_4;
                    retval =
                        pkinit_as_req_create(context, plgctx, reqctx, ctsec,
                                             cusec, nonce, &mut cksum,
                                             (*request).client,
                                             (*request).server,
                                             &mut out_data);
                    if retval != 0 {
                        pkiDebug(b"error %d on pkinit_as_req_create; aborting PKINIT\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 retval);
                    } else {
                        return_pa_data =
                            k5calloc(2 as libc::c_int as size_t,
                                     ::std::mem::size_of::<*mut krb5_pa_data>()
                                         as libc::c_ulong, &mut retval) as
                                *mut *mut krb5_pa_data;
                        if !return_pa_data.is_null() {
                            let ref mut fresh0 =
                                *return_pa_data.offset(0 as libc::c_int as
                                                           isize);
                            *fresh0 =
                                k5alloc(::std::mem::size_of::<krb5_pa_data>()
                                            as libc::c_ulong, &mut retval) as
                                    *mut krb5_pa_data;
                            if !(*return_pa_data.offset(0 as libc::c_int as
                                                            isize)).is_null()
                               {
                                (**return_pa_data.offset(0 as libc::c_int as
                                                             isize)).magic =
                                    -(1760647406 as libc::c_long) as
                                        krb5_magic;
                                (**return_pa_data.offset(0 as libc::c_int as
                                                             isize)).pa_type =
                                    pa_type;
                                (**return_pa_data.offset(0 as libc::c_int as
                                                             isize)).length =
                                    (*out_data).length;
                                let ref mut fresh1 =
                                    (**return_pa_data.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).contents;
                                *fresh1 = (*out_data).data as *mut krb5_octet;
                                *out_data = empty_data();
                                *out_padata = return_pa_data;
                                return_pa_data = 0 as *mut *mut krb5_pa_data;
                                (*cb).disable_fallback.expect("non-null function pointer")(context,
                                                                                           rock);
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_data(context, der_req);
    krb5_free_checksum_contents(context, &mut cksum);
    krb5_free_data(context, out_data);
    krb5_free_pa_data(context, return_pa_data);
    return retval;
}
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn pkinit_as_req_create(mut context: krb5_context,
                                          mut plgctx: pkinit_context,
                                          mut reqctx: pkinit_req_context,
                                          mut ctsec: krb5_timestamp,
                                          mut cusec: krb5_int32,
                                          mut nonce: krb5_ui_4,
                                          mut cksum: *const krb5_checksum,
                                          mut client: krb5_principal,
                                          mut server: krb5_principal,
                                          mut as_req: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut info: krb5_subject_pk_info =
        krb5_subject_pk_info{algorithm:
                                 krb5_algorithm_identifier{algorithm:
                                                               krb5_data{magic:
                                                                             0,
                                                                         length:
                                                                             0,
                                                                         data:
                                                                             0
                                                                                 as
                                                                                 *mut libc::c_char,},
                                                           parameters:
                                                               krb5_data{magic:
                                                                             0,
                                                                         length:
                                                                             0,
                                                                         data:
                                                                             0
                                                                                 as
                                                                                 *mut libc::c_char,},},
                             subjectPublicKey:
                                 krb5_data{magic: 0,
                                           length: 0,
                                           data: 0 as *mut libc::c_char,},};
    let mut coded_auth_pack: *mut krb5_data = 0 as *mut krb5_data;
    let mut auth_pack: krb5_auth_pack =
        krb5_auth_pack{pkAuthenticator:
                           krb5_pk_authenticator{cusec: 0,
                                                 ctime: 0,
                                                 nonce: 0,
                                                 paChecksum:
                                                     krb5_checksum{magic: 0,
                                                                   checksum_type:
                                                                       0,
                                                                   length: 0,
                                                                   contents:
                                                                       0 as
                                                                           *mut krb5_octet,},
                                                 freshnessToken:
                                                     0 as *mut krb5_data,},
                       clientPublicValue: 0 as *mut krb5_subject_pk_info,
                       supportedCMSTypes:
                           0 as *mut *mut krb5_algorithm_identifier,
                       clientDHNonce:
                           krb5_data{magic: 0,
                                     length: 0,
                                     data: 0 as *mut libc::c_char,},
                       supportedKDFs: 0 as *mut *mut krb5_data,};
    let mut req: *mut krb5_pa_pk_as_req = 0 as *mut krb5_pa_pk_as_req;
    let mut cmstypes: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    let mut protocol: libc::c_int = (*(*reqctx).opts).dh_or_rsa;
    let mut dh_params: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_pubkey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_params_len: libc::c_uint = 0;
    let mut dh_pubkey_len: libc::c_uint = 0;
    pkiDebug(b"pkinit_as_req_create pa_type = %d\n\x00" as *const u8 as
                 *const libc::c_char, (*reqctx).pa_type);
    /* Create the authpack */
    memset(&mut info as *mut krb5_subject_pk_info as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_subject_pk_info>() as libc::c_ulong);
    memset(&mut auth_pack as *mut krb5_auth_pack as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_auth_pack>() as libc::c_ulong);
    auth_pack.pkAuthenticator.ctime = ctsec;
    auth_pack.pkAuthenticator.cusec = cusec;
    auth_pack.pkAuthenticator.nonce = nonce as krb5_int32;
    auth_pack.pkAuthenticator.paChecksum = *cksum;
    if (*(*reqctx).opts).disable_freshness == 0 {
        auth_pack.pkAuthenticator.freshnessToken = (*reqctx).freshness_token
    }
    auth_pack.clientDHNonce.length = 0 as libc::c_int as libc::c_uint;
    auth_pack.clientPublicValue = &mut info;
    auth_pack.supportedKDFs =
        supported_kdf_alg_ids.as_ptr() as *mut *mut krb5_data;
    /* add List of CMS algorithms */
    retval =
        create_krb5_supportedCMSTypes(context, (*plgctx).cryptoctx,
                                      (*reqctx).cryptoctx, (*reqctx).idctx,
                                      &mut cmstypes);
    auth_pack.supportedCMSTypes = cmstypes;
    if !(retval != 0) {
        match protocol {
            1 => {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client making DH request\x00" as
                                      *const u8 as *const libc::c_char);
                }
                pkiDebug(b"as_req: DH key transport algorithm\n\x00" as
                             *const u8 as *const libc::c_char);
                info.algorithm.algorithm = dh_oid;
                /* create client-side DH keys */
                retval =
                    client_create_dh(context, (*plgctx).cryptoctx,
                                     (*reqctx).cryptoctx, (*reqctx).idctx,
                                     (*(*reqctx).opts).dh_size,
                                     &mut dh_params, &mut dh_params_len,
                                     &mut dh_pubkey, &mut dh_pubkey_len);
                if retval != 0 as libc::c_int {
                    pkiDebug(b"failed to create dh parameters\n\x00" as
                                 *const u8 as *const libc::c_char);
                    current_block = 8339236560111139848;
                } else {
                    info.algorithm.parameters =
                        make_data(dh_params as *mut libc::c_void,
                                  dh_params_len);
                    info.subjectPublicKey =
                        make_data(dh_pubkey as *mut libc::c_void,
                                  dh_pubkey_len);
                    current_block = 980989089337379490;
                }
            }
            2 => {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client making RSA request\x00" as
                                      *const u8 as *const libc::c_char);
                }
                pkiDebug(b"as_req: RSA key transport algorithm\n\x00" as
                             *const u8 as *const libc::c_char);
                auth_pack.clientPublicValue = 0 as *mut krb5_subject_pk_info;
                current_block = 980989089337379490;
            }
            _ => {
                pkiDebug(b"as_req: unknown key transport protocol %d\n\x00" as
                             *const u8 as *const libc::c_char, protocol);
                retval = -(1 as libc::c_int);
                current_block = 8339236560111139848;
            }
        }
        match current_block {
            8339236560111139848 => { }
            _ => {
                retval =
                    k5int_encode_krb5_auth_pack.expect("non-null function pointer")(&mut auth_pack,
                                                                                    &mut coded_auth_pack);
                if retval != 0 {
                    pkiDebug(b"failed to encode the AuthPack %d\n\x00" as
                                 *const u8 as *const libc::c_char, retval);
                } else {
                    /* create PKCS7 object from authpack */
                    init_krb5_pa_pk_as_req(&mut req);
                    if req.is_null() {
                        retval = 12 as libc::c_int
                    } else {
                        if use_content_info(context, reqctx, client) != 0 {
                            retval =
                                cms_contentinfo_create(context,
                                                       (*plgctx).cryptoctx,
                                                       (*reqctx).cryptoctx,
                                                       (*reqctx).idctx,
                                                       CMS_SIGN_CLIENT as
                                                           libc::c_int,
                                                       (*coded_auth_pack).data
                                                           as
                                                           *mut libc::c_uchar,
                                                       (*coded_auth_pack).length,
                                                       &mut (*req).signedAuthPack.data
                                                           as
                                                           *mut *mut libc::c_char
                                                           as
                                                           *mut *mut libc::c_uchar,
                                                       &mut (*req).signedAuthPack.length)
                        } else {
                            retval =
                                cms_signeddata_create(context,
                                                      (*plgctx).cryptoctx,
                                                      (*reqctx).cryptoctx,
                                                      (*reqctx).idctx,
                                                      CMS_SIGN_CLIENT as
                                                          libc::c_int,
                                                      1 as libc::c_int,
                                                      (*coded_auth_pack).data
                                                          as
                                                          *mut libc::c_uchar,
                                                      (*coded_auth_pack).length,
                                                      &mut (*req).signedAuthPack.data
                                                          as
                                                          *mut *mut libc::c_char
                                                          as
                                                          *mut *mut libc::c_uchar,
                                                      &mut (*req).signedAuthPack.length)
                        }
                        krb5_free_data(context, coded_auth_pack);
                        if retval != 0 {
                            pkiDebug(b"failed to create pkcs7 signed data\n\x00"
                                         as *const u8 as *const libc::c_char);
                        } else {
                            /* create a list of trusted CAs */
                            retval =
                                create_krb5_trustedCertifiers(context,
                                                              (*plgctx).cryptoctx,
                                                              (*reqctx).cryptoctx,
                                                              (*reqctx).idctx,
                                                              &mut (*req).trustedCertifiers);
                            if !(retval != 0) {
                                retval =
                                    create_issuerAndSerial(context,
                                                           (*plgctx).cryptoctx,
                                                           (*reqctx).cryptoctx,
                                                           (*reqctx).idctx,
                                                           &mut (*req).kdcPkId.data
                                                               as
                                                               *mut *mut libc::c_char
                                                               as
                                                               *mut *mut libc::c_uchar,
                                                           &mut (*req).kdcPkId.length);
                                if !(retval != 0) {
                                    /* Encode the as-req */
                                    retval =
                                        k5int_encode_krb5_pa_pk_as_req.expect("non-null function pointer")(req,
                                                                                                           as_req)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free_krb5_algorithm_identifiers(&mut cmstypes);
    free(dh_params as *mut libc::c_void);
    free(dh_pubkey as *mut libc::c_void);
    free_krb5_pa_pk_as_req(&mut req);
    pkiDebug(b"pkinit_as_req_create retval=%d\n\x00" as *const u8 as
                 *const libc::c_char, retval);
    return retval;
}
#[c2rust::src_loc = "337:1"]
unsafe extern "C" fn pa_pkinit_parse_rep(mut context: krb5_context,
                                         mut plgctx: pkinit_context,
                                         mut reqctx: pkinit_req_context,
                                         mut request: *mut krb5_kdc_req,
                                         mut in_padata: *mut krb5_pa_data,
                                         mut etype: krb5_enctype,
                                         mut as_key: *mut krb5_keyblock,
                                         mut encoded_request: *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut asRep: krb5_data =
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length: 0 as libc::c_int as libc::c_uint,
                           data: 0 as *mut libc::c_char,};
            init
        };
    /*
     * One way or the other - success or failure - no other PA systems can
     * work if the server sent us a PKINIT reply, since only we know how to
     * decrypt the key.
     */
    if in_padata.is_null() ||
           (*in_padata).length == 0 as libc::c_int as libc::c_uint {
        pkiDebug(b"pa_pkinit_parse_rep: no in_padata\n\x00" as *const u8 as
                     *const libc::c_char);
        return -(1765328360 as libc::c_long) as krb5_error_code
    }
    asRep.data = (*in_padata).contents as *mut libc::c_char;
    asRep.length = (*in_padata).length;
    retval =
        pkinit_as_rep_parse(context, plgctx, reqctx, (*in_padata).pa_type,
                            request, &mut asRep, as_key, etype,
                            encoded_request);
    if retval != 0 {
        pkiDebug(b"pkinit_as_rep_parse returned %d (%s)\n\x00" as *const u8 as
                     *const libc::c_char, retval,
                 error_message(retval as errcode_t));
    } else { retval = 0 as libc::c_int }
    return retval;
}
#[c2rust::src_loc = "379:1"]
unsafe extern "C" fn verify_kdc_san(mut context: krb5_context,
                                    mut plgctx: pkinit_context,
                                    mut reqctx: pkinit_req_context,
                                    mut kdcprinc: krb5_principal,
                                    mut valid_san: *mut libc::c_int,
                                    mut need_eku_checking: *mut libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut certhosts: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut cfghosts: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut hostptr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut princs: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut get_dns: *mut *mut *mut libc::c_uchar =
        0 as *mut *mut *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    *valid_san = 0 as libc::c_int;
    *need_eku_checking = 1 as libc::c_int;
    retval =
        pkinit_libdefault_strings(context, &mut (*kdcprinc).realm,
                                  b"pkinit_kdc_hostname\x00" as *const u8 as
                                      *const libc::c_char, &mut cfghosts);
    if retval != 0 || cfghosts.is_null() {
        pkiDebug(b"%s: No pkinit_kdc_hostname values found in config file\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 15],
                                           &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
        get_dns = 0 as *mut *mut *mut libc::c_uchar
    } else {
        pkiDebug(b"%s: pkinit_kdc_hostname values found in config file\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 15],
                                           &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
        hostptr = cfghosts;
        while !(*hostptr).is_null() {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT client config accepts KDC dNSName SAN {str}\x00"
                                  as *const u8 as *const libc::c_char,
                              *hostptr);
            }
            hostptr = hostptr.offset(1)
        }
        get_dns =
            &mut certhosts as *mut *mut *mut libc::c_char as
                *mut *mut *mut libc::c_uchar
    }
    retval =
        crypto_retrieve_cert_sans(context, (*plgctx).cryptoctx,
                                  (*reqctx).cryptoctx, (*reqctx).idctx,
                                  &mut princs,
                                  0 as *mut *mut *mut libc::c_char, get_dns);
    if retval != 0 {
        pkiDebug(b"%s: error from retrieve_certificate_sans()\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 15],
                                           &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT client failed to decode SANs in KDC cert\x00"
                              as *const u8 as *const libc::c_char);
        }
        retval = -(1765328308 as libc::c_long) as krb5_error_code
    } else {
        i = 0 as libc::c_int;
        while !princs.is_null() && !(*princs.offset(i as isize)).is_null() {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT client found id-pkinit-san in KDC cert: {princ}\x00"
                                  as *const u8 as *const libc::c_char,
                              *princs.offset(i as isize));
            }
            i += 1
        }
        if !certhosts.is_null() {
            hostptr = certhosts;
            while !(*hostptr).is_null() {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client found dNSName SAN in KDC cert: {str}\x00"
                                      as *const u8 as *const libc::c_char,
                                  *hostptr);
                }
                hostptr = hostptr.offset(1)
            }
        }
        pkiDebug(b"%s: Checking pkinit sans\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 15],
                                           &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
        i = 0 as libc::c_int;
        loop  {
            if !(!princs.is_null() && !(*princs.offset(i as isize)).is_null())
               {
                current_block = 13321564401369230990;
                break ;
            }
            if krb5_principal_compare(context,
                                      *princs.offset(i as isize) as
                                          krb5_const_principal,
                                      kdcprinc as krb5_const_principal) != 0 {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client matched KDC principal {princ} against id-pkinit-san; no EKU check required\x00"
                                      as *const u8 as *const libc::c_char,
                                  kdcprinc);
                }
                pkiDebug(b"%s: pkinit san match found\n\x00" as *const u8 as
                             *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 15],
                                                   &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
                *valid_san = 1 as libc::c_int;
                *need_eku_checking = 0 as libc::c_int;
                retval = 0 as libc::c_int;
                current_block = 14483393026010652255;
                break ;
            } else { i += 1 }
        }
        match current_block {
            14483393026010652255 => { }
            _ => {
                pkiDebug(b"%s: no pkinit san match found\n\x00" as *const u8
                             as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 15],
                                                   &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
                if certhosts.is_null() {
                    pkiDebug(b"%s: no certhosts (or we wouldn\'t accept them anyway)\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 15],
                                                       &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
                    retval = -(1765328308 as libc::c_long) as krb5_error_code
                } else {
                    i = 0 as libc::c_int;
                    's_289:
                        loop  {
                            if (*certhosts.offset(i as isize)).is_null() {
                                current_block = 7178192492338286402;
                                break ;
                            }
                            j = 0 as libc::c_int;
                            while !cfghosts.is_null() &&
                                      !(*cfghosts.offset(j as
                                                             isize)).is_null()
                                  {
                                pkiDebug(b"%s: comparing cert name \'%s\' with config name \'%s\'\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*::std::mem::transmute::<&[u8; 15],
                                                                   &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr(),
                                         *certhosts.offset(i as isize),
                                         *cfghosts.offset(j as isize));
                                if strcasecmp(*certhosts.offset(i as isize),
                                              *cfghosts.offset(j as isize)) ==
                                       0 as libc::c_int {
                                    if (*context).trace_callback.is_some() {
                                        krb5int_trace(context,
                                                      b"PKINIT client matched KDC hostname {str} against dNSName SAN; EKU check still required\x00"
                                                          as *const u8 as
                                                          *const libc::c_char,
                                                      *certhosts.offset(i as
                                                                            isize));
                                    }
                                    pkiDebug(b"%s: we have a dnsName match\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             (*::std::mem::transmute::<&[u8; 15],
                                                                       &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
                                    *valid_san = 1 as libc::c_int;
                                    retval = 0 as libc::c_int;
                                    current_block = 14483393026010652255;
                                    break 's_289 ;
                                } else { j += 1 }
                            }
                            i += 1
                        }
                    match current_block {
                        14483393026010652255 => { }
                        _ => {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"PKINIT client found no acceptable SAN in KDC cert\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                            }
                            pkiDebug(b"%s: no dnsName san match found\n\x00"
                                         as *const u8 as *const libc::c_char,
                                     (*::std::mem::transmute::<&[u8; 15],
                                                               &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr());
                            /* We found no match */
                            retval = 0 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    if !princs.is_null() {
        i = 0 as libc::c_int;
        while !(*princs.offset(i as isize)).is_null() {
            krb5_free_principal(context, *princs.offset(i as isize));
            i += 1
        }
        free(princs as *mut libc::c_void);
    }
    if !certhosts.is_null() {
        i = 0 as libc::c_int;
        while !(*certhosts.offset(i as isize)).is_null() {
            free(*certhosts.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(certhosts as *mut libc::c_void);
    }
    if !cfghosts.is_null() { profile_free_list(cfghosts); }
    pkiDebug(b"%s: returning retval %d, valid_san %d, need_eku_checking %d\n\x00"
                 as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 15],
                                       &[libc::c_char; 15]>(b"verify_kdc_san\x00")).as_ptr(),
             retval, *valid_san, *need_eku_checking);
    return retval;
}
#[c2rust::src_loc = "486:1"]
unsafe extern "C" fn verify_kdc_eku(mut context: krb5_context,
                                    mut plgctx: pkinit_context,
                                    mut reqctx: pkinit_req_context,
                                    mut eku_accepted: *mut libc::c_int)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    *eku_accepted = 0 as libc::c_int;
    if (*(*reqctx).opts).require_eku == 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT client skipping EKU check due to configuration\x00"
                              as *const u8 as *const libc::c_char);
        }
        pkiDebug(b"%s: configuration requests no EKU checking\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 15],
                                           &[libc::c_char; 15]>(b"verify_kdc_eku\x00")).as_ptr());
        *eku_accepted = 1 as libc::c_int;
        retval = 0 as libc::c_int
    } else {
        retval =
            crypto_check_cert_eku(context, (*plgctx).cryptoctx,
                                  (*reqctx).cryptoctx, (*reqctx).idctx,
                                  1 as libc::c_int,
                                  (*(*reqctx).opts).accept_secondary_eku,
                                  eku_accepted);
        if retval != 0 {
            pkiDebug(b"%s: Error from crypto_check_cert_eku %d (%s)\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 15],
                                               &[libc::c_char; 15]>(b"verify_kdc_eku\x00")).as_ptr(),
                     retval, error_message(retval as errcode_t));
        }
    }
    if *eku_accepted != 0 {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT client found acceptable EKU in KDC cert\x00"
                              as *const u8 as *const libc::c_char);
        }
    } else if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"PKINIT client found no acceptable EKU in KDC cert\x00"
                          as *const u8 as *const libc::c_char);
    }
    pkiDebug(b"%s: returning retval %d, eku_accepted %d\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 15],
                                       &[libc::c_char; 15]>(b"verify_kdc_eku\x00")).as_ptr(),
             retval, *eku_accepted);
    return retval;
}
/*
 * Parse PA-PK-AS-REP message. Optionally evaluates the message's
 * certificate chain.
 * Optionally returns various components.
 */
#[c2rust::src_loc = "529:1"]
unsafe extern "C" fn pkinit_as_rep_parse(mut context: krb5_context,
                                         mut plgctx: pkinit_context,
                                         mut reqctx: pkinit_req_context,
                                         mut pa_type: krb5_preauthtype,
                                         mut request: *mut krb5_kdc_req,
                                         mut as_rep: *const krb5_data,
                                         mut key_block: *mut krb5_keyblock,
                                         mut etype: krb5_enctype,
                                         mut encoded_request: *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut kdc_princ: krb5_principal = 0 as krb5_principal;
    let mut kdc_reply: *mut krb5_pa_pk_as_rep = 0 as *mut krb5_pa_pk_as_rep;
    let mut kdc_dh: *mut krb5_kdc_dh_key_info =
        0 as *mut krb5_kdc_dh_key_info;
    let mut key_pack: *mut krb5_reply_key_pack =
        0 as *mut krb5_reply_key_pack;
    let mut dh_data: krb5_data =
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length: 0 as libc::c_int as libc::c_uint,
                           data: 0 as *mut libc::c_char,};
            init
        };
    let mut client_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut kdc_hostname: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut client_key_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cksum: krb5_checksum =
        {
            let mut init =
                _krb5_checksum{magic: 0 as libc::c_int,
                               checksum_type: 0 as libc::c_int,
                               length: 0 as libc::c_int as libc::c_uint,
                               contents: 0 as *mut krb5_octet,};
            init
        };
    let mut k5data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut secret: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut valid_san: libc::c_int = 0 as libc::c_int;
    let mut valid_eku: libc::c_int = 0 as libc::c_int;
    let mut need_eku_checking: libc::c_int = 1 as libc::c_int;
    if !as_rep.is_null() && !key_block.is_null() {
    } else {
        __assert_fail(b"(as_rep != NULL) && (key_block != NULL)\x00" as
                          *const u8 as *const libc::c_char,
                      b"pkinit_clnt.c\x00" as *const u8 as
                          *const libc::c_char,
                      555 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 183],
                                                &[libc::c_char; 183]>(b"krb5_error_code pkinit_as_rep_parse(krb5_context, pkinit_context, pkinit_req_context, krb5_preauthtype, krb5_kdc_req *, const krb5_data *, krb5_keyblock *, krb5_enctype, krb5_data *)\x00")).as_ptr());
    }
    retval =
        k5int_decode_krb5_pa_pk_as_rep.expect("non-null function pointer")(as_rep,
                                                                           &mut kdc_reply);
    if retval != 0 {
        pkiDebug(b"decode_krb5_as_rep failed %d\n\x00" as *const u8 as
                     *const libc::c_char, retval);
        return retval
    }
    match (*kdc_reply).choice as libc::c_int {
        0 => {
            pkiDebug(b"as_rep: DH key transport algorithm\n\x00" as *const u8
                         as *const libc::c_char);
            retval =
                cms_signeddata_verify(context, (*plgctx).cryptoctx,
                                      (*reqctx).cryptoctx, (*reqctx).idctx,
                                      CMS_SIGN_SERVER as libc::c_int,
                                      (*(*reqctx).opts).require_crl_checking,
                                      (*kdc_reply).u.dh_Info.dhSignedData.data
                                          as *mut libc::c_uchar,
                                      (*kdc_reply).u.dh_Info.dhSignedData.length,
                                      &mut dh_data.data as
                                          *mut *mut libc::c_char as
                                          *mut *mut libc::c_uchar,
                                      &mut dh_data.length,
                                      0 as *mut *mut libc::c_uchar,
                                      0 as *mut libc::c_uint,
                                      0 as *mut libc::c_int);
            if retval != 0 as libc::c_int {
                pkiDebug(b"failed to verify pkcs7 signed data\n\x00" as
                             *const u8 as *const libc::c_char);
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client could not verify DH reply\x00"
                                      as *const u8 as *const libc::c_char);
                }
                current_block = 3550238336004337377;
            } else {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client verified DH reply\x00" as
                                      *const u8 as *const libc::c_char);
                }
                current_block = 7226443171521532240;
            }
        }
        1 => {
            pkiDebug(b"as_rep: RSA key transport algorithm\n\x00" as *const u8
                         as *const libc::c_char);
            retval =
                cms_envelopeddata_verify(context, (*plgctx).cryptoctx,
                                         (*reqctx).cryptoctx, (*reqctx).idctx,
                                         pa_type,
                                         (*(*reqctx).opts).require_crl_checking,
                                         (*kdc_reply).u.encKeyPack.data as
                                             *mut libc::c_uchar,
                                         (*kdc_reply).u.encKeyPack.length,
                                         &mut dh_data.data as
                                             *mut *mut libc::c_char as
                                             *mut *mut libc::c_uchar,
                                         &mut dh_data.length);
            if retval != 0 as libc::c_int {
                pkiDebug(b"failed to verify pkcs7 enveloped data\n\x00" as
                             *const u8 as *const libc::c_char);
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client could not verify RSA reply\x00"
                                      as *const u8 as *const libc::c_char);
                }
                current_block = 3550238336004337377;
            } else {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client verified RSA reply\x00" as
                                      *const u8 as *const libc::c_char);
                }
                current_block = 7226443171521532240;
            }
        }
        _ => {
            pkiDebug(b"unknown as_rep type %d\n\x00" as *const u8 as
                         *const libc::c_char,
                     (*kdc_reply).choice as libc::c_int);
            retval = -(1 as libc::c_int);
            current_block = 3550238336004337377;
        }
    }
    match current_block {
        7226443171521532240 => {
            retval =
                krb5_build_principal_ext(context,
                                         &mut kdc_princ as
                                             *mut krb5_principal,
                                         (*(*request).server).realm.length,
                                         (*(*request).server).realm.data,
                                         strlen(b"krbtgt\x00" as *const u8 as
                                                    *const libc::c_char),
                                         b"krbtgt\x00" as *const u8 as
                                             *const libc::c_char,
                                         (*(*request).server).realm.length,
                                         (*(*request).server).realm.data,
                                         0 as libc::c_int);
            if !(retval != 0) {
                retval =
                    verify_kdc_san(context, plgctx, reqctx, kdc_princ,
                                   &mut valid_san, &mut need_eku_checking);
                if !(retval != 0) {
                    if valid_san == 0 {
                        pkiDebug(b"%s: did not find an acceptable SAN in KDC certificate\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*::std::mem::transmute::<&[u8; 20],
                                                           &[libc::c_char; 20]>(b"pkinit_as_rep_parse\x00")).as_ptr());
                        retval =
                            -(1765328308 as libc::c_long) as krb5_error_code
                    } else {
                        if need_eku_checking != 0 {
                            retval =
                                verify_kdc_eku(context, plgctx, reqctx,
                                               &mut valid_eku);
                            if retval != 0 {
                                current_block = 3550238336004337377;
                            } else if valid_eku == 0 {
                                pkiDebug(b"%s: did not find an acceptable EKU in KDC certificate\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*::std::mem::transmute::<&[u8; 20],
                                                                   &[libc::c_char; 20]>(b"pkinit_as_rep_parse\x00")).as_ptr());
                                retval =
                                    -(1765328307 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 3550238336004337377;
                            } else { current_block = 12829669402821218572; }
                        } else {
                            pkiDebug(b"%s: skipping EKU check\n\x00" as
                                         *const u8 as *const libc::c_char,
                                     (*::std::mem::transmute::<&[u8; 20],
                                                               &[libc::c_char; 20]>(b"pkinit_as_rep_parse\x00")).as_ptr());
                            current_block = 12829669402821218572;
                        }
                        match current_block {
                            3550238336004337377 => { }
                            _ => {
                                k5data.length = dh_data.length;
                                k5data.data = dh_data.data;
                                match (*kdc_reply).choice as libc::c_int {
                                    0 => {
                                        retval =
                                            k5int_decode_krb5_kdc_dh_key_info.expect("non-null function pointer")(&mut k5data,
                                                                                                                  &mut kdc_dh);
                                        if retval != 0 as libc::c_int {
                                            pkiDebug(b"failed to decode kdc_dh_key_info\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            current_block =
                                                3550238336004337377;
                                        } else {
                                            /* client after KDC reply */
                                            retval =
                                                client_process_dh(context,
                                                                  (*plgctx).cryptoctx,
                                                                  (*reqctx).cryptoctx,
                                                                  (*reqctx).idctx,
                                                                  (*kdc_dh).subjectPublicKey.data
                                                                      as
                                                                      *mut libc::c_uchar,
                                                                  (*kdc_dh).subjectPublicKey.length,
                                                                  &mut client_key,
                                                                  &mut client_key_len);
                                            if retval != 0 as libc::c_int {
                                                pkiDebug(b"failed to process dh params\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                                current_block =
                                                    3550238336004337377;
                                            } else if !(*kdc_reply).u.dh_Info.kdfID.is_null()
                                             {
                                                secret.length =
                                                    client_key_len;
                                                secret.data =
                                                    client_key as
                                                        *mut libc::c_char;
                                                retval =
                                                    pkinit_alg_agility_kdf(context,
                                                                           &mut secret,
                                                                           (*kdc_reply).u.dh_Info.kdfID,
                                                                           (*request).client
                                                                               as
                                                                               krb5_const_principal,
                                                                           (*request).server
                                                                               as
                                                                               krb5_const_principal,
                                                                           etype,
                                                                           encoded_request,
                                                                           as_rep
                                                                               as
                                                                               *mut krb5_data,
                                                                           key_block);
                                                if retval != 0 {
                                                    pkiDebug(b"failed to create key pkinit_alg_agility_kdf %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             error_message(retval
                                                                               as
                                                                               errcode_t));
                                                    current_block =
                                                        3550238336004337377;
                                                } else {
                                                    if (*context).trace_callback.is_some()
                                                       {
                                                        krb5int_trace(context,
                                                                      b"PKINIT client used KDF {hexdata} to compute reply key {keyblock}\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      (*kdc_reply).u.dh_Info.kdfID,
                                                                      key_block);
                                                    }
                                                    current_block =
                                                        11064061988481400464;
                                                }
                                                /* If we have a KDF algorithm ID, call the algorithm agility KDF... */
                                                /* ...otherwise, use the older octetstring2key function. */
                                            } else {
                                                retval =
                                                    pkinit_octetstring2key(context,
                                                                           etype,
                                                                           client_key,
                                                                           client_key_len,
                                                                           key_block);
                                                if retval != 0 {
                                                    pkiDebug(b"failed to create key pkinit_octetstring2key %s\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             error_message(retval
                                                                               as
                                                                               errcode_t));
                                                    current_block =
                                                        3550238336004337377;
                                                } else {
                                                    if (*context).trace_callback.is_some()
                                                       {
                                                        krb5int_trace(context,
                                                                      b"PKINIT client used octetstring2key to compute reply key {keyblock}\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      key_block);
                                                    }
                                                    current_block =
                                                        11064061988481400464;
                                                }
                                            }
                                        }
                                    }
                                    1 => {
                                        retval =
                                            k5int_decode_krb5_reply_key_pack.expect("non-null function pointer")(&mut k5data,
                                                                                                                 &mut key_pack);
                                        if retval != 0 {
                                            pkiDebug(b"failed to decode reply_key_pack\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            current_block =
                                                3550238336004337377;
                                        } else {
                                            /*
         * This is hack but Windows sends back SHA1 checksum
         * with checksum type of 14. There is currently no
         * checksum type of 14 defined.
         */
                                            if (*key_pack).asChecksum.checksum_type
                                                   == 14 as libc::c_int {
                                                (*key_pack).asChecksum.checksum_type
                                                    = 0x9 as libc::c_int
                                            }
                                            retval =
                                                krb5_c_make_checksum(context,
                                                                     (*key_pack).asChecksum.checksum_type,
                                                                     &mut (*key_pack).replyKey,
                                                                     6 as
                                                                         libc::c_int,
                                                                     encoded_request,
                                                                     &mut cksum);
                                            if retval != 0 {
                                                pkiDebug(b"failed to make a checksum\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                                current_block =
                                                    3550238336004337377;
                                            } else if cksum.length !=
                                                          (*key_pack).asChecksum.length
                                                          ||
                                                          k5_bcmp(cksum.contents
                                                                      as
                                                                      *const libc::c_void,
                                                                  (*key_pack).asChecksum.contents
                                                                      as
                                                                      *const libc::c_void,
                                                                  cksum.length
                                                                      as
                                                                      size_t)
                                                              !=
                                                              0 as libc::c_int
                                             {
                                                if (*context).trace_callback.is_some()
                                                   {
                                                    krb5int_trace(context,
                                                                  b"PKINIT client checksum mismatch: expected {cksum}, received {cksum}\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  &mut cksum
                                                                      as
                                                                      *mut krb5_checksum,
                                                                  &mut (*key_pack).asChecksum
                                                                      as
                                                                      *mut krb5_checksum);
                                                }
                                                pkiDebug(b"failed to match the checksums\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                                current_block =
                                                    3550238336004337377;
                                            } else {
                                                pkiDebug(b"checksums match\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                                krb5_copy_keyblock_contents(context,
                                                                            &mut (*key_pack).replyKey,
                                                                            key_block);
                                                if (*context).trace_callback.is_some()
                                                   {
                                                    krb5int_trace(context,
                                                                  b"PKINIT client retrieved reply key {keyblock} from RSA reply (checksum {cksum})\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char,
                                                                  key_block,
                                                                  &mut cksum
                                                                      as
                                                                      *mut krb5_checksum);
                                                }
                                                current_block =
                                                    11064061988481400464;
                                            }
                                        }
                                    }
                                    _ => {
                                        pkiDebug(b"unknow as_rep type %d\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*kdc_reply).choice as
                                                     libc::c_int);
                                        current_block = 3550238336004337377;
                                    }
                                }
                                match current_block {
                                    3550238336004337377 => { }
                                    _ => { retval = 0 as libc::c_int }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    free(dh_data.data as *mut libc::c_void);
    krb5_free_principal(context, kdc_princ);
    free(client_key as *mut libc::c_void);
    free_krb5_kdc_dh_key_info(&mut kdc_dh);
    free_krb5_pa_pk_as_rep(&mut kdc_reply);
    if !key_pack.is_null() {
        free_krb5_reply_key_pack(&mut key_pack);
        free(cksum.contents as *mut libc::c_void);
    }
    free(kdc_hostname as *mut libc::c_void);
    pkiDebug(b"pkinit_as_rep_parse returning %d (%s)\n\x00" as *const u8 as
                 *const libc::c_char, retval,
             error_message(retval as errcode_t));
    return retval;
}
#[c2rust::src_loc = "786:1"]
unsafe extern "C" fn pkinit_client_profile(mut context: krb5_context,
                                           mut plgctx: pkinit_context,
                                           mut reqctx: pkinit_req_context,
                                           mut cb: krb5_clpreauth_callbacks,
                                           mut rock: krb5_clpreauth_rock,
                                           mut realm: *const krb5_data) {
    let mut configured_identity: *const libc::c_char =
        0 as *const libc::c_char;
    let mut eku_string: *mut libc::c_char = 0 as *mut libc::c_char;
    pkiDebug(b"pkinit_client_profile %p %p %p %p\n\x00" as *const u8 as
                 *const libc::c_char, context, plgctx, reqctx, realm);
    pkinit_libdefault_boolean(context, realm,
                              b"pkinit_require_crl_checking\x00" as *const u8
                                  as *const libc::c_char,
                              (*(*reqctx).opts).require_crl_checking,
                              &mut (*(*reqctx).opts).require_crl_checking);
    pkinit_libdefault_integer(context, realm,
                              b"pkinit_dh_min_bits\x00" as *const u8 as
                                  *const libc::c_char,
                              (*(*reqctx).opts).dh_size,
                              &mut (*(*reqctx).opts).dh_size);
    if (*(*reqctx).opts).dh_size != 1024 as libc::c_int &&
           (*(*reqctx).opts).dh_size != 2048 as libc::c_int &&
           (*(*reqctx).opts).dh_size != 4096 as libc::c_int {
        pkiDebug(b"%s: invalid value (%d) for pkinit_dh_min_bits, using default value (%d) instead\n\x00"
                     as *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 22],
                                           &[libc::c_char; 22]>(b"pkinit_client_profile\x00")).as_ptr(),
                 (*(*reqctx).opts).dh_size, 2048 as libc::c_int);
        (*(*reqctx).opts).dh_size = 2048 as libc::c_int
    }
    pkinit_libdefault_string(context, realm,
                             b"pkinit_eku_checking\x00" as *const u8 as
                                 *const libc::c_char, &mut eku_string);
    if !eku_string.is_null() {
        if strcasecmp(eku_string,
                      b"kpKDC\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*(*reqctx).opts).require_eku = 1 as libc::c_int;
            (*(*reqctx).opts).accept_secondary_eku = 0 as libc::c_int
        } else if strcasecmp(eku_string,
                             b"kpServerAuth\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            (*(*reqctx).opts).require_eku = 1 as libc::c_int;
            (*(*reqctx).opts).accept_secondary_eku = 1 as libc::c_int
        } else if strcasecmp(eku_string,
                             b"none\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            (*(*reqctx).opts).require_eku = 0 as libc::c_int;
            (*(*reqctx).opts).accept_secondary_eku = 0 as libc::c_int
        } else {
            pkiDebug(b"%s: Invalid value for pkinit_eku_checking: \'%s\'\n\x00"
                         as *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 22],
                                               &[libc::c_char; 22]>(b"pkinit_client_profile\x00")).as_ptr(),
                     eku_string);
        }
        free(eku_string as *mut libc::c_void);
    }
    /* Only process anchors here if they were not specified on command line */
    if (*(*reqctx).idopts).anchors.is_null() {
        pkinit_libdefault_strings(context, realm,
                                  b"pkinit_anchors\x00" as *const u8 as
                                      *const libc::c_char,
                                  &mut (*(*reqctx).idopts).anchors);
    }
    pkinit_libdefault_strings(context, realm,
                              b"pkinit_pool\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*(*reqctx).idopts).intermediates);
    pkinit_libdefault_strings(context, realm,
                              b"pkinit_revoke\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*(*reqctx).idopts).crls);
    pkinit_libdefault_strings(context, realm,
                              b"pkinit_identities\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut (*(*reqctx).idopts).identity_alt);
    (*reqctx).do_identity_matching = 1 as libc::c_int;
    /* If we had a primary identity in the stored configuration, pick it up. */
    configured_identity =
        (*cb).get_cc_config.expect("non-null function pointer")(context, rock,
                                                                b"X509_user_identity\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char);
    if !configured_identity.is_null() {
        free((*(*reqctx).idopts).identity as *mut libc::c_void);
        (*(*reqctx).idopts).identity = strdup(configured_identity);
        (*reqctx).do_identity_matching = 0 as libc::c_int
    };
}
/*
 * Convert a PKCS11 token flags value to the subset that we're interested in
 * passing along to our API callers.
 */
#[c2rust::src_loc = "865:1"]
unsafe extern "C" fn pkinit_client_get_token_flags(mut pkcs11_token_flags:
                                                       libc::c_ulong)
 -> libc::c_longlong {
    let mut flags: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if pkcs11_token_flags &
           ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong != 0 {
        flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_longlong
    }
    if pkcs11_token_flags &
           ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong != 0 {
        flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_longlong
    }
    if pkcs11_token_flags &
           ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0 {
        flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_longlong
    }
    return flags;
}
/*
 * Phase one of loading client identity information - call
 * identity_initialize() to load any identities which we can without requiring
 * help from the calling user, and use their names of those which we can't load
 * to construct the challenge for the responder callback.
 */
#[c2rust::src_loc = "885:1"]
unsafe extern "C" fn pkinit_client_prep_questions(mut context: krb5_context,
                                                  mut moddata:
                                                      krb5_clpreauth_moddata,
                                                  mut modreq:
                                                      krb5_clpreauth_modreq,
                                                  mut opt:
                                                      *mut krb5_get_init_creds_opt,
                                                  mut cb:
                                                      krb5_clpreauth_callbacks,
                                                  mut rock:
                                                      krb5_clpreauth_rock,
                                                  mut request:
                                                      *mut krb5_kdc_req,
                                                  mut encoded_request_body:
                                                      *mut krb5_data,
                                                  mut encoded_previous_request:
                                                      *mut krb5_data,
                                                  mut pa_data:
                                                      *mut krb5_pa_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut plgctx: pkinit_context = moddata as pkinit_context;
    let mut reqctx: pkinit_req_context = modreq as pkinit_req_context;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut deferred_ids: *const pkinit_deferred_id =
        0 as *const pkinit_deferred_id;
    let mut identity: *const libc::c_char = 0 as *const libc::c_char;
    let mut ck_flags: libc::c_ulong = 0;
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut jval: k5_json_object = 0 as k5_json_object;
    let mut jflag: k5_json_number = 0 as k5_json_number;
    if (*reqctx).identity_initialized == 0 {
        pkinit_client_profile(context, plgctx, reqctx, cb, rock,
                              &mut (*(*request).server).realm);
        retval =
            pkinit_identity_initialize(context, (*plgctx).cryptoctx,
                                       (*reqctx).cryptoctx, (*reqctx).idopts,
                                       (*reqctx).idctx, cb, rock,
                                       (*request).client);
        if retval != 0 as libc::c_int {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT client has no configured identity; giving up\x00"
                                  as *const u8 as *const libc::c_char);
            }
            pkiDebug(b"pkinit_identity_initialize returned %d (%s)\n\x00" as
                         *const u8 as *const libc::c_char, retval,
                     error_message(retval as errcode_t));
        }
        (*reqctx).identity_initialized = 1 as libc::c_int;
        if retval != 0 as libc::c_int {
            pkiDebug(b"%s: not asking responder question\n\x00" as *const u8
                         as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 29],
                                               &[libc::c_char; 29]>(b"pkinit_client_prep_questions\x00")).as_ptr());
            retval = 0 as libc::c_int;
            current_block = 9077090635031960817;
        } else { current_block = 13797916685926291137; }
    } else { current_block = 13797916685926291137; }
    match current_block {
        13797916685926291137 => {
            deferred_ids = crypto_get_deferred_ids(context, (*reqctx).idctx);
            i = 0 as libc::c_int;
            while !deferred_ids.is_null() &&
                      !(*deferred_ids.offset(i as isize)).is_null() {
                i += 1
            }
            n = i;
            /* Make sure we don't just return an empty challenge. */
            if n == 0 as libc::c_int {
                pkiDebug(b"%s: no questions to ask\n\x00" as *const u8 as
                             *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 29],
                                                   &[libc::c_char; 29]>(b"pkinit_client_prep_questions\x00")).as_ptr());
                retval = 0 as libc::c_int
            } else {
                /* Create the top-level object. */
                retval = k5_json_object_create(&mut jval);
                if !(retval != 0 as libc::c_int) {
                    i = 0 as libc::c_int;
                    loop  {
                        if !(i < n) {
                            current_block = 9007357115414505193;
                            break ;
                        }
                        /* Add an entry to the top-level object for the identity. */
                        identity =
                            (**deferred_ids.offset(i as isize)).identity;
                        ck_flags =
                            (**deferred_ids.offset(i as isize)).ck_flags;
                        /* Calculate the flags value for the bits that that we care about. */
                        retval =
                            k5_json_number_create(pkinit_client_get_token_flags(ck_flags),
                                                  &mut jflag);
                        if retval != 0 as libc::c_int {
                            current_block = 9077090635031960817;
                            break ;
                        }
                        retval =
                            k5_json_object_set(jval, identity,
                                               jflag as k5_json_value);
                        if retval != 0 as libc::c_int {
                            current_block = 9077090635031960817;
                            break ;
                        }
                        k5_json_release(jflag as k5_json_value);
                        jflag = 0 as k5_json_number;
                        i += 1
                    }
                    match current_block {
                        9077090635031960817 => { }
                        _ => {
                            /* Encode and done. */
                            retval =
                                k5_json_encode(jval as k5_json_value,
                                               &mut encoded);
                            if retval == 0 as libc::c_int {
                                (*cb).ask_responder_question.expect("non-null function pointer")(context,
                                                                                                 rock,
                                                                                                 b"pkinit\x00"
                                                                                                     as
                                                                                                     *const u8
                                                                                                     as
                                                                                                     *const libc::c_char,
                                                                                                 encoded);
                                pkiDebug(b"%s: asking question \'%s\'\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*::std::mem::transmute::<&[u8; 29],
                                                                   &[libc::c_char; 29]>(b"pkinit_client_prep_questions\x00")).as_ptr(),
                                         encoded);
                                free(encoded as *mut libc::c_void);
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    k5_json_release(jval as k5_json_value);
    k5_json_release(jflag as k5_json_value);
    pkiDebug(b"%s returning %d\n\x00" as *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 29],
                                       &[libc::c_char; 29]>(b"pkinit_client_prep_questions\x00")).as_ptr(),
             retval);
    return retval;
}
#[c2rust::src_loc = "991:1"]
unsafe extern "C" fn save_one_password(mut arg: *mut libc::c_void,
                                       mut key: *const libc::c_char,
                                       mut val: k5_json_value) {
    let mut data: *mut save_one_password_data =
        arg as *mut save_one_password_data;
    let mut reqctx: pkinit_req_context = (*data).modreq as pkinit_req_context;
    let mut password: *const libc::c_char = 0 as *const libc::c_char;
    if k5_json_get_tid(val) == 131 as libc::c_int as libc::c_uint {
        password = k5_json_string_utf8(val as k5_json_string);
        pkiDebug(b"%s: \"%s\": %p\n\x00" as *const u8 as *const libc::c_char,
                 (*data).caller, key, password);
        crypto_set_deferred_id((*data).context, (*reqctx).idctx, key,
                               password);
    };
}
#[c2rust::src_loc = "1005:1"]
unsafe extern "C" fn pkinit_client_parse_answers(mut context: krb5_context,
                                                 mut moddata:
                                                     krb5_clpreauth_moddata,
                                                 mut modreq:
                                                     krb5_clpreauth_modreq,
                                                 mut cb:
                                                     krb5_clpreauth_callbacks,
                                                 mut rock:
                                                     krb5_clpreauth_rock)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut encoded: *const libc::c_char = 0 as *const libc::c_char;
    let mut jval: k5_json_value = 0 as *mut libc::c_void;
    let mut data: save_one_password_data =
        save_one_password_data{context: 0 as *mut _krb5_context,
                               modreq: 0 as *mut krb5_clpreauth_modreq_st,
                               caller: 0 as *const libc::c_char,};
    data.context = context;
    data.modreq = modreq;
    data.caller =
        (*::std::mem::transmute::<&[u8; 28],
                                  &[libc::c_char; 28]>(b"pkinit_client_parse_answers\x00")).as_ptr();
    encoded =
        (*cb).get_responder_answer.expect("non-null function pointer")(context,
                                                                       rock,
                                                                       b"pkinit\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char);
    if encoded.is_null() { return 0 as libc::c_int }
    pkiDebug(b"pkinit_client_parse_answers: %s\n\x00" as *const u8 as
                 *const libc::c_char, encoded);
    retval = k5_json_decode(encoded, &mut jval);
    if !(retval != 0 as libc::c_int) {
        /* Expect that the top-level answer is an object. */
        if k5_json_get_tid(jval) != 130 as libc::c_int as libc::c_uint {
            retval = 22 as libc::c_int
        } else {
            /* Store the passed-in per-identity passwords. */
            k5_json_object_iterate(jval as k5_json_object,
                                   Some(save_one_password as
                                            unsafe extern "C" fn(_:
                                                                     *mut libc::c_void,
                                                                 _:
                                                                     *const libc::c_char,
                                                                 _:
                                                                     k5_json_value)
                                                -> ()),
                                   &mut data as *mut save_one_password_data as
                                       *mut libc::c_void);
            retval = 0 as libc::c_int
        }
    }
    if !jval.is_null() { k5_json_release(jval); }
    return retval;
}
#[c2rust::src_loc = "1048:1"]
unsafe extern "C" fn pkinit_client_process(mut context: krb5_context,
                                           mut moddata:
                                               krb5_clpreauth_moddata,
                                           mut modreq: krb5_clpreauth_modreq,
                                           mut gic_opt:
                                               *mut krb5_get_init_creds_opt,
                                           mut cb: krb5_clpreauth_callbacks,
                                           mut rock: krb5_clpreauth_rock,
                                           mut request: *mut krb5_kdc_req,
                                           mut encoded_request_body:
                                               *mut krb5_data,
                                           mut encoded_previous_request:
                                               *mut krb5_data,
                                           mut in_padata: *mut krb5_pa_data,
                                           mut prompter: krb5_prompter_fct,
                                           mut prompter_data:
                                               *mut libc::c_void,
                                           mut out_padata:
                                               *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut enctype: krb5_enctype = -(1 as libc::c_int);
    let mut processing_request: libc::c_int = 0 as libc::c_int;
    let mut plgctx: pkinit_context = moddata as pkinit_context;
    let mut reqctx: pkinit_req_context = modreq as pkinit_req_context;
    let mut as_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    pkiDebug(b"pkinit_client_process %p %p %p %p\n\x00" as *const u8 as
                 *const libc::c_char, context, plgctx, reqctx, request);
    if plgctx.is_null() || reqctx.is_null() { return 22 as libc::c_int }
    match (*in_padata).pa_type {
        147 => {
            (*reqctx).rfc6112_kdc = 1 as libc::c_int;
            return 0 as libc::c_int
        }
        150 => {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT client received freshness token from KDC\x00"
                                  as *const u8 as *const libc::c_char);
            }
            krb5_free_data(context, (*reqctx).freshness_token);
            (*reqctx).freshness_token = 0 as *mut krb5_data;
            d =
                make_data((*in_padata).contents as *mut libc::c_void,
                          (*in_padata).length);
            return krb5_copy_data(context, &mut d,
                                  &mut (*reqctx).freshness_token)
        }
        16 => {
            pkiDebug(b"processing KRB5_PADATA_PK_AS_REQ\n\x00" as *const u8 as
                         *const libc::c_char);
            processing_request = 1 as libc::c_int
        }
        17 => {
            pkiDebug(b"processing KRB5_PADATA_PK_AS_REP\n\x00" as *const u8 as
                         *const libc::c_char);
        }
        _ => {
            pkiDebug(b"unrecognized patype = %d for PKINIT\n\x00" as *const u8
                         as *const libc::c_char, (*in_padata).pa_type);
            return 22 as libc::c_int
        }
    }
    if processing_request != 0 {
        pkinit_client_profile(context, plgctx, reqctx, cb, rock,
                              &mut (*(*request).server).realm);
        /* Pull in PINs and passwords for identities which we deferred
         * loading earlier. */
        retval =
            pkinit_client_parse_answers(context, moddata, modreq, cb, rock);
        if retval != 0 {
            if retval as libc::c_long == -(1765328324 as libc::c_long) {
                pkiDebug(b"pkinit responder answers were invalid\n\x00" as
                             *const u8 as *const libc::c_char);
            }
            return retval
        }
        if (*reqctx).identity_prompted == 0 {
            (*reqctx).identity_prompted = 1 as libc::c_int;
            /*
             * Load identities (again, potentially), prompting, if we can, for
             * anything for which we didn't get an answer from the responder
             * callback.
             */
            pkinit_identity_set_prompter((*reqctx).idctx, prompter,
                                         prompter_data);
            retval =
                pkinit_identity_prompt(context, (*plgctx).cryptoctx,
                                       (*reqctx).cryptoctx, (*reqctx).idopts,
                                       (*reqctx).idctx, cb, rock,
                                       (*reqctx).do_identity_matching,
                                       (*request).client);
            pkinit_identity_set_prompter((*reqctx).idctx, None,
                                         0 as *mut libc::c_void);
            (*reqctx).identity_prompt_retval = retval;
            if retval != 0 {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client has no configured identity; giving up\x00"
                                      as *const u8 as *const libc::c_char);
                }
                pkiDebug(b"pkinit_identity_prompt returned %d (%s)\n\x00" as
                             *const u8 as *const libc::c_char, retval,
                         error_message(retval as errcode_t));
                return retval
            }
        } else if (*reqctx).identity_prompt_retval != 0 {
            retval = (*reqctx).identity_prompt_retval;
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT client has no configured identity; giving up\x00"
                                  as *const u8 as *const libc::c_char);
            }
            pkiDebug(b"pkinit_identity_prompt previously returned %d (%s)\n\x00"
                         as *const u8 as *const libc::c_char, retval,
                     error_message(retval as errcode_t));
            return retval
        }
        retval =
            pa_pkinit_gen_req(context, plgctx, reqctx, cb, rock, request,
                              (*in_padata).pa_type, out_padata, prompter,
                              prompter_data, gic_opt)
    } else {
        /*
         * Get the enctype of the reply.
         */
        enctype =
            (*cb).get_etype.expect("non-null function pointer")(context,
                                                                rock);
        retval =
            pa_pkinit_parse_rep(context, plgctx, reqctx, request, in_padata,
                                enctype, &mut as_key,
                                encoded_previous_request);
        if retval == 0 as libc::c_int {
            retval =
                (*cb).set_as_key.expect("non-null function pointer")(context,
                                                                     rock,
                                                                     &mut as_key);
            krb5_free_keyblock_contents(context, &mut as_key);
        }
    }
    pkiDebug(b"pkinit_client_process: returning %d (%s)\n\x00" as *const u8 as
                 *const libc::c_char, retval,
             error_message(retval as errcode_t));
    return retval;
}
#[c2rust::src_loc = "1161:1"]
unsafe extern "C" fn pkinit_client_tryagain(mut context: krb5_context,
                                            mut moddata:
                                                krb5_clpreauth_moddata,
                                            mut modreq: krb5_clpreauth_modreq,
                                            mut gic_opt:
                                                *mut krb5_get_init_creds_opt,
                                            mut cb: krb5_clpreauth_callbacks,
                                            mut rock: krb5_clpreauth_rock,
                                            mut request: *mut krb5_kdc_req,
                                            mut encoded_request_body:
                                                *mut krb5_data,
                                            mut encoded_previous_request:
                                                *mut krb5_data,
                                            mut pa_type: krb5_preauthtype,
                                            mut err_reply: *mut krb5_error,
                                            mut err_padata:
                                                *mut *mut krb5_pa_data,
                                            mut prompter: krb5_prompter_fct,
                                            mut prompter_data:
                                                *mut libc::c_void,
                                            mut out_padata:
                                                *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code =
        -(1765328360 as libc::c_long) as krb5_error_code;
    let mut plgctx: pkinit_context = moddata as pkinit_context;
    let mut reqctx: pkinit_req_context = modreq as pkinit_req_context;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut certifiers: *mut *mut krb5_external_principal_identifier =
        0 as *mut *mut krb5_external_principal_identifier;
    let mut algId: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    let mut do_again: libc::c_int = 0 as libc::c_int;
    pkiDebug(b"pkinit_client_tryagain %p %p %p %p\n\x00" as *const u8 as
                 *const libc::c_char, context, plgctx, reqctx, request);
    if (*reqctx).pa_type != pa_type || err_padata.is_null() { return retval }
    loop  {
        if !(!(*err_padata).is_null() && do_again == 0) {
            current_block = 3437258052017859086;
            break ;
        }
        pa = *err_padata;
        scratch.length = (*pa).length;
        scratch.data = (*pa).contents as *mut libc::c_char;
        match (*pa).pa_type {
            104 | 105 => {
                retval =
                    k5int_decode_krb5_td_trusted_certifiers.expect("non-null function pointer")(&mut scratch,
                                                                                                &mut certifiers);
                if retval != 0 {
                    pkiDebug(b"failed to decode sequence of trusted certifiers\n\x00"
                                 as *const u8 as *const libc::c_char);
                    current_block = 17569042085630905371;
                    break ;
                } else {
                    retval =
                        pkinit_process_td_trusted_certifiers(context,
                                                             (*plgctx).cryptoctx,
                                                             (*reqctx).cryptoctx,
                                                             (*reqctx).idctx,
                                                             certifiers,
                                                             (*pa).pa_type);
                    if retval == 0 { do_again = 1 as libc::c_int }
                }
            }
            109 => {
                retval =
                    k5int_decode_krb5_td_dh_parameters.expect("non-null function pointer")(&mut scratch,
                                                                                           &mut algId);
                if retval != 0 {
                    pkiDebug(b"failed to decode td_dh_parameters\n\x00" as
                                 *const u8 as *const libc::c_char);
                    current_block = 17569042085630905371;
                    break ;
                } else {
                    retval =
                        pkinit_process_td_dh_params(context,
                                                    (*plgctx).cryptoctx,
                                                    (*reqctx).cryptoctx,
                                                    (*reqctx).idctx, algId,
                                                    &mut (*(*reqctx).opts).dh_size);
                    if retval == 0 { do_again = 1 as libc::c_int }
                }
            }
            _ => { }
        }
        err_padata = err_padata.offset(1)
    }
    match current_block {
        3437258052017859086 => {
            if do_again != 0 {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT client trying again with KDC-provided parameters\x00"
                                      as *const u8 as *const libc::c_char);
                }
                retval =
                    pa_pkinit_gen_req(context, plgctx, reqctx, cb, rock,
                                      request, pa_type, out_padata, prompter,
                                      prompter_data, gic_opt);
                if retval != 0 {
                    current_block = 17569042085630905371;
                } else { current_block = 2604890879466389055; }
            } else { current_block = 2604890879466389055; }
            match current_block {
                17569042085630905371 => { }
                _ => { retval = 0 as libc::c_int }
            }
        }
        _ => { }
    }
    if !certifiers.is_null() {
        free_krb5_external_principal_identifier(&mut certifiers);
    }
    if !algId.is_null() { free_krb5_algorithm_identifiers(&mut algId); }
    pkiDebug(b"pkinit_client_tryagain: returning %d (%s)\n\x00" as *const u8
                 as *const libc::c_char, retval,
             error_message(retval as errcode_t));
    return retval;
}
#[c2rust::src_loc = "1248:1"]
unsafe extern "C" fn pkinit_client_get_flags(mut kcontext: krb5_context,
                                             mut patype: krb5_preauthtype)
 -> libc::c_int {
    if patype == 147 as libc::c_int || patype == 150 as libc::c_int {
        return 0x2 as libc::c_int
    }
    return 0x1 as libc::c_int;
}
/*
 * We want to be notified about KRB5_PADATA_PKINIT_KX in addition to the actual
 * pkinit patypes because RFC 6112 requires anonymous KDCs to send it. We use
 * that to determine whether to use the broken MIT 1.9 behavior of sending
 * ContentInfo rather than SignedData or the RFC 6112 behavior
 */
#[c2rust::src_loc = "1262:25"]
static mut supported_client_pa_types: [krb5_preauthtype; 5] =
    [17 as libc::c_int, 16 as libc::c_int, 147 as libc::c_int,
     150 as libc::c_int, 0 as libc::c_int];
#[c2rust::src_loc = "1270:1"]
unsafe extern "C" fn pkinit_client_req_init(mut context: krb5_context,
                                            mut moddata:
                                                krb5_clpreauth_moddata,
                                            mut modreq_out:
                                                *mut krb5_clpreauth_modreq) {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut reqctx: pkinit_req_context = 0 as pkinit_req_context;
    let mut plgctx: pkinit_context = moddata as pkinit_context;
    *modreq_out = 0 as krb5_clpreauth_modreq;
    reqctx =
        malloc(::std::mem::size_of::<_pkinit_req_context>() as libc::c_ulong)
            as pkinit_req_context;
    if reqctx.is_null() { return }
    memset(reqctx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_pkinit_req_context>() as libc::c_ulong);
    (*reqctx).magic = 0xdeadbeef as libc::c_uint;
    (*reqctx).cryptoctx = 0 as pkinit_req_crypto_context;
    (*reqctx).opts = 0 as *mut pkinit_req_opts;
    (*reqctx).idctx = 0 as pkinit_identity_crypto_context;
    (*reqctx).idopts = 0 as *mut pkinit_identity_opts;
    (*reqctx).freshness_token = 0 as *mut krb5_data;
    retval = pkinit_init_req_opts(&mut (*reqctx).opts);
    if !(retval != 0) {
        (*(*reqctx).opts).require_eku = (*(*plgctx).opts).require_eku;
        (*(*reqctx).opts).accept_secondary_eku =
            (*(*plgctx).opts).accept_secondary_eku;
        (*(*reqctx).opts).dh_or_rsa = (*(*plgctx).opts).dh_or_rsa;
        (*(*reqctx).opts).allow_upn = (*(*plgctx).opts).allow_upn;
        (*(*reqctx).opts).require_crl_checking =
            (*(*plgctx).opts).require_crl_checking;
        (*(*reqctx).opts).disable_freshness =
            (*(*plgctx).opts).disable_freshness;
        retval = pkinit_init_req_crypto(&mut (*reqctx).cryptoctx);
        if !(retval != 0) {
            retval = pkinit_init_identity_crypto(&mut (*reqctx).idctx);
            if !(retval != 0) {
                retval =
                    pkinit_dup_identity_opts((*plgctx).idopts,
                                             &mut (*reqctx).idopts);
                if !(retval != 0) {
                    *modreq_out = reqctx as krb5_clpreauth_modreq;
                    pkiDebug(b"%s: returning reqctx at %p\n\x00" as *const u8
                                 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 23],
                                                       &[libc::c_char; 23]>(b"pkinit_client_req_init\x00")).as_ptr(),
                             reqctx);
                }
            }
        }
    }
    if retval != 0 {
        if !(*reqctx).idctx.is_null() {
            pkinit_fini_identity_crypto((*reqctx).idctx);
        }
        if !(*reqctx).cryptoctx.is_null() {
            pkinit_fini_req_crypto((*reqctx).cryptoctx);
        }
        if !(*reqctx).opts.is_null() { pkinit_fini_req_opts((*reqctx).opts); }
        if !(*reqctx).idopts.is_null() {
            pkinit_fini_identity_opts((*reqctx).idopts);
        }
        free(reqctx as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1335:1"]
unsafe extern "C" fn pkinit_client_req_fini(mut context: krb5_context,
                                            mut moddata:
                                                krb5_clpreauth_moddata,
                                            mut modreq:
                                                krb5_clpreauth_modreq) {
    let mut reqctx: pkinit_req_context = modreq as pkinit_req_context;
    pkiDebug(b"%s: received reqctx at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 23],
                                       &[libc::c_char; 23]>(b"pkinit_client_req_fini\x00")).as_ptr(),
             reqctx);
    if reqctx.is_null() { return }
    if (*reqctx).magic != 0xdeadbeef as libc::c_uint {
        pkiDebug(b"%s: Bad magic value (%x) in req ctx\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 23],
                                           &[libc::c_char; 23]>(b"pkinit_client_req_fini\x00")).as_ptr(),
                 (*reqctx).magic);
        return
    }
    if !(*reqctx).opts.is_null() { pkinit_fini_req_opts((*reqctx).opts); }
    if !(*reqctx).cryptoctx.is_null() {
        pkinit_fini_req_crypto((*reqctx).cryptoctx);
    }
    if !(*reqctx).idctx.is_null() {
        pkinit_fini_identity_crypto((*reqctx).idctx);
    }
    if !(*reqctx).idopts.is_null() {
        pkinit_fini_identity_opts((*reqctx).idopts);
    }
    krb5_free_data(context, (*reqctx).freshness_token);
    free(reqctx as *mut libc::c_void);
}
#[c2rust::src_loc = "1367:1"]
unsafe extern "C" fn pkinit_client_plugin_init(mut context: krb5_context,
                                               mut moddata_out:
                                                   *mut krb5_clpreauth_moddata)
 -> libc::c_int {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut ctx: pkinit_context = 0 as pkinit_context;
    ctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_pkinit_context>() as libc::c_ulong) as
            pkinit_context;
    if ctx.is_null() { return 12 as libc::c_int }
    memset(ctx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_pkinit_context>() as libc::c_ulong);
    (*ctx).magic = 0x5551212 as libc::c_int;
    (*ctx).opts = 0 as *mut pkinit_plg_opts;
    (*ctx).cryptoctx = 0 as pkinit_plg_crypto_context;
    (*ctx).idopts = 0 as *mut pkinit_identity_opts;
    retval = pkinit_accessor_init();
    if !(retval != 0) {
        retval = pkinit_init_plg_opts(&mut (*ctx).opts);
        if !(retval != 0) {
            retval = pkinit_init_plg_crypto(&mut (*ctx).cryptoctx);
            if !(retval != 0) {
                retval = pkinit_init_identity_opts(&mut (*ctx).idopts);
                if !(retval != 0) {
                    *moddata_out = ctx as krb5_clpreauth_moddata;
                    pkiDebug(b"%s: returning plgctx at %p\n\x00" as *const u8
                                 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 26],
                                                       &[libc::c_char; 26]>(b"pkinit_client_plugin_init\x00")).as_ptr(),
                             ctx);
                }
            }
        }
    }
    if retval != 0 {
        pkinit_client_plugin_fini(context, ctx as krb5_clpreauth_moddata);
    }
    return retval;
}
#[c2rust::src_loc = "1410:1"]
unsafe extern "C" fn pkinit_client_plugin_fini(mut context: krb5_context,
                                               mut moddata:
                                                   krb5_clpreauth_moddata) {
    let mut ctx: pkinit_context = moddata as pkinit_context;
    if ctx.is_null() || (*ctx).magic != 0x5551212 as libc::c_int {
        pkiDebug(b"pkinit_lib_fini: got bad plgctx (%p)!\n\x00" as *const u8
                     as *const libc::c_char, ctx);
        return
    }
    pkiDebug(b"%s: got plgctx at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 26],
                                       &[libc::c_char; 26]>(b"pkinit_client_plugin_fini\x00")).as_ptr(),
             ctx);
    pkinit_fini_identity_opts((*ctx).idopts);
    pkinit_fini_plg_crypto((*ctx).cryptoctx);
    pkinit_fini_plg_opts((*ctx).opts);
    free(ctx as *mut libc::c_void);
}
#[c2rust::src_loc = "1428:1"]
unsafe extern "C" fn add_string_to_array(mut context: krb5_context,
                                         mut array:
                                             *mut *mut *mut libc::c_char,
                                         mut addition: *const libc::c_char)
 -> krb5_error_code {
    let mut a: *mut *mut libc::c_char = *array;
    let mut len: size_t = 0;
    len = 0 as libc::c_int as size_t;
    while !a.is_null() && !(*a.offset(len as isize)).is_null() {
        len = len.wrapping_add(1)
    }
    a =
        realloc(a as *mut libc::c_void,
                len.wrapping_add(2 as libc::c_int as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                     as
                                                                     libc::c_ulong))
            as *mut *mut libc::c_char;
    if a.is_null() { return 12 as libc::c_int }
    *array = a;
    let ref mut fresh2 = *a.offset(len as isize);
    *fresh2 = strdup(addition);
    if (*a.offset(len as isize)).is_null() { return 12 as libc::c_int }
    let ref mut fresh3 =
        *a.offset(len.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                      isize);
    *fresh3 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1446:1"]
unsafe extern "C" fn handle_gic_opt(mut context: krb5_context,
                                    mut plgctx: pkinit_context,
                                    mut attr: *const libc::c_char,
                                    mut value: *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    if strcmp(attr,
              b"X509_user_identity\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        if !(*(*plgctx).idopts).identity.is_null() {
            krb5_set_error_message(context,
                                   -(1765328174 as libc::c_long) as
                                       krb5_error_code,
                                   b"X509_user_identity can not be given twice\n\x00"
                                       as *const u8 as *const libc::c_char);
            return -(1765328174 as libc::c_long) as krb5_error_code
        }
        (*(*plgctx).idopts).identity = strdup(value);
        if (*(*plgctx).idopts).identity.is_null() {
            krb5_set_error_message(context, 12 as libc::c_int,
                                   b"Could not duplicate X509_user_identity value\n\x00"
                                       as *const u8 as *const libc::c_char);
            return 12 as libc::c_int
        }
    } else if strcmp(attr,
                     b"X509_anchors\x00" as *const u8 as *const libc::c_char)
                  == 0 as libc::c_int {
        retval =
            add_string_to_array(context, &mut (*(*plgctx).idopts).anchors,
                                value);
        if retval != 0 { return retval }
    } else if strcmp(attr,
                     b"flag_RSA_PROTOCOL\x00" as *const u8 as
                         *const libc::c_char) == 0 as libc::c_int {
        if strcmp(value, b"yes\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            pkiDebug(b"Setting flag to use RSA_PROTOCOL\n\x00" as *const u8 as
                         *const libc::c_char);
            (*(*plgctx).opts).dh_or_rsa = 2 as libc::c_int
        }
    } else if strcmp(attr,
                     b"disable_freshness\x00" as *const u8 as
                         *const libc::c_char) == 0 as libc::c_int {
        if strcmp(value, b"yes\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*(*plgctx).opts).disable_freshness = 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1482:1"]
unsafe extern "C" fn pkinit_client_gic_opt(mut context: krb5_context,
                                           mut moddata:
                                               krb5_clpreauth_moddata,
                                           mut gic_opt:
                                               *mut krb5_get_init_creds_opt,
                                           mut attr: *const libc::c_char,
                                           mut value: *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut plgctx: pkinit_context = moddata as pkinit_context;
    pkiDebug(b"(pkinit) received \'%s\' = \'%s\'\n\x00" as *const u8 as
                 *const libc::c_char, attr, value);
    retval = handle_gic_opt(context, plgctx, attr, value);
    if retval != 0 { return retval }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1503:1"]
pub unsafe extern "C" fn clpreauth_pkinit_initvt(mut context: krb5_context,
                                                 mut maj_ver: libc::c_int,
                                                 mut min_ver: libc::c_int,
                                                 mut vtable:
                                                     krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_clpreauth_vtable = 0 as *mut krb5_clpreauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_clpreauth_vtable;
    (*vt).name =
        b"pkinit\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*vt).pa_type_list = supported_client_pa_types.as_mut_ptr();
    (*vt).init =
        Some(pkinit_client_plugin_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut krb5_clpreauth_moddata)
                     -> libc::c_int);
    (*vt).fini =
        Some(pkinit_client_plugin_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata) -> ());
    (*vt).flags =
        Some(pkinit_client_get_flags as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                     -> libc::c_int);
    (*vt).request_init =
        Some(pkinit_client_req_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: *mut krb5_clpreauth_modreq) -> ());
    (*vt).prep_questions =
        Some(pkinit_client_prep_questions as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: krb5_clpreauth_modreq,
                                      _: *mut krb5_get_init_creds_opt,
                                      _: krb5_clpreauth_callbacks,
                                      _: krb5_clpreauth_rock,
                                      _: *mut krb5_kdc_req, _: *mut krb5_data,
                                      _: *mut krb5_data, _: *mut krb5_pa_data)
                     -> krb5_error_code);
    (*vt).request_fini =
        Some(pkinit_client_req_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: krb5_clpreauth_modreq) -> ());
    (*vt).process =
        Some(pkinit_client_process as
                 unsafe extern "C" fn(_: krb5_context,
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
                     -> krb5_error_code);
    (*vt).tryagain =
        Some(pkinit_client_tryagain as
                 unsafe extern "C" fn(_: krb5_context,
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
                     -> krb5_error_code);
    (*vt).gic_opts =
        Some(pkinit_client_gic_opt as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_clpreauth_moddata,
                                      _: *mut krb5_get_init_creds_opt,
                                      _: *const libc::c_char,
                                      _: *const libc::c_char)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
