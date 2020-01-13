use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "32:1"]
    pub type __u_short = libc::c_ushort;
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
    #[c2rust::src_loc = "34:1"]
    pub type __u_long = libc::c_ulong;
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
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:27"]
pub mod sys_types_h {
    #[c2rust::src_loc = "34:1"]
    pub type u_short = __u_short;
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "36:1"]
    pub type u_long = __u_long;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_short, __u_int, __u_long, __ssize_t, __caddr_t};
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
#[c2rust::header_src = "/usr/include/sys/select.h:27"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub fds_bits: [__fd_mask; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_iovec.h:27"]
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
#[c2rust::header_src = "/usr/include/unistd.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:27"]
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
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
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
    use super::stdint_uintn_h::{uint8_t, uint32_t};
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_error_code};
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
        #[c2rust::src_loc = "1778:1"]
        pub fn k5_parse_host_string(address: *const libc::c_char,
                                    default_port: libc::c_int,
                                    host_out: *mut *mut libc::c_char,
                                    port_out: *mut libc::c_int)
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:27"]
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
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:27"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:27"]
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
#[c2rust::header_src = "/usr/include/sys/socket.h:27"]
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
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "173:1"]
        pub fn sendmsg(__fd: libc::c_int, __message: *const msghdr,
                       __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                      __addr_len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:27"]
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
    #[c2rust::src_loc = "40:1"]
    pub type C2RustUnnamed_3 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_3 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_3 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_3 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_3 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_3 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_3 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_3 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_3 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_3 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_3 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_3 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_3 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_3 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_3 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_3 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_3 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_3 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_3 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_3 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_3 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_3 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_3 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_3 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_3 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_3 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_3 = 0;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/port-sockets.h:27"]
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
    use super::sys_types_h::ssize_t;
    use super::socket_h::{msghdr, MSG_NOSIGNAL};
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
    use super::string_h::memset;
    use super::sys_socket_h::sendmsg;
    /*_PORT_SOCKET_H*/
    /* UNIX or ...?  */
    /* _WIN32 */
    /* Use TMP to avoid compiler warnings and keep things consistent with
 * Windows version. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:36"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    /* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)types.h 1.18 87/07/24 SMI      */
    /*
 * Rpc additions to <sys/types.h>
 */
    /*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
    /* #include <netdb.h> */
    /* Get htonl(), ntohl(), etc. */
    /* Define if we need to fake up some BSD type aliases. */
    /* Allow application to override. */
    /* #undef GSSRPC__BSD_TYPEALIASES */
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:36"]
pub mod xdr_h {
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct XDR {
        pub x_op: xdr_op,
        pub x_ops: *mut xdr_ops,
        pub x_public: caddr_t,
        pub x_private: *mut libc::c_void,
        pub x_base: caddr_t,
        pub x_handy: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct xdr_ops {
        pub x_getlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_putlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_getbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_putbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_getpostn: Option<unsafe extern "C" fn(_: *mut XDR) -> u_int>,
        pub x_setpostn: Option<unsafe extern "C" fn(_: *mut XDR, _: u_int)
                                   -> libc::c_int>,
        pub x_inline: Option<unsafe extern "C" fn(_: *mut XDR, _: libc::c_int)
                                 -> *mut rpc_inline_t>,
        pub x_destroy: Option<unsafe extern "C" fn(_: *mut XDR) -> ()>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:36"]
pub mod auth_h {
    /* @(#)auth.h	2.3 88/08/07 4.0 RPCSRC; from 1.17 88/02/08 SMI */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    /*
 * auth.h, Authentication interface.
 *
 * The data structures are completely opaque to the client.  The client
 * is required to pass a AUTH * to routines that create rpc
 * "sessions".
 */
    /* maximum length of network user's name */
    /*
 * Status returned from authentication check
 */
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    /* some unknown reason */
    /*
	 * RPCSEC_GSS errors
	 */
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    /* bogus response verifier */
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    /* rejected due to security reasons */
    /*
	 * failed locally
	*/
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    /* verifier expired or was replayed */
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    /* bogus verifier (seal broken) */
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    /* client should begin new session */
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    /* bogus credentials (seal broken) */
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    /*
	 * failed at remote end
	 */
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
    /*
 * Authentication info.  Opaque to client.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    use super::sys_types_h::{caddr_t, u_int};
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
    /* GSS-API style */
    /* des style (encrypted timestamps) */
    /* short hand unix style */
    /* unix style (uid, gids) */
    /* backward compatibility */
    /* no authentication */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:36"]
pub mod rpc_msg_h {
    /* @(#)rpc_msg.h	2.1 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)rpc_msg.h 1.7 86/07/16 SMI      */
    /*
 * rpc_msg.h
 * rpc message definition
 */
    /*
 * Bottom up definition of an rpc message.
 * NOTE: call and reply use the same overall stuct but
 * different parts of unions within it.
 */
    /*
 * Reply part of an rpc exchange
 */
    /*
 * Reply to an rpc request that was accepted by the server.
 * Note: there could be an error even though the request was
 * accepted.
 */
    /* and many other null cases */
    /*
 * Reply to an rpc request that was rejected by the server.
 */
    /* why authentication did not work */
    /*
 * Body of a reply to an rpc request.
 */
    /*
 * Body of an rpc request call.
 */
    /* must be equal to two */
    /* protocol specific - provided by client */
    /*
 * The rpc message
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:8"]
    pub struct rpc_msg {
        pub rm_xid: uint32_t,
        pub rm_direction: msg_type,
        pub ru: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_4 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_5 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_6 {
        pub RJ_versions: C2RustUnnamed_7,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_7 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_8,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_8 {
        pub AR_versions: C2RustUnnamed_10,
        pub AR_results: C2RustUnnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_9 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_10 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "65:1"]
    pub type accept_stat = libc::c_uint;
    #[c2rust::src_loc = "71:2"]
    pub const SYSTEM_ERR: accept_stat = 5;
    #[c2rust::src_loc = "70:2"]
    pub const GARBAGE_ARGS: accept_stat = 4;
    #[c2rust::src_loc = "69:2"]
    pub const PROC_UNAVAIL: accept_stat = 3;
    #[c2rust::src_loc = "68:2"]
    pub const PROG_MISMATCH: accept_stat = 2;
    #[c2rust::src_loc = "67:2"]
    pub const PROG_UNAVAIL: accept_stat = 1;
    #[c2rust::src_loc = "66:2"]
    pub const SUCCESS: accept_stat = 0;
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct call_body {
        pub cb_rpcvers: rpcvers_t,
        pub cb_prog: rpcprog_t,
        pub cb_vers: rpcvers_t,
        pub cb_proc: rpcproc_t,
        pub cb_cred: opaque_auth,
        pub cb_verf: opaque_auth,
    }
    #[c2rust::src_loc = "55:1"]
    pub type msg_type = libc::c_uint;
    #[c2rust::src_loc = "57:2"]
    pub const REPLY: msg_type = 1;
    #[c2rust::src_loc = "56:2"]
    pub const CALL: msg_type = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::auth_h::{auth_stat, opaque_auth};
    use super::gssrpc_types_h::{rpcvers_t, rpcprog_t, rpcproc_t};
    use super::sys_types_h::caddr_t;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc.h:36"]
pub mod svc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:8"]
    pub struct svc_req {
        pub rq_prog: rpcprog_t,
        pub rq_vers: rpcvers_t,
        pub rq_proc: rpcproc_t,
        pub rq_cred: opaque_auth,
        pub rq_clntcred: *mut libc::c_void,
        pub rq_svccred: *mut libc::c_void,
        pub rq_clntname: *mut libc::c_void,
        pub rq_xprt: *mut SVCXPRT,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct SVCXPRT {
        pub xp_sock: libc::c_int,
        pub xp_port: u_short,
        pub xp_ops: *mut xp_ops,
        pub xp_addrlen: libc::c_int,
        pub xp_raddr: sockaddr_in,
        pub xp_verf: opaque_auth,
        pub xp_auth: *mut SVCAUTH,
        pub xp_p1: *mut libc::c_void,
        pub xp_p2: *mut libc::c_void,
        pub xp_laddrlen: libc::c_int,
        pub xp_laddr: sockaddr_in,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub struct xp_ops {
        pub xp_recv: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                 _: *mut rpc_msg)
                                -> libc::c_int>,
        pub xp_stat: Option<unsafe extern "C" fn(_: *mut SVCXPRT)
                                -> xprt_stat>,
        pub xp_getargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub xp_reply: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                  _: *mut rpc_msg)
                                 -> libc::c_int>,
        pub xp_freeargs: Option<unsafe extern "C" fn(_: *mut SVCXPRT,
                                                     _: xdrproc_t,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int>,
        pub xp_destroy: Option<unsafe extern "C" fn(_: *mut SVCXPRT) -> ()>,
    }
    #[c2rust::src_loc = "67:1"]
    pub type xprt_stat = libc::c_uint;
    #[c2rust::src_loc = "70:2"]
    pub const XPRT_IDLE: xprt_stat = 2;
    #[c2rust::src_loc = "69:2"]
    pub const XPRT_MOREREQS: xprt_stat = 1;
    #[c2rust::src_loc = "68:2"]
    pub const XPRT_DIED: xprt_stat = 0;
    use super::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t};
    use super::auth_h::opaque_auth;
    use super::sys_types_h::{u_short, u_int};
    use super::in_h::sockaddr_in;
    use super::svc_auth_h::SVCAUTH;
    use super::rpc_msg_h::rpc_msg;
    use super::xdr_h::xdrproc_t;
    use super::select_h::{fd_set, __fd_mask};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "284:15"]
        pub static mut gssrpc_svc_fdset: fd_set;
        #[no_mangle]
        #[c2rust::src_loc = "198:1"]
        pub fn gssrpc_svc_register(_: *mut SVCXPRT, _: rpcprog_t,
                                   _: rpcvers_t,
                                   _:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut svc_req,
                                                                   _:
                                                                       *mut SVCXPRT)
                                                  -> ()>, _: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "300:1"]
        pub fn gssrpc_svc_getreqset(_: *mut fd_set);
        /*
 * Tcp based rpc.
 */
        #[no_mangle]
        #[c2rust::src_loc = "331:1"]
        pub fn gssrpc_svctcp_create(_: libc::c_int, _: u_int, _: u_int)
         -> *mut SVCXPRT;
    }
    /* !defined(GSSRPC_SVC_H) */
    /* XXX add auth_gsapi_log_*? */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/svc_auth.h:36"]
pub mod svc_auth_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:16"]
    pub struct SVCAUTH {
        pub svc_ah_ops: *mut svc_auth_ops,
        pub svc_ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct svc_auth_ops {
        pub svc_ah_wrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                     _: *mut XDR,
                                                     _: xdrproc_t, _: caddr_t)
                                    -> libc::c_int>,
        pub svc_ah_unwrap: Option<unsafe extern "C" fn(_: *mut SVCAUTH,
                                                       _: *mut XDR,
                                                       _: xdrproc_t,
                                                       _: caddr_t)
                                      -> libc::c_int>,
        pub svc_ah_destroy: Option<unsafe extern "C" fn(_: *mut SVCAUTH)
                                       -> libc::c_int>,
    }
    use super::xdr_h::{XDR, xdrproc_t};
    use super::sys_types_h::caddr_t;
    /* !defined(GSSRPC_SVC_AUTH_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:61"]
pub mod verto_h {
    #[c2rust::src_loc = "51:9"]
    pub type verto_ev_type = libc::c_uint;
    #[c2rust::src_loc = "57:5"]
    pub const VERTO_EV_TYPE_CHILD: verto_ev_type = 16;
    #[c2rust::src_loc = "56:5"]
    pub const VERTO_EV_TYPE_SIGNAL: verto_ev_type = 8;
    #[c2rust::src_loc = "55:5"]
    pub const VERTO_EV_TYPE_IDLE: verto_ev_type = 4;
    #[c2rust::src_loc = "54:5"]
    pub const VERTO_EV_TYPE_TIMEOUT: verto_ev_type = 2;
    #[c2rust::src_loc = "53:5"]
    pub const VERTO_EV_TYPE_IO: verto_ev_type = 1;
    #[c2rust::src_loc = "52:5"]
    pub const VERTO_EV_TYPE_NONE: verto_ev_type = 0;
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
    extern "C" {
        #[c2rust::src_loc = "48:16"]
        pub type verto_ctx;
        #[c2rust::src_loc = "49:16"]
        pub type verto_ev;
        /* *
 * Gets the default event context using an optionally specified implementation.
 *
 * This function is essentially a singleton version of verto_new().  However,
 * since this function must return the same loop as the *_default() call of
 * the underlying implementation (if such a function exists), it is NOT a
 * global singleton, but a per-implementation singleton. For this reason, you
 * must call verto_free() when you are done with this loop. Even after calling
 * verto_free() on the default verto_ctx, you can safely call verto_default()
 * again and receive a new reference to the same (internally default) loop.
 *
 * In all other respects, verto_default() acts exactly like verto_new().
 *
 * @see verto_new()
 * @see verto_free()
 * @param impl The implementation to use, or NULL.
 * @param reqtypes A bitwise or'd list of required event type features.
 * @return The default verto_ctx, or NULL on error.  Call verto_free() when done.
 */
        #[no_mangle]
        #[c2rust::src_loc = "159:1"]
        pub fn verto_default(impl_0: *const libc::c_char,
                             reqtypes: verto_ev_type) -> *mut verto_ctx;
        /* *
 * Frees a verto_ctx.
 *
 * When called on a default verto_ctx, the reference will be freed but the
 * internal default loop will still be available via another call to
 * verto_default().
 *
 * @see verto_new()
 * @see verto_default()
 * @param ctx The verto_ctx to free.
 */
        #[no_mangle]
        #[c2rust::src_loc = "217:1"]
        pub fn verto_free(ctx: *mut verto_ctx);
        /* *
 * Exits the currently running verto_ctx.
 *
 * @see verto_run()
 * @param ctx The verto_ctx to exit.
 */
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn verto_break(ctx: *mut verto_ctx);
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
        #[no_mangle]
        #[c2rust::src_loc = "300:1"]
        pub fn verto_add_io(ctx: *mut verto_ctx, flags: verto_ev_flag,
                            callback: Option<verto_callback>, fd: libc::c_int)
         -> *mut verto_ev;
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
        #[no_mangle]
        #[c2rust::src_loc = "379:1"]
        pub fn verto_add_signal(ctx: *mut verto_ctx, flags: verto_ev_flag,
                                callback: Option<verto_callback>,
                                signal: libc::c_int) -> *mut verto_ev;
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
        #[no_mangle]
        #[c2rust::src_loc = "416:1"]
        pub fn verto_set_private(ev: *mut verto_ev, priv_0: *mut libc::c_void,
                                 free_0: Option<verto_callback>);
        /* *
 * Gets the private pointer of the verto_ev.
 *
 * @see verto_set_private()
 * @param ev The verto_ev
 * @return The verto_ev private pointer
 */
        #[no_mangle]
        #[c2rust::src_loc = "426:1"]
        pub fn verto_get_private(ev: *const verto_ev) -> *mut libc::c_void;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/net-server.h:61"]
pub mod net_server_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct _krb5_fulladdr {
        pub address: *mut krb5_address,
        pub port: krb5_ui_4,
    }
    #[c2rust::src_loc = "37:1"]
    pub type krb5_fulladdr = _krb5_fulladdr;
    #[c2rust::src_loc = "87:1"]
    pub type loop_respond_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: krb5_error_code,
                                    _: *mut krb5_data) -> ()>;
    use super::krb5_h::{krb5_address, krb5_ui_4, krb5_error_code, krb5_data,
                        krb5_context};
    use super::verto_h::verto_ctx;
    extern "C" {
        /* to be supplied by the server application */
        /*
 * Two routines for processing an incoming message and getting a
 * result to send back.
 *
 * The first, dispatch(), is for normal processing of a request.  The
 * second, make_toolong_error(), is obviously for generating an error
 * to send back when the incoming message is bigger than
 * the main loop can accept.
 */
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn dispatch(handle: *mut libc::c_void,
                        local_addr: *const krb5_fulladdr,
                        remote_addr: *const krb5_fulladdr,
                        request: *mut krb5_data, is_tcp: libc::c_int,
                        vctx: *mut verto_ctx, respond: loop_respond_fn,
                        arg: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn make_toolong_error(handle: *mut libc::c_void,
                                  _: *mut *mut krb5_data) -> krb5_error_code;
        /*
 * Contexts are needed in lots of places.  Opaque application-provided
 * handles are passed around in lots of place, but contexts are not.
 * For now, we'll require that the application provide us an easy way
 * to get at a context; eventually it should probably be explicity.
 */
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn get_context(handle: *mut libc::c_void) -> krb5_context;
    }
    /* NET_SERVER_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/apputils/udppktinfo.h:65"]
pub mod udppktinfo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:15"]
    pub union aux_addressing_info {
        pub ipv6_ifindex: libc::c_int,
    }
    use super::krb5_h::krb5_error_code;
    use super::stddef_h::size_t;
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn set_pktinfo(sock: libc::c_int, family: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub fn recv_from_to(sock: libc::c_int, buf: *mut libc::c_void,
                            len: size_t, flags: libc::c_int,
                            from: *mut sockaddr, fromlen: *mut socklen_t,
                            to: *mut sockaddr, tolen: *mut socklen_t,
                            auxaddr: *mut aux_addressing_info)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn send_to_from(sock: libc::c_int, buf: *mut libc::c_void,
                            len: size_t, flags: libc::c_int,
                            to: *const sockaddr, tolen: socklen_t,
                            from: *mut sockaddr, fromlen: socklen_t,
                            auxaddr: *mut aux_addressing_info)
         -> krb5_error_code;
    }
    /* UDPPKTINFO_H */
}
#[c2rust::header_src = "/usr/include/fcntl.h:27"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:27"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:27"]
pub mod time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "345:1"]
        pub fn strtok_r(__s: *mut libc::c_char, __delim: *const libc::c_char,
                        __save_ptr: *mut *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/socket-utils.h:27"]
pub mod socket_utils_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2005 by the Massachusetts Institute of Technology,
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
    /* Some useful stuff cross-platform for manipulating socket addresses.
   We assume at least ipv4 sockaddr_in support.  The sockaddr_storage
   stuff comes from the ipv6 socket api enhancements; socklen_t is
   provided on some systems; the rest is just convenience for internal
   use in the krb5 tree.

   Do NOT install this file.  */
    /* for HAVE_SOCKLEN_T etc */
    /* for sockaddr_storage */
    /* for "inline" if needed */
    /*
 * There's a lot of confusion between pointers to different sockaddr
 * types, and pointers with different degrees of indirection, as in
 * the locate_kdc type functions.  Use these function to ensure we
 * don't do something silly like cast a "sockaddr **" to a
 * "sockaddr_in *".
 *
 * The casts to (void *) are to get GCC to shut up about alignment
 * increasing.
 */
    #[inline]
    #[c2rust::src_loc = "71:1"]
    pub unsafe extern "C" fn sa2sin(mut sa: *mut sockaddr)
     -> *mut sockaddr_in {
        return sa as *mut libc::c_void as *mut sockaddr_in;
    }
    #[inline]
    #[c2rust::src_loc = "75:1"]
    pub unsafe extern "C" fn sa2sin6(mut sa: *mut sockaddr)
     -> *mut sockaddr_in6 {
        return sa as *mut libc::c_void as *mut sockaddr_in6;
    }
    #[inline]
    #[c2rust::src_loc = "79:1"]
    pub unsafe extern "C" fn ss2sa(mut ss: *mut sockaddr_storage)
     -> *mut sockaddr {
        return ss as *mut sockaddr;
    }
    /* Set the IPv4 or IPv6 port on sa to port.  Do nothing if sa is not an
 * Internet socket. */
    #[inline]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn sa_setport(mut sa: *mut sockaddr,
                                        mut port: uint16_t) {
        if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
            (*sa2sin(sa)).sin_port = htons(port)
        } else if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            (*sa2sin6(sa)).sin6_port = htons(port)
        };
    }
    /* Return true if sa is an IPv4 or IPv6 wildcard address. */
    #[inline]
    #[c2rust::src_loc = "123:1"]
    pub unsafe extern "C" fn sa_is_wildcard(mut sa: *mut sockaddr)
     -> libc::c_int {
        if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            return ({
                        let mut __a: *const in6_addr =
                            &mut (*(sa2sin6 as
                                        unsafe extern "C" fn(_: *mut sockaddr)
                                            ->
                                                *mut sockaddr_in6)(sa)).sin6_addr
                                as *mut in6_addr as *const in6_addr;
                        ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[1 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[2 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint &&
                             (*__a).__in6_u.__u6_addr32[3 as libc::c_int as
                                                            usize] ==
                                 0 as libc::c_int as libc::c_uint) as
                            libc::c_int
                    })
        } else {
            if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
                return ((*sa2sin(sa)).sin_addr.s_addr ==
                            0 as libc::c_int as in_addr_t) as libc::c_int
            }
        }
        return 0 as libc::c_int;
    }
    /* Return the length of an IPv4 or IPv6 socket structure; abort if it is
 * neither. */
    #[inline]
    #[c2rust::src_loc = "135:1"]
    pub unsafe extern "C" fn sa_socklen(mut sa: *mut sockaddr) -> socklen_t {
        if (*sa).sa_family as libc::c_int == 10 as libc::c_int {
            return ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                       socklen_t
        } else if (*sa).sa_family as libc::c_int == 2 as libc::c_int {
            return ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                       socklen_t
        } else { abort(); };
    }
    use super::socket_h::{sockaddr, sockaddr_storage};
    use super::in_h::{sockaddr_in, sockaddr_in6, htons, in_port_t, in6_addr,
                      in_addr_t};
    use super::stdint_uintn_h::uint16_t;
    use super::sockaddr_h::sa_family_t;
    use super::unistd_h::socklen_t;
    use super::stdlib_h::abort;
    /* SOCKET_UTILS_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:28"]
pub mod adm_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "53:1"]
        pub fn krb5_klog_syslog(_: libc::c_int, _: *const libc::c_char,
                                _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "58:1"]
        pub fn krb5_klog_reopen(_: krb5_context);
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src = "/usr/include/sys/ioctl.h:29"]
pub mod ioctl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:60"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn krb5int_gai_strerror(err: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn krb5int_getnameinfo(sa: *const sockaddr, salen: socklen_t,
                                   hbuf: *mut libc::c_char, hbuflen: size_t,
                                   sbuf: *mut libc::c_char, sbuflen: size_t,
                                   flags: libc::c_int) -> libc::c_int;
    }
    /* FAI_DEFINED */
}
use c2rust_asm_casts::AsmCastTrait;
pub use self::types_h::{__u_short, __u_int, __u_long, __uint8_t, __uint16_t,
                        __int32_t, __uint32_t, __time_t, __ssize_t, __caddr_t,
                        __socklen_t};
pub use self::sys_types_h::{u_short, u_int, u_long, ssize_t, caddr_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::select_h::{__fd_mask, fd_set};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_iovec_h::iovec;
pub use self::unistd_h::{socklen_t, close, read};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be, krb5int_strlcpy};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_addrtype, krb5_enctype, krb5_flags, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       _krb5_address, krb5_address, krb5_post_recv_fn,
                       krb5_context, krb5_pre_send_fn, krb5_trace_callback,
                       krb5_trace_info, _krb5_trace_info, krb5_prompt_type,
                       _profile_t, krb5_free_data};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         k5_parse_host_string};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err, error_message};
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
                             sockaddr_at, socket, bind, getsockname,
                             getpeername, sendmsg, setsockopt, listen,
                             accept};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed_2, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_3,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, ntohs, htonl, htons};
pub use self::netdb_h::addrinfo;
pub use self::port_sockets_h::{sg_buf, socket_sendmsg};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, opaque_auth};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_4, reply_body,
                          C2RustUnnamed_5, rejected_reply, C2RustUnnamed_6,
                          C2RustUnnamed_7, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_8,
                          C2RustUnnamed_9, C2RustUnnamed_10, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL};
pub use self::svc_h::{svc_req, SVCXPRT, xp_ops, xprt_stat, XPRT_IDLE,
                      XPRT_MOREREQS, XPRT_DIED, gssrpc_svc_fdset,
                      gssrpc_svc_register, gssrpc_svc_getreqset,
                      gssrpc_svctcp_create};
pub use self::svc_auth_h::{SVCAUTH, svc_auth_ops};
pub use self::verto_h::{verto_ev_type, VERTO_EV_TYPE_CHILD,
                        VERTO_EV_TYPE_SIGNAL, VERTO_EV_TYPE_IDLE,
                        VERTO_EV_TYPE_TIMEOUT, VERTO_EV_TYPE_IO,
                        VERTO_EV_TYPE_NONE, verto_ev_flag, _VERTO_EV_FLAG_MAX,
                        _VERTO_EV_FLAG_MUTABLE_MASK,
                        VERTO_EV_FLAG_REINITIABLE, VERTO_EV_FLAG_IO_CLOSE_FD,
                        VERTO_EV_FLAG_IO_ERROR, VERTO_EV_FLAG_IO_WRITE,
                        VERTO_EV_FLAG_IO_READ, VERTO_EV_FLAG_PRIORITY_HIGH,
                        VERTO_EV_FLAG_PRIORITY_MEDIUM,
                        VERTO_EV_FLAG_PRIORITY_LOW, VERTO_EV_FLAG_PERSIST,
                        VERTO_EV_FLAG_NONE, verto_callback, verto_ctx,
                        verto_ev, verto_default, verto_free, verto_break,
                        verto_add_io, verto_add_signal, verto_set_private,
                        verto_get_private, verto_get_fd, verto_del};
pub use self::net_server_h::{_krb5_fulladdr, krb5_fulladdr, loop_respond_fn,
                             dispatch, make_toolong_error, get_context};
pub use self::udppktinfo_h::{aux_addressing_info, set_pktinfo, recv_from_to,
                             send_to_from};
use self::fcntl_h::fcntl;
use self::errno_h::__errno_location;
use self::time_h::time;
use self::libintl_h::dgettext;
use self::stdlib_h::{exit, abort, free, realloc, malloc};
use self::string_h::{strerror, strlen, strtok_r, strdup, strcmp, strncpy,
                     memset};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::__bswap_32;
pub use self::socket_utils_h::{sa2sin, sa2sin6, ss2sa, sa_setport,
                               sa_is_wildcard, sa_socklen};
use self::adm_proto_h::{krb5_klog_syslog, krb5_klog_reopen};
use self::ioctl_h::ioctl;
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_freeaddrinfo,
                            krb5int_gai_strerror, krb5int_getnameinfo};
/* Per-connection info.  */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "125:8"]
pub struct connection {
    pub handle: *mut libc::c_void,
    pub prog: *const libc::c_char,
    pub type_0: conn_type,
    pub addr_s: sockaddr_storage,
    pub addrlen: socklen_t,
    pub addrbuf: [libc::c_char; 56],
    pub remote_addr_buf: krb5_address,
    pub remote_addr: krb5_fulladdr,
    pub bufsiz: size_t,
    pub offset: size_t,
    pub buffer: *mut libc::c_char,
    pub msglen: size_t,
    pub response: *mut krb5_data,
    pub lenbuf: [libc::c_uchar; 4],
    pub sgbuf: [sg_buf; 2],
    pub sgp: *mut sg_buf,
    pub sgnum: libc::c_int,
    pub start_time: time_t,
    pub transp: *mut SVCXPRT,
    pub rpc_force_close: libc::c_int,
}
/* KDC data.  */
#[c2rust::src_loc = "110:1"]
pub type conn_type = libc::c_uint;
#[c2rust::src_loc = "111:63"]
pub const CONN_RPC: conn_type = 4;
#[c2rust::src_loc = "111:44"]
pub const CONN_RPC_LISTENER: conn_type = 3;
#[c2rust::src_loc = "111:34"]
pub const CONN_TCP: conn_type = 2;
#[c2rust::src_loc = "111:15"]
pub const CONN_TCP_LISTENER: conn_type = 1;
#[c2rust::src_loc = "111:5"]
pub const CONN_UDP: conn_type = 0;
/* Start at the top and work down -- this should allow for deletions
   without disrupting the iteration, since we delete by overwriting
   the element to be removed with the last element.  */
/* overflow */
/* 1 = success, 0 = failure */
/*
 * N.B.: The Emacs cc-mode indentation code seems to get confused if
 * the macro argument here is one word only.  So use "unsigned short"
 * instead of the "u_short" we were using before.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "193:8"]
pub struct rpc_svc_data {
    pub prognum: u_long,
    pub versnum: u_long,
    pub dispatch: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "1197:8"]
pub struct tcp_dispatch_state {
    pub local_saddr: sockaddr_storage,
    pub local_addr_buf: krb5_address,
    pub local_addr: krb5_fulladdr,
    pub conn: *mut connection,
    pub request: krb5_data,
    pub ctx: *mut verto_ctx,
    pub sock: libc::c_int,
}
#[c2rust::src_loc = "114:1"]
pub type bind_type = libc::c_uint;
#[c2rust::src_loc = "115:15"]
pub const RPC: bind_type = 2;
#[c2rust::src_loc = "115:10"]
pub const TCP: bind_type = 1;
#[c2rust::src_loc = "115:5"]
pub const UDP: bind_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "199:8"]
pub struct bind_address {
    pub address: *mut libc::c_char,
    pub port: u_short,
    pub type_0: bind_type,
    pub rpc_svc_data: rpc_svc_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "207:8"]
pub struct C2RustUnnamed_11 {
    pub data: *mut bind_address,
    pub n: size_t,
    pub max: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "206:8"]
pub struct C2RustUnnamed_12 {
    pub data: *mut *mut verto_ev,
    pub n: size_t,
    pub max: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "950:8"]
pub struct udp_dispatch_state {
    pub handle: *mut libc::c_void,
    pub prog: *const libc::c_char,
    pub port_fd: libc::c_int,
    pub remote_addr_buf: krb5_address,
    pub remote_addr: krb5_fulladdr,
    pub local_addr_buf: krb5_address,
    pub local_addr: krb5_fulladdr,
    pub saddr_len: socklen_t,
    pub daddr_len: socklen_t,
    pub saddr: sockaddr_storage,
    pub daddr: sockaddr_storage,
    pub auxaddr: aux_addressing_info,
    pub request: krb5_data,
    pub pktbuf: [libc::c_char; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "225:8"]
pub struct sighup_context {
    pub handle: *mut libc::c_void,
    pub reset: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/apputils/net-server.c - Network code for krb5 servers (kdc, kadmind) */
/*
 * Copyright 1990,2000,2007,2008,2009,2010,2016 by the Massachusetts Institute
 * of Technology.
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
/* OpenBSD breaks on multiple inclusions */
/* XXX */
#[c2rust::src_loc = "70:12"]
static mut tcp_or_rpc_data_counter: libc::c_int = 0;
#[c2rust::src_loc = "71:12"]
static mut max_tcp_or_rpc_data_connections: libc::c_int = 45 as libc::c_int;
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn setreuseaddr(mut sock: libc::c_int,
                                  mut value: libc::c_int) -> libc::c_int {
    return setsockopt(sock, 1 as libc::c_int, 2 as libc::c_int,
                      &mut value as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t);
}
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn setv6only(mut sock: libc::c_int, mut value: libc::c_int)
 -> libc::c_int {
    return setsockopt(sock, IPPROTO_IPV6 as libc::c_int, 26 as libc::c_int,
                      &mut value as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t);
}
#[c2rust::src_loc = "87:1"]
unsafe extern "C" fn paddr(mut sa: *mut sockaddr) -> *const libc::c_char {
    static mut buf: [libc::c_char; 100] = [0; 100];
    let mut portbuf: [libc::c_char; 10] = [0; 10];
    if krb5int_getnameinfo(sa, sa_socklen(sa), buf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 100]>() as
                               libc::c_ulong, portbuf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong,
                           1 as libc::c_int | 2 as libc::c_int) != 0 {
        krb5int_strlcpy(buf.as_mut_ptr(),
                        b"<unprintable>\x00" as *const u8 as
                            *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 100]>() as
                            libc::c_ulong);
    } else {
        let mut len: libc::c_uint =
            (::std::mem::size_of::<[libc::c_char; 100]>() as
                 libc::c_ulong).wrapping_sub(strlen(buf.as_mut_ptr())) as
                libc::c_uint;
        let mut p: *mut libc::c_char =
            buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
        if len as libc::c_ulong >
               (2 as libc::c_int as
                    libc::c_ulong).wrapping_add(strlen(portbuf.as_mut_ptr()))
           {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '.' as i32 as libc::c_char;
            len = len.wrapping_sub(1);
            strncpy(p, portbuf.as_mut_ptr(), len as libc::c_ulong);
        }
    }
    return buf.as_mut_ptr();
}
#[c2rust::src_loc = "118:26"]
static mut bind_type_names: [*const libc::c_char; 3] =
    [b"UDP\x00" as *const u8 as *const libc::c_char,
     b"TCP\x00" as *const u8 as *const libc::c_char,
     b"RPC\x00" as *const u8 as *const libc::c_char];
#[c2rust::src_loc = "206:24"]
static mut events: C2RustUnnamed_12 =
    C2RustUnnamed_12{data: 0 as *const *mut verto_ev as *mut *mut verto_ev,
                     n: 0,
                     max: 0,};
#[c2rust::src_loc = "207:33"]
static mut bind_addresses: C2RustUnnamed_11 =
    C2RustUnnamed_11{data: 0 as *const bind_address as *mut bind_address,
                     n: 0,
                     max: 0,};
#[no_mangle]
#[c2rust::src_loc = "209:1"]
pub unsafe extern "C" fn loop_init(mut types: verto_ev_type)
 -> *mut verto_ctx {
    types =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_type>(types as libc::c_uint |
                                                   VERTO_EV_TYPE_IO as
                                                       libc::c_int as
                                                       libc::c_uint);
    types =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_type>(types as libc::c_uint |
                                                   VERTO_EV_TYPE_SIGNAL as
                                                       libc::c_int as
                                                       libc::c_uint);
    types =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_type>(types as libc::c_uint |
                                                   VERTO_EV_TYPE_TIMEOUT as
                                                       libc::c_int as
                                                       libc::c_uint);
    return verto_default(0 as *const libc::c_char, types);
}
#[c2rust::src_loc = "218:1"]
unsafe extern "C" fn do_break(mut ctx: *mut verto_ctx,
                              mut ev: *mut verto_ev) {
    krb5_klog_syslog(7 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Got signal to request exit\x00" as *const u8
                                  as *const libc::c_char));
    verto_break(ctx);
}
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn do_reset(mut ctx: *mut verto_ctx,
                              mut ev: *mut verto_ev) {
    let mut sc: *mut sighup_context =
        verto_get_private(ev) as *mut sighup_context;
    krb5_klog_syslog(7 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Got signal to reset\x00" as *const u8 as
                                  *const libc::c_char));
    krb5_klog_reopen(get_context((*sc).handle));
    if (*sc).reset.is_some() {
        (*sc).reset.expect("non-null function pointer")((*sc).handle);
    };
}
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn free_sighup_context(mut ctx: *mut verto_ctx,
                                         mut ev: *mut verto_ev) {
    free(verto_get_private(ev));
}
#[no_mangle]
#[c2rust::src_loc = "247:1"]
pub unsafe extern "C" fn loop_setup_signals(mut ctx: *mut verto_ctx,
                                            mut handle: *mut libc::c_void,
                                            mut reset:
                                                Option<unsafe extern "C" fn()
                                                           -> ()>)
 -> krb5_error_code {
    let mut sc: *mut sighup_context = 0 as *mut sighup_context;
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    if verto_add_signal(ctx, VERTO_EV_FLAG_PERSIST,
                        Some(do_break as
                                 unsafe extern "C" fn(_: *mut verto_ctx,
                                                      _: *mut verto_ev)
                                     -> ()), 2 as libc::c_int).is_null() ||
           verto_add_signal(ctx, VERTO_EV_FLAG_PERSIST,
                            Some(do_break as
                                     unsafe extern "C" fn(_: *mut verto_ctx,
                                                          _: *mut verto_ev)
                                         -> ()), 15 as libc::c_int).is_null()
           ||
           verto_add_signal(ctx, VERTO_EV_FLAG_PERSIST,
                            Some(do_break as
                                     unsafe extern "C" fn(_: *mut verto_ctx,
                                                          _: *mut verto_ev)
                                         -> ()), 3 as libc::c_int).is_null()
           ||
           verto_add_signal(ctx, VERTO_EV_FLAG_PERSIST,
                            ::std::mem::transmute::<libc::intptr_t,
                                                    Option<verto_callback>>(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::intptr_t),
                            13 as libc::c_int).is_null() {
        return 12 as libc::c_int
    }
    ev =
        verto_add_signal(ctx, VERTO_EV_FLAG_PERSIST,
                         Some(do_reset as
                                  unsafe extern "C" fn(_: *mut verto_ctx,
                                                       _: *mut verto_ev)
                                      -> ()), 1 as libc::c_int);
    if ev.is_null() { return 12 as libc::c_int }
    sc =
        malloc(::std::mem::size_of::<sighup_context>() as libc::c_ulong) as
            *mut sighup_context;
    if sc.is_null() { return 12 as libc::c_int }
    (*sc).handle = handle;
    (*sc).reset =
        ::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           -> ()>>(reset);
    verto_set_private(ev, sc as *mut libc::c_void,
                      Some(free_sighup_context as
                               unsafe extern "C" fn(_: *mut verto_ctx,
                                                    _: *mut verto_ev) -> ()));
    return 0 as libc::c_int;
}
/*
 * Add a bind address to the loop.
 *
 * Arguments:
 * - address
 *      A string for the address (or hostname).  Pass NULL to use the wildcard
 *      address.  The address is parsed with k5_parse_host_string().
 * - port
 *      What port the socket should be set to.
 * - type
 *      bind_type for the socket.
 * - rpc_data
 *      For RPC addresses, the svc_register() arguments to use when TCP
 *      connections are created.  Ignored for other types.
 */
#[c2rust::src_loc = "288:1"]
unsafe extern "C" fn loop_add_address(mut address: *const libc::c_char,
                                      mut port: libc::c_int,
                                      mut type_0: bind_type,
                                      mut rpc_data: *mut rpc_svc_data)
 -> krb5_error_code {
    let mut addr: bind_address =
        bind_address{address: 0 as *mut libc::c_char,
                     port: 0,
                     type_0: UDP,
                     rpc_svc_data:
                         rpc_svc_data{prognum: 0,
                                      versnum: 0,
                                      dispatch: None,},};
    let mut val: bind_address =
        bind_address{address: 0 as *mut libc::c_char,
                     port: 0,
                     type_0: UDP,
                     rpc_svc_data:
                         rpc_svc_data{prognum: 0,
                                      versnum: 0,
                                      dispatch: None,},};
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut addr_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(type_0 as libc::c_uint == RPC as libc::c_int as libc::c_uint &&
             rpc_data.is_null()) {
    } else {
        __assert_fail(b"!(type == RPC && rpc_data == NULL)\x00" as *const u8
                          as *const libc::c_char,
                      b"net-server.c\x00" as *const u8 as *const libc::c_char,
                      297 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 91],
                                                &[libc::c_char; 91]>(b"krb5_error_code loop_add_address(const char *, int, enum bind_type, struct rpc_svc_data *)\x00")).as_ptr());
    }
    /* Make sure a valid port number was passed. */
    if port < 0 as libc::c_int || port > 65535 as libc::c_int {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Invalid port %d\x00" as *const u8 as
                                      *const libc::c_char), port);
        return 22 as libc::c_int
    }
    /* Check for conflicting addresses. */
    i =
        bind_addresses.n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    while i >= 0 as libc::c_int &&
              {
                  val = *bind_addresses.data.offset(i as isize);
                  (1 as libc::c_int) != 0
              } {
        if !(type_0 as libc::c_uint != val.type_0 as libc::c_uint ||
                 port != val.port as libc::c_int) {
            /* If a wildcard address is being added, make sure to remove any direct
         * addresses. */
            if address.is_null() && !val.address.is_null() {
                krb5_klog_syslog(7 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Removing address %s since wildcard address is being added\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 val.address);
                free(val.address as *mut libc::c_void);
                bind_addresses.n = bind_addresses.n.wrapping_sub(1);
                *bind_addresses.data.offset(i as isize) =
                    *bind_addresses.data.offset(bind_addresses.n as isize)
            } else if val.address.is_null() ||
                          strcmp(address, val.address) == 0 {
                krb5_klog_syslog(7 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Address already added to server\x00"
                                              as *const u8 as
                                              *const libc::c_char));
                return 0 as libc::c_int
            }
        }
        i -= 1
    }
    /* Copy the address if it is specified. */
    if !address.is_null() {
        addr_copy = strdup(address);
        if addr_copy.is_null() { return 12 as libc::c_int }
    }
    /* Add the new address to bind_addresses. */
    memset(&mut addr as *mut bind_address as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<bind_address>() as libc::c_ulong);
    addr.address = addr_copy;
    addr.port = port as u_short;
    addr.type_0 = type_0;
    if !rpc_data.is_null() { addr.rpc_svc_data = *rpc_data }
    if if bind_addresses.n < bind_addresses.max ||
              (if bind_addresses.max.wrapping_add(10 as libc::c_int as
                                                      libc::c_ulong) <
                      bind_addresses.max ||
                      bind_addresses.max.wrapping_add(10 as libc::c_int as
                                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<bind_address>()
                                                                                          as
                                                                                          libc::c_ulong).wrapping_div(::std::mem::size_of::<bind_address>()
                                                                                                                          as
                                                                                                                          libc::c_ulong)
                          !=
                          bind_addresses.max.wrapping_add(10 as libc::c_int as
                                                              libc::c_ulong) {
                   0 as libc::c_int
               } else {
                   tmp =
                       realloc(bind_addresses.data as *mut libc::c_void,
                               bind_addresses.max.wrapping_add(10 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<bind_address>()
                                                                                                   as
                                                                                                   libc::c_ulong));
                   (if !tmp.is_null() {
                        bind_addresses.data = tmp as *mut bind_address;
                        bind_addresses.max =
                            (bind_addresses.max as
                                 libc::c_ulong).wrapping_add(10 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t;
                        1 as libc::c_int
                    } else { 0 as libc::c_int })
               }) != 0 {
           let fresh1 = bind_addresses.n;
           bind_addresses.n = bind_addresses.n.wrapping_add(1);
           *bind_addresses.data.offset(fresh1 as isize) = addr;
           1 as libc::c_int
       } else { 0 as libc::c_int } == 0 {
        free(addr_copy as *mut libc::c_void);
        return 12 as libc::c_int
    }
    return 0 as libc::c_int;
}
/*
 * Add bind addresses to the loop.
 *
 * Arguments:
 *
 * - addresses
 *      A string for the addresses.  Pass NULL to use the wildcard address.
 *      Supported delimeters can be found in ADDRESSES_DELIM.  Addresses are
 *      parsed with k5_parse_host_name().
 * - default_port
 *      What port the socket should be set to if not specified in addresses.
 * - type
 *      bind_type for the socket.
 * - rpc_data
 *      For RPC addresses, the svc_register() arguments to use when TCP
 *      connections are created.  Ignored for other types.
 */
#[c2rust::src_loc = "365:1"]
unsafe extern "C" fn loop_add_addresses(mut addresses: *const libc::c_char,
                                        mut default_port: libc::c_int,
                                        mut type_0: bind_type,
                                        mut rpc_data: *mut rpc_svc_data)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut addresses_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut saveptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: libc::c_int = 0;
    /* If no addresses are set, add a wildcard address. */
    if addresses.is_null() {
        return loop_add_address(0 as *const libc::c_char, default_port,
                                type_0, rpc_data)
    }
    /* Copy the addresses string before using strtok(). */
    addresses_copy = strdup(addresses);
    if addresses_copy.is_null() {
        ret = 12 as libc::c_int
    } else {
        /* Start tokenizing the addresses string.  If we get NULL the string
     * contained no addresses, so add a wildcard address. */
        addr =
            strtok_r(addresses_copy,
                     b",; \x00" as *const u8 as *const libc::c_char,
                     &mut saveptr);
        if addr.is_null() {
            ret =
                loop_add_address(0 as *const libc::c_char, default_port,
                                 type_0, rpc_data)
        } else {
            /* Loop through each address and add it to the loop. */
            while !addr.is_null() {
                /* Parse the host string. */
                ret =
                    k5_parse_host_string(addr, default_port, &mut host,
                                         &mut port);
                if ret != 0 { break ; }
                ret = loop_add_address(host, port, type_0, rpc_data);
                if ret != 0 { break ; }
                free(host as *mut libc::c_void);
                host = 0 as *mut libc::c_char;
                addr =
                    strtok_r(0 as *mut libc::c_char,
                             b",; \x00" as *const u8 as *const libc::c_char,
                             &mut saveptr)
            }
        }
    }
    free(addresses_copy as *mut libc::c_void);
    free(host as *mut libc::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "413:1"]
pub unsafe extern "C" fn loop_add_udp_address(mut default_port: libc::c_int,
                                              mut addresses:
                                                  *const libc::c_char)
 -> krb5_error_code {
    return loop_add_addresses(addresses, default_port, UDP,
                              0 as *mut rpc_svc_data);
}
#[no_mangle]
#[c2rust::src_loc = "419:1"]
pub unsafe extern "C" fn loop_add_tcp_address(mut default_port: libc::c_int,
                                              mut addresses:
                                                  *const libc::c_char)
 -> krb5_error_code {
    return loop_add_addresses(addresses, default_port, TCP,
                              0 as *mut rpc_svc_data);
}
#[no_mangle]
#[c2rust::src_loc = "425:1"]
pub unsafe extern "C" fn loop_add_rpc_service(mut default_port: libc::c_int,
                                              mut addresses:
                                                  *const libc::c_char,
                                              mut prognum: u_long,
                                              mut versnum: u_long,
                                              mut dispatchfn:
                                                  Option<unsafe extern "C" fn()
                                                             -> ()>)
 -> krb5_error_code {
    let mut svc: rpc_svc_data =
        rpc_svc_data{prognum: 0, versnum: 0, dispatch: None,};
    svc.prognum = prognum;
    svc.versnum = versnum;
    svc.dispatch = dispatchfn;
    return loop_add_addresses(addresses, default_port, RPC, &mut svc);
}
#[c2rust::src_loc = "443:1"]
unsafe extern "C" fn free_connection(mut conn: *mut connection) {
    if conn.is_null() { return }
    if !(*conn).response.is_null() {
        krb5_free_data(get_context((*conn).handle), (*conn).response);
    }
    if !(*conn).buffer.is_null() {
        free((*conn).buffer as *mut libc::c_void);
    }
    if (*conn).type_0 as libc::c_uint ==
           CONN_RPC_LISTENER as libc::c_int as libc::c_uint &&
           !(*conn).transp.is_null() {
        Some((*(*(*conn).transp).xp_ops).xp_destroy.expect("non-null function pointer")).expect("non-null function pointer")((*conn).transp);
    }
    free(conn as *mut libc::c_void);
}
#[c2rust::src_loc = "457:1"]
unsafe extern "C" fn remove_event_from_set(mut ev: *mut verto_ev) {
    let mut tmp: *mut verto_ev = 0 as *mut verto_ev;
    let mut i: libc::c_int = 0;
    /* Remove the event from the events. */
    i =
        events.n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    while i >= 0 as libc::c_int &&
              {
                  tmp = *events.data.offset(i as isize);
                  (1 as libc::c_int) != 0
              } {
        if tmp == ev {
            events.n = events.n.wrapping_sub(1);
            let ref mut fresh2 = *events.data.offset(i as isize);
            *fresh2 = *events.data.offset(events.n as isize);
            break ;
        } else { i -= 1 }
    };
}
#[c2rust::src_loc = "471:1"]
unsafe extern "C" fn free_socket(mut ctx: *mut verto_ctx,
                                 mut ev: *mut verto_ev) {
    let mut conn: *mut connection = 0 as *mut connection;
    let mut fds: fd_set = fd_set{fds_bits: [0; 16],};
    let mut fd: libc::c_int = 0;
    remove_event_from_set(ev);
    fd = verto_get_fd(ev);
    conn = verto_get_private(ev) as *mut connection;
    /* Close the file descriptor. */
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"closing down fd %d\x00" as *const u8 as
                                  *const libc::c_char), fd);
    if fd >= 0 as libc::c_int &&
           (conn.is_null() ||
                (*conn).type_0 as libc::c_uint !=
                    CONN_RPC as libc::c_int as libc::c_uint ||
                (*conn).rpc_force_close != 0) {
        close(fd);
    }
    /* Free the connection struct. */
    if !conn.is_null() {
        let mut current_block_16: u64;
        match (*conn).type_0 as libc::c_uint {
            4 => {
                if (*conn).rpc_force_close != 0 {
                    let mut __d0: libc::c_int = 0;
                    let mut __d1: libc::c_int = 0;
                    let fresh3 = &mut __d0;
                    let fresh4;
                    let fresh5 = &mut __d1;
                    let fresh6;
                    let fresh7 =
                        (::std::mem::size_of::<fd_set>() as
                             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>()
                                                             as
                                                             libc::c_ulong);
                    let fresh8 =
                        &mut *fds.fds_bits.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize)
                            as *mut __fd_mask;
                    asm!("cld; rep; stosq" : "={cx}" (fresh4), "={di}"
                         (fresh6) : "{ax}" (0 as libc::c_int), "0"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh7)),
                         "1"
                         (c2rust_asm_casts::AsmCast::cast_in(fresh5, fresh8))
                         : "memory" : "volatile");
                    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh7,
                                                        fresh4);
                    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh8,
                                                        fresh6);
                    fds.fds_bits[(fd /
                                      (8 as libc::c_int *
                                           ::std::mem::size_of::<__fd_mask>()
                                               as libc::c_ulong as
                                               libc::c_int)) as usize] |=
                        ((1 as libc::c_ulong) <<
                             fd %
                                 (8 as libc::c_int *
                                      ::std::mem::size_of::<__fd_mask>() as
                                          libc::c_ulong as libc::c_int)) as
                            __fd_mask;
                    gssrpc_svc_getreqset(&mut fds);
                    if gssrpc_svc_fdset.fds_bits[(fd /
                                                      (8 as libc::c_int *
                                                           ::std::mem::size_of::<__fd_mask>()
                                                               as
                                                               libc::c_ulong
                                                               as
                                                               libc::c_int))
                                                     as usize] &
                           ((1 as libc::c_ulong) <<
                                fd %
                                    (8 as libc::c_int *
                                         ::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong as libc::c_int)) as
                               __fd_mask != 0 as libc::c_int as libc::c_long {
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"descriptor %d closed but still in svc_fdset\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         fd);
                    }
                }
                current_block_16 = 2749206429124311488;
            }
            2 => { current_block_16 = 2749206429124311488; }
            _ => { current_block_16 = 5601891728916014340; }
        }
        match current_block_16 {
            2749206429124311488 =>
            /* Fall through. */
            {
                tcp_or_rpc_data_counter -= 1
            }
            _ => { }
        }
        free_connection(conn);
    };
}
#[c2rust::src_loc = "515:1"]
unsafe extern "C" fn make_event(mut ctx: *mut verto_ctx,
                                mut flags: verto_ev_flag,
                                mut callback: Option<verto_callback>,
                                mut sock: libc::c_int,
                                mut conn: *mut connection) -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    ev = verto_add_io(ctx, flags, callback, sock);
    if ev.is_null() {
        com_err((*conn).prog, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"cannot create io event\x00" as *const u8 as
                             *const libc::c_char));
        return 0 as *mut verto_ev
    }
    if if events.n < events.max ||
              (if events.max.wrapping_add(10 as libc::c_int as libc::c_ulong)
                      < events.max ||
                      events.max.wrapping_add(10 as libc::c_int as
                                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut verto_ev>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut verto_ev>()
                                                                                                                  as
                                                                                                                  libc::c_ulong)
                          !=
                          events.max.wrapping_add(10 as libc::c_int as
                                                      libc::c_ulong) {
                   0 as libc::c_int
               } else {
                   tmp =
                       realloc(events.data as *mut libc::c_void,
                               events.max.wrapping_add(10 as libc::c_int as
                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut verto_ev>()
                                                                                           as
                                                                                           libc::c_ulong));
                   (if !tmp.is_null() {
                        events.data = tmp as *mut *mut verto_ev;
                        events.max =
                            (events.max as
                                 libc::c_ulong).wrapping_add(10 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t;
                        1 as libc::c_int
                    } else { 0 as libc::c_int })
               }) != 0 {
           let fresh9 = events.n;
           events.n = events.n.wrapping_add(1);
           let ref mut fresh10 = *events.data.offset(fresh9 as isize);
           *fresh10 = ev;
           1 as libc::c_int
       } else { 0 as libc::c_int } == 0 {
        com_err((*conn).prog, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"cannot save event\x00" as *const u8 as
                             *const libc::c_char));
        verto_del(ev);
        return 0 as *mut verto_ev
    }
    verto_set_private(ev, conn as *mut libc::c_void,
                      Some(free_socket as
                               unsafe extern "C" fn(_: *mut verto_ctx,
                                                    _: *mut verto_ev) -> ()));
    return ev;
}
#[c2rust::src_loc = "538:1"]
unsafe extern "C" fn add_fd(mut sock: libc::c_int, mut conntype: conn_type,
                            mut flags: verto_ev_flag,
                            mut handle: *mut libc::c_void,
                            mut prog: *const libc::c_char,
                            mut ctx: *mut verto_ctx,
                            mut callback: Option<verto_callback>,
                            mut ev_out: *mut *mut verto_ev)
 -> krb5_error_code {
    let mut newconn: *mut connection = 0 as *mut connection;
    *ev_out = 0 as *mut verto_ev;
    if sock >= 1024 as libc::c_int {
        com_err(prog, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"file descriptor number %d too high\x00" as
                             *const u8 as *const libc::c_char), sock);
        return 24 as libc::c_int
    }
    newconn =
        malloc(::std::mem::size_of::<connection>() as libc::c_ulong) as
            *mut connection;
    if newconn.is_null() {
        com_err(prog, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"cannot allocate storage for connection info\x00" as
                             *const u8 as *const libc::c_char));
        return 12 as libc::c_int
    }
    memset(newconn as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<connection>() as libc::c_ulong);
    (*newconn).handle = handle;
    (*newconn).prog = prog;
    (*newconn).type_0 = conntype;
    *ev_out = make_event(ctx, flags, callback, sock, newconn);
    return 0 as libc::c_int;
}
/*
 * Create a socket and bind it to addr.  Ensure the socket will work with
 * select().  Set the socket cloexec, reuseaddr, and if applicable v6-only.
 * Does not call listen().  On failure, log an error and return an error code.
 */
#[c2rust::src_loc = "580:1"]
unsafe extern "C" fn create_server_socket(mut addr: *mut sockaddr,
                                          mut type_0: libc::c_int,
                                          mut prog: *const libc::c_char,
                                          mut fd_out: *mut libc::c_int)
 -> krb5_error_code {
    let mut sock: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    *fd_out = -(1 as libc::c_int);
    sock = socket((*addr).sa_family as libc::c_int, type_0, 0 as libc::c_int);
    if sock == -(1 as libc::c_int) {
        e = *__errno_location();
        com_err(prog, e as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Cannot create TCP server socket on %s\x00" as
                             *const u8 as *const libc::c_char), paddr(addr));
        return e
    }
    fcntl(sock, 2 as libc::c_int, 1 as libc::c_int);
    /* Windows FD_SETSIZE is a count. */
    if sock >= 1024 as libc::c_int {
        close(sock);
        com_err(prog, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"TCP socket fd number %d (for %s) too high\x00" as
                             *const u8 as *const libc::c_char), sock,
                paddr(addr));
        return 24 as libc::c_int
    }
    if setreuseaddr(sock, 1 as libc::c_int) < 0 as libc::c_int {
        com_err(prog, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Cannot enable SO_REUSEADDR on fd %d\x00" as
                             *const u8 as *const libc::c_char), sock);
    }
    if (*addr).sa_family as libc::c_int == 10 as libc::c_int {
        if setv6only(sock, 1 as libc::c_int) != 0 {
            com_err(prog, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"setsockopt(%d,IPV6_V6ONLY,1) failed\x00" as
                                 *const u8 as *const libc::c_char), sock);
        } else {
            com_err(prog, 0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"setsockopt(%d,IPV6_V6ONLY,1) worked\x00" as
                                 *const u8 as *const libc::c_char), sock);
        }
        /* IPV6_V6ONLY */
    }
    if bind(sock, __CONST_SOCKADDR_ARG{__sockaddr__: addr,}, sa_socklen(addr))
           == -(1 as libc::c_int) {
        e = *__errno_location();
        com_err(prog, e as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Cannot bind server socket on %s\x00" as *const u8
                             as *const libc::c_char), paddr(addr));
        close(sock);
        return e
    }
    *fd_out = sock;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "633:18"]
static mut one: libc::c_int = 1 as libc::c_int;
#[c2rust::src_loc = "635:1"]
unsafe extern "C" fn setnbio(mut sock: libc::c_int) -> libc::c_int {
    return ioctl(sock, 0x5421 as libc::c_int as libc::c_ulong,
                 &one as *const libc::c_int as *const libc::c_void);
}
#[c2rust::src_loc = "641:1"]
unsafe extern "C" fn setkeepalive(mut sock: libc::c_int) -> libc::c_int {
    return setsockopt(sock, 1 as libc::c_int, 9 as libc::c_int,
                      &one as *const libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t);
}
#[c2rust::src_loc = "647:1"]
unsafe extern "C" fn setnolinger(mut s: libc::c_int) -> libc::c_int {
    static mut ling: linger =
        {
            let mut init =
                linger{l_onoff: 0 as libc::c_int,
                       l_linger: 0 as libc::c_int,};
            init
        };
    return setsockopt(s, 1 as libc::c_int, 13 as libc::c_int,
                      &ling as *const linger as *const libc::c_void,
                      ::std::mem::size_of::<linger>() as libc::c_ulong as
                          socklen_t);
}
/* An enum map to socket families for each bind_type. */
#[c2rust::src_loc = "655:18"]
static mut bind_socktypes: [libc::c_int; 3] =
    [SOCK_DGRAM as libc::c_int, SOCK_STREAM as libc::c_int,
     SOCK_STREAM as libc::c_int];
/* An enum map containing conn_type (for struct connection) for each
 * bind_type.  */
#[c2rust::src_loc = "664:29"]
static mut bind_conn_types: [conn_type; 3] =
    [CONN_UDP, CONN_TCP_LISTENER, CONN_RPC_LISTENER];
/*
 * Set up a listening socket.
 *
 * Arguments:
 *
 * - ba
 *      The bind address and port for the socket.
 * - ai
 *      The addrinfo struct to use for creating the socket.
 * - ctype
 *      The conn_type of this socket.
 */
#[c2rust::src_loc = "683:1"]
unsafe extern "C" fn setup_socket(mut ba: *mut bind_address,
                                  mut sock_address: *mut sockaddr,
                                  mut handle: *mut libc::c_void,
                                  mut prog: *const libc::c_char,
                                  mut ctx: *mut verto_ctx,
                                  mut tcp_listen_backlog: libc::c_int,
                                  mut vcb_0: Option<verto_callback>,
                                  mut ctype: conn_type) -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut conn: *mut connection = 0 as *mut connection;
    let mut flags: verto_ev_flag = VERTO_EV_FLAG_NONE;
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    let mut sock: libc::c_int = -(1 as libc::c_int);
    krb5_klog_syslog(7 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"Setting up %s socket for address %s\x00" as
                                  *const u8 as *const libc::c_char),
                     bind_type_names[(*ba).type_0 as usize],
                     paddr(sock_address));
    /* Create the socket. */
    ret =
        create_server_socket(sock_address,
                             bind_socktypes[(*ba).type_0 as usize], prog,
                             &mut sock);
    if !(ret != 0) {
        /* Listen for backlogged connections on TCP sockets.  (For RPC sockets this
     * will be done by svc_register().) */
        if (*ba).type_0 as libc::c_uint == TCP as libc::c_int as libc::c_uint
               && listen(sock, tcp_listen_backlog) != 0 as libc::c_int {
            ret = *__errno_location();
            com_err(prog, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Cannot listen on %s server socket on %s\x00" as
                                 *const u8 as *const libc::c_char),
                    bind_type_names[(*ba).type_0 as usize],
                    paddr(sock_address));
        } else if (*ba).type_0 as libc::c_uint !=
                      RPC as libc::c_int as libc::c_uint &&
                      setnbio(sock) != 0 as libc::c_int {
            ret = *__errno_location();
            com_err(prog, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"cannot set listening %s socket on %s non-blocking\x00"
                                 as *const u8 as *const libc::c_char),
                    bind_type_names[(*ba).type_0 as usize],
                    paddr(sock_address));
        } else if (*ba).type_0 as libc::c_uint ==
                      TCP as libc::c_int as libc::c_uint &&
                      setnolinger(sock) != 0 as libc::c_int {
            ret = *__errno_location();
            com_err(prog, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"cannot set SO_LINGER on %s socket on %s\x00" as
                                 *const u8 as *const libc::c_char),
                    bind_type_names[(*ba).type_0 as usize],
                    paddr(sock_address));
        } else {
            /* Set non-blocking I/O for UDP and TCP listener sockets. */
            /* Turn off the linger option for TCP sockets. */
            /* Try to turn on pktinfo for UDP wildcard sockets. */
            if (*ba).type_0 as libc::c_uint ==
                   UDP as libc::c_int as libc::c_uint &&
                   sa_is_wildcard(sock_address) != 0 {
                krb5_klog_syslog(7 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Setting pktinfo on socket %s\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 paddr(sock_address));
                ret =
                    set_pktinfo(sock,
                                (*sock_address).sa_family as libc::c_int);
                if ret != 0 {
                    com_err(prog, ret as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Cannot request packet info for UDP socket address %s port %d\x00"
                                         as *const u8 as *const libc::c_char),
                            paddr(sock_address), (*ba).port as libc::c_int);
                    krb5_klog_syslog(6 as libc::c_int,
                                     dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"System does not support pktinfo yet binding to a wildcard address.  Packets are not guaranteed to return on the received address.\x00"
                                                  as *const u8 as
                                                  *const libc::c_char));
                }
            }
            /* Add the socket to the event loop. */
            flags =
                (VERTO_EV_FLAG_IO_READ as libc::c_int |
                     VERTO_EV_FLAG_PERSIST as libc::c_int |
                     VERTO_EV_FLAG_REINITIABLE as libc::c_int) as
                    verto_ev_flag;
            ret =
                add_fd(sock, ctype, flags, handle, prog, ctx, vcb_0, &mut ev);
            if ret != 0 {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Error attempting to add verto event\x00"
                                              as *const u8 as
                                              *const libc::c_char));
            } else {
                if (*ba).type_0 as libc::c_uint ==
                       RPC as libc::c_int as libc::c_uint {
                    conn = verto_get_private(ev) as *mut connection;
                    (*conn).transp =
                        gssrpc_svctcp_create(sock, 0 as libc::c_int as u_int,
                                             0 as libc::c_int as u_int);
                    if (*conn).transp.is_null() {
                        ret = *__errno_location();
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"Cannot create RPC service: %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         strerror(ret));
                        current_block = 8996098429107287059;
                    } else {
                        ret =
                            gssrpc_svc_register((*conn).transp,
                                                (*ba).rpc_svc_data.prognum as
                                                    rpcprog_t,
                                                (*ba).rpc_svc_data.versnum as
                                                    rpcvers_t,
                                                ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                   ->
                                                                                       ()>,
                                                                        Option<unsafe extern "C" fn(_:
                                                                                                        *mut svc_req,
                                                                                                    _:
                                                                                                        *mut SVCXPRT)
                                                                                   ->
                                                                                       ()>>((*ba).rpc_svc_data.dispatch),
                                                0 as libc::c_int);
                        if ret == 0 {
                            ret = *__errno_location();
                            krb5_klog_syslog(3 as libc::c_int,
                                             dgettext(b"mit-krb5\x00" as
                                                          *const u8 as
                                                          *const libc::c_char,
                                                      b"Cannot register RPC service: %s\x00"
                                                          as *const u8 as
                                                          *const libc::c_char),
                                             strerror(ret));
                            current_block = 8996098429107287059;
                        } else { current_block = 4090602189656566074; }
                    }
                } else { current_block = 4090602189656566074; }
                match current_block {
                    8996098429107287059 => { }
                    _ => {
                        ev = 0 as *mut verto_ev;
                        sock = -(1 as libc::c_int);
                        ret = 0 as libc::c_int
                    }
                }
            }
        }
    }
    if sock >= 0 as libc::c_int { close(sock); }
    if !ev.is_null() { verto_del(ev); }
    return ret;
}
/*
 * Setup all the socket addresses that the net-server should listen to.
 *
 * This function uses getaddrinfo to figure out all the addresses. This will
 * automatically figure out which socket families that should be used on the
 * host making it useful even for wildcard addresses.
 */
#[c2rust::src_loc = "794:1"]
unsafe extern "C" fn setup_addresses(mut ctx: *mut verto_ctx,
                                     mut handle: *mut libc::c_void,
                                     mut prog: *const libc::c_char,
                                     mut tcp_listen_backlog: libc::c_int)
 -> krb5_error_code {
    /* An bind_type enum map for the verto callback functions. */
    static mut verto_callbacks: [Option<verto_callback>; 3] =
        [Some(process_packet as
                  unsafe extern "C" fn(_: *mut verto_ctx, _: *mut verto_ev)
                      -> ()),
         Some(accept_tcp_connection as
                  unsafe extern "C" fn(_: *mut verto_ctx, _: *mut verto_ev)
                      -> ()),
         Some(accept_rpc_connection as
                  unsafe extern "C" fn(_: *mut verto_ctx, _: *mut verto_ev)
                      -> ())];
    let mut ret: krb5_error_code = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut err: libc::c_int = 0;
    let mut bound_any: libc::c_int = 0;
    let mut addr: bind_address =
        bind_address{address: 0 as *mut libc::c_char,
                     port: 0,
                     type_0: UDP,
                     rpc_svc_data:
                         rpc_svc_data{prognum: 0,
                                      versnum: 0,
                                      dispatch: None,},};
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut ai_list: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    extern "C" {
        #[link_name = "vcb"]
        pub fn vcb_0(_: *mut verto_ctx, _: *mut verto_ev);
    }
    /* Check to make sure addresses were added to the server. */
    if bind_addresses.n == 0 as libc::c_int as libc::c_ulong {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"No addresses added to the net server\x00"
                                      as *const u8 as *const libc::c_char));
        return 22 as libc::c_int
    }
    /* Ask for all address families, listener addresses, and no port name
     * resolution. */
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_family = 0 as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int | 0x400 as libc::c_int;
    /* Add all the requested addresses. */
    i = 0 as libc::c_int as size_t;
    's_52:
        while i < bind_addresses.n {
            addr = *bind_addresses.data.offset(i as isize);
            hints.ai_socktype = bind_socktypes[addr.type_0 as usize];
            /* Call getaddrinfo, using a dummy port value. */
            err =
                krb5int_getaddrinfo(addr.address,
                                    b"0\x00" as *const u8 as
                                        *const libc::c_char, &mut hints,
                                    &mut ai_list);
            if err != 0 {
                krb5_klog_syslog(3 as libc::c_int,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Failed getting address info (for %s): %s\x00"
                                              as *const u8 as
                                              *const libc::c_char),
                                 if addr.address.is_null() {
                                     b"<wildcard>\x00" as *const u8 as
                                         *const libc::c_char
                                 } else {
                                     addr.address as *const libc::c_char
                                 }, krb5int_gai_strerror(err));
                ret = 5 as libc::c_int;
                break ;
            } else {
                /*
         * Loop through all the sockets that getaddrinfo could find to match
         * the requested address.  For wildcard listeners, this should usually
         * have two results, one for each of IPv4 and IPv6, or one or the
         * other, depending on the system.  On IPv4-only systems, getaddrinfo()
         * may return both IPv4 and IPv6 addresses, but creating an IPv6 socket
         * may give an EAFNOSUPPORT error, so tolerate that error as long as we
         * can bind at least one socket.
         */
                bound_any = 0 as libc::c_int;
                ai = ai_list;
                while !ai.is_null() {
                    /* Make sure getaddrinfo returned a socket with the same type that
             * was requested. */
                    if hints.ai_socktype == (*ai).ai_socktype {
                    } else {
                        __assert_fail(b"hints.ai_socktype == ai->ai_socktype\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"net-server.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      852 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 72],
                                                                &[libc::c_char; 72]>(b"krb5_error_code setup_addresses(verto_ctx *, void *, const char *, int)\x00")).as_ptr());
                    }
                    /* Set the real port number. */
                    sa_setport((*ai).ai_addr, addr.port);
                    ret =
                        setup_socket(&mut addr, (*ai).ai_addr, handle, prog,
                                     ctx, tcp_listen_backlog,
                                     verto_callbacks[addr.type_0 as usize],
                                     bind_conn_types[addr.type_0 as usize]);
                    if ret != 0 {
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"Failed setting up a %s socket (for %s)\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         bind_type_names[addr.type_0 as
                                                             usize],
                                         paddr((*ai).ai_addr));
                        if ret != 97 as libc::c_int { break 's_52 ; }
                    } else { bound_any = 1 as libc::c_int }
                    ai = (*ai).ai_next
                }
                if bound_any == 0 { break ; }
                ret = 0 as libc::c_int;
                if !ai_list.is_null() { krb5int_freeaddrinfo(ai_list); }
                ai_list = 0 as *mut addrinfo;
                i = i.wrapping_add(1)
            }
        }
    if !ai_list.is_null() { krb5int_freeaddrinfo(ai_list); }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "886:1"]
pub unsafe extern "C" fn loop_setup_network(mut ctx: *mut verto_ctx,
                                            mut handle: *mut libc::c_void,
                                            mut prog: *const libc::c_char,
                                            mut tcp_listen_backlog:
                                                libc::c_int)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    let mut i: libc::c_int = 0;
    /* Check to make sure that at least one address was added to the loop. */
    if bind_addresses.n == 0 as libc::c_int as libc::c_ulong {
        return 22 as libc::c_int
    }
    /* Close any open connections. */
    i =
        events.n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    while i >= 0 as libc::c_int &&
              {
                  ev = *events.data.offset(i as isize);
                  (1 as libc::c_int) != 0
              } {
        verto_del(ev);
        i -= 1
    }
    events.n = 0 as libc::c_int as size_t;
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"setting up network...\x00" as *const u8 as
                                  *const libc::c_char));
    ret = setup_addresses(ctx, handle, prog, tcp_listen_backlog);
    if ret != 0 {
        com_err(prog, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Error setting up network\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"set up %d sockets\x00" as *const u8 as
                                  *const libc::c_char),
                     events.n as libc::c_int);
    if events.n == 0 as libc::c_int as libc::c_ulong {
        /* If no sockets were set up, we can't continue. */
        com_err(prog, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"no sockets set up?\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "919:1"]
pub unsafe extern "C" fn init_addr(mut faddr: *mut krb5_fulladdr,
                                   mut sa: *mut sockaddr) {
    match (*sa).sa_family as libc::c_int {
        2 => {
            (*(*faddr).address).addrtype = 0x2 as libc::c_int;
            (*(*faddr).address).length = 4 as libc::c_int as libc::c_uint;
            (*(*faddr).address).contents =
                &mut (*(sa2sin as
                            unsafe extern "C" fn(_: *mut sockaddr)
                                -> *mut sockaddr_in)(sa)).sin_addr as
                    *mut in_addr as *mut krb5_octet;
            (*faddr).port = ntohs((*sa2sin(sa)).sin_port) as krb5_ui_4
        }
        10 => {
            if ({
                    let mut __a: *const in6_addr =
                        &mut (*(sa2sin6 as
                                    unsafe extern "C" fn(_: *mut sockaddr)
                                        -> *mut sockaddr_in6)(sa)).sin6_addr
                            as *mut in6_addr as *const in6_addr;
                    ((*__a).__in6_u.__u6_addr32[0 as libc::c_int as usize] ==
                         0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[1 as libc::c_int as usize]
                             == 0 as libc::c_int as libc::c_uint &&
                         (*__a).__in6_u.__u6_addr32[2 as libc::c_int as usize]
                             == htonl(0xffff as libc::c_int as uint32_t)) as
                        libc::c_int
                }) != 0 {
                (*(*faddr).address).addrtype = 0x2 as libc::c_int;
                (*(*faddr).address).length = 4 as libc::c_int as libc::c_uint;
                (*(*faddr).address).contents =
                    (&mut (*(sa2sin6 as
                                 unsafe extern "C" fn(_: *mut sockaddr)
                                     -> *mut sockaddr_in6)(sa)).sin6_addr as
                         *mut in6_addr as
                         *mut krb5_octet).offset(12 as libc::c_int as isize)
            } else {
                (*(*faddr).address).addrtype = 0x18 as libc::c_int;
                (*(*faddr).address).length =
                    16 as libc::c_int as libc::c_uint;
                (*(*faddr).address).contents =
                    &mut (*(sa2sin6 as
                                unsafe extern "C" fn(_: *mut sockaddr)
                                    -> *mut sockaddr_in6)(sa)).sin6_addr as
                        *mut in6_addr as *mut krb5_octet
            }
            (*faddr).port = ntohs((*sa2sin6(sa)).sin6_port) as krb5_ui_4
        }
        _ => {
            (*(*faddr).address).addrtype = -(1 as libc::c_int);
            (*(*faddr).address).length = 0 as libc::c_int as libc::c_uint;
            (*(*faddr).address).contents = 0 as *mut krb5_octet;
            (*faddr).port = 0 as libc::c_int as krb5_ui_4
        }
    };
}
#[c2rust::src_loc = "967:1"]
unsafe extern "C" fn process_packet_response(mut arg: *mut libc::c_void,
                                             mut code: krb5_error_code,
                                             mut response: *mut krb5_data) {
    let mut state: *mut udp_dispatch_state = arg as *mut udp_dispatch_state;
    let mut cc: libc::c_int = 0;
    if code != 0 {
        com_err(if !(*state).prog.is_null() {
                    (*state).prog
                } else { 0 as *const libc::c_char }, code as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while dispatching (udp)\x00" as *const u8 as
                             *const libc::c_char));
    }
    if !(code != 0 || response.is_null()) {
        cc =
            send_to_from((*state).port_fd,
                         (*response).data as *mut libc::c_void,
                         (*response).length as size_t, 0 as libc::c_int,
                         &mut (*state).saddr as *mut sockaddr_storage as
                             *mut sockaddr, (*state).saddr_len,
                         &mut (*state).daddr as *mut sockaddr_storage as
                             *mut sockaddr, (*state).daddr_len,
                         &mut (*state).auxaddr);
        if cc == -(1 as libc::c_int) {
            /* Note that the local address (daddr*) has no port number
         * info associated with it. */
            let mut saddrbuf: [libc::c_char; 1025] = [0; 1025];
            let mut sportbuf: [libc::c_char; 32] = [0; 32];
            let mut daddrbuf: [libc::c_char; 1025] = [0; 1025];
            let mut e: libc::c_int = *__errno_location();
            if krb5int_getnameinfo(&mut (*state).daddr as
                                       *mut sockaddr_storage as *mut sockaddr,
                                   (*state).daddr_len, daddrbuf.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 1025]>()
                                       as libc::c_ulong,
                                   0 as *mut libc::c_char,
                                   0 as libc::c_int as size_t,
                                   1 as libc::c_int) != 0 as libc::c_int {
                krb5int_strlcpy(daddrbuf.as_mut_ptr(),
                                b"?\x00" as *const u8 as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 1025]>()
                                    as libc::c_ulong);
            }
            if krb5int_getnameinfo(&mut (*state).saddr as
                                       *mut sockaddr_storage as *mut sockaddr,
                                   (*state).saddr_len, saddrbuf.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 1025]>()
                                       as libc::c_ulong,
                                   sportbuf.as_mut_ptr(),
                                   ::std::mem::size_of::<[libc::c_char; 32]>()
                                       as libc::c_ulong,
                                   1 as libc::c_int | 2 as libc::c_int) !=
                   0 as libc::c_int {
                krb5int_strlcpy(saddrbuf.as_mut_ptr(),
                                b"?\x00" as *const u8 as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 1025]>()
                                    as libc::c_ulong);
                krb5int_strlcpy(sportbuf.as_mut_ptr(),
                                b"?\x00" as *const u8 as *const libc::c_char,
                                ::std::mem::size_of::<[libc::c_char; 32]>() as
                                    libc::c_ulong);
            }
            com_err((*state).prog, e as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while sending reply to %s/%s from %s\x00" as
                                 *const u8 as *const libc::c_char),
                    saddrbuf.as_mut_ptr(), sportbuf.as_mut_ptr(),
                    daddrbuf.as_mut_ptr());
        } else if cc as size_t != (*response).length as libc::c_ulong {
            com_err((*state).prog, 0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"short reply write %d vs %d\n\x00" as *const u8
                                 as *const libc::c_char), (*response).length,
                    cc);
        }
    }
    krb5_free_data(get_context((*state).handle), response);
    free(state as *mut libc::c_void);
}
#[c2rust::src_loc = "1018:1"]
unsafe extern "C" fn process_packet(mut ctx: *mut verto_ctx,
                                    mut ev: *mut verto_ev) {
    let mut cc: libc::c_int = 0;
    let mut conn: *mut connection = 0 as *mut connection;
    let mut state: *mut udp_dispatch_state = 0 as *mut udp_dispatch_state;
    conn = verto_get_private(ev) as *mut connection;
    state =
        malloc(::std::mem::size_of::<udp_dispatch_state>() as libc::c_ulong)
            as *mut udp_dispatch_state;
    if state.is_null() {
        com_err((*conn).prog, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while dispatching (udp)\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    (*state).handle = (*conn).handle;
    (*state).prog = (*conn).prog;
    (*state).port_fd = verto_get_fd(ev);
    if (*state).port_fd >= 0 as libc::c_int {
    } else {
        __assert_fail(b"state->port_fd >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"net-server.c\x00" as *const u8 as *const libc::c_char,
                      1036 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"void process_packet(verto_ctx *, verto_ev *)\x00")).as_ptr());
    }
    (*state).saddr_len =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    (*state).daddr_len =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    memset(&mut (*state).auxaddr as *mut aux_addressing_info as
               *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<aux_addressing_info>() as libc::c_ulong);
    cc =
        recv_from_to((*state).port_fd,
                     (*state).pktbuf.as_mut_ptr() as *mut libc::c_void,
                     ::std::mem::size_of::<[libc::c_char; 65536]>() as
                         libc::c_ulong, 0 as libc::c_int,
                     &mut (*state).saddr as *mut sockaddr_storage as
                         *mut sockaddr, &mut (*state).saddr_len,
                     &mut (*state).daddr as *mut sockaddr_storage as
                         *mut sockaddr, &mut (*state).daddr_len,
                     &mut (*state).auxaddr);
    if cc == -(1 as libc::c_int) {
        if *__errno_location() != 4 as libc::c_int &&
               *__errno_location() != 11 as libc::c_int &&
               *__errno_location() != 111 as libc::c_int {
            com_err((*conn).prog, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while receiving from network\x00" as *const u8
                                 as *const libc::c_char));
        }
        free(state as *mut libc::c_void);
        return
    }
    if cc == 0 {
        /* zero-length packet? */
        free(state as *mut libc::c_void);
        return
    }
    if (*state).daddr_len == 0 as libc::c_int as libc::c_uint &&
           (*conn).type_0 as libc::c_uint ==
               CONN_UDP as libc::c_int as libc::c_uint {
        /*
         * An address couldn't be obtained, so the PKTINFO option probably
         * isn't available.  If the socket is bound to a specific address, then
         * try to get the address here.
         */
        (*state).daddr_len =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        if getsockname((*state).port_fd,
                       __SOCKADDR_ARG{__sockaddr__:
                                          &mut (*state).daddr as
                                              *mut sockaddr_storage as
                                              *mut sockaddr,},
                       &mut (*state).daddr_len) != 0 as libc::c_int {
            (*state).daddr_len = 0 as libc::c_int as socklen_t
        }
        /* On failure, keep going anyways. */
    }
    (*state).request.length = cc as libc::c_uint;
    (*state).request.data = (*state).pktbuf.as_mut_ptr();
    (*state).remote_addr.address = &mut (*state).remote_addr_buf;
    init_addr(&mut (*state).remote_addr, ss2sa(&mut (*state).saddr));
    (*state).local_addr.address = &mut (*state).local_addr_buf;
    init_addr(&mut (*state).local_addr, ss2sa(&mut (*state).daddr));
    /* This address is in net order. */
    dispatch((*state).handle, &mut (*state).local_addr,
             &mut (*state).remote_addr, &mut (*state).request,
             0 as libc::c_int, ctx,
             Some(process_packet_response as
                      unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: krb5_error_code,
                                           _: *mut krb5_data) -> ()),
             state as *mut libc::c_void);
}
#[c2rust::src_loc = "1090:1"]
unsafe extern "C" fn kill_lru_tcp_or_rpc_connection(mut handle:
                                                        *mut libc::c_void,
                                                    mut newev: *mut verto_ev)
 -> libc::c_int {
    let mut c: *mut connection = 0 as *mut connection;
    let mut oldest_c: *mut connection = 0 as *mut connection;
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    let mut oldest_ev: *mut verto_ev = 0 as *mut verto_ev;
    let mut i: libc::c_int = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    krb5_klog_syslog(6 as libc::c_int,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"too many connections\x00" as *const u8 as
                                  *const libc::c_char));
    i =
        events.n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    while i >= 0 as libc::c_int &&
              {
                  ev = *events.data.offset(i as isize);
                  (1 as libc::c_int) != 0
              } {
        if !(ev == newev) {
            c = verto_get_private(ev) as *mut connection;
            if !c.is_null() {
                if !((*c).type_0 as libc::c_uint !=
                         CONN_TCP as libc::c_int as libc::c_uint &&
                         (*c).type_0 as libc::c_uint !=
                             CONN_RPC as libc::c_int as libc::c_uint) {
                    if oldest_c.is_null() ||
                           (*oldest_c).start_time > (*c).start_time {
                        oldest_ev = ev;
                        oldest_c = c
                    }
                }
            }
        }
        i -= 1
    }
    if !oldest_c.is_null() {
        krb5_klog_syslog(6 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"dropping %s fd %d from %s\x00" as
                                      *const u8 as *const libc::c_char),
                         if (*c).type_0 as libc::c_uint ==
                                CONN_RPC as libc::c_int as libc::c_uint {
                             b"rpc\x00" as *const u8 as *const libc::c_char
                         } else {
                             b"tcp\x00" as *const u8 as *const libc::c_char
                         }, verto_get_fd(oldest_ev),
                         (*oldest_c).addrbuf.as_mut_ptr());
        if (*oldest_c).type_0 as libc::c_uint ==
               CONN_RPC as libc::c_int as libc::c_uint {
            (*oldest_c).rpc_force_close = 1 as libc::c_int
        }
        verto_del(oldest_ev);
    }
    return fd;
}
#[c2rust::src_loc = "1125:1"]
unsafe extern "C" fn accept_tcp_connection(mut ctx: *mut verto_ctx,
                                           mut ev: *mut verto_ev) {
    let mut s: libc::c_int = 0;
    let mut addr_s: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut addr: *mut sockaddr =
        &mut addr_s as *mut sockaddr_storage as *mut sockaddr;
    let mut addrlen: socklen_t =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    let mut newconn: *mut connection = 0 as *mut connection;
    let mut conn: *mut connection = 0 as *mut connection;
    let mut tmpbuf: [libc::c_char; 10] = [0; 10];
    let mut flags: verto_ev_flag = VERTO_EV_FLAG_NONE;
    let mut newev: *mut verto_ev = 0 as *mut verto_ev;
    conn = verto_get_private(ev) as *mut connection;
    s =
        accept(verto_get_fd(ev), __SOCKADDR_ARG{__sockaddr__: addr,},
               &mut addrlen);
    if s < 0 as libc::c_int { return }
    fcntl(s, 2 as libc::c_int, 1 as libc::c_int);
    if s >= 1024 as libc::c_int { close(s); return }
    setnbio(s);
    setnolinger(s);
    setkeepalive(s);
    flags =
        (VERTO_EV_FLAG_IO_READ as libc::c_int |
             VERTO_EV_FLAG_PERSIST as libc::c_int) as verto_ev_flag;
    if add_fd(s, CONN_TCP, flags, (*conn).handle, (*conn).prog, ctx,
              Some(process_tcp_connection_read as
                       unsafe extern "C" fn(_: *mut verto_ctx,
                                            _: *mut verto_ev) -> ()),
              &mut newev) != 0 as libc::c_int {
        close(s);
        return
    }
    newconn = verto_get_private(newev) as *mut connection;
    if krb5int_getnameinfo(&mut addr_s as *mut sockaddr_storage as
                               *mut sockaddr, addrlen,
                           (*newconn).addrbuf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 56]>() as
                               libc::c_ulong, tmpbuf.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 10]>() as
                               libc::c_ulong,
                           1 as libc::c_int | 2 as libc::c_int) != 0 {
        krb5int_strlcpy((*newconn).addrbuf.as_mut_ptr(),
                        b"???\x00" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 56]>() as
                            libc::c_ulong);
    } else {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        p = (*newconn).addrbuf.as_mut_ptr();
        end =
            p.offset(::std::mem::size_of::<[libc::c_char; 56]>() as
                         libc::c_ulong as isize);
        p = p.offset(strlen(p) as isize);
        if end.wrapping_offset_from(p) as libc::c_long as size_t >
               (2 as libc::c_int as
                    libc::c_ulong).wrapping_add(strlen(tmpbuf.as_mut_ptr())) {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = '.' as i32 as libc::c_char;
            krb5int_strlcpy(p, tmpbuf.as_mut_ptr(),
                            end.wrapping_offset_from(p) as libc::c_long as
                                size_t);
        }
    }
    (*newconn).addr_s = addr_s;
    (*newconn).addrlen = addrlen;
    (*newconn).bufsiz = (1024 as libc::c_int * 1024 as libc::c_int) as size_t;
    (*newconn).buffer = malloc((*newconn).bufsiz) as *mut libc::c_char;
    (*newconn).start_time = time(0 as *mut time_t);
    tcp_or_rpc_data_counter += 1;
    if tcp_or_rpc_data_counter > max_tcp_or_rpc_data_connections {
        kill_lru_tcp_or_rpc_connection((*conn).handle, newev);
    }
    if (*newconn).buffer.is_null() {
        com_err((*conn).prog, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"allocating buffer for new TCP session from %s\x00"
                             as *const u8 as *const libc::c_char),
                (*newconn).addrbuf.as_mut_ptr());
        verto_del(newev);
        return
    }
    (*newconn).offset = 0 as libc::c_int as size_t;
    (*newconn).remote_addr.address = &mut (*newconn).remote_addr_buf;
    init_addr(&mut (*newconn).remote_addr, ss2sa(&mut (*newconn).addr_s));
    (*newconn).sgbuf[0 as libc::c_int as usize].iov_base =
        (*newconn).lenbuf.as_mut_ptr() as *mut libc::c_char as
            *mut libc::c_void;
    (*newconn).sgbuf[0 as libc::c_int as usize].iov_len =
        4 as libc::c_int as size_t;
    (*newconn).sgbuf[1 as libc::c_int as usize].iov_base =
        0 as *mut libc::c_char as *mut libc::c_void;
    (*newconn).sgbuf[1 as libc::c_int as usize].iov_len =
        0 as libc::c_int as size_t;
}
#[c2rust::src_loc = "1207:1"]
unsafe extern "C" fn process_tcp_response(mut arg: *mut libc::c_void,
                                          mut code: krb5_error_code,
                                          mut response: *mut krb5_data) {
    let mut state: *mut tcp_dispatch_state = arg as *mut tcp_dispatch_state;
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    if !state.is_null() {
    } else {
        __assert_fail(b"state\x00" as *const u8 as *const libc::c_char,
                      b"net-server.c\x00" as *const u8 as *const libc::c_char,
                      1213 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"void process_tcp_response(void *, krb5_error_code, krb5_data *)\x00")).as_ptr());
    }
    (*(*state).conn).response = response;
    if code != 0 {
        com_err((*(*state).conn).prog, code as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while dispatching (tcp)\x00" as *const u8 as
                             *const libc::c_char));
    }
    if !(code != 0 || response.is_null()) {
        /* Queue outgoing response. */
        store_32_be((*response).length,
                    (*(*state).conn).lenbuf.as_mut_ptr() as
                        *mut libc::c_void);
        (*(*state).conn).sgbuf[1 as libc::c_int as usize].iov_base =
            (*response).data as *mut libc::c_void;
        (*(*state).conn).sgbuf[1 as libc::c_int as usize].iov_len =
            (*response).length as size_t;
        (*(*state).conn).sgp = (*(*state).conn).sgbuf.as_mut_ptr();
        (*(*state).conn).sgnum = 2 as libc::c_int;
        ev =
            make_event((*state).ctx,
                       (VERTO_EV_FLAG_IO_WRITE as libc::c_int |
                            VERTO_EV_FLAG_PERSIST as libc::c_int) as
                           verto_ev_flag,
                       Some(process_tcp_connection_write as
                                unsafe extern "C" fn(_: *mut verto_ctx,
                                                     _: *mut verto_ev) -> ()),
                       (*state).sock, (*state).conn);
        if !ev.is_null() { free(state as *mut libc::c_void); return }
    }
    tcp_or_rpc_data_counter -= 1;
    free_connection((*state).conn);
    close((*state).sock);
    free(state as *mut libc::c_void);
}
/* Creates the tcp_dispatch_state and deletes the verto event. */
#[c2rust::src_loc = "1242:1"]
unsafe extern "C" fn prepare_for_dispatch(mut ctx: *mut verto_ctx,
                                          mut ev: *mut verto_ev)
 -> *mut tcp_dispatch_state {
    let mut state: *mut tcp_dispatch_state =
        0 as *mut tcp_dispatch_state; /* Don't close the fd or free conn! */
    state =
        malloc(::std::mem::size_of::<tcp_dispatch_state>() as libc::c_ulong)
            as *mut tcp_dispatch_state; /* Remove it from the set. */
    if state.is_null() {
        krb5_klog_syslog(3 as libc::c_int,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"error allocating tcp dispatch private!\x00"
                                      as *const u8 as *const libc::c_char));
        return 0 as *mut tcp_dispatch_state
    }
    (*state).conn = verto_get_private(ev) as *mut connection;
    (*state).sock = verto_get_fd(ev);
    (*state).ctx = ctx;
    verto_set_private(ev, 0 as *mut libc::c_void, None);
    remove_event_from_set(ev);
    verto_del(ev);
    return state;
}
#[c2rust::src_loc = "1261:1"]
unsafe extern "C" fn process_tcp_connection_read(mut ctx: *mut verto_ctx,
                                                 mut ev: *mut verto_ev) {
    let mut current_block: u64;
    let mut state: *mut tcp_dispatch_state = 0 as *mut tcp_dispatch_state;
    let mut conn: *mut connection = 0 as *mut connection;
    let mut nread: ssize_t = 0;
    let mut len: size_t = 0;
    conn = verto_get_private(ev) as *mut connection;
    /*
     * Read message length and data into one big buffer, already allocated
     * at connect time.  If we have a complete message, we stop reading, so
     * we should only be here if there is no data in the buffer, or only an
     * incomplete message.
     */
    if (*conn).offset < 4 as libc::c_int as libc::c_ulong {
        let mut response: *mut krb5_data = 0 as *mut krb5_data;
        /* msglen has not been computed.  XXX Doing at least two reads
         * here, letting the kernel worry about buffering. */
        len =
            (4 as libc::c_int as libc::c_ulong).wrapping_sub((*conn).offset);
        nread =
            read(verto_get_fd(ev),
                 (*conn).buffer.offset((*conn).offset as isize) as
                     *mut libc::c_void, len);
        if nread < 0 as libc::c_int as libc::c_long {
            /* error */
            current_block = 1260003311578913931;
        } else if nread == 0 as libc::c_int as libc::c_long {
            /* eof */
            current_block = 1260003311578913931;
        } else {
            (*conn).offset =
                ((*conn).offset as
                     libc::c_ulong).wrapping_add(nread as libc::c_ulong) as
                    size_t as size_t;
            if (*conn).offset == 4 as libc::c_int as libc::c_ulong {
                let mut p: *mut libc::c_uchar =
                    (*conn).buffer as *mut libc::c_uchar;
                (*conn).msglen =
                    load_32_be(p as *const libc::c_void) as size_t;
                if (*conn).msglen >
                       (*conn).bufsiz.wrapping_sub(4 as libc::c_int as
                                                       libc::c_ulong) {
                    let mut err: krb5_error_code = 0;
                    /* Message too big. */
                    krb5_klog_syslog(3 as libc::c_int,
                                     dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"TCP client %s wants %lu bytes, cap is %lu\x00"
                                                  as *const u8 as
                                                  *const libc::c_char),
                                     (*conn).addrbuf.as_mut_ptr(),
                                     (*conn).msglen,
                                     (*conn).bufsiz.wrapping_sub(4 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong));
                    /* XXX Should return an error.  */
                    err = make_toolong_error((*conn).handle, &mut response);
                    if err != 0 {
                        krb5_klog_syslog(3 as libc::c_int,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"error constructing KRB_ERR_FIELD_TOOLONG error! %s\x00"
                                                      as *const u8 as
                                                      *const libc::c_char),
                                         error_message(err as errcode_t));
                        current_block = 1260003311578913931;
                    } else {
                        state = prepare_for_dispatch(ctx, ev);
                        if state.is_null() {
                            krb5_free_data(get_context((*conn).handle),
                                           response);
                            current_block = 1260003311578913931;
                        } else {
                            process_tcp_response(state as *mut libc::c_void,
                                                 0 as libc::c_int, response);
                            current_block = 15090052786889560393;
                        }
                    }
                } else { current_block = 15090052786889560393; }
            } else { current_block = 15090052786889560393; }
        }
    } else {
        /* msglen known. */
        let mut local_saddrlen: socklen_t =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        len =
            (*conn).msglen.wrapping_sub((*conn).offset.wrapping_sub(4 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong));
        nread =
            read(verto_get_fd(ev),
                 (*conn).buffer.offset((*conn).offset as isize) as
                     *mut libc::c_void, len);
        if nread < 0 as libc::c_int as libc::c_long {
            /* error */
            current_block = 1260003311578913931;
        } else if nread == 0 as libc::c_int as libc::c_long {
            current_block = 1260003311578913931;
        } else {
            (*conn).offset =
                ((*conn).offset as
                     libc::c_ulong).wrapping_add(nread as libc::c_ulong) as
                    size_t as size_t;
            if (*conn).offset <
                   (*conn).msglen.wrapping_add(4 as libc::c_int as
                                                   libc::c_ulong) {
                return
            }
            /* Have a complete message, and exactly one message. */
            state = prepare_for_dispatch(ctx, ev);
            if state.is_null() {
                current_block = 1260003311578913931;
            } else {
                (*state).request.length = (*conn).msglen as libc::c_uint;
                (*state).request.data =
                    (*conn).buffer.offset(4 as libc::c_int as isize);
                if getsockname(verto_get_fd(ev),
                               __SOCKADDR_ARG{__sockaddr__:
                                                  ss2sa(&mut (*state).local_saddr),},
                               &mut local_saddrlen) < 0 as libc::c_int {
                    krb5_klog_syslog(3 as libc::c_int,
                                     dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"getsockname failed: %s\x00" as
                                                  *const u8 as
                                                  *const libc::c_char),
                                     error_message(*__errno_location() as
                                                       errcode_t));
                    current_block = 1260003311578913931;
                } else {
                    (*state).local_addr.address =
                        &mut (*state).local_addr_buf;
                    init_addr(&mut (*state).local_addr,
                              ss2sa(&mut (*state).local_saddr));
                    dispatch((*(*state).conn).handle,
                             &mut (*state).local_addr,
                             &mut (*conn).remote_addr, &mut (*state).request,
                             1 as libc::c_int, ctx,
                             Some(process_tcp_response as
                                      unsafe extern "C" fn(_:
                                                               *mut libc::c_void,
                                                           _: krb5_error_code,
                                                           _: *mut krb5_data)
                                          -> ()), state as *mut libc::c_void);
                    current_block = 15090052786889560393;
                }
            }
        }
    }
    match current_block {
        15090052786889560393 => { return }
        _ =>
        /* eof */
        {
            verto_del(ev);
            return;
        }
    };
}
#[c2rust::src_loc = "1359:1"]
unsafe extern "C" fn process_tcp_connection_write(mut ctx: *mut verto_ctx,
                                                  mut ev: *mut verto_ev) {
    let mut conn: *mut connection = 0 as *mut connection;
    let mut tmp: libc::c_int = 0;
    let mut nwrote: ssize_t = 0;
    let mut sock: libc::c_int = 0;
    conn = verto_get_private(ev) as *mut connection;
    sock = verto_get_fd(ev);
    tmp = socket_sendmsg(sock, (*conn).sgp, (*conn).sgnum) as libc::c_int;
    nwrote = tmp as ssize_t;
    if nwrote > 0 as libc::c_int as libc::c_long {
        /* non-error and non-eof */
        while nwrote != 0 {
            let mut sgp: *mut sg_buf = (*conn).sgp;
            if (nwrote as size_t) <
                   (*sgp).iov_len.wrapping_add(0 as libc::c_int as
                                                   libc::c_ulong) {
                if (*sgp).iov_len < nwrote as size_t {
                    abort();
                } else {
                    (*sgp).iov_base =
                        ((*sgp).iov_base as
                             *mut libc::c_char).offset(nwrote as size_t as
                                                           isize) as
                            *mut libc::c_void;
                    (*sgp).iov_len =
                        ((*sgp).iov_len as
                             libc::c_ulong).wrapping_sub(nwrote as size_t) as
                            size_t as size_t
                };
                nwrote = 0 as libc::c_int as ssize_t
            } else {
                nwrote =
                    (nwrote as
                         libc::c_ulong).wrapping_sub((*sgp).iov_len.wrapping_add(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
                        as ssize_t as ssize_t;
                (*conn).sgp = (*conn).sgp.offset(1);
                (*conn).sgnum -= 1;
                if (*conn).sgnum == 0 as libc::c_int &&
                       nwrote != 0 as libc::c_int as libc::c_long {
                    abort();
                }
            }
        }
        /* If we still have more data to send, just return so that
         * the main loop can call this function again when the socket
         * is ready for more writing. */
        if (*conn).sgnum > 0 as libc::c_int { return }
    }
    /* Finished sending.  We should go back to reading, though if we
     * sent a FIELD_TOOLONG error in reply to a length with the high
     * bit set, RFC 4120 says we have to close the TCP stream. */
    verto_del(ev);
}
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
/* exported from network.c */
/* exported from net-server.c */
/*
 * Add listener addresses to the loop configuration.
 *
 * Arguments:
 *
 * - default_port
 *      The port for the sockets if not specified in addresses.
 * - addresses
 *      The optional addresses for the listener sockets.  Pass NULL for the
 *      wildcard address.  Addresses may be delimited by the characters in
 *      ADDRESSES_DELIM.  Addresses are parsed with k5_parse_host_string().
 * - prognum, versnum, dispatchfn
 *      For RPC listener sockets, the svc_register() arguments to use when new
 *      TCP connections are created.
 */
#[no_mangle]
#[c2rust::src_loc = "1400:1"]
pub unsafe extern "C" fn loop_free(mut ctx: *mut verto_ctx) {
    let mut i: libc::c_int = 0;
    let mut val: bind_address =
        bind_address{address: 0 as *mut libc::c_char,
                     port: 0,
                     type_0: UDP,
                     rpc_svc_data:
                         rpc_svc_data{prognum: 0,
                                      versnum: 0,
                                      dispatch: None,},};
    verto_free(ctx);
    /* Free each addresses added to the loop. */
    i =
        bind_addresses.n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    while i >= 0 as libc::c_int &&
              {
                  val = *bind_addresses.data.offset(i as isize);
                  (1 as libc::c_int) != 0
              } {
        free(val.address as *mut libc::c_void);
        i -= 1
    }
    free(bind_addresses.data as *mut libc::c_void);
    bind_addresses.data = 0 as *mut bind_address;
    bind_addresses.max = 0 as libc::c_int as size_t;
    bind_addresses.n = 0 as libc::c_int as size_t;
    free(events.data as *mut libc::c_void);
    events.data = 0 as *mut *mut verto_ev;
    events.max = 0 as libc::c_int as size_t;
    events.n = 0 as libc::c_int as size_t;
}
#[c2rust::src_loc = "1415:1"]
unsafe extern "C" fn have_event_for_fd(mut fd: libc::c_int) -> libc::c_int {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    let mut i: libc::c_int = 0;
    i =
        events.n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as
            libc::c_int;
    while i >= 0 as libc::c_int &&
              {
                  ev = *events.data.offset(i as isize);
                  (1 as libc::c_int) != 0
              } {
        if verto_get_fd(ev) == fd { return 1 as libc::c_int }
        i -= 1
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1429:1"]
unsafe extern "C" fn accept_rpc_connection(mut ctx: *mut verto_ctx,
                                           mut ev: *mut verto_ev) {
    let mut flags: verto_ev_flag = VERTO_EV_FLAG_NONE;
    let mut conn: *mut connection = 0 as *mut connection;
    let mut fds: fd_set = fd_set{fds_bits: [0; 16],};
    let mut s: libc::c_int = 0;
    conn = verto_get_private(ev) as *mut connection;
    /* Service the woken RPC listener descriptor. */
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh12 = &mut __d0;
    let fresh13;
    let fresh14 = &mut __d1;
    let fresh15;
    let fresh16 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh17 =
        &mut *fds.fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh13), "={di}" (fresh15) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh16)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh14, fresh17)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh16, fresh13);
    c2rust_asm_casts::AsmCast::cast_out(fresh14, fresh17, fresh15);
    fds.fds_bits[(verto_get_fd(ev) /
                      (8 as libc::c_int *
                           ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                               as libc::c_int)) as usize] |=
        ((1 as libc::c_ulong) <<
             verto_get_fd(ev) %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    gssrpc_svc_getreqset(&mut fds);
    /* Scan svc_fdset for any new connections. */
    s = 0 as libc::c_int;
    while s < 1024 as libc::c_int {
        let mut addr_s: sockaddr_storage =
            sockaddr_storage{ss_family: 0,
                             __ss_padding: [0; 118],
                             __ss_align: 0,};
        let mut addr: *mut sockaddr =
            &mut addr_s as *mut sockaddr_storage as *mut sockaddr;
        let mut addrlen: socklen_t =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        let mut newconn: *mut connection = 0 as *mut connection;
        let mut tmpbuf: [libc::c_char; 10] = [0; 10];
        let mut newev: *mut verto_ev = 0 as *mut verto_ev;
        /* If we already have this fd, continue. */
        if !(!(gssrpc_svc_fdset.fds_bits[(s /
                                              (8 as libc::c_int *
                                                   ::std::mem::size_of::<__fd_mask>()
                                                       as libc::c_ulong as
                                                       libc::c_int)) as usize]
                   &
                   ((1 as libc::c_ulong) <<
                        s %
                            (8 as libc::c_int *
                                 ::std::mem::size_of::<__fd_mask>() as
                                     libc::c_ulong as libc::c_int)) as
                       __fd_mask != 0 as libc::c_int as libc::c_long) ||
                 have_event_for_fd(s) != 0) {
            flags =
                (VERTO_EV_FLAG_IO_READ as libc::c_int |
                     VERTO_EV_FLAG_PERSIST as libc::c_int) as verto_ev_flag;
            if !(add_fd(s, CONN_RPC, flags, (*conn).handle, (*conn).prog, ctx,
                        Some(process_rpc_connection as
                                 unsafe extern "C" fn(_: *mut verto_ctx,
                                                      _: *mut verto_ev)
                                     -> ()), &mut newev) != 0 as libc::c_int)
               {
                newconn = verto_get_private(newev) as *mut connection;
                fcntl(s, 2 as libc::c_int, 1 as libc::c_int);
                if getpeername(s, __SOCKADDR_ARG{__sockaddr__: addr,},
                               &mut addrlen) != 0 ||
                       krb5int_getnameinfo(addr, addrlen,
                                           (*newconn).addrbuf.as_mut_ptr(),
                                           ::std::mem::size_of::<[libc::c_char; 56]>()
                                               as libc::c_ulong,
                                           tmpbuf.as_mut_ptr(),
                                           ::std::mem::size_of::<[libc::c_char; 10]>()
                                               as libc::c_ulong,
                                           1 as libc::c_int |
                                               2 as libc::c_int) != 0 {
                    krb5int_strlcpy((*newconn).addrbuf.as_mut_ptr(),
                                    b"???\x00" as *const u8 as
                                        *const libc::c_char,
                                    ::std::mem::size_of::<[libc::c_char; 56]>()
                                        as libc::c_ulong);
                } else {
                    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                    p = (*newconn).addrbuf.as_mut_ptr();
                    end =
                        p.offset(::std::mem::size_of::<[libc::c_char; 56]>()
                                     as libc::c_ulong as isize);
                    p = p.offset(strlen(p) as isize);
                    if end.wrapping_offset_from(p) as libc::c_long as size_t >
                           (2 as libc::c_int as
                                libc::c_ulong).wrapping_add(strlen(tmpbuf.as_mut_ptr()))
                       {
                        let fresh18 = p;
                        p = p.offset(1);
                        *fresh18 = '.' as i32 as libc::c_char;
                        krb5int_strlcpy(p, tmpbuf.as_mut_ptr(),
                                        end.wrapping_offset_from(p) as
                                            libc::c_long as size_t);
                    }
                }
                (*newconn).addr_s = addr_s;
                (*newconn).addrlen = addrlen;
                (*newconn).start_time = time(0 as *mut time_t);
                tcp_or_rpc_data_counter += 1;
                if tcp_or_rpc_data_counter > max_tcp_or_rpc_data_connections {
                    kill_lru_tcp_or_rpc_connection((*newconn).handle, newev);
                }
                (*newconn).remote_addr.address =
                    &mut (*newconn).remote_addr_buf;
                init_addr(&mut (*newconn).remote_addr,
                          ss2sa(&mut (*newconn).addr_s));
            }
        }
        s += 1
    };
}
#[c2rust::src_loc = "1496:1"]
unsafe extern "C" fn process_rpc_connection(mut ctx: *mut verto_ctx,
                                            mut ev: *mut verto_ev) {
    let mut fds: fd_set = fd_set{fds_bits: [0; 16],};
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh19 = &mut __d0;
    let fresh20;
    let fresh21 = &mut __d1;
    let fresh22;
    let fresh23 =
        (::std::mem::size_of::<fd_set>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<__fd_mask>() as
                                             libc::c_ulong);
    let fresh24 =
        &mut *fds.fds_bits.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut __fd_mask;
    asm!("cld; rep; stosq" : "={cx}" (fresh20), "={di}" (fresh22) : "{ax}"
         (0 as libc::c_int), "0"
         (c2rust_asm_casts::AsmCast::cast_in(fresh19, fresh23)), "1"
         (c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh24)) : "memory" :
         "volatile");
    c2rust_asm_casts::AsmCast::cast_out(fresh19, fresh23, fresh20);
    c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh24, fresh22);
    fds.fds_bits[(verto_get_fd(ev) /
                      (8 as libc::c_int *
                           ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                               as libc::c_int)) as usize] |=
        ((1 as libc::c_ulong) <<
             verto_get_fd(ev) %
                 (8 as libc::c_int *
                      ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                          libc::c_int)) as __fd_mask;
    gssrpc_svc_getreqset(&mut fds);
    if !(gssrpc_svc_fdset.fds_bits[(verto_get_fd(ev) /
                                        (8 as libc::c_int *
                                             ::std::mem::size_of::<__fd_mask>()
                                                 as libc::c_ulong as
                                                 libc::c_int)) as usize] &
             ((1 as libc::c_ulong) <<
                  verto_get_fd(ev) %
                      (8 as libc::c_int *
                           ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                               as libc::c_int)) as __fd_mask !=
             0 as libc::c_int as libc::c_long) {
        verto_del(ev);
    };
}
/* INET */
