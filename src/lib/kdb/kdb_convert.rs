use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:13"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
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
#[c2rust::header_src = "/usr/include/sys/types.h:13"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    use super::types_h::__u_int;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:13"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:13"]
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
    use super::kdb_log_h::_kdb_log_context;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:20"]
pub mod kdb_log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t};
    /* Log header magic # */
    /* Kerberos database version no. */
    /* # of updates in log */
    /* Timestamp of first update */
    /* Timestamp of last update */
    /* First serial # in the update log */
    /* Last serial # in the update log */
    /* State of update log */
    /* Block size of each element */
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:18"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:9"]
    pub struct utf8str_t {
        pub utf8str_t_len: u_int,
        pub utf8str_t_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct kdbe_key_t {
        pub k_ver: int32_t,
        pub k_kvno: int32_t,
        pub k_enctype: C2RustUnnamed_0,
        pub k_contents: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_0 {
        pub k_enctype_len: u_int,
        pub k_enctype_val: *mut int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct kdbe_data_t {
        pub k_magic: int32_t,
        pub k_data: utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct kdbe_princ_t {
        pub k_realm: utf8str_t,
        pub k_components: C2RustUnnamed_1,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_1 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_2 {
        pub tl_data_len: u_int,
        pub tl_data_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:9"]
    pub struct kdbe_pw_hist_t {
        pub kdbe_pw_hist_t_len: u_int,
        pub kdbe_pw_hist_t_val: *mut kdbe_key_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type kdbe_attr_type_t = libc::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const AT_PW_HIST: kdbe_attr_type_t = 19;
    #[c2rust::src_loc = "93:2"]
    pub const AT_PW_HIST_KVNO: kdbe_attr_type_t = 18;
    #[c2rust::src_loc = "92:2"]
    pub const AT_PW_POLICY_SWITCH: kdbe_attr_type_t = 17;
    #[c2rust::src_loc = "91:2"]
    pub const AT_PW_POLICY: kdbe_attr_type_t = 16;
    #[c2rust::src_loc = "90:2"]
    pub const AT_PW_LAST_CHANGE: kdbe_attr_type_t = 15;
    #[c2rust::src_loc = "89:2"]
    pub const AT_MOD_WHERE: kdbe_attr_type_t = 14;
    #[c2rust::src_loc = "88:2"]
    pub const AT_MOD_TIME: kdbe_attr_type_t = 13;
    #[c2rust::src_loc = "87:2"]
    pub const AT_MOD_PRINC: kdbe_attr_type_t = 12;
    #[c2rust::src_loc = "86:2"]
    pub const AT_LEN: kdbe_attr_type_t = 11;
    #[c2rust::src_loc = "85:2"]
    pub const AT_TL_DATA: kdbe_attr_type_t = 10;
    #[c2rust::src_loc = "84:2"]
    pub const AT_KEYDATA: kdbe_attr_type_t = 9;
    #[c2rust::src_loc = "83:2"]
    pub const AT_PRINC: kdbe_attr_type_t = 8;
    #[c2rust::src_loc = "82:2"]
    pub const AT_FAIL_AUTH_COUNT: kdbe_attr_type_t = 7;
    #[c2rust::src_loc = "81:2"]
    pub const AT_LAST_FAILED: kdbe_attr_type_t = 6;
    #[c2rust::src_loc = "80:2"]
    pub const AT_LAST_SUCCESS: kdbe_attr_type_t = 5;
    #[c2rust::src_loc = "79:2"]
    pub const AT_PW_EXP: kdbe_attr_type_t = 4;
    #[c2rust::src_loc = "78:2"]
    pub const AT_EXP: kdbe_attr_type_t = 3;
    #[c2rust::src_loc = "77:2"]
    pub const AT_MAX_RENEW_LIFE: kdbe_attr_type_t = 2;
    #[c2rust::src_loc = "76:2"]
    pub const AT_MAX_LIFE: kdbe_attr_type_t = 1;
    #[c2rust::src_loc = "75:2"]
    pub const AT_ATTRFLAGS: kdbe_attr_type_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct kdbe_val_t {
        pub av_type: kdbe_attr_type_t,
        pub kdbe_val_t_u: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_3 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_7,
        pub av_tldata: C2RustUnnamed_6,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_5,
        pub av_extension: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_4 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_5 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_6 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_7 {
        pub av_keydata_len: u_int,
        pub av_keydata_val: *mut kdbe_key_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:9"]
    pub struct kdbe_t {
        pub kdbe_t_len: u_int,
        pub kdbe_t_val: *mut kdbe_val_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:8"]
    pub struct kdb_incr_update_t {
        pub kdb_princ_name: utf8str_t,
        pub kdb_entry_sno: kdb_sno_t,
        pub kdb_time: kdbe_time_t,
        pub kdb_update: kdbe_t,
        pub kdb_deleted: libc::c_int,
        pub kdb_commit: libc::c_int,
        pub kdb_kdcs_seen_by: C2RustUnnamed_9,
        pub kdb_futures: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_8 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_9 {
        pub kdb_kdcs_seen_by_len: u_int,
        pub kdb_kdcs_seen_by_val: *mut utf8str_t,
    }
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::u_int;
    use super::stdint_intn_h::{int32_t, int16_t};
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:17"]
pub mod iprop_hdr_h {
    /* Backoff for a maximum for 5 mts */
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:13"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:13"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:19"]
pub mod kdb_h {
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
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    /* NOT saved */
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
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
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
        #[c2rust::src_loc = "492:1"]
        pub fn krb5_dbe_lookup_mod_princ_data(context: krb5_context,
                                              entry: *mut krb5_db_entry,
                                              mod_time: *mut krb5_timestamp,
                                              mod_princ: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn krb5_dbe_update_last_pwd_change(context: krb5_context,
                                               entry: *mut krb5_db_entry,
                                               stamp: krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "542:1"]
        pub fn krb5_dbe_update_mod_princ_data(context: krb5_context,
                                              entry: *mut krb5_db_entry,
                                              mod_date: krb5_timestamp,
                                              mod_princ: krb5_const_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn krb5_dbe_lookup_last_pwd_change(context: krb5_context,
                                               entry: *mut krb5_db_entry,
                                               stamp: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "602:1"]
        pub fn krb5_dbe_update_tl_data(context: krb5_context,
                                       entry: *mut krb5_db_entry,
                                       new_tl_data: *mut krb5_tl_data)
         -> krb5_error_code;
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
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/stdlib.h:13"]
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
#[c2rust::header_src = "/usr/include/string.h:13"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
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
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::sys_types_h::u_int;
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
                         _krb5_os_context, plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t, utf8str_t, kdbe_key_t,
                        C2RustUnnamed, C2RustUnnamed_0, kdbe_data_t,
                        kdbe_princ_t, C2RustUnnamed_1, kdbe_tl_t,
                        C2RustUnnamed_2, kdbe_pw_hist_t, kdbe_attr_type_t,
                        AT_PW_HIST, AT_PW_HIST_KVNO, AT_PW_POLICY_SWITCH,
                        AT_PW_POLICY, AT_PW_LAST_CHANGE, AT_MOD_WHERE,
                        AT_MOD_TIME, AT_MOD_PRINC, AT_LEN, AT_TL_DATA,
                        AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS,
                        kdbe_val_t, C2RustUnnamed_3, C2RustUnnamed_4,
                        C2RustUnnamed_5, C2RustUnnamed_6, C2RustUnnamed_7,
                        kdbe_t, kdb_incr_update_t, C2RustUnnamed_8,
                        C2RustUnnamed_9};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data, _krb5_db_entry_new, krb5_db_entry,
                      krb5_db_get_principal, krb5_db_free_principal,
                      krb5_dbe_lookup_mod_princ_data,
                      krb5_dbe_update_last_pwd_change,
                      krb5_dbe_update_mod_princ_data,
                      krb5_dbe_lookup_last_pwd_change,
                      krb5_dbe_update_tl_data};
use self::stdlib_h::{free, realloc, calloc, malloc};
use self::string_h::{strncmp, strncpy, memcmp, memset, memcpy};
#[c2rust::src_loc = "34:9"]
pub type princ_type = libc::c_uint;
#[c2rust::src_loc = "36:5"]
pub const MOD_PRINC: princ_type = 1;
#[c2rust::src_loc = "35:5"]
pub const REG_PRINC: princ_type = 0;
/*
 * This routine tracks the krb5_db_entry fields that have been modified
 * (by comparing it to the db_entry currently present in principal.db)
 * in the update.
 */
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn find_changed_attrs(mut current: *mut krb5_db_entry,
                                        mut new: *mut krb5_db_entry,
                                        mut exclude_nra: krb5_boolean,
                                        mut attrs: *mut kdbe_attr_type_t,
                                        mut nattrs: *mut libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut first: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut second: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    if (*current).attributes != (*new).attributes {
        let fresh0 = i;
        i = i + 1;
        *attrs.offset(fresh0 as isize) = AT_ATTRFLAGS
    }
    if (*current).max_life != (*new).max_life {
        let fresh1 = i;
        i = i + 1;
        *attrs.offset(fresh1 as isize) = AT_MAX_LIFE
    }
    if (*current).max_renewable_life != (*new).max_renewable_life {
        let fresh2 = i;
        i = i + 1;
        *attrs.offset(fresh2 as isize) = AT_MAX_RENEW_LIFE
    }
    if (*current).expiration != (*new).expiration {
        let fresh3 = i;
        i = i + 1;
        *attrs.offset(fresh3 as isize) = AT_EXP
    }
    if (*current).pw_expiration != (*new).pw_expiration {
        let fresh4 = i;
        i = i + 1;
        *attrs.offset(fresh4 as isize) = AT_PW_EXP
    }
    if exclude_nra == 0 {
        if (*current).last_success != (*new).last_success {
            let fresh5 = i;
            i = i + 1;
            *attrs.offset(fresh5 as isize) = AT_LAST_SUCCESS
        }
        if (*current).last_failed != (*new).last_failed {
            let fresh6 = i;
            i = i + 1;
            *attrs.offset(fresh6 as isize) = AT_LAST_FAILED
        }
        if (*current).fail_auth_count != (*new).fail_auth_count {
            let fresh7 = i;
            i = i + 1;
            *attrs.offset(fresh7 as isize) = AT_FAIL_AUTH_COUNT
        }
    }
    if (*(*current).princ).type_0 == (*(*new).princ).type_0 &&
           (*(*current).princ).length == (*(*new).princ).length {
        if (*(*current).princ).realm.length == (*(*new).princ).realm.length &&
               strncmp((*(*current).princ).realm.data,
                       (*(*new).princ).realm.data,
                       (*(*current).princ).realm.length as libc::c_ulong) != 0
           {
            j = 0 as libc::c_int;
            while j < (*(*current).princ).length {
                if !(*(*(*current).princ).data.offset(j as
                                                          isize)).data.is_null()
                       &&
                       strncmp((*(*(*current).princ).data.offset(j as
                                                                     isize)).data,
                               (*(*(*new).princ).data.offset(j as
                                                                 isize)).data,
                               (*(*(*current).princ).data.offset(j as
                                                                     isize)).length
                                   as libc::c_ulong) != 0 {
                    let fresh8 = i;
                    i = i + 1;
                    *attrs.offset(fresh8 as isize) = AT_PRINC;
                    break ;
                } else { j += 1 }
            }
        } else {
            let fresh9 = i;
            i = i + 1;
            *attrs.offset(fresh9 as isize) = AT_PRINC
        }
    } else {
        let fresh10 = i;
        i = i + 1;
        *attrs.offset(fresh10 as isize) = AT_PRINC
    }
    if (*current).n_key_data as libc::c_int ==
           (*new).n_key_data as libc::c_int {
        /* Assuming key ordering is the same in new & current */
        j = 0 as libc::c_int;
        while j < (*new).n_key_data as libc::c_int {
            if (*(*current).key_data.offset(j as isize)).key_data_kvno as
                   libc::c_int !=
                   (*(*new).key_data.offset(j as isize)).key_data_kvno as
                       libc::c_int {
                let fresh11 = i;
                i = i + 1;
                *attrs.offset(fresh11 as isize) = AT_KEYDATA;
                break ;
            } else { j += 1 }
        }
    } else {
        let fresh12 = i;
        i = i + 1;
        *attrs.offset(fresh12 as isize) = AT_KEYDATA
    }
    if (*current).n_tl_data as libc::c_int == (*new).n_tl_data as libc::c_int
       {
        /* Assuming we preserve the TL_DATA ordering between updates */
        first = (*current).tl_data;
        second = (*new).tl_data;
        while !first.is_null() {
            if (*first).tl_data_length as libc::c_int ==
                   (*second).tl_data_length as libc::c_int &&
                   (*first).tl_data_type as libc::c_int ==
                       (*second).tl_data_type as libc::c_int {
                if memcmp((*first).tl_data_contents as *mut libc::c_char as
                              *const libc::c_void,
                          (*second).tl_data_contents as *mut libc::c_char as
                              *const libc::c_void,
                          (*first).tl_data_length as libc::c_ulong) !=
                       0 as libc::c_int {
                    let fresh13 = i;
                    i = i + 1;
                    *attrs.offset(fresh13 as isize) = AT_TL_DATA;
                    break ;
                } else {
                    first = (*first).tl_data_next;
                    second = (*second).tl_data_next
                }
            } else {
                let fresh14 = i;
                i = i + 1;
                *attrs.offset(fresh14 as isize) = AT_TL_DATA;
                break ;
            }
        }
    } else {
        let fresh15 = i;
        i = i + 1;
        *attrs.offset(fresh15 as isize) = AT_TL_DATA
    }
    if (*current).len as libc::c_int != (*new).len as libc::c_int {
        let fresh16 = i;
        i = i + 1;
        *attrs.offset(fresh16 as isize) = AT_LEN
    }
    /*
     * Store the no. of (possibly :)) changed attributes
     */
    *nattrs = i;
}
/* Initialize *u with a copy of d.  Return 0 on success, -1 on failure. */
#[c2rust::src_loc = "148:1"]
unsafe extern "C" fn data_to_utf8str(mut u: *mut utf8str_t, mut d: krb5_data)
 -> libc::c_int {
    (*u).utf8str_t_len = d.length;
    if !d.data.is_null() {
        (*u).utf8str_t_val =
            malloc(d.length as libc::c_ulong) as *mut libc::c_char;
        if (*u).utf8str_t_val.is_null() { return -(1 as libc::c_int) }
        memcpy((*u).utf8str_t_val as *mut libc::c_void,
               d.data as *const libc::c_void, d.length as libc::c_ulong);
    } else { (*u).utf8str_t_val = 0 as *mut libc::c_char }
    return 0 as libc::c_int;
}
/*
 * Converts the krb5_principal struct from db2 to ulog format.
 */
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn conv_princ_2ulog(mut princ: krb5_principal,
                                      mut upd: *mut kdb_incr_update_t,
                                      mut cnt: libc::c_int,
                                      mut tp: princ_type) -> krb5_error_code {
    let mut i: libc::c_int = 0 as libc::c_int; /* or av_mod_princ */
    let mut p: *mut kdbe_princ_t = 0 as *mut kdbe_princ_t;
    let mut components: *mut kdbe_data_t = 0 as *mut kdbe_data_t;
    if upd.is_null() || princ.is_null() {
        return -(1765328324 as libc::c_long) as krb5_error_code
    }
    match tp as libc::c_uint {
        0 | 1 => {
            p =
                &mut (*(*upd).kdb_update.kdbe_t_val.offset(cnt as
                                                               isize)).kdbe_val_t_u.av_princ;
            (*p).k_nametype = (*princ).type_0;
            if data_to_utf8str(&mut (*p).k_realm, (*princ).realm) <
                   0 as libc::c_int {
                return 12 as libc::c_int
            }
            (*p).k_components.k_components_len = (*princ).length as u_int;
            components =
                malloc(((*princ).length as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<kdbe_data_t>()
                                                            as libc::c_ulong))
                    as *mut kdbe_data_t;
            (*p).k_components.k_components_val = components;
            if (*p).k_components.k_components_val.is_null() {
                free((*p).k_realm.utf8str_t_val as *mut libc::c_void);
                (*p).k_realm.utf8str_t_val = 0 as *mut libc::c_char;
                return 12 as libc::c_int
            }
            memset(components as *mut libc::c_void, 0 as libc::c_int,
                   ((*princ).length as
                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<kdbe_data_t>()
                                                        as libc::c_ulong));
            i = 0 as libc::c_int;
            while i < (*princ).length {
                let ref mut fresh17 =
                    (*components.offset(i as isize)).k_data.utf8str_t_val;
                *fresh17 = 0 as *mut libc::c_char;
                i += 1
            }
            i = 0 as libc::c_int;
            while i < (*princ).length {
                (*components.offset(i as isize)).k_magic =
                    (*(*princ).data.offset(i as isize)).magic;
                if data_to_utf8str(&mut (*components.offset(i as
                                                                isize)).k_data,
                                   *(*princ).data.offset(i as isize)) <
                       0 as libc::c_int {
                    let mut j: libc::c_int = 0;
                    j = 0 as libc::c_int;
                    while j < i {
                        free((*components.offset(j as
                                                     isize)).k_data.utf8str_t_val
                                 as *mut libc::c_void);
                        let ref mut fresh18 =
                            (*components.offset(j as
                                                    isize)).k_data.utf8str_t_val;
                        *fresh18 = 0 as *mut libc::c_char;
                        j += 1
                    }
                    free(components as *mut libc::c_void);
                    (*p).k_components.k_components_val =
                        0 as *mut kdbe_data_t;
                    free((*p).k_realm.utf8str_t_val as *mut libc::c_void);
                    (*p).k_realm.utf8str_t_val = 0 as *mut libc::c_char;
                    return 12 as libc::c_int
                }
                i += 1
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/*
 * Copies a UTF-8 string from ulog to a krb5_data object, which may
 * already have allocated storage associated with it.
 *
 * Maybe a return value should indicate success/failure?
 */
#[c2rust::src_loc = "228:1"]
unsafe extern "C" fn set_from_utf8str(mut d: *mut krb5_data,
                                      mut u: utf8str_t) {
    if u.utf8str_t_len >
           (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        (*d).data = 0 as *mut libc::c_char;
        return
    }
    (*d).length = u.utf8str_t_len;
    (*d).data =
        malloc((*d).length.wrapping_add(1 as libc::c_int as libc::c_uint) as
                   libc::c_ulong) as *mut libc::c_char;
    if (*d).data.is_null() { return }
    if (*d).length != 0 {
        /* Pointer may be null if length = 0.  */
        strncpy((*d).data, u.utf8str_t_val, (*d).length as libc::c_ulong);
    }
    *(*d).data.offset((*d).length as isize) =
        0 as libc::c_int as libc::c_char;
}
/*
 * Converts the krb5_principal struct from ulog to db2 format.
 */
#[c2rust::src_loc = "247:1"]
unsafe extern "C" fn conv_princ_2db(mut context: krb5_context,
                                    mut kdbe_princ: *mut kdbe_princ_t)
 -> krb5_principal {
    let mut current_block: u64;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut components: *mut kdbe_data_t = 0 as *mut kdbe_data_t;
    princ =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_principal_data>() as libc::c_ulong)
            as krb5_principal;
    if princ.is_null() { return 0 as krb5_principal }
    (*princ).length = 0 as libc::c_int;
    (*princ).data = 0 as *mut krb5_data;
    components = (*kdbe_princ).k_components.k_components_val;
    (*princ).type_0 = (*kdbe_princ).k_nametype;
    (*princ).realm.data = 0 as *mut libc::c_char;
    set_from_utf8str(&mut (*princ).realm, (*kdbe_princ).k_realm);
    if !(*princ).realm.data.is_null() {
        (*princ).data =
            calloc((*kdbe_princ).k_components.k_components_len as
                       libc::c_ulong,
                   ::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
                *mut krb5_data;
        if !(*princ).data.is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*kdbe_princ).k_components.k_components_len {
                let ref mut fresh19 =
                    (*(*princ).data.offset(i as isize)).data;
                *fresh19 = 0 as *mut libc::c_char;
                i = i.wrapping_add(1)
            }
            (*princ).length =
                (*kdbe_princ).k_components.k_components_len as krb5_int32;
            j = 0 as libc::c_int;
            loop  {
                if !(j < (*princ).length) {
                    current_block = 1109700713171191020;
                    break ;
                }
                (*(*princ).data.offset(j as isize)).magic =
                    (*components.offset(j as isize)).k_magic;
                set_from_utf8str(&mut *(*princ).data.offset(j as isize),
                                 (*components.offset(j as isize)).k_data);
                if (*(*princ).data.offset(j as isize)).data.is_null() {
                    current_block = 16938667499150799833;
                    break ;
                }
                j += 1
            }
            match current_block {
                16938667499150799833 => { }
                _ => { return princ }
            }
        }
    }
    krb5_free_principal(context, princ);
    return 0 as krb5_principal;
}
/*
 * This routine converts a krb5 DB record into update log (ulog) entry format.
 * Space for the update log entry should be allocated prior to invocation of
 * this routine.
 */
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn ulog_conv_2logentry(mut context: krb5_context,
                                             mut entry: *mut krb5_db_entry,
                                             mut update:
                                                 *mut kdb_incr_update_t)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut final_0: libc::c_int = 0;
    let mut nattrs: libc::c_int = 0;
    let mut tmpint: libc::c_int = 0;
    let mut tmpprinc: krb5_principal = 0 as *mut krb5_principal_data;
    let mut newtl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut curr: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut ret: krb5_error_code = 0;
    let mut attr_types: *mut kdbe_attr_type_t = 0 as *mut kdbe_attr_type_t;
    let mut kadm_data_yes: libc::c_int = 0;
    tmpint = 0 as libc::c_int;
    nattrs = tmpint;
    final_0 = -(1 as libc::c_int);
    kadm_data_yes = 0 as libc::c_int;
    attr_types = 0 as *mut kdbe_attr_type_t;
    /*
     * XXX we rely on the good behaviour of the database not to
     * exceed this limit.
     */
    (*update).kdb_update.kdbe_t_val =
        malloc(2048 as libc::c_int as libc::c_ulong) as *mut kdbe_val_t;
    if (*update).kdb_update.kdbe_t_val.is_null() { return 12 as libc::c_int }
    /*
     * Find out which attrs have been modified
     */
    attr_types =
        malloc((::std::mem::size_of::<kdbe_attr_type_t>() as
                    libc::c_ulong).wrapping_mul(20 as libc::c_int as
                                                    libc::c_ulong)) as
            *mut kdbe_attr_type_t;
    if attr_types.is_null() { return 12 as libc::c_int }
    ret =
        krb5_db_get_principal(context, (*entry).princ as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint, &mut curr);
    if ret != 0 && ret as libc::c_long != -(1780008443 as libc::c_long) {
        free(attr_types as *mut libc::c_void);
        return ret
    }
    if ret as libc::c_long == -(1780008443 as libc::c_long) {
        /*
         * This is a new entry to the database, hence will
         * include all the attribute-value pairs
         *
         * We leave out the TL_DATA types which we model as
         * attrs in kdbe_attr_type_t, since listing AT_TL_DATA
         * encompasses these other types-turned-attributes
         *
         * So, we do *NOT* consider AT_MOD_PRINC, AT_MOD_TIME,
         * AT_MOD_WHERE, AT_PW_LAST_CHANGE, AT_PW_POLICY,
         * AT_PW_POLICY_SWITCH, AT_PW_HIST_KVNO and AT_PW_HIST,
         * totalling 8 attrs.
         */
        while nattrs < 20 as libc::c_int - 8 as libc::c_int {
            *attr_types.offset(nattrs as isize) = nattrs as kdbe_attr_type_t;
            nattrs += 1
        }
    } else {
        /* Always exclude non-replicated attributes for now. */
        find_changed_attrs(curr, entry, 1 as libc::c_int as krb5_boolean,
                           attr_types, &mut nattrs);
        krb5_db_free_principal(context, curr);
    }
    i = 0 as libc::c_int;
    while i < nattrs {
        match *attr_types.offset(i as isize) as libc::c_uint {
            0 => {
                if (*entry).attributes >= 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_ATTRFLAGS;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_attrflags
                        = (*entry).attributes as uint32_t
                }
            }
            1 => {
                if (*entry).max_life >= 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_MAX_LIFE;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_max_life
                        = (*entry).max_life as uint32_t
                }
            }
            2 => {
                if (*entry).max_renewable_life >= 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_MAX_RENEW_LIFE;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_max_renew_life
                        = (*entry).max_renewable_life as uint32_t
                }
            }
            3 => {
                if (*entry).expiration >= 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_EXP;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_exp
                        = (*entry).expiration as uint32_t
                }
            }
            4 => {
                if (*entry).pw_expiration >= 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_PW_EXP;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_pw_exp
                        = (*entry).pw_expiration as uint32_t
                }
            }
            8 => {
                if (*(*entry).princ).length > 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_PRINC;
                    ret =
                        conv_princ_2ulog((*entry).princ, update, final_0,
                                         REG_PRINC);
                    if ret != 0 {
                        free(attr_types as *mut libc::c_void);
                        return ret
                    }
                }
            }
            9 => {
                /* BEGIN CSTYLED */
                if (*entry).n_key_data as libc::c_int >= 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_KEYDATA;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_keydata.av_keydata_len
                        = (*entry).n_key_data as u_int;
                    let ref mut fresh20 =
                        (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                     isize)).kdbe_val_t_u.av_keydata.av_keydata_val;
                    *fresh20 =
                        malloc(((*entry).n_key_data as
                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<kdbe_key_t>()
                                                                    as
                                                                    libc::c_ulong))
                            as *mut kdbe_key_t;
                    if (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                    isize)).kdbe_val_t_u.av_keydata.av_keydata_val.is_null()
                       {
                        free(attr_types as *mut libc::c_void);
                        return 12 as libc::c_int
                    }
                    j = 0 as libc::c_int;
                    while j < (*entry).n_key_data as libc::c_int {
                        (*(*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                       isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                 as
                                                                                                                                 isize)).k_ver
                            =
                            (*(*entry).key_data.offset(j as
                                                           isize)).key_data_ver
                                as int32_t;
                        (*(*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                       isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                 as
                                                                                                                                 isize)).k_kvno
                            =
                            (*(*entry).key_data.offset(j as
                                                           isize)).key_data_kvno
                                as int32_t;
                        (*(*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                       isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                 as
                                                                                                                                 isize)).k_enctype.k_enctype_len
                            =
                            (*(*entry).key_data.offset(j as
                                                           isize)).key_data_ver
                                as u_int;
                        (*(*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                       isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                 as
                                                                                                                                 isize)).k_contents.k_contents_len
                            =
                            (*(*entry).key_data.offset(j as
                                                           isize)).key_data_ver
                                as u_int;
                        let ref mut fresh21 =
                            (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                           as
                                                                           isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                     as
                                                                                                                                     isize)).k_enctype.k_enctype_val;
                        *fresh21 =
                            malloc(((*(*entry).key_data.offset(j as
                                                                   isize)).key_data_ver
                                        as
                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>()
                                                                        as
                                                                        libc::c_ulong))
                                as *mut int32_t;
                        if (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                          as
                                                                          isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                    as
                                                                                                                                    isize)).k_enctype.k_enctype_val.is_null()
                           {
                            free(attr_types as *mut libc::c_void);
                            return 12 as libc::c_int
                        }
                        let ref mut fresh22 =
                            (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                           as
                                                                           isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                     as
                                                                                                                                     isize)).k_contents.k_contents_val;
                        *fresh22 =
                            malloc(((*(*entry).key_data.offset(j as
                                                                   isize)).key_data_ver
                                        as
                                        libc::c_ulong).wrapping_mul(::std::mem::size_of::<utf8str_t>()
                                                                        as
                                                                        libc::c_ulong))
                                as *mut utf8str_t;
                        if (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                          as
                                                                          isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                    as
                                                                                                                                    isize)).k_contents.k_contents_val.is_null()
                           {
                            free(attr_types as *mut libc::c_void);
                            return 12 as libc::c_int
                        }
                        cnt = 0 as libc::c_int;
                        while cnt <
                                  (*(*entry).key_data.offset(j as
                                                                 isize)).key_data_ver
                                      as libc::c_int {
                            *(*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                            as
                                                                            isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                      as
                                                                                                                                      isize)).k_enctype.k_enctype_val.offset(cnt
                                                                                                                                                                                 as
                                                                                                                                                                                 isize)
                                =
                                (*(*entry).key_data.offset(j as
                                                               isize)).key_data_type[cnt
                                                                                         as
                                                                                         usize]
                                    as int32_t;
                            (*(*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                             as
                                                                             isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                       as
                                                                                                                                       isize)).k_contents.k_contents_val.offset(cnt
                                                                                                                                                                                    as
                                                                                                                                                                                    isize)).utf8str_t_len
                                =
                                (*(*entry).key_data.offset(j as
                                                               isize)).key_data_length[cnt
                                                                                           as
                                                                                           usize]
                                    as u_int;
                            let ref mut fresh23 =
                                (*(*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                                 as
                                                                                 isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                           as
                                                                                                                                           isize)).k_contents.k_contents_val.offset(cnt
                                                                                                                                                                                        as
                                                                                                                                                                                        isize)).utf8str_t_val;
                            *fresh23 =
                                malloc(((*(*entry).key_data.offset(j as
                                                                       isize)).key_data_length[cnt
                                                                                                   as
                                                                                                   usize]
                                            as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                            as
                                                                            libc::c_ulong))
                                    as *mut libc::c_char;
                            if (*(*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                                as
                                                                                isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                          as
                                                                                                                                          isize)).k_contents.k_contents_val.offset(cnt
                                                                                                                                                                                       as
                                                                                                                                                                                       isize)).utf8str_t_val.is_null()
                               {
                                free(attr_types as *mut libc::c_void);
                                return 12 as libc::c_int
                            }
                            memcpy((*(*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                                    as
                                                                                    isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                              as
                                                                                                                                              isize)).k_contents.k_contents_val.offset(cnt
                                                                                                                                                                                           as
                                                                                                                                                                                           isize)).utf8str_t_val
                                       as *mut libc::c_void,
                                   (*(*entry).key_data.offset(j as
                                                                  isize)).key_data_contents[cnt
                                                                                                as
                                                                                                usize]
                                       as *const libc::c_void,
                                   (*(*entry).key_data.offset(j as
                                                                  isize)).key_data_length[cnt
                                                                                              as
                                                                                              usize]
                                       as libc::c_ulong);
                            cnt += 1
                        }
                        j += 1
                    }
                }
            }
            10 => {
                ret =
                    krb5_dbe_lookup_last_pwd_change(context, entry,
                                                    &mut tmpint);
                if ret == 0 as libc::c_int {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_PW_LAST_CHANGE;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_pw_last_change
                        = tmpint as uint32_t
                }
                tmpint = 0 as libc::c_int;
                ret =
                    krb5_dbe_lookup_mod_princ_data(context, entry,
                                                   &mut tmpint,
                                                   &mut tmpprinc);
                if ret == 0 {
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_MOD_PRINC;
                    ret =
                        conv_princ_2ulog(tmpprinc, update, final_0,
                                         MOD_PRINC);
                    krb5_free_principal(context, tmpprinc);
                    if ret != 0 {
                        free(attr_types as *mut libc::c_void);
                        return ret
                    }
                    final_0 += 1;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).av_type
                        = AT_MOD_TIME;
                    (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                                 isize)).kdbe_val_t_u.av_mod_time
                        = tmpint as uint32_t
                }
                newtl = (*entry).tl_data;
                while !newtl.is_null() {
                    match (*newtl).tl_data_type as libc::c_int {
                        1 | 2 => { }
                        3 | _ => {
                            if kadm_data_yes == 0 as libc::c_int {
                                final_0 += 1;
                                (*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                             as
                                                                             isize)).av_type
                                    = AT_TL_DATA;
                                (*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                             as
                                                                             isize)).kdbe_val_t_u.av_tldata.av_tldata_len
                                    = 0 as libc::c_int as u_int;
                                let ref mut fresh24 =
                                    (*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                                 as
                                                                                 isize)).kdbe_val_t_u.av_tldata.av_tldata_val;
                                *fresh24 =
                                    malloc(((*entry).n_tl_data as
                                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<kdbe_tl_t>()
                                                                                as
                                                                                libc::c_ulong))
                                        as *mut kdbe_tl_t;
                                if (*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                                as
                                                                                isize)).kdbe_val_t_u.av_tldata.av_tldata_val.is_null()
                                   {
                                    free(attr_types as *mut libc::c_void);
                                    return 12 as libc::c_int
                                }
                                kadm_data_yes = 1 as libc::c_int
                            }
                            tmpint =
                                (*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                             as
                                                                             isize)).kdbe_val_t_u.av_tldata.av_tldata_len
                                    as libc::c_int;
                            let ref mut fresh25 =
                                (*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                             as
                                                                             isize)).kdbe_val_t_u.av_tldata.av_tldata_len;
                            *fresh25 = (*fresh25).wrapping_add(1);
                            (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                           as
                                                                           isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(tmpint
                                                                                                                                   as
                                                                                                                                   isize)).tl_type
                                = (*newtl).tl_data_type;
                            (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                           as
                                                                           isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(tmpint
                                                                                                                                   as
                                                                                                                                   isize)).tl_data.tl_data_len
                                = (*newtl).tl_data_length as u_int;
                            let ref mut fresh26 =
                                (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                               as
                                                                               isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(tmpint
                                                                                                                                       as
                                                                                                                                       isize)).tl_data.tl_data_val;
                            *fresh26 =
                                malloc(((*newtl).tl_data_length as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                            as
                                                                            libc::c_ulong))
                                    as *mut libc::c_char;
                            if (*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                              as
                                                                              isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(tmpint
                                                                                                                                      as
                                                                                                                                      isize)).tl_data.tl_data_val.is_null()
                               {
                                free(attr_types as *mut libc::c_void);
                                return 12 as libc::c_int
                            }
                            memcpy((*(*(*update).kdb_update.kdbe_t_val.offset(final_0
                                                                                  as
                                                                                  isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(tmpint
                                                                                                                                          as
                                                                                                                                          isize)).tl_data.tl_data_val
                                       as *mut libc::c_void,
                                   (*newtl).tl_data_contents as
                                       *const libc::c_void,
                                   (*newtl).tl_data_length as libc::c_ulong);
                        }
                    }
                    newtl = (*newtl).tl_data_next
                }
            }
            11 => {
                /* END CSTYLED */
                final_0 += 1;
                (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                             isize)).av_type =
                    AT_LEN;
                (*(*update).kdb_update.kdbe_t_val.offset(final_0 as
                                                             isize)).kdbe_val_t_u.av_len
                    = (*entry).len as int16_t
            }
            _ => { }
        }
        i += 1
    }
    free(attr_types as *mut libc::c_void);
    /*
     * Update len field in kdb_update
     */
    final_0 += 1;
    (*update).kdb_update.kdbe_t_len = final_0 as u_int;
    return 0 as libc::c_int;
}
/* Convert an update log (ulog) entry into a kerberos record. */
#[no_mangle]
#[c2rust::src_loc = "549:1"]
pub unsafe extern "C" fn ulog_conv_2dbentry(mut context: krb5_context,
                                            mut entry:
                                                *mut *mut krb5_db_entry,
                                            mut update:
                                                *mut kdb_incr_update_t)
 -> krb5_error_code {
    let mut ent: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut replica: libc::c_int = 0;
    let mut mod_princ: krb5_principal = 0 as krb5_principal;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut mod_time: libc::c_int = 0 as libc::c_int;
    let mut nattrs: libc::c_int = 0;
    let mut dbprinc: krb5_principal = 0 as *mut krb5_principal_data;
    let mut dbprincstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newtl: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut ret: krb5_error_code = 0;
    let mut prev_n_keys: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut is_add: krb5_boolean = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    *entry = 0 as *mut krb5_db_entry;
    replica =
        (!(*context).kdblog_context.is_null() &&
             (*(*context).kdblog_context).iproprole as libc::c_uint ==
                 IPROP_REPLICA as libc::c_int as libc::c_uint) as libc::c_int;
    /*
     * Store the no. of changed attributes in nattrs
     */
    nattrs = (*update).kdb_update.kdbe_t_len as libc::c_int;
    dbprincstr =
        malloc(((*update).kdb_princ_name.utf8str_t_len.wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                    as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut libc::c_char;
    if dbprincstr.is_null() { return 12 as libc::c_int }
    strncpy(dbprincstr, (*update).kdb_princ_name.utf8str_t_val,
            (*update).kdb_princ_name.utf8str_t_len as libc::c_ulong);
    *dbprincstr.offset((*update).kdb_princ_name.utf8str_t_len as isize) =
        0 as libc::c_int as libc::c_char;
    ret = krb5_parse_name(context, dbprincstr, &mut dbprinc);
    free(dbprincstr as *mut libc::c_void);
    if ret != 0 { return ret }
    ret =
        krb5_db_get_principal(context, dbprinc as krb5_const_principal,
                              0 as libc::c_int as libc::c_uint, &mut ent);
    krb5_free_principal(context, dbprinc);
    if ret != 0 && ret as libc::c_long != -(1780008443 as libc::c_long) {
        return ret
    }
    is_add =
        (ret as libc::c_long == -(1780008443 as libc::c_long)) as libc::c_int
            as krb5_boolean;
    /*
     * Set ent->n_tl_data = 0 initially, if this is an ADD update
     */
    if is_add != 0 {
        ent =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_db_entry>() as libc::c_ulong) as
                *mut krb5_db_entry;
        if ent.is_null() { return 12 as libc::c_int }
        (*ent).n_tl_data = 0 as libc::c_int as krb5_int16
    }
    i = 0 as libc::c_int;
    while i < nattrs {
        let mut tmpprinc: krb5_principal = 0 as krb5_principal;
        match (*(*update).kdb_update.kdbe_t_val.offset(i as isize)).av_type as
                  libc::c_uint {
            0 => {
                (*ent).attributes =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_attrflags
                        as krb5_flags
            }
            1 => {
                (*ent).max_life =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_max_life
                        as krb5_deltat
            }
            2 => {
                (*ent).max_renewable_life =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_max_renew_life
                        as krb5_deltat
            }
            3 => {
                (*ent).expiration =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_exp
                        as krb5_timestamp
            }
            4 => {
                (*ent).pw_expiration =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_pw_exp
                        as krb5_timestamp
            }
            5 => {
                if replica == 0 {
                    (*ent).last_success =
                        (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                     isize)).kdbe_val_t_u.av_last_success
                            as krb5_timestamp
                }
            }
            6 => {
                if replica == 0 {
                    (*ent).last_failed =
                        (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                     isize)).kdbe_val_t_u.av_last_failed
                            as krb5_timestamp
                }
            }
            7 => {
                if replica == 0 {
                    (*ent).fail_auth_count =
                        (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                     isize)).kdbe_val_t_u.av_fail_auth_count
                }
            }
            8 => {
                tmpprinc =
                    conv_princ_2db(context,
                                   &mut (*(*update).kdb_update.kdbe_t_val.offset(i
                                                                                     as
                                                                                     isize)).kdbe_val_t_u.av_princ);
                if tmpprinc.is_null() { return 12 as libc::c_int }
                krb5_free_principal(context, (*ent).princ);
                (*ent).princ = tmpprinc
            }
            9 => {
                if is_add == 0 {
                    prev_n_keys = (*ent).n_key_data as libc::c_uint
                } else { prev_n_keys = 0 as libc::c_int as libc::c_uint }
                (*ent).n_key_data =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_keydata.av_keydata_len
                        as krb5_int16;
                if is_add != 0 { (*ent).key_data = 0 as *mut krb5_key_data }
                /* Allocate one extra key data to avoid allocating zero bytes. */
                newptr =
                    realloc((*ent).key_data as *mut libc::c_void,
                            (((*ent).n_key_data as libc::c_int +
                                  1 as libc::c_int) as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_key_data>()
                                                                 as
                                                                 libc::c_ulong));
                if newptr.is_null() { return 12 as libc::c_int }
                (*ent).key_data = newptr as *mut krb5_key_data;
                /* BEGIN CSTYLED */
                j = prev_n_keys as libc::c_int;
                while j < (*ent).n_key_data as libc::c_int {
                    cnt = 0 as libc::c_int;
                    while cnt < 2 as libc::c_int {
                        let ref mut fresh27 =
                            (*(*ent).key_data.offset(j as
                                                         isize)).key_data_contents[cnt
                                                                                       as
                                                                                       usize];
                        *fresh27 = 0 as *mut krb5_octet;
                        cnt += 1
                    }
                    j += 1
                }
                j = 0 as libc::c_int;
                while j < (*ent).n_key_data as libc::c_int {
                    let mut kp: *mut krb5_key_data =
                        &mut *(*ent).key_data.offset(j as isize) as
                            *mut krb5_key_data;
                    let mut kv: *mut kdbe_key_t =
                        &mut *(*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                           isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                     as
                                                                                                                                     isize)
                            as *mut kdbe_key_t;
                    (*kp).key_data_ver = (*kv).k_ver as krb5_int16;
                    (*kp).key_data_kvno = (*kv).k_kvno as krb5_ui_2;
                    if (*kp).key_data_ver as libc::c_int > 2 as libc::c_int {
                        return 22 as libc::c_int
                        /* XXX ? */
                    }
                    cnt = 0 as libc::c_int;
                    while cnt < (*kp).key_data_ver as libc::c_int {
                        (*kp).key_data_type[cnt as usize] =
                            *(*kv).k_enctype.k_enctype_val.offset(cnt as
                                                                      isize)
                                as krb5_int16;
                        (*kp).key_data_length[cnt as usize] =
                            (*(*kv).k_contents.k_contents_val.offset(cnt as
                                                                         isize)).utf8str_t_len
                                as krb5_int16 as krb5_ui_2;
                        newptr =
                            realloc((*kp).key_data_contents[cnt as usize] as
                                        *mut libc::c_void,
                                    (*kp).key_data_length[cnt as usize] as
                                        libc::c_ulong);
                        if newptr.is_null() { return 12 as libc::c_int }
                        (*kp).key_data_contents[cnt as usize] =
                            newptr as *mut krb5_octet;
                        memset((*kp).key_data_contents[cnt as usize] as
                                   *mut libc::c_void, 0 as libc::c_int,
                               (*kp).key_data_length[cnt as usize] as
                                   libc::c_ulong);
                        memcpy((*kp).key_data_contents[cnt as usize] as
                                   *mut libc::c_void,
                               (*(*kv).k_contents.k_contents_val.offset(cnt as
                                                                            isize)).utf8str_t_val
                                   as *const libc::c_void,
                               (*kp).key_data_length[cnt as usize] as
                                   libc::c_ulong);
                        cnt += 1
                    }
                    j += 1
                }
            }
            10 => {
                j = 0 as libc::c_int;
                while j <
                          (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                       isize)).kdbe_val_t_u.av_tldata.av_tldata_len
                              as libc::c_int {
                    newtl.tl_data_type =
                        (*(*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                       isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(j
                                                                                                                               as
                                                                                                                               isize)).tl_type;
                    newtl.tl_data_length =
                        (*(*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                       isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(j
                                                                                                                               as
                                                                                                                               isize)).tl_data.tl_data_len
                            as krb5_int16 as krb5_ui_2;
                    newtl.tl_data_contents =
                        (*(*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                       isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(j
                                                                                                                               as
                                                                                                                               isize)).tl_data.tl_data_val
                            as *mut krb5_octet;
                    newtl.tl_data_next = 0 as *mut _krb5_tl_data;
                    ret = krb5_dbe_update_tl_data(context, ent, &mut newtl);
                    if ret != 0 { return ret }
                    j += 1
                }
                /* END CSTYLED */
            }
            15 => {
                ret =
                    krb5_dbe_update_last_pwd_change(context, ent,
                                                    (*(*update).kdb_update.kdbe_t_val.offset(i
                                                                                                 as
                                                                                                 isize)).kdbe_val_t_u.av_pw_last_change
                                                        as krb5_timestamp);
                if ret != 0 { return ret }
            }
            12 => {
                tmpprinc =
                    conv_princ_2db(context,
                                   &mut (*(*update).kdb_update.kdbe_t_val.offset(i
                                                                                     as
                                                                                     isize)).kdbe_val_t_u.av_mod_princ);
                if tmpprinc.is_null() { return 12 as libc::c_int }
                mod_princ = tmpprinc
            }
            13 => {
                mod_time =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_mod_time
                        as libc::c_int
            }
            11 => {
                (*ent).len =
                    (*(*update).kdb_update.kdbe_t_val.offset(i as
                                                                 isize)).kdbe_val_t_u.av_len
                        as krb5_ui_2
            }
            _ => { }
        }
        i += 1
    }
    /*
     * process mod_princ_data request
     */
    if mod_time != 0 && !mod_princ.is_null() {
        ret =
            krb5_dbe_update_mod_princ_data(context, ent, mod_time,
                                           mod_princ as krb5_const_principal);
        krb5_free_principal(context, mod_princ);
        mod_princ = 0 as krb5_principal;
        if ret != 0 { return ret }
    }
    *entry = ent;
    return 0 as libc::c_int;
}
/*
 * This routine frees up memory associated with the bunched ulog entries.
 */
#[no_mangle]
#[c2rust::src_loc = "761:1"]
pub unsafe extern "C" fn ulog_free_entries(mut updates:
                                               *mut kdb_incr_update_t,
                                           mut no_of_updates: libc::c_int) {
    let mut upd: *mut kdb_incr_update_t = 0 as *mut kdb_incr_update_t;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    if updates.is_null() { return }
    upd = updates;
    /*
     * Loop thru each ulog entry
     */
    cnt = 0 as libc::c_int;
    while cnt < no_of_updates {
        /*
         * ulog entry - kdb_princ_name
         */
        free((*upd).kdb_princ_name.utf8str_t_val as *mut libc::c_void);
        /* BEGIN CSTYLED */
        /*
         * ulog entry - kdb_kdcs_seen_by
         */
        if !(*upd).kdb_kdcs_seen_by.kdb_kdcs_seen_by_val.is_null() {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*upd).kdb_kdcs_seen_by.kdb_kdcs_seen_by_len {
                free((*(*upd).kdb_kdcs_seen_by.kdb_kdcs_seen_by_val.offset(i
                                                                               as
                                                                               isize)).utf8str_t_val
                         as *mut libc::c_void);
                i = i.wrapping_add(1)
            }
            free((*upd).kdb_kdcs_seen_by.kdb_kdcs_seen_by_val as
                     *mut libc::c_void);
        }
        /*
         * ulog entry - kdb_futures
         */
        free((*upd).kdb_futures.kdb_futures_val as *mut libc::c_void);
        /*
         * ulog entry - kdb_update
         */
        if !(*upd).kdb_update.kdbe_t_val.is_null() {
            /*
             * Loop thru all the attributes and free up stuff
             */
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*upd).kdb_update.kdbe_t_len {
                /*
                 * Free av_key_data
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_KEYDATA as libc::c_int as libc::c_uint &&
                       !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_keydata.av_keydata_val.is_null()
                   {
                    j = 0 as libc::c_int as libc::c_uint;
                    while j <
                              (*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                        isize)).kdbe_val_t_u.av_keydata.av_keydata_len
                          {
                        free((*(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                         isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                   as
                                                                                                                                   isize)).k_enctype.k_enctype_val
                                 as *mut libc::c_void);
                        if !(*(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                        isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                  as
                                                                                                                                  isize)).k_contents.k_contents_val.is_null()
                           {
                            k = 0 as libc::c_int;
                            while k <
                                      (*(*(*upd).kdb_update.kdbe_t_val.offset(i
                                                                                  as
                                                                                  isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                            as
                                                                                                                                            isize)).k_ver
                                  {
                                free((*(*(*(*upd).kdb_update.kdbe_t_val.offset(i
                                                                                   as
                                                                                   isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                             as
                                                                                                                                             isize)).k_contents.k_contents_val.offset(k
                                                                                                                                                                                          as
                                                                                                                                                                                          isize)).utf8str_t_val
                                         as *mut libc::c_void);
                                k += 1
                            }
                            free((*(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                             isize)).kdbe_val_t_u.av_keydata.av_keydata_val.offset(j
                                                                                                                                       as
                                                                                                                                       isize)).k_contents.k_contents_val
                                     as *mut libc::c_void);
                        }
                        j = j.wrapping_add(1)
                    }
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_keydata.av_keydata_val
                             as *mut libc::c_void);
                }
                /*
                 * Free av_tl_data
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_TL_DATA as libc::c_int as libc::c_uint &&
                       !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_tldata.av_tldata_val.is_null()
                   {
                    j = 0 as libc::c_int as libc::c_uint;
                    while j <
                              (*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                        isize)).kdbe_val_t_u.av_tldata.av_tldata_len
                          {
                        free((*(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                         isize)).kdbe_val_t_u.av_tldata.av_tldata_val.offset(j
                                                                                                                                 as
                                                                                                                                 isize)).tl_data.tl_data_val
                                 as *mut libc::c_void);
                        j = j.wrapping_add(1)
                    }
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_tldata.av_tldata_val
                             as *mut libc::c_void);
                }
                /*
                 * Free av_princ
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_PRINC as libc::c_int as libc::c_uint {
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_princ.k_realm.utf8str_t_val
                             as *mut libc::c_void);
                    if !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_princ.k_components.k_components_val.is_null()
                       {
                        j = 0 as libc::c_int as libc::c_uint;
                        while j <
                                  (*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                            isize)).kdbe_val_t_u.av_princ.k_components.k_components_len
                              {
                            free((*(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                             isize)).kdbe_val_t_u.av_princ.k_components.k_components_val.offset(j
                                                                                                                                                    as
                                                                                                                                                    isize)).k_data.utf8str_t_val
                                     as *mut libc::c_void);
                            j = j.wrapping_add(1)
                        }
                        free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                       isize)).kdbe_val_t_u.av_princ.k_components.k_components_val
                                 as *mut libc::c_void);
                    }
                }
                /*
                 * Free av_mod_princ
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_MOD_PRINC as libc::c_int as libc::c_uint {
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_mod_princ.k_realm.utf8str_t_val
                             as *mut libc::c_void);
                    if !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_mod_princ.k_components.k_components_val.is_null()
                       {
                        j = 0 as libc::c_int as libc::c_uint;
                        while j <
                                  (*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                            isize)).kdbe_val_t_u.av_mod_princ.k_components.k_components_len
                              {
                            free((*(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                             isize)).kdbe_val_t_u.av_mod_princ.k_components.k_components_val.offset(j
                                                                                                                                                        as
                                                                                                                                                        isize)).k_data.utf8str_t_val
                                     as *mut libc::c_void);
                            j = j.wrapping_add(1)
                        }
                        free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                       isize)).kdbe_val_t_u.av_mod_princ.k_components.k_components_val
                                 as *mut libc::c_void);
                    }
                }
                /*
                 * Free av_mod_where
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_MOD_WHERE as libc::c_int as libc::c_uint &&
                       !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_mod_where.utf8str_t_val.is_null()
                   {
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_mod_where.utf8str_t_val
                             as *mut libc::c_void);
                }
                /*
                 * Free av_pw_policy
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_PW_POLICY as libc::c_int as libc::c_uint &&
                       !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_pw_policy.utf8str_t_val.is_null()
                   {
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_pw_policy.utf8str_t_val
                             as *mut libc::c_void);
                }
                /*
                 * XXX: Free av_pw_hist
                 *
                 * For now, we just free the pointer
                 * to av_pw_hist_val, since we aren't
                 * populating this union member in
                 * the conv api function(s) anyways.
                 */
                if (*(*upd).kdb_update.kdbe_t_val.offset(i as isize)).av_type
                       as libc::c_uint ==
                       AT_PW_HIST as libc::c_int as libc::c_uint &&
                       !(*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                  isize)).kdbe_val_t_u.av_pw_hist.av_pw_hist_val.is_null()
                   {
                    free((*(*upd).kdb_update.kdbe_t_val.offset(i as
                                                                   isize)).kdbe_val_t_u.av_pw_hist.av_pw_hist_val
                             as *mut libc::c_void);
                }
                i = i.wrapping_add(1)
            }
            /*
             * Free up the pointer to kdbe_t_val
             */
            free((*upd).kdb_update.kdbe_t_val as *mut libc::c_void);
        }
        /* END CSTYLED */
        /*
         * Bump up to next struct
         */
        upd = upd.offset(1);
        cnt += 1
    }
    /*
     * Finally, free up the pointer to the bunched ulog entries
     */
    free(updates as *mut libc::c_void);
}
