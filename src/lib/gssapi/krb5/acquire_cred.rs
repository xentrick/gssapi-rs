use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:73"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:73"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:73"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:73"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:73"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:73"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:73"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:73"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:73"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:73"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn k5_os_mutex_init(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:73"]
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
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
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
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
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
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Credentials structure including ticket, session key, and lifetime info. */
    #[c2rust::src_loc = "2013:1"]
    pub type krb5_creds = _krb5_creds;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
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
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
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
    #[c2rust::src_loc = "2710:1"]
    pub type krb5_rcache = *mut krb5_rc_st;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "6424:16"]
    pub struct _krb5_prompt {
        pub prompt: *mut libc::c_char,
        pub hidden: libc::c_int,
        pub reply: *mut krb5_data,
    }
    /* *< Principal of this key */
    /* *< Time entry written to keytable */
    /* *< Key version number */
    /* *< The secret key */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    #[c2rust::src_loc = "6424:1"]
    pub type krb5_prompt = _krb5_prompt;
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Pointer to a prompter callback function. */
    #[c2rust::src_loc = "6431:1"]
    pub type krb5_prompter_fct
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char, _: libc::c_int,
                                    _: *mut krb5_prompt) -> krb5_error_code>;
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
    /* * Store options for @c _krb5_get_init_creds */
    #[c2rust::src_loc = "6811:1"]
    pub type krb5_get_init_creds_opt = _krb5_get_init_creds_opt;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, _krb5_kt};
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
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        /*
 * end "ccache.h"
 */
        /*
 * begin "rcache.h"
 */
        #[c2rust::src_loc = "2709:8"]
        pub type krb5_rc_st;
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
 * Get the default principal of a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] principal       Primary principal
 *
 * Returns the default client principal of a credential cache as set by
 * krb5_cc_initialize().
 *
 * Use krb5_free_principal() to free @a principal when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2479:1"]
        pub fn krb5_cc_get_principal(context: krb5_context,
                                     cache: krb5_ccache,
                                     principal: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Prepare to sequentially read every credential in a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] cursor          Cursor
 *
 * krb5_cc_end_seq_get() must be called to complete the retrieve operation.
 *
 * @note If the cache represented by @a cache is modified between the time of
 * the call to this function and the time of the final krb5_cc_end_seq_get(),
 * these changes may not be reflected in the results of krb5_cc_next_cred()
 * calls.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2499:1"]
        pub fn krb5_cc_start_seq_get(context: krb5_context,
                                     cache: krb5_ccache,
                                     cursor: *mut krb5_cc_cursor)
         -> krb5_error_code;
        /* *
 * Retrieve the next entry from the credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  cursor          Cursor
 * @param [out] creds           Next credential cache entry
 *
 * This function fills in @a creds with the next entry in @a cache and advances
 * @a cursor.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @sa krb5_cc_start_seq_get(), krb5_end_seq_get()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2520:1"]
        pub fn krb5_cc_next_cred(context: krb5_context, cache: krb5_ccache,
                                 cursor: *mut krb5_cc_cursor,
                                 creds: *mut krb5_creds) -> krb5_error_code;
        /* *
 * Finish a series of sequential processing credential cache entries.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] cursor           Cursor
 *
 * This function finishes processing credential cache entries and invalidates
 * @a cursor.
 *
 * @sa krb5_cc_start_seq_get(), krb5_cc_next_cred()
 *
 * @retval 0 (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "2538:1"]
        pub fn krb5_cc_end_seq_get(context: krb5_context, cache: krb5_ccache,
                                   cursor: *mut krb5_cc_cursor)
         -> krb5_error_code;
        /* *
 * Set options flags on a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] flags            Flag bit mask
 *
 * This function resets @a cache flags to @a flags.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2572:1"]
        pub fn krb5_cc_set_flags(context: krb5_context, cache: krb5_ccache,
                                 flags: krb5_flags) -> krb5_error_code;
        /* *
 * Retrieve the type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @return The type of a credential cache as an alias that must not be modified
 * or freed by the caller.
 */
        #[no_mangle]
        #[c2rust::src_loc = "2598:1"]
        pub fn krb5_cc_get_type(context: krb5_context, cache: krb5_ccache)
         -> *const libc::c_char;
        /* *
 * Check if the credential cache collection contains any credentials.
 *
 * @param [in]  context         Library context
 *
 * @version New in 1.11
 *
 * @retval 0 Credentials are available in the collection
 * @retval KRB5_CC_NOTFOUND The collection contains no credentials
 */
        #[no_mangle]
        #[c2rust::src_loc = "2681:1"]
        pub fn krb5_cccol_have_content(context: krb5_context)
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
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
        #[no_mangle]
        #[c2rust::src_loc = "2782:1"]
        pub fn krb5_kt_close(context: krb5_context, keytab: krb5_keytab)
         -> krb5_error_code;
        /* *
 * Get an entry from a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [in]  principal       Principal name
 * @param [in]  vno             Key version number (0 for highest available)
 * @param [in]  enctype         Encryption type (0 zero for any enctype)
 * @param [out] entry           Returned entry from key table
 *
 * Retrieve an entry from a key table which matches the @a keytab, @a
 * principal, @a vno, and @a enctype.  If @a vno is zero, retrieve the
 * highest-numbered kvno matching the other fields.  If @a enctype is 0, match
 * any enctype.
 *
 * Use krb5_free_keytab_entry_contents() to free @a entry when it is no longer
 * needed.
 *
 * @note If @a vno is zero, the function retrieves the highest-numbered-kvno
 * entry that matches the specified principal.
 *
 * @retval
 * 0 Success
 * @retval
 * Kerberos error codes on failure
 */
        #[no_mangle]
        #[c2rust::src_loc = "2811:1"]
        pub fn krb5_kt_get_entry(context: krb5_context, keytab: krb5_keytab,
                                 principal: krb5_const_principal,
                                 vno: krb5_kvno, enctype: krb5_enctype,
                                 entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        /* *
 * Start a sequential retrieval of key table entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * Prepare to read sequentially every key in the specified key table.  Use
 * krb5_kt_end_seq_get() to release the cursor when it is no longer needed.
 *
 * @sa krb5_kt_next_entry(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2833:1"]
        pub fn krb5_kt_start_seq_get(context: krb5_context,
                                     keytab: krb5_keytab,
                                     cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Retrieve the next entry from the key table.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] entry           Returned key table entry
 * @param [in]  cursor          Key table cursor
 *
 * Return the next sequential entry in @a keytab and advance @a cursor.
 * Callers must release the returned entry with krb5_kt_free_entry().
 *
 * @sa krb5_kt_start_seq_get(), krb5_kt_end_seq_get()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_END - if the last entry was reached
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2857:1"]
        pub fn krb5_kt_next_entry(context: krb5_context, keytab: krb5_keytab,
                                  entry: *mut krb5_keytab_entry,
                                  cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Release a keytab cursor.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] cursor          Cursor
 *
 * This function should be called to release the cursor created by
 * krb5_kt_start_seq_get().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2876:1"]
        pub fn krb5_kt_end_seq_get(context: krb5_context, keytab: krb5_keytab,
                                   cursor: *mut krb5_kt_cursor)
         -> krb5_error_code;
        /* *
 * Check if a keytab exists and contains entries.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 *
 * @version New in 1.11
 *
 * @retval 0 Keytab exists and contains entries
 * @retval KRB5_KT_NOTFOUND Keytab does not contain entries
 */
        #[no_mangle]
        #[c2rust::src_loc = "2891:1"]
        pub fn krb5_kt_have_content(context: krb5_context,
                                    keytab: krb5_keytab) -> krb5_error_code;
        /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
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
 * Generate a replay cache object for server use and open it.
 *
 * @param [in]  context         Library context
 * @param [in]  piece           Unused (replay cache identifier)
 * @param [out] rcptr           Handle to an open rcache
 *
 * This function creates a handle to the default replay cache.  Use
 * krb5_rc_close() to close @a rcptr when it is no longer needed.
 *
 * @version Prior to release 1.18, this function creates a handle to a
 * different replay cache for each unique value of @a piece.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3962:1"]
        pub fn krb5_get_server_rcache(context: krb5_context,
                                      piece: *const krb5_data,
                                      rcptr: *mut krb5_rcache)
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
        /* libkt.spec */
        /* *
 * Get a handle for a key table.
 *
 * @param [in]  context         Library context
 * @param [in]  name            Name of the key table
 * @param [out] ktid            Key table handle
 *
 * Resolve the key table name @a name and set @a ktid to a handle identifying
 * the key table.  Use krb5_kt_close() to free @a ktid when it is no longer
 * needed.
 *
 * @a name must be of the form @c type:residual, where @a type must be a type
 * known to the library and @a residual portion should be specific to the
 * particular keytab type.  If no @a type is given, the default is @c FILE.
 *
 * If @a name is of type @c FILE, the keytab file is not opened by this call.
 *
 * @code
 *  Example: krb5_kt_resolve(context, "FILE:/tmp/filename", &ktid);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        /* *
 * Duplicate keytab handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Key table handle to be duplicated
 * @param [out] out             Key table handle
 *
 * Create a new handle referring to the same key table as @a in.  The new
 * handle and @a in can be closed independently.
 *
 * @version New in 1.12
 */
        #[no_mangle]
        #[c2rust::src_loc = "4188:1"]
        pub fn krb5_kt_dup(context: krb5_context, in_0: krb5_keytab,
                           out: *mut krb5_keytab) -> krb5_error_code;
        /* *
 * Resolve the default key table.
 *
 * @param [in]  context         Library context
 * @param [out] id              Key table handle
 *
 * Set @a id to a handle to the default key table.  The key table is not
 * opened.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4226:1"]
        pub fn krb5_kt_default(context: krb5_context, id: *mut krb5_keytab)
         -> krb5_error_code;
        /* *
 * Resolve the default client key table.
 *
 * @param [in]     context      Library context
 * @param [out]    keytab_out   Key table handle
 *
 * Fill @a keytab_out with a handle to the default client key table.
 *
 * @version New in 1.11
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4244:1"]
        pub fn krb5_kt_client_default(context: krb5_context,
                                      keytab_out: *mut krb5_keytab)
         -> krb5_error_code;
        /* *
 * Free the contents of a key table entry.
 *
 * @param [in] context          Library context
 * @param [in] entry            Key table entry whose contents are to be freed
 *
 * @note The pointer is not freed.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4257:1"]
        pub fn krb5_free_keytab_entry_contents(context: krb5_context,
                                               entry: *mut krb5_keytab_entry)
         -> krb5_error_code;
        /* * @deprecated Use krb5_free_keytab_entry_contents instead. */
        #[no_mangle]
        #[c2rust::src_loc = "4261:1"]
        pub fn krb5_kt_free_entry(context: krb5_context,
                                  entry: *mut krb5_keytab_entry)
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
 * Duplicate ccache handle.
 *
 * @param [in]  context         Library context
 * @param [in]  in              Credential cache handle to be duplicated
 * @param [out] out             Credential cache handle
 *
 * Create a new handle referring to the same cache as @a in.
 * The new handle and @a in can be closed independently.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4354:1"]
        pub fn krb5_cc_dup(context: krb5_context, in_0: krb5_ccache,
                           out: *mut krb5_ccache) -> krb5_error_code;
        /* *
 * Resolve the default credential cache name.
 *
 * @param [in]  context         Library context
 * @param [out] ccache          Pointer to credential cache name
 *
 * Create a handle to the default credential cache as given by
 * krb5_cc_default_name().
 *
 * @retval
 * 0  Success
 * @retval
 * KV5M_CONTEXT            Bad magic number for @c _krb5_context structure
 * @retval
 * KRB5_FCC_INTERNAL       The name of the default credential cache cannot be
 *                         obtained
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        /* *
 * Store a configuration value in a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for a specific principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [in]     data         Data to store, or NULL to remove
 *
 * @note Existing configuration under the same key is over-written.
 *
 * @warning Before version 1.10 @a data was assumed to be always non-null.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4481:1"]
        pub fn krb5_cc_set_config(context: krb5_context, id: krb5_ccache,
                                  principal: krb5_const_principal,
                                  key: *const libc::c_char,
                                  data: *mut krb5_data) -> krb5_error_code;
        /* *
 * Test whether a principal is a configuration principal.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal to check
 *
 * @return
 * @c TRUE if the principal is a configuration principal (generated part of
 * krb5_cc_set_config()); @c FALSE otherwise.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4496:1"]
        pub fn krb5_is_config_principal(context: krb5_context,
                                        principal: krb5_const_principal)
         -> krb5_boolean;
        /* *
 * Determine whether a credential cache type supports switching.
 *
 * @param [in] context          Library context
 * @param [in] type             Credential cache type
 *
 * @version New in 1.10
 *
 * @retval TRUE if @a type supports switching
 * @retval FALSE if it does not or is not a valid credential cache type.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4527:1"]
        pub fn krb5_cc_support_switch(context: krb5_context,
                                      type_0: *const libc::c_char)
         -> krb5_boolean;
        /* *
 * Find a credential cache with a specified client principal.
 *
 * @param [in]  context         Library context
 * @param [in]  client          Client principal
 * @param [out] cache_out       Credential cache handle
 *
 * Find a cache within the collection whose default principal is @a client.
 * Use @a krb5_cc_close to close @a ccache when it is no longer needed.
 *
 * @retval 0 Success
 * @retval KRB5_CC_NOTFOUND
 *
 * @sa krb5_cccol_cursor_new
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "4547:1"]
        pub fn krb5_cc_cache_match(context: krb5_context,
                                   client: krb5_principal,
                                   cache_out: *mut krb5_ccache)
         -> krb5_error_code;
        /* *
 * Select a credential cache to use with a server principal.
 *
 * @param [in]  context         Library context
 * @param [in]  server          Server principal
 * @param [out] cache_out       Credential cache handle
 * @param [out] princ_out       Client principal
 *
 * Select a cache within the collection containing credentials most appropriate
 * for use with @a server, according to configured rules and heuristics.
 *
 * Use krb5_cc_close() to release @a cache_out when it is no longer needed.
 * Use krb5_free_principal() to release @a princ_out when it is no longer
 * needed.  Note that @a princ_out is set in some error conditions.
 *
 * @return
 * If an appropriate cache is found, 0 is returned, @a cache_out is set to the
 * selected cache, and @a princ_out is set to the default principal of that
 * cache.
 *
 * If the appropriate client principal can be authoritatively determined but
 * the cache collection contains no credentials for that principal, then
 * KRB5_CC_NOTFOUND is returned, @a cache_out is set to NULL, and @a princ_out
 * is set to the appropriate client principal.
 *
 * If no configured mechanism can determine the appropriate cache or principal,
 * KRB5_CC_NOTFOUND is returned and @a cache_out and @a princ_out are set to
 * NULL.
 *
 * Any other error code indicates a fatal error in the processing of a cache
 * selection mechanism.
 *
 * @version New in 1.10
 */
        #[no_mangle]
        #[c2rust::src_loc = "4585:1"]
        pub fn krb5_cc_select(context: krb5_context, server: krb5_principal,
                              cache_out: *mut krb5_ccache,
                              princ_out: *mut krb5_principal)
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
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4677:1"]
        pub fn krb5_free_cred_contents(context: krb5_context,
                                       val: *mut krb5_creds);
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
        /* *
 * Test whether a principal matches a matching principal.
 *
 * @param [in]  context         Library context
 * @param [in]  matching        Matching principal
 * @param [in]  princ           Principal to test
 *
 * @note A matching principal is a host-based principal with an empty realm
 * and/or second data component (hostname).  Profile configuration may cause
 * the hostname to be ignored even if it is present.  A principal matches a
 * matching principal if the former has the same non-empty (and non-ignored)
 * components of the latter.
 *
 * If @a matching is NULL, return TRUE.  If @a matching is not a matching
 * principal, return the value of krb5_principal_compare(context, matching,
 * princ).
 *
 * @return
 * TRUE if @a princ matches @a matching, FALSE otherwise.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4985:1"]
        pub fn krb5_sname_match(context: krb5_context,
                                matching: krb5_const_principal,
                                princ: krb5_const_principal) -> krb5_boolean;
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
        #[no_mangle]
        #[c2rust::src_loc = "6851:1"]
        pub fn krb5_get_init_creds_opt_alloc(context: krb5_context,
                                             opt:
                                                 *mut *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        /* *
 * Free initial credential options.
 *
 * @param [in] context          Library context
 * @param [in] opt              Options structure to free
 *
 * @sa krb5_get_init_creds_opt_alloc()
 */
        #[no_mangle]
        #[c2rust::src_loc = "6863:1"]
        pub fn krb5_get_init_creds_opt_free(context: krb5_context,
                                            opt:
                                                *mut krb5_get_init_creds_opt);
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
        #[no_mangle]
        #[c2rust::src_loc = "7097:1"]
        pub fn krb5_get_init_creds_opt_set_out_ccache(context: krb5_context,
                                                      opt:
                                                          *mut krb5_get_init_creds_opt,
                                                      ccache: krb5_ccache)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7268:1"]
        pub fn krb5_get_init_creds_password(context: krb5_context,
                                            creds: *mut krb5_creds,
                                            client: krb5_principal,
                                            password: *const libc::c_char,
                                            prompter: krb5_prompter_fct,
                                            data: *mut libc::c_void,
                                            start_time: krb5_deltat,
                                            in_tkt_service:
                                                *const libc::c_char,
                                            k5_gic_options:
                                                *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7661:1"]
        pub fn krb5_get_init_creds_keytab(context: krb5_context,
                                          creds: *mut krb5_creds,
                                          client: krb5_principal,
                                          arg_keytab: krb5_keytab,
                                          start_time: krb5_deltat,
                                          in_tkt_service: *const libc::c_char,
                                          k5_gic_options:
                                              *mut krb5_get_init_creds_opt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "7892:1"]
        pub fn krb5_set_error_message(ctx: krb5_context,
                                      code: krb5_error_code,
                                      fmt: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:73"]
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
    #[c2rust::src_loc = "996:1"]
    pub type krb5_authdata_context = *mut _krb5_authdata_context;
    #[inline]
    #[c2rust::src_loc = "666:1"]
    pub unsafe extern "C" fn zapfreestr(mut str: *mut libc::c_void) {
        if !str.is_null() {
            explicit_bzero(str, strlen(str as *mut libc::c_char));
            free(str);
        };
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
    #[inline]
    #[c2rust::src_loc = "2268:1"]
    pub unsafe extern "C" fn string2data(mut str: *mut libc::c_char)
     -> krb5_data {
        return make_data(str as *mut libc::c_void,
                         strlen(str) as libc::c_uint);
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
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2338:1"]
    pub unsafe extern "C" fn ts2tt(mut timestamp: krb5_timestamp) -> time_t {
        return timestamp as uint32_t as time_t;
    }
    #[inline]
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
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
                        krb5_keytab_entry, krb5_kt_cursor, krb5_authdatatype,
                        krb5_data, krb5_ccache, krb5_rcache, krb5_rc_st,
                        krb5_principal, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
    use super::string_h::{explicit_bzero, strlen, memcmp};
    use super::stdlib_h::{free, calloc};
    use super::stddef_h::size_t;
    use super::time_t_h::time_t;
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
        #[c2rust::src_loc = "901:1"]
        pub fn krb5int_copy_data_contents_add0(_: krb5_context,
                                               _: *const krb5_data,
                                               _: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1908:1"]
        pub fn krb5int_cc_default(_: krb5_context, _: *mut krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2023:1"]
        pub fn k5_rc_resolve(context: krb5_context, name: *const libc::c_char,
                             rc_out: *mut krb5_rcache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2025:1"]
        pub fn k5_rc_close(context: krb5_context, rc: krb5_rcache);
        #[no_mangle]
        #[c2rust::src_loc = "2126:1"]
        pub fn k5_kt_get_principal(context: krb5_context, keytab: krb5_keytab,
                                   princ_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2402:1"]
        pub fn k5_change_error_message_code(ctx: krb5_context,
                                            oldcode: krb5_error_code,
                                            newcode: krb5_error_code);
    }
    /* _KRB5_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:73"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:73"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:73"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:73"]
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
    #[c2rust::src_loc = "50:1"]
    pub type authdata_client_plugin_init_proc
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *mut *mut libc::c_void)
                   -> krb5_error_code>;
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
    use super::krb5_h::{krb5_error_code, krb5_context, krb5_authdatatype,
                        krb5_octet, krb5_auth_context, krb5_keyblock,
                        krb5_ap_req, krb5_boolean, krb5_authdata,
                        krb5_const_principal, krb5_flags, krb5_data};
    use super::k5_int_h::_krb5_authdata_context;
    use super::stddef_h::size_t;
    /* KRB5_AUTHDATA_PLUGIN_H_INCLUDED */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:74"]
pub mod gssapi_h {
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:74"]
pub mod gssapi_ext_h {
    /* Credential store extensions */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "530:8"]
    pub struct gss_key_value_element_struct {
        pub key: *const libc::c_char,
        pub value: *const libc::c_char,
    }
    #[c2rust::src_loc = "534:1"]
    pub type gss_key_value_element_desc = gss_key_value_element_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "536:8"]
    pub struct gss_key_value_set_struct {
        pub count: OM_uint32,
        pub elements: *mut gss_key_value_element_desc,
    }
    #[c2rust::src_loc = "540:1"]
    pub type gss_key_value_set_desc = gss_key_value_set_struct;
    #[c2rust::src_loc = "541:1"]
    pub type gss_const_key_value_set_t = *const gss_key_value_set_desc;
    use super::gssapi_h::OM_uint32;
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:74"]
pub mod gssapiP_krb5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "162:16"]
    pub struct _krb5_gss_name_rec {
        pub princ: krb5_principal,
        pub service: *mut libc::c_char,
        pub host: *mut libc::c_char,
        pub lock: k5_mutex_t,
        pub ad_context: krb5_authdata_context,
    }
    #[c2rust::src_loc = "162:1"]
    pub type krb5_gss_name_rec = _krb5_gss_name_rec;
    #[c2rust::src_loc = "162:1"]
    pub type krb5_gss_name_t = *mut _krb5_gss_name_rec;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "170:16"]
    pub struct _krb5_gss_cred_id_rec {
        pub lock: k5_mutex_t,
        pub usage: gss_cred_usage_t,
        pub name: krb5_gss_name_t,
        pub impersonator: krb5_principal,
        #[bitfield(name = "default_identity", ty = "libc::c_uint", bits =
                   "0..=0")]
        #[bitfield(name = "iakerb_mech", ty = "libc::c_uint", bits = "1..=1")]
        #[bitfield(name = "destroy_ccache", ty = "libc::c_uint", bits =
                   "2..=2")]
        #[bitfield(name = "suppress_ci_flags", ty = "libc::c_uint", bits =
                   "3..=3")]
        pub default_identity_iakerb_mech_destroy_ccache_suppress_ci_flags: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
        pub keytab: krb5_keytab,
        pub rcache: krb5_rcache,
        pub ccache: krb5_ccache,
        pub client_keytab: krb5_keytab,
        pub have_tgt: krb5_boolean,
        pub expire: krb5_timestamp,
        pub refresh_time: krb5_timestamp,
        pub req_enctypes: *mut krb5_enctype,
        pub password: *mut libc::c_char,
    }
    #[c2rust::src_loc = "170:1"]
    pub type krb5_gss_cred_id_rec = _krb5_gss_cred_id_rec;
    #[c2rust::src_loc = "170:1"]
    pub type krb5_gss_cred_id_t = *mut _krb5_gss_cred_id_rec;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1154:8"]
    pub struct krb5_gss_import_cred_req {
        pub id: krb5_ccache,
        pub keytab_principal: krb5_principal,
        pub keytab: krb5_keytab,
    }
    use super::krb5_h::{krb5_principal, krb5_keytab, krb5_rcache, krb5_ccache,
                        krb5_boolean, krb5_timestamp, krb5_enctype,
                        krb5_context, krb5_principal_data, krb5_flags,
                        krb5_error_code};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::{krb5_authdata_context, _krb5_context,
                          _krb5_authdata_context};
    use super::gssapi_h::{gss_cred_usage_t, OM_uint32, gss_cred_id_struct,
                          gss_cred_id_t};
    use super::gssapi_ext_h::{gss_key_value_set_desc,
                              gss_const_key_value_set_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn kg_sync_ccache_name(context: krb5_context,
                                   minor_status: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn kg_caller_provided_ccache_name(minor_status: *mut OM_uint32,
                                              out_caller_provided_name:
                                                  *mut libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1186:5"]
        pub fn krb5_gss_save_error_info(minor_code: OM_uint32,
                                        ctx: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "932:1"]
        pub fn kg_init_name(context: krb5_context, principal: krb5_principal,
                            service: *mut libc::c_char,
                            host: *mut libc::c_char,
                            ad_context: krb5_authdata_context,
                            flags: krb5_flags, name: *mut krb5_gss_name_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "905:1"]
        pub fn krb5_gss_validate_cred_1(_: *mut OM_uint32, _: gss_cred_id_t,
                                        _: krb5_context) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "937:1"]
        pub fn kg_release_name(context: krb5_context,
                               name: *mut krb5_gss_name_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "949:1"]
        pub fn kg_acceptor_princ(context: krb5_context, name: krb5_gss_name_t,
                                 princ_out: *mut krb5_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "940:1"]
        pub fn kg_duplicate_name(context: krb5_context, src: krb5_gss_name_t,
                                 dst: *mut krb5_gss_name_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1105:1"]
        pub fn krb5_gss_init_context(ctxp: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1171:1"]
        pub fn gss_krb5int_initialize_library() -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1288:1"]
        pub fn kg_value_from_cred_store(cred_store: gss_const_key_value_set_t,
                                        type_0: *const libc::c_char,
                                        value: *mut *const libc::c_char)
         -> OM_uint32;
    }
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:73"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:73"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:73"]
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
        #[c2rust::src_loc = "107:1"]
        pub fn atol(__nptr: *const libc::c_char) -> libc::c_long;
    }
}
#[c2rust::header_src = "/usr/include/string.h:73"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
#[c2rust::header_src = "/usr/include/assert.h:73"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-trace.h:73"]
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
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __off_t, __off64_t,
                        __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_init,
                            k5_mutex_lock, k5_mutex_unlock, k5_os_mutex_init,
                            k5_os_mutex_destroy, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_auth_context,
                       _krb5_keyblock, krb5_keyblock, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_creds, krb5_creds,
                       _krb5_ap_req, krb5_ap_req, krb5_cc_cursor, krb5_ccache,
                       krb5_rcache, krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _krb5_prompt,
                       krb5_prompt, krb5_prompter_fct,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       _profile_t, _krb5_auth_context, _krb5_ccache,
                       krb5_rc_st, krb5_cc_destroy, krb5_cc_close,
                       krb5_cc_get_principal, krb5_cc_start_seq_get,
                       krb5_cc_next_cred, krb5_cc_end_seq_get,
                       krb5_cc_set_flags, krb5_cc_get_type,
                       krb5_cccol_have_content, krb5_cc_new_unique,
                       krb5_kt_close, krb5_kt_get_entry,
                       krb5_kt_start_seq_get, krb5_kt_next_entry,
                       krb5_kt_end_seq_get, krb5_kt_have_content,
                       krb5_free_context, krb5_parse_name, krb5_unparse_name,
                       krb5_principal_compare, krb5_get_server_rcache,
                       krb5_build_principal_ext, krb5_kt_resolve, krb5_kt_dup,
                       krb5_kt_default, krb5_kt_client_default,
                       krb5_free_keytab_entry_contents, krb5_kt_free_entry,
                       krb5_cc_resolve, krb5_cc_dup, krb5_cc_default,
                       krb5_cc_set_config, krb5_is_config_principal,
                       krb5_cc_support_switch, krb5_cc_cache_match,
                       krb5_cc_select, krb5_free_principal,
                       krb5_free_cred_contents, krb5_free_data_contents,
                       krb5_timeofday, krb5_sname_match,
                       krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_set_out_ccache,
                       krb5_get_init_creds_password,
                       krb5_get_init_creds_keytab, krb5_set_error_message,
                       krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_authdata_context,
                         zapfreestr, data_eq_string, make_data, empty_data,
                         string2data, k5calloc, k5alloc, ts2tt, ts_delta,
                         ts_incr, ts_after, plugin_mapping, _kdb_log_context,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle,
                         krb5int_copy_data_contents_add0, krb5int_cc_default,
                         k5_rc_resolve, k5_rc_close, k5_kt_get_principal,
                         k5_change_error_message_code};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::authdata_plugin_h::{authdata_client_plugin_init_proc,
                                  authdata_client_request_fini_proc,
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
                                  authdata_client_plugin_fini_proc};
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_name_t, gss_cred_id_t,
                         gss_OID_desc_struct, gss_OID,
                         gss_OID_set_desc_struct, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_t,
                         gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct};
pub use self::gssapi_ext_h::{gss_key_value_element_struct,
                             gss_key_value_element_desc,
                             gss_key_value_set_struct, gss_key_value_set_desc,
                             gss_const_key_value_set_t};
pub use self::gssapiP_krb5_h::{_krb5_gss_name_rec, krb5_gss_name_rec,
                               krb5_gss_name_t, _krb5_gss_cred_id_rec,
                               krb5_gss_cred_id_rec, krb5_gss_cred_id_t,
                               krb5_gss_import_cred_req, kg_sync_ccache_name,
                               kg_caller_provided_ccache_name,
                               krb5_gss_save_error_info, kg_init_name,
                               krb5_gss_validate_cred_1, kg_release_name,
                               kg_acceptor_princ, kg_duplicate_name,
                               krb5_gss_init_context,
                               gss_krb5int_initialize_library,
                               kg_value_from_cred_store};
use self::stdio_h::{fprintf, snprintf, stderr};
use self::libintl_h::dgettext;
use self::stdlib_h::{free, calloc, atol};
use self::string_h::{explicit_bzero, strerror, strlen, strdup, memcmp,
                     memset};
use self::assert_h::__assert_fail;
use self::k5_trace_h::krb5int_trace;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2007-2010 by the Massachusetts Institute of Technology.
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
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
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
#[no_mangle]
#[c2rust::src_loc = "92:12"]
pub static mut gssint_krb5_keytab_lock: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
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
                        },};
#[c2rust::src_loc = "93:14"]
static mut krb5_gss_keytab: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Heimdal calls this gsskrb5_register_acceptor_identity. */
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn gss_krb5int_register_acceptor_identity(mut minor_status:
                                                                    *mut OM_uint32,
                                                                desired_mech:
                                                                    gss_OID,
                                                                desired_object:
                                                                    gss_OID,
                                                                mut value:
                                                                    gss_buffer_t)
 -> OM_uint32 {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: libc::c_int = 0;
    err = gss_krb5int_initialize_library() as libc::c_int;
    if err != 0 as libc::c_int {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !(*value).value.is_null() {
        new = strdup((*value).value as *mut libc::c_char);
        if new.is_null() {
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    }
    k5_mutex_lock(&mut gssint_krb5_keytab_lock);
    old = krb5_gss_keytab;
    krb5_gss_keytab = new;
    k5_mutex_unlock(&mut gssint_krb5_keytab_lock);
    free(old as *mut libc::c_void);
    return 0 as libc::c_int as OM_uint32;
}
/* Try to verify that keytab contains at least one entry for name.  Return 0 if
 * it does, KRB5_KT_NOTFOUND if it doesn't, or another error as appropriate. */
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn check_keytab(mut context: krb5_context,
                                  mut kt: krb5_keytab,
                                  mut name: krb5_gss_name_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut ent: krb5_keytab_entry =
        krb5_keytab_entry{magic: 0,
                          principal: 0 as *mut krb5_principal_data,
                          timestamp: 0,
                          vno: 0,
                          key:
                              krb5_keyblock{magic: 0,
                                            enctype: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    let mut cursor: krb5_kt_cursor = 0 as *mut libc::c_void;
    let mut accprinc: krb5_principal = 0 as krb5_principal;
    let mut match_0: krb5_boolean = 0;
    let mut princname: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*name).service.is_null() {
        code =
            krb5_kt_get_entry(context, kt,
                              (*name).princ as krb5_const_principal,
                              0 as libc::c_int as krb5_kvno, 0 as libc::c_int,
                              &mut ent);
        if code == 0 as libc::c_int { krb5_kt_free_entry(context, &mut ent); }
        return code
    }
    /* If we can't iterate through the keytab, skip this check. */
    if (*(*kt).ops).start_seq_get.is_none() { return 0 as libc::c_int }
    /* Get the partial principal for the acceptor name. */
    code = kg_acceptor_princ(context, name, &mut accprinc) as krb5_error_code;
    if code != 0 { return code }
    /* Scan the keytab for host-based entries matching accprinc. */
    code = krb5_kt_start_seq_get(context, kt, &mut cursor);
    if !(code != 0) {
        loop  {
            code = krb5_kt_next_entry(context, kt, &mut ent, &mut cursor);
            if !(code == 0 as libc::c_int) { break ; }
            match_0 =
                krb5_sname_match(context, accprinc as krb5_const_principal,
                                 ent.principal as krb5_const_principal);
            krb5_free_keytab_entry_contents(context, &mut ent);
            if match_0 != 0 { break ; }
        }
        krb5_kt_end_seq_get(context, kt, &mut cursor);
        if code as libc::c_long == -(1765328202 as libc::c_long) {
            code = -(1765328203 as libc::c_long) as krb5_error_code;
            if krb5_unparse_name(context, accprinc as krb5_const_principal,
                                 &mut princname) == 0 as libc::c_int {
                krb5_set_error_message(context, code,
                                       dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"No key table entry found matching %s\x00"
                                                    as *const u8 as
                                                    *const libc::c_char),
                                       princname);
                free(princname as *mut libc::c_void);
            }
        }
    }
    krb5_free_principal(context, accprinc);
    return code;
}
/* get credentials corresponding to a key in the krb5 keytab.
   If successful, set the keytab-specific fields in cred
*/
#[c2rust::src_loc = "180:1"]
unsafe extern "C" fn acquire_accept_cred(mut context: krb5_context,
                                         mut minor_status: *mut OM_uint32,
                                         mut req_keytab: krb5_keytab,
                                         mut rcname: *const libc::c_char,
                                         mut cred: *mut krb5_gss_cred_id_rec)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut major: OM_uint32 = 0;
    let mut code: krb5_error_code = 0;
    let mut kt: krb5_keytab = 0 as krb5_keytab;
    let mut rc: krb5_rcache = 0 as krb5_rcache;
    if (*cred).keytab.is_null() {
    } else {
        __assert_fail(b"cred->keytab == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const libc::c_char,
                      190 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 108],
                                                &[libc::c_char; 108]>(b"OM_uint32 acquire_accept_cred(krb5_context, OM_uint32 *, krb5_keytab, const char *, krb5_gss_cred_id_rec *)\x00")).as_ptr());
    }
    /* If we have an explicit rcache name, open it. */
    if !rcname.is_null() {
        code = k5_rc_resolve(context, rcname, &mut rc);
        if code != 0 {
            major = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
            current_block = 6869834285901761501;
        } else { current_block = 10886091980245723256; }
    } else { current_block = 10886091980245723256; }
    match current_block {
        10886091980245723256 => {
            if !req_keytab.is_null() {
                code = krb5_kt_dup(context, req_keytab, &mut kt)
            } else {
                k5_mutex_lock(&mut gssint_krb5_keytab_lock);
                if !krb5_gss_keytab.is_null() {
                    code = krb5_kt_resolve(context, krb5_gss_keytab, &mut kt);
                    k5_mutex_unlock(&mut gssint_krb5_keytab_lock);
                } else {
                    k5_mutex_unlock(&mut gssint_krb5_keytab_lock);
                    code = krb5_kt_default(context, &mut kt)
                }
            }
            if code != 0 {
                major =
                    (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
            } else {
                if !(*cred).name.is_null() {
                    /* Make sure we have keys matching the desired name in the keytab. */
                    code = check_keytab(context, kt, (*cred).name);
                    if code != 0 {
                        if code as libc::c_long ==
                               -(1765328203 as libc::c_long) {
                            k5_change_error_message_code(context, code,
                                                         39756033 as
                                                             libc::c_long as
                                                             krb5_error_code);
                            code = 39756033 as libc::c_long as krb5_error_code
                        }
                        major =
                            (13 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int;
                        current_block = 6869834285901761501;
                    } else if rc.is_null() {
                        /* Open the replay cache for this principal. */
                        code =
                            krb5_get_server_rcache(context,
                                                   &mut *(*(*(*cred).name).princ).data.offset(0
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  isize),
                                                   &mut rc);
                        if code != 0 {
                            major =
                                (13 as libc::c_ulong as OM_uint32) <<
                                    16 as libc::c_int;
                            current_block = 6869834285901761501;
                        } else { current_block = 13131896068329595644; }
                    } else { current_block = 13131896068329595644; }
                } else {
                    /* Make sure we have a keytab with keys in it. */
                    code = krb5_kt_have_content(context, kt);
                    if code != 0 {
                        major =
                            (13 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int;
                        current_block = 6869834285901761501;
                    } else { current_block = 13131896068329595644; }
                }
                match current_block {
                    6869834285901761501 => { }
                    _ => {
                        (*cred).keytab = kt;
                        kt = 0 as krb5_keytab;
                        (*cred).rcache = rc;
                        rc = 0 as krb5_rcache;
                        major = 0 as libc::c_int as OM_uint32
                    }
                }
            }
        }
        _ => { }
    }
    if !kt.is_null() { krb5_kt_close(context, kt); }
    if !rc.is_null() { k5_rc_close(context, rc); }
    *minor_status = code as OM_uint32;
    return major;
}
/* LEAN_CLIENT */
/* USE_LEASH */
/* Set fields in cred according to a ccache config entry whose key (in
 * principal form) is config_princ and whose value is value. */
#[c2rust::src_loc = "306:1"]
unsafe extern "C" fn scan_cc_config(mut context: krb5_context,
                                    mut cred: *mut krb5_gss_cred_id_rec,
                                    mut config_princ: krb5_const_principal,
                                    mut value: *const krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut data0: krb5_data = empty_data();
    if (*config_princ).length != 2 as libc::c_int { return 0 as libc::c_int }
    if data_eq_string(*(*config_princ).data.offset(1 as libc::c_int as isize),
                      b"proxy_impersonator\x00" as *const u8 as
                          *const libc::c_char) != 0 &&
           (*cred).impersonator.is_null() {
        code = krb5int_copy_data_contents_add0(context, value, &mut data0);
        if code != 0 { return code }
        code =
            krb5_parse_name(context, data0.data, &mut (*cred).impersonator);
        krb5_free_data_contents(context, &mut data0);
        if code != 0 { return code }
    } else if data_eq_string(*(*config_princ).data.offset(1 as libc::c_int as
                                                              isize),
                             b"refresh_time\x00" as *const u8 as
                                 *const libc::c_char) != 0 &&
                  (*cred).refresh_time == 0 as libc::c_int {
        code = krb5int_copy_data_contents_add0(context, value, &mut data0);
        if code != 0 { return code }
        (*cred).refresh_time = atol(data0.data) as krb5_timestamp;
        krb5_free_data_contents(context, &mut data0);
    }
    return 0 as libc::c_int;
}
/* Return true if it appears that we can non-interactively get initial
 * tickets for cred. */
#[c2rust::src_loc = "337:1"]
unsafe extern "C" fn can_get_initial_creds(mut context: krb5_context,
                                           mut cred:
                                               *mut krb5_gss_cred_id_rec)
 -> krb5_boolean {
    let mut code: krb5_error_code = 0;
    let mut entry: krb5_keytab_entry =
        krb5_keytab_entry{magic: 0,
                          principal: 0 as *mut krb5_principal_data,
                          timestamp: 0,
                          vno: 0,
                          key:
                              krb5_keyblock{magic: 0,
                                            enctype: 0,
                                            length: 0,
                                            contents:
                                                0 as *mut krb5_octet,},};
    if !(*cred).password.is_null() { return 1 as libc::c_int as krb5_boolean }
    if (*cred).client_keytab.is_null() {
        return 0 as libc::c_int as krb5_boolean
    }
    /* If we don't know the client principal yet, check for any keytab keys. */
    if (*cred).name.is_null() {
        return (krb5_kt_have_content(context, (*cred).client_keytab) == 0) as
                   libc::c_int as krb5_boolean
    }
    /* Check if we have a keytab key for the client principal. */
    code =
        krb5_kt_get_entry(context, (*cred).client_keytab,
                          (*(*cred).name).princ as krb5_const_principal,
                          0 as libc::c_int as krb5_kvno, 0 as libc::c_int,
                          &mut entry);
    if code != 0 {
        krb5_clear_error_message(context);
        return 0 as libc::c_int as krb5_boolean
    }
    krb5_free_keytab_entry_contents(context, &mut entry);
    return 1 as libc::c_int as krb5_boolean;
}
/* Scan cred->ccache for name, expiry time, impersonator, refresh time. */
#[c2rust::src_loc = "365:1"]
unsafe extern "C" fn scan_ccache(mut context: krb5_context,
                                 mut cred: *mut krb5_gss_cred_id_rec)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut ccache: krb5_ccache = (*cred).ccache;
    let mut ccache_princ: krb5_principal = 0 as krb5_principal;
    let mut tgt_princ: krb5_principal = 0 as krb5_principal;
    let mut realm: *mut krb5_data = 0 as *mut krb5_data;
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
    let mut endtime: krb5_timestamp = 0;
    let mut is_tgt: krb5_boolean = 0;
    /* Turn on NOTICKET, as we don't need session keys here. */
    code = krb5_cc_set_flags(context, ccache, 0x2 as libc::c_int);
    if code != 0 { return code }
    /* Credentials cache principal must match the initiator name. */
    code = krb5_cc_get_principal(context, ccache, &mut ccache_princ);
    if !(code != 0 as libc::c_int) {
        if !(*cred).name.is_null() &&
               krb5_principal_compare(context,
                                      ccache_princ as krb5_const_principal,
                                      (*(*cred).name).princ as
                                          krb5_const_principal) == 0 {
            code = 39756032 as libc::c_long as krb5_error_code
        } else {
            /* Save the ccache principal as the credential name if not already set. */
            if (*cred).name.is_null() {
                code =
                    kg_init_name(context, ccache_princ,
                                 0 as *mut libc::c_char,
                                 0 as *mut libc::c_char,
                                 0 as krb5_authdata_context,
                                 0x1 as libc::c_int, &mut (*cred).name);
                if code != 0 {
                    current_block = 16016496521494030081;
                } else {
                    ccache_princ = 0 as krb5_principal;
                    current_block = 1054647088692577877;
                }
            } else { current_block = 1054647088692577877; }
            match current_block {
                16016496521494030081 => { }
                _ => {
                    if !(*(*cred).name).princ.is_null() {
                    } else {
                        __assert_fail(b"cred->name->princ != NULL\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"acquire_cred.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      401 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 66],
                                                                &[libc::c_char; 66]>(b"krb5_error_code scan_ccache(krb5_context, krb5_gss_cred_id_rec *)\x00")).as_ptr());
                    }
                    realm = &mut (*(*(*cred).name).princ).realm;
                    code =
                        krb5_build_principal_ext(context,
                                                 &mut tgt_princ as
                                                     *mut krb5_principal,
                                                 (*realm).length,
                                                 (*realm).data,
                                                 6 as libc::c_int,
                                                 b"krbtgt\x00" as *const u8 as
                                                     *const libc::c_char,
                                                 (*realm).length,
                                                 (*realm).data,
                                                 0 as libc::c_int);
                    if code != 0 { return code }
                    /* If there's a tgt for the principal's local realm in here, use its expiry
     * time.  Otherwise use the first key. */
                    code =
                        krb5_cc_start_seq_get(context, ccache, &mut cursor);
                    if code != 0 {
                        krb5_free_principal(context, tgt_princ);
                        return code
                    }
                    loop  {
                        code =
                            krb5_cc_next_cred(context, ccache, &mut cursor,
                                              &mut creds);
                        if !(code == 0) { break ; }
                        if krb5_is_config_principal(context,
                                                    creds.server as
                                                        krb5_const_principal)
                               != 0 {
                            code =
                                scan_cc_config(context, cred,
                                               creds.server as
                                                   krb5_const_principal,
                                               &mut creds.ticket);
                            krb5_free_cred_contents(context, &mut creds);
                            if code != 0 { break ; }
                        } else {
                            is_tgt =
                                krb5_principal_compare(context,
                                                       tgt_princ as
                                                           krb5_const_principal,
                                                       creds.server as
                                                           krb5_const_principal);
                            endtime = creds.times.endtime;
                            krb5_free_cred_contents(context, &mut creds);
                            if is_tgt != 0 {
                                (*cred).have_tgt =
                                    1 as libc::c_int as krb5_boolean
                            }
                            if is_tgt != 0 ||
                                   (*cred).expire == 0 as libc::c_int {
                                (*cred).expire = endtime
                            }
                        }
                    }
                    krb5_cc_end_seq_get(context, ccache, &mut cursor);
                    if !(code != 0 &&
                             code as libc::c_long !=
                                 -(1765328242 as libc::c_long)) {
                        code = 0 as libc::c_int;
                        if (*cred).expire == 0 as libc::c_int &&
                               can_get_initial_creds(context, cred) == 0 {
                            code = 39756044 as libc::c_long as krb5_error_code
                        }
                    }
                }
            }
        }
    }
    krb5_cc_set_flags(context, ccache, 0 as libc::c_int);
    krb5_free_principal(context, ccache_princ);
    krb5_free_principal(context, tgt_princ);
    return code;
}
/* Find an existing or destination ccache for cred->name. */
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn get_cache_for_name(mut context: krb5_context,
                                        mut cred: *mut krb5_gss_cred_id_rec)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut can_get: krb5_boolean = 0;
    let mut have_collection: krb5_boolean = 0;
    let mut defcc: krb5_ccache = 0 as krb5_ccache;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut cctype: *const libc::c_char = 0 as *const libc::c_char;
    if !(*cred).name.is_null() && (*cred).ccache.is_null() {
    } else {
        __assert_fail(b"cred->name != NULL && cred->ccache == NULL\x00" as
                          *const u8 as *const libc::c_char,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const libc::c_char,
                      461 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 73],
                                                &[libc::c_char; 73]>(b"krb5_error_code get_cache_for_name(krb5_context, krb5_gss_cred_id_rec *)\x00")).as_ptr());
    }
    /* Check first whether we can acquire tickets, to avoid overwriting the
     * extended error message from krb5_cc_cache_match. */
    can_get = can_get_initial_creds(context, cred);
    /* Look for an existing cache for the client principal. */
    code =
        krb5_cc_cache_match(context, (*(*cred).name).princ,
                            &mut (*cred).ccache);
    if code == 0 as libc::c_int { return scan_ccache(context, cred) }
    if code as libc::c_long != -(1765328243 as libc::c_long) || can_get == 0 {
        return code
    }
    krb5_clear_error_message(context);
    /* There is no existing ccache, but we can acquire credentials.  Get the
     * default ccache to help decide where we should put them. */
    code = krb5_cc_default(context, &mut defcc);
    if code != 0 { return code }
    cctype = krb5_cc_get_type(context, defcc);
    have_collection = krb5_cc_support_switch(context, cctype);
    /* We can use an empty default ccache if we're using a password or if
     * there's no collection. */
    if !(*cred).password.is_null() || have_collection == 0 {
        if krb5_cc_get_principal(context, defcc, &mut princ) as libc::c_long
               == -(1765328189 as libc::c_long) {
            (*cred).ccache = defcc;
            defcc = 0 as krb5_ccache
        }
        krb5_clear_error_message(context);
    }
    /* Otherwise, try to use a new cache in the collection. */
    if (*cred).ccache.is_null() {
        if have_collection == 0 {
            code = 39756032 as libc::c_long as krb5_error_code
        } else {
            code =
                krb5_cc_new_unique(context, cctype, 0 as *const libc::c_char,
                                   &mut (*cred).ccache);
            (code) != 0;
        }
    }
    krb5_free_principal(context, princ);
    if !defcc.is_null() { krb5_cc_close(context, defcc); }
    return code;
    /* not USE_LEASH */
}
/* Try to set cred->name using the client keytab. */
#[c2rust::src_loc = "516:1"]
unsafe extern "C" fn get_name_from_client_keytab(mut context: krb5_context,
                                                 mut cred:
                                                     *mut krb5_gss_cred_id_rec)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    if (*cred).name.is_null() {
    } else {
        __assert_fail(b"cred->name == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const libc::c_char,
                      522 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 82],
                                                &[libc::c_char; 82]>(b"krb5_error_code get_name_from_client_keytab(krb5_context, krb5_gss_cred_id_rec *)\x00")).as_ptr());
    }
    if (*cred).client_keytab.is_null() {
        return -(1765328203 as libc::c_long) as krb5_error_code
    }
    code = k5_kt_get_principal(context, (*cred).client_keytab, &mut princ);
    if code != 0 { return code }
    code =
        kg_init_name(context, princ, 0 as *mut libc::c_char,
                     0 as *mut libc::c_char, 0 as krb5_authdata_context,
                     0x1 as libc::c_int, &mut (*cred).name);
    if code != 0 { krb5_free_principal(context, princ); return code }
    return 0 as libc::c_int;
}
/* Make a note in ccache that we should attempt to refresh it from the client
 * keytab at refresh_time. */
#[c2rust::src_loc = "541:1"]
unsafe extern "C" fn set_refresh_time(mut context: krb5_context,
                                      mut ccache: krb5_ccache,
                                      mut refresh_time: krb5_timestamp) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    snprintf(buf.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
             b"%u\x00" as *const u8 as *const libc::c_char,
             ts2tt(refresh_time) as libc::c_uint);
    d = string2data(buf.as_mut_ptr());
    krb5_cc_set_config(context, ccache, 0 as krb5_const_principal,
                       b"refresh_time\x00" as *const u8 as
                           *const libc::c_char, &mut d);
    krb5_clear_error_message(context);
}
/* Return true if it's time to refresh cred from the client keytab.  If
 * returning true, avoid retrying for 30 seconds. */
#[no_mangle]
#[c2rust::src_loc = "557:1"]
pub unsafe extern "C" fn kg_cred_time_to_refresh(mut context: krb5_context,
                                                 mut cred:
                                                     *mut krb5_gss_cred_id_rec)
 -> krb5_boolean {
    let mut now: krb5_timestamp = 0;
    if krb5_timeofday(context, &mut now) != 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    if (*cred).refresh_time != 0 as libc::c_int &&
           ts_after((*cred).refresh_time, now) == 0 {
        set_refresh_time(context, (*cred).ccache,
                         ts_incr((*cred).refresh_time, 30 as libc::c_int));
        return 1 as libc::c_int as krb5_boolean
    }
    return 0 as libc::c_int as krb5_boolean;
}
/* If appropriate, make a note to refresh cred from the client keytab when it
 * is halfway to expired. */
#[no_mangle]
#[c2rust::src_loc = "574:1"]
pub unsafe extern "C" fn kg_cred_set_initial_refresh(mut context:
                                                         krb5_context,
                                                     mut cred:
                                                         *mut krb5_gss_cred_id_rec,
                                                     mut times:
                                                         *mut krb5_ticket_times) {
    let mut refresh: krb5_timestamp = 0;
    /* For now, we only mark keytab-acquired credentials for refresh. */
    if !(*cred).password.is_null() { return }
    /* Make a note to refresh these when they are halfway to expired. */
    refresh =
        ts_incr((*times).starttime,
                ts_delta((*times).endtime, (*times).starttime) /
                    2 as libc::c_int);
    set_refresh_time(context, (*cred).ccache, refresh);
}
/* Get initial credentials using the supplied password or client keytab. */
#[c2rust::src_loc = "591:1"]
unsafe extern "C" fn get_initial_cred(mut context: krb5_context,
                                      mut cred: *mut krb5_gss_cred_id_rec)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut opt: *mut krb5_get_init_creds_opt =
        0 as *mut krb5_get_init_creds_opt;
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
    code = krb5_get_init_creds_opt_alloc(context, &mut opt);
    if code != 0 { return code }
    code =
        krb5_get_init_creds_opt_set_out_ccache(context, opt, (*cred).ccache);
    if !(code != 0) {
        if !(*cred).password.is_null() {
            code =
                krb5_get_init_creds_password(context, &mut creds,
                                             (*(*cred).name).princ,
                                             (*cred).password, None,
                                             0 as *mut libc::c_void,
                                             0 as libc::c_int,
                                             0 as *const libc::c_char, opt)
        } else if !(*cred).client_keytab.is_null() {
            code =
                krb5_get_init_creds_keytab(context, &mut creds,
                                           (*(*cred).name).princ,
                                           (*cred).client_keytab,
                                           0 as libc::c_int,
                                           0 as *const libc::c_char, opt)
        } else { code = -(1765328203 as libc::c_long) as krb5_error_code }
        if !(code != 0) {
            kg_cred_set_initial_refresh(context, cred, &mut creds.times);
            (*cred).have_tgt = 1 as libc::c_int as krb5_boolean;
            (*cred).expire = creds.times.endtime;
            krb5_free_cred_contents(context, &mut creds);
        }
    }
    krb5_get_init_creds_opt_free(context, opt);
    return code;
}
/* Get initial credentials if we ought to and are able to. */
#[c2rust::src_loc = "626:1"]
unsafe extern "C" fn maybe_get_initial_cred(mut context: krb5_context,
                                            mut cred:
                                                *mut krb5_gss_cred_id_rec)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    /* Don't get creds if we don't know the name or are doing IAKERB. */
    if (*cred).name.is_null() || (*cred).iakerb_mech() as libc::c_int != 0 {
        return 0 as libc::c_int
    }
    /* Get creds if we have none or if it's time to refresh. */
    if (*cred).expire == 0 as libc::c_int ||
           kg_cred_time_to_refresh(context, cred) != 0 {
        code = get_initial_cred(context, cred);
        /* If we were trying to refresh and failed, we can keep going. */
        if code != 0 && (*cred).expire == 0 as libc::c_int { return code }
        krb5_clear_error_message(context);
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "646:1"]
unsafe extern "C" fn acquire_init_cred(mut context: krb5_context,
                                       mut minor_status: *mut OM_uint32,
                                       mut req_ccache: krb5_ccache,
                                       mut password: gss_buffer_t,
                                       mut client_keytab: krb5_keytab,
                                       mut cred: *mut krb5_gss_cred_id_rec)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut pwdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut pwcopy: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut caller_ccname: libc::c_int = 0 as libc::c_int;
    /* Get ccache from caller if available. */
    if kg_sync_ccache_name(context, minor_status) &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if kg_caller_provided_ccache_name(minor_status, &mut caller_ccname) &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !password.is_null() {
        pwdata =
            make_data((*password).value, (*password).length as libc::c_uint);
        code =
            krb5int_copy_data_contents_add0(context, &mut pwdata,
                                            &mut pwcopy);
        if code != 0 {
            current_block = 15700575537292975296;
        } else {
            (*cred).password = pwcopy.data;
            /* We will fetch the credential into a private memory ccache. */
            if req_ccache.is_null() {
            } else {
                __assert_fail(b"req_ccache == NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"acquire_cred.c\x00" as *const u8 as
                                  *const libc::c_char,
                              673 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 119],
                                                        &[libc::c_char; 119]>(b"OM_uint32 acquire_init_cred(krb5_context, OM_uint32 *, krb5_ccache, gss_buffer_t, krb5_keytab, krb5_gss_cred_id_rec *)\x00")).as_ptr());
            }
            code =
                krb5_cc_new_unique(context,
                                   b"MEMORY\x00" as *const u8 as
                                       *const libc::c_char,
                                   0 as *const libc::c_char,
                                   &mut (*cred).ccache);
            if code != 0 {
                current_block = 15700575537292975296;
            } else {
                (*cred).set_destroy_ccache(1 as libc::c_int as libc::c_uint);
                current_block = 12147880666119273379;
            }
        }
    } else if !req_ccache.is_null() {
        code = krb5_cc_dup(context, req_ccache, &mut (*cred).ccache);
        if code != 0 {
            current_block = 15700575537292975296;
        } else { current_block = 12147880666119273379; }
    } else if caller_ccname != 0 {
        /* Caller's ccache name has been set as the context default. */
        code = krb5int_cc_default(context, &mut (*cred).ccache);
        if code != 0 {
            current_block = 15700575537292975296;
        } else { current_block = 12147880666119273379; }
    } else { current_block = 12147880666119273379; }
    match current_block {
        12147880666119273379 => {
            if !client_keytab.is_null() {
                code =
                    krb5_kt_dup(context, client_keytab,
                                &mut (*cred).client_keytab)
            } else {
                code =
                    krb5_kt_client_default(context,
                                           &mut (*cred).client_keytab);
                if code != 0 {
                    /* Treat resolution failure similarly to a client keytab which
             * resolves but doesn't exist or has no content. */
                    if (*context).trace_callback.is_some() {
                        krb5int_trace(context,
                                      b"Unable to resolve default client keytab: {kerr}\x00"
                                          as *const u8 as *const libc::c_char,
                                      code);
                    }
                    krb5_clear_error_message(context);
                    code = 0 as libc::c_int
                }
            }
            if !(code != 0) {
                if !(*cred).ccache.is_null() {
                    /* The caller specified a ccache; check what's in it. */
                    code = scan_ccache(context, cred);
                    if code as libc::c_long == -(1765328189 as libc::c_long) {
                        /* See if we can get initial creds.  If the caller didn't specify
             * a name, pick one from the client keytab. */
                        if (*cred).name.is_null() {
                            if get_name_from_client_keytab(context, cred) == 0
                               {
                                code = 0 as libc::c_int
                            }
                        } else if can_get_initial_creds(context, cred) != 0 {
                            code = 0 as libc::c_int
                        }
                    }
                    if code != 0 {
                        current_block = 15700575537292975296;
                    } else { current_block = 13678349939556791712; }
                } else if !(*cred).name.is_null() {
                    /* The caller specified a name but not a ccache; pick a cache. */
                    code = get_cache_for_name(context, cred);
                    if code != 0 {
                        current_block = 15700575537292975296;
                    } else { current_block = 13678349939556791712; }
                } else { current_block = 13678349939556791712; }
                match current_block {
                    15700575537292975296 => { }
                    _ =>
                    /* If we haven't picked a name, make sure we have or can get any creds,
     * unless we're using Leash and might be able to get them interactively. */
                    {
                        if (*cred).name.is_null() &&
                               can_get_initial_creds(context, cred) == 0 {
                            code = krb5_cccol_have_content(context);
                            if code != 0 {
                                current_block = 15700575537292975296;
                            } else { current_block = 2290177392965769716; }
                        } else { current_block = 2290177392965769716; }
                        match current_block {
                            15700575537292975296 => { }
                            _ => {
                                code = maybe_get_initial_cred(context, cred);
                                if !(code != 0) {
                                    *minor_status =
                                        0 as libc::c_int as OM_uint32;
                                    return 0 as libc::c_int as OM_uint32
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    *minor_status = code as OM_uint32;
    return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[c2rust::src_loc = "748:1"]
unsafe extern "C" fn acquire_cred_context(mut context: krb5_context,
                                          mut minor_status: *mut OM_uint32,
                                          mut desired_name: gss_name_t,
                                          mut password: gss_buffer_t,
                                          mut time_req: OM_uint32,
                                          mut cred_usage: gss_cred_usage_t,
                                          mut ccache: krb5_ccache,
                                          mut client_keytab: krb5_keytab,
                                          mut keytab: krb5_keytab,
                                          mut rcname: *const libc::c_char,
                                          mut iakerb: krb5_boolean,
                                          mut output_cred_handle:
                                              *mut gss_cred_id_t,
                                          mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut cred: krb5_gss_cred_id_t = 0 as krb5_gss_cred_id_t;
    let mut name: krb5_gss_name_t = desired_name as krb5_gss_name_t;
    let mut ret: OM_uint32 = 0;
    let mut code: krb5_error_code = 0 as libc::c_int;
    /* make sure all outputs are valid */
    *output_cred_handle = 0 as gss_cred_id_t;
    if !time_rec.is_null() { *time_rec = 0 as libc::c_int as OM_uint32 }
    /* create the gss cred structure */
    cred =
        k5alloc(::std::mem::size_of::<krb5_gss_cred_id_rec>() as
                    libc::c_ulong, &mut code) as krb5_gss_cred_id_t;
    if cred.is_null() {
        current_block = 6108249593228632422;
    } else {
        (*cred).usage = cred_usage;
        (*cred).name = 0 as krb5_gss_name_t;
        (*cred).impersonator = 0 as krb5_principal;
        (*cred).set_iakerb_mech(iakerb);
        (*cred).set_default_identity((name ==
                                          0 as *mut libc::c_void as
                                              krb5_gss_name_t) as libc::c_int
                                         as libc::c_uint);
        (*cred).keytab = 0 as krb5_keytab;
        /* LEAN_CLIENT */
        (*cred).set_destroy_ccache(0 as libc::c_int as libc::c_uint);
        (*cred).set_suppress_ci_flags(0 as libc::c_int as libc::c_uint);
        (*cred).ccache = 0 as krb5_ccache;
        code = k5_mutex_init(&mut (*cred).lock);
        if code != 0 {
            current_block = 6108249593228632422;
        } else {
            match cred_usage {
                1 | 2 | 0 => {
                    if !name.is_null() {
                        code =
                            kg_duplicate_name(context, name,
                                              &mut (*cred).name);
                        if code != 0 {
                            current_block = 6108249593228632422;
                        } else { current_block = 13472856163611868459; }
                    } else { current_block = 13472856163611868459; }
                    match current_block {
                        6108249593228632422 => { }
                        _ =>
                        /*
     * If requested, acquire credentials for accepting. This will fill
     * in cred->name if desired_princ is specified.
     */
                        {
                            if cred_usage == 2 as libc::c_int ||
                                   cred_usage == 0 as libc::c_int {
                                ret =
                                    acquire_accept_cred(context, minor_status,
                                                        keytab, rcname, cred);
                                if ret != 0 as libc::c_int as libc::c_uint {
                                    current_block = 11183415134345190833;
                                } else {
                                    current_block = 15345278821338558188;
                                }
                            } else { current_block = 15345278821338558188; }
                            match current_block {
                                11183415134345190833 => { }
                                _ =>
                                /* LEAN_CLIENT */
                                /*
     * If requested, acquire credentials for initiation. This will fill
     * in cred->name if it wasn't set above.
     */
                                {
                                    if cred_usage == 1 as libc::c_int ||
                                           cred_usage == 0 as libc::c_int {
                                        ret =
                                            acquire_init_cred(context,
                                                              minor_status,
                                                              ccache,
                                                              password,
                                                              client_keytab,
                                                              cred);
                                        if ret !=
                                               0 as libc::c_int as
                                                   libc::c_uint {
                                            current_block =
                                                11183415134345190833;
                                        } else {
                                            current_block =
                                                15897653523371991391;
                                        }
                                    } else {
                                        current_block = 15897653523371991391;
                                    }
                                    match current_block {
                                        11183415134345190833 => { }
                                        _ => {
                                            if (*cred).default_identity() as
                                                   libc::c_int != 0 ||
                                                   !(*cred).name.is_null() {
                                            } else {
                                                __assert_fail(b"cred->default_identity || cred->name != NULL\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              b"acquire_cred.c\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char,
                                                              828 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint,
                                                              (*::std::mem::transmute::<&[u8; 210],
                                                                                        &[libc::c_char; 210]>(b"OM_uint32 acquire_cred_context(krb5_context, OM_uint32 *, gss_name_t, gss_buffer_t, OM_uint32, gss_cred_usage_t, krb5_ccache, krb5_keytab, krb5_keytab, const char *, krb5_boolean, gss_cred_id_t *, OM_uint32 *)\x00")).as_ptr());
                                            }
                                            /* ** at this point, the cred structure has been completely created */
                                            if cred_usage == 2 as libc::c_int
                                               {
                                                if !time_rec.is_null() {
                                                    *time_rec =
                                                        0xffffffff as
                                                            libc::c_ulong as
                                                            OM_uint32
                                                }
                                                current_block =
                                                    1622411330066726685;
                                            } else {
                                                let mut now: krb5_timestamp =
                                                    0;
                                                code =
                                                    krb5_timeofday(context,
                                                                   &mut now);
                                                if code != 0 as libc::c_int {
                                                    current_block =
                                                        6108249593228632422;
                                                } else if !time_rec.is_null()
                                                 {
                                                    /* Resolve cred now to determine the expiration time. */
                                                    ret =
                                                        kg_cred_resolve(minor_status,
                                                                        context,
                                                                        cred
                                                                            as
                                                                            gss_cred_id_t,
                                                                        0 as
                                                                            gss_name_t);
                                                    if ret &
                                                           ((0o377 as
                                                                 libc::c_ulong
                                                                 as OM_uint32)
                                                                <<
                                                                24 as
                                                                    libc::c_int
                                                                |
                                                                (0o377 as
                                                                     libc::c_ulong
                                                                     as
                                                                     OM_uint32)
                                                                    <<
                                                                    16 as
                                                                        libc::c_int)
                                                           != 0 {
                                                        current_block =
                                                            11183415134345190833;
                                                    } else {
                                                        *time_rec =
                                                            if ts_after((*cred).expire,
                                                                        now)
                                                                   != 0 {
                                                                ts_delta((*cred).expire,
                                                                         now)
                                                            } else {
                                                                0 as
                                                                    libc::c_int
                                                            } as OM_uint32;
                                                        k5_mutex_unlock(&mut (*cred).lock);
                                                        current_block =
                                                            1622411330066726685;
                                                    }
                                                } else {
                                                    current_block =
                                                        1622411330066726685;
                                                }
                                            }
                                            match current_block {
                                                11183415134345190833 => { }
                                                6108249593228632422 => { }
                                                _ => {
                                                    *minor_status =
                                                        0 as libc::c_int as
                                                            OM_uint32;
                                                    *output_cred_handle =
                                                        cred as gss_cred_id_t;
                                                    return 0 as libc::c_int as
                                                               OM_uint32
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    ret =
                        (13 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int;
                    *minor_status =
                        -(2045022969 as libc::c_long) as OM_uint32;
                    current_block = 11183415134345190833;
                }
            }
        }
    }
    match current_block {
        6108249593228632422 => {
            *minor_status = code as OM_uint32;
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        _ => { }
    }
    if !cred.is_null() {
        if !(*cred).ccache.is_null() {
            if (*cred).destroy_ccache() != 0 {
                krb5_cc_destroy(context, (*cred).ccache);
            } else { krb5_cc_close(context, (*cred).ccache); }
        }
        if !(*cred).client_keytab.is_null() {
            krb5_kt_close(context, (*cred).client_keytab);
        }
        if !(*cred).keytab.is_null() {
            krb5_kt_close(context, (*cred).keytab);
        }
        /* LEAN_CLIENT */
        if !(*cred).rcache.is_null() { k5_rc_close(context, (*cred).rcache); }
        if !(*cred).name.is_null() {
            kg_release_name(context, &mut (*cred).name);
        }
        krb5_free_principal(context, (*cred).impersonator);
        zapfreestr((*cred).password as *mut libc::c_void);
        k5_os_mutex_destroy(&mut (*cred).lock);
        free(cred as *mut libc::c_void);
    }
    krb5_gss_save_error_info(*minor_status, context);
    return ret;
}
#[c2rust::src_loc = "890:1"]
unsafe extern "C" fn acquire_cred(mut minor_status: *mut OM_uint32,
                                  mut desired_name: gss_name_t,
                                  mut password: gss_buffer_t,
                                  mut time_req: OM_uint32,
                                  mut cred_usage: gss_cred_usage_t,
                                  mut ccache: krb5_ccache,
                                  mut keytab: krb5_keytab,
                                  mut iakerb: krb5_boolean,
                                  mut output_cred_handle: *mut gss_cred_id_t,
                                  mut time_rec: *mut OM_uint32) -> OM_uint32 {
    let mut context: krb5_context = 0 as krb5_context;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut ret: OM_uint32 = 0;
    code = gss_krb5int_initialize_library() as krb5_error_code;
    if code != 0 {
        *minor_status = code as OM_uint32;
        ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        code = krb5_gss_init_context(&mut context);
        if code != 0 {
            *minor_status = code as OM_uint32;
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            ret =
                acquire_cred_context(context, minor_status, desired_name,
                                     password, time_req, cred_usage, ccache,
                                     0 as krb5_keytab, keytab,
                                     0 as *const libc::c_char, iakerb,
                                     output_cred_handle, time_rec)
        }
    }
    krb5_free_context(context);
    return ret;
}
/*
 * Resolve the name and ccache for an initiator credential if it has not yet
 * been done.  If specified, use the target name to pick an appropriate ccache
 * within the collection.  Validates cred_handle and leaves it locked on
 * success.
 */
#[no_mangle]
#[c2rust::src_loc = "930:1"]
pub unsafe extern "C" fn kg_cred_resolve(mut minor_status: *mut OM_uint32,
                                         mut context: krb5_context,
                                         mut cred_handle: gss_cred_id_t,
                                         mut target_name: gss_name_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut maj: OM_uint32 = 0;
    let mut code: krb5_error_code = 0;
    let mut cred: krb5_gss_cred_id_t = cred_handle as krb5_gss_cred_id_t;
    let mut tname: krb5_gss_name_t = target_name as krb5_gss_name_t;
    let mut client_princ: krb5_principal = 0 as *mut krb5_principal_data;
    *minor_status = 0 as libc::c_int as OM_uint32;
    maj = krb5_gss_validate_cred_1(minor_status, cred_handle, context);
    if maj != 0 as libc::c_int as libc::c_uint { return maj }
    if (*cred).usage == 2 as libc::c_int || !(*cred).name.is_null() {
        return 0 as libc::c_int as OM_uint32
    }
    /* acquire_init_cred should have set both name and ccache, or neither. */
    if (*cred).ccache.is_null() {
    } else {
        __assert_fail(b"cred->ccache == NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const libc::c_char,
                      950 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 80],
                                                &[libc::c_char; 80]>(b"OM_uint32 kg_cred_resolve(OM_uint32 *, krb5_context, gss_cred_id_t, gss_name_t)\x00")).as_ptr());
    }
    if !tname.is_null() {
        /* Use the target name to select an existing ccache or a principal. */
        code =
            krb5_cc_select(context, (*tname).princ, &mut (*cred).ccache,
                           &mut client_princ);
        if code != 0 && code as libc::c_long != -(1765328243 as libc::c_long)
           {
            current_block = 721532972098336643;
        } else {
            if !client_princ.is_null() {
                code =
                    kg_init_name(context, client_princ,
                                 0 as *mut libc::c_char,
                                 0 as *mut libc::c_char,
                                 0 as krb5_authdata_context,
                                 0x1 as libc::c_int, &mut (*cred).name);
                if code != 0 {
                    krb5_free_principal(context, client_princ);
                    current_block = 721532972098336643;
                } else { current_block = 15976848397966268834; }
            } else { current_block = 15976848397966268834; }
            match current_block {
                721532972098336643 => { }
                _ => {
                    if !(*cred).ccache.is_null() {
                        code = scan_ccache(context, cred);
                        if code != 0 {
                            current_block = 721532972098336643;
                        } else { current_block = 4495394744059808450; }
                    } else { current_block = 4495394744059808450; }
                }
            }
        }
    } else { current_block = 4495394744059808450; }
    match current_block {
        4495394744059808450 =>
        /* If we still haven't picked a client principal, try using an existing
     * default ccache.  (On Windows, this may acquire initial creds.) */
        {
            if (*cred).name.is_null() {
                code = krb5int_cc_default(context, &mut (*cred).ccache);
                if code != 0 {
                    current_block = 721532972098336643;
                } else {
                    code = scan_ccache(context, cred);
                    if code as libc::c_long == -(1765328189 as libc::c_long) {
                        /* Default ccache doesn't exist; fall through to client keytab. */
                        krb5_cc_close(context, (*cred).ccache);
                        (*cred).ccache = 0 as krb5_ccache;
                        current_block = 11636175345244025579;
                    } else if code != 0 {
                        current_block = 721532972098336643;
                    } else { current_block = 11636175345244025579; }
                }
            } else { current_block = 11636175345244025579; }
            match current_block {
                721532972098336643 => { }
                _ =>
                /* If that didn't work, try getting a name from the client keytab. */
                {
                    if (*cred).name.is_null() {
                        code = get_name_from_client_keytab(context, cred);
                        if code != 0 {
                            code =
                                39756044 as libc::c_long as krb5_error_code;
                            current_block = 721532972098336643;
                        } else { current_block = 2604890879466389055; }
                    } else { current_block = 2604890879466389055; }
                    match current_block {
                        721532972098336643 => { }
                        _ => {
                            if !(*cred).name.is_null() &&
                                   (*cred).ccache.is_null() {
                                /* Pick a cache for the name we chose (from krb5_cc_select or from the
         * client keytab). */
                                code = get_cache_for_name(context, cred);
                                if code != 0 {
                                    current_block = 721532972098336643;
                                } else {
                                    current_block = 9007357115414505193;
                                }
                            } else { current_block = 9007357115414505193; }
                            match current_block {
                                721532972098336643 => { }
                                _ => {
                                    /* Resolve name to ccache and possibly get initial credentials. */
                                    code =
                                        maybe_get_initial_cred(context, cred);
                                    if !(code != 0) {
                                        return 0 as libc::c_int as OM_uint32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    k5_mutex_unlock(&mut (*cred).lock);
    krb5_gss_save_error_info(code as OM_uint32, context);
    *minor_status = code as OM_uint32;
    return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1020:1"]
pub unsafe extern "C" fn gss_krb5int_set_cred_rcache(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut cred_handle:
                                                         *mut gss_cred_id_t,
                                                     desired_oid: gss_OID,
                                                     value: gss_buffer_t)
 -> OM_uint32 {
    let mut cred: krb5_gss_cred_id_t = 0 as *mut _krb5_gss_cred_id_rec;
    let mut code: krb5_error_code = 0;
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut rcache: krb5_rcache = 0 as *mut krb5_rc_st;
    if (*value).length ==
           ::std::mem::size_of::<krb5_rcache>() as libc::c_ulong {
    } else {
        __assert_fail(b"value->length == sizeof(rcache)\x00" as *const u8 as
                          *const libc::c_char,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const libc::c_char,
                      1031 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 103],
                                                &[libc::c_char; 103]>(b"OM_uint32 gss_krb5int_set_cred_rcache(OM_uint32 *, gss_cred_id_t *, const gss_OID, const gss_buffer_t)\x00")).as_ptr());
    }
    if (*value).length !=
           ::std::mem::size_of::<krb5_rcache>() as libc::c_ulong {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    rcache = (*value).value as krb5_rcache;
    cred = *cred_handle as krb5_gss_cred_id_t;
    code = krb5_gss_init_context(&mut context);
    if code != 0 {
        *minor_status = code as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !(*cred).rcache.is_null() { k5_rc_close(context, (*cred).rcache); }
    (*cred).rcache = rcache;
    krb5_free_context(context);
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
/*
 * krb5 and IAKERB mech API functions follow.  The mechglue always passes null
 * desired_mechs and actual_mechs, so we ignore those parameters.
 */
#[no_mangle]
#[c2rust::src_loc = "1061:1"]
pub unsafe extern "C" fn krb5_gss_acquire_cred(mut minor_status:
                                                   *mut OM_uint32,
                                               mut desired_name: gss_name_t,
                                               mut time_req: OM_uint32,
                                               mut desired_mechs: gss_OID_set,
                                               mut cred_usage:
                                                   gss_cred_usage_t,
                                               mut output_cred_handle:
                                                   *mut gss_cred_id_t,
                                               mut actual_mechs:
                                                   *mut gss_OID_set,
                                               mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    return acquire_cred(minor_status, desired_name, 0 as gss_buffer_t,
                        time_req, cred_usage, 0 as krb5_ccache,
                        0 as krb5_keytab, 0 as libc::c_int as krb5_boolean,
                        output_cred_handle, time_rec);
}
#[no_mangle]
#[c2rust::src_loc = "1072:1"]
pub unsafe extern "C" fn iakerb_gss_acquire_cred(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut desired_name: gss_name_t,
                                                 mut time_req: OM_uint32,
                                                 mut desired_mechs:
                                                     gss_OID_set,
                                                 mut cred_usage:
                                                     gss_cred_usage_t,
                                                 mut output_cred_handle:
                                                     *mut gss_cred_id_t,
                                                 mut actual_mechs:
                                                     *mut gss_OID_set,
                                                 mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    return acquire_cred(minor_status, desired_name, 0 as gss_buffer_t,
                        time_req, cred_usage, 0 as krb5_ccache,
                        0 as krb5_keytab, 1 as libc::c_int as krb5_boolean,
                        output_cred_handle, time_rec);
}
#[no_mangle]
#[c2rust::src_loc = "1083:1"]
pub unsafe extern "C" fn krb5_gss_acquire_cred_with_password(mut minor_status:
                                                                 *mut OM_uint32,
                                                             desired_name:
                                                                 gss_name_t,
                                                             password:
                                                                 gss_buffer_t,
                                                             mut time_req:
                                                                 OM_uint32,
                                                             desired_mechs:
                                                                 gss_OID_set,
                                                             mut cred_usage:
                                                                 libc::c_int,
                                                             mut output_cred_handle:
                                                                 *mut gss_cred_id_t,
                                                             mut actual_mechs:
                                                                 *mut gss_OID_set,
                                                             mut time_rec:
                                                                 *mut OM_uint32)
 -> OM_uint32 {
    return acquire_cred(minor_status, desired_name, password, time_req,
                        cred_usage, 0 as krb5_ccache, 0 as krb5_keytab,
                        0 as libc::c_int as krb5_boolean, output_cred_handle,
                        time_rec);
}
#[no_mangle]
#[c2rust::src_loc = "1099:1"]
pub unsafe extern "C" fn iakerb_gss_acquire_cred_with_password(mut minor_status:
                                                                   *mut OM_uint32,
                                                               desired_name:
                                                                   gss_name_t,
                                                               password:
                                                                   gss_buffer_t,
                                                               mut time_req:
                                                                   OM_uint32,
                                                               desired_mechs:
                                                                   gss_OID_set,
                                                               mut cred_usage:
                                                                   libc::c_int,
                                                               mut output_cred_handle:
                                                                   *mut gss_cred_id_t,
                                                               mut actual_mechs:
                                                                   *mut gss_OID_set,
                                                               mut time_rec:
                                                                   *mut OM_uint32)
 -> OM_uint32 {
    return acquire_cred(minor_status, desired_name, password, time_req,
                        cred_usage, 0 as krb5_ccache, 0 as krb5_keytab,
                        1 as libc::c_int as krb5_boolean, output_cred_handle,
                        time_rec);
}
#[no_mangle]
#[c2rust::src_loc = "1115:1"]
pub unsafe extern "C" fn gss_krb5int_import_cred(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut cred_handle:
                                                     *mut gss_cred_id_t,
                                                 desired_oid: gss_OID,
                                                 value: gss_buffer_t)
 -> OM_uint32 {
    let mut req: *mut krb5_gss_import_cred_req =
        0 as *mut krb5_gss_import_cred_req;
    let mut name: krb5_gss_name_rec =
        krb5_gss_name_rec{princ: 0 as *mut krb5_principal_data,
                          service: 0 as *mut libc::c_char,
                          host: 0 as *mut libc::c_char,
                          lock:
                              pthread_mutex_t{__data:
                                                  __pthread_mutex_s{__lock: 0,
                                                                    __count:
                                                                        0,
                                                                    __owner:
                                                                        0,
                                                                    __nusers:
                                                                        0,
                                                                    __kind: 0,
                                                                    __spins:
                                                                        0,
                                                                    __elision:
                                                                        0,
                                                                    __list:
                                                                        __pthread_list_t{__prev:
                                                                                             0
                                                                                                 as
                                                                                                 *mut __pthread_internal_list,
                                                                                         __next:
                                                                                             0
                                                                                                 as
                                                                                                 *mut __pthread_internal_list,},},},
                          ad_context: 0 as *mut _krb5_authdata_context,};
    let mut time_rec: OM_uint32 = 0;
    let mut code: krb5_error_code = 0;
    let mut usage: gss_cred_usage_t = 0;
    let mut desired_name: gss_name_t = 0 as gss_name_t;
    if (*value).length ==
           ::std::mem::size_of::<krb5_gss_import_cred_req>() as libc::c_ulong
       {
    } else {
        __assert_fail(b"value->length == sizeof(*req)\x00" as *const u8 as
                          *const libc::c_char,
                      b"acquire_cred.c\x00" as *const u8 as
                          *const libc::c_char,
                      1128 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 99],
                                                &[libc::c_char; 99]>(b"OM_uint32 gss_krb5int_import_cred(OM_uint32 *, gss_cred_id_t *, const gss_OID, const gss_buffer_t)\x00")).as_ptr());
    }
    if (*value).length !=
           ::std::mem::size_of::<krb5_gss_import_cred_req>() as libc::c_ulong
       {
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    req = (*value).value as *mut krb5_gss_import_cred_req;
    if !(*req).id.is_null() {
        usage =
            if !(*req).keytab.is_null() {
                0 as libc::c_int
            } else { 1 as libc::c_int }
    } else if !(*req).keytab.is_null() {
        usage = 2 as libc::c_int
    } else {
        *minor_status = 22 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    if !(*req).keytab_principal.is_null() {
        memset(&mut name as *mut krb5_gss_name_rec as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<krb5_gss_name_rec>() as libc::c_ulong);
        code = k5_mutex_init(&mut name.lock);
        if code != 0 as libc::c_int {
            *minor_status = code as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        name.princ = (*req).keytab_principal;
        desired_name = &mut name as *mut krb5_gss_name_rec as gss_name_t
    }
    code =
        acquire_cred(minor_status, desired_name, 0 as gss_buffer_t,
                     0xffffffff as libc::c_ulong as OM_uint32, usage,
                     (*req).id, (*req).keytab,
                     0 as libc::c_int as krb5_boolean, cred_handle,
                     &mut time_rec) as krb5_error_code;
    if !(*req).keytab_principal.is_null() {
        k5_os_mutex_destroy(&mut name.lock);
    }
    return code as OM_uint32;
}
/* * CFX flags **/
/* These are to be stored in little-endian order, i.e., des-mac is
   stored as 02 00.  */
/* SGN_ALG_DES_MAC_MD5           = 0x0000, */
    /* SGN_ALG_MD2_5                 = 0x0001, */
    /* SGN_ALG_DES_MAC               = 0x0002, */
    /* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
/* microsoft w2k;  */
/* SEAL_ALG_DES             = 0x0000, */
    /* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
/* microsoft w2k;  */
/* for 3DES */
/* for draft-ietf-krb-wg-gssapi-cfx-01 */
/* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
/* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
/* * internal types **/
/* immutable */
/* immutable */
/* immutable */
/* protects ad_context only for now */
/* protect against simultaneous accesses */
/* name/type of credential */
/* keytab (accept) data */
/* ccache (init) data */
/* limit negotiated enctypes to this list */
/* nonzero if initiating, zero if accepting */
/* XXX tested but never actually set */
/* One of two potential keys to use with RFC 4121
                      * packets; this key must always be set. */
/* RFC 1964 encryption key; seq xored with a constant
                   * for DES, seq for other RFC 1964 enctypes  */
/* RFC 1964 sequencing key */
/* XXX these used to be signed.  the old spec is inspecific, and
       the new spec specifies unsigned.  I don't believe that the change
       affects the wire encoding. */
/* Protocol spec revision for sending packets
       0 => RFC 1964 with 3DES and RC4 enhancements
       1 => RFC 4121
       No others defined so far.  It is always permitted to receive
       tokens in RFC 4121 format.  If enc is non-null, receiving RFC
       1964 tokens is permitted.*/
/* for "main" subkey */
/* CFX only */
/* did we get rcache from creds? */
/* LEAN_CLIENT */
/* * helper functions **/
/* Encrypt length bytes at ptr in place, with the given key and usage.  If
 * iv is not NULL, use it as the cipher state. */
/* AEAD */
/* for conf len */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* exts */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/*exts */
/* LEAN_CLIENT */
/* minor_status */
/* context_handle */
/* desired_object */
/* data_set */
/* minor_status */
/* context_handle */
/* desired_object */
/* value */
/* minor_status */
/* context_handle */
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type */
/* message_context */
/* status_string */
/* minor_status */
/* mech_set */
/* minor_status */
/* name1 */
/* name2 */
/* name_equal */
/* minor_status */
/* input_name */
/* output_name_buffer */
/* output_name_type */
/* minor_status */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* minor_status */
/* context_handle */
/* initiator_name */
/* acceptor_name */
/* lifetime_rec */
/* mech_type */
/* ret_flags */
/* locally_initiated */
/* open */
/* New V2 entry points */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* LEAN_CLIENT */
/* minor_status */
/* oid */
/* minor_status */
/* oid */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* input_name */
/* mech_type */
/* output_name */
/* minor_status */
/* input_name */
/* exported_name */
/* minor_status */
/* input_name */
/* dest_name */
/* minor_status */
/* cred */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* context */
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* _GSS_STATIC_LINK */
/* For error message handling.  */
/* Returns a shared string, not a private copy!  */
/* Prefix concatenated with Kerberos encryption type */
/* IAKERB */
/*
 * Transfer contents of a krb5_data to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
/* Credential store extensions */
#[no_mangle]
#[c2rust::src_loc = "1163:1"]
pub unsafe extern "C" fn krb5_gss_acquire_cred_from(mut minor_status:
                                                        *mut OM_uint32,
                                                    desired_name: gss_name_t,
                                                    mut time_req: OM_uint32,
                                                    desired_mechs:
                                                        gss_OID_set,
                                                    mut cred_usage:
                                                        gss_cred_usage_t,
                                                    mut cred_store:
                                                        gss_const_key_value_set_t,
                                                    mut output_cred_handle:
                                                        *mut gss_cred_id_t,
                                                    mut actual_mechs:
                                                        *mut gss_OID_set,
                                                    mut time_rec:
                                                        *mut OM_uint32)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut context: krb5_context = 0 as krb5_context;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut client_keytab: krb5_keytab = 0 as krb5_keytab;
    let mut keytab: krb5_keytab = 0 as krb5_keytab;
    let mut ccache: krb5_ccache = 0 as krb5_ccache;
    let mut rcname: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: OM_uint32 = 0;
    code = gss_krb5int_initialize_library() as krb5_error_code;
    if code != 0 {
        *minor_status = code as OM_uint32;
        ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    } else {
        code = krb5_gss_init_context(&mut context);
        if code != 0 {
            *minor_status = code as OM_uint32;
            ret = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            ret =
                kg_value_from_cred_store(cred_store,
                                         b"ccache\x00" as *const u8 as
                                             *const libc::c_char, &mut value);
            if !(ret &
                     ((0o377 as libc::c_ulong as OM_uint32) <<
                          24 as libc::c_int |
                          (0o377 as libc::c_ulong as OM_uint32) <<
                              16 as libc::c_int) != 0) {
                if !value.is_null() {
                    code = krb5_cc_resolve(context, value, &mut ccache);
                    if code != 0 as libc::c_int {
                        *minor_status = code as OM_uint32;
                        ret =
                            (13 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int;
                        current_block = 11412884123443887248;
                    } else { current_block = 4808432441040389987; }
                } else { current_block = 4808432441040389987; }
                match current_block {
                    11412884123443887248 => { }
                    _ => {
                        ret =
                            kg_value_from_cred_store(cred_store,
                                                     b"client_keytab\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     &mut value);
                        if !(ret &
                                 ((0o377 as libc::c_ulong as OM_uint32) <<
                                      24 as libc::c_int |
                                      (0o377 as libc::c_ulong as OM_uint32) <<
                                          16 as libc::c_int) != 0) {
                            if !value.is_null() {
                                code =
                                    krb5_kt_resolve(context, value,
                                                    &mut client_keytab);
                                if code != 0 as libc::c_int {
                                    *minor_status = code as OM_uint32;
                                    ret =
                                        (13 as libc::c_ulong as OM_uint32) <<
                                            16 as libc::c_int;
                                    current_block = 11412884123443887248;
                                } else {
                                    current_block = 15345278821338558188;
                                }
                            } else { current_block = 15345278821338558188; }
                            match current_block {
                                11412884123443887248 => { }
                                _ => {
                                    ret =
                                        kg_value_from_cred_store(cred_store,
                                                                 b"keytab\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 &mut value);
                                    if !(ret &
                                             ((0o377 as libc::c_ulong as
                                                   OM_uint32) <<
                                                  24 as libc::c_int |
                                                  (0o377 as libc::c_ulong as
                                                       OM_uint32) <<
                                                      16 as libc::c_int) != 0)
                                       {
                                        if !value.is_null() {
                                            code =
                                                krb5_kt_resolve(context,
                                                                value,
                                                                &mut keytab);
                                            if code != 0 as libc::c_int {
                                                *minor_status =
                                                    code as OM_uint32;
                                                ret =
                                                    (13 as libc::c_ulong as
                                                         OM_uint32) <<
                                                        16 as libc::c_int;
                                                current_block =
                                                    11412884123443887248;
                                            } else {
                                                current_block =
                                                    17184638872671510253;
                                            }
                                        } else {
                                            current_block =
                                                17184638872671510253;
                                        }
                                        match current_block {
                                            11412884123443887248 => { }
                                            _ => {
                                                ret =
                                                    kg_value_from_cred_store(cred_store,
                                                                             b"rcache\x00"
                                                                                 as
                                                                                 *const u8
                                                                                 as
                                                                                 *const libc::c_char,
                                                                             &mut rcname);
                                                if !(ret &
                                                         ((0o377 as
                                                               libc::c_ulong
                                                               as OM_uint32)
                                                              <<
                                                              24 as
                                                                  libc::c_int
                                                              |
                                                              (0o377 as
                                                                   libc::c_ulong
                                                                   as
                                                                   OM_uint32)
                                                                  <<
                                                                  16 as
                                                                      libc::c_int)
                                                         != 0) {
                                                    ret =
                                                        acquire_cred_context(context,
                                                                             minor_status,
                                                                             desired_name,
                                                                             0
                                                                                 as
                                                                                 gss_buffer_t,
                                                                             time_req,
                                                                             cred_usage,
                                                                             ccache,
                                                                             client_keytab,
                                                                             keytab,
                                                                             rcname,
                                                                             0
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 krb5_boolean,
                                                                             output_cred_handle,
                                                                             time_rec)
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
    if !ccache.is_null() { krb5_cc_close(context, ccache); }
    if !client_keytab.is_null() { krb5_kt_close(context, client_keytab); }
    if !keytab.is_null() { krb5_kt_close(context, keytab); }
    krb5_free_context(context);
    return ret;
}
