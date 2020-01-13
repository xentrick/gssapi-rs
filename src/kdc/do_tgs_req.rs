use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:54"]
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
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
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
        /* * @defgroup KRB5_KEYUSAGE KRB5_KEYUSAGE
 * @{
 */
        /* XXX need to register these */
        /* Defined in Integrating SAM Mechanisms with Kerberos draft */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY */
        /* Defined in [MS-SFU] */
/* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID */
        /* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_RESPONSE */
        /* unused */
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
        /* *
 * Format and encode a @c KRB_ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  dec_err         Error structure to be encoded
 * @param [out] enc_err         Encoded error structure
 *
 * This function creates a @c KRB_ERROR message in @a enc_err.  Use
 * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3292:1"]
        pub fn krb5_mk_error(context: krb5_context,
                             dec_err: *const krb5_error,
                             enc_err: *mut krb5_data) -> krb5_error_code;
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
 * Build a principal name using null-terminated strings.
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal name
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ...             List of char * components, ending with NULL
 *
 * Call krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
 * same task.  krb5_build_principal() takes variadic arguments.
 * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
 *
 * @code
 * Example of how to build principal H/S@R
 *     krb5_build_principal(context, &principal,
 *                          strlen("R"), "R", "H", "S", (char*)NULL);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4025:1"]
        pub fn krb5_build_principal(context: krb5_context,
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
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context: krb5_context, val: *mut krb5_ticket);
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
        /* From krb5/os, but needed by the outside world */
/* *
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
         -> krb5_error_code;
        /* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        /* *
 * Get the Kerberos realm names for a host.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Host name (or NULL)
 * @param [out] realmsp         Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names.
 * If there are no known realms for the host, a list containing the referral
 * (empty) realm is returned.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 *
 * @retval
 *  0   Success
 * @retval
 *  ENOMEM  Insufficient memory
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6151:1"]
        pub fn krb5_get_host_realm(context: krb5_context,
                                   host: *const libc::c_char,
                                   realmsp: *mut *mut *mut libc::c_char)
         -> krb5_error_code;
        /* *
 * Free the memory allocated by krb5_get_host_realm().
 *
 * @param [in] context          Library context
 * @param [in] realmlist        List of realm names to be released
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6184:1"]
        pub fn krb5_free_host_realm(context: krb5_context,
                                    realmlist: *const *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:54"]
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
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
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
    /* Return the delta between two timestamps (a - b) as a signed 32-bit value,
 * without relying on undefined behavior. */
    #[inline]
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
    }
    /* Increment a timestamp by a signed 32-bit interval, without relying on
 * undefined behavior. */
    #[inline]
    #[c2rust::src_loc = "2354:1"]
    pub unsafe extern "C" fn ts_incr(mut ts: krb5_timestamp,
                                     mut delta: krb5_deltat)
     -> krb5_timestamp {
        return (ts as uint32_t).wrapping_add(delta as uint32_t) as
                   krb5_timestamp;
    }
    /* Return true if a comes after b. */
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_checksum,
                        krb5_principal, krb5_data, krb5_pa_data,
                        krb5_preauthtype, krb5_keyblock, krb5_ticket,
                        krb5_msgtype, krb5_enc_kdc_rep_part, krb5_kdc_rep,
                        krb5_kdc_req, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::string_h::{strlen, memcmp};
    use super::stdint_uintn_h::uint32_t;
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
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_free_pa_s4u_x509_user(_: krb5_context,
                                          _: *mut krb5_pa_s4u_x509_user);
        #[no_mangle]
        #[c2rust::src_loc = "998:1"]
        pub fn k5_free_data_ptr_list(list: *mut *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "1445:1"]
        pub fn encode_krb5_padata_sequence(rep: *const *mut krb5_pa_data,
                                           code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1766:1"]
        pub fn krb5_encrypt_tkt_part(_: krb5_context, _: *const krb5_keyblock,
                                     _: *mut krb5_ticket) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1769:1"]
        pub fn krb5_encode_kdc_rep(_: krb5_context, _: krb5_msgtype,
                                   _: *const krb5_enc_kdc_rep_part,
                                   using_subkey: libc::c_int,
                                   _: *const krb5_keyblock,
                                   _: *mut krb5_kdc_rep,
                                   _: *mut *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2101:1"]
        pub fn krb5_decrypt_tkt_part(_: krb5_context, _: *const krb5_keyblock,
                                     _: *mut krb5_ticket) -> krb5_error_code;
        /* free_rtree.c */
        #[no_mangle]
        #[c2rust::src_loc = "2140:1"]
        pub fn krb5_free_realm_tree(_: krb5_context, _: *mut krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "2150:1"]
        pub fn krb5_free_kdc_req(_: krb5_context, _: *mut krb5_kdc_req);
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "2180:1"]
        pub fn krb5_walk_realm_tree(_: krb5_context, _: *const krb5_data,
                                    _: *const krb5_data,
                                    _: *mut *mut krb5_principal,
                                    _: libc::c_int) -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:54"]
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
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:65"]
pub mod kdb_h {
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
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    use super::krb5_h::{krb5_magic, krb5_ui_2, krb5_ui_4, krb5_flags,
                        krb5_deltat, krb5_timestamp, krb5_kvno, krb5_int16,
                        krb5_octet, krb5_principal, krb5_data, krb5_context,
                        krb5_principal_data, krb5_const_principal,
                        krb5_error_code, krb5_int32, krb5_keyblock,
                        krb5_authdata};
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
        #[c2rust::src_loc = "418:1"]
        pub fn krb5_dbe_find_enctype(kcontext: krb5_context,
                                     dbentp: *mut krb5_db_entry,
                                     ktype: krb5_int32, stype: krb5_int32,
                                     kvno: krb5_int32,
                                     kdatap: *mut *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "723:1"]
        pub fn krb5_db_get_authdata_info(context: krb5_context,
                                         flags: libc::c_uint,
                                         in_authdata: *mut *mut krb5_authdata,
                                         client_princ: krb5_const_principal,
                                         server_princ: krb5_const_principal,
                                         server_key: *mut krb5_keyblock,
                                         krbtgt_key: *mut krb5_keyblock,
                                         krbtgt: *mut krb5_db_entry,
                                         authtime: krb5_timestamp,
                                         ad_info_out: *mut *mut libc::c_void,
                                         client_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "735:1"]
        pub fn krb5_db_free_authdata_info(context: krb5_context,
                                          ad_info: *mut libc::c_void);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/reqstate.h:65"]
pub mod reqstate_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:8"]
    pub struct kdc_request_state {
        pub armor_key: *mut krb5_keyblock,
        pub strengthen_key: *mut krb5_keyblock,
        pub in_cookie_padata: *mut *mut krb5_pa_data,
        pub out_cookie_padata: *mut *mut krb5_pa_data,
        pub fast_options: krb5_int32,
        pub fast_internal_flags: krb5_int32,
        pub realm_data: *mut kdc_realm_t,
    }
    use super::krb5_h::{krb5_keyblock, krb5_pa_data, krb5_int32,
                        krb5_error_code};
    use super::realm_data_h::kdc_realm_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/reqstate.h */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
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
        /* Request state */
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn kdc_make_rstate(active_realm: *mut kdc_realm_t,
                               out: *mut *mut kdc_request_state)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn kdc_free_rstate(s: *mut kdc_request_state);
    }
    /* REQSTATE_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/realm_data.h:65"]
pub mod realm_data_h {
    #[c2rust::src_loc = "36:1"]
    pub type kdc_realm_t = __kdc_realm_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct __kdc_realm_data {
        pub realm_name: *mut libc::c_char,
        pub realm_context: krb5_context,
        pub realm_keytab: krb5_keytab,
        pub realm_hostbased: *mut libc::c_char,
        pub realm_no_referral: *mut libc::c_char,
        pub realm_stash: *mut libc::c_char,
        pub realm_mpname: *mut libc::c_char,
        pub realm_mprinc: krb5_principal,
        pub realm_mkey: krb5_keyblock,
        pub realm_tgsprinc: krb5_principal,
        pub realm_listen: *mut libc::c_char,
        pub realm_tcp_listen: *mut libc::c_char,
        pub realm_maxlife: krb5_deltat,
        pub realm_maxrlife: krb5_deltat,
        pub realm_reject_bad_transit: krb5_boolean,
        pub realm_restrict_anon: krb5_boolean,
    }
    use super::krb5_h::{krb5_context, krb5_keytab, krb5_principal,
                        krb5_keyblock, krb5_deltat, krb5_boolean};
    /* REALM_DATA_H */
    /*
 * These macros used to refer to a global pointer to the active realm state
 * structure for a request.  They now refer to a local variable that must be
 * properly declared in each function that uses these macros.
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/audit_plugin.h:66"]
pub mod audit_plugin_h {
    #[c2rust::src_loc = "80:1"]
    pub type krb5_audit_state = _krb5_audit_state;
    /* *< local or protocol policy problem */
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/krb5/audit_plugin.h - Audit plugin interface */
/*
 * Copyright (C) 2013 by the Massachusetts Institute of Technology.
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
 * NOTE: This is a private interface and may change incompatibly
 *       between versions.
 */
/*
 * Declarations for KDC audit plugin module implementers.  Audit modules allow
 * the KDC to produce log output or audit records in any desired form.
 *
 * The audit interface has a single supported major version, which is 1.  Major
 * version 1 has a current minor version of 1.  Audit modules should define a
 * function named audit_<modulename>_initvt, matching the signature:
 *
 *   krb5_error_code
 *   audit_modname_initvt(krb5_context context, int maj_ver, int min_ver,
 *                        krb5_plugin_vtable vtable);
 *
 * The initvt function should:
 *
 * - Check that the supplied maj_ver number is supported by the module, or
 *   return KRB5_PLUGIN_VER_NOTSUPP if it is not.
 *
 * - Cast the vtable pointer as appropriate for the interface and maj_ver:
 *   maj_ver == 1: Cast to krb5_audit_vtable
 *
 * - Initialize the methods of the vtable, stopping as appropriate for the
 *   supplied min_ver.  Optional methods may be left uninitialized.
 *
 * Memory for the vtable is allocated by the caller, not by the module.
 */
    /* * KDC processing steps */
    /* *< Authenticate request and client */
    /* *< Determine service principal */
    /* *< Validate local and protocol policies */
    /* *< Issue ticket */
    /* *< Encrypt reply */
    /* * Types of violations */
    /* *< Protocol constraint */
    /* *< Local policy violation */
    /* Size of the alpha-numeric request ID */
    /* * KDC audit state structure and declarations */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:16"]
    pub struct _krb5_audit_state {
        pub request: *mut krb5_kdc_req,
        pub reply: *mut krb5_kdc_rep,
        pub cl_addr: *mut krb5_address,
        pub cl_port: krb5_ui_4,
        pub stage: libc::c_int,
        pub status: *const libc::c_char,
        pub tkt_in_id: *mut libc::c_char,
        pub tkt_out_id: *mut libc::c_char,
        pub evid_tkt_id: *mut libc::c_char,
        pub req_id: [libc::c_char; 32],
        pub cl_realm: *mut krb5_data,
        pub s4u2self_user: krb5_principal,
        pub violation: libc::c_int,
    }
    use super::krb5_h::{krb5_kdc_req, krb5_kdc_rep, krb5_address, krb5_ui_4,
                        krb5_data, krb5_principal};
    /* KRB5_AU_PLUGIN_H_INCLUDED */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/net-server.h:65"]
pub mod net_server_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/net-server.h */
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
    /* Declarations for "API" of network listener/dispatcher in libapputils. */
    /* The delimeter characters supported by the addresses string. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct _krb5_fulladdr {
        pub address: *mut krb5_address,
        pub port: krb5_ui_4,
    }
    #[c2rust::src_loc = "37:1"]
    pub type krb5_fulladdr = _krb5_fulladdr;
    use super::krb5_h::{krb5_address, krb5_ui_4};
    /* NET_SERVER_H */
}
#[c2rust::header_src = "/usr/include/ctype.h:70"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:54"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:54"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "329:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
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
    }
}
#[c2rust::header_src = "/usr/include/assert.h:54"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_util.h:65"]
pub mod kdc_util_h {
    use super::reqstate_h::kdc_request_state;
    use super::krb5_h::{krb5_kdc_req, krb5_kdc_rep, krb5_enctype,
                        krb5_error_code, krb5_principal_data,
                        krb5_const_principal, krb5_boolean, krb5_data,
                        krb5_principal, krb5_ticket, krb5_keyblock,
                        krb5_pa_data, krb5_context, krb5_kvno, krb5_magic,
                        krb5_ui_2, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_int16, krb5_octet,
                        krb5_enc_tkt_part, krb5_error, krb5_enc_kdc_rep_part};
    use super::realm_data_h::kdc_realm_t;
    use super::net_server_h::krb5_fulladdr;
    use super::kdb_h::{krb5_db_entry, krb5_tl_data, krb5_key_data};
    use super::k5_int_h::{_krb5_context, krb5_pa_s4u_x509_user};
    use super::com_err_h::errcode_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_util.h */
/*
 * Portions Copyright (C) 2007 Apple Inc.
 * Copyright 1990, 2007, 2014 by the Massachusetts Institute of Technology.
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
 *
 * Declarations for policy.c
 */
        /* authind.c */
        /* cammac.c */
        /* do_as_req.c */
        /* do_tgs_req.c */
        /* dispatch.c */
        /* kdc_preauth.c */
        /* kdc_preauth_ec.c */
        /* kdc_preauth_enctsc.c */
        /* kdc_authdata.c */
        /* replay.c */
        /* kdc_util.c */
        /* FAST*/
        /*
 * If *requestptr contains FX_FAST padata, compute the armor key, verify the
 * checksum over checksummed_data, decode the FAST request, and substitute
 * *requestptr with the inner request.  Set the armor_key, cookie, and
 * fast_options fields in state.  state->cookie will be set for a non-FAST
 * request if it contains FX_COOKIE padata.  If inner_body_out is non-NULL, set
 * *inner_body_out to a copy of the encoded inner body, or to NULL if the
 * request is not a FAST request.
 */
        #[no_mangle]
        #[c2rust::src_loc = "367:1"]
        pub fn kdc_fast_response_handle_padata(state: *mut kdc_request_state,
                                               request: *mut krb5_kdc_req,
                                               rep: *mut krb5_kdc_rep,
                                               enctype: krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "40:1"]
        pub fn is_local_principal(kdc_active_realm: *mut kdc_realm_t,
                                  princ1: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "42:1"]
        pub fn krb5_is_tgs_principal(_: krb5_const_principal) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "43:1"]
        pub fn is_cross_tgs_principal(_: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn add_to_transited(_: *mut krb5_data, _: *mut krb5_data,
                                _: krb5_principal, _: krb5_principal,
                                _: krb5_principal) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn kdc_process_tgs_req(_: *mut kdc_realm_t, _: *mut krb5_kdc_req,
                                   _: *const krb5_fulladdr, _: *mut krb5_data,
                                   _: *mut *mut krb5_ticket,
                                   krbtgt_ptr: *mut *mut krb5_db_entry,
                                   _: *mut *mut krb5_keyblock,
                                   _: *mut *mut krb5_keyblock,
                                   pa_tgs_req: *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn kdc_get_server_key(_: krb5_context, _: *mut krb5_ticket,
                                  _: libc::c_uint,
                                  match_enctype: krb5_boolean,
                                  _: *mut *mut krb5_db_entry,
                                  _: *mut *mut krb5_keyblock,
                                  _: *mut krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn get_local_tgt(context: krb5_context, realm: *const krb5_data,
                             candidate: *mut krb5_db_entry,
                             alias_out: *mut *mut krb5_db_entry,
                             storage_out: *mut *mut krb5_db_entry,
                             kb_out: *mut krb5_keyblock) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn validate_tgs_request(_: *mut kdc_realm_t, _: *mut krb5_kdc_req,
                                    _: krb5_db_entry, _: *mut krb5_ticket,
                                    _: krb5_timestamp,
                                    _: *mut *const libc::c_char,
                                    _: *mut *mut *mut krb5_pa_data)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "93:1"]
        pub fn get_ticket_flags(reqflags: krb5_flags,
                                client: *mut krb5_db_entry,
                                server: *mut krb5_db_entry,
                                header_enc: *mut krb5_enc_tkt_part)
         -> krb5_flags;
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn check_indicators(context: krb5_context,
                                server: *mut krb5_db_entry,
                                indicators: *const *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn select_session_keytype(kdc_active_realm: *mut kdc_realm_t,
                                      server: *mut krb5_db_entry,
                                      nktypes: libc::c_int,
                                      ktypes: *mut krb5_enctype)
         -> krb5_enctype;
        #[no_mangle]
        #[c2rust::src_loc = "384:1"]
        pub fn kdc_fast_hide_client(state: *mut kdc_request_state)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn kdc_fast_handle_error(context: krb5_context,
                                     state: *mut kdc_request_state,
                                     request: *mut krb5_kdc_req,
                                     in_padata: *mut *mut krb5_pa_data,
                                     err: *mut krb5_error,
                                     fast_edata_out: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "331:1"]
        pub fn log_tgs_req(ctx: krb5_context, from: *const krb5_fulladdr,
                           request: *mut krb5_kdc_req,
                           reply: *mut krb5_kdc_rep, cprinc: krb5_principal,
                           sprinc: krb5_principal, altcprinc: krb5_principal,
                           authtime: krb5_timestamp, c_flags: libc::c_uint,
                           status: *const libc::c_char,
                           errcode: krb5_error_code,
                           emsg: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn return_enc_padata(context: krb5_context,
                                 req_pkt: *mut krb5_data,
                                 request: *mut krb5_kdc_req,
                                 reply_key: *mut krb5_keyblock,
                                 server: *mut krb5_db_entry,
                                 reply_encpart: *mut krb5_enc_kdc_rep_part,
                                 is_referral: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn kdc_make_s4u2self_rep(context: krb5_context,
                                     tgs_subkey: *mut krb5_keyblock,
                                     tgs_session: *mut krb5_keyblock,
                                     req_s4u_user: *mut krb5_pa_s4u_x509_user,
                                     reply: *mut krb5_kdc_rep,
                                     reply_encpart:
                                         *mut krb5_enc_kdc_rep_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "224:1"]
        pub fn handle_authdata(context: krb5_context, flags: libc::c_uint,
                               client: *mut krb5_db_entry,
                               server: *mut krb5_db_entry,
                               header_server: *mut krb5_db_entry,
                               local_tgt: *mut krb5_db_entry,
                               local_tgt_key: *mut krb5_keyblock,
                               client_key: *mut krb5_keyblock,
                               server_key: *mut krb5_keyblock,
                               header_key: *mut krb5_keyblock,
                               req_pkt: *mut krb5_data,
                               request: *mut krb5_kdc_req,
                               altcprinc: krb5_const_principal,
                               ad_info: *mut libc::c_void,
                               enc_tkt_request: *mut krb5_enc_tkt_part,
                               auth_indicators: *mut *mut *mut krb5_data,
                               enc_tkt_reply: *mut krb5_enc_tkt_part)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "339:1"]
        pub fn log_tgs_badtrans(ctx: krb5_context, cprinc: krb5_principal,
                                sprinc: krb5_principal,
                                trcont: *mut krb5_data,
                                errcode: krb5_error_code);
        #[no_mangle]
        #[c2rust::src_loc = "302:1"]
        pub fn kdc_check_transited_list(kdc_active_realm: *mut kdc_realm_t,
                                        trans: *const krb5_data,
                                        realm1: *const krb5_data,
                                        realm2: *const krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "379:1"]
        pub fn kdc_fast_handle_reply_key(state: *mut kdc_request_state,
                                         existing_key: *mut krb5_keyblock,
                                         out_key: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "317:1"]
        pub fn kdc_get_ticket_renewtime(realm: *mut kdc_realm_t,
                                        request: *mut krb5_kdc_req,
                                        tgt: *mut krb5_enc_tkt_part,
                                        client: *mut krb5_db_entry,
                                        server: *mut krb5_db_entry,
                                        tkt: *mut krb5_enc_tkt_part);
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn kdc_get_ticket_endtime(kdc_active_realm: *mut kdc_realm_t,
                                      now: krb5_timestamp,
                                      endtime: krb5_timestamp,
                                      till: krb5_timestamp,
                                      client: *mut krb5_db_entry,
                                      server: *mut krb5_db_entry,
                                      out_endtime: *mut krb5_timestamp);
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn get_auth_indicators(context: krb5_context,
                                   enc_tkt: *mut krb5_enc_tkt_part,
                                   local_tgt: *mut krb5_db_entry,
                                   local_tgt_key: *mut krb5_keyblock,
                                   indicators_out: *mut *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "285:1"]
        pub fn kdc_process_s4u2proxy_req(kdc_active_realm: *mut kdc_realm_t,
                                         flags: libc::c_uint,
                                         request: *mut krb5_kdc_req,
                                         t2enc: *const krb5_enc_tkt_part,
                                         krbtgt: *mut krb5_db_entry,
                                         krbtgt_key: *mut krb5_keyblock,
                                         server: *const krb5_db_entry,
                                         server_key: *mut krb5_keyblock,
                                         server_princ: krb5_const_principal,
                                         proxy: *const krb5_db_entry,
                                         proxy_princ: krb5_const_principal,
                                         ad_info: *mut libc::c_void,
                                         stkt_ad_info: *mut *mut libc::c_void,
                                         stkt_ad_client: *mut krb5_principal,
                                         status: *mut *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "264:1"]
        pub fn kdc_process_s4u2self_req(kdc_active_realm: *mut kdc_realm_t,
                                        request: *mut krb5_kdc_req,
                                        client_princ: krb5_const_principal,
                                        c_flags: libc::c_uint,
                                        server: *const krb5_db_entry,
                                        tgs_subkey: *mut krb5_keyblock,
                                        tgs_session: *mut krb5_keyblock,
                                        kdc_time: krb5_timestamp,
                                        s4u2self_req:
                                            *mut *mut krb5_pa_s4u_x509_user,
                                        princ_ptr: *mut *mut krb5_db_entry,
                                        status: *mut *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "344:1"]
        pub fn log_tgs_alt_tgt(context: krb5_context, p: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn kdc_err(call_context: krb5_context, code: errcode_t,
                       fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "532:1"]
        pub fn data2string(d: *mut krb5_data) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "362:1"]
        pub fn kdc_find_fast(requestptr: *mut *mut krb5_kdc_req,
                             checksummed_data: *mut krb5_data,
                             tgs_subkey: *mut krb5_keyblock,
                             tgs_session: *mut krb5_keyblock,
                             state: *mut kdc_request_state,
                             inner_body_out: *mut *mut krb5_data)
         -> krb5_error_code;
    }
    /* __KRB5_KDC_UTIL__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_audit.h:66"]
pub mod kdc_audit_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_ticket, krb5_error_code,
                        krb5_boolean, krb5_kdc_req};
    use super::audit_plugin_h::krb5_audit_state;
    use super::net_server_h::krb5_fulladdr;
    extern "C" {
        /* Utilities */
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn kau_make_tkt_id(context: krb5_context,
                               ticket: *const krb5_ticket,
                               out: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn kau_free_kdc_req(state: *mut krb5_audit_state);
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn kau_tgs_req(context: krb5_context, ev_success: krb5_boolean,
                           state: *mut krb5_audit_state);
        #[no_mangle]
        #[c2rust::src_loc = "48:1"]
        pub fn kau_init_kdc_req(context: krb5_context,
                                request: *mut krb5_kdc_req,
                                from: *const krb5_fulladdr,
                                au_state: *mut *mut krb5_audit_state)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn kau_s4u2proxy(context: krb5_context, ev_success: krb5_boolean,
                             state: *mut krb5_audit_state);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn kau_s4u2self(context: krb5_context, ev_success: krb5_boolean,
                            state: *mut krb5_audit_state);
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn kau_u2u(context: krb5_context, ev_success: krb5_boolean,
                       state: *mut krb5_audit_state);
    }
    /* KRB5_KDC_AUDIT__ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kdc/policy.h:67"]
pub mod policy_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_kdc_req, krb5_ticket, krb5_data,
                        krb5_timestamp, krb5_ticket_times, krb5_error_code};
    use super::kdb_h::krb5_db_entry;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn check_kdcpolicy_tgs(context: krb5_context,
                                   request: *const krb5_kdc_req,
                                   server: *const krb5_db_entry,
                                   ticket: *const krb5_ticket,
                                   auth_indicators: *const *mut krb5_data,
                                   kdc_time: krb5_timestamp,
                                   times: *mut krb5_ticket_times,
                                   status: *mut *const libc::c_char)
         -> krb5_error_code;
    }
    /* __KRB5_KDC_POLICY__ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kdc/extern.h:68"]
pub mod extern_h {
    use super::krb5_h::krb5_data;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/extern.h */
/*
 * Copyright 1990,2001,2007,2009 by the Massachusetts Institute of Technology.
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
        /* various externs for KDC */
        #[no_mangle]
        #[c2rust::src_loc = "30:25"]
        pub static mut empty_string: krb5_data;
    }
    /* __KRB5_KDC_EXTERN__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:69"]
pub mod adm_proto_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
    }
    /* KRB5_ADM_PROTO_H__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
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
                       _krb5_error, krb5_error, krb5_kt_cursor,
                       krb5_keytab_entry_st, krb5_keytab_entry, krb5_keytab,
                       _profile_t, krb5_anonymous_principal,
                       krb5_c_make_random_key, krb5_c_valid_enctype,
                       krb5_mk_error, krb5_realm_compare,
                       krb5_principal_compare, krb5_build_principal,
                       krb5_free_principal, krb5_free_authdata,
                       krb5_free_ticket, krb5_free_keyblock,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_us_timeofday, krb5_timeofday, krb5_get_host_realm,
                       krb5_free_host_realm, krb5_get_error_message,
                       krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         krb5_pa_s4u_x509_user, _krb5_pa_s4u_x509_user,
                         krb5_s4u_userid, _krb5_s4u_userid, data_eq_string,
                         make_data, empty_data, ts_delta, ts_incr, ts_after,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_find_pa_data,
                         krb5_free_pa_s4u_x509_user, k5_free_data_ptr_list,
                         encode_krb5_padata_sequence, krb5_encrypt_tkt_part,
                         krb5_encode_kdc_rep, krb5_decrypt_tkt_part,
                         krb5_free_realm_tree, krb5_free_kdc_req,
                         krb5_free_pa_data, krb5_walk_realm_tree};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::errcode_t;
pub use self::kdb_h::{krb5_db_entry, _krb5_db_entry_new, krb5_key_data,
                      _krb5_key_data, krb5_tl_data, _krb5_tl_data,
                      krb5_keysalt, _krb5_keysalt, krb5_db_get_principal,
                      krb5_db_free_principal, krb5_dbe_find_enctype,
                      krb5_dbe_decrypt_key_data, krb5_db_get_authdata_info,
                      krb5_db_free_authdata_info};
pub use self::reqstate_h::{kdc_request_state, kdc_make_rstate,
                           kdc_free_rstate};
pub use self::realm_data_h::{kdc_realm_t, __kdc_realm_data};
pub use self::audit_plugin_h::{krb5_audit_state, _krb5_audit_state};
pub use self::net_server_h::{_krb5_fulladdr, krb5_fulladdr};
pub use self::ctype_h::{_ISspace, C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, malloc};
use self::string_h::{strlen, strstr, strchr, strdup, memcmp, memset};
use self::assert_h::__assert_fail;
use self::kdc_util_h::{kdc_fast_response_handle_padata, is_local_principal,
                       krb5_is_tgs_principal, is_cross_tgs_principal,
                       add_to_transited, kdc_process_tgs_req,
                       kdc_get_server_key, get_local_tgt,
                       validate_tgs_request, get_ticket_flags,
                       check_indicators, select_session_keytype,
                       kdc_fast_hide_client, kdc_fast_handle_error,
                       log_tgs_req, return_enc_padata, kdc_make_s4u2self_rep,
                       handle_authdata, log_tgs_badtrans,
                       kdc_check_transited_list, kdc_fast_handle_reply_key,
                       kdc_get_ticket_renewtime, kdc_get_ticket_endtime,
                       get_auth_indicators, kdc_process_s4u2proxy_req,
                       kdc_process_s4u2self_req, log_tgs_alt_tgt, kdc_err,
                       data2string, kdc_find_fast};
use self::kdc_audit_h::{kau_make_tkt_id, kau_free_kdc_req, kau_tgs_req,
                        kau_init_kdc_req, kau_s4u2proxy, kau_s4u2self,
                        kau_u2u};
use self::policy_h::check_kdcpolicy_tgs;
use self::extern_h::empty_string;
use self::adm_proto_h::krb5_klog_syslog;
/*ARGSUSED*/
#[no_mangle]
#[c2rust::src_loc = "100:1"]
pub unsafe extern "C" fn process_tgs_req(mut request: *mut krb5_kdc_req,
                                         mut pkt: *mut krb5_data,
                                         mut from: *const krb5_fulladdr,
                                         mut kdc_active_realm:
                                             *mut kdc_realm_t,
                                         mut response: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64; /* TGT */
    let mut subkey: *mut krb5_keyblock =
        0 as *mut krb5_keyblock; /* TGT or evidence ticket */
    let mut header_key: *mut krb5_keyblock =
        0 as *mut krb5_keyblock; /* protocol transition request */
    let mut stkt_server_key: *mut krb5_keyblock =
        0 as *mut krb5_keyblock; /* auth data issued by KDC */
    let mut subject_key: *mut krb5_keyblock =
        0 as *mut krb5_keyblock; /* client/server KDB flags */
    let mut server: *mut krb5_db_entry =
        0 as *mut krb5_db_entry; /*points into request*/
    let mut stkt_server: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut subject_server: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut reply: krb5_kdc_rep =
        krb5_kdc_rep{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     client: 0 as *mut krb5_principal_data,
                     ticket: 0 as *mut krb5_ticket,
                     enc_part:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     enc_part2: 0 as *mut krb5_enc_kdc_rep_part,};
    let mut reply_encpart: krb5_enc_kdc_rep_part =
        krb5_enc_kdc_rep_part{magic: 0,
                              msg_type: 0,
                              session: 0 as *mut krb5_keyblock,
                              last_req: 0 as *mut *mut krb5_last_req_entry,
                              nonce: 0,
                              key_exp: 0,
                              flags: 0,
                              times:
                                  krb5_ticket_times{authtime: 0,
                                                    starttime: 0,
                                                    endtime: 0,
                                                    renew_till: 0,},
                              server: 0 as *mut krb5_principal_data,
                              caddrs: 0 as *mut *mut krb5_address,
                              enc_padata: 0 as *mut *mut krb5_pa_data,};
    let mut ticket_reply: krb5_ticket =
        krb5_ticket{magic: 0,
                    server: 0 as *mut krb5_principal_data,
                    enc_part:
                        krb5_enc_data{magic: 0,
                                      enctype: 0,
                                      kvno: 0,
                                      ciphertext:
                                          krb5_data{magic: 0,
                                                    length: 0,
                                                    data:
                                                        0 as
                                                            *mut libc::c_char,},},
                    enc_part2: 0 as *mut krb5_enc_tkt_part,};
    let mut header_ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    let mut st_idx: libc::c_int = 0 as libc::c_int;
    let mut enc_tkt_reply: krb5_enc_tkt_part =
        krb5_enc_tkt_part{magic: 0,
                          flags: 0,
                          session: 0 as *mut krb5_keyblock,
                          client: 0 as *mut krb5_principal_data,
                          transited:
                              krb5_transited{magic: 0,
                                             tr_type: 0,
                                             tr_contents:
                                                 krb5_data{magic: 0,
                                                           length: 0,
                                                           data:
                                                               0 as
                                                                   *mut libc::c_char,},},
                          times:
                              krb5_ticket_times{authtime: 0,
                                                starttime: 0,
                                                endtime: 0,
                                                renew_till: 0,},
                          caddrs: 0 as *mut *mut krb5_address,
                          authorization_data: 0 as *mut *mut krb5_authdata,};
    let mut newtransited: libc::c_int = 0 as libc::c_int;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut server_keyblock: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut encrypting_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut kdc_time: krb5_timestamp = 0;
    let mut authtime: krb5_timestamp = 0 as libc::c_int;
    let mut session_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut local_tgt_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut reply_key: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut server_key: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut cprinc: krb5_principal = 0 as krb5_principal;
    let mut sprinc: krb5_principal = 0 as krb5_principal;
    let mut altcprinc: krb5_principal = 0 as krb5_principal;
    let mut authdata_client: krb5_const_principal =
        0 as *const krb5_principal_data;
    let mut stkt_authdata_client: krb5_principal = 0 as krb5_principal;
    let mut nolrarray: [*mut krb5_last_req_entry; 2] =
        [0 as *mut krb5_last_req_entry; 2];
    let mut nolrentry: krb5_last_req_entry =
        krb5_last_req_entry{magic: 0, lr_type: 0, value: 0,};
    let mut errcode: libc::c_int = 0;
    let mut status: *const libc::c_char = 0 as *const libc::c_char;
    let mut header_enc_tkt: *mut krb5_enc_tkt_part =
        0 as *mut krb5_enc_tkt_part;
    let mut subject_tkt: *mut krb5_enc_tkt_part = 0 as *mut krb5_enc_tkt_part;
    let mut client: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut header_server: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut local_tgt: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut local_tgt_storage: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut s4u_x509_user: *mut krb5_pa_s4u_x509_user =
        0 as *mut krb5_pa_s4u_x509_user;
    let mut kdc_issued_auth_data: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut c_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut s_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut is_referral: krb5_boolean = 0;
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut ticket_kvno: krb5_kvno = 0 as libc::c_int as krb5_kvno;
    let mut state: *mut kdc_request_state = 0 as *mut kdc_request_state;
    let mut pa_tgs_req: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut scratch: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut e_data: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut au_state: *mut krb5_audit_state = 0 as *mut krb5_audit_state;
    let mut auth_indicators: *mut *mut krb5_data = 0 as *mut *mut krb5_data;
    let mut ad_info: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut stkt_ad_info: *mut libc::c_void = 0 as *mut libc::c_void;
    memset(&mut reply as *mut krb5_kdc_rep as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_kdc_rep>() as libc::c_ulong);
    memset(&mut reply_encpart as *mut krb5_enc_kdc_rep_part as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_enc_kdc_rep_part>() as libc::c_ulong);
    memset(&mut ticket_reply as *mut krb5_ticket as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_ticket>() as libc::c_ulong);
    memset(&mut enc_tkt_reply as *mut krb5_enc_tkt_part as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_enc_tkt_part>() as libc::c_ulong);
    memset(&mut server_keyblock as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    memset(&mut local_tgt_key as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    session_key.contents = 0 as *mut krb5_octet;
    /* Save pointer to client-requested service principal, in case of
     * errors before a successful call to search_sprinc(). */
    sprinc = (*request).server;
    if (*request).msg_type != 12 as libc::c_int as krb5_msgtype {
        krb5_free_kdc_req((*kdc_active_realm).realm_context, request);
        return -(1765328246 as libc::c_long) as krb5_error_code
    }
    errcode = kdc_make_rstate(kdc_active_realm, &mut state);
    if errcode != 0 as libc::c_int {
        krb5_free_kdc_req((*kdc_active_realm).realm_context, request);
        return errcode
    }
    /* Initialize audit state. */
    errcode =
        kau_init_kdc_req((*kdc_active_realm).realm_context, request, from,
                         &mut au_state);
    if errcode != 0 {
        krb5_free_kdc_req((*kdc_active_realm).realm_context, request);
        return errcode
    }
    /* Seed the audit trail with the request ID and basic information. */
    kau_tgs_req((*kdc_active_realm).realm_context,
                1 as libc::c_int as krb5_boolean, au_state); /* XXX? */
    errcode =
        kdc_process_tgs_req(kdc_active_realm, request, from, pkt,
                            &mut header_ticket, &mut header_server,
                            &mut header_key, &mut subkey, &mut pa_tgs_req);
    if !header_ticket.is_null() && !(*header_ticket).enc_part2.is_null() {
        cprinc = (*(*header_ticket).enc_part2).client
    }
    if errcode != 0 {
        status = b"PROCESS_TGS\x00" as *const u8 as *const libc::c_char
    } else if header_ticket.is_null() {
        errcode = -(1765328241 as libc::c_long) as libc::c_int
    } else {
        errcode =
            kau_make_tkt_id((*kdc_active_realm).realm_context, header_ticket,
                            &mut (*au_state).tkt_in_id);
        if !(errcode != 0) {
            scratch.length = (*pa_tgs_req).length;
            scratch.data = (*pa_tgs_req).contents as *mut libc::c_char;
            errcode =
                kdc_find_fast(&mut request, &mut scratch, subkey,
                              (*(*header_ticket).enc_part2).session, state,
                              0 as *mut *mut krb5_data);
            /* Reset sprinc because kdc_find_fast() can replace request. */
            sprinc = (*request).server;
            if errcode != 0 as libc::c_int {
                status = b"FIND_FAST\x00" as *const u8 as *const libc::c_char
            } else {
                errcode =
                    get_local_tgt((*kdc_active_realm).realm_context,
                                  &mut (*sprinc).realm, header_server,
                                  &mut local_tgt, &mut local_tgt_storage,
                                  &mut local_tgt_key);
                if errcode != 0 {
                    status =
                        b"GET_LOCAL_TGT\x00" as *const u8 as
                            *const libc::c_char
                } else {
                    /* Ignore (for now) the request modification due to FAST processing. */
                    (*au_state).request = request;
                    /*
     * Pointer to the encrypted part of the header ticket, which may be
     * replaced to point to the encrypted part of the evidence ticket
     * if constrained delegation is used. This simplifies the number of
     * special cases for constrained delegation.
     */
                    header_enc_tkt = (*header_ticket).enc_part2;
                    /*
     * We've already dealt with the AP_REQ authentication, so we can
     * use header_ticket freely.  The encrypted part (if any) has been
     * decrypted with the session key.
     */
                    (*au_state).stage = 2 as libc::c_int;
                    /* XXX make sure server here has the proper realm...taken from AP_REQ
       header? */
                    if (*request).kdc_options & 0x10000 as libc::c_int != 0 {
                        c_flags |= 0x10 as libc::c_int as libc::c_uint;
                        s_flags |= 0x10 as libc::c_int as libc::c_uint
                    }
                    errcode =
                        search_sprinc(kdc_active_realm, request,
                                      s_flags as krb5_flags, &mut server,
                                      &mut status);
                    if !(errcode != 0 as libc::c_int) {
                        sprinc = (*server).princ;
                        /* If we got a cross-realm TGS which is not the requested server, we are
     * issuing a referral (or alternate TGT, which we treat similarly). */
                        is_referral =
                            (is_cross_tgs_principal((*server).princ as
                                                        krb5_const_principal)
                                 != 0 &&
                                 krb5_principal_compare((*kdc_active_realm).realm_context,
                                                        (*request).server as
                                                            krb5_const_principal,
                                                        (*server).princ as
                                                            krb5_const_principal)
                                     == 0) as libc::c_int as krb5_boolean;
                        (*au_state).stage = 3 as libc::c_int;
                        errcode =
                            krb5_timeofday((*kdc_active_realm).realm_context,
                                           &mut kdc_time);
                        if !(errcode != 0) {
                            retval =
                                validate_tgs_request(kdc_active_realm,
                                                     request, *server,
                                                     header_ticket, kdc_time,
                                                     &mut status,
                                                     &mut e_data);
                            if retval != 0 {
                                if retval == 12 as libc::c_int ||
                                       retval == 13 as libc::c_int {
                                    (*au_state).violation = 1 as libc::c_int
                                }
                                errcode =
                                    (retval as libc::c_long +
                                         -(1765328384 as libc::c_long)) as
                                        libc::c_int
                            } else {
                                if is_local_principal(kdc_active_realm,
                                                      (*header_ticket).server
                                                          as
                                                          krb5_const_principal)
                                       == 0 {
                                    c_flags |=
                                        0x1000 as libc::c_int as libc::c_uint
                                }
                                if is_referral != 0 {
                                    c_flags |=
                                        0x4000 as libc::c_int as libc::c_uint
                                }
                                /* Check for protocol transition */
                                errcode =
                                    kdc_process_s4u2self_req(kdc_active_realm,
                                                             request,
                                                             (*header_enc_tkt).client
                                                                 as
                                                                 krb5_const_principal,
                                                             c_flags, server,
                                                             subkey,
                                                             (*header_enc_tkt).session,
                                                             kdc_time,
                                                             &mut s4u_x509_user,
                                                             &mut client,
                                                             &mut status);
                                if !s4u_x509_user.is_null() ||
                                       errcode != 0 as libc::c_int {
                                    if !s4u_x509_user.is_null() {
                                        (*au_state).s4u2self_user =
                                            (*s4u_x509_user).user_id.user
                                    }
                                    if errcode == 12 as libc::c_int ||
                                           errcode == 13 as libc::c_int {
                                        (*au_state).violation =
                                            1 as libc::c_int
                                    }
                                    (*au_state).status = status;
                                    kau_s4u2self((*kdc_active_realm).realm_context,
                                                 if errcode != 0 {
                                                     0 as libc::c_int
                                                 } else { 1 as libc::c_int }
                                                     as krb5_boolean,
                                                 au_state);
                                    (*au_state).s4u2self_user =
                                        0 as krb5_principal
                                }
                                if !(errcode != 0) {
                                    if !s4u_x509_user.is_null() &&
                                           client.is_null() {
                                        /*
         * For an S4U2Self referral request (the requesting service is
         * following a referral back to its own realm), the authdata in the
         * header ticket should be for the requested client.
         */
                                        c_flags |=
                                            0x100 as libc::c_int as
                                                libc::c_uint;
                                        authdata_client =
                                            (*s4u_x509_user).user_id.user as
                                                krb5_const_principal
                                    } else {
                                        /* Otherwise (including for initial S4U2Self requests), the authdata
         * should be for the header ticket client. */
                                        authdata_client =
                                            (*header_enc_tkt).client as
                                                krb5_const_principal
                                    }
                                    errcode =
                                        krb5_db_get_authdata_info((*kdc_active_realm).realm_context,
                                                                  c_flags,
                                                                  (*header_enc_tkt).authorization_data,
                                                                  authdata_client,
                                                                  (*request).server
                                                                      as
                                                                      krb5_const_principal,
                                                                  header_key,
                                                                  &mut local_tgt_key,
                                                                  local_tgt,
                                                                  (*header_enc_tkt).times.authtime,
                                                                  &mut ad_info,
                                                                  0 as
                                                                      *mut krb5_principal);
                                    if !(errcode != 0 &&
                                             errcode as libc::c_long !=
                                                 -(1765328134 as
                                                       libc::c_long)) {
                                        /* Flag all S4U2Self requests now that we have checked the authdata. */
                                        if !s4u_x509_user.is_null() {
                                            c_flags |=
                                                0x100 as libc::c_int as
                                                    libc::c_uint
                                        }
                                        /* Deal with user-to-user and constrained delegation */
                                        errcode =
                                            decrypt_2ndtkt(kdc_active_realm,
                                                           request,
                                                           c_flags as
                                                               krb5_flags,
                                                           &mut stkt_server,
                                                           &mut stkt_server_key,
                                                           &mut status);
                                        if !(errcode != 0) {
                                            if (*request).kdc_options &
                                                   0x20000 as libc::c_int != 0
                                               {
                                                /* Do constrained delegation protocol and authorization checks */
                                                c_flags |=
                                                    0x200 as libc::c_int as
                                                        libc::c_uint;
                                                errcode =
                                                    kdc_process_s4u2proxy_req(kdc_active_realm,
                                                                              c_flags,
                                                                              request,
                                                                              (**(*request).second_ticket.offset(st_idx
                                                                                                                     as
                                                                                                                     isize)).enc_part2,
                                                                              local_tgt,
                                                                              &mut local_tgt_key,
                                                                              stkt_server,
                                                                              stkt_server_key,
                                                                              (*(*header_ticket).enc_part2).client
                                                                                  as
                                                                                  krb5_const_principal,
                                                                              server,
                                                                              (*request).server
                                                                                  as
                                                                                  krb5_const_principal,
                                                                              ad_info,
                                                                              &mut stkt_ad_info,
                                                                              &mut stkt_authdata_client,
                                                                              &mut status);
                                                if errcode ==
                                                       12 as libc::c_int ||
                                                       errcode ==
                                                           13 as libc::c_int {
                                                    (*au_state).violation =
                                                        1 as libc::c_int
                                                } else if errcode != 0 {
                                                    (*au_state).violation =
                                                        2 as libc::c_int
                                                }
                                                (*au_state).status = status;
                                                retval =
                                                    kau_make_tkt_id((*kdc_active_realm).realm_context,
                                                                    *(*request).second_ticket.offset(st_idx
                                                                                                         as
                                                                                                         isize),
                                                                    &mut (*au_state).evid_tkt_id);
                                                if retval != 0 {
                                                    errcode = retval;
                                                    current_block =
                                                        1132313830116969717;
                                                } else {
                                                    kau_s4u2proxy((*kdc_active_realm).realm_context,
                                                                  if errcode
                                                                         != 0
                                                                     {
                                                                      0 as
                                                                          libc::c_int
                                                                  } else {
                                                                      1 as
                                                                          libc::c_int
                                                                  } as
                                                                      krb5_boolean,
                                                                  au_state);
                                                    if errcode != 0 {
                                                        current_block =
                                                            1132313830116969717;
                                                    } else {
                                                        if krb5_is_tgs_principal((*header_ticket).server
                                                                                     as
                                                                                     krb5_const_principal)
                                                               != 0 {
                                                        } else {
                                                            __assert_fail(b"krb5_is_tgs_principal(header_ticket->server)\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          b"do_tgs_req.c\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char,
                                                                          361
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint,
                                                                          (*::std::mem::transmute::<&[u8; 113],
                                                                                                    &[libc::c_char; 113]>(b"krb5_error_code process_tgs_req(krb5_kdc_req *, krb5_data *, const krb5_fulladdr *, kdc_realm_t *, krb5_data **)\x00")).as_ptr());
                                                        }
                                                        current_block =
                                                            11718254377427810743;
                                                    }
                                                }
                                            } else {
                                                current_block =
                                                    11718254377427810743;
                                            }
                                            match current_block {
                                                1132313830116969717 => { }
                                                _ => {
                                                    (*au_state).stage =
                                                        4 as libc::c_int;
                                                    errcode =
                                                        gen_session_key(kdc_active_realm,
                                                                        request,
                                                                        server,
                                                                        &mut session_key,
                                                                        &mut status);
                                                    if !(errcode != 0) {
                                                        /*
     * subject_tkt will refer to the evidence ticket (for constrained
     * delegation) or the TGT. The distinction from header_enc_tkt is
     * necessary because the TGS signature only protects some fields:
     * the others could be forged by a malicious server.
     */
                                                        if c_flags &
                                                               0x200 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                               != 0 {
                                                            subject_tkt =
                                                                (**(*request).second_ticket.offset(st_idx
                                                                                                       as
                                                                                                       isize)).enc_part2;
                                                            subject_server =
                                                                stkt_server;
                                                            subject_key =
                                                                stkt_server_key
                                                        } else {
                                                            subject_tkt =
                                                                header_enc_tkt;
                                                            subject_server =
                                                                header_server;
                                                            subject_key =
                                                                header_key
                                                        }
                                                        authtime =
                                                            (*subject_tkt).times.authtime;
                                                        /* Extract auth indicators from the subject ticket, except for S4U2Self
     * requests (where the client didn't authenticate). */
                                                        if s4u_x509_user.is_null()
                                                           {
                                                            errcode =
                                                                get_auth_indicators((*kdc_active_realm).realm_context,
                                                                                    subject_tkt,
                                                                                    local_tgt,
                                                                                    &mut local_tgt_key,
                                                                                    &mut auth_indicators); /* XXX careful for realm... */
                                                            if errcode != 0 {
                                                                status =
                                                                    b"GET_AUTH_INDICATORS\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char;
                                                                current_block
                                                                    =
                                                                    1132313830116969717;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    8507773468922410051;
                                                            }
                                                        } else {
                                                            current_block =
                                                                8507773468922410051;
                                                        }
                                                        match current_block {
                                                            1132313830116969717
                                                            => {
                                                            }
                                                            _ => {
                                                                errcode =
                                                                    check_indicators((*kdc_active_realm).realm_context,
                                                                                     server,
                                                                                     auth_indicators);
                                                                if errcode !=
                                                                       0 {
                                                                    status =
                                                                        b"HIGHER_AUTHENTICATION_REQUIRED\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                } else {
                                                                    if is_referral
                                                                           !=
                                                                           0 {
                                                                        ticket_reply.server
                                                                            =
                                                                            (*server).princ
                                                                    } else {
                                                                        ticket_reply.server
                                                                            =
                                                                            (*request).server
                                                                    }
                                                                    enc_tkt_reply.flags
                                                                        =
                                                                        get_ticket_flags((*request).kdc_options,
                                                                                         client,
                                                                                         server,
                                                                                         header_enc_tkt);
                                                                    enc_tkt_reply.times.starttime
                                                                        =
                                                                        0 as
                                                                            libc::c_int;
                                                                    /* OK_TO_AUTH_AS_DELEGATE must be set on the service requesting S4U2Self
     * for forwardable tickets to be issued. */
                                                                    if c_flags
                                                                           &
                                                                           0x100
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                           !=
                                                                           0
                                                                           &&
                                                                           is_referral
                                                                               ==
                                                                               0
                                                                           &&
                                                                           (*server).attributes
                                                                               &
                                                                               0x200000
                                                                                   as
                                                                                   libc::c_int
                                                                               ==
                                                                               0
                                                                       {
                                                                        enc_tkt_reply.flags
                                                                            &=
                                                                            !(0x40000000
                                                                                  as
                                                                                  libc::c_int)
                                                                    }
                                                                    /* don't use new addresses unless forwarded, see below */
                                                                    enc_tkt_reply.caddrs
                                                                        =
                                                                        (*header_enc_tkt).caddrs;
                                                                    /* noaddrarray[0] = 0; */
                                                                    reply_encpart.caddrs
                                                                        =
                                                                        0 as
                                                                            *mut *mut krb5_address; /* optional...don't put it in */
                                                                    reply_encpart.enc_padata
                                                                        =
                                                                        0 as
                                                                            *mut *mut krb5_pa_data;
                                                                    /*
     * It should be noted that local policy may affect the
     * processing of any of these flags.  For example, some
     * realms may refuse to issue renewable tickets
     */
                                                                    if (*request).kdc_options
                                                                           &
                                                                           0x20000000
                                                                               as
                                                                               libc::c_int
                                                                           !=
                                                                           0
                                                                           ||
                                                                           (*request).kdc_options
                                                                               &
                                                                               0x8000000
                                                                                   as
                                                                                   libc::c_int
                                                                               !=
                                                                               0
                                                                       {
                                                                        /* include new addresses in ticket & reply */
                                                                        enc_tkt_reply.caddrs
                                                                            =
                                                                            (*request).addresses;
                                                                        reply_encpart.caddrs
                                                                            =
                                                                            (*request).addresses
                                                                    }
                                                                    if (*request).kdc_options
                                                                           &
                                                                           0x2000000
                                                                               as
                                                                               libc::c_int
                                                                           !=
                                                                           0 {
                                                                        enc_tkt_reply.times.starttime
                                                                            =
                                                                            (*request).from
                                                                    } else {
                                                                        enc_tkt_reply.times.starttime
                                                                            =
                                                                            kdc_time
                                                                    }
                                                                    if (*request).kdc_options
                                                                           &
                                                                           0x1
                                                                               as
                                                                               libc::c_int
                                                                           !=
                                                                           0 {
                                                                        if c_flags
                                                                               &
                                                                               (0x100
                                                                                    as
                                                                                    libc::c_int
                                                                                    |
                                                                                    0x200
                                                                                        as
                                                                                        libc::c_int)
                                                                                   as
                                                                                   libc::c_uint
                                                                               ==
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                           {
                                                                        } else {
                                                                            __assert_fail(b"isflagset(c_flags, KRB5_KDB_FLAGS_S4U) == 0\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char,
                                                                                          b"do_tgs_req.c\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char,
                                                                                          450
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint,
                                                                                          (*::std::mem::transmute::<&[u8; 113],
                                                                                                                    &[libc::c_char; 113]>(b"krb5_error_code process_tgs_req(krb5_kdc_req *, krb5_data *, const krb5_fulladdr *, kdc_realm_t *, krb5_data **)\x00")).as_ptr());
                                                                        }
                                                                        /* BEWARE of allocation hanging off of ticket & enc_part2, it belongs
           to the caller */
                                                                        ticket_reply
                                                                            =
                                                                            *header_ticket;
                                                                        enc_tkt_reply
                                                                            =
                                                                            *(*header_ticket).enc_part2;
                                                                        enc_tkt_reply.authorization_data
                                                                            =
                                                                            0
                                                                                as
                                                                                *mut *mut krb5_authdata;
                                                                        enc_tkt_reply.flags
                                                                            &=
                                                                            !(0x1000000
                                                                                  as
                                                                                  libc::c_int)
                                                                    }
                                                                    if (*request).kdc_options
                                                                           &
                                                                           0x2
                                                                               as
                                                                               libc::c_int
                                                                           !=
                                                                           0 {
                                                                        let mut old_starttime:
                                                                                krb5_timestamp =
                                                                            0;
                                                                        let mut old_life:
                                                                                krb5_deltat =
                                                                            0;
                                                                        if c_flags
                                                                               &
                                                                               (0x100
                                                                                    as
                                                                                    libc::c_int
                                                                                    |
                                                                                    0x200
                                                                                        as
                                                                                        libc::c_int)
                                                                                   as
                                                                                   libc::c_uint
                                                                               ==
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                           {
                                                                        } else {
                                                                            __assert_fail(b"isflagset(c_flags, KRB5_KDB_FLAGS_S4U) == 0\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char,
                                                                                          b"do_tgs_req.c\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char,
                                                                                          463
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint,
                                                                                          (*::std::mem::transmute::<&[u8; 113],
                                                                                                                    &[libc::c_char; 113]>(b"krb5_error_code process_tgs_req(krb5_kdc_req *, krb5_data *, const krb5_fulladdr *, kdc_realm_t *, krb5_data **)\x00")).as_ptr());
                                                                        }
                                                                        /* BEWARE of allocation hanging off of ticket & enc_part2, it belongs
           to the caller */
                                                                        ticket_reply
                                                                            =
                                                                            *header_ticket;
                                                                        enc_tkt_reply
                                                                            =
                                                                            *(*header_ticket).enc_part2;
                                                                        enc_tkt_reply.authorization_data
                                                                            =
                                                                            0
                                                                                as
                                                                                *mut *mut krb5_authdata;
                                                                        old_starttime
                                                                            =
                                                                            if enc_tkt_reply.times.starttime
                                                                                   !=
                                                                                   0
                                                                               {
                                                                                enc_tkt_reply.times.starttime
                                                                            } else {
                                                                                enc_tkt_reply.times.authtime
                                                                            };
                                                                        old_life
                                                                            =
                                                                            ts_delta(enc_tkt_reply.times.endtime,
                                                                                     old_starttime);
                                                                        enc_tkt_reply.times.starttime
                                                                            =
                                                                            kdc_time;
                                                                        enc_tkt_reply.times.endtime
                                                                            =
                                                                            if ts_after((*(*header_ticket).enc_part2).times.renew_till,
                                                                                        ts_incr(kdc_time,
                                                                                                old_life))
                                                                                   !=
                                                                                   0
                                                                               {
                                                                                ts_incr(kdc_time,
                                                                                        old_life)
                                                                            } else {
                                                                                (*(*header_ticket).enc_part2).times.renew_till
                                                                            }
                                                                    } else {
                                                                        /* not a renew request */
                                                                        enc_tkt_reply.times.starttime
                                                                            =
                                                                            kdc_time;
                                                                        kdc_get_ticket_endtime(kdc_active_realm,
                                                                                               enc_tkt_reply.times.starttime,
                                                                                               (*header_enc_tkt).times.endtime,
                                                                                               (*request).till,
                                                                                               client,
                                                                                               server,
                                                                                               &mut enc_tkt_reply.times.endtime);
                                                                    }
                                                                    kdc_get_ticket_renewtime(kdc_active_realm,
                                                                                             request,
                                                                                             header_enc_tkt,
                                                                                             client,
                                                                                             server,
                                                                                             &mut enc_tkt_reply);
                                                                    errcode =
                                                                        check_kdcpolicy_tgs((*kdc_active_realm).realm_context,
                                                                                            request,
                                                                                            server,
                                                                                            header_ticket,
                                                                                            auth_indicators,
                                                                                            kdc_time,
                                                                                            &mut enc_tkt_reply.times,
                                                                                            &mut status);
                                                                    if !(errcode
                                                                             !=
                                                                             0)
                                                                       {
                                                                        /*
     * Set authtime to be the same as header or evidence ticket's
     */
                                                                        enc_tkt_reply.times.authtime
                                                                            =
                                                                            authtime;
                                                                        /* starttime is optional, and treated as authtime if not present.
       so we can nuke it if it matches */
                                                                        if enc_tkt_reply.times.starttime
                                                                               ==
                                                                               enc_tkt_reply.times.authtime
                                                                           {
                                                                            enc_tkt_reply.times.starttime
                                                                                =
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                        }
                                                                        if c_flags
                                                                               &
                                                                               0x100
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                               !=
                                                                               0
                                                                           {
                                                                            altcprinc
                                                                                =
                                                                                (*s4u_x509_user).user_id.user
                                                                        } else if c_flags
                                                                                      &
                                                                                      0x200
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint
                                                                                      !=
                                                                                      0
                                                                         {
                                                                            /* kdc_process_s4u2proxy_req() only allows cross-realm requests if
         * stkt_authdata_client is set. */
                                                                            altcprinc
                                                                                =
                                                                                if c_flags
                                                                                       &
                                                                                       0x1000
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    stkt_authdata_client
                                                                                } else {
                                                                                    (*subject_tkt).client
                                                                                }
                                                                        } else {
                                                                            altcprinc
                                                                                =
                                                                                0
                                                                                    as
                                                                                    krb5_principal
                                                                        }
                                                                        if (*request).kdc_options
                                                                               &
                                                                               0x8
                                                                                   as
                                                                                   libc::c_int
                                                                               !=
                                                                               0
                                                                           {
                                                                            let mut t2enc:
                                                                                    *mut krb5_enc_tkt_part =
                                                                                (**(*request).second_ticket.offset(st_idx
                                                                                                                       as
                                                                                                                       isize)).enc_part2;
                                                                            encrypting_key
                                                                                =
                                                                                (*t2enc).session;
                                                                            current_block
                                                                                =
                                                                                4466262843398566590;
                                                                        } else {
                                                                            /*
         * Find the server key
         */
                                                                            errcode
                                                                                =
                                                                                krb5_dbe_find_enctype((*kdc_active_realm).realm_context,
                                                                                                      server,
                                                                                                      -(1
                                                                                                            as
                                                                                                            libc::c_int),
                                                                                                      -(1
                                                                                                            as
                                                                                                            libc::c_int),
                                                                                                      0
                                                                                                          as
                                                                                                          libc::c_int,
                                                                                                      &mut server_key);
                                                                            if errcode
                                                                                   !=
                                                                                   0
                                                                               {
                                                                                status
                                                                                    =
                                                                                    b"FINDING_SERVER_KEY\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char;
                                                                                current_block
                                                                                    =
                                                                                    1132313830116969717;
                                                                            } else {
                                                                                /*
         * Convert server.key into a real key
         * (it may be encrypted in the database)
         */
                                                                                errcode
                                                                                    =
                                                                                    krb5_dbe_decrypt_key_data((*kdc_active_realm).realm_context,
                                                                                                              0
                                                                                                                  as
                                                                                                                  *const krb5_keyblock,
                                                                                                              server_key,
                                                                                                              &mut server_keyblock,
                                                                                                              0
                                                                                                                  as
                                                                                                                  *mut krb5_keysalt);
                                                                                if errcode
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    status
                                                                                        =
                                                                                        b"DECRYPT_SERVER_KEY\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char;
                                                                                    current_block
                                                                                        =
                                                                                        1132313830116969717;
                                                                                } else {
                                                                                    encrypting_key
                                                                                        =
                                                                                        &mut server_keyblock;
                                                                                    current_block
                                                                                        =
                                                                                        4466262843398566590;
                                                                                }
                                                                            }
                                                                        }
                                                                        match current_block
                                                                            {
                                                                            1132313830116969717
                                                                            =>
                                                                            {
                                                                            }
                                                                            _
                                                                            =>
                                                                            {
                                                                                if c_flags
                                                                                       &
                                                                                       0x200
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    /*
         * Don't allow authorization data to be disabled if constrained
         * delegation is requested. We don't want to deny the server
         * the ability to validate that delegation was used.
         */
                                                                                    (*server).attributes
                                                                                        &=
                                                                                        !(0x400000
                                                                                              as
                                                                                              libc::c_int)
                                                                                }
                                                                                if (*server).attributes
                                                                                       &
                                                                                       0x400000
                                                                                           as
                                                                                           libc::c_int
                                                                                       ==
                                                                                       0
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    /* If we are not doing protocol transition, try to look up the subject
         * principal so that KDB modules can add additional authdata. */
                                                                                    if c_flags
                                                                                           &
                                                                                           0x100
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint
                                                                                           ==
                                                                                           0
                                                                                       {
                                                                                        /* Generate authorization data so we can include it in ticket */
                                                                                        c_flags
                                                                                            |=
                                                                                            0x20
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint;
                                                                                        /* Map principals from foreign (possibly non-AD) realms */
                                                                                        c_flags
                                                                                            |=
                                                                                            0x80
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint; /* should not have been set already */
                                                                                        if client.is_null()
                                                                                           {
                                                                                        } else {
                                                                                            __assert_fail(b"client == NULL\x00"
                                                                                                              as
                                                                                                              *const u8
                                                                                                              as
                                                                                                              *const libc::c_char,
                                                                                                          b"do_tgs_req.c\x00"
                                                                                                              as
                                                                                                              *const u8
                                                                                                              as
                                                                                                              *const libc::c_char,
                                                                                                          562
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint,
                                                                                                          (*::std::mem::transmute::<&[u8; 113],
                                                                                                                                    &[libc::c_char; 113]>(b"krb5_error_code process_tgs_req(krb5_kdc_req *, krb5_data *, const krb5_fulladdr *, kdc_realm_t *, krb5_data **)\x00")).as_ptr()); /* equivalent of "" */
                                                                                        }
                                                                                        errcode
                                                                                            =
                                                                                            krb5_db_get_principal((*kdc_active_realm).realm_context,
                                                                                                                  (*subject_tkt).client
                                                                                                                      as
                                                                                                                      krb5_const_principal,
                                                                                                                  c_flags,
                                                                                                                  &mut client)
                                                                                    }
                                                                                }
                                                                                if c_flags
                                                                                       &
                                                                                       (0x100
                                                                                            as
                                                                                            libc::c_int
                                                                                            |
                                                                                            0x200
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           libc::c_uint
                                                                                       !=
                                                                                       0
                                                                                       &&
                                                                                       is_referral
                                                                                           ==
                                                                                           0
                                                                                   {
                                                                                    enc_tkt_reply.client
                                                                                        =
                                                                                        altcprinc
                                                                                } else {
                                                                                    enc_tkt_reply.client
                                                                                        =
                                                                                        (*header_enc_tkt).client
                                                                                }
                                                                                enc_tkt_reply.session
                                                                                    =
                                                                                    &mut session_key;
                                                                                enc_tkt_reply.transited.tr_type
                                                                                    =
                                                                                    1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        krb5_octet;
                                                                                enc_tkt_reply.transited.tr_contents
                                                                                    =
                                                                                    empty_string;
                                                                                /*
     * Only add the realm of the presented tgt to the transited list if
     * it is different than the local realm (cross-realm) and it is different
     * than the realm of the client (since the realm of the client is already
     * implicitly part of the transited list and should not be explicitly
     * listed).
     */
    /* realm compare is like strcmp, but knows how to deal with these args */
                                                                                if krb5_realm_compare((*kdc_active_realm).realm_context,
                                                                                                      (*header_ticket).server
                                                                                                          as
                                                                                                          krb5_const_principal,
                                                                                                      (*kdc_active_realm).realm_tgsprinc
                                                                                                          as
                                                                                                          krb5_const_principal)
                                                                                       !=
                                                                                       0
                                                                                       ||
                                                                                       krb5_realm_compare((*kdc_active_realm).realm_context,
                                                                                                          (*header_ticket).server
                                                                                                              as
                                                                                                              krb5_const_principal,
                                                                                                          enc_tkt_reply.client
                                                                                                              as
                                                                                                              krb5_const_principal)
                                                                                           !=
                                                                                           0
                                                                                   {
                                                                                    /* tgt issued by local realm or issued by realm of client */
                                                                                    enc_tkt_reply.transited
                                                                                        =
                                                                                        (*header_enc_tkt).transited;
                                                                                    current_block
                                                                                        =
                                                                                        15905385608785565022;
                                                                                } else if (*header_enc_tkt).transited.tr_type
                                                                                              as
                                                                                              libc::c_int
                                                                                              !=
                                                                                              1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                 {
                                                                                    status
                                                                                        =
                                                                                        b"VALIDATE_TRANSIT_TYPE\x00"
                                                                                            as
                                                                                            *const u8
                                                                                            as
                                                                                            *const libc::c_char;
                                                                                    errcode
                                                                                        =
                                                                                        -(1765328367
                                                                                              as
                                                                                              libc::c_long)
                                                                                            as
                                                                                            libc::c_int;
                                                                                    current_block
                                                                                        =
                                                                                        1132313830116969717;
                                                                                } else {
                                                                                    memset(&mut enc_tkt_reply.transited
                                                                                               as
                                                                                               *mut krb5_transited
                                                                                               as
                                                                                               *mut libc::c_void,
                                                                                           0
                                                                                               as
                                                                                               libc::c_int,
                                                                                           ::std::mem::size_of::<krb5_transited>()
                                                                                               as
                                                                                               libc::c_ulong);
                                                                                    enc_tkt_reply.transited.tr_type
                                                                                        =
                                                                                        1
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            krb5_octet;
                                                                                    errcode
                                                                                        =
                                                                                        add_to_transited(&mut (*header_enc_tkt).transited.tr_contents,
                                                                                                         &mut enc_tkt_reply.transited.tr_contents,
                                                                                                         (*header_ticket).server,
                                                                                                         enc_tkt_reply.client,
                                                                                                         (*request).server);
                                                                                    if errcode
                                                                                           !=
                                                                                           0
                                                                                       {
                                                                                        status
                                                                                            =
                                                                                            b"ADD_TO_TRANSITED_LIST\x00"
                                                                                                as
                                                                                                *const u8
                                                                                                as
                                                                                                *const libc::c_char;
                                                                                        current_block
                                                                                            =
                                                                                            1132313830116969717;
                                                                                    } else {
                                                                                        newtransited
                                                                                            =
                                                                                            1
                                                                                                as
                                                                                                libc::c_int;
                                                                                        current_block
                                                                                            =
                                                                                            15905385608785565022;
                                                                                    }
                                                                                }
                                                                                match current_block
                                                                                    {
                                                                                    1132313830116969717
                                                                                    =>
                                                                                    {
                                                                                    }
                                                                                    _
                                                                                    =>
                                                                                    {
                                                                                        if (*request).kdc_options
                                                                                               &
                                                                                               0x20
                                                                                                   as
                                                                                                   libc::c_int
                                                                                               ==
                                                                                               0
                                                                                           {
                                                                                            errcode
                                                                                                =
                                                                                                kdc_check_transited_list(kdc_active_realm,
                                                                                                                         &mut enc_tkt_reply.transited.tr_contents,
                                                                                                                         &mut (*(*header_enc_tkt).client).realm,
                                                                                                                         &mut (*(*request).server).realm);
                                                                                            if errcode
                                                                                                   ==
                                                                                                   0
                                                                                                       as
                                                                                                       libc::c_int
                                                                                               {
                                                                                                enc_tkt_reply.flags
                                                                                                    |=
                                                                                                    0x80000
                                                                                                        as
                                                                                                        libc::c_int
                                                                                            } else {
                                                                                                log_tgs_badtrans((*kdc_active_realm).realm_context,
                                                                                                                 cprinc,
                                                                                                                 sprinc,
                                                                                                                 &mut enc_tkt_reply.transited.tr_contents,
                                                                                                                 errcode);
                                                                                            }
                                                                                        } else {
                                                                                            krb5_klog_syslog(6
                                                                                                                 as
                                                                                                                 libc::c_int,
                                                                                                             dgettext(b"mit-krb5\x00"
                                                                                                                          as
                                                                                                                          *const u8
                                                                                                                          as
                                                                                                                          *const libc::c_char,
                                                                                                                      b"not checking transit path\x00"
                                                                                                                          as
                                                                                                                          *const u8
                                                                                                                          as
                                                                                                                          *const libc::c_char));
                                                                                        }
                                                                                        if (*kdc_active_realm).realm_reject_bad_transit
                                                                                               !=
                                                                                               0
                                                                                               &&
                                                                                               enc_tkt_reply.flags
                                                                                                   &
                                                                                                   0x80000
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                   ==
                                                                                                   0
                                                                                           {
                                                                                            errcode
                                                                                                =
                                                                                                -(1765328372
                                                                                                      as
                                                                                                      libc::c_long)
                                                                                                    as
                                                                                                    libc::c_int;
                                                                                            status
                                                                                                =
                                                                                                b"BAD_TRANSIT\x00"
                                                                                                    as
                                                                                                    *const u8
                                                                                                    as
                                                                                                    *const libc::c_char;
                                                                                            (*au_state).violation
                                                                                                =
                                                                                                2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                        } else {
                                                                                            errcode
                                                                                                =
                                                                                                handle_authdata((*kdc_active_realm).realm_context,
                                                                                                                c_flags,
                                                                                                                client,
                                                                                                                server,
                                                                                                                subject_server,
                                                                                                                local_tgt,
                                                                                                                &mut local_tgt_key,
                                                                                                                if !subkey.is_null()
                                                                                                                   {
                                                                                                                    subkey
                                                                                                                } else {
                                                                                                                    (*(*header_ticket).enc_part2).session
                                                                                                                },
                                                                                                                encrypting_key,
                                                                                                                subject_key,
                                                                                                                pkt,
                                                                                                                request,
                                                                                                                altcprinc
                                                                                                                    as
                                                                                                                    krb5_const_principal,
                                                                                                                if !stkt_ad_info.is_null()
                                                                                                                   {
                                                                                                                    stkt_ad_info
                                                                                                                } else {
                                                                                                                    ad_info
                                                                                                                },
                                                                                                                subject_tkt,
                                                                                                                &mut auth_indicators,
                                                                                                                &mut enc_tkt_reply);
                                                                                            if errcode
                                                                                                   !=
                                                                                                   0
                                                                                               {
                                                                                                krb5_klog_syslog(6
                                                                                                                     as
                                                                                                                     libc::c_int,
                                                                                                                 dgettext(b"mit-krb5\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const libc::c_char,
                                                                                                                          b"TGS_REQ : handle_authdata (%d)\x00"
                                                                                                                              as
                                                                                                                              *const u8
                                                                                                                              as
                                                                                                                              *const libc::c_char),
                                                                                                                 errcode);
                                                                                                status
                                                                                                    =
                                                                                                    b"HANDLE_AUTHDATA\x00"
                                                                                                        as
                                                                                                        *const u8
                                                                                                        as
                                                                                                        *const libc::c_char
                                                                                            } else {
                                                                                                ticket_reply.enc_part2
                                                                                                    =
                                                                                                    &mut enc_tkt_reply;
                                                                                                /* tgt issued by some other realm and not the realm of the client */
        /* assemble new transited field into allocated storage */
                                                                                                /*
     * If we are doing user-to-user authentication, then make sure
     * that the client for the second ticket matches the request
     * server, and then encrypt the ticket using the session key of
     * the second ticket.
     */
                                                                                                if (*request).kdc_options
                                                                                                       &
                                                                                                       0x8
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                       !=
                                                                                                       0
                                                                                                   {
                                                                                                    /*
         * Make sure the client for the second ticket matches
         * requested server.
         */
                                                                                                    let mut t2enc_0:
                                                                                                            *mut krb5_enc_tkt_part =
                                                                                                        (**(*request).second_ticket.offset(st_idx
                                                                                                                                               as
                                                                                                                                               isize)).enc_part2;
                                                                                                    let mut client2:
                                                                                                            krb5_principal =
                                                                                                        (*t2enc_0).client;
                                                                                                    if krb5_principal_compare((*kdc_active_realm).realm_context,
                                                                                                                              (*request).server
                                                                                                                                  as
                                                                                                                                  krb5_const_principal,
                                                                                                                              client2
                                                                                                                                  as
                                                                                                                                  krb5_const_principal)
                                                                                                           ==
                                                                                                           0
                                                                                                       {
                                                                                                        altcprinc
                                                                                                            =
                                                                                                            client2;
                                                                                                        errcode
                                                                                                            =
                                                                                                            -(1765328358
                                                                                                                  as
                                                                                                                  libc::c_long)
                                                                                                                as
                                                                                                                libc::c_int;
                                                                                                        status
                                                                                                            =
                                                                                                            b"2ND_TKT_MISMATCH\x00"
                                                                                                                as
                                                                                                                *const u8
                                                                                                                as
                                                                                                                *const libc::c_char;
                                                                                                        (*au_state).status
                                                                                                            =
                                                                                                            status;
                                                                                                        kau_u2u((*kdc_active_realm).realm_context,
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    krb5_boolean,
                                                                                                                au_state);
                                                                                                        current_block
                                                                                                            =
                                                                                                            1132313830116969717;
                                                                                                    } else {
                                                                                                        ticket_kvno
                                                                                                            =
                                                                                                            0
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                krb5_kvno;
                                                                                                        ticket_reply.enc_part.enctype
                                                                                                            =
                                                                                                            (*(*t2enc_0).session).enctype;
                                                                                                        kau_u2u((*kdc_active_realm).realm_context,
                                                                                                                1
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    krb5_boolean,
                                                                                                                au_state);
                                                                                                        st_idx
                                                                                                            +=
                                                                                                            1;
                                                                                                        current_block
                                                                                                            =
                                                                                                            15455430299222214173;
                                                                                                    }
                                                                                                } else {
                                                                                                    ticket_kvno
                                                                                                        =
                                                                                                        (*server_key).key_data_kvno
                                                                                                            as
                                                                                                            krb5_kvno;
                                                                                                    current_block
                                                                                                        =
                                                                                                        15455430299222214173;
                                                                                                }
                                                                                                match current_block
                                                                                                    {
                                                                                                    1132313830116969717
                                                                                                    =>
                                                                                                    {
                                                                                                    }
                                                                                                    _
                                                                                                    =>
                                                                                                    {
                                                                                                        errcode
                                                                                                            =
                                                                                                            krb5_encrypt_tkt_part((*kdc_active_realm).realm_context,
                                                                                                                                  encrypting_key,
                                                                                                                                  &mut ticket_reply);
                                                                                                        if !(errcode
                                                                                                                 !=
                                                                                                                 0)
                                                                                                           {
                                                                                                            ticket_reply.enc_part.kvno
                                                                                                                =
                                                                                                                ticket_kvno;
                                                                                                            /* Start assembling the response */
                                                                                                            (*au_state).stage
                                                                                                                =
                                                                                                                5
                                                                                                                    as
                                                                                                                    libc::c_int; /* We are using the session key */
                                                                                                            reply.msg_type
                                                                                                                =
                                                                                                                13
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    krb5_msgtype;
                                                                                                            if c_flags
                                                                                                                   &
                                                                                                                   0x100
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint
                                                                                                                   !=
                                                                                                                   0
                                                                                                                   &&
                                                                                                                   !krb5int_find_pa_data((*kdc_active_realm).realm_context,
                                                                                                                                         (*request).padata,
                                                                                                                                         130
                                                                                                                                             as
                                                                                                                                             libc::c_int).is_null()
                                                                                                               {
                                                                                                                errcode
                                                                                                                    =
                                                                                                                    kdc_make_s4u2self_rep((*kdc_active_realm).realm_context,
                                                                                                                                          subkey,
                                                                                                                                          (*(*header_ticket).enc_part2).session,
                                                                                                                                          s4u_x509_user,
                                                                                                                                          &mut reply,
                                                                                                                                          &mut reply_encpart);
                                                                                                                if errcode
                                                                                                                       !=
                                                                                                                       0
                                                                                                                   {
                                                                                                                    (*au_state).status
                                                                                                                        =
                                                                                                                        status
                                                                                                                }
                                                                                                                kau_s4u2self((*kdc_active_realm).realm_context,
                                                                                                                             if errcode
                                                                                                                                    !=
                                                                                                                                    0
                                                                                                                                {
                                                                                                                                 0
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                             } else {
                                                                                                                                 1
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                             }
                                                                                                                                 as
                                                                                                                                 krb5_boolean,
                                                                                                                             au_state);
                                                                                                                if errcode
                                                                                                                       !=
                                                                                                                       0
                                                                                                                   {
                                                                                                                    current_block
                                                                                                                        =
                                                                                                                        1132313830116969717;
                                                                                                                } else {
                                                                                                                    current_block
                                                                                                                        =
                                                                                                                        10337218291927598571;
                                                                                                                }
                                                                                                            } else {
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    10337218291927598571;
                                                                                                            }
                                                                                                            match current_block
                                                                                                                {
                                                                                                                1132313830116969717
                                                                                                                =>
                                                                                                                {
                                                                                                                }
                                                                                                                _
                                                                                                                =>
                                                                                                                {
                                                                                                                    reply.client
                                                                                                                        =
                                                                                                                        enc_tkt_reply.client;
                                                                                                                    reply.enc_part.kvno
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            krb5_kvno;
                                                                                                                    reply.ticket
                                                                                                                        =
                                                                                                                        &mut ticket_reply;
                                                                                                                    reply_encpart.session
                                                                                                                        =
                                                                                                                        &mut session_key;
                                                                                                                    reply_encpart.nonce
                                                                                                                        =
                                                                                                                        (*request).nonce;
                                                                                                                    /* copy the time fields */
                                                                                                                    reply_encpart.times
                                                                                                                        =
                                                                                                                        enc_tkt_reply.times; /* not available for TGS reqs */
                                                                                                                    nolrentry.lr_type
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int; /* ditto */
                                                                                                                    nolrentry.value
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int;
                                                                                                                    nolrentry.magic
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int;
                                                                                                                    nolrarray[0
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  usize]
                                                                                                                        =
                                                                                                                        &mut nolrentry;
                                                                                                                    nolrarray[1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  usize]
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            *mut krb5_last_req_entry;
                                                                                                                    reply_encpart.last_req
                                                                                                                        =
                                                                                                                        nolrarray.as_mut_ptr();
                                                                                                                    reply_encpart.key_exp
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            libc::c_int;
                                                                                                                    reply_encpart.flags
                                                                                                                        =
                                                                                                                        enc_tkt_reply.flags;
                                                                                                                    reply_encpart.server
                                                                                                                        =
                                                                                                                        ticket_reply.server;
                                                                                                                    /* use the session key in the ticket, unless there's a subsession key
       in the AP_REQ */
                                                                                                                    reply.enc_part.enctype
                                                                                                                        =
                                                                                                                        if !subkey.is_null()
                                                                                                                           {
                                                                                                                            (*subkey).enctype
                                                                                                                        } else {
                                                                                                                            (*(*(*header_ticket).enc_part2).session).enctype
                                                                                                                        };
                                                                                                                    errcode
                                                                                                                        =
                                                                                                                        kdc_fast_response_handle_padata(state,
                                                                                                                                                        request,
                                                                                                                                                        &mut reply,
                                                                                                                                                        if !subkey.is_null()
                                                                                                                                                           {
                                                                                                                                                            (*subkey).enctype
                                                                                                                                                        } else {
                                                                                                                                                            (*(*(*header_ticket).enc_part2).session).enctype
                                                                                                                                                        });
                                                                                                                    if !(errcode
                                                                                                                             !=
                                                                                                                             0)
                                                                                                                       {
                                                                                                                        errcode
                                                                                                                            =
                                                                                                                            kdc_fast_handle_reply_key(state,
                                                                                                                                                      if !subkey.is_null()
                                                                                                                                                         {
                                                                                                                                                          subkey
                                                                                                                                                      } else {
                                                                                                                                                          (*(*header_ticket).enc_part2).session
                                                                                                                                                      },
                                                                                                                                                      &mut reply_key);
                                                                                                                        if !(errcode
                                                                                                                                 !=
                                                                                                                                 0)
                                                                                                                           {
                                                                                                                            errcode
                                                                                                                                =
                                                                                                                                return_enc_padata((*kdc_active_realm).realm_context,
                                                                                                                                                  pkt,
                                                                                                                                                  request,
                                                                                                                                                  reply_key,
                                                                                                                                                  server,
                                                                                                                                                  &mut reply_encpart,
                                                                                                                                                  (is_referral
                                                                                                                                                       !=
                                                                                                                                                       0
                                                                                                                                                       &&
                                                                                                                                                       s_flags
                                                                                                                                                           &
                                                                                                                                                           0x10
                                                                                                                                                               as
                                                                                                                                                               libc::c_int
                                                                                                                                                               as
                                                                                                                                                               libc::c_uint
                                                                                                                                                           !=
                                                                                                                                                           0)
                                                                                                                                                      as
                                                                                                                                                      libc::c_int
                                                                                                                                                      as
                                                                                                                                                      krb5_boolean);
                                                                                                                            if errcode
                                                                                                                                   !=
                                                                                                                                   0
                                                                                                                               {
                                                                                                                                status
                                                                                                                                    =
                                                                                                                                    b"KDC_RETURN_ENC_PADATA\x00"
                                                                                                                                        as
                                                                                                                                        *const u8
                                                                                                                                        as
                                                                                                                                        *const libc::c_char
                                                                                                                            } else {
                                                                                                                                errcode
                                                                                                                                    =
                                                                                                                                    kau_make_tkt_id((*kdc_active_realm).realm_context,
                                                                                                                                                    &mut ticket_reply,
                                                                                                                                                    &mut (*au_state).tkt_out_id);
                                                                                                                                if !(errcode
                                                                                                                                         !=
                                                                                                                                         0)
                                                                                                                                   {
                                                                                                                                    if kdc_fast_hide_client(state)
                                                                                                                                           !=
                                                                                                                                           0
                                                                                                                                       {
                                                                                                                                        reply.client
                                                                                                                                            =
                                                                                                                                            krb5_anonymous_principal()
                                                                                                                                                as
                                                                                                                                                krb5_principal
                                                                                                                                    }
                                                                                                                                    errcode
                                                                                                                                        =
                                                                                                                                        krb5_encode_kdc_rep((*kdc_active_realm).realm_context,
                                                                                                                                                            13
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                krb5_msgtype,
                                                                                                                                                            &mut reply_encpart,
                                                                                                                                                            if !subkey.is_null()
                                                                                                                                                               {
                                                                                                                                                                1
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                            } else {
                                                                                                                                                                0
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                            },
                                                                                                                                                            reply_key,
                                                                                                                                                            &mut reply,
                                                                                                                                                            response);
                                                                                                                                    if errcode
                                                                                                                                           ==
                                                                                                                                           0
                                                                                                                                       {
                                                                                                                                        status
                                                                                                                                            =
                                                                                                                                            b"ISSUE\x00"
                                                                                                                                                as
                                                                                                                                                *const u8
                                                                                                                                                as
                                                                                                                                                *const libc::c_char
                                                                                                                                    }
                                                                                                                                    memset(ticket_reply.enc_part.ciphertext.data
                                                                                                                                               as
                                                                                                                                               *mut libc::c_void,
                                                                                                                                           0
                                                                                                                                               as
                                                                                                                                               libc::c_int,
                                                                                                                                           ticket_reply.enc_part.ciphertext.length
                                                                                                                                               as
                                                                                                                                               libc::c_ulong);
                                                                                                                                    free(ticket_reply.enc_part.ciphertext.data
                                                                                                                                             as
                                                                                                                                             *mut libc::c_void);
                                                                                                                                    /* these parts are left on as a courtesy from krb5_encode_kdc_rep so we
       can use them in raw form if needed.  But, we don't... */
                                                                                                                                    memset(reply.enc_part.ciphertext.data
                                                                                                                                               as
                                                                                                                                               *mut libc::c_void,
                                                                                                                                           0
                                                                                                                                               as
                                                                                                                                               libc::c_int,
                                                                                                                                           reply.enc_part.ciphertext.length
                                                                                                                                               as
                                                                                                                                               libc::c_ulong);
                                                                                                                                    free(reply.enc_part.ciphertext.data
                                                                                                                                             as
                                                                                                                                             *mut libc::c_void);
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
    if status.is_null() {
        status = b"UNKNOWN_REASON\x00" as *const u8 as *const libc::c_char
    }
    krb5_free_keyblock_contents((*kdc_active_realm).realm_context,
                                &mut server_keyblock);
    if !reply_key.is_null() {
        krb5_free_keyblock((*kdc_active_realm).realm_context, reply_key);
    }
    if !stkt_server_key.is_null() {
        krb5_free_keyblock((*kdc_active_realm).realm_context,
                           stkt_server_key);
    }
    if errcode != 0 {
        emsg =
            krb5_get_error_message((*kdc_active_realm).realm_context, errcode)
    }
    (*au_state).status = status;
    if errcode == 0 { (*au_state).reply = &mut reply }
    kau_tgs_req((*kdc_active_realm).realm_context,
                if errcode != 0 { 0 as libc::c_int } else { 1 as libc::c_int }
                    as krb5_boolean, au_state);
    kau_free_kdc_req(au_state);
    log_tgs_req((*kdc_active_realm).realm_context, from, request, &mut reply,
                cprinc, sprinc, altcprinc, authtime, c_flags, status, errcode,
                emsg);
    if errcode != 0 {
        krb5_free_error_message((*kdc_active_realm).realm_context, emsg);
        emsg = 0 as *const libc::c_char
    }
    if errcode != 0 {
        let mut got_err: libc::c_int = 0 as libc::c_int;
        if status.is_null() {
            status =
                krb5_get_error_message((*kdc_active_realm).realm_context,
                                       errcode);
            got_err = 1 as libc::c_int
        }
        errcode =
            (errcode as libc::c_long - -(1765328384 as libc::c_long)) as
                libc::c_int;
        if errcode < 0 as libc::c_int || errcode > 127 as libc::c_int {
            errcode = 60 as libc::c_int
        }
        retval =
            prepare_error_tgs(state, request, header_ticket, errcode,
                              if !server.is_null() {
                                  (*server).princ
                              } else { 0 as krb5_principal }, response,
                              status, e_data);
        if got_err != 0 {
            krb5_free_error_message((*kdc_active_realm).realm_context,
                                    status);
            status = 0 as *const libc::c_char
        }
    }
    if !header_ticket.is_null() {
        krb5_free_ticket((*kdc_active_realm).realm_context, header_ticket);
    }
    if !request.is_null() {
        krb5_free_kdc_req((*kdc_active_realm).realm_context, request);
    }
    if !state.is_null() { kdc_free_rstate(state); }
    krb5_db_free_principal((*kdc_active_realm).realm_context, server);
    krb5_db_free_principal((*kdc_active_realm).realm_context, stkt_server);
    krb5_db_free_principal((*kdc_active_realm).realm_context, header_server);
    krb5_db_free_principal((*kdc_active_realm).realm_context, client);
    krb5_db_free_principal((*kdc_active_realm).realm_context,
                           local_tgt_storage);
    if !local_tgt_key.contents.is_null() {
        krb5_free_keyblock_contents((*kdc_active_realm).realm_context,
                                    &mut local_tgt_key);
    }
    if !session_key.contents.is_null() {
        krb5_free_keyblock_contents((*kdc_active_realm).realm_context,
                                    &mut session_key);
    }
    if newtransited != 0 {
        free(enc_tkt_reply.transited.tr_contents.data as *mut libc::c_void);
    }
    if !s4u_x509_user.is_null() {
        krb5_free_pa_s4u_x509_user((*kdc_active_realm).realm_context,
                                   s4u_x509_user);
    }
    if !kdc_issued_auth_data.is_null() {
        krb5_free_authdata((*kdc_active_realm).realm_context,
                           kdc_issued_auth_data);
    }
    if !subkey.is_null() {
        krb5_free_keyblock((*kdc_active_realm).realm_context, subkey);
    }
    if !header_key.is_null() {
        krb5_free_keyblock((*kdc_active_realm).realm_context, header_key);
    }
    if !reply.padata.is_null() {
        krb5_free_pa_data((*kdc_active_realm).realm_context, reply.padata);
    }
    if !reply_encpart.enc_padata.is_null() {
        krb5_free_pa_data((*kdc_active_realm).realm_context,
                          reply_encpart.enc_padata);
    }
    if !enc_tkt_reply.authorization_data.is_null() {
        krb5_free_authdata((*kdc_active_realm).realm_context,
                           enc_tkt_reply.authorization_data);
    }
    krb5_free_pa_data((*kdc_active_realm).realm_context, e_data);
    k5_free_data_ptr_list(auth_indicators);
    krb5_db_free_authdata_info((*kdc_active_realm).realm_context, ad_info);
    krb5_db_free_authdata_info((*kdc_active_realm).realm_context,
                               stkt_ad_info);
    krb5_free_principal((*kdc_active_realm).realm_context,
                        stkt_authdata_client);
    return retval;
}
#[c2rust::src_loc = "852:1"]
unsafe extern "C" fn prepare_error_tgs(mut state: *mut kdc_request_state,
                                       mut request: *mut krb5_kdc_req,
                                       mut ticket: *mut krb5_ticket,
                                       mut error: libc::c_int,
                                       mut canon_server: krb5_principal,
                                       mut response: *mut *mut krb5_data,
                                       mut status: *const libc::c_char,
                                       mut e_data: *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut errpkt: krb5_error =
        krb5_error{magic: 0,
                   ctime: 0,
                   cusec: 0,
                   susec: 0,
                   stime: 0,
                   error: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   text:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},};
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut scratch: *mut krb5_data = 0 as *mut krb5_data;
    let mut e_data_asn1: *mut krb5_data = 0 as *mut krb5_data;
    let mut fast_edata: *mut krb5_data = 0 as *mut krb5_data;
    let mut kdc_active_realm: *mut kdc_realm_t = (*state).realm_data;
    errpkt.magic = -(1760647402 as libc::c_long) as krb5_magic;
    errpkt.ctime = 0 as libc::c_int;
    errpkt.cusec = 0 as libc::c_int;
    retval =
        krb5_us_timeofday((*kdc_active_realm).realm_context,
                          &mut errpkt.stime, &mut errpkt.susec);
    if retval != 0 { return retval }
    errpkt.error = error as krb5_ui_4;
    errpkt.server = (*request).server;
    if !ticket.is_null() && !(*ticket).enc_part2.is_null() {
        errpkt.client = (*(*ticket).enc_part2).client
    } else { errpkt.client = 0 as krb5_principal }
    errpkt.text.length = strlen(status) as libc::c_uint;
    errpkt.text.data = strdup(status);
    if errpkt.text.data.is_null() { return 12 as libc::c_int }
    scratch =
        malloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if scratch.is_null() {
        free(errpkt.text.data as *mut libc::c_void);
        return 12 as libc::c_int
    }
    if !e_data.is_null() {
        retval = encode_krb5_padata_sequence(e_data, &mut e_data_asn1);
        if retval != 0 {
            free(scratch as *mut libc::c_void);
            free(errpkt.text.data as *mut libc::c_void);
            return retval
        }
        errpkt.e_data = *e_data_asn1
    } else { errpkt.e_data = empty_data() }
    retval =
        kdc_fast_handle_error((*kdc_active_realm).realm_context, state,
                              request, e_data, &mut errpkt, &mut fast_edata);
    if retval != 0 {
        free(scratch as *mut libc::c_void);
        free(errpkt.text.data as *mut libc::c_void);
        krb5_free_data((*kdc_active_realm).realm_context, e_data_asn1);
        return retval
    }
    if !fast_edata.is_null() { errpkt.e_data = *fast_edata }
    if kdc_fast_hide_client(state) != 0 && !errpkt.client.is_null() {
        errpkt.client = krb5_anonymous_principal() as krb5_principal
    }
    retval =
        krb5_mk_error((*kdc_active_realm).realm_context, &mut errpkt,
                      scratch);
    free(errpkt.text.data as *mut libc::c_void);
    krb5_free_data((*kdc_active_realm).realm_context, e_data_asn1);
    krb5_free_data((*kdc_active_realm).realm_context, fast_edata);
    if retval != 0 {
        free(scratch as *mut libc::c_void);
    } else { *response = scratch }
    return retval;
}
/*
 * Get the key for the second ticket, if any, and decrypt it.
 */
#[c2rust::src_loc = "926:1"]
unsafe extern "C" fn decrypt_2ndtkt(mut kdc_active_realm: *mut kdc_realm_t,
                                    mut req: *mut krb5_kdc_req,
                                    mut flags: krb5_flags,
                                    mut server_out: *mut *mut krb5_db_entry,
                                    mut key_out: *mut *mut krb5_keyblock,
                                    mut status: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut server: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut kvno: krb5_kvno = 0;
    let mut stkt: *mut krb5_ticket = 0 as *mut krb5_ticket;
    if (*req).kdc_options & (0x20000 as libc::c_int | 0x8 as libc::c_int) == 0
       {
        return 0 as libc::c_int
    }
    stkt = *(*req).second_ticket.offset(0 as libc::c_int as isize);
    retval =
        kdc_get_server_key((*kdc_active_realm).realm_context, stkt,
                           flags as libc::c_uint,
                           1 as libc::c_int as krb5_boolean, &mut server,
                           key_out, &mut kvno);
    if retval != 0 as libc::c_int {
        *status = b"2ND_TKT_SERVER\x00" as *const u8 as *const libc::c_char
    } else {
        retval =
            krb5_decrypt_tkt_part((*kdc_active_realm).realm_context, *key_out,
                                  *(*req).second_ticket.offset(0 as
                                                                   libc::c_int
                                                                   as isize));
        if retval != 0 as libc::c_int {
            *status =
                b"2ND_TKT_DECRYPT\x00" as *const u8 as *const libc::c_char
        } else { *server_out = server; server = 0 as *mut krb5_db_entry }
    }
    krb5_db_free_principal((*kdc_active_realm).realm_context, server);
    return retval;
}
#[c2rust::src_loc = "959:1"]
unsafe extern "C" fn get_2ndtkt_enctype(mut kdc_active_realm:
                                            *mut kdc_realm_t,
                                        mut req: *mut krb5_kdc_req,
                                        mut useenctype: *mut krb5_enctype,
                                        mut status: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut etype: krb5_enctype = 0;
    let mut stkt: *mut krb5_ticket =
        *(*req).second_ticket.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    etype = (*(*(*stkt).enc_part2).session).enctype;
    if krb5_c_valid_enctype(etype) == 0 {
        *status =
            b"BAD_ETYPE_IN_2ND_TKT\x00" as *const u8 as *const libc::c_char;
        return -(1765328370 as libc::c_long) as krb5_error_code
    }
    i = 0 as libc::c_int;
    while i < (*req).nktypes {
        if *(*req).ktype.offset(i as isize) == etype {
            *useenctype = etype;
            break ;
        } else { i += 1 }
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "981:1"]
unsafe extern "C" fn gen_session_key(mut kdc_active_realm: *mut kdc_realm_t,
                                     mut req: *mut krb5_kdc_req,
                                     mut server: *mut krb5_db_entry,
                                     mut skey: *mut krb5_keyblock,
                                     mut status: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut useenctype: krb5_enctype = 0 as libc::c_int;
    /*
     * Some special care needs to be taken in the user-to-user
     * case, since we don't know what keytypes the application server
     * which is doing user-to-user authentication can support.  We
     * know that it at least must be able to support the encryption
     * type of the session key in the TGT, since otherwise it won't be
     * able to decrypt the U2U ticket!  So we use that in preference
     * to anything else.
     */
    if (*req).kdc_options & 0x8 as libc::c_int != 0 {
        retval =
            get_2ndtkt_enctype(kdc_active_realm, req, &mut useenctype,
                               status);
        if retval != 0 as libc::c_int { return retval }
    }
    if useenctype == 0 as libc::c_int {
        useenctype =
            select_session_keytype(kdc_active_realm, server, (*req).nktypes,
                                   (*req).ktype)
    }
    if useenctype == 0 as libc::c_int {
        /* unsupported ktype */
        *status =
            b"BAD_ENCRYPTION_TYPE\x00" as *const u8 as *const libc::c_char;
        return -(1765328370 as libc::c_long) as krb5_error_code
    }
    return krb5_c_make_random_key((*kdc_active_realm).realm_context,
                                  useenctype, skey);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/do_tgs_req.c - KDC Routines to deal with TGS_REQ's */
/*
 * Copyright 1990, 1991, 2001, 2007, 2008, 2009, 2013, 2014 by the
 * Massachusetts Institute of Technology.  All Rights Reserved.
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
 * The request seems to be for a ticket-granting service somewhere else,
 * but we don't have a ticket for the final TGS.  Try to give the requestor
 * some intermediate realm.
 */
#[c2rust::src_loc = "1023:1"]
unsafe extern "C" fn find_alternate_tgs(mut kdc_active_realm:
                                            *mut kdc_realm_t,
                                        mut princ: krb5_principal,
                                        mut server_ptr:
                                            *mut *mut krb5_db_entry,
                                        mut status: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut plist: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut pl2: *mut krb5_principal = 0 as *mut krb5_principal;
    let mut tmp: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut server: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    *server_ptr = 0 as *mut krb5_db_entry;
    if is_cross_tgs_principal(princ as krb5_const_principal) != 0 {
    } else {
        __assert_fail(b"is_cross_tgs_principal(princ)\x00" as *const u8 as
                          *const libc::c_char,
                      b"do_tgs_req.c\x00" as *const u8 as *const libc::c_char,
                      1033 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 99],
                                                &[libc::c_char; 99]>(b"krb5_error_code find_alternate_tgs(kdc_realm_t *, krb5_principal, krb5_db_entry **, const char **)\x00")).as_ptr());
    }
    retval =
        krb5_walk_realm_tree((*kdc_active_realm).realm_context,
                             &mut (*princ).realm,
                             if (1 as libc::c_int) < (*princ).length {
                                 (*princ).data.offset(1 as libc::c_int as
                                                          isize)
                             } else { 0 as *mut krb5_data }, &mut plist,
                             '.' as i32);
    if !(retval != 0) {
        /* move to the end */
        pl2 = plist;
        while !(*pl2).is_null() { pl2 = pl2.offset(1) }
        loop 
             /* the first entry in this array is for krbtgt/local@local, so we
       ignore it */
             {
            pl2 = pl2.offset(-1);
            if !(pl2 > plist) { break ; }
            tmp = (**pl2).realm;
            (**pl2).realm = (*princ).realm;
            retval =
                db_get_svc_princ((*kdc_active_realm).realm_context, *pl2,
                                 0 as libc::c_int, &mut server, status);
            (**pl2).realm = tmp;
            if retval as libc::c_long == -(1780008443 as libc::c_long) {
                continue ;
            }
            if retval != 0 { break ; }
            log_tgs_alt_tgt((*kdc_active_realm).realm_context,
                            (*server).princ);
            *server_ptr = server;
            server = 0 as *mut krb5_db_entry;
            break ;
        }
    }
    if retval == 0 as libc::c_int && (*server_ptr).is_null() {
        retval = -(1780008443 as libc::c_long) as krb5_error_code
    }
    if retval != 0 as libc::c_int {
        *status = b"UNKNOWN_SERVER\x00" as *const u8 as *const libc::c_char
    }
    krb5_free_realm_tree((*kdc_active_realm).realm_context, plist);
    krb5_db_free_principal((*kdc_active_realm).realm_context, server);
    return retval;
}
/* Return true if item is an element of the space/comma-separated list. */
#[c2rust::src_loc = "1073:1"]
unsafe extern "C" fn in_list(mut list: *const libc::c_char,
                             mut item: *const libc::c_char) -> krb5_boolean {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = strlen(item) as libc::c_int;
    if list.is_null() { return 0 as libc::c_int as krb5_boolean }
    p = strstr(list, item);
    while !p.is_null() {
        if (p == list ||
                *(*__ctype_b_loc()).offset(*p.offset(-(1 as libc::c_int) as
                                                         isize) as
                                               libc::c_uchar as libc::c_int as
                                               isize) as libc::c_int &
                    _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                    != 0 ||
                *p.offset(-(1 as libc::c_int) as isize) as libc::c_int ==
                    ',' as i32) &&
               (*p.offset(len as isize) as libc::c_int == '\u{0}' as i32 ||
                    *(*__ctype_b_loc()).offset(*p.offset(len as isize) as
                                                   libc::c_uchar as
                                                   libc::c_int as isize) as
                        libc::c_int &
                        _ISspace as libc::c_int as libc::c_ushort as
                            libc::c_int != 0 ||
                    *p.offset(len as isize) as libc::c_int == ',' as i32) {
            return 1 as libc::c_int as krb5_boolean
        }
        p = strstr(p.offset(1 as libc::c_int as isize), item)
    }
    return 0 as libc::c_int as krb5_boolean;
}
/*
 * Check whether the request satisfies the conditions for generating a referral
 * TGT.  The caller checks whether the hostname component looks like a FQDN.
 */
#[c2rust::src_loc = "1094:1"]
unsafe extern "C" fn is_referral_req(mut kdc_active_realm: *mut kdc_realm_t,
                                     mut request: *mut krb5_kdc_req)
 -> krb5_boolean {
    let mut current_block: u64;
    let mut ret: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut stype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hostbased: *mut libc::c_char =
        (*kdc_active_realm).realm_hostbased;
    let mut no_referral: *mut libc::c_char =
        (*kdc_active_realm).realm_no_referral;
    if (*request).kdc_options & 0x10000 as libc::c_int == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*request).kdc_options & 0x8 as libc::c_int != 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*(*request).server).length != 2 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    stype =
        data2string(if (0 as libc::c_int) < (*(*request).server).length {
                        (*(*request).server).data.offset(0 as libc::c_int as
                                                             isize)
                    } else { 0 as *mut krb5_data });
    if stype.is_null() { return 0 as libc::c_int as krb5_boolean }
    match (*(*request).server).type_0 {
        0 => {
            /* Allow referrals for NT-UNKNOWN principals, if configured. */
            if in_list(hostbased, stype) == 0 &&
                   in_list(hostbased,
                           b"*\x00" as *const u8 as *const libc::c_char) == 0
               {
                current_block = 10426274076028679679;
            } else {
                /* FALLTHROUGH */
                current_block = 1736698841113242520;
            }
        }
        3 | 2 => { current_block = 1736698841113242520; }
        _ => { current_block = 10426274076028679679; }
    }
    match current_block {
        1736698841113242520 =>
        /* Deny referrals for specific service types, if configured. */
        {
            if !(in_list(no_referral, stype) != 0 ||
                     in_list(no_referral,
                             b"*\x00" as *const u8 as *const libc::c_char) !=
                         0) {
                ret = 1 as libc::c_int as krb5_boolean
            }
        }
        _ => { }
    }
    free(stype as *mut libc::c_void);
    return ret;
}
/*
 * Find a remote realm TGS principal for an unknown host-based service
 * principal.
 */
#[c2rust::src_loc = "1139:1"]
unsafe extern "C" fn find_referral_tgs(mut kdc_active_realm: *mut kdc_realm_t,
                                       mut request: *mut krb5_kdc_req,
                                       mut krbtgt_princ: *mut krb5_principal)
 -> krb5_int32 {
    let mut retval: krb5_error_code =
        -(1765328377 as libc::c_long) as krb5_error_code;
    let mut realms: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srealm: krb5_data = (*(*request).server).realm;
    if !(is_referral_req(kdc_active_realm, request) == 0) {
        hostname =
            data2string(if (1 as libc::c_int) < (*(*request).server).length {
                            (*(*request).server).data.offset(1 as libc::c_int
                                                                 as isize)
                        } else { 0 as *mut krb5_data });
        if hostname.is_null() {
            retval = 12 as libc::c_int
        } else if !strchr(hostname, '.' as i32).is_null() {
            retval =
                krb5_get_host_realm((*kdc_active_realm).realm_context,
                                    hostname, &mut realms);
            if retval != 0 {
                /* If the hostname doesn't contain a '.', it's not a FQDN. */
                /* no match found */
                kdc_err((*kdc_active_realm).realm_context,
                        retval as errcode_t,
                        b"unable to find realm of host\x00" as *const u8 as
                            *const libc::c_char);
            } else if realms.is_null() ||
                          (*realms.offset(0 as libc::c_int as
                                              isize)).is_null() ||
                          **realms.offset(0 as libc::c_int as isize) as
                              libc::c_int == '\u{0}' as i32 ||
                          data_eq_string(srealm,
                                         *realms.offset(0 as libc::c_int as
                                                            isize)) != 0 {
                retval = -(1765328377 as libc::c_long) as krb5_error_code
            } else {
                retval =
                    krb5_build_principal((*kdc_active_realm).realm_context,
                                         krbtgt_princ, srealm.length,
                                         srealm.data,
                                         b"krbtgt\x00" as *const u8 as
                                             *const libc::c_char,
                                         *realms.offset(0 as libc::c_int as
                                                            isize),
                                         0 as *mut libc::c_char)
            }
        }
    }
    krb5_free_host_realm((*kdc_active_realm).realm_context, realms);
    free(hostname as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "1181:1"]
unsafe extern "C" fn db_get_svc_princ(mut ctx: krb5_context,
                                      mut princ: krb5_principal,
                                      mut flags: krb5_flags,
                                      mut server: *mut *mut krb5_db_entry,
                                      mut status: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    ret =
        krb5_db_get_principal(ctx, princ as krb5_const_principal,
                              flags as libc::c_uint, server);
    if ret as libc::c_long == -(1780008424 as libc::c_long) {
        ret = -(1765328355 as libc::c_long) as krb5_error_code
    }
    if ret != 0 as libc::c_int {
        *status = b"LOOKING_UP_SERVER\x00" as *const u8 as *const libc::c_char
    }
    return ret;
}
#[c2rust::src_loc = "1197:1"]
unsafe extern "C" fn search_sprinc(mut kdc_active_realm: *mut kdc_realm_t,
                                   mut req: *mut krb5_kdc_req,
                                   mut flags: krb5_flags,
                                   mut server: *mut *mut krb5_db_entry,
                                   mut status: *mut *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut princ: krb5_principal = (*req).server;
    let mut reftgs: krb5_principal = 0 as krb5_principal;
    let mut allow_referral: krb5_boolean = 0;
    /* Don't return a referral to the empty realm or the service realm. */
    /* Do not allow referrals for u2u or ticket modification requests, because
     * the server is supposed to match an already-issued ticket. */
    allow_referral =
        ((*req).kdc_options &
             (0x20000000 as libc::c_int | 0x8000000 as libc::c_int |
                  0x2 as libc::c_int | 0x1 as libc::c_int |
                  0x8 as libc::c_int) == 0) as libc::c_int as krb5_boolean;
    if allow_referral == 0 { flags &= !(0x10 as libc::c_int) }
    ret =
        db_get_svc_princ((*kdc_active_realm).realm_context, princ, flags,
                         server, status);
    if !(ret == 0 as libc::c_int ||
             ret as libc::c_long != -(1780008443 as libc::c_long) ||
             allow_referral == 0) {
        if is_cross_tgs_principal((*req).server as krb5_const_principal) == 0
           {
            ret = find_referral_tgs(kdc_active_realm, req, &mut reftgs);
            if ret != 0 as libc::c_int {
                current_block = 4006400033293761279;
            } else {
                ret =
                    db_get_svc_princ((*kdc_active_realm).realm_context,
                                     reftgs, flags, server, status);
                if ret == 0 as libc::c_int ||
                       ret as libc::c_long != -(1780008443 as libc::c_long) {
                    current_block = 4006400033293761279;
                } else {
                    princ = reftgs;
                    current_block = 13586036798005543211;
                }
            }
        } else { current_block = 13586036798005543211; }
        match current_block {
            4006400033293761279 => { }
            _ => {
                ret =
                    find_alternate_tgs(kdc_active_realm, princ, server,
                                       status)
            }
        }
    }
    if ret != 0 as libc::c_int &&
           ret as libc::c_long != -(1765328355 as libc::c_long) {
        ret = -(1765328377 as libc::c_long) as krb5_error_code;
        if (*status).is_null() {
            *status =
                b"LOOKING_UP_SERVER\x00" as *const u8 as *const libc::c_char
        }
    }
    krb5_free_principal((*kdc_active_realm).realm_context, reftgs);
    return ret;
}
