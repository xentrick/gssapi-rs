use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
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
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:27"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:27"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:27"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    use super::pthreadtypes_h::pthread_mutex_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
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
    /*
 * end "safepriv.h"
 */
    /*
 * begin "ccache.h"
 */
    /* * Cursor for sequential lookup */
    #[c2rust::src_loc = "2278:1"]
    pub type krb5_cc_cursor = krb5_pointer;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2283:1"]
    pub type krb5_cc_ops = _krb5_cc_ops;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::_krb5_context;
    use super::cc_int_h::{_krb5_ccache, _krb5_cc_ops};
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
 * Generate pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [out] data            Random data
 *
 * Fills in @a data with bytes from the PRNG used by krb5 crypto operations.
 * The caller must preinitialize @a data and allocate the desired amount of
 * space.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "800:1"]
        pub fn krb5_c_random_make_octets(context: krb5_context,
                                         data: *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Copy a krb5_creds structure.
 *
 * @param [in]  context         Library context
 * @param [in]  incred          Credentials structure to be copied
 * @param [out] outcred         Copy of @a incred
 *
 * This function creates a new credential with the contents of @a incred.  Use
 * krb5_free_creds() to free @a outcred when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3782:1"]
        pub fn krb5_copy_creds(context: krb5_context,
                               incred: *const krb5_creds,
                               outcred: *mut *mut krb5_creds)
         -> krb5_error_code;
        /* *
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
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
        /* *
 * Return the name of the default credential cache.
 *
 * @param [in] context          Library context
 *
 * Return a pointer to the default credential cache name for @a context, as
 * determined by a prior call to krb5_cc_set_default_name(), by the KRB5CCNAME
 * environment variable, by the default_ccache_name profile variable, or by the
 * operating system or build-time default value.  The returned value must not
 * be modified or freed by the caller.  The returned value becomes invalid when
 * @a context is destroyed krb5_free_context() or if a subsequent call to
 * krb5_cc_set_default_name() is made on @a context.
 *
 * The default credential cache name is cached in @a context between calls to
 * this function, so if the value of KRB5CCNAME changes in the process
 * environment after the first call to this function on, that change will not
 * be reflected in later calls with the same context.  The caller can invoke
 * krb5_cc_set_default_name() with a NULL value of @a name to clear the cached
 * value and force the default name to be recomputed.
 *
 * @return
 * Name of default credential cache for the current user.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4380:1"]
        pub fn krb5_cc_default_name(context: krb5_context)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4666:1"]
        pub fn krb5_free_creds(context: krb5_context, val: *mut krb5_creds);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
    #[c2rust::src_loc = "702:1"]
    pub type krb5_os_context = *mut _krb5_os_context;
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
                        krb5_context, krb5_error_code, krb5_data};
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
        #[no_mangle]
        #[c2rust::src_loc = "1912:1"]
        pub fn krb5int_random_string(_: krb5_context,
                                     string: *mut libc::c_char,
                                     length: libc::c_uint) -> krb5_error_code;
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/ccache/cc-int.h:27"]
pub mod cc_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/ccache/cc-int.h */
/*
 * Copyright 1990,1991 by the Massachusetts Institute of Technology.
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
    /* This file contains constant and function declarations used in the
 * file-based credential cache routines. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "35:8"]
    pub struct _krb5_ccache {
        pub magic: krb5_magic,
        pub ops: *const _krb5_cc_ops,
        pub data: krb5_pointer,
    }
    /*
 * Cursor for iterating over ccache types
 */
    /* reentrant mutex used by krb5_cc_* functions */
    /*
 * Per-type ccache cursor.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct _krb5_cc_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_ccache)
                                 -> *const libc::c_char>,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut krb5_ccache,
                                                 _: *const libc::c_char)
                                -> krb5_error_code>,
        pub gen_new: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut krb5_ccache)
                                -> krb5_error_code>,
        pub init: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_ccache,
                                              _: krb5_principal)
                             -> krb5_error_code>,
        pub destroy: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_ccache)
                                -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_ccache)
                              -> krb5_error_code>,
        pub store: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_ccache,
                                               _: *mut krb5_creds)
                              -> krb5_error_code>,
        pub retrieve: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_ccache,
                                                  _: krb5_flags,
                                                  _: *mut krb5_creds,
                                                  _: *mut krb5_creds)
                                 -> krb5_error_code>,
        pub get_princ: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: *mut krb5_principal)
                                  -> krb5_error_code>,
        pub get_first: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: *mut krb5_cc_cursor)
                                  -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_ccache,
                                                  _: *mut krb5_cc_cursor,
                                                  _: *mut krb5_creds)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_ccache,
                                                 _: *mut krb5_cc_cursor)
                                -> krb5_error_code>,
        pub remove_cred: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: krb5_ccache,
                                                     _: krb5_flags,
                                                     _: *mut krb5_creds)
                                    -> krb5_error_code>,
        pub set_flags: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: krb5_flags)
                                  -> krb5_error_code>,
        pub get_flags: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache,
                                                   _: *mut krb5_flags)
                                  -> krb5_error_code>,
        pub ptcursor_new: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _:
                                                          *mut krb5_cc_ptcursor)
                                     -> krb5_error_code>,
        pub ptcursor_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_cc_ptcursor,
                                                       _: *mut krb5_ccache)
                                      -> krb5_error_code>,
        pub ptcursor_free: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           *mut krb5_cc_ptcursor)
                                      -> krb5_error_code>,
        pub move_0: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_ccache,
                                                _: krb5_ccache)
                               -> krb5_error_code>,
        pub wasdefault: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: krb5_ccache,
                                                    _: *mut krb5_timestamp)
                                   -> krb5_error_code>,
        pub lock: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_ccache)
                             -> krb5_error_code>,
        pub unlock: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_ccache)
                               -> krb5_error_code>,
        pub switch_to: Option<unsafe extern "C" fn(_: krb5_context,
                                                   _: krb5_ccache)
                                  -> krb5_error_code>,
    }
    #[c2rust::src_loc = "174:1"]
    pub type krb5_cc_ptcursor = *mut krb5_cc_ptcursor_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "170:8"]
    pub struct krb5_cc_ptcursor_s {
        pub ops: *const _krb5_cc_ops,
        pub data: krb5_pointer,
    }
    #[c2rust::src_loc = "75:1"]
    pub type k5_cc_mutex = _k5_cc_mutex;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:16"]
    pub struct _k5_cc_mutex {
        pub lock: k5_mutex_t,
        pub owner: krb5_context,
        pub refcount: krb5_int32,
    }
    use super::krb5_h::{krb5_magic, krb5_pointer, krb5_context, krb5_ccache,
                        krb5_error_code, krb5_principal, krb5_creds,
                        krb5_flags, krb5_cc_cursor, krb5_timestamp,
                        krb5_int32, krb5_boolean};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn k5_cc_retrieve_cred_default(_: krb5_context, _: krb5_ccache,
                                           _: krb5_flags, _: *mut krb5_creds,
                                           _: *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "45:1"]
        pub fn krb5int_cc_creds_match_request(_: krb5_context,
                                              whichfields: krb5_flags,
                                              mcreds: *mut krb5_creds,
                                              creds: *mut krb5_creds)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn k5_cc_mutex_init(m: *mut k5_cc_mutex) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn k5_cc_mutex_lock(context: krb5_context, m: *mut k5_cc_mutex);
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn k5_cc_mutex_unlock(context: krb5_context, m: *mut k5_cc_mutex);
    }
    /* __KRB5_CCACHE_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-hashtab.h:29"]
pub mod k5_hashtab_h {
    use super::stdint_uintn_h::uint8_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-hash.h - hash table interface definitions */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
 * This file contains declarations for a simple hash table using siphash.  Some
 * limitations which might need to be addressed in the future:
 *
 * - The table does not manage caller memory.  This limitation could be
 *   addressed by adding an optional free callback to k5_hashtab_create(), to
 *   be called by k5_hashtab_free() and k5_hashtab_remove().
 *
 * - There is no way to iterate over a hash table.
 *
 * - k5_hashtab_add() does not check for duplicate entries.
 */
        #[c2rust::src_loc = "51:8"]
        pub type k5_hashtab;
        /*
 * Create a new hash table in *ht_out.  seed must point to random bytes if keys
 * might be under the control of an attacker; otherwise it may be NULL.
 * initial_buckets controls the initial allocation of hash buckets; pass zero
 * to use a default value.  The number of hash buckets will be doubled as the
 * number of entries increases.  Return 0 on success, ENOMEM on failure.
 */
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn k5_hashtab_create(seed: *const uint8_t,
                                 initial_buckets: size_t,
                                 ht_out: *mut *mut k5_hashtab) -> libc::c_int;
        /* Add an entry to a hash table.  key and val must remain valid until the entry
 * is removed or the hash table is freed.  The caller must avoid duplicates. */
        #[no_mangle]
        #[c2rust::src_loc = "69:1"]
        pub fn k5_hashtab_add(ht: *mut k5_hashtab, key: *const libc::c_void,
                              klen: size_t, val: *mut libc::c_void)
         -> libc::c_int;
        /* Remove an entry from a hash table by key.  Does not free key or the
 * associated value.  Return 1 if the key was found and removed, 0 if not. */
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_hashtab_remove(ht: *mut k5_hashtab,
                                 key: *const libc::c_void, klen: size_t)
         -> libc::c_int;
        /* Retrieve a value from a hash table by key. */
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_hashtab_get(ht: *mut k5_hashtab, key: *const libc::c_void,
                              klen: size_t) -> *mut libc::c_void;
    }
    /* K5_HASH_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
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
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/krb/int-proto.h:28"]
pub mod int_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_creds, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "314:1"]
        pub fn k5_copy_creds_contents(_: krb5_context, _: *const krb5_creds,
                                      _: *mut krb5_creds) -> krb5_error_code;
    }
    /* KRB5_INT_FUNC_PROTO__ */
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_os_mutex_destroy};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_addrtype,
                       krb5_enctype, krb5_authdatatype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, _krb5_keyblock, krb5_keyblock,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_creds, krb5_creds, krb5_cc_cursor,
                       krb5_ccache, krb5_cc_ops, _profile_t,
                       krb5_c_random_make_octets, krb5_copy_creds,
                       krb5_copy_principal, krb5_cc_resolve,
                       krb5_cc_default_name, krb5_free_principal,
                       krb5_free_creds};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_os_context, make_data,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_random_string};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::cc_int_h::{_krb5_ccache, _krb5_cc_ops, krb5_cc_ptcursor,
                         krb5_cc_ptcursor_s, k5_cc_mutex, _k5_cc_mutex,
                         k5_cc_retrieve_cred_default,
                         krb5int_cc_creds_match_request, k5_cc_mutex_init,
                         k5_cc_mutex_lock, k5_cc_mutex_unlock};
use self::k5_hashtab_h::{k5_hashtab, k5_hashtab_create, k5_hashtab_add,
                         k5_hashtab_remove, k5_hashtab_get};
use self::stdlib_h::{malloc, free};
use self::string_h::{strlen, strdup, strncmp, memset};
use self::int_proto_h::k5_copy_creds_contents;
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "95:1"]
    pub fn krb5_change_cache() -> krb5_error_code;
}
/* Individual credentials within a cache, in a linked list.  */
#[c2rust::src_loc = "100:1"]
pub type krb5_mcc_link = _krb5_mcc_link;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "100:16"]
pub struct _krb5_mcc_link {
    pub next: *mut _krb5_mcc_link,
    pub creds: *mut krb5_creds,
}
/* Iterator over memory caches.  */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "125:8"]
pub struct krb5_mcc_ptcursor_data {
    pub first: krb5_boolean,
}
/* Per-cache data header.  */
#[c2rust::src_loc = "106:1"]
pub type krb5_mcc_data = _krb5_mcc_data;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "106:16"]
pub struct _krb5_mcc_data {
    pub name: *mut libc::c_char,
    pub lock: k5_cc_mutex,
    pub prin: krb5_principal,
    pub link: *mut krb5_mcc_link,
    pub time_offset: krb5_int32,
    pub usec_offset: krb5_int32,
    pub refcount: libc::c_int,
    pub generation: libc::c_int,
}
/* Time offsets for clock-skewed clients.  */
/* One for the table slot, one per handle */
/* Incremented at each initialize */
/* Iterator over credentials in a memory cache. */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "119:8"]
pub struct mcc_cursor {
    pub generation: libc::c_int,
    pub next_link: *mut krb5_mcc_link,
}
#[no_mangle]
#[c2rust::src_loc = "129:13"]
pub static mut krb5int_mcc_mutex: k5_cc_mutex =
    {
        let mut init =
            _k5_cc_mutex{lock:
                             pthread_mutex_t{__data:
                                                 {
                                                     let mut init =
                                                         __pthread_mutex_s{__lock:
                                                                               0
                                                                                   as
                                                                                   libc::c_int,
                                                                           __count:
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint,
                                                                           __owner:
                                                                               0
                                                                                   as
                                                                                   libc::c_int,
                                                                           __nusers:
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint,
                                                                           __kind:
                                                                               0
                                                                                   as
                                                                                   libc::c_int,
                                                                           __spins:
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_short,
                                                                           __elision:
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_short,
                                                                           __list:
                                                                               {
                                                                                   let mut init =
                                                                                       __pthread_internal_list{__prev:
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *const __pthread_internal_list
                                                                                                                       as
                                                                                                                       *mut __pthread_internal_list,
                                                                                                               __next:
                                                                                                                   0
                                                                                                                       as
                                                                                                                       *const __pthread_internal_list
                                                                                                                       as
                                                                                                                       *mut __pthread_internal_list,};
                                                                                   init
                                                                               },};
                                                     init
                                                 },},
                         owner: 0 as *const _krb5_context as krb5_context,
                         refcount: 0 as libc::c_int,};
        init
    };
#[c2rust::src_loc = "130:27"]
static mut mcc_hashtab: *mut k5_hashtab =
    0 as *const k5_hashtab as *mut k5_hashtab;
/* Ensure that mcc_hashtab is initialized.  Call with krb5int_mcc_mutex
 * locked. */
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn init_table(mut context: krb5_context)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut seed: [uint8_t; 16] = [0; 16];
    let mut d: krb5_data =
        make_data(seed.as_mut_ptr() as *mut libc::c_void,
                  ::std::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong as
                      libc::c_uint);
    if !mcc_hashtab.is_null() { return 0 as libc::c_int }
    ret = krb5_c_random_make_octets(context, &mut d);
    if ret != 0 { return ret }
    return k5_hashtab_create(seed.as_mut_ptr() as *const uint8_t,
                             64 as libc::c_int as size_t, &mut mcc_hashtab);
}
/* Remove creds from d, invalidate any existing cursors, and unset the client
 * principal.  The caller is responsible for locking. */
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn empty_mcc_cache(mut context: krb5_context,
                                     mut d: *mut krb5_mcc_data) {
    let mut curr: *mut krb5_mcc_link = 0 as *mut krb5_mcc_link;
    let mut next: *mut krb5_mcc_link = 0 as *mut krb5_mcc_link;
    curr = (*d).link;
    while !curr.is_null() {
        next = (*curr).next;
        krb5_free_creds(context, (*curr).creds);
        free(curr as *mut libc::c_void);
        curr = next
    }
    (*d).link = 0 as *mut krb5_mcc_link;
    (*d).generation += 1;
    krb5_free_principal(context, (*d).prin);
    (*d).prin = 0 as krb5_principal;
}
/*
 * Modifies:
 * id
 *
 * Effects:
 * Creates/refreshes the memory cred cache id.  If the cache exists, its
 * contents are destroyed.
 *
 * Errors:
 * system errors
 */
#[c2rust::src_loc = "178:1"]
unsafe extern "C" fn krb5_mcc_initialize(mut context: krb5_context,
                                         mut id: krb5_ccache,
                                         mut princ: krb5_principal)
 -> krb5_error_code {
    let mut os_ctx: krb5_os_context = &mut (*context).os_context;
    let mut ret: krb5_error_code = 0;
    let mut d: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    k5_cc_mutex_lock(context, &mut (*d).lock);
    empty_mcc_cache(context, d);
    ret =
        krb5_copy_principal(context, princ as krb5_const_principal,
                            &mut (*d).prin);
    if (*os_ctx).os_flags & 1 as libc::c_int != 0 {
        /* Store client time offsets in the cache */
        (*d).time_offset = (*os_ctx).time_offset;
        (*d).usec_offset = (*os_ctx).usec_offset
    }
    k5_cc_mutex_unlock(context, &mut (*d).lock);
    if ret == 0 as libc::c_int { krb5_change_cache(); }
    return ret;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/ccache/cc_memory.c - Memory-based credential cache */
/*
 * Copyright 1990,1991,2000,2004,2008 by the Massachusetts Institute of
 * Technology.  All Rights Reserved.
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
 * Modifies:
 * id
 *
 * Effects:
 * Invalidates the id, and frees any resources associated with accessing
 * the cache.
 */
#[c2rust::src_loc = "210:1"]
unsafe extern "C" fn krb5_mcc_close(mut context: krb5_context,
                                    mut id: krb5_ccache) -> krb5_error_code {
    let mut d: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    let mut count: libc::c_int = 0;
    free(id as *mut libc::c_void);
    k5_cc_mutex_lock(context, &mut (*d).lock);
    (*d).refcount -= 1;
    count = (*d).refcount;
    k5_cc_mutex_unlock(context, &mut (*d).lock);
    if count == 0 as libc::c_int {
        /* This is the last active handle referencing d and d has been removed
         * from the table, so we can release it. */
        empty_mcc_cache(context, d);
        free((*d).name as *mut libc::c_void);
        k5_os_mutex_destroy(&mut (*d).lock.lock);
        free(d as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
/*
 * Effects:
 * Destroys the contents of id. id is invalid after call.
 */
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn krb5_mcc_destroy(mut context: krb5_context,
                                      mut id: krb5_ccache)
 -> krb5_error_code {
    let mut d: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    let mut removed_from_table: krb5_boolean =
        0 as libc::c_int as krb5_boolean;
    /* Remove this node from the table if it is still present. */
    k5_cc_mutex_lock(context, &mut krb5int_mcc_mutex);
    if k5_hashtab_remove(mcc_hashtab, (*d).name as *const libc::c_void,
                         strlen((*d).name)) != 0 {
        removed_from_table = 1 as libc::c_int as krb5_boolean
    }
    k5_cc_mutex_unlock(context, &mut krb5int_mcc_mutex);
    /* Empty the cache and remove the reference for the table slot.  There will
     * always be at least one reference left for the handle being destroyed. */
    k5_cc_mutex_lock(context, &mut (*d).lock);
    empty_mcc_cache(context, d);
    if removed_from_table != 0 { (*d).refcount -= 1 }
    k5_cc_mutex_unlock(context, &mut (*d).lock);
    /* Invalidate the handle, possibly removing the last reference to d and
     * freeing it. */
    krb5_mcc_close(context, id);
    krb5_change_cache();
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "284:1"]
unsafe extern "C" fn krb5_mcc_resolve(mut context: krb5_context,
                                      mut id: *mut krb5_ccache,
                                      mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut os_ctx: krb5_os_context = &mut (*context).os_context;
    let mut lid: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut err: krb5_error_code = 0;
    let mut d: *mut krb5_mcc_data = 0 as *mut krb5_mcc_data;
    k5_cc_mutex_lock(context, &mut krb5int_mcc_mutex);
    init_table(context);
    d =
        k5_hashtab_get(mcc_hashtab, residual as *const libc::c_void,
                       strlen(residual)) as *mut krb5_mcc_data;
    if !d.is_null() {
        k5_cc_mutex_lock(context, &mut (*d).lock);
        (*d).refcount += 1;
        k5_cc_mutex_unlock(context, &mut (*d).lock);
    } else {
        err = new_mcc_data(residual, &mut d);
        if err != 0 {
            k5_cc_mutex_unlock(context, &mut krb5int_mcc_mutex);
            return err
        }
    }
    k5_cc_mutex_unlock(context, &mut krb5int_mcc_mutex);
    lid =
        malloc(::std::mem::size_of::<_krb5_ccache>() as libc::c_ulong) as
            krb5_ccache;
    if lid.is_null() {
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    if (*context).library_options & 0x1 as libc::c_int != 0 &&
           (*os_ctx).os_flags & 1 as libc::c_int == 0 {
        /* Use the time offset from the cache entry */
        (*os_ctx).time_offset = (*d).time_offset;
        (*os_ctx).usec_offset = (*d).usec_offset;
        (*os_ctx).os_flags =
            (*os_ctx).os_flags & !(2 as libc::c_int) | 1 as libc::c_int
    }
    (*lid).ops = &krb5_mcc_ops;
    (*lid).data = d as krb5_pointer;
    *id = lid;
    return 0 as libc::c_int;
}
/*
 * Effects:
 * Prepares for a sequential search of the credentials cache.
 * Returns a krb5_cc_cursor to be used with krb5_mcc_next_cred and
 * krb5_mcc_end_seq_get.
 *
 * If the cache is modified between the time of this call and the time
 * of the final krb5_mcc_end_seq_get, the results are undefined.
 *
 * Errors:
 * KRB5_CC_NOMEM
 * system errors
 */
#[c2rust::src_loc = "340:1"]
unsafe extern "C" fn krb5_mcc_start_seq_get(mut context: krb5_context,
                                            mut id: krb5_ccache,
                                            mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    let mut mcursor: *mut mcc_cursor = 0 as *mut mcc_cursor;
    let mut d: *mut krb5_mcc_data = 0 as *mut krb5_mcc_data;
    mcursor =
        malloc(::std::mem::size_of::<mcc_cursor>() as libc::c_ulong) as
            *mut mcc_cursor;
    if mcursor.is_null() {
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    d = (*id).data as *mut krb5_mcc_data;
    k5_cc_mutex_lock(context, &mut (*d).lock);
    (*mcursor).generation = (*d).generation;
    (*mcursor).next_link = (*d).link;
    k5_cc_mutex_unlock(context, &mut (*d).lock);
    *cursor = mcursor as krb5_cc_cursor;
    return 0 as libc::c_int;
}
/*
 * Requires:
 * cursor is a krb5_cc_cursor originally obtained from
 * krb5_mcc_start_seq_get.
 *
 * Modifes:
 * cursor, creds
 *
 * Effects:
 * Fills in creds with the "next" credentals structure from the cache
 * id.  The actual order the creds are returned in is arbitrary.
 * Space is allocated for the variable length fields in the
 * credentials structure, so the object returned must be passed to
 * krb5_destroy_credential.
 *
 * The cursor is updated for the next call to krb5_mcc_next_cred.
 *
 * Errors:
 * system errors
 */
#[c2rust::src_loc = "379:1"]
unsafe extern "C" fn krb5_mcc_next_cred(mut context: krb5_context,
                                        mut id: krb5_ccache,
                                        mut cursor: *mut krb5_cc_cursor,
                                        mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut mcursor: *mut mcc_cursor = 0 as *mut mcc_cursor;
    let mut retval: krb5_error_code = 0;
    let mut d: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    memset(creds as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    mcursor = *cursor as *mut mcc_cursor;
    if (*mcursor).next_link.is_null() {
        return -(1765328242 as libc::c_long) as krb5_error_code
    }
    /*
     * Check the cursor generation against the cache generation in case the
     * cache has been reinitialized or destroyed, freeing the pointer in the
     * cursor.  Keep the cache locked while we copy the creds and advance the
     * pointer, in case another thread reinitializes the cache after we check
     * the generation.
     */
    k5_cc_mutex_lock(context, &mut (*d).lock);
    if (*mcursor).generation != (*d).generation {
        retval = -(1765328242 as libc::c_long) as krb5_error_code
    } else {
        /* Skip over removed creds. */
        while !(*mcursor).next_link.is_null() &&
                  (*(*mcursor).next_link).creds.is_null() {
            (*mcursor).next_link = (*(*mcursor).next_link).next
        }
        if (*mcursor).next_link.is_null() {
            retval = -(1765328242 as libc::c_long) as krb5_error_code
        } else {
            retval =
                k5_copy_creds_contents(context, (*(*mcursor).next_link).creds,
                                       creds);
            if retval == 0 as libc::c_int {
                (*mcursor).next_link = (*(*mcursor).next_link).next
            }
        }
    }
    k5_cc_mutex_unlock(context, &mut (*d).lock);
    return retval;
}
/*
 * Requires:
 * cursor is a krb5_cc_cursor originally obtained from
 * krb5_mcc_start_seq_get.
 *
 * Modifies:
 * id, cursor
 *
 * Effects:
 * Finishes sequential processing of the memory credentials ccache id,
 * and invalidates the cursor (it must never be used after this call).
 */
/* ARGSUSED */
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn krb5_mcc_end_seq_get(mut context: krb5_context,
                                          mut id: krb5_ccache,
                                          mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    free(*cursor);
    *cursor = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
/*
 * Requires:
 * residual is a legal path name, and a null-terminated string
 *
 * Modifies:
 * id
 *
 * Effects:
 * creates or accesses a memory-based cred cache that is referenced by
 * residual.
 *
 * Returns:
 * A filled in krb5_ccache structure "id".
 *
 * Errors:
 * KRB5_CC_NOMEM - there was insufficient memory to allocate the
 *              krb5_ccache.  id is undefined.
 * system errors (mutex locks related)
 */
/*
 * Utility routine: Creates the back-end data for a memory cache, and adds it
 * to the global table.  Give the new object two references, one for the table
 * slot and one for the caller's handle.
 *
 * Call with the global table lock held.
 */
#[c2rust::src_loc = "450:1"]
unsafe extern "C" fn new_mcc_data(mut name: *const libc::c_char,
                                  mut dataptr: *mut *mut krb5_mcc_data)
 -> krb5_error_code {
    let mut err: krb5_error_code = 0;
    let mut d: *mut krb5_mcc_data = 0 as *mut krb5_mcc_data;
    d =
        malloc(::std::mem::size_of::<krb5_mcc_data>() as libc::c_ulong) as
            *mut krb5_mcc_data;
    if d.is_null() { return -(1765328186 as libc::c_long) as krb5_error_code }
    err = k5_cc_mutex_init(&mut (*d).lock);
    if err != 0 { free(d as *mut libc::c_void); return err }
    (*d).name = strdup(name);
    if (*d).name.is_null() {
        k5_os_mutex_destroy(&mut (*d).lock.lock);
        free(d as *mut libc::c_void);
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    (*d).link = 0 as *mut krb5_mcc_link;
    (*d).prin = 0 as krb5_principal;
    (*d).time_offset = 0 as libc::c_int;
    (*d).usec_offset = 0 as libc::c_int;
    (*d).refcount = 2 as libc::c_int;
    (*d).generation = 0 as libc::c_int;
    if k5_hashtab_add(mcc_hashtab, (*d).name as *const libc::c_void,
                      strlen((*d).name), d as *mut libc::c_void) !=
           0 as libc::c_int {
        free((*d).name as *mut libc::c_void);
        k5_os_mutex_destroy(&mut (*d).lock.lock);
        free(d as *mut libc::c_void);
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    *dataptr = d;
    return 0 as libc::c_int;
}
/*
 * Effects:
 * Creates a new memory cred cache whose name is guaranteed to be
 * unique.  The name begins with the string TKT_ROOT (from mcc.h).
 *
 * Returns:
 * The filled in krb5_ccache id.
 *
 * Errors:
 * KRB5_CC_NOMEM - there was insufficient memory to allocate the
 *              krb5_ccache.  id is undefined.
 * system errors (from open, mutex locking)
 */
#[c2rust::src_loc = "504:1"]
unsafe extern "C" fn krb5_mcc_generate_new(mut context: krb5_context,
                                           mut id: *mut krb5_ccache)
 -> krb5_error_code {
    let mut lid: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut uniquename: [libc::c_char; 8] = [0; 8];
    let mut err: krb5_error_code = 0;
    let mut d: *mut krb5_mcc_data = 0 as *mut krb5_mcc_data;
    /* Allocate memory */
    lid =
        malloc(::std::mem::size_of::<_krb5_ccache>() as libc::c_ulong) as
            krb5_ccache;
    if lid.is_null() {
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    (*lid).ops = &krb5_mcc_ops;
    k5_cc_mutex_lock(context, &mut krb5int_mcc_mutex);
    init_table(context);
    loop 
         /* Check for uniqueness with mutex locked to avoid race conditions */
         {
        err =
            krb5int_random_string(context, uniquename.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 8]>()
                                      as libc::c_ulong as libc::c_uint);
        if err != 0 {
            k5_cc_mutex_unlock(context, &mut krb5int_mcc_mutex);
            free(lid as *mut libc::c_void);
            return err
        }
        if k5_hashtab_get(mcc_hashtab,
                          uniquename.as_mut_ptr() as *const libc::c_void,
                          strlen(uniquename.as_mut_ptr())).is_null() {
            break ;
        }
    }
    err = new_mcc_data(uniquename.as_mut_ptr(), &mut d);
    k5_cc_mutex_unlock(context, &mut krb5int_mcc_mutex);
    if err != 0 { free(lid as *mut libc::c_void); return err }
    (*lid).data = d as krb5_pointer;
    *id = lid;
    krb5_change_cache();
    return 0 as libc::c_int;
}
/*
 * Requires:
 * id is a file credential cache
 *
 * Returns:
 * A pointer to the name of the file cred cache id.
 */
#[c2rust::src_loc = "556:1"]
unsafe extern "C" fn krb5_mcc_get_name(mut context: krb5_context,
                                       mut id: krb5_ccache)
 -> *const libc::c_char {
    return (*((*id).data as *mut krb5_mcc_data)).name;
}
/*
 * Modifies:
 * id, princ
 *
 * Effects:
 * Retrieves the primary principal from id, as set with
 * krb5_mcc_initialize.  The principal is returned is allocated
 * storage that must be freed by the caller via krb5_free_principal.
 *
 * Errors:
 * system errors
 * ENOMEM
 */
#[c2rust::src_loc = "575:1"]
unsafe extern "C" fn krb5_mcc_get_principal(mut context: krb5_context,
                                            mut id: krb5_ccache,
                                            mut princ: *mut krb5_principal)
 -> krb5_error_code {
    let mut ptr: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    if (*ptr).prin.is_null() {
        *princ = 0 as krb5_principal;
        return -(1765328189 as libc::c_long) as krb5_error_code
    }
    return krb5_copy_principal(context, (*ptr).prin as krb5_const_principal,
                               princ);
}
#[c2rust::src_loc = "586:1"]
unsafe extern "C" fn krb5_mcc_retrieve(mut context: krb5_context,
                                       mut id: krb5_ccache,
                                       mut whichfields: krb5_flags,
                                       mut mcreds: *mut krb5_creds,
                                       mut creds: *mut krb5_creds)
 -> krb5_error_code {
    return k5_cc_retrieve_cred_default(context, id, whichfields, mcreds,
                                       creds);
}
/*
 * Modifies:
 * the memory cache
 *
 * Effects:
 * Remove the given creds from the ccache.
 */
#[c2rust::src_loc = "601:1"]
unsafe extern "C" fn krb5_mcc_remove_cred(mut context: krb5_context,
                                          mut cache: krb5_ccache,
                                          mut flags: krb5_flags,
                                          mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut data: *mut krb5_mcc_data = (*cache).data as *mut krb5_mcc_data;
    let mut l: *mut krb5_mcc_link = 0 as *mut krb5_mcc_link;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    l = (*data).link;
    while !l.is_null() {
        if !(*l).creds.is_null() &&
               krb5int_cc_creds_match_request(context, flags, creds,
                                              (*l).creds) != 0 {
            krb5_free_creds(context, (*l).creds);
            (*l).creds = 0 as *mut krb5_creds
        }
        l = (*l).next
    }
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return 0 as libc::c_int;
}
/*
 * Requires:
 * id is a cred cache returned by krb5_mcc_resolve or
 * krb5_mcc_generate_new.
 *
 * Modifies:
 * id
 *
 * Effects:
 * Sets the operational flags of id to flags.
 */
#[c2rust::src_loc = "634:1"]
unsafe extern "C" fn krb5_mcc_set_flags(mut context: krb5_context,
                                        mut id: krb5_ccache,
                                        mut flags: krb5_flags)
 -> krb5_error_code {
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "640:1"]
unsafe extern "C" fn krb5_mcc_get_flags(mut context: krb5_context,
                                        mut id: krb5_ccache,
                                        mut flags: *mut krb5_flags)
 -> krb5_error_code {
    *flags = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/*
 * Modifies:
 * the memory cache
 *
 * Effects:
 * Save away creds in the ccache.
 *
 * Errors:
 * system errors (mutex locking)
 * ENOMEM
 */
#[c2rust::src_loc = "658:1"]
unsafe extern "C" fn krb5_mcc_store(mut ctx: krb5_context,
                                    mut id: krb5_ccache,
                                    mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut err: krb5_error_code = 0;
    let mut new_node: *mut krb5_mcc_link = 0 as *mut krb5_mcc_link;
    let mut mptr: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    new_node =
        malloc(::std::mem::size_of::<krb5_mcc_link>() as libc::c_ulong) as
            *mut krb5_mcc_link;
    if new_node.is_null() { return 12 as libc::c_int }
    err = krb5_copy_creds(ctx, creds, &mut (*new_node).creds);
    if err != 0 {
        free(new_node as *mut libc::c_void);
        return err
    } else {
        k5_cc_mutex_lock(ctx, &mut (*mptr).lock);
        (*new_node).next = (*mptr).link;
        (*mptr).link = new_node;
        k5_cc_mutex_unlock(ctx, &mut (*mptr).lock);
        return 0 as libc::c_int
    };
}
#[c2rust::src_loc = "681:1"]
unsafe extern "C" fn krb5_mcc_ptcursor_new(mut context: krb5_context,
                                           mut cursor: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut n: krb5_cc_ptcursor = 0 as krb5_cc_ptcursor;
    let mut cdata: *mut krb5_mcc_ptcursor_data =
        0 as *mut krb5_mcc_ptcursor_data;
    *cursor = 0 as krb5_cc_ptcursor;
    n =
        malloc(::std::mem::size_of::<krb5_cc_ptcursor_s>() as libc::c_ulong)
            as krb5_cc_ptcursor;
    if n.is_null() { return 12 as libc::c_int }
    (*n).ops = &krb5_mcc_ops;
    cdata =
        malloc(::std::mem::size_of::<krb5_mcc_ptcursor_data>() as
                   libc::c_ulong) as *mut krb5_mcc_ptcursor_data;
    if cdata.is_null() {
        free(n as *mut libc::c_void);
        return 12 as libc::c_int
    }
    (*n).data = cdata as krb5_pointer;
    (*cdata).first = 1 as libc::c_int as krb5_boolean;
    *cursor = n;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "706:1"]
unsafe extern "C" fn krb5_mcc_ptcursor_next(mut context: krb5_context,
                                            mut cursor: krb5_cc_ptcursor,
                                            mut ccache: *mut krb5_ccache)
 -> krb5_error_code {
    let mut cdata: *mut krb5_mcc_ptcursor_data =
        0 as *mut krb5_mcc_ptcursor_data;
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    *ccache = 0 as krb5_ccache;
    cdata = (*cursor).data as *mut krb5_mcc_ptcursor_data;
    if (*cdata).first == 0 { return 0 as libc::c_int }
    (*cdata).first = 0 as libc::c_int as krb5_boolean;
    defname = krb5_cc_default_name(context);
    if defname.is_null() ||
           strncmp(defname,
                   b"MEMORY:\x00" as *const u8 as *const libc::c_char,
                   7 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return krb5_cc_resolve(context, defname, ccache);
}
#[c2rust::src_loc = "728:1"]
unsafe extern "C" fn krb5_mcc_ptcursor_free(mut context: krb5_context,
                                            mut cursor: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    if (*cursor).is_null() { return 0 as libc::c_int }
    if !(**cursor).data.is_null() { free((**cursor).data); }
    free(*cursor as *mut libc::c_void);
    *cursor = 0 as krb5_cc_ptcursor;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "742:1"]
unsafe extern "C" fn krb5_mcc_lock(mut context: krb5_context,
                                   mut id: krb5_ccache) -> krb5_error_code {
    let mut data: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "751:1"]
unsafe extern "C" fn krb5_mcc_unlock(mut context: krb5_context,
                                     mut id: krb5_ccache) -> krb5_error_code {
    let mut data: *mut krb5_mcc_data = (*id).data as *mut krb5_mcc_data;
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "760:19"]
pub static mut krb5_mcc_ops: krb5_cc_ops =
    unsafe {
        {
            let mut init =
                _krb5_cc_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"MEMORY\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             get_name:
                                 Some(krb5_mcc_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> *const libc::c_char),
                             resolve:
                                 Some(krb5_mcc_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache,
                                                               _:
                                                                   *const libc::c_char)
                                              -> krb5_error_code),
                             gen_new:
                                 Some(krb5_mcc_generate_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             init:
                                 Some(krb5_mcc_initialize as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   krb5_principal)
                                              -> krb5_error_code),
                             destroy:
                                 Some(krb5_mcc_destroy as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             close:
                                 Some(krb5_mcc_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             store:
                                 Some(krb5_mcc_store as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             retrieve:
                                 Some(krb5_mcc_retrieve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags,
                                                               _:
                                                                   *mut krb5_creds,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             get_princ:
                                 Some(krb5_mcc_get_principal as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_principal)
                                              -> krb5_error_code),
                             get_first:
                                 Some(krb5_mcc_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(krb5_mcc_next_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             end_get:
                                 Some(krb5_mcc_end_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             remove_cred:
                                 Some(krb5_mcc_remove_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             set_flags:
                                 Some(krb5_mcc_set_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags)
                                              -> krb5_error_code),
                             get_flags:
                                 Some(krb5_mcc_get_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_flags)
                                              -> krb5_error_code),
                             ptcursor_new:
                                 Some(krb5_mcc_ptcursor_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             ptcursor_next:
                                 Some(krb5_mcc_ptcursor_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_cc_ptcursor,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             ptcursor_free:
                                 Some(krb5_mcc_ptcursor_free as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             move_0: None,
                             wasdefault: None,
                             lock:
                                 Some(krb5_mcc_lock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             unlock:
                                 Some(krb5_mcc_unlock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             switch_to: None,};
            init
        }
    };
