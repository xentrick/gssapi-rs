use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:55"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:55"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:55"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:55"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:55"]
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2710:1"]
    pub type krb5_rcache = *mut krb5_rc_st;
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
    use super::k5_int_h::{_krb5_context, _krb5_kt};
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:55"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:55"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:55"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:55"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:55"]
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
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* token_buffer */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:55"]
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
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_ctx_id_struct,
                          gss_ctx_id_t, gss_OID_desc_struct, gss_OID,
                          gss_cred_id_t, gss_buffer_desc_struct,
                          gss_buffer_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn gss_release_buffer_set(_: *mut OM_uint32,
                                      _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn gss_inquire_sec_context_by_oid(_: *mut OM_uint32,
                                              _: gss_ctx_id_t, _: gss_OID,
                                              _: *mut gss_buffer_set_t)
         -> OM_uint32;
        /*
 * Heimdal extension
 */
        #[no_mangle]
        #[c2rust::src_loc = "188:1"]
        pub fn gss_set_cred_option(_: *mut OM_uint32, _: *mut gss_cred_id_t,
                                   _: gss_OID, _: gss_buffer_t) -> OM_uint32;
        /*
 * Call the given method on the given mechanism
 */
        #[no_mangle]
        #[c2rust::src_loc = "197:1"]
        pub fn gssspi_mech_invoke(_: *mut OM_uint32, _: gss_OID, _: gss_OID,
                                  _: gss_buffer_t) -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:55"]
pub mod gssapiP_krb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1059:8"]
    pub struct krb5_gss_ccache_name_req {
        pub name: *const libc::c_char,
        pub out_name: *mut *const libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1077:8"]
    pub struct krb5_gss_set_allowable_enctypes_req {
        pub num_ktypes: OM_uint32,
        pub ktypes: *mut krb5_enctype,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1154:8"]
    pub struct krb5_gss_import_cred_req {
        pub id: krb5_ccache,
        pub keytab_principal: krb5_principal,
        pub keytab: krb5_keytab,
    }
    use super::gssapi_h::OM_uint32;
    use super::krb5_h::{krb5_enctype, krb5_ccache, krb5_principal,
                        krb5_keytab};
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:55"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:55"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{OM_uint32, gss_OID_desc};
    use super::stddef_h::size_t;
    use super::gssapi_ext_h::gss_buffer_set_t;
    extern "C" {
        /* oid */
        #[no_mangle]
        #[c2rust::src_loc = "242:1"]
        pub fn generic_gss_oid_compose(_: *mut OM_uint32,
                                       _: *const libc::c_char, _: size_t,
                                       _: libc::c_int, _: *mut gss_OID_desc)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "312:1"]
        pub fn generic_gss_release_buffer_set(_: *mut OM_uint32,
                                              _: *mut gss_buffer_set_t)
         -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapi_krb5.h:55"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_enctype, krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, krb5_ccache, krb5_rcache,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       _krb5_ccache, krb5_rc_st};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_cred_id_t, gss_ctx_id_t,
                         gss_OID_desc_struct, gss_OID_desc, gss_OID,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_cred_id_struct, gss_ctx_id_struct,
                         gss_delete_sec_context};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_release_buffer_set,
                             gss_inquire_sec_context_by_oid,
                             gss_set_cred_option, gssspi_mech_invoke};
pub use self::gssapiP_krb5_h::{krb5_gss_ccache_name_req,
                               krb5_gss_set_allowable_enctypes_req,
                               krb5_gss_import_cred_req};
use self::string_h::strlen;
use self::gssapiP_generic_h::{generic_gss_oid_compose,
                              generic_gss_release_buffer_set};
use self::gssapi_krb5_h::gss_mech_krb5;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * $Id$
 */
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn gss_krb5_get_tkt_flags(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    gss_ctx_id_t,
                                                mut ticket_flags:
                                                    *mut krb5_flags)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x01\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut data_set: gss_buffer_set_t = 0 as gss_buffer_set_t;
    if ticket_flags.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    major_status =
        gss_inquire_sec_context_by_oid(minor_status, context_handle,
                                       &req_oid as *const gss_OID_desc as
                                           gss_OID, &mut data_set);
    if major_status != 0 as libc::c_int as libc::c_uint {
        return major_status
    }
    if data_set.is_null() ||
           (*data_set).count != 1 as libc::c_int as libc::c_ulong ||
           (*(*data_set).elements.offset(0 as libc::c_int as isize)).length !=
               ::std::mem::size_of::<krb5_flags>() as libc::c_ulong {
        *minor_status = 22 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *ticket_flags =
        *((*(*data_set).elements.offset(0 as libc::c_int as isize)).value as
              *mut krb5_flags);
    gss_release_buffer_set(minor_status, &mut data_set);
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "94:1"]
pub unsafe extern "C" fn gss_krb5_copy_ccache(mut minor_status:
                                                  *mut OM_uint32,
                                              mut cred_handle: gss_cred_id_t,
                                              mut out_ccache: krb5_ccache)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x02\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    if out_ccache.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    req_buffer.value = out_ccache as *mut libc::c_void;
    req_buffer.length = ::std::mem::size_of::<krb5_ccache>() as libc::c_ulong;
    major_status =
        gss_set_cred_option(minor_status, &mut cred_handle,
                            &req_oid as *const gss_OID_desc as gss_OID,
                            &mut req_buffer);
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn gss_krb5_import_cred(mut minor_status:
                                                  *mut OM_uint32,
                                              mut id: krb5_ccache,
                                              mut keytab_principal:
                                                  krb5_principal,
                                              mut keytab: krb5_keytab,
                                              mut cred: *mut gss_cred_id_t)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\r\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut req: krb5_gss_import_cred_req =
        krb5_gss_import_cred_req{id: 0 as *mut _krb5_ccache,
                                 keytab_principal:
                                     0 as *mut krb5_principal_data,
                                 keytab: 0 as *mut _krb5_kt,};
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    if cred.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *cred = 0 as gss_cred_id_t;
    req.id = id;
    req.keytab_principal = keytab_principal;
    req.keytab = keytab;
    req_buffer.value =
        &mut req as *mut krb5_gss_import_cred_req as *mut libc::c_void;
    req_buffer.length =
        ::std::mem::size_of::<krb5_gss_import_cred_req>() as libc::c_ulong;
    major_status =
        gss_set_cred_option(minor_status, cred,
                            &req_oid as *const gss_OID_desc as gss_OID,
                            &mut req_buffer);
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn gss_krb5_export_lucid_sec_context(mut minor_status:
                                                               *mut OM_uint32,
                                                           mut context_handle:
                                                               *mut gss_ctx_id_t,
                                                           mut version:
                                                               OM_uint32,
                                                           mut kctx:
                                                               *mut *mut libc::c_void)
 -> OM_uint32 {
    let mut oid_buf: [libc::c_uchar; 17] = [0; 17];
    let mut req_oid: gss_OID_desc =
        gss_OID_desc{length: 0, elements: 0 as *mut libc::c_void,};
    let mut major_status: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut data_set: gss_buffer_set_t = 0 as gss_buffer_set_t;
    if kctx.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    *kctx = 0 as *mut libc::c_void;
    req_oid.elements = oid_buf.as_mut_ptr() as *mut libc::c_void;
    req_oid.length =
        ::std::mem::size_of::<[libc::c_uchar; 17]>() as libc::c_ulong as
            OM_uint32;
    major_status =
        generic_gss_oid_compose(minor_status,
                                b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x06\x00"
                                    as *const u8 as *const libc::c_char,
                                11 as libc::c_int as size_t,
                                version as libc::c_int, &mut req_oid);
    if major_status &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return major_status
    }
    major_status =
        gss_inquire_sec_context_by_oid(minor_status, *context_handle,
                                       &mut req_oid, &mut data_set);
    if major_status &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return major_status
    }
    if data_set.is_null() ||
           (*data_set).count != 1 as libc::c_int as libc::c_ulong ||
           (*(*data_set).elements.offset(0 as libc::c_int as isize)).length !=
               ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong {
        *minor_status = 22 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *kctx =
        *((*(*data_set).elements.offset(0 as libc::c_int as isize)).value as
              *mut *mut libc::c_void);
    /* Clean up the context state (it is an error for
     * someone to attempt to use this context again)
     */
    gss_delete_sec_context(minor_status, context_handle, 0 as gss_buffer_t);
    *context_handle = 0 as gss_ctx_id_t;
    generic_gss_release_buffer_set(&mut minor, &mut data_set);
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "207:1"]
pub unsafe extern "C" fn gss_krb5_set_allowable_enctypes(mut minor_status:
                                                             *mut OM_uint32,
                                                         mut cred:
                                                             gss_cred_id_t,
                                                         mut num_ktypes:
                                                             OM_uint32,
                                                         mut ktypes:
                                                             *mut krb5_enctype)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x04\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut req: krb5_gss_set_allowable_enctypes_req =
        krb5_gss_set_allowable_enctypes_req{num_ktypes: 0,
                                            ktypes: 0 as *mut krb5_enctype,};
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    req.num_ktypes = num_ktypes;
    req.ktypes = ktypes;
    req_buffer.length =
        ::std::mem::size_of::<krb5_gss_set_allowable_enctypes_req>() as
            libc::c_ulong;
    req_buffer.value =
        &mut req as *mut krb5_gss_set_allowable_enctypes_req as
            *mut libc::c_void;
    major_status =
        gss_set_cred_option(minor_status, &mut cred,
                            &req_oid as *const gss_OID_desc as gss_OID,
                            &mut req_buffer);
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "234:1"]
pub unsafe extern "C" fn gss_krb5_ccache_name(mut minor_status:
                                                  *mut OM_uint32,
                                              mut name: *const libc::c_char,
                                              mut out_name:
                                                  *mut *const libc::c_char)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x03\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut req: krb5_gss_ccache_name_req =
        krb5_gss_ccache_name_req{name: 0 as *const libc::c_char,
                                 out_name: 0 as *mut *const libc::c_char,};
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    req.name = name;
    req.out_name = out_name;
    req_buffer.length =
        ::std::mem::size_of::<krb5_gss_ccache_name_req>() as libc::c_ulong;
    req_buffer.value =
        &mut req as *mut krb5_gss_ccache_name_req as *mut libc::c_void;
    major_status =
        gssspi_mech_invoke(minor_status, gss_mech_krb5,
                           &req_oid as *const gss_OID_desc as gss_OID,
                           &mut req_buffer);
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "260:1"]
pub unsafe extern "C" fn gss_krb5_free_lucid_sec_context(mut minor_status:
                                                             *mut OM_uint32,
                                                         mut kctx:
                                                             *mut libc::c_void)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x07\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    req_buffer.length =
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong;
    req_buffer.value = kctx;
    major_status =
        gssspi_mech_invoke(minor_status, gss_mech_krb5,
                           &req_oid as *const gss_OID_desc as gss_OID,
                           &mut req_buffer);
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "280:1"]
pub unsafe extern "C" fn krb5_gss_register_acceptor_identity(mut keytab:
                                                                 *const libc::c_char)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\t\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut minor_status: OM_uint32 = 0;
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    req_buffer.length =
        if keytab.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else { strlen(keytab) };
    req_buffer.value = keytab as *mut libc::c_char as *mut libc::c_void;
    major_status =
        gssspi_mech_invoke(&mut minor_status, gss_mech_krb5,
                           &req_oid as *const gss_OID_desc as gss_OID,
                           &mut req_buffer);
    return major_status;
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
#[no_mangle]
#[c2rust::src_loc = "302:1"]
pub unsafe extern "C" fn krb5_gss_use_kdc_context() -> krb5_error_code {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x08\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut minor_status: OM_uint32 = 0;
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut ret: krb5_error_code = 0;
    req_buffer.length = 0 as libc::c_int as size_t;
    req_buffer.value = 0 as *mut libc::c_void;
    major_status =
        gssspi_mech_invoke(&mut minor_status, gss_mech_krb5,
                           &req_oid as *const gss_OID_desc as gss_OID,
                           &mut req_buffer);
    if major_status != 0 as libc::c_int as libc::c_uint {
        if minor_status != 0 as libc::c_int as libc::c_uint {
            ret = minor_status as krb5_error_code
        } else { ret = -(1765328324 as libc::c_long) as krb5_error_code }
    } else { ret = 0 as libc::c_int }
    return ret;
}
/*
 * This API should go away and be replaced with an accessor
 * into a gss_name_t.
 */
#[no_mangle]
#[c2rust::src_loc = "337:1"]
pub unsafe extern "C" fn gsskrb5_extract_authz_data_from_sec_context(mut minor_status:
                                                                         *mut OM_uint32,
                                                                     context_handle:
                                                                         gss_ctx_id_t,
                                                                     mut ad_type:
                                                                         libc::c_int,
                                                                     mut ad_data:
                                                                         gss_buffer_t)
 -> OM_uint32 {
    let mut req_oid: gss_OID_desc =
        gss_OID_desc{length: 0, elements: 0 as *mut libc::c_void,};
    let mut oid_buf: [libc::c_uchar; 17] = [0; 17];
    let mut major_status: OM_uint32 = 0;
    let mut data_set: gss_buffer_set_t = 0 as gss_buffer_set_t;
    if ad_data.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    req_oid.elements = oid_buf.as_mut_ptr() as *mut libc::c_void;
    req_oid.length =
        ::std::mem::size_of::<[libc::c_uchar; 17]>() as libc::c_ulong as
            OM_uint32;
    major_status =
        generic_gss_oid_compose(minor_status,
                                b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\n\x00" as
                                    *const u8 as *const libc::c_char,
                                11 as libc::c_int as size_t, ad_type,
                                &mut req_oid);
    if major_status &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return major_status
    }
    major_status =
        gss_inquire_sec_context_by_oid(minor_status, context_handle,
                                       &mut req_oid as *mut gss_OID_desc as
                                           gss_OID, &mut data_set);
    if major_status != 0 as libc::c_int as libc::c_uint {
        return major_status
    }
    if data_set.is_null() ||
           (*data_set).count != 1 as libc::c_int as libc::c_ulong {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*ad_data).length =
        (*(*data_set).elements.offset(0 as libc::c_int as isize)).length;
    (*ad_data).value =
        (*(*data_set).elements.offset(0 as libc::c_int as isize)).value;
    (*(*data_set).elements.offset(0 as libc::c_int as isize)).length =
        0 as libc::c_int as size_t;
    let ref mut fresh0 =
        (*(*data_set).elements.offset(0 as libc::c_int as isize)).value;
    *fresh0 = 0 as *mut libc::c_void;
    (*data_set).count = 0 as libc::c_int as size_t;
    gss_release_buffer_set(minor_status, &mut data_set);
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "388:1"]
pub unsafe extern "C" fn gss_krb5_set_cred_rcache(mut minor_status:
                                                      *mut OM_uint32,
                                                  mut cred: gss_cred_id_t,
                                                  mut rcache: krb5_rcache)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0b\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut req_buffer: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    req_buffer.length = ::std::mem::size_of::<krb5_rcache>() as libc::c_ulong;
    req_buffer.value = rcache as *mut libc::c_void;
    major_status =
        gss_set_cred_option(minor_status, &mut cred,
                            &req_oid as *const gss_OID_desc as gss_OID,
                            &mut req_buffer);
    return major_status;
}
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
/* C++ friendlyness */
/* __cplusplus */
/* Reserved static storage for GSS_oids.  See rfc 1964 for more details. */
/* 2.1.1. Kerberos Principal Name Form: */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5_name(1)}.  The recommended symbolic name for this type
 * is "GSS_KRB5_NT_PRINCIPAL_NAME". */
/* 2.1.2. Host-Based Service Name Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The previously recommended symbolic
 * name for this type is "GSS_KRB5_NT_HOSTBASED_SERVICE_NAME".  The
 * currently preferred symbolic name for this type is
 * "GSS_C_NT_HOSTBASED_SERVICE". */
/* 2.2.1. User Name Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) user_name(1)}.  The recommended symbolic name for this
 * type is "GSS_KRB5_NT_USER_NAME". */
/* 2.2.2. Machine UID Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) machine_uid_name(2)}.  The recommended symbolic name for
 * this type is "GSS_KRB5_NT_MACHINE_UID_NAME". */
/* 2.2.3. String UID Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) string_uid_name(3)}.  The recommended symbolic name for
 * this type is "GSS_KRB5_NT_STRING_UID_NAME". */
/* Kerberos Enterprise Name Form (see RFC 6806 section 5): */
/* {iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-enterprise-name(6)}. */
/*
 * This OID can be used with gss_set_cred_option() to suppress the
 * confidentiality and integrity flags from being asserted in initial context
 * tokens.
 *
 * iso(1) member-body(2) Sweden(752) Stockholm University(43) Heimdal GSS-API
 * Extensions(13) no_ci_flags(29)
 */
/*
 * This OID can be used with gss_inquire_cred_by_oid(0 to retrieve the
 * impersonator name (if any).
 *
 * iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-gssapi-ext(5) get-cred-impersonator(14)
 */
/* key encryption type */
/* length of key data */
/* actual key data */
/* signing algorthm */
/* seal/encrypt algorithm */
/* Context key
       (Kerberos session key or subkey) */
/* 1 if there is an acceptor_subkey
       present, 0 otherwise */
/* Context key
       (Kerberos session key or subkey) */
/* acceptor-asserted subkey or
       0's if no acceptor subkey */
/* Structure version number (1)
                                       MUST be at beginning of struct! */
/* Are we the initiator? */
/* expiration time of context */
/* sender sequence number */
/* receive sequence number */
/* 0: rfc1964,
                                       1: draft-ietf-krb-wg-gssapi-cfx-07 */
/*
     * if (protocol == 0) rfc1964_kd should be used
     * and cfx_kd contents are invalid and should be zero
     * if (protocol == 1) cfx_kd should be used
     * and rfc1964_kd contents are invalid and should be zero
     */
/*
 * Mask for determining the version of a lucid context structure.  Callers
 * should not require this.
 */
/* Structure version number */
/* Alias for Heimdal compat. */
/*
 * Copy krb5 creds from cred_handle into out_ccache, which must already be
 * initialized.  Use gss_store_cred_into() (new in krb5 1.11) instead, if
 * possible.
 */
/*
 * gss_krb5_set_allowable_enctypes
 *
 * This function may be called by a context initiator after calling
 * gss_acquire_cred(), but before calling gss_init_sec_context(),
 * to restrict the set of enctypes which will be negotiated during
 * context establishment to those in the provided array.
 *
 * 'cred' must be a valid credential handle obtained via
 * gss_acquire_cred().  It may not be GSS_C_NO_CREDENTIAL.
 * gss_acquire_cred() may have been called to get a handle to
 * the default credential.
 *
 * The purpose of this function is to limit the keys that may
 * be exported via gss_krb5_export_lucid_sec_context(); thus it
 * should limit the enctypes of all keys that will be needed
 * after the security context has been established.
 * (i.e. context establishment may use a session key with a
 * stronger enctype than in the provided array, however a
 * subkey must be established within the enctype limits
 * established by this function.)
 *
 */
/*
 * Returns a non-opaque (lucid) version of the internal context
 * information.
 *
 * Note that context_handle must not be used again by the caller
 * after this call.  The GSS implementation is free to release any
 * resources associated with the original context.  It is up to the
 * GSS implementation whether it returns pointers to existing data,
 * or copies of the data.  The caller should treat the returned
 * lucid context as read-only.
 *
 * The caller must call gss_krb5_free_lucid_context() to free
 * the context and allocated resources when it is finished with it.
 *
 * 'version' is an integer indicating the requested version of the lucid
 * context.  If the implementation does not understand the requested version,
 * it will return an error.
 *
 * For example:
 *      void *return_ctx;
 *      gss_krb5_lucid_context_v1_t *ctx;
 *      OM_uint32 min_stat, maj_stat;
 *      OM_uint32 vers;
 *      gss_ctx_id_t *ctx_handle;
 *
 *      maj_stat = gss_krb5_export_lucid_sec_context(&min_stat,
 *                      ctx_handle, 1, &return_ctx);
 *      // Verify success
 *      ctx = (gss_krb5_lucid_context_v1_t *) return_ctx;
 */
/*
 * Frees the allocated storage associated with an
 * exported struct gss_krb5_lucid_context.
 */
#[no_mangle]
#[c2rust::src_loc = "410:1"]
pub unsafe extern "C" fn gsskrb5_extract_authtime_from_sec_context(mut minor_status:
                                                                       *mut OM_uint32,
                                                                   mut context_handle:
                                                                       gss_ctx_id_t,
                                                                   mut authtime:
                                                                       *mut krb5_timestamp)
 -> OM_uint32 {
    static mut req_oid: gss_OID_desc =
        {
            let mut init =
                gss_OID_desc_struct{length: 11 as libc::c_int as OM_uint32,
                                    elements:
                                        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0c\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_void,};
            init
        };
    let mut major_status: OM_uint32 = 0;
    let mut data_set: gss_buffer_set_t = 0 as gss_buffer_set_t;
    if authtime.is_null() {
        return (2 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
    }
    major_status =
        gss_inquire_sec_context_by_oid(minor_status, context_handle,
                                       &req_oid as *const gss_OID_desc as
                                           gss_OID, &mut data_set);
    if major_status != 0 as libc::c_int as libc::c_uint {
        return major_status
    }
    if data_set.is_null() ||
           (*data_set).count != 1 as libc::c_int as libc::c_ulong ||
           (*(*data_set).elements.offset(0 as libc::c_int as isize)).length !=
               ::std::mem::size_of::<krb5_timestamp>() as libc::c_ulong {
        *minor_status = 22 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *authtime =
        *((*(*data_set).elements.offset(0 as libc::c_int as isize)).value as
              *mut krb5_timestamp);
    gss_release_buffer_set(minor_status, &mut data_set);
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
