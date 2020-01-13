use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:66"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:66"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:66"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:66"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:66"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:66"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:66"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    use super::pthreadtypes_h::pthread_mutex_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:66"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "561:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "644:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint16_t,
    }
    /* Do it in macro form so we get the file/line of the invocation if
   the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
   multiple active threads mucking around in our library at this
   point.  So ignore the once_t object and just look at the flag.

   XXX Could we have problems with memory coherence between processors
   if we don't invoke mutex/once routines?  Probably not, the
   application code should already be coordinating things such that
   the library code is not in use by this point, and memory
   synchronization will be needed there.  */
    /*
 * In this case, we just don't care about finalization.  The code will still
 * define the function, but we won't do anything with it.
 */
    /* Read and write integer values as (unaligned) octet strings in
   specific byte orders.  Add per-platform optimizations as
   needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
   it.  If both are defined, then BYTE_ORDER should be defined and
   match one of them.  Try those symbols, then try again with an
   underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

   GCC's packed structures can be written to with any alignment; the
   compiler will use byte operations, unaligned-word operations, or
   normal memory ops as appropriate for the architecture.

   This assumes the availability of uint##_t types, which should work
   on most of our platforms except Windows, where we're not using
   GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
   with the indicated numbers of bits.

   Linux: byteswap.h, bswap_16 etc.
   Solaris 10: none
   macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
   NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
   source, so casts *from* void* are required.  */
    #[inline]
    #[c2rust::src_loc = "554:1"]
    pub unsafe extern "C" fn store_16_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_16(val as __uint16_t);
    }
    #[inline]
    #[c2rust::src_loc = "639:1"]
    pub unsafe extern "C" fn store_16_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = val as uint16_t;
    }
    use super::stdint_uintn_h::uint16_t;
    use super::byteswap_h::__bswap_16;
    use super::types_h::__uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:66"]
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
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
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
    #[c2rust::src_loc = "2122:16"]
    pub struct _krb5_ap_req {
        pub magic: krb5_magic,
        pub ap_options: krb5_flags,
        pub ticket: *mut krb5_ticket,
        pub authenticator: krb5_enc_data,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Authentication header. */
    #[c2rust::src_loc = "2122:1"]
    pub type krb5_ap_req = _krb5_ap_req;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, krb5_key_st};
    extern "C" {
        /* *< Requested options */
        /* *< Ticket */
        /* *< Encrypted authenticator */
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
        /* *
 * Create a krb5_key from the enctype and key data in a keyblock.
 *
 * @param [in]  context      Library context
 * @param [in]  key_data     Keyblock
 * @param [out] out          Opaque key
 *
 * The reference count on a key @a out is set to 1.
 * Use krb5_k_free_key() to free @a out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
 */
        #[no_mangle]
        #[c2rust::src_loc = "1285:1"]
        pub fn krb5_k_create_key(context: krb5_context,
                                 key_data: *const krb5_keyblock,
                                 out: *mut krb5_key) -> krb5_error_code;
        /* * Decrement the reference count on a key and free it if it hits zero. */
        #[no_mangle]
        #[c2rust::src_loc = "1294:1"]
        pub fn krb5_k_free_key(context: krb5_context, key: krb5_key);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:66"]
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
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_keyblock, krb5_data, krb5_key,
                        krb5_authdatatype};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:66"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:66"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:66"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:66"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:67"]
pub mod gssapi_h {
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    #[c2rust::src_loc = "134:1"]
    pub type gss_qop_t = OM_uint32;
    use super::stdint_uintn_h::uint32_t;
    use super::mglueP_h::{gss_name_struct, gss_cred_id_struct};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "474:1"]
        pub fn gss_delete_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: gss_buffer_t, _: gss_buffer_t,
                              _: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "520:1"]
        pub fn gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t, _: gss_buffer_t,
                          _: gss_buffer_t, _: *mut libc::c_int,
                          _: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/mechglue/mglueP.h:68"]
pub mod mglueP_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:16"]
    pub struct gss_name_struct {
        pub loopback: *mut gss_name_struct,
        pub name_type: gss_OID,
        pub external_name: gss_buffer_t,
        pub mech_type: gss_OID,
        pub mech_name: gss_name_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:16"]
    pub struct gss_cred_id_struct {
        pub loopback: *mut gss_cred_id_struct,
        pub count: libc::c_int,
        pub mechs_array: gss_OID,
        pub cred_array: *mut gss_cred_id_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:16"]
    pub struct gss_union_ctx_id_struct {
        pub loopback: *mut gss_union_ctx_id_struct,
        pub mech_type: gss_OID,
        pub internal_ctx_id: gss_ctx_id_t,
    }
    #[c2rust::src_loc = "26:1"]
    pub type gss_union_ctx_id_t = *mut gss_union_ctx_id_struct;
    use super::gssapi_h::{gss_OID, gss_buffer_t, gss_name_t, gss_cred_id_t,
                          gss_ctx_id_t};
    /* _GSS_MECHGLUEP_H */
    /* Use this to map an errno value or com_err error code being
   generated within the mechglue code (e.g., by calling generic oid
   ops).  Any errno or com_err values produced by mech operations
   should be processed with map_error.  This means they'll be stored
   separately even if the mech uses com_err, because we can't assume
   that it will use com_err.  */
    /* Use this to map an error code that was returned from a mech
   operation; the mech will be asked to produce the associated error
   messages.

   Remember that if the minor status code cannot be returned to the
   caller (e.g., if it's stuffed in an automatic variable and then
   ignored), then we don't care about producing a mapping.  */
    /* qop_state */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:67"]
pub mod gssapi_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    use super::gssapi_h::{OM_uint32, gss_buffer_desc, gss_ctx_id_struct,
                          gss_ctx_id_t, gss_qop_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "328:1"]
        pub fn gss_unwrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                              _: *mut libc::c_int, _: *mut gss_qop_t,
                              _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "383:1"]
        pub fn gss_verify_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                  _: *mut gss_qop_t,
                                  _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:68"]
pub mod gssapiP_generic_h {
    #[c2rust::src_loc = "129:1"]
    pub type g_seqnum_state = *mut g_seqnum_state_st;
    use super::stdint_uintn_h::uint64_t;
    extern "C" {
        #[c2rust::src_loc = "129:16"]
        pub type g_seqnum_state_st;
        #[no_mangle]
        #[c2rust::src_loc = "177:1"]
        pub fn gssint_g_seqstate_init(state_out: *mut g_seqnum_state,
                                      seqnum: uint64_t,
                                      do_replay: libc::c_int,
                                      do_sequence: libc::c_int,
                                      wide: libc::c_int) -> libc::c_long;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:69"]
pub mod gssapiP_krb5_h {
    /* * CFX flags **/
    /* These are to be stored in little-endian order, i.e., des-mac is
   stored as 02 00.  */
    #[c2rust::src_loc = "122:1"]
    pub type sgn_alg = libc::c_uint;
    /* microsoft w2k;  */
    #[c2rust::src_loc = "128:5"]
    pub const SGN_ALG_HMAC_SHA1_DES3_KD: sgn_alg = 4;
    /* SGN_ALG_DES_MAC_MD5           = 0x0000, */
    /* SGN_ALG_MD2_5                 = 0x0001, */
    /* SGN_ALG_DES_MAC               = 0x0002, */
    /* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
    #[c2rust::src_loc = "127:5"]
    pub const SGN_ALG_HMAC_MD5: sgn_alg = 17;
    #[c2rust::src_loc = "130:1"]
    pub type seal_alg = libc::c_uint;
    /* microsoft w2k;  */
    #[c2rust::src_loc = "135:5"]
    pub const SEAL_ALG_DES3KD: seal_alg = 2;
    /* SEAL_ALG_DES             = 0x0000, */
    /* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
    #[c2rust::src_loc = "134:5"]
    pub const SEAL_ALG_MICROSOFT_RC4: seal_alg = 16;
    #[c2rust::src_loc = "131:5"]
    pub const SEAL_ALG_NONE: seal_alg = 65535;
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
    /* * internal types **/
    #[c2rust::src_loc = "162:1"]
    pub type krb5_gss_name_t = *mut _krb5_gss_name_rec;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "204:16"]
    pub struct _krb5_gss_ctx_id_rec {
        pub magic: krb5_magic,
        #[bitfield(name = "initiate", ty = "libc::c_uint", bits = "0..=0")]
        #[bitfield(name = "established", ty = "libc::c_uint", bits = "1..=1")]
        #[bitfield(name = "have_acceptor_subkey", ty = "libc::c_uint", bits =
                   "2..=2")]
        #[bitfield(name = "seed_init", ty = "libc::c_uint", bits = "3..=3")]
        #[bitfield(name = "terminated", ty = "libc::c_uint", bits = "4..=4")]
        pub initiate_established_have_acceptor_subkey_seed_init_terminated: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 3],
        pub gss_flags: OM_uint32,
        pub seed: [libc::c_uchar; 16],
        pub here: krb5_gss_name_t,
        pub there: krb5_gss_name_t,
        pub subkey: krb5_key,
        pub signalg: libc::c_int,
        pub cksum_size: size_t,
        pub sealalg: libc::c_int,
        pub enc: krb5_key,
        pub seq: krb5_key,
        pub krb_times: krb5_ticket_times,
        pub krb_flags: krb5_flags,
        pub seq_send: uint64_t,
        pub seq_recv: uint64_t,
        pub seqstate: g_seqnum_state,
        pub k5_context: krb5_context,
        pub auth_context: krb5_auth_context,
        pub mech_used: *mut gss_OID_desc,
        pub proto: libc::c_int,
        pub cksumtype: krb5_cksumtype,
        pub acceptor_subkey: krb5_key,
        pub acceptor_subkey_cksumtype: krb5_cksumtype,
        pub cred_rcache: libc::c_int,
        pub authdata: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "204:1"]
    pub type krb5_gss_ctx_id_t = *mut _krb5_gss_ctx_id_rec;
    use super::krb5_h::{krb5_principal, krb5_magic, krb5_key,
                        krb5_ticket_times, krb5_flags, krb5_context,
                        krb5_auth_context, krb5_cksumtype, krb5_authdata};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::krb5_authdata_context;
    use super::gssapi_h::{OM_uint32, gss_OID_desc};
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::gssapiP_generic_h::g_seqnum_state;
    /* immutable */
    /* immutable */
    /* immutable */
    /* protects ad_context only for now */
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
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:66"]
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
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:66"]
pub mod string_h {
    extern "C" {
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
#[c2rust::header_src = "/usr/include/assert.h:66"]
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:66"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::__uint16_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:67"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc, OM_uint32};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/common.h - Declarations for GSSAPI test utility functions */
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
        #[no_mangle]
        #[c2rust::src_loc = "38:21"]
        pub static mut mech_krb5: gss_OID_desc;
    }
    /* COMMON_H */
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_16_be,
                              store_16_le};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, krb5_key, _krb5_enc_data, krb5_enc_data,
                       _krb5_ticket_times, krb5_ticket_times, _krb5_authdata,
                       krb5_authdata, _krb5_transited, krb5_transited,
                       _krb5_enc_tkt_part, krb5_enc_tkt_part, _krb5_ticket,
                       krb5_ticket, _krb5_ap_req, krb5_ap_req, _profile_t,
                       _krb5_auth_context, krb5_k_create_key,
                       krb5_k_free_key};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_key_st, derived_key,
                         _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_authdata_context,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle};
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
pub use self::gssapi_h::{OM_uint32, gss_uint32, gss_name_t, gss_OID,
                         gss_OID_desc_struct, gss_buffer_t,
                         gss_buffer_desc_struct, gss_cred_id_t, gss_ctx_id_t,
                         gss_OID_desc, gss_buffer_desc,
                         gss_channel_bindings_struct, gss_channel_bindings_t,
                         gss_qop_t, gss_ctx_id_struct, gss_accept_sec_context,
                         gss_delete_sec_context, gss_verify_mic, gss_unwrap,
                         gss_release_buffer};
pub use self::mglueP_h::{gss_name_struct, gss_cred_id_struct,
                         gss_union_ctx_id_struct, gss_union_ctx_id_t};
pub use self::gssapi_ext_h::{gss_iov_buffer_desc_struct, gss_iov_buffer_desc,
                             gss_unwrap_iov, gss_verify_mic_iov};
pub use self::gssapiP_generic_h::{g_seqnum_state, g_seqnum_state_st,
                                  gssint_g_seqstate_init};
pub use self::gssapiP_krb5_h::{sgn_alg, SGN_ALG_HMAC_SHA1_DES3_KD,
                               SGN_ALG_HMAC_MD5, seal_alg, SEAL_ALG_DES3KD,
                               SEAL_ALG_MICROSOFT_RC4, SEAL_ALG_NONE,
                               _krb5_gss_name_rec, krb5_gss_name_t,
                               _krb5_gss_ctx_id_rec, krb5_gss_ctx_id_t};
use self::stdlib_h::{abort, free, calloc, malloc};
use self::string_h::{memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::__bswap_16;
use self::common_h::mech_krb5;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_invalid.c - Invalid message token regression tests */
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
 * This file contains regression tests for some GSSAPI invalid token
 * vulnerabilities.
 *
 * 1. A pre-CFX wrap or MIC token processed with a CFX-only context causes a
 *    null pointer dereference.  (The token must use SEAL_ALG_NONE or it will
 *    be rejected.)
 *
 * 2. A pre-CFX wrap or MIC token with fewer than 24 bytes after the ASN.1
 *    header causes an input buffer overrun, usually leading to either a segv
 *    or a GSS_S_DEFECTIVE_TOKEN error due to garbage algorithm, filler, or
 *    sequence number values.
 *
 * 3. A pre-CFX wrap token with fewer than 16 + cksumlen bytes after the ASN.1
 *    header causes an integer underflow when computing the ciphertext length,
 *    leading to an allocation error on 32-bit platforms or a segv on 64-bit
 *    platforms.  A pre-CFX MIC token of this size causes an input buffer
 *    overrun when comparing the checksum, perhaps leading to a segv.
 *
 * 4. A pre-CFX wrap token with fewer than conflen + padlen bytes in the
 *    ciphertext (where padlen is the last byte of the decrypted ciphertext)
 *    causes an integer underflow when computing the original message length,
 *    leading to an allocation error.
 *
 * 5. In the mechglue, truncated encapsulation in the initial context token can
 *    cause input buffer overruns in gss_accept_sec_context().
 *
 * Vulnerabilities #1 and #2 also apply to IOV unwrap, although tokens with
 * fewer than 16 bytes after the ASN.1 header will be rejected.
 * Vulnerabilities #2 and #5 can only be robustly detected using a
 * memory-checking environment such as valgrind.
 */
/*
 * The following samples contain context parameters and otherwise valid seal
 * tokens where the plain text is padded with byte value 100 instead of the
 * proper value 1.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "76:8"]
pub struct test {
    pub enctype: krb5_enctype,
    pub encseq_enctype: krb5_enctype,
    pub sealalg: libc::c_int,
    pub signalg: libc::c_int,
    pub cksum_size: size_t,
    pub keylen: size_t,
    pub keydata: *const libc::c_char,
    pub toklen: size_t,
    pub token: *const libc::c_char,
}
#[no_mangle]
#[c2rust::src_loc = "86:3"]
pub static mut tests: [test; 2] =
    [{
         let mut init =
             test{enctype: 0x10 as libc::c_int,
                  encseq_enctype: 0x6 as libc::c_int,
                  sealalg: SEAL_ALG_DES3KD as libc::c_int,
                  signalg: SGN_ALG_HMAC_SHA1_DES3_KD as libc::c_int,
                  cksum_size: 20 as libc::c_int as size_t,
                  keylen: 24 as libc::c_int as size_t,
                  keydata:
                      b"O\xea\x19\x19^\x0e\x10\xdf=)\xb5\x13\x8f\x01\xc7\xa7\x92=8\xf7&s\rm\x00"
                          as *const u8 as *const libc::c_char,
                  toklen: 65 as libc::c_int as size_t,
                  token:
                      b"`?\x06\t*\x86H\x86\xf7\x12\x01\x02\x02\x02\x01\x04\x00\x02\x00\xff\xff\xeb\xf3\x9a\x89$W\xb8c\x95%\xe8n\x8ey\xe6.\xca\xd3\xffW\x9f\x8c\xab\xef\xdd(\x10/\x93!.\xf2R\xb6o\xa8\xbb\x8am\xaao\xb7\xf4\xd4\x00"
                          as *const u8 as *const libc::c_char,};
         init
     },
     {
         let mut init =
             test{enctype: 0x17 as libc::c_int,
                  encseq_enctype: 0x17 as libc::c_int,
                  sealalg: SEAL_ALG_MICROSOFT_RC4 as libc::c_int,
                  signalg: SGN_ALG_HMAC_MD5 as libc::c_int,
                  cksum_size: 8 as libc::c_int as size_t,
                  keylen: 16 as libc::c_int as size_t,
                  keydata:
                      b"fdAdUx!\xd0\xd0\xfd\x05j\xffo\xe8\t\x00" as *const u8
                          as *const libc::c_char,
                  toklen: 53 as libc::c_int as size_t,
                  token:
                      b"`3\x06\t*\x86H\x86\xf7\x12\x01\x02\x02\x02\x01\x11\x00\x10\x00\xff\xff5\xd4y\xf3\x8cG\x8fn#o>\xcc^W\\j\x89\xf0\xa2\x03O\x0bQ\x11\xee\x89~\xd6\xf6\xb5\xd6Q\x00"
                          as *const u8 as *const libc::c_char,};
         init
     }];
/* Fake up enough of a CFX GSS context for gss_unwrap, using an AES key. */
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn make_fake_cfx_context() -> gss_ctx_id_t {
    let mut uctx: gss_union_ctx_id_t = 0 as *mut gss_union_ctx_id_struct;
    let mut kgctx: krb5_gss_ctx_id_t = 0 as *mut _krb5_gss_ctx_id_rec;
    let mut kb: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    kgctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_krb5_gss_ctx_id_rec>() as libc::c_ulong)
            as krb5_gss_ctx_id_t;
    if kgctx.is_null() { abort(); }
    (*kgctx).set_established(1 as libc::c_int as libc::c_uint);
    (*kgctx).proto = 1 as libc::c_int;
    if gssint_g_seqstate_init(&mut (*kgctx).seqstate,
                              0 as libc::c_int as uint64_t, 0 as libc::c_int,
                              0 as libc::c_int, 0 as libc::c_int) !=
           0 as libc::c_int as libc::c_long {
        abort();
    }
    (*kgctx).mech_used = &mut mech_krb5;
    (*kgctx).sealalg = -(1 as libc::c_int);
    (*kgctx).signalg = -(1 as libc::c_int);
    kb.enctype = 0x11 as libc::c_int;
    kb.length = 16 as libc::c_int as libc::c_uint;
    kb.contents =
        b"1234567887654321\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_uchar;
    if krb5_k_create_key(0 as krb5_context, &mut kb, &mut (*kgctx).subkey) !=
           0 as libc::c_int {
        abort();
    }
    uctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_union_ctx_id_struct>() as
                   libc::c_ulong) as gss_union_ctx_id_t;
    if uctx.is_null() { abort(); }
    (*uctx).mech_type = &mut mech_krb5;
    (*uctx).internal_ctx_id = kgctx as gss_ctx_id_t;
    return uctx as gss_ctx_id_t;
}
/* Fake up enough of a GSS context for gss_unwrap, using keys from test. */
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn make_fake_context(mut test: *const test)
 -> gss_ctx_id_t {
    let mut uctx: gss_union_ctx_id_t = 0 as *mut gss_union_ctx_id_struct;
    let mut kgctx: krb5_gss_ctx_id_t = 0 as *mut _krb5_gss_ctx_id_rec;
    let mut kb: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    kgctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<_krb5_gss_ctx_id_rec>() as libc::c_ulong)
            as krb5_gss_ctx_id_t;
    if kgctx.is_null() { abort(); }
    (*kgctx).set_established(1 as libc::c_int as libc::c_uint);
    if gssint_g_seqstate_init(&mut (*kgctx).seqstate,
                              0 as libc::c_int as uint64_t, 0 as libc::c_int,
                              0 as libc::c_int, 0 as libc::c_int) !=
           0 as libc::c_int as libc::c_long {
        abort();
    }
    (*kgctx).mech_used = &mut mech_krb5;
    (*kgctx).sealalg = (*test).sealalg;
    (*kgctx).signalg = (*test).signalg;
    (*kgctx).cksum_size = (*test).cksum_size;
    kb.enctype = (*test).enctype;
    kb.length = (*test).keylen as libc::c_uint;
    kb.contents = (*test).keydata as *mut libc::c_uchar;
    if krb5_k_create_key(0 as krb5_context, &mut kb, &mut (*kgctx).subkey) !=
           0 as libc::c_int {
        abort();
    }
    kb.enctype = (*test).encseq_enctype;
    if krb5_k_create_key(0 as krb5_context, &mut kb, &mut (*kgctx).seq) !=
           0 as libc::c_int {
        abort();
    }
    if krb5_k_create_key(0 as krb5_context, &mut kb, &mut (*kgctx).enc) !=
           0 as libc::c_int {
        abort();
    }
    uctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<gss_union_ctx_id_struct>() as
                   libc::c_ulong) as gss_union_ctx_id_t;
    if uctx.is_null() { abort(); }
    (*uctx).mech_type = &mut mech_krb5;
    (*uctx).internal_ctx_id = kgctx as gss_ctx_id_t;
    return uctx as gss_ctx_id_t;
}
/* Free a context created by make_fake_context. */
#[c2rust::src_loc = "186:1"]
unsafe extern "C" fn free_fake_context(mut ctx: gss_ctx_id_t) {
    let mut uctx: gss_union_ctx_id_t = ctx as gss_union_ctx_id_t;
    let mut kgctx: krb5_gss_ctx_id_t =
        (*uctx).internal_ctx_id as krb5_gss_ctx_id_t;
    free((*kgctx).seqstate as *mut libc::c_void);
    krb5_k_free_key(0 as krb5_context, (*kgctx).subkey);
    krb5_k_free_key(0 as krb5_context, (*kgctx).seq);
    krb5_k_free_key(0 as krb5_context, (*kgctx).enc);
    free(kgctx as *mut libc::c_void);
    free(uctx as *mut libc::c_void);
}
/* Prefix a token (starting at the two-byte ID) with an ASN.1 header and return
 * it in an allocated block to facilitate checking by valgrind or similar. */
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn make_token(mut token: *mut libc::c_uchar,
                                mut len: size_t, mut out: gss_buffer_t) {
    let mut wrapped: *mut libc::c_char = 0 as *mut libc::c_char;
    if mech_krb5.length == 9 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"mech_krb5.length == 9\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_invalid.c\x00" as *const u8 as *const libc::c_char,
                      207 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void make_token(unsigned char *, size_t, gss_buffer_t)\x00")).as_ptr());
    }
    if len.wrapping_add(11 as libc::c_int as libc::c_ulong) <
           128 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"len + 11 < 128\x00" as *const u8 as
                          *const libc::c_char,
                      b"t_invalid.c\x00" as *const u8 as *const libc::c_char,
                      208 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"void make_token(unsigned char *, size_t, gss_buffer_t)\x00")).as_ptr());
    }
    wrapped =
        malloc(len.wrapping_add(13 as libc::c_int as libc::c_ulong)) as
            *mut libc::c_char;
    if wrapped.is_null() { abort(); }
    *wrapped.offset(0 as libc::c_int as isize) =
        0x60 as libc::c_int as libc::c_char;
    *wrapped.offset(1 as libc::c_int as isize) =
        len.wrapping_add(11 as libc::c_int as libc::c_ulong) as libc::c_char;
    *wrapped.offset(2 as libc::c_int as isize) =
        0x6 as libc::c_int as libc::c_char;
    *wrapped.offset(3 as libc::c_int as isize) =
        9 as libc::c_int as libc::c_char;
    memcpy(wrapped.offset(4 as libc::c_int as isize) as *mut libc::c_void,
           mech_krb5.elements, 9 as libc::c_int as libc::c_ulong);
    memcpy(wrapped.offset(13 as libc::c_int as isize) as *mut libc::c_void,
           token as *const libc::c_void, len);
    (*out).length = len.wrapping_add(13 as libc::c_int as libc::c_ulong);
    (*out).value = wrapped as *mut libc::c_void;
}
/* Unwrap a superficially valid RFC 1964 token with a CFX-only context, with
 * regular and IOV unwrap. */
#[c2rust::src_loc = "224:1"]
unsafe extern "C" fn test_bogus_1964_token(mut ctx: gss_ctx_id_t) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut tokbuf: [libc::c_uchar; 128] = [0; 128];
    let mut in_0: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut iov: gss_iov_buffer_desc =
        gss_iov_buffer_desc{type_0: 0,
                            buffer:
                                gss_buffer_desc{length: 0,
                                                value:
                                                    0 as
                                                        *mut libc::c_void,},};
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    store_16_le(SGN_ALG_HMAC_MD5 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(2 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le(SEAL_ALG_NONE as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le(0xffff as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(6 as libc::c_int as isize) as
                    *mut libc::c_void);
    memset(tokbuf.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_void, 0 as libc::c_int,
           16 as libc::c_int as libc::c_ulong);
    make_token(tokbuf.as_mut_ptr(), 24 as libc::c_int as size_t, &mut in_0);
    major =
        gss_unwrap(&mut minor, ctx, &mut in_0, &mut out,
                   0 as *mut libc::c_int, 0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    gss_release_buffer(&mut minor, &mut out);
    iov.type_0 = 2 as libc::c_int as OM_uint32;
    iov.buffer = in_0;
    major =
        gss_unwrap_iov(&mut minor, ctx, 0 as *mut libc::c_int,
                       0 as *mut gss_qop_t, &mut iov, 1 as libc::c_int);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
}
/* Process wrap and MIC tokens with incomplete headers. */
#[c2rust::src_loc = "254:1"]
unsafe extern "C" fn test_short_header(mut ctx: gss_ctx_id_t) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut tokbuf: [libc::c_uchar; 128] = [0; 128];
    let mut in_0: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut empty: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    /* Seal token, 2-24 bytes */
    store_16_be(0x201 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    make_token(tokbuf.as_mut_ptr(), 2 as libc::c_int as size_t, &mut in_0);
    major =
        gss_unwrap(&mut minor, ctx, &mut in_0, &mut out,
                   0 as *mut libc::c_int, 0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
    gss_release_buffer(&mut minor, &mut out);
    /* Sign token, 2-24 bytes */
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    make_token(tokbuf.as_mut_ptr(), 2 as libc::c_int as size_t, &mut in_0);
    major =
        gss_unwrap(&mut minor, ctx, &mut in_0, &mut out,
                   0 as *mut libc::c_int, 0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
    gss_release_buffer(&mut minor, &mut out);
    /* MIC token, 2-24 bytes */
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    make_token(tokbuf.as_mut_ptr(), 2 as libc::c_int as size_t, &mut in_0);
    major =
        gss_verify_mic(&mut minor, ctx, &mut empty, &mut in_0,
                       0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
}
/* Process wrap and MIC tokens with incomplete headers. */
#[c2rust::src_loc = "289:1"]
unsafe extern "C" fn test_short_header_iov(mut ctx: gss_ctx_id_t,
                                           mut test: *const test) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut tokbuf: [libc::c_uchar; 128] = [0; 128];
    let mut iov: gss_iov_buffer_desc =
        gss_iov_buffer_desc{type_0: 0,
                            buffer:
                                gss_buffer_desc{length: 0,
                                                value:
                                                    0 as
                                                        *mut libc::c_void,},};
    /* IOV seal token, 16-23 bytes */
    store_16_be(0x201 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    store_16_le((*test).signalg as libc::c_uint,
                tokbuf.as_mut_ptr().offset(2 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le((*test).sealalg as libc::c_uint,
                tokbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_be(0xffff as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(6 as libc::c_int as isize) as
                    *mut libc::c_void);
    memset(tokbuf.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_void, 0 as libc::c_int,
           8 as libc::c_int as libc::c_ulong);
    iov.type_0 = 2 as libc::c_int as OM_uint32;
    make_token(tokbuf.as_mut_ptr(), 16 as libc::c_int as size_t,
               &mut iov.buffer);
    major =
        gss_unwrap_iov(&mut minor, ctx, 0 as *mut libc::c_int,
                       0 as *mut gss_qop_t, &mut iov, 1 as libc::c_int);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(iov.buffer.value);
    /* IOV sign token, 16-23 bytes */
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    store_16_le((*test).signalg as libc::c_uint,
                tokbuf.as_mut_ptr().offset(2 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le(SEAL_ALG_NONE as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le(0xffff as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(6 as libc::c_int as isize) as
                    *mut libc::c_void);
    memset(tokbuf.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_void, 0 as libc::c_int,
           8 as libc::c_int as libc::c_ulong);
    iov.type_0 = 2 as libc::c_int as OM_uint32;
    make_token(tokbuf.as_mut_ptr(), 16 as libc::c_int as size_t,
               &mut iov.buffer);
    major =
        gss_unwrap_iov(&mut minor, ctx, 0 as *mut libc::c_int,
                       0 as *mut gss_qop_t, &mut iov, 1 as libc::c_int);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(iov.buffer.value);
    /* IOV MIC token, 16-23 bytes */
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    store_16_be((*test).signalg as libc::c_uint,
                tokbuf.as_mut_ptr().offset(2 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le(SEAL_ALG_NONE as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    store_16_le(0xffff as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(6 as libc::c_int as isize) as
                    *mut libc::c_void);
    memset(tokbuf.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_void, 0 as libc::c_int,
           8 as libc::c_int as libc::c_ulong);
    iov.type_0 = 12 as libc::c_int as OM_uint32;
    make_token(tokbuf.as_mut_ptr(), 16 as libc::c_int as size_t,
               &mut iov.buffer);
    major =
        gss_verify_mic_iov(&mut minor, ctx, 0 as *mut gss_qop_t, &mut iov,
                           1 as libc::c_int);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(iov.buffer.value);
}
/* Process wrap and MIC tokens with incomplete checksums. */
#[c2rust::src_loc = "337:1"]
unsafe extern "C" fn test_short_checksum(mut ctx: gss_ctx_id_t,
                                         mut test: *const test) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut tokbuf: [libc::c_uchar; 128] = [0; 128];
    let mut in_0: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut empty: gss_buffer_desc =
        {
            let mut init =
                gss_buffer_desc_struct{length: 0 as libc::c_int as size_t,
                                       value: 0 as *mut libc::c_void,};
            init
        };
    /* Can only do this with the DES3 checksum, as we can't easily get past
     * retrieving the sequence number when the checksum is only eight bytes. */
    if (*test).cksum_size <= 8 as libc::c_int as libc::c_ulong { return }
    /* Seal token, fewer than 16 + cksum_size bytes.  Use the token from the
     * test data to get a valid sequence number. */
    make_token(((*test).token as
                    *mut libc::c_uchar).offset(13 as libc::c_int as isize),
               24 as libc::c_int as size_t, &mut in_0);
    major =
        gss_unwrap(&mut minor, ctx, &mut in_0, &mut out,
                   0 as *mut libc::c_int, 0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
    gss_release_buffer(&mut minor, &mut out);
    /* Sign token, fewer than 16 + cksum_size bytes. */
    memcpy(tokbuf.as_mut_ptr() as *mut libc::c_void,
           (*test).token.offset(13 as libc::c_int as isize) as
               *const libc::c_void, 24 as libc::c_int as libc::c_ulong);
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    store_16_le(SEAL_ALG_NONE as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    make_token(tokbuf.as_mut_ptr(), 24 as libc::c_int as size_t, &mut in_0);
    major =
        gss_unwrap(&mut minor, ctx, &mut in_0, &mut out,
                   0 as *mut libc::c_int, 0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
    gss_release_buffer(&mut minor, &mut out);
    /* MIC token, fewer than 16 + cksum_size bytes. */
    memcpy(tokbuf.as_mut_ptr() as *mut libc::c_void,
           (*test).token.offset(13 as libc::c_int as isize) as
               *const libc::c_void, 24 as libc::c_int as libc::c_ulong);
    store_16_be(0x101 as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr() as *mut libc::c_void);
    store_16_le(SEAL_ALG_NONE as libc::c_int as libc::c_uint,
                tokbuf.as_mut_ptr().offset(4 as libc::c_int as isize) as
                    *mut libc::c_void);
    make_token(tokbuf.as_mut_ptr(), 24 as libc::c_int as size_t, &mut in_0);
    major =
        gss_verify_mic(&mut minor, ctx, &mut empty, &mut in_0,
                       0 as *mut gss_qop_t);
    if major != (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    free(in_0.value);
}
/* Unwrap a token with a bogus padding byte in the decrypted ciphertext. */
#[c2rust::src_loc = "380:1"]
unsafe extern "C" fn test_bad_pad(mut ctx: gss_ctx_id_t,
                                  mut test: *const test) {
    let mut minor: OM_uint32 = 0;
    let mut major: OM_uint32 = 0;
    let mut in_0: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    in_0.length = (*test).toklen;
    in_0.value = (*test).token as *mut libc::c_char as *mut libc::c_void;
    major =
        gss_unwrap(&mut minor, ctx, &mut in_0, &mut out,
                   0 as *mut libc::c_int, 0 as *mut gss_qop_t);
    if major != (6 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
        abort();
    }
    gss_release_buffer(&mut minor, &mut out);
}
#[c2rust::src_loc = "394:1"]
unsafe extern "C" fn try_accept(mut value: *mut libc::c_void,
                                mut len: size_t) {
    let mut minor: OM_uint32 = 0;
    let mut in_0: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut out: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut ctx: gss_ctx_id_t = 0 as gss_ctx_id_t;
    /* Copy the provided value to make input overruns more obvious. */
    in_0.value = malloc(len);
    if in_0.value.is_null() { abort(); }
    memcpy(in_0.value, value, len);
    in_0.length = len;
    gss_accept_sec_context(&mut minor, &mut ctx, 0 as gss_cred_id_t,
                           &mut in_0, 0 as gss_channel_bindings_t,
                           0 as *mut gss_name_t, 0 as *mut gss_OID, &mut out,
                           0 as *mut OM_uint32, 0 as *mut OM_uint32,
                           0 as *mut gss_cred_id_t);
    gss_release_buffer(&mut minor, &mut out);
    gss_delete_sec_context(&mut minor, &mut ctx, 0 as gss_buffer_t);
    free(in_0.value);
}
/* Accept contexts using superficially valid but truncated encapsulations. */
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn test_short_encapsulation() {
    /* Include just the initial application tag, to see if we overrun reading
     * the sequence length. */
    try_accept(b"`\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_void, 1 as libc::c_int as size_t);
    /* Indicate four additional sequence length bytes, to see if we overrun
     * reading them (or skipping them and reading the next byte). */
    try_accept(b"`\x84\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_void, 2 as libc::c_int as size_t);
    /* Include an object identifier tag but no length, to see if we overrun
     * reading the length. */
    try_accept(b"`@\x06\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_void, 3 as libc::c_int as size_t);
    /* Include an object identifier tag with a length matching the krb5 mech,
     * but no OID bytes, to see if we overrun comparing against mechs. */
    try_accept(b"`@\x06\t\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_void, 4 as libc::c_int as size_t);
}
#[c2rust::src_loc = "436:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut ctx: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut i: size_t = 0;
    ctx = make_fake_cfx_context();
    test_bogus_1964_token(ctx);
    free_fake_context(ctx);
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[test; 2]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<test>()
                                                   as libc::c_ulong) {
        ctx = make_fake_context(&mut *tests.as_mut_ptr().offset(i as isize));
        test_short_header(ctx);
        test_short_header_iov(ctx,
                              &mut *tests.as_mut_ptr().offset(i as isize));
        test_short_checksum(ctx, &mut *tests.as_mut_ptr().offset(i as isize));
        test_bad_pad(ctx, &mut *tests.as_mut_ptr().offset(i as isize));
        free_fake_context(ctx);
        i = i.wrapping_add(1)
    }
    test_short_encapsulation();
    return 0 as libc::c_int;
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
