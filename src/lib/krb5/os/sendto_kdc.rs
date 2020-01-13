use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:56"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:56"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:56"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:56"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:56"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:56"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_iovec.h:56"]
pub mod struct_iovec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct iovec {
        pub iov_base: *mut libc::c_void,
        pub iov_len: size_t,
    }
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/unistd.h:56"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::__socklen_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:56"]
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
    use super::stddef_h::size_t;
    extern "C" {
        /* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
        /* int k5_getpwnam_r(const char *, blah blah) */
        /* POSIX */
        /* no getpwnam_r, or can't figure out #args or return type */
        /* int k5_getpwuid_r(uid_t, blah blah) */
        /* POSIX */
        /* no getpwuid_r, or can't figure out #args or return type */
        /* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
        /* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
        /* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
        /* Do nothing.  */
        /* Provide strlcpy/strlcat interfaces. */
        #[no_mangle]
        #[c2rust::src_loc = "887:1"]
        pub fn krb5int_strlcpy(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:56"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    /* * Error message structure */
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:56"]
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
    #[c2rust::src_loc = "552:1"]
    pub type krb5_kkdcp_message = _krb5_kkdcp_message;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "552:16"]
    pub struct _krb5_kkdcp_message {
        pub kerb_message: krb5_data,
        pub target_domain: krb5_data,
        pub dclocator_hint: krb5_int32,
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
    #[inline]
    #[c2rust::src_loc = "2274:1"]
    pub unsafe extern "C" fn alloc_data(mut data: *mut krb5_data,
                                        mut len: libc::c_uint)
     -> krb5_error_code {
        let mut ptr: *mut libc::c_char =
            calloc(if len > 0 as libc::c_int as libc::c_uint {
                       len
                   } else { 1 as libc::c_int as libc::c_uint } as
                       libc::c_ulong, 1 as libc::c_int as libc::c_ulong) as
                *mut libc::c_char;
        if ptr.is_null() { return 12 as libc::c_int }
        (*data).magic = -(1760647422 as libc::c_long) as krb5_magic;
        (*data).data = ptr;
        (*data).length = len;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2326:1"]
    pub unsafe extern "C" fn k5memdup0(mut in_0: *const libc::c_void,
                                       mut len: size_t,
                                       mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void =
            k5alloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data, krb5_error, krb5_error_code, krb5_context};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_tls_h::k5_tls_vtable_st;
    use super::k5_err_h::errinfo;
    use super::stdlib_h::calloc;
    use super::stddef_h::size_t;
    use super::string_h::memcpy;
    use super::plugin_h::krb5_plugin_initvt_fn;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1202:8"]
        pub type _kdb_log_context;
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
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1535:1"]
        pub fn encode_krb5_kkdcp_message(_: *const krb5_kkdcp_message,
                                         _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "962:1"]
        pub fn k5_free_kkdcp_message(context: krb5_context,
                                     val: *mut krb5_kkdcp_message);
        #[no_mangle]
        #[c2rust::src_loc = "1720:1"]
        pub fn decode_krb5_kkdcp_message(_: *const krb5_data,
                                         _: *mut *mut krb5_kkdcp_message)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1162:1"]
        pub fn k5_plugin_load(context: krb5_context,
                              interface_id: libc::c_int,
                              modname: *const libc::c_char,
                              module: *mut krb5_plugin_initvt_fn)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1186:1"]
        pub fn k5_plugin_register_dyn(context: krb5_context,
                                      interface_id: libc::c_int,
                                      modname: *const libc::c_char,
                                      modsubdir: *const libc::c_char)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:56"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-tls.h:57"]
pub mod k5_tls_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:16"]
    pub struct k5_tls_vtable_st {
        pub setup: k5_tls_setup_fn,
        pub write: k5_tls_write_fn,
        pub read: k5_tls_read_fn,
        pub free_handle: k5_tls_free_handle_fn,
    }
    /* Release a handle.  Do not pass a null pointer. */
    #[c2rust::src_loc = "92:1"]
    pub type k5_tls_free_handle_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle) -> ()>;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-tls.h - internal pluggable interface for TLS */
/*
 * Copyright (C) 2014 by the Massachusetts Institute of Technology.
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
 * This internal pluggable interface allows libkrb5 to load an in-tree module
 * providing TLS support at runtime.  It is currently tailored for the needs of
 * the OpenSSL module as used for HTTP proxy support.  As an internal
 * interface, it can be changed to fit different implementations and consumers
 * without regard for backward compatibility.
 */
    /* An abstract type for localauth module data. */
    #[c2rust::src_loc = "47:1"]
    pub type k5_tls_handle = *mut k5_tls_handle_st;
    /*
 * Read up to data_size bytes of data using TLS.  Return DATA_READ and set
 * *len_out if any data is read.  Return DONE if there is no more data to be
 * read on the connection, WANT_READ or WANT_WRITE if the underlying socket
 * must be readable or writable to continue, and ERROR_TLS if the TLS channel
 * or underlying socket experienced an error.
 *
 * After DATA_READ, there may still be pending buffered data to read.  The
 * caller must call this method again with additional buffer space before
 * selecting for reading on the underlying socket.
 */
    #[c2rust::src_loc = "87:1"]
    pub type k5_tls_read_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle,
                                    _: *mut libc::c_void, _: size_t,
                                    _: *mut size_t) -> k5_tls_status>;
    #[c2rust::src_loc = "49:9"]
    pub type k5_tls_status = libc::c_uint;
    #[c2rust::src_loc = "50:45"]
    pub const ERROR_TLS: k5_tls_status = 4;
    #[c2rust::src_loc = "50:33"]
    pub const WANT_WRITE: k5_tls_status = 3;
    #[c2rust::src_loc = "50:22"]
    pub const WANT_READ: k5_tls_status = 2;
    #[c2rust::src_loc = "50:16"]
    pub const DONE: k5_tls_status = 1;
    #[c2rust::src_loc = "50:5"]
    pub const DATA_READ: k5_tls_status = 0;
    /*
 * Write len bytes of data using TLS.  Return DONE if writing is complete,
 * WANT_READ or WANT_WRITE if the underlying socket must be readable or
 * writable to continue, and ERROR_TLS if the TLS channel or underlying socket
 * experienced an error.  After WANT_READ or WANT_WRITE, the operation will be
 * retried with the same arguments even if some data has already been written.
 * (OpenSSL makes this contract easy to fulfill.  For other implementations we
 * might want to change it.)
 */
    #[c2rust::src_loc = "72:1"]
    pub type k5_tls_write_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: k5_tls_handle,
                                    _: *const libc::c_void, _: size_t)
                   -> k5_tls_status>;
    /*
 * Create a handle for fd, where the server certificate must match servername
 * and be trusted according to anchors.  anchors is a null-terminated list
 * using the DIR:/FILE:/ENV: syntax borrowed from PKINIT.  If anchors is null,
 * use the system default trust anchors.
 */
    #[c2rust::src_loc = "59:1"]
    pub type k5_tls_setup_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int,
                                    _: *const libc::c_char,
                                    _: *mut *mut libc::c_char,
                                    _: *mut k5_tls_handle)
                   -> krb5_error_code>;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "47:16"]
        pub type k5_tls_handle_st;
    }
    /* K5_TLS_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:56"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:56"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn profile_get_values(profile: profile_t,
                                  names: *const *const libc::c_char,
                                  ret_values: *mut *mut *mut libc::c_char)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn profile_free_list(list: *mut *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn profile_get_integer(profile: profile_t,
                                   name: *const libc::c_char,
                                   subname: *const libc::c_char,
                                   subsubname: *const libc::c_char,
                                   def_val: libc::c_int,
                                   ret_default: *mut libc::c_int)
         -> libc::c_long;
    }
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/plugin.h:56"]
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
#[c2rust::header_src = "/usr/include/bits/socket_type.h:56"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:56"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:56"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:8"]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    #[c2rust::src_loc = "200:1"]
    pub type C2RustUnnamed_1 = libc::c_uint;
    #[c2rust::src_loc = "248:5"]
    pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_1 = 1073741824;
    #[c2rust::src_loc = "245:5"]
    pub const MSG_FASTOPEN: C2RustUnnamed_1 = 536870912;
    #[c2rust::src_loc = "243:5"]
    pub const MSG_ZEROCOPY: C2RustUnnamed_1 = 67108864;
    #[c2rust::src_loc = "241:5"]
    pub const MSG_BATCH: C2RustUnnamed_1 = 262144;
    #[c2rust::src_loc = "239:5"]
    pub const MSG_WAITFORONE: C2RustUnnamed_1 = 65536;
    #[c2rust::src_loc = "237:5"]
    pub const MSG_MORE: C2RustUnnamed_1 = 32768;
    #[c2rust::src_loc = "235:5"]
    pub const MSG_NOSIGNAL: C2RustUnnamed_1 = 16384;
    #[c2rust::src_loc = "233:5"]
    pub const MSG_ERRQUEUE: C2RustUnnamed_1 = 8192;
    #[c2rust::src_loc = "231:5"]
    pub const MSG_RST: C2RustUnnamed_1 = 4096;
    #[c2rust::src_loc = "229:5"]
    pub const MSG_CONFIRM: C2RustUnnamed_1 = 2048;
    #[c2rust::src_loc = "227:5"]
    pub const MSG_SYN: C2RustUnnamed_1 = 1024;
    #[c2rust::src_loc = "225:5"]
    pub const MSG_FIN: C2RustUnnamed_1 = 512;
    #[c2rust::src_loc = "223:5"]
    pub const MSG_WAITALL: C2RustUnnamed_1 = 256;
    #[c2rust::src_loc = "221:5"]
    pub const MSG_EOR: C2RustUnnamed_1 = 128;
    #[c2rust::src_loc = "219:5"]
    pub const MSG_DONTWAIT: C2RustUnnamed_1 = 64;
    #[c2rust::src_loc = "217:5"]
    pub const MSG_TRUNC: C2RustUnnamed_1 = 32;
    #[c2rust::src_loc = "215:5"]
    pub const MSG_PROXY: C2RustUnnamed_1 = 16;
    #[c2rust::src_loc = "213:5"]
    pub const MSG_CTRUNC: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "210:5"]
    pub const MSG_TRYHARD: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "206:5"]
    pub const MSG_DONTROUTE: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "204:5"]
    pub const MSG_PEEK: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "202:5"]
    pub const MSG_OOB: C2RustUnnamed_1 = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "257:8"]
    pub struct msghdr {
        pub msg_name: *mut libc::c_void,
        pub msg_namelen: socklen_t,
        pub msg_iov: *mut iovec,
        pub msg_iovlen: size_t,
        pub msg_control: *mut libc::c_void,
        pub msg_controllen: size_t,
        pub msg_flags: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "361:8"]
    pub struct linger {
        pub l_onoff: libc::c_int,
        pub l_linger: libc::c_int,
    }
    use super::sockaddr_h::sa_family_t;
    use super::unistd_h::socklen_t;
    use super::struct_iovec_h::iovec;
    use super::stddef_h::size_t;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:56"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:9"]
    pub union __SOCKADDR_ARG {
        pub __sockaddr__: *mut sockaddr,
        pub __sockaddr_at__: *mut sockaddr_at,
        pub __sockaddr_ax25__: *mut sockaddr_ax25,
        pub __sockaddr_dl__: *mut sockaddr_dl,
        pub __sockaddr_eon__: *mut sockaddr_eon,
        pub __sockaddr_in__: *mut sockaddr_in,
        pub __sockaddr_in6__: *mut sockaddr_in6,
        pub __sockaddr_inarp__: *mut sockaddr_inarp,
        pub __sockaddr_ipx__: *mut sockaddr_ipx,
        pub __sockaddr_iso__: *mut sockaddr_iso,
        pub __sockaddr_ns__: *mut sockaddr_ns,
        pub __sockaddr_un__: *mut sockaddr_un,
        pub __sockaddr_x25__: *mut sockaddr_x25,
    }
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
    use super::socket_h::{sockaddr, msghdr};
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_un;
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
        #[c2rust::src_loc = "130:1"]
        pub fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "138:1"]
        pub fn send(__fd: libc::c_int, __buf: *const libc::c_void,
                    __n: size_t, __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "145:1"]
        pub fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                    __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "173:1"]
        pub fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
                       __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "208:1"]
        pub fn getsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int, __optval: *mut libc::c_void,
                          __optlen: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:56"]
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
        pub __in6_u: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed_2 {
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
#[c2rust::header_src = "/usr/include/netdb.h:56"]
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
#[c2rust::header_src = "/usr/include/sys/time.h:56"]
pub mod time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct timezone {
        pub tz_minuteswest: libc::c_int,
        pub tz_dsttime: libc::c_int,
    }
    #[c2rust::src_loc = "58:1"]
    pub type __timezone_ptr_t = *mut timezone;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/port-sockets.h:56"]
pub mod port_sockets_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* UNIX variants */
    /* For struct sockaddr_in and in_addr */
    /* For inet_ntoa */
    /* For memset */
    /* For MAXHOSTNAMELEN */
    /* For SOCK_*, AF_*, etc */
    /* For struct timeval */
    /* For struct ifconf, for localaddr.c */
    /* For struct iovec, for sg_buf */
    /*
 * Either size_t or int or unsigned int is probably right.  Under
 * SunOS 4, it looks like int is desired, according to the accept man
 * page.
 */
    /* Unix equivalents of Winsock calls */
    #[c2rust::src_loc = "204:1"]
    pub type sg_buf = iovec;
    /* No error (or anything else) */
    /* nothing */
    /* select() arg for a single fd */
    #[inline]
    #[c2rust::src_loc = "223:1"]
    pub unsafe extern "C" fn socket_connect(mut fd: libc::c_int,
                                            mut addr: *const sockaddr,
                                            mut addrlen: socklen_t)
     -> libc::c_int {
        let mut st: libc::c_int = 0;
        st = connect(fd, __CONST_SOCKADDR_ARG{__sockaddr__: addr,}, addrlen);
        if st == -(1 as libc::c_int) { return st }
        return st;
    }
    #[inline]
    #[c2rust::src_loc = "248:1"]
    pub unsafe extern "C" fn socket_sendmsg(mut fd: libc::c_int,
                                            mut iov: *mut sg_buf,
                                            mut iovcnt: libc::c_int)
     -> ssize_t {
        let mut msg: msghdr =
            msghdr{msg_name: 0 as *mut libc::c_void,
                   msg_namelen: 0,
                   msg_iov: 0 as *mut iovec,
                   msg_iovlen: 0,
                   msg_control: 0 as *mut libc::c_void,
                   msg_controllen: 0,
                   msg_flags: 0,};
        let mut flags: libc::c_int = 0 as libc::c_int;
        flags |= MSG_NOSIGNAL as libc::c_int;
        memset(&mut msg as *mut msghdr as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<msghdr>() as libc::c_ulong);
        msg.msg_iov = iov;
        msg.msg_iovlen = iovcnt as size_t;
        return sendmsg(fd, &mut msg, flags);
    }
    use super::struct_iovec_h::iovec;
    use super::socket_h::{sockaddr, msghdr, MSG_NOSIGNAL};
    use super::unistd_h::socklen_t;
    use super::sys_socket_h::{connect, sendmsg};
    use super::sys_types_h::ssize_t;
    use super::stddef_h::size_t;
    use super::string_h::memset;
    /*_PORT_SOCKET_H*/
    /* UNIX or ...?  */
    /* _WIN32 */
    /* Use TMP to avoid compiler warnings and keep things consistent with
 * Windows version. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:56"]
pub mod k5_buf_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-buf.h - k5buf interface declarations */
/*
 * Copyright 2008 Massachusetts Institute of Technology.
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
 * The k5buf module is intended to allow multi-step string construction in a
 * fixed or dynamic buffer without the need to check for a failure at each step
 * (and without aborting on malloc failure).  If an allocation failure occurs
 * or the fixed buffer runs out of room, the buffer will be set to an error
 * state which can be detected with k5_buf_status.  Data in a buffer is
 * terminated with a zero byte so that it can be used as a C string.
 *
 * k5buf structures are usually stack-allocated.  Do not put k5buf structure
 * pointers into public APIs.  It is okay to reference the data and len fields
 * of a buffer (they will be NULL/0 if the buffer is in an error state), but do
 * not change them.
 */
    /* Buffer type values */
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = libc::c_uint;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: size_t,
        pub len: size_t,
    }
    use super::stddef_h::size_t;
    extern "C" {
        /* Initialize a k5buf using an internally allocated dynamic buffer. */
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
        /* Add a C string to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const libc::c_char);
        /* Add a counted series of bytes to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void,
                              len: size_t);
        /* Add sprintf-style formatted data to BUF. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const libc::c_char,
                              _: ...);
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
    }
    /* K5_BUF_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/os/os-proto.h:60"]
pub mod os_proto_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "73:8"]
    pub struct remote_address {
        pub transport: k5_transport,
        pub family: libc::c_int,
        pub len: socklen_t,
        pub saddr: sockaddr_storage,
    }
    #[c2rust::src_loc = "41:9"]
    pub type k5_transport = libc::c_uint;
    #[c2rust::src_loc = "45:5"]
    pub const HTTPS: k5_transport = 3;
    #[c2rust::src_loc = "44:5"]
    pub const UDP: k5_transport = 2;
    #[c2rust::src_loc = "43:5"]
    pub const TCP: k5_transport = 1;
    #[c2rust::src_loc = "42:5"]
    pub const TCP_OR_UDP: k5_transport = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:8"]
    pub struct sendto_callback_info {
        pub pfn_callback: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut libc::c_void,
                                                      _: *mut krb5_data)
                                     -> libc::c_int>,
        pub pfn_cleanup: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: *mut krb5_data)
                                    -> ()>,
        pub data: *mut libc::c_void,
    }
    /* A list of server hostnames/addresses. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:8"]
    pub struct serverlist {
        pub servers: *mut server_entry,
        pub nservers: size_t,
    }
    /* A single server hostname or address. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct server_entry {
        pub hostname: *mut libc::c_char,
        pub port: libc::c_int,
        pub transport: k5_transport,
        pub uri_path: *mut libc::c_char,
        pub family: libc::c_int,
        pub master: libc::c_int,
        pub addrlen: size_t,
        pub addr: sockaddr_storage,
    }
    #[c2rust::src_loc = "48:9"]
    pub type k5_transport_strategy = libc::c_uint;
    #[c2rust::src_loc = "51:5"]
    pub const NO_UDP: k5_transport_strategy = 2;
    #[c2rust::src_loc = "50:5"]
    pub const UDP_LAST: k5_transport_strategy = 1;
    #[c2rust::src_loc = "49:5"]
    pub const UDP_FIRST: k5_transport_strategy = 0;
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr_storage;
    use super::krb5_h::{krb5_data, krb5_context, krb5_boolean,
                        krb5_error_code};
    use super::stddef_h::size_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "95:1"]
        pub fn k5_locate_kdc(context: krb5_context, realm: *const krb5_data,
                             serverlist: *mut serverlist,
                             get_masters: krb5_boolean, no_udp: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn k5_kdc_is_master(context: krb5_context,
                                realm: *const krb5_data,
                                server: *mut server_entry) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn k5_free_serverlist(_: *mut serverlist);
    }
    /* KRB5_LIBOS_INT_PROTO__ */
}
#[c2rust::header_src = "/usr/include/sys/poll.h:63"]
pub mod poll_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:8"]
    pub struct pollfd {
        pub fd: libc::c_int,
        pub events: libc::c_short,
        pub revents: libc::c_short,
    }
    #[c2rust::src_loc = "33:1"]
    pub type nfds_t = libc::c_ulong;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn poll(__fds: *mut pollfd, __nfds: nfds_t,
                    __timeout: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:56"]
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
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:56"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:56"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:56"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:56"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:56"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "329:14"]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/assert.h:56"]
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:56"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:56"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:58"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2002,2003,2004 by the Massachusetts Institute of Technology,
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
        /* Approach overview:

   If a system version is available but buggy, save handles to it (via
   inline functions in a support library), redefine the names to refer
   to library functions, and in those functions, call the system
   versions and fix up the returned data.  Use the native data
   structures and flag values.

   If no system version exists, use gethostby* and fake it.  Define
   the data structures and flag values locally.


   On macOS, getaddrinfo results aren't cached (though
   gethostbyname results are), so we need to build a cache here.  Now
   things are getting really messy.  Because the cache is in use, we
   use getservbyname, and throw away thread safety.  (Not that the
   cache is thread safe, but when we get locking support, that'll be
   dealt with.)  This code needs tearing down and rebuilding, soon.


   Note that recent Windows developers' code has an interesting hack:
   When you include the right header files, with the right set of
   macros indicating system versions, you'll get an inline function
   that looks for getaddrinfo (or whatever) in the system library, and
   calls it if it's there.  If it's not there, it fakes it with
   gethostby* calls.

   We're taking a simpler approach: A system provides these routines or
   it does not.

   Someday, we may want to take into account different versions (say,
   different revs of GNU libc) where some are broken in one way, and
   some work or are broken in another way.  Cross that bridge when we
   come to it.  */
        /* To do, maybe:

   + For AIX 4.3.3, using the RFC 2133 definition: Implement
   AI_NUMERICHOST.  It's not defined in the header file.

   For certain (old?) versions of GNU libc, AI_NUMERICHOST is
   defined but not implemented.

   + Use gethostbyname2, inet_aton and other IPv6 or thread-safe
   functions if available.  But, see
   https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=135182 for one
   gethostbyname2 problem on Linux.  And besides, if a platform is
   supporting IPv6 at all, they really should be doing getaddrinfo
   by now.

   + inet_ntop, inet_pton

   + Conditionally export/import the function definitions, so a
   library can have a single copy instead of multiple.

   + Upgrade host requirements to include working implementations of
   these functions, and throw all this away.  Pleeease?  :-)  */
        /* ! HAVE_GETADDRINFO */
        /* Fudge things on older gai implementations.  */
/* AIX 4.3.3 is based on RFC 2133; no AI_NUMERICHOST.  */
        /* Partial RFC 2553 implementations may not have AI_ADDRCONFIG and
   friends, which RFC 3493 says are now part of the getaddrinfo
   interface, and we'll want to use.  */
        /* Call out to stuff defined in libkrb5support.  */
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn krb5int_getaddrinfo(node: *const libc::c_char,
                                   service: *const libc::c_char,
                                   hints: *const addrinfo,
                                   aip: *mut *mut addrinfo) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn krb5int_freeaddrinfo(ai: *mut addrinfo);
    }
    /* FAI_DEFINED */
}
#[c2rust::header_src = "/usr/include/sys/ioctl.h:72"]
pub mod ioctl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
         -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __int64_t, __time_t, __suseconds_t, __ssize_t,
                        __socklen_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int32_t, int64_t};
pub use self::struct_timeval_h::timeval;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_iovec_h::iovec;
pub use self::unistd_h::{socklen_t, close, read};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be, krb5int_strlcpy};
pub use self::krb5_h::{krb5_int32, krb5_ui_4, krb5_boolean, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, krb5_post_recv_fn,
                       krb5_context, krb5_pre_send_fn, krb5_trace_callback,
                       krb5_trace_info, _krb5_trace_info, krb5_prompt_type,
                       _krb5_error, krb5_error, _profile_t, krb5_free_error,
                       krb5_free_data, krb5_free_data_contents,
                       krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_kkdcp_message,
                         _krb5_kkdcp_message, empty_data, make_data,
                         alloc_data, k5memdup0, k5alloc, k5calloc,
                         plugin_mapping, _kdb_log_context,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, decode_krb5_error,
                         encode_krb5_kkdcp_message, k5_free_kkdcp_message,
                         decode_krb5_kkdcp_message, k5_plugin_load,
                         k5_plugin_register_dyn};
pub use self::k5_err_h::errinfo;
pub use self::k5_tls_h::{k5_tls_vtable_st, k5_tls_free_handle_fn,
                         k5_tls_handle, k5_tls_read_fn, k5_tls_status,
                         ERROR_TLS, WANT_WRITE, WANT_READ, DONE, DATA_READ,
                         k5_tls_write_fn, k5_tls_setup_fn, k5_tls_handle_st};
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::{profile_t, profile_get_values, profile_free_list,
                          profile_get_integer};
pub use self::plugin_h::{krb5_plugin_vtable, krb5_plugin_initvt_fn,
                         krb5_plugin_vtable_st};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage, C2RustUnnamed_1,
                         MSG_CMSG_CLOEXEC, MSG_FASTOPEN, MSG_ZEROCOPY,
                         MSG_BATCH, MSG_WAITFORONE, MSG_MORE, MSG_NOSIGNAL,
                         MSG_ERRQUEUE, MSG_RST, MSG_CONFIRM, MSG_SYN, MSG_FIN,
                         MSG_WAITALL, MSG_EOR, MSG_DONTWAIT, MSG_TRUNC,
                         MSG_PROXY, MSG_CTRUNC, MSG_TRYHARD, MSG_DONTROUTE,
                         MSG_PEEK, MSG_OOB, msghdr, linger};
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, connect, getpeername, send,
                             recv, sendmsg, getsockopt, setsockopt};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_2, in_port_t,
                     sockaddr_in, in_addr, in_addr_t};
pub use self::netdb_h::addrinfo;
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::port_sockets_h::{sg_buf, socket_connect, socket_sendmsg};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_add, k5_buf_add_len, k5_buf_add_fmt,
                         k5_buf_status};
pub use self::os_proto_h::{remote_address, k5_transport, HTTPS, UDP, TCP,
                           TCP_OR_UDP, sendto_callback_info, serverlist,
                           server_entry, k5_transport_strategy, NO_UDP,
                           UDP_LAST, UDP_FIRST, k5_locate_kdc,
                           k5_kdc_is_master, k5_free_serverlist};
pub use self::poll_h::{pollfd, nfds_t, poll};
use self::stdlib_h::{malloc, calloc, realloc, free, abort};
use self::stdio_h::snprintf;
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::libintl_h::dgettext;
use self::string_h::{strstr, memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::__bswap_32;
use self::k5_trace_h::krb5int_trace;
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_freeaddrinfo};
use self::ioctl_h::ioctl;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "125:8"]
pub struct conn_state {
    pub fd: libc::c_int,
    pub state: conn_states,
    pub service_connect: Option<fd_handler_fn>,
    pub service_write: Option<fd_handler_fn>,
    pub service_read: Option<fd_handler_fn>,
    pub addr: remote_address,
    pub in_0: incoming_message,
    pub out: outgoing_message,
    pub callback_buffer: krb5_data,
    pub server_index: size_t,
    pub next: *mut conn_state,
    pub endtime: time_ms,
    pub defer: krb5_boolean,
    pub http: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "139:5"]
pub struct C2RustUnnamed_3 {
    pub uri_path: *const libc::c_char,
    pub servername: *const libc::c_char,
    pub port: [libc::c_char; 6],
    pub https_request: *mut libc::c_char,
    pub tls: k5_tls_handle,
}
#[c2rust::src_loc = "88:1"]
pub type time_ms = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "112:8"]
pub struct outgoing_message {
    pub sgbuf: [sg_buf; 2],
    pub sgp: *mut sg_buf,
    pub sg_count: libc::c_int,
    pub msg_len_buf: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "103:8"]
pub struct incoming_message {
    pub bufsizebytes_read: size_t,
    pub bufsize: size_t,
    pub pos: size_t,
    pub buf: *mut libc::c_char,
    pub bufsizebytes: [libc::c_uchar; 4],
    pub n_left: size_t,
}
#[c2rust::src_loc = "120:1"]
pub type fd_handler_fn
    =
    unsafe extern "C" fn(_: krb5_context, _: *const krb5_data,
                         _: *mut conn_state, _: *mut select_state)
        -> krb5_boolean;
/* This can be pretty large, so should not be stack-allocated. */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "91:8"]
pub struct select_state {
    pub fds: [pollfd; 1024],
    pub nfds: libc::c_int,
}
/* connection states */
#[c2rust::src_loc = "102:1"]
pub type conn_states = libc::c_uint;
#[c2rust::src_loc = "102:64"]
pub const FAILED: conn_states = 4;
#[c2rust::src_loc = "102:55"]
pub const READING: conn_states = 3;
#[c2rust::src_loc = "102:46"]
pub const WRITING: conn_states = 2;
#[c2rust::src_loc = "102:34"]
pub const CONNECTING: conn_states = 1;
#[c2rust::src_loc = "102:20"]
pub const INITIALIZING: conn_states = 0;
/* Set up context->tls.  On allocation failure, return ENOMEM.  On plugin load
 * failure, set context->tls to point to a nulled vtable and return 0. */
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn init_tls_vtable(mut context: krb5_context)
 -> krb5_error_code {
    let mut initfn: krb5_plugin_initvt_fn = None;
    let mut ret: krb5_error_code = 0;
    if !(*context).tls.is_null() { return 0 as libc::c_int }
    (*context).tls =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<k5_tls_vtable_st>() as libc::c_ulong) as
            *mut k5_tls_vtable_st;
    if (*context).tls.is_null() { return 12 as libc::c_int }
    /* Attempt to load the module; just let it stay nulled out on failure. */
    k5_plugin_register_dyn(context, 8 as libc::c_int,
                           b"k5tls\x00" as *const u8 as *const libc::c_char,
                           b"tls\x00" as *const u8 as *const libc::c_char);
    ret =
        k5_plugin_load(context, 8 as libc::c_int,
                       b"k5tls\x00" as *const u8 as *const libc::c_char,
                       &mut initfn);
    if ret == 0 {
        Some(initfn.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int,
                                                                                             (*context).tls
                                                                                                 as
                                                                                                 krb5_plugin_vtable);
    } else if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Error loading k5tls module: {kerr}\x00" as *const u8
                          as *const libc::c_char, ret);
    }
    return 0 as libc::c_int;
}
/* Get current time in milliseconds. */
#[c2rust::src_loc = "175:1"]
unsafe extern "C" fn get_curtime_ms(mut time_out: *mut time_ms)
 -> krb5_error_code {
    let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    *time_out = 0 as libc::c_int as time_ms;
    if gettimeofday(&mut tv, 0 as *mut timezone) != 0 {
        return *__errno_location()
    }
    *time_out =
        tv.tv_sec * 1000 as libc::c_int as libc::c_long +
            tv.tv_usec / 1000 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "188:1"]
unsafe extern "C" fn free_http_tls_data(mut context: krb5_context,
                                        mut state: *mut conn_state) {
    if !(*state).http.tls.is_null() {
        (*(*context).tls).free_handle.expect("non-null function pointer")(context,
                                                                          (*state).http.tls);
    }
    (*state).http.tls = 0 as k5_tls_handle;
    free((*state).http.https_request as *mut libc::c_void);
    (*state).http.https_request = 0 as *mut libc::c_char;
}
/* Find a pollfd in selstate by fd, or abort if we can't find it. */
#[inline]
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn find_pollfd(mut selstate: *mut select_state,
                                 mut fd: libc::c_int) -> *mut pollfd {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*selstate).nfds {
        if (*selstate).fds[i as usize].fd == fd {
            return &mut *(*selstate).fds.as_mut_ptr().offset(i as isize) as
                       *mut pollfd
        }
        i += 1
    }
    abort();
}
#[c2rust::src_loc = "213:1"]
unsafe extern "C" fn cm_init_selstate(mut selstate: *mut select_state) {
    (*selstate).nfds = 0 as libc::c_int;
}
#[c2rust::src_loc = "219:1"]
unsafe extern "C" fn cm_add_fd(mut selstate: *mut select_state,
                               mut fd: libc::c_int) -> krb5_boolean {
    if (*selstate).nfds >= 1024 as libc::c_int {
        return 0 as libc::c_int as krb5_boolean
    }
    (*selstate).fds[(*selstate).nfds as usize].fd = fd;
    (*selstate).fds[(*selstate).nfds as usize].events =
        0 as libc::c_int as libc::c_short;
    (*selstate).nfds += 1;
    return 1 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn cm_remove_fd(mut selstate: *mut select_state,
                                  mut fd: libc::c_int) {
    let mut pfd: *mut pollfd = find_pollfd(selstate, fd);
    *pfd = (*selstate).fds[((*selstate).nfds - 1 as libc::c_int) as usize];
    (*selstate).nfds -= 1;
}
/* Poll for reading (and not writing) on fd the next time we poll. */
#[c2rust::src_loc = "240:1"]
unsafe extern "C" fn cm_read(mut selstate: *mut select_state,
                             mut fd: libc::c_int) {
    (*find_pollfd(selstate, fd)).events = 0x1 as libc::c_int as libc::c_short;
}
/* Poll for writing (and not reading) on fd the next time we poll. */
#[c2rust::src_loc = "247:1"]
unsafe extern "C" fn cm_write(mut selstate: *mut select_state,
                              mut fd: libc::c_int) {
    (*find_pollfd(selstate, fd)).events = 0x4 as libc::c_int as libc::c_short;
}
/* Get the output events for fd in the form of ssflags. */
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn cm_get_ssflags(mut selstate: *mut select_state,
                                    mut fd: libc::c_int) -> libc::c_uint {
    let mut pfd: *mut pollfd = find_pollfd(selstate, fd);
    /*
     * macOS sets POLLHUP without POLLOUT on connection error.  Catch this as
     * well as other error events such as POLLNVAL, but only if POLLIN and
     * POLLOUT aren't set, as we can get POLLHUP along with POLLIN with TCP
     * data still to be read.
     */
    if (*pfd).revents as libc::c_int != 0 as libc::c_int &&
           (*pfd).revents as libc::c_int &
               (0x1 as libc::c_int | 0x4 as libc::c_int) == 0 {
        return 0x4 as libc::c_int as libc::c_uint
    }
    return ((if (*pfd).revents as libc::c_int & 0x1 as libc::c_int != 0 {
                 0x1 as libc::c_int
             } else { 0 as libc::c_int }) |
                (if (*pfd).revents as libc::c_int & 0x4 as libc::c_int != 0 {
                     0x2 as libc::c_int
                 } else { 0 as libc::c_int }) |
                (if (*pfd).revents as libc::c_int & 0x8 as libc::c_int != 0 {
                     0x4 as libc::c_int
                 } else { 0 as libc::c_int })) as libc::c_uint;
}
/* not USE_POLL */
/* not USE_POLL */
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn cm_select_or_poll(mut in_0: *const select_state,
                                       mut endtime: time_ms,
                                       mut out: *mut select_state,
                                       mut sret: *mut libc::c_int)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut curtime: time_ms = 0;
    let mut interval: time_ms = 0;
    retval = get_curtime_ms(&mut curtime);
    if retval != 0 as libc::c_int { return retval }
    interval =
        if curtime < endtime {
            (endtime) - curtime
        } else { 0 as libc::c_int as libc::c_long };
    /* We don't need a separate copy of the selstate for poll, but use one for
     * consistency with how we use select. */
    *out = *in_0;
    *sret =
        poll((*out).fds.as_mut_ptr(), (*out).nfds as nfds_t,
             interval as libc::c_int);
    return if *sret < 0 as libc::c_int {
               *__errno_location()
           } else { 0 as libc::c_int };
}
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn socktype_for_transport(mut transport: k5_transport)
 -> libc::c_int {
    match transport as libc::c_uint {
        2 => { return SOCK_DGRAM as libc::c_int }
        1 | 3 => { return SOCK_STREAM as libc::c_int }
        _ => { return 0 as libc::c_int }
    };
}
#[c2rust::src_loc = "386:1"]
unsafe extern "C" fn check_for_svc_unavailable(mut context: krb5_context,
                                               mut reply: *const krb5_data,
                                               mut msg_handler_data:
                                                   *mut libc::c_void)
 -> libc::c_int {
    let mut retval: *mut krb5_error_code =
        msg_handler_data as *mut krb5_error_code;
    *retval = 0 as libc::c_int;
    if !reply.is_null() && (*reply).length != 0 &&
           *(*reply).data.offset(0 as libc::c_int as isize) as libc::c_int &
               !(0x20 as libc::c_int) ==
               30 as libc::c_int | 0x40 as libc::c_int {
        let mut err_reply: *mut krb5_error = 0 as *mut krb5_error;
        if decode_krb5_error(reply, &mut err_reply) == 0 as libc::c_int {
            *retval = (*err_reply).error as krb5_error_code;
            krb5_free_error(context, err_reply);
            /* Returning 0 means continue to next KDC */
            return (*retval != 29 as libc::c_int) as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "410:1"]
pub unsafe extern "C" fn krb5_set_kdc_send_hook(mut context: krb5_context,
                                                mut send_hook:
                                                    krb5_pre_send_fn,
                                                mut data: *mut libc::c_void) {
    (*context).kdc_send_hook = send_hook;
    (*context).kdc_send_hook_data = data;
}
/* *
 * Set a KDC pre-send hook function.
 *
 * @param [in] context          Library context
 * @param [in] send_hook        Hook function (or NULL to disable the hook)
 * @param [in] data             Callback data to be passed to @a send_hook
 *
 * @a send_hook will be called before messages are sent to KDCs by library
 * functions such as krb5_get_credentials().  The hook function may inspect,
 * override, or synthesize its own reply to the message.
 *
 * @version New in 1.15
 */
/* *
 * Set a KDC post-receive hook function.
 *
 * @param [in] context          The library context.
 * @param [in] recv_hook        Hook function (or NULL to disable the hook)
 * @param [in] data             Callback data to be passed to @a recv_hook
 *
 * @a recv_hook will be called after a reply is received from a KDC during a
 * call to a library function such as krb5_get_credentials().  The hook
 * function may inspect or override the reply.  This hook will not be executed
 * if the pre-send hook returns a synthetic reply.
 *
 * @version New in 1.15
 */
#[no_mangle]
#[c2rust::src_loc = "418:1"]
pub unsafe extern "C" fn krb5_set_kdc_recv_hook(mut context: krb5_context,
                                                mut recv_hook:
                                                    krb5_post_recv_fn,
                                                mut data: *mut libc::c_void) {
    (*context).kdc_recv_hook = recv_hook;
    (*context).kdc_recv_hook_data = data;
}
/*
 * send the formatted request 'message' to a KDC for realm 'realm' and
 * return the response (if any) in 'reply'.
 *
 * If the message is sent and a response is received, 0 is returned,
 * otherwise an error code is returned.
 *
 * The storage for 'reply' is allocated and should be freed by the caller
 * when finished.
 */
#[no_mangle]
#[c2rust::src_loc = "437:1"]
pub unsafe extern "C" fn krb5_sendto_kdc(mut context: krb5_context,
                                         mut message: *const krb5_data,
                                         mut realm: *const krb5_data,
                                         mut reply_out: *mut krb5_data,
                                         mut use_master: *mut libc::c_int,
                                         mut no_udp: libc::c_int)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut oldret: krb5_error_code = 0;
    let mut err: krb5_error_code = 0;
    let mut servers: serverlist =
        serverlist{servers: 0 as *mut server_entry, nservers: 0,};
    let mut server_used: libc::c_int = 0;
    let mut strategy: k5_transport_strategy = UDP_FIRST;
    let mut reply: krb5_data = empty_data();
    let mut hook_message: *mut krb5_data = 0 as *mut krb5_data;
    let mut hook_reply: *mut krb5_data = 0 as *mut krb5_data;
    *reply_out = empty_data();
    /*
     * find KDC location(s) for realm
     */
    /*
     * BUG: This code won't return "interesting" errors (e.g., out of mem,
     * bad config file) from locate_kdc.  KRB5_REALM_CANT_RESOLVE can be
     * ignored from one query of two, but if only one query is done, or
     * both return that error, it should be returned to the caller.  Also,
     * "interesting" errors (not KRB5_KDC_UNREACH) from sendto_{udp,tcp}
     * should probably be returned as well.
     */
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Sending request ({int} bytes) to {data}{str}{str}\x00"
                          as *const u8 as *const libc::c_char,
                      (*message).length, realm,
                      if *use_master != 0 {
                          b" (master)\x00" as *const u8 as *const libc::c_char
                      } else { b"\x00" as *const u8 as *const libc::c_char },
                      if no_udp != 0 {
                          b" (tcp only)\x00" as *const u8 as
                              *const libc::c_char
                      } else { b"\x00" as *const u8 as *const libc::c_char });
    }
    if no_udp == 0 && (*context).udp_pref_limit < 0 as libc::c_int {
        let mut tmp: libc::c_int = 0;
        retval =
            profile_get_integer((*context).profile,
                                b"libdefaults\x00" as *const u8 as
                                    *const libc::c_char,
                                b"udp_preference_limit\x00" as *const u8 as
                                    *const libc::c_char,
                                0 as *const libc::c_char, 1465 as libc::c_int,
                                &mut tmp) as krb5_error_code;
        if retval != 0 { return retval }
        if tmp < 0 as libc::c_int {
            tmp = 1465 as libc::c_int
        } else if tmp > 32700 as libc::c_int {
            /* In the unlikely case that a *really* big value is
               given, let 'em use as big as we think we can
               support.  */
            tmp = 32700 as libc::c_int
        }
        (*context).udp_pref_limit = tmp
    }
    if no_udp != 0 {
        strategy = NO_UDP
    } else if (*message).length <= (*context).udp_pref_limit as libc::c_uint {
        strategy = UDP_FIRST
    } else { strategy = UDP_LAST }
    retval =
        k5_locate_kdc(context, realm, &mut servers,
                      *use_master as krb5_boolean, no_udp as krb5_boolean);
    if retval != 0 { return retval }
    if (*context).kdc_send_hook.is_some() {
        retval =
            (*context).kdc_send_hook.expect("non-null function pointer")(context,
                                                                         (*context).kdc_send_hook_data,
                                                                         realm,
                                                                         message,
                                                                         &mut hook_message,
                                                                         &mut hook_reply);
        if retval != 0 {
            current_block = 17013359281981807571;
        } else if !hook_reply.is_null() {
            *reply_out = *hook_reply;
            free(hook_reply as *mut libc::c_void);
            current_block = 17013359281981807571;
        } else {
            if !hook_message.is_null() { message = hook_message }
            current_block = 18377268871191777778;
        }
    } else { current_block = 18377268871191777778; }
    match current_block {
        18377268871191777778 => {
            err = 0 as libc::c_int;
            retval =
                k5_sendto(context, message, realm, &mut servers, strategy,
                          0 as *mut sendto_callback_info, &mut reply,
                          0 as *mut sockaddr, 0 as *mut socklen_t,
                          &mut server_used,
                          Some(check_for_svc_unavailable as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _: *const krb5_data,
                                                        _: *mut libc::c_void)
                                       -> libc::c_int),
                          &mut err as *mut krb5_error_code as
                              *mut libc::c_void);
            if retval as libc::c_long == -(1765328228 as libc::c_long) {
                if err == 29 as libc::c_int {
                    retval = -(1765328355 as libc::c_long) as krb5_error_code
                } else {
                    krb5_set_error_message(context, retval,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"Cannot contact any KDC for realm \'%.*s\'\x00"
                                                        as *const u8 as
                                                        *const libc::c_char),
                                           (*realm).length, (*realm).data);
                }
            }
            if (*context).kdc_recv_hook.is_some() {
                oldret = retval;
                retval =
                    (*context).kdc_recv_hook.expect("non-null function pointer")(context,
                                                                                 (*context).kdc_recv_hook_data,
                                                                                 retval,
                                                                                 realm,
                                                                                 message,
                                                                                 &mut reply,
                                                                                 &mut hook_reply);
                if oldret != 0 && retval == 0 {
                    /* The hook must set a reply if it overrides an error from
             * k5_sendto().  Treat this reply as coming from the master KDC. */
                    if !hook_reply.is_null() {
                    } else {
                        __assert_fail(b"hook_reply != NULL\x00" as *const u8
                                          as *const libc::c_char,
                                      b"sendto_kdc.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      532 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 109],
                                                                &[libc::c_char; 109]>(b"krb5_error_code krb5_sendto_kdc(krb5_context, const krb5_data *, const krb5_data *, krb5_data *, int *, int)\x00")).as_ptr());
                    }
                    *use_master = 1 as libc::c_int
                }
            }
            if !(retval != 0) {
                if !hook_reply.is_null() {
                    *reply_out = *hook_reply;
                    free(hook_reply as *mut libc::c_void);
                } else { *reply_out = reply; reply = empty_data() }
                /* Set use_master to 1 if we ended up talking to a master when we didn't
     * explicitly request to. */
                if *use_master == 0 as libc::c_int {
                    *use_master =
                        k5_kdc_is_master(context, realm,
                                         &mut *servers.servers.offset(server_used
                                                                          as
                                                                          isize))
                            as libc::c_int;
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"Response was{str} from master KDC\x00"
                                          as *const u8 as *const libc::c_char,
                                      if *use_master != 0 {
                                          b"\x00" as *const u8 as
                                              *const libc::c_char
                                      } else {
                                          b" not\x00" as *const u8 as
                                              *const libc::c_char
                                      });
                    }
                }
            }
        }
        _ => { }
    }
    krb5_free_data(context, hook_message);
    krb5_free_data_contents(context, &mut reply);
    k5_free_serverlist(&mut servers);
    return retval;
}
#[c2rust::src_loc = "589:1"]
unsafe extern "C" fn make_proxy_request(mut state: *mut conn_state,
                                        mut realm: *const krb5_data,
                                        mut message: *const krb5_data,
                                        mut req_out: *mut *mut libc::c_char,
                                        mut len_out: *mut size_t)
 -> krb5_error_code {
    let mut pm: krb5_kkdcp_message =
        krb5_kkdcp_message{kerb_message:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           target_domain:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           dclocator_hint: 0,};
    let mut encoded_pm: *mut krb5_data = 0 as *mut krb5_data;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut uri_path: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: krb5_error_code = 0;
    *req_out = 0 as *mut libc::c_char;
    *len_out = 0 as libc::c_int as size_t;
    /*
     * Stuff the message length in at the front of the kerb_message field
     * before encoding.  The proxied messages are actually the payload we'd
     * be sending and receiving if we were using plain TCP.
     */
    memset(&mut pm as *mut krb5_kkdcp_message as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_kkdcp_message>() as libc::c_ulong);
    ret =
        alloc_data(&mut pm.kerb_message,
                   (*message).length.wrapping_add(4 as libc::c_int as
                                                      libc::c_uint));
    if !(ret != 0 as libc::c_int) {
        store_32_be((*message).length,
                    pm.kerb_message.data as *mut libc::c_void);
        memcpy(pm.kerb_message.data.offset(4 as libc::c_int as isize) as
                   *mut libc::c_void, (*message).data as *const libc::c_void,
               (*message).length as libc::c_ulong);
        pm.target_domain = *realm;
        ret = encode_krb5_kkdcp_message(&mut pm, &mut encoded_pm);
        if !(ret != 0 as libc::c_int) {
            /* Build the request to transmit: the headers + the proxy message. */
            k5_buf_init_dynamic(&mut buf);
            uri_path =
                if !(*state).http.uri_path.is_null() {
                    (*state).http.uri_path
                } else { b"\x00" as *const u8 as *const libc::c_char };
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"POST /%s HTTP/1.0\r\n\x00" as *const u8 as
                               *const libc::c_char, uri_path);
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"Host: %s:%s\r\n\x00" as *const u8 as
                               *const libc::c_char, (*state).http.servername,
                           (*state).http.port.as_mut_ptr());
            k5_buf_add(&mut buf,
                       b"Cache-Control: no-cache\r\n\x00" as *const u8 as
                           *const libc::c_char);
            k5_buf_add(&mut buf,
                       b"Pragma: no-cache\r\n\x00" as *const u8 as
                           *const libc::c_char);
            k5_buf_add(&mut buf,
                       b"User-Agent: kerberos/1.0\r\n\x00" as *const u8 as
                           *const libc::c_char);
            k5_buf_add(&mut buf,
                       b"Content-type: application/kerberos\r\n\x00" as
                           *const u8 as *const libc::c_char);
            k5_buf_add_fmt(&mut buf as *mut k5buf,
                           b"Content-Length: %d\r\n\r\n\x00" as *const u8 as
                               *const libc::c_char, (*encoded_pm).length);
            k5_buf_add_len(&mut buf,
                           (*encoded_pm).data as *const libc::c_void,
                           (*encoded_pm).length as size_t);
            if k5_buf_status(&mut buf) != 0 as libc::c_int {
                ret = 12 as libc::c_int
            } else {
                *req_out = buf.data as *mut libc::c_char;
                *len_out = buf.len
            }
        }
    }
    krb5_free_data_contents(0 as krb5_context, &mut pm.kerb_message);
    krb5_free_data(0 as krb5_context, encoded_pm);
    return ret;
}
/* Set up the actual message we will send across the underlying transport to
 * communicate the payload message, using one or both of state->out.sgbuf. */
#[c2rust::src_loc = "646:1"]
unsafe extern "C" fn set_transport_message(mut state: *mut conn_state,
                                           mut realm: *const krb5_data,
                                           mut message: *const krb5_data)
 -> krb5_error_code {
    let mut out: *mut outgoing_message = &mut (*state).out;
    let mut req: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reqlen: size_t = 0;
    let mut ret: krb5_error_code = 0;
    if message.is_null() ||
           (*message).length == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (*state).addr.transport as libc::c_uint ==
           TCP as libc::c_int as libc::c_uint {
        store_32_be((*message).length,
                    (*out).msg_len_buf.as_mut_ptr() as *mut libc::c_void);
        (*out).sgbuf[0 as libc::c_int as usize].iov_base =
            (*out).msg_len_buf.as_mut_ptr() as *mut libc::c_char as
                *mut libc::c_void;
        (*out).sgbuf[0 as libc::c_int as usize].iov_len =
            4 as libc::c_int as size_t;
        (*out).sgbuf[1 as libc::c_int as usize].iov_base =
            (*message).data as *mut libc::c_void;
        (*out).sgbuf[1 as libc::c_int as usize].iov_len =
            (*message).length as size_t;
        (*out).sg_count = 2 as libc::c_int;
        return 0 as libc::c_int
    } else if (*state).addr.transport as libc::c_uint ==
                  HTTPS as libc::c_int as libc::c_uint {
        ret =
            make_proxy_request(state, realm, message, &mut req, &mut reqlen);
        if ret != 0 as libc::c_int { return ret }
        (*state).out.sgbuf[0 as libc::c_int as usize].iov_base =
            req as *mut libc::c_void;
        (*state).out.sgbuf[0 as libc::c_int as usize].iov_len = reqlen;
        (*state).out.sgbuf[1 as libc::c_int as usize].iov_base =
            0 as *mut libc::c_char as *mut libc::c_void;
        (*state).out.sgbuf[1 as libc::c_int as usize].iov_len =
            0 as libc::c_int as size_t;
        (*state).out.sg_count = 1 as libc::c_int;
        free((*state).http.https_request as *mut libc::c_void);
        (*state).http.https_request = req;
        return 0 as libc::c_int
    } else {
        (*out).sgbuf[0 as libc::c_int as usize].iov_base =
            (*message).data as *mut libc::c_void;
        (*out).sgbuf[0 as libc::c_int as usize].iov_len =
            (*message).length as size_t;
        (*out).sgbuf[1 as libc::c_int as usize].iov_base =
            0 as *mut libc::c_void as *mut libc::c_char as *mut libc::c_void;
        (*out).sgbuf[1 as libc::c_int as usize].iov_len =
            0 as libc::c_int as size_t;
        (*out).sg_count = 1 as libc::c_int;
        return 0 as libc::c_int
    };
}
#[c2rust::src_loc = "682:1"]
unsafe extern "C" fn add_connection(mut conns: *mut *mut conn_state,
                                    mut transport: k5_transport,
                                    mut defer: krb5_boolean,
                                    mut ai: *mut addrinfo,
                                    mut server_index: size_t,
                                    mut realm: *const krb5_data,
                                    mut hostname: *const libc::c_char,
                                    mut port: *const libc::c_char,
                                    mut uri_path: *const libc::c_char,
                                    mut udpbufp: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut state: *mut conn_state = 0 as *mut conn_state;
    let mut tailptr: *mut *mut conn_state = 0 as *mut *mut conn_state;
    state =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<conn_state>() as libc::c_ulong) as
            *mut conn_state;
    if state.is_null() { return 12 as libc::c_int }
    (*state).state = INITIALIZING;
    (*state).out.sgp = (*state).out.sgbuf.as_mut_ptr();
    (*state).addr.transport = transport;
    (*state).addr.family = (*ai).ai_family;
    (*state).addr.len = (*ai).ai_addrlen;
    memcpy(&mut (*state).addr.saddr as *mut sockaddr_storage as
               *mut libc::c_void, (*ai).ai_addr as *const libc::c_void,
           (*ai).ai_addrlen as libc::c_ulong);
    (*state).defer = defer;
    (*state).fd = !(0 as libc::c_int);
    (*state).server_index = server_index;
    (*state).out.sgbuf[1 as libc::c_int as usize].iov_base =
        0 as *mut libc::c_void as *mut libc::c_char as *mut libc::c_void;
    (*state).out.sgbuf[1 as libc::c_int as usize].iov_len =
        0 as libc::c_int as size_t;
    if transport as libc::c_uint == TCP as libc::c_int as libc::c_uint {
        (*state).service_connect = Some(service_tcp_connect as fd_handler_fn);
        (*state).service_write = Some(service_tcp_write as fd_handler_fn);
        (*state).service_read = Some(service_tcp_read as fd_handler_fn)
    } else if transport as libc::c_uint ==
                  HTTPS as libc::c_int as libc::c_uint {
        if !hostname.is_null() && !port.is_null() {
        } else {
            __assert_fail(b"hostname != NULL && port != NULL\x00" as *const u8
                              as *const libc::c_char,
                          b"sendto_kdc.c\x00" as *const u8 as
                              *const libc::c_char,
                          708 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 178],
                                                    &[libc::c_char; 178]>(b"krb5_error_code add_connection(struct conn_state **, k5_transport, krb5_boolean, struct addrinfo *, size_t, const krb5_data *, const char *, const char *, const char *, char **)\x00")).as_ptr());
        }
        (*state).service_connect = Some(service_tcp_connect as fd_handler_fn);
        (*state).service_write = Some(service_https_write as fd_handler_fn);
        (*state).service_read = Some(service_https_read as fd_handler_fn);
        (*state).http.uri_path = uri_path;
        (*state).http.servername = hostname;
        krb5int_strlcpy((*state).http.port.as_mut_ptr(), port,
                        6 as libc::c_int as size_t);
    } else {
        (*state).service_connect = None;
        (*state).service_write = None;
        (*state).service_read = Some(service_udp_read as fd_handler_fn);
        if (*udpbufp).is_null() {
            *udpbufp =
                malloc(65536 as libc::c_int as libc::c_ulong) as
                    *mut libc::c_char;
            if (*udpbufp).is_null() { return 12 as libc::c_int }
        }
        (*state).in_0.buf = *udpbufp;
        (*state).in_0.bufsize = 65536 as libc::c_int as size_t
    }
    /* Chain the new state onto the tail of the list. */
    tailptr = conns;
    while !(*tailptr).is_null() { tailptr = &mut (**tailptr).next }
    *tailptr = state;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "736:1"]
unsafe extern "C" fn translate_ai_error(mut err: libc::c_int) -> libc::c_int {
    match err {
        0 => { return 0 as libc::c_int }
        -1 | -6 | -7 | -8 => {
            /* All of these indicate bad inputs to getaddrinfo.  */
            return 22 as libc::c_int
        }
        -3 => {
            /* Translate to standard errno code.  */
            return 11 as libc::c_int
        }
        -10 => {
            /* Translate to standard errno code.  */
            return 12 as libc::c_int
        }
        -9 | -5 | -2 => {
            /* Name not known or no address data, but no error.  Do
           nothing more.  */
            return 0 as libc::c_int
        }
        -12 => { return 22 as libc::c_int }
        -11 => { /* XXX */
            /* An argument buffer overflowed.  */
            /* System error, obviously.  */
            return *__errno_location()
        }
        _ => {
            /* An error code we haven't handled?  */
            return 22 as libc::c_int
        }
    };
}
/*
 * Resolve the entry in servers with index ind, adding connections to the list
 * *conns.  Connections are added for each of socktype1 and (if not zero)
 * socktype2.  message and udpbufp are used to initialize the connections; see
 * add_connection above.  If no addresses are available for an entry but no
 * internal name resolution failure occurs, return 0 without adding any new
 * connections.
 */
#[c2rust::src_loc = "788:1"]
unsafe extern "C" fn resolve_server(mut context: krb5_context,
                                    mut realm: *const krb5_data,
                                    mut servers: *const serverlist,
                                    mut ind: size_t,
                                    mut strategy: k5_transport_strategy,
                                    mut message: *const krb5_data,
                                    mut udpbufp: *mut *mut libc::c_char,
                                    mut conns: *mut *mut conn_state)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut entry: *mut server_entry =
        &mut *(*servers).servers.offset(ind as isize) as *mut server_entry;
    let mut transport: k5_transport = TCP_OR_UDP;
    let mut addrs: *mut addrinfo = 0 as *mut addrinfo;
    let mut a: *mut addrinfo = 0 as *mut addrinfo;
    let mut hint: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut ai: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut defer: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut err: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    let mut portbuf: [libc::c_char; 6] = [0; 6];
    /* Skip UDP entries if we don't want UDP. */
    if strategy as libc::c_uint == NO_UDP as libc::c_int as libc::c_uint &&
           (*entry).transport as libc::c_uint ==
               UDP as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    transport =
        if strategy as libc::c_uint ==
               UDP_FIRST as libc::c_int as libc::c_uint {
            UDP as libc::c_int
        } else { TCP as libc::c_int } as k5_transport;
    if (*entry).hostname.is_null() {
        /* Added by a module, so transport is either TCP or UDP. */
        ai.ai_socktype = socktype_for_transport((*entry).transport);
        ai.ai_family = (*entry).family;
        ai.ai_addrlen = (*entry).addrlen as socklen_t;
        ai.ai_addr =
            &mut (*entry).addr as *mut sockaddr_storage as *mut sockaddr;
        defer =
            ((*entry).transport as libc::c_uint != transport as libc::c_uint)
                as libc::c_int as krb5_boolean;
        return add_connection(conns, (*entry).transport, defer, &mut ai, ind,
                              realm, 0 as *const libc::c_char,
                              0 as *const libc::c_char, (*entry).uri_path,
                              udpbufp)
    }
    /* If the entry has a specified transport, use it, but possibly defer the
     * addresses we add based on the strategy. */
    if (*entry).transport as libc::c_uint !=
           TCP_OR_UDP as libc::c_int as libc::c_uint {
        transport = (*entry).transport;
        defer =
            ((*entry).transport as libc::c_uint ==
                 TCP as libc::c_int as libc::c_uint &&
                 strategy as libc::c_uint ==
                     UDP_FIRST as libc::c_int as libc::c_uint ||
                 (*entry).transport as libc::c_uint ==
                     UDP as libc::c_int as libc::c_uint &&
                     strategy as libc::c_uint ==
                         UDP_LAST as libc::c_int as libc::c_uint) as
                libc::c_int as krb5_boolean
    }
    memset(&mut hint as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hint.ai_family = (*entry).family;
    hint.ai_socktype = socktype_for_transport(transport);
    hint.ai_flags = 0x20 as libc::c_int;
    hint.ai_flags |= 0x400 as libc::c_int;
    result =
        snprintf(portbuf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                 b"%d\x00" as *const u8 as *const libc::c_char,
                 (*entry).port);
    if result as libc::c_uint as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong {
        return 22 as libc::c_int
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Resolving hostname {str}\x00" as *const u8 as
                          *const libc::c_char, (*entry).hostname);
    }
    err =
        krb5int_getaddrinfo((*entry).hostname, portbuf.as_mut_ptr(),
                            &mut hint, &mut addrs);
    if err != 0 { return translate_ai_error(err) }
    /* Add each address with the specified or preferred transport. */
    retval = 0 as libc::c_int;
    a = addrs;
    while !a.is_null() && retval == 0 as libc::c_int {
        retval =
            add_connection(conns, transport, defer, a, ind, realm,
                           (*entry).hostname, portbuf.as_mut_ptr(),
                           (*entry).uri_path, udpbufp);
        a = (*a).ai_next
    }
    /* For TCP_OR_UDP entries, add each address again with the non-preferred
     * transport, unless we are avoiding UDP.  Flag these as deferred. */
    if retval == 0 as libc::c_int &&
           (*entry).transport as libc::c_uint ==
               TCP_OR_UDP as libc::c_int as libc::c_uint &&
           strategy as libc::c_uint != NO_UDP as libc::c_int as libc::c_uint {
        transport =
            if strategy as libc::c_uint ==
                   UDP_FIRST as libc::c_int as libc::c_uint {
                TCP as libc::c_int
            } else { UDP as libc::c_int } as
                k5_transport; /* try other hosts */
        a = addrs;
        while !a.is_null() && retval == 0 as libc::c_int {
            (*a).ai_socktype = socktype_for_transport(transport);
            retval =
                add_connection(conns, transport,
                               1 as libc::c_int as krb5_boolean, a, ind,
                               realm, (*entry).hostname, portbuf.as_mut_ptr(),
                               (*entry).uri_path, udpbufp);
            a = (*a).ai_next
        }
    }
    krb5int_freeaddrinfo(addrs);
    return retval;
}
#[c2rust::src_loc = "864:1"]
unsafe extern "C" fn start_connection(mut context: krb5_context,
                                      mut state: *mut conn_state,
                                      mut message: *const krb5_data,
                                      mut selstate: *mut select_state,
                                      mut realm: *const krb5_data,
                                      mut callback_info:
                                          *mut sendto_callback_info)
 -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    static mut one: libc::c_int = 1 as libc::c_int;
    static mut lopt: linger =
        {
            let mut init =
                linger{l_onoff: 0 as libc::c_int,
                       l_linger: 0 as libc::c_int,};
            init
        };
    type_0 = socktype_for_transport((*state).addr.transport);
    fd = socket((*state).addr.family, type_0, 0 as libc::c_int);
    if fd == !(0 as libc::c_int) { return -(1 as libc::c_int) }
    fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
    /* Make it non-blocking.  */
    ioctl(fd, 0x5421 as libc::c_int as libc::c_ulong,
          &one as *const libc::c_int as *const libc::c_void);
    if (*state).addr.transport as libc::c_uint ==
           TCP as libc::c_int as libc::c_uint {
        setsockopt(fd, 1 as libc::c_int, 13 as libc::c_int,
                   &lopt as *const linger as *const libc::c_void,
                   ::std::mem::size_of::<linger>() as libc::c_ulong as
                       socklen_t);
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Initiating TCP connection to {raddr}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*state).addr as *mut remote_address);
        }
    }
    /* Start connecting to KDC.  */
    e =
        socket_connect(fd,
                       &mut (*state).addr.saddr as *mut sockaddr_storage as
                           *mut sockaddr, (*state).addr.len);
    if e != 0 as libc::c_int {
        /*
         * This is the path that should be followed for non-blocking
         * connections.
         */
        if *__errno_location() == 115 as libc::c_int ||
               *__errno_location() == 11 as libc::c_int {
            (*state).state = CONNECTING;
            (*state).fd = fd
        } else {
            close(fd);
            (*state).state = FAILED;
            return -(2 as libc::c_int)
        }
    } else {
        /*
         * Connect returned zero even though we made it non-blocking.  This
         * happens normally for UDP sockets, and can perhaps also happen for
         * TCP sockets connecting to localhost.
         */
        (*state).state = WRITING;
        (*state).fd = fd
    }
    /*
     * Here's where KPASSWD callback gets the socket information it needs for
     * a kpasswd request
     */
    if !callback_info.is_null() {
        e =
            (*callback_info).pfn_callback.expect("non-null function pointer")((*state).fd,
                                                                              (*callback_info).data,
                                                                              &mut (*state).callback_buffer);
        if e != 0 as libc::c_int {
            close(fd);
            (*state).fd = !(0 as libc::c_int);
            (*state).state = FAILED;
            return -(3 as libc::c_int)
        }
        message = &mut (*state).callback_buffer
    }
    e = set_transport_message(state, realm, message);
    if e != 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Error preparing message to send to {raddr}: {errno}\x00"
                              as *const u8 as *const libc::c_char,
                          &mut (*state).addr as *mut remote_address, e);
        }
        close((*state).fd);
        (*state).fd = !(0 as libc::c_int);
        (*state).state = FAILED;
        return -(4 as libc::c_int)
    }
    if (*state).addr.transport as libc::c_uint ==
           UDP as libc::c_int as libc::c_uint {
        /* Send it now.  */
        let mut ret: ssize_t = 0;
        let mut sg: *mut sg_buf =
            &mut *(*state).out.sgbuf.as_mut_ptr().offset(0 as libc::c_int as
                                                             isize) as
                *mut sg_buf;
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Sending initial UDP request to {raddr}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*state).addr as *mut remote_address);
        }
        ret =
            send((*state).fd,
                 ((*sg).iov_base as
                      *mut libc::c_char).offset(0 as libc::c_int as isize) as
                     *const libc::c_void,
                 (*sg).iov_len.wrapping_add(0 as libc::c_int as
                                                libc::c_ulong),
                 0 as libc::c_int);
        if ret < 0 as libc::c_int as libc::c_long ||
               ret as size_t !=
                   (*sg).iov_len.wrapping_add(0 as libc::c_int as
                                                  libc::c_ulong) {
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"UDP error sending to {raddr}: {errno}\x00" as
                                  *const u8 as *const libc::c_char,
                              &mut (*state).addr as *mut remote_address,
                              *__errno_location());
            }
            close((*state).fd);
            (*state).fd = !(0 as libc::c_int);
            (*state).state = FAILED;
            return -(5 as libc::c_int)
        } else { (*state).state = READING }
    }
    if cm_add_fd(selstate, (*state).fd) == 0 {
        close((*state).fd);
        (*state).fd = !(0 as libc::c_int);
        (*state).state = FAILED;
        return -(1 as libc::c_int)
    }
    if (*state).state as libc::c_uint ==
           CONNECTING as libc::c_int as libc::c_uint ||
           (*state).state as libc::c_uint ==
               WRITING as libc::c_int as libc::c_uint {
        cm_write(selstate, (*state).fd);
    } else { cm_read(selstate, (*state).fd); }
    return 0 as libc::c_int;
}
/* Return 0 if we sent something, non-0 otherwise.
   If 0 is returned, the caller should delay waiting for a response.
   Otherwise, the caller should immediately move on to process the
   next connection.  */
#[c2rust::src_loc = "976:1"]
unsafe extern "C" fn maybe_send(mut context: krb5_context,
                                mut conn: *mut conn_state,
                                mut message: *const krb5_data,
                                mut selstate: *mut select_state,
                                mut realm: *const krb5_data,
                                mut callback_info: *mut sendto_callback_info)
 -> libc::c_int {
    let mut sg: *mut sg_buf = 0 as *mut sg_buf;
    let mut ret: ssize_t = 0;
    if (*conn).state as libc::c_uint ==
           INITIALIZING as libc::c_int as libc::c_uint {
        return start_connection(context, conn, message, selstate, realm,
                                callback_info)
    }
    /* Did we already shut down this channel?  */
    if (*conn).state as libc::c_uint == FAILED as libc::c_int as libc::c_uint
       {
        return -(1 as libc::c_int)
    }
    if (*conn).addr.transport as libc::c_uint !=
           UDP as libc::c_int as libc::c_uint {
        /* The select callback will handle flushing any data we
           haven't written yet, and we only write it once.  */
        return -(1 as libc::c_int)
    }
    /* UDP - retransmit after a previous attempt timed out. */
    sg =
        &mut *(*conn).out.sgbuf.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut sg_buf;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Sending retry UDP request to {raddr}\x00" as *const u8
                          as *const libc::c_char,
                      &mut (*conn).addr as *mut remote_address);
    }
    ret =
        send((*conn).fd,
             ((*sg).iov_base as
                  *mut libc::c_char).offset(0 as libc::c_int as isize) as
                 *const libc::c_void,
             (*sg).iov_len.wrapping_add(0 as libc::c_int as libc::c_ulong),
             0 as libc::c_int);
    if ret < 0 as libc::c_int as libc::c_long ||
           ret as size_t !=
               (*sg).iov_len.wrapping_add(0 as libc::c_int as libc::c_ulong) {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"UDP error sending to {raddr}: {errno}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address,
                          *__errno_location());
        }
        /* Keep connection alive, we'll try again next pass.

           Is this likely to catch any errors we didn't get from the
           select callbacks?  */
        return -(1 as libc::c_int)
    }
    /* Yay, it worked.  */
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1018:1"]
unsafe extern "C" fn kill_conn(mut context: krb5_context,
                               mut conn: *mut conn_state,
                               mut selstate: *mut select_state) {
    free_http_tls_data(context, conn);
    if socktype_for_transport((*conn).addr.transport) ==
           SOCK_STREAM as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Terminating TCP connection to {raddr}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address);
        }
    }
    cm_remove_fd(selstate, (*conn).fd);
    close((*conn).fd);
    (*conn).fd = !(0 as libc::c_int);
    (*conn).state = FAILED;
}
/* Check socket for error.  */
#[c2rust::src_loc = "1034:1"]
unsafe extern "C" fn get_so_error(mut fd: libc::c_int) -> libc::c_int {
    let mut e: libc::c_int = 0;
    let mut sockerr: libc::c_int = 0;
    let mut sockerrlen: socklen_t = 0;
    sockerr = 0 as libc::c_int;
    sockerrlen =
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    e =
        getsockopt(fd, 1 as libc::c_int, 4 as libc::c_int,
                   &mut sockerr as *mut libc::c_int as *mut libc::c_void,
                   &mut sockerrlen);
    if e != 0 as libc::c_int {
        /* What to do now?  */
        e = *__errno_location();
        return e
    }
    return sockerr;
}
/* Perform next step in sending.  Return true on usable data. */
#[c2rust::src_loc = "1052:1"]
unsafe extern "C" fn service_dispatch(mut context: krb5_context,
                                      mut realm: *const krb5_data,
                                      mut conn: *mut conn_state,
                                      mut selstate: *mut select_state,
                                      mut ssflags: libc::c_int)
 -> krb5_boolean {
    /* Check for a socket exception. */
    if ssflags & 0x4 as libc::c_int != 0 {
        kill_conn(context, conn, selstate);
        return 0 as libc::c_int as krb5_boolean
    }
    match (*conn).state as libc::c_uint {
        1 => {
            if (*conn).service_connect.is_some() {
            } else {
                __assert_fail(b"conn->service_connect != NULL\x00" as
                                  *const u8 as *const libc::c_char,
                              b"sendto_kdc.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1065 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 112],
                                                        &[libc::c_char; 112]>(b"krb5_boolean service_dispatch(krb5_context, const krb5_data *, struct conn_state *, struct select_state *, int)\x00")).as_ptr());
            }
            return (*conn).service_connect.expect("non-null function pointer")(context,
                                                                               realm,
                                                                               conn,
                                                                               selstate)
        }
        2 => {
            if (*conn).service_write.is_some() {
            } else {
                __assert_fail(b"conn->service_write != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"sendto_kdc.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1068 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 112],
                                                        &[libc::c_char; 112]>(b"krb5_boolean service_dispatch(krb5_context, const krb5_data *, struct conn_state *, struct select_state *, int)\x00")).as_ptr());
            }
            return (*conn).service_write.expect("non-null function pointer")(context,
                                                                             realm,
                                                                             conn,
                                                                             selstate)
        }
        3 => {
            if (*conn).service_read.is_some() {
            } else {
                __assert_fail(b"conn->service_read != NULL\x00" as *const u8
                                  as *const libc::c_char,
                              b"sendto_kdc.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1071 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 112],
                                                        &[libc::c_char; 112]>(b"krb5_boolean service_dispatch(krb5_context, const krb5_data *, struct conn_state *, struct select_state *, int)\x00")).as_ptr());
            }
            return (*conn).service_read.expect("non-null function pointer")(context,
                                                                            realm,
                                                                            conn,
                                                                            selstate)
        }
        _ => { abort(); }
    };
}
/*
 * Notes:
 *
 * Getting "connection refused" on a connected UDP socket causes
 * select to indicate write capability on UNIX, but only shows up
 * as an exception on Windows.  (I don't think any UNIX system flags
 * the error as an exception.)  So we check for both, or make it
 * system-specific.
 *
 * Always watch for responses from *any* of the servers.  Eventually
 * fix the UDP code to do the same.
 *
 * To do:
 * - TCP NOPUSH/CORK socket options?
 * - error codes that don't suck
 * - getsockopt(SO_ERROR) to check connect status
 * - handle error RESPONSE_TOO_BIG from UDP server and use TCP
 *   connections already in progress
 */
/* Initialize TCP transport. */
#[c2rust::src_loc = "1079:1"]
unsafe extern "C" fn service_tcp_connect(mut context: krb5_context,
                                         mut realm: *const krb5_data,
                                         mut conn: *mut conn_state,
                                         mut selstate: *mut select_state)
 -> krb5_boolean {
    /* Check whether the connection succeeded. */
    let mut e: libc::c_int = get_so_error((*conn).fd);
    if e != 0 {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TCP error connecting to {raddr}: {errno}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address, e);
        }
        kill_conn(context, conn, selstate);
        return 0 as libc::c_int as krb5_boolean
    }
    (*conn).state = WRITING;
    /* Record this connection's timeout for service_fds. */
    if get_curtime_ms(&mut (*conn).endtime) == 0 as libc::c_int {
        (*conn).endtime += 10000 as libc::c_int as libc::c_long
    }
    return (*conn).service_write.expect("non-null function pointer")(context,
                                                                     realm,
                                                                     conn,
                                                                     selstate);
}
/* Sets conn->state to READING when done. */
#[c2rust::src_loc = "1102:1"]
unsafe extern "C" fn service_tcp_write(mut context: krb5_context,
                                       mut realm: *const krb5_data,
                                       mut conn: *mut conn_state,
                                       mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut nwritten: ssize_t = 0;
    let mut tmp: libc::c_int = 0;
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"Sending TCP request to {raddr}\x00" as *const u8 as
                          *const libc::c_char,
                      &mut (*conn).addr as *mut remote_address);
    }
    tmp =
        socket_sendmsg((*conn).fd, (*conn).out.sgp, (*conn).out.sg_count) as
            libc::c_int;
    nwritten = tmp as ssize_t;
    if nwritten < 0 as libc::c_int as libc::c_long {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"TCP error sending to {raddr}: {errno}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address,
                          *__errno_location());
        }
        kill_conn(context, conn, selstate);
        return 0 as libc::c_int as krb5_boolean
    }
    while nwritten != 0 {
        let mut sgp: *mut sg_buf = (*conn).out.sgp;
        if (nwritten as size_t) <
               (*sgp).iov_len.wrapping_add(0 as libc::c_int as libc::c_ulong)
           {
            if (*sgp).iov_len < nwritten as size_t {
                abort();
            } else {
                (*sgp).iov_base =
                    ((*sgp).iov_base as
                         *mut libc::c_char).offset(nwritten as size_t as
                                                       isize) as
                        *mut libc::c_void;
                (*sgp).iov_len =
                    ((*sgp).iov_len as
                         libc::c_ulong).wrapping_sub(nwritten as size_t) as
                        size_t as size_t
            };
            nwritten = 0 as libc::c_int as ssize_t
        } else {
            nwritten =
                (nwritten as
                     libc::c_ulong).wrapping_sub((*sgp).iov_len.wrapping_add(0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong))
                    as ssize_t as ssize_t;
            (*conn).out.sgp = (*conn).out.sgp.offset(1);
            (*conn).out.sg_count -= 1
        }
    }
    if (*conn).out.sg_count == 0 as libc::c_int {
        /* Done writing, switch to reading. */
        cm_read(selstate, (*conn).fd);
        (*conn).state = READING
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Return true on usable data. */
#[c2rust::src_loc = "1136:1"]
unsafe extern "C" fn service_tcp_read(mut context: krb5_context,
                                      mut realm: *const krb5_data,
                                      mut conn: *mut conn_state,
                                      mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut nread: ssize_t = 0;
    let mut e: libc::c_int = 0 as libc::c_int;
    let mut in_0: *mut incoming_message = &mut (*conn).in_0;
    if (*in_0).bufsizebytes_read == 4 as libc::c_int as libc::c_ulong {
        /* Reading data.  */
        nread =
            read((*conn).fd,
                 &mut *(*in_0).buf.offset((*in_0).pos as isize) as
                     *mut libc::c_char as *mut libc::c_void, (*in_0).n_left);
        if nread <= 0 as libc::c_int as libc::c_long {
            e =
                if nread != 0 {
                    *__errno_location()
                } else { 104 as libc::c_int };
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"TCP error receiving from {raddr}: {errno}\x00"
                                  as *const u8 as *const libc::c_char,
                              &mut (*conn).addr as *mut remote_address, e);
            }
            kill_conn(context, conn, selstate);
            return 0 as libc::c_int as krb5_boolean
        }
        (*in_0).n_left =
            ((*in_0).n_left as
                 libc::c_ulong).wrapping_sub(nread as libc::c_ulong) as size_t
                as size_t;
        (*in_0).pos =
            ((*in_0).pos as
                 libc::c_ulong).wrapping_add(nread as libc::c_ulong) as size_t
                as size_t;
        if (*in_0).n_left <= 0 as libc::c_int as libc::c_ulong {
            return 1 as libc::c_int as krb5_boolean
        }
    } else {
        /* Reading length.  */
        nread =
            read((*conn).fd,
                 (*in_0).bufsizebytes.as_mut_ptr().offset((*in_0).bufsizebytes_read
                                                              as isize) as
                     *mut libc::c_void,
                 (4 as libc::c_int as
                      libc::c_ulong).wrapping_sub((*in_0).bufsizebytes_read));
        if nread <= 0 as libc::c_int as libc::c_long {
            e =
                if nread != 0 {
                    *__errno_location()
                } else { 104 as libc::c_int };
            if (*context).trace_callback.is_some() {
                krb5int_trace(context,
                              b"TCP error receiving from {raddr}: {errno}\x00"
                                  as *const u8 as *const libc::c_char,
                              &mut (*conn).addr as *mut remote_address, e);
            }
            kill_conn(context, conn, selstate);
            return 0 as libc::c_int as krb5_boolean
        }
        (*in_0).bufsizebytes_read =
            ((*in_0).bufsizebytes_read as
                 libc::c_ulong).wrapping_add(nread as libc::c_ulong) as size_t
                as size_t;
        if (*in_0).bufsizebytes_read == 4 as libc::c_int as libc::c_ulong {
            let mut len: libc::c_ulong =
                load_32_be((*in_0).bufsizebytes.as_mut_ptr() as
                               *const libc::c_void) as libc::c_ulong;
            /* Arbitrary 1M cap.  */
            if len >
                   (1 as libc::c_int * 1024 as libc::c_int *
                        1024 as libc::c_int) as libc::c_ulong {
                kill_conn(context, conn, selstate);
                return 0 as libc::c_int as krb5_boolean
            }
            (*in_0).n_left = len;
            (*in_0).bufsize = (*in_0).n_left;
            (*in_0).pos = 0 as libc::c_int as size_t;
            (*in_0).buf = malloc(len) as *mut libc::c_char;
            if (*in_0).buf.is_null() {
                kill_conn(context, conn, selstate);
                return 0 as libc::c_int as krb5_boolean
            }
        }
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Process events on a UDP socket.  Return true if we get a reply. */
#[c2rust::src_loc = "1188:1"]
unsafe extern "C" fn service_udp_read(mut context: krb5_context,
                                      mut realm: *const krb5_data,
                                      mut conn: *mut conn_state,
                                      mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut nread: libc::c_int = 0;
    nread =
        recv((*conn).fd, (*conn).in_0.buf as *mut libc::c_void,
             (*conn).in_0.bufsize, 0 as libc::c_int) as libc::c_int;
    if nread < 0 as libc::c_int {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"UDP error receiving from {raddr}: {errno}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address,
                          *__errno_location());
        }
        kill_conn(context, conn, selstate);
        return 0 as libc::c_int as krb5_boolean
    }
    (*conn).in_0.pos = nread as size_t;
    return 1 as libc::c_int as krb5_boolean;
}
/* Set up conn->http.tls.  Return true on success. */
#[c2rust::src_loc = "1205:1"]
unsafe extern "C" fn setup_tls(mut context: krb5_context,
                               mut realm: *const krb5_data,
                               mut conn: *mut conn_state,
                               mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut ret: krb5_error_code = 0;
    let mut ok: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut anchors: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut realmstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut names: [*const libc::c_char; 4] = [0 as *const libc::c_char; 4];
    if init_tls_vtable(context) != 0 as libc::c_int ||
           (*(*context).tls).setup.is_none() {
        return 0 as libc::c_int as krb5_boolean
    }
    realmstr =
        k5memdup0((*realm).data as *const libc::c_void,
                  (*realm).length as size_t, &mut ret) as *mut libc::c_char;
    if !realmstr.is_null() {
        /* Load the configured anchors. */
        names[0 as libc::c_int as usize] =
            b"realms\x00" as *const u8 as *const libc::c_char;
        names[1 as libc::c_int as usize] = realmstr;
        names[2 as libc::c_int as usize] =
            b"http_anchors\x00" as *const u8 as *const libc::c_char;
        names[3 as libc::c_int as usize] = 0 as *const libc::c_char;
        ret =
            profile_get_values((*context).profile, names.as_mut_ptr(),
                               &mut anchors) as krb5_error_code;
        if !(ret != 0 as libc::c_int &&
                 ret as libc::c_long != -(1429577725 as libc::c_long)) {
            if (*(*context).tls).setup.expect("non-null function pointer")(context,
                                                                           (*conn).fd,
                                                                           (*conn).http.servername,
                                                                           anchors,
                                                                           &mut (*conn).http.tls)
                   != 0 as libc::c_int {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"HTTPS error connecting to {raddr}\x00" as
                                      *const u8 as *const libc::c_char,
                                  &mut (*conn).addr as *mut remote_address);
                }
            } else { ok = 1 as libc::c_int as krb5_boolean }
        }
    }
    free(realmstr as *mut libc::c_void);
    profile_free_list(anchors);
    return ok;
}
/* Set conn->state to READING when done; otherwise, call a cm_set_. */
#[c2rust::src_loc = "1245:1"]
unsafe extern "C" fn service_https_write(mut context: krb5_context,
                                         mut realm: *const krb5_data,
                                         mut conn: *mut conn_state,
                                         mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut st: k5_tls_status = DATA_READ;
    /* If this is our first time in here, set up the SSL context. */
    if (*conn).http.tls.is_null() &&
           setup_tls(context, realm, conn, selstate) == 0 {
        kill_conn(context, conn, selstate);
        return 0 as libc::c_int as krb5_boolean
    }
    /* Try to transmit our request to the server. */
    st =
        (*(*context).tls).write.expect("non-null function pointer")(context,
                                                                    (*conn).http.tls,
                                                                    ((*(*conn).out.sgp).iov_base
                                                                         as
                                                                         *mut libc::c_char).offset(0
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       isize)
                                                                        as
                                                                        *const libc::c_void,
                                                                    (*(*conn).out.sgbuf.as_mut_ptr()).iov_len.wrapping_add(0
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_ulong));
    if st as libc::c_uint == DONE as libc::c_int as libc::c_uint {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"Sending HTTPS request to {raddr}\x00" as *const u8
                              as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address);
        }
        cm_read(selstate, (*conn).fd);
        (*conn).state = READING
    } else if st as libc::c_uint == WANT_READ as libc::c_int as libc::c_uint {
        cm_read(selstate, (*conn).fd);
    } else if st as libc::c_uint == WANT_WRITE as libc::c_int as libc::c_uint
     {
        cm_write(selstate, (*conn).fd);
    } else if st as libc::c_uint == ERROR_TLS as libc::c_int as libc::c_uint {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"HTTPS error sending to {raddr}\x00" as *const u8
                              as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address);
        }
        kill_conn(context, conn, selstate);
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Return true on finished data.  Call a cm_read/write function and return
 * false if the TLS layer needs it.  Kill the connection on error. */
#[c2rust::src_loc = "1278:1"]
unsafe extern "C" fn https_read_bytes(mut context: krb5_context,
                                      mut conn: *mut conn_state,
                                      mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut bufsize: size_t = 0;
    let mut nread: size_t = 0;
    let mut st: k5_tls_status = DATA_READ;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut in_0: *mut incoming_message = &mut (*conn).in_0;
    loop  {
        if (*in_0).buf.is_null() ||
               (*in_0).bufsize.wrapping_sub((*in_0).pos) <
                   1024 as libc::c_int as libc::c_ulong {
            bufsize =
                if (*in_0).bufsize != 0 {
                    (*in_0).bufsize.wrapping_mul(2 as libc::c_int as
                                                     libc::c_ulong)
                } else { 8192 as libc::c_int as libc::c_ulong };
            if bufsize >
                   (1024 as libc::c_int * 1024 as libc::c_int) as
                       libc::c_ulong {
                kill_conn(context, conn, selstate);
                return 0 as libc::c_int as krb5_boolean
            }
            tmp =
                realloc((*in_0).buf as *mut libc::c_void, bufsize) as
                    *mut libc::c_char;
            if tmp.is_null() {
                kill_conn(context, conn, selstate);
                return 0 as libc::c_int as krb5_boolean
            }
            (*in_0).buf = tmp;
            (*in_0).bufsize = bufsize
        }
        st =
            (*(*context).tls).read.expect("non-null function pointer")(context,
                                                                       (*conn).http.tls,
                                                                       &mut *(*in_0).buf.offset((*in_0).pos
                                                                                                    as
                                                                                                    isize)
                                                                           as
                                                                           *mut libc::c_char
                                                                           as
                                                                           *mut libc::c_void,
                                                                       (*in_0).bufsize.wrapping_sub((*in_0).pos).wrapping_sub(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_ulong),
                                                                       &mut nread);
        if st as libc::c_uint != DATA_READ as libc::c_int as libc::c_uint {
            break ;
        }
        (*in_0).pos =
            ((*in_0).pos as libc::c_ulong).wrapping_add(nread) as size_t as
                size_t;
        *(*in_0).buf.offset((*in_0).pos as isize) =
            '\u{0}' as i32 as libc::c_char
    }
    if st as libc::c_uint == DONE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as krb5_boolean
    }
    if st as libc::c_uint == WANT_READ as libc::c_int as libc::c_uint {
        cm_read(selstate, (*conn).fd);
    } else if st as libc::c_uint == WANT_WRITE as libc::c_int as libc::c_uint
     {
        cm_write(selstate, (*conn).fd);
    } else if st as libc::c_uint == ERROR_TLS as libc::c_int as libc::c_uint {
        if (*context).trace_callback.is_some() {
            krb5int_trace(context,
                          b"HTTPS error receiving from {raddr}\x00" as
                              *const u8 as *const libc::c_char,
                          &mut (*conn).addr as *mut remote_address);
        }
        kill_conn(context, conn, selstate);
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* Return true on readable, valid KKDCPP data. */
#[c2rust::src_loc = "1327:1"]
unsafe extern "C" fn service_https_read(mut context: krb5_context,
                                        mut realm: *const krb5_data,
                                        mut conn: *mut conn_state,
                                        mut selstate: *mut select_state)
 -> krb5_boolean {
    let mut pm: *mut krb5_kkdcp_message = 0 as *mut krb5_kkdcp_message;
    let mut buf: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut rep: *const libc::c_char = 0 as *const libc::c_char;
    let mut in_0: *mut incoming_message = &mut (*conn).in_0;
    /* Read data through the encryption layer. */
    if https_read_bytes(context, conn, selstate) == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    /* Find the beginning of the response body. */
    rep =
        strstr((*in_0).buf,
               b"\r\n\r\n\x00" as *const u8 as *const libc::c_char);
    if !rep.is_null() {
        rep = rep.offset(4 as libc::c_int as isize);
        /* Decode the response. */
        buf =
            make_data(rep as *mut libc::c_char as *mut libc::c_void,
                      (*in_0).pos.wrapping_sub(rep.wrapping_offset_from((*in_0).buf)
                                                   as libc::c_long as
                                                   libc::c_ulong) as
                          libc::c_uint);
        if !(decode_krb5_kkdcp_message(&mut buf, &mut pm) != 0 as libc::c_int)
           {
            /* Check and discard the message length at the front of the kerb_message
     * field after decoding.  If it's wrong or missing, something broke. */
            if !((*pm).kerb_message.length < 4 as libc::c_int as libc::c_uint
                     ||
                     load_32_be((*pm).kerb_message.data as
                                    *const libc::c_void) !=
                         (*pm).kerb_message.length.wrapping_sub(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint))
               {
                /* Replace all of the content that we read back with just the message. */
                memcpy((*in_0).buf as *mut libc::c_void,
                       (*pm).kerb_message.data.offset(4 as libc::c_int as
                                                          isize) as
                           *const libc::c_void,
                       (*pm).kerb_message.length.wrapping_sub(4 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                           as libc::c_ulong);
                (*in_0).pos =
                    (*pm).kerb_message.length.wrapping_sub(4 as libc::c_int as
                                                               libc::c_uint)
                        as size_t;
                k5_free_kkdcp_message(context, pm);
                return 1 as libc::c_int as krb5_boolean
            }
        }
    }
    if (*context).trace_callback.is_some() {
        krb5int_trace(context,
                      b"HTTPS error: {str}\x00" as *const u8 as
                          *const libc::c_char, (*in_0).buf);
    }
    k5_free_kkdcp_message(context, pm);
    kill_conn(context, conn, selstate);
    return 0 as libc::c_int as krb5_boolean;
}
/* Return the maximum of endtime and the endtime fields of all currently active
 * TCP connections. */
#[c2rust::src_loc = "1374:1"]
unsafe extern "C" fn get_endtime(mut endtime: time_ms,
                                 mut conns: *mut conn_state) -> time_ms {
    let mut state: *mut conn_state = 0 as *mut conn_state;
    state = conns;
    while !state.is_null() {
        if ((*state).state as libc::c_uint ==
                READING as libc::c_int as libc::c_uint ||
                (*state).state as libc::c_uint ==
                    WRITING as libc::c_int as libc::c_uint) &&
               (*state).endtime > endtime {
            endtime = (*state).endtime
        }
        state = (*state).next
    }
    return endtime;
}
#[c2rust::src_loc = "1387:1"]
unsafe extern "C" fn service_fds(mut context: krb5_context,
                                 mut selstate: *mut select_state,
                                 mut interval: time_ms,
                                 mut conns: *mut conn_state,
                                 mut seltemp: *mut select_state,
                                 mut realm: *const krb5_data,
                                 mut msg_handler:
                                     Option<unsafe extern "C" fn(_:
                                                                     krb5_context,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut libc::c_void)
                                                -> libc::c_int>,
                                 mut msg_handler_data: *mut libc::c_void,
                                 mut winner_out: *mut *mut conn_state)
 -> krb5_boolean {
    let mut e: libc::c_int = 0;
    let mut selret: libc::c_int = 0 as libc::c_int;
    let mut endtime: time_ms = 0;
    let mut state: *mut conn_state = 0 as *mut conn_state;
    *winner_out = 0 as *mut conn_state;
    e = get_curtime_ms(&mut endtime);
    if e != 0 { return 1 as libc::c_int as krb5_boolean }
    endtime += interval;
    e = 0 as libc::c_int;
    while (*selstate).nfds > 0 as libc::c_int {
        e =
            cm_select_or_poll(selstate, get_endtime(endtime, conns), seltemp,
                              &mut selret);
        if e == 4 as libc::c_int { continue ; }
        if e != 0 as libc::c_int { break ; }
        if selret == 0 as libc::c_int {
            /* Timeout, return to caller.  */
            return 0 as libc::c_int as krb5_boolean
        }
        /* Got something on a socket, process it.  */
        state = conns;
        while !state.is_null() {
            let mut ssflags: libc::c_int = 0;
            if !((*state).fd == !(0 as libc::c_int)) {
                ssflags = cm_get_ssflags(seltemp, (*state).fd) as libc::c_int;
                if !(ssflags == 0) {
                    if service_dispatch(context, realm, state, selstate,
                                        ssflags) != 0 {
                        let mut stop: libc::c_int = 1 as libc::c_int;
                        if msg_handler.is_some() {
                            let mut reply: krb5_data =
                                make_data((*state).in_0.buf as
                                              *mut libc::c_void,
                                          (*state).in_0.pos as libc::c_uint);
                            stop =
                                (msg_handler.expect("non-null function pointer")(context,
                                                                                 &mut reply,
                                                                                 msg_handler_data)
                                     != 0 as libc::c_int) as libc::c_int
                        }
                        if stop != 0 {
                            *winner_out = state;
                            return 1 as libc::c_int as krb5_boolean
                        }
                    }
                }
            }
            state = (*state).next
        }
    }
    if e != 0 as libc::c_int { return 1 as libc::c_int as krb5_boolean }
    return 0 as libc::c_int as krb5_boolean;
}
/*
 * Current worst-case timeout behavior:
 *
 * First pass, 1s per udp or tcp server, plus 2s at end.
 * Second pass, 1s per udp server, plus 4s.
 * Third pass, 1s per udp server, plus 8s.
 * Fourth => 16s, etc.
 *
 * Restated:
 * Per UDP server, 1s per pass.
 * Per TCP server, 1s.
 * Backoff delay, 2**(P+1) - 2, where P is total number of passes.
 *
 * Total = 2**(P+1) + U*P + T - 2.
 *
 * If P=3, Total = 3*U + T + 14.
 * If P=4, Total = 4*U + T + 30.
 *
 * Note that if you try to reach two ports on one server, it counts as two.
 *
 * There is one exception to the above rules.  Whenever a TCP connection is
 * established, we wait up to ten seconds for it to finish or fail before
 * moving on.  This reduces network traffic significantly in a TCP environment.
 */
#[no_mangle]
#[c2rust::src_loc = "1474:1"]
pub unsafe extern "C" fn k5_sendto(mut context: krb5_context,
                                   mut message: *const krb5_data,
                                   mut realm: *const krb5_data,
                                   mut servers: *const serverlist,
                                   mut strategy: k5_transport_strategy,
                                   mut callback_info:
                                       *mut sendto_callback_info,
                                   mut reply: *mut krb5_data,
                                   mut remoteaddr: *mut sockaddr,
                                   mut remoteaddrlen: *mut socklen_t,
                                   mut server_used: *mut libc::c_int,
                                   mut msg_handler:
                                       Option<unsafe extern "C" fn(_:
                                                                       krb5_context,
                                                                   _:
                                                                       *const krb5_data,
                                                                   _:
                                                                       *mut libc::c_void)
                                                  -> libc::c_int>,
                                   mut msg_handler_data: *mut libc::c_void)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut pass: libc::c_int = 0;
    let mut delay: time_ms = 0;
    let mut retval: krb5_error_code = 0;
    let mut conns: *mut conn_state = 0 as *mut conn_state;
    let mut state: *mut conn_state = 0 as *mut conn_state;
    let mut tailptr: *mut *mut conn_state = 0 as *mut *mut conn_state;
    let mut next: *mut conn_state = 0 as *mut conn_state;
    let mut winner: *mut conn_state = 0 as *mut conn_state;
    let mut s: size_t = 0;
    let mut sel_state: *mut select_state = 0 as *mut select_state;
    let mut seltemp: *mut select_state = 0 as *mut select_state;
    let mut udpbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut done: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    *reply = empty_data();
    /* One for use here, listing all our fds in use, and one for
     * temporary use in service_fds, for the fds of interest.  */
    sel_state =
        malloc((2 as libc::c_int as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<select_state>()
                                                    as libc::c_ulong)) as
            *mut select_state;
    if sel_state.is_null() {
        retval = 12 as libc::c_int
    } else {
        seltemp =
            &mut *sel_state.offset(1 as libc::c_int as isize) as
                *mut select_state;
        cm_init_selstate(sel_state);
        /* First pass: resolve server hosts, communicate with resulting addresses
     * of the preferred transport, and wait 1s for an answer from each. */
        s = 0 as libc::c_int as size_t;
        loop  {
            if !(s < (*servers).nservers && done == 0) {
                current_block = 13472856163611868459;
                break ;
            }
            /* Find the current tail pointer. */
            tailptr = &mut conns;
            while !(*tailptr).is_null() { tailptr = &mut (**tailptr).next }
            retval =
                resolve_server(context, realm, servers, s, strategy, message,
                               &mut udpbuf, &mut conns);
            if retval != 0 { current_block = 8807635080396349557; break ; }
            state = *tailptr;
            while !state.is_null() && done == 0 {
                /* Contact each new connection, deferring those which use the
             * non-preferred RFC 4120 transport. */
                if !((*state).defer != 0) {
                    if !(maybe_send(context, state, message, sel_state, realm,
                                    callback_info) != 0) {
                        done =
                            service_fds(context, sel_state,
                                        1000 as libc::c_int as time_ms, conns,
                                        seltemp, realm, msg_handler,
                                        msg_handler_data, &mut winner)
                    }
                }
                state = (*state).next
            }
            s = s.wrapping_add(1)
        }
        match current_block {
            8807635080396349557 => { }
            _ => {
                /* Complete the first pass by contacting servers of the non-preferred RFC
     * 4120 transport (if given), waiting 1s for an answer from each. */
                state = conns;
                while !state.is_null() && done == 0 {
                    if !((*state).defer == 0) {
                        if !(maybe_send(context, state, message, sel_state,
                                        realm, callback_info) != 0) {
                            done =
                                service_fds(context, sel_state,
                                            1000 as libc::c_int as time_ms,
                                            conns, seltemp, realm,
                                            msg_handler, msg_handler_data,
                                            &mut winner)
                        }
                    }
                    state = (*state).next
                }
                /* Wait for two seconds at the end of the first pass. */
                if done == 0 {
                    done =
                        service_fds(context, sel_state,
                                    2000 as libc::c_int as time_ms, conns,
                                    seltemp, realm, msg_handler,
                                    msg_handler_data, &mut winner)
                }
                /* Make remaining passes over all of the connections. */
                delay = 4000 as libc::c_int as time_ms;
                pass = 1 as libc::c_int;
                while pass < 3 as libc::c_int && done == 0 {
                    state = conns;
                    while !state.is_null() && done == 0 {
                        if !(maybe_send(context, state, message, sel_state,
                                        realm, callback_info) != 0) {
                            done =
                                service_fds(context, sel_state,
                                            1000 as libc::c_int as time_ms,
                                            conns, seltemp, realm,
                                            msg_handler, msg_handler_data,
                                            &mut winner);
                            if (*sel_state).nfds == 0 as libc::c_int {
                                break ;
                            }
                        }
                        state = (*state).next
                    }
                    /* Wait for the delay backoff at the end of this pass. */
                    if done == 0 {
                        done =
                            service_fds(context, sel_state, delay, conns,
                                        seltemp, realm, msg_handler,
                                        msg_handler_data, &mut winner)
                    }
                    if (*sel_state).nfds == 0 as libc::c_int { break ; }
                    delay *= 2 as libc::c_int as libc::c_long;
                    pass += 1
                }
                if (*sel_state).nfds == 0 as libc::c_int || done == 0 ||
                       winner.is_null() {
                    retval = -(1765328228 as libc::c_long) as krb5_error_code
                } else {
                    /* Success!  */
                    *reply =
                        make_data((*winner).in_0.buf as *mut libc::c_void,
                                  (*winner).in_0.pos as libc::c_uint);
                    retval = 0 as libc::c_int;
                    (*winner).in_0.buf = 0 as *mut libc::c_char;
                    if !server_used.is_null() {
                        *server_used = (*winner).server_index as libc::c_int
                    }
                    if !remoteaddr.is_null() && !remoteaddrlen.is_null() &&
                           *remoteaddrlen > 0 as libc::c_int as libc::c_uint {
                        getpeername((*winner).fd,
                                    __SOCKADDR_ARG{__sockaddr__: remoteaddr,},
                                    remoteaddrlen);
                    }
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"Received answer ({int} bytes) from {raddr}\x00"
                                          as *const u8 as *const libc::c_char,
                                      (*reply).length,
                                      &mut (*winner).addr as
                                          *mut remote_address);
                    }
                }
            }
        }
    }
    state = conns;
    while !state.is_null() {
        next = (*state).next;
        if (*state).fd != !(0 as libc::c_int) {
            if socktype_for_transport((*state).addr.transport) ==
                   SOCK_STREAM as libc::c_int {
                if (*context).trace_callback.is_some() {
                    krb5int_trace(context,
                                  b"Terminating TCP connection to {raddr}\x00"
                                      as *const u8 as *const libc::c_char,
                                  &mut (*state).addr as *mut remote_address);
                }
            }
            close((*state).fd);
            free_http_tls_data(context, state);
        }
        if (*state).in_0.buf != udpbuf {
            free((*state).in_0.buf as *mut libc::c_void);
        }
        if !callback_info.is_null() {
            (*callback_info).pfn_cleanup.expect("non-null function pointer")((*callback_info).data,
                                                                             &mut (*state).callback_buffer);
        }
        free(state as *mut libc::c_void);
        state = next
    }
    if (*reply).data != udpbuf { free(udpbuf as *mut libc::c_void); }
    free(sel_state as *mut libc::c_void);
    return retval;
}
