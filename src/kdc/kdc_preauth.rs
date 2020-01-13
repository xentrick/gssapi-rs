use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:79"]
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:79"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:79"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:79"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_0)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:79"]
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
    #[c2rust::src_loc = "2216:16"]
    pub struct _krb5_pa_pac_req {
        pub include_pac: krb5_boolean,
    }
    #[c2rust::src_loc = "2216:1"]
    pub type krb5_pa_pac_req = _krb5_pa_pac_req;
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* * TRUE if a PAC should be included in TGS-REP */
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
 * Verify a checksum (operates on keyblock).
 *
 * @param [in]  context         Library context
 * @param [in]  key             Encryption key for a keyed checksum
 * @param [in]  usage           @a key usage
 * @param [in]  data            Data to be used to compute a new checksum
 *                              using @a key to compare @a cksum against
 * @param [in]  cksum           Checksum to be verified
 * @param [out] valid           Non-zero for success, zero for failure
 *
 * This function verifies that @a cksum is a valid checksum for @a data.  If
 * the checksum type of @a cksum is a keyed checksum, @a key is used to verify
 * the checksum.  If the checksum type in @a cksum is 0 and @a key is not NULL,
 * the mandatory checksum type for @a key will be used.  The actual checksum
 * key will be derived from @a key and @a usage if key derivation is specified
 * for the checksum type.
 *
 * @note This function is similar to krb5_k_verify_checksum(), but operates
 * on keyblock @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "939:1"]
        pub fn krb5_c_verify_checksum(context: krb5_context,
                                      key: *const krb5_keyblock,
                                      usage: krb5_keyusage,
                                      data: *const krb5_data,
                                      cksum: *const krb5_checksum,
                                      valid: *mut krb5_boolean)
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
        /* *< UPNs as real principals */
        /* *< case-insensitive */
        /* *< treat principals as UTF-8 */
        /* *
 * Compare two principals with additional flags.
 *
 * @param [in] context           Library context
 * @param [in] princ1            First principal
 * @param [in] princ2            Second principal
 * @param [in] flags             Flags
 *
 * Valid flags are:
 * @li #KRB5_PRINCIPAL_COMPARE_IGNORE_REALM - ignore realm component
 * @li #KRB5_PRINCIPAL_COMPARE_ENTERPRISE - UPNs as real principals
 * @li #KRB5_PRINCIPAL_COMPARE_CASEFOLD case-insensitive
 * @li #KRB5_PRINCIPAL_COMPARE_UTF8 - treat principals as UTF-8
 *
 * @sa krb5_principal_compare()
 *
 * @retval
 * TRUE if the principal names are the same; FALSE otherwise
 */
        #[no_mangle]
        #[c2rust::src_loc = "3711:1"]
        pub fn krb5_principal_compare_flags(context: krb5_context,
                                            princ1: krb5_const_principal,
                                            princ2: krb5_const_principal,
                                            flags: libc::c_int)
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:79"]
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
    #[c2rust::src_loc = "436:1"]
    pub type krb5_etype_info = *mut *mut krb5_etype_info_entry;
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    #[inline]
    #[c2rust::src_loc = "2354:1"]
    pub unsafe extern "C" fn ts_incr(mut ts: krb5_timestamp,
                                     mut delta: krb5_deltat)
     -> krb5_timestamp {
        return (ts as uint32_t).wrapping_add(delta as uint32_t) as
                   krb5_timestamp;
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
    #[c2rust::src_loc = "2262:1"]
    pub unsafe extern "C" fn empty_data() -> krb5_data {
        return make_data(0 as *mut libc::c_void,
                         0 as libc::c_int as libc::c_uint);
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
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_octet,
                        krb5_data, krb5_timestamp, krb5_pa_data,
                        krb5_preauthtype, krb5_pa_pac_req};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::plugin_h::krb5_plugin_initvt_fn;
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
        #[c2rust::src_loc = "863:1"]
        pub fn krb5_free_etype_info(_: krb5_context, _: krb5_etype_info);
        #[no_mangle]
        #[c2rust::src_loc = "865:1"]
        pub fn krb5int_find_pa_data(_: krb5_context,
                                    _: *const *mut krb5_pa_data,
                                    _: krb5_preauthtype) -> *mut krb5_pa_data;
        #[no_mangle]
        #[c2rust::src_loc = "871:1"]
        pub fn k5_alloc_pa_data(pa_type: krb5_preauthtype, len: size_t,
                                out: *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "875:1"]
        pub fn k5_free_pa_data_element(pa: *mut krb5_pa_data);
        #[no_mangle]
        #[c2rust::src_loc = "880:1"]
        pub fn k5_add_pa_data_element(list: *mut *mut *mut krb5_pa_data,
                                      pa: *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "885:1"]
        pub fn k5_add_pa_data_from_data(list: *mut *mut *mut krb5_pa_data,
                                        pa_type: krb5_preauthtype,
                                        data: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "890:1"]
        pub fn k5_add_empty_pa_data(list: *mut *mut *mut krb5_pa_data,
                                    pa_type: krb5_preauthtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "942:1"]
        pub fn krb5_free_pa_pac_req(_: krb5_context, _: *mut krb5_pa_pac_req);
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
        #[no_mangle]
        #[c2rust::src_loc = "1186:1"]
        pub fn k5_plugin_register_dyn(context: krb5_context,
                                      interface_id: libc::c_int,
                                      modname: *const libc::c_char,
                                      modsubdir: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1451:1"]
        pub fn encode_krb5_etype_info(_: *const *mut krb5_etype_info_entry,
                                      code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1454:1"]
        pub fn encode_krb5_etype_info2(_: *const *mut krb5_etype_info_entry,
                                       code: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1678:1"]
        pub fn decode_krb5_pa_pac_req(_: *const krb5_data,
                                      _: *mut *mut krb5_pa_pac_req)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2159:1"]
        pub fn krb5_free_pa_data(_: krb5_context, _: *mut *mut krb5_pa_data);
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:79"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/kdcpreauth_plugin.h:80"]
pub mod kdcpreauth_plugin_h {
    #[c2rust::src_loc = "115:1"]
    pub type krb5_kdcpreauth_modreq = *mut krb5_kdcpreauth_modreq_st;
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
    /* The verto context structure type (typedef is in verto.h; we want to avoid a
 * header dependency for the moment). */
    /* Before using a callback after version 1, modules must check the vers
 * field of the callback structure. */
    /*
     * Get an array of krb5_keyblock structures containing the client keys
     * matching the request enctypes, terminated by an entry with key type = 0.
     * Returns ENOENT if no keys are available for the request enctypes.  Free
     * the resulting object with the free_keys callback.
     */
    /* Free the result of client_keys. */
    /*
     * Get the encoded request body, which is sometimes needed for checksums.
     * For a FAST request this is the encoded inner request body.  The returned
     * pointer is an alias and should not be freed.
     */
    /* Get a pointer to the FAST armor key, or NULL if the request did not use
     * FAST.  The returned pointer is an alias and should not be freed. */
    /* Retrieve a string attribute from the client DB entry, or NULL if no such
     * attribute is set.  Free the result with the free_string callback. */
    /* Free the result of get_string. */
    /* Get a pointer to the client DB entry (returned as a void pointer to
     * avoid a dependency on a libkdb5 type). */
    /* Get a pointer to the verto context which should be used by an
     * asynchronous edata or verify method. */
    /* End of version 1 kdcpreauth callbacks. */
    /* Return true if the client DB entry contains any keys matching the
     * request enctypes. */
    /* End of version 2 kdcpreauth callbacks. */
    /*
     * Get the decrypted client long-term key chosen according to the request
     * enctype list, or NULL if no matching key was found.  The returned
     * pointer is an alias and should not be freed.  If invoked from
     * return_padata, the result will be the same as the encrypting_key
     * parameter if it is not NULL, and will therefore reflect the modified
     * reply key if a return_padata handler has replaced the reply key.
     */
    /* Assert an authentication indicator in the AS-REP authdata.  Duplicate
     * indicators will be ignored. */
    /*
     * Read a data value for pa_type from the request cookie, placing it in
     * *out.  The value placed there is an alias and must not be freed.
     * Returns true if a value for pa_type was retrieved, false if not.
     */
    /*
     * Set a data value for pa_type to be sent in a secure cookie in the next
     * error response.  If pa_type is already present, the value is ignored.
     * If the preauth mechanism has different preauth types for requests and
     * responses, use the request type.  Secure cookies are encrypted in a key
     * known only to the KDCs, but can be replayed within a short time window
     * for requests using the same client principal.
     */
    /* End of version 3 kdcpreauth callbacks. */
    /*
     * Return true if princ matches the principal named in the request or the
     * client principal (possibly canonicalized).  If princ does not match,
     * attempt a database lookup of princ with aliases allowed and compare the
     * result to the client principal, returning true if it matches.
     * Otherwise, return false.
     */
    /*
     * Get an alias to the client DB entry principal (possibly canonicalized).
     */
    /* End of version 4 kdcpreauth callbacks. */
    /*
     * Instruct the KDC to send a freshness token in the method data
     * accompanying a PREAUTH_REQUIRED or PREAUTH_FAILED error, if the client
     * indicated support for freshness tokens.  This callback should only be
     * invoked from the edata method.
     */
    /* Validate a freshness token sent by the client.  Return 0 on success,
     * KRB5KDC_ERR_PREAUTH_EXPIRED on error. */
    /* End of version 5 kdcpreauth callbacks. */
    /* Optional: preauth plugin initialization function. */
    /* Optional: preauth plugin cleanup function. */
    /*
 * Optional: return the flags which the KDC should use for this module.  This
 * is a callback instead of a static value because the module may or may not
 * wish to count itself as a hardware preauthentication module (in other words,
 * the flags may be affected by the configuration, for example if a site
 * administrator can force a particular preauthentication type to be supported
 * using only hardware).  This function is called for each entry entry in the
 * server_pa_type_list.
 */
    /*
 * Responder for krb5_kdcpreauth_edata_fn.  If invoked with a non-zero code, pa
 * will be ignored and the padata type will not be included in the hint list.
 * If invoked with a zero code and a null pa value, the padata type will be
 * included in the list with an empty value.  If invoked with a zero code and a
 * non-null pa value, pa will be included in the hint list and will later be
 * freed by the KDC.
 */
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
    /*
 * Optional: verify preauthentication data sent by the client, setting the
 * TKT_FLG_PRE_AUTH or TKT_FLG_HW_AUTH flag in the enc_tkt_reply's "flags"
 * field as appropriate.  The implementation must invoke the respond function
 * when complete, whether successful or not, either before returning or
 * asynchronously using the verto context returned by cb->event_context().
 */
    /*
 * Optional: generate preauthentication response data to send to the client as
 * part of the AS-REP.  If it needs to override the key which is used to
 * encrypt the response, it can do so.
 */
    /* Optional: free a per-request context. */
    /* Optional: invoked after init_fn to provide the module with a pointer to the
 * verto main loop. */
    #[c2rust::src_loc = "376:1"]
    pub type krb5_kdcpreauth_loop_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata,
                                    _: *mut verto_ctx) -> krb5_error_code>;
    #[c2rust::src_loc = "114:1"]
    pub type krb5_kdcpreauth_moddata = *mut krb5_kdcpreauth_moddata_st;
    #[c2rust::src_loc = "369:1"]
    pub type krb5_kdcpreauth_free_modreq_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata,
                                    _: krb5_kdcpreauth_modreq) -> ()>;
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
    #[c2rust::src_loc = "111:1"]
    pub type krb5_kdcpreauth_rock = *mut krb5_kdcpreauth_rock_st;
    #[c2rust::src_loc = "123:1"]
    pub type krb5_kdcpreauth_callbacks = *mut krb5_kdcpreauth_callbacks_st;
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
    #[c2rust::src_loc = "326:1"]
    pub type krb5_kdcpreauth_verify_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: krb5_kdcpreauth_modreq,
                                    _: *mut *mut krb5_pa_data,
                                    _: *mut *mut krb5_authdata) -> ()>;
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
    #[c2rust::src_loc = "293:1"]
    pub type krb5_kdcpreauth_edata_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: *mut krb5_pa_data) -> ()>;
    #[c2rust::src_loc = "282:1"]
    pub type krb5_kdcpreauth_flags_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: krb5_preauthtype)
                   -> libc::c_int>;
    #[c2rust::src_loc = "269:1"]
    pub type krb5_kdcpreauth_fini_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: krb5_kdcpreauth_moddata) -> ()>;
    #[c2rust::src_loc = "263:1"]
    pub type krb5_kdcpreauth_init_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut krb5_kdcpreauth_moddata,
                                    _: *mut *const libc::c_char)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_preauthtype, krb5_error_code, krb5_context,
                        krb5_pa_data, krb5_data, krb5_kdc_req, krb5_kdc_rep,
                        krb5_keyblock, krb5_deltat, krb5_boolean,
                        krb5_principal, krb5_enc_tkt_part, krb5_authdata};
    use super::kdc_util_h::krb5_kdcpreauth_rock_st;
    extern "C" {
        #[c2rust::src_loc = "115:16"]
        pub type krb5_kdcpreauth_modreq_st;
        #[c2rust::src_loc = "119:8"]
        pub type verto_ctx;
        #[c2rust::src_loc = "114:16"]
        pub type krb5_kdcpreauth_moddata_st;
    }
    /* KRB5_KDCPREAUTH_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/kdc_util.h:80"]
pub mod kdc_util_h {
    /* Information handle for kdcpreauth callbacks.  All pointers are aliases. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "424:8"]
    pub struct krb5_kdcpreauth_rock_st {
        pub request: *mut krb5_kdc_req,
        pub inner_body: *mut krb5_data,
        pub client: *mut krb5_db_entry,
        pub local_tgt: *mut krb5_db_entry,
        pub client_key: *mut krb5_key_data,
        pub client_keyblock: *mut krb5_keyblock,
        pub rstate: *mut kdc_request_state,
        pub vctx: *mut verto_ctx,
        pub auth_indicators: *mut *mut *mut krb5_data,
        pub send_freshness_token: krb5_boolean,
    }
    #[c2rust::src_loc = "186:1"]
    pub type kdc_preauth_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code)
                   -> ()>;
    #[c2rust::src_loc = "175:1"]
    pub type kdc_hint_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    use super::krb5_h::{krb5_kdc_req, krb5_data, krb5_keyblock, krb5_boolean,
                        krb5_error_code, krb5_context, krb5_enctype,
                        krb5_preauthtype, krb5_pa_data};
    use super::kdb_h::{krb5_db_entry, krb5_key_data};
    use super::reqstate_h::kdc_request_state;
    use super::kdcpreauth_plugin_h::verto_ctx;
    use super::k5_int_h::_krb5_context;
    use super::plugin_h::{krb5_plugin_vtable_st, krb5_plugin_vtable};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "120:1"]
        pub fn authind_add(context: krb5_context, ind: *const libc::c_char,
                           indicators: *mut *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "168:1"]
        pub fn enctype_requires_etype_info_2(enctype: krb5_enctype)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "209:1"]
        pub fn kdcpreauth_encrypted_timestamp_initvt(context: krb5_context,
                                                     maj_ver: libc::c_int,
                                                     min_ver: libc::c_int,
                                                     vtable:
                                                         krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "204:1"]
        pub fn kdcpreauth_encrypted_challenge_initvt(context: krb5_context,
                                                     maj_ver: libc::c_int,
                                                     min_ver: libc::c_int,
                                                     vtable:
                                                         krb5_plugin_vtable)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "401:1"]
        pub fn kdc_fast_set_cookie(state: *mut kdc_request_state,
                                   pa_type: krb5_preauthtype,
                                   data: *const krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "387:1"]
        pub fn kdc_handle_protected_negotiation(context: krb5_context,
                                                req_pkt: *mut krb5_data,
                                                request: *mut krb5_kdc_req,
                                                reply_key:
                                                    *const krb5_keyblock,
                                                out_enc_padata:
                                                    *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "398:1"]
        pub fn kdc_fast_search_cookie(state: *mut kdc_request_state,
                                      pa_type: krb5_preauthtype,
                                      out: *mut krb5_data) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "411:1"]
        pub fn kdc_add_pa_pac_options(context: krb5_context,
                                      request: *mut krb5_kdc_req,
                                      out_enc_padata:
                                          *mut *mut *mut krb5_pa_data)
         -> krb5_error_code;
    }
    /* __KRB5_KDC_UTIL__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/reqstate.h:80"]
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
    use super::krb5_h::{krb5_keyblock, krb5_pa_data, krb5_int32};
    use super::realm_data_h::kdc_realm_t;
    /* REQSTATE_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kdc/realm_data.h:80"]
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
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/realm_data.h */
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
    /*
     * General Kerberos per-realm data.
     */
    /* Realm name                       */
    /* XXX the real context should go away once the db_context is done.
 * The db_context is then associated with the realm keytab using
 * krb5_ktkdb_resolv(). There should be nothing in the context which
 * cannot span multiple realms -- proven */
    /* Context to be used for realm     */
    /* keytab to be used for this realm */
    /* referral services for NT-UNKNOWN */
    /* non-referral services         */
    /*
     * Database per-realm data.
     */
    /* Stash file name for realm        */
    /* Master principal name for realm  */
    /* Master principal for realm       */
    /*
     * Note realm_mkey is mkey read from stash or keyboard and may not be the
     * latest.
     */
    /* Master key for this realm        */
    /*
     * TGS per-realm data.
     */
    /* TGS principal for this realm     */
    /*
     * Other per-realm data.
     */
    /* Per-realm KDC UDP listen */
    /* Per-realm KDC TCP listen */
    /*
     * Per-realm parameters.
     */
    /* Maximum ticket life for realm    */
    /* Maximum renewable life for realm */
    /* Accept unverifiable transited_realm ? */
    /* Anon to local TGT only */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:8"]
    pub struct server_handle {
        pub kdc_realmlist: *mut *mut kdc_realm_t,
        pub kdc_numrealms: libc::c_int,
        pub kdc_err_context: krb5_context,
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:80"]
pub mod kdb_h {
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
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* NOT saved */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_magic,
                        krb5_ui_4, krb5_flags, krb5_deltat, krb5_timestamp,
                        krb5_kvno, krb5_principal, krb5_data, krb5_context,
                        krb5_principal_data, krb5_const_principal,
                        krb5_error_code, krb5_int32, krb5_keyblock};
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
        /* Length, data */
        /* *
 * Decrypts the key given in @@a key_data. If @a mkey is specified, that
 * master key is used. If @a mkey is NULL, then all master keys are tried.
 */
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "532:1"]
        pub fn krb5_dbe_lookup_tl_data(context: krb5_context,
                                       entry: *mut krb5_db_entry,
                                       ret_tl_data: *mut krb5_tl_data)
         -> krb5_error_code;
        /* Retrieve a single string attribute from entry, or NULL if there is no
 * attribute for key.  Free *value_out with krb5_dbe_free_string when done. */
        #[no_mangle]
        #[c2rust::src_loc = "582:1"]
        pub fn krb5_dbe_get_string(context: krb5_context,
                                   entry: *mut krb5_db_entry,
                                   key: *const libc::c_char,
                                   value_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* Compute the salt for a key data entry given the corresponding principal. */
        #[no_mangle]
        #[c2rust::src_loc = "608:1"]
        pub fn krb5_dbe_compute_salt(context: krb5_context,
                                     key: *const krb5_key_data,
                                     princ: krb5_const_principal,
                                     salttype_out: *mut krb5_int16,
                                     salt_out: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "866:1"]
        pub fn krb5_dbe_free_string(_: krb5_context, _: *mut libc::c_char);
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/libintl.h:79"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:79"]
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
#[c2rust::header_src = "/usr/include/string.h:79"]
pub mod string_h {
    extern "C" {
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
#[c2rust::header_src = "/usr/include/assert.h:79"]
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:79"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:83"]
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
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
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
                       _krb5_pa_pac_req, krb5_pa_pac_req, krb5_kt_cursor,
                       krb5_keytab_entry_st, krb5_keytab_entry, krb5_keytab,
                       _profile_t, krb5_c_make_checksum,
                       krb5_c_verify_checksum, krb5_principal_compare,
                       krb5_principal_compare_flags,
                       krb5_copy_keyblock_contents, krb5_free_authdata,
                       krb5_free_checksum_contents,
                       krb5_free_keyblock_contents, krb5_free_data,
                       krb5_free_data_contents, krb5_timeofday,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         krb5_etype_info_entry, _krb5_etype_info_entry,
                         krb5_etype_info, ts_after, ts_incr, k5calloc,
                         empty_data, make_data, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_free_etype_info,
                         krb5int_find_pa_data, k5_alloc_pa_data,
                         k5_free_pa_data_element, k5_add_pa_data_element,
                         k5_add_pa_data_from_data, k5_add_empty_pa_data,
                         krb5_free_pa_pac_req, k5_plugin_load_all,
                         k5_plugin_free_modules, k5_plugin_register,
                         k5_plugin_register_dyn, encode_krb5_etype_info,
                         encode_krb5_etype_info2, decode_krb5_pa_pac_req,
                         krb5_free_pa_data};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_initvt_fn,
                         krb5_plugin_vtable_st};
pub use self::kdcpreauth_plugin_h::{krb5_kdcpreauth_modreq,
                                    krb5_kdcpreauth_vtable_st,
                                    krb5_kdcpreauth_loop_fn,
                                    krb5_kdcpreauth_moddata,
                                    krb5_kdcpreauth_free_modreq_fn,
                                    krb5_kdcpreauth_return_fn,
                                    krb5_kdcpreauth_rock,
                                    krb5_kdcpreauth_callbacks,
                                    krb5_kdcpreauth_callbacks_st,
                                    krb5_kdcpreauth_verify_fn,
                                    krb5_kdcpreauth_verify_respond_fn,
                                    krb5_kdcpreauth_edata_fn,
                                    krb5_kdcpreauth_edata_respond_fn,
                                    krb5_kdcpreauth_flags_fn,
                                    krb5_kdcpreauth_fini_fn,
                                    krb5_kdcpreauth_init_fn,
                                    krb5_kdcpreauth_modreq_st, verto_ctx,
                                    krb5_kdcpreauth_moddata_st};
pub use self::kdc_util_h::{krb5_kdcpreauth_rock_st, kdc_preauth_respond_fn,
                           kdc_hint_respond_fn, authind_add,
                           enctype_requires_etype_info_2,
                           kdcpreauth_encrypted_timestamp_initvt,
                           kdcpreauth_encrypted_challenge_initvt,
                           kdc_fast_set_cookie,
                           kdc_handle_protected_negotiation,
                           kdc_fast_search_cookie, kdc_add_pa_pac_options};
pub use self::reqstate_h::kdc_request_state;
pub use self::realm_data_h::{kdc_realm_t, __kdc_realm_data, server_handle};
pub use self::kdb_h::{krb5_key_data, _krb5_key_data, krb5_db_entry,
                      _krb5_db_entry_new, krb5_tl_data, _krb5_tl_data,
                      krb5_keysalt, _krb5_keysalt, krb5_db_get_principal,
                      krb5_db_free_principal, krb5_dbe_find_enctype,
                      krb5_dbe_decrypt_key_data, krb5_dbe_lookup_tl_data,
                      krb5_dbe_get_string, krb5_dbe_compute_salt,
                      krb5_dbe_free_string};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, calloc, malloc};
use self::string_h::{memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::__bswap_32;
use self::adm_proto_h::krb5_klog_syslog;
#[c2rust::src_loc = "93:1"]
pub type preauth_system = preauth_system_st;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "93:16"]
pub struct preauth_system_st {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub moddata: krb5_kdcpreauth_moddata,
    pub init: krb5_kdcpreauth_init_fn,
    pub fini: krb5_kdcpreauth_fini_fn,
    pub get_edata: krb5_kdcpreauth_edata_fn,
    pub verify_padata: krb5_kdcpreauth_verify_fn,
    pub return_padata: krb5_kdcpreauth_return_fn,
    pub free_modreq: krb5_kdcpreauth_free_modreq_fn,
    pub loop_0: krb5_kdcpreauth_loop_fn,
}
/*
 * The make_padata_context() function creates a space for storing any
 * request-specific module data which will be needed by return_padata() later.
 * Each preauth type gets a storage location of its own.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "282:8"]
pub struct request_pa_context {
    pub n_contexts: libc::c_int,
    pub contexts: *mut C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "284:5"]
pub struct C2RustUnnamed_1 {
    pub pa_system: *mut preauth_system,
    pub modreq: krb5_kdcpreauth_modreq,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1056:8"]
pub struct padata_state {
    pub respond: kdc_preauth_respond_fn,
    pub arg: *mut libc::c_void,
    pub realm: *mut kdc_realm_t,
    pub modreq_ptr: *mut krb5_kdcpreauth_modreq,
    pub padata: *mut *mut krb5_pa_data,
    pub pa_found: libc::c_int,
    pub context: krb5_context,
    pub rock: krb5_kdcpreauth_rock,
    pub req_pkt: *mut krb5_data,
    pub request: *mut krb5_kdc_req,
    pub enc_tkt_reply: *mut krb5_enc_tkt_part,
    pub padata_context: *mut *mut libc::c_void,
    pub pa_sys: *mut preauth_system,
    pub pa_e_data: *mut *mut krb5_pa_data,
    pub typed_e_data_flag: krb5_boolean,
    pub pa_ok: libc::c_int,
    pub saved_code: krb5_error_code,
    pub e_data_out: *mut *mut *mut krb5_pa_data,
    pub typed_e_data_out: *mut krb5_boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "864:8"]
pub struct hint_state {
    pub respond: kdc_hint_respond_fn,
    pub arg: *mut libc::c_void,
    pub realm: *mut kdc_realm_t,
    pub rock: krb5_kdcpreauth_rock,
    pub request: *mut krb5_kdc_req,
    pub e_data_out: *mut *mut *mut krb5_pa_data,
    pub hw_only: libc::c_int,
    pub ap: *mut preauth_system,
    pub pa_data: *mut *mut krb5_pa_data,
    pub pa_type: krb5_preauthtype,
}
#[c2rust::src_loc = "107:24"]
static mut preauth_systems: *mut preauth_system =
    0 as *const preauth_system as *mut preauth_system;
#[c2rust::src_loc = "108:15"]
static mut n_preauth_systems: size_t = 0;
/* Get all available kdcpreauth vtables and a count of preauth types they
 * support.  Return an empty list on failure. */
#[c2rust::src_loc = "117:1"]
unsafe extern "C" fn get_plugin_vtables(mut context: krb5_context,
                                        mut vtables_out:
                                            *mut *mut krb5_kdcpreauth_vtable_st,
                                        mut n_tables_out: *mut size_t,
                                        mut n_systems_out: *mut size_t) {
    let mut plugins: *mut krb5_plugin_initvt_fn =
        0 as *mut krb5_plugin_initvt_fn;
    let mut pl: *mut krb5_plugin_initvt_fn = 0 as *mut krb5_plugin_initvt_fn;
    let mut vtables: *mut krb5_kdcpreauth_vtable_st =
        0 as *mut krb5_kdcpreauth_vtable_st;
    let mut count: size_t = 0;
    let mut n_tables: size_t = 0;
    let mut n_systems: size_t = 0;
    let mut i: size_t = 0;
    *vtables_out = 0 as *mut krb5_kdcpreauth_vtable_st;
    *n_systems_out = 0 as libc::c_int as size_t;
    *n_tables_out = *n_systems_out;
    /* Auto-register encrypted challenge and (if possible) pkinit. */
    k5_plugin_register_dyn(context, 3 as libc::c_int,
                           b"pkinit\x00" as *const u8 as *const libc::c_char,
                           b"preauth\x00" as *const u8 as
                               *const libc::c_char);
    k5_plugin_register_dyn(context, 3 as libc::c_int,
                           b"otp\x00" as *const u8 as *const libc::c_char,
                           b"preauth\x00" as *const u8 as
                               *const libc::c_char);
    k5_plugin_register_dyn(context, 3 as libc::c_int,
                           b"spake\x00" as *const u8 as *const libc::c_char,
                           b"preauth\x00" as *const u8 as
                               *const libc::c_char);
    k5_plugin_register(context, 3 as libc::c_int,
                       b"encrypted_challenge\x00" as *const u8 as
                           *const libc::c_char,
                       Some(kdcpreauth_encrypted_challenge_initvt as
                                unsafe extern "C" fn(_: krb5_context,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: krb5_plugin_vtable)
                                    -> krb5_error_code));
    k5_plugin_register(context, 3 as libc::c_int,
                       b"encrypted_timestamp\x00" as *const u8 as
                           *const libc::c_char,
                       Some(kdcpreauth_encrypted_timestamp_initvt as
                                unsafe extern "C" fn(_: krb5_context,
                                                     _: libc::c_int,
                                                     _: libc::c_int,
                                                     _: krb5_plugin_vtable)
                                    -> krb5_error_code));
    if k5_plugin_load_all(context, 3 as libc::c_int, &mut plugins) != 0 {
        return
    }
    count = 0 as libc::c_int as size_t;
    while (*plugins.offset(count as isize)).is_some() {
        count = count.wrapping_add(1)
    }
    vtables =
        calloc(count.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<krb5_kdcpreauth_vtable_st>() as
                   libc::c_ulong) as *mut krb5_kdcpreauth_vtable_st;
    if !vtables.is_null() {
        pl = plugins;
        n_tables = 0 as libc::c_int as size_t;
        while (*pl).is_some() {
            if (*pl).expect("non-null function pointer")(context,
                                                         1 as libc::c_int,
                                                         2 as libc::c_int,
                                                         &mut *vtables.offset(n_tables
                                                                                  as
                                                                                  isize)
                                                             as
                                                             *mut krb5_kdcpreauth_vtable_st
                                                             as
                                                             krb5_plugin_vtable)
                   == 0 as libc::c_int {
                n_tables = n_tables.wrapping_add(1)
            }
            pl = pl.offset(1)
        }
        i = 0 as libc::c_int as size_t;
        n_systems = 0 as libc::c_int as size_t;
        while i < n_tables {
            count = 0 as libc::c_int as size_t;
            while *(*vtables.offset(i as
                                        isize)).pa_type_list.offset(count as
                                                                        isize)
                      != 0 as libc::c_int {
                count = count.wrapping_add(1)
            }
            n_systems =
                (n_systems as libc::c_ulong).wrapping_add(count) as size_t as
                    size_t;
            i = i.wrapping_add(1)
        }
        *vtables_out = vtables;
        *n_tables_out = n_tables;
        *n_systems_out = n_systems
    }
    k5_plugin_free_modules(context, plugins);
}
/* Make a list of realm names.  The caller should free the list container but
 * not the list elements (which are aliases into kdc_realmlist). */
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn get_realm_names(mut handle: *mut server_handle,
                                     mut list_out:
                                         *mut *mut *const libc::c_char)
 -> krb5_error_code {
    let mut list: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: libc::c_int = 0;
    list =
        calloc(((*handle).kdc_numrealms + 1 as libc::c_int) as libc::c_ulong,
               ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as *mut *const libc::c_char;
    if list.is_null() { return 12 as libc::c_int }
    i = 0 as libc::c_int;
    while i < (*handle).kdc_numrealms {
        let ref mut fresh0 = *list.offset(i as isize);
        *fresh0 = (**(*handle).kdc_realmlist.offset(i as isize)).realm_name;
        i += 1
    }
    let ref mut fresh1 = *list.offset(i as isize);
    *fresh1 = 0 as *const libc::c_char;
    *list_out = list;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "183:1"]
pub unsafe extern "C" fn load_preauth_plugins(mut handle: *mut server_handle,
                                              mut context: krb5_context,
                                              mut ctx: *mut verto_ctx) {
    let mut ret: krb5_error_code = 0;
    let mut vtables: *mut krb5_kdcpreauth_vtable_st =
        0 as *mut krb5_kdcpreauth_vtable_st;
    let mut vt: *mut krb5_kdcpreauth_vtable_st =
        0 as *mut krb5_kdcpreauth_vtable_st;
    let mut n_systems: size_t = 0;
    let mut n_tables: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut moddata: krb5_kdcpreauth_moddata =
        0 as *mut krb5_kdcpreauth_moddata_st;
    let mut realm_names: *mut *const libc::c_char =
        0 as *mut *const libc::c_char;
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut sys: *mut preauth_system = 0 as *mut preauth_system;
    /* Get all available kdcpreauth vtables. */
    get_plugin_vtables(context, &mut vtables, &mut n_tables, &mut n_systems);
    /* Allocate the list of static and plugin preauth systems. */
    preauth_systems =
        calloc(n_systems.wrapping_add(1 as libc::c_int as libc::c_ulong),
               ::std::mem::size_of::<preauth_system>() as libc::c_ulong) as
            *mut preauth_system;
    if !preauth_systems.is_null() {
        if !(get_realm_names(handle, &mut realm_names) != 0) {
            /* Add the dynamically-loaded mechanisms to the list. */
            n_systems = 0 as libc::c_int as size_t;
            let mut current_block_31: u64;
            i = 0 as libc::c_int as size_t;
            while i < n_tables {
                /* Try to initialize this module. */
                vt =
                    &mut *vtables.offset(i as isize) as
                        *mut krb5_kdcpreauth_vtable_st;
                moddata = 0 as krb5_kdcpreauth_moddata;
                if (*vt).init.is_some() {
                    ret =
                        (*vt).init.expect("non-null function pointer")(context,
                                                                       &mut moddata,
                                                                       realm_names);
                    if ret != 0 {
                        emsg = krb5_get_error_message(context, ret);
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"preauth %s failed to initialize: %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         (*vt).name, emsg);
                        krb5_free_error_message(context, emsg);
                        current_block_31 = 3276175668257526147;
                    } else { current_block_31 = 5948590327928692120; }
                } else { current_block_31 = 5948590327928692120; }
                match current_block_31 {
                    5948590327928692120 => {
                        if (*vt).loop_0.is_some() {
                            ret =
                                (*vt).loop_0.expect("non-null function pointer")(context,
                                                                                 moddata,
                                                                                 ctx);
                            if ret != 0 {
                                emsg = krb5_get_error_message(context, ret);
                                krb5_klog_syslog(3 as libc::c_int,
                                                 dgettext(b"mit-krb5\x00" as
                                                              *const u8 as
                                                              *const libc::c_char,
                                                          b"preauth %s failed to setup loop: %s\x00"
                                                              as *const u8 as
                                                              *const libc::c_char),
                                                 (*vt).name, emsg);
                                krb5_free_error_message(context, emsg);
                                if (*vt).fini.is_some() {
                                    (*vt).fini.expect("non-null function pointer")(context,
                                                                                   moddata);
                                }
                                current_block_31 = 3276175668257526147;
                            } else {
                                current_block_31 = 11057878835866523405;
                            }
                        } else { current_block_31 = 11057878835866523405; }
                        match current_block_31 {
                            3276175668257526147 => { }
                            _ => {
                                /* Add this module to the systems list once for each pa type. */
                                j = 0 as libc::c_int as size_t;
                                while *(*vt).pa_type_list.offset(j as isize)
                                          != 0 as libc::c_int {
                                    sys =
                                        &mut *preauth_systems.offset(n_systems
                                                                         as
                                                                         isize)
                                            as *mut preauth_system;
                                    (*sys).name = (*vt).name;
                                    (*sys).type_0 =
                                        *(*vt).pa_type_list.offset(j as
                                                                       isize);
                                    (*sys).flags =
                                        if (*vt).flags.is_some() {
                                            (*vt).flags.expect("non-null function pointer")(context,
                                                                                            (*sys).type_0)
                                        } else { 0 as libc::c_int };
                                    (*sys).moddata = moddata;
                                    (*sys).init = (*vt).init;
                                    /* Only call fini once for each plugin. */
                                    (*sys).fini =
                                        if j ==
                                               0 as libc::c_int as
                                                   libc::c_ulong {
                                            (*vt).fini
                                        } else { None };
                                    (*sys).get_edata = (*vt).edata;
                                    (*sys).verify_padata = (*vt).verify;
                                    (*sys).return_padata =
                                        (*vt).return_padata;
                                    (*sys).free_modreq = (*vt).free_modreq;
                                    (*sys).loop_0 = (*vt).loop_0;
                                    n_systems = n_systems.wrapping_add(1);
                                    j = j.wrapping_add(1)
                                }
                            }
                        }
                    }
                    _ => { }
                }
                i = i.wrapping_add(1)
            }
            n_preauth_systems = n_systems;
            /* Add the end-of-list marker. */
            let ref mut fresh2 =
                (*preauth_systems.offset(n_systems as isize)).name;
            *fresh2 = b"[end]\x00" as *const u8 as *const libc::c_char;
            (*preauth_systems.offset(n_systems as isize)).type_0 =
                -(1 as libc::c_int)
        }
    }
    free(vtables as *mut libc::c_void);
    free(realm_names as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "263:1"]
pub unsafe extern "C" fn unload_preauth_plugins(mut context: krb5_context) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n_preauth_systems {
        if (*preauth_systems.offset(i as isize)).fini.is_some() {
            (*preauth_systems.offset(i as
                                         isize)).fini.expect("non-null function pointer")(context,
                                                                                          (*preauth_systems.offset(i
                                                                                                                       as
                                                                                                                       isize)).moddata);
        }
        i = i.wrapping_add(1)
    }
    free(preauth_systems as *mut libc::c_void);
    preauth_systems = 0 as *mut preauth_system;
    n_preauth_systems = 0 as libc::c_int as size_t;
}
#[c2rust::src_loc = "290:1"]
unsafe extern "C" fn make_padata_context(mut context: krb5_context,
                                         mut padata_context:
                                             *mut *mut libc::c_void)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut ret: *mut request_pa_context = 0 as *mut request_pa_context;
    ret =
        malloc(::std::mem::size_of::<request_pa_context>() as libc::c_ulong)
            as *mut request_pa_context;
    if ret.is_null() { return 12 as libc::c_int }
    (*ret).n_contexts = n_preauth_systems as libc::c_int;
    (*ret).contexts =
        malloc((::std::mem::size_of::<C2RustUnnamed_1>() as
                    libc::c_ulong).wrapping_mul((*ret).n_contexts as
                                                    libc::c_ulong)) as
            *mut C2RustUnnamed_1;
    if (*ret).contexts.is_null() {
        free(ret as *mut libc::c_void);
        return 12 as libc::c_int
    }
    memset((*ret).contexts as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<C2RustUnnamed_1>() as
                libc::c_ulong).wrapping_mul((*ret).n_contexts as
                                                libc::c_ulong));
    i = 0 as libc::c_int;
    while i < (*ret).n_contexts {
        let ref mut fresh3 = (*(*ret).contexts.offset(i as isize)).pa_system;
        *fresh3 =
            &mut *preauth_systems.offset(i as isize) as *mut preauth_system;
        let ref mut fresh4 = (*(*ret).contexts.offset(i as isize)).modreq;
        *fresh4 = 0 as krb5_kdcpreauth_modreq;
        i += 1
    }
    *padata_context = ret as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * The free_padata_context function frees any context information pointers
 * which the check_padata() function created but which weren't already cleaned
 * up by return_padata().
 */
#[no_mangle]
#[c2rust::src_loc = "325:1"]
pub unsafe extern "C" fn free_padata_context(mut kcontext: krb5_context,
                                             mut padata_context:
                                                 *mut libc::c_void) {
    let mut context: *mut request_pa_context =
        padata_context as *mut request_pa_context;
    let mut sys: *mut preauth_system = 0 as *mut preauth_system;
    let mut i: libc::c_int = 0;
    if context.is_null() { return }
    i = 0 as libc::c_int;
    while i < (*context).n_contexts {
        sys = (*(*context).contexts.offset(i as isize)).pa_system;
        if !((*sys).free_modreq.is_none() ||
                 (*(*context).contexts.offset(i as isize)).modreq.is_null()) {
            (*sys).free_modreq.expect("non-null function pointer")(kcontext,
                                                                   (*sys).moddata,
                                                                   (*(*context).contexts.offset(i
                                                                                                    as
                                                                                                    isize)).modreq);
            let ref mut fresh5 =
                (*(*context).contexts.offset(i as isize)).modreq;
            *fresh5 = 0 as krb5_kdcpreauth_modreq
        }
        i += 1
    }
    free((*context).contexts as *mut libc::c_void);
    free(context as *mut libc::c_void);
}
#[c2rust::src_loc = "346:1"]
unsafe extern "C" fn max_time_skew(mut context: krb5_context,
                                   mut rock: krb5_kdcpreauth_rock)
 -> krb5_deltat {
    return (*context).clockskew;
}
#[c2rust::src_loc = "352:1"]
unsafe extern "C" fn client_keys(mut context: krb5_context,
                                 mut rock: krb5_kdcpreauth_rock,
                                 mut keys_out: *mut *mut krb5_keyblock)
 -> krb5_error_code {
    let mut request: *mut krb5_kdc_req = (*rock).request;
    let mut client: *mut krb5_db_entry = (*rock).client;
    let mut keys: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut entry_key: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    keys =
        calloc(((*request).nktypes + 1 as libc::c_int) as libc::c_ulong,
               ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong) as
            *mut krb5_keyblock;
    if keys.is_null() { return 12 as libc::c_int }
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*request).nktypes {
        entry_key = 0 as *mut krb5_key_data;
        if !(krb5_dbe_find_enctype(context, client,
                                   *(*request).ktype.offset(i as isize),
                                   -(1 as libc::c_int), 0 as libc::c_int,
                                   &mut entry_key) != 0 as libc::c_int) {
            if !(krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock,
                                           entry_key, &mut key,
                                           0 as *mut krb5_keysalt) !=
                     0 as libc::c_int) {
                let fresh6 = k;
                k = k + 1;
                *keys.offset(fresh6 as isize) = key
            }
        }
        i += 1
    }
    if k == 0 as libc::c_int {
        free(keys as *mut libc::c_void);
        return 2 as libc::c_int
    }
    *keys_out = keys;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn free_keys(mut context: krb5_context,
                               mut rock: krb5_kdcpreauth_rock,
                               mut keys: *mut krb5_keyblock) {
    let mut k: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    if keys.is_null() { return }
    k = keys;
    while (*k).enctype != 0 as libc::c_int {
        krb5_free_keyblock_contents(context, k);
        k = k.offset(1)
    }
    free(keys as *mut libc::c_void);
}
#[c2rust::src_loc = "397:1"]
unsafe extern "C" fn request_body(mut context: krb5_context,
                                  mut rock: krb5_kdcpreauth_rock)
 -> *mut krb5_data {
    return (*rock).inner_body;
}
#[c2rust::src_loc = "403:1"]
unsafe extern "C" fn fast_armor(mut context: krb5_context,
                                mut rock: krb5_kdcpreauth_rock)
 -> *mut krb5_keyblock {
    return (*(*rock).rstate).armor_key;
}
#[c2rust::src_loc = "409:1"]
unsafe extern "C" fn get_string(mut context: krb5_context,
                                mut rock: krb5_kdcpreauth_rock,
                                mut key: *const libc::c_char,
                                mut value_out: *mut *mut libc::c_char)
 -> krb5_error_code {
    return krb5_dbe_get_string(context, (*rock).client, key, value_out);
}
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn free_string(mut context: krb5_context,
                                 mut rock: krb5_kdcpreauth_rock,
                                 mut string: *mut libc::c_char) {
    krb5_dbe_free_string(context, string);
}
#[c2rust::src_loc = "422:1"]
unsafe extern "C" fn client_entry(mut context: krb5_context,
                                  mut rock: krb5_kdcpreauth_rock)
 -> *mut libc::c_void {
    return (*rock).client as *mut libc::c_void;
}
#[c2rust::src_loc = "428:1"]
unsafe extern "C" fn event_context(mut context: krb5_context,
                                   mut rock: krb5_kdcpreauth_rock)
 -> *mut verto_ctx {
    return (*rock).vctx;
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn have_client_keys(mut context: krb5_context,
                                      mut rock: krb5_kdcpreauth_rock)
 -> krb5_boolean {
    let mut request: *mut krb5_kdc_req = (*rock).request;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*request).nktypes {
        if krb5_dbe_find_enctype(context, (*rock).client,
                                 *(*request).ktype.offset(i as isize),
                                 -(1 as libc::c_int), 0 as libc::c_int,
                                 &mut kd) == 0 as libc::c_int {
            return 1 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return 0 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn client_keyblock(mut context: krb5_context,
                                     mut rock: krb5_kdcpreauth_rock)
 -> *const krb5_keyblock {
    return (*rock).client_keyblock;
}
#[c2rust::src_loc = "455:1"]
unsafe extern "C" fn add_auth_indicator(mut context: krb5_context,
                                        mut rock: krb5_kdcpreauth_rock,
                                        mut indicator: *const libc::c_char)
 -> krb5_error_code {
    return authind_add(context, indicator, (*rock).auth_indicators);
}
#[c2rust::src_loc = "462:1"]
unsafe extern "C" fn get_cookie(mut context: krb5_context,
                                mut rock: krb5_kdcpreauth_rock,
                                mut pa_type: krb5_preauthtype,
                                mut out: *mut krb5_data) -> krb5_boolean {
    return kdc_fast_search_cookie((*rock).rstate, pa_type, out);
}
#[c2rust::src_loc = "469:1"]
unsafe extern "C" fn set_cookie(mut context: krb5_context,
                                mut rock: krb5_kdcpreauth_rock,
                                mut pa_type: krb5_preauthtype,
                                mut data: *const krb5_data)
 -> krb5_error_code {
    return kdc_fast_set_cookie((*rock).rstate, pa_type, data);
}
#[c2rust::src_loc = "476:1"]
unsafe extern "C" fn match_client(mut context: krb5_context,
                                  mut rock: krb5_kdcpreauth_rock,
                                  mut princ: krb5_principal) -> krb5_boolean {
    let mut ent: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut match_0: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut req_client: krb5_principal = (*(*rock).request).client;
    let mut client: krb5_principal = (*(*rock).client).princ;
    /* Check for a direct match against the request principal or
     * the post-canon client principal. */
    if krb5_principal_compare_flags(context, princ as krb5_const_principal,
                                    req_client as krb5_const_principal,
                                    2 as libc::c_int) != 0 ||
           krb5_principal_compare(context, princ as krb5_const_principal,
                                  client as krb5_const_principal) != 0 {
        return 1 as libc::c_int as krb5_boolean
    }
    if krb5_db_get_principal(context, princ as krb5_const_principal,
                             0 as libc::c_int as libc::c_uint, &mut ent) != 0
       {
        return 0 as libc::c_int as krb5_boolean
    }
    match_0 =
        krb5_principal_compare(context, (*ent).princ as krb5_const_principal,
                               client as krb5_const_principal);
    krb5_db_free_principal(context, ent);
    return match_0;
}
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn client_name(mut context: krb5_context,
                                 mut rock: krb5_kdcpreauth_rock)
 -> krb5_principal {
    return (*(*rock).client).princ;
}
#[c2rust::src_loc = "505:1"]
unsafe extern "C" fn send_freshness_token(mut context: krb5_context,
                                          mut rock: krb5_kdcpreauth_rock) {
    (*rock).send_freshness_token = 1 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "511:1"]
unsafe extern "C" fn check_freshness_token(mut context: krb5_context,
                                           mut rock: krb5_kdcpreauth_rock,
                                           mut token: *const krb5_data)
 -> krb5_error_code {
    let mut token_ts: krb5_timestamp = 0;
    let mut now: krb5_timestamp = 0;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut kb: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut token_kvno: krb5_kvno = 0;
    let mut cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut token_cksum: *mut uint8_t = 0 as *mut uint8_t;
    let mut token_cksum_len: size_t = 0;
    let mut valid: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut ckbuf: [libc::c_char; 4] = [0; 4];
    memset(&mut kb as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if !(krb5_timeofday(context, &mut now) != 0 as libc::c_int) {
        if !((*token).length <= 8 as libc::c_int as libc::c_uint) {
            token_ts =
                load_32_be((*token).data as *const libc::c_void) as
                    krb5_timestamp;
            token_kvno =
                load_32_be((*token).data.offset(4 as libc::c_int as isize) as
                               *const libc::c_void);
            token_cksum =
                ((*token).data as
                     *mut uint8_t).offset(8 as libc::c_int as isize);
            token_cksum_len =
                (*token).length.wrapping_sub(8 as libc::c_int as libc::c_uint)
                    as size_t;
            /* Check if the token timestamp is too old. */
            if !(ts_after(now, ts_incr(token_ts, 600 as libc::c_int)) != 0) {
                /* Fetch and decrypt the local krbtgt key of the token's kvno. */
                if !(krb5_dbe_find_enctype(context, (*rock).local_tgt,
                                           -(1 as libc::c_int),
                                           -(1 as libc::c_int),
                                           token_kvno as krb5_int32, &mut kd)
                         != 0 as libc::c_int) {
                    if !(krb5_dbe_decrypt_key_data(context,
                                                   0 as *const krb5_keyblock,
                                                   kd, &mut kb,
                                                   0 as *mut krb5_keysalt) !=
                             0 as libc::c_int) {
                        /* Verify the token checksum against the current KDC time.  The checksum
     * must use the mandatory checksum type of the krbtgt key's enctype. */
                        store_32_be(token_ts as libc::c_uint,
                                    ckbuf.as_mut_ptr() as *mut libc::c_void);
                        d =
                            make_data(ckbuf.as_mut_ptr() as *mut libc::c_void,
                                      ::std::mem::size_of::<[libc::c_char; 4]>()
                                          as libc::c_ulong as libc::c_uint);
                        cksum.magic =
                            -(1760647420 as libc::c_long) as krb5_magic;
                        cksum.checksum_type = 0 as libc::c_int;
                        cksum.length = token_cksum_len as libc::c_uint;
                        cksum.contents = token_cksum;
                        krb5_c_verify_checksum(context, &mut kb,
                                               514 as libc::c_int, &mut d,
                                               &mut cksum, &mut valid);
                    }
                }
            }
        }
    }
    krb5_free_keyblock_contents(context, &mut kb);
    return if valid != 0 {
               0 as libc::c_int as libc::c_long
           } else { -(1765328294 as libc::c_long) } as krb5_error_code;
}
#[c2rust::src_loc = "565:44"]
static mut callbacks: krb5_kdcpreauth_callbacks_st =
    unsafe {
        {
            let mut init =
                krb5_kdcpreauth_callbacks_st{vers: 5 as libc::c_int,
                                             max_time_skew:
                                                 Some(max_time_skew as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              -> krb5_deltat),
                                             client_keys:
                                                 Some(client_keys as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   *mut *mut krb5_keyblock)
                                                              ->
                                                                  krb5_error_code),
                                             free_keys:
                                                 Some(free_keys as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   *mut krb5_keyblock)
                                                              -> ()),
                                             request_body:
                                                 Some(request_body as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  *mut krb5_data),
                                             fast_armor:
                                                 Some(fast_armor as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  *mut krb5_keyblock),
                                             get_string:
                                                 Some(get_string as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   *const libc::c_char,
                                                                               _:
                                                                                   *mut *mut libc::c_char)
                                                              ->
                                                                  krb5_error_code),
                                             free_string:
                                                 Some(free_string as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   *mut libc::c_char)
                                                              -> ()),
                                             client_entry:
                                                 Some(client_entry as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  *mut libc::c_void),
                                             event_context:
                                                 Some(event_context as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  *mut verto_ctx),
                                             have_client_keys:
                                                 Some(have_client_keys as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  krb5_boolean),
                                             client_keyblock:
                                                 Some(client_keyblock as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  *const krb5_keyblock),
                                             add_auth_indicator:
                                                 Some(add_auth_indicator as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   *const libc::c_char)
                                                              ->
                                                                  krb5_error_code),
                                             get_cookie:
                                                 Some(get_cookie as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   krb5_preauthtype,
                                                                               _:
                                                                                   *mut krb5_data)
                                                              ->
                                                                  krb5_boolean),
                                             set_cookie:
                                                 Some(set_cookie as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   krb5_preauthtype,
                                                                               _:
                                                                                   *const krb5_data)
                                                              ->
                                                                  krb5_error_code),
                                             match_client:
                                                 Some(match_client as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   krb5_principal)
                                                              ->
                                                                  krb5_boolean),
                                             client_name:
                                                 Some(client_name as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              ->
                                                                  krb5_principal),
                                             send_freshness_token:
                                                 Some(send_freshness_token as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock)
                                                              -> ()),
                                             check_freshness_token:
                                                 Some(check_freshness_token as
                                                          unsafe extern "C" fn(_:
                                                                                   krb5_context,
                                                                               _:
                                                                                   krb5_kdcpreauth_rock,
                                                                               _:
                                                                                   *const krb5_data)
                                                              ->
                                                                  krb5_error_code),};
            init
        }
    };
#[c2rust::src_loc = "587:1"]
unsafe extern "C" fn find_pa_system(mut type_0: libc::c_int,
                                    mut preauth: *mut *mut preauth_system)
 -> krb5_error_code {
    let mut ap: *mut preauth_system = 0 as *mut preauth_system;
    if preauth_systems.is_null() {
        return -(1765328176 as libc::c_long) as krb5_error_code
    }
    ap = preauth_systems;
    while (*ap).type_0 != -(1 as libc::c_int) && (*ap).type_0 != type_0 {
        ap = ap.offset(1)
    }
    if (*ap).type_0 == -(1 as libc::c_int) {
        return -(1765328176 as libc::c_long) as krb5_error_code
    }
    *preauth = ap;
    return 0 as libc::c_int;
}
/* Find a pointer to the request-specific module data for pa_sys. */
#[c2rust::src_loc = "604:1"]
unsafe extern "C" fn find_modreq(mut pa_sys: *mut preauth_system,
                                 mut context: *mut request_pa_context,
                                 mut modreq_out:
                                     *mut *mut krb5_kdcpreauth_modreq)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    *modreq_out = 0 as *mut krb5_kdcpreauth_modreq;
    if context.is_null() {
        return -(1765328324 as libc::c_long) as krb5_error_code
    }
    i = 0 as libc::c_int;
    while i < (*context).n_contexts {
        if (*(*context).contexts.offset(i as isize)).pa_system == pa_sys {
            *modreq_out =
                &mut (*(*context).contexts.offset(i as isize)).modreq;
            return 0 as libc::c_int
        }
        i += 1
    }
    return -(1765328324 as libc::c_long) as krb5_error_code;
}
/*
 * Create a list of indices into the preauth_systems array, sorted by order of
 * preference.
 */
#[c2rust::src_loc = "628:1"]
unsafe extern "C" fn pa_list_includes(mut pa_data: *mut *mut krb5_pa_data,
                                      mut pa_type: krb5_preauthtype)
 -> krb5_boolean {
    while !(*pa_data).is_null() {
        if (**pa_data).pa_type == pa_type {
            return 1 as libc::c_int as krb5_boolean
        }
        pa_data = pa_data.offset(1)
    }
    return 0 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "638:1"]
unsafe extern "C" fn sort_pa_order(mut context: krb5_context,
                                   mut request: *mut krb5_kdc_req,
                                   mut pa_order: *mut libc::c_int) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    let mut n_repliers: size_t = 0;
    let mut n_key_replacers: size_t = 0;
    /* First, set up the default order. */
    i = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int as size_t;
    while j < n_preauth_systems {
        if (*preauth_systems.offset(j as isize)).return_padata.is_some() {
            let fresh7 = i;
            i = i.wrapping_add(1);
            *pa_order.offset(fresh7 as isize) = j as libc::c_int
        }
        j = j.wrapping_add(1)
    }
    n_repliers = i;
    *pa_order.offset(n_repliers as isize) = -(1 as libc::c_int);
    /* Reorder so that PA_REPLACES_KEY modules are listed first. */
    i = 0 as libc::c_int as size_t;
    while i < n_repliers {
        /* If this module replaces the key, then it's okay to leave it where it
         * is in the order. */
        if !((*preauth_systems.offset(*pa_order.offset(i as isize) as
                                          isize)).flags & 0x20 as libc::c_int
                 != 0) {
            /* If not, search for a module which does, and swap in the first one we
         * find. */
            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while j < n_repliers {
                if (*preauth_systems.offset(*pa_order.offset(j as isize) as
                                                isize)).flags &
                       0x20 as libc::c_int != 0 {
                    k = *pa_order.offset(j as isize) as size_t;
                    *pa_order.offset(j as isize) =
                        *pa_order.offset(i as isize);
                    *pa_order.offset(i as isize) = k as libc::c_int;
                    break ;
                } else { j = j.wrapping_add(1) }
            }
            /* If we didn't find one, we have moved all of the key-replacing
         * modules, and i is the count of those modules. */
            if j == n_repliers { break ; }
        }
        i = i.wrapping_add(1)
    }
    n_key_replacers = i;
    if !(*request).padata.is_null() {
        /* Now reorder the subset of modules which replace the key,
         * bubbling those which handle pa_data types provided by the
         * client ahead of the others.
         */
        i = 0 as libc::c_int as size_t;
        while i < n_key_replacers {
            if !(pa_list_includes((*request).padata,
                                  (*preauth_systems.offset(*pa_order.offset(i
                                                                                as
                                                                                isize)
                                                               as
                                                               isize)).type_0)
                     != 0) {
                j = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
                while j < n_key_replacers {
                    if pa_list_includes((*request).padata,
                                        (*preauth_systems.offset(*pa_order.offset(j
                                                                                      as
                                                                                      isize)
                                                                     as
                                                                     isize)).type_0)
                           != 0 {
                        k = *pa_order.offset(j as isize) as size_t;
                        *pa_order.offset(j as isize) =
                            *pa_order.offset(i as isize);
                        *pa_order.offset(i as isize) = k as libc::c_int;
                        break ;
                    } else { j = j.wrapping_add(1) }
                }
            }
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "711:1"]
pub unsafe extern "C" fn missing_required_preauth(mut client:
                                                      *mut krb5_db_entry,
                                                  mut server:
                                                      *mut krb5_db_entry,
                                                  mut enc_tkt_reply:
                                                      *mut krb5_enc_tkt_part)
 -> *const libc::c_char {
    if (*client).attributes & 0x80 as libc::c_int != 0 &&
           (*enc_tkt_reply).flags & 0x200000 as libc::c_int == 0 {
        return b"NEEDED_PREAUTH\x00" as *const u8 as *const libc::c_char
    }
    if (*client).attributes & 0x100 as libc::c_int != 0 &&
           (*enc_tkt_reply).flags & 0x100000 as libc::c_int == 0 {
        return b"NEEDED_HW_PREAUTH\x00" as *const u8 as *const libc::c_char
    }
    return 0 as *const libc::c_char;
}
/* Return true if request's enctypes indicate support for etype-info2. */
#[c2rust::src_loc = "737:1"]
unsafe extern "C" fn requires_info2(mut request: *const krb5_kdc_req)
 -> krb5_boolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*request).nktypes {
        if enctype_requires_etype_info_2(*(*request).ktype.offset(i as isize))
               != 0 {
            return 1 as libc::c_int as krb5_boolean
        }
        i += 1
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Add PA-ETYPE-INFO2 and possibly PA-ETYPE-INFO entries to pa_list as
 * appropriate for the request and client principal. */
#[c2rust::src_loc = "751:1"]
unsafe extern "C" fn add_etype_info(mut context: krb5_context,
                                    mut rock: krb5_kdcpreauth_rock,
                                    mut pa_list: *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut der: *mut krb5_data = 0 as *mut krb5_data;
    if (*rock).client_key.is_null() { return 0 as libc::c_int }
    if requires_info2((*rock).request) == 0 {
        /* Include PA-ETYPE-INFO only for old clients. */
        ret =
            make_etype_info(context, 0 as libc::c_int as krb5_boolean,
                            (*(*rock).client).princ, (*rock).client_key,
                            (*(*rock).client_keyblock).enctype, &mut der);
        if ret != 0 { return ret }
        ret = k5_add_pa_data_from_data(pa_list, 11 as libc::c_int, der);
        krb5_free_data(context, der);
        if ret != 0 { return ret }
    }
    /* Always include PA-ETYPE-INFO2. */
    ret =
        make_etype_info(context, 1 as libc::c_int as krb5_boolean,
                        (*(*rock).client).princ, (*rock).client_key,
                        (*(*rock).client_keyblock).enctype, &mut der);
    if ret != 0 { return ret }
    ret = k5_add_pa_data_from_data(pa_list, 19 as libc::c_int, der);
    krb5_free_data(context, der);
    return ret;
}
/* Add PW-SALT entries to pa_list as appropriate for the request and client
 * principal. */
#[c2rust::src_loc = "786:1"]
unsafe extern "C" fn add_pw_salt(mut context: krb5_context,
                                 mut rock: krb5_kdcpreauth_rock,
                                 mut pa_list: *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut salt: *mut krb5_data = 0 as *mut krb5_data;
    let mut salttype: krb5_int16 = 0;
    /* Only include this pa-data for old clients. */
    if (*rock).client_key.is_null() || requires_info2((*rock).request) != 0 {
        return 0 as libc::c_int
    }
    ret =
        krb5_dbe_compute_salt(context, (*rock).client_key,
                              (*(*rock).request).client as
                                  krb5_const_principal, &mut salttype,
                              &mut salt);
    if ret != 0 { return 0 as libc::c_int }
    ret = k5_add_pa_data_from_data(pa_list, 3 as libc::c_int, salt);
    krb5_free_data(context, salt);
    return ret;
}
#[c2rust::src_loc = "808:1"]
unsafe extern "C" fn add_freshness_token(mut context: krb5_context,
                                         mut rock: krb5_kdcpreauth_rock,
                                         mut pa_list:
                                             *mut *mut *mut krb5_pa_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut now: krb5_timestamp = 0;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut kb: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut cksum: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut ckbuf: [libc::c_char; 4] = [0; 4];
    memset(&mut cksum as *mut krb5_checksum as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_checksum>() as libc::c_ulong);
    memset(&mut kb as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if (*rock).send_freshness_token == 0 { return 0 as libc::c_int }
    if krb5int_find_pa_data(context, (*(*rock).request).padata,
                            150 as libc::c_int).is_null() {
        return 0 as libc::c_int
    }
    /* Fetch and decrypt the current local krbtgt key. */
    ret =
        krb5_dbe_find_enctype(context, (*rock).local_tgt, -(1 as libc::c_int),
                              -(1 as libc::c_int), 0 as libc::c_int, &mut kd);
    if !(ret != 0) {
        ret =
            krb5_dbe_decrypt_key_data(context, 0 as *const krb5_keyblock, kd,
                                      &mut kb, 0 as *mut krb5_keysalt);
        if !(ret != 0) {
            /* Compute a checksum over the current KDC time. */
            ret = krb5_timeofday(context, &mut now);
            if !(ret != 0) {
                store_32_be(now as libc::c_uint,
                            ckbuf.as_mut_ptr() as *mut libc::c_void);
                d =
                    make_data(ckbuf.as_mut_ptr() as *mut libc::c_void,
                              ::std::mem::size_of::<[libc::c_char; 4]>() as
                                  libc::c_ulong as libc::c_uint);
                ret =
                    krb5_c_make_checksum(context, 0 as libc::c_int, &mut kb,
                                         514 as libc::c_int, &mut d,
                                         &mut cksum);
                /* Compose a freshness token from the time, krbtgt kvno, and checksum. */
                ret =
                    k5_alloc_pa_data(150 as libc::c_int,
                                     (8 as libc::c_int as
                                          libc::c_uint).wrapping_add(cksum.length)
                                         as size_t, &mut pa);
                if !(ret != 0) {
                    store_32_be(now as libc::c_uint,
                                (*pa).contents as *mut libc::c_void);
                    store_32_be((*kd).key_data_kvno as libc::c_uint,
                                (*pa).contents.offset(4 as libc::c_int as
                                                          isize) as
                                    *mut libc::c_void);
                    memcpy((*pa).contents.offset(8 as libc::c_int as isize) as
                               *mut libc::c_void,
                           cksum.contents as *const libc::c_void,
                           cksum.length as libc::c_ulong);
                    ret = k5_add_pa_data_element(pa_list, &mut pa)
                }
            }
        }
    }
    krb5_free_keyblock_contents(context, &mut kb);
    krb5_free_checksum_contents(context, &mut cksum);
    k5_free_pa_data_element(pa);
    return ret;
}
#[c2rust::src_loc = "879:1"]
unsafe extern "C" fn hint_list_finish(mut state: *mut hint_state,
                                      mut code: krb5_error_code) {
    let mut oldrespond: kdc_hint_respond_fn = (*state).respond;
    let mut oldarg: *mut libc::c_void = (*state).arg;
    let mut kdc_active_realm: *mut kdc_realm_t = (*state).realm;
    /* Add a freshness token if a preauth module requested it and the client
     * request indicates support for it. */
    if code == 0 {
        code =
            add_freshness_token((*kdc_active_realm).realm_context,
                                (*state).rock, &mut (*state).pa_data)
    }
    if code == 0 {
        if (*state).pa_data.is_null() {
            krb5_klog_syslog(6 as libc::c_int,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"%spreauth required but hint list is empty\x00"
                                          as *const u8 as
                                          *const libc::c_char),
                             if (*state).hw_only != 0 {
                                 b"hw\x00" as *const u8 as *const libc::c_char
                             } else {
                                 b"\x00" as *const u8 as *const libc::c_char
                             });
        }
        *(*state).e_data_out = (*state).pa_data;
        (*state).pa_data = 0 as *mut *mut krb5_pa_data
    }
    krb5_free_pa_data((*kdc_active_realm).realm_context, (*state).pa_data);
    free(state as *mut libc::c_void);
    Some(oldrespond.expect("non-null function pointer")).expect("non-null function pointer")(oldarg);
}
#[c2rust::src_loc = "910:1"]
unsafe extern "C" fn finish_get_edata(mut arg: *mut libc::c_void,
                                      mut code: krb5_error_code,
                                      mut pa: *mut krb5_pa_data) {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut state: *mut hint_state = arg as *mut hint_state;
    if code == 0 as libc::c_int {
        if pa.is_null() {
            ret =
                k5_alloc_pa_data((*state).pa_type, 0 as libc::c_int as size_t,
                                 &mut pa);
            if ret != 0 {
                current_block = 10200659599895573121;
            } else { current_block = 15240798224410183470; }
        } else { current_block = 15240798224410183470; }
        match current_block {
            15240798224410183470 => {
                ret = k5_add_pa_data_element(&mut (*state).pa_data, &mut pa);
                k5_free_pa_data_element(pa);
                if ret != 0 {
                    current_block = 10200659599895573121;
                } else { current_block = 5720623009719927633; }
            }
            _ => { }
        }
        match current_block {
            5720623009719927633 => { }
            _ => { hint_list_finish(state, ret); return; }
        }
    }
    (*state).ap = (*state).ap.offset(1);
    hint_list_next(state);
}
#[c2rust::src_loc = "936:1"]
unsafe extern "C" fn hint_list_next(mut state: *mut hint_state) {
    let mut ap: *mut preauth_system = (*state).ap;
    let mut kdc_active_realm: *mut kdc_realm_t = (*state).realm;
    if (*ap).type_0 == -(1 as libc::c_int) {
        hint_list_finish(state, 0 as libc::c_int);
        return
    }
    if !((*state).hw_only != 0 && (*ap).flags & 0x4 as libc::c_int == 0) {
        if !((*ap).flags & 0x80 as libc::c_int != 0) {
            (*state).pa_type = (*ap).type_0;
            if (*ap).get_edata.is_some() {
                (*ap).get_edata.expect("non-null function pointer")((*kdc_active_realm).realm_context,
                                                                    (*state).request,
                                                                    &mut callbacks,
                                                                    (*state).rock,
                                                                    (*ap).moddata,
                                                                    (*ap).type_0,
                                                                    Some(finish_get_edata
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut libc::c_void,
                                                                                                  _:
                                                                                                      krb5_error_code,
                                                                                                  _:
                                                                                                      *mut krb5_pa_data)
                                                                                 ->
                                                                                     ()),
                                                                    state as
                                                                        *mut libc::c_void);
            } else {
                finish_get_edata(state as *mut libc::c_void, 0 as libc::c_int,
                                 0 as *mut krb5_pa_data);
            }
            return
        }
    }
    (*state).ap = (*state).ap.offset(1);
    hint_list_next(state);
}
#[no_mangle]
#[c2rust::src_loc = "965:1"]
pub unsafe extern "C" fn get_preauth_hint_list(mut request: *mut krb5_kdc_req,
                                               mut rock: krb5_kdcpreauth_rock,
                                               mut e_data_out:
                                                   *mut *mut *mut krb5_pa_data,
                                               mut respond:
                                                   kdc_hint_respond_fn,
                                               mut arg: *mut libc::c_void) {
    let mut kdc_active_realm: *mut kdc_realm_t = (*(*rock).rstate).realm_data;
    let mut state: *mut hint_state = 0 as *mut hint_state;
    *e_data_out = 0 as *mut *mut krb5_pa_data;
    /* Allocate our state. */
    state =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<hint_state>() as libc::c_ulong) as
            *mut hint_state;
    if !state.is_null() {
        (*state).hw_only =
            (*(*rock).client).attributes & 0x100 as libc::c_int;
        (*state).respond = respond;
        (*state).arg = arg;
        (*state).request = request;
        (*state).rock = rock;
        (*state).realm = (*(*rock).rstate).realm_data;
        (*state).e_data_out = e_data_out;
        (*state).pa_data = 0 as *mut *mut krb5_pa_data;
        (*state).ap = preauth_systems;
        /* Add an empty PA-FX-FAST element to advertise FAST support. */
        if !(k5_add_empty_pa_data(&mut (*state).pa_data, 136 as libc::c_int)
                 != 0 as libc::c_int) {
            if !(add_etype_info((*kdc_active_realm).realm_context, rock,
                                &mut (*state).pa_data) != 0 as libc::c_int) {
                hint_list_next(state);
                return
            }
        }
    }
    if !state.is_null() {
        krb5_free_pa_data((*kdc_active_realm).realm_context,
                          (*state).pa_data);
    }
    free(state as *mut libc::c_void);
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg);
}
/*
 * Add authorization data returned from preauth modules to the ticket
 * It is assumed that ad is a "null-terminated" array of krb5_authdata ptrs
 */
#[c2rust::src_loc = "1011:1"]
unsafe extern "C" fn add_authorization_data(mut enc_tkt_part:
                                                *mut krb5_enc_tkt_part,
                                            mut ad: *mut *mut krb5_authdata)
 -> krb5_error_code {
    let mut newad: *mut *mut krb5_authdata =
        0 as *mut *mut krb5_authdata; /* nothing to add */
    let mut oldones: libc::c_int = 0;
    let mut newones: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if enc_tkt_part.is_null() || ad.is_null() { return 22 as libc::c_int }
    newones = 0 as libc::c_int;
    while !(*ad.offset(newones as isize)).is_null() { newones += 1 }
    if newones == 0 as libc::c_int { return 0 as libc::c_int }
    if (*enc_tkt_part).authorization_data.is_null() {
        oldones = 0 as libc::c_int
    } else {
        oldones = 0 as libc::c_int;
        while !(*(*enc_tkt_part).authorization_data.offset(oldones as
                                                               isize)).is_null()
              {
            oldones += 1
        }
    }
    newad =
        malloc(((oldones + newones + 1 as libc::c_int) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut krb5_authdata>()
                                                    as libc::c_ulong)) as
            *mut *mut krb5_authdata;
    if newad.is_null() { return 12 as libc::c_int }
    /* Copy any existing pointers */
    i = 0 as libc::c_int;
    while i < oldones {
        let ref mut fresh8 = *newad.offset(i as isize);
        *fresh8 = *(*enc_tkt_part).authorization_data.offset(i as isize);
        i += 1
    }
    /* Add the new ones */
    i = 0 as libc::c_int;
    while i < newones {
        let ref mut fresh9 = *newad.offset((oldones + i) as isize);
        *fresh9 = *ad.offset(i as isize);
        i += 1
    }
    /* Terminate the new list */
    let ref mut fresh10 = *newad.offset((oldones + i) as isize);
    *fresh10 = 0 as *mut krb5_authdata;
    /* Free any existing list */
    if !(*enc_tkt_part).authorization_data.is_null() {
        free((*enc_tkt_part).authorization_data as *mut libc::c_void);
    }
    /* Install our new list */
    (*enc_tkt_part).authorization_data = newad;
    return 0 as libc::c_int;
}
/* Return code if it is 0 or one of the codes we pass through to the client.
 * Otherwise return KRB5KDC_ERR_PREAUTH_FAILED. */
#[c2rust::src_loc = "1083:1"]
unsafe extern "C" fn filter_preauth_error(mut code: krb5_error_code)
 -> krb5_error_code {
    let mut current_block_1: u64;
    /* The following switch statement allows us
     * to return some preauth system errors back to the client.
     */
    match code {
        -1765328303 => {
            /* earlier drafts of what became rfc 4556 */
            current_block_1 = 13963937407232170429;
        }
        -1765328310 => {
            /* This value is shared with
         *     KRB5KDC_ERR_DH_KEY_PARAMETERS_NOT_ACCEPTED. */
        /* case KRB5KDC_ERR_KEY_TOO_WEAK: */
            current_block_1 = 15494386574687498198;
        }
        -1750600189 => { current_block_1 = 15494386574687498198; }
        -1765328284 => { current_block_1 = 2355172110000774960; }
        0 | -1765328353 | -1765328347 | -1765328359 | -1765328370 |
        -1765328322 | -1765328320 | -1765328319 | -1765328314 | -1765328313 |
        -1765328312 | -1765328311 | -1765328309 | -1765328307 | -1765328306 |
        -1765328305 | -1765328304 | -1765328318 | -1765328321 | -1765328293 =>
        {
            current_block_1 = 13963937407232170429;
        }
        _ => { return -(1765328360 as libc::c_long) as krb5_error_code }
    }
    match current_block_1 {
        15494386574687498198 =>
        /* pkinit alg-agility */
        {
            current_block_1 = 2355172110000774960;
        }
        _ => { }
    }
    match current_block_1 {
        2355172110000774960 =>
        /* rfc 6113 */
        {
        }
        _ => { }
    }
    /* rfc 4556 */
    return code;
}
/*
 * If the client performed optimistic pre-authentication for a multi-round-trip
 * mechanism, it may need key information to complete the exchange, so send it
 * a PA-ETYPE-INFO2 element in addition to the pa-data from the module.
 */
#[c2rust::src_loc = "1132:1"]
unsafe extern "C" fn maybe_add_etype_info2(mut state: *mut padata_state,
                                           mut code: krb5_error_code)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut context: krb5_context = (*state).context;
    let mut rock: krb5_kdcpreauth_rock = (*state).rock;
    let mut der: *mut krb5_data = 0 as *mut krb5_data;
    /* Only add key information when requesting another preauth round trip. */
    if code as libc::c_long != -(1765328293 as libc::c_long) {
        return 0 as libc::c_int
    }
    /* Don't try to add key information when there is no key. */
    if (*rock).client_key.is_null() { return 0 as libc::c_int }
    /* If the client sent a cookie, it has already seen a KDC response with key
     * information. */
    if !krb5int_find_pa_data(context, (*(*state).request).padata,
                             133 as libc::c_int).is_null() {
        return 0 as libc::c_int
    }
    ret =
        make_etype_info(context, 1 as libc::c_int as krb5_boolean,
                        (*(*rock).client).princ, (*rock).client_key,
                        (*(*rock).client_keyblock).enctype, &mut der);
    if ret != 0 { return ret }
    ret =
        k5_add_pa_data_from_data(&mut (*state).pa_e_data, 19 as libc::c_int,
                                 der);
    krb5_free_data(context, der);
    return ret;
}
/* Release state and respond to the AS-REQ processing code with the result of
 * checking pre-authentication data. */
#[c2rust::src_loc = "1166:1"]
unsafe extern "C" fn finish_check_padata(mut state: *mut padata_state,
                                         mut code: krb5_error_code) {
    let mut respond: kdc_preauth_respond_fn = None;
    let mut arg: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*state).pa_ok != 0 || (*state).pa_found == 0 {
        /* Return successfully.  If we didn't match a preauth system, we may
         * return PREAUTH_REQUIRED later, but we didn't fail to verify. */
        code = 0 as libc::c_int
    } else if maybe_add_etype_info2(state, code) != 0 as libc::c_int {
        code = -(1765328360 as libc::c_long) as krb5_error_code
    } else {
        /* Add key information to the saved error pa-data if required. */
        /* Return any saved error pa-data, stealing the pointer from state. */
        *(*state).e_data_out = (*state).pa_e_data;
        *(*state).typed_e_data_out = (*state).typed_e_data_flag;
        (*state).pa_e_data = 0 as *mut *mut krb5_pa_data
    }
    /* Discard saved error pa-data if we aren't returning it, free state, and
     * respond to the AS-REQ processing code. */
    respond = (*state).respond;
    arg = (*state).arg;
    krb5_free_pa_data((*state).context, (*state).pa_e_data);
    free(state as *mut libc::c_void);
    Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                          filter_preauth_error(code));
}
#[c2rust::src_loc = "1203:1"]
unsafe extern "C" fn finish_verify_padata(mut arg: *mut libc::c_void,
                                          mut code: krb5_error_code,
                                          mut modreq: krb5_kdcpreauth_modreq,
                                          mut e_data: *mut *mut krb5_pa_data,
                                          mut authz_data:
                                              *mut *mut krb5_authdata) {
    let mut state: *mut padata_state = arg as *mut padata_state;
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut typed_e_data_flag: krb5_boolean = 0;
    if !state.is_null() {
    } else {
        __assert_fail(b"state\x00" as *const u8 as *const libc::c_char,
                      b"kdc_preauth.c\x00" as *const u8 as
                          *const libc::c_char,
                      1212 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 110],
                                                &[libc::c_char; 110]>(b"void finish_verify_padata(void *, krb5_error_code, krb5_kdcpreauth_modreq, krb5_pa_data **, krb5_authdata **)\x00")).as_ptr());
    }
    *(*state).modreq_ptr = modreq;
    if code != 0 {
        emsg = krb5_get_error_message((*state).context, code);
        krb5_klog_syslog(6 as libc::c_int,
                         b"preauth (%s) verify failure: %s\x00" as *const u8
                             as *const libc::c_char, (*(*state).pa_sys).name,
                         emsg);
        krb5_free_error_message((*state).context, emsg);
        /* Ignore authorization data returned from modules that fail */
        if !authz_data.is_null() {
            krb5_free_authdata((*state).context, authz_data);
            authz_data = 0 as *mut *mut krb5_authdata
        }
        typed_e_data_flag =
            ((*(*state).pa_sys).flags & 0x100 as libc::c_int !=
                 0 as libc::c_int) as libc::c_int as krb5_boolean;
        /*
         * We'll return edata from either the first PA_REQUIRED module
         * that fails, or the first non-PA_REQUIRED module that fails.
         * Hang on to edata from the first non-PA_REQUIRED module.
         * If we've already got one saved, simply discard this one.
         */
        if (*(*state).pa_sys).flags & 0x8 as libc::c_int != 0 {
            /* free up any previous edata we might have been saving */
            if !(*state).pa_e_data.is_null() {
                krb5_free_pa_data((*state).context, (*state).pa_e_data);
            }
            (*state).pa_e_data = e_data;
            (*state).typed_e_data_flag = typed_e_data_flag;
            /* Make sure we use the current retval */
            (*state).pa_ok = 0 as libc::c_int;
            finish_check_padata(state, code);
            return
        } else {
            if (*state).pa_e_data.is_null() {
                /* save the first error code and e-data */
                (*state).pa_e_data = e_data;
                (*state).typed_e_data_flag = typed_e_data_flag;
                (*state).saved_code = code
            } else if !e_data.is_null() {
                /* discard this extra e-data from non-PA_REQUIRED module */
                krb5_free_pa_data((*state).context, e_data);
            }
        }
    } else {
        /* Ignore any edata returned on success */
        if !e_data.is_null() { krb5_free_pa_data((*state).context, e_data); }
        /* Add any authorization data to the ticket */
        if !authz_data.is_null() {
            add_authorization_data((*state).enc_tkt_reply, authz_data);
            free(authz_data as *mut libc::c_void);
        }
        (*state).pa_ok = 1 as libc::c_int;
        if (*(*state).pa_sys).flags & 0x10 as libc::c_int != 0 {
            finish_check_padata(state, (*state).saved_code);
            return
        }
    }
    next_padata(state);
}
#[c2rust::src_loc = "1280:1"]
unsafe extern "C" fn next_padata(mut state: *mut padata_state) {
    if !state.is_null() {
    } else {
        __assert_fail(b"state\x00" as *const u8 as *const libc::c_char,
                      b"kdc_preauth.c\x00" as *const u8 as
                          *const libc::c_char,
                      1283 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 40],
                                                &[libc::c_char; 40]>(b"void next_padata(struct padata_state *)\x00")).as_ptr());
    }
    if (*state).padata.is_null() {
        (*state).padata = (*(*state).request).padata
    } else { (*state).padata = (*state).padata.offset(1) }
    if (*(*state).padata).is_null() {
        finish_check_padata(state, (*state).saved_code);
        return
    }
    if !(find_pa_system((**(*state).padata).pa_type, &mut (*state).pa_sys) !=
             0) {
        if !(find_modreq((*state).pa_sys,
                         *(*state).padata_context as *mut request_pa_context,
                         &mut (*state).modreq_ptr) != 0) {
            if !(*(*state).pa_sys).verify_padata.is_none() {
                (*state).pa_found += 1;
                (*(*state).pa_sys).verify_padata.expect("non-null function pointer")((*state).context,
                                                                                     (*state).req_pkt,
                                                                                     (*state).request,
                                                                                     (*state).enc_tkt_reply,
                                                                                     *(*state).padata,
                                                                                     &mut callbacks,
                                                                                     (*state).rock,
                                                                                     (*(*state).pa_sys).moddata,
                                                                                     Some(finish_verify_padata
                                                                                              as
                                                                                              unsafe extern "C" fn(_:
                                                                                                                       *mut libc::c_void,
                                                                                                                   _:
                                                                                                                       krb5_error_code,
                                                                                                                   _:
                                                                                                                       krb5_kdcpreauth_modreq,
                                                                                                                   _:
                                                                                                                       *mut *mut krb5_pa_data,
                                                                                                                   _:
                                                                                                                       *mut *mut krb5_authdata)
                                                                                                  ->
                                                                                                      ()),
                                                                                     state
                                                                                         as
                                                                                         *mut libc::c_void);
                return
            }
        }
    }
    next_padata(state);
}
/*
 * This routine is called to verify the preauthentication information
 * for a V5 request.
 *
 * Returns 0 if the pre-authentication is valid, non-zero to indicate
 * an error code of some sort.
 */
#[no_mangle]
#[c2rust::src_loc = "1327:1"]
pub unsafe extern "C" fn check_padata(mut context: krb5_context,
                                      mut rock: krb5_kdcpreauth_rock,
                                      mut req_pkt: *mut krb5_data,
                                      mut request: *mut krb5_kdc_req,
                                      mut enc_tkt_reply:
                                          *mut krb5_enc_tkt_part,
                                      mut padata_context:
                                          *mut *mut libc::c_void,
                                      mut e_data: *mut *mut *mut krb5_pa_data,
                                      mut typed_e_data: *mut krb5_boolean,
                                      mut respond: kdc_preauth_respond_fn,
                                      mut arg: *mut libc::c_void) {
    let mut state: *mut padata_state = 0 as *mut padata_state;
    if (*request).padata.is_null() {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              0
                                                                                                  as
                                                                                                  libc::c_int);
        return
    }
    if make_padata_context(context, padata_context) != 0 as libc::c_int {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              -(1765328324
                                                                                                    as
                                                                                                    libc::c_long)
                                                                                                  as
                                                                                                  krb5_error_code);
        return
    }
    state =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<padata_state>() as libc::c_ulong) as
            *mut padata_state;
    if state.is_null() {
        Some(respond.expect("non-null function pointer")).expect("non-null function pointer")(arg,
                                                                                              12
                                                                                                  as
                                                                                                  libc::c_int);
        return
    }
    (*state).respond = respond;
    (*state).arg = arg;
    (*state).context = context;
    (*state).rock = rock;
    (*state).req_pkt = req_pkt;
    (*state).request = request;
    (*state).enc_tkt_reply = enc_tkt_reply;
    (*state).padata_context = padata_context;
    (*state).e_data_out = e_data;
    (*state).typed_e_data_out = typed_e_data;
    (*state).realm = (*(*rock).rstate).realm_data;
    next_padata(state);
}
/* Return true if k1 and k2 have the same type and contents. */
#[c2rust::src_loc = "1371:1"]
unsafe extern "C" fn keyblock_equal(mut k1: *const krb5_keyblock,
                                    mut k2: *const krb5_keyblock)
 -> krb5_boolean {
    if (*k1).enctype != (*k2).enctype {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*k1).length != (*k2).length {
        return 0 as libc::c_int as krb5_boolean
    }
    return (memcmp((*k1).contents as *const libc::c_void,
                   (*k2).contents as *const libc::c_void,
                   (*k1).length as libc::c_ulong) == 0 as libc::c_int) as
               libc::c_int as krb5_boolean;
}
/*
 * return_padata creates any necessary preauthentication
 * structures which should be returned by the KDC to the client
 */
#[no_mangle]
#[c2rust::src_loc = "1385:1"]
pub unsafe extern "C" fn return_padata(mut context: krb5_context,
                                       mut rock: krb5_kdcpreauth_rock,
                                       mut req_pkt: *mut krb5_data,
                                       mut request: *mut krb5_kdc_req,
                                       mut reply: *mut krb5_kdc_rep,
                                       mut encrypting_key: *mut krb5_keyblock,
                                       mut padata_context:
                                           *mut *mut libc::c_void)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut send_pa_list: *mut *mut krb5_pa_data =
        0 as *mut *mut krb5_pa_data;
    let mut send_pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    let mut null_item: krb5_pa_data =
        krb5_pa_data{magic: 0,
                     pa_type: 0,
                     length: 0,
                     contents: 0 as *mut krb5_octet,};
    let mut ap: *mut preauth_system = 0 as *mut preauth_system;
    let mut pa_order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pa_type: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut modreq_ptr: *mut krb5_kdcpreauth_modreq =
        0 as *mut krb5_kdcpreauth_modreq;
    let mut key_modified: krb5_boolean = 0;
    let mut original_key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    memset(&mut original_key as *mut krb5_keyblock as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_keyblock>() as libc::c_ulong);
    if (*padata_context).is_null() &&
           make_padata_context(context, padata_context) != 0 as libc::c_int {
        return -(1765328324 as libc::c_long) as krb5_error_code
    }
    ap = preauth_systems;
    while (*ap).type_0 != -(1 as libc::c_int) {
        if (*ap).return_padata.is_some() { size += 1 }
        ap = ap.offset(1)
    }
    pa_order =
        k5calloc((size + 1 as libc::c_int) as size_t,
                 ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                 &mut retval) as *mut libc::c_int;
    if !pa_order.is_null() {
        sort_pa_order(context, request, pa_order);
        retval =
            krb5_copy_keyblock_contents(context, encrypting_key,
                                        &mut original_key);
        if !(retval != 0) {
            key_modified = 0 as libc::c_int as krb5_boolean;
            null_item.contents = 0 as *mut krb5_octet;
            null_item.length = 0 as libc::c_int as libc::c_uint;
            pa_type = pa_order;
            loop  {
                if !(*pa_type != -(1 as libc::c_int)) {
                    current_block = 2989495919056355252;
                    break ;
                }
                ap =
                    &mut *preauth_systems.offset(*pa_type as isize) as
                        *mut preauth_system;
                if !(key_modified != 0 &&
                         (*ap).flags & 0x20 as libc::c_int != 0) {
                    if !(*ap).return_padata.is_none() {
                        if !(find_modreq(ap,
                                         *padata_context as
                                             *mut request_pa_context,
                                         &mut modreq_ptr) != 0) {
                            pa = &mut null_item;
                            null_item.pa_type = (*ap).type_0;
                            if !(*request).padata.is_null() {
                                padata = (*request).padata;
                                while !(*padata).is_null() {
                                    if (**padata).pa_type == (*ap).type_0 {
                                        pa = *padata;
                                        break ;
                                    } else { padata = padata.offset(1) }
                                }
                            }
                            send_pa = 0 as *mut krb5_pa_data;
                            retval =
                                (*ap).return_padata.expect("non-null function pointer")(context,
                                                                                        pa,
                                                                                        req_pkt,
                                                                                        request,
                                                                                        reply,
                                                                                        encrypting_key,
                                                                                        &mut send_pa,
                                                                                        &mut callbacks,
                                                                                        rock,
                                                                                        (*ap).moddata,
                                                                                        *modreq_ptr);
                            if retval != 0 {
                                current_block = 8628974057293894316;
                                break ;
                            }
                            if !send_pa.is_null() {
                                retval =
                                    k5_add_pa_data_element(&mut send_pa_list,
                                                           &mut send_pa);
                                k5_free_pa_data_element(send_pa);
                                if retval != 0 {
                                    current_block = 8628974057293894316;
                                    break ;
                                }
                            }
                            if key_modified == 0 &&
                                   keyblock_equal(&mut original_key,
                                                  encrypting_key) == 0 {
                                key_modified =
                                    1 as libc::c_int as krb5_boolean
                            }
                        }
                    }
                }
                pa_type = pa_type.offset(1)
            }
            match current_block {
                8628974057293894316 => { }
                _ =>
                /*
     * Add etype-info and pw-salt pa-data as needed.  If we replaced the reply
     * key, we can't send consistent etype-info; the salt from the client key
     * data doesn't correspond to the replaced reply key, and RFC 4120 section
     * 5.2.7.5 forbids us from sending etype-info describing the initial reply
     * key in an AS-REP if it doesn't have the same enctype as the replaced
     * reply key.  For all current and forseeable preauth mechs, we can assume
     * the client received etype-info2 in an earlier step and already computed
     * the initial reply key if it needed it.  The client can determine the
     * enctype of the replaced reply key from the etype field of the enc-part
     * field of the AS-REP.
     */
                {
                    if key_modified == 0 {
                        retval =
                            add_etype_info(context, rock, &mut send_pa_list);
                        if retval != 0 {
                            current_block = 8628974057293894316;
                        } else {
                            retval =
                                add_pw_salt(context, rock, &mut send_pa_list);
                            if retval != 0 {
                                current_block = 8628974057293894316;
                            } else { current_block = 13281731871476506071; }
                        }
                    } else { current_block = 13281731871476506071; }
                    match current_block {
                        8628974057293894316 => { }
                        _ => {
                            if !send_pa_list.is_null() {
                                (*reply).padata = send_pa_list;
                                send_pa_list = 0 as *mut *mut krb5_pa_data
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_keyblock_contents(context, &mut original_key);
    free(pa_order as *mut libc::c_void);
    krb5_free_pa_data(context, send_pa_list);
    return retval;
}
#[c2rust::src_loc = "1499:1"]
unsafe extern "C" fn _make_etype_info_entry(mut context: krb5_context,
                                            mut client_princ: krb5_principal,
                                            mut client_key:
                                                *mut krb5_key_data,
                                            mut etype: krb5_enctype,
                                            mut entry_out:
                                                *mut *mut krb5_etype_info_entry,
                                            mut etype_info2: libc::c_int)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut salttype: krb5_int16 = 0;
    let mut salt: *mut krb5_data = 0 as *mut krb5_data;
    let mut entry: *mut krb5_etype_info_entry =
        0 as *mut krb5_etype_info_entry;
    *entry_out = 0 as *mut krb5_etype_info_entry;
    entry =
        malloc(::std::mem::size_of::<krb5_etype_info_entry>() as
                   libc::c_ulong) as *mut krb5_etype_info_entry;
    if entry.is_null() { return 12 as libc::c_int }
    (*entry).magic = -(1760647385 as libc::c_long) as krb5_magic;
    (*entry).etype = etype;
    (*entry).length =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    (*entry).salt = 0 as *mut krb5_octet;
    (*entry).s2kparams = empty_data();
    retval =
        krb5_dbe_compute_salt(context, client_key,
                              client_princ as krb5_const_principal,
                              &mut salttype, &mut salt);
    if !(retval != 0) {
        (*entry).length = (*salt).length;
        (*entry).salt = (*salt).data as *mut libc::c_uchar;
        (*salt).data = 0 as *mut libc::c_char;
        *entry_out = entry;
        entry = 0 as *mut krb5_etype_info_entry
    }
    if !entry.is_null() {
        krb5_free_data_contents(context, &mut (*entry).s2kparams);
    }
    free(entry as *mut libc::c_void);
    krb5_free_data(context, salt);
    return retval;
}
/* Encode an etype-info or etype-info2 message for client_key with the given
 * enctype, using client to compute the salt if necessary. */
#[c2rust::src_loc = "1541:1"]
unsafe extern "C" fn make_etype_info(mut context: krb5_context,
                                     mut etype_info2: krb5_boolean,
                                     mut client: krb5_principal,
                                     mut client_key: *mut krb5_key_data,
                                     mut enctype: krb5_enctype,
                                     mut der_out: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut entry: *mut *mut krb5_etype_info_entry =
        0 as *mut *mut krb5_etype_info_entry;
    *der_out = 0 as *mut krb5_data;
    entry =
        k5calloc(2 as libc::c_int as size_t,
                 ::std::mem::size_of::<*mut krb5_etype_info_entry>() as
                     libc::c_ulong, &mut retval) as
            *mut *mut krb5_etype_info_entry;
    if !entry.is_null() {
        retval =
            _make_etype_info_entry(context, client, client_key, enctype,
                                   &mut *entry.offset(0 as libc::c_int as
                                                          isize),
                                   etype_info2 as libc::c_int);
        if !(retval != 0 as libc::c_int) {
            if etype_info2 != 0 {
                retval = encode_krb5_etype_info2(entry, der_out)
            } else { retval = encode_krb5_etype_info(entry, der_out) }
        }
    }
    krb5_free_etype_info(context, entry);
    return retval;
}
/*
 * Returns TRUE if the PAC should be included
 */
#[no_mangle]
#[c2rust::src_loc = "1572:1"]
pub unsafe extern "C" fn include_pac_p(mut context: krb5_context,
                                       mut request: *mut krb5_kdc_req)
 -> krb5_boolean {
    let mut code: krb5_error_code = 0; /* default is to return PAC */
    let mut padata: *mut *mut krb5_pa_data = 0 as *mut *mut krb5_pa_data;
    let mut retval: krb5_boolean = 1 as libc::c_int as krb5_boolean;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut req: *mut krb5_pa_pac_req = 0 as *mut krb5_pa_pac_req;
    if (*request).padata.is_null() { return retval }
    padata = (*request).padata;
    while !(*padata).is_null() {
        if (**padata).pa_type == 128 as libc::c_int {
            data.data = (**padata).contents as *mut libc::c_char;
            data.length = (**padata).length;
            code = decode_krb5_pa_pac_req(&mut data, &mut req);
            if code == 0 as libc::c_int {
                retval = (*req).include_pac;
                krb5_free_pa_pac_req(context, req);
                req = 0 as *mut krb5_pa_pac_req
            }
            break ;
        } else { padata = padata.offset(1) }
    }
    return retval;
}
#[c2rust::src_loc = "1603:1"]
unsafe extern "C" fn return_referral_enc_padata(mut context: krb5_context,
                                                mut reply:
                                                    *mut krb5_enc_kdc_rep_part,
                                                mut server:
                                                    *mut krb5_db_entry)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut tl_data: krb5_tl_data =
        krb5_tl_data{tl_data_next: 0 as *mut _krb5_tl_data,
                     tl_data_type: 0,
                     tl_data_length: 0,
                     tl_data_contents: 0 as *mut krb5_octet,};
    let mut pa: *mut krb5_pa_data = 0 as *mut krb5_pa_data;
    tl_data.tl_data_type = 0x300 as libc::c_int as krb5_int16;
    code = krb5_dbe_lookup_tl_data(context, server, &mut tl_data);
    if code != 0 || tl_data.tl_data_length as libc::c_int == 0 as libc::c_int
       {
        return 0 as libc::c_int
    }
    code =
        k5_alloc_pa_data(20 as libc::c_int, tl_data.tl_data_length as size_t,
                         &mut pa);
    if code != 0 { return code }
    memcpy((*pa).contents as *mut libc::c_void,
           tl_data.tl_data_contents as *const libc::c_void,
           tl_data.tl_data_length as libc::c_ulong);
    code = k5_add_pa_data_element(&mut (*reply).enc_padata, &mut pa);
    k5_free_pa_data_element(pa);
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "1627:1"]
pub unsafe extern "C" fn return_enc_padata(mut context: krb5_context,
                                           mut req_pkt: *mut krb5_data,
                                           mut request: *mut krb5_kdc_req,
                                           mut reply_key: *mut krb5_keyblock,
                                           mut server: *mut krb5_db_entry,
                                           mut reply_encpart:
                                               *mut krb5_enc_kdc_rep_part,
                                           mut is_referral: krb5_boolean)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0 as libc::c_int;
    /* This should be initialized and only used for Win2K compat and other
     * specific standardized uses such as FAST negotiation. */
    if is_referral != 0 {
        code = return_referral_enc_padata(context, reply_encpart, server);
        if code != 0 { return code }
    }
    code =
        kdc_handle_protected_negotiation(context, req_pkt, request, reply_key,
                                         &mut (*reply_encpart).enc_padata);
    if !(code != 0) {
        code =
            kdc_add_pa_pac_options(context, request,
                                   &mut (*reply_encpart).enc_padata);
        (code) != 0;
    }
    /*Add potentially other enc_padata providers*/
    return code;
}
