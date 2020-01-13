use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:97"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:97"]
pub mod sys_types_h {
    #[c2rust::src_loc = "79:1"]
    pub type uid_t = __uid_t;
    use super::types_h::__uid_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:97"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:97"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:97"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:97"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:97"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:97"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:97"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:97"]
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
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "4767:1"]
        pub fn krb5_free_unparsed_name(context: krb5_context,
                                       val: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:97"]
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
    #[c2rust::src_loc = "702:1"]
    pub type krb5_os_context = *mut _krb5_os_context;
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
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
    }
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::string_h::memcpy;
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
        #[c2rust::src_loc = "1912:1"]
        pub fn krb5int_random_string(_: krb5_context,
                                     string: *mut libc::c_char,
                                     length: libc::c_uint) -> krb5_error_code;
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:97"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:97"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:97"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/ccache/cc-int.h:97"]
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
                        krb5_int32, krb5_boolean, krb5_principal_data};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
    use super::k5_buf_h::k5buf;
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
        #[c2rust::src_loc = "93:1"]
        pub fn k5_cc_mutex_assert_locked(context: krb5_context,
                                         m: *mut k5_cc_mutex);
        #[no_mangle]
        #[c2rust::src_loc = "99:1"]
        pub fn k5_cc_mutex_lock(context: krb5_context, m: *mut k5_cc_mutex);
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn k5_cc_mutex_unlock(context: krb5_context, m: *mut k5_cc_mutex);
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn k5_unmarshal_cred(data: *const libc::c_uchar, len: size_t,
                                 version: libc::c_int, creds: *mut krb5_creds)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn k5_unmarshal_princ(data: *const libc::c_uchar, len: size_t,
                                  version: libc::c_int,
                                  princ_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "158:1"]
        pub fn k5_marshal_cred(buf: *mut k5buf, version: libc::c_int,
                               creds: *mut krb5_creds);
        #[no_mangle]
        #[c2rust::src_loc = "164:1"]
        pub fn k5_marshal_princ(buf: *mut k5buf, version: libc::c_int,
                                princ: krb5_principal);
    }
    /* __KRB5_CCACHE_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:97"]
pub mod k5_buf_h {
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
        /* Initialize a k5buf using an internally allocated dynamic buffer, zeroing
 * memory when reallocating or freeing. */
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn k5_buf_init_dynamic_zap(buf: *mut k5buf);
        /* Return ENOMEM if buf is in an error state, 0 otherwise. */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> libc::c_int;
        /*
 * Free the storage used in the dynamic buffer BUF.  The caller may choose to
 * take responsibility for freeing the data pointer instead of using this
 * function.  If BUF is a fixed buffer, an assertion failure will result.
 * Freeing a buffer in the error state, a buffer initialized with EMPTY_K5BUF,
 * or a zeroed k5buf structure is a no-op.
 */
        #[no_mangle]
        #[c2rust::src_loc = "109:1"]
        pub fn k5_buf_free(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/keyutils.h:102"]
pub mod keyutils_h {
    #[c2rust::src_loc = "22:1"]
    pub type key_serial_t = int32_t;
    use super::stdint_intn_h::int32_t;
    use super::stddef_h::size_t;
    use super::sys_types_h::uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "169:1"]
        pub fn add_key(type_0: *const libc::c_char,
                       description: *const libc::c_char,
                       payload: *const libc::c_void, plen: size_t,
                       ringid: key_serial_t) -> key_serial_t;
        #[no_mangle]
        #[c2rust::src_loc = "185:1"]
        pub fn keyctl_get_keyring_ID(id: key_serial_t, create: libc::c_int)
         -> key_serial_t;
        #[no_mangle]
        #[c2rust::src_loc = "192:1"]
        pub fn keyctl_clear(ringid: key_serial_t) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "193:1"]
        pub fn keyctl_link(id: key_serial_t, ringid: key_serial_t)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "194:1"]
        pub fn keyctl_unlink(id: key_serial_t, ringid: key_serial_t)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn keyctl_search(ringid: key_serial_t,
                             type_0: *const libc::c_char,
                             description: *const libc::c_char,
                             destringid: key_serial_t) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn keyctl_set_timeout(key: key_serial_t, timeout: libc::c_uint)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn keyctl_get_persistent(uid: uid_t, id: key_serial_t)
         -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn keyctl_describe_alloc(id: key_serial_t,
                                     _buffer: *mut *mut libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "257:1"]
        pub fn keyctl_read_alloc(id: key_serial_t,
                                 _buffer: *mut *mut libc::c_void)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:97"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "176:17"]
        pub fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
                      _: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:97"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:97"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:97"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn geteuid() -> __uid_t;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:97"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:97"]
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
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:97"]
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
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __uid_t};
pub use self::sys_types_h::uid_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_os_mutex_destroy};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be};
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
                       krb5_unparse_name, krb5_cc_default_name,
                       krb5_free_cred_contents, krb5_free_unparsed_name,
                       krb5_timeofday, krb5_set_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_os_context, k5calloc, k5alloc,
                         k5memdup0, ts_delta, ts_after, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
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
                         k5_cc_mutex_assert_locked, k5_cc_mutex_lock,
                         k5_cc_mutex_unlock, k5_unmarshal_cred,
                         k5_unmarshal_princ, k5_marshal_cred,
                         k5_marshal_princ};
pub use self::k5_buf_h::{k5buftype, K5BUF_DYNAMIC_ZAP, K5BUF_DYNAMIC,
                         K5BUF_FIXED, K5BUF_ERROR, k5buf, k5_buf_init_dynamic,
                         k5_buf_init_dynamic_zap, k5_buf_status, k5_buf_free};
pub use self::keyutils_h::{key_serial_t, add_key, keyctl_get_keyring_ID,
                           keyctl_clear, keyctl_link, keyctl_unlink,
                           keyctl_search, keyctl_set_timeout,
                           keyctl_get_persistent, keyctl_describe_alloc,
                           keyctl_read_alloc};
use self::stdlib_h::{strtol, malloc, calloc, free};
use self::stdio_h::asprintf;
use self::errno_h::__errno_location;
use self::unistd_h::geteuid;
use self::libintl_h::dgettext;
use self::string_h::{strlen, strrchr, strchr, strdup, strncmp, strcmp, memset,
                     memcpy};
pub use self::byteswap_h::__bswap_32;
extern "C" {
    /* Note the following is a stub function for Linux */
    #[no_mangle]
    #[c2rust::src_loc = "278:1"]
    pub fn krb5_change_cache() -> krb5_error_code;
}
/*
 * This represents a credentials cache "file"
 * where cache_id is the keyring serial number for
 * this credentials cache "file".  Each key
 * in the keyring contains a separate key.
 */
#[c2rust::src_loc = "226:1"]
pub type krcc_data = _krcc_data;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "226:16"]
pub struct _krcc_data {
    pub name: *mut libc::c_char,
    pub lock: k5_cc_mutex,
    pub collection_id: key_serial_t,
    pub cache_id: key_serial_t,
    pub princ_id: key_serial_t,
    pub is_legacy_type: krb5_boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1492:8"]
pub struct krcc_ptcursor_data {
    pub collection_id: key_serial_t,
    pub anchor_name: *mut libc::c_char,
    pub collection_name: *mut libc::c_char,
    pub subsidiary_name: *mut libc::c_char,
    pub primary_name: *mut libc::c_char,
    pub first: krb5_boolean,
    pub num_keys: libc::c_long,
    pub next_key: libc::c_long,
    pub keys: *mut key_serial_t,
}
#[c2rust::src_loc = "211:1"]
pub type krcc_cursor = *mut _krcc_cursor;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "211:16"]
pub struct _krcc_cursor {
    pub numkeys: libc::c_int,
    pub currkey: libc::c_int,
    pub princ_id: key_serial_t,
    pub offsets_id: key_serial_t,
    pub keys: *mut key_serial_t,
}
/* Name for this credentials cache */
/* synchronization */
/* collection containing this cache keyring */
/* keyring representing ccache */
/* key holding principal info */
/* Global mutex */
#[no_mangle]
#[c2rust::src_loc = "237:13"]
pub static mut krb5int_krcc_mutex: k5_cc_mutex =
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
/*
 * GET_PERSISTENT(uid) acquires the persistent keyring for uid, or falls back
 * to the user keyring if uid matches the current effective uid.
 */
#[c2rust::src_loc = "285:1"]
unsafe extern "C" fn get_persistent_fallback(mut uid: uid_t) -> key_serial_t {
    return if uid == geteuid() {
               -(4 as libc::c_int)
           } else { -(1 as libc::c_int) };
}
#[c2rust::src_loc = "293:1"]
unsafe extern "C" fn get_persistent_real(mut uid: uid_t) -> key_serial_t {
    let mut key: key_serial_t = 0;
    key = keyctl_get_persistent(uid, -(2 as libc::c_int)) as key_serial_t;
    return if key == -(1 as libc::c_int) &&
                  *__errno_location() == 95 as libc::c_int {
               get_persistent_fallback(uid)
           } else { key };
}
/*
 * If a process has no explicitly set session keyring, KEY_SPEC_SESSION_KEYRING
 * will resolve to the user session keyring for ID lookup and reading, but in
 * some kernel versions, writing to that special keyring will instead create a
 * new empty session keyring for the process.  We do not want that; the keys we
 * create would be invisible to other processes.  We can work around that
 * behavior by explicitly writing to the user session keyring when it matches
 * the session keyring.  This function returns the keyring we should write to
 * for the session anchor.
 */
#[c2rust::src_loc = "316:1"]
unsafe extern "C" fn session_write_anchor() -> key_serial_t {
    let mut s: key_serial_t = 0;
    let mut u: key_serial_t = 0;
    s = keyctl_get_keyring_ID(-(3 as libc::c_int), 0 as libc::c_int);
    u = keyctl_get_keyring_ID(-(5 as libc::c_int), 0 as libc::c_int);
    return if s == u { -(5 as libc::c_int) } else { -(3 as libc::c_int) };
}
/*
 * Find or create a keyring within parent with the given name.  If possess is
 * nonzero, also make sure the key is linked from possess.  This is necessary
 * to ensure that we have possession rights on the key when the parent is the
 * user or persistent keyring.
 */
#[c2rust::src_loc = "332:1"]
unsafe extern "C" fn find_or_create_keyring(mut parent: key_serial_t,
                                            mut possess: key_serial_t,
                                            mut name: *const libc::c_char,
                                            mut key_out: *mut key_serial_t)
 -> krb5_error_code {
    let mut key: key_serial_t = 0;
    *key_out = -(1 as libc::c_int);
    key =
        keyctl_search(parent,
                      b"keyring\x00" as *const u8 as *const libc::c_char,
                      name, possess) as key_serial_t;
    if key == -(1 as libc::c_int) {
        if possess != 0 as libc::c_int {
            key =
                add_key(b"keyring\x00" as *const u8 as *const libc::c_char,
                        name, 0 as *const libc::c_void,
                        0 as libc::c_int as size_t, possess);
            if key == -(1 as libc::c_int) { return *__errno_location() }
            if keyctl_link(key, parent) == -(1 as libc::c_int) as libc::c_long
               {
                return *__errno_location()
            }
        } else {
            key =
                add_key(b"keyring\x00" as *const u8 as *const libc::c_char,
                        name, 0 as *const libc::c_void,
                        0 as libc::c_int as size_t, parent);
            if key == -(1 as libc::c_int) { return *__errno_location() }
        }
    }
    *key_out = key;
    return 0 as libc::c_int;
}
/* Parse a residual name into an anchor name, a collection name, and possibly a
 * subsidiary name. */
#[c2rust::src_loc = "359:1"]
unsafe extern "C" fn parse_residual(mut residual: *const libc::c_char,
                                    mut anchor_name_out:
                                        *mut *mut libc::c_char,
                                    mut collection_name_out:
                                        *mut *mut libc::c_char,
                                    mut subsidiary_name_out:
                                        *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut anchor_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collection_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subsidiary_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    *anchor_name_out = 0 as *mut libc::c_char;
    *collection_name_out = 0 as *mut libc::c_char;
    *subsidiary_name_out = 0 as *mut libc::c_char;
    /* Parse out the anchor name.  Use the legacy anchor if not present. */
    sep = strchr(residual, ':' as i32);
    if sep.is_null() {
        anchor_name =
            strdup(b"legacy\x00" as *const u8 as *const libc::c_char);
        if anchor_name.is_null() {
            current_block = 7874591680840749080;
        } else { current_block = 7651349459974463963; }
    } else {
        anchor_name =
            k5memdup0(residual as *const libc::c_void,
                      sep.wrapping_offset_from(residual) as libc::c_long as
                          size_t, &mut ret) as *mut libc::c_char;
        if anchor_name.is_null() {
            current_block = 7874591680840749080;
        } else {
            residual = sep.offset(1 as libc::c_int as isize);
            current_block = 7651349459974463963;
        }
    }
    match current_block {
        7651349459974463963 => {
            /* Parse out the collection and subsidiary name. */
            sep = strchr(residual, ':' as i32);
            if sep.is_null() {
                collection_name = strdup(residual);
                if collection_name.is_null() {
                    current_block = 7874591680840749080;
                } else {
                    subsidiary_name = 0 as *mut libc::c_char;
                    current_block = 2668756484064249700;
                }
            } else {
                collection_name =
                    k5memdup0(residual as *const libc::c_void,
                              sep.wrapping_offset_from(residual) as
                                  libc::c_long as size_t, &mut ret) as
                        *mut libc::c_char;
                if collection_name.is_null() {
                    current_block = 7874591680840749080;
                } else {
                    subsidiary_name =
                        strdup(sep.offset(1 as libc::c_int as isize));
                    if subsidiary_name.is_null() {
                        current_block = 7874591680840749080;
                    } else { current_block = 2668756484064249700; }
                }
            }
            match current_block {
                7874591680840749080 => { }
                _ => {
                    *anchor_name_out = anchor_name;
                    *collection_name_out = collection_name;
                    *subsidiary_name_out = subsidiary_name;
                    return 0 as libc::c_int
                }
            }
        }
        _ => { }
    }
    free(anchor_name as *mut libc::c_void);
    free(collection_name as *mut libc::c_void);
    free(subsidiary_name as *mut libc::c_void);
    return 12 as libc::c_int;
}
/*
 * Return true if residual identifies a subsidiary cache which should be linked
 * into the anchor so it can be visible to old code.  This is the case if the
 * residual has the legacy anchor and the subsidiary name matches the
 * collection name.
 */
#[c2rust::src_loc = "418:1"]
unsafe extern "C" fn is_legacy_cache_name(mut residual: *const libc::c_char)
 -> krb5_boolean {
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut aname: *const libc::c_char = 0 as *const libc::c_char;
    let mut cname: *const libc::c_char = 0 as *const libc::c_char;
    let mut sname: *const libc::c_char = 0 as *const libc::c_char;
    let mut alen: size_t = 0;
    let mut clen: size_t = 0;
    let mut legacy_len: size_t =
        (::std::mem::size_of::<[libc::c_char; 7]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    /* Get pointers to the anchor, collection, and subsidiary names. */
    aname = residual;
    sep = strchr(residual, ':' as i32);
    if sep.is_null() { return 0 as libc::c_int as krb5_boolean }
    alen = sep.wrapping_offset_from(aname) as libc::c_long as size_t;
    cname = sep.offset(1 as libc::c_int as isize);
    sep = strchr(cname, ':' as i32);
    if sep.is_null() { return 0 as libc::c_int as krb5_boolean }
    clen = sep.wrapping_offset_from(cname) as libc::c_long as size_t;
    sname = sep.offset(1 as libc::c_int as isize);
    return (alen == legacy_len && clen == strlen(sname) &&
                strncmp(aname,
                        b"legacy\x00" as *const u8 as *const libc::c_char,
                        alen) == 0 as libc::c_int &&
                strncmp(cname, sname, clen) == 0 as libc::c_int) as
               libc::c_int as krb5_boolean;
}
/* If the default cache name for context is a KEYRING cache, parse its residual
 * string.  Otherwise set all outputs to NULL. */
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn get_default(mut context: krb5_context,
                                 mut anchor_name_out: *mut *mut libc::c_char,
                                 mut collection_name_out:
                                     *mut *mut libc::c_char,
                                 mut subsidiary_name_out:
                                     *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut defname: *const libc::c_char = 0 as *const libc::c_char;
    *subsidiary_name_out = 0 as *mut libc::c_char;
    *collection_name_out = *subsidiary_name_out;
    *anchor_name_out = *collection_name_out;
    defname = krb5_cc_default_name(context);
    if defname.is_null() ||
           strncmp(defname,
                   b"KEYRING:\x00" as *const u8 as *const libc::c_char,
                   8 as libc::c_int as libc::c_ulong) != 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return parse_residual(defname.offset(8 as libc::c_int as isize),
                          anchor_name_out, collection_name_out,
                          subsidiary_name_out);
}
/* Create a residual identifying a subsidiary cache. */
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn make_subsidiary_residual(mut anchor_name:
                                                  *const libc::c_char,
                                              mut collection_name:
                                                  *const libc::c_char,
                                              mut subsidiary_name:
                                                  *const libc::c_char,
                                              mut residual_out:
                                                  *mut *mut libc::c_char)
 -> krb5_error_code {
    if asprintf(residual_out,
                b"%s:%s:%s\x00" as *const u8 as *const libc::c_char,
                anchor_name, collection_name, subsidiary_name) <
           0 as libc::c_int {
        *residual_out = 0 as *mut libc::c_char;
        return 12 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Retrieve or create a keyring for collection_name within the anchor, and set
 * *collection_id_out to its serial number. */
#[c2rust::src_loc = "473:1"]
unsafe extern "C" fn get_collection(mut anchor_name: *const libc::c_char,
                                    mut collection_name: *const libc::c_char,
                                    mut collection_id_out: *mut key_serial_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut persistent_id: key_serial_t = 0;
    let mut anchor_id: key_serial_t = 0;
    let mut possess_id: key_serial_t = 0 as libc::c_int;
    let mut ckname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cnend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uidnum: libc::c_long = 0;
    *collection_id_out = 0 as libc::c_int;
    if strcmp(anchor_name,
              b"persistent\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        /*
         * The collection name is a uid (or empty for the current effective
         * uid), and we look up a fixed keyring name within the persistent
         * keyring for that uid.  We link it to the process keyring to ensure
         * that we have possession rights on the collection key.
         */
        if *collection_name as libc::c_int != '\u{0}' as i32 {
            *__errno_location() = 0 as libc::c_int;
            uidnum = strtol(collection_name, &mut cnend, 10 as libc::c_int);
            if *__errno_location() != 0 ||
                   *cnend as libc::c_int != '\u{0}' as i32 {
                return -(1750600185 as libc::c_long) as krb5_error_code
            }
        } else { uidnum = geteuid() as libc::c_long }
        persistent_id = get_persistent_real(uidnum as uid_t);
        if persistent_id == -(1 as libc::c_int) {
            return -(1750600185 as libc::c_long) as krb5_error_code
        }
        return find_or_create_keyring(persistent_id, -(2 as libc::c_int),
                                      b"_krb\x00" as *const u8 as
                                          *const libc::c_char,
                                      collection_id_out)
    }
    if strcmp(anchor_name, b"process\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        anchor_id = -(2 as libc::c_int)
    } else if strcmp(anchor_name,
                     b"thread\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        anchor_id = -(1 as libc::c_int)
    } else if strcmp(anchor_name,
                     b"session\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        anchor_id = session_write_anchor()
    } else if strcmp(anchor_name,
                     b"user\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        /* The user keyring does not confer possession, so we need to link the
         * collection to the process keyring to maintain possession rights. */
        anchor_id = -(4 as libc::c_int);
        possess_id = -(2 as libc::c_int)
    } else if strcmp(anchor_name,
                     b"legacy\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        anchor_id = session_write_anchor()
    } else { return -(1750600187 as libc::c_long) as krb5_error_code }
    /* Look up the collection keyring name within the anchor keyring. */
    if asprintf(&mut ckname as *mut *mut libc::c_char,
                b"%s%s\x00" as *const u8 as *const libc::c_char,
                b"_krb_\x00" as *const u8 as *const libc::c_char,
                collection_name) == -(1 as libc::c_int) {
        return 12 as libc::c_int
    }
    ret =
        find_or_create_keyring(anchor_id, possess_id, ckname,
                               collection_id_out);
    free(ckname as *mut libc::c_void);
    return ret;
}
/* Store subsidiary_name into the primary index key for collection_id. */
#[c2rust::src_loc = "534:1"]
unsafe extern "C" fn set_primary_name(mut context: krb5_context,
                                      mut collection_id: key_serial_t,
                                      mut subsidiary_name:
                                          *const libc::c_char)
 -> krb5_error_code {
    let mut key: key_serial_t = 0;
    let mut len: uint32_t = strlen(subsidiary_name) as uint32_t;
    let mut plen: uint32_t =
        (8 as libc::c_int as libc::c_uint).wrapping_add(len);
    let mut payload: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    payload = malloc(plen as libc::c_ulong) as *mut libc::c_uchar;
    if payload.is_null() { return 12 as libc::c_int }
    store_32_be(1 as libc::c_int as libc::c_uint,
                payload as *mut libc::c_void);
    store_32_be(len,
                payload.offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    memcpy(payload.offset(8 as libc::c_int as isize) as *mut libc::c_void,
           subsidiary_name as *const libc::c_void, len as libc::c_ulong);
    key =
        add_key(b"user\x00" as *const u8 as *const libc::c_char,
                b"krb_ccache:primary\x00" as *const u8 as *const libc::c_char,
                payload as *const libc::c_void, plen as size_t,
                collection_id);
    free(payload as *mut libc::c_void);
    return if key == -(1 as libc::c_int) {
               *__errno_location()
           } else { 0 as libc::c_int };
}
#[c2rust::src_loc = "554:1"]
unsafe extern "C" fn parse_index(mut context: krb5_context,
                                 mut version: *mut int32_t,
                                 mut primary: *mut *mut libc::c_char,
                                 mut payload: *const libc::c_uchar,
                                 mut psize: size_t) -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut len: uint32_t = 0;
    if psize < 8 as libc::c_int as libc::c_ulong {
        return -(1765328242 as libc::c_long) as krb5_error_code
    }
    *version = load_32_be(payload as *const libc::c_void) as int32_t;
    len =
        load_32_be(payload.offset(4 as libc::c_int as isize) as
                       *const libc::c_void);
    if len as libc::c_ulong >
           psize.wrapping_sub(8 as libc::c_int as libc::c_ulong) {
        return -(1765328242 as libc::c_long) as krb5_error_code
    }
    *primary =
        k5memdup0(payload.offset(8 as libc::c_int as isize) as
                      *const libc::c_void, len as size_t, &mut ret) as
            *mut libc::c_char;
    return if (*primary).is_null() { ret } else { 0 as libc::c_int };
}
/*
 * Get or initialize the primary name within collection_id and set
 * *subsidiary_out to its value.  If initializing a legacy collection, look
 * for a legacy cache and add it to the collection.
 */
#[c2rust::src_loc = "577:1"]
unsafe extern "C" fn get_primary_name(mut context: krb5_context,
                                      mut anchor_name: *const libc::c_char,
                                      mut collection_name:
                                          *const libc::c_char,
                                      mut collection_id: key_serial_t,
                                      mut subsidiary_out:
                                          *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut primary_id: key_serial_t = 0;
    let mut legacy: key_serial_t = 0;
    let mut payload: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut payloadlen: libc::c_int = 0;
    let mut version: int32_t = 0;
    let mut subsidiary_name: *mut libc::c_char = 0 as *mut libc::c_char;
    *subsidiary_out = 0 as *mut libc::c_char;
    primary_id =
        keyctl_search(collection_id,
                      b"user\x00" as *const u8 as *const libc::c_char,
                      b"krb_ccache:primary\x00" as *const u8 as
                          *const libc::c_char, 0 as libc::c_int) as
            key_serial_t;
    if primary_id == -(1 as libc::c_int) {
        /* Initialize the primary key using the collection name.  We can't name
         * a key with the empty string, so map that to an arbitrary string. */
        subsidiary_name =
            strdup(if *collection_name as libc::c_int == '\u{0}' as i32 {
                       b"tkt\x00" as *const u8 as *const libc::c_char
                   } else { collection_name });
        if subsidiary_name.is_null() {
            ret = 12 as libc::c_int;
            current_block = 8522321847195001863;
        } else {
            ret = set_primary_name(context, collection_id, subsidiary_name);
            if ret != 0 {
                current_block = 8522321847195001863;
            } else if strcmp(anchor_name,
                             b"legacy\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
                /* Look for a cache created by old code.  If we find one, add it to
             * the collection. */
                legacy =
                    keyctl_search(-(3 as libc::c_int),
                                  b"keyring\x00" as *const u8 as
                                      *const libc::c_char, subsidiary_name,
                                  0 as libc::c_int) as key_serial_t;
                if legacy != -(1 as libc::c_int) &&
                       keyctl_link(legacy, collection_id) ==
                           -(1 as libc::c_int) as libc::c_long {
                    ret = *__errno_location();
                    current_block = 8522321847195001863;
                } else { current_block = 18386322304582297246; }
            } else { current_block = 18386322304582297246; }
        }
    } else {
        /* Read, parse, and free the primary key's payload. */
        payloadlen = keyctl_read_alloc(primary_id, &mut payload);
        if payloadlen == -(1 as libc::c_int) {
            ret = *__errno_location();
            current_block = 8522321847195001863;
        } else {
            ret =
                parse_index(context, &mut version, &mut subsidiary_name,
                            payload as *const libc::c_uchar,
                            payloadlen as size_t);
            if ret != 0 {
                current_block = 8522321847195001863;
            } else if version != 1 as libc::c_int {
                ret = -(1750600186 as libc::c_long) as krb5_error_code;
                current_block = 8522321847195001863;
            } else { current_block = 18386322304582297246; }
        }
    }
    match current_block {
        18386322304582297246 => {
            *subsidiary_out = subsidiary_name;
            subsidiary_name = 0 as *mut libc::c_char
        }
        _ => { }
    }
    free(payload);
    free(subsidiary_name as *mut libc::c_void);
    return ret;
}
/*
 * Create a keyring with a unique random name within collection_id.  Set
 * *subsidiary to its name and *cache_id_out to its key serial number.
 */
#[c2rust::src_loc = "647:1"]
unsafe extern "C" fn unique_keyring(mut context: krb5_context,
                                    mut collection_id: key_serial_t,
                                    mut subsidiary_out:
                                        *mut *mut libc::c_char,
                                    mut cache_id_out: *mut key_serial_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut key: key_serial_t = 0;
    let mut ret: krb5_error_code = 0;
    let mut uniquename: [libc::c_char; 20] = [0; 20];
    let mut prefixlen: libc::c_int =
        (::std::mem::size_of::<[libc::c_char; 12]>() as
             libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    let mut tries: libc::c_int = 0;
    *subsidiary_out = 0 as *mut libc::c_char;
    *cache_id_out = 0 as libc::c_int;
    memcpy(uniquename.as_mut_ptr() as *mut libc::c_void,
           b"krb_ccache_\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong);
    k5_cc_mutex_lock(context, &mut krb5int_krcc_mutex);
    /* Loop until we successfully create a new ccache keyring with
     * a unique name, or we get an error. Limit to 100 tries. */
    tries = 100 as libc::c_int;
    loop  {
        let fresh0 = tries;
        tries = tries - 1;
        if !(fresh0 > 0 as libc::c_int) {
            current_block = 7149356873433890176;
            break ;
        }
        ret =
            krb5int_random_string(context,
                                  uniquename.as_mut_ptr().offset(prefixlen as
                                                                     isize),
                                  8 as libc::c_int as libc::c_uint);
        if ret != 0 { current_block = 1411540298341020386; break ; }
        key =
            keyctl_search(collection_id,
                          b"keyring\x00" as *const u8 as *const libc::c_char,
                          uniquename.as_mut_ptr(), 0 as libc::c_int) as
                key_serial_t;
        if !(key < 0 as libc::c_int) { continue ; }
        /* Name does not already exist.  Create it to reserve the name. */
        key =
            add_key(b"keyring\x00" as *const u8 as *const libc::c_char,
                    uniquename.as_mut_ptr(), 0 as *const libc::c_void,
                    0 as libc::c_int as size_t, collection_id);
        if !(key < 0 as libc::c_int) {
            current_block = 7149356873433890176;
            break ;
        }
        ret = *__errno_location();
        current_block = 1411540298341020386;
        break ;
    }
    match current_block {
        7149356873433890176 => {
            if tries <= 0 as libc::c_int {
                ret = -(1765328245 as libc::c_long) as krb5_error_code
            } else {
                *subsidiary_out = strdup(uniquename.as_mut_ptr());
                if (*subsidiary_out).is_null() {
                    ret = 12 as libc::c_int
                } else { *cache_id_out = key; ret = 0 as libc::c_int }
            }
        }
        _ => { }
    }
    k5_cc_mutex_unlock(context, &mut krb5int_krcc_mutex);
    return ret;
}
#[c2rust::src_loc = "703:1"]
unsafe extern "C" fn add_cred_key(mut name: *const libc::c_char,
                                  mut payload: *const libc::c_void,
                                  mut plen: size_t,
                                  mut cache_id: key_serial_t,
                                  mut legacy_type: krb5_boolean,
                                  mut key_out: *mut key_serial_t)
 -> krb5_error_code {
    let mut key: key_serial_t = 0;
    *key_out = -(1 as libc::c_int);
    if legacy_type == 0 {
        /* Try the preferred cred key type; fall back if no kernel support. */
        key =
            add_key(b"big_key\x00" as *const u8 as *const libc::c_char, name,
                    payload, plen, cache_id);
        if key != -(1 as libc::c_int) {
            *key_out = key;
            return 0 as libc::c_int
        } else {
            if *__errno_location() != 22 as libc::c_int &&
                   *__errno_location() != 19 as libc::c_int {
                return *__errno_location()
            }
        }
    }
    /* Use the user key type. */
    key =
        add_key(b"user\x00" as *const u8 as *const libc::c_char, name,
                payload, plen, cache_id);
    if key == -(1 as libc::c_int) { return *__errno_location() }
    *key_out = key;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "729:1"]
unsafe extern "C" fn update_keyring_expiration(mut context: krb5_context,
                                               mut id: krb5_ccache) {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut cursor: krb5_cc_cursor = 0 as *mut libc::c_void;
    let mut creds: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    let mut now: krb5_timestamp = 0;
    let mut endtime: krb5_timestamp = 0 as libc::c_int;
    let mut timeout: libc::c_uint = 0;
    /*
     * We have no way to know what is the actual timeout set on the keyring.
     * We also cannot keep track of it in a local variable as another process
     * can always modify the keyring independently, so just always enumerate
     * all keys and find out the highest endtime time.
     */
    /* Find the maximum endtime of all creds in the cache. */
    if krcc_start_seq_get(context, id, &mut cursor) != 0 as libc::c_int {
        return
    }
    while !(krcc_next_cred(context, id, &mut cursor, &mut creds) !=
                0 as libc::c_int) {
        if ts_after(creds.times.endtime, endtime) != 0 {
            endtime = creds.times.endtime
        }
        krb5_free_cred_contents(context, &mut creds);
    }
    krcc_end_seq_get(context, id, &mut cursor);
    if endtime == 0 as libc::c_int {
        /* No creds with end times */
        return
    }
    if krb5_timeofday(context, &mut now) != 0 as libc::c_int { return }
    /* Setting the timeout to zero would reset the timeout, so we set it to one
     * second instead if creds are already expired. */
    timeout =
        if ts_after(endtime, now) != 0 {
            ts_delta(endtime, now)
        } else { 1 as libc::c_int } as libc::c_uint;
    keyctl_set_timeout((*data).cache_id, timeout);
}
/* Create or overwrite the cache keyring, and set the default principal. */
#[c2rust::src_loc = "770:1"]
unsafe extern "C" fn krcc_initialize(mut context: krb5_context,
                                     mut id: krb5_ccache,
                                     mut princ: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut os_ctx: krb5_os_context = &mut (*context).os_context;
    let mut ret: krb5_error_code = 0;
    let mut cache_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    ret = clear_cache_keyring(context, id);
    if !(ret != 0) {
        if (*data).cache_id == 0 {
            /* The key didn't exist at resolve time.  Check again and create the
         * key if it still isn't there. */
            p = strrchr((*data).name, ':' as i32);
            cache_name =
                if !p.is_null() {
                    p.offset(1 as libc::c_int as isize)
                } else { (*data).name as *const libc::c_char };
            ret =
                find_or_create_keyring((*data).collection_id,
                                       0 as libc::c_int, cache_name,
                                       &mut (*data).cache_id);
            if ret != 0 {
                current_block = 13319521530526845468;
            } else { current_block = 1917311967535052937; }
        } else { current_block = 1917311967535052937; }
        match current_block {
            13319521530526845468 => { }
            _ => {
                /* If this is the legacy cache in a legacy session collection, link it
     * directly to the session keyring so that old code can see it. */
                if is_legacy_cache_name((*data).name) != 0 {
                    keyctl_link((*data).cache_id, session_write_anchor());
                }
                ret = save_principal(context, id, princ);
                /* Save time offset if it is valid and this is not a legacy cache.  Legacy
     * applications would fail to parse the new key in the cache keyring. */
                if is_legacy_cache_name((*data).name) == 0 &&
                       (*os_ctx).os_flags & 1 as libc::c_int != 0 {
                    ret =
                        save_time_offsets(context, id, (*os_ctx).time_offset,
                                          (*os_ctx).usec_offset)
                }
                if ret == 0 as libc::c_int { krb5_change_cache(); }
            }
        }
    }
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return ret;
}
/* Release the ccache handle. */
#[c2rust::src_loc = "819:1"]
unsafe extern "C" fn krcc_close(mut context: krb5_context,
                                mut id: krb5_ccache) -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    k5_os_mutex_destroy(&mut (*data).lock.lock);
    free((*data).name as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(id as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* Clear out a ccache keyring, unlinking all keys within it.  Call with the
 * mutex locked. */
#[c2rust::src_loc = "833:1"]
unsafe extern "C" fn clear_cache_keyring(mut context: krb5_context,
                                         mut id: krb5_ccache)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut res: libc::c_int = 0;
    k5_cc_mutex_assert_locked(context, &mut (*data).lock);
    if (*data).cache_id != 0 {
        res = keyctl_clear((*data).cache_id) as libc::c_int;
        if res != 0 as libc::c_int { return *__errno_location() }
    }
    (*data).princ_id = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/* Destroy the cache keyring and release the handle. */
#[c2rust::src_loc = "855:1"]
unsafe extern "C" fn krcc_destroy(mut context: krb5_context,
                                  mut id: krb5_ccache) -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut res: libc::c_int = 0;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    clear_cache_keyring(context, id);
    if (*data).cache_id != 0 {
        res =
            keyctl_unlink((*data).cache_id, (*data).collection_id) as
                libc::c_int;
        if res < 0 as libc::c_int { ret = *__errno_location() }
        /* If this is a legacy cache, unlink it from the session anchor. */
        if is_legacy_cache_name((*data).name) != 0 {
            keyctl_unlink((*data).cache_id, session_write_anchor());
        }
    }
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    k5_os_mutex_destroy(&mut (*data).lock.lock);
    free((*data).name as *mut libc::c_void);
    free(data as *mut libc::c_void);
    free(id as *mut libc::c_void);
    krb5_change_cache();
    return ret;
}
/* Create a cache handle for a cache ID. */
#[c2rust::src_loc = "887:1"]
unsafe extern "C" fn make_cache(mut context: krb5_context,
                                mut collection_id: key_serial_t,
                                mut cache_id: key_serial_t,
                                mut anchor_name: *const libc::c_char,
                                mut collection_name: *const libc::c_char,
                                mut subsidiary_name: *const libc::c_char,
                                mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut os_ctx: krb5_os_context = &mut (*context).os_context;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    let mut data: *mut krcc_data = 0 as *mut krcc_data;
    let mut pkey: key_serial_t = 0 as libc::c_int;
    /* Determine the key containing principal information, if present. */
    pkey =
        keyctl_search(cache_id,
                      b"user\x00" as *const u8 as *const libc::c_char,
                      b"__krb5_princ__\x00" as *const u8 as
                          *const libc::c_char, 0 as libc::c_int) as
            key_serial_t;
    if pkey < 0 as libc::c_int { pkey = 0 as libc::c_int }
    ccache =
        malloc(::std::mem::size_of::<_krb5_ccache>() as libc::c_ulong) as
            krb5_ccache;
    if ccache.is_null() { return 12 as libc::c_int }
    ret =
        make_krcc_data(anchor_name, collection_name, subsidiary_name,
                       cache_id, collection_id, &mut data);
    if ret != 0 { free(ccache as *mut libc::c_void); return ret }
    (*data).princ_id = pkey;
    (*ccache).ops = &krb5_krcc_ops;
    (*ccache).data = data as krb5_pointer;
    (*ccache).magic = -(1760647380 as libc::c_long) as krb5_magic;
    *cache_out = ccache;
    /* Look up time offsets if necessary. */
    if (*context).library_options & 0x1 as libc::c_int != 0 &&
           (*os_ctx).os_flags & 1 as libc::c_int == 0 {
        if get_time_offsets(context, ccache, &mut (*os_ctx).time_offset,
                            &mut (*os_ctx).usec_offset) == 0 as libc::c_int {
            (*os_ctx).os_flags &= !(2 as libc::c_int);
            (*os_ctx).os_flags |= 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* Create a keyring ccache handle for the given residual string. */
#[c2rust::src_loc = "936:1"]
unsafe extern "C" fn krcc_resolve(mut context: krb5_context,
                                  mut id: *mut krb5_ccache,
                                  mut residual: *const libc::c_char)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut collection_id: key_serial_t = 0;
    let mut cache_id: key_serial_t = 0;
    let mut anchor_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collection_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subsidiary_name: *mut libc::c_char = 0 as *mut libc::c_char;
    ret =
        parse_residual(residual, &mut anchor_name, &mut collection_name,
                       &mut subsidiary_name);
    if !(ret != 0) {
        ret =
            get_collection(anchor_name, collection_name, &mut collection_id);
        if !(ret != 0) {
            if subsidiary_name.is_null() {
                /* Retrieve or initialize the primary name for the collection. */
                ret =
                    get_primary_name(context, anchor_name, collection_name,
                                     collection_id, &mut subsidiary_name);
                if ret != 0 {
                    current_block = 1851333510549271337;
                } else { current_block = 3276175668257526147; }
            } else { current_block = 3276175668257526147; }
            match current_block {
                1851333510549271337 => { }
                _ => {
                    /* Look up the cache keyring ID, if the cache is already initialized. */
                    cache_id =
                        keyctl_search(collection_id,
                                      b"keyring\x00" as *const u8 as
                                          *const libc::c_char,
                                      subsidiary_name, 0 as libc::c_int) as
                            key_serial_t;
                    if cache_id < 0 as libc::c_int {
                        cache_id = 0 as libc::c_int
                    }
                    ret =
                        make_cache(context, collection_id, cache_id,
                                   anchor_name, collection_name,
                                   subsidiary_name, id);
                    (ret) != 0;
                }
            }
        }
    }
    free(anchor_name as *mut libc::c_void);
    free(collection_name as *mut libc::c_void);
    free(subsidiary_name as *mut libc::c_void);
    return ret;
}
/* Prepare for a sequential iteration over the cache keyring. */
#[c2rust::src_loc = "978:1"]
unsafe extern "C" fn krcc_start_seq_get(mut context: krb5_context,
                                        mut id: krb5_ccache,
                                        mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    let mut krcursor: krcc_cursor = 0 as *mut _krcc_cursor;
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut keys: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: libc::c_long = 0;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    if (*data).cache_id == 0 {
        k5_cc_mutex_unlock(context, &mut (*data).lock);
        return -(1765328189 as libc::c_long) as krb5_error_code
    }
    size = keyctl_read_alloc((*data).cache_id, &mut keys) as libc::c_long;
    if size == -(1 as libc::c_int) as libc::c_long {
        k5_cc_mutex_unlock(context, &mut (*data).lock);
        return -(1765328191 as libc::c_long) as krb5_error_code
    }
    krcursor =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_krcc_cursor>() as libc::c_ulong) as
            krcc_cursor;
    if krcursor.is_null() {
        free(keys);
        k5_cc_mutex_unlock(context, &mut (*data).lock);
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    (*krcursor).princ_id = (*data).princ_id;
    (*krcursor).offsets_id =
        keyctl_search((*data).cache_id,
                      b"user\x00" as *const u8 as *const libc::c_char,
                      b"__krb5_time_offsets__\x00" as *const u8 as
                          *const libc::c_char, 0 as libc::c_int) as
            key_serial_t;
    (*krcursor).numkeys =
        (size as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<key_serial_t>()
                                             as libc::c_ulong) as libc::c_int;
    (*krcursor).keys = keys as *mut key_serial_t;
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    *cursor = krcursor as krb5_cc_cursor;
    return 0 as libc::c_int;
}
/* Get the next credential from the cache keyring. */
#[c2rust::src_loc = "1020:1"]
unsafe extern "C" fn krcc_next_cred(mut context: krb5_context,
                                    mut id: krb5_ccache,
                                    mut cursor: *mut krb5_cc_cursor,
                                    mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut krcursor: krcc_cursor = 0 as *mut _krcc_cursor;
    let mut ret: krb5_error_code = 0;
    let mut psize: libc::c_int = 0;
    let mut payload: *mut libc::c_void = 0 as *mut libc::c_void;
    memset(creds as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    /* The cursor has the entire list of keys. */
    krcursor = *cursor as krcc_cursor;
    if krcursor.is_null() {
        return -(1765328242 as libc::c_long) as krb5_error_code
    }
    while (*krcursor).currkey < (*krcursor).numkeys {
        /* If we're pointing at the entry with the principal, or at the key
         * with the time offsets, skip it. */
        if *(*krcursor).keys.offset((*krcursor).currkey as isize) ==
               (*krcursor).princ_id ||
               *(*krcursor).keys.offset((*krcursor).currkey as isize) ==
                   (*krcursor).offsets_id {
            (*krcursor).currkey += 1
        } else {
            /* Read the key; the right size buffer will be allocated and
         * returned. */
            psize =
                keyctl_read_alloc(*(*krcursor).keys.offset((*krcursor).currkey
                                                               as isize),
                                  &mut payload);
            if psize != -(1 as libc::c_int) {
                (*krcursor).currkey += 1;
                /* Unmarshal the cred using the file ccache version 4 format. */
                ret =
                    k5_unmarshal_cred(payload as *const libc::c_uchar,
                                      psize as size_t, 4 as libc::c_int,
                                      creds);
                free(payload);
                return ret
            } else {
                if *__errno_location() != 126 as libc::c_int &&
                       *__errno_location() != 13 as libc::c_int {
                    return -(1765328189 as libc::c_long) as krb5_error_code
                }
            }
            /* The current key was unlinked, probably by a remove_cred call; move
         * on to the next one. */
            (*krcursor).currkey += 1
        }
    }
    /* No more keys in keyring. */
    return -(1765328242 as libc::c_long) as krb5_error_code;
}
/* Release an iteration cursor. */
#[c2rust::src_loc = "1072:1"]
unsafe extern "C" fn krcc_end_seq_get(mut context: krb5_context,
                                      mut id: krb5_ccache,
                                      mut cursor: *mut krb5_cc_cursor)
 -> krb5_error_code {
    let mut krcursor: krcc_cursor = *cursor as krcc_cursor;
    if !krcursor.is_null() {
        free((*krcursor).keys as *mut libc::c_void);
        free(krcursor as *mut libc::c_void);
    }
    *cursor = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* Create keyring data for a credential cache. */
#[c2rust::src_loc = "1086:1"]
unsafe extern "C" fn make_krcc_data(mut anchor_name: *const libc::c_char,
                                    mut collection_name: *const libc::c_char,
                                    mut subsidiary_name: *const libc::c_char,
                                    mut cache_id: key_serial_t,
                                    mut collection_id: key_serial_t,
                                    mut data_out: *mut *mut krcc_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut data: *mut krcc_data = 0 as *mut krcc_data;
    *data_out = 0 as *mut krcc_data;
    data =
        malloc(::std::mem::size_of::<krcc_data>() as libc::c_ulong) as
            *mut krcc_data;
    if data.is_null() {
        return -(1765328186 as libc::c_long) as krb5_error_code
    }
    ret = k5_cc_mutex_init(&mut (*data).lock);
    if ret != 0 { free(data as *mut libc::c_void); return ret }
    ret =
        make_subsidiary_residual(anchor_name, collection_name,
                                 subsidiary_name, &mut (*data).name);
    if ret != 0 {
        k5_os_mutex_destroy(&mut (*data).lock.lock);
        free(data as *mut libc::c_void);
        return ret
    }
    (*data).princ_id = 0 as libc::c_int;
    (*data).cache_id = cache_id;
    (*data).collection_id = collection_id;
    (*data).is_legacy_type =
        (strcmp(anchor_name,
                b"legacy\x00" as *const u8 as *const libc::c_char) ==
             0 as libc::c_int) as libc::c_int as krb5_boolean;
    *data_out = data;
    return 0 as libc::c_int;
}
/* Create a new keyring cache with a unique name. */
#[c2rust::src_loc = "1123:1"]
unsafe extern "C" fn krcc_generate_new(mut context: krb5_context,
                                       mut id_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut id: krb5_ccache = 0 as krb5_ccache;
    let mut ret: krb5_error_code = 0;
    let mut anchor_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collection_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subsidiary_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_subsidiary_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: *mut krcc_data = 0 as *mut krcc_data;
    let mut collection_id: key_serial_t = 0;
    let mut cache_id: key_serial_t = 0 as libc::c_int;
    *id_out = 0 as krb5_ccache;
    /* Determine the collection in which we will create the cache.*/
    ret =
        get_default(context, &mut anchor_name, &mut collection_name,
                    &mut subsidiary_name);
    if ret != 0 { return ret }
    if anchor_name.is_null() {
        ret =
            parse_residual(b"session:__krb5_unique__\x00" as *const u8 as
                               *const libc::c_char, &mut anchor_name,
                           &mut collection_name, &mut subsidiary_name);
        if ret != 0 { return ret }
    }
    if !subsidiary_name.is_null() {
        krb5_set_error_message(context,
                               -(1750600188 as libc::c_long) as
                                   krb5_error_code,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Can\'t create new subsidiary cache because default cache is already a subsidiary\x00"
                                            as *const u8 as
                                            *const libc::c_char));
        ret = -(1750600188 as libc::c_long) as krb5_error_code
    } else {
        /* Allocate memory */
        id =
            malloc(::std::mem::size_of::<_krb5_ccache>() as libc::c_ulong) as
                krb5_ccache;
        if id.is_null() {
            ret = 12 as libc::c_int
        } else {
            (*id).ops = &krb5_krcc_ops;
            /* Make a unique keyring within the chosen collection. */
            ret =
                get_collection(anchor_name, collection_name,
                               &mut collection_id);
            if !(ret != 0) {
                ret =
                    unique_keyring(context, collection_id,
                                   &mut new_subsidiary_name, &mut cache_id);
                if !(ret != 0) {
                    ret =
                        make_krcc_data(anchor_name, collection_name,
                                       new_subsidiary_name, cache_id,
                                       collection_id, &mut data);
                    if !(ret != 0) {
                        (*id).data = data as krb5_pointer;
                        krb5_change_cache();
                    }
                }
            }
        }
    }
    free(anchor_name as *mut libc::c_void);
    free(collection_name as *mut libc::c_void);
    free(subsidiary_name as *mut libc::c_void);
    free(new_subsidiary_name as *mut libc::c_void);
    free(new_residual as *mut libc::c_void);
    if ret != 0 { free(id as *mut libc::c_void); return ret }
    *id_out = id;
    return 0 as libc::c_int;
}
/* Return an alias to the residual string of the cache. */
#[c2rust::src_loc = "1196:1"]
unsafe extern "C" fn krcc_get_name(mut context: krb5_context,
                                   mut id: krb5_ccache)
 -> *const libc::c_char {
    return (*((*id).data as *mut krcc_data)).name;
}
/* Retrieve a copy of the default principal, if the cache is initialized. */
#[c2rust::src_loc = "1203:1"]
unsafe extern "C" fn krcc_get_principal(mut context: krb5_context,
                                        mut id: krb5_ccache,
                                        mut princ_out: *mut krb5_principal)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut ret: krb5_error_code = 0;
    let mut payload: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut psize: libc::c_int = 0;
    *princ_out = 0 as krb5_principal;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    if (*data).cache_id == 0 || (*data).princ_id == 0 {
        ret = -(1765328189 as libc::c_long) as krb5_error_code;
        krb5_set_error_message(context, ret,
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Credentials cache keyring \'%s\' not found\x00"
                                            as *const u8 as
                                            *const libc::c_char),
                               (*data).name);
    } else {
        psize = keyctl_read_alloc((*data).princ_id, &mut payload);
        if psize == -(1 as libc::c_int) {
            ret = -(1765328191 as libc::c_long) as krb5_error_code
        } else {
            /* Unmarshal the principal using the file ccache version 4 format. */
            ret =
                k5_unmarshal_princ(payload as *const libc::c_uchar,
                                   psize as size_t, 4 as libc::c_int,
                                   princ_out)
        }
    }
    free(payload);
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return ret;
}
/* Search for a credential within the cache keyring. */
#[c2rust::src_loc = "1240:1"]
unsafe extern "C" fn krcc_retrieve(mut context: krb5_context,
                                   mut id: krb5_ccache,
                                   mut whichfields: krb5_flags,
                                   mut mcreds: *mut krb5_creds,
                                   mut creds: *mut krb5_creds)
 -> krb5_error_code {
    return k5_cc_retrieve_cred_default(context, id, whichfields, mcreds,
                                       creds);
}
/* Remove a credential from the cache keyring. */
#[c2rust::src_loc = "1250:1"]
unsafe extern "C" fn krcc_remove_cred(mut context: krb5_context,
                                      mut cache: krb5_ccache,
                                      mut flags: krb5_flags,
                                      mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut data: *mut krcc_data = (*cache).data as *mut krcc_data;
    let mut cursor: krb5_cc_cursor = 0 as *mut libc::c_void;
    let mut c: krb5_creds =
        krb5_creds{magic: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   keyblock:
                       krb5_keyblock{magic: 0,
                                     enctype: 0,
                                     length: 0,
                                     contents: 0 as *mut krb5_octet,},
                   times:
                       krb5_ticket_times{authtime: 0,
                                         starttime: 0,
                                         endtime: 0,
                                         renew_till: 0,},
                   is_skey: 0,
                   ticket_flags: 0,
                   addresses: 0 as *mut *mut krb5_address,
                   ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   second_ticket:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   authdata: 0 as *mut *mut krb5_authdata,};
    let mut krcursor: krcc_cursor = 0 as *mut _krcc_cursor;
    let mut key: key_serial_t = 0;
    let mut match_0: krb5_boolean = 0;
    ret = krcc_start_seq_get(context, cache, &mut cursor);
    if ret != 0 { return ret }
    loop  {
        ret = krcc_next_cred(context, cache, &mut cursor, &mut c);
        if ret != 0 { break ; }
        match_0 =
            krb5int_cc_creds_match_request(context, flags, creds, &mut c);
        krb5_free_cred_contents(context, &mut c);
        if !(match_0 != 0) { continue ; }
        krcursor = cursor as krcc_cursor;
        key =
            *(*krcursor).keys.offset(((*krcursor).currkey - 1 as libc::c_int)
                                         as isize);
        if !(keyctl_unlink(key, (*data).cache_id) ==
                 -(1 as libc::c_int) as libc::c_long) {
            continue ;
        }
        ret = *__errno_location();
        break ;
    }
    krcc_end_seq_get(context, cache, &mut cursor);
    return if ret as libc::c_long == -(1765328242 as libc::c_long) {
               0 as libc::c_int
           } else { ret };
}
/* Set flags on the cache.  (We don't care about any flags.) */
#[c2rust::src_loc = "1287:1"]
unsafe extern "C" fn krcc_set_flags(mut context: krb5_context,
                                    mut id: krb5_ccache,
                                    mut flags: krb5_flags)
 -> krb5_error_code {
    return 0 as libc::c_int;
}
/* Get the current operational flags (of which we have none) for the cache. */
#[c2rust::src_loc = "1294:1"]
unsafe extern "C" fn krcc_get_flags(mut context: krb5_context,
                                    mut id: krb5_ccache,
                                    mut flags_out: *mut krb5_flags)
 -> krb5_error_code {
    *flags_out = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/* Store a credential in the cache keyring. */
#[c2rust::src_loc = "1302:1"]
unsafe extern "C" fn krcc_store(mut context: krb5_context,
                                mut id: krb5_ccache,
                                mut creds: *mut krb5_creds)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut buf: k5buf =
        {
            let mut init =
                k5buf{buftype: K5BUF_ERROR,
                      data: 0 as *mut libc::c_void,
                      space: 0,
                      len: 0,};
            init
        };
    let mut keyname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cred_key: key_serial_t = 0;
    let mut now: krb5_timestamp = 0;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    if (*data).cache_id == 0 {
        k5_cc_mutex_unlock(context, &mut (*data).lock);
        return -(1765328189 as libc::c_long) as krb5_error_code
    }
    /* Get the service principal name and use it as the key name */
    ret =
        krb5_unparse_name(context, (*creds).server as krb5_const_principal,
                          &mut keyname);
    if !(ret != 0) {
        /* Serialize credential using the file ccache version 4 format. */
        k5_buf_init_dynamic_zap(&mut buf);
        k5_marshal_cred(&mut buf, 4 as libc::c_int, creds);
        ret = k5_buf_status(&mut buf);
        if !(ret != 0) {
            /* Add new key (credentials) into keyring */
            ret =
                add_cred_key(keyname, buf.data, buf.len, (*data).cache_id,
                             (*data).is_legacy_type, &mut cred_key);
            if !(ret != 0) {
                /* Set appropriate timeouts on cache keys. */
                ret = krb5_timeofday(context, &mut now);
                if !(ret != 0) {
                    if ts_after((*creds).times.endtime, now) != 0 {
                        keyctl_set_timeout(cred_key,
                                           ts_delta((*creds).times.endtime,
                                                    now) as libc::c_uint);
                    }
                    update_keyring_expiration(context, id);
                }
            }
        }
    }
    k5_buf_free(&mut buf);
    krb5_free_unparsed_name(context, keyname);
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return ret;
}
/* Lock the cache handle against other threads.  (This does not lock the cache
 * keyring against other processes.) */
#[c2rust::src_loc = "1360:1"]
unsafe extern "C" fn krcc_lock(mut context: krb5_context, mut id: krb5_ccache)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    return 0 as libc::c_int;
}
/* Unlock the cache handle. */
#[c2rust::src_loc = "1370:1"]
unsafe extern "C" fn krcc_unlock(mut context: krb5_context,
                                 mut id: krb5_ccache) -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1379:1"]
unsafe extern "C" fn save_principal(mut context: krb5_context,
                                    mut id: krb5_ccache,
                                    mut princ: krb5_principal)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut ret: krb5_error_code = 0;
    let mut buf: k5buf =
        k5buf{buftype: K5BUF_ERROR,
              data: 0 as *mut libc::c_void,
              space: 0,
              len: 0,};
    let mut newkey: key_serial_t = 0;
    k5_cc_mutex_assert_locked(context, &mut (*data).lock);
    /* Serialize princ using the file ccache version 4 format. */
    k5_buf_init_dynamic(&mut buf);
    k5_marshal_princ(&mut buf, 4 as libc::c_int, princ);
    if k5_buf_status(&mut buf) != 0 as libc::c_int {
        return 12 as libc::c_int
    }
    /* Add new key into keyring */
    newkey =
        add_key(b"user\x00" as *const u8 as *const libc::c_char,
                b"__krb5_princ__\x00" as *const u8 as *const libc::c_char,
                buf.data, buf.len, (*data).cache_id);
    if newkey < 0 as libc::c_int {
        ret = *__errno_location()
    } else { (*data).princ_id = newkey; ret = 0 as libc::c_int }
    k5_buf_free(&mut buf);
    return ret;
}
/* Add a key to the cache keyring containing the given time offsets. */
#[c2rust::src_loc = "1424:1"]
unsafe extern "C" fn save_time_offsets(mut context: krb5_context,
                                       mut id: krb5_ccache,
                                       mut time_offset: int32_t,
                                       mut usec_offset: int32_t)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut newkey: key_serial_t = 0;
    let mut payload: [libc::c_uchar; 8] = [0; 8];
    k5_cc_mutex_assert_locked(context, &mut (*data).lock);
    /* Prepare the payload. */
    store_32_be(time_offset as libc::c_uint,
                payload.as_mut_ptr() as *mut libc::c_void);
    store_32_be(usec_offset as libc::c_uint,
                payload.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    /* Add new key into keyring. */
    newkey =
        add_key(b"user\x00" as *const u8 as *const libc::c_char,
                b"__krb5_time_offsets__\x00" as *const u8 as
                    *const libc::c_char,
                payload.as_mut_ptr() as *const libc::c_void,
                8 as libc::c_int as size_t, (*data).cache_id);
    if newkey == -(1 as libc::c_int) { return *__errno_location() }
    return 0 as libc::c_int;
}
/* Retrieve and parse the key in the cache keyring containing time offsets. */
#[c2rust::src_loc = "1447:1"]
unsafe extern "C" fn get_time_offsets(mut context: krb5_context,
                                      mut id: krb5_ccache,
                                      mut time_offset: *mut int32_t,
                                      mut usec_offset: *mut int32_t)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*id).data as *mut krcc_data;
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut key: key_serial_t = 0;
    let mut payload: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut psize: libc::c_int = 0;
    k5_cc_mutex_lock(context, &mut (*data).lock);
    if (*data).cache_id == 0 {
        ret = -(1765328189 as libc::c_long) as krb5_error_code
    } else {
        key =
            keyctl_search((*data).cache_id,
                          b"user\x00" as *const u8 as *const libc::c_char,
                          b"__krb5_time_offsets__\x00" as *const u8 as
                              *const libc::c_char, 0 as libc::c_int) as
                key_serial_t;
        if key == -(1 as libc::c_int) {
            ret = 2 as libc::c_int
        } else {
            psize = keyctl_read_alloc(key, &mut payload);
            if psize == -(1 as libc::c_int) {
                ret = -(1765328191 as libc::c_long) as krb5_error_code
            } else if psize < 8 as libc::c_int {
                ret = -(1765328242 as libc::c_long) as krb5_error_code
            } else {
                *time_offset = load_32_be(payload) as int32_t;
                *usec_offset =
                    load_32_be((payload as
                                    *mut libc::c_char).offset(4 as libc::c_int
                                                                  as isize) as
                                   *const libc::c_void) as int32_t
            }
        }
    }
    free(payload);
    k5_cc_mutex_unlock(context, &mut (*data).lock);
    return ret;
}
#[c2rust::src_loc = "1504:1"]
unsafe extern "C" fn krcc_ptcursor_new(mut context: krb5_context,
                                       mut cursor_out: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ptd: *mut krcc_ptcursor_data = 0 as *mut krcc_ptcursor_data;
    let mut cursor: krb5_cc_ptcursor = 0 as *mut krb5_cc_ptcursor_s;
    let mut ret: krb5_error_code = 0;
    let mut keys: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: libc::c_long = 0;
    *cursor_out = 0 as krb5_cc_ptcursor;
    cursor =
        k5alloc(::std::mem::size_of::<krb5_cc_ptcursor_s>() as libc::c_ulong,
                &mut ret) as krb5_cc_ptcursor;
    if cursor.is_null() { return 12 as libc::c_int }
    ptd =
        k5alloc(::std::mem::size_of::<krcc_ptcursor_data>() as libc::c_ulong,
                &mut ret) as *mut krcc_ptcursor_data;
    if !ptd.is_null() {
        (*cursor).ops = &krb5_krcc_ops;
        (*cursor).data = ptd as krb5_pointer;
        (*ptd).first = 1 as libc::c_int as krb5_boolean;
        ret =
            get_default(context, &mut (*ptd).anchor_name,
                        &mut (*ptd).collection_name,
                        &mut (*ptd).subsidiary_name);
        if !(ret != 0) {
            /* If there is no default collection, return an empty cursor. */
            if (*ptd).anchor_name.is_null() {
                *cursor_out = cursor;
                return 0 as libc::c_int
            }
            ret =
                get_collection((*ptd).anchor_name, (*ptd).collection_name,
                               &mut (*ptd).collection_id);
            if !(ret != 0) {
                if (*ptd).subsidiary_name.is_null() {
                    ret =
                        get_primary_name(context, (*ptd).anchor_name,
                                         (*ptd).collection_name,
                                         (*ptd).collection_id,
                                         &mut (*ptd).primary_name);
                    if ret != 0 {
                        current_block = 11206778511404137033;
                    } else {
                        size =
                            keyctl_read_alloc((*ptd).collection_id, &mut keys)
                                as libc::c_long;
                        if size == -(1 as libc::c_int) as libc::c_long {
                            ret = *__errno_location();
                            current_block = 11206778511404137033;
                        } else {
                            (*ptd).keys = keys as *mut key_serial_t;
                            (*ptd).num_keys =
                                (size as
                                     libc::c_ulong).wrapping_div(::std::mem::size_of::<key_serial_t>()
                                                                     as
                                                                     libc::c_ulong)
                                    as libc::c_long;
                            current_block = 2232869372362427478;
                        }
                    }
                } else { current_block = 2232869372362427478; }
                match current_block {
                    11206778511404137033 => { }
                    _ => { *cursor_out = cursor; return 0 as libc::c_int }
                }
            }
        }
    }
    krcc_ptcursor_free(context, &mut cursor);
    return ret;
}
#[c2rust::src_loc = "1565:1"]
unsafe extern "C" fn krcc_ptcursor_next(mut context: krb5_context,
                                        mut cursor: krb5_cc_ptcursor,
                                        mut cache_out: *mut krb5_ccache)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut ptd: *mut krcc_ptcursor_data =
        (*cursor).data as *mut krcc_ptcursor_data;
    let mut key: key_serial_t = 0;
    let mut cache_id: key_serial_t = 0 as libc::c_int;
    let mut first_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut keytype: *const libc::c_char = 0 as *const libc::c_char;
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut subsidiary_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut keytypelen: size_t = 0;
    let mut description: *mut libc::c_char = 0 as *mut libc::c_char;
    *cache_out = 0 as krb5_ccache;
    /* No keyring available */
    if (*ptd).collection_id == 0 as libc::c_int { return 0 as libc::c_int }
    if (*ptd).first != 0 {
        /* Look for the primary cache for a collection cursor, or the
         * subsidiary cache for a subsidiary cursor. */
        (*ptd).first = 0 as libc::c_int as krb5_boolean;
        first_name =
            if !(*ptd).primary_name.is_null() {
                (*ptd).primary_name
            } else { (*ptd).subsidiary_name };
        cache_id =
            keyctl_search((*ptd).collection_id,
                          b"keyring\x00" as *const u8 as *const libc::c_char,
                          first_name, 0 as libc::c_int) as key_serial_t;
        if cache_id != -(1 as libc::c_int) {
            return make_cache(context, (*ptd).collection_id, cache_id,
                              (*ptd).anchor_name, (*ptd).collection_name,
                              first_name, cache_out)
        }
    }
    /* A subsidiary cursor yields at most the first cache. */
    if !(*ptd).subsidiary_name.is_null() { return 0 as libc::c_int }
    keytype = b"keyring;\x00" as *const u8 as *const libc::c_char;
    keytypelen = strlen(keytype);
    while (*ptd).next_key < (*ptd).num_keys {
        /* Free any previously retrieved key description. */
        free(description as *mut libc::c_void);
        description = 0 as *mut libc::c_char;
        /*
         * Get the key description, which should have the form:
         *   typename;UID;GID;permissions;description
         */
        key = *(*ptd).keys.offset((*ptd).next_key as isize);
        if !(keyctl_describe_alloc(key, &mut description) < 0 as libc::c_int)
           {
            sep = strrchr(description, ';' as i32);
            if !sep.is_null() {
                subsidiary_name = sep.offset(1 as libc::c_int as isize);
                /* Skip this key if it isn't a keyring. */
                if !(strncmp(description, keytype, keytypelen) !=
                         0 as libc::c_int) {
                    /* Don't repeat the primary cache. */
                    if !(strcmp(subsidiary_name, (*ptd).primary_name) ==
                             0 as libc::c_int) {
                        /* We found a valid key */
                        (*ptd).next_key += 1;
                        ret =
                            make_cache(context, (*ptd).collection_id, key,
                                       (*ptd).anchor_name,
                                       (*ptd).collection_name,
                                       subsidiary_name, cache_out);
                        free(description as *mut libc::c_void);
                        return ret
                    }
                }
            }
        }
        (*ptd).next_key += 1
    }
    free(description as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1641:1"]
unsafe extern "C" fn krcc_ptcursor_free(mut context: krb5_context,
                                        mut cursor: *mut krb5_cc_ptcursor)
 -> krb5_error_code {
    let mut ptd: *mut krcc_ptcursor_data =
        (**cursor).data as *mut krcc_ptcursor_data;
    if !ptd.is_null() {
        free((*ptd).anchor_name as *mut libc::c_void);
        free((*ptd).collection_name as *mut libc::c_void);
        free((*ptd).subsidiary_name as *mut libc::c_void);
        free((*ptd).primary_name as *mut libc::c_void);
        free((*ptd).keys as *mut libc::c_void);
        free(ptd as *mut libc::c_void);
    }
    free(*cursor as *mut libc::c_void);
    *cursor = 0 as krb5_cc_ptcursor;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1659:1"]
unsafe extern "C" fn krcc_switch_to(mut context: krb5_context,
                                    mut cache: krb5_ccache)
 -> krb5_error_code {
    let mut data: *mut krcc_data = (*cache).data as *mut krcc_data;
    let mut ret: krb5_error_code = 0;
    let mut anchor_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collection_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut subsidiary_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut collection_id: key_serial_t = 0;
    ret =
        parse_residual((*data).name, &mut anchor_name, &mut collection_name,
                       &mut subsidiary_name);
    if !(ret != 0) {
        ret =
            get_collection(anchor_name, collection_name, &mut collection_id);
        if !(ret != 0) {
            ret = set_primary_name(context, collection_id, subsidiary_name)
        }
    }
    free(anchor_name as *mut libc::c_void);
    free(collection_name as *mut libc::c_void);
    free(subsidiary_name as *mut libc::c_void);
    return ret;
}
/*
 * ccache implementation storing credentials in the Linux keyring facility
 * The default is to put them at the session keyring level.
 * If "KEYRING:process:" or "KEYRING:thread:" is specified, then they will
 * be stored at the process or thread level respectively.
 */
#[no_mangle]
#[c2rust::src_loc = "1689:19"]
pub static mut krb5_krcc_ops: krb5_cc_ops =
    unsafe {
        {
            let mut init =
                _krb5_cc_ops{magic: 0 as libc::c_int,
                             prefix:
                                 b"KEYRING\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             get_name:
                                 Some(krcc_get_name as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> *const libc::c_char),
                             resolve:
                                 Some(krcc_resolve as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache,
                                                               _:
                                                                   *const libc::c_char)
                                              -> krb5_error_code),
                             gen_new:
                                 Some(krcc_generate_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             init:
                                 Some(krcc_initialize as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   krb5_principal)
                                              -> krb5_error_code),
                             destroy:
                                 Some(krcc_destroy as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             close:
                                 Some(krcc_close as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             store:
                                 Some(krcc_store as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             retrieve:
                                 Some(krcc_retrieve as
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
                                 Some(krcc_get_principal as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_principal)
                                              -> krb5_error_code),
                             get_first:
                                 Some(krcc_start_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             get_next:
                                 Some(krcc_next_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             end_get:
                                 Some(krcc_end_seq_get as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_cc_cursor)
                                              -> krb5_error_code),
                             remove_cred:
                                 Some(krcc_remove_cred as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags,
                                                               _:
                                                                   *mut krb5_creds)
                                              -> krb5_error_code),
                             set_flags:
                                 Some(krcc_set_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _: krb5_flags)
                                              -> krb5_error_code),
                             get_flags:
                                 Some(krcc_get_flags as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache,
                                                               _:
                                                                   *mut krb5_flags)
                                              -> krb5_error_code),
                             ptcursor_new:
                                 Some(krcc_ptcursor_new as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             ptcursor_next:
                                 Some(krcc_ptcursor_next as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_cc_ptcursor,
                                                               _:
                                                                   *mut krb5_ccache)
                                              -> krb5_error_code),
                             ptcursor_free:
                                 Some(krcc_ptcursor_free as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_cc_ptcursor)
                                              -> krb5_error_code),
                             move_0: None,
                             wasdefault: None,
                             lock:
                                 Some(krcc_lock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             unlock:
                                 Some(krcc_unlock as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),
                             switch_to:
                                 Some(krcc_switch_to as
                                          unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _: krb5_ccache)
                                              -> krb5_error_code),};
            init
        }
    };
/* USE_KEYRING_CCACHE */
/* !USE_KEYRING_CCACHE */
