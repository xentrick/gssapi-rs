use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:27"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:27"]
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
    #[c2rust::src_loc = "1993:16"]
    pub struct _krb5_authenticator {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub checksum: *mut krb5_checksum,
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* the unencrypted version */
/* *
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
    #[c2rust::src_loc = "1993:1"]
    pub type krb5_authenticator = _krb5_authenticator;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2031:16"]
    pub struct _krb5_last_req_entry {
        pub magic: krb5_magic,
        pub lr_type: krb5_int32,
        pub value: krb5_timestamp,
    }
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2134:16"]
    pub struct _krb5_ap_rep {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
    /* *
 * C representaton of AP-REP message.
 *
 * The server's response to a client's request for mutual authentication.
 */
    #[c2rust::src_loc = "2134:1"]
    pub type krb5_ap_rep = _krb5_ap_rep;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2140:16"]
    pub struct _krb5_ap_rep_enc_part {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
    }
    /* *< Ciphertext of ApRepEncPart */
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2158:16"]
    pub struct _krb5_cred_info {
        pub magic: krb5_magic,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub flags: krb5_flags,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
    }
    /* *< Client time, seconds portion */
    /* *< Client time, microseconds portion */
    /* *< Subkey (optional) */
    /* *< Sequence number */
    /* * Credentials information inserted into @c EncKrbCredPart. */
    #[c2rust::src_loc = "2158:1"]
    pub type krb5_cred_info = _krb5_cred_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2169:16"]
    pub struct _krb5_cred_enc_part {
        pub magic: krb5_magic,
        pub nonce: krb5_int32,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub ticket_info: *mut *mut krb5_cred_info,
    }
    /* *< Session key used to encrypt ticket */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Ticket flags */
    /* *< Auth, start, end, renew_till */
    /* *< Array of pointers to addrs (optional) */
    /* * Cleartext credentials information.  */
    #[c2rust::src_loc = "2169:1"]
    pub type krb5_cred_enc_part = _krb5_cred_enc_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2180:16"]
    pub struct _krb5_cred {
        pub magic: krb5_magic,
        pub tickets: *mut *mut krb5_ticket,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_cred_enc_part,
    }
    /* *< Nonce (optional) */
    /* *< Generation time, seconds portion */
    /* *< Generation time, microseconds portion */
    /* *< Sender address (optional) */
    /* *< Recipient address (optional) */
    /* * Credentials data structure.*/
    #[c2rust::src_loc = "2180:1"]
    pub type krb5_cred = _krb5_cred;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Tickets */
        /* *< Encrypted part */
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
        /* *
 * Set the realm field of a principal
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] realm            Realm name
 *
 * Set the realm name part of @a principal to @a realm, overwriting the
 * previous realm.
 *
 * @retval
 * 0   Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3586:1"]
        pub fn krb5_set_principal_realm(context: krb5_context,
                                        principal: krb5_principal,
                                        realm: *const libc::c_char)
         -> krb5_error_code;
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
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
    #[c2rust::src_loc = "515:1"]
    pub type krb5_otp_tokeninfo = _krb5_otp_tokeninfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "515:16"]
    pub struct _krb5_otp_tokeninfo {
        pub flags: krb5_flags,
        pub vendor: krb5_data,
        pub challenge: krb5_data,
        pub length: krb5_int32,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub supported_hash_alg: *mut *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
    }
    #[c2rust::src_loc = "798:1"]
    pub type krb5_fast_finished = _krb5_fast_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "798:16"]
    pub struct _krb5_fast_finished {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub client: krb5_principal,
        pub ticket_checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "843:1"]
    pub type krb5_verifier_mac = _krb5_verifier_mac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "843:16"]
    pub struct _krb5_verifier_mac {
        pub princ: krb5_principal,
        pub kvno: krb5_kvno,
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "420:1"]
    pub type krb5_etype_info_entry = _krb5_etype_info_entry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "420:16"]
    pub struct _krb5_etype_info_entry {
        pub magic: krb5_magic,
        pub etype: krb5_enctype,
        pub length: libc::c_uint,
        pub salt: *mut krb5_octet,
        pub s2kparams: krb5_data,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "460:16"]
    pub struct _krb5_sam_challenge_2 {
        pub sam_challenge_2_body: krb5_data,
        pub sam_cksum: *mut *mut krb5_checksum,
    }
    #[c2rust::src_loc = "460:1"]
    pub type krb5_sam_challenge_2 = _krb5_sam_challenge_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "465:16"]
    pub struct _krb5_sam_challenge_2_body {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_type_name: krb5_data,
        pub sam_track_id: krb5_data,
        pub sam_challenge_label: krb5_data,
        pub sam_challenge: krb5_data,
        pub sam_response_prompt: krb5_data,
        pub sam_pk_for_sad: krb5_data,
        pub sam_nonce: krb5_int32,
        pub sam_etype: krb5_enctype,
    }
    #[c2rust::src_loc = "465:1"]
    pub type krb5_sam_challenge_2_body = _krb5_sam_challenge_2_body;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "479:16"]
    pub struct _krb5_sam_response_2 {
        pub magic: krb5_magic,
        pub sam_type: krb5_int32,
        pub sam_flags: krb5_flags,
        pub sam_track_id: krb5_data,
        pub sam_enc_nonce_or_sad: krb5_enc_data,
        pub sam_nonce: krb5_int32,
    }
    #[c2rust::src_loc = "479:1"]
    pub type krb5_sam_response_2 = _krb5_sam_response_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "488:16"]
    pub struct _krb5_enc_sam_response_enc_2 {
        pub magic: krb5_magic,
        pub sam_nonce: krb5_int32,
        pub sam_sad: krb5_data,
    }
    #[c2rust::src_loc = "488:1"]
    pub type krb5_enc_sam_response_enc_2 = _krb5_enc_sam_response_enc_2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "527:16"]
    pub struct _krb5_pa_otp_challenge {
        pub nonce: krb5_data,
        pub service: krb5_data,
        pub tokeninfo: *mut *mut krb5_otp_tokeninfo,
        pub salt: krb5_data,
        pub s2kparams: krb5_data,
    }
    #[c2rust::src_loc = "527:1"]
    pub type krb5_pa_otp_challenge = _krb5_pa_otp_challenge;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "535:16"]
    pub struct _krb5_pa_otp_req {
        pub flags: krb5_int32,
        pub nonce: krb5_data,
        pub enc_data: krb5_enc_data,
        pub hash_alg: *mut krb5_algorithm_identifier,
        pub iteration_count: krb5_int32,
        pub otp_value: krb5_data,
        pub pin: krb5_data,
        pub challenge: krb5_data,
        pub time: krb5_timestamp,
        pub counter: krb5_data,
        pub format: krb5_int32,
        pub token_id: krb5_data,
        pub alg_id: krb5_data,
        pub vendor: krb5_data,
    }
    #[c2rust::src_loc = "535:1"]
    pub type krb5_pa_otp_req = _krb5_pa_otp_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "552:16"]
    pub struct _krb5_kkdcp_message {
        pub kerb_message: krb5_data,
        pub target_domain: krb5_data,
        pub dclocator_hint: krb5_int32,
    }
    #[c2rust::src_loc = "552:1"]
    pub type krb5_kkdcp_message = _krb5_kkdcp_message;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "559:16"]
    pub struct _krb5_secure_cookie {
        pub time: time_t,
        pub data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "559:1"]
    pub type krb5_secure_cookie = _krb5_secure_cookie;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "746:16"]
    pub struct _krb5_pa_enc_ts {
        pub patimestamp: krb5_timestamp,
        pub pausec: krb5_int32,
    }
    #[c2rust::src_loc = "746:1"]
    pub type krb5_pa_enc_ts = _krb5_pa_enc_ts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "751:16"]
    pub struct _krb5_pa_for_user {
        pub user: krb5_principal,
        pub cksum: krb5_checksum,
        pub auth_package: krb5_data,
    }
    #[c2rust::src_loc = "751:1"]
    pub type krb5_pa_for_user = _krb5_pa_for_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "757:16"]
    pub struct _krb5_s4u_userid {
        pub nonce: krb5_int32,
        pub user: krb5_principal,
        pub subject_cert: krb5_data,
        pub options: krb5_flags,
    }
    #[c2rust::src_loc = "757:1"]
    pub type krb5_s4u_userid = _krb5_s4u_userid;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "767:16"]
    pub struct _krb5_pa_s4u_x509_user {
        pub user_id: krb5_s4u_userid,
        pub cksum: krb5_checksum,
    }
    #[c2rust::src_loc = "767:1"]
    pub type krb5_pa_s4u_x509_user = _krb5_pa_s4u_x509_user;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "805:16"]
    pub struct _krb5_fast_response {
        pub magic: krb5_magic,
        pub padata: *mut *mut krb5_pa_data,
        pub strengthen_key: *mut krb5_keyblock,
        pub finished: *mut krb5_fast_finished,
        pub nonce: krb5_int32,
    }
    #[c2rust::src_loc = "805:1"]
    pub type krb5_fast_response = _krb5_fast_response;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "813:16"]
    pub struct _krb5_ad_kdcissued {
        pub ad_checksum: krb5_checksum,
        pub i_principal: krb5_principal,
        pub elements: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "813:1"]
    pub type krb5_ad_kdcissued = _krb5_ad_kdcissued;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "819:16"]
    pub struct _krb5_ad_signedpath_data {
        pub client: krb5_principal,
        pub authtime: krb5_timestamp,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "819:1"]
    pub type krb5_ad_signedpath_data = _krb5_ad_signedpath_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "827:16"]
    pub struct _krb5_ad_signedpath {
        pub enctype: krb5_enctype,
        pub checksum: krb5_checksum,
        pub delegated: *mut krb5_principal,
        pub method_data: *mut *mut krb5_pa_data,
    }
    #[c2rust::src_loc = "827:1"]
    pub type krb5_ad_signedpath = _krb5_ad_signedpath;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "834:16"]
    pub struct _krb5_iakerb_header {
        pub target_realm: krb5_data,
        pub cookie: *mut krb5_data,
    }
    #[c2rust::src_loc = "834:1"]
    pub type krb5_iakerb_header = _krb5_iakerb_header;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "839:16"]
    pub struct _krb5_iakerb_finished {
        pub checksum: krb5_checksum,
    }
    #[c2rust::src_loc = "839:1"]
    pub type krb5_iakerb_finished = _krb5_iakerb_finished;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "856:16"]
    pub struct _krb5_cammac {
        pub elements: *mut *mut krb5_authdata,
        pub kdc_verifier: *mut krb5_verifier_mac,
        pub svc_verifier: *mut krb5_verifier_mac,
        pub other_verifiers: *mut *mut krb5_verifier_mac,
    }
    #[c2rust::src_loc = "856:1"]
    pub type krb5_cammac = _krb5_cammac;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1281:16"]
    pub struct _krb5_safe {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
        pub checksum: *mut krb5_checksum,
    }
    #[c2rust::src_loc = "1281:1"]
    pub type krb5_safe = _krb5_safe;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1293:16"]
    pub struct _krb5_priv {
        pub magic: krb5_magic,
        pub enc_part: krb5_enc_data,
    }
    #[c2rust::src_loc = "1293:1"]
    pub type krb5_priv = _krb5_priv;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1298:16"]
    pub struct _krb5_priv_enc_part {
        pub magic: krb5_magic,
        pub user_data: krb5_data,
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq_number: krb5_ui_4,
        pub s_address: *mut krb5_address,
        pub r_address: *mut krb5_address,
    }
    #[c2rust::src_loc = "1298:1"]
    pub type krb5_priv_enc_part = _krb5_priv_enc_part;
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_timestamp, krb5_principal,
                        krb5_checksum, krb5_kvno, krb5_octet, krb5_enc_data,
                        krb5_pa_data, krb5_keyblock, krb5_authdata, krb5_ui_4,
                        krb5_address, krb5_kdc_req, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::k5_int_pkinit_h::krb5_algorithm_identifier;
    use super::time_t_h::time_t;
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
        #[c2rust::src_loc = "1405:1"]
        pub fn encode_krb5_as_req(rep: *const krb5_kdc_req,
                                  code: *mut *mut krb5_data)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:27"]
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
    /* (0..999999) */
    /* (0..4294967295) */
    /* AlgorithmIdentifier */
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    /* OID */
    /* Optional */
    /* SubjectPublicKeyInfo */
    #[c2rust::src_loc = "55:1"]
    pub type krb5_subject_pk_info = _krb5_subject_pk_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct _krb5_subject_pk_info {
        pub algorithm: krb5_algorithm_identifier,
        pub subjectPublicKey: krb5_data,
    }
    /* BIT STRING */
    /* * AuthPack from RFC 4556*/
    /* Optional */
    /* Optional */
    /* Optional */
    /* OIDs of KDFs; OPTIONAL */
    /* ExternalPrincipalIdentifier */
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
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
    /* Optional */
    /* Optional */
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
    /* SP80056A OtherInfo, for pkinit algorithm agility */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct _krb5_sp80056a_other_info {
        pub algorithm_identifier: krb5_algorithm_identifier,
        pub party_u_info: krb5_principal,
        pub party_v_info: krb5_principal,
        pub supp_pub_info: krb5_data,
    }
    #[c2rust::src_loc = "117:1"]
    pub type krb5_sp80056a_other_info = _krb5_sp80056a_other_info;
    /* PkinitSuppPubInfo, for pkinit algorithm agility */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:16"]
    pub struct _krb5_pkinit_supp_pub_info {
        pub enctype: krb5_enctype,
        pub as_req: krb5_data,
        pub pk_as_rep: krb5_data,
    }
    #[c2rust::src_loc = "125:1"]
    pub type krb5_pkinit_supp_pub_info = _krb5_pkinit_supp_pub_info;
    use super::krb5_h::{krb5_data, krb5_int32, krb5_timestamp, krb5_checksum,
                        krb5_keyblock, krb5_principal, krb5_enctype};
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-spake.h:27"]
pub mod k5_spake_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-spake.h - SPAKE preauth mech declarations */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
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
 * The SPAKE preauth mechanism allows long-term client keys to be used for
 * preauthentication without exposing them to offline dictionary attacks.  The
 * negotiated key can also be used for second-factor authentication.  This
 * header file declares structures and encoder/decoder functions for the
 * mechanism's padata messages.
 */
    /* SPAKESecondFactor is contained within a SPAKEChallenge, SPAKEResponse, or
 * EncryptedData message and contains a second-factor challenge or response. */
    #[c2rust::src_loc = "48:1"]
    pub type krb5_spake_factor = krb5_spake_factor_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:16"]
    pub struct krb5_spake_factor_st {
        pub type_0: int32_t,
        pub data: *mut krb5_data,
    }
    /* SPAKESupport is sent from the client to the KDC to indicate which group the
 * client supports. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct krb5_spake_support_st {
        pub ngroups: int32_t,
        pub groups: *mut int32_t,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_spake_support = krb5_spake_support_st;
    /* SPAKEChallenge is sent from the KDC to the client to communicate its group
 * selection, public value, and second-factor challenge options. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct krb5_spake_challenge_st {
        pub group: int32_t,
        pub pubkey: krb5_data,
        pub factors: *mut *mut krb5_spake_factor,
    }
    #[c2rust::src_loc = "62:1"]
    pub type krb5_spake_challenge = krb5_spake_challenge_st;
    /* SPAKEResponse is sent from the client to the KDC to communicate its public
 * value and encrypted second-factor response. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct krb5_spake_response_st {
        pub pubkey: krb5_data,
        pub factor: krb5_enc_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_spake_response = krb5_spake_response_st;
    #[c2rust::src_loc = "75:1"]
    pub type krb5_spake_msgtype = libc::c_int;
    #[c2rust::src_loc = "80:5"]
    pub const SPAKE_MSGTYPE_ENCDATA: krb5_spake_msgtype = 3;
    #[c2rust::src_loc = "79:5"]
    pub const SPAKE_MSGTYPE_RESPONSE: krb5_spake_msgtype = 2;
    #[c2rust::src_loc = "78:5"]
    pub const SPAKE_MSGTYPE_CHALLENGE: krb5_spake_msgtype = 1;
    #[c2rust::src_loc = "77:5"]
    pub const SPAKE_MSGTYPE_SUPPORT: krb5_spake_msgtype = 0;
    #[c2rust::src_loc = "76:5"]
    pub const SPAKE_MSGTYPE_UNKNOWN: krb5_spake_msgtype = -1;
    /* PA-SPAKE is a choice among the message types which can appear in a PA-SPAKE
 * padata element. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "85:16"]
    pub struct krb5_pa_spake_st {
        pub choice: krb5_spake_msgtype,
        pub u: krb5_spake_message_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "87:11"]
    pub union krb5_spake_message_choices {
        pub support: krb5_spake_support,
        pub challenge: krb5_spake_challenge,
        pub response: krb5_spake_response,
        pub encdata: krb5_enc_data,
    }
    #[c2rust::src_loc = "85:1"]
    pub type krb5_pa_spake = krb5_pa_spake_st;
    use super::stdint_intn_h::int32_t;
    use super::krb5_h::{krb5_data, krb5_enc_data};
    /* K5_SPAKE_H */
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/ktest.h:27"]
pub mod ktest_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "211:21"]
        pub static mut test_context: krb5_context;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/asn.1/utility.h:28"]
pub mod utility_h {
    use super::stddef_h::size_t;
    use super::krb5_h::krb5_data;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/utility.h */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
        /* Aborts on failure.  ealloc returns zero-filled memory. */
        #[no_mangle]
        #[c2rust::src_loc = "33:1"]
        pub fn ealloc(size: size_t) -> *mut libc::c_void;
        /* modifies  *s;
   effects   Instantiates *s with a string representation of the series
              of hex octets in *code.  (e.g. "02 02 00 7F")  If code==NULL,
              the string rep is "<NULL>".  If code is empty (it contains no
              data or has length <= 0), the string rep is "<EMPTY>".
             If *s is non-NULL, then its currently-allocated storage
              will be freed prior to the instantiation.
             Returns ENOMEM or the string rep cannot be created. */
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn krb5_data_parse(d: *mut krb5_data, s: *const libc::c_char);
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_msgtype, krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_cksumtype, krb5_authdatatype, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_authenticator, krb5_authenticator,
                       _krb5_last_req_entry, krb5_last_req_entry,
                       _krb5_pa_data, krb5_pa_data, _krb5_kdc_req,
                       krb5_kdc_req, _krb5_enc_kdc_rep_part,
                       krb5_enc_kdc_rep_part, _krb5_kdc_rep, krb5_kdc_rep,
                       _krb5_error, krb5_error, _krb5_ap_req, krb5_ap_req,
                       _krb5_ap_rep, krb5_ap_rep, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, _krb5_cred_info, krb5_cred_info,
                       _krb5_cred_enc_part, krb5_cred_enc_part, _krb5_cred,
                       krb5_cred, _profile_t, krb5_parse_name,
                       krb5_set_principal_realm, krb5_free_checksum_contents,
                       krb5_free_data, krb5_free_data_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_otp_tokeninfo,
                         _krb5_otp_tokeninfo, krb5_fast_finished,
                         _krb5_fast_finished, krb5_verifier_mac,
                         _krb5_verifier_mac, krb5_etype_info_entry,
                         _krb5_etype_info_entry, _krb5_sam_challenge_2,
                         krb5_sam_challenge_2, _krb5_sam_challenge_2_body,
                         krb5_sam_challenge_2_body, _krb5_sam_response_2,
                         krb5_sam_response_2, _krb5_enc_sam_response_enc_2,
                         krb5_enc_sam_response_enc_2, _krb5_pa_otp_challenge,
                         krb5_pa_otp_challenge, _krb5_pa_otp_req,
                         krb5_pa_otp_req, _krb5_kkdcp_message,
                         krb5_kkdcp_message, _krb5_secure_cookie,
                         krb5_secure_cookie, _krb5_pa_enc_ts, krb5_pa_enc_ts,
                         _krb5_pa_for_user, krb5_pa_for_user,
                         _krb5_s4u_userid, krb5_s4u_userid,
                         _krb5_pa_s4u_x509_user, krb5_pa_s4u_x509_user,
                         _krb5_fast_response, krb5_fast_response,
                         _krb5_ad_kdcissued, krb5_ad_kdcissued,
                         _krb5_ad_signedpath_data, krb5_ad_signedpath_data,
                         _krb5_ad_signedpath, krb5_ad_signedpath,
                         _krb5_iakerb_header, krb5_iakerb_header,
                         _krb5_iakerb_finished, krb5_iakerb_finished,
                         _krb5_cammac, krb5_cammac, _krb5_safe, krb5_safe,
                         _krb5_priv, krb5_priv, _krb5_priv_enc_part,
                         krb5_priv_enc_part, make_data, empty_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, encode_krb5_as_req};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_int_pkinit_h::{krb5_algorithm_identifier,
                                _krb5_algorithm_identifier,
                                krb5_subject_pk_info, _krb5_subject_pk_info,
                                krb5_external_principal_identifier,
                                _krb5_external_principal_identifier,
                                _krb5_pk_authenticator, krb5_pk_authenticator,
                                _krb5_auth_pack, krb5_auth_pack,
                                _krb5_pa_pk_as_req, krb5_pa_pk_as_req,
                                _krb5_dh_rep_info, krb5_dh_rep_info,
                                _krb5_kdc_dh_key_info, krb5_kdc_dh_key_info,
                                _krb5_reply_key_pack, krb5_reply_key_pack,
                                _krb5_pa_pk_as_rep, krb5_pa_pk_as_rep_choices,
                                krb5_pa_pk_as_rep_selection,
                                choice_pa_pk_as_rep_encKeyPack,
                                choice_pa_pk_as_rep_dhInfo,
                                choice_pa_pk_as_rep_UNKNOWN,
                                krb5_pa_pk_as_rep, _krb5_sp80056a_other_info,
                                krb5_sp80056a_other_info,
                                _krb5_pkinit_supp_pub_info,
                                krb5_pkinit_supp_pub_info};
pub use self::k5_spake_h::{krb5_spake_factor, krb5_spake_factor_st,
                           krb5_spake_support_st, krb5_spake_support,
                           krb5_spake_challenge_st, krb5_spake_challenge,
                           krb5_spake_response_st, krb5_spake_response,
                           krb5_spake_msgtype, SPAKE_MSGTYPE_ENCDATA,
                           SPAKE_MSGTYPE_RESPONSE, SPAKE_MSGTYPE_CHALLENGE,
                           SPAKE_MSGTYPE_SUPPORT, SPAKE_MSGTYPE_UNKNOWN,
                           krb5_pa_spake_st, krb5_spake_message_choices,
                           krb5_pa_spake};
use self::stdio_h::asprintf;
use self::stdlib_h::{abort, free, calloc};
use self::string_h::{strlen, memset, memcpy};
use self::ktest_h::test_context;
use self::utility_h::{ealloc, krb5_data_parse};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/ktest.c */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "31:7"]
pub static mut sample_principal_name: *mut libc::c_char =
    b"hftsai/extra@ATHENA.MIT.EDU\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn ktest_make_sample_authenticator(mut a:
                                                             *mut krb5_authenticator) {
    ktest_make_sample_principal(&mut (*a).client);
    (*a).checksum =
        ealloc(::std::mem::size_of::<krb5_checksum>() as libc::c_ulong) as
            *mut krb5_checksum;
    ktest_make_sample_checksum((*a).checksum);
    (*a).cusec = 123456 as libc::c_int;
    (*a).ctime = 771228197 as libc::c_int;
    (*a).subkey =
        ealloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    ktest_make_sample_keyblock((*a).subkey);
    (*a).seq_number = 17 as libc::c_int as krb5_ui_4;
    ktest_make_sample_authorization_data(&mut (*a).authorization_data);
}
#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn ktest_make_sample_principal(mut p:
                                                         *mut krb5_principal) {
    if krb5_parse_name(test_context, sample_principal_name, p) != 0 {
        abort();
    };
}
#[no_mangle]
#[c2rust::src_loc = "54:1"]
pub unsafe extern "C" fn ktest_make_sample_checksum(mut cs:
                                                        *mut krb5_checksum) {
    (*cs).checksum_type = 1 as libc::c_int;
    (*cs).length = 4 as libc::c_int as libc::c_uint;
    (*cs).contents = ealloc(4 as libc::c_int as size_t) as *mut krb5_octet;
    memcpy((*cs).contents as *mut libc::c_void,
           b"1234\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 4 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn ktest_make_sample_keyblock(mut kb:
                                                        *mut krb5_keyblock) {
    (*kb).magic = -(1760647421 as libc::c_long) as krb5_magic;
    (*kb).enctype = 1 as libc::c_int;
    (*kb).length = 8 as libc::c_int as libc::c_uint;
    (*kb).contents = ealloc(8 as libc::c_int as size_t) as *mut krb5_octet;
    memcpy((*kb).contents as *mut libc::c_void,
           b"12345678\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 8 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn ktest_make_sample_ticket(mut tkt: *mut krb5_ticket) {
    ktest_make_sample_principal(&mut (*tkt).server);
    ktest_make_sample_enc_data(&mut (*tkt).enc_part);
    (*tkt).enc_part2 = 0 as *mut krb5_enc_tkt_part;
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn ktest_make_sample_enc_data(mut ed:
                                                        *mut krb5_enc_data) {
    (*ed).kvno = 5 as libc::c_int as krb5_kvno;
    (*ed).enctype = 0 as libc::c_int;
    krb5_data_parse(&mut (*ed).ciphertext,
                    b"krbASN.1 test message\x00" as *const u8 as
                        *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "89:1"]
pub unsafe extern "C" fn ktest_make_sample_enc_tkt_part(mut etp:
                                                            *mut krb5_enc_tkt_part) {
    (*etp).flags = 0xfedcba98 as libc::c_uint as krb5_flags;
    (*etp).session =
        ealloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    ktest_make_sample_keyblock((*etp).session);
    ktest_make_sample_principal(&mut (*etp).client);
    ktest_make_sample_transited(&mut (*etp).transited);
    ktest_make_sample_ticket_times(&mut (*etp).times);
    ktest_make_sample_addresses(&mut (*etp).caddrs);
    ktest_make_sample_authorization_data(&mut (*etp).authorization_data);
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn ktest_make_sample_addresses(mut caddrs:
                                                         *mut *mut *mut krb5_address) {
    let mut i: libc::c_int = 0;
    *caddrs =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_address>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_address;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let ref mut fresh0 = *(*caddrs).offset(i as isize);
        *fresh0 =
            ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
                *mut krb5_address;
        ktest_make_sample_address(*(*caddrs).offset(i as isize));
        i += 1
    }
    let ref mut fresh1 = *(*caddrs).offset(2 as libc::c_int as isize);
    *fresh1 = 0 as *mut krb5_address;
}
#[no_mangle]
#[c2rust::src_loc = "115:1"]
pub unsafe extern "C" fn ktest_make_sample_authorization_data(mut ad:
                                                                  *mut *mut *mut krb5_authdata) {
    let mut i: libc::c_int = 0;
    *ad =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_authdata;
    i = 0 as libc::c_int;
    while i <= 1 as libc::c_int {
        let ref mut fresh2 = *(*ad).offset(i as isize);
        *fresh2 =
            ealloc(::std::mem::size_of::<krb5_authdata>() as libc::c_ulong) as
                *mut krb5_authdata;
        ktest_make_sample_authdata(*(*ad).offset(i as isize));
        i += 1
    }
    let ref mut fresh3 = *(*ad).offset(2 as libc::c_int as isize);
    *fresh3 = 0 as *mut krb5_authdata;
}
#[no_mangle]
#[c2rust::src_loc = "128:1"]
pub unsafe extern "C" fn ktest_make_sample_transited(mut t:
                                                         *mut krb5_transited) {
    (*t).tr_type = 1 as libc::c_int as krb5_octet;
    krb5_data_parse(&mut (*t).tr_contents,
                    b"EDU,MIT.,ATHENA.,WASHINGTON.EDU,CS.\x00" as *const u8 as
                        *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn ktest_make_sample_ticket_times(mut tt:
                                                            *mut krb5_ticket_times) {
    (*tt).authtime = 771228197 as libc::c_int;
    (*tt).starttime = 771228197 as libc::c_int;
    (*tt).endtime = 771228197 as libc::c_int;
    (*tt).renew_till = 771228197 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "144:1"]
pub unsafe extern "C" fn ktest_make_sample_address(mut a: *mut krb5_address) {
    (*a).addrtype = 0x2 as libc::c_int;
    (*a).length = 4 as libc::c_int as libc::c_uint;
    (*a).contents =
        ealloc((4 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_octet>()
                                                    as libc::c_ulong)) as
            *mut krb5_octet;
    *(*a).contents.offset(0 as libc::c_int as isize) =
        18 as libc::c_int as krb5_octet;
    *(*a).contents.offset(1 as libc::c_int as isize) =
        208 as libc::c_int as krb5_octet;
    *(*a).contents.offset(2 as libc::c_int as isize) =
        0 as libc::c_int as krb5_octet;
    *(*a).contents.offset(3 as libc::c_int as isize) =
        35 as libc::c_int as krb5_octet;
}
#[no_mangle]
#[c2rust::src_loc = "156:1"]
pub unsafe extern "C" fn ktest_make_sample_authdata(mut ad:
                                                        *mut krb5_authdata) {
    (*ad).ad_type = 1 as libc::c_int;
    (*ad).length = 6 as libc::c_int as libc::c_uint;
    (*ad).contents =
        ealloc((6 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_octet>()
                                                    as libc::c_ulong)) as
            *mut krb5_octet;
    memcpy((*ad).contents as *mut libc::c_void,
           b"foobar\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 6 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "165:1"]
pub unsafe extern "C" fn ktest_make_sample_enc_kdc_rep_part(mut ekr:
                                                                *mut krb5_enc_kdc_rep_part) {
    (*ekr).session =
        ealloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    ktest_make_sample_keyblock((*ekr).session);
    ktest_make_sample_last_req(&mut (*ekr).last_req);
    (*ekr).nonce = 42 as libc::c_int;
    (*ekr).key_exp = 771228197 as libc::c_int;
    (*ekr).flags = 0xfedcba98 as libc::c_uint as krb5_flags;
    (*ekr).times.authtime = 771228197 as libc::c_int;
    (*ekr).times.starttime = 771228197 as libc::c_int;
    (*ekr).times.endtime = 771228197 as libc::c_int;
    (*ekr).times.renew_till = 771228197 as libc::c_int;
    ktest_make_sample_principal(&mut (*ekr).server);
    ktest_make_sample_addresses(&mut (*ekr).caddrs);
}
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn ktest_make_sample_last_req(mut lr:
                                                        *mut *mut *mut krb5_last_req_entry) {
    let mut i: libc::c_int = 0;
    *lr =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_last_req_entry>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_last_req_entry;
    i = 0 as libc::c_int;
    while i <= 1 as libc::c_int {
        ktest_make_sample_last_req_entry(&mut *(*lr).offset(i as isize));
        i += 1
    }
    let ref mut fresh4 = *(*lr).offset(2 as libc::c_int as isize);
    *fresh4 = 0 as *mut krb5_last_req_entry;
}
#[no_mangle]
#[c2rust::src_loc = "193:1"]
pub unsafe extern "C" fn ktest_make_sample_last_req_entry(mut lre:
                                                              *mut *mut krb5_last_req_entry) {
    *lre =
        ealloc(::std::mem::size_of::<krb5_last_req_entry>() as libc::c_ulong)
            as *mut krb5_last_req_entry;
    (**lre).lr_type = -(5 as libc::c_int);
    (**lre).value = 771228197 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn ktest_make_sample_kdc_rep(mut kdcr:
                                                       *mut krb5_kdc_rep) {
    ktest_make_sample_pa_data_array(&mut (*kdcr).padata);
    ktest_make_sample_principal(&mut (*kdcr).client);
    (*kdcr).ticket =
        ealloc(::std::mem::size_of::<krb5_ticket>() as libc::c_ulong) as
            *mut krb5_ticket;
    ktest_make_sample_ticket((*kdcr).ticket);
    ktest_make_sample_enc_data(&mut (*kdcr).enc_part);
    (*kdcr).enc_part2 = 0 as *mut krb5_enc_kdc_rep_part;
}
#[no_mangle]
#[c2rust::src_loc = "212:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_data_array(mut pad:
                                                             *mut *mut *mut krb5_pa_data) {
    let mut i: libc::c_int = 0;
    *pad =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_pa_data>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_pa_data;
    i = 0 as libc::c_int;
    while i <= 1 as libc::c_int {
        let ref mut fresh5 = *(*pad).offset(i as isize);
        *fresh5 =
            ealloc(::std::mem::size_of::<krb5_pa_data>() as libc::c_ulong) as
                *mut krb5_pa_data;
        ktest_make_sample_pa_data(*(*pad).offset(i as isize));
        i += 1
    }
    let ref mut fresh6 = *(*pad).offset(2 as libc::c_int as isize);
    *fresh6 = 0 as *mut krb5_pa_data;
}
#[no_mangle]
#[c2rust::src_loc = "225:1"]
pub unsafe extern "C" fn ktest_make_sample_empty_pa_data_array(mut pad:
                                                                   *mut *mut *mut krb5_pa_data) {
    *pad =
        ealloc(::std::mem::size_of::<*mut krb5_pa_data>() as libc::c_ulong) as
            *mut *mut krb5_pa_data;
    let ref mut fresh7 = *(*pad).offset(0 as libc::c_int as isize);
    *fresh7 = 0 as *mut krb5_pa_data;
}
#[no_mangle]
#[c2rust::src_loc = "232:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_data(mut pad:
                                                       *mut krb5_pa_data) {
    (*pad).pa_type = 13 as libc::c_int;
    (*pad).length = 7 as libc::c_int as libc::c_uint;
    (*pad).contents = ealloc(7 as libc::c_int as size_t) as *mut krb5_octet;
    memcpy((*pad).contents as *mut libc::c_void,
           b"pa-data\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void, 7 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "241:1"]
pub unsafe extern "C" fn ktest_make_sample_ap_req(mut ar: *mut krb5_ap_req) {
    (*ar).ap_options = 0xfedcba98 as libc::c_uint as krb5_flags;
    (*ar).ticket =
        ealloc(::std::mem::size_of::<krb5_ticket>() as libc::c_ulong) as
            *mut krb5_ticket;
    ktest_make_sample_ticket((*ar).ticket);
    ktest_make_sample_enc_data(&mut (*ar).authenticator);
}
#[no_mangle]
#[c2rust::src_loc = "250:1"]
pub unsafe extern "C" fn ktest_make_sample_ap_rep(mut ar: *mut krb5_ap_rep) {
    ktest_make_sample_enc_data(&mut (*ar).enc_part);
}
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn ktest_make_sample_ap_rep_enc_part(mut arep:
                                                               *mut krb5_ap_rep_enc_part) {
    (*arep).ctime = 771228197 as libc::c_int;
    (*arep).cusec = 123456 as libc::c_int;
    (*arep).subkey =
        ealloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    ktest_make_sample_keyblock((*arep).subkey);
    (*arep).seq_number = 17 as libc::c_int as krb5_ui_4;
}
#[no_mangle]
#[c2rust::src_loc = "266:1"]
pub unsafe extern "C" fn ktest_make_sample_kdc_req(mut kr:
                                                       *mut krb5_kdc_req) {
    /* msg_type is left up to the calling procedure */
    ktest_make_sample_pa_data_array(&mut (*kr).padata);
    (*kr).kdc_options = 0xfedcba98 as libc::c_uint as krb5_flags;
    ktest_make_sample_principal(&mut (*kr).client);
    ktest_make_sample_principal(&mut (*kr).server);
    (*kr).from = 771228197 as libc::c_int;
    (*kr).till = 771228197 as libc::c_int;
    (*kr).rtime = 771228197 as libc::c_int;
    (*kr).nonce = 42 as libc::c_int;
    (*kr).nktypes = 2 as libc::c_int;
    (*kr).ktype =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_enctype>()
                                                    as libc::c_ulong)) as
            *mut krb5_enctype;
    *(*kr).ktype.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *(*kr).ktype.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    ktest_make_sample_addresses(&mut (*kr).addresses);
    ktest_make_sample_enc_data(&mut (*kr).authorization_data);
    ktest_make_sample_authorization_data(&mut (*kr).unenc_authdata);
    ktest_make_sample_sequence_of_ticket(&mut (*kr).second_ticket);
}
#[no_mangle]
#[c2rust::src_loc = "288:1"]
pub unsafe extern "C" fn ktest_make_sample_kdc_req_body(mut krb:
                                                            *mut krb5_kdc_req) {
    (*krb).kdc_options = 0xfedcba98 as libc::c_uint as krb5_flags;
    ktest_make_sample_principal(&mut (*krb).client);
    ktest_make_sample_principal(&mut (*krb).server);
    (*krb).from = 771228197 as libc::c_int;
    (*krb).till = 771228197 as libc::c_int;
    (*krb).rtime = 771228197 as libc::c_int;
    (*krb).nonce = 42 as libc::c_int;
    (*krb).nktypes = 2 as libc::c_int;
    (*krb).ktype =
        calloc(2 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_enctype>() as libc::c_ulong) as
            *mut krb5_enctype;
    *(*krb).ktype.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *(*krb).ktype.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    ktest_make_sample_addresses(&mut (*krb).addresses);
    ktest_make_sample_enc_data(&mut (*krb).authorization_data);
    ktest_make_sample_authorization_data(&mut (*krb).unenc_authdata);
    ktest_make_sample_sequence_of_ticket(&mut (*krb).second_ticket);
}
#[no_mangle]
#[c2rust::src_loc = "308:1"]
pub unsafe extern "C" fn ktest_make_sample_safe(mut s: *mut krb5_safe) {
    ktest_make_sample_data(&mut (*s).user_data);
    (*s).timestamp = 771228197 as libc::c_int;
    (*s).usec = 123456 as libc::c_int;
    (*s).seq_number = 17 as libc::c_int as krb5_ui_4;
    (*s).s_address =
        ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    ktest_make_sample_address((*s).s_address);
    (*s).r_address =
        ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    ktest_make_sample_address((*s).r_address);
    (*s).checksum =
        ealloc(::std::mem::size_of::<krb5_checksum>() as libc::c_ulong) as
            *mut krb5_checksum;
    ktest_make_sample_checksum((*s).checksum);
}
#[no_mangle]
#[c2rust::src_loc = "323:1"]
pub unsafe extern "C" fn ktest_make_sample_priv(mut p: *mut krb5_priv) {
    ktest_make_sample_enc_data(&mut (*p).enc_part);
}
#[no_mangle]
#[c2rust::src_loc = "329:1"]
pub unsafe extern "C" fn ktest_make_sample_priv_enc_part(mut pep:
                                                             *mut krb5_priv_enc_part) {
    ktest_make_sample_data(&mut (*pep).user_data);
    (*pep).timestamp = 771228197 as libc::c_int;
    (*pep).usec = 123456 as libc::c_int;
    (*pep).seq_number = 17 as libc::c_int as krb5_ui_4;
    (*pep).s_address =
        ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    ktest_make_sample_address((*pep).s_address);
    (*pep).r_address =
        ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    ktest_make_sample_address((*pep).r_address);
}
#[no_mangle]
#[c2rust::src_loc = "342:1"]
pub unsafe extern "C" fn ktest_make_sample_cred(mut c: *mut krb5_cred) {
    ktest_make_sample_sequence_of_ticket(&mut (*c).tickets);
    ktest_make_sample_enc_data(&mut (*c).enc_part);
}
#[no_mangle]
#[c2rust::src_loc = "349:1"]
pub unsafe extern "C" fn ktest_make_sample_sequence_of_ticket(mut sot:
                                                                  *mut *mut *mut krb5_ticket) {
    let mut i: libc::c_int = 0;
    *sot =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_ticket>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_ticket;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let ref mut fresh8 = *(*sot).offset(i as isize);
        *fresh8 =
            ealloc(::std::mem::size_of::<krb5_ticket>() as libc::c_ulong) as
                *mut krb5_ticket;
        ktest_make_sample_ticket(*(*sot).offset(i as isize));
        i += 1
    }
    let ref mut fresh9 = *(*sot).offset(2 as libc::c_int as isize);
    *fresh9 = 0 as *mut krb5_ticket;
}
#[no_mangle]
#[c2rust::src_loc = "362:1"]
pub unsafe extern "C" fn ktest_make_sample_cred_enc_part(mut cep:
                                                             *mut krb5_cred_enc_part) {
    (*cep).nonce = 42 as libc::c_int;
    (*cep).timestamp = 771228197 as libc::c_int;
    (*cep).usec = 123456 as libc::c_int;
    (*cep).s_address =
        ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    ktest_make_sample_address((*cep).s_address);
    (*cep).r_address =
        ealloc(::std::mem::size_of::<krb5_address>() as libc::c_ulong) as
            *mut krb5_address;
    ktest_make_sample_address((*cep).r_address);
    ktest_make_sequence_of_cred_info(&mut (*cep).ticket_info);
}
#[no_mangle]
#[c2rust::src_loc = "375:1"]
pub unsafe extern "C" fn ktest_make_sequence_of_cred_info(mut soci:
                                                              *mut *mut *mut krb5_cred_info) {
    let mut i: libc::c_int = 0;
    *soci =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_cred_info>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_cred_info;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let ref mut fresh10 = *(*soci).offset(i as isize);
        *fresh10 =
            ealloc(::std::mem::size_of::<krb5_cred_info>() as libc::c_ulong)
                as *mut krb5_cred_info;
        ktest_make_sample_cred_info(*(*soci).offset(i as isize));
        i += 1
    }
    let ref mut fresh11 = *(*soci).offset(2 as libc::c_int as isize);
    *fresh11 = 0 as *mut krb5_cred_info;
}
#[no_mangle]
#[c2rust::src_loc = "388:1"]
pub unsafe extern "C" fn ktest_make_sample_cred_info(mut ci:
                                                         *mut krb5_cred_info) {
    (*ci).session =
        ealloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    ktest_make_sample_keyblock((*ci).session);
    ktest_make_sample_principal(&mut (*ci).client);
    ktest_make_sample_principal(&mut (*ci).server);
    (*ci).flags = 0xfedcba98 as libc::c_uint as krb5_flags;
    (*ci).times.authtime = 771228197 as libc::c_int;
    (*ci).times.starttime = 771228197 as libc::c_int;
    (*ci).times.endtime = 771228197 as libc::c_int;
    (*ci).times.renew_till = 771228197 as libc::c_int;
    ktest_make_sample_addresses(&mut (*ci).caddrs);
}
#[no_mangle]
#[c2rust::src_loc = "403:1"]
pub unsafe extern "C" fn ktest_make_sample_error(mut kerr: *mut krb5_error) {
    (*kerr).ctime = 771228197 as libc::c_int;
    (*kerr).cusec = 123456 as libc::c_int;
    (*kerr).susec = 123456 as libc::c_int;
    (*kerr).stime = 771228197 as libc::c_int;
    (*kerr).error = 0x3c as libc::c_int as krb5_ui_4;
    ktest_make_sample_principal(&mut (*kerr).client);
    ktest_make_sample_principal(&mut (*kerr).server);
    ktest_make_sample_data(&mut (*kerr).text);
    ktest_make_sample_data(&mut (*kerr).e_data);
}
#[no_mangle]
#[c2rust::src_loc = "417:1"]
pub unsafe extern "C" fn ktest_make_sample_data(mut d: *mut krb5_data) {
    krb5_data_parse(d, b"krb5data\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "423:1"]
pub unsafe extern "C" fn ktest_make_sample_etype_info(mut p:
                                                          *mut *mut *mut krb5_etype_info_entry) {
    let mut info: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    info =
        ealloc((4 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_etype_info_entry>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_etype_info_entry;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let ref mut fresh12 = *info.offset(i as isize);
        *fresh12 =
            ealloc(::std::mem::size_of::<krb5_etype_info_entry>() as
                       libc::c_ulong) as *mut krb5_etype_info_entry;
        (**info.offset(i as isize)).etype = i;
        len =
            asprintf(&mut str as *mut *mut libc::c_char,
                     b"Morton\'s #%d\x00" as *const u8 as *const libc::c_char,
                     i);
        if len < 0 as libc::c_int { abort(); }
        let ref mut fresh13 = (**info.offset(i as isize)).salt;
        *fresh13 = str as *mut krb5_octet;
        (**info.offset(i as isize)).length = len as libc::c_uint;
        let ref mut fresh14 = (**info.offset(i as isize)).s2kparams.data;
        *fresh14 = 0 as *mut libc::c_char;
        (**info.offset(i as isize)).s2kparams.length =
            0 as libc::c_int as libc::c_uint;
        (**info.offset(i as isize)).magic =
            -(1760647385 as libc::c_long) as krb5_magic;
        i += 1
    }
    free((**info.offset(1 as libc::c_int as isize)).salt as
             *mut libc::c_void);
    (**info.offset(1 as libc::c_int as isize)).length =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    let ref mut fresh15 = (**info.offset(1 as libc::c_int as isize)).salt;
    *fresh15 = 0 as *mut krb5_octet;
    *p = info;
}
#[no_mangle]
#[c2rust::src_loc = "450:1"]
pub unsafe extern "C" fn ktest_make_sample_etype_info2(mut p:
                                                           *mut *mut *mut krb5_etype_info_entry) {
    let mut info: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    info =
        ealloc((4 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_etype_info_entry>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_etype_info_entry;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let ref mut fresh16 = *info.offset(i as isize);
        *fresh16 =
            ealloc(::std::mem::size_of::<krb5_etype_info_entry>() as
                       libc::c_ulong) as *mut krb5_etype_info_entry;
        (**info.offset(i as isize)).etype = i;
        len =
            asprintf(&mut str as *mut *mut libc::c_char,
                     b"Morton\'s #%d\x00" as *const u8 as *const libc::c_char,
                     i);
        if len < 0 as libc::c_int { abort(); }
        let ref mut fresh17 = (**info.offset(i as isize)).salt;
        *fresh17 = str as *mut krb5_octet;
        (**info.offset(i as isize)).length = len as libc::c_uint;
        len =
            asprintf(&mut (**info.offset(i as isize)).s2kparams.data as
                         *mut *mut libc::c_char,
                     b"s2k: %d\x00" as *const u8 as *const libc::c_char, i);
        if len < 0 as libc::c_int { abort(); }
        (**info.offset(i as isize)).s2kparams.length = len as libc::c_uint;
        (**info.offset(i as isize)).magic =
            -(1760647385 as libc::c_long) as krb5_magic;
        i += 1
    }
    free((**info.offset(1 as libc::c_int as isize)).salt as
             *mut libc::c_void);
    (**info.offset(1 as libc::c_int as isize)).length =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    let ref mut fresh18 = (**info.offset(1 as libc::c_int as isize)).salt;
    *fresh18 = 0 as *mut krb5_octet;
    *p = info;
}
#[no_mangle]
#[c2rust::src_loc = "479:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_enc_ts(mut pa_enc:
                                                         *mut krb5_pa_enc_ts) {
    (*pa_enc).patimestamp = 771228197 as libc::c_int;
    (*pa_enc).pausec = 123456 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "486:1"]
pub unsafe extern "C" fn ktest_make_sample_sam_challenge_2(mut p:
                                                               *mut krb5_sam_challenge_2) {
    /* Need a valid DER sequence encoding here; this one contains the OCTET
     * STRING "challenge". */
    krb5_data_parse(&mut (*p).sam_challenge_2_body,
                    b"0\x0b\x04\tchallenge\x00" as *const u8 as
                        *const libc::c_char); /* information */
    (*p).sam_cksum =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_checksum>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_checksum; /* KRB5_SAM_* values */
    let ref mut fresh19 = *(*p).sam_cksum.offset(0 as libc::c_int as isize);
    *fresh19 =
        ealloc(::std::mem::size_of::<krb5_checksum>() as libc::c_ulong) as
            *mut krb5_checksum;
    ktest_make_sample_checksum(*(*p).sam_cksum.offset(0 as libc::c_int as
                                                          isize));
    let ref mut fresh20 = *(*p).sam_cksum.offset(1 as libc::c_int as isize);
    *fresh20 = 0 as *mut krb5_checksum;
}
#[no_mangle]
#[c2rust::src_loc = "498:1"]
pub unsafe extern "C" fn ktest_make_sample_sam_challenge_2_body(mut p:
                                                                    *mut krb5_sam_challenge_2_body) {
    (*p).sam_type = 42 as libc::c_int;
    (*p).sam_flags = 0x80000000 as libc::c_uint as krb5_flags;
    krb5_data_parse(&mut (*p).sam_type_name,
                    b"type name\x00" as *const u8 as *const libc::c_char);
    (*p).sam_track_id = empty_data();
    krb5_data_parse(&mut (*p).sam_challenge_label,
                    b"challenge label\x00" as *const u8 as
                        *const libc::c_char);
    krb5_data_parse(&mut (*p).sam_challenge,
                    b"challenge ipse\x00" as *const u8 as
                        *const libc::c_char);
    krb5_data_parse(&mut (*p).sam_response_prompt,
                    b"response_prompt ipse\x00" as *const u8 as
                        *const libc::c_char);
    (*p).sam_pk_for_sad = empty_data();
    (*p).sam_nonce = 0x543210 as libc::c_int;
    (*p).sam_etype = 0x14 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "513:1"]
pub unsafe extern "C" fn ktest_make_sample_sam_response_2(mut p:
                                                              *mut krb5_sam_response_2) {
    (*p).magic = -(1760647373 as libc::c_long) as krb5_magic;
    (*p).sam_type = 43 as libc::c_int;
    (*p).sam_flags = 0x80000000 as libc::c_uint as krb5_flags;
    krb5_data_parse(&mut (*p).sam_track_id,
                    b"track data\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).sam_enc_nonce_or_sad.ciphertext,
                    b"nonce or sad\x00" as *const u8 as *const libc::c_char);
    (*p).sam_enc_nonce_or_sad.enctype = 0x14 as libc::c_int;
    (*p).sam_enc_nonce_or_sad.kvno = 3382 as libc::c_int as krb5_kvno;
    (*p).sam_nonce = 0x543210 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "526:1"]
pub unsafe extern "C" fn ktest_make_sample_enc_sam_response_enc_2(mut p:
                                                                      *mut krb5_enc_sam_response_enc_2) {
    (*p).magic = 83 as libc::c_int;
    (*p).sam_nonce = 88 as libc::c_int;
    krb5_data_parse(&mut (*p).sam_sad,
                    b"enc_sam_response_enc_2\x00" as *const u8 as
                        *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "534:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_for_user(mut p:
                                                           *mut krb5_pa_for_user) {
    ktest_make_sample_principal(&mut (*p).user);
    ktest_make_sample_checksum(&mut (*p).cksum);
    ktest_make_sample_data(&mut (*p).auth_package);
}
#[no_mangle]
#[c2rust::src_loc = "542:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_s4u_x509_user(mut p:
                                                                *mut krb5_pa_s4u_x509_user) {
    let mut u: *mut krb5_s4u_userid = &mut (*p).user_id;
    (*u).nonce = 13243546 as libc::c_int;
    ktest_make_sample_principal(&mut (*u).user);
    krb5_data_parse(&mut (*u).subject_cert,
                    b"pa_s4u_x509_user\x00" as *const u8 as
                        *const libc::c_char);
    (*u).options = 0x80000000 as libc::c_uint as krb5_flags;
    ktest_make_sample_checksum(&mut (*p).cksum);
}
#[no_mangle]
#[c2rust::src_loc = "554:1"]
pub unsafe extern "C" fn ktest_make_sample_ad_kdcissued(mut p:
                                                            *mut krb5_ad_kdcissued) {
    ktest_make_sample_checksum(&mut (*p).ad_checksum);
    ktest_make_sample_principal(&mut (*p).i_principal);
    ktest_make_sample_authorization_data(&mut (*p).elements);
}
#[no_mangle]
#[c2rust::src_loc = "562:1"]
pub unsafe extern "C" fn ktest_make_sample_ad_signedpath_data(mut p:
                                                                  *mut krb5_ad_signedpath_data) {
    ktest_make_sample_principal(&mut (*p).client);
    (*p).authtime = 771228197 as libc::c_int;
    (*p).delegated =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_principal>()
                                                    as libc::c_ulong)) as
            *mut krb5_principal;
    ktest_make_sample_principal(&mut *(*p).delegated.offset(0 as libc::c_int
                                                                as isize));
    let ref mut fresh21 = *(*p).delegated.offset(1 as libc::c_int as isize);
    *fresh21 = 0 as krb5_principal;
    ktest_make_sample_authorization_data(&mut (*p).authorization_data);
    ktest_make_sample_pa_data_array(&mut (*p).method_data);
}
#[no_mangle]
#[c2rust::src_loc = "574:1"]
pub unsafe extern "C" fn ktest_make_sample_ad_signedpath(mut p:
                                                             *mut krb5_ad_signedpath) {
    (*p).enctype = 1 as libc::c_int;
    ktest_make_sample_checksum(&mut (*p).checksum);
    (*p).delegated =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_principal>()
                                                    as libc::c_ulong)) as
            *mut krb5_principal;
    let ref mut fresh22 = *(*p).delegated.offset(1 as libc::c_int as isize);
    *fresh22 = 0 as krb5_principal;
    ktest_make_sample_pa_data_array(&mut (*p).method_data);
}
#[no_mangle]
#[c2rust::src_loc = "584:1"]
pub unsafe extern "C" fn ktest_make_sample_iakerb_header(mut ih:
                                                             *mut krb5_iakerb_header) {
    ktest_make_sample_data(&mut (*ih).target_realm);
    (*ih).cookie =
        ealloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    ktest_make_sample_data((*ih).cookie);
}
#[no_mangle]
#[c2rust::src_loc = "592:1"]
pub unsafe extern "C" fn ktest_make_sample_iakerb_finished(mut ih:
                                                               *mut krb5_iakerb_finished) {
    ktest_make_sample_checksum(&mut (*ih).checksum);
}
#[c2rust::src_loc = "598:1"]
unsafe extern "C" fn ktest_make_sample_fast_finished(mut p:
                                                         *mut krb5_fast_finished) {
    (*p).timestamp = 771228197 as libc::c_int;
    (*p).usec = 123456 as libc::c_int;
    ktest_make_sample_principal(&mut (*p).client);
    ktest_make_sample_checksum(&mut (*p).ticket_checksum);
}
#[no_mangle]
#[c2rust::src_loc = "607:1"]
pub unsafe extern "C" fn ktest_make_sample_fast_response(mut p:
                                                             *mut krb5_fast_response) {
    ktest_make_sample_pa_data_array(&mut (*p).padata);
    (*p).strengthen_key =
        ealloc(::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    ktest_make_sample_keyblock((*p).strengthen_key);
    (*p).finished =
        ealloc(::std::mem::size_of::<krb5_fast_finished>() as libc::c_ulong)
            as *mut krb5_fast_finished;
    ktest_make_sample_fast_finished((*p).finished);
    (*p).nonce = 42 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "618:1"]
pub unsafe extern "C" fn ktest_make_sha256_alg(mut p:
                                                   *mut krb5_algorithm_identifier) {
    /* { 2 16 840 1 101 3 4 2 1 } */
    krb5_data_parse(&mut (*p).algorithm,
                    b"`\x86H\x01e\x03\x04\x02\x01\x00" as *const u8 as
                        *const libc::c_char);
    (*p).parameters = empty_data();
}
#[no_mangle]
#[c2rust::src_loc = "626:1"]
pub unsafe extern "C" fn ktest_make_sha1_alg(mut p:
                                                 *mut krb5_algorithm_identifier) {
    /* { 1 3 14 3 2 26 } */
    krb5_data_parse(&mut (*p).algorithm,
                    b"+\x0e\x03\x02\x1a\x00" as *const u8 as
                        *const libc::c_char);
    (*p).parameters = empty_data();
}
#[no_mangle]
#[c2rust::src_loc = "634:1"]
pub unsafe extern "C" fn ktest_make_minimal_otp_tokeninfo(mut p:
                                                              *mut krb5_otp_tokeninfo) {
    memset(p as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_otp_tokeninfo>() as libc::c_ulong);
    (*p).iteration_count = -(1 as libc::c_int);
    (*p).format = (*p).iteration_count;
    (*p).length = (*p).format;
}
#[no_mangle]
#[c2rust::src_loc = "641:1"]
pub unsafe extern "C" fn ktest_make_maximal_otp_tokeninfo(mut p:
                                                              *mut krb5_otp_tokeninfo) {
    (*p).flags =
        0x40000000 as libc::c_int | 0x20000000 as libc::c_int |
            0x10000000 as libc::c_int | 0x4000000 as libc::c_int |
            0x2000000 as libc::c_int | 0x1000000 as libc::c_int;
    krb5_data_parse(&mut (*p).vendor,
                    b"Examplecorp\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).challenge,
                    b"hark!\x00" as *const u8 as *const libc::c_char);
    (*p).length = 10 as libc::c_int;
    (*p).format = 2 as libc::c_int;
    krb5_data_parse(&mut (*p).token_id,
                    b"yourtoken\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).alg_id,
                    b"urn:ietf:params:xml:ns:keyprov:pskc:hotp\x00" as
                        *const u8 as *const libc::c_char);
    (*p).supported_hash_alg =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_algorithm_identifier>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_algorithm_identifier;
    let ref mut fresh23 =
        *(*p).supported_hash_alg.offset(0 as libc::c_int as isize);
    *fresh23 =
        ealloc(::std::mem::size_of::<krb5_algorithm_identifier>() as
                   libc::c_ulong) as *mut krb5_algorithm_identifier;
    ktest_make_sha256_alg(*(*p).supported_hash_alg.offset(0 as libc::c_int as
                                                              isize));
    let ref mut fresh24 =
        *(*p).supported_hash_alg.offset(1 as libc::c_int as isize);
    *fresh24 =
        ealloc(::std::mem::size_of::<krb5_algorithm_identifier>() as
                   libc::c_ulong) as *mut krb5_algorithm_identifier;
    ktest_make_sha1_alg(*(*p).supported_hash_alg.offset(1 as libc::c_int as
                                                            isize));
    let ref mut fresh25 =
        *(*p).supported_hash_alg.offset(2 as libc::c_int as isize);
    *fresh25 = 0 as *mut krb5_algorithm_identifier;
    (*p).iteration_count = 1000 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "662:1"]
pub unsafe extern "C" fn ktest_make_minimal_pa_otp_challenge(mut p:
                                                                 *mut krb5_pa_otp_challenge) {
    memset(p as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_pa_otp_challenge>() as libc::c_ulong);
    krb5_data_parse(&mut (*p).nonce,
                    b"minnonce\x00" as *const u8 as *const libc::c_char);
    (*p).tokeninfo =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_otp_tokeninfo>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_otp_tokeninfo;
    let ref mut fresh26 = *(*p).tokeninfo.offset(0 as libc::c_int as isize);
    *fresh26 =
        ealloc(::std::mem::size_of::<krb5_otp_tokeninfo>() as libc::c_ulong)
            as *mut krb5_otp_tokeninfo;
    ktest_make_minimal_otp_tokeninfo(*(*p).tokeninfo.offset(0 as libc::c_int
                                                                as isize));
    let ref mut fresh27 = *(*p).tokeninfo.offset(1 as libc::c_int as isize);
    *fresh27 = 0 as *mut krb5_otp_tokeninfo;
}
#[no_mangle]
#[c2rust::src_loc = "673:1"]
pub unsafe extern "C" fn ktest_make_maximal_pa_otp_challenge(mut p:
                                                                 *mut krb5_pa_otp_challenge) {
    krb5_data_parse(&mut (*p).nonce,
                    b"maxnonce\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).service,
                    b"testservice\x00" as *const u8 as *const libc::c_char);
    (*p).tokeninfo =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_otp_tokeninfo>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_otp_tokeninfo;
    let ref mut fresh28 = *(*p).tokeninfo.offset(0 as libc::c_int as isize);
    *fresh28 =
        ealloc(::std::mem::size_of::<krb5_otp_tokeninfo>() as libc::c_ulong)
            as *mut krb5_otp_tokeninfo;
    ktest_make_minimal_otp_tokeninfo(*(*p).tokeninfo.offset(0 as libc::c_int
                                                                as isize));
    let ref mut fresh29 = *(*p).tokeninfo.offset(1 as libc::c_int as isize);
    *fresh29 =
        ealloc(::std::mem::size_of::<krb5_otp_tokeninfo>() as libc::c_ulong)
            as *mut krb5_otp_tokeninfo;
    ktest_make_maximal_otp_tokeninfo(*(*p).tokeninfo.offset(1 as libc::c_int
                                                                as isize));
    let ref mut fresh30 = *(*p).tokeninfo.offset(2 as libc::c_int as isize);
    *fresh30 = 0 as *mut krb5_otp_tokeninfo;
    krb5_data_parse(&mut (*p).salt,
                    b"keysalt\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).s2kparams,
                    b"1234\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "688:1"]
pub unsafe extern "C" fn ktest_make_minimal_pa_otp_req(mut p:
                                                           *mut krb5_pa_otp_req) {
    memset(p as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_pa_otp_req>() as libc::c_ulong);
    (*p).iteration_count = -(1 as libc::c_int);
    (*p).format = -(1 as libc::c_int);
    ktest_make_sample_enc_data(&mut (*p).enc_data);
}
#[no_mangle]
#[c2rust::src_loc = "697:1"]
pub unsafe extern "C" fn ktest_make_maximal_pa_otp_req(mut p:
                                                           *mut krb5_pa_otp_req) {
    (*p).flags = 0x40000000 as libc::c_int | 0x20000000 as libc::c_int;
    krb5_data_parse(&mut (*p).nonce,
                    b"nonce\x00" as *const u8 as *const libc::c_char);
    ktest_make_sample_enc_data(&mut (*p).enc_data);
    (*p).hash_alg =
        ealloc(::std::mem::size_of::<krb5_algorithm_identifier>() as
                   libc::c_ulong) as *mut krb5_algorithm_identifier;
    ktest_make_sha256_alg((*p).hash_alg);
    (*p).iteration_count = 1000 as libc::c_int;
    krb5_data_parse(&mut (*p).otp_value,
                    b"frogs\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).pin,
                    b"myfirstpin\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).challenge,
                    b"hark!\x00" as *const u8 as *const libc::c_char);
    (*p).time = 771228197 as libc::c_int;
    krb5_data_parse(&mut (*p).counter,
                    b"346\x00" as *const u8 as *const libc::c_char);
    (*p).format = 2 as libc::c_int;
    krb5_data_parse(&mut (*p).token_id,
                    b"yourtoken\x00" as *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).alg_id,
                    b"urn:ietf:params:xml:ns:keyprov:pskc:hotp\x00" as
                        *const u8 as *const libc::c_char);
    krb5_data_parse(&mut (*p).vendor,
                    b"Examplecorp\x00" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "719:1"]
unsafe extern "C" fn ktest_make_sample_pk_authenticator(mut p:
                                                            *mut krb5_pk_authenticator) {
    (*p).cusec = 123456 as libc::c_int;
    (*p).ctime = 771228197 as libc::c_int;
    (*p).nonce = 42 as libc::c_int;
    ktest_make_sample_checksum(&mut (*p).paChecksum);
    /* We don't encode the checksum type, only the contents. */
    (*p).paChecksum.checksum_type = 0 as libc::c_int;
    (*p).freshnessToken =
        ealloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    ktest_make_sample_data((*p).freshnessToken);
}
#[c2rust::src_loc = "732:1"]
unsafe extern "C" fn ktest_make_sample_oid(mut p: *mut krb5_data) {
    krb5_data_parse(p,
                    b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as *const u8 as
                        *const libc::c_char);
}
#[c2rust::src_loc = "738:1"]
unsafe extern "C" fn ktest_make_sample_algorithm_identifier(mut p:
                                                                *mut krb5_algorithm_identifier) {
    ktest_make_sample_oid(&mut (*p).algorithm);
    /* Need a valid DER encoding here; this is the OCTET STRING "params". */
    krb5_data_parse(&mut (*p).parameters,
                    b"\x04\x06params\x00" as *const u8 as
                        *const libc::c_char);
}
#[c2rust::src_loc = "746:1"]
unsafe extern "C" fn ktest_make_sample_algorithm_identifier_no_params(mut p:
                                                                          *mut krb5_algorithm_identifier) {
    ktest_make_sample_oid(&mut (*p).algorithm);
    (*p).parameters = empty_data();
}
#[c2rust::src_loc = "753:1"]
unsafe extern "C" fn ktest_make_sample_subject_pk_info(mut p:
                                                           *mut krb5_subject_pk_info) {
    ktest_make_sample_algorithm_identifier(&mut (*p).algorithm);
    ktest_make_sample_data(&mut (*p).subjectPublicKey);
}
#[c2rust::src_loc = "760:1"]
unsafe extern "C" fn ktest_make_sample_external_principal_identifier(mut p:
                                                                         *mut krb5_external_principal_identifier) {
    ktest_make_sample_data(&mut (*p).subjectName);
    ktest_make_sample_data(&mut (*p).issuerAndSerialNumber);
    ktest_make_sample_data(&mut (*p).subjectKeyIdentifier);
}
#[no_mangle]
#[c2rust::src_loc = "769:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_pk_as_req(mut p:
                                                            *mut krb5_pa_pk_as_req) {
    ktest_make_sample_data(&mut (*p).signedAuthPack);
    (*p).trustedCertifiers =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_external_principal_identifier>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_external_principal_identifier;
    let ref mut fresh31 =
        *(*p).trustedCertifiers.offset(0 as libc::c_int as isize);
    *fresh31 =
        ealloc(::std::mem::size_of::<krb5_external_principal_identifier>() as
                   libc::c_ulong) as *mut krb5_external_principal_identifier;
    ktest_make_sample_external_principal_identifier(*(*p).trustedCertifiers.offset(0
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize));
    let ref mut fresh32 =
        *(*p).trustedCertifiers.offset(1 as libc::c_int as isize);
    *fresh32 = 0 as *mut krb5_external_principal_identifier;
    ktest_make_sample_data(&mut (*p).kdcPkId);
}
#[c2rust::src_loc = "782:1"]
unsafe extern "C" fn ktest_make_sample_dh_rep_info(mut p:
                                                       *mut krb5_dh_rep_info) {
    ktest_make_sample_data(&mut (*p).dhSignedData);
    ktest_make_sample_data(&mut (*p).serverDHNonce);
    (*p).kdfID =
        ealloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    ktest_make_sample_data((*p).kdfID);
}
#[no_mangle]
#[c2rust::src_loc = "791:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_pk_as_rep_dhInfo(mut p:
                                                                   *mut krb5_pa_pk_as_rep) {
    (*p).choice = choice_pa_pk_as_rep_dhInfo;
    ktest_make_sample_dh_rep_info(&mut (*p).u.dh_Info);
}
#[no_mangle]
#[c2rust::src_loc = "798:1"]
pub unsafe extern "C" fn ktest_make_sample_pa_pk_as_rep_encKeyPack(mut p:
                                                                       *mut krb5_pa_pk_as_rep) {
    (*p).choice = choice_pa_pk_as_rep_encKeyPack;
    ktest_make_sample_data(&mut (*p).u.encKeyPack);
}
#[no_mangle]
#[c2rust::src_loc = "805:1"]
pub unsafe extern "C" fn ktest_make_sample_auth_pack(mut p:
                                                         *mut krb5_auth_pack) {
    ktest_make_sample_pk_authenticator(&mut (*p).pkAuthenticator);
    (*p).clientPublicValue =
        ealloc(::std::mem::size_of::<krb5_subject_pk_info>() as libc::c_ulong)
            as *mut krb5_subject_pk_info;
    ktest_make_sample_subject_pk_info((*p).clientPublicValue);
    (*p).supportedCMSTypes =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_algorithm_identifier>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_algorithm_identifier;
    let ref mut fresh33 =
        *(*p).supportedCMSTypes.offset(0 as libc::c_int as isize);
    *fresh33 =
        ealloc(::std::mem::size_of::<krb5_algorithm_identifier>() as
                   libc::c_ulong) as *mut krb5_algorithm_identifier;
    ktest_make_sample_algorithm_identifier(*(*p).supportedCMSTypes.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize));
    let ref mut fresh34 =
        *(*p).supportedCMSTypes.offset(1 as libc::c_int as isize);
    *fresh34 =
        ealloc(::std::mem::size_of::<krb5_algorithm_identifier>() as
                   libc::c_ulong) as *mut krb5_algorithm_identifier;
    ktest_make_sample_algorithm_identifier_no_params(*(*p).supportedCMSTypes.offset(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize));
    let ref mut fresh35 =
        *(*p).supportedCMSTypes.offset(2 as libc::c_int as isize);
    *fresh35 = 0 as *mut krb5_algorithm_identifier;
    ktest_make_sample_data(&mut (*p).clientDHNonce);
    (*p).supportedKDFs =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_data>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_data;
    let ref mut fresh36 =
        *(*p).supportedKDFs.offset(0 as libc::c_int as isize);
    *fresh36 =
        ealloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    ktest_make_sample_data(*(*p).supportedKDFs.offset(0 as libc::c_int as
                                                          isize));
    let ref mut fresh37 =
        *(*p).supportedKDFs.offset(1 as libc::c_int as isize);
    *fresh37 = 0 as *mut krb5_data;
}
#[no_mangle]
#[c2rust::src_loc = "824:1"]
pub unsafe extern "C" fn ktest_make_sample_kdc_dh_key_info(mut p:
                                                               *mut krb5_kdc_dh_key_info) {
    ktest_make_sample_data(&mut (*p).subjectPublicKey);
    (*p).nonce = 42 as libc::c_int;
    (*p).dhKeyExpiration = 771228197 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "832:1"]
pub unsafe extern "C" fn ktest_make_sample_reply_key_pack(mut p:
                                                              *mut krb5_reply_key_pack) {
    ktest_make_sample_keyblock(&mut (*p).replyKey);
    ktest_make_sample_checksum(&mut (*p).asChecksum);
}
#[no_mangle]
#[c2rust::src_loc = "839:1"]
pub unsafe extern "C" fn ktest_make_sample_sp80056a_other_info(mut p:
                                                                   *mut krb5_sp80056a_other_info) {
    ktest_make_sample_algorithm_identifier_no_params(&mut (*p).algorithm_identifier);
    ktest_make_sample_principal(&mut (*p).party_u_info);
    ktest_make_sample_principal(&mut (*p).party_v_info);
    ktest_make_sample_data(&mut (*p).supp_pub_info);
}
#[no_mangle]
#[c2rust::src_loc = "848:1"]
pub unsafe extern "C" fn ktest_make_sample_pkinit_supp_pub_info(mut p:
                                                                    *mut krb5_pkinit_supp_pub_info) {
    (*p).enctype = 0x14 as libc::c_int;
    ktest_make_sample_data(&mut (*p).as_req);
    ktest_make_sample_data(&mut (*p).pk_as_rep);
}
/* not DISABLE_PKINIT */
#[no_mangle]
#[c2rust::src_loc = "894:1"]
pub unsafe extern "C" fn ktest_make_sample_kkdcp_message(mut p:
                                                             *mut krb5_kkdcp_message) {
    let mut req: krb5_kdc_req =
        krb5_kdc_req{magic: 0,
                     msg_type: 0,
                     padata: 0 as *mut *mut krb5_pa_data,
                     kdc_options: 0,
                     client: 0 as *mut krb5_principal_data,
                     server: 0 as *mut krb5_principal_data,
                     from: 0,
                     till: 0,
                     rtime: 0,
                     nonce: 0,
                     nktypes: 0,
                     ktype: 0 as *mut krb5_enctype,
                     addresses: 0 as *mut *mut krb5_address,
                     authorization_data:
                         krb5_enc_data{magic: 0,
                                       enctype: 0,
                                       kvno: 0,
                                       ciphertext:
                                           krb5_data{magic: 0,
                                                     length: 0,
                                                     data:
                                                         0 as
                                                             *mut libc::c_char,},},
                     unenc_authdata: 0 as *mut *mut krb5_authdata,
                     second_ticket: 0 as *mut *mut krb5_ticket,};
    let mut message: *mut krb5_data = 0 as *mut krb5_data;
    ktest_make_sample_kdc_req(&mut req);
    req.msg_type = 10 as libc::c_int as krb5_msgtype;
    encode_krb5_as_req(&mut req, &mut message);
    (*p).kerb_message = *message;
    free(message as *mut libc::c_void);
    ktest_empty_kdc_req(&mut req);
    ktest_make_sample_data(&mut (*p).target_domain);
    (*p).dclocator_hint = 0 as libc::c_int;
}
#[c2rust::src_loc = "910:1"]
unsafe extern "C" fn make_ad_element(mut ad_type: krb5_authdatatype,
                                     mut str: *const libc::c_char)
 -> *mut krb5_authdata {
    let mut ad: *mut krb5_authdata = 0 as *mut krb5_authdata;
    ad =
        ealloc(::std::mem::size_of::<krb5_authdata>() as libc::c_ulong) as
            *mut krb5_authdata;
    (*ad).ad_type = ad_type;
    (*ad).length = strlen(str) as libc::c_uint;
    (*ad).contents = ealloc((*ad).length as size_t) as *mut krb5_octet;
    memcpy((*ad).contents as *mut libc::c_void, str as *const libc::c_void,
           (*ad).length as libc::c_ulong);
    return ad;
}
#[c2rust::src_loc = "923:1"]
unsafe extern "C" fn make_vmac(mut include_princ: krb5_boolean,
                               mut kvno: krb5_kvno, mut enctype: krb5_enctype,
                               mut cksumstr: *const libc::c_char)
 -> *mut krb5_verifier_mac {
    let mut vmac: *mut krb5_verifier_mac = 0 as *mut krb5_verifier_mac;
    vmac =
        ealloc(::std::mem::size_of::<krb5_verifier_mac>() as libc::c_ulong) as
            *mut krb5_verifier_mac;
    if include_princ != 0 {
        ktest_make_sample_principal(&mut (*vmac).princ);
        krb5_set_principal_realm(0 as krb5_context, (*vmac).princ,
                                 b"\x00" as *const u8 as *const libc::c_char);
    } else { (*vmac).princ = 0 as krb5_principal }
    (*vmac).kvno = kvno;
    (*vmac).enctype = enctype;
    (*vmac).checksum.checksum_type = 1 as libc::c_int;
    (*vmac).checksum.length = strlen(cksumstr) as libc::c_uint;
    (*vmac).checksum.contents =
        ealloc((*vmac).checksum.length as size_t) as *mut krb5_octet;
    memcpy((*vmac).checksum.contents as *mut libc::c_void,
           cksumstr as *const libc::c_void,
           (*vmac).checksum.length as libc::c_ulong);
    return vmac;
}
#[no_mangle]
#[c2rust::src_loc = "945:1"]
pub unsafe extern "C" fn ktest_make_minimal_cammac(mut p: *mut krb5_cammac) {
    memset(p as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_cammac>() as libc::c_ulong);
    (*p).elements =
        ealloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_authdata;
    let ref mut fresh38 = *(*p).elements.offset(0 as libc::c_int as isize);
    *fresh38 =
        make_ad_element(1 as libc::c_int,
                        b"ad1\x00" as *const u8 as *const libc::c_char);
    let ref mut fresh39 = *(*p).elements.offset(1 as libc::c_int as isize);
    *fresh39 = 0 as *mut krb5_authdata;
}
#[no_mangle]
#[c2rust::src_loc = "954:1"]
pub unsafe extern "C" fn ktest_make_maximal_cammac(mut p: *mut krb5_cammac) {
    (*p).elements =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_authdata;
    let ref mut fresh40 = *(*p).elements.offset(0 as libc::c_int as isize);
    *fresh40 =
        make_ad_element(1 as libc::c_int,
                        b"ad1\x00" as *const u8 as *const libc::c_char);
    let ref mut fresh41 = *(*p).elements.offset(1 as libc::c_int as isize);
    *fresh41 =
        make_ad_element(2 as libc::c_int,
                        b"ad2\x00" as *const u8 as *const libc::c_char);
    let ref mut fresh42 = *(*p).elements.offset(2 as libc::c_int as isize);
    *fresh42 = 0 as *mut krb5_authdata;
    (*p).kdc_verifier =
        make_vmac(1 as libc::c_int as krb5_boolean,
                  5 as libc::c_int as krb5_kvno, 16 as libc::c_int,
                  b"cksumkdc\x00" as *const u8 as *const libc::c_char);
    (*p).svc_verifier =
        make_vmac(1 as libc::c_int as krb5_boolean,
                  5 as libc::c_int as krb5_kvno, 16 as libc::c_int,
                  b"cksumsvc\x00" as *const u8 as *const libc::c_char);
    (*p).other_verifiers =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_verifier_mac>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_verifier_mac;
    let ref mut fresh43 =
        *(*p).other_verifiers.offset(0 as libc::c_int as isize);
    *fresh43 =
        make_vmac(0 as libc::c_int as krb5_boolean,
                  0 as libc::c_int as krb5_kvno, 0 as libc::c_int,
                  b"cksum1\x00" as *const u8 as *const libc::c_char);
    let ref mut fresh44 =
        *(*p).other_verifiers.offset(1 as libc::c_int as isize);
    *fresh44 =
        make_vmac(1 as libc::c_int as krb5_boolean,
                  5 as libc::c_int as krb5_kvno, 16 as libc::c_int,
                  b"cksum2\x00" as *const u8 as *const libc::c_char);
    let ref mut fresh45 =
        *(*p).other_verifiers.offset(2 as libc::c_int as isize);
    *fresh45 = 0 as *mut krb5_verifier_mac;
}
#[no_mangle]
#[c2rust::src_loc = "969:1"]
pub unsafe extern "C" fn ktest_make_sample_secure_cookie(mut p:
                                                             *mut krb5_secure_cookie) {
    ktest_make_sample_pa_data_array(&mut (*p).data);
    (*p).time = 771228197 as libc::c_int as time_t;
}
#[no_mangle]
#[c2rust::src_loc = "976:1"]
pub unsafe extern "C" fn ktest_make_minimal_spake_factor(mut p:
                                                             *mut krb5_spake_factor) {
    (*p).type_0 = 1 as libc::c_int;
    (*p).data = 0 as *mut krb5_data;
}
#[no_mangle]
#[c2rust::src_loc = "983:1"]
pub unsafe extern "C" fn ktest_make_maximal_spake_factor(mut p:
                                                             *mut krb5_spake_factor) {
    (*p).type_0 = 2 as libc::c_int;
    (*p).data =
        ealloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    krb5_data_parse((*p).data,
                    b"fdata\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "991:1"]
pub unsafe extern "C" fn ktest_make_support_pa_spake(mut p:
                                                         *mut krb5_pa_spake) {
    let mut s: *mut krb5_spake_support = &mut (*p).u.support;
    (*s).ngroups = 2 as libc::c_int;
    (*s).groups =
        ealloc(((*s).ngroups as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>()
                                                    as libc::c_ulong)) as
            *mut int32_t;
    *(*s).groups.offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    *(*s).groups.offset(1 as libc::c_int as isize) = 2 as libc::c_int;
    (*p).choice = SPAKE_MSGTYPE_SUPPORT;
}
#[no_mangle]
#[c2rust::src_loc = "1003:1"]
pub unsafe extern "C" fn ktest_make_challenge_pa_spake(mut p:
                                                           *mut krb5_pa_spake) {
    let mut c: *mut krb5_spake_challenge = &mut (*p).u.challenge;
    (*c).group = 1 as libc::c_int;
    krb5_data_parse(&mut (*c).pubkey,
                    b"T value\x00" as *const u8 as *const libc::c_char);
    (*c).factors =
        ealloc((3 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_spake_factor>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_spake_factor;
    let ref mut fresh46 = *(*c).factors.offset(0 as libc::c_int as isize);
    *fresh46 =
        ealloc(::std::mem::size_of::<krb5_spake_factor>() as libc::c_ulong) as
            *mut krb5_spake_factor;
    ktest_make_minimal_spake_factor(*(*c).factors.offset(0 as libc::c_int as
                                                             isize));
    let ref mut fresh47 = *(*c).factors.offset(1 as libc::c_int as isize);
    *fresh47 =
        ealloc(::std::mem::size_of::<krb5_spake_factor>() as libc::c_ulong) as
            *mut krb5_spake_factor;
    ktest_make_maximal_spake_factor(*(*c).factors.offset(1 as libc::c_int as
                                                             isize));
    let ref mut fresh48 = *(*c).factors.offset(2 as libc::c_int as isize);
    *fresh48 = 0 as *mut krb5_spake_factor;
    (*p).choice = SPAKE_MSGTYPE_CHALLENGE;
}
#[no_mangle]
#[c2rust::src_loc = "1019:1"]
pub unsafe extern "C" fn ktest_make_response_pa_spake(mut p:
                                                          *mut krb5_pa_spake) {
    let mut r: *mut krb5_spake_response = &mut (*p).u.response;
    krb5_data_parse(&mut (*r).pubkey,
                    b"S value\x00" as *const u8 as *const libc::c_char);
    ktest_make_sample_enc_data(&mut (*r).factor);
    (*p).choice = SPAKE_MSGTYPE_RESPONSE;
}
#[no_mangle]
#[c2rust::src_loc = "1029:1"]
pub unsafe extern "C" fn ktest_make_encdata_pa_spake(mut p:
                                                         *mut krb5_pa_spake) {
    ktest_make_sample_enc_data(&mut (*p).u.encdata);
    (*p).choice = SPAKE_MSGTYPE_ENCDATA;
}
/* ***************************************************************/
/* destructors */
#[no_mangle]
#[c2rust::src_loc = "1039:1"]
pub unsafe extern "C" fn ktest_destroy_data(mut d: *mut *mut krb5_data) {
    if !(*d).is_null() {
        free((**d).data as *mut libc::c_void);
        free(*d as *mut libc::c_void);
        *d = 0 as *mut krb5_data
    };
}
#[no_mangle]
#[c2rust::src_loc = "1049:1"]
pub unsafe extern "C" fn ktest_empty_data(mut d: *mut krb5_data) {
    if !(*d).data.is_null() {
        free((*d).data as *mut libc::c_void);
        (*d).data = 0 as *mut libc::c_char;
        (*d).length = 0 as libc::c_int as libc::c_uint
    };
}
#[c2rust::src_loc = "1059:1"]
unsafe extern "C" fn ktest_empty_checksum(mut cs: *mut krb5_checksum) {
    free((*cs).contents as *mut libc::c_void);
    (*cs).contents = 0 as *mut krb5_octet;
}
#[no_mangle]
#[c2rust::src_loc = "1066:1"]
pub unsafe extern "C" fn ktest_destroy_checksum(mut cs:
                                                    *mut *mut krb5_checksum) {
    if !(*cs).is_null() {
        free((**cs).contents as *mut libc::c_void);
        free(*cs as *mut libc::c_void);
        *cs = 0 as *mut krb5_checksum
    };
}
#[no_mangle]
#[c2rust::src_loc = "1076:1"]
pub unsafe extern "C" fn ktest_empty_keyblock(mut kb: *mut krb5_keyblock) {
    if !kb.is_null() {
        if !(*kb).contents.is_null() {
            free((*kb).contents as *mut libc::c_void);
            (*kb).contents = 0 as *mut krb5_octet
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1087:1"]
pub unsafe extern "C" fn ktest_destroy_keyblock(mut kb:
                                                    *mut *mut krb5_keyblock) {
    if !(*kb).is_null() {
        free((**kb).contents as *mut libc::c_void);
        free(*kb as *mut libc::c_void);
        *kb = 0 as *mut krb5_keyblock
    };
}
#[no_mangle]
#[c2rust::src_loc = "1097:1"]
pub unsafe extern "C" fn ktest_empty_authorization_data(mut ad:
                                                            *mut *mut krb5_authdata) {
    let mut i: libc::c_int = 0;
    if !(*ad).is_null() {
        i = 0 as libc::c_int;
        while !(*ad.offset(i as isize)).is_null() {
            ktest_destroy_authdata(&mut *ad.offset(i as isize));
            i += 1
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1108:1"]
pub unsafe extern "C" fn ktest_destroy_authorization_data(mut ad:
                                                              *mut *mut *mut krb5_authdata) {
    ktest_empty_authorization_data(*ad);
    free(*ad as *mut libc::c_void);
    *ad = 0 as *mut *mut krb5_authdata;
}
#[no_mangle]
#[c2rust::src_loc = "1116:1"]
pub unsafe extern "C" fn ktest_destroy_authdata(mut ad:
                                                    *mut *mut krb5_authdata) {
    if !(*ad).is_null() {
        free((**ad).contents as *mut libc::c_void);
        free(*ad as *mut libc::c_void);
        *ad = 0 as *mut krb5_authdata
    };
}
#[no_mangle]
#[c2rust::src_loc = "1126:1"]
pub unsafe extern "C" fn ktest_empty_pa_data_array(mut pad:
                                                       *mut *mut krb5_pa_data) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*pad.offset(i as isize)).is_null() {
        ktest_destroy_pa_data(&mut *pad.offset(i as isize));
        i += 1
    };
}
#[no_mangle]
#[c2rust::src_loc = "1135:1"]
pub unsafe extern "C" fn ktest_destroy_pa_data_array(mut pad:
                                                         *mut *mut *mut krb5_pa_data) {
    ktest_empty_pa_data_array(*pad);
    free(*pad as *mut libc::c_void);
    *pad = 0 as *mut *mut krb5_pa_data;
}
#[no_mangle]
#[c2rust::src_loc = "1143:1"]
pub unsafe extern "C" fn ktest_destroy_pa_data(mut pad:
                                                   *mut *mut krb5_pa_data) {
    if !(*pad).is_null() {
        free((**pad).contents as *mut libc::c_void);
        free(*pad as *mut libc::c_void);
        *pad = 0 as *mut krb5_pa_data
    };
}
#[no_mangle]
#[c2rust::src_loc = "1153:1"]
pub unsafe extern "C" fn ktest_destroy_address(mut a:
                                                   *mut *mut krb5_address) {
    if !(*a).is_null() {
        free((**a).contents as *mut libc::c_void);
        free(*a as *mut libc::c_void);
        *a = 0 as *mut krb5_address
    };
}
#[no_mangle]
#[c2rust::src_loc = "1163:1"]
pub unsafe extern "C" fn ktest_empty_addresses(mut a:
                                                   *mut *mut krb5_address) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*a.offset(i as isize)).is_null() {
        ktest_destroy_address(&mut *a.offset(i as isize));
        i += 1
    };
}
#[no_mangle]
#[c2rust::src_loc = "1172:1"]
pub unsafe extern "C" fn ktest_destroy_addresses(mut a:
                                                     *mut *mut *mut krb5_address) {
    ktest_empty_addresses(*a);
    free(*a as *mut libc::c_void);
    *a = 0 as *mut *mut krb5_address;
}
#[no_mangle]
#[c2rust::src_loc = "1180:1"]
pub unsafe extern "C" fn ktest_destroy_principal(mut p: *mut krb5_principal) {
    let mut i: libc::c_int = 0;
    if (*p).is_null() { return }
    i = 0 as libc::c_int;
    while i < (**p).length {
        ktest_empty_data(&mut *(**p).data.offset(i as isize));
        i += 1
    }
    ktest_empty_data(&mut (**p).realm);
    free((**p).data as *mut libc::c_void);
    free(*p as *mut libc::c_void);
    *p = 0 as krb5_principal;
}
#[no_mangle]
#[c2rust::src_loc = "1195:1"]
pub unsafe extern "C" fn ktest_destroy_sequence_of_integer(mut soi:
                                                               *mut *mut libc::c_long) {
    free(*soi as *mut libc::c_void);
    *soi = 0 as *mut libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "1202:1"]
pub unsafe extern "C" fn ktest_destroy_sequence_of_ticket(mut sot:
                                                              *mut *mut *mut krb5_ticket) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*(*sot).offset(i as isize)).is_null() {
        ktest_destroy_ticket(&mut *(*sot).offset(i as isize));
        i += 1
    }
    free(*sot as *mut libc::c_void);
    *sot = 0 as *mut *mut krb5_ticket;
}
#[no_mangle]
#[c2rust::src_loc = "1213:1"]
pub unsafe extern "C" fn ktest_destroy_ticket(mut tkt:
                                                  *mut *mut krb5_ticket) {
    ktest_destroy_principal(&mut (**tkt).server);
    ktest_destroy_enc_data(&mut (**tkt).enc_part);
    /*  ktest_empty_enc_tkt_part(((*tkt)->enc_part2));*/
    free(*tkt as *mut libc::c_void);
    *tkt = 0 as *mut krb5_ticket;
}
#[no_mangle]
#[c2rust::src_loc = "1223:1"]
pub unsafe extern "C" fn ktest_empty_ticket(mut tkt: *mut krb5_ticket) {
    if !(*tkt).server.is_null() {
        ktest_destroy_principal(&mut (*tkt).server);
    }
    ktest_destroy_enc_data(&mut (*tkt).enc_part);
    if !(*tkt).enc_part2.is_null() {
        ktest_destroy_enc_tkt_part(&mut (*tkt).enc_part2);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1233:1"]
pub unsafe extern "C" fn ktest_destroy_enc_data(mut ed: *mut krb5_enc_data) {
    ktest_empty_data(&mut (*ed).ciphertext);
    (*ed).kvno = 0 as libc::c_int as krb5_kvno;
}
#[no_mangle]
#[c2rust::src_loc = "1240:1"]
pub unsafe extern "C" fn ktest_destroy_etype_info_entry(mut i:
                                                            *mut krb5_etype_info_entry) {
    if !(*i).salt.is_null() { free((*i).salt as *mut libc::c_void); }
    ktest_empty_data(&mut (*i).s2kparams);
    free(i as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1249:1"]
pub unsafe extern "C" fn ktest_destroy_etype_info(mut info:
                                                      *mut *mut krb5_etype_info_entry) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*info.offset(i as isize)).is_null() {
        ktest_destroy_etype_info_entry(*info.offset(i as isize));
        i += 1
    }
    free(info as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1259:1"]
pub unsafe extern "C" fn ktest_empty_kdc_req(mut kr: *mut krb5_kdc_req) {
    if !(*kr).padata.is_null() {
        ktest_destroy_pa_data_array(&mut (*kr).padata);
    }
    if !(*kr).client.is_null() { ktest_destroy_principal(&mut (*kr).client); }
    if !(*kr).server.is_null() { ktest_destroy_principal(&mut (*kr).server); }
    free((*kr).ktype as *mut libc::c_void);
    if !(*kr).addresses.is_null() {
        ktest_destroy_addresses(&mut (*kr).addresses);
    }
    ktest_destroy_enc_data(&mut (*kr).authorization_data);
    if !(*kr).unenc_authdata.is_null() {
        ktest_destroy_authorization_data(&mut (*kr).unenc_authdata);
    }
    if !(*kr).second_ticket.is_null() {
        ktest_destroy_sequence_of_ticket(&mut (*kr).second_ticket);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1281:1"]
pub unsafe extern "C" fn ktest_empty_kdc_rep(mut kr: *mut krb5_kdc_rep) {
    if !(*kr).padata.is_null() {
        ktest_destroy_pa_data_array(&mut (*kr).padata);
    }
    if !(*kr).client.is_null() { ktest_destroy_principal(&mut (*kr).client); }
    if !(*kr).ticket.is_null() { ktest_destroy_ticket(&mut (*kr).ticket); }
    ktest_destroy_enc_data(&mut (*kr).enc_part);
    if !(*kr).enc_part2.is_null() {
        ktest_empty_enc_kdc_rep_part((*kr).enc_part2);
        free((*kr).enc_part2 as *mut libc::c_void);
        (*kr).enc_part2 = 0 as *mut krb5_enc_kdc_rep_part
    };
}
#[no_mangle]
#[c2rust::src_loc = "1302:1"]
pub unsafe extern "C" fn ktest_empty_authenticator(mut a:
                                                       *mut krb5_authenticator) {
    if !(*a).client.is_null() { ktest_destroy_principal(&mut (*a).client); }
    if !(*a).checksum.is_null() {
        ktest_destroy_checksum(&mut (*a).checksum);
    }
    if !(*a).subkey.is_null() { ktest_destroy_keyblock(&mut (*a).subkey); }
    if !(*a).authorization_data.is_null() {
        ktest_destroy_authorization_data(&mut (*a).authorization_data);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1315:1"]
pub unsafe extern "C" fn ktest_empty_enc_tkt_part(mut etp:
                                                      *mut krb5_enc_tkt_part) {
    if !(*etp).session.is_null() {
        ktest_destroy_keyblock(&mut (*etp).session);
    }
    if !(*etp).client.is_null() {
        ktest_destroy_principal(&mut (*etp).client);
    }
    if !(*etp).caddrs.is_null() {
        ktest_destroy_addresses(&mut (*etp).caddrs);
    }
    if !(*etp).authorization_data.is_null() {
        ktest_destroy_authorization_data(&mut (*etp).authorization_data);
    }
    ktest_destroy_transited(&mut (*etp).transited);
}
#[no_mangle]
#[c2rust::src_loc = "1329:1"]
pub unsafe extern "C" fn ktest_destroy_enc_tkt_part(mut etp:
                                                        *mut *mut krb5_enc_tkt_part) {
    if !(*etp).is_null() {
        ktest_empty_enc_tkt_part(*etp);
        free(*etp as *mut libc::c_void);
        *etp = 0 as *mut krb5_enc_tkt_part
    };
}
#[no_mangle]
#[c2rust::src_loc = "1339:1"]
pub unsafe extern "C" fn ktest_empty_enc_kdc_rep_part(mut ekr:
                                                          *mut krb5_enc_kdc_rep_part) {
    if !(*ekr).session.is_null() {
        ktest_destroy_keyblock(&mut (*ekr).session);
    }
    if !(*ekr).server.is_null() {
        ktest_destroy_principal(&mut (*ekr).server);
    }
    if !(*ekr).caddrs.is_null() {
        ktest_destroy_addresses(&mut (*ekr).caddrs);
    }
    ktest_destroy_last_req(&mut (*ekr).last_req);
}
#[no_mangle]
#[c2rust::src_loc = "1353:1"]
pub unsafe extern "C" fn ktest_destroy_transited(mut t: *mut krb5_transited) {
    if !(*t).tr_contents.data.is_null() {
        ktest_empty_data(&mut (*t).tr_contents);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1360:1"]
pub unsafe extern "C" fn ktest_empty_ap_rep(mut ar: *mut krb5_ap_rep) {
    ktest_destroy_enc_data(&mut (*ar).enc_part);
}
#[no_mangle]
#[c2rust::src_loc = "1366:1"]
pub unsafe extern "C" fn ktest_empty_ap_req(mut ar: *mut krb5_ap_req) {
    if !(*ar).ticket.is_null() { ktest_destroy_ticket(&mut (*ar).ticket); }
    ktest_destroy_enc_data(&mut (*ar).authenticator);
}
#[no_mangle]
#[c2rust::src_loc = "1374:1"]
pub unsafe extern "C" fn ktest_empty_cred_enc_part(mut cep:
                                                       *mut krb5_cred_enc_part) {
    if !(*cep).s_address.is_null() {
        ktest_destroy_address(&mut (*cep).s_address);
    }
    if !(*cep).r_address.is_null() {
        ktest_destroy_address(&mut (*cep).r_address);
    }
    if !(*cep).ticket_info.is_null() {
        ktest_destroy_sequence_of_cred_info(&mut (*cep).ticket_info);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1385:1"]
pub unsafe extern "C" fn ktest_destroy_cred_info(mut ci:
                                                     *mut *mut krb5_cred_info) {
    if !(**ci).session.is_null() {
        ktest_destroy_keyblock(&mut (**ci).session);
    }
    if !(**ci).client.is_null() {
        ktest_destroy_principal(&mut (**ci).client);
    }
    if !(**ci).server.is_null() {
        ktest_destroy_principal(&mut (**ci).server);
    }
    if !(**ci).caddrs.is_null() {
        ktest_destroy_addresses(&mut (**ci).caddrs);
    }
    free(*ci as *mut libc::c_void);
    *ci = 0 as *mut krb5_cred_info;
}
#[no_mangle]
#[c2rust::src_loc = "1400:1"]
pub unsafe extern "C" fn ktest_destroy_sequence_of_cred_info(mut soci:
                                                                 *mut *mut *mut krb5_cred_info) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*(*soci).offset(i as isize)).is_null() {
        ktest_destroy_cred_info(&mut *(*soci).offset(i as isize));
        i += 1
    }
    free(*soci as *mut libc::c_void);
    *soci = 0 as *mut *mut krb5_cred_info;
}
#[no_mangle]
#[c2rust::src_loc = "1411:1"]
pub unsafe extern "C" fn ktest_empty_safe(mut s: *mut krb5_safe) {
    ktest_empty_data(&mut (*s).user_data);
    ktest_destroy_address(&mut (*s).s_address);
    ktest_destroy_address(&mut (*s).r_address);
    ktest_destroy_checksum(&mut (*s).checksum);
}
#[no_mangle]
#[c2rust::src_loc = "1420:1"]
pub unsafe extern "C" fn ktest_empty_priv_enc_part(mut pep:
                                                       *mut krb5_priv_enc_part) {
    ktest_empty_data(&mut (*pep).user_data);
    ktest_destroy_address(&mut (*pep).s_address);
    ktest_destroy_address(&mut (*pep).r_address);
}
#[no_mangle]
#[c2rust::src_loc = "1428:1"]
pub unsafe extern "C" fn ktest_empty_priv(mut p: *mut krb5_priv) {
    ktest_destroy_enc_data(&mut (*p).enc_part);
}
#[no_mangle]
#[c2rust::src_loc = "1434:1"]
pub unsafe extern "C" fn ktest_empty_cred(mut c: *mut krb5_cred) {
    ktest_destroy_sequence_of_ticket(&mut (*c).tickets);
    ktest_destroy_enc_data(&mut (*c).enc_part);
    /* enc_part2 */
}
#[no_mangle]
#[c2rust::src_loc = "1442:1"]
pub unsafe extern "C" fn ktest_destroy_last_req(mut lr:
                                                    *mut *mut *mut krb5_last_req_entry) {
    let mut i: libc::c_int = 0;
    if !(*lr).is_null() {
        i = 0 as libc::c_int;
        while !(*(*lr).offset(i as isize)).is_null() {
            free(*(*lr).offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(*lr as *mut libc::c_void);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1455:1"]
pub unsafe extern "C" fn ktest_empty_error(mut kerr: *mut krb5_error) {
    if !(*kerr).client.is_null() {
        ktest_destroy_principal(&mut (*kerr).client);
    }
    if !(*kerr).server.is_null() {
        ktest_destroy_principal(&mut (*kerr).server);
    }
    ktest_empty_data(&mut (*kerr).text);
    ktest_empty_data(&mut (*kerr).e_data);
}
#[no_mangle]
#[c2rust::src_loc = "1466:1"]
pub unsafe extern "C" fn ktest_empty_ap_rep_enc_part(mut arep:
                                                         *mut krb5_ap_rep_enc_part) {
    ktest_destroy_keyblock(&mut (*arep).subkey);
}
#[no_mangle]
#[c2rust::src_loc = "1472:1"]
pub unsafe extern "C" fn ktest_empty_sam_challenge_2(mut p:
                                                         *mut krb5_sam_challenge_2) {
    let mut ck: *mut *mut krb5_checksum = 0 as *mut *mut krb5_checksum;
    ktest_empty_data(&mut (*p).sam_challenge_2_body);
    if !(*p).sam_cksum.is_null() {
        ck = (*p).sam_cksum;
        while !(*ck).is_null() {
            ktest_destroy_checksum(ck);
            ck = ck.offset(1)
        }
        free((*p).sam_cksum as *mut libc::c_void);
        (*p).sam_cksum = 0 as *mut *mut krb5_checksum
    };
}
#[no_mangle]
#[c2rust::src_loc = "1486:1"]
pub unsafe extern "C" fn ktest_empty_sam_challenge_2_body(mut p:
                                                              *mut krb5_sam_challenge_2_body) {
    ktest_empty_data(&mut (*p).sam_type_name);
    ktest_empty_data(&mut (*p).sam_track_id);
    ktest_empty_data(&mut (*p).sam_challenge_label);
    ktest_empty_data(&mut (*p).sam_challenge);
    ktest_empty_data(&mut (*p).sam_response_prompt);
    ktest_empty_data(&mut (*p).sam_pk_for_sad);
}
#[no_mangle]
#[c2rust::src_loc = "1497:1"]
pub unsafe extern "C" fn ktest_empty_sam_response_2(mut p:
                                                        *mut krb5_sam_response_2) {
    ktest_empty_data(&mut (*p).sam_track_id);
    ktest_empty_data(&mut (*p).sam_enc_nonce_or_sad.ciphertext);
}
#[no_mangle]
#[c2rust::src_loc = "1504:1"]
pub unsafe extern "C" fn ktest_empty_enc_sam_response_enc_2(mut p:
                                                                *mut krb5_enc_sam_response_enc_2) {
    ktest_empty_data(&mut (*p).sam_sad);
}
#[no_mangle]
#[c2rust::src_loc = "1510:1"]
pub unsafe extern "C" fn ktest_empty_pa_for_user(mut p:
                                                     *mut krb5_pa_for_user) {
    ktest_destroy_principal(&mut (*p).user);
    ktest_empty_checksum(&mut (*p).cksum);
    ktest_empty_data(&mut (*p).auth_package);
}
#[no_mangle]
#[c2rust::src_loc = "1518:1"]
pub unsafe extern "C" fn ktest_empty_pa_s4u_x509_user(mut p:
                                                          *mut krb5_pa_s4u_x509_user) {
    ktest_destroy_principal(&mut (*p).user_id.user);
    ktest_empty_data(&mut (*p).user_id.subject_cert);
    free((*p).cksum.contents as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1526:1"]
pub unsafe extern "C" fn ktest_empty_ad_kdcissued(mut p:
                                                      *mut krb5_ad_kdcissued) {
    free((*p).ad_checksum.contents as *mut libc::c_void);
    ktest_destroy_principal(&mut (*p).i_principal);
    ktest_destroy_authorization_data(&mut (*p).elements);
}
#[no_mangle]
#[c2rust::src_loc = "1534:1"]
pub unsafe extern "C" fn ktest_empty_ad_signedpath_data(mut p:
                                                            *mut krb5_ad_signedpath_data) {
    let mut i: libc::c_int = 0;
    ktest_destroy_principal(&mut (*p).client);
    if !(*p).delegated.is_null() {
        i = 0 as libc::c_int;
        while !(*(*p).delegated.offset(i as isize)).is_null() {
            let mut princ: krb5_principal =
                *(*p).delegated.offset(i as isize);
            ktest_destroy_principal(&mut princ);
            i += 1
        }
        free((*p).delegated as *mut libc::c_void);
    }
    ktest_destroy_pa_data_array(&mut (*p).method_data);
    ktest_destroy_authorization_data(&mut (*p).authorization_data);
}
#[no_mangle]
#[c2rust::src_loc = "1551:1"]
pub unsafe extern "C" fn ktest_empty_ad_signedpath(mut p:
                                                       *mut krb5_ad_signedpath) {
    let mut i: libc::c_int = 0;
    free((*p).checksum.contents as *mut libc::c_void);
    if !(*p).delegated.is_null() {
        i = 0 as libc::c_int;
        while !(*(*p).delegated.offset(i as isize)).is_null() {
            let mut princ: krb5_principal =
                *(*p).delegated.offset(i as isize);
            ktest_destroy_principal(&mut princ);
            i += 1
        }
        free((*p).delegated as *mut libc::c_void);
    }
    ktest_destroy_pa_data_array(&mut (*p).method_data);
}
#[no_mangle]
#[c2rust::src_loc = "1567:1"]
pub unsafe extern "C" fn ktest_empty_iakerb_header(mut p:
                                                       *mut krb5_iakerb_header) {
    krb5_free_data_contents(0 as krb5_context, &mut (*p).target_realm);
    krb5_free_data(0 as krb5_context, (*p).cookie);
}
#[no_mangle]
#[c2rust::src_loc = "1574:1"]
pub unsafe extern "C" fn ktest_empty_iakerb_finished(mut p:
                                                         *mut krb5_iakerb_finished) {
    krb5_free_checksum_contents(0 as krb5_context, &mut (*p).checksum);
}
#[c2rust::src_loc = "1580:1"]
unsafe extern "C" fn ktest_empty_fast_finished(mut p:
                                                   *mut krb5_fast_finished) {
    ktest_destroy_principal(&mut (*p).client);
    ktest_empty_checksum(&mut (*p).ticket_checksum);
}
#[no_mangle]
#[c2rust::src_loc = "1587:1"]
pub unsafe extern "C" fn ktest_empty_fast_response(mut p:
                                                       *mut krb5_fast_response) {
    ktest_destroy_pa_data_array(&mut (*p).padata);
    ktest_destroy_keyblock(&mut (*p).strengthen_key);
    if !(*p).finished.is_null() {
        ktest_empty_fast_finished((*p).finished);
        free((*p).finished as *mut libc::c_void);
        (*p).finished = 0 as *mut krb5_fast_finished
    };
}
#[c2rust::src_loc = "1599:1"]
unsafe extern "C" fn ktest_empty_algorithm_identifier(mut p:
                                                          *mut krb5_algorithm_identifier) {
    ktest_empty_data(&mut (*p).algorithm);
    ktest_empty_data(&mut (*p).parameters);
}
#[no_mangle]
#[c2rust::src_loc = "1606:1"]
pub unsafe extern "C" fn ktest_empty_otp_tokeninfo(mut p:
                                                       *mut krb5_otp_tokeninfo) {
    let mut alg: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    (*p).flags = 0 as libc::c_int;
    krb5_free_data_contents(0 as krb5_context, &mut (*p).vendor);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).challenge);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).token_id);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).alg_id);
    alg = (*p).supported_hash_alg;
    while !alg.is_null() && !(*alg).is_null() {
        ktest_empty_algorithm_identifier(*alg);
        free(*alg as *mut libc::c_void);
        alg = alg.offset(1)
    }
    free((*p).supported_hash_alg as *mut libc::c_void);
    (*p).supported_hash_alg = 0 as *mut *mut krb5_algorithm_identifier;
    (*p).iteration_count = -(1 as libc::c_int);
    (*p).format = (*p).iteration_count;
    (*p).length = (*p).format;
}
#[no_mangle]
#[c2rust::src_loc = "1625:1"]
pub unsafe extern "C" fn ktest_empty_pa_otp_challenge(mut p:
                                                          *mut krb5_pa_otp_challenge) {
    let mut ti: *mut *mut krb5_otp_tokeninfo =
        0 as *mut *mut krb5_otp_tokeninfo;
    krb5_free_data_contents(0 as krb5_context, &mut (*p).nonce);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).service);
    ti = (*p).tokeninfo;
    while !(*ti).is_null() {
        ktest_empty_otp_tokeninfo(*ti);
        free(*ti as *mut libc::c_void);
        ti = ti.offset(1)
    }
    free((*p).tokeninfo as *mut libc::c_void);
    (*p).tokeninfo = 0 as *mut *mut krb5_otp_tokeninfo;
    krb5_free_data_contents(0 as krb5_context, &mut (*p).salt);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).s2kparams);
}
#[no_mangle]
#[c2rust::src_loc = "1642:1"]
pub unsafe extern "C" fn ktest_empty_pa_otp_req(mut p: *mut krb5_pa_otp_req) {
    (*p).flags = 0 as libc::c_int;
    krb5_free_data_contents(0 as krb5_context, &mut (*p).nonce);
    ktest_destroy_enc_data(&mut (*p).enc_data);
    if !(*p).hash_alg.is_null() {
        ktest_empty_algorithm_identifier((*p).hash_alg);
    }
    free((*p).hash_alg as *mut libc::c_void);
    (*p).hash_alg = 0 as *mut krb5_algorithm_identifier;
    (*p).iteration_count = -(1 as libc::c_int);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).otp_value);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).pin);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).challenge);
    (*p).time = 0 as libc::c_int;
    krb5_free_data_contents(0 as krb5_context, &mut (*p).counter);
    (*p).format = -(1 as libc::c_int);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).token_id);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).alg_id);
    krb5_free_data_contents(0 as krb5_context, &mut (*p).vendor);
}
#[c2rust::src_loc = "1666:1"]
unsafe extern "C" fn ktest_empty_pk_authenticator(mut p:
                                                      *mut krb5_pk_authenticator) {
    ktest_empty_checksum(&mut (*p).paChecksum);
    (*p).paChecksum.contents = 0 as *mut krb5_octet;
    krb5_free_data(0 as krb5_context, (*p).freshnessToken);
    (*p).freshnessToken = 0 as *mut krb5_data;
}
#[c2rust::src_loc = "1675:1"]
unsafe extern "C" fn ktest_empty_subject_pk_info(mut p:
                                                     *mut krb5_subject_pk_info) {
    ktest_empty_algorithm_identifier(&mut (*p).algorithm);
    ktest_empty_data(&mut (*p).subjectPublicKey);
}
#[c2rust::src_loc = "1682:1"]
unsafe extern "C" fn ktest_empty_external_principal_identifier(mut p:
                                                                   *mut krb5_external_principal_identifier) {
    ktest_empty_data(&mut (*p).subjectName);
    ktest_empty_data(&mut (*p).issuerAndSerialNumber);
    ktest_empty_data(&mut (*p).subjectKeyIdentifier);
}
#[no_mangle]
#[c2rust::src_loc = "1691:1"]
pub unsafe extern "C" fn ktest_empty_pa_pk_as_req(mut p:
                                                      *mut krb5_pa_pk_as_req) {
    let mut pi: *mut *mut krb5_external_principal_identifier =
        0 as *mut *mut krb5_external_principal_identifier;
    ktest_empty_data(&mut (*p).signedAuthPack);
    pi = (*p).trustedCertifiers;
    while !(*pi).is_null() {
        ktest_empty_external_principal_identifier(*pi);
        free(*pi as *mut libc::c_void);
        pi = pi.offset(1)
    }
    free((*p).trustedCertifiers as *mut libc::c_void);
    (*p).trustedCertifiers =
        0 as *mut *mut krb5_external_principal_identifier;
    ktest_empty_data(&mut (*p).kdcPkId);
}
#[c2rust::src_loc = "1706:1"]
unsafe extern "C" fn ktest_empty_dh_rep_info(mut p: *mut krb5_dh_rep_info) {
    ktest_empty_data(&mut (*p).dhSignedData);
    ktest_empty_data(&mut (*p).serverDHNonce);
    ktest_destroy_data(&mut (*p).kdfID);
}
#[no_mangle]
#[c2rust::src_loc = "1714:1"]
pub unsafe extern "C" fn ktest_empty_pa_pk_as_rep(mut p:
                                                      *mut krb5_pa_pk_as_rep) {
    if (*p).choice as libc::c_int == choice_pa_pk_as_rep_dhInfo as libc::c_int
       {
        ktest_empty_dh_rep_info(&mut (*p).u.dh_Info);
    } else if (*p).choice as libc::c_int ==
                  choice_pa_pk_as_rep_encKeyPack as libc::c_int {
        ktest_empty_data(&mut (*p).u.encKeyPack);
    }
    (*p).choice = choice_pa_pk_as_rep_UNKNOWN;
}
#[no_mangle]
#[c2rust::src_loc = "1724:1"]
pub unsafe extern "C" fn ktest_empty_auth_pack(mut p: *mut krb5_auth_pack) {
    let mut ai: *mut *mut krb5_algorithm_identifier =
        0 as *mut *mut krb5_algorithm_identifier;
    let mut d: *mut *mut krb5_data = 0 as *mut *mut krb5_data;
    ktest_empty_pk_authenticator(&mut (*p).pkAuthenticator);
    if !(*p).clientPublicValue.is_null() {
        ktest_empty_subject_pk_info((*p).clientPublicValue);
        free((*p).clientPublicValue as *mut libc::c_void);
        (*p).clientPublicValue = 0 as *mut krb5_subject_pk_info
    }
    if !(*p).supportedCMSTypes.is_null() {
        ai = (*p).supportedCMSTypes;
        while !(*ai).is_null() {
            ktest_empty_algorithm_identifier(*ai);
            free(*ai as *mut libc::c_void);
            ai = ai.offset(1)
        }
        free((*p).supportedCMSTypes as *mut libc::c_void);
        (*p).supportedCMSTypes = 0 as *mut *mut krb5_algorithm_identifier
    }
    ktest_empty_data(&mut (*p).clientDHNonce);
    if !(*p).supportedKDFs.is_null() {
        d = (*p).supportedKDFs;
        while !(*d).is_null() {
            ktest_empty_data(*d);
            free(*d as *mut libc::c_void);
            d = d.offset(1)
        }
        free((*p).supportedKDFs as *mut libc::c_void);
        (*p).supportedKDFs = 0 as *mut *mut krb5_data
    };
}
#[no_mangle]
#[c2rust::src_loc = "1755:1"]
pub unsafe extern "C" fn ktest_empty_kdc_dh_key_info(mut p:
                                                         *mut krb5_kdc_dh_key_info) {
    ktest_empty_data(&mut (*p).subjectPublicKey);
}
#[no_mangle]
#[c2rust::src_loc = "1761:1"]
pub unsafe extern "C" fn ktest_empty_reply_key_pack(mut p:
                                                        *mut krb5_reply_key_pack) {
    ktest_empty_keyblock(&mut (*p).replyKey);
    ktest_empty_checksum(&mut (*p).asChecksum);
}
#[no_mangle]
#[c2rust::src_loc = "1768:1"]
pub unsafe extern "C" fn ktest_empty_sp80056a_other_info(mut p:
                                                             *mut krb5_sp80056a_other_info) {
    ktest_empty_algorithm_identifier(&mut (*p).algorithm_identifier);
    ktest_destroy_principal(&mut (*p).party_u_info);
    ktest_destroy_principal(&mut (*p).party_v_info);
    ktest_empty_data(&mut (*p).supp_pub_info);
}
#[no_mangle]
#[c2rust::src_loc = "1776:1"]
pub unsafe extern "C" fn ktest_empty_pkinit_supp_pub_info(mut p:
                                                              *mut krb5_pkinit_supp_pub_info) {
    ktest_empty_data(&mut (*p).as_req);
    ktest_empty_data(&mut (*p).pk_as_rep);
}
/* not DISABLE_PKINIT */
#[no_mangle]
#[c2rust::src_loc = "1798:1"]
pub unsafe extern "C" fn ktest_empty_kkdcp_message(mut p:
                                                       *mut krb5_kkdcp_message) {
    ktest_empty_data(&mut (*p).kerb_message);
    ktest_empty_data(&mut (*p).target_domain);
    (*p).dclocator_hint = -(1 as libc::c_int);
}
#[c2rust::src_loc = "1806:1"]
unsafe extern "C" fn destroy_verifier_mac(mut vmac:
                                              *mut *mut krb5_verifier_mac) {
    if (*vmac).is_null() { return }
    ktest_destroy_principal(&mut (**vmac).princ);
    ktest_empty_checksum(&mut (**vmac).checksum);
    free(*vmac as *mut libc::c_void);
    *vmac = 0 as *mut krb5_verifier_mac;
}
#[no_mangle]
#[c2rust::src_loc = "1817:1"]
pub unsafe extern "C" fn ktest_empty_cammac(mut p: *mut krb5_cammac) {
    let mut vmacp: *mut *mut krb5_verifier_mac =
        0 as *mut *mut krb5_verifier_mac;
    ktest_destroy_authorization_data(&mut (*p).elements);
    destroy_verifier_mac(&mut (*p).kdc_verifier);
    destroy_verifier_mac(&mut (*p).svc_verifier);
    vmacp = (*p).other_verifiers;
    while !vmacp.is_null() && !(*vmacp).is_null() {
        destroy_verifier_mac(vmacp);
        vmacp = vmacp.offset(1)
    }
    free((*p).other_verifiers as *mut libc::c_void);
    (*p).other_verifiers = 0 as *mut *mut krb5_verifier_mac;
}
#[no_mangle]
#[c2rust::src_loc = "1831:1"]
pub unsafe extern "C" fn ktest_empty_secure_cookie(mut p:
                                                       *mut krb5_secure_cookie) {
    ktest_empty_pa_data_array((*p).data);
}
#[no_mangle]
#[c2rust::src_loc = "1837:1"]
pub unsafe extern "C" fn ktest_empty_spake_factor(mut p:
                                                      *mut krb5_spake_factor) {
    krb5_free_data(0 as krb5_context, (*p).data);
    (*p).data = 0 as *mut krb5_data;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/asn.1/ktest.h */
/*
 * Copyright (C) 1994 by the Massachusetts Institute of Technology.
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
/* Fri Jun 10  6:03:17 GMT 1994 */
/*----------------------------------------------------------------------*/
#[no_mangle]
#[c2rust::src_loc = "1844:1"]
pub unsafe extern "C" fn ktest_empty_pa_spake(mut p: *mut krb5_pa_spake) {
    let mut f: *mut *mut krb5_spake_factor = 0 as *mut *mut krb5_spake_factor;
    match (*p).choice as libc::c_int {
        0 => { free((*p).u.support.groups as *mut libc::c_void); }
        1 => {
            ktest_empty_data(&mut (*p).u.challenge.pubkey);
            f = (*p).u.challenge.factors;
            while !(*f).is_null() {
                ktest_empty_spake_factor(*f);
                free(*f as *mut libc::c_void);
                f = f.offset(1)
            }
            free((*p).u.challenge.factors as *mut libc::c_void);
        }
        2 => {
            ktest_empty_data(&mut (*p).u.response.pubkey);
            ktest_destroy_enc_data(&mut (*p).u.response.factor);
        }
        3 => { ktest_destroy_enc_data(&mut (*p).u.encdata); }
        _ => { }
    }
    (*p).choice = SPAKE_MSGTYPE_UNKNOWN;
}
