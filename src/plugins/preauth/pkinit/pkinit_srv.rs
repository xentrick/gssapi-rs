use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
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
    use super::types_h::__uint8_t;
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
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Last request entry */
    #[c2rust::src_loc = "2031:1"]
    pub type krb5_last_req_entry = _krb5_last_req_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< LR type */
    /* *< Timestamp */
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
    #[c2rust::src_loc = "2079:16"]
    pub struct _krb5_enc_kdc_rep_part {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub session: *mut krb5_keyblock,
        pub last_req: *mut *mut krb5_last_req_entry,
        pub nonce: krb5_int32,
        pub key_exp: krb5_timestamp,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub server: krb5_principal,
        pub caddrs: *mut *mut krb5_address,
        pub enc_padata: *mut *mut krb5_pa_data,
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
 * C representation of @c EncKDCRepPart protocol message.
 *
 * This is the cleartext message that is encrypted and inserted in @c KDC-REP.
 */
    #[c2rust::src_loc = "2079:1"]
    pub type krb5_enc_kdc_rep_part = _krb5_enc_kdc_rep_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2095:16"]
    pub struct _krb5_kdc_rep {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub client: krb5_principal,
        pub ticket: *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_kdc_rep_part,
    }
    /* encrypted part: */
    /* *< krb5 message type */
    /* *< Session key */
    /* *< Array of pointers to entries */
    /* *< Nonce from request */
    /* *< Expiration date */
    /* *< Ticket flags */
    /* *< Lifetime info */
    /* *< Server's principal identifier */
    /* *< Array of ptrs to addrs, optional */
    /* *< Encrypted preauthentication data */
    /* * Representation of the @c KDC-REP protocol message. */
    #[c2rust::src_loc = "2095:1"]
    pub type krb5_kdc_rep = _krb5_kdc_rep;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* cleartext part: */
        /* *< KRB5_AS_REP or KRB5_KDC_REP */
        /* *< Preauthentication data from KDC */
        /* *< Client principal and realm */
        /* *< Ticket */
        /* *< Encrypted part of reply */
        /* *< Unencrypted version, if available */
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
 * Compute the KRB-FX-CF2 combination of two keys and pepper strings.
 *
 * @param [in]  context         Library context
 * @param [in]  k1              KDC contribution key
 * @param [in]  pepper1         String "PKINIT"
 * @param [in]  k2              Reply key
 * @param [in]  pepper2         String "KeyExchange"
 * @param [out] out             Output key
 *
 * This function computes the KRB-FX-CF2 function over its inputs and places
 * the results in a newly allocated keyblock.  This function is simple in that
 * it assumes that @a pepper1 and @a pepper2 are C strings with no internal
 * nulls and that the enctype of the result will be the same as that of @a k1.
 * @a k1 and @a k2 may be of different enctypes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "727:1"]
        pub fn krb5_c_fx_cf2_simple(context: krb5_context,
                                    k1: *const krb5_keyblock,
                                    pepper1: *const libc::c_char,
                                    k2: *const krb5_keyblock,
                                    pepper2: *const libc::c_char,
                                    out: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Generate an enctype-specific random encryption key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type of the generated key
 * @param [out] k5_random_key   An allocated and initialized keyblock
 *
 * Use krb5_free_keyblock_contents() to free @a k5_random_key when
 * no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "745:1"]
        pub fn krb5_c_make_random_key(context: krb5_context,
                                      enctype: krb5_enctype,
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
        /* *< See RFC 6560 section 4.2 */
        /* define in draft-ietf-krb-wg-preauth-framework*/
        /* Key usage values 512-1023 are reserved for uses internal to a Kerberos
 * implementation. */
        /* *< Used for encrypted FAST cookies */
        /* *< Used for freshness tokens */
        /* * @} */
        /* end of KRB5_KEYUSAGE group */
        /* *
 * Verify that a specified encryption type is a valid Kerberos encryption type.
 *
 * @param [in] ktype            Encryption type
 *
 * @return @c TRUE if @a ktype is valid, @c FALSE if not
 */
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
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
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
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
        /* *
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
        #[no_mangle]
        #[c2rust::src_loc = "4854:1"]
        pub fn krb5_check_clockskew(context: krb5_context,
                                    date: krb5_timestamp) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "7926:1"]
        pub fn krb5_prepend_error_message(ctx: krb5_context,
                                          code: krb5_error_code,
                                          fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
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
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2315:1"]
    pub unsafe extern "C" fn k5memdup(mut in_0: *const libc::c_void,
                                      mut len: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = k5alloc(len, code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::plugin_h::krb5_plugin_initvt_fn;
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
        #[no_mangle]
        #[c2rust::src_loc = "1168:1"]
        pub fn k5_plugin_load_all(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modules: *mut *mut krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1173:1"]
        pub fn k5_plugin_free_modules(context: krb5_context,
                                      modules: *mut krb5_plugin_initvt_fn);
        #[no_mangle]
        #[c2rust::src_loc = "1177:1"]
        pub fn k5_plugin_register(context: krb5_context,
                                  interface_id: libc::c_int,
                                  modname: *const libc::c_char,
                                  module: krb5_plugin_initvt_fn)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
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
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
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
    /*
 * krb5_plugin_initvt_fn is the type of all module initvt functions.  Based on
 * the maj_ver argument, the initvt function should cast vtable to the
 * appropriate type and then fill it in.  If a vtable has been expanded,
 * min_ver indicates which version of the vtable is being filled in.
 */
    #[c2rust::src_loc = "42:1"]
    pub type krb5_plugin_initvt_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int,
                                    _: libc::c_int, _: krb5_plugin_vtable)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_error_code, krb5_context};
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
                        krb5_keyblock, krb5_enc_data, krb5_error_code,
                        krb5_context, krb5_keyusage};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn encode_krb5_enc_data(_: *const krb5_enc_data,
                                    _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn encode_krb5_encryption_key(rep: *const krb5_keyblock,
                                          code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn krb5_encrypt_helper(context: krb5_context,
                                   key: *const krb5_keyblock,
                                   keyusage: krb5_keyusage,
                                   plain: *const krb5_data,
                                   cipher: *mut krb5_enc_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/clpreauth_plugin.h:33"]
pub mod clpreauth_plugin_h {
    /* Abstract type for a client request information handle. */
    #[c2rust::src_loc = "75:1"]
    pub type krb5_clpreauth_rock = *mut krb5_clpreauth_rock_st;
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
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
    use super::krb5_h::{krb5_enctype, krb5_context, krb5_keyblock,
                        krb5_error_code, krb5_boolean, krb5_timestamp,
                        krb5_int32};
    extern "C" {
        #[c2rust::src_loc = "75:16"]
        pub type krb5_clpreauth_rock_st;
    }
    /* End of version 3 clpreauth callbacks (added in 1.17). */
    /* KRB5_CLPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kdcpreauth_plugin.h:33"]
pub mod kdcpreauth_plugin_h {
    #[c2rust::src_loc = "111:1"]
    pub type krb5_kdcpreauth_rock = *mut krb5_kdcpreauth_rock_st;
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
 * Declarations for kdcpreauth plugin module implementors.
 *
 * The kdcpreauth interface has a single supported major version, which is 1.
 * Major version 1 has a current minor version of 2.  kdcpreauth modules should
 * define a function named kdcpreauth_<modulename>_initvt, matching the
 * signature:
 *
 *   krb5_error_code
 *   kdcpreauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                             krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *     kdcpreauth, maj_ver == 1: Cast to krb5_kdcpreauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* kdcpreauth mechanism property flags */
    /*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring hardware-based
 * preauthentication.
 */
    /*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring preauthentication,
 * and to fail preauthentication if we can't verify the client data.  The
 * flipside of PA_SUFFICIENT.
 */
    /*
 * Causes the KDC to include this mechanism in a list of supported preauth
 * types if the user's DB entry flags the user as requiring preauthentication,
 * and to mark preauthentication as successful if we can verify the client
 * data.  The flipside of PA_REQUIRED.
 */
    /*
 * Marks this preauthentication mechanism as one which changes the key which is
 * used for encrypting the response to the client.  Modules which have this
 * flag have their server_return_fn called before modules which do not, and are
 * passed over if a previously-called module has modified the encrypting key.
 */
    /*
 * Not really a padata type, so don't include it in any list of preauth types
 * which gets sent over the wire.
 */
    /*
 * Indicates that e_data in non-FAST errors should be encoded as typed data
 * instead of padata.
 */
    /* Abstract type for a KDC callback data handle. */
    /* Abstract type for module data and per-request module data. */
    #[c2rust::src_loc = "114:1"]
    pub type krb5_kdcpreauth_moddata = *mut krb5_kdcpreauth_moddata_st;
    #[c2rust::src_loc = "115:1"]
    pub type krb5_kdcpreauth_modreq = *mut krb5_kdcpreauth_modreq_st;
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "123:16"]
    pub struct krb5_kdcpreauth_callbacks_st {
        pub vers: libc::c_int,
        pub max_time_skew: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_kdcpreauth_rock)
                                      -> krb5_deltat>,
        pub client_keys: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_kdcpreauth_rock,
                                                     _:
                                                         *mut *mut krb5_keyblock)
                                    -> krb5_error_code>,
        pub free_keys: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_kdcpreauth_rock,
                                                   _: *mut krb5_keyblock)
                                  -> ()>,
        pub request_body: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: krb5_kdcpreauth_rock)
                                     -> *mut krb5_data>,
        pub fast_armor: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock)
                                   -> *mut krb5_keyblock>,
        pub get_string: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock,
                                                    _: *const libc::c_char,
                                                    _: *mut *mut libc::c_char)
                                   -> krb5_error_code>,
        pub free_string: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_kdcpreauth_rock,
                                                     _: *mut libc::c_char)
                                    -> ()>,
        pub client_entry: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: krb5_kdcpreauth_rock)
                                     -> *mut libc::c_void>,
        pub event_context: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_kdcpreauth_rock)
                                      -> *mut verto_ctx>,
        pub have_client_keys: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_kdcpreauth_rock)
                                         -> krb5_boolean>,
        pub client_keyblock: Option<unsafe extern "C" fn(_: krb5_context,
                                                         _:
                                                             krb5_kdcpreauth_rock)
                                        -> *const krb5_keyblock>,
        pub add_auth_indicator: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                krb5_kdcpreauth_rock,
                                                            _:
                                                                *const libc::c_char)
                                           -> krb5_error_code>,
        pub get_cookie: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock,
                                                    _: krb5_preauthtype,
                                                    _: *mut krb5_data)
                                   -> krb5_boolean>,
        pub set_cookie: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_kdcpreauth_rock,
                                                    _: krb5_preauthtype,
                                                    _: *const krb5_data)
                                   -> krb5_error_code>,
        pub match_client: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: krb5_kdcpreauth_rock,
                                                      _: krb5_principal)
                                     -> krb5_boolean>,
        pub client_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_kdcpreauth_rock)
                                    -> krb5_principal>,
        pub send_freshness_token: Option<unsafe extern "C" fn(_: krb5_context,
                                                              _:
                                                                  krb5_kdcpreauth_rock)
                                             -> ()>,
        pub check_freshness_token: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_kdcpreauth_rock,
                                                               _:
                                                                   *const krb5_data)
                                              -> krb5_error_code>,
    }
    #[c2rust::src_loc = "123:1"]
    pub type krb5_kdcpreauth_callbacks = *mut krb5_kdcpreauth_callbacks_st;
    /* End of version 5 kdcpreauth callbacks. */
    /* Optional: preauth plugin initialization function. */
    #[c2rust::src_loc = "263:1"]
    pub type krb5_kdcpreauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_kdcpreauth_moddata,
                                    _: *mut *const libc::c_char)
                   -> krb5_error_code>;
    /* Optional: preauth plugin cleanup function. */
    #[c2rust::src_loc = "269:1"]
    pub type krb5_kdcpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata) -> ()>;
    /*
 * Optional: return the flags which the KDC should use for this module.  This
 * is a callback instead of a static value because the module may or may not
 * wish to count itself as a hardware preauthentication module (in other words,
 * the flags may be affected by the configuration, for example if a site
 * administrator can force a particular preauthentication type to be supported
 * using only hardware).  This function is called for each entry entry in the
 * server_pa_type_list.
 */
    #[c2rust::src_loc = "282:1"]
    pub type krb5_kdcpreauth_flags_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                   -> libc::c_int>;
    /*
 * Responder for krb5_kdcpreauth_edata_fn.  If invoked with a non-zero code, pa
 * will be ignored and the padata type will not be included in the hint list.
 * If invoked with a zero code and a null pa value, the padata type will be
 * included in the list with an empty value.  If invoked with a zero code and a
 * non-null pa value, pa will be included in the hint list and will later be
 * freed by the KDC.
 */
    #[c2rust::src_loc = "293:1"]
    pub type krb5_kdcpreauth_edata_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: *mut krb5_pa_data) -> ()>;
    /*
 * Optional: provide pa_data to send to the client as part of the "you need to
 * use preauthentication" error.  The implementation must invoke the respond
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 *
 * This function is not allowed to create a modreq object because we have no
 * guarantee that the client will ever make a follow-up request, or that it
 * will hit this KDC if it does.
 */
    #[c2rust::src_loc = "307:1"]
    pub type krb5_kdcpreauth_edata_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_kdc_req,
                                    _: krb5_kdcpreauth_callbacks,
                                    _: krb5_kdcpreauth_rock,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_preauthtype,
                                    _: krb5_kdcpreauth_edata_respond_fn,
                                    _: *mut libc::c_void) -> ()>;
    /*
 * Responder for krb5_kdcpreauth_verify_fn.  Invoke with the arg parameter
 * supplied to verify, the error code (0 for success), an optional module
 * request state object to be consumed by return_fn or free_modreq_fn, optional
 * e_data to be passed to the caller if code is nonzero, and optional
 * authorization data to be included in the ticket.  In non-FAST replies,
 * e_data will be encoded as typed-data if the module sets the PA_TYPED_E_DATA
 * flag, and as pa-data otherwise.  e_data and authz_data will be freed by the
 * KDC.
 */
    #[c2rust::src_loc = "326:1"]
    pub type krb5_kdcpreauth_verify_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: krb5_kdcpreauth_modreq,
                                    _: *mut *mut krb5_pa_data,
                                    _: *mut *mut krb5_authdata) -> ()>;
    /*
 * Optional: verify preauthentication data sent by the client, setting the
 * TKT_FLG_PRE_AUTH or TKT_FLG_HW_AUTH flag in the enc_tkt_reply's "flags"
 * field as appropriate.  The implementation must invoke the respond function
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 */
    #[c2rust::src_loc = "339:1"]
    pub type krb5_kdcpreauth_verify_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_data,
                                    _: *mut krb5_kdc_req,
                                    _: *mut krb5_enc_tkt_part,
                                    _: *mut krb5_pa_data,
                                    _: krb5_kdcpreauth_callbacks,
                                    _: krb5_kdcpreauth_rock,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_verify_respond_fn,
                                    _: *mut libc::c_void) -> ()>;
    /*
 * Optional: generate preauthentication response data to send to the client as
 * part of the AS-REP.  If it needs to override the key which is used to
 * encrypt the response, it can do so.
 */
    #[c2rust::src_loc = "355:1"]
    pub type krb5_kdcpreauth_return_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_pa_data,
                                    _: *mut krb5_data, _: *mut krb5_kdc_req,
                                    _: *mut krb5_kdc_rep,
                                    _: *mut krb5_keyblock,
                                    _: *mut *mut krb5_pa_data,
                                    _: krb5_kdcpreauth_callbacks,
                                    _: krb5_kdcpreauth_rock,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_modreq)
                   -> krb5_error_code>;
    /* Optional: free a per-request context. */
    #[c2rust::src_loc = "369:1"]
    pub type krb5_kdcpreauth_free_modreq_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_modreq) -> ()>;
    /* Optional: invoked after init_fn to provide the module with a pointer to the
 * verto main loop. */
    #[c2rust::src_loc = "376:1"]
    pub type krb5_kdcpreauth_loop_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata,
                                    _: *mut verto_ctx) -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "381:16"]
    pub struct krb5_kdcpreauth_vtable_st {
        pub name: *mut libc::c_char,
        pub pa_type_list: *mut krb5_preauthtype,
        pub init: krb5_kdcpreauth_init_fn,
        pub fini: krb5_kdcpreauth_fini_fn,
        pub flags: krb5_kdcpreauth_flags_fn,
        pub edata: krb5_kdcpreauth_edata_fn,
        pub verify: krb5_kdcpreauth_verify_fn,
        pub return_padata: krb5_kdcpreauth_return_fn,
        pub free_modreq: krb5_kdcpreauth_free_modreq_fn,
        pub loop_0: krb5_kdcpreauth_loop_fn,
    }
    #[c2rust::src_loc = "381:1"]
    pub type krb5_kdcpreauth_vtable = *mut krb5_kdcpreauth_vtable_st;
    use super::krb5_kdcpreauth_moddata_st;
    use super::krb5_h::{krb5_deltat, krb5_context, krb5_error_code,
                        krb5_keyblock, krb5_data, krb5_boolean,
                        krb5_preauthtype, krb5_principal, krb5_pa_data,
                        krb5_kdc_req, krb5_authdata, krb5_enc_tkt_part,
                        krb5_kdc_rep};
    extern "C" {
        #[c2rust::src_loc = "111:16"]
        pub type krb5_kdcpreauth_rock_st;
        #[c2rust::src_loc = "115:16"]
        pub type krb5_kdcpreauth_modreq_st;
        /* Minor 2 ends here. */
        /* The verto context structure type (typedef is in verto.h; we want to avoid a
 * header dependency for the moment). */
        #[c2rust::src_loc = "119:8"]
        pub type verto_ctx;
    }
    /* KRB5_KDCPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/certauth_plugin.h:34"]
pub mod certauth_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/krb5/certauth_plugin.h - certauth plugin header. */
/*
 * Copyright (C) 2017 by Red Hat, Inc.
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
 * Declarations for certauth plugin module implementors.
 *
 * The certauth pluggable interface currently has only one supported major
 * version, which is 1.  Major version 1 has a current minor version number of
 * 1.
 *
 * certauth plugin modules should define a function named
 * certauth_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   certauth_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                           krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for maj_ver:
 *     maj_ver == 1: Cast to krb5_certauth_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* Abstract module data type. */
    #[c2rust::src_loc = "68:1"]
    pub type krb5_certauth_moddata = *mut krb5_certauth_moddata_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "120:16"]
    pub struct krb5_certauth_vtable_st {
        pub name: *const libc::c_char,
        pub init: krb5_certauth_init_fn,
        pub fini: krb5_certauth_fini_fn,
        pub authorize: krb5_certauth_authorize_fn,
        pub free_ind: krb5_certauth_free_indicator_fn,
    }
    /*
 * Free indicators allocated by a module.  Mandatory if authorize returns
 * authentication indicators.
 */
    #[c2rust::src_loc = "115:1"]
    pub type krb5_certauth_free_indicator_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_certauth_moddata,
                                    _: *mut *mut libc::c_char) -> ()>;
    /*
 * Mandatory:
 * Return 0 if the DER-encoded cert is authorized for PKINIT authentication by
 * princ; otherwise return one of the following error codes:
 * - KRB5KDC_ERR_CLIENT_NAME_MISMATCH - incorrect SAN value
 * - KRB5KDC_ERR_INCONSISTENT_KEY_PURPOSE - incorrect EKU
 * - KRB5KDC_ERR_CERTIFICATE_MISMATCH - other extension error
 * - KRB5_PLUGIN_NO_HANDLE - the module has no opinion about cert
 *
 * - opts is used by built-in modules to receive internal data, and must be
 *   ignored by other modules.
 * - db_entry receives the client principal database entry, and can be ignored
 *   by modules that do not link with libkdb5.
 * - *authinds_out optionally returns a null-terminated list of authentication
 *   indicator strings upon KRB5_PLUGIN_NO_HANDLE or accepted authorization.
 */
    #[c2rust::src_loc = "103:1"]
    pub type krb5_certauth_authorize_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_certauth_moddata,
                                    _: *const uint8_t, _: size_t,
                                    _: krb5_const_principal,
                                    _: *const libc::c_void,
                                    _: *const _krb5_db_entry_new,
                                    _: *mut *mut *mut libc::c_char)
                   -> krb5_error_code>;
    /*
 * Optional: Clean up the module data.
 */
    #[c2rust::src_loc = "84:1"]
    pub type krb5_certauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_certauth_moddata)
                   -> ()>;
    /*
 * Optional: Initialize module data.
 */
    #[c2rust::src_loc = "77:1"]
    pub type krb5_certauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_certauth_moddata)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "120:1"]
    pub type krb5_certauth_vtable = *mut krb5_certauth_vtable_st;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_const_principal};
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "68:16"]
        pub type krb5_certauth_moddata_st;
        /* A module can optionally include <kdb.h> to inspect the client principal
 * entry when authorizing a request. */
        #[c2rust::src_loc = "72:8"]
        pub type _krb5_db_entry_new;
    }
    /* KRB5_CERTAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit.h:33"]
pub mod pkinit_h {
    /*
 * Client's plugin context
 */
    /*
 * Client's per-request context
 */
    /*
 * KDC's (per-realm) plugin context
 */
    #[c2rust::src_loc = "237:1"]
    pub type pkinit_kdc_context = *mut _pkinit_kdc_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "227:8"]
    pub struct _pkinit_kdc_context {
        pub magic: libc::c_int,
        pub cryptoctx: pkinit_plg_crypto_context,
        pub opts: *mut pkinit_plg_opts,
        pub idctx: pkinit_identity_crypto_context,
        pub idopts: *mut pkinit_identity_opts,
        pub realmname: *mut libc::c_char,
        pub realmname_len: libc::c_uint,
        pub auth_indicators: *mut *mut libc::c_char,
    }
    #[c2rust::src_loc = "175:1"]
    pub type pkinit_identity_opts = _pkinit_identity_opts;
    /*
 * this structure keeps options used for a given request
 */
    /* initial request DH modulus size (default=1024) */
    /*
 * information about identity from config file or command line
 */
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
    #[c2rust::src_loc = "141:1"]
    pub type pkinit_identity_crypto_context
        =
        *mut _pkinit_identity_crypto_context;
    /*
 * this structure keeps information about the config options
 */
    #[c2rust::src_loc = "146:1"]
    pub type pkinit_plg_opts = _pkinit_plg_opts;
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
    #[c2rust::src_loc = "128:1"]
    pub type pkinit_plg_crypto_context = *mut _pkinit_plg_crypto_context;
    #[c2rust::src_loc = "134:1"]
    pub type pkinit_req_crypto_context = *mut _pkinit_req_crypto_context;
    /* require EKU checking (default is true) */
    /* accept secondary EKU (default is false) */
    /* allow UPN-SAN instead of pkinit-SAN */
    /* selects DH or RSA based pkinit */
    /* require CRL for a CA (default is false) */
    /* require freshness token (default is false) */
    /* disable freshness token on client for testing */
    /* minimum DH modulus size allowed */
    /*
 * KDC's per-request context
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "242:8"]
    pub struct _pkinit_kdc_req_context {
        pub magic: libc::c_int,
        pub cryptoctx: pkinit_req_crypto_context,
        pub rcv_auth_pack: *mut krb5_auth_pack,
        pub pa_type: krb5_preauthtype,
    }
    #[c2rust::src_loc = "248:1"]
    pub type pkinit_kdc_req_context = *mut _pkinit_kdc_req_context;
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn pkiDebug(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    }
    use super::pkcs11_h::CK_SLOT_ID;
    use super::k5_int_pkinit_h::{krb5_auth_pack, krb5_reply_key_pack,
                                 krb5_pa_pk_as_rep, krb5_pa_pk_as_req};
    use super::krb5_h::{krb5_preauthtype, krb5_error_code, krb5_context,
                        krb5_principal_data, krb5_principal, krb5_boolean};
    use super::k5_int_h::_krb5_context;
    use super::clpreauth_plugin_h::{krb5_clpreauth_callbacks_st,
                                    krb5_clpreauth_callbacks,
                                    krb5_clpreauth_rock_st,
                                    krb5_clpreauth_rock};
    extern "C" {
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
        /* pkinit_kdc_ocsp has been removed */
        /* Make pkiDebug(fmt,...) print, or not.  */
        /* Still evaluates for side effects.  */
        /* This is better if the compiler doesn't inline variadic functions
   well, but gcc will warn about "left-hand operand of comma
   expression has no effect".  Still evaluates for side effects.  */
/* #define pkiDebug	(void) */
        /* Solaris compiler doesn't grok __FUNCTION__
 * hack for now.  Fix all the uses eventually. */
        /* Macros to deal with converting between various data types... */
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
        /*
 * request crypto context should keep reqyest common information,
 * eg., received credentials, DH parameters of this request
 */
        /*
 * identity context should keep information about credentials
 * for the request, eg., my credentials, trusted ca certs,
 * intermediate ca certs, crls, pkcs11 info
 */
        #[c2rust::src_loc = "141:16"]
        pub type _pkinit_identity_crypto_context;
        #[c2rust::src_loc = "128:16"]
        pub type _pkinit_plg_crypto_context;
        #[c2rust::src_loc = "134:16"]
        pub type _pkinit_req_crypto_context;
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
        #[no_mangle]
        #[c2rust::src_loc = "299:1"]
        pub fn pkinit_client_cert_match(context: krb5_context,
                                        plgctx: pkinit_plg_crypto_context,
                                        reqctx: pkinit_req_crypto_context,
                                        match_rule: *const libc::c_char,
                                        matched: *mut krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "330:1"]
        pub fn init_krb5_reply_key_pack(in_0: *mut *mut krb5_reply_key_pack);
        #[no_mangle]
        #[c2rust::src_loc = "332:1"]
        pub fn init_krb5_pa_pk_as_rep(in_0: *mut *mut krb5_pa_pk_as_rep);
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn free_krb5_pa_pk_as_req(in_0: *mut *mut krb5_pa_pk_as_req);
        #[no_mangle]
        #[c2rust::src_loc = "336:1"]
        pub fn free_krb5_reply_key_pack(in_0: *mut *mut krb5_reply_key_pack);
        #[no_mangle]
        #[c2rust::src_loc = "337:1"]
        pub fn free_krb5_auth_pack(in_0: *mut *mut krb5_auth_pack);
        #[no_mangle]
        #[c2rust::src_loc = "338:1"]
        pub fn free_krb5_pa_pk_as_rep(in_0: *mut *mut krb5_pa_pk_as_rep);
        /*
 * Functions in pkinit_profile.c
 */
        #[no_mangle]
        #[c2rust::src_loc = "350:1"]
        pub fn pkinit_kdcdefault_strings(context: krb5_context,
                                         realmname: *const libc::c_char,
                                         option: *const libc::c_char,
                                         ret_value:
                                             *mut *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn pkinit_kdcdefault_string(context: krb5_context,
                                        realmname: *const libc::c_char,
                                        option: *const libc::c_char,
                                        ret_value: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "356:1"]
        pub fn pkinit_kdcdefault_boolean(context: krb5_context,
                                         realmname: *const libc::c_char,
                                         option: *const libc::c_char,
                                         default_value: libc::c_int,
                                         ret_value: *mut libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "359:1"]
        pub fn pkinit_kdcdefault_integer(context: krb5_context,
                                         realmname: *const libc::c_char,
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
                          _pkinit_identity_crypto_context, pkinit_plg_opts};
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_preauthtype,
                        krb5_principal, krb5_enctype, krb5_keyblock,
                        krb5_data, krb5_pa_data, krb5_principal_data,
                        krb5_const_principal};
    use super::k5_int_h::_krb5_context;
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
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
		    receives whether message is signed */
        /*
 * this function creates a CMS message where eContentType is EnvelopedData
 */
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn cms_envelopeddata_create(context: krb5_context,
                                        plg_cryptoctx:
                                            pkinit_plg_crypto_context,
                                        req_cryptoctx:
                                            pkinit_req_crypto_context,
                                        id_cryptoctx:
                                            pkinit_identity_crypto_context,
                                        pa_type: krb5_preauthtype,
                                        include_certchain: libc::c_int,
                                        key_pack: *mut libc::c_uchar,
                                        key_pack_len: libc::c_uint,
                                        envel_data: *mut *mut libc::c_uchar,
                                        envel_data_len: *mut libc::c_uint)
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
		    receives length of DH secret key */
        /*
 * this function implements the KDC first part of the DH protocol.
 * it decodes the client's DH parameters and pub key and checks
 * if they are acceptable.
 */
        #[no_mangle]
        #[c2rust::src_loc = "351:1"]
        pub fn server_check_dh(context: krb5_context,
                               plg_cryptoctx: pkinit_plg_crypto_context,
                               req_cryptoctx: pkinit_req_crypto_context,
                               id_cryptoctx: pkinit_identity_crypto_context,
                               dh_params: *mut krb5_data,
                               minbits: libc::c_int) -> krb5_error_code;
        /* IN
		    the mininum number of key bits acceptable */
        /*
 * this function completes the KDC's DH protocol. The KDC generates
 * its DH pub key and computes the DH secret key
 */
        #[no_mangle]
        #[c2rust::src_loc = "365:1"]
        pub fn server_process_dh(context: krb5_context,
                                 plg_cryptoctx: pkinit_plg_crypto_context,
                                 req_cryptoctx: pkinit_req_crypto_context,
                                 id_cryptoctx: pkinit_identity_crypto_context,
                                 received_pubkey: *mut libc::c_uchar,
                                 received_pub_len: libc::c_uint,
                                 dh_pubkey_out: *mut *mut libc::c_uchar,
                                 dh_pubkey_len_out: *mut libc::c_uint,
                                 server_key_out: *mut *mut libc::c_uchar,
                                 server_key_len_out: *mut libc::c_uint)
         -> krb5_error_code;
        /* IN */
        /*
 * this function creates edata that contains TD-DH-PARAMETERS
 */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn pkinit_create_td_dh_parameters(context: krb5_context,
                                              plg_cryptoctx:
                                                  pkinit_plg_crypto_context,
                                              req_cryptoctx:
                                                  pkinit_req_crypto_context,
                                              id_cryptoctx:
                                                  pkinit_identity_crypto_context,
                                              opts: *mut pkinit_plg_opts,
                                              e_data_out:
                                                  *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        /* OUT
		    receives the new DH modulus to use in the new AS-REQ */
        /*
 * this function creates edata that contains TD-INVALID-CERTIFICATES
 */
        #[no_mangle]
        #[c2rust::src_loc = "556:1"]
        pub fn pkinit_create_td_invalid_certificate(context: krb5_context,
                                                    plg_cryptoctx:
                                                        pkinit_plg_crypto_context,
                                                    req_cryptoctx:
                                                        pkinit_req_crypto_context,
                                                    id_cryptoctx:
                                                        pkinit_identity_crypto_context,
                                                    e_data_out:
                                                        *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        /* OUT */
        /*
 * this function creates edata that contains TD-TRUSTED-CERTIFIERS
 */
        #[no_mangle]
        #[c2rust::src_loc = "566:1"]
        pub fn pkinit_create_td_trusted_certifiers(context: krb5_context,
                                                   plg_cryptoctx:
                                                       pkinit_plg_crypto_context,
                                                   req_cryptoctx:
                                                       pkinit_req_crypto_context,
                                                   id_cryptoctx:
                                                       pkinit_identity_crypto_context,
                                                   e_data_out:
                                                       *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        /* IN */
        /*
 * this function checks if the received kdcPKId matches
 * the KDC's certificate
 */
        #[no_mangle]
        #[c2rust::src_loc = "591:1"]
        pub fn pkinit_check_kdc_pkid(context: krb5_context,
                                     plg_cryptoctx: pkinit_plg_crypto_context,
                                     req_cryptoctx: pkinit_req_crypto_context,
                                     id_cryptoctx:
                                         pkinit_identity_crypto_context,
                                     pdid_buf: *mut libc::c_uchar,
                                     pkid_len: libc::c_uint,
                                     valid_kdcPkId: *mut libc::c_int)
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
        #[no_mangle]
        #[c2rust::src_loc = "632:1"]
        pub fn crypto_encode_der_cert(context: krb5_context,
                                      reqctx: pkinit_req_crypto_context,
                                      der_out: *mut *mut uint8_t,
                                      der_len: *mut size_t)
         -> krb5_error_code;
    }
    /* _PKINIT_CRYPTO_H */
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
#[c2rust::header_src = "/usr/include/libintl.h:32"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
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
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
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
    use super::krb5_h::{krb5_error_code, krb5_data};
    use super::k5_int_pkinit_h::{krb5_auth_pack, krb5_kdc_dh_key_info,
                                 krb5_pa_pk_as_rep, krb5_pa_pk_as_req,
                                 krb5_reply_key_pack};
    extern "C" {
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
        /*
 * Function prototypes
 */
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn pkinit_accessor_init() -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub static mut k5int_decode_krb5_auth_pack:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _: *mut *mut krb5_auth_pack)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub static mut k5int_encode_krb5_kdc_dh_key_info:
                   Option<unsafe extern "C" fn(_: *const krb5_kdc_dh_key_info,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub static mut k5int_encode_krb5_pa_pk_as_rep:
                   Option<unsafe extern "C" fn(_: *const krb5_pa_pk_as_rep,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub static mut k5int_decode_krb5_pa_pk_as_req:
                   Option<unsafe extern "C" fn(_: *const krb5_data,
                                               _: *mut *mut krb5_pa_pk_as_req)
                              -> krb5_error_code>;
        #[no_mangle]
        #[c2rust::src_loc = "51:1"]
        pub static mut k5int_encode_krb5_reply_key_pack:
                   Option<unsafe extern "C" fn(_: *const krb5_reply_key_pack,
                                               _: *mut *mut krb5_data)
                              -> krb5_error_code>;
    }
    /* _PKINIT_ACCESSOR_H */
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_msgtype,
                       krb5_kvno, krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_checksum, krb5_checksum, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_last_req_entry,
                       krb5_last_req_entry, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _profile_t, krb5_anonymous_principal,
                       krb5_c_fx_cf2_simple, krb5_c_make_random_key,
                       krb5_c_make_checksum, krb5_c_valid_enctype,
                       krb5_parse_name_flags, krb5_unparse_name,
                       krb5_principal_compare, krb5_copy_keyblock_contents,
                       krb5_free_principal, krb5_free_keyblock,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_free_data_contents, krb5_free_unparsed_name,
                       krb5_check_clockskew, krb5_set_error_message,
                       krb5_prepend_error_message, krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5calloc, k5alloc, k5memdup,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, k5_plugin_load_all,
                         k5_plugin_free_modules, k5_plugin_register};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err, error_message};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_initvt_fn,
                         krb5_plugin_vtable_st};
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
                                krb5_pa_pk_as_rep, encode_krb5_enc_data,
                                encode_krb5_encryption_key,
                                krb5_encrypt_helper};
pub use self::clpreauth_plugin_h::{krb5_clpreauth_rock,
                                   krb5_clpreauth_callbacks_st,
                                   krb5_clpreauth_callbacks,
                                   krb5_clpreauth_rock_st};
pub use self::kdcpreauth_plugin_h::{krb5_kdcpreauth_rock,
                                    krb5_kdcpreauth_moddata,
                                    krb5_kdcpreauth_modreq,
                                    krb5_kdcpreauth_callbacks_st,
                                    krb5_kdcpreauth_callbacks,
                                    krb5_kdcpreauth_init_fn,
                                    krb5_kdcpreauth_fini_fn,
                                    krb5_kdcpreauth_flags_fn,
                                    krb5_kdcpreauth_edata_respond_fn,
                                    krb5_kdcpreauth_edata_fn,
                                    krb5_kdcpreauth_verify_respond_fn,
                                    krb5_kdcpreauth_verify_fn,
                                    krb5_kdcpreauth_return_fn,
                                    krb5_kdcpreauth_free_modreq_fn,
                                    krb5_kdcpreauth_loop_fn,
                                    krb5_kdcpreauth_vtable_st,
                                    krb5_kdcpreauth_vtable,
                                    krb5_kdcpreauth_rock_st,
                                    krb5_kdcpreauth_modreq_st, verto_ctx};
pub use self::certauth_plugin_h::{krb5_certauth_moddata,
                                  krb5_certauth_vtable_st,
                                  krb5_certauth_free_indicator_fn,
                                  krb5_certauth_authorize_fn,
                                  krb5_certauth_fini_fn,
                                  krb5_certauth_init_fn, krb5_certauth_vtable,
                                  krb5_certauth_moddata_st,
                                  _krb5_db_entry_new};
pub use self::pkinit_h::{pkinit_kdc_context, _pkinit_kdc_context,
                         pkinit_identity_opts, _pkinit_identity_opts,
                         pkinit_identity_crypto_context, pkinit_plg_opts,
                         _pkinit_plg_opts, pkinit_plg_crypto_context,
                         pkinit_req_crypto_context, _pkinit_kdc_req_context,
                         pkinit_kdc_req_context, pkiDebug,
                         _pkinit_identity_crypto_context,
                         _pkinit_plg_crypto_context,
                         _pkinit_req_crypto_context, pkinit_init_plg_opts,
                         pkinit_fini_plg_opts, pkinit_init_identity_opts,
                         pkinit_fini_identity_opts,
                         pkinit_identity_initialize, pkinit_identity_prompt,
                         pkinit_client_cert_match, init_krb5_reply_key_pack,
                         init_krb5_pa_pk_as_rep, free_krb5_pa_pk_as_req,
                         free_krb5_reply_key_pack, free_krb5_auth_pack,
                         free_krb5_pa_pk_as_rep, pkinit_kdcdefault_strings,
                         pkinit_kdcdefault_string, pkinit_kdcdefault_boolean,
                         pkinit_kdcdefault_integer};
pub use self::pkcs11_h::CK_SLOT_ID;
pub use self::pkinit_crypto_h::{cms_msg_types, CMS_ENVEL_SERVER,
                                CMS_SIGN_SERVER, CMS_SIGN_CLIENT,
                                pkinit_init_plg_crypto,
                                pkinit_fini_plg_crypto,
                                pkinit_init_req_crypto,
                                pkinit_fini_req_crypto,
                                pkinit_init_identity_crypto,
                                pkinit_fini_identity_crypto,
                                cms_signeddata_create, cms_signeddata_verify,
                                cms_envelopeddata_create,
                                crypto_retrieve_cert_sans,
                                crypto_check_cert_eku, pkinit_octetstring2key,
                                server_check_dh, server_process_dh,
                                pkinit_create_td_dh_parameters,
                                pkinit_create_td_invalid_certificate,
                                pkinit_create_td_trusted_certifiers,
                                pkinit_check_kdc_pkid, pkinit_alg_agility_kdf,
                                supported_kdf_alg_ids,
                                crypto_encode_der_cert};
use self::k5_platform_h::k5_bcmp;
use self::libintl_h::dgettext;
use self::stdlib_h::{free, calloc, malloc};
use self::strings_h::strcasecmp;
use self::string_h::{strlen, strdup, strncmp, memcmp, memset, memcpy};
use self::k5_trace_h::krb5int_trace;
use self::pkinit_accessor_h::{pkinit_accessor_init,
                              k5int_decode_krb5_auth_pack,
                              k5int_encode_krb5_kdc_dh_key_info,
                              k5int_encode_krb5_pa_pk_as_rep,
                              k5int_decode_krb5_pa_pk_as_req,
                              k5int_encode_krb5_reply_key_pack};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "49:8"]
pub struct krb5_kdcpreauth_moddata_st {
    pub realm_contexts: *mut pkinit_kdc_context,
    pub certauth_modules: *mut certauth_handle,
}
#[c2rust::src_loc = "44:1"]
pub type certauth_handle = *mut certauth_module_handle_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "44:16"]
pub struct certauth_module_handle_st {
    pub vt: krb5_certauth_vtable_st,
    pub moddata: krb5_certauth_moddata,
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
/* Aliases used by the built-in certauth modules */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "37:8"]
pub struct certauth_req_opts {
    pub cb: krb5_kdcpreauth_callbacks,
    pub rock: krb5_kdcpreauth_rock,
    pub plgctx: pkinit_kdc_context,
    pub reqctx: pkinit_kdc_req_context,
}
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn free_realm_contexts(mut context: krb5_context,
                                         mut realm_contexts:
                                             *mut pkinit_kdc_context) {
    let mut i: libc::c_int = 0;
    if realm_contexts.is_null() { return }
    i = 0 as libc::c_int;
    while !(*realm_contexts.offset(i as isize)).is_null() {
        pkinit_server_plugin_fini_realm(context,
                                        *realm_contexts.offset(i as isize));
        i += 1
    }
    pkiDebug(b"%s: freeing context at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 20],
                                       &[libc::c_char; 20]>(b"free_realm_contexts\x00")).as_ptr(),
             realm_contexts);
    free(realm_contexts as *mut libc::c_void);
}
#[c2rust::src_loc = "86:1"]
unsafe extern "C" fn free_certauth_handles(mut context: krb5_context,
                                           mut list: *mut certauth_handle) {
    let mut i: libc::c_int = 0;
    if list.is_null() { return }
    i = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        if (**list.offset(i as isize)).vt.fini.is_some() {
            (**list.offset(i as
                               isize)).vt.fini.expect("non-null function pointer")(context,
                                                                                   (**list.offset(i
                                                                                                      as
                                                                                                      isize)).moddata);
        }
        free(*list.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(list as *mut libc::c_void);
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn pkinit_create_edata(mut context: krb5_context,
                                         mut plg_cryptoctx:
                                             pkinit_plg_crypto_context,
                                         mut req_cryptoctx:
                                             pkinit_req_crypto_context,
                                         mut id_cryptoctx:
                                             pkinit_identity_crypto_context,
                                         mut opts: *mut pkinit_plg_opts,
                                         mut err_code: krb5_error_code,
                                         mut e_data_out:
                                             *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code =
        -(1765328324 as libc::c_long) as krb5_error_code;
    pkiDebug(b"pkinit_create_edata: creating edata for error %d (%s)\n\x00" as
                 *const u8 as *const libc::c_char, err_code,
             error_message(err_code as errcode_t));
    match err_code {
        -1765328314 => {
            retval =
                pkinit_create_td_trusted_certifiers(context, plg_cryptoctx,
                                                    req_cryptoctx,
                                                    id_cryptoctx, e_data_out)
        }
        -1765328319 => {
            retval =
                pkinit_create_td_dh_parameters(context, plg_cryptoctx,
                                               req_cryptoctx, id_cryptoctx,
                                               opts, e_data_out)
        }
        -1765328313 | -1765328312 => {
            retval =
                pkinit_create_td_invalid_certificate(context, plg_cryptoctx,
                                                     req_cryptoctx,
                                                     id_cryptoctx, e_data_out)
        }
        _ => {
            pkiDebug(b"no edata needed for error %d (%s)\n\x00" as *const u8
                         as *const libc::c_char, err_code,
                     error_message(err_code as errcode_t));
            retval = 0 as libc::c_int
        }
    }
    return retval;
}
#[c2rust::src_loc = "140:1"]
unsafe extern "C" fn pkinit_server_get_edata(mut context: krb5_context,
                                             mut request: *mut krb5_kdc_req,
                                             mut cb:
                                                 krb5_kdcpreauth_callbacks,
                                             mut rock: krb5_kdcpreauth_rock,
                                             mut moddata:
                                                 krb5_kdcpreauth_moddata,
                                             mut pa_type: krb5_preauthtype,
                                             mut respond:
                                                 krb5_kdcpreauth_edata_respond_fn,
                                             mut arg: *mut libc::c_void) {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut plgctx: pkinit_kdc_context = 0 as pkinit_kdc_context;
    pkiDebug(b"pkinit_server_get_edata: entered!\n\x00" as *const u8 as
                 *const libc::c_char);
    /*
     * If we don't have a realm context for the given realm,
     * don't tell the client that we support pkinit!
     */
    plgctx = pkinit_find_realm_context(context, moddata, (*request).server);
    if plgctx.is_null() { retval = 22 as libc::c_int }
    /* Send a freshness token if the client requested one. */
    if retval == 0 {
        (*cb).send_freshness_token.expect("non-null function pointer")(context,
                                                                       rock);
    }
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          retval,
                                                                                          0
                                                                                              as
                                                                                              *mut krb5_pa_data);
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn verify_client_san(mut context: krb5_context,
                                       mut plgctx: pkinit_kdc_context,
                                       mut reqctx: pkinit_kdc_req_context,
                                       mut cb: krb5_kdcpreauth_callbacks,
                                       mut rock: krb5_kdcpreauth_rock,
                                       mut client: krb5_const_principal,
                                       mut valid_san: *mut libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut princs: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut upn: krb5_principal = 0 as *mut krb5_principal_data;
    let mut match_0: krb5_boolean = 0;
    let mut upns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    *valid_san = 0 as libc::c_int;
    retval =
        crypto_retrieve_cert_sans(context, (*plgctx).cryptoctx,
                                  (*reqctx).cryptoctx, (*plgctx).idctx,
                                  &mut princs,
                                  if (*(*plgctx).opts).allow_upn != 0 {
                                      &mut upns
                                  } else { 0 as *mut *mut *mut libc::c_char },
                                  0 as *mut *mut *mut libc::c_uchar);
    if retval != 0 {
        pkiDebug(b"%s: error from retrieve_certificate_sans()\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 18],
                                           &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr());
        retval = -(1765328309 as libc::c_long) as krb5_error_code
    } else if princs.is_null() && upns.is_null() {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT server found no SAN in client cert\x00" as
                              *const u8 as *const libc::c_char);
        }
        retval = 2 as libc::c_int
    } else {
        pkiDebug(b"%s: Checking pkinit sans\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 18],
                                           &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr());
        i = 0 as libc::c_int;
        loop  {
            if !(!princs.is_null() && !(*princs.offset(i as isize)).is_null())
               {
                current_block = 16203760046146113240;
                break ;
            }
            if (*cb).match_client.expect("non-null function pointer")(context,
                                                                      rock,
                                                                      *princs.offset(i
                                                                                         as
                                                                                         isize))
                   != 0 {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT server found a matching SAN in client cert\x00"
                                      as *const u8 as *const libc::c_char);
                }
                *valid_san = 1 as libc::c_int;
                retval = 0 as libc::c_int;
                current_block = 2497782172142719783;
                break ;
            } else { i += 1 }
        }
        match current_block {
            2497782172142719783 => { }
            _ => {
                pkiDebug(b"%s: no pkinit san match found\n\x00" as *const u8
                             as *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 18],
                                                   &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr());
                /*
     * XXX if cert has names but none match, should we
     * be returning KRB5KDC_ERR_CLIENT_NAME_MISMATCH here?
     */
                if upns.is_null() {
                    pkiDebug(b"%s: no upn sans (or we wouldn\'t accept them anyway)\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr());
                    retval = -(1765328309 as libc::c_long) as krb5_error_code
                } else {
                    pkiDebug(b"%s: Checking upn sans\n\x00" as *const u8 as
                                 *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 18],
                                                       &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr());
                    i = 0 as libc::c_int;
                    loop  {
                        if (*upns.offset(i as isize)).is_null() {
                            current_block = 6476622998065200121;
                            break ;
                        }
                        retval =
                            krb5_parse_name_flags(context,
                                                  *upns.offset(i as isize),
                                                  0x4 as libc::c_int,
                                                  &mut upn);
                        if retval != 0 {
                            if (*context).trace_callback.is_some() {
                                krb5int_trace(context,
                                              b"PKINIT server could not parse UPN \"{str}\": {kerr}\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              *upns.offset(i as isize),
                                              retval);
                            }
                        } else {
                            match_0 =
                                (*cb).match_client.expect("non-null function pointer")(context,
                                                                                       rock,
                                                                                       upn);
                            krb5_free_principal(context, upn);
                            if match_0 != 0 {
                                if (*context).trace_callback.is_some() {
                                    krb5int_trace(context,
                                                  b"PKINIT server found a matching UPN SAN in client cert\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                }
                                *valid_san = 1 as libc::c_int;
                                retval = 0 as libc::c_int;
                                current_block = 2497782172142719783;
                                break ;
                            }
                        }
                        i += 1
                    }
                    match current_block {
                        2497782172142719783 => { }
                        _ => {
                            pkiDebug(b"%s: no upn san match found\n\x00" as
                                         *const u8 as *const libc::c_char,
                                     (*::std::mem::transmute::<&[u8; 18],
                                                               &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr());
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
    if !upns.is_null() {
        i = 0 as libc::c_int;
        while !(*upns.offset(i as isize)).is_null() {
            free(*upns.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(upns as *mut libc::c_void);
    }
    pkiDebug(b"%s: returning retval %d, valid_san %d\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 18],
                                       &[libc::c_char; 18]>(b"verify_client_san\x00")).as_ptr(),
             retval, *valid_san);
    return retval;
}
#[c2rust::src_loc = "282:1"]
unsafe extern "C" fn verify_client_eku(mut context: krb5_context,
                                       mut plgctx: pkinit_kdc_context,
                                       mut reqctx: pkinit_kdc_req_context,
                                       mut eku_accepted: *mut libc::c_int)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    *eku_accepted = 0 as libc::c_int;
    if (*(*plgctx).opts).require_eku == 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT server skipping EKU check due to configuration\x00"
                              as *const u8 as *const libc::c_char);
        }
        *eku_accepted = 1 as libc::c_int;
        retval = 0 as libc::c_int
    } else {
        retval =
            crypto_check_cert_eku(context, (*plgctx).cryptoctx,
                                  (*reqctx).cryptoctx, (*plgctx).idctx,
                                  0 as libc::c_int,
                                  (*(*plgctx).opts).accept_secondary_eku,
                                  eku_accepted);
        if retval != 0 {
            pkiDebug(b"%s: Error from crypto_check_cert_eku %d (%s)\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 18],
                                               &[libc::c_char; 18]>(b"verify_client_eku\x00")).as_ptr(),
                     retval, error_message(retval as errcode_t));
        }
    }
    pkiDebug(b"%s: returning retval %d, eku_accepted %d\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 18],
                                       &[libc::c_char; 18]>(b"verify_client_eku\x00")).as_ptr(),
             retval, *eku_accepted);
    return retval;
}
/* Run the received, verified certificate through certauth modules, to verify
 * that it is authorized to authenticate as client. */
#[c2rust::src_loc = "319:1"]
unsafe extern "C" fn authorize_cert(mut context: krb5_context,
                                    mut certauth_modules:
                                        *mut certauth_handle,
                                    mut plgctx: pkinit_kdc_context,
                                    mut reqctx: pkinit_kdc_req_context,
                                    mut cb: krb5_kdcpreauth_callbacks,
                                    mut rock: krb5_kdcpreauth_rock,
                                    mut client: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut h: certauth_handle = 0 as *mut certauth_module_handle_st;
    let mut opts: certauth_req_opts =
        certauth_req_opts{cb: 0 as *mut krb5_kdcpreauth_callbacks_st,
                          rock: 0 as *mut krb5_kdcpreauth_rock_st,
                          plgctx: 0 as *mut _pkinit_kdc_context,
                          reqctx: 0 as *mut _pkinit_kdc_req_context,};
    let mut accepted: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut cert: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: size_t = 0;
    let mut cert_len: size_t = 0;
    let mut db_ent: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ais: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ai: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* Re-encode the received certificate into DER, which is extra work, but
     * avoids creating an X.509 library dependency in the interface. */
    ret =
        crypto_encode_der_cert(context, (*reqctx).cryptoctx, &mut cert,
                               &mut cert_len);
    if !(ret != 0) {
        /* Set options for the builtin module. */
        opts.plgctx = plgctx;
        opts.reqctx = reqctx;
        opts.cb = cb;
        opts.rock = rock;
        db_ent =
            (*cb).client_entry.expect("non-null function pointer")(context,
                                                                   rock);
        /*
     * Check the certificate against each certauth module.  For the certificate
     * to be authorized at least one module must return 0, and no module can an
     * error code other than KRB5_PLUGIN_NO_HANDLE (pass).  Add indicators from
     * modules that return 0 or pass.
     */
        ret = -(1765328135 as libc::c_long) as krb5_error_code;
        i = 0 as libc::c_int as size_t;
        's_61:
            loop  {
                if !(!certauth_modules.is_null() &&
                         !(*certauth_modules.offset(i as isize)).is_null()) {
                    current_block = 8693738493027456495;
                    break ;
                }
                h = *certauth_modules.offset(i as isize);
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"PKINIT server authorizing cert with module {str}\x00"
                                      as *const u8 as *const libc::c_char,
                                  (*h).vt.name);
                }
                ret =
                    (*h).vt.authorize.expect("non-null function pointer")(context,
                                                                          (*h).moddata,
                                                                          cert,
                                                                          cert_len,
                                                                          client
                                                                              as
                                                                              krb5_const_principal,
                                                                          &mut opts
                                                                              as
                                                                              *mut certauth_req_opts
                                                                              as
                                                                              *const libc::c_void,
                                                                          db_ent
                                                                              as
                                                                              *const _krb5_db_entry_new,
                                                                          &mut ais);
                if ret == 0 as libc::c_int {
                    accepted = 1 as libc::c_int as krb5_boolean
                } else if ret as libc::c_long != -(1765328135 as libc::c_long)
                 {
                    current_block = 7032140445288991170;
                    break ;
                }
                if !ais.is_null() {
                    /* Assert authentication indicators from the module. */
                    ai = ais;
                    while !(*ai).is_null() {
                        ret =
                            (*cb).add_auth_indicator.expect("non-null function pointer")(context,
                                                                                         rock,
                                                                                         *ai);
                        if ret != 0 {
                            current_block = 7032140445288991170;
                            break 's_61 ;
                        }
                        ai = ai.offset(1)
                    }
                    (*h).vt.free_ind.expect("non-null function pointer")(context,
                                                                         (*h).moddata,
                                                                         ais);
                    ais = 0 as *mut *mut libc::c_char
                }
                i = i.wrapping_add(1)
            }
        match current_block {
            7032140445288991170 => { }
            _ => {
                ret =
                    if accepted != 0 {
                        0 as libc::c_int as libc::c_long
                    } else { -(1765328309 as libc::c_long) } as
                        krb5_error_code
            }
        }
    }
    free(cert as *mut libc::c_void);
    return ret;
}
/* Return an error if freshness tokens are required and one was not received.
 * Log an appropriate message indicating whether a valid token was received. */
#[c2rust::src_loc = "386:1"]
unsafe extern "C" fn check_log_freshness(mut context: krb5_context,
                                         mut plgctx: pkinit_kdc_context,
                                         mut request: *mut krb5_kdc_req,
                                         mut valid_freshness_token:
                                             krb5_boolean)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    ret =
        krb5_unparse_name(context, (*request).client as krb5_const_principal,
                          &mut name);
    if ret != 0 { return ret }
    if (*(*plgctx).opts).require_freshness != 0 && valid_freshness_token == 0
       {
        com_err(b"\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"PKINIT: no freshness token, rejecting auth from %s\x00"
                             as *const u8 as *const libc::c_char), name);
        ret = -(1765328360 as libc::c_long) as krb5_error_code
    } else if valid_freshness_token != 0 {
        com_err(b"\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"PKINIT: freshness token received from %s\x00" as
                             *const u8 as *const libc::c_char), name);
    } else {
        com_err(b"\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"PKINIT: no freshness token received from %s\x00" as
                             *const u8 as *const libc::c_char), name);
    }
    krb5_free_unparsed_name(context, name);
    return ret;
}
#[c2rust::src_loc = "409:1"]
unsafe extern "C" fn pkinit_server_verify_padata(mut context: krb5_context,
                                                 mut req_pkt: *mut krb5_data,
                                                 mut request:
                                                     *mut krb5_kdc_req,
                                                 mut enc_tkt_reply:
                                                     *mut krb5_enc_tkt_part,
                                                 mut data: *mut krb5_pa_data,
                                                 mut cb:
                                                     krb5_kdcpreauth_callbacks,
                                                 mut rock:
                                                     krb5_kdcpreauth_rock,
                                                 mut moddata:
                                                     krb5_kdcpreauth_moddata,
                                                 mut respond:
                                                     krb5_kdcpreauth_verify_respond_fn,
                                                 mut arg: *mut libc::c_void) {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut authp_data: krb5_data =
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length: 0 as libc::c_int as libc::c_uint,
                           data: 0 as *mut libc::c_char,};
            init
        };
    let mut krb5_authz: krb5_data =
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length: 0 as libc::c_int as libc::c_uint,
                           data: 0 as *mut libc::c_char,};
            init
        };
    let mut reqp: *mut krb5_pa_pk_as_req = 0 as *mut krb5_pa_pk_as_req;
    let mut auth_pack: *mut krb5_auth_pack = 0 as *mut krb5_auth_pack;
    let mut plgctx: pkinit_kdc_context = 0 as pkinit_kdc_context;
    let mut reqctx: pkinit_kdc_req_context = 0 as pkinit_kdc_req_context;
    let mut cksum: krb5_checksum =
        {
            let mut init =
                _krb5_checksum{magic: 0 as libc::c_int,
                               checksum_type: 0 as libc::c_int,
                               length: 0 as libc::c_int as libc::c_uint,
                               contents: 0 as *mut krb5_octet,};
            init
        };
    let mut der_req: *mut krb5_data = 0 as *mut krb5_data;
    let mut k5data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut ftoken: *mut krb5_data = 0 as *mut krb5_data;
    let mut is_signed: libc::c_int = 1 as libc::c_int;
    let mut e_data: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut modreq: krb5_kdcpreauth_modreq = 0 as krb5_kdcpreauth_modreq;
    let mut valid_freshness_token: krb5_boolean =
        0 as libc::c_int as krb5_boolean;
    let mut sp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    pkiDebug(b"pkinit_verify_padata: entered!\n\x00" as *const u8 as
                 *const libc::c_char);
    if data.is_null() || (*data).length <= 0 as libc::c_int as libc::c_uint ||
           (*data).contents.is_null() {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              22
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              0
                                                                                                  as
                                                                                                  krb5_kdcpreauth_modreq,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_pa_data,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_authdata);
        return
    }
    if moddata.is_null() {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              22
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              0
                                                                                                  as
                                                                                                  krb5_kdcpreauth_modreq,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_pa_data,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_authdata);
        return
    }
    plgctx = pkinit_find_realm_context(context, moddata, (*request).server);
    if plgctx.is_null() {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              22
                                                                                                  as
                                                                                                  libc::c_int,
                                                                                              0
                                                                                                  as
                                                                                                  krb5_kdcpreauth_modreq,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_pa_data,
                                                                                              0
                                                                                                  as
                                                                                                  *mut *mut krb5_authdata);
        return
    }
    /* create a per-request context */
    retval =
        pkinit_init_kdc_req_context(context, &mut reqctx); /* !is_signed */
    if !(retval != 0) {
        (*reqctx).pa_type = (*data).pa_type;
        k5data.length = (*data).length;
        k5data.data = (*data).contents as *mut libc::c_char;
        if (*data).pa_type != 16 as libc::c_int {
            pkiDebug(b"unrecognized pa_type = %d\n\x00" as *const u8 as
                         *const libc::c_char, (*data).pa_type);
            retval = 22 as libc::c_int
        } else {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT server verifying KRB5_PADATA_PK_AS_REQ\x00"
                                  as *const u8 as *const libc::c_char);
            }
            retval =
                k5int_decode_krb5_pa_pk_as_req.expect("non-null function pointer")(&mut k5data,
                                                                                   &mut reqp);
            if retval != 0 {
                pkiDebug(b"decode_krb5_pa_pk_as_req failed\n\x00" as *const u8
                             as *const libc::c_char);
            } else {
                retval =
                    cms_signeddata_verify(context, (*plgctx).cryptoctx,
                                          (*reqctx).cryptoctx,
                                          (*plgctx).idctx,
                                          CMS_SIGN_CLIENT as libc::c_int,
                                          (*(*plgctx).opts).require_crl_checking,
                                          (*reqp).signedAuthPack.data as
                                              *mut libc::c_uchar,
                                          (*reqp).signedAuthPack.length,
                                          &mut authp_data.data as
                                              *mut *mut libc::c_char as
                                              *mut *mut libc::c_uchar,
                                          &mut authp_data.length,
                                          &mut krb5_authz.data as
                                              *mut *mut libc::c_char as
                                              *mut *mut libc::c_uchar,
                                          &mut krb5_authz.length,
                                          &mut is_signed);
                if retval != 0 {
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"PKINIT server failed to verify PA data\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                    }
                } else {
                    if is_signed != 0 {
                        retval =
                            authorize_cert(context,
                                           (*moddata).certauth_modules,
                                           plgctx, reqctx, cb, rock,
                                           (*request).client);
                        if retval != 0 {
                            current_block = 609915172598826250;
                        } else { current_block = 3689906465960840878; }
                    } else if krb5_principal_compare(context,
                                                     (*request).client as
                                                         krb5_const_principal,
                                                     krb5_anonymous_principal())
                                  == 0 {
                        retval =
                            -(1765328360 as libc::c_long) as krb5_error_code;
                        krb5_set_error_message(context, retval,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"Pkinit request not signed, but client not anonymous.\x00"
                                                            as *const u8 as
                                                            *const libc::c_char));
                        current_block = 609915172598826250;
                    } else { current_block = 3689906465960840878; }
                    match current_block {
                        609915172598826250 => { }
                        _ => {
                            k5data.length = authp_data.length;
                            k5data.data = authp_data.data;
                            retval =
                                k5int_decode_krb5_auth_pack.expect("non-null function pointer")(&mut k5data,
                                                                                                &mut auth_pack);
                            if retval != 0 {
                                pkiDebug(b"failed to decode krb5_auth_pack\n\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                            } else {
                                retval =
                                    krb5_check_clockskew(context,
                                                         (*auth_pack).pkAuthenticator.ctime);
                                if !(retval != 0) {
                                    /* check dh parameters */
                                    if !(*auth_pack).clientPublicValue.is_null()
                                       {
                                        retval =
                                            server_check_dh(context,
                                                            (*plgctx).cryptoctx,
                                                            (*reqctx).cryptoctx,
                                                            (*plgctx).idctx,
                                                            &mut (*(*auth_pack).clientPublicValue).algorithm.parameters,
                                                            (*(*plgctx).opts).dh_min_bits);
                                        if retval != 0 {
                                            pkiDebug(b"bad dh parameters\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            current_block =
                                                609915172598826250;
                                        } else {
                                            current_block =
                                                7494008139977416618;
                                        }
                                    } else if is_signed == 0 {
                                        /*Anonymous pkinit requires DH*/
                                        retval =
                                            -(1765328360 as libc::c_long) as
                                                krb5_error_code;
                                        krb5_set_error_message(context,
                                                               retval,
                                                               dgettext(b"mit-krb5\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        b"Anonymous pkinit without DH public value not supported.\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char));
                                        current_block = 609915172598826250;
                                    } else {
                                        current_block = 7494008139977416618;
                                    }
                                    match current_block {
                                        609915172598826250 => { }
                                        _ => {
                                            der_req =
                                                (*cb).request_body.expect("non-null function pointer")(context,
                                                                                                       rock);
                                            retval =
                                                krb5_c_make_checksum(context,
                                                                     0x9 as
                                                                         libc::c_int,
                                                                     0 as
                                                                         *const krb5_keyblock,
                                                                     0 as
                                                                         libc::c_int,
                                                                     der_req,
                                                                     &mut cksum);
                                            if retval != 0 {
                                                pkiDebug(b"unable to calculate AS REQ checksum\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                            } else if cksum.length !=
                                                          (*auth_pack).pkAuthenticator.paChecksum.length
                                                          ||
                                                          k5_bcmp(cksum.contents
                                                                      as
                                                                      *const libc::c_void,
                                                                  (*auth_pack).pkAuthenticator.paChecksum.contents
                                                                      as
                                                                      *const libc::c_void,
                                                                  cksum.length
                                                                      as
                                                                      size_t)
                                                              !=
                                                              0 as libc::c_int
                                             {
                                                pkiDebug(b"failed to match the checksum\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char);
                                                retval =
                                                    -(1765328305 as
                                                          libc::c_long) as
                                                        krb5_error_code
                                            } else {
                                                ftoken =
                                                    (*auth_pack).pkAuthenticator.freshnessToken;
                                                if !ftoken.is_null() {
                                                    retval =
                                                        (*cb).check_freshness_token.expect("non-null function pointer")(context,
                                                                                                                        rock,
                                                                                                                        ftoken);
                                                    if retval != 0 {
                                                        current_block =
                                                            609915172598826250;
                                                    } else {
                                                        valid_freshness_token
                                                            =
                                                            1 as libc::c_int
                                                                as
                                                                krb5_boolean;
                                                        current_block =
                                                            10778260831612459202;
                                                    }
                                                } else {
                                                    current_block =
                                                        10778260831612459202;
                                                }
                                                match current_block {
                                                    609915172598826250 => { }
                                                    _ =>
                                                    /* check if kdcPkId present and match KDC's subjectIdentifier */
                                                    {
                                                        if !(*reqp).kdcPkId.data.is_null()
                                                           {
                                                            let mut valid_kdcPkId:
                                                                    libc::c_int =
                                                                0 as
                                                                    libc::c_int;
                                                            retval =
                                                                pkinit_check_kdc_pkid(context,
                                                                                      (*plgctx).cryptoctx,
                                                                                      (*reqctx).cryptoctx,
                                                                                      (*plgctx).idctx,
                                                                                      (*reqp).kdcPkId.data
                                                                                          as
                                                                                          *mut libc::c_uchar,
                                                                                      (*reqp).kdcPkId.length,
                                                                                      &mut valid_kdcPkId);
                                                            if retval != 0 {
                                                                current_block
                                                                    =
                                                                    609915172598826250;
                                                            } else {
                                                                if valid_kdcPkId
                                                                       == 0 {
                                                                    pkiDebug(b"kdcPkId in AS_REQ does not match KDC\'s cert; RFC says to ignore and proceed\n\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char);
                                                                }
                                                                current_block
                                                                    =
                                                                    15237655884915618618;
                                                            }
                                                        } else {
                                                            current_block =
                                                                15237655884915618618;
                                                        }
                                                        match current_block {
                                                            609915172598826250
                                                            => {
                                                            }
                                                            _ => {
                                                                /* remember the decoded auth_pack for verify_padata routine */
                                                                (*reqctx).rcv_auth_pack
                                                                    =
                                                                    auth_pack;
                                                                auth_pack =
                                                                    0 as
                                                                        *mut krb5_auth_pack;
                                                                if is_signed
                                                                       != 0 {
                                                                    retval =
                                                                        check_log_freshness(context,
                                                                                            plgctx,
                                                                                            request,
                                                                                            valid_freshness_token);
                                                                    if retval
                                                                           !=
                                                                           0 {
                                                                        current_block
                                                                            =
                                                                            609915172598826250;
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            4928646496496689183;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        4928646496496689183;
                                                                }
                                                                match current_block
                                                                    {
                                                                    609915172598826250
                                                                    => {
                                                                    }
                                                                    _ => {
                                                                        if is_signed
                                                                               !=
                                                                               0
                                                                               &&
                                                                               !(*plgctx).auth_indicators.is_null()
                                                                           {
                                                                            /* Assert configured authentication indicators. */
                                                                            sp
                                                                                =
                                                                                (*plgctx).auth_indicators;
                                                                            loop 
                                                                                 {
                                                                                if (*sp).is_null()
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        9500030526577190060;
                                                                                    break
                                                                                        ;
                                                                                }
                                                                                retval
                                                                                    =
                                                                                    (*cb).add_auth_indicator.expect("non-null function pointer")(context,
                                                                                                                                                 rock,
                                                                                                                                                 *sp);
                                                                                if retval
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    current_block
                                                                                        =
                                                                                        609915172598826250;
                                                                                    break
                                                                                        ;
                                                                                }
                                                                                sp
                                                                                    =
                                                                                    sp.offset(1)
                                                                            }
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                9500030526577190060;
                                                                        }
                                                                        match current_block
                                                                            {
                                                                            609915172598826250
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                /* remember to set the PREAUTH flag in the reply */
                                                                                (*enc_tkt_reply).flags
                                                                                    |=
                                                                                    0x200000
                                                                                        as
                                                                                        libc::c_int;
                                                                                modreq
                                                                                    =
                                                                                    reqctx
                                                                                        as
                                                                                        krb5_kdcpreauth_modreq;
                                                                                reqctx
                                                                                    =
                                                                                    0
                                                                                        as
                                                                                        pkinit_kdc_req_context
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
            }
        }
    }
    if retval != 0 && (*data).pa_type == 16 as libc::c_int {
        pkiDebug(b"pkinit_verify_padata failed: creating e-data\n\x00" as
                     *const u8 as *const libc::c_char);
        if pkinit_create_edata(context, (*plgctx).cryptoctx,
                               (*reqctx).cryptoctx, (*plgctx).idctx,
                               (*plgctx).opts, retval, &mut e_data) != 0 {
            pkiDebug(b"pkinit_create_edata failed\n\x00" as *const u8 as
                         *const libc::c_char);
        }
    }
    free_krb5_pa_pk_as_req(&mut reqp);
    free(cksum.contents as *mut libc::c_void);
    free(authp_data.data as *mut libc::c_void);
    free(krb5_authz.data as *mut libc::c_void);
    if !reqctx.is_null() {
        pkinit_fini_kdc_req_context(context, reqctx as *mut libc::c_void);
    }
    free_krb5_auth_pack(&mut auth_pack);
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          retval,
                                                                                          modreq,
                                                                                          e_data,
                                                                                          0
                                                                                              as
                                                                                              *mut *mut krb5_authdata);
}
#[c2rust::src_loc = "637:1"]
unsafe extern "C" fn return_pkinit_kx(mut context: krb5_context,
                                      mut request: *mut krb5_kdc_req,
                                      mut reply: *mut krb5_kdc_rep,
                                      mut encrypting_key: *mut krb5_keyblock,
                                      mut out_padata: *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut session: *mut krb5_keyblock =
        (*(*(*reply).ticket).enc_part2).session;
    let mut new_session: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut enc: krb5_enc_data =
        krb5_enc_data{magic: 0,
                      enctype: 0,
                      kvno: 0,
                      ciphertext:
                          krb5_data{magic: 0,
                                    length: 0,
                                    data: 0 as *mut libc::c_char,},};
    let mut scratch: *mut krb5_data = 0 as *mut krb5_data;
    *out_padata = 0 as *mut krb5_pa_data;
    enc.ciphertext.data = 0 as *mut libc::c_char;
    if krb5_principal_compare(context,
                              (*request).client as krb5_const_principal,
                              krb5_anonymous_principal()) == 0 {
        return 0 as libc::c_int
    }
    /*
     * The KDC contribution key needs to be a fresh key of an enctype supported
     * by the client and server. The existing session key meets these
     * requirements so we use it.
     */
    ret =
        krb5_c_fx_cf2_simple(context, session,
                             b"PKINIT\x00" as *const u8 as
                                 *const libc::c_char, encrypting_key,
                             b"KEYEXCHANGE\x00" as *const u8 as
                                 *const libc::c_char, &mut new_session);
    if !(ret != 0) {
        ret = encode_krb5_encryption_key(session, &mut scratch);
        if !(ret != 0) {
            ret =
                krb5_encrypt_helper(context, encrypting_key,
                                    44 as libc::c_int, scratch, &mut enc);
            if !(ret != 0) {
                memset((*scratch).data as *mut libc::c_void, 0 as libc::c_int,
                       (*scratch).length as libc::c_ulong);
                krb5_free_data(context, scratch);
                scratch = 0 as *mut krb5_data;
                ret = encode_krb5_enc_data(&mut enc, &mut scratch);
                if !(ret != 0) {
                    pa =
                        malloc(::std::mem::size_of::<krb5_pa_data>() as
                                   libc::c_ulong) as *mut krb5_pa_data;
                    if pa.is_null() {
                        ret = 12 as libc::c_int
                    } else {
                        (*pa).pa_type = 147 as libc::c_int;
                        (*pa).length = (*scratch).length;
                        (*pa).contents = (*scratch).data as *mut krb5_octet;
                        *out_padata = pa;
                        (*scratch).data = 0 as *mut libc::c_char;
                        memset((*session).contents as *mut libc::c_void,
                               0 as libc::c_int,
                               (*session).length as libc::c_ulong);
                        krb5_free_keyblock_contents(context, session);
                        *session = *new_session;
                        (*new_session).contents = 0 as *mut krb5_octet
                    }
                }
            }
        }
    }
    krb5_free_data_contents(context, &mut enc.ciphertext);
    krb5_free_keyblock(context, new_session);
    krb5_free_data(context, scratch);
    return ret;
}
#[c2rust::src_loc = "698:1"]
unsafe extern "C" fn pkinit_pick_kdf_alg(mut context: krb5_context,
                                         mut kdf_list: *mut *mut krb5_data,
                                         mut alg_oid: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut req_oid: *mut krb5_data = 0 as *mut krb5_data;
    let mut supp_oid: *const krb5_data = 0 as *const krb5_data;
    let mut tmp_oid: *mut krb5_data = 0 as *mut krb5_data;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    /* if we don't find a match, return NULL value */
    *alg_oid = 0 as *mut krb5_data;
    /* for each of the OIDs that the server supports... */
    i = 0 as libc::c_int;
    's_22:
        loop  {
            supp_oid = *supported_kdf_alg_ids.as_ptr().offset(i as isize);
            if supp_oid.is_null() { break ; }
            /* if the requested OID is in the client's list, use it. */
            j = 0 as libc::c_int;
            loop  {
                req_oid = *kdf_list.offset(j as isize);
                if req_oid.is_null() { break ; }
                if (*req_oid).length == (*supp_oid).length &&
                       0 as libc::c_int ==
                           memcmp((*req_oid).data as *const libc::c_void,
                                  (*supp_oid).data as *const libc::c_void,
                                  (*req_oid).length as libc::c_ulong) {
                    tmp_oid =
                        k5alloc(::std::mem::size_of::<krb5_data>() as
                                    libc::c_ulong, &mut retval) as
                            *mut krb5_data;
                    if retval != 0 { break 's_22 ; }
                    (*tmp_oid).data =
                        k5memdup((*supp_oid).data as *const libc::c_void,
                                 (*supp_oid).length as size_t, &mut retval) as
                            *mut libc::c_char;
                    if retval != 0 { break 's_22 ; }
                    (*tmp_oid).length = (*supp_oid).length;
                    *alg_oid = tmp_oid;
                    /* don't free the OID in clean-up if we are returning it */
                    tmp_oid = 0 as *mut krb5_data;
                    break 's_22 ;
                } else { j += 1 }
            }
            i += 1
        }
    if !tmp_oid.is_null() { krb5_free_data(context, tmp_oid); }
    return retval;
}
#[c2rust::src_loc = "738:1"]
unsafe extern "C" fn pkinit_server_return_padata(mut context: krb5_context,
                                                 mut padata:
                                                     *mut krb5_pa_data,
                                                 mut req_pkt: *mut krb5_data,
                                                 mut request:
                                                     *mut krb5_kdc_req,
                                                 mut reply: *mut krb5_kdc_rep,
                                                 mut encrypting_key:
                                                     *mut krb5_keyblock,
                                                 mut send_pa:
                                                     *mut *mut krb5_pa_data,
                                                 mut cb:
                                                     krb5_kdcpreauth_callbacks,
                                                 mut rock:
                                                     krb5_kdcpreauth_rock,
                                                 mut moddata:
                                                     krb5_kdcpreauth_moddata,
                                                 mut modreq:
                                                     krb5_kdcpreauth_modreq)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut scratch: krb5_data =
        {
            let mut init =
                _krb5_data{magic: 0 as libc::c_int,
                           length: 0 as libc::c_int as libc::c_uint,
                           data: 0 as *mut libc::c_char,};
            init
        };
    let mut reqp: *mut krb5_pa_pk_as_req = 0 as *mut krb5_pa_pk_as_req;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut subjectPublicKey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dh_pubkey: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut server_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut subjectPublicKey_len: libc::c_uint =
        0 as libc::c_int as libc::c_uint;
    let mut server_key_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dh_pubkey_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dhkey_info: krb5_kdc_dh_key_info =
        krb5_kdc_dh_key_info{subjectPublicKey:
                                 krb5_data{magic: 0,
                                           length: 0,
                                           data: 0 as *mut libc::c_char,},
                             nonce: 0,
                             dhKeyExpiration: 0,};
    let mut encoded_dhkey_info: *mut krb5_data = 0 as *mut krb5_data;
    let mut rep: *mut krb5_pa_pk_as_rep = 0 as *mut krb5_pa_pk_as_rep;
    let mut out_data: *mut krb5_data = 0 as *mut krb5_data;
    let mut secret: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut enctype: krb5_enctype = -(1 as libc::c_int);
    let mut key_pack: *mut krb5_reply_key_pack =
        0 as *mut krb5_reply_key_pack;
    let mut encoded_key_pack: *mut krb5_data = 0 as *mut krb5_data;
    let mut plgctx: pkinit_kdc_context = 0 as *mut _pkinit_kdc_context;
    let mut reqctx: pkinit_kdc_req_context =
        0 as *mut _pkinit_kdc_req_context;
    *send_pa = 0 as *mut krb5_pa_data;
    if (*padata).pa_type == 147 as libc::c_int {
        return return_pkinit_kx(context, request, reply, encrypting_key,
                                send_pa)
    }
    if (*padata).length <= 0 as libc::c_int as libc::c_uint ||
           (*padata).contents.is_null() {
        return 0 as libc::c_int
    }
    if modreq.is_null() {
        pkiDebug(b"missing request context \n\x00" as *const u8 as
                     *const libc::c_char);
        return 22 as libc::c_int
    }
    plgctx = pkinit_find_realm_context(context, moddata, (*request).server);
    if plgctx.is_null() {
        pkiDebug(b"Unable to locate correct realm context\n\x00" as *const u8
                     as *const libc::c_char);
        return 2 as libc::c_int
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"PKINIT server returning PA data\x00" as *const u8 as
                          *const libc::c_char);
    }
    reqctx = modreq as pkinit_kdc_req_context;
    if !(*encrypting_key).contents.is_null() {
        free((*encrypting_key).contents as *mut libc::c_void);
        (*encrypting_key).length = 0 as libc::c_int as libc::c_uint;
        (*encrypting_key).contents = 0 as *mut krb5_octet
    }
    i = 0 as libc::c_int;
    while i < (*request).nktypes {
        enctype = *(*request).ktype.offset(i as isize);
        if krb5_c_valid_enctype(enctype) == 0 {
            i += 1
        } else {
            pkiDebug(b"KDC picked etype = %d\n\x00" as *const u8 as
                         *const libc::c_char, enctype);
            break ;
        }
    }
    if i == (*request).nktypes {
        retval = -(1765328370 as libc::c_long) as krb5_error_code
    } else {
        init_krb5_pa_pk_as_rep(&mut rep);
        if rep.is_null() {
            retval = 12 as libc::c_int
        } else {
            /* let's assume it's RSA. we'll reset it to DH if needed */
            (*rep).choice = choice_pa_pk_as_rep_encKeyPack;
            if !(*reqctx).rcv_auth_pack.is_null() &&
                   !(*(*reqctx).rcv_auth_pack).clientPublicValue.is_null() {
                subjectPublicKey =
                    (*(*(*reqctx).rcv_auth_pack).clientPublicValue).subjectPublicKey.data
                        as *mut libc::c_uchar;
                subjectPublicKey_len =
                    (*(*(*reqctx).rcv_auth_pack).clientPublicValue).subjectPublicKey.length;
                (*rep).choice = choice_pa_pk_as_rep_dhInfo;
                pkiDebug(b"received DH key delivery AS REQ\n\x00" as *const u8
                             as *const libc::c_char);
                retval =
                    server_process_dh(context, (*plgctx).cryptoctx,
                                      (*reqctx).cryptoctx, (*plgctx).idctx,
                                      subjectPublicKey, subjectPublicKey_len,
                                      &mut dh_pubkey, &mut dh_pubkey_len,
                                      &mut server_key, &mut server_key_len);
                if retval != 0 {
                    pkiDebug(b"failed to process/create dh parameters\n\x00"
                                 as *const u8 as *const libc::c_char);
                    current_block = 15329334428063834850;
                } else {
                    /*
         * This is DH, so don't generate the key until after we
         * encode the reply, because the encoded reply is needed
         * to generate the key in some cases.
         */
                    dhkey_info.subjectPublicKey.length = dh_pubkey_len;
                    dhkey_info.subjectPublicKey.data =
                        dh_pubkey as *mut libc::c_char;
                    dhkey_info.nonce = (*request).nonce;
                    dhkey_info.dhKeyExpiration = 0 as libc::c_int;
                    retval =
                        k5int_encode_krb5_kdc_dh_key_info.expect("non-null function pointer")(&mut dhkey_info,
                                                                                              &mut encoded_dhkey_info);
                    if retval != 0 {
                        pkiDebug(b"encode_krb5_kdc_dh_key_info failed\n\x00"
                                     as *const u8 as *const libc::c_char);
                        current_block = 15329334428063834850;
                    } else {
                        retval =
                            cms_signeddata_create(context,
                                                  (*plgctx).cryptoctx,
                                                  (*reqctx).cryptoctx,
                                                  (*plgctx).idctx,
                                                  CMS_SIGN_SERVER as
                                                      libc::c_int,
                                                  1 as libc::c_int,
                                                  (*encoded_dhkey_info).data
                                                      as *mut libc::c_uchar,
                                                  (*encoded_dhkey_info).length,
                                                  &mut (*rep).u.dh_Info.dhSignedData.data
                                                      as
                                                      *mut *mut libc::c_char
                                                      as
                                                      *mut *mut libc::c_uchar,
                                                  &mut (*rep).u.dh_Info.dhSignedData.length);
                        if retval != 0 {
                            pkiDebug(b"failed to create pkcs7 signed data\n\x00"
                                         as *const u8 as *const libc::c_char);
                            current_block = 15329334428063834850;
                        } else { current_block = 8716029205547827362; }
                    }
                }
            } else {
                pkiDebug(b"received RSA key delivery AS REQ\n\x00" as
                             *const u8 as *const libc::c_char);
                retval =
                    krb5_c_make_random_key(context, enctype, encrypting_key);
                if retval != 0 {
                    pkiDebug(b"unable to make a session key\n\x00" as
                                 *const u8 as *const libc::c_char);
                    current_block = 15329334428063834850;
                } else {
                    init_krb5_reply_key_pack(&mut key_pack);
                    if key_pack.is_null() {
                        retval = 12 as libc::c_int;
                        current_block = 15329334428063834850;
                    } else {
                        retval =
                            krb5_c_make_checksum(context, 0 as libc::c_int,
                                                 encrypting_key,
                                                 6 as libc::c_int, req_pkt,
                                                 &mut (*key_pack).asChecksum);
                        if retval != 0 {
                            pkiDebug(b"unable to calculate AS REQ checksum\n\x00"
                                         as *const u8 as *const libc::c_char);
                            current_block = 15329334428063834850;
                        } else {
                            krb5_copy_keyblock_contents(context,
                                                        encrypting_key,
                                                        &mut (*key_pack).replyKey);
                            retval =
                                k5int_encode_krb5_reply_key_pack.expect("non-null function pointer")(key_pack,
                                                                                                     &mut encoded_key_pack);
                            if retval != 0 {
                                pkiDebug(b"failed to encode reply_key_pack\n\x00"
                                             as *const u8 as
                                             *const libc::c_char);
                                current_block = 15329334428063834850;
                            } else {
                                (*rep).choice =
                                    choice_pa_pk_as_rep_encKeyPack;
                                retval =
                                    cms_envelopeddata_create(context,
                                                             (*plgctx).cryptoctx,
                                                             (*reqctx).cryptoctx,
                                                             (*plgctx).idctx,
                                                             (*padata).pa_type,
                                                             1 as libc::c_int,
                                                             (*encoded_key_pack).data
                                                                 as
                                                                 *mut libc::c_uchar,
                                                             (*encoded_key_pack).length,
                                                             &mut (*rep).u.encKeyPack.data
                                                                 as
                                                                 *mut *mut libc::c_char
                                                                 as
                                                                 *mut *mut libc::c_uchar,
                                                             &mut (*rep).u.encKeyPack.length);
                                if retval != 0 {
                                    pkiDebug(b"failed to create pkcs7 enveloped data: %s\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             error_message(retval as
                                                               errcode_t));
                                    current_block = 15329334428063834850;
                                } else {
                                    current_block = 8716029205547827362;
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                15329334428063834850 => { }
                _ => {
                    if (*rep).choice as libc::c_int ==
                           choice_pa_pk_as_rep_dhInfo as libc::c_int &&
                           (!(*reqctx).rcv_auth_pack.is_null() &&
                                !(*(*reqctx).rcv_auth_pack).supportedKDFs.is_null())
                       {
                        /* If using the alg-agility KDF, put the algorithm in the reply
         * before encoding it.
         */
                        if !(*reqctx).rcv_auth_pack.is_null() &&
                               !(*(*reqctx).rcv_auth_pack).supportedKDFs.is_null()
                           {
                            retval =
                                pkinit_pick_kdf_alg(context,
                                                    (*(*reqctx).rcv_auth_pack).supportedKDFs,
                                                    &mut (*rep).u.dh_Info.kdfID);
                            if retval != 0 {
                                pkiDebug(b"pkinit_pick_kdf_alg failed: %s\n\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         error_message(retval as errcode_t));
                                current_block = 15329334428063834850;
                            } else { current_block = 14541395414537699361; }
                        } else { current_block = 14541395414537699361; }
                    } else { current_block = 14541395414537699361; }
                    match current_block {
                        15329334428063834850 => { }
                        _ => {
                            retval =
                                k5int_encode_krb5_pa_pk_as_rep.expect("non-null function pointer")(rep,
                                                                                                   &mut out_data);
                            if retval != 0 {
                                pkiDebug(b"failed to encode AS_REP\n\x00" as
                                             *const u8 as
                                             *const libc::c_char);
                            } else {
                                /* If this is DH, we haven't computed the key yet, so do it now. */
                                if (*rep).choice as libc::c_int ==
                                       choice_pa_pk_as_rep_dhInfo as
                                           libc::c_int {
                                    /* If mutually supported KDFs were found, use the algorithm agility
         * KDF. */
                                    if !(*rep).u.dh_Info.kdfID.is_null() {
                                        secret.data =
                                            server_key as *mut libc::c_char;
                                        secret.length = server_key_len;
                                        retval =
                                            pkinit_alg_agility_kdf(context,
                                                                   &mut secret,
                                                                   (*rep).u.dh_Info.kdfID,
                                                                   (*request).client
                                                                       as
                                                                       krb5_const_principal,
                                                                   (*request).server
                                                                       as
                                                                       krb5_const_principal,
                                                                   enctype,
                                                                   req_pkt,
                                                                   out_data,
                                                                   encrypting_key);
                                        if retval != 0 {
                                            pkiDebug(b"pkinit_alg_agility_kdf failed: %s\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     error_message(retval as
                                                                       errcode_t));
                                            current_block =
                                                15329334428063834850;
                                        } else {
                                            current_block =
                                                4804377075063615140;
                                        }
                                        /* Otherwise, use the older octetstring2key() function */
                                    } else {
                                        retval =
                                            pkinit_octetstring2key(context,
                                                                   enctype,
                                                                   server_key,
                                                                   server_key_len,
                                                                   encrypting_key);
                                        if retval != 0 {
                                            pkiDebug(b"pkinit_octetstring2key failed: %s\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char,
                                                     error_message(retval as
                                                                       errcode_t));
                                            current_block =
                                                15329334428063834850;
                                        } else {
                                            current_block =
                                                4804377075063615140;
                                        }
                                    }
                                } else {
                                    current_block = 4804377075063615140;
                                }
                                match current_block {
                                    15329334428063834850 => { }
                                    _ => {
                                        *send_pa =
                                            malloc(::std::mem::size_of::<krb5_pa_data>()
                                                       as libc::c_ulong) as
                                                *mut krb5_pa_data;
                                        if (*send_pa).is_null() {
                                            retval = 12 as libc::c_int;
                                            free((*out_data).data as
                                                     *mut libc::c_void);
                                            free(out_data as
                                                     *mut libc::c_void);
                                            out_data = 0 as *mut krb5_data
                                        } else {
                                            (**send_pa).magic =
                                                -(1760647406 as libc::c_long)
                                                    as krb5_magic;
                                            (**send_pa).pa_type =
                                                17 as libc::c_int;
                                            (**send_pa).length =
                                                (*out_data).length;
                                            (**send_pa).contents =
                                                (*out_data).data as
                                                    *mut krb5_octet
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
    pkinit_fini_kdc_req_context(context, reqctx as *mut libc::c_void);
    free(scratch.data as *mut libc::c_void);
    free(out_data as *mut libc::c_void);
    if !encoded_dhkey_info.is_null() {
        krb5_free_data(context, encoded_dhkey_info);
    }
    if !encoded_key_pack.is_null() {
        krb5_free_data(context, encoded_key_pack);
    }
    free(dh_pubkey as *mut libc::c_void);
    free(server_key as *mut libc::c_void);
    free_krb5_pa_pk_as_req(&mut reqp);
    free_krb5_pa_pk_as_rep(&mut rep);
    free_krb5_reply_key_pack(&mut key_pack);
    if retval != 0 {
        pkiDebug(b"pkinit_verify_padata failure\x00" as *const u8 as
                     *const libc::c_char);
    }
    return retval;
}
#[c2rust::src_loc = "1042:1"]
unsafe extern "C" fn pkinit_server_get_flags(mut kcontext: krb5_context,
                                             mut patype: krb5_preauthtype)
 -> libc::c_int {
    if patype == 147 as libc::c_int { return 0x2 as libc::c_int }
    return 0x10 as libc::c_int | 0x20 as libc::c_int | 0x100 as libc::c_int;
}
#[c2rust::src_loc = "1050:25"]
static mut supported_server_pa_types: [krb5_preauthtype; 3] =
    [16 as libc::c_int, 147 as libc::c_int, 0 as libc::c_int];
#[c2rust::src_loc = "1056:1"]
unsafe extern "C" fn pkinit_fini_kdc_profile(mut context: krb5_context,
                                             mut plgctx: pkinit_kdc_context) {
    /*
     * There is nothing currently allocated by pkinit_init_kdc_profile()
     * which needs to be freed here.
     */
}
#[c2rust::src_loc = "1065:1"]
unsafe extern "C" fn pkinit_init_kdc_profile(mut context: krb5_context,
                                             mut plgctx: pkinit_kdc_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut eku_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ocsp_check: *mut libc::c_char = 0 as *mut libc::c_char;
    pkiDebug(b"%s: entered for realm %s\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 24],
                                       &[libc::c_char; 24]>(b"pkinit_init_kdc_profile\x00")).as_ptr(),
             (*plgctx).realmname);
    retval =
        pkinit_kdcdefault_string(context, (*plgctx).realmname,
                                 b"pkinit_identity\x00" as *const u8 as
                                     *const libc::c_char,
                                 &mut (*(*plgctx).idopts).identity);
    if retval != 0 as libc::c_int || (*(*plgctx).idopts).identity.is_null() {
        retval = 22 as libc::c_int;
        krb5_set_error_message(context, retval,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"No pkinit_identity supplied for realm %s\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               (*plgctx).realmname);
    } else {
        retval =
            pkinit_kdcdefault_strings(context, (*plgctx).realmname,
                                      b"pkinit_anchors\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut (*(*plgctx).idopts).anchors);
        if retval != 0 as libc::c_int || (*(*plgctx).idopts).anchors.is_null()
           {
            retval = 22 as libc::c_int;
            krb5_set_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"No pkinit_anchors supplied for realm %s\x00"
                                                as *const u8 as
                                                *const libc::c_char),
                                   (*plgctx).realmname);
        } else {
            pkinit_kdcdefault_strings(context, (*plgctx).realmname,
                                      b"pkinit_pool\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut (*(*plgctx).idopts).intermediates);
            pkinit_kdcdefault_strings(context, (*plgctx).realmname,
                                      b"pkinit_revoke\x00" as *const u8 as
                                          *const libc::c_char,
                                      &mut (*(*plgctx).idopts).crls);
            pkinit_kdcdefault_string(context, (*plgctx).realmname,
                                     b"pkinit_kdc_ocsp\x00" as *const u8 as
                                         *const libc::c_char,
                                     &mut ocsp_check);
            if !ocsp_check.is_null() {
                free(ocsp_check as *mut libc::c_void);
                retval = 95 as libc::c_int;
                krb5_set_error_message(context, retval,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"OCSP is not supported: (realm: %s)\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       (*plgctx).realmname);
            } else {
                pkinit_kdcdefault_integer(context, (*plgctx).realmname,
                                          b"pkinit_dh_min_bits\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          2048 as libc::c_int,
                                          &mut (*(*plgctx).opts).dh_min_bits);
                if (*(*plgctx).opts).dh_min_bits < 1024 as libc::c_int {
                    pkiDebug(b"%s: invalid value (%d < %d) for pkinit_dh_min_bits, using default value (%d) instead\n\x00"
                                 as *const u8 as *const libc::c_char,
                             (*::std::mem::transmute::<&[u8; 24],
                                                       &[libc::c_char; 24]>(b"pkinit_init_kdc_profile\x00")).as_ptr(),
                             (*(*plgctx).opts).dh_min_bits,
                             1024 as libc::c_int, 2048 as libc::c_int);
                    (*(*plgctx).opts).dh_min_bits = 2048 as libc::c_int
                }
                pkinit_kdcdefault_boolean(context, (*plgctx).realmname,
                                          b"pkinit_allow_upn\x00" as *const u8
                                              as *const libc::c_char,
                                          0 as libc::c_int,
                                          &mut (*(*plgctx).opts).allow_upn);
                pkinit_kdcdefault_boolean(context, (*plgctx).realmname,
                                          b"pkinit_require_crl_checking\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          0 as libc::c_int,
                                          &mut (*(*plgctx).opts).require_crl_checking);
                pkinit_kdcdefault_boolean(context, (*plgctx).realmname,
                                          b"pkinit_require_freshness\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          0 as libc::c_int,
                                          &mut (*(*plgctx).opts).require_freshness);
                pkinit_kdcdefault_string(context, (*plgctx).realmname,
                                         b"pkinit_eku_checking\x00" as
                                             *const u8 as *const libc::c_char,
                                         &mut eku_string);
                if !eku_string.is_null() {
                    if strcasecmp(eku_string,
                                  b"kpClientAuth\x00" as *const u8 as
                                      *const libc::c_char) == 0 as libc::c_int
                       {
                        (*(*plgctx).opts).require_eku = 1 as libc::c_int;
                        (*(*plgctx).opts).accept_secondary_eku =
                            0 as libc::c_int
                    } else if strcasecmp(eku_string,
                                         b"scLogin\x00" as *const u8 as
                                             *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*(*plgctx).opts).require_eku = 1 as libc::c_int;
                        (*(*plgctx).opts).accept_secondary_eku =
                            1 as libc::c_int
                    } else if strcasecmp(eku_string,
                                         b"none\x00" as *const u8 as
                                             *const libc::c_char) ==
                                  0 as libc::c_int {
                        (*(*plgctx).opts).require_eku = 0 as libc::c_int;
                        (*(*plgctx).opts).accept_secondary_eku =
                            0 as libc::c_int
                    } else {
                        pkiDebug(b"%s: Invalid value for pkinit_eku_checking: \'%s\'\n\x00"
                                     as *const u8 as *const libc::c_char,
                                 (*::std::mem::transmute::<&[u8; 24],
                                                           &[libc::c_char; 24]>(b"pkinit_init_kdc_profile\x00")).as_ptr(),
                                 eku_string);
                    }
                    free(eku_string as *mut libc::c_void);
                }
                pkinit_kdcdefault_strings(context, (*plgctx).realmname,
                                          b"pkinit_indicator\x00" as *const u8
                                              as *const libc::c_char,
                                          &mut (*plgctx).auth_indicators);
                return 0 as libc::c_int
            }
        }
    }
    pkinit_fini_kdc_profile(context, plgctx);
    return retval;
}
#[c2rust::src_loc = "1168:1"]
unsafe extern "C" fn pkinit_find_realm_context(mut context: krb5_context,
                                               mut moddata:
                                                   krb5_kdcpreauth_moddata,
                                               mut princ: krb5_principal)
 -> pkinit_kdc_context {
    let mut i: libc::c_int = 0;
    let mut realm_contexts: *mut pkinit_kdc_context =
        0 as *mut pkinit_kdc_context;
    if moddata.is_null() { return 0 as pkinit_kdc_context }
    realm_contexts = (*moddata).realm_contexts;
    if realm_contexts.is_null() { return 0 as pkinit_kdc_context }
    i = 0 as libc::c_int;
    while !(*realm_contexts.offset(i as isize)).is_null() {
        let mut p: pkinit_kdc_context = *realm_contexts.offset(i as isize);
        if (*p).realmname_len == (*princ).realm.length &&
               strncmp((*p).realmname, (*princ).realm.data,
                       (*p).realmname_len as libc::c_ulong) ==
                   0 as libc::c_int {
            pkiDebug(b"%s: returning context at %p for realm \'%s\'\n\x00" as
                         *const u8 as *const libc::c_char,
                     (*::std::mem::transmute::<&[u8; 26],
                                               &[libc::c_char; 26]>(b"pkinit_find_realm_context\x00")).as_ptr(),
                     p, (*p).realmname);
            return p
        }
        i += 1
    }
    pkiDebug(b"%s: unable to find realm context for realm \'%.*s\'\n\x00" as
                 *const u8 as *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 26],
                                       &[libc::c_char; 26]>(b"pkinit_find_realm_context\x00")).as_ptr(),
             (*princ).realm.length, (*princ).realm.data);
    return 0 as pkinit_kdc_context;
}
#[c2rust::src_loc = "1198:1"]
unsafe extern "C" fn pkinit_server_plugin_init_realm(mut context:
                                                         krb5_context,
                                                     mut realmname:
                                                         *const libc::c_char,
                                                     mut pplgctx:
                                                         *mut pkinit_kdc_context)
 -> libc::c_int {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut plgctx: pkinit_kdc_context = 0 as pkinit_kdc_context;
    *pplgctx = 0 as pkinit_kdc_context;
    plgctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_pkinit_kdc_context>() as libc::c_ulong)
            as pkinit_kdc_context;
    if !plgctx.is_null() {
        pkiDebug(b"%s: initializing context at %p for realm \'%s\'\n\x00" as
                     *const u8 as *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 32],
                                           &[libc::c_char; 32]>(b"pkinit_server_plugin_init_realm\x00")).as_ptr(),
                 plgctx, realmname);
        memset(plgctx as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_pkinit_kdc_context>() as libc::c_ulong);
        (*plgctx).magic = 0x5551212 as libc::c_int;
        (*plgctx).realmname = strdup(realmname);
        if !(*plgctx).realmname.is_null() {
            (*plgctx).realmname_len =
                strlen((*plgctx).realmname) as libc::c_uint;
            retval = pkinit_init_plg_crypto(&mut (*plgctx).cryptoctx);
            if !(retval != 0) {
                retval = pkinit_init_plg_opts(&mut (*plgctx).opts);
                if !(retval != 0) {
                    retval =
                        pkinit_init_identity_crypto(&mut (*plgctx).idctx);
                    if !(retval != 0) {
                        retval =
                            pkinit_init_identity_opts(&mut (*plgctx).idopts);
                        if !(retval != 0) {
                            retval = pkinit_init_kdc_profile(context, plgctx);
                            if !(retval != 0) {
                                retval =
                                    pkinit_identity_initialize(context,
                                                               (*plgctx).cryptoctx,
                                                               0 as
                                                                   pkinit_req_crypto_context,
                                                               (*plgctx).idopts,
                                                               (*plgctx).idctx,
                                                               0 as
                                                                   krb5_clpreauth_callbacks,
                                                               0 as
                                                                   krb5_clpreauth_rock,
                                                               0 as
                                                                   krb5_principal);
                                if !(retval != 0) {
                                    retval =
                                        pkinit_identity_prompt(context,
                                                               (*plgctx).cryptoctx,
                                                               0 as
                                                                   pkinit_req_crypto_context,
                                                               (*plgctx).idopts,
                                                               (*plgctx).idctx,
                                                               0 as
                                                                   krb5_clpreauth_callbacks,
                                                               0 as
                                                                   krb5_clpreauth_rock,
                                                               0 as
                                                                   libc::c_int,
                                                               0 as
                                                                   krb5_principal);
                                    if !(retval != 0) {
                                        pkiDebug(b"%s: returning context at %p for realm \'%s\'\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 (*::std::mem::transmute::<&[u8; 32],
                                                                           &[libc::c_char; 32]>(b"pkinit_server_plugin_init_realm\x00")).as_ptr(),
                                                 plgctx, realmname);
                                        *pplgctx = plgctx;
                                        retval = 0 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if retval != 0 { pkinit_server_plugin_fini_realm(context, plgctx); }
    return retval;
}
#[c2rust::src_loc = "1264:1"]
unsafe extern "C" fn pkinit_san_authorize(mut context: krb5_context,
                                          mut moddata: krb5_certauth_moddata,
                                          mut cert: *const uint8_t,
                                          mut cert_len: size_t,
                                          mut princ: krb5_const_principal,
                                          mut opts: *const libc::c_void,
                                          mut db_entry:
                                              *const _krb5_db_entry_new,
                                          mut authinds_out:
                                              *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut valid_san: libc::c_int = 0;
    let mut req_opts: *const certauth_req_opts =
        opts as *const certauth_req_opts;
    *authinds_out = 0 as *mut *mut libc::c_char;
    ret =
        verify_client_san(context, (*req_opts).plgctx, (*req_opts).reqctx,
                          (*req_opts).cb, (*req_opts).rock, princ,
                          &mut valid_san);
    if ret == 2 as libc::c_int {
        return -(1765328135 as libc::c_long) as krb5_error_code
    } else { if ret != 0 { return ret } }
    if valid_san == 0 {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT server found no acceptable SAN in client cert\x00"
                              as *const u8 as *const libc::c_char);
        }
        return -(1765328309 as libc::c_long) as krb5_error_code
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1292:1"]
unsafe extern "C" fn pkinit_eku_authorize(mut context: krb5_context,
                                          mut moddata: krb5_certauth_moddata,
                                          mut cert: *const uint8_t,
                                          mut cert_len: size_t,
                                          mut princ: krb5_const_principal,
                                          mut opts: *const libc::c_void,
                                          mut db_entry:
                                              *const _krb5_db_entry_new,
                                          mut authinds_out:
                                              *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut valid_eku: libc::c_int = 0;
    let mut req_opts: *const certauth_req_opts =
        opts as *const certauth_req_opts;
    *authinds_out = 0 as *mut *mut libc::c_char;
    /* Verify the client EKU. */
    ret =
        verify_client_eku(context, (*req_opts).plgctx, (*req_opts).reqctx,
                          &mut valid_eku);
    if ret != 0 { return ret }
    if valid_eku == 0 {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT server found no acceptable EKU in client cert\x00"
                              as *const u8 as *const libc::c_char);
        }
        return -(1765328307 as libc::c_long) as krb5_error_code
    }
    return -(1765328135 as libc::c_long) as krb5_error_code;
}
#[c2rust::src_loc = "1319:1"]
unsafe extern "C" fn certauth_pkinit_san_initvt(mut context: krb5_context,
                                                mut maj_ver: libc::c_int,
                                                mut min_ver: libc::c_int,
                                                mut vtable:
                                                    krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_certauth_vtable = 0 as *mut krb5_certauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_certauth_vtable;
    (*vt).name = b"pkinit_san\x00" as *const u8 as *const libc::c_char;
    (*vt).authorize =
        Some(pkinit_san_authorize as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *const uint8_t, _: size_t,
                                      _: krb5_const_principal,
                                      _: *const libc::c_void,
                                      _: *const _krb5_db_entry_new,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1333:1"]
unsafe extern "C" fn certauth_pkinit_eku_initvt(mut context: krb5_context,
                                                mut maj_ver: libc::c_int,
                                                mut min_ver: libc::c_int,
                                                mut vtable:
                                                    krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_certauth_vtable = 0 as *mut krb5_certauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_certauth_vtable;
    (*vt).name = b"pkinit_eku\x00" as *const u8 as *const libc::c_char;
    (*vt).authorize =
        Some(pkinit_eku_authorize as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *const uint8_t, _: size_t,
                                      _: krb5_const_principal,
                                      _: *const libc::c_void,
                                      _: *const _krb5_db_entry_new,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
/*
 * Do certificate auth based on a match expression in the pkinit_cert_match
 * attribute string.  An expression should be in the same form as those used
 * for the pkinit_cert_match configuration option.
 */
#[c2rust::src_loc = "1352:1"]
unsafe extern "C" fn dbmatch_authorize(mut context: krb5_context,
                                       mut moddata: krb5_certauth_moddata,
                                       mut cert: *const uint8_t,
                                       mut cert_len: size_t,
                                       mut princ: krb5_const_principal,
                                       mut opts: *const libc::c_void,
                                       mut db_entry:
                                           *const _krb5_db_entry_new,
                                       mut authinds_out:
                                           *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut req_opts: *const certauth_req_opts =
        opts as *const certauth_req_opts;
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut matched: krb5_boolean = 0;
    *authinds_out = 0 as *mut *mut libc::c_char;
    /* Fetch the matching pattern.  Pass if it isn't specified. */
    ret =
        (*(*req_opts).cb).get_string.expect("non-null function pointer")(context,
                                                                         (*req_opts).rock,
                                                                         b"pkinit_cert_match\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         &mut pattern);
    if ret != 0 { return ret }
    if pattern.is_null() {
        return -(1765328135 as libc::c_long) as krb5_error_code
    }
    /* Check the certificate against the match expression. */
    ret =
        pkinit_client_cert_match(context, (*(*req_opts).plgctx).cryptoctx,
                                 (*(*req_opts).reqctx).cryptoctx, pattern,
                                 &mut matched);
    (*(*req_opts).cb).free_string.expect("non-null function pointer")(context,
                                                                      (*req_opts).rock,
                                                                      pattern);
    if ret != 0 { return ret }
    return if matched != 0 {
               0 as libc::c_int as libc::c_long
           } else { -(1765328318 as libc::c_long) } as krb5_error_code;
}
#[c2rust::src_loc = "1384:1"]
unsafe extern "C" fn certauth_dbmatch_initvt(mut context: krb5_context,
                                             mut maj_ver: libc::c_int,
                                             mut min_ver: libc::c_int,
                                             mut vtable: krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_certauth_vtable = 0 as *mut krb5_certauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_certauth_vtable;
    (*vt).name = b"dbmatch\x00" as *const u8 as *const libc::c_char;
    (*vt).authorize =
        Some(dbmatch_authorize as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_certauth_moddata,
                                      _: *const uint8_t, _: size_t,
                                      _: krb5_const_principal,
                                      _: *const libc::c_void,
                                      _: *const _krb5_db_entry_new,
                                      _: *mut *mut *mut libc::c_char)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1398:1"]
unsafe extern "C" fn load_certauth_plugins(mut context: krb5_context,
                                           mut handle_out:
                                               *mut *mut certauth_handle)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut modules: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut mod_0: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut list: *mut certauth_handle = 0 as *mut certauth_handle;
    let mut h: certauth_handle = 0 as *mut certauth_module_handle_st;
    let mut count: size_t = 0;
    /* Register the builtin modules. */
    ret =
        k5_plugin_register(context, 10 as libc::c_int,
                           b"pkinit_san\x00" as *const u8 as
                               *const libc::c_char,
                           Some(certauth_pkinit_san_initvt as
                                    unsafe extern "C" fn(_: krb5_context,
                                                         _: libc::c_int,
                                                         _: libc::c_int,
                                                         _:
                                                             krb5_plugin_vtable)
                                        -> krb5_error_code));
    if !(ret != 0) {
        ret =
            k5_plugin_register(context, 10 as libc::c_int,
                               b"pkinit_eku\x00" as *const u8 as
                                   *const libc::c_char,
                               Some(certauth_pkinit_eku_initvt as
                                        unsafe extern "C" fn(_: krb5_context,
                                                             _: libc::c_int,
                                                             _: libc::c_int,
                                                             _:
                                                                 krb5_plugin_vtable)
                                            -> krb5_error_code));
        if !(ret != 0) {
            ret =
                k5_plugin_register(context, 10 as libc::c_int,
                                   b"dbmatch\x00" as *const u8 as
                                       *const libc::c_char,
                                   Some(certauth_dbmatch_initvt as
                                            unsafe extern "C" fn(_:
                                                                     krb5_context,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     krb5_plugin_vtable)
                                                -> krb5_error_code));
            if !(ret != 0) {
                ret =
                    k5_plugin_load_all(context, 10 as libc::c_int,
                                       &mut modules);
                if !(ret != 0) {
                    /* Allocate handle list. */
                    count = 0 as libc::c_int as size_t;
                    while (*modules.offset(count as isize)).is_some() {
                        count = count.wrapping_add(1)
                    }
                    list =
                        k5calloc(count.wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong),
                                 ::std::mem::size_of::<certauth_handle>() as
                                     libc::c_ulong, &mut ret) as
                            *mut certauth_handle;
                    if !list.is_null() {
                        /* Initialize each module, ignoring ones that fail. */
                        count = 0 as libc::c_int as size_t;
                        mod_0 = modules;
                        loop  {
                            if !(*mod_0).is_some() {
                                current_block = 9007357115414505193;
                                break ;
                            }
                            h =
                                k5calloc(1 as libc::c_int as size_t,
                                         ::std::mem::size_of::<certauth_module_handle_st>()
                                             as libc::c_ulong, &mut ret) as
                                    certauth_handle;
                            if h.is_null() {
                                current_block = 3397095492618554932;
                                break ;
                            }
                            ret =
                                (*mod_0).expect("non-null function pointer")(context,
                                                                             1
                                                                                 as
                                                                                 libc::c_int,
                                                                             1
                                                                                 as
                                                                                 libc::c_int,
                                                                             &mut (*h).vt
                                                                                 as
                                                                                 *mut krb5_certauth_vtable_st
                                                                                 as
                                                                                 krb5_plugin_vtable);
                            if ret != 0 {
                                if (*context).trace_callback.is_some() {
                                    krb5int_trace(context,
                                                  b"certauth module failed to init vtable: {kerr}\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  ret);
                                }
                                free(h as *mut libc::c_void);
                            } else {
                                (*h).moddata = 0 as krb5_certauth_moddata;
                                if (*h).vt.init.is_some() {
                                    ret =
                                        (*h).vt.init.expect("non-null function pointer")(context,
                                                                                         &mut (*h).moddata);
                                    if ret != 0 {
                                        if (*context).trace_callback.is_some()
                                           {
                                            krb5int_trace(context,
                                                          b"certauth module {str} failed to init: {kerr}\x00"
                                                              as *const u8 as
                                                              *const libc::c_char,
                                                          (*h).vt.name, ret);
                                        }
                                        free(h as *mut libc::c_void);
                                        current_block = 15976848397966268834;
                                    } else {
                                        current_block = 6417057564578538666;
                                    }
                                } else {
                                    current_block = 6417057564578538666;
                                }
                                match current_block {
                                    15976848397966268834 => { }
                                    _ => {
                                        let fresh0 = count;
                                        count = count.wrapping_add(1);
                                        let ref mut fresh1 =
                                            *list.offset(fresh0 as isize);
                                        *fresh1 = h;
                                        let ref mut fresh2 =
                                            *list.offset(count as isize);
                                        *fresh2 = 0 as certauth_handle
                                    }
                                }
                            }
                            mod_0 = mod_0.offset(1)
                        }
                        match current_block {
                            3397095492618554932 => { }
                            _ => {
                                let ref mut fresh3 =
                                    *list.offset(count as isize);
                                *fresh3 = 0 as certauth_handle;
                                ret = 0 as libc::c_int;
                                *handle_out = list;
                                list = 0 as *mut certauth_handle
                            }
                        }
                    }
                }
            }
        }
    }
    k5_plugin_free_modules(context, modules);
    free_certauth_handles(context, list);
    return ret;
}
#[c2rust::src_loc = "1469:1"]
unsafe extern "C" fn pkinit_server_plugin_init(mut context: krb5_context,
                                               mut moddata_out:
                                                   *mut krb5_kdcpreauth_moddata,
                                               mut realmnames:
                                                   *mut *const libc::c_char)
 -> libc::c_int {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut plgctx: pkinit_kdc_context = 0 as *mut _pkinit_kdc_context;
    let mut realm_contexts: *mut pkinit_kdc_context =
        0 as *mut pkinit_kdc_context;
    let mut certauth_modules: *mut certauth_handle =
        0 as *mut certauth_handle;
    let mut moddata: krb5_kdcpreauth_moddata =
        0 as *mut krb5_kdcpreauth_moddata_st;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut numrealms: size_t = 0;
    retval = pkinit_accessor_init();
    if retval != 0 { return retval }
    /* Determine how many realms we may need to support */
    i = 0 as libc::c_int as size_t;
    while !(*realmnames.offset(i as isize)).is_null() {
        i = i.wrapping_add(1)
    }
    numrealms = i;
    realm_contexts =
        calloc(numrealms.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<pkinit_kdc_context>() as libc::c_ulong)
            as *mut pkinit_kdc_context;
    if realm_contexts.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while i < numrealms {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"PKINIT server initializing realm {str}\x00" as
                              *const u8 as *const libc::c_char,
                          *realmnames.offset(i as isize));
        }
        krb5_clear_error_message(context);
        retval =
            pkinit_server_plugin_init_realm(context,
                                            *realmnames.offset(i as isize),
                                            &mut plgctx);
        if retval != 0 {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"PKINIT server initialization failed for realm {str}: {kerr}\x00"
                                  as *const u8 as *const libc::c_char,
                              *realmnames.offset(i as isize), retval);
            }
        } else {
            let fresh4 = j;
            j = j.wrapping_add(1);
            let ref mut fresh5 = *realm_contexts.offset(fresh4 as isize);
            *fresh5 = plgctx
        }
        i = i.wrapping_add(1)
    }
    if j == 0 as libc::c_int as libc::c_ulong {
        if numrealms == 1 as libc::c_int as libc::c_ulong {
            krb5_prepend_error_message(context, retval,
                                       b"PKINIT initialization failed\x00" as
                                           *const u8 as *const libc::c_char);
        } else {
            retval = 22 as libc::c_int;
            krb5_set_error_message(context, retval,
                                   dgettext(b"mit-krb5\x00" as *const u8 as
                                                *const libc::c_char,
                                            b"No realms configured correctly for pkinit support\x00"
                                                as *const u8 as
                                                *const libc::c_char));
        }
    } else {
        retval = load_certauth_plugins(context, &mut certauth_modules);
        if !(retval != 0) {
            moddata =
                k5calloc(1 as libc::c_int as size_t,
                         ::std::mem::size_of::<krb5_kdcpreauth_moddata_st>()
                             as libc::c_ulong, &mut retval) as
                    krb5_kdcpreauth_moddata;
            if !moddata.is_null() {
                (*moddata).realm_contexts = realm_contexts;
                (*moddata).certauth_modules = certauth_modules;
                *moddata_out = moddata;
                pkiDebug(b"%s: returning context at %p\n\x00" as *const u8 as
                             *const libc::c_char,
                         (*::std::mem::transmute::<&[u8; 26],
                                                   &[libc::c_char; 26]>(b"pkinit_server_plugin_init\x00")).as_ptr(),
                         moddata);
                return 0 as libc::c_int
            }
        }
    }
    free_realm_contexts(context, realm_contexts);
    free_certauth_handles(context, certauth_modules);
    return retval;
}
#[c2rust::src_loc = "1534:1"]
unsafe extern "C" fn pkinit_server_plugin_fini_realm(mut context:
                                                         krb5_context,
                                                     mut plgctx:
                                                         pkinit_kdc_context) {
    let mut sp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    if plgctx.is_null() { return }
    pkinit_fini_kdc_profile(context, plgctx);
    pkinit_fini_identity_opts((*plgctx).idopts);
    pkinit_fini_identity_crypto((*plgctx).idctx);
    pkinit_fini_plg_crypto((*plgctx).cryptoctx);
    pkinit_fini_plg_opts((*plgctx).opts);
    sp = (*plgctx).auth_indicators;
    while !sp.is_null() && !(*sp).is_null() {
        free(*sp as *mut libc::c_void);
        sp = sp.offset(1)
    }
    free((*plgctx).auth_indicators as *mut libc::c_void);
    free((*plgctx).realmname as *mut libc::c_void);
    free(plgctx as *mut libc::c_void);
}
#[c2rust::src_loc = "1554:1"]
unsafe extern "C" fn pkinit_server_plugin_fini(mut context: krb5_context,
                                               mut moddata:
                                                   krb5_kdcpreauth_moddata) {
    if moddata.is_null() { return }
    free_realm_contexts(context, (*moddata).realm_contexts);
    free_certauth_handles(context, (*moddata).certauth_modules);
    free(moddata as *mut libc::c_void);
}
#[c2rust::src_loc = "1565:1"]
unsafe extern "C" fn pkinit_init_kdc_req_context(mut context: krb5_context,
                                                 mut ctx:
                                                     *mut pkinit_kdc_req_context)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut reqctx: pkinit_kdc_req_context = 0 as pkinit_kdc_req_context;
    reqctx =
        malloc(::std::mem::size_of::<_pkinit_kdc_req_context>() as
                   libc::c_ulong) as pkinit_kdc_req_context;
    if reqctx.is_null() { return retval }
    memset(reqctx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_pkinit_kdc_req_context>() as libc::c_ulong);
    (*reqctx).magic = 0x5551212 as libc::c_int;
    retval = pkinit_init_req_crypto(&mut (*reqctx).cryptoctx);
    if !(retval != 0) {
        (*reqctx).rcv_auth_pack = 0 as *mut krb5_auth_pack;
        pkiDebug(b"%s: returning reqctx at %p\n\x00" as *const u8 as
                     *const libc::c_char,
                 (*::std::mem::transmute::<&[u8; 28],
                                           &[libc::c_char; 28]>(b"pkinit_init_kdc_req_context\x00")).as_ptr(),
                 reqctx);
        *ctx = reqctx;
        retval = 0 as libc::c_int
    }
    if retval != 0 {
        pkinit_fini_kdc_req_context(context, reqctx as *mut libc::c_void);
    }
    return retval;
}
#[c2rust::src_loc = "1592:1"]
unsafe extern "C" fn pkinit_fini_kdc_req_context(mut context: krb5_context,
                                                 mut ctx: *mut libc::c_void) {
    let mut reqctx: pkinit_kdc_req_context = ctx as pkinit_kdc_req_context;
    if reqctx.is_null() || (*reqctx).magic != 0x5551212 as libc::c_int {
        pkiDebug(b"pkinit_fini_kdc_req_context: got bad reqctx (%p)!\n\x00" as
                     *const u8 as *const libc::c_char, reqctx);
        return
    }
    pkiDebug(b"%s: freeing reqctx at %p\n\x00" as *const u8 as
                 *const libc::c_char,
             (*::std::mem::transmute::<&[u8; 28],
                                       &[libc::c_char; 28]>(b"pkinit_fini_kdc_req_context\x00")).as_ptr(),
             reqctx);
    pkinit_fini_req_crypto((*reqctx).cryptoctx);
    if !(*reqctx).rcv_auth_pack.is_null() {
        free_krb5_auth_pack(&mut (*reqctx).rcv_auth_pack);
    }
    free(reqctx as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1614:1"]
pub unsafe extern "C" fn kdcpreauth_pkinit_initvt(mut context: krb5_context,
                                                  mut maj_ver: libc::c_int,
                                                  mut min_ver: libc::c_int,
                                                  mut vtable:
                                                      krb5_plugin_vtable)
 -> krb5_error_code {
    let mut vt: krb5_kdcpreauth_vtable = 0 as *mut krb5_kdcpreauth_vtable_st;
    if maj_ver != 1 as libc::c_int {
        return -(1750600192 as libc::c_long) as krb5_error_code
    }
    vt = vtable as krb5_kdcpreauth_vtable;
    (*vt).name =
        b"pkinit\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*vt).pa_type_list = supported_server_pa_types.as_mut_ptr();
    (*vt).init =
        Some(pkinit_server_plugin_init as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: *mut krb5_kdcpreauth_moddata,
                                      _: *mut *const libc::c_char)
                     -> libc::c_int);
    (*vt).fini =
        Some(pkinit_server_plugin_fini as
                 unsafe extern "C" fn(_: krb5_context,
                                      _: krb5_kdcpreauth_moddata) -> ());
    (*vt).flags =
        Some(pkinit_server_get_flags as
                 unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                     -> libc::c_int);
    (*vt).edata =
        Some(pkinit_server_get_edata as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_kdc_req,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_preauthtype,
                                      _: krb5_kdcpreauth_edata_respond_fn,
                                      _: *mut libc::c_void) -> ());
    (*vt).verify =
        Some(pkinit_server_verify_padata as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_data,
                                      _: *mut krb5_kdc_req,
                                      _: *mut krb5_enc_tkt_part,
                                      _: *mut krb5_pa_data,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_kdcpreauth_verify_respond_fn,
                                      _: *mut libc::c_void) -> ());
    (*vt).return_padata =
        Some(pkinit_server_return_padata as
                 unsafe extern "C" fn(_: krb5_context, _: *mut krb5_pa_data,
                                      _: *mut krb5_data, _: *mut krb5_kdc_req,
                                      _: *mut krb5_kdc_rep,
                                      _: *mut krb5_keyblock,
                                      _: *mut *mut krb5_pa_data,
                                      _: krb5_kdcpreauth_callbacks,
                                      _: krb5_kdcpreauth_rock,
                                      _: krb5_kdcpreauth_moddata,
                                      _: krb5_kdcpreauth_modreq)
                     -> krb5_error_code);
    return 0 as libc::c_int;
}
