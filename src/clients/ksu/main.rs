use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/sys/types.h:29"]
pub mod sys_types_h {
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::__uid_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:29"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:29"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:29"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:29"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:29"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:29"]
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
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
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
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* KRB5_OLD_CRYPTO */
    /*
 * end "encryption.h"
 */
    /*
 * begin "fieldbits.h"
 */
    /* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
    /* #define      KDC_OPT_UNUSED          0x01000000 */
    /* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
    /* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
    /* #define      KDC_OPT_UNUSED          0x00000004 */
    /*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivitive tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
    /* definitions for ap_options fields */
    /* * @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
    /* *< Use session key */
    /* *< Perform a mutual
                                                 authentication exchange */
    /* *< Generate a subsession key
                                                 from the current session key
                                                 obtained from the
                                                 credentials */
    /* #define      AP_OPTS_RESERVED        0x10000000 */
/* #define      AP_OPTS_RESERVED        0x08000000 */
/* #define      AP_OPTS_RESERVED        0x04000000 */
/* #define      AP_OPTS_RESERVED        0x02000000 */
/* #define      AP_OPTS_RESERVED        0x01000000 */
/* #define      AP_OPTS_RESERVED        0x00800000 */
/* #define      AP_OPTS_RESERVED        0x00400000 */
/* #define      AP_OPTS_RESERVED        0x00200000 */
/* #define      AP_OPTS_RESERVED        0x00100000 */
/* #define      AP_OPTS_RESERVED        0x00080000 */
/* #define      AP_OPTS_RESERVED        0x00040000 */
/* #define      AP_OPTS_RESERVED        0x00020000 */
/* #define      AP_OPTS_RESERVED        0x00010000 */
/* #define      AP_OPTS_RESERVED        0x00008000 */
/* #define      AP_OPTS_RESERVED        0x00004000 */
/* #define      AP_OPTS_RESERVED        0x00002000 */
/* #define      AP_OPTS_RESERVED        0x00001000 */
/* #define      AP_OPTS_RESERVED        0x00000800 */
/* #define      AP_OPTS_RESERVED        0x00000400 */
/* #define      AP_OPTS_RESERVED        0x00000200 */
/* #define      AP_OPTS_RESERVED        0x00000100 */
/* #define      AP_OPTS_RESERVED        0x00000080 */
/* #define      AP_OPTS_RESERVED        0x00000040 */
/* #define      AP_OPTS_RESERVED        0x00000020 */
/* #define      AP_OPTS_RESERVED        0x00000010 */
/* #define      AP_OPTS_RESERVED        0x00000008 */
/* #define      AP_OPTS_RESERVED        0x00000004 */
    /* * @} */
    /* end of AP_OPTS group */
    /* definitions for ad_type fields. */
    /* Ticket flags */
/* flags are 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      TKT_FLG_RESERVED        0x80000000 */
    /* #define      TKT_FLG_RESERVED        0x00004000 */
/* #define      TKT_FLG_RESERVED        0x00002000 */
/* #define      TKT_FLG_RESERVED        0x00001000 */
/* #define      TKT_FLG_RESERVED        0x00000800 */
/* #define      TKT_FLG_RESERVED        0x00000400 */
/* #define      TKT_FLG_RESERVED        0x00000200 */
/* #define      TKT_FLG_RESERVED        0x00000100 */
/* #define      TKT_FLG_RESERVED        0x00000080 */
/* #define      TKT_FLG_RESERVED        0x00000040 */
/* #define      TKT_FLG_RESERVED        0x00000020 */
/* #define      TKT_FLG_RESERVED        0x00000010 */
/* #define      TKT_FLG_RESERVED        0x00000008 */
/* #define      TKT_FLG_RESERVED        0x00000004 */
/* #define      TKT_FLG_RESERVED        0x00000002 */
/* #define      TKT_FLG_RESERVED        0x00000001 */
    /* definitions for lr_type fields. */
    /* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */
    /*
 * end "fieldbits.h"
 */
    /*
 * begin "proto.h"
 */
    /* * Protocol version number */
    /* Message types */
    /* *< Initial authentication request */
    /* *< Response to AS request */
    /* *< Ticket granting server request */
    /* *< Response to TGS request */
    /* *< Auth req to application server */
    /* *< Response to mutual AP request */
    /* *< Safe application message */
    /* *< Private application message */
    /* *< Cred forwarding message */
    /* *< Error response */
    /* LastReq types */
    /* PADATA types */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* Not used */
    /* *< timestamp encrypted in key. RFC 4120 */
    /* *< SecurId passcode. RFC 4120 */
    /* *< Sesame project. RFC 4120 */
    /* *< OSF DCE. RFC 4120 */
    /* *< Cybersafe. RFC 4120 */
    /* *< Cygnus. RFC 4120, 3961 */
    /* *< Etype info for preauth. RFC 4120 */
    /* *< SAM/OTP */
    /* *< SAM/OTP */
    /* *< PKINIT */
    /* *< PKINIT */
    /* *< PKINIT. RFC 4556 */
    /* *< PKINIT. RFC 4556 */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* *< Windows 2000 referrals. RFC 6820 */
    /* *< SAM/OTP. RFC 4120 */
    /* *< Embedded in typed data. RFC 4120 */
    /* *< draft referral system */
    /* *< draft challenge system, updated */
    /* *< draft challenge system, updated */
    /* MS-KILE */
    /* *< include Windows PAC */
    /* *< username protocol transition request */
    /* *< certificate protocol transition request */
    /* *< AS checksum */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6560 section 4.1 */
    /* *< RFC 6560 section 4.2 */
    /* *< RFC 6560 section 4.3 */
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
    #[c2rust::src_loc = "2013:16"]
    pub struct _krb5_creds {
        pub magic: krb5_magic,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub keyblock: krb5_keyblock,
        pub times: krb5_ticket_times,
        pub is_skey: krb5_boolean,
        pub ticket_flags: krb5_flags,
        pub addresses: *mut *mut krb5_address,
        pub ticket: krb5_data,
        pub second_ticket: krb5_data,
        pub authdata: *mut *mut krb5_authdata,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Credentials structure including ticket, session key, and lifetime info. */
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
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
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< client's principal identifier */
        /* *< server's principal identifier */
        /* *< session encryption key info */
        /* *< lifetime info */
        /* *< true if ticket is encrypted in
                                           another ticket's skey */
        /* *< flags in ticket */
        /* *< addrs in ticket */
        /* *< ticket string itself */
        /* *< second ticket, if related to
                                           ticket (via DUPLICATE-SKEY or
                                           ENC-TKT-IN-SKEY) */
        /* *< authorization data */
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
        /* Flags for krb5_cc_retrieve_cred. */
/* * The requested lifetime must be at least as great as the time specified. */
        /* * The is_skey field must match exactly. */
        /* * All the flags set in the match credentials must be set. */
        /* * All the time fields must match exactly. */
        /* * All the flags must match exactly. */
        /* * The authorization data must match. */
        /* * Only the name portion of the principal name must match. */
        /* * The second ticket must match. */
        /* * The encryption key type must match. */
        /* * The supported key types must match. */
        /* Flags for krb5_cc_set_flags and similar. */
/* * Open and close the file for each cache operation. */
        /* *< @deprecated has no effect */
        /* *
 * Retrieve the name, but not type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @warning Returns the name of the credential cache.  The result is an alias
 * into @a cache and should not be freed or modified by the caller.  This name
 * does not include the cache type, so should not be used as input to
 * krb5_cc_resolve().
 *
 * @return
 * On success - the name of the credential cache.
 */
        #[no_mangle]
        #[c2rust::src_loc = "2330:1"]
        pub fn krb5_cc_get_name(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        /* *
 * Retrieve the full name of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] fullname_out    Full name of cache
 *
 * Use krb5_free_string() to free @a fullname_out when it is no longer needed.
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "2344:1"]
        pub fn krb5_cc_get_full_name(context: krb5_context,
                                     cache: krb5_ccache,
                                     fullname_out: *mut *mut libc::c_char)
         -> krb5_error_code;
        /* KRB5_DEPRECATED */
        /* *
 * Initialize a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] principal        Default principal name
 *
 * Destroy any existing contents of @a cache and initialize it for the default
 * principal @a principal.
 *
 * @retval
 *  0  Success
 * @return
 *  System errors; Permission errors; Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2368:1"]
        pub fn krb5_cc_initialize(context: krb5_context, cache: krb5_ccache,
                                  principal: krb5_principal)
         -> krb5_error_code;
        /* *
 * Destroy a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function destroys any existing contents of @a cache and closes the
 * handle to it.
 *
 * @retval
 * 0  Success
 * @return
 * Permission errors
 */
        #[no_mangle]
        #[c2rust::src_loc = "2386:1"]
        pub fn krb5_cc_destroy(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Close a credential cache handle.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * This function closes a credential cache handle @a cache without affecting
 * the contents of the cache.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        /* *
 * Create a new credential cache of the specified type with a unique name.
 *
 * @param [in]  context         Library context
 * @param [in]  type            Credential cache type name
 * @param [in]  hint            Unused
 * @param [out] id              Credential cache handle
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2697:1"]
        pub fn krb5_cc_new_unique(context: krb5_context,
                                  type_0: *const libc::c_char,
                                  hint: *const libc::c_char,
                                  id: *mut krb5_ccache) -> krb5_error_code;
        /* *
 * Create a krb5 library context using only configuration files.
 *
 * @param [out] context         Library context
 *
 * Create a context structure, using only system configuration files.  All
 * information passed through the environment variables is ignored.
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2941:1"]
        pub fn krb5_init_secure_context(context: *mut krb5_context)
         -> krb5_error_code;
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
        /* librc.spec--see rcache.h */
        /* libcc.spec */
        /* *
 * Resolve a credential cache name.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Credential cache name to be resolved
 * @param [out] cache           Credential cache handle
 *
 * Fills in @a cache with a @a cache handle that corresponds to the name in @a
 * name.  @a name should be of the form @c type:residual, and @a type must be a
 * type known to the library.  If the @a name does not contain a colon,
 * interpret it as a file name.
 *
 * @code
 * Example: krb5_cc_resolve(context, "MEMORY:C_", &cache);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4380:1"]
        pub fn krb5_cc_default_name(context: krb5_context)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "4403:1"]
        pub fn krb5_cc_set_default_name(context: krb5_context,
                                        name: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4527:1"]
        pub fn krb5_cc_support_switch(context: krb5_context,
                                      type_0: *const libc::c_char)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4547:1"]
        pub fn krb5_cc_cache_match(context: krb5_context,
                                   client: krb5_principal,
                                   cache_out: *mut krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4778:1"]
        pub fn krb5_free_string(context: krb5_context,
                                val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6851:1"]
        pub fn krb5_get_init_creds_opt_alloc(context: krb5_context,
                                             opt:
                                                 *mut *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6877:1"]
        pub fn krb5_get_init_creds_opt_set_tkt_life(opt:
                                                        *mut krb5_get_init_creds_opt,
                                                    tkt_life: krb5_deltat);
        #[no_mangle]
        #[c2rust::src_loc = "6887:1"]
        pub fn krb5_get_init_creds_opt_set_renew_life(opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      renew_life:
                                                          krb5_deltat);
        #[no_mangle]
        #[c2rust::src_loc = "6897:1"]
        pub fn krb5_get_init_creds_opt_set_forwardable(opt:
                                                           *mut krb5_get_init_creds_opt,
                                                       forwardable:
                                                           libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "6907:1"]
        pub fn krb5_get_init_creds_opt_set_proxiable(opt:
                                                         *mut krb5_get_init_creds_opt,
                                                     proxiable: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "7097:1"]
        pub fn krb5_get_init_creds_opt_set_out_ccache(context: krb5_context,
                                                      opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      ccache: krb5_ccache)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:29"]
pub mod k5_int_h {
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:29"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:29"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:29"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:29"]
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
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/pwd.h:29"]
pub mod pwd_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct passwd {
        pub pw_name: *mut libc::c_char,
        pub pw_passwd: *mut libc::c_char,
        pub pw_uid: __uid_t,
        pub pw_gid: __gid_t,
        pub pw_gecos: *mut libc::c_char,
        pub pw_dir: *mut libc::c_char,
        pub pw_shell: *mut libc::c_char,
    }
    use super::types_h::{__uid_t, __gid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
        #[no_mangle]
        #[c2rust::src_loc = "110:1"]
        pub fn getpwuid(__uid: __uid_t) -> *mut passwd;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "647:1"]
        pub fn putenv(__string: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "657:1"]
        pub fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:29"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "341:12"]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: ::std::ffi::VaList) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/errno.h:29"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:29"]
pub mod unistd_h {
    use super::types_h::{__pid_t, __uid_t, __gid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "563:1"]
        pub fn execv(__path: *const libc::c_char,
                     __argv: *const *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn geteuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "700:1"]
        pub fn setuid(__uid: __uid_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "710:1"]
        pub fn seteuid(__uid: __uid_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "717:1"]
        pub fn setgid(__gid: __gid_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "838:1"]
        pub fn tcgetpgrp(__fd: libc::c_int) -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "841:1"]
        pub fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: __pid_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "848:1"]
        pub fn getlogin() -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "927:1"]
        pub fn getusershell() -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:29"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:29"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:29"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "252:14"]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/signal.h:29"]
pub mod signal_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/clients/ksu/ksu.h:29"]
pub mod ksu_h {
    use super::stddef_h::size_t;
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, _krb5_ccache, krb5_ccache,
                        krb5_get_init_creds_opt, krb5_principal,
                        krb5_error_code, krb5_principal_data, krb5_boolean,
                        krb5_creds};
    use super::sys_types_h::uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "242:30"]
        pub fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "239:1"]
        pub fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "223:1"]
        pub fn get_best_princ_for_target(_: krb5_context, _: uid_t, _: uid_t,
                                         _: *mut libc::c_char,
                                         _: *mut libc::c_char, _: krb5_ccache,
                                         _: *mut krb5_get_init_creds_opt,
                                         _: *mut libc::c_char,
                                         _: *mut libc::c_char,
                                         _: *mut krb5_principal,
                                         _: *mut libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn init_auth_names(_: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "158:1"]
        pub fn krb5_authorization(_: krb5_context, _: krb5_principal,
                                  _: *const libc::c_char,
                                  _: *mut libc::c_char, _: *mut krb5_boolean,
                                  _: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "144:1"]
        pub fn krb5_ccache_filter(_: krb5_context, _: krb5_ccache,
                                  _: krb5_principal) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "138:1"]
        pub fn ks_ccache_is_initialized(_: krb5_context, _: krb5_ccache)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "135:1"]
        pub fn ks_ccache_name_is_initialized(_: krb5_context,
                                             _: *const libc::c_char)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "126:1"]
        pub fn gen_sym(context: krb5_context, sym: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn krb5_ccache_copy(_: krb5_context, _: krb5_ccache,
                                _: krb5_principal, _: krb5_ccache,
                                _: krb5_boolean, _: krb5_principal,
                                _: *mut krb5_boolean) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn plain_dump_principal(_: krb5_context, _: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn ksu_get_tgt_via_passwd(_: krb5_context, _: krb5_principal,
                                      _: *mut krb5_get_init_creds_opt,
                                      _: *mut krb5_boolean,
                                      _: *mut krb5_creds) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn krb5_auth_check(_: krb5_context, _: krb5_principal,
                               _: *mut libc::c_char,
                               _: *mut krb5_get_init_creds_opt,
                               _: *mut libc::c_char, _: krb5_ccache,
                               _: *mut libc::c_int, _: uid_t) -> krb5_boolean;
    }
}
#[c2rust::header_src = "/usr/include/sys/syslog.h:29"]
pub mod syslog_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "190:1"]
        pub fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "181:1"]
        pub fn openlog(__ident: *const libc::c_char, __option: libc::c_int,
                       __facility: libc::c_int);
    }
}
#[c2rust::header_src = "/usr/include/sys/wait.h:32"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
                       __options: libc::c_int) -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/grp.h:34"]
pub mod grp_h {
    use super::types_h::__gid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "197:1"]
        pub fn initgroups(__user: *const libc::c_char, __group: __gid_t)
         -> libc::c_int;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__uint8_t, __int32_t, __uid_t, __gid_t, __off_t,
                        __off64_t, __pid_t};
pub use self::sys_types_h::uid_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdarg_h::va_list;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_addrtype,
                       krb5_enctype, krb5_authdatatype, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds, krb5_ccache,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, _krb5_ccache, krb5_cc_get_name,
                       krb5_cc_get_full_name, krb5_cc_initialize,
                       krb5_cc_destroy, krb5_cc_close, krb5_cc_new_unique,
                       krb5_init_secure_context, krb5_parse_name,
                       krb5_unparse_name, krb5_build_principal_ext,
                       krb5_cc_resolve, krb5_cc_default_name,
                       krb5_cc_set_default_name, krb5_cc_support_switch,
                       krb5_cc_cache_match, krb5_free_string,
                       krb5_string_to_deltat, krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_set_tkt_life,
                       krb5_get_init_creds_opt_set_renew_life,
                       krb5_get_init_creds_opt_set_forwardable,
                       krb5_get_init_creds_opt_set_proxiable,
                       krb5_get_init_creds_opt_set_out_ccache};
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
pub use self::com_err_h::{errcode_t, com_err};
pub use self::pwd_h::{passwd, getpwnam, getpwuid};
use self::stdlib_h::{calloc, free, exit, putenv, unsetenv};
use self::stdio_h::{stderr, fprintf, printf, vfprintf, snprintf, asprintf,
                    perror};
use self::errno_h::__errno_location;
use self::unistd_h::{execv, getpid, getuid, geteuid, setuid, seteuid, setgid,
                     fork, ttyname, tcgetpgrp, tcsetpgrp, getlogin,
                     getusershell};
use self::getopt_core_h::{optarg, optind, getopt};
use self::libintl_h::dgettext;
use self::string_h::{strlen, strrchr, strchr, strdup, strcmp};
use self::signal_h::kill;
use self::ksu_h::{xstrdup, xcalloc, get_best_princ_for_target,
                  init_auth_names, krb5_authorization, krb5_ccache_filter,
                  ks_ccache_is_initialized, ks_ccache_name_is_initialized,
                  gen_sym, krb5_ccache_copy, plain_dump_principal,
                  ksu_get_tgt_via_passwd, krb5_auth_check};
use self::syslog_h::{syslog, openlog};
use self::wait_h::waitpid;
use self::grp_h::initgroups;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 1994 by the University of Southern California
 *
 * EXPORT OF THIS SOFTWARE from the United States of America may
 *     require a specific license from the United States Government.
 *     It is the responsibility of any person or organization contemplating
 *     export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to copy, modify, and distribute
 *     this software and its documentation in source and binary forms is
 *     hereby granted, provided that any documentation or other materials
 *     related to such distribution or use acknowledge that the software
 *     was developed by the University of Southern California.
 *
 * DISCLAIMER OF WARRANTY.  THIS SOFTWARE IS PROVIDED "AS IS".  The
 *     University of Southern California MAKES NO REPRESENTATIONS OR
 *     WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 *     limitation, the University of Southern California MAKES NO
 *     REPRESENTATIONS OR WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY
 *     PARTICULAR PURPOSE. The University of Southern
 *     California shall not be held liable for any liability nor for any
 *     direct, indirect, or consequential damages with respect to any
 *     claim by the user or distributor of the ksu software.
 *
 * KSU was writen by:  Ari Medvinsky, ari@isi.edu
 */
/* globals */
#[no_mangle]
#[c2rust::src_loc = "37:8"]
pub static mut prog_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "38:5"]
pub static mut auth_debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "39:6"]
pub static mut k5login_path: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
#[c2rust::src_loc = "40:6"]
pub static mut k5users_path: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
#[c2rust::src_loc = "41:8"]
pub static mut gb_err: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "42:5"]
pub static mut quiet: libc::c_int = 0 as libc::c_int;
/* Note -e and -a options are mutually exclusive */
/* insure the proper specification of target user as well as catching
   ill specified arguments to commands */
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"Usage: %s [target user] [-n principal] [-c source cachename] [-k] [-r time] [-p|-P] [-f|-F] [-l lifetime] [-zZ] [-q] [-e command [args... ] ] [-a [args... ] ]\n\x00"
                         as *const u8 as *const libc::c_char), prog_name);
}
/* for Ultrix and friends ... */
/* These are file static so sweep_up can get to them*/
#[c2rust::src_loc = "79:14"]
static mut source_uid: uid_t = 0;
#[c2rust::src_loc = "79:26"]
static mut target_uid: uid_t = 0;
#[c2rust::src_loc = "81:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut hp: libc::c_int = 0 as libc::c_int;
    let mut some_rest_copy: libc::c_int = 0 as libc::c_int;
    let mut all_rest_copy: libc::c_int = 0 as libc::c_int;
    let mut localhostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut options: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
    let mut option: libc::c_int = 0 as libc::c_int;
    let mut statusp: libc::c_int = 0 as libc::c_int;
    let mut retval: krb5_error_code = 0 as libc::c_int;
    let mut client: krb5_principal = 0 as krb5_principal;
    let mut tmp_princ: krb5_principal = 0 as krb5_principal;
    let mut cc_tmp: krb5_ccache = 0 as krb5_ccache;
    let mut cc_target: krb5_ccache = 0 as krb5_ccache;
    let mut ksu_context: krb5_context = 0 as *mut _krb5_context;
    let mut cc_target_tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target_user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut source_user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cc_source: krb5_ccache = 0 as krb5_ccache;
    let mut cc_source_tag: *const libc::c_char = 0 as *const libc::c_char;
    let mut cc_source_tag_tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut exec_cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errflg: libc::c_int = 0 as libc::c_int;
    let mut auth_val: krb5_boolean = 0;
    let mut authorization_val: krb5_boolean =
        0 as libc::c_int as krb5_boolean;
    let mut path_passwd: libc::c_int = 0 as libc::c_int;
    let mut done: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ruid: uid_t = getuid();
    let mut pwd: *mut passwd = 0 as *mut passwd;
    let mut target_pwd: *mut passwd = 0 as *mut passwd;
    let mut shell: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut params: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut keep_target_cache: libc::c_int = 0 as libc::c_int;
    let mut child_pid: libc::c_int = 0;
    let mut child_pgrp: libc::c_int = 0;
    let mut ret_pid: libc::c_int = 0;
    let mut pargc: libc::c_int = 0;
    let mut pargv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut stored: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut cc_reused: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut given_princ: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut zero_password: krb5_boolean = 0;
    let mut restrict_creds: krb5_boolean = 0;
    let mut lifetime: krb5_deltat = 0;
    let mut rlife: krb5_deltat = 0;
    if argc == 0 as libc::c_int { exit(1 as libc::c_int); }
    params =
        xcalloc(2 as libc::c_int as size_t,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as *mut *mut libc::c_char;
    let ref mut fresh0 = *params.offset(1 as libc::c_int as isize);
    *fresh0 = 0 as *mut libc::c_char;
    unsetenv(b"KRB5_CONFIG\x00" as *const u8 as *const libc::c_char);
    retval = krb5_init_secure_context(&mut ksu_context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval = krb5_get_init_creds_opt_alloc(ksu_context, &mut options);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32).is_null()
       {
        let ref mut fresh1 = *argv.offset(0 as libc::c_int as isize);
        *fresh1 =
            strrchr(*argv.offset(0 as libc::c_int as isize),
                    '/' as i32).offset(1 as libc::c_int as isize)
    }
    prog_name = *argv.offset(0 as libc::c_int as isize);
    if strlen(prog_name) > 50 as libc::c_int as libc::c_ulong {
        /* this many chars *after* last / ?? */
        com_err(prog_name, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"program name too long - quitting to avoid triggering system logging bugs\x00"
                             as *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* 4.2 syslog */
    openlog(prog_name, 0x1 as libc::c_int | 0x8 as libc::c_int,
            (4 as libc::c_int) << 3 as libc::c_int);
    /* 4.2 syslog */
    if argc == 1 as libc::c_int ||
           *(*argv.offset(1 as libc::c_int as
                              isize)).offset(0 as libc::c_int as isize) as
               libc::c_int == '-' as i32 {
        target_user =
            xstrdup(b"root\x00" as *const u8 as *const libc::c_char);
        pargc = argc;
        pargv = argv
    } else {
        target_user = xstrdup(*argv.offset(1 as libc::c_int as isize));
        pargc = argc - 1 as libc::c_int;
        pargv =
            calloc((pargc + 1 as libc::c_int) as libc::c_ulong,
                   ::std::mem::size_of::<*mut libc::c_char>() as
                       libc::c_ulong) as *mut *mut libc::c_char;
        if pargv.is_null() {
            com_err(prog_name, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while allocating memory\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
        let ref mut fresh2 = *pargv.offset(pargc as isize);
        *fresh2 = 0 as *mut libc::c_char;
        let ref mut fresh3 = *pargv.offset(0 as libc::c_int as isize);
        *fresh3 = *argv.offset(0 as libc::c_int as isize);
        i = 1 as libc::c_int;
        while i < pargc {
            let ref mut fresh4 = *pargv.offset(i as isize);
            *fresh4 = *argv.offset((i + 1 as libc::c_int) as isize);
            i += 1
        }
    }
    if seteuid(ruid) != 0 {
        com_err(prog_name, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting euid to source user\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    while done == 0 &&
              {
                  option =
                      getopt(pargc, pargv,
                             b"n:c:r:a:zZDfFpPkql:e:\x00" as *const u8 as
                                 *const libc::c_char);
                  (option) != -(1 as libc::c_int)
              } {
        match option {
            114 => {
                if strlen(optarg) >= 14 as libc::c_int as libc::c_ulong {
                    optarg =
                        b"bad-time\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                }
                retval = krb5_string_to_deltat(optarg, &mut rlife);
                if retval != 0 as libc::c_int || rlife == 0 as libc::c_int {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Bad lifetime value (%s hours?)\n\x00"
                                         as *const u8 as *const libc::c_char),
                            optarg);
                    errflg += 1
                }
                krb5_get_init_creds_opt_set_renew_life(options, rlife);
            }
            97 => {
                /* when integrating this remember to pass in pargc, pargv and
               take care of params argument */
                optind -= 1;
                if auth_debug != 0 {
                    printf(b"Before get_params optind=%d\n\x00" as *const u8
                               as *const libc::c_char, optind);
                }
                retval = get_params(&mut optind, pargc, pargv, &mut params);
                if retval != 0 {
                    com_err(prog_name, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"when gathering parameters\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                }
                if auth_debug != 0 {
                    printf(b"After get_params optind=%d\n\x00" as *const u8 as
                               *const libc::c_char, optind);
                }
                done = 1 as libc::c_int
            }
            112 => {
                krb5_get_init_creds_opt_set_proxiable(options,
                                                      1 as libc::c_int);
            }
            80 => {
                krb5_get_init_creds_opt_set_proxiable(options,
                                                      0 as libc::c_int);
            }
            102 => {
                krb5_get_init_creds_opt_set_forwardable(options,
                                                        1 as libc::c_int);
            }
            70 => {
                krb5_get_init_creds_opt_set_forwardable(options,
                                                        0 as libc::c_int);
            }
            107 => { keep_target_cache = 1 as libc::c_int }
            113 => { quiet = 1 as libc::c_int }
            108 => {
                if strlen(optarg) >= 14 as libc::c_int as libc::c_ulong {
                    optarg =
                        b"bad-time\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char
                }
                retval = krb5_string_to_deltat(optarg, &mut lifetime);
                if retval != 0 as libc::c_int || lifetime == 0 as libc::c_int
                   {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Bad lifetime value (%s hours?)\n\x00"
                                         as *const u8 as *const libc::c_char),
                            optarg);
                    errflg += 1
                }
                krb5_get_init_creds_opt_set_tkt_life(options, lifetime);
            }
            110 => {
                retval = krb5_parse_name(ksu_context, optarg, &mut client);
                if retval != 0 {
                    com_err(prog_name, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"when parsing name %s\x00" as *const u8
                                         as *const libc::c_char), optarg);
                    errflg += 1
                }
                given_princ = 1 as libc::c_int as krb5_boolean
            }
            122 => {
                some_rest_copy = 1 as libc::c_int;
                if all_rest_copy != 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"-z option is mutually exclusive with -Z.\n\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    errflg += 1
                }
            }
            90 => {
                all_rest_copy = 1 as libc::c_int;
                if some_rest_copy != 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"-Z option is mutually exclusive with -z.\n\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    errflg += 1
                }
            }
            99 => {
                if cc_source_tag.is_null() {
                    cc_source_tag = xstrdup(optarg);
                    if !strchr(cc_source_tag, ':' as i32).is_null() {
                        cc_source_tag_tmp =
                            strchr(cc_source_tag,
                                   ':' as
                                       i32).offset(1 as libc::c_int as isize);
                        if ks_ccache_name_is_initialized(ksu_context,
                                                         cc_source_tag) == 0 {
                            com_err(prog_name,
                                    *__errno_location() as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while looking for credentials cache %s\x00"
                                                 as *const u8 as
                                                 *const libc::c_char),
                                    cc_source_tag_tmp);
                            exit(1 as libc::c_int);
                        }
                    } else {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"malformed credential cache name %s\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                cc_source_tag);
                        errflg += 1
                    }
                } else {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Only one -c option allowed\n\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                }
            }
            101 => {
                cmd = xstrdup(optarg);
                if auth_debug != 0 {
                    printf(b"Before get_params optind=%d\n\x00" as *const u8
                               as *const libc::c_char, optind);
                }
                retval = get_params(&mut optind, pargc, pargv, &mut params);
                if retval != 0 {
                    com_err(prog_name, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"when gathering parameters\x00" as
                                         *const u8 as *const libc::c_char));
                    errflg += 1
                }
                if auth_debug != 0 {
                    printf(b"After get_params optind=%d\n\x00" as *const u8 as
                               *const libc::c_char, optind);
                }
                done = 1 as libc::c_int;
                if auth_debug != 0 {
                    fprintf(stderr,
                            b"Command to be executed: %s\n\x00" as *const u8
                                as *const libc::c_char, cmd);
                }
            }
            63 | _ => { errflg += 1 }
        }
    }
    if errflg != 0 { usage(); exit(2 as libc::c_int); }
    if optind != pargc { usage(); exit(2 as libc::c_int); }
    if auth_debug != 0 {
        j = 1 as libc::c_int;
        while !(*params.offset(j as isize)).is_null() {
            fprintf(stderr,
                    b"params[%d]= %s\n\x00" as *const u8 as
                        *const libc::c_char, j, *params.offset(j as isize));
            j += 1
        }
    }
    /* **********************************/
    source_user = getlogin(); /*checks for the the login name in /etc/utmp*/
    /* verify that that the user exists and get his passwd structure */
    if source_user.is_null() || { pwd = getpwnam(source_user); pwd.is_null() }
           || (*pwd).pw_uid != ruid {
        pwd = getpwuid(ruid)
    }
    if pwd.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: who are you?\n\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if (*pwd).pw_uid != ruid {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Your uid doesn\'t match your passwd entry?!\n\x00"
                             as *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* Okay, now we have *some* passwd entry that matches the
       current real uid.  */
    /* allocate space and copy the usernamane there */
    source_user = xstrdup((*pwd).pw_name);
    source_uid = (*pwd).pw_uid;
    if strcmp(b".\x00" as *const u8 as *const libc::c_char, target_user) == 0
       {
        target_user = xstrdup(source_user)
    }
    target_pwd = getpwnam(target_user);
    if target_pwd.is_null() {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: unknown login %s\n\x00" as *const u8 as
                             *const libc::c_char), target_user);
        exit(1 as libc::c_int);
    }
    target_uid = (*target_pwd).pw_uid;
    init_auth_names((*target_pwd).pw_dir);
    /* **********************************/
    if cc_source_tag.is_null() {
        cc_source_tag = krb5_cc_default_name(ksu_context);
        cc_source_tag_tmp = strchr(cc_source_tag, ':' as i32);
        if cc_source_tag_tmp.is_null() {
            cc_source_tag_tmp = cc_source_tag
        } else { cc_source_tag_tmp = cc_source_tag_tmp.offset(1) }
    }
    /* get a handle for the cache */
    retval = krb5_cc_resolve(ksu_context, cc_source_tag, &mut cc_source);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting source cache\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval =
        get_best_princ_for_target(ksu_context, source_uid, target_uid,
                                  source_user, target_user, cc_source,
                                  options, cmd, localhostname, &mut client,
                                  &mut hp);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while selecting the best principal\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* We may be running as either source or target, depending on
       what happened; become source.*/
    if geteuid() != source_uid {
        if seteuid(0 as libc::c_int as uid_t) != 0 || seteuid(source_uid) != 0
           {
            com_err(prog_name, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while returning to source uid after finding best principal\x00"
                                 as *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if auth_debug != 0 {
        if hp != 0 {
            fprintf(stderr,
                    b"GET_best_princ_for_target result: NOT AUTHORIZED\n\x00"
                        as *const u8 as *const libc::c_char);
        } else {
            fprintf(stderr,
                    b"GET_best_princ_for_target result-best principal \x00" as
                        *const u8 as *const libc::c_char);
            plain_dump_principal(ksu_context, client);
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    if hp != 0 {
        if !gb_err.is_null() {
            fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                    gb_err);
        }
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"account %s: authorization failed\n\x00" as
                             *const u8 as *const libc::c_char), target_user);
        if !cmd.is_null() {
            syslog(4 as libc::c_int,
                   b"Account %s: authorization for %s for execution of %s failed\x00"
                       as *const u8 as *const libc::c_char, target_user,
                   source_user, cmd);
        } else {
            syslog(4 as libc::c_int,
                   b"Account %s: authorization of %s failed\x00" as *const u8
                       as *const libc::c_char, target_user, source_user);
        }
        exit(1 as libc::c_int);
    }
    if auth_debug != 0 {
        fprintf(stderr,
                b" source cache =  %s\n\x00" as *const u8 as
                    *const libc::c_char, cc_source_tag);
    }
    /*
     * After proper authentication and authorization, populate a cache for the
     * target user.
     */
    /*
     * We read the set of creds we want to copy from the source ccache as the
     * source uid, become root for authentication, and then become the target
     * user to handle authorization and creating the target user's cache.
     */
    /* if root ksu's to a regular user, then
       then only the credentials for that particular user
       should be copied */
    restrict_creds =
        (source_uid == 0 as libc::c_int as libc::c_uint &&
             target_uid != 0 as libc::c_int as libc::c_uint) as libc::c_int as
            krb5_boolean;
    retval =
        krb5_parse_name(ksu_context,
                        b"_ksu/_ksu@_ksu\x00" as *const u8 as
                            *const libc::c_char, &mut tmp_princ);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing temporary name\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_cc_resolve(ksu_context,
                        b"MEMORY:_ksu\x00" as *const u8 as
                            *const libc::c_char, &mut cc_tmp);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating temporary cache\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_ccache_copy(ksu_context, cc_source, tmp_princ, cc_tmp,
                         restrict_creds, client, &mut stored);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while copying cache %s to %s\x00" as *const u8 as
                             *const libc::c_char),
                krb5_cc_get_name(ksu_context, cc_source),
                b"MEMORY:_ksu\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    krb5_cc_close(ksu_context, cc_source);
    krb5_get_init_creds_opt_set_out_ccache(ksu_context, options, cc_tmp);
    /* Become root for authentication*/
    if seteuid(0 as libc::c_int as uid_t) != 0 {
        com_err(prog_name, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while reclaiming root uid\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if source_uid == 0 as libc::c_int as libc::c_uint ||
           target_uid == source_uid {
        if all_rest_copy == 0 && given_princ != 0 && !client.is_null() &&
               stored == 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"WARNING: Your password may be exposed if you enter it here and are logged\n\x00"
                                 as *const u8 as *const libc::c_char));
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"         in remotely using an unsecure (non-encrypted) channel.\n\x00"
                                 as *const u8 as *const libc::c_char));
            if ksu_get_tgt_via_passwd(ksu_context, client, options,
                                      &mut zero_password,
                                      0 as *mut krb5_creds) ==
                   0 as libc::c_int as libc::c_uint {
                if zero_password == 0 as libc::c_int as libc::c_uint {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Goodbye\n\x00" as *const u8 as
                                         *const libc::c_char));
                    exit(1 as libc::c_int);
                }
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Could not get a tgt for \x00" as *const u8
                                     as *const libc::c_char));
                plain_dump_principal(ksu_context, client);
                fprintf(stderr,
                        b"\n\x00" as *const u8 as *const libc::c_char);
            }
            stored = 1 as libc::c_int as krb5_boolean
        }
        /* GET_TGT_VIA_PASSWD */
    }
    /* if the user is root or same uid then authentication is not neccesary,
       root gets in automatically */
    if source_uid != 0 && source_uid != target_uid {
        let mut client_name: *mut libc::c_char = 0 as *mut libc::c_char;
        auth_val =
            krb5_auth_check(ksu_context, client, localhostname, options,
                            target_user, cc_tmp, &mut path_passwd,
                            target_uid);
        /* if Kerberos authentication failed then exit */
        if auth_val == 0 as libc::c_int as libc::c_uint {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Authentication failed.\n\x00" as *const u8 as
                                 *const libc::c_char));
            syslog(4 as libc::c_int,
                   b"\'%s %s\' authentication failed for %s%s\x00" as
                       *const u8 as *const libc::c_char, prog_name,
                   target_user, source_user, ontty());
            exit(1 as libc::c_int);
        }
        stored = 1 as libc::c_int as krb5_boolean;
        retval =
            krb5_unparse_name(ksu_context, client as krb5_const_principal,
                              &mut client_name);
        if retval != 0 {
            com_err(prog_name, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"When unparsing name\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
        print_status(dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Authenticated %s\n\x00" as *const u8 as
                                  *const libc::c_char), client_name);
        syslog(5 as libc::c_int,
               b"\'%s %s\' authenticated %s for %s%s\x00" as *const u8 as
                   *const libc::c_char, prog_name, target_user, client_name,
               source_user, ontty());
        /* Run authorization as target.*/
        if seteuid(target_uid) != 0 {
            com_err(prog_name, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while switching to target for authorization check\x00"
                                 as *const u8 as
                                 *const libc::c_char)); /*So we have some chance of sweeping up*/
            exit(1 as libc::c_int);
        }
        retval =
            krb5_authorization(ksu_context, client, target_user, cmd,
                               &mut authorization_val, &mut exec_cmd);
        if retval != 0 {
            com_err(prog_name, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while checking authorization\x00" as *const u8
                                 as *const libc::c_char));
            seteuid(0 as libc::c_int as uid_t);
            exit(1 as libc::c_int);
        }
        if seteuid(0 as libc::c_int as uid_t) != 0 {
            com_err(prog_name, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while switching back from target after authorization check\x00"
                                 as *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        if authorization_val == 1 as libc::c_int as libc::c_uint {
            if !cmd.is_null() {
                print_status(dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"Account %s: authorization for %s for execution of\n\x00"
                                          as *const u8 as
                                          *const libc::c_char), target_user,
                             client_name);
                print_status(dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"               %s successful\n\x00" as
                                          *const u8 as *const libc::c_char),
                             exec_cmd);
                syslog(5 as libc::c_int,
                       b"Account %s: authorization for %s for execution of %s successful\x00"
                           as *const u8 as *const libc::c_char, target_user,
                       client_name, exec_cmd);
            } else {
                print_status(dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"Account %s: authorization for %s successful\n\x00"
                                          as *const u8 as
                                          *const libc::c_char), target_user,
                             client_name);
                syslog(5 as libc::c_int,
                       b"Account %s: authorization for %s successful\x00" as
                           *const u8 as *const libc::c_char, target_user,
                       client_name);
            }
        } else {
            if !cmd.is_null() {
                if !exec_cmd.is_null() {
                    /* was used to pass back the error msg */
                    fprintf(stderr,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            exec_cmd);
                    syslog(4 as libc::c_int,
                           b"%s\x00" as *const u8 as *const libc::c_char,
                           exec_cmd);
                }
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Account %s: authorization for %s for execution of %s failed\n\x00"
                                     as *const u8 as *const libc::c_char),
                        target_user, client_name, cmd);
                syslog(4 as libc::c_int,
                       b"Account %s: authorization for %s for execution of %s failed\x00"
                           as *const u8 as *const libc::c_char, target_user,
                       client_name, cmd);
            } else {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Account %s: authorization of %s failed\n\x00"
                                     as *const u8 as *const libc::c_char),
                        target_user, client_name);
                syslog(4 as libc::c_int,
                       b"Account %s: authorization of %s failed\x00" as
                           *const u8 as *const libc::c_char, target_user,
                       client_name);
            }
            exit(1 as libc::c_int);
        }
    }
    if some_rest_copy != 0 {
        retval = krb5_ccache_filter(ksu_context, cc_tmp, client);
        if retval != 0 {
            com_err(prog_name, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while calling cc_filter\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if all_rest_copy != 0 {
        retval = krb5_cc_initialize(ksu_context, cc_tmp, tmp_princ);
        if retval != 0 {
            com_err(prog_name, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while erasing target cache\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
        stored = 0 as libc::c_int as krb5_boolean
    }
    /* get the shell of the user, this will be the shell used by su */
    target_pwd = getpwnam(target_user);
    if !(*target_pwd).pw_shell.is_null() {
        shell = xstrdup((*target_pwd).pw_shell)
    } else {
        shell =
            b"/bin/csh\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
        /* default is cshell */
    }
    /* insist that the target login uses a standard shell (root is omited) */
    if standard_shell((*target_pwd).pw_shell) == 0 && source_uid != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: permission denied (shell).\n\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* HAVE_GETUSERSHELL */
    if (*target_pwd).pw_uid != 0 {
        if set_env_var(b"USER\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char, (*target_pwd).pw_name) != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"ksu: couldn\'t set environment variable USER\n\x00"
                                 as *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    if set_env_var(b"HOME\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char, (*target_pwd).pw_dir) != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: couldn\'t set environment variable HOME\n\x00"
                             as *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if set_env_var(b"SHELL\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char, shell) != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: couldn\'t set environment variable SHELL\n\x00"
                             as *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* set permissions */
    if setgid((*target_pwd).pw_gid) < 0 as libc::c_int {
        perror(b"ksu: setgid\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if initgroups(target_user, (*target_pwd).pw_gid) != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: initgroups failed.\n\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if strcmp(target_user, source_user) == 0 {
        print_status(dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Leaving uid as %s (%ld)\n\x00" as *const u8 as
                                  *const libc::c_char), target_user,
                     (*target_pwd).pw_uid as libc::c_long);
    } else {
        print_status(dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Changing uid to %s (%ld)\n\x00" as *const u8
                                  as *const libc::c_char), target_user,
                     (*target_pwd).pw_uid as libc::c_long);
    }
    /* HAVE_SETLUID */
    if setuid((*target_pwd).pw_uid) < 0 as libc::c_int {
        perror(b"ksu: setuid\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    retval =
        resolve_target_cache(ksu_context, client, &mut cc_target,
                             &mut cc_reused);
    if retval != 0 { exit(1 as libc::c_int); }
    retval =
        krb5_cc_get_full_name(ksu_context, cc_target, &mut cc_target_tag);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting name of target ccache\x00" as
                             *const u8 as *const libc::c_char));
        sweep_up(ksu_context, cc_target);
        exit(1 as libc::c_int);
    }
    if auth_debug != 0 {
        fprintf(stderr,
                b" target cache =  %s\n\x00" as *const u8 as
                    *const libc::c_char, cc_target_tag);
    }
    if cc_reused != 0 { keep_target_cache = 1 as libc::c_int }
    if stored != 0 {
        retval =
            krb5_ccache_copy(ksu_context, cc_tmp, client, cc_target,
                             0 as libc::c_int as krb5_boolean, client,
                             &mut stored);
        if retval != 0 {
            com_err(prog_name, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while copying cache %s to %s\x00" as *const u8
                                 as *const libc::c_char),
                    b"MEMORY:_ksu\x00" as *const u8 as *const libc::c_char,
                    cc_target_tag);
            exit(1 as libc::c_int);
        }
        if ks_ccache_is_initialized(ksu_context, cc_target) == 0 {
            com_err(prog_name, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s does not have correct permissions for %s, %s aborted\x00"
                                 as *const u8 as *const libc::c_char),
                    target_user, cc_target_tag, prog_name);
            exit(1 as libc::c_int);
        }
    }
    krb5_free_string(ksu_context, cc_target_tag);
    /* Set the cc env name to target. */
    retval = set_ccname_env(ksu_context, cc_target);
    if retval != 0 as libc::c_int {
        sweep_up(ksu_context, cc_target);
        exit(1 as libc::c_int);
    }
    if !cmd.is_null() {
        if source_uid == 0 as libc::c_int as libc::c_uint ||
               source_uid == target_uid {
            exec_cmd = cmd
        }
        if exec_cmd.is_null() {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Internal error: command %s did not get resolved\n\x00"
                                 as *const u8 as *const libc::c_char), cmd);
            exit(1 as libc::c_int);
        }
        let ref mut fresh5 = *params.offset(0 as libc::c_int as isize);
        *fresh5 = exec_cmd
    } else {
        let ref mut fresh6 = *params.offset(0 as libc::c_int as isize);
        *fresh6 = shell
    }
    if auth_debug != 0 {
        fprintf(stderr,
                b"program to be execed %s\n\x00" as *const u8 as
                    *const libc::c_char,
                *params.offset(0 as libc::c_int as isize));
    }
    if keep_target_cache != 0 {
        execv(*params.offset(0 as libc::c_int as isize),
              params as *const *mut libc::c_char);
        com_err(prog_name, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to execv %s\x00" as *const u8 as
                             *const libc::c_char),
                *params.offset(0 as libc::c_int as isize));
        sweep_up(ksu_context, cc_target);
        exit(1 as libc::c_int);
    } else {
        statusp = 1 as libc::c_int;
        child_pid = fork();
        match child_pid {
            -1 => {
                com_err(prog_name, *__errno_location() as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while trying to fork.\x00" as *const u8 as
                                     *const libc::c_char));
                sweep_up(ksu_context, cc_target);
                exit(1 as libc::c_int);
            }
            0 => {
                execv(*params.offset(0 as libc::c_int as isize),
                      params as *const *mut libc::c_char);
                com_err(prog_name, *__errno_location() as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while trying to execv %s\x00" as *const u8
                                     as *const libc::c_char),
                        *params.offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
            _ => {
                if auth_debug != 0 {
                    printf(b" The child pid is %ld\n\x00" as *const u8 as
                               *const libc::c_char,
                           child_pid as libc::c_long);
                    printf(b" The parent pid is %ld\n\x00" as *const u8 as
                               *const libc::c_char, getpid() as libc::c_long);
                }
                loop  {
                    ret_pid =
                        waitpid(child_pid, &mut statusp, 2 as libc::c_int);
                    if !(ret_pid != -(1 as libc::c_int)) { break ; }
                    if !(statusp & 0xff as libc::c_int == 0x7f as libc::c_int)
                       {
                        break ;
                    }
                    child_pgrp = tcgetpgrp(1 as libc::c_int);
                    kill(getpid(), 19 as libc::c_int);
                    tcsetpgrp(1 as libc::c_int, child_pgrp);
                    kill(child_pid, 18 as libc::c_int);
                    statusp = 1 as libc::c_int
                }
                if auth_debug != 0 {
                    printf(b"The exit status of the child is %d\n\x00" as
                               *const u8 as *const libc::c_char, statusp);
                }
                if ret_pid == -(1 as libc::c_int) {
                    com_err(prog_name, *__errno_location() as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while calling waitpid\x00" as *const u8
                                         as *const libc::c_char));
                }
                sweep_up(ksu_context, cc_target);
                exit(statusp);
            }
        }
    };
}
/* Set KRB5CCNAME in the environment to point to ccache.  Print an error
 * message on failure. */
#[c2rust::src_loc = "799:1"]
unsafe extern "C" fn set_ccname_env(mut ksu_context: krb5_context,
                                    mut ccache: krb5_ccache)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut ccname: *mut libc::c_char = 0 as *mut libc::c_char;
    retval = krb5_cc_get_full_name(ksu_context, ccache, &mut ccname);
    if retval != 0 {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while reading cache name from ccache\x00" as
                             *const u8 as *const libc::c_char));
        return retval
    }
    if set_env_var(b"KRB5CCNAME\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char, ccname) != 0 {
        retval = *__errno_location();
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"ksu: couldn\'t set environment variable %s\n\x00"
                             as *const u8 as *const libc::c_char),
                b"KRB5CCNAME\x00" as *const u8 as *const libc::c_char);
    }
    krb5_free_string(ksu_context, ccname);
    return retval;
}
/*
 * Get the configured default ccache name.  Unset KRB5CCNAME and force a
 * recomputation so we don't use values for the source user.  Print an error
 * message on failure.
 */
#[c2rust::src_loc = "825:1"]
unsafe extern "C" fn get_configured_defccname(mut context: krb5_context,
                                              mut target_out:
                                                  *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    *target_out = 0 as *mut libc::c_char;
    unsetenv(b"KRB5CCNAME\x00" as *const u8 as *const libc::c_char);
    /* Make sure we don't have a cached value for a different uid. */
    retval = krb5_cc_set_default_name(context, 0 as *const libc::c_char);
    if retval != 0 as libc::c_int {
        com_err(prog_name, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while resetting target ccache name\x00" as
                             *const u8 as *const libc::c_char));
        return retval
    }
    defname = krb5_cc_default_name(context);
    if !defname.is_null() {
        if !strchr(defname, ':' as i32).is_null() {
            target = strdup(defname)
        } else if asprintf(&mut target as *mut *mut libc::c_char,
                           b"FILE:%s\x00" as *const u8 as *const libc::c_char,
                           defname) < 0 as libc::c_int {
            target = 0 as *mut libc::c_char
        }
    }
    if target.is_null() {
        com_err(prog_name, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while determining target ccache name\x00" as
                             *const u8 as *const libc::c_char));
        return 12 as libc::c_int
    }
    *target_out = target;
    return 0 as libc::c_int;
}
/* Determine where the target user's creds should be stored.  Print an error
 * message on failure. */
#[c2rust::src_loc = "862:1"]
unsafe extern "C" fn resolve_target_cache(mut context: krb5_context,
                                          mut princ: krb5_principal,
                                          mut ccache_out: *mut krb5_ccache,
                                          mut ccache_reused:
                                              *mut krb5_boolean)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut switchable: krb5_boolean = 0;
    let mut reused: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ccname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sym: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    *ccache_out = 0 as krb5_ccache;
    *ccache_reused = 0 as libc::c_int as krb5_boolean;
    retval = get_configured_defccname(context, &mut target);
    if retval != 0 as libc::c_int { return retval }
    /* Check if the configured default name uses a switchable type. */
    sep = strchr(target, ':' as i32);
    *sep = '\u{0}' as i32 as libc::c_char;
    switchable = krb5_cc_support_switch(context, target);
    *sep = ':' as i32 as libc::c_char;
    if switchable == 0 {
        loop 
             /* Try to avoid destroying an in-use target ccache by coming up with
         * the name of a cache that doesn't exist yet. */
             {
            free(ccname as *mut libc::c_void);
            retval = gen_sym(context, &mut sym);
            if retval != 0 {
                com_err(prog_name, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while generating part of the target ccache name\x00"
                                     as *const u8 as *const libc::c_char));
                return retval
            }
            if asprintf(&mut ccname as *mut *mut libc::c_char,
                        b"%s.%s\x00" as *const u8 as *const libc::c_char,
                        target, sym) < 0 as libc::c_int {
                retval = 12 as libc::c_int;
                free(sym as *mut libc::c_void);
                com_err(prog_name, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while allocating memory for the target ccache name\x00"
                                     as *const u8 as *const libc::c_char));
                current_block = 13571693658060686192;
                break ;
            } else {
                free(sym as *mut libc::c_void);
                if !(ks_ccache_name_is_initialized(context, ccname) != 0) {
                    current_block = 2668756484064249700;
                    break ;
                }
            }
        }
        match current_block {
            13571693658060686192 => { }
            _ => {
                retval = krb5_cc_resolve(context, ccname, &mut ccache);
                current_block = 11048769245176032998;
            }
        }
    } else {
        /* Look for a cache in the collection that we can reuse. */
        retval = krb5_cc_cache_match(context, princ, &mut ccache);
        if retval == 0 as libc::c_int {
            reused = 1 as libc::c_int as krb5_boolean;
            current_block = 11048769245176032998;
        } else {
            /* There isn't one, so create a new one. */
            *sep = '\u{0}' as i32 as libc::c_char;
            retval =
                krb5_cc_new_unique(context, target, 0 as *const libc::c_char,
                                   &mut ccache);
            *sep = ':' as i32 as libc::c_char;
            if retval != 0 {
                com_err(prog_name, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while creating new target ccache\x00" as
                                     *const u8 as *const libc::c_char));
                current_block = 13571693658060686192;
            } else {
                retval = krb5_cc_initialize(context, ccache, princ);
                if retval != 0 {
                    com_err(prog_name, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while initializing target cache\x00" as
                                         *const u8 as *const libc::c_char));
                    current_block = 13571693658060686192;
                } else { current_block = 11048769245176032998; }
            }
        }
    }
    match current_block {
        11048769245176032998 => {
            *ccache_out = ccache;
            *ccache_reused = reused
        }
        _ => { }
    }
    free(target as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "939:1"]
pub unsafe extern "C" fn standard_shell(mut sh: *mut libc::c_char)
 -> libc::c_int {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        cp = getusershell();
        if cp.is_null() { break ; }
        if strcmp(cp, sh) == 0 { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/* HAVE_GETUSERSHELL */
#[c2rust::src_loc = "953:1"]
unsafe extern "C" fn ontty() -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut buf: [libc::c_char; 4101] = [0; 4101];
    let mut result: libc::c_int = 0;
    buf[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    p = ttyname(2 as libc::c_int);
    if !p.is_null() {
        result =
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 4101]>() as
                         libc::c_ulong,
                     b" on %s\x00" as *const u8 as *const libc::c_char, p);
        if result as libc::c_uint as libc::c_ulong >=
               ::std::mem::size_of::<[libc::c_char; 4101]>() as libc::c_ulong
           {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"terminal name %s too long\n\x00" as *const u8
                                 as *const libc::c_char), p);
            exit(1 as libc::c_int);
        }
    }
    return buf.as_mut_ptr();
}
#[c2rust::src_loc = "971:1"]
unsafe extern "C" fn set_env_var(mut name: *mut libc::c_char,
                                 mut value: *mut libc::c_char)
 -> libc::c_int {
    let mut env_var_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    asprintf(&mut env_var_buf as *mut *mut libc::c_char,
             b"%s=%s\x00" as *const u8 as *const libc::c_char, name, value);
    return putenv(env_var_buf);
}
#[c2rust::src_loc = "982:1"]
unsafe extern "C" fn sweep_up(mut context: krb5_context,
                              mut cc: krb5_ccache) {
    let mut retval: krb5_error_code = 0;
    seteuid(0 as libc::c_int as uid_t);
    if seteuid(target_uid) < 0 as libc::c_int {
        com_err(prog_name, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while changing to target uid for destroying ccache\x00"
                             as *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if ks_ccache_is_initialized(context, cc) != 0 {
        retval = krb5_cc_destroy(context, cc);
        if retval != 0 {
            com_err(prog_name, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while destroying cache\x00" as *const u8 as
                                 *const libc::c_char));
        }
    };
}
/* ****************************************************************
get_params is to be called for the -a option or -e option to
           collect all params passed in for the shell or for
           cmd.  An aray is returned containing all params.
           optindex is incremented accordingly and the first
           element in the returned array is reserved for the
           name of the command to be executed or the name of the
           shell.
*****************************************************************/
#[no_mangle]
#[c2rust::src_loc = "1011:1"]
pub unsafe extern "C" fn get_params(mut optindex: *mut libc::c_int,
                                    mut pargc: libc::c_int,
                                    mut pargv: *mut *mut libc::c_char,
                                    mut params: *mut *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret_params: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut size: libc::c_int = pargc - *optindex + 2 as libc::c_int;
    ret_params =
        calloc(size as libc::c_ulong,
               ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong) as
            *mut *mut libc::c_char;
    if ret_params.is_null() { return 12 as libc::c_int }
    i = *optindex;
    j = 1 as libc::c_int;
    while i < pargc {
        let ref mut fresh7 = *ret_params.offset(j as isize);
        *fresh7 = *pargv.offset(i as isize);
        *optindex = *optindex + 1 as libc::c_int;
        i += 1;
        j += 1
    }
    let ref mut fresh8 =
        *ret_params.offset((size - 1 as libc::c_int) as isize);
    *fresh8 = 0 as *mut libc::c_char;
    *params = ret_params;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1037:1"]
unsafe extern "C" fn print_status(mut fmt: *const libc::c_char,
                                  mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if quiet == 0 {
        ap = args.clone();
        vfprintf(stderr, fmt, ap.as_va_list());
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (c) 1994 by the University of Southern California
 *
 * EXPORT OF THIS SOFTWARE from the United States of America may
 *     require a specific license from the United States Government.
 *     It is the responsibility of any person or organization contemplating
 *     export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to copy, modify, and distribute
 *     this software and its documentation in source and binary forms is
 *     hereby granted, provided that any documentation or other materials
 *     related to such distribution or use acknowledge that the software
 *     was developed by the University of Southern California.
 *
 * DISCLAIMER OF WARRANTY.  THIS SOFTWARE IS PROVIDED "AS IS".  The
 *     University of Southern California MAKES NO REPRESENTATIONS OR
 *     WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 *     limitation, the University of Southern California MAKES NO
 *     REPRESENTATIONS OR WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY
 *     PARTICULAR PURPOSE. The University of Southern
 *     California shall not be held liable for any liability nor for any
 *     direct, indirect, or consequential damages with respect to any
 *     claim by the user or distributor of the ksu software.
 *
 * KSU was writen by:  Ari Medvinsky, ari@isi.edu
 */
/* <stdarg.h> or <varargs.h> is already included by com_err.h.  */
/* 12 hours */
/* this is temp, should use realloc instead,
                        as done in most of the code */
/* globals */
/* **********/
/* krb_auth_su.c */
/* ccache.c */
/* authorization.c */
/* main.c */
/* heuristic.c */
#[no_mangle]
#[c2rust::src_loc = "1048:1"]
pub unsafe extern "C" fn ksu_tgtname(mut context: krb5_context,
                                     mut server: *const krb5_data,
                                     mut client: *const krb5_data,
                                     mut tgtprinc: *mut krb5_principal)
 -> krb5_error_code {
    return krb5_build_principal_ext(context, tgtprinc, (*client).length,
                                    (*client).data, 6 as libc::c_int,
                                    b"krbtgt\x00" as *const u8 as
                                        *const libc::c_char, (*server).length,
                                    (*server).data, 0 as libc::c_int);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
