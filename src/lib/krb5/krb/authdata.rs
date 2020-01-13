use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
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
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
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
    /* *< Transited encoding type */
    /* *< Contents */
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
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
    /* Flags for krb5_auth_con_genaddrs(). */
    /* * Generate the local network address. */
    /* * Generate the remote network address.  */
    /* * Generate the local network address and the local port. */
    /* * Generate the remote network address and the remote port. */
    /* * Type of function used as a callback to generate checksum data for mk_req */
    #[c2rust::src_loc = "2264:1"]
    pub type krb5_mk_req_checksum_func
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_auth_context,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "2710:1"]
    pub type krb5_rcache = *mut krb5_rc_st;
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
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
    /* checksum type */
    /* *
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
    #[c2rust::src_loc = "379:1"]
    pub type krb5_key = *mut krb5_key_st;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, krb5_key_st};
    use super::auth_con_h::_krb5_auth_context;
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
        /*
 * end "ccache.h"
 */
        /*
 * begin "rcache.h"
 */
        #[c2rust::src_loc = "2709:8"]
        pub type krb5_rc_st;
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
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "8120:1"]
        pub fn krb5_verify_authdata_kdc_issued(context: krb5_context,
                                               key: *const krb5_keyblock,
                                               ad_kdcissued:
                                                   *const krb5_authdata,
                                               issuer: *mut krb5_principal,
                                               authdata:
                                                   *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
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
    #[c2rust::src_loc = "996:1"]
    pub type krb5_authdata_context = *mut _krb5_authdata_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "978:8"]
    pub struct _krb5_authdata_context {
        pub magic: krb5_magic,
        pub n_modules: libc::c_int,
        pub modules: *mut _krb5_authdata_context_module,
        pub plugins: plugin_dir_handle,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "981:12"]
    pub struct _krb5_authdata_context_module {
        pub ad_type: krb5_authdatatype,
        pub plugin_context: *mut libc::c_void,
        pub client_fini: authdata_client_plugin_fini_proc,
        pub flags: krb5_flags,
        pub ftable: *mut krb5plugin_authdata_client_ftable_v0,
        pub client_req_init: authdata_client_request_init_proc,
        pub client_req_fini: authdata_client_request_fini_proc,
        pub name: *const libc::c_char,
        pub request_context: *mut libc::c_void,
        pub request_context_pp: *mut *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "628:8"]
    pub struct krb5_key_st {
        pub keyblock: krb5_keyblock,
        pub refcount: libc::c_int,
        pub derived: *mut derived_key,
        pub cache: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "621:8"]
    pub struct derived_key {
        pub constant: krb5_data,
        pub dkey: krb5_key,
        pub next: *mut derived_key,
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_authdatatype, krb5_keyblock, krb5_data, krb5_key,
                        krb5_context, krb5_authdata, krb5_error_code,
                        krb5_octet};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
    use super::stddef_h::size_t;
    use super::string_h::{explicit_bzero, strlen, memcmp};
    use super::stdlib_h::free;
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
        #[c2rust::src_loc = "966:1"]
        pub fn k5_unwrap_cammac_svc(context: krb5_context,
                                    ad: *const krb5_authdata,
                                    key: *const krb5_keyblock,
                                    adata_out: *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1004:1"]
        pub fn krb5int_free_data_list(context: krb5_context,
                                      data: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "1891:1"]
        pub fn krb5_ser_unpack_int32(_: *mut krb5_int32,
                                     _: *mut *mut krb5_octet, _: *mut size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1888:1"]
        pub fn krb5_ser_pack_int32(_: krb5_int32, _: *mut *mut krb5_octet,
                                   _: *mut size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1902:1"]
        pub fn krb5_ser_pack_bytes(_: *mut krb5_octet, _: size_t,
                                   _: *mut *mut krb5_octet, _: *mut size_t)
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
    use super::k5_err_h::errinfo;
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
        #[no_mangle]
        #[c2rust::src_loc = "103:1"]
        pub fn krb5int_open_plugin_dirs(_: *const *const libc::c_char,
                                        _: *const *const libc::c_char,
                                        _: *mut plugin_dir_handle,
                                        _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "106:1"]
        pub fn krb5int_close_plugin_dirs(_: *mut plugin_dir_handle);
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn krb5int_get_plugin_dir_data(_: *mut plugin_dir_handle,
                                           _: *const libc::c_char,
                                           _: *mut *mut *mut libc::c_void,
                                           _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn krb5int_free_plugin_dir_data(_: *mut *mut libc::c_void);
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/auth_con.h:29"]
pub mod auth_con_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct _krb5_auth_context {
        pub magic: krb5_magic,
        pub remote_addr: *mut krb5_address,
        pub remote_port: *mut krb5_address,
        pub local_addr: *mut krb5_address,
        pub local_port: *mut krb5_address,
        pub key: krb5_key,
        pub send_subkey: krb5_key,
        pub recv_subkey: krb5_key,
        pub auth_context_flags: krb5_int32,
        pub remote_seq_number: krb5_ui_4,
        pub local_seq_number: krb5_ui_4,
        pub authentp: *mut krb5_authenticator,
        pub req_cksumtype: krb5_cksumtype,
        pub safe_cksumtype: krb5_cksumtype,
        pub cstate: krb5_data,
        pub rcache: krb5_rcache,
        pub memrcache: k5_memrcache,
        pub permitted_etypes: *mut krb5_enctype,
        pub checksum_func: krb5_mk_req_checksum_func,
        pub checksum_func_data: *mut libc::c_void,
        pub negotiated_etype: krb5_enctype,
        pub ad_context: krb5_authdata_context,
    }
    use super::krb5_h::{krb5_magic, krb5_address, krb5_key, krb5_int32,
                        krb5_ui_4, krb5_authenticator, krb5_cksumtype,
                        krb5_data, krb5_rcache, krb5_enctype,
                        krb5_mk_req_checksum_func};
    use super::memrcache_h::k5_memrcache;
    use super::k5_int_h::krb5_authdata_context;
    /* Internal auth_context_flags */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:27"]
pub mod authdata_plugin_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2007 Apple Inc.  All Rights Reserved.
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
 * Authorization data plugin definitions for Kerberos 5.
 * This is considered an INTERNAL interface at this time.
 *
 * Some work is needed before exporting it:
 *
 * + Documentation.
 * + Sample code.
 * + Test cases (preferably automated testing under "make check").
 *
 * Other changes that would be nice to have, but not necessarily
 * before making this interface public:
 *
 * + Library support for AD-IF-RELEVANT and similar wrappers.  (We can
 *   make the plugin construct them if it wants them.)
 * + KDC could combine/optimize wrapped AD elements provided by
 *   multiple plugins, e.g., two IF-RELEVANT sequences could be
 *   merged.  (The preauth plugin API also has this bug, we're going
 *   to need a general fix.)
 */
    #[c2rust::src_loc = "80:1"]
    pub type authdata_client_request_fini_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "74:1"]
    pub type authdata_client_request_init_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "193:16"]
    pub struct krb5plugin_authdata_client_ftable_v0 {
        pub name: *mut libc::c_char,
        pub ad_type_list: *mut krb5_authdatatype,
        pub init: authdata_client_plugin_init_proc,
        pub fini: authdata_client_plugin_fini_proc,
        pub flags: authdata_client_plugin_flags_proc,
        pub request_init: authdata_client_request_init_proc,
        pub request_fini: authdata_client_request_fini_proc,
        pub get_attribute_types: authdata_client_get_attribute_types_proc,
        pub get_attribute: authdata_client_get_attribute_proc,
        pub set_attribute: authdata_client_set_attribute_proc,
        pub delete_attribute: authdata_client_delete_attribute_proc,
        pub export_authdata: authdata_client_export_authdata_proc,
        pub import_authdata: authdata_client_import_authdata_proc,
        pub export_internal: authdata_client_export_internal_proc,
        pub free_internal: authdata_client_free_internal_proc,
        pub verify: authdata_client_verify_proc,
        pub size: authdata_client_size_proc,
        pub externalize: authdata_client_externalize_proc,
        pub internalize: authdata_client_internalize_proc,
        pub copy: authdata_client_copy_proc,
    }
    #[c2rust::src_loc = "185:1"]
    pub type authdata_client_copy_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void) -> krb5_error_code>;
    #[c2rust::src_loc = "177:1"]
    pub type authdata_client_internalize_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_octet, _: *mut size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "169:1"]
    pub type authdata_client_externalize_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_octet, _: *mut size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "162:1"]
    pub type authdata_client_size_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: *mut size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "153:1"]
    pub type authdata_client_verify_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *const krb5_auth_context,
                                    _: *const krb5_keyblock,
                                    _: *const krb5_ap_req)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "146:1"]
    pub type authdata_client_free_internal_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "138:1"]
    pub type authdata_client_export_internal_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: krb5_boolean,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "86:1"]
    pub type authdata_client_import_authdata_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_authdata,
                                    _: krb5_boolean, _: krb5_const_principal)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "95:1"]
    pub type authdata_client_export_authdata_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: krb5_flags,
                                    _: *mut *mut *mut krb5_authdata)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "131:1"]
    pub type authdata_client_delete_attribute_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: *const krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "122:1"]
    pub type authdata_client_set_attribute_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: krb5_boolean,
                                    _: *const krb5_data, _: *const krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "110:1"]
    pub type authdata_client_get_attribute_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void, _: *const krb5_data,
                                    _: *mut krb5_boolean,
                                    _: *mut krb5_boolean, _: *mut krb5_data,
                                    _: *mut krb5_data, _: *mut libc::c_int)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "103:1"]
    pub type authdata_client_get_attribute_types_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut _krb5_authdata_context,
                                    _: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "64:1"]
    pub type authdata_client_plugin_flags_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_authdatatype, _: *mut krb5_flags)
                   -> ()>;
    #[c2rust::src_loc = "70:1"]
    pub type authdata_client_plugin_fini_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void)
                   -> ()>;
    #[c2rust::src_loc = "50:1"]
    pub type authdata_client_plugin_init_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_authdatatype,
                        krb5_octet, krb5_auth_context, krb5_keyblock,
                        krb5_ap_req, krb5_boolean, krb5_authdata,
                        krb5_const_principal, krb5_flags, krb5_data};
    use super::k5_int_h::_krb5_authdata_context;
    use super::stddef_h::size_t;
    /* KRB5_AUTHDATA_PLUGIN_H_INCLUDED */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/rcache/memrcache.h:29"]
pub mod memrcache_h {
    #[c2rust::src_loc = "36:1"]
    pub type k5_memrcache = *mut k5_memrcache_st;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/rcache/memrcache.h - declarations for in-memory replay cache */
/*
 * Copyright (C) 2019 by the Massachusetts Institute of Technology.
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
        #[c2rust::src_loc = "36:16"]
        pub type k5_memrcache_st;
    }
    /* MEMRCACHE_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/assert.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/authdata.h:28"]
pub mod authdata_h {
    use super::authdata_plugin_h::{krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_plugin_init_proc,
                                   authdata_client_plugin_fini_proc,
                                   authdata_client_plugin_flags_proc,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc,
                                   authdata_client_get_attribute_types_proc,
                                   authdata_client_get_attribute_proc,
                                   authdata_client_set_attribute_proc,
                                   authdata_client_delete_attribute_proc,
                                   authdata_client_export_authdata_proc,
                                   authdata_client_import_authdata_proc,
                                   authdata_client_export_internal_proc,
                                   authdata_client_free_internal_proc,
                                   authdata_client_verify_proc,
                                   authdata_client_size_proc,
                                   authdata_client_externalize_proc,
                                   authdata_client_internalize_proc,
                                   authdata_client_copy_proc};
    use super::krb5_h::krb5_authdatatype;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:45"]
        pub static mut k5_mspac_ad_client_ftable:
                   krb5plugin_authdata_client_ftable_v0;
        #[no_mangle]
        #[c2rust::src_loc = "80:45"]
        pub static mut k5_s4u2proxy_ad_client_ftable:
                   krb5plugin_authdata_client_ftable_v0;
        #[no_mangle]
        #[c2rust::src_loc = "81:45"]
        pub static mut k5_authind_ad_client_ftable:
                   krb5plugin_authdata_client_ftable_v0;
    }
    /* !KRB_AUTHDATA_H */
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_ap_req, _krb5_ap_req,
                       krb5_enc_data, _krb5_enc_data, krb5_ticket,
                       _krb5_ticket, krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_authdata, _krb5_authdata, krb5_ticket_times,
                       _krb5_ticket_times, krb5_transited, _krb5_transited,
                       krb5_keyblock, _krb5_keyblock, krb5_auth_context,
                       krb5_mk_req_checksum_func, krb5_rcache,
                       krb5_authenticator, _krb5_authenticator, krb5_checksum,
                       _krb5_checksum, krb5_key, _profile_t, krb5_rc_st,
                       krb5_find_authdata, krb5_free_principal,
                       krb5_free_data, krb5_verify_authdata_kdc_issued};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_authdata_context,
                         _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_key_st,
                         derived_key, zapfree, data_eq_string, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, k5_unwrap_cammac_svc,
                         krb5int_free_data_list, krb5_ser_unpack_int32,
                         krb5_ser_pack_int32, krb5_ser_pack_bytes};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle,
                            krb5int_open_plugin_dirs,
                            krb5int_close_plugin_dirs,
                            krb5int_get_plugin_dir_data,
                            krb5int_free_plugin_dir_data};
pub use self::profile_h::profile_t;
pub use self::auth_con_h::_krb5_auth_context;
pub use self::authdata_plugin_h::{authdata_client_request_fini_proc,
                                  authdata_client_request_init_proc,
                                  krb5plugin_authdata_client_ftable_v0,
                                  authdata_client_copy_proc,
                                  authdata_client_internalize_proc,
                                  authdata_client_externalize_proc,
                                  authdata_client_size_proc,
                                  authdata_client_verify_proc,
                                  authdata_client_free_internal_proc,
                                  authdata_client_export_internal_proc,
                                  authdata_client_import_authdata_proc,
                                  authdata_client_export_authdata_proc,
                                  authdata_client_delete_attribute_proc,
                                  authdata_client_set_attribute_proc,
                                  authdata_client_get_attribute_proc,
                                  authdata_client_get_attribute_types_proc,
                                  authdata_client_plugin_flags_proc,
                                  authdata_client_plugin_fini_proc,
                                  authdata_client_plugin_init_proc};
pub use self::memrcache_h::{k5_memrcache, k5_memrcache_st};
use self::stdlib_h::{malloc, calloc, realloc, free};
use self::string_h::{explicit_bzero, strlen, strcmp, memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
use self::authdata_h::{k5_mspac_ad_client_ftable,
                       k5_s4u2proxy_ad_client_ftable,
                       k5_authind_ad_client_ftable};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009 by the Massachusetts Institute of Technology.  All
 * Rights Reserved.
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
/* Loosely based on preauth2.c */
#[c2rust::src_loc = "36:20"]
static mut objdirs: [*const libc::c_char; 2] =
    [b"/usr/local/lib/krb5/plugins/authdata\x00" as *const u8 as
         *const libc::c_char, 0 as *const libc::c_char];
/* should be a list */
/* Internal authdata systems */
#[c2rust::src_loc = "45:46"]
static mut authdata_systems: [*mut krb5plugin_authdata_client_ftable_v0; 4] =
    unsafe {
        [&k5_mspac_ad_client_ftable as
             *const krb5plugin_authdata_client_ftable_v0 as
             *mut krb5plugin_authdata_client_ftable_v0,
         &k5_s4u2proxy_ad_client_ftable as
             *const krb5plugin_authdata_client_ftable_v0 as
             *mut krb5plugin_authdata_client_ftable_v0,
         &k5_authind_ad_client_ftable as
             *const krb5plugin_authdata_client_ftable_v0 as
             *mut krb5plugin_authdata_client_ftable_v0,
         0 as *const krb5plugin_authdata_client_ftable_v0 as
             *mut krb5plugin_authdata_client_ftable_v0]
    };
#[inline]
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn k5_ad_module_count(mut table:
                                            *mut krb5plugin_authdata_client_ftable_v0)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if (*table).ad_type_list.is_null() { return 0 as libc::c_int }
    i = 0 as libc::c_int;
    while *(*table).ad_type_list.offset(i as isize) != 0 { i += 1 }
    return i;
}
#[c2rust::src_loc = "66:1"]
unsafe extern "C" fn k5_ad_init_modules(mut kcontext: krb5_context,
                                        mut context: krb5_authdata_context,
                                        mut table:
                                            *mut krb5plugin_authdata_client_ftable_v0,
                                        mut module_count: *mut libc::c_int)
 -> krb5_error_code {
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = *module_count;
    let mut code: krb5_error_code = 0;
    let mut plugin_context: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rcpp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    if (*table).ad_type_list.is_null() { return 2 as libc::c_int }
    if (*table).init.is_none() { return 38 as libc::c_int }
    code =
        Some((*table).init.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                    &mut plugin_context);
    if code != 0 as libc::c_int { return code }
    j = 0 as libc::c_int;
    while *(*table).ad_type_list.offset(j as isize) != 0 as libc::c_int {
        (*(*context).modules.offset(k as isize)).ad_type =
            *(*table).ad_type_list.offset(j as isize);
        let ref mut fresh0 =
            (*(*context).modules.offset(k as isize)).plugin_context;
        *fresh0 = plugin_context;
        if j == 0 as libc::c_int {
            let ref mut fresh1 =
                (*(*context).modules.offset(k as isize)).client_fini;
            *fresh1 = (*table).fini
        } else {
            let ref mut fresh2 =
                (*(*context).modules.offset(k as isize)).client_fini;
            *fresh2 = None
        }
        let ref mut fresh3 = (*(*context).modules.offset(k as isize)).ftable;
        *fresh3 = table;
        let ref mut fresh4 = (*(*context).modules.offset(k as isize)).name;
        *fresh4 = (*table).name;
        if (*table).flags.is_some() {
            Some((*table).flags.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                         plugin_context,
                                                                                                         (*(*context).modules.offset(k
                                                                                                                                         as
                                                                                                                                         isize)).ad_type,
                                                                                                         &mut (*(*context).modules.offset(k
                                                                                                                                              as
                                                                                                                                              isize)).flags);
        } else {
            (*(*context).modules.offset(k as isize)).flags = 0 as libc::c_int
        }
        let ref mut fresh5 =
            (*(*context).modules.offset(k as isize)).request_context;
        *fresh5 = 0 as *mut libc::c_void;
        if j == 0 as libc::c_int {
            let ref mut fresh6 =
                (*(*context).modules.offset(k as isize)).client_req_init;
            *fresh6 = (*table).request_init;
            let ref mut fresh7 =
                (*(*context).modules.offset(k as isize)).client_req_fini;
            *fresh7 = (*table).request_fini;
            rcpp =
                &mut (*(*context).modules.offset(k as isize)).request_context;
            /* For now, single request per context. That may change */
            code =
                Some((*table).request_init.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                    context,
                                                                                                                    plugin_context,
                                                                                                                    rcpp);
            if code != 0 as libc::c_int && code != 12 as libc::c_int &&
                   (*(*context).modules.offset(k as isize)).flags &
                       0x10 as libc::c_int != 0 {
                code = 0 as libc::c_int
            }
            if code != 0 as libc::c_int { break ; }
        } else {
            let ref mut fresh8 =
                (*(*context).modules.offset(k as isize)).client_req_init;
            *fresh8 = None;
            let ref mut fresh9 =
                (*(*context).modules.offset(k as isize)).client_req_fini;
            *fresh9 = None
        }
        let ref mut fresh10 =
            (*(*context).modules.offset(k as isize)).request_context_pp;
        *fresh10 = rcpp;
        k += 1;
        j += 1
    }
    *module_count = k;
    return code;
}
/*
 * Determine size of to-be-externalized authdata context, for
 * modules that match given flags mask. Note that this size
 * does not include the magic identifier/trailer.
 */
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn k5_ad_size(mut kcontext: krb5_context,
                                mut context: krb5_authdata_context,
                                mut flags: krb5_flags, mut sizep: *mut size_t)
 -> krb5_error_code {
    let mut i: libc::c_int = 0; /* count */
    let mut code: krb5_error_code = 0 as libc::c_int;
    *sizep =
        (*sizep as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<krb5_int32>()
                                             as libc::c_ulong) as size_t as
            size_t;
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        let mut size: size_t = 0;
        if !((*module).flags & flags == 0 as libc::c_int) {
            /* externalize request context for the first instance only */
            if !(*module).client_req_init.is_none() {
                if !(*(*module).ftable).size.is_none() {
                    if (*(*module).ftable).externalize.is_some() {
                    } else {
                        __assert_fail(b"module->ftable->externalize != NULL\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"authdata.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      178 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 86],
                                                                &[libc::c_char; 86]>(b"krb5_error_code k5_ad_size(krb5_context, krb5_authdata_context, krb5_flags, size_t *)\x00")).as_ptr());
                    }
                    size =
                        (::std::mem::size_of::<krb5_int32>() as
                             libc::c_ulong).wrapping_add(strlen((*module).name));
                    code =
                        Some((*(*module).ftable).size.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                               context,
                                                                                                                               (*module).plugin_context,
                                                                                                                               *(*module).request_context_pp,
                                                                                                                               &mut size);
                    if code != 0 as libc::c_int { break ; }
                    *sizep =
                        (*sizep as libc::c_ulong).wrapping_add(size) as size_t
                            as size_t
                }
            }
        }
        i += 1
    }
    return code;
}
/*
 * Externalize authdata context, for modules that match given flags
 * mask. Note that the magic identifier/trailer is not included.
 */
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn k5_ad_externalize(mut kcontext: krb5_context,
                                       mut context: krb5_authdata_context,
                                       mut flags: krb5_flags,
                                       mut buffer: *mut *mut krb5_octet,
                                       mut lenremain: *mut size_t)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 0;
    let mut ad_count: krb5_int32 = 0 as libc::c_int;
    let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut remain: size_t = 0;
    bp = *buffer;
    remain = *lenremain;
    /* placeholder for count */
    code = krb5_ser_pack_int32(0 as libc::c_int, &mut bp, &mut remain);
    if code != 0 as libc::c_int { return code }
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        let mut namelen: size_t = 0;
        if !((*module).flags & flags == 0 as libc::c_int) {
            /* externalize request context for the first instance only */
            if !(*module).client_req_init.is_none() {
                if !(*(*module).ftable).externalize.is_none() {
                    /*
         * We use the module name rather than the authdata type, because
         * there may be multiple modules for a particular authdata type.
         */
                    namelen = strlen((*module).name);
                    code =
                        krb5_ser_pack_int32(namelen as krb5_int32, &mut bp,
                                            &mut remain);
                    if code != 0 as libc::c_int { break ; }
                    code =
                        krb5_ser_pack_bytes((*module).name as *mut krb5_octet,
                                            namelen, &mut bp, &mut remain);
                    if code != 0 as libc::c_int { break ; }
                    code =
                        Some((*(*module).ftable).externalize.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                      context,
                                                                                                                                      (*module).plugin_context,
                                                                                                                                      *(*module).request_context_pp,
                                                                                                                                      &mut bp,
                                                                                                                                      &mut remain);
                    if code != 0 as libc::c_int { break ; }
                    ad_count += 1
                }
            }
        }
        i += 1
    }
    if code == 0 as libc::c_int {
        /* store actual count */
        krb5_ser_pack_int32(ad_count, buffer, lenremain);
        *buffer = bp;
        *lenremain = remain
    }
    return code;
}
/*
 * Find authdata module for authdata type that matches flag mask
 */
#[c2rust::src_loc = "276:1"]
unsafe extern "C" fn k5_ad_find_module(mut kcontext: krb5_context,
                                       mut context: krb5_authdata_context,
                                       mut flags: krb5_flags,
                                       mut name: *const krb5_data)
 -> *mut _krb5_authdata_context_module {
    let mut i: libc::c_int = 0;
    let mut ret: *mut _krb5_authdata_context_module =
        0 as *mut _krb5_authdata_context_module;
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        if !((*module).flags & flags == 0 as libc::c_int) {
            /* internalize request context for the first instance only */
            if !(*module).client_req_init.is_none() {
                /* check for name match */
                if !(data_eq_string(*name, (*module).name) == 0) {
                    ret = module;
                    break ;
                }
            }
        }
        i += 1
    }
    return ret;
}
/*
 * In-place internalize authdata context, for modules that match given
 * flags mask. The magic identifier/trailer is not expected by this.
 */
#[c2rust::src_loc = "310:1"]
unsafe extern "C" fn k5_ad_internalize(mut kcontext: krb5_context,
                                       mut context: krb5_authdata_context,
                                       mut flags: krb5_flags,
                                       mut buffer: *mut *mut krb5_octet,
                                       mut lenremain: *mut size_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut i: krb5_int32 = 0;
    let mut count: krb5_int32 = 0;
    let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut remain: size_t = 0;
    bp = *buffer;
    remain = *lenremain;
    code = krb5_ser_unpack_int32(&mut count, &mut bp, &mut remain);
    if code != 0 as libc::c_int { return code }
    i = 0 as libc::c_int;
    while i < count {
        let mut module: *mut _krb5_authdata_context_module =
            0 as *mut _krb5_authdata_context_module;
        let mut namelen: krb5_int32 = 0;
        let mut name: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        code = krb5_ser_unpack_int32(&mut namelen, &mut bp, &mut remain);
        if code != 0 as libc::c_int { break ; }
        if remain < namelen as size_t {
            code = 12 as libc::c_int;
            break ;
        } else {
            name.length = namelen as libc::c_uint;
            name.data = bp as *mut libc::c_char;
            module = k5_ad_find_module(kcontext, context, flags, &mut name);
            if module.is_null() || (*(*module).ftable).internalize.is_none() {
                code = 22 as libc::c_int;
                break ;
            } else {
                bp = bp.offset(namelen as isize);
                remain =
                    (remain as
                         libc::c_ulong).wrapping_sub(namelen as libc::c_ulong)
                        as size_t as size_t;
                code =
                    Some((*(*module).ftable).internalize.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                  context,
                                                                                                                                  (*module).plugin_context,
                                                                                                                                  *(*module).request_context_pp,
                                                                                                                                  &mut bp,
                                                                                                                                  &mut remain);
                if code != 0 as libc::c_int { break ; }
                i += 1
            }
        }
    }
    if code == 0 as libc::c_int { *buffer = bp; *lenremain = remain }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "373:1"]
pub unsafe extern "C" fn krb5_authdata_context_init(mut kcontext:
                                                        krb5_context,
                                                    mut pcontext:
                                                        *mut krb5_authdata_context)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut n_modules: libc::c_int = 0;
    let mut n_tables: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tables: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut table: *mut krb5plugin_authdata_client_ftable_v0 =
        0 as *mut krb5plugin_authdata_client_ftable_v0;
    let mut context: krb5_authdata_context = 0 as krb5_authdata_context;
    let mut internal_count: libc::c_int = 0 as libc::c_int;
    let mut plugins: plugin_dir_handle =
        plugin_dir_handle{files: 0 as *mut *mut plugin_file_handle,};
    let mut code: krb5_error_code = 0;
    *pcontext = 0 as krb5_authdata_context;
    memset(&mut plugins as *mut plugin_dir_handle as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<plugin_dir_handle>() as libc::c_ulong);
    n_modules = 0 as libc::c_int;
    n_tables = 0 as libc::c_int;
    while !authdata_systems[n_tables as usize].is_null() {
        n_modules += k5_ad_module_count(authdata_systems[n_tables as usize]);
        n_tables += 1
    }
    internal_count = n_tables;
    if (plugins.files !=
            0 as *mut libc::c_void as *mut *mut plugin_file_handle) as
           libc::c_int == 0 as libc::c_int &&
           krb5int_open_plugin_dirs(objdirs.as_mut_ptr(),
                                    0 as *const *const libc::c_char,
                                    &mut plugins, &mut (*kcontext).err) ==
               0 as libc::c_int as libc::c_long &&
           krb5int_get_plugin_dir_data(&mut plugins,
                                       b"authdata_client_0\x00" as *const u8
                                           as *const libc::c_char,
                                       &mut tables, &mut (*kcontext).err) ==
               0 as libc::c_int as libc::c_long && !tables.is_null() {
        while !(*tables.offset((n_tables - internal_count) as
                                   isize)).is_null() {
            table =
                *tables.offset((n_tables - internal_count) as isize) as
                    *mut krb5plugin_authdata_client_ftable_v0;
            n_modules += k5_ad_module_count(table);
            n_tables += 1
        }
    }
    context =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_krb5_authdata_context>() as
                   libc::c_ulong) as krb5_authdata_context;
    if context.is_null() {
        code = 12 as libc::c_int
    } else {
        (*context).magic = -(1760647364 as libc::c_long) as krb5_magic;
        (*context).modules =
            calloc(n_modules as libc::c_ulong,
                   ::std::mem::size_of::<_krb5_authdata_context_module>() as
                       libc::c_ulong) as *mut _krb5_authdata_context_module;
        if (*context).modules.is_null() {
            code = 12 as libc::c_int
        } else {
            (*context).n_modules = n_modules;
            /* fill in the structure */
            i = 0 as libc::c_int;
            k = 0 as libc::c_int;
            code = 0 as libc::c_int;
            loop  {
                if !(i < n_tables - internal_count) {
                    current_block = 11636175345244025579;
                    break ;
                }
                code =
                    k5_ad_init_modules(kcontext, context,
                                       *tables.offset(i as isize) as
                                           *mut krb5plugin_authdata_client_ftable_v0,
                                       &mut k);
                if code != 0 as libc::c_int {
                    current_block = 11611314597530049747;
                    break ;
                }
                i += 1
            }
            match current_block {
                11611314597530049747 => { }
                _ => {
                    i = 0 as libc::c_int;
                    loop  {
                        if !(i < internal_count) {
                            current_block = 7245201122033322888;
                            break ;
                        }
                        code =
                            k5_ad_init_modules(kcontext, context,
                                               authdata_systems[i as usize],
                                               &mut k);
                        if code != 0 as libc::c_int {
                            current_block = 11611314597530049747;
                            break ;
                        }
                        i += 1
                    }
                    match current_block {
                        11611314597530049747 => { }
                        _ => { (*context).plugins = plugins }
                    }
                }
            }
        }
    }
    if !tables.is_null() { krb5int_free_plugin_dir_data(tables); }
    if code != 0 as libc::c_int {
        krb5int_close_plugin_dirs(&mut plugins);
        krb5_authdata_context_free(kcontext, context);
    } else {
        /* plugins is owned by context now */
        *pcontext = context
    }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "453:1"]
pub unsafe extern "C" fn krb5_authdata_context_free(mut kcontext:
                                                        krb5_context,
                                                    mut context:
                                                        krb5_authdata_context) {
    let mut i: libc::c_int = 0;
    if context.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        if (*module).client_req_fini.is_some() &&
               !(*module).request_context.is_null() {
            Some((*module).client_req_fini.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                    context,
                                                                                                                    (*module).plugin_context,
                                                                                                                    (*module).request_context);
        }
        if (*module).client_fini.is_some() {
            Some((*module).client_fini.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                (*module).plugin_context);
        }
        memset(module as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_krb5_authdata_context_module>() as
                   libc::c_ulong);
        i += 1
    }
    if !(*context).modules.is_null() {
        free((*context).modules as *mut libc::c_void);
        (*context).modules = 0 as *mut _krb5_authdata_context_module
    }
    krb5int_close_plugin_dirs(&mut (*context).plugins);
    zapfree(context as *mut libc::c_void,
            ::std::mem::size_of::<_krb5_authdata_context>() as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "486:1"]
pub unsafe extern "C" fn krb5_authdata_import_attributes(mut kcontext:
                                                             krb5_context,
                                                         mut context:
                                                             krb5_authdata_context,
                                                         mut usage:
                                                             krb5_flags,
                                                         mut attrs:
                                                             *const krb5_data)
 -> krb5_error_code {
    let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut remain: size_t = 0;
    bp = (*attrs).data as *mut krb5_octet;
    remain = (*attrs).length as size_t;
    return k5_ad_internalize(kcontext, context, usage, &mut bp, &mut remain);
}
/* Return 0 with *kdc_issued_authdata == NULL on verification failure. */
#[c2rust::src_loc = "502:1"]
unsafe extern "C" fn k5_get_kdc_issued_authdata(mut kcontext: krb5_context,
                                                mut ap_req:
                                                    *const krb5_ap_req,
                                                mut kdc_issuer:
                                                    *mut krb5_principal,
                                                mut kdc_issued_authdata:
                                                    *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut authdata: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut ticket_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    *kdc_issuer = 0 as krb5_principal;
    *kdc_issued_authdata = 0 as *mut *mut krb5_authdata;
    ticket_authdata = (*(*(*ap_req).ticket).enc_part2).authorization_data;
    code =
        krb5_find_authdata(kcontext, ticket_authdata,
                           0 as *const *mut krb5_authdata, 4 as libc::c_int,
                           &mut authdata);
    if code != 0 as libc::c_int || authdata.is_null() { return code }
    /*
     * Note: a module must still implement a verify_authdata
     * method, even it is a NOOP that simply records the value
     * of the kdc_issued_flag.
     */
    code =
        krb5_verify_authdata_kdc_issued(kcontext,
                                        (*(*(*ap_req).ticket).enc_part2).session,
                                        *authdata.offset(0 as libc::c_int as
                                                             isize),
                                        kdc_issuer, kdc_issued_authdata);
    if code as libc::c_long == -(1765328353 as libc::c_long) ||
           code as libc::c_long == -(1765328334 as libc::c_long) ||
           code as libc::c_long == -(1765328196 as libc::c_long) ||
           code as libc::c_long == -(1765328194 as libc::c_long) {
        code = 0 as libc::c_int
    }
    krb5_free_authdata(kcontext, authdata);
    return code;
}
/* Decode and verify each CAMMAC and collect the resulting authdata,
 * ignoring those that failed verification. */
#[c2rust::src_loc = "545:1"]
unsafe extern "C" fn extract_cammacs(mut kcontext: krb5_context,
                                     mut cammacs: *mut *mut krb5_authdata,
                                     mut key: *const krb5_keyblock,
                                     mut ad_out: *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut list: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut elements: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut new_list: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut i: size_t = 0;
    let mut n_elements: size_t = 0;
    let mut count: size_t = 0 as libc::c_int as size_t;
    *ad_out = 0 as *mut *mut krb5_authdata;
    i = 0 as libc::c_int as size_t;
    loop  {
        if !(!cammacs.is_null() && !(*cammacs.offset(i as isize)).is_null()) {
            current_block = 13242334135786603907;
            break ;
        }
        ret =
            k5_unwrap_cammac_svc(kcontext, *cammacs.offset(i as isize), key,
                                 &mut elements);
        if ret != 0 && ret as libc::c_long != -(1765328353 as libc::c_long) {
            current_block = 4452497026146476505;
            break ;
        }
        ret = 0 as libc::c_int;
        /* Add the verified elements to list and free the container array. */
        n_elements = 0 as libc::c_int as size_t;
        while !(*elements.offset(n_elements as isize)).is_null() {
            n_elements = n_elements.wrapping_add(1)
        }
        new_list =
            realloc(list as *mut libc::c_void,
                    count.wrapping_add(n_elements).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                                                                    as
                                                                                                    libc::c_ulong))
                as *mut *mut krb5_authdata;
        if new_list.is_null() {
            ret = 12 as libc::c_int;
            current_block = 4452497026146476505;
            break ;
        } else {
            list = new_list;
            memcpy(list.offset(count as isize) as *mut libc::c_void,
                   elements as *const libc::c_void,
                   n_elements.wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                               as libc::c_ulong));
            count =
                (count as libc::c_ulong).wrapping_add(n_elements) as size_t as
                    size_t;
            let ref mut fresh11 = *list.offset(count as isize);
            *fresh11 = 0 as *mut krb5_authdata;
            free(elements as *mut libc::c_void);
            elements = 0 as *mut *mut krb5_authdata;
            i = i.wrapping_add(1)
        }
    }
    match current_block {
        13242334135786603907 => {
            *ad_out = list;
            list = 0 as *mut *mut krb5_authdata
        }
        _ => { }
    }
    krb5_free_authdata(kcontext, list);
    krb5_free_authdata(kcontext, elements);
    return ret;
}
/* Retrieve verified CAMMAC contained elements. */
#[c2rust::src_loc = "586:1"]
unsafe extern "C" fn get_cammac_authdata(mut kcontext: krb5_context,
                                         mut ap_req: *const krb5_ap_req,
                                         mut key: *const krb5_keyblock,
                                         mut elems_out:
                                             *mut *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut ticket_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut cammacs: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    let mut elements: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    *elems_out = 0 as *mut *mut krb5_authdata;
    ticket_authdata = (*(*(*ap_req).ticket).enc_part2).authorization_data;
    ret =
        krb5_find_authdata(kcontext, ticket_authdata,
                           0 as *const *mut krb5_authdata, 96 as libc::c_int,
                           &mut cammacs);
    if ret != 0 || cammacs.is_null() { return ret }
    ret = extract_cammacs(kcontext, cammacs, key, &mut elements);
    if ret == 0 { *elems_out = elements }
    krb5_free_authdata(kcontext, cammacs);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "609:1"]
pub unsafe extern "C" fn krb5int_authdata_verify(mut kcontext: krb5_context,
                                                 mut context:
                                                     krb5_authdata_context,
                                                 mut usage: krb5_flags,
                                                 mut auth_context:
                                                     *const krb5_auth_context,
                                                 mut key:
                                                     *const krb5_keyblock,
                                                 mut ap_req:
                                                     *const krb5_ap_req)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut authen_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut ticket_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut kdc_issuer: krb5_principal = 0 as krb5_principal;
    let mut kdc_issued_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    let mut cammac_authdata: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata;
    authen_authdata = (*(**auth_context).authentp).authorization_data;
    ticket_authdata = (*(*(*ap_req).ticket).enc_part2).authorization_data;
    code =
        k5_get_kdc_issued_authdata(kcontext, ap_req, &mut kdc_issuer,
                                   &mut kdc_issued_authdata);
    if !(code != 0) {
        code =
            get_cammac_authdata(kcontext, ap_req, key, &mut cammac_authdata);
        if !(code != 0) {
            i = 0 as libc::c_int;
            while i < (*context).n_modules {
                let mut module: *mut _krb5_authdata_context_module =
                    &mut *(*context).modules.offset(i as isize) as
                        *mut _krb5_authdata_context_module;
                let mut authdata: *mut *mut krb5_authdata =
                    0 as *mut *mut krb5_authdata;
                let mut kdc_issued_flag: krb5_boolean =
                    0 as libc::c_int as krb5_boolean;
                if !((*module).flags & usage == 0 as libc::c_int) {
                    if !(*(*module).ftable).import_authdata.is_none() {
                        if !kdc_issued_authdata.is_null() &&
                               (*module).flags & 0x8 as libc::c_int != 0 {
                            code =
                                krb5_find_authdata(kcontext,
                                                   kdc_issued_authdata,
                                                   0 as
                                                       *const *mut krb5_authdata,
                                                   (*module).ad_type,
                                                   &mut authdata);
                            if code != 0 as libc::c_int { break ; }
                            kdc_issued_flag = 1 as libc::c_int as krb5_boolean
                        }
                        if !cammac_authdata.is_null() &&
                               (*module).flags & 0x20 as libc::c_int != 0 {
                            code =
                                krb5_find_authdata(kcontext, cammac_authdata,
                                                   0 as
                                                       *const *mut krb5_authdata,
                                                   (*module).ad_type,
                                                   &mut authdata);
                            if code != 0 { break ; }
                            kdc_issued_flag = 1 as libc::c_int as krb5_boolean
                        }
                        if authdata.is_null() {
                            let mut ticket_usage: krb5_boolean =
                                0 as libc::c_int as krb5_boolean;
                            let mut authen_usage: krb5_boolean =
                                0 as libc::c_int as krb5_boolean;
                            /*
             * Determine which authdata sources to interrogate based on the
             * module's usage. This is important if the authdata is signed
             * by the KDC with the TGT key (as the user can forge that in
             * the AP-REQ).
             */
                            if (*module).flags &
                                   (0x1 as libc::c_int | 0x2 as libc::c_int)
                                   != 0 {
                                ticket_usage =
                                    1 as libc::c_int as krb5_boolean
                            }
                            if (*module).flags & 0x4 as libc::c_int != 0 {
                                authen_usage =
                                    1 as libc::c_int as krb5_boolean
                            }
                            code =
                                krb5_find_authdata(kcontext,
                                                   if ticket_usage != 0 {
                                                       ticket_authdata
                                                   } else {
                                                       0 as
                                                           *mut *mut krb5_authdata
                                                   },
                                                   if authen_usage != 0 {
                                                       authen_authdata
                                                   } else {
                                                       0 as
                                                           *mut *mut krb5_authdata
                                                   }, (*module).ad_type,
                                                   &mut authdata);
                            if code != 0 as libc::c_int { break ; }
                        }
                        if !authdata.is_null() {
                            if !(*authdata.offset(0 as libc::c_int as
                                                      isize)).is_null() {
                            } else {
                                __assert_fail(b"authdata[0] != NULL\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              b"authdata.c\x00" as *const u8
                                                  as *const libc::c_char,
                                              693 as libc::c_int as
                                                  libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 160],
                                                                        &[libc::c_char; 160]>(b"krb5_error_code krb5int_authdata_verify(krb5_context, krb5_authdata_context, krb5_flags, const krb5_auth_context *, const krb5_keyblock *, const krb5_ap_req *)\x00")).as_ptr());
                            }
                            code =
                                Some((*(*module).ftable).import_authdata.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                                  context,
                                                                                                                                                  (*module).plugin_context,
                                                                                                                                                  *(*module).request_context_pp,
                                                                                                                                                  authdata,
                                                                                                                                                  kdc_issued_flag,
                                                                                                                                                  kdc_issuer
                                                                                                                                                      as
                                                                                                                                                      krb5_const_principal);
                            if code == 0 as libc::c_int &&
                                   (*(*module).ftable).verify.is_some() {
                                code =
                                    Some((*(*module).ftable).verify.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                             context,
                                                                                                                                             (*module).plugin_context,
                                                                                                                                             *(*module).request_context_pp,
                                                                                                                                             auth_context,
                                                                                                                                             key,
                                                                                                                                             ap_req)
                            }
                            if code != 0 as libc::c_int &&
                                   (*module).flags & 0x10 as libc::c_int != 0
                               {
                                code = 0 as libc::c_int
                            }
                            krb5_free_authdata(kcontext, authdata);
                            if code != 0 as libc::c_int { break ; }
                        }
                    }
                }
                i += 1
            }
        }
    }
    krb5_free_principal(kcontext, kdc_issuer);
    krb5_free_authdata(kcontext, kdc_issued_authdata);
    krb5_free_authdata(kcontext, cammac_authdata);
    return code;
}
#[c2rust::src_loc = "726:1"]
unsafe extern "C" fn k5_merge_data_list(mut dst: *mut *mut krb5_data,
                                        mut src: *mut krb5_data,
                                        mut len: *mut libc::c_uint)
 -> krb5_error_code {
    let mut i: libc::c_uint = 0;
    let mut d: *mut krb5_data = 0 as *mut krb5_data;
    if src.is_null() { return 0 as libc::c_int }
    i = 0 as libc::c_int as libc::c_uint;
    while !(*src.offset(i as isize)).data.is_null() { i = i.wrapping_add(1) }
    d =
        realloc(*dst as *mut libc::c_void,
                ((*len).wrapping_add(i).wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_data>()
                                                     as libc::c_ulong)) as
            *mut krb5_data;
    if d.is_null() { return 12 as libc::c_int }
    memcpy(&mut *d.offset(*len as isize) as *mut krb5_data as
               *mut libc::c_void, src as *const libc::c_void,
           (i as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_data>()
                                                as libc::c_ulong));
    *len = (*len).wrapping_add(i);
    let ref mut fresh12 = (*d.offset(*len as isize)).data;
    *fresh12 = 0 as *mut libc::c_char;
    (*d.offset(*len as isize)).length = 0 as libc::c_int as libc::c_uint;
    *dst = d;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "754:1"]
pub unsafe extern "C" fn krb5_authdata_get_attribute_types(mut kcontext:
                                                               krb5_context,
                                                           mut context:
                                                               krb5_authdata_context,
                                                           mut out_attrs:
                                                               *mut *mut krb5_data)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut attrs: *mut krb5_data = 0 as *mut krb5_data;
    let mut attrs_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        let mut attrs2: *mut krb5_data = 0 as *mut krb5_data;
        if !(*(*module).ftable).get_attribute_types.is_none() {
            if !(Some((*(*module).ftable).get_attribute_types.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                       context,
                                                                                                                                       (*module).plugin_context,
                                                                                                                                       *(*module).request_context_pp,
                                                                                                                                       &mut attrs2)
                     != 0) {
                code = k5_merge_data_list(&mut attrs, attrs2, &mut attrs_len);
                if code != 0 as libc::c_int {
                    krb5int_free_data_list(kcontext, attrs2);
                    break ;
                } else if !attrs2.is_null() {
                    free(attrs2 as *mut libc::c_void);
                }
            }
        }
        i += 1
    }
    if code != 0 as libc::c_int {
        krb5int_free_data_list(kcontext, attrs);
        attrs = 0 as *mut krb5_data
    }
    *out_attrs = attrs;
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "797:1"]
pub unsafe extern "C" fn krb5_authdata_get_attribute(mut kcontext:
                                                         krb5_context,
                                                     mut context:
                                                         krb5_authdata_context,
                                                     mut attribute:
                                                         *const krb5_data,
                                                     mut authenticated:
                                                         *mut krb5_boolean,
                                                     mut complete:
                                                         *mut krb5_boolean,
                                                     mut value:
                                                         *mut krb5_data,
                                                     mut display_value:
                                                         *mut krb5_data,
                                                     mut more:
                                                         *mut libc::c_int)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 2 as libc::c_int;
    *authenticated = 0 as libc::c_int as krb5_boolean;
    *complete = 0 as libc::c_int as krb5_boolean;
    (*value).data = 0 as *mut libc::c_char;
    (*value).length = 0 as libc::c_int as libc::c_uint;
    (*display_value).data = 0 as *mut libc::c_char;
    (*display_value).length = 0 as libc::c_int as libc::c_uint;
    /*
     * NB at present a module is presumed to be authoritative for
     * an attribute; not sure how to federate "more" across module
     * yet
     */
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        if !(*(*module).ftable).get_attribute.is_none() {
            code =
                Some((*(*module).ftable).get_attribute.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                context,
                                                                                                                                (*module).plugin_context,
                                                                                                                                *(*module).request_context_pp,
                                                                                                                                attribute,
                                                                                                                                authenticated,
                                                                                                                                complete,
                                                                                                                                value,
                                                                                                                                display_value,
                                                                                                                                more);
            if code == 0 as libc::c_int { break ; }
        }
        i += 1
    }
    if code != 0 as libc::c_int { *more = 0 as libc::c_int }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "850:1"]
pub unsafe extern "C" fn krb5_authdata_set_attribute(mut kcontext:
                                                         krb5_context,
                                                     mut context:
                                                         krb5_authdata_context,
                                                     mut complete:
                                                         krb5_boolean,
                                                     mut attribute:
                                                         *const krb5_data,
                                                     mut value:
                                                         *const krb5_data)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        if !(*(*module).ftable).set_attribute.is_none() {
            code =
                Some((*(*module).ftable).set_attribute.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                context,
                                                                                                                                (*module).plugin_context,
                                                                                                                                *(*module).request_context_pp,
                                                                                                                                complete,
                                                                                                                                attribute,
                                                                                                                                value);
            if code == 2 as libc::c_int {
                code = 0 as libc::c_int
            } else { if !(code == 0 as libc::c_int) { break ; } found += 1 }
        }
        i += 1
    }
    if code == 0 as libc::c_int && found == 0 as libc::c_int {
        code = 2 as libc::c_int
    }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "888:1"]
pub unsafe extern "C" fn krb5_authdata_delete_attribute(mut kcontext:
                                                            krb5_context,
                                                        mut context:
                                                            krb5_authdata_context,
                                                        mut attribute:
                                                            *const krb5_data)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 2 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*context).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*context).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        if !(*(*module).ftable).delete_attribute.is_none() {
            code =
                Some((*(*module).ftable).delete_attribute.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                                   context,
                                                                                                                                   (*module).plugin_context,
                                                                                                                                   *(*module).request_context_pp,
                                                                                                                                   attribute);
            if code == 2 as libc::c_int {
                code = 0 as libc::c_int
            } else { if !(code == 0 as libc::c_int) { break ; } found += 1 }
        }
        i += 1
    }
    if code == 0 as libc::c_int && found == 0 as libc::c_int {
        code = 2 as libc::c_int
    }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "922:1"]
pub unsafe extern "C" fn krb5_authdata_export_attributes(mut kcontext:
                                                             krb5_context,
                                                         mut context:
                                                             krb5_authdata_context,
                                                         mut flags:
                                                             krb5_flags,
                                                         mut attrsp:
                                                             *mut *mut krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut required: size_t = 0 as libc::c_int as size_t;
    let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut remain: size_t = 0;
    let mut attrs: *mut krb5_data = 0 as *mut krb5_data;
    code = k5_ad_size(kcontext, context, 0x2f as libc::c_int, &mut required);
    if code != 0 as libc::c_int { return code }
    attrs =
        malloc(::std::mem::size_of::<krb5_data>() as libc::c_ulong) as
            *mut krb5_data;
    if attrs.is_null() { return 12 as libc::c_int }
    (*attrs).magic = -(1760647422 as libc::c_long) as krb5_magic;
    (*attrs).length = 0 as libc::c_int as libc::c_uint;
    (*attrs).data = malloc(required) as *mut libc::c_char;
    if (*attrs).data.is_null() {
        free(attrs as *mut libc::c_void);
        return 12 as libc::c_int
    }
    bp = (*attrs).data as *mut krb5_octet;
    remain = required;
    code =
        k5_ad_externalize(kcontext, context, 0x2f as libc::c_int, &mut bp,
                          &mut remain);
    if code != 0 as libc::c_int {
        krb5_free_data(kcontext, attrs);
        return code
    }
    (*attrs).length =
        bp.wrapping_offset_from((*attrs).data as *mut krb5_octet) as
            libc::c_long as libc::c_uint;
    *attrsp = attrs;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "966:1"]
pub unsafe extern "C" fn krb5_authdata_export_internal(mut kcontext:
                                                           krb5_context,
                                                       mut context:
                                                           krb5_authdata_context,
                                                       mut restrict_authenticated:
                                                           krb5_boolean,
                                                       mut module_name:
                                                           *const libc::c_char,
                                                       mut ptr:
                                                           *mut *mut libc::c_void)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut name: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut module: *mut _krb5_authdata_context_module =
        0 as *mut _krb5_authdata_context_module;
    *ptr = 0 as *mut libc::c_void;
    name =
        make_data(module_name as *mut libc::c_char as *mut libc::c_void,
                  strlen(module_name) as libc::c_uint);
    module =
        k5_ad_find_module(kcontext, context, 0x2f as libc::c_int, &mut name);
    if module.is_null() { return 2 as libc::c_int }
    if (*(*module).ftable).export_internal.is_none() {
        return 2 as libc::c_int
    }
    code =
        Some((*(*module).ftable).export_internal.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                          context,
                                                                                                                          (*module).plugin_context,
                                                                                                                          *(*module).request_context_pp,
                                                                                                                          restrict_authenticated,
                                                                                                                          ptr);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "997:1"]
pub unsafe extern "C" fn krb5_authdata_free_internal(mut kcontext:
                                                         krb5_context,
                                                     mut context:
                                                         krb5_authdata_context,
                                                     mut module_name:
                                                         *const libc::c_char,
                                                     mut ptr:
                                                         *mut libc::c_void)
 -> krb5_error_code {
    let mut name: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut module: *mut _krb5_authdata_context_module =
        0 as *mut _krb5_authdata_context_module;
    name =
        make_data(module_name as *mut libc::c_char as *mut libc::c_void,
                  strlen(module_name) as libc::c_uint);
    module =
        k5_ad_find_module(kcontext, context, 0x2f as libc::c_int, &mut name);
    if module.is_null() { return 2 as libc::c_int }
    if (*(*module).ftable).free_internal.is_none() { return 2 as libc::c_int }
    Some((*(*module).ftable).free_internal.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                    context,
                                                                                                                    (*module).plugin_context,
                                                                                                                    *(*module).request_context_pp,
                                                                                                                    ptr);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1023:1"]
unsafe extern "C" fn k5_copy_ad_module_data(mut kcontext: krb5_context,
                                            mut context:
                                                krb5_authdata_context,
                                            mut src_module:
                                                *mut _krb5_authdata_context_module,
                                            mut dst: krb5_authdata_context)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 0;
    let mut dst_module: *mut _krb5_authdata_context_module =
        0 as *mut _krb5_authdata_context_module;
    i = 0 as libc::c_int;
    while i < (*dst).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*dst).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        if (*module).ftable == (*src_module).ftable {
            /* XXX is this safe to assume these pointers are interned? */
            dst_module = module;
            break ;
        } else { i += 1 }
    }
    if dst_module.is_null() { return 2 as libc::c_int }
    /* copy request context for the first instance only */
    if (*dst_module).client_req_init.is_none() { return 0 as libc::c_int }
    if strcmp((*dst_module).name, (*src_module).name) == 0 as libc::c_int {
    } else {
        __assert_fail(b"strcmp(dst_module->name, src_module->name) == 0\x00"
                          as *const u8 as *const libc::c_char,
                      b"authdata.c\x00" as *const u8 as *const libc::c_char,
                      1050 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 139],
                                                &[libc::c_char; 139]>(b"krb5_error_code k5_copy_ad_module_data(krb5_context, krb5_authdata_context, struct _krb5_authdata_context_module *, krb5_authdata_context)\x00")).as_ptr());
    }
    /* If copy is unimplemented, externalize/internalize */
    if (*(*src_module).ftable).copy.is_none() {
        let mut size: size_t = 0 as libc::c_int as size_t;
        let mut remain: size_t = 0;
        let mut contents: *mut krb5_octet = 0 as *mut krb5_octet;
        let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
        if (*(*src_module).ftable).size.is_some() {
        } else {
            __assert_fail(b"src_module->ftable->size != NULL\x00" as *const u8
                              as *const libc::c_char,
                          b"authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          1057 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[libc::c_char; 139]>(b"krb5_error_code k5_copy_ad_module_data(krb5_context, krb5_authdata_context, struct _krb5_authdata_context_module *, krb5_authdata_context)\x00")).as_ptr());
        }
        if (*(*src_module).ftable).externalize.is_some() {
        } else {
            __assert_fail(b"src_module->ftable->externalize != NULL\x00" as
                              *const u8 as *const libc::c_char,
                          b"authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          1058 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[libc::c_char; 139]>(b"krb5_error_code k5_copy_ad_module_data(krb5_context, krb5_authdata_context, struct _krb5_authdata_context_module *, krb5_authdata_context)\x00")).as_ptr());
        }
        if (*(*dst_module).ftable).internalize.is_some() {
        } else {
            __assert_fail(b"dst_module->ftable->internalize != NULL\x00" as
                              *const u8 as *const libc::c_char,
                          b"authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          1059 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[libc::c_char; 139]>(b"krb5_error_code k5_copy_ad_module_data(krb5_context, krb5_authdata_context, struct _krb5_authdata_context_module *, krb5_authdata_context)\x00")).as_ptr());
        }
        code =
            Some((*(*src_module).ftable).size.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                       context,
                                                                                                                       (*src_module).plugin_context,
                                                                                                                       (*src_module).request_context,
                                                                                                                       &mut size);
        if code != 0 as libc::c_int { return code }
        contents = malloc(size) as *mut krb5_octet;
        if contents.is_null() { return 12 as libc::c_int }
        bp = contents;
        remain = size;
        code =
            Some((*(*src_module).ftable).externalize.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                              context,
                                                                                                                              (*src_module).plugin_context,
                                                                                                                              *(*src_module).request_context_pp,
                                                                                                                              &mut bp,
                                                                                                                              &mut remain);
        if code != 0 as libc::c_int {
            free(contents as *mut libc::c_void);
            return code
        }
        remain = bp.wrapping_offset_from(contents) as libc::c_long as size_t;
        bp = contents;
        code =
            Some((*(*dst_module).ftable).internalize.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                              context,
                                                                                                                              (*dst_module).plugin_context,
                                                                                                                              *(*dst_module).request_context_pp,
                                                                                                                              &mut bp,
                                                                                                                              &mut remain);
        if code != 0 as libc::c_int {
            free(contents as *mut libc::c_void);
            return code
        }
        free(contents as *mut libc::c_void);
    } else {
        if (*src_module).request_context_pp ==
               &mut (*src_module).request_context as *mut *mut libc::c_void {
        } else {
            __assert_fail(b"src_module->request_context_pp == &src_module->request_context\x00"
                              as *const u8 as *const libc::c_char,
                          b"authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          1103 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[libc::c_char; 139]>(b"krb5_error_code k5_copy_ad_module_data(krb5_context, krb5_authdata_context, struct _krb5_authdata_context_module *, krb5_authdata_context)\x00")).as_ptr());
        }
        if (*dst_module).request_context_pp ==
               &mut (*dst_module).request_context as *mut *mut libc::c_void {
        } else {
            __assert_fail(b"dst_module->request_context_pp == &dst_module->request_context\x00"
                              as *const u8 as *const libc::c_char,
                          b"authdata.c\x00" as *const u8 as
                              *const libc::c_char,
                          1104 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 139],
                                                    &[libc::c_char; 139]>(b"krb5_error_code k5_copy_ad_module_data(krb5_context, krb5_authdata_context, struct _krb5_authdata_context_module *, krb5_authdata_context)\x00")).as_ptr());
        }
        code =
            Some((*(*src_module).ftable).copy.expect("non-null function pointer")).expect("non-null function pointer")(kcontext,
                                                                                                                       context,
                                                                                                                       (*src_module).plugin_context,
                                                                                                                       (*src_module).request_context,
                                                                                                                       (*dst_module).plugin_context,
                                                                                                                       (*dst_module).request_context)
    }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1117:1"]
pub unsafe extern "C" fn krb5_authdata_context_copy(mut kcontext:
                                                        krb5_context,
                                                    mut src:
                                                        krb5_authdata_context,
                                                    mut pdst:
                                                        *mut krb5_authdata_context)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut code: krb5_error_code = 0;
    let mut dst: krb5_authdata_context = 0 as *mut _krb5_authdata_context;
    /* XXX we need to init a new context because we can't copy plugins */
    code = krb5_authdata_context_init(kcontext, &mut dst);
    if code != 0 as libc::c_int { return code }
    i = 0 as libc::c_int;
    while i < (*src).n_modules {
        let mut module: *mut _krb5_authdata_context_module =
            &mut *(*src).modules.offset(i as isize) as
                *mut _krb5_authdata_context_module;
        code = k5_copy_ad_module_data(kcontext, src, module, dst);
        if code != 0 as libc::c_int { break ; }
        i += 1
    }
    if code != 0 as libc::c_int {
        krb5_authdata_context_free(kcontext, dst);
        return code
    }
    *pdst = dst;
    return 0 as libc::c_int;
}
/*
 * Calculate size of to-be-externalized authdata context.
 */
#[no_mangle]
#[c2rust::src_loc = "1152:1"]
pub unsafe extern "C" fn k5_size_authdata_context(mut kcontext: krb5_context,
                                                  mut context:
                                                      krb5_authdata_context,
                                                  mut sizep: *mut size_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0; /* identifier/trailer */
    code = k5_ad_size(kcontext, context, 0x2f as libc::c_int, sizep);
    if code != 0 as libc::c_int { return code }
    *sizep =
        (*sizep as
             libc::c_ulong).wrapping_add((2 as libc::c_int as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<krb5_int32>()
                                                                              as
                                                                              libc::c_ulong))
            as size_t as size_t;
    return 0 as libc::c_int;
}
/*
 * Externalize an authdata context.
 */
#[no_mangle]
#[c2rust::src_loc = "1170:1"]
pub unsafe extern "C" fn k5_externalize_authdata_context(mut kcontext:
                                                             krb5_context,
                                                         mut context:
                                                             krb5_authdata_context,
                                                         mut buffer:
                                                             *mut *mut krb5_octet,
                                                         mut lenremain:
                                                             *mut size_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut remain: size_t = 0;
    bp = *buffer;
    remain = *lenremain;
    /* Our identifier */
    code =
        krb5_ser_pack_int32(-(1760647364 as libc::c_long) as krb5_int32,
                            &mut bp, &mut remain);
    if code != 0 as libc::c_int { return code }
    /* The actual context data */
    code =
        k5_ad_externalize(kcontext, context, 0x2f as libc::c_int, &mut bp,
                          &mut remain);
    if code != 0 as libc::c_int { return code }
    /* Our trailer */
    code =
        krb5_ser_pack_int32(-(1760647364 as libc::c_long) as krb5_int32,
                            &mut bp, &mut remain);
    if code != 0 as libc::c_int { return code }
    *buffer = bp;
    *lenremain = remain;
    return 0 as libc::c_int;
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
 * Internalize an authdata context.
 */
#[no_mangle]
#[c2rust::src_loc = "1207:1"]
pub unsafe extern "C" fn k5_internalize_authdata_context(mut kcontext:
                                                             krb5_context,
                                                         mut ptr:
                                                             *mut krb5_authdata_context,
                                                         mut buffer:
                                                             *mut *mut krb5_octet,
                                                         mut lenremain:
                                                             *mut size_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut context: krb5_authdata_context = 0 as *mut _krb5_authdata_context;
    let mut ibuf: krb5_int32 = 0;
    let mut bp: *mut krb5_octet = 0 as *mut krb5_octet;
    let mut remain: size_t = 0;
    bp = *buffer;
    remain = *lenremain;
    code = krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
    if code != 0 as libc::c_int { return code }
    if ibuf as libc::c_long != -(1760647364 as libc::c_long) {
        return 22 as libc::c_int
    }
    code = krb5_authdata_context_init(kcontext, &mut context);
    if code != 0 as libc::c_int { return code }
    code =
        k5_ad_internalize(kcontext, context, 0x2f as libc::c_int, &mut bp,
                          &mut remain);
    if code != 0 as libc::c_int {
        krb5_authdata_context_free(kcontext, context);
        return code
    }
    code = krb5_ser_unpack_int32(&mut ibuf, &mut bp, &mut remain);
    if code != 0 as libc::c_int { return code }
    if ibuf as libc::c_long != -(1760647364 as libc::c_long) {
        krb5_authdata_context_free(kcontext, context);
        return 22 as libc::c_int
    }
    *buffer = bp;
    *lenremain = remain;
    *ptr = context;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1255:1"]
pub unsafe extern "C" fn krb5int_copy_authdatum(mut context: krb5_context,
                                                mut inad:
                                                    *const krb5_authdata,
                                                mut outad:
                                                    *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut tmpad: *mut krb5_authdata = 0 as *mut krb5_authdata;
    tmpad =
        malloc(::std::mem::size_of::<krb5_authdata>() as libc::c_ulong) as
            *mut krb5_authdata;
    if tmpad.is_null() { return 12 as libc::c_int }
    *tmpad = *inad;
    (*tmpad).contents =
        malloc((*inad).length as libc::c_ulong) as *mut krb5_octet;
    if (*tmpad).contents.is_null() {
        free(tmpad as *mut libc::c_void);
        return 12 as libc::c_int
    }
    memcpy((*tmpad).contents as *mut libc::c_void,
           (*inad).contents as *const libc::c_void,
           (*inad).length as libc::c_ulong);
    *outad = tmpad;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1273:1"]
pub unsafe extern "C" fn krb5_free_authdata(mut context: krb5_context,
                                            mut val:
                                                *mut *mut krb5_authdata) {
    let mut temp: *mut *mut krb5_authdata = 0 as *mut *mut krb5_authdata;
    if val.is_null() { return }
    temp = val;
    while !(*temp).is_null() {
        free((**temp).contents as *mut libc::c_void);
        free(*temp as *mut libc::c_void);
        temp = temp.offset(1)
    }
    free(val as *mut libc::c_void);
}
