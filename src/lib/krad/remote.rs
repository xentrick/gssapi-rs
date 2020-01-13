use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:30"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:30"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:30"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:30"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/unistd.h:30"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:30"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:30"]
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
    /* Return a copy of the len bytes of memory at in; set *code to 0 or ENOMEM. */
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
                        krb5_data, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:30"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:30"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:30"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:30"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:30"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::un_h::sockaddr_un;
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn connect(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                       __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                    __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn sendto(__fd: libc::c_int, __buf: *const libc::c_void,
                      __n: size_t, __flags: libc::c_int,
                      __addr: __CONST_SOCKADDR_ARG, __addr_len: socklen_t)
         -> ssize_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/un.h:37"]
pub mod un_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:8"]
    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [libc::c_char; 108],
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/netinet/in.h:30"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
}
#[c2rust::header_src = "/usr/include/netdb.h:30"]
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
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:32"]
pub mod verto_h {
    #[c2rust::src_loc = "60:9"]
    pub type verto_ev_flag = libc::c_uint;
    #[c2rust::src_loc = "76:5"]
    pub const _VERTO_EV_FLAG_MAX: verto_ev_flag = 256;
    #[c2rust::src_loc = "71:5"]
    pub const _VERTO_EV_FLAG_MUTABLE_MASK: verto_ev_flag = 62;
    #[c2rust::src_loc = "70:5"]
    pub const VERTO_EV_FLAG_REINITIABLE: verto_ev_flag = 64;
    #[c2rust::src_loc = "69:5"]
    pub const VERTO_EV_FLAG_IO_CLOSE_FD: verto_ev_flag = 256;
    #[c2rust::src_loc = "68:5"]
    pub const VERTO_EV_FLAG_IO_ERROR: verto_ev_flag = 128;
    #[c2rust::src_loc = "67:5"]
    pub const VERTO_EV_FLAG_IO_WRITE: verto_ev_flag = 32;
    #[c2rust::src_loc = "66:5"]
    pub const VERTO_EV_FLAG_IO_READ: verto_ev_flag = 16;
    #[c2rust::src_loc = "65:5"]
    pub const VERTO_EV_FLAG_PRIORITY_HIGH: verto_ev_flag = 8;
    #[c2rust::src_loc = "64:5"]
    pub const VERTO_EV_FLAG_PRIORITY_MEDIUM: verto_ev_flag = 4;
    #[c2rust::src_loc = "63:5"]
    pub const VERTO_EV_FLAG_PRIORITY_LOW: verto_ev_flag = 2;
    #[c2rust::src_loc = "62:5"]
    pub const VERTO_EV_FLAG_PERSIST: verto_ev_flag = 1;
    #[c2rust::src_loc = "61:5"]
    pub const VERTO_EV_FLAG_NONE: verto_ev_flag = 0;
    #[c2rust::src_loc = "79:1"]
    pub type verto_callback
        =
        unsafe extern "C" fn(_: *mut verto_ctx, _: *mut verto_ev) -> ();
    use super::time_t_h::time_t;
    extern "C" {
        #[c2rust::src_loc = "48:16"]
        pub type verto_ctx;
        #[c2rust::src_loc = "49:16"]
        pub type verto_ev;
        /* *
 * Adds a callback executed when a file descriptor is ready to be read/written.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 * If VERTO_EV_FLAG_IO_CLOSE_FD is provided the passed in fd is automatically
 * closed when the event is freed with verto_del()
 *
 * NOTE: On Windows, the underlying select() only works with sockets. As such,
 * any attempt to add a non-socket io event on Windows will produce undefined
 * results and may even crash.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set (at least one VERTO_EV_FLAG_IO* required).
 * @param callback The callback to fire.
 * @param fd The file descriptor to watch for reads.
 * @return The verto_ev registered with the event context or NULL on error.
 */
        /* *
 * Adds a callback executed after a period of time.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @param interval Time period to wait before firing (in milliseconds).
 * @return The verto_ev registered with the event context.
 */
        /* *
 * Adds a callback executed when there is nothing else to do.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @return The verto_ev registered with the event context.
 */
        /* *
 * Adds a callback executed when a signal is received.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 *
 * NOTE: If you attempt to ignore a signal without the VERTO_EV_FLAG_PERSIST
 * flag, this function fails.
 *
 * NOTE: SIGCHLD is expressly not supported. If you want this notification,
 * please use verto_add_child().
 *
 * WARNNIG: Signal events can only be reliably received in the default verto_ctx
 * in some implementations.  Attempting to receive signal events in non-default
 * loops may result in assert() failures.
 *
 * WARNING: While verto does its best to protect you from crashes, there is
 * essentially no way to do signal events if you mix multiple implementations in
 * a single process. Attempting to do so will result in undefined behavior,
 * and potentially even a crash. You have been warned.
 *
 * @see verto_add_child()
 * @see verto_repeat()
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @param signal The signal to watch for.
 * @return The verto_ev registered with the event context.
 */
        /* *
 * Adds a callback executed when a child process exits.
 *
 * This event will be freed automatically after its execution. Due to the
 * nature of a process' life-cycle, child events cannot persist (processes only
 * exit once). This function returns NULL if you attempt to use
 * VERTO_EV_FLAG_PERSIST. You may, of course, call verto_del() at any time to
 * prevent the callback from firing.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @param child The pid (POSIX) or handle (Win32) of the child to watch for.
 * @return The verto_ev registered with the event context.
 */
        /* *
 * Sets the private pointer of the verto_ev.
 *
 * The free callback will be called in two cases:
 *   1. When the event is deleted (manually or automatically)
 *   2. When verto_set_private() is called again, unless
 *      free is NULL.
 *
 * @see verto_get_private()
 * @param ev The verto_ev
 * @param priv The private value to store
 * @param free The callback used to free the data or NULL
 */
        /* *
 * Gets the private pointer of the verto_ev.
 *
 * @see verto_set_private()
 * @param ev The verto_ev
 * @return The verto_ev private pointer
 */
        /* *
 * Gets the type of the verto_ev.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @param ev The verto_ev
 * @return The verto_ev type
 */
        /* *
 * Gets the flags associated with the given verto_ev.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @see verto_set_flags()
 * @param ev The verto_ev
 * @return The verto_ev type
 */
        #[no_mangle]
        #[c2rust::src_loc = "455:1"]
        pub fn verto_get_flags(ev: *const verto_ev) -> verto_ev_flag;
        #[no_mangle]
        #[c2rust::src_loc = "416:1"]
        pub fn verto_set_private(ev: *mut verto_ev, priv_0: *mut libc::c_void,
                                 free_0: Option<verto_callback>);
        #[no_mangle]
        #[c2rust::src_loc = "300:1"]
        pub fn verto_add_io(ctx: *mut verto_ctx, flags: verto_ev_flag,
                            callback: Option<verto_callback>, fd: libc::c_int)
         -> *mut verto_ev;
        #[no_mangle]
        #[c2rust::src_loc = "321:1"]
        pub fn verto_add_timeout(ctx: *mut verto_ctx, flags: verto_ev_flag,
                                 callback: Option<verto_callback>,
                                 interval: time_t) -> *mut verto_ev;
        #[no_mangle]
        #[c2rust::src_loc = "426:1"]
        pub fn verto_get_private(ev: *const verto_ev) -> *mut libc::c_void;
        /* *
 * Sets the flags associated with the given verto_ev.
 *
 * See _VERTO_EV_FLAG_MUTABLE_MASK for the flags that can be changed
 * with this function. All others will be ignored. If the flags specified
 * are the same as the flags the event already has, this function is a no-op.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @see verto_get_flags()
 * @param ev The verto_ev
 * @param flags The flags for the event
 */
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn verto_set_flags(ev: *mut verto_ev, flags: verto_ev_flag);
        /* *
 * Gets the file descriptor associated with a read/write verto_ev.
 *
 * @see verto_add_io()
 * @param ev The verto_ev to retrieve the file descriptor from.
 * @return The file descriptor, or -1 if not a read/write event.
 */
        #[no_mangle]
        #[c2rust::src_loc = "484:1"]
        pub fn verto_get_fd(ev: *const verto_ev) -> libc::c_int;
        /* *
 * Gets the file descriptor state from when the event fires.
 *
 * @see verto_add_io()
 * @param ev The verto_ev to retrieve the fd state from.
 * @return The fd state.
 */
        #[no_mangle]
        #[c2rust::src_loc = "494:1"]
        pub fn verto_get_fd_state(ev: *const verto_ev) -> verto_ev_flag;
        /* *
 * Removes an event from from the event context and frees it.
 *
 * The event and its contents cannot be used after this call.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @param ev The event to delete.
 */
        #[no_mangle]
        #[c2rust::src_loc = "560:1"]
        pub fn verto_del(ev: *mut verto_ev);
    }
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krad.h:32"]
pub mod krad_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
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
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature releases
 *   (e.g. from 1.12 to 1.13).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    #[c2rust::src_loc = "60:1"]
    pub type krad_attrset = krad_attrset_st;
    #[c2rust::src_loc = "61:1"]
    pub type krad_packet = krad_packet_st;
    #[c2rust::src_loc = "63:1"]
    pub type krad_code = libc::c_uchar;
    /* Called when a response is received or the request times out. */
    #[c2rust::src_loc = "67:1"]
    pub type krad_cb
        =
        Option<unsafe extern "C" fn(_: krb5_error_code, _: *const krad_packet,
                                    _: *const krad_packet,
                                    _: *mut libc::c_void) -> ()>;
    /*
 * Called to iterate over a set of requests.  Either the callback will be
 * called until it returns NULL, or it will be called with cancel = TRUE to
 * terminate in the middle of an iteration.
 */
    #[c2rust::src_loc = "76:1"]
    pub type krad_packet_iter_cb
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_boolean)
                   -> *const krad_packet>;
    use super::krb5_h::{krb5_error_code, krb5_boolean, krb5_data,
                        krb5_context};
    use super::sys_types_h::ssize_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[c2rust::src_loc = "60:16"]
        pub type krad_attrset_st;
        #[c2rust::src_loc = "61:16"]
        pub type krad_packet_st;
        /*
 * Packet
 */
        /* Determine the bytes needed from the socket to get the whole packet.  Don't
 * cache the return value as it can change! Returns -1 on EBADMSG. */
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn krad_packet_bytes_needed(buffer: *const krb5_data) -> ssize_t;
        /* Free a packet. */
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn krad_packet_free(pkt: *mut krad_packet);
        /*
 * Create a new request packet.
 *
 * This function takes the attributes specified in set and converts them into a
 * radius packet. The packet will have a randomized id. If cb is not NULL, it
 * will be called passing data as the argument to iterate over a set of
 * outstanding requests. In this case, the id will be both random and unique
 * across the set of requests.
 */
        #[no_mangle]
        #[c2rust::src_loc = "161:1"]
        pub fn krad_packet_new_request(ctx: krb5_context,
                                       secret: *const libc::c_char,
                                       code: krad_code,
                                       set: *const krad_attrset,
                                       cb: krad_packet_iter_cb,
                                       data: *mut libc::c_void,
                                       request: *mut *mut krad_packet)
         -> krb5_error_code;
        /*
 * Decode a response radius packet from krb5_data.
 *
 * The resulting decoded packet will be a response packet stored in *rsppkt.
 *
 * If cb is NULL, *reqpkt will always be NULL.
 *
 * If cb is not NULL, it will be called (with the data argument) to iterate
 * over a set of requests awaiting responses. In this case, if the response
 * packet matches one of these requests, the original request will be set in
 * *reqpkt.
 */
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn krad_packet_decode_response(ctx: krb5_context,
                                           secret: *const libc::c_char,
                                           buffer: *const krb5_data,
                                           cb: krad_packet_iter_cb,
                                           data: *mut libc::c_void,
                                           reqpkt: *mut *const krad_packet,
                                           rsppkt: *mut *mut krad_packet)
         -> krb5_error_code;
        /* Encode packet. */
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn krad_packet_encode(pkt: *const krad_packet)
         -> *const krb5_data;
    }
    /* KRAD_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krad/internal.h:32"]
pub mod internal_h {
    #[c2rust::src_loc = "50:1"]
    pub type krad_remote = krad_remote_st;
    use super::krad_remote_st;
    /* INTERNAL_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/errno.h:30"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:30"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __time_t, __ssize_t, __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::unistd_h::{socklen_t, close};
pub use self::krb5_h::{krb5_int32, krb5_boolean, krb5_enctype, krb5_flags,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _profile_t};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, make_data, k5calloc, k5alloc,
                         k5memdup, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::sockaddr;
pub use self::sys_socket_h::{__CONST_SOCKADDR_ARG, sockaddr_x25, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, connect, recv, sendto};
pub use self::un_h::sockaddr_un;
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::netdb_h::addrinfo;
pub use self::verto_h::{verto_ev_flag, _VERTO_EV_FLAG_MAX,
                        _VERTO_EV_FLAG_MUTABLE_MASK,
                        VERTO_EV_FLAG_REINITIABLE, VERTO_EV_FLAG_IO_CLOSE_FD,
                        VERTO_EV_FLAG_IO_ERROR, VERTO_EV_FLAG_IO_WRITE,
                        VERTO_EV_FLAG_IO_READ, VERTO_EV_FLAG_PRIORITY_HIGH,
                        VERTO_EV_FLAG_PRIORITY_MEDIUM,
                        VERTO_EV_FLAG_PRIORITY_LOW, VERTO_EV_FLAG_PERSIST,
                        VERTO_EV_FLAG_NONE, verto_callback, verto_ctx,
                        verto_ev, verto_get_flags, verto_set_private,
                        verto_add_io, verto_add_timeout, verto_get_private,
                        verto_set_flags, verto_get_fd, verto_get_fd_state,
                        verto_del};
pub use self::krad_h::{krad_attrset, krad_packet, krad_code, krad_cb,
                       krad_packet_iter_cb, krad_attrset_st, krad_packet_st,
                       krad_packet_bytes_needed, krad_packet_free,
                       krad_packet_new_request, krad_packet_decode_response,
                       krad_packet_encode};
pub use self::internal_h::krad_remote;
use self::stdlib_h::{calloc, free};
use self::errno_h::__errno_location;
use self::string_h::{strncmp, strdup, strcmp, memcmp, memcpy};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "59:8"]
pub struct krad_remote_st {
    pub kctx: krb5_context,
    pub vctx: *mut verto_ctx,
    pub fd: libc::c_int,
    pub io: *mut verto_ev,
    pub secret: *mut libc::c_char,
    pub info: *mut addrinfo,
    pub list: request_head,
    pub buffer_: [libc::c_char; 4096],
    pub buffer: krb5_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "44:15"]
pub struct request_head {
    pub tqh_first: *mut request_st,
    pub tqh_last: *mut *mut request_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "47:8"]
pub struct request_st {
    pub list: C2RustUnnamed_0,
    pub rr: *mut krad_remote,
    pub request: *mut krad_packet,
    pub cb: krad_cb,
    pub data: *mut libc::c_void,
    pub timer: *mut verto_ev,
    pub timeout: libc::c_int,
    pub retries: size_t,
    pub sent: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "48:5"]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut request_st,
    pub tqe_prev: *mut *mut request_st,
}
#[c2rust::src_loc = "46:1"]
pub type request = request_st;
/* Iterate over the set of outstanding packets. */
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn iterator(mut out: *mut *mut request)
 -> *const krad_packet {
    let mut tmp: *mut request = *out;
    if tmp.is_null() { return 0 as *const krad_packet }
    *out = (*tmp).list.tqe_next;
    return (*tmp).request;
}
/* Create a new request. */
#[c2rust::src_loc = "91:1"]
unsafe extern "C" fn request_new(mut rr: *mut krad_remote,
                                 mut rqst: *mut krad_packet,
                                 mut timeout: libc::c_int,
                                 mut retries: size_t, mut cb: krad_cb,
                                 mut data: *mut libc::c_void,
                                 mut out: *mut *mut request)
 -> krb5_error_code {
    let mut tmp: *mut request = 0 as *mut request;
    tmp =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<request>() as libc::c_ulong) as
            *mut request;
    if tmp.is_null() { return 12 as libc::c_int }
    (*tmp).rr = rr;
    (*tmp).request = rqst;
    (*tmp).cb = cb;
    (*tmp).data = data;
    (*tmp).timeout = timeout;
    (*tmp).retries = retries;
    *out = tmp;
    return 0 as libc::c_int;
}
/* Finish a request, calling the callback and freeing it. */
#[inline]
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn request_finish(mut req: *mut request,
                                    mut retval: krb5_error_code,
                                    mut response: *const krad_packet) {
    if retval != 110 as libc::c_int {
        if !(*req).list.tqe_next.is_null() {
            (*(*req).list.tqe_next).list.tqe_prev = (*req).list.tqe_prev
        } else { (*(*req).rr).list.tqh_last = (*req).list.tqe_prev }
        *(*req).list.tqe_prev = (*req).list.tqe_next
    }
    (*req).cb.expect("non-null function pointer")(retval, (*req).request,
                                                  response, (*req).data);
    if retval != 110 as libc::c_int {
        krad_packet_free((*req).request);
        verto_del((*req).timer);
        free(req as *mut libc::c_void);
    };
}
/* Start the timeout timer for the request. */
#[c2rust::src_loc = "130:1"]
unsafe extern "C" fn request_start_timer(mut r: *mut request,
                                         mut vctx: *mut verto_ctx)
 -> krb5_error_code {
    verto_del((*r).timer);
    (*r).timer =
        verto_add_timeout(vctx, VERTO_EV_FLAG_NONE,
                          Some(on_timeout as
                                   unsafe extern "C" fn(_: *mut verto_ctx,
                                                        _: *mut verto_ev)
                                       -> ()), (*r).timeout as time_t);
    if !(*r).timer.is_null() {
        verto_set_private((*r).timer, r as *mut libc::c_void, None);
    }
    return if (*r).timer.is_null() {
               12 as libc::c_int
           } else { 0 as libc::c_int };
}
/* Disconnect from the remote host. */
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn remote_disconnect(mut rr: *mut krad_remote) {
    if (*rr).fd >= 0 as libc::c_int { close((*rr).fd); }
    verto_del((*rr).io);
    (*rr).fd = -(1 as libc::c_int);
    (*rr).io = 0 as *mut verto_ev;
}
/* Add the specified flags to the remote. This automatically manages the
 * lifecyle of the underlying event. Also connects if disconnected. */
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn remote_add_flags(mut remote: *mut krad_remote,
                                      mut flags: verto_ev_flag)
 -> krb5_error_code {
    let mut curflags: verto_ev_flag = VERTO_EV_FLAG_NONE;
    let mut i: libc::c_int = 0;
    flags =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_flag>(flags as libc::c_uint &
                                                   (VERTO_EV_FLAG_IO_READ as
                                                        libc::c_int |
                                                        VERTO_EV_FLAG_IO_WRITE
                                                            as libc::c_int) as
                                                       libc::c_uint);
    if remote.is_null() ||
           flags as libc::c_uint ==
               VERTO_EV_FLAG_NONE as libc::c_int as libc::c_uint {
        return 22 as libc::c_int
    }
    /* If there is no connection, connect. */
    if (*remote).fd < 0 as libc::c_int {
        verto_del((*remote).io);
        (*remote).io = 0 as *mut verto_ev;
        (*remote).fd =
            socket((*(*remote).info).ai_family, (*(*remote).info).ai_socktype,
                   (*(*remote).info).ai_protocol);
        if (*remote).fd < 0 as libc::c_int { return *__errno_location() }
        i =
            connect((*remote).fd,
                    __CONST_SOCKADDR_ARG{__sockaddr__:
                                             (*(*remote).info).ai_addr,},
                    (*(*remote).info).ai_addrlen);
        if i < 0 as libc::c_int {
            i = *__errno_location();
            remote_disconnect(remote);
            return i
        }
    }
    if (*remote).io.is_null() {
        (*remote).io =
            verto_add_io((*remote).vctx,
                         ((VERTO_EV_FLAG_PERSIST as libc::c_int |
                               VERTO_EV_FLAG_IO_ERROR as libc::c_int) as
                              libc::c_uint | flags as libc::c_uint) as
                             verto_ev_flag,
                         Some(on_io as
                                  unsafe extern "C" fn(_: *mut verto_ctx,
                                                       _: *mut verto_ev)
                                      -> ()), (*remote).fd);
        if (*remote).io.is_null() { return 12 as libc::c_int }
        verto_set_private((*remote).io, remote as *mut libc::c_void, None);
    }
    curflags = verto_get_flags((*remote).io);
    if curflags as libc::c_uint & flags as libc::c_uint !=
           flags as libc::c_uint {
        verto_set_flags((*remote).io,
                        ((VERTO_EV_FLAG_PERSIST as libc::c_int |
                              VERTO_EV_FLAG_IO_ERROR as libc::c_int) as
                             libc::c_uint | curflags as libc::c_uint |
                             flags as libc::c_uint) as verto_ev_flag);
    }
    return 0 as libc::c_int;
}
/* Remove the specified flags to the remote. This automatically manages the
 * lifecyle of the underlying event. */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn remote_del_flags(mut remote: *mut krad_remote,
                                      mut flags: verto_ev_flag) {
    if remote.is_null() || (*remote).io.is_null() { return }
    flags =
        (verto_get_flags((*remote).io) as libc::c_uint &
             (VERTO_EV_FLAG_IO_READ as libc::c_int |
                  VERTO_EV_FLAG_IO_WRITE as libc::c_int) as libc::c_uint &
             !(flags as libc::c_uint)) as verto_ev_flag;
    if flags as libc::c_uint ==
           VERTO_EV_FLAG_NONE as libc::c_int as libc::c_uint {
        verto_del((*remote).io);
        (*remote).io = 0 as *mut verto_ev;
        return
    }
    verto_set_flags((*remote).io,
                    ((VERTO_EV_FLAG_PERSIST as libc::c_int |
                          VERTO_EV_FLAG_IO_ERROR as libc::c_int) as
                         libc::c_uint | flags as libc::c_uint) as
                        verto_ev_flag);
}
/* Close the connection and start the timers of all outstanding requests. */
#[c2rust::src_loc = "219:1"]
unsafe extern "C" fn remote_shutdown(mut rr: *mut krad_remote) {
    let mut retval: krb5_error_code = 0;
    let mut r: *mut request = 0 as *mut request;
    remote_disconnect(rr);
    /* Start timers for all unsent packets. */
    r = (*rr).list.tqh_first;
    while !r.is_null() {
        if (*r).timer.is_null() {
            retval = request_start_timer(r, (*rr).vctx);
            if retval != 0 as libc::c_int {
                request_finish(r, retval, 0 as *const krad_packet);
            }
        }
        r = (*r).list.tqe_next
    };
}
/* Handle when packets receive no response within their alloted time. */
#[c2rust::src_loc = "238:1"]
unsafe extern "C" fn on_timeout(mut ctx: *mut verto_ctx,
                                mut ev: *mut verto_ev) {
    let mut req: *mut request =
        verto_get_private(ev) as *mut request; /* Void the timer event. */
    let mut retval: krb5_error_code = 110 as libc::c_int;
    (*req).timer = 0 as *mut verto_ev;
    /* If we have more retries to perform, resend the packet. */
    let fresh0 = (*req).retries;
    (*req).retries = (*req).retries.wrapping_sub(1);
    if fresh0 > 0 as libc::c_int as libc::c_ulong {
        (*req).sent = 0 as libc::c_int as size_t;
        retval = remote_add_flags((*req).rr, VERTO_EV_FLAG_IO_WRITE);
        if retval == 0 as libc::c_int { return }
    }
    request_finish(req, retval, 0 as *const krad_packet);
}
/* Write data to the socket. */
#[c2rust::src_loc = "258:1"]
unsafe extern "C" fn on_io_write(mut rr: *mut krad_remote) {
    let mut tmp: *const krb5_data = 0 as *const krb5_data;
    let mut written: ssize_t = 0;
    let mut r: *mut request = 0 as *mut request;
    r = (*rr).list.tqh_first;
    while !r.is_null() {
        tmp = krad_packet_encode((*r).request);
        /* If the packet has already been sent, do nothing. */
        if (*r).sent == (*tmp).length as libc::c_ulong {
            r = (*r).list.tqe_next
        } else {
            /* Send the packet. */
            written =
                sendto(verto_get_fd((*rr).io),
                       (*tmp).data.offset((*r).sent as isize) as
                           *const libc::c_void,
                       ((*tmp).length as
                            libc::c_ulong).wrapping_sub((*r).sent),
                       0 as libc::c_int,
                       __CONST_SOCKADDR_ARG{__sockaddr__:
                                                0 as *mut libc::c_void as
                                                    *const sockaddr,},
                       0 as libc::c_int as socklen_t);
            if written < 0 as libc::c_int as libc::c_long {
                /* Should we try again? */
                if *__errno_location() == 11 as libc::c_int ||
                       *__errno_location() == 11 as libc::c_int ||
                       *__errno_location() == 105 as libc::c_int ||
                       *__errno_location() == 4 as libc::c_int {
                    return
                }
                /* This error can't be worked around. */
                remote_shutdown(rr);
                return
            }
            /* If the packet was completely sent, set a timeout. */
            (*r).sent =
                ((*r).sent as
                     libc::c_ulong).wrapping_add(written as libc::c_ulong) as
                    size_t as size_t;
            if (*r).sent == (*tmp).length as libc::c_ulong {
                if request_start_timer(r, (*rr).vctx) != 0 as libc::c_int {
                    request_finish(r, 12 as libc::c_int,
                                   0 as *const krad_packet);
                    return
                }
                if remote_add_flags(rr, VERTO_EV_FLAG_IO_READ) !=
                       0 as libc::c_int {
                    remote_shutdown(rr);
                    return
                }
            }
            return
        }
    }
    remote_del_flags(rr, VERTO_EV_FLAG_IO_WRITE);
}
/* Read data from the socket. */
#[c2rust::src_loc = "308:1"]
unsafe extern "C" fn on_io_read(mut rr: *mut krad_remote) {
    let mut req: *const krad_packet = 0 as *const krad_packet;
    let mut rsp: *mut krad_packet = 0 as *mut krad_packet;
    let mut retval: krb5_error_code = 0;
    let mut pktlen: ssize_t = 0;
    let mut tmp: *mut request = 0 as *mut request;
    let mut r: *mut request = 0 as *mut request;
    let mut i: libc::c_int = 0;
    pktlen =
        (::std::mem::size_of::<[libc::c_char; 4096]>() as
             libc::c_ulong).wrapping_sub((*rr).buffer.length as libc::c_ulong)
            as ssize_t;
    if (*(*rr).info).ai_socktype == SOCK_STREAM as libc::c_int {
        pktlen = krad_packet_bytes_needed(&mut (*rr).buffer);
        if pktlen < 0 as libc::c_int as libc::c_long {
            /* If we received a malformed packet on a stream socket,
             * assume the socket to be unrecoverable. */
            remote_shutdown(rr);
            return
        }
    }
    /* Read the packet. */
    i =
        recv(verto_get_fd((*rr).io),
             (*rr).buffer.data.offset((*rr).buffer.length as isize) as
                 *mut libc::c_void, pktlen as size_t, 0 as libc::c_int) as
            libc::c_int;
    /* On these errors, try again. */
    if i < 0 as libc::c_int &&
           (*__errno_location() == 11 as libc::c_int ||
                *__errno_location() == 11 as libc::c_int ||
                *__errno_location() == 4 as libc::c_int) {
        return
    }
    /* On any other errors or on EOF, the socket is unrecoverable. */
    if i <= 0 as libc::c_int { remote_shutdown(rr); return }
    /* If we have a partial read or just the header, try again. */
    (*rr).buffer.length = (*rr).buffer.length.wrapping_add(i as libc::c_uint);
    pktlen = krad_packet_bytes_needed(&mut (*rr).buffer);
    if (*(*rr).info).ai_socktype == SOCK_STREAM as libc::c_int &&
           pktlen > 0 as libc::c_int as libc::c_long {
        return
    }
    /* Decode the packet. */
    tmp = (*rr).list.tqh_first;
    retval =
        krad_packet_decode_response((*rr).kctx, (*rr).secret,
                                    &mut (*rr).buffer,
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                            *mut *mut request)
                                                                       ->
                                                                           *const krad_packet>,
                                                            krad_packet_iter_cb>(Some(iterator
                                                                                          as
                                                                                          unsafe extern "C" fn(_:
                                                                                                                   *mut *mut request)
                                                                                              ->
                                                                                                  *const krad_packet)),
                                    &mut tmp as *mut *mut request as
                                        *mut libc::c_void, &mut req,
                                    &mut rsp);
    (*rr).buffer.length = 0 as libc::c_int as libc::c_uint;
    if retval != 0 as libc::c_int { return }
    /* Match the response with an outstanding request. */
    if !req.is_null() {
        r = (*rr).list.tqh_first;
        while !r.is_null() {
            if (*r).request == req as *mut krad_packet &&
                   (*r).sent ==
                       (*krad_packet_encode(req)).length as libc::c_ulong {
                request_finish(r, 0 as libc::c_int, rsp);
                break ;
            } else { r = (*r).list.tqe_next }
        }
    }
    krad_packet_free(rsp);
}
/* Handle when IO is ready on the socket. */
#[c2rust::src_loc = "373:1"]
unsafe extern "C" fn on_io(mut ctx: *mut verto_ctx, mut ev: *mut verto_ev) {
    let mut rr: *mut krad_remote = 0 as *mut krad_remote;
    rr = verto_get_private(ev) as *mut krad_remote;
    if verto_get_fd_state(ev) as libc::c_uint &
           VERTO_EV_FLAG_IO_WRITE as libc::c_int as libc::c_uint != 0 {
        on_io_write(rr);
    } else { on_io_read(rr); };
}
#[no_mangle]
#[c2rust::src_loc = "386:1"]
pub unsafe extern "C" fn kr_remote_new(mut kctx: krb5_context,
                                       mut vctx: *mut verto_ctx,
                                       mut info: *const addrinfo,
                                       mut secret: *const libc::c_char,
                                       mut rr: *mut *mut krad_remote)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut tmp: *mut krad_remote = 0 as *mut krad_remote;
    tmp =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krad_remote>() as libc::c_ulong) as
            *mut krad_remote;
    if !tmp.is_null() {
        (*tmp).kctx = kctx;
        (*tmp).vctx = vctx;
        (*tmp).buffer =
            make_data((*tmp).buffer_.as_mut_ptr() as *mut libc::c_void,
                      0 as libc::c_int as libc::c_uint);
        (*tmp).list.tqh_first = 0 as *mut request_st;
        (*tmp).list.tqh_last = &mut (*tmp).list.tqh_first;
        (*tmp).fd = -(1 as libc::c_int);
        (*tmp).secret = strdup(secret);
        if !(*tmp).secret.is_null() {
            (*tmp).info =
                k5memdup(info as *const libc::c_void,
                         ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
                         &mut retval) as *mut addrinfo;
            if !(*tmp).info.is_null() {
                (*(*tmp).info).ai_addr =
                    k5memdup((*info).ai_addr as *const libc::c_void,
                             (*info).ai_addrlen as size_t, &mut retval) as
                        *mut sockaddr;
                if !(*tmp).info.is_null() {
                    (*(*tmp).info).ai_next = 0 as *mut addrinfo;
                    (*(*tmp).info).ai_canonname = 0 as *mut libc::c_char;
                    *rr = tmp;
                    return 0 as libc::c_int
                }
            }
        }
    }
    kr_remote_free(tmp);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "424:1"]
pub unsafe extern "C" fn kr_remote_free(mut rr: *mut krad_remote) {
    if rr.is_null() { return }
    while !(*rr).list.tqh_first.is_null() {
        request_finish((*rr).list.tqh_first, 125 as libc::c_int,
                       0 as *const krad_packet);
    }
    free((*rr).secret as *mut libc::c_void);
    if !(*rr).info.is_null() {
        free((*(*rr).info).ai_addr as *mut libc::c_void);
    }
    free((*rr).info as *mut libc::c_void);
    remote_disconnect(rr);
    free(rr as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "441:1"]
pub unsafe extern "C" fn kr_remote_send(mut rr: *mut krad_remote,
                                        mut code: krad_code,
                                        mut attrs: *mut krad_attrset,
                                        mut cb: krad_cb,
                                        mut data: *mut libc::c_void,
                                        mut timeout: libc::c_int,
                                        mut retries: size_t,
                                        mut pkt: *mut *const krad_packet)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut tmp: *mut krad_packet = 0 as *mut krad_packet;
    let mut retval: krb5_error_code = 0;
    let mut r: *mut request = 0 as *mut request;
    if (*(*rr).info).ai_socktype == SOCK_STREAM as libc::c_int {
        retries = 0 as libc::c_int as size_t
    }
    r = (*rr).list.tqh_first;
    retval =
        krad_packet_new_request((*rr).kctx, (*rr).secret, code, attrs,
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut *mut request)
                                                                   ->
                                                                       *const krad_packet>,
                                                        krad_packet_iter_cb>(Some(iterator
                                                                                      as
                                                                                      unsafe extern "C" fn(_:
                                                                                                               *mut *mut request)
                                                                                          ->
                                                                                              *const krad_packet)),
                                &mut r as *mut *mut request as
                                    *mut libc::c_void, &mut tmp);
    if !(retval != 0 as libc::c_int) {
        r = (*rr).list.tqh_first;
        loop  {
            if r.is_null() { current_block = 2968425633554183086; break ; }
            if (*r).request == tmp {
                retval = 114 as libc::c_int;
                current_block = 5905158741307645451;
                break ;
            } else { r = (*r).list.tqe_next }
        }
        match current_block {
            5905158741307645451 => { }
            _ => {
                timeout =
                    (timeout as
                         libc::c_ulong).wrapping_div(retries.wrapping_add(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
                        as libc::c_int;
                retval =
                    request_new(rr, tmp, timeout, retries, cb, data, &mut r);
                if !(retval != 0 as libc::c_int) {
                    retval = remote_add_flags(rr, VERTO_EV_FLAG_IO_WRITE);
                    if !(retval != 0 as libc::c_int) {
                        (*r).list.tqe_next = 0 as *mut request_st;
                        (*r).list.tqe_prev = (*rr).list.tqh_last;
                        *(*rr).list.tqh_last = r;
                        (*rr).list.tqh_last = &mut (*r).list.tqe_next;
                        if !pkt.is_null() { *pkt = tmp }
                        return 0 as libc::c_int
                    }
                }
            }
        }
    }
    krad_packet_free(tmp);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "485:1"]
pub unsafe extern "C" fn kr_remote_cancel(mut rr: *mut krad_remote,
                                          mut pkt: *const krad_packet) {
    let mut r: *mut request = 0 as *mut request;
    r = (*rr).list.tqh_first;
    while !r.is_null() {
        if (*r).request == pkt as *mut krad_packet {
            request_finish(r, 125 as libc::c_int, 0 as *const krad_packet);
            return
        }
        r = (*r).list.tqe_next
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krad/internal.h - Internal declarations for libkrad */
/*
 * Copyright 2013 Red Hat, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *    1. Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *    2. Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
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
/* RFC 2865 */
/* Validate constraints of an attribute. */
/* Encode an attribute. */
/* Decode an attribute. */
/* Encode the attributes into the buffer. */
/* Decode attributes from a buffer. */
/* Create a new remote object which manages a socket and the state of
 * outstanding requests. */
/* Free a remote object. */
/*
 * Send the packet to the remote. The cb will be called when a response is
 * received, the request times out, the request is canceled or an error occurs.
 *
 * The timeout parameter is the total timeout across all retries in
 * milliseconds.
 *
 * If the cb is called with a retval of ETIMEDOUT it indicates that the alloted
 * time has elapsed. However, in the case of a timeout, we continue to listen
 * for the packet until krad_remote_cancel() is called or a response is
 * received. This means that cb will always be called twice in the event of a
 * timeout. This permits you to pursue other remotes while still listening for
 * a response from the first one.
 */
/* Remove packet from the queue of requests awaiting responses. */
/* Determine if this remote object refers to the remote resource identified
 * by the addrinfo struct and the secret. */
#[no_mangle]
#[c2rust::src_loc = "498:1"]
pub unsafe extern "C" fn kr_remote_equals(mut rr: *const krad_remote,
                                          mut info: *const addrinfo,
                                          mut secret: *const libc::c_char)
 -> krb5_boolean {
    let mut a: *mut sockaddr_un = 0 as *mut sockaddr_un;
    let mut b: *mut sockaddr_un = 0 as *mut sockaddr_un;
    if strcmp((*rr).secret, secret) != 0 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*info).ai_addrlen != (*(*rr).info).ai_addrlen {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*info).ai_family != (*(*rr).info).ai_family {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*info).ai_socktype != (*(*rr).info).ai_socktype {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*info).ai_protocol != (*(*rr).info).ai_protocol {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*info).ai_flags != (*(*rr).info).ai_flags {
        return 0 as libc::c_int as krb5_boolean
    }
    if memcmp((*(*rr).info).ai_addr as *const libc::c_void,
              (*info).ai_addr as *const libc::c_void,
              (*info).ai_addrlen as libc::c_ulong) != 0 as libc::c_int {
        /* AF_UNIX fails the memcmp() test due to uninitialized bytes after the
         * socket name. */
        if (*info).ai_family != 1 as libc::c_int {
            return 0 as libc::c_int as krb5_boolean
        }
        a = (*info).ai_addr as *mut sockaddr_un;
        b = (*(*rr).info).ai_addr as *mut sockaddr_un;
        if strncmp((*a).sun_path.as_mut_ptr(), (*b).sun_path.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 108]>() as
                       libc::c_ulong) != 0 as libc::c_int {
            return 0 as libc::c_int as krb5_boolean
        }
    }
    return 1 as libc::c_int as krb5_boolean;
}
