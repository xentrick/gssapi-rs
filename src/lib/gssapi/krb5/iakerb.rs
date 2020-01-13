use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:25"]
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:25"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:25"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:25"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:25"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:25"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:25"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:25"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:25"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:25"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:25"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "561:5"]
    pub struct C2RustUnnamed {
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
    /* If we're using gcc, if the C++ support works, the compiler should
   build executables and shared libraries that support the use of
   static constructors and destructors.  The C compiler supports a
   function attribute that makes use of the same facility as C++.

   XXX How do we know if the C++ support actually works?  */
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
    use super::stdint_uintn_h::uint16_t;
    use super::byteswap_h::__bswap_16;
    use super::types_h::__uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:25"]
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
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    #[c2rust::src_loc = "2710:1"]
    pub type krb5_rcache = *mut krb5_rc_st;
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
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
    /* The name of the Kerberos ticket granting service... and its size */
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
    #[c2rust::src_loc = "7313:1"]
    pub type krb5_init_creds_context = *mut _krb5_init_creds_context;
    #[c2rust::src_loc = "7505:1"]
    pub type krb5_tkt_creds_context = *mut _krb5_tkt_creds_context;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, krb5_key_st, _krb5_kt};
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
        #[c2rust::src_loc = "7312:8"]
        pub type _krb5_init_creds_context;
        #[c2rust::src_loc = "7504:8"]
        pub type _krb5_tkt_creds_context;
        /* *
 * Compute a checksum (operates on opaque key).
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
 * @note This function is similar to krb5_c_make_checksum(), but operates
 * on opaque @a key.
 *
 * @sa krb5_c_verify_checksum()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "1457:1"]
        pub fn krb5_k_make_checksum(context: krb5_context,
                                    cksumtype: krb5_cksumtype, key: krb5_key,
                                    usage: krb5_keyusage,
                                    input: *const krb5_data,
                                    cksum: *mut krb5_checksum)
         -> krb5_error_code;
        /* *
 * Verify a checksum (operates on opaque key).
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
 * @note This function is similar to krb5_c_verify_checksum(), but operates
 * on opaque @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "1511:1"]
        pub fn krb5_k_verify_checksum(context: krb5_context, key: krb5_key,
                                      usage: krb5_keyusage,
                                      data: *const krb5_data,
                                      cksum: *const krb5_checksum,
                                      valid: *mut krb5_boolean)
         -> krb5_error_code;
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
        /* *< Want cached ticket only */
        /* *< Set canonicalize KDC option */
        /* *< Do not store in credential cache */
        /* *< Acquire forwardable tickets */
        /* *< Disable transited check */
        /* *< Constrained delegation */
        /* * @} */
        /* end of KRB5_GC group */
        /* *
 * Get an additional ticket.
 *
 * @param [in]  context         Library context
 * @param [in]  options         Options
 * @param [in]  ccache          Credential cache handle
 * @param [in]  in_creds        Input credentials
 * @param [out] out_creds       Output updated credentials
 *
 * Use @a ccache or a TGS exchange to get a service ticket matching @a
 * in_creds.
 *
 * Valid values for @a options are:
 * @li #KRB5_GC_CACHED     Search only credential cache for the ticket
 * @li #KRB5_GC_USER_USER  Return a user to user authentication ticket
 *
 * @a in_creds must be non-null.  @a in_creds->client and @a in_creds->server
 * must be filled in to specify the client and the server respectively.  If any
 * authorization data needs to be requested for the service ticket (such as
 * restrictions on how the ticket can be used), specify it in @a
 * in_creds->authdata; otherwise set @a in_creds->authdata to NULL.  The
 * session key type is specified in @a in_creds->keyblock.enctype, if it is
 * nonzero.
 *
 * The expiration date is specified in @a in_creds->times.endtime.
 * The KDC may return tickets with an earlier expiration date.
 * If @a in_creds->times.endtime is set to 0, the latest possible
 * expiration date will be requested.
 *
 * Any returned ticket and intermediate ticket-granting tickets are stored
 * in @a ccache.
 *
 * Use krb5_free_creds() to free @a out_creds when it is no longer needed.
 *
 * @retval
 *  0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3119:1"]
        pub fn krb5_get_credentials(context: krb5_context,
                                    options: krb5_flags, ccache: krb5_ccache,
                                    in_creds: *mut krb5_creds,
                                    out_creds: *mut *mut krb5_creds)
         -> krb5_error_code;
        /* *
 * Format and encode a @c KRB_ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  dec_err         Error structure to be encoded
 * @param [out] enc_err         Encoded error structure
 *
 * This function creates a @c KRB_ERROR message in @a enc_err.  Use
 * krb5_free_data_contents() to free @a enc_err when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3292:1"]
        pub fn krb5_mk_error(context: krb5_context,
                             dec_err: *const krb5_error,
                             enc_err: *mut krb5_data) -> krb5_error_code;
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
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        /* *
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4666:1"]
        pub fn krb5_free_creds(context: krb5_context, val: *mut krb5_creds);
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
 * Set the ticket lifetime in initial credential options.
 *
 * @param [in] opt              Options structure
 * @param [in] tkt_life         Ticket lifetime
 */
        #[no_mangle]
        #[c2rust::src_loc = "6877:1"]
        pub fn krb5_get_init_creds_opt_set_tkt_life(opt:
                                                        *mut krb5_get_init_creds_opt,
                                                    tkt_life: krb5_deltat);
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
        #[no_mangle]
        #[c2rust::src_loc = "7326:1"]
        pub fn krb5_init_creds_free(context: krb5_context,
                                    ctx: krb5_init_creds_context);
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
        #[no_mangle]
        #[c2rust::src_loc = "7398:1"]
        pub fn krb5_init_creds_init(context: krb5_context,
                                    client: krb5_principal,
                                    prompter: krb5_prompter_fct,
                                    data: *mut libc::c_void,
                                    start_time: krb5_deltat,
                                    options: *mut krb5_get_init_creds_opt,
                                    ctx: *mut krb5_init_creds_context)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7416:1"]
        pub fn krb5_init_creds_set_keytab(context: krb5_context,
                                          ctx: krb5_init_creds_context,
                                          keytab: krb5_keytab)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7450:1"]
        pub fn krb5_init_creds_step(context: krb5_context,
                                    ctx: krb5_init_creds_context,
                                    in_0: *mut krb5_data, out: *mut krb5_data,
                                    realm: *mut krb5_data,
                                    flags: *mut libc::c_uint)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7467:1"]
        pub fn krb5_init_creds_set_password(context: krb5_context,
                                            ctx: krb5_init_creds_context,
                                            password: *const libc::c_char)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7500:1"]
        pub fn krb5_init_creds_get_times(context: krb5_context,
                                         ctx: krb5_init_creds_context,
                                         times: *mut krb5_ticket_times)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7531:1"]
        pub fn krb5_tkt_creds_init(context: krb5_context, ccache: krb5_ccache,
                                   creds: *mut krb5_creds,
                                   options: krb5_flags,
                                   ctx: *mut krb5_tkt_creds_context)
         -> krb5_error_code;
        /* *
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
        #[no_mangle]
        #[c2rust::src_loc = "7581:1"]
        pub fn krb5_tkt_creds_free(context: krb5_context,
                                   ctx: krb5_tkt_creds_context);
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
        #[no_mangle]
        #[c2rust::src_loc = "7614:1"]
        pub fn krb5_tkt_creds_step(context: krb5_context,
                                   ctx: krb5_tkt_creds_context,
                                   in_0: *mut krb5_data, out: *mut krb5_data,
                                   realm: *mut krb5_data,
                                   flags: *mut libc::c_uint)
         -> krb5_error_code;
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
        #[no_mangle]
        #[c2rust::src_loc = "7633:1"]
        pub fn krb5_tkt_creds_get_times(context: krb5_context,
                                        ctx: krb5_tkt_creds_context,
                                        times: *mut krb5_ticket_times)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:25"]
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    /* Increment a timestamp by a signed 32-bit interval, without relying on
 * undefined behavior. */
    #[inline]
    #[c2rust::src_loc = "2354:1"]
    pub unsafe extern "C" fn ts_incr(mut ts: krb5_timestamp,
                                     mut delta: krb5_deltat)
     -> krb5_timestamp {
        return (ts as uint32_t).wrapping_add(delta as uint32_t) as
                   krb5_timestamp;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_keyblock, krb5_data, krb5_key, krb5_pointer,
                        krb5_error_code, krb5_context, krb5_keytab,
                        krb5_const_principal, krb5_kvno, krb5_keytab_entry,
                        krb5_kt_cursor, krb5_checksum, krb5_authdatatype,
                        krb5_authdata, krb5_error, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
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
        #[c2rust::src_loc = "616:1"]
        pub fn krb5_sendto_kdc(_: krb5_context, _: *const krb5_data,
                               _: *const krb5_data, _: *mut krb5_data,
                               _: *mut libc::c_int, _: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "953:1"]
        pub fn krb5_free_iakerb_header(_: krb5_context,
                                       _: *mut krb5_iakerb_header);
        #[no_mangle]
        #[c2rust::src_loc = "954:1"]
        pub fn krb5_free_iakerb_finished(_: krb5_context,
                                         _: *mut krb5_iakerb_finished);
        #[no_mangle]
        #[c2rust::src_loc = "1015:1"]
        pub fn krb5_authdata_export_authdata(kcontext: krb5_context,
                                             context: krb5_authdata_context,
                                             usage: krb5_flags,
                                             pauthdata:
                                                 *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1505:1"]
        pub fn encode_krb5_iakerb_header(_: *const krb5_iakerb_header,
                                         _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1508:1"]
        pub fn encode_krb5_iakerb_finished(_: *const krb5_iakerb_finished,
                                           _: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1645:1"]
        pub fn decode_krb5_error(output: *const krb5_data,
                                 rep: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1702:1"]
        pub fn decode_krb5_iakerb_header(_: *const krb5_data,
                                         _: *mut *mut krb5_iakerb_header)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1705:1"]
        pub fn decode_krb5_iakerb_finished(_: *const krb5_data,
                                           _: *mut *mut krb5_iakerb_finished)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:26"]
pub mod gssapi_h {
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    #[c2rust::src_loc = "92:1"]
    pub type gss_int32 = int32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
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
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
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
    #[c2rust::src_loc = "135:1"]
    pub type gss_cred_usage_t = libc::c_int;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:26"]
pub mod gssapi_ext_h {
    /* acceptor_time_rec */
    /*
 * GGF extensions
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "248:16"]
    pub struct gss_iov_buffer_desc_struct {
        pub type_0: OM_uint32,
        pub buffer: gss_buffer_desc,
    }
    #[c2rust::src_loc = "248:1"]
    pub type gss_iov_buffer_desc = gss_iov_buffer_desc_struct;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32};
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:26"]
pub mod gssapiP_generic_h {
    #[c2rust::src_loc = "129:1"]
    pub type g_seqnum_state = *mut g_seqnum_state_st;
    use super::gssapi_h::{gss_OID_desc, gss_int32};
    extern "C" {
        #[c2rust::src_loc = "129:16"]
        pub type g_seqnum_state_st;
        #[no_mangle]
        #[c2rust::src_loc = "154:1"]
        pub fn gssint_g_token_size(mech: *const gss_OID_desc,
                                   body_size: libc::c_uint) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "156:1"]
        pub fn gssint_g_make_token_header(mech: *const gss_OID_desc,
                                          body_size: libc::c_uint,
                                          buf: *mut *mut libc::c_uchar,
                                          tok_type: libc::c_int);
        /* flags for g_verify_token_header() */
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn gssint_g_verify_token_header(mech: *const gss_OID_desc,
                                            body_size: *mut libc::c_uint,
                                            buf: *mut *mut libc::c_uchar,
                                            tok_type: libc::c_int,
                                            toksize_in: libc::c_uint,
                                            flags: libc::c_int) -> gss_int32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:26"]
pub mod gssapiP_krb5_h {
    /* for 3DES */
    /* for draft-ietf-krb-wg-gssapi-cfx-01 */
    /* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
    /* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
    /* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
    /* * internal types **/
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
    #[c2rust::src_loc = "197:16"]
    pub struct _krb5_gss_ctx_ext_rec {
        pub iakerb: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "198:5"]
    pub struct C2RustUnnamed_0 {
        pub conv: *mut krb5_data,
        pub verified: libc::c_int,
    }
    #[c2rust::src_loc = "197:1"]
    pub type krb5_gss_ctx_ext_rec = _krb5_gss_ctx_ext_rec;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_gss_ctx_ext_t = *mut _krb5_gss_ctx_ext_rec;
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
    use super::krb5_h::{krb5_principal, krb5_keytab, krb5_rcache, krb5_ccache,
                        krb5_boolean, krb5_timestamp, krb5_enctype, krb5_data,
                        krb5_magic, krb5_key, krb5_ticket_times, krb5_flags,
                        krb5_context, krb5_auth_context, krb5_cksumtype,
                        krb5_authdata, krb5_error_code};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::{krb5_authdata_context, _krb5_context};
    use super::gssapi_h::{gss_cred_usage_t, OM_uint32, gss_OID_desc,
                          gss_cred_id_struct, gss_cred_id_t, gss_name_struct,
                          gss_name_t, gss_OID_set_desc_struct, gss_OID_set,
                          gss_ctx_id_t, gss_OID_desc_struct, gss_OID,
                          gss_channel_bindings_struct, gss_channel_bindings_t,
                          gss_buffer_desc_struct, gss_buffer_t,
                          gss_ctx_id_struct, gss_qop_t};
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::gssapiP_generic_h::g_seqnum_state;
    use super::gssapi_ext_h::{gss_buffer_set_t, gss_iov_buffer_desc};
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "492:1"]
        pub fn kg_cred_time_to_refresh(context: krb5_context,
                                       cred: *mut krb5_gss_cred_id_rec)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "495:1"]
        pub fn kg_cred_set_initial_refresh(context: krb5_context,
                                           cred: *mut krb5_gss_cred_id_rec,
                                           times: *mut krb5_ticket_times);
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn kg_cred_resolve(minor_status: *mut OM_uint32,
                               context: krb5_context,
                               cred_handle: gss_cred_id_t,
                               target_name: gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "516:1"]
        pub fn iakerb_gss_acquire_cred(_: *mut OM_uint32, _: gss_name_t,
                                       _: OM_uint32, _: gss_OID_set,
                                       _: gss_cred_usage_t,
                                       _: *mut gss_cred_id_t,
                                       _: *mut gss_OID_set, _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn krb5_gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "573:1"]
        pub fn krb5_gss_init_sec_context_ext(_: *mut OM_uint32,
                                             _: gss_cred_id_t,
                                             _: *mut gss_ctx_id_t,
                                             _: gss_name_t, _: gss_OID,
                                             _: OM_uint32, _: OM_uint32,
                                             _: gss_channel_bindings_t,
                                             _: gss_buffer_t, _: *mut gss_OID,
                                             _: gss_buffer_t,
                                             _: *mut OM_uint32,
                                             _: *mut OM_uint32,
                                             _: krb5_gss_ctx_ext_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "607:1"]
        pub fn krb5_gss_accept_sec_context_ext(_: *mut OM_uint32,
                                               _: *mut gss_ctx_id_t,
                                               _: gss_cred_id_t,
                                               _: gss_buffer_t,
                                               _: gss_channel_bindings_t,
                                               _: *mut gss_name_t,
                                               _: *mut gss_OID,
                                               _: gss_buffer_t,
                                               _: *mut OM_uint32,
                                               _: *mut OM_uint32,
                                               _: *mut gss_cred_id_t,
                                               _: krb5_gss_ctx_ext_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "624:1"]
        pub fn krb5_gss_inquire_sec_context_by_oid(_: *mut OM_uint32,
                                                   _: gss_ctx_id_t,
                                                   _: gss_OID,
                                                   _: *mut gss_buffer_set_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "632:1"]
        pub fn krb5_gss_set_sec_context_option(_: *mut OM_uint32,
                                               _: *mut gss_ctx_id_t,
                                               _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn krb5_gss_process_context_token(_: *mut OM_uint32,
                                              _: gss_ctx_id_t,
                                              _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "645:1"]
        pub fn krb5_gss_delete_sec_context(_: *mut OM_uint32,
                                           _: *mut gss_ctx_id_t,
                                           _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "651:1"]
        pub fn krb5_gss_context_time(_: *mut OM_uint32, _: gss_ctx_id_t,
                                     _: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn krb5_gss_inquire_context(_: *mut OM_uint32, _: gss_ctx_id_t,
                                        _: *mut gss_name_t,
                                        _: *mut gss_name_t, _: *mut OM_uint32,
                                        _: *mut gss_OID, _: *mut OM_uint32,
                                        _: *mut libc::c_int,
                                        _: *mut libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "720:1"]
        pub fn krb5_gss_get_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                                _: gss_qop_t, _: gss_buffer_t,
                                _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "728:1"]
        pub fn krb5_gss_get_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                    _: gss_qop_t, _: *mut gss_iov_buffer_desc,
                                    _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "736:1"]
        pub fn krb5_gss_get_mic_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                           _: gss_qop_t,
                                           _: *mut gss_iov_buffer_desc,
                                           _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "744:1"]
        pub fn krb5_gss_verify_mic(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: gss_buffer_t, _: gss_buffer_t,
                                   _: *mut gss_qop_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "752:1"]
        pub fn krb5_gss_verify_mic_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                       _: *mut gss_qop_t,
                                       _: *mut gss_iov_buffer_desc,
                                       _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "760:1"]
        pub fn krb5_gss_wrap(_: *mut OM_uint32, _: gss_ctx_id_t,
                             _: libc::c_int, _: gss_qop_t, _: gss_buffer_t,
                             _: *mut libc::c_int, _: gss_buffer_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "770:1"]
        pub fn krb5_gss_wrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                 _: libc::c_int, _: gss_qop_t,
                                 _: *mut libc::c_int,
                                 _: *mut gss_iov_buffer_desc, _: libc::c_int)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "780:1"]
        pub fn krb5_gss_wrap_iov_length(_: *mut OM_uint32, _: gss_ctx_id_t,
                                        _: libc::c_int, _: gss_qop_t,
                                        _: *mut libc::c_int,
                                        _: *mut gss_iov_buffer_desc,
                                        _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "790:1"]
        pub fn krb5_gss_unwrap(_: *mut OM_uint32, _: gss_ctx_id_t,
                               _: gss_buffer_t, _: gss_buffer_t,
                               _: *mut libc::c_int, _: *mut gss_qop_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "799:1"]
        pub fn krb5_gss_unwrap_iov(_: *mut OM_uint32, _: gss_ctx_id_t,
                                   _: *mut libc::c_int, _: *mut gss_qop_t,
                                   _: *mut gss_iov_buffer_desc,
                                   _: libc::c_int) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "808:1"]
        pub fn krb5_gss_wrap_size_limit(_: *mut OM_uint32, _: gss_ctx_id_t,
                                        _: libc::c_int, _: gss_qop_t,
                                        _: OM_uint32, _: *mut OM_uint32)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "841:1"]
        pub fn krb5_gss_export_sec_context(_: *mut OM_uint32,
                                           _: *mut gss_ctx_id_t,
                                           _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "847:1"]
        pub fn krb5_gss_import_sec_context(_: *mut OM_uint32, _: gss_buffer_t,
                                           _: *mut gss_ctx_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1006:1"]
        pub fn krb5_gss_pseudo_random(minor_status: *mut OM_uint32,
                                      context: gss_ctx_id_t,
                                      prf_key: libc::c_int,
                                      prf_in: gss_buffer_t,
                                      desired_output_len: ssize_t,
                                      prf_out: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "1105:1"]
        pub fn krb5_gss_init_context(ctxp: *mut krb5_context)
         -> krb5_error_code;
    }
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:25"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:25"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:25"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/assert.h:25"]
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:25"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:26"]
pub mod gssapi_alloc_h {
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: size_t)
     -> *mut libc::c_void {
        return malloc(size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::malloc;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapi_krb5.h:26"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        /* {iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-enterprise-name(6)}. */
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "84:33"]
        pub static gss_mech_iakerb: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t,
                        __uint64_t, __off_t, __off64_t, __ssize_t};
pub use self::sys_types_h::ssize_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_unlock,
                            k5_os_mutex_unlock};
pub use self::k5_platform_h::{C2RustUnnamed, store_16_be};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_keyusage, krb5_preauthtype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, krb5_key, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_creds, krb5_creds, _krb5_error, krb5_error,
                       _krb5_ap_req, krb5_ap_req, krb5_ccache, krb5_rcache,
                       krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _krb5_prompt,
                       krb5_prompt, krb5_prompter_fct,
                       _krb5_get_init_creds_opt, krb5_get_init_creds_opt,
                       krb5_init_creds_context, krb5_tkt_creds_context,
                       _profile_t, _krb5_auth_context, _krb5_ccache,
                       krb5_rc_st, _krb5_init_creds_context,
                       _krb5_tkt_creds_context, krb5_k_make_checksum,
                       krb5_k_verify_checksum, krb5_free_context,
                       krb5_get_credentials, krb5_mk_error,
                       krb5_free_authdata, krb5_free_error, krb5_free_creds,
                       krb5_free_checksum_contents, krb5_free_data,
                       krb5_free_data_contents, krb5_timeofday,
                       krb5_get_init_creds_opt_alloc,
                       krb5_get_init_creds_opt_free,
                       krb5_get_init_creds_opt_set_tkt_life,
                       krb5_get_init_creds_opt_set_out_ccache,
                       krb5_init_creds_free, krb5_init_creds_init,
                       krb5_init_creds_set_keytab, krb5_init_creds_step,
                       krb5_init_creds_set_password,
                       krb5_init_creds_get_times, krb5_tkt_creds_init,
                       krb5_tkt_creds_free, krb5_tkt_creds_step,
                       krb5_tkt_creds_get_times, krb5_clear_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_key_st, derived_key, _krb5_kt,
                         _krb5_kt_ops, _krb5_iakerb_header,
                         krb5_iakerb_header, _krb5_iakerb_finished,
                         krb5_iakerb_finished, _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_authdata_context,
                         make_data, empty_data, k5calloc, k5alloc, ts_incr,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_sendto_kdc,
                         krb5_free_iakerb_header, krb5_free_iakerb_finished,
                         krb5_authdata_export_authdata,
                         encode_krb5_iakerb_header,
                         encode_krb5_iakerb_finished, decode_krb5_error,
                         decode_krb5_iakerb_header,
                         decode_krb5_iakerb_finished};
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
                         gss_ctx_id_t, gss_int32, gss_OID_desc_struct,
                         gss_OID_desc, gss_OID, gss_OID_set_desc_struct,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_qop_t, gss_cred_usage_t,
                         gss_name_struct, gss_cred_id_struct,
                         gss_ctx_id_struct, gss_release_buffer};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_iov_buffer_desc_struct, gss_iov_buffer_desc};
pub use self::gssapiP_generic_h::{g_seqnum_state, g_seqnum_state_st,
                                  gssint_g_token_size,
                                  gssint_g_make_token_header,
                                  gssint_g_verify_token_header};
pub use self::gssapiP_krb5_h::{_krb5_gss_name_rec, krb5_gss_name_t,
                               _krb5_gss_cred_id_rec, krb5_gss_cred_id_rec,
                               krb5_gss_cred_id_t, _krb5_gss_ctx_ext_rec,
                               C2RustUnnamed_0, krb5_gss_ctx_ext_rec,
                               krb5_gss_ctx_ext_t, _krb5_gss_ctx_id_rec,
                               krb5_gss_ctx_id_t, kg_cred_time_to_refresh,
                               kg_cred_set_initial_refresh, kg_cred_resolve,
                               iakerb_gss_acquire_cred, krb5_gss_release_cred,
                               krb5_gss_init_sec_context_ext,
                               krb5_gss_accept_sec_context_ext,
                               krb5_gss_inquire_sec_context_by_oid,
                               krb5_gss_set_sec_context_option,
                               krb5_gss_process_context_token,
                               krb5_gss_delete_sec_context,
                               krb5_gss_context_time,
                               krb5_gss_inquire_context, krb5_gss_get_mic,
                               krb5_gss_get_mic_iov,
                               krb5_gss_get_mic_iov_length,
                               krb5_gss_verify_mic, krb5_gss_verify_mic_iov,
                               krb5_gss_wrap, krb5_gss_wrap_iov,
                               krb5_gss_wrap_iov_length, krb5_gss_unwrap,
                               krb5_gss_unwrap_iov, krb5_gss_wrap_size_limit,
                               krb5_gss_export_sec_context,
                               krb5_gss_import_sec_context,
                               krb5_gss_pseudo_random, krb5_gss_init_context};
use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::{free, realloc, calloc, malloc};
use self::string_h::{strerror, memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::__bswap_16;
pub use self::gssapi_alloc_h::gssalloc_malloc;
use self::gssapi_krb5_h::{gss_mech_krb5, gss_mech_iakerb};
extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2009  by the Massachusetts Institute of Technology.
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
 * IAKERB implementation
 */
    #[no_mangle]
    #[c2rust::src_loc = "32:1"]
    pub fn gssint_get_der_length(_: *mut *mut libc::c_uchar, _: OM_uint32,
                                 _: *mut libc::c_uint) -> libc::c_int;
}
#[c2rust::src_loc = "58:1"]
pub type iakerb_ctx_id_t = *mut iakerb_ctx_id_rec;
#[c2rust::src_loc = "57:1"]
pub type iakerb_ctx_id_rec = _iakerb_ctx_id_rec;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:8"]
pub struct _iakerb_ctx_id_rec {
    pub magic: krb5_magic,
    pub k5c: krb5_context,
    pub defcred: gss_cred_id_t,
    pub state: iakerb_state,
    pub icc: krb5_init_creds_context,
    pub tcc: krb5_tkt_creds_context,
    pub gssc: gss_ctx_id_t,
    pub conv: krb5_data,
    pub count: libc::c_uint,
    pub initiate: libc::c_int,
    pub established: libc::c_int,
    pub gic_opts: *mut krb5_get_init_creds_opt,
}
#[c2rust::src_loc = "34:1"]
pub type iakerb_state = libc::c_uint;
/* hand-off to normal GSS AP-REQ exchange */
/* acquiring ticket with TGT */
#[c2rust::src_loc = "37:5"]
pub const IAKERB_AP_REQ: iakerb_state = 2;
/* acquiring ticket with initial creds */
#[c2rust::src_loc = "36:5"]
pub const IAKERB_TGS_REQ: iakerb_state = 1;
#[c2rust::src_loc = "35:5"]
pub const IAKERB_AS_REQ: iakerb_state = 0;
/*
 * Release an IAKERB context
 */
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn iakerb_release_context(mut ctx: iakerb_ctx_id_t) {
    let mut tmp: OM_uint32 = 0;
    if ctx.is_null() { return }
    krb5_gss_release_cred(&mut tmp, &mut (*ctx).defcred);
    krb5_init_creds_free((*ctx).k5c, (*ctx).icc);
    krb5_tkt_creds_free((*ctx).k5c, (*ctx).tcc);
    krb5_gss_delete_sec_context(&mut tmp, &mut (*ctx).gssc,
                                0 as gss_buffer_t);
    krb5_free_data_contents((*ctx).k5c, &mut (*ctx).conv);
    krb5_get_init_creds_opt_free((*ctx).k5c, (*ctx).gic_opts);
    krb5_free_context((*ctx).k5c);
    free(ctx as *mut libc::c_void);
}
/*
 * Create a IAKERB-FINISHED structure containing a checksum of
 * the entire IAKERB exchange.
 */
#[no_mangle]
#[c2rust::src_loc = "85:1"]
pub unsafe extern "C" fn iakerb_make_finished(mut context: krb5_context,
                                              mut key: krb5_key,
                                              mut conv: *const krb5_data,
                                              mut finished:
                                                  *mut *mut krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut iaf: krb5_iakerb_finished =
        krb5_iakerb_finished{checksum:
                                 krb5_checksum{magic: 0,
                                               checksum_type: 0,
                                               length: 0,
                                               contents:
                                                   0 as *mut krb5_octet,},};
    *finished = 0 as *mut krb5_data;
    memset(&mut iaf as *mut krb5_iakerb_finished as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_iakerb_finished>() as libc::c_ulong);
    if key.is_null() {
        return -(1765328375 as libc::c_long) as krb5_error_code
    }
    code =
        krb5_k_make_checksum(context, 0 as libc::c_int, key,
                             42 as libc::c_int, conv, &mut iaf.checksum);
    if code != 0 as libc::c_int { return code }
    code = encode_krb5_iakerb_finished(&mut iaf, finished);
    krb5_free_checksum_contents(context, &mut iaf.checksum);
    return code;
}
/*
 * Verify a IAKERB-FINISHED structure submitted by the initiator
 */
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn iakerb_verify_finished(mut context: krb5_context,
                                                mut key: krb5_key,
                                                mut conv: *const krb5_data,
                                                mut finished:
                                                    *const krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut iaf: *mut krb5_iakerb_finished = 0 as *mut krb5_iakerb_finished;
    let mut valid: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    if key.is_null() {
        return -(1765328375 as libc::c_long) as krb5_error_code
    }
    code = decode_krb5_iakerb_finished(finished, &mut iaf);
    if code != 0 as libc::c_int { return code }
    code =
        krb5_k_verify_checksum(context, key, 42 as libc::c_int, conv,
                               &mut (*iaf).checksum, &mut valid);
    if code == 0 as libc::c_int && valid == 0 as libc::c_int as libc::c_uint {
        code = -(1765328353 as libc::c_long) as krb5_error_code
    }
    krb5_free_iakerb_finished(context, iaf);
    return code;
}
/*
 * Save a token for future checksumming.
 */
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn iakerb_save_token(mut ctx: iakerb_ctx_id_t,
                                       token: gss_buffer_t)
 -> krb5_error_code {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p =
        realloc((*ctx).conv.data as *mut libc::c_void,
                ((*ctx).conv.length as
                     libc::c_ulong).wrapping_add((*token).length)) as
            *mut libc::c_char;
    if p.is_null() { return 12 as libc::c_int }
    memcpy(p.offset((*ctx).conv.length as isize) as *mut libc::c_void,
           (*token).value, (*token).length);
    (*ctx).conv.data = p;
    (*ctx).conv.length =
        ((*ctx).conv.length as libc::c_ulong).wrapping_add((*token).length) as
            libc::c_uint as libc::c_uint;
    return 0 as libc::c_int;
}
/*
 * Parse a token into IAKERB-HEADER and KRB-KDC-REQ/REP
 */
#[c2rust::src_loc = "165:1"]
unsafe extern "C" fn iakerb_parse_token(mut ctx: iakerb_ctx_id_t,
                                        mut initialContextToken: libc::c_int,
                                        token: gss_buffer_t,
                                        mut realm: *mut krb5_data,
                                        mut cookie: *mut *mut krb5_data,
                                        mut request: *mut krb5_data)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut iah: *mut krb5_iakerb_header = 0 as *mut krb5_iakerb_header;
    let mut bodysize: libc::c_uint = 0;
    let mut lenlen: libc::c_uint = 0;
    let mut length: libc::c_int = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut data: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    if token.is_null() || (*token).length == 0 as libc::c_int as libc::c_ulong
       {
        code = -(1765328194 as libc::c_long) as krb5_error_code
    } else {
        if initialContextToken != 0 { flags |= 0x1 as libc::c_int }
        ptr = (*token).value as *mut libc::c_uchar;
        code =
            gssint_g_verify_token_header(gss_mech_iakerb as
                                             *const gss_OID_desc,
                                         &mut bodysize, &mut ptr,
                                         0x501 as libc::c_int,
                                         (*token).length as libc::c_uint,
                                         flags);
        if !(code != 0 as libc::c_int) {
            data.data = ptr as *mut libc::c_char;
            let fresh0 = bodysize;
            bodysize = bodysize.wrapping_sub(1);
            if fresh0 == 0 as libc::c_int as libc::c_uint ||
                   {
                       let fresh1 = ptr;
                       ptr = ptr.offset(1);
                       (*fresh1 as libc::c_int) != 0x30 as libc::c_int
                   } {
                /* SEQUENCE */
                code = 1859794438 as libc::c_long as krb5_error_code
            } else {
                length =
                    gssint_get_der_length(&mut ptr, bodysize, &mut lenlen);
                if length < 0 as libc::c_int ||
                       bodysize.wrapping_sub(lenlen) < length as libc::c_uint
                   {
                    code = -(1765328194 as libc::c_long) as krb5_error_code
                } else {
                    data.length =
                        (1 as libc::c_int as
                             libc::c_uint).wrapping_add(lenlen).wrapping_add(length
                                                                                 as
                                                                                 libc::c_uint);
                    ptr = ptr.offset(length as isize);
                    bodysize =
                        bodysize.wrapping_sub(lenlen.wrapping_add(length as
                                                                      libc::c_uint));
                    code = decode_krb5_iakerb_header(&mut data, &mut iah);
                    if !(code != 0 as libc::c_int) {
                        if !realm.is_null() {
                            *realm = (*iah).target_realm;
                            (*iah).target_realm.data = 0 as *mut libc::c_char
                        }
                        if !cookie.is_null() {
                            *cookie = (*iah).cookie;
                            (*iah).cookie = 0 as *mut krb5_data
                        }
                        (*request).data = ptr as *mut libc::c_char;
                        (*request).length = bodysize;
                        if (*request).data.offset((*request).length as isize)
                               ==
                               ((*token).value as
                                    *mut libc::c_char).offset((*token).length
                                                                  as isize) {
                        } else {
                            __assert_fail(b"request->data + request->length == (char *)token->value + token->length\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"iakerb.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          233 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 117],
                                                                    &[libc::c_char; 117]>(b"krb5_error_code iakerb_parse_token(iakerb_ctx_id_t, int, const gss_buffer_t, krb5_data *, krb5_data **, krb5_data *)\x00")).as_ptr());
                        }
                    }
                }
            }
        }
    }
    krb5_free_iakerb_header((*ctx).k5c, iah);
    return code;
}
/*
 * Create a token from IAKERB-HEADER and KRB-KDC-REQ/REP
 */
#[c2rust::src_loc = "244:1"]
unsafe extern "C" fn iakerb_make_token(mut ctx: iakerb_ctx_id_t,
                                       mut realm: *mut krb5_data,
                                       mut cookie: *mut krb5_data,
                                       mut request: *mut krb5_data,
                                       mut initialContextToken: libc::c_int,
                                       mut token: gss_buffer_t)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    let mut iah: krb5_iakerb_header =
        krb5_iakerb_header{target_realm:
                               krb5_data{magic: 0,
                                         length: 0,
                                         data: 0 as *mut libc::c_char,},
                           cookie: 0 as *mut krb5_data,};
    let mut data: *mut krb5_data = 0 as *mut krb5_data;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tokenSize: libc::c_uint = 0;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    (*token).value = 0 as *mut libc::c_void;
    (*token).length = 0 as libc::c_int as size_t;
    /*
     * Assemble the IAKERB-HEADER from the realm and cookie
     */
    iah.target_realm = *realm;
    iah.cookie = cookie;
    code = encode_krb5_iakerb_header(&mut iah, &mut data);
    if !(code != 0 as libc::c_int) {
        /*
     * Concatenate Kerberos request.
     */
        p =
            realloc((*data).data as *mut libc::c_void,
                    (*data).length.wrapping_add((*request).length) as
                        libc::c_ulong) as *mut libc::c_char;
        if p.is_null() {
            code = 12 as libc::c_int
        } else {
            (*data).data = p;
            if (*request).length > 0 as libc::c_int as libc::c_uint {
                memcpy((*data).data.offset((*data).length as isize) as
                           *mut libc::c_void,
                       (*request).data as *const libc::c_void,
                       (*request).length as libc::c_ulong);
            }
            (*data).length = (*data).length.wrapping_add((*request).length);
            if initialContextToken != 0 {
                tokenSize =
                    gssint_g_token_size(gss_mech_iakerb as
                                            *const gss_OID_desc,
                                        (*data).length)
            } else {
                tokenSize =
                    (2 as libc::c_int as
                         libc::c_uint).wrapping_add((*data).length)
            }
            q = gssalloc_malloc(tokenSize as size_t) as *mut libc::c_uchar;
            (*token).value = q as *mut libc::c_void;
            if q.is_null() {
                code = 12 as libc::c_int
            } else {
                (*token).length = tokenSize as size_t;
                if initialContextToken != 0 {
                    gssint_g_make_token_header(gss_mech_iakerb as
                                                   *const gss_OID_desc,
                                               (*data).length, &mut q,
                                               0x501 as libc::c_int);
                } else {
                    store_16_be(0x501 as libc::c_int as libc::c_uint,
                                q as *mut libc::c_void);
                    q = q.offset(2 as libc::c_int as isize)
                }
                memcpy(q as *mut libc::c_void,
                       (*data).data as *const libc::c_void,
                       (*data).length as libc::c_ulong);
                q = q.offset((*data).length as isize);
                if q ==
                       ((*token).value as
                            *mut libc::c_uchar).offset((*token).length as
                                                           isize) {
                } else {
                    __assert_fail(b"q == (unsigned char *)token->value + token->length\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"iakerb.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  308 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 109],
                                                            &[libc::c_char; 109]>(b"krb5_error_code iakerb_make_token(iakerb_ctx_id_t, krb5_data *, krb5_data *, krb5_data *, int, gss_buffer_t)\x00")).as_ptr());
                }
            }
        }
    }
    krb5_free_data((*ctx).k5c, data);
    return code;
}
/*
 * Parse the IAKERB token in input_token and send the contained KDC
 * request to the KDC for the realm.
 *
 * Wrap the KDC reply in output_token.
 */
#[c2rust::src_loc = "322:1"]
unsafe extern "C" fn iakerb_acceptor_step(mut ctx: iakerb_ctx_id_t,
                                          mut initialContextToken:
                                              libc::c_int,
                                          input_token: gss_buffer_t,
                                          mut output_token: gss_buffer_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut request: krb5_data = empty_data();
    let mut reply: krb5_data = empty_data();
    let mut realm: krb5_data = empty_data();
    let mut tmp: OM_uint32 = 0;
    let mut tcp_only: libc::c_int = 0;
    let mut use_master: libc::c_int = 0;
    let mut kdc_code: krb5_ui_4 = 0;
    (*output_token).length = 0 as libc::c_int as size_t;
    (*output_token).value = 0 as *mut libc::c_void;
    if (*ctx).count >= (16 as libc::c_int + 10 as libc::c_int) as libc::c_uint
       {
        code = -(1765328228 as libc::c_long) as krb5_error_code
    } else {
        code =
            iakerb_parse_token(ctx, initialContextToken, input_token,
                               &mut realm, 0 as *mut *mut krb5_data,
                               &mut request);
        if !(code != 0 as libc::c_int) {
            if realm.length == 0 as libc::c_int as libc::c_uint ||
                   request.length == 0 as libc::c_int as libc::c_uint {
                code = -(1765328194 as libc::c_long) as krb5_error_code
            } else {
                code = iakerb_save_token(ctx, input_token);
                if !(code != 0 as libc::c_int) {
                    tcp_only = 0 as libc::c_int;
                    loop  {
                        if !(tcp_only <= 1 as libc::c_int) {
                            current_block = 15345278821338558188;
                            break ;
                        }
                        use_master = 0 as libc::c_int;
                        code =
                            krb5_sendto_kdc((*ctx).k5c, &mut request,
                                            &mut realm, &mut reply,
                                            &mut use_master, tcp_only);
                        if !(code == 0 as libc::c_int &&
                                 (!(&mut reply as *mut krb5_data).is_null() &&
                                      reply.length != 0 &&
                                      *reply.data.offset(0 as libc::c_int as
                                                             isize) as
                                          libc::c_int & !(0x20 as libc::c_int)
                                          ==
                                          30 as libc::c_int |
                                              0x40 as libc::c_int)) {
                            current_block = 15345278821338558188;
                            break ;
                        }
                        let mut error: *mut krb5_error = 0 as *mut krb5_error;
                        code = decode_krb5_error(&mut reply, &mut error);
                        if code != 0 as libc::c_int {
                            current_block = 16875317906461707385;
                            break ;
                        }
                        kdc_code = (*error).error;
                        krb5_free_error((*ctx).k5c, error);
                        if !(kdc_code == 52 as libc::c_int as libc::c_uint) {
                            current_block = 15345278821338558188;
                            break ;
                        }
                        krb5_free_data_contents((*ctx).k5c, &mut reply);
                        reply = empty_data();
                        tcp_only += 1
                    }
                    match current_block {
                        16875317906461707385 => { }
                        _ => {
                            if code as libc::c_long ==
                                   -(1765328228 as libc::c_long) ||
                                   code as libc::c_long ==
                                       -(1765328230 as libc::c_long) {
                                let mut error_0: krb5_error =
                                    krb5_error{magic: 0,
                                               ctime: 0,
                                               cusec: 0,
                                               susec: 0,
                                               stime: 0,
                                               error: 0,
                                               client:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               server:
                                                   0 as
                                                       *mut krb5_principal_data,
                                               text:
                                                   krb5_data{magic: 0,
                                                             length: 0,
                                                             data:
                                                                 0 as
                                                                     *mut libc::c_char,},
                                               e_data:
                                                   krb5_data{magic: 0,
                                                             length: 0,
                                                             data:
                                                                 0 as
                                                                     *mut libc::c_char,},};
                                memset(&mut error_0 as *mut krb5_error as
                                           *mut libc::c_void,
                                       0 as libc::c_int,
                                       ::std::mem::size_of::<krb5_error>() as
                                           libc::c_ulong);
                                if code as libc::c_long ==
                                       -(1765328228 as libc::c_long) {
                                    error_0.error =
                                        86 as libc::c_int as krb5_ui_4
                                } else if code as libc::c_long ==
                                              -(1765328230 as libc::c_long) {
                                    error_0.error =
                                        85 as libc::c_int as krb5_ui_4
                                }
                                code =
                                    krb5_mk_error((*ctx).k5c, &mut error_0,
                                                  &mut reply);
                                if code != 0 as libc::c_int {
                                    current_block = 16875317906461707385;
                                } else {
                                    current_block = 14832935472441733737;
                                }
                            } else if code != 0 as libc::c_int {
                                current_block = 16875317906461707385;
                            } else { current_block = 14832935472441733737; }
                            match current_block {
                                16875317906461707385 => { }
                                _ => {
                                    code =
                                        iakerb_make_token(ctx, &mut realm,
                                                          0 as *mut krb5_data,
                                                          &mut reply,
                                                          0 as libc::c_int,
                                                          output_token);
                                    if !(code != 0 as libc::c_int) {
                                        code =
                                            iakerb_save_token(ctx,
                                                              output_token);
                                        if !(code != 0 as libc::c_int) {
                                            (*ctx).count =
                                                (*ctx).count.wrapping_add(1)
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
    if code != 0 as libc::c_int {
        gss_release_buffer(&mut tmp, output_token);
    }
    /* request is a pointer into input_token, no need to free */
    krb5_free_data_contents((*ctx).k5c, &mut realm);
    krb5_free_data_contents((*ctx).k5c, &mut reply);
    return code;
}
/*
 * Initialise the krb5_init_creds context for the IAKERB context
 */
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn iakerb_init_creds_ctx(mut ctx: iakerb_ctx_id_t,
                                           mut cred: krb5_gss_cred_id_t,
                                           mut time_req: OM_uint32)
 -> krb5_error_code {
    let mut code: krb5_error_code = 0;
    if (*cred).iakerb_mech() as libc::c_int == 0 as libc::c_int {
        code = 22 as libc::c_int
    } else {
        if !(*cred).name.is_null() {
        } else {
            __assert_fail(b"cred->name != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"iakerb.c\x00" as *const u8 as *const libc::c_char,
                          428 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 86],
                                                    &[libc::c_char; 86]>(b"krb5_error_code iakerb_init_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, OM_uint32)\x00")).as_ptr());
        }
        if !(*(*cred).name).princ.is_null() {
        } else {
            __assert_fail(b"cred->name->princ != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"iakerb.c\x00" as *const u8 as *const libc::c_char,
                          429 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 86],
                                                    &[libc::c_char; 86]>(b"krb5_error_code iakerb_init_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, OM_uint32)\x00")).as_ptr());
        }
        code =
            krb5_get_init_creds_opt_alloc((*ctx).k5c, &mut (*ctx).gic_opts);
        if !(code != 0 as libc::c_int) {
            if time_req != 0 as libc::c_int as libc::c_uint &&
                   time_req != 0xffffffff as libc::c_ulong as OM_uint32 {
                krb5_get_init_creds_opt_set_tkt_life((*ctx).gic_opts,
                                                     time_req as krb5_deltat);
            }
            code =
                krb5_get_init_creds_opt_set_out_ccache((*ctx).k5c,
                                                       (*ctx).gic_opts,
                                                       (*cred).ccache);
            if !(code != 0 as libc::c_int) {
                code =
                    krb5_init_creds_init((*ctx).k5c, (*(*cred).name).princ,
                                         None, 0 as *mut libc::c_void,
                                         0 as libc::c_int, (*ctx).gic_opts,
                                         &mut (*ctx).icc);
                if !(code != 0 as libc::c_int) {
                    if !(*cred).password.is_null() {
                        code =
                            krb5_init_creds_set_password((*ctx).k5c,
                                                         (*ctx).icc,
                                                         (*cred).password)
                    } else if !(*cred).client_keytab.is_null() {
                        code =
                            krb5_init_creds_set_keytab((*ctx).k5c, (*ctx).icc,
                                                       (*cred).client_keytab)
                    } else {
                        code =
                            -(1765328203 as libc::c_long) as krb5_error_code
                    }
                    (code) != 0 as libc::c_int;
                }
            }
        }
    }
    return code;
}
/*
 * Initialise the krb5_tkt_creds context for the IAKERB context
 */
#[c2rust::src_loc = "472:1"]
unsafe extern "C" fn iakerb_tkt_creds_ctx(mut ctx: iakerb_ctx_id_t,
                                          mut cred: krb5_gss_cred_id_t,
                                          mut name: krb5_gss_name_t,
                                          mut time_req: OM_uint32)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
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
    if !(*cred).name.is_null() {
    } else {
        __assert_fail(b"cred->name != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"iakerb.c\x00" as *const u8 as *const libc::c_char,
                      483 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 102],
                                                &[libc::c_char; 102]>(b"krb5_error_code iakerb_tkt_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, krb5_gss_name_t, OM_uint32)\x00")).as_ptr());
    }
    if !(*(*cred).name).princ.is_null() {
    } else {
        __assert_fail(b"cred->name->princ != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"iakerb.c\x00" as *const u8 as *const libc::c_char,
                      484 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 102],
                                                &[libc::c_char; 102]>(b"krb5_error_code iakerb_tkt_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, krb5_gss_name_t, OM_uint32)\x00")).as_ptr());
    }
    memset(&mut creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    creds.client = (*(*cred).name).princ;
    creds.server = (*name).princ;
    if time_req != 0 as libc::c_int as libc::c_uint &&
           time_req != 0xffffffff as libc::c_ulong as OM_uint32 {
        code = krb5_timeofday((*ctx).k5c, &mut now);
        if code != 0 as libc::c_int {
            current_block = 14846341633757719488;
        } else {
            creds.times.endtime = ts_incr(now, time_req as krb5_deltat);
            current_block = 11812396948646013369;
        }
    } else { current_block = 11812396948646013369; }
    match current_block {
        11812396948646013369 => {
            if !(*(*cred).name).ad_context.is_null() {
                code =
                    krb5_authdata_export_authdata((*ctx).k5c,
                                                  (*(*cred).name).ad_context,
                                                  0x2 as libc::c_int,
                                                  &mut creds.authdata);
                if code != 0 as libc::c_int {
                    current_block = 14846341633757719488;
                } else { current_block = 10599921512955367680; }
            } else { current_block = 10599921512955367680; }
            match current_block {
                14846341633757719488 => { }
                _ => {
                    code =
                        krb5_tkt_creds_init((*ctx).k5c, (*cred).ccache,
                                            &mut creds, 0 as libc::c_int,
                                            &mut (*ctx).tcc);
                    (code) != 0 as libc::c_int;
                }
            }
        }
        _ => { }
    }
    krb5_free_authdata((*ctx).k5c, creds.authdata);
    return code;
}
/*
 * Parse the IAKERB token in input_token and process the KDC
 * response.
 *
 * Emit the next KDC request, if any, in output_token.
 */
#[c2rust::src_loc = "524:1"]
unsafe extern "C" fn iakerb_initiator_step(mut ctx: iakerb_ctx_id_t,
                                           mut cred: krb5_gss_cred_id_t,
                                           mut name: krb5_gss_name_t,
                                           mut time_req: OM_uint32,
                                           input_token: gss_buffer_t,
                                           mut output_token: gss_buffer_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0 as libc::c_int;
    let mut in_0: krb5_data = empty_data();
    let mut out: krb5_data = empty_data();
    let mut realm: krb5_data = empty_data();
    let mut cookie: *mut krb5_data = 0 as *mut krb5_data;
    let mut tmp: OM_uint32 = 0;
    let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut times: krb5_ticket_times =
        krb5_ticket_times{authtime: 0,
                          starttime: 0,
                          endtime: 0,
                          renew_till: 0,};
    (*output_token).length = 0 as libc::c_int as size_t;
    (*output_token).value = 0 as *mut libc::c_void;
    if !input_token.is_null() {
        code =
            iakerb_parse_token(ctx, 0 as libc::c_int, input_token,
                               0 as *mut krb5_data, &mut cookie, &mut in_0);
        if code != 0 as libc::c_int {
            current_block = 3793315583556022455;
        } else {
            code = iakerb_save_token(ctx, input_token);
            if code != 0 as libc::c_int {
                current_block = 3793315583556022455;
            } else { current_block = 2868539653012386629; }
        }
    } else { current_block = 2868539653012386629; }
    match current_block {
        2868539653012386629 => {
            match (*ctx).state as libc::c_uint {
                0 => {
                    if (*ctx).icc.is_null() {
                        code = iakerb_init_creds_ctx(ctx, cred, time_req);
                        if code != 0 as libc::c_int {
                            current_block = 3793315583556022455;
                        } else { current_block = 9606288038608642794; }
                    } else { current_block = 9606288038608642794; }
                    match current_block {
                        3793315583556022455 => { }
                        _ => {
                            code =
                                krb5_init_creds_step((*ctx).k5c, (*ctx).icc,
                                                     &mut in_0, &mut out,
                                                     &mut realm, &mut flags);
                            if code != 0 as libc::c_int {
                                if (*cred).have_tgt != 0 {
                                    /* We were trying to refresh; keep going with current creds. */
                                    (*ctx).state = IAKERB_TGS_REQ;
                                    krb5_clear_error_message((*ctx).k5c);
                                    current_block = 4775909272756257391;
                                } else {
                                    current_block = 3793315583556022455;
                                }
                            } else if flags &
                                          0x1 as libc::c_int as libc::c_uint
                                          == 0 {
                                krb5_init_creds_get_times((*ctx).k5c,
                                                          (*ctx).icc,
                                                          &mut times);
                                kg_cred_set_initial_refresh((*ctx).k5c, cred,
                                                            &mut times);
                                (*cred).expire = times.endtime;
                                krb5_init_creds_free((*ctx).k5c, (*ctx).icc);
                                (*ctx).icc = 0 as krb5_init_creds_context;
                                (*ctx).state = IAKERB_TGS_REQ;
                                current_block = 4775909272756257391;
                            } else { current_block = 12997042908615822766; }
                            match current_block {
                                3793315583556022455 => { }
                                12997042908615822766 => { }
                                _ => {
                                    in_0 = empty_data();
                                    current_block = 14728543630564123267;
                                }
                            }
                        }
                    }
                }
                1 => { current_block = 14728543630564123267; }
                2 | _ => { current_block = 12997042908615822766; }
            }
            match current_block {
                3793315583556022455 => { }
                _ => {
                    match current_block {
                        14728543630564123267 =>
                        /* Done with AS request; fall through to TGS request. */
                        {
                            if (*ctx).tcc.is_null() {
                                code =
                                    iakerb_tkt_creds_ctx(ctx, cred, name,
                                                         time_req);
                                if code != 0 as libc::c_int {
                                    current_block = 3793315583556022455;
                                } else {
                                    current_block = 14434620278749266018;
                                }
                            } else { current_block = 14434620278749266018; }
                            match current_block {
                                3793315583556022455 => { }
                                _ => {
                                    code =
                                        krb5_tkt_creds_step((*ctx).k5c,
                                                            (*ctx).tcc,
                                                            &mut in_0,
                                                            &mut out,
                                                            &mut realm,
                                                            &mut flags);
                                    if code != 0 as libc::c_int {
                                        current_block = 3793315583556022455;
                                    } else if flags &
                                                  0x1 as libc::c_int as
                                                      libc::c_uint == 0 {
                                        krb5_tkt_creds_get_times((*ctx).k5c,
                                                                 (*ctx).tcc,
                                                                 &mut times);
                                        (*cred).expire = times.endtime;
                                        krb5_tkt_creds_free((*ctx).k5c,
                                                            (*ctx).tcc);
                                        (*ctx).tcc =
                                            0 as krb5_tkt_creds_context;
                                        (*ctx).state = IAKERB_AP_REQ;
                                        current_block = 12997042908615822766;
                                    } else {
                                        current_block = 12997042908615822766;
                                    }
                                }
                            }
                        }
                        _ => { }
                    }
                    match current_block {
                        3793315583556022455 => { }
                        _ =>
                        /* Done with TGS request; fall through to AP request. */
                        {
                            if out.length != 0 as libc::c_int as libc::c_uint
                               {
                                if (*ctx).state as libc::c_uint !=
                                       IAKERB_AP_REQ as libc::c_int as
                                           libc::c_uint {
                                } else {
                                    __assert_fail(b"ctx->state != IAKERB_AP_REQ\x00"
                                                      as *const u8 as
                                                      *const libc::c_char,
                                                  b"iakerb.c\x00" as *const u8
                                                      as *const libc::c_char,
                                                  610 as libc::c_int as
                                                      libc::c_uint,
                                                  (*::std::mem::transmute::<&[u8; 137],
                                                                            &[libc::c_char; 137]>(b"krb5_error_code iakerb_initiator_step(iakerb_ctx_id_t, krb5_gss_cred_id_t, krb5_gss_name_t, OM_uint32, const gss_buffer_t, gss_buffer_t)\x00")).as_ptr());
                                }
                                code =
                                    iakerb_make_token(ctx, &mut realm, cookie,
                                                      &mut out,
                                                      (input_token ==
                                                           0 as gss_buffer_t)
                                                          as libc::c_int,
                                                      output_token);
                                if !(code != 0 as libc::c_int) {
                                    /* Save the token for generating a future checksum */
                                    code =
                                        iakerb_save_token(ctx, output_token);
                                    if !(code != 0 as libc::c_int) {
                                        (*ctx).count =
                                            (*ctx).count.wrapping_add(1)
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
    if code != 0 as libc::c_int {
        gss_release_buffer(&mut tmp, output_token);
    }
    krb5_free_data((*ctx).k5c, cookie);
    krb5_free_data_contents((*ctx).k5c, &mut out);
    krb5_free_data_contents((*ctx).k5c, &mut realm);
    return code;
}
/*
 * Determine the starting IAKERB state for a context. If we already
 * have a ticket, we may not need to do IAKERB at all.
 */
#[c2rust::src_loc = "640:1"]
unsafe extern "C" fn iakerb_get_initial_state(mut ctx: iakerb_ctx_id_t,
                                              mut cred: krb5_gss_cred_id_t,
                                              mut target: krb5_gss_name_t,
                                              mut time_req: OM_uint32,
                                              mut state: *mut iakerb_state)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut in_creds: krb5_creds =
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
    let mut out_creds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut code: krb5_error_code = 0;
    memset(&mut in_creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    in_creds.client = (*(*cred).name).princ;
    in_creds.server = (*target).princ;
    if !(*(*cred).name).ad_context.is_null() {
        code =
            krb5_authdata_export_authdata((*ctx).k5c,
                                          (*(*cred).name).ad_context,
                                          0x2 as libc::c_int,
                                          &mut in_creds.authdata);
        if code != 0 as libc::c_int {
            current_block = 11666045669424323853;
        } else { current_block = 10879442775620481940; }
    } else { current_block = 10879442775620481940; }
    match current_block {
        10879442775620481940 => {
            if time_req != 0 as libc::c_int as libc::c_uint &&
                   time_req != 0xffffffff as libc::c_ulong as OM_uint32 {
                let mut now: krb5_timestamp = 0;
                code = krb5_timeofday((*ctx).k5c, &mut now);
                if code != 0 as libc::c_int {
                    current_block = 11666045669424323853;
                } else {
                    in_creds.times.endtime =
                        ts_incr(now, time_req as krb5_deltat);
                    current_block = 7651349459974463963;
                }
            } else { current_block = 7651349459974463963; }
            match current_block {
                11666045669424323853 => { }
                _ =>
                /* Make an AS request if we have no creds or it's time to refresh them. */
                {
                    if (*cred).expire == 0 as libc::c_int ||
                           kg_cred_time_to_refresh((*ctx).k5c, cred) != 0 {
                        *state = IAKERB_AS_REQ;
                        code = 0 as libc::c_int
                    } else {
                        code =
                            krb5_get_credentials((*ctx).k5c, 2 as libc::c_int,
                                                 (*cred).ccache,
                                                 &mut in_creds,
                                                 &mut out_creds);
                        if code as libc::c_long ==
                               -(1765328243 as libc::c_long) ||
                               code as libc::c_long ==
                                   -(1765328184 as libc::c_long) {
                            *state =
                                if (*cred).have_tgt != 0 {
                                    IAKERB_TGS_REQ as libc::c_int
                                } else { IAKERB_AS_REQ as libc::c_int } as
                                    iakerb_state;
                            code = 0 as libc::c_int
                        } else if code == 0 as libc::c_int {
                            *state = IAKERB_AP_REQ;
                            krb5_free_creds((*ctx).k5c, out_creds);
                        }
                    }
                }
            }
        }
        _ => { }
    }
    krb5_free_authdata((*ctx).k5c, in_creds.authdata);
    return code;
}
/*
 * Allocate and initialise an IAKERB context
 */
#[c2rust::src_loc = "700:1"]
unsafe extern "C" fn iakerb_alloc_context(mut pctx: *mut iakerb_ctx_id_t,
                                          mut initiate: libc::c_int)
 -> krb5_error_code {
    let mut ctx: iakerb_ctx_id_t = 0 as *mut iakerb_ctx_id_rec;
    let mut code: krb5_error_code = 0;
    *pctx = 0 as iakerb_ctx_id_t;
    ctx =
        k5alloc(::std::mem::size_of::<iakerb_ctx_id_rec>() as libc::c_ulong,
                &mut code) as iakerb_ctx_id_t;
    if !ctx.is_null() {
        (*ctx).defcred = 0 as gss_cred_id_t;
        (*ctx).magic = 39756048 as libc::c_long as krb5_magic;
        (*ctx).state = IAKERB_AS_REQ;
        (*ctx).count = 0 as libc::c_int as libc::c_uint;
        (*ctx).initiate = initiate;
        (*ctx).established = 0 as libc::c_int;
        code = krb5_gss_init_context(&mut (*ctx).k5c);
        if !(code != 0 as libc::c_int) { *pctx = ctx }
    }
    if code != 0 as libc::c_int { iakerb_release_context(ctx); }
    return code;
}
#[no_mangle]
#[c2rust::src_loc = "731:1"]
pub unsafe extern "C" fn iakerb_gss_delete_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut output_token:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut iakerb_ctx: iakerb_ctx_id_t = *context_handle as iakerb_ctx_id_t;
    if !output_token.is_null() {
        (*output_token).length = 0 as libc::c_int as size_t;
        (*output_token).value = 0 as *mut libc::c_void
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    *context_handle = 0 as gss_ctx_id_t;
    iakerb_release_context(iakerb_ctx);
    return 0 as libc::c_int as OM_uint32;
}
#[c2rust::src_loc = "750:1"]
unsafe extern "C" fn iakerb_is_iakerb_token(token: gss_buffer_t)
 -> krb5_boolean {
    let mut code: krb5_error_code = 0;
    let mut bodysize: libc::c_uint = (*token).length as libc::c_uint;
    let mut ptr: *mut libc::c_uchar = (*token).value as *mut libc::c_uchar;
    code =
        gssint_g_verify_token_header(gss_mech_iakerb as *const gss_OID_desc,
                                     &mut bodysize, &mut ptr,
                                     0x501 as libc::c_int,
                                     (*token).length as libc::c_uint,
                                     0 as libc::c_int);
    return (code == 0 as libc::c_int) as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "765:1"]
unsafe extern "C" fn iakerb_make_exts(mut ctx: iakerb_ctx_id_t,
                                      mut exts: *mut krb5_gss_ctx_ext_rec) {
    memset(exts as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_gss_ctx_ext_rec>() as libc::c_ulong);
    if (*ctx).conv.length != 0 as libc::c_int as libc::c_uint {
        (*exts).iakerb.conv = &mut (*ctx).conv
    };
}
#[no_mangle]
#[c2rust::src_loc = "774:1"]
pub unsafe extern "C" fn iakerb_gss_accept_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut verifier_cred_handle:
                                                           gss_cred_id_t,
                                                       mut input_token:
                                                           gss_buffer_t,
                                                       mut input_chan_bindings:
                                                           gss_channel_bindings_t,
                                                       mut src_name:
                                                           *mut gss_name_t,
                                                       mut mech_type:
                                                           *mut gss_OID,
                                                       mut output_token:
                                                           gss_buffer_t,
                                                       mut ret_flags:
                                                           *mut OM_uint32,
                                                       mut time_rec:
                                                           *mut OM_uint32,
                                                       mut delegated_cred_handle:
                                                           *mut gss_cred_id_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut major_status: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut code: OM_uint32 = 0;
    let mut ctx: iakerb_ctx_id_t = 0 as *mut iakerb_ctx_id_rec;
    let mut initialContextToken: libc::c_int =
        (*context_handle == 0 as gss_ctx_id_t) as libc::c_int;
    if initialContextToken != 0 {
        code = iakerb_alloc_context(&mut ctx, 0 as libc::c_int) as OM_uint32;
        if code != 0 as libc::c_int as libc::c_uint {
            current_block = 10280779239138868270;
        } else { current_block = 2473556513754201174; }
    } else {
        ctx = *context_handle as iakerb_ctx_id_t;
        current_block = 2473556513754201174;
    }
    match current_block {
        2473556513754201174 => {
            if iakerb_is_iakerb_token(input_token) != 0 {
                if !(*ctx).gssc.is_null() {
                    /* We shouldn't get an IAKERB token now. */
                    code = -(2045022960 as libc::c_long) as OM_uint32;
                    major_status =
                        (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                } else {
                    code =
                        iakerb_acceptor_step(ctx, initialContextToken,
                                             input_token, output_token) as
                            OM_uint32;
                    if code == -(1765328194 as libc::c_long) as OM_uint32 {
                        major_status =
                            (9 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int
                    }
                    if !(code != 0 as libc::c_int as libc::c_uint) {
                        if initialContextToken != 0 {
                            *context_handle = ctx as gss_ctx_id_t;
                            ctx = 0 as iakerb_ctx_id_t
                        }
                        if !src_name.is_null() { *src_name = 0 as gss_name_t }
                        if !mech_type.is_null() {
                            *mech_type = gss_mech_iakerb
                        }
                        if !ret_flags.is_null() {
                            *ret_flags = 0 as libc::c_int as OM_uint32
                        }
                        if !time_rec.is_null() {
                            *time_rec = 0 as libc::c_int as OM_uint32
                        }
                        if !delegated_cred_handle.is_null() {
                            *delegated_cred_handle = 0 as gss_cred_id_t
                        }
                        major_status =
                            ((1 as libc::c_int) <<
                                 0 as libc::c_int + 0 as libc::c_int) as
                                OM_uint32
                    }
                }
            } else {
                let mut exts: krb5_gss_ctx_ext_rec =
                    krb5_gss_ctx_ext_rec{iakerb:
                                             C2RustUnnamed_0{conv:
                                                                 0 as
                                                                     *mut krb5_data,
                                                             verified: 0,},};
                iakerb_make_exts(ctx, &mut exts);
                major_status =
                    krb5_gss_accept_sec_context_ext(&mut code,
                                                    &mut (*ctx).gssc,
                                                    verifier_cred_handle,
                                                    input_token,
                                                    input_chan_bindings,
                                                    src_name,
                                                    0 as *mut gss_OID,
                                                    output_token, ret_flags,
                                                    time_rec,
                                                    delegated_cred_handle,
                                                    &mut exts);
                if major_status == 0 as libc::c_int as libc::c_uint {
                    (*ctx).established = 1 as libc::c_int
                }
                if !mech_type.is_null() { *mech_type = gss_mech_krb5 }
            }
        }
        _ => { }
    }
    if initialContextToken != 0 &&
           major_status &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
        iakerb_release_context(ctx);
        *context_handle = 0 as gss_ctx_id_t
    }
    *minor_status = code;
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "861:1"]
pub unsafe extern "C" fn iakerb_gss_init_sec_context(mut minor_status:
                                                         *mut OM_uint32,
                                                     mut claimant_cred_handle:
                                                         gss_cred_id_t,
                                                     mut context_handle:
                                                         *mut gss_ctx_id_t,
                                                     mut target_name:
                                                         gss_name_t,
                                                     mut mech_type: gss_OID,
                                                     mut req_flags: OM_uint32,
                                                     mut time_req: OM_uint32,
                                                     mut input_chan_bindings:
                                                         gss_channel_bindings_t,
                                                     mut input_token:
                                                         gss_buffer_t,
                                                     mut actual_mech_type:
                                                         *mut gss_OID,
                                                     mut output_token:
                                                         gss_buffer_t,
                                                     mut ret_flags:
                                                         *mut OM_uint32,
                                                     mut time_rec:
                                                         *mut OM_uint32)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut major_status: OM_uint32 =
        (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    let mut code: krb5_error_code = 0;
    let mut ctx: iakerb_ctx_id_t = 0 as *mut iakerb_ctx_id_rec;
    let mut kcred: krb5_gss_cred_id_t = 0 as *mut _krb5_gss_cred_id_rec;
    let mut kname: krb5_gss_name_t = 0 as *mut _krb5_gss_name_rec;
    let mut cred_locked: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut initialContextToken: libc::c_int =
        (*context_handle == 0 as gss_ctx_id_t) as libc::c_int;
    if initialContextToken != 0 {
        code = iakerb_alloc_context(&mut ctx, 1 as libc::c_int);
        if code != 0 as libc::c_int {
            *minor_status = code as OM_uint32;
            current_block = 12791752257761306173;
        } else if claimant_cred_handle.is_null() {
            major_status =
                iakerb_gss_acquire_cred(minor_status, 0 as gss_name_t,
                                        0xffffffff as libc::c_ulong as
                                            OM_uint32, 0 as gss_OID_set,
                                        1 as libc::c_int, &mut (*ctx).defcred,
                                        0 as *mut gss_OID_set,
                                        0 as *mut OM_uint32);
            if major_status &
                   ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
                        |
                        (0o377 as libc::c_ulong as OM_uint32) <<
                            16 as libc::c_int) != 0 {
                current_block = 12791752257761306173;
            } else {
                claimant_cred_handle = (*ctx).defcred;
                current_block = 15976848397966268834;
            }
        } else { current_block = 15976848397966268834; }
    } else {
        ctx = *context_handle as iakerb_ctx_id_t;
        if claimant_cred_handle.is_null() {
            claimant_cred_handle = (*ctx).defcred
        }
        current_block = 15976848397966268834;
    }
    match current_block {
        15976848397966268834 => {
            kname = target_name as krb5_gss_name_t;
            major_status =
                kg_cred_resolve(minor_status, (*ctx).k5c,
                                claimant_cred_handle, target_name);
            if !(major_status &
                     ((0o377 as libc::c_ulong as OM_uint32) <<
                          24 as libc::c_int |
                          (0o377 as libc::c_ulong as OM_uint32) <<
                              16 as libc::c_int) != 0) {
                cred_locked = 1 as libc::c_int as krb5_boolean;
                kcred = claimant_cred_handle as krb5_gss_cred_id_t;
                major_status =
                    (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
                if initialContextToken != 0 {
                    code =
                        iakerb_get_initial_state(ctx, kcred, kname, time_req,
                                                 &mut (*ctx).state);
                    if code != 0 as libc::c_int {
                        *minor_status = code as OM_uint32;
                        current_block = 12791752257761306173;
                    } else {
                        *context_handle = ctx as gss_ctx_id_t;
                        current_block = 15345278821338558188;
                    }
                } else { current_block = 15345278821338558188; }
                match current_block {
                    12791752257761306173 => { }
                    _ => {
                        if (*ctx).state as libc::c_uint !=
                               IAKERB_AP_REQ as libc::c_int as libc::c_uint {
                            /* We need to do IAKERB. */
                            code =
                                iakerb_initiator_step(ctx, kcred, kname,
                                                      time_req, input_token,
                                                      output_token);
                            if code as libc::c_long ==
                                   -(1765328194 as libc::c_long) {
                                major_status =
                                    (9 as libc::c_ulong as OM_uint32) <<
                                        16 as libc::c_int
                            }
                            if code != 0 as libc::c_int {
                                *minor_status = code as OM_uint32;
                                current_block = 12791752257761306173;
                            } else { current_block = 9520865839495247062; }
                        } else { current_block = 9520865839495247062; }
                        match current_block {
                            12791752257761306173 => { }
                            _ => {
                                if (*ctx).state as libc::c_uint ==
                                       IAKERB_AP_REQ as libc::c_int as
                                           libc::c_uint {
                                    let mut exts: krb5_gss_ctx_ext_rec =
                                        krb5_gss_ctx_ext_rec{iakerb:
                                                                 C2RustUnnamed_0{conv:
                                                                                     0
                                                                                         as
                                                                                         *mut krb5_data,
                                                                                 verified:
                                                                                     0,},};
                                    if cred_locked != 0 {
                                        k5_mutex_unlock(&mut (*kcred).lock);
                                        cred_locked =
                                            0 as libc::c_int as krb5_boolean
                                    }
                                    iakerb_make_exts(ctx, &mut exts);
                                    if (*ctx).gssc.is_null() {
                                        input_token = 0 as gss_buffer_t
                                    }
                                    /* IAKERB is finished, or we skipped to Kerberos directly. */
                                    major_status =
                                        krb5_gss_init_sec_context_ext(minor_status,
                                                                      kcred as
                                                                          gss_cred_id_t,
                                                                      &mut (*ctx).gssc,
                                                                      target_name,
                                                                      gss_mech_iakerb,
                                                                      req_flags,
                                                                      time_req,
                                                                      input_chan_bindings,
                                                                      input_token,
                                                                      0 as
                                                                          *mut gss_OID,
                                                                      output_token,
                                                                      ret_flags,
                                                                      time_rec,
                                                                      &mut exts);
                                    if major_status ==
                                           0 as libc::c_int as libc::c_uint {
                                        (*ctx).established = 1 as libc::c_int
                                    }
                                    if !actual_mech_type.is_null() {
                                        *actual_mech_type = gss_mech_krb5
                                    }
                                } else {
                                    if !actual_mech_type.is_null() {
                                        *actual_mech_type = gss_mech_iakerb
                                    }
                                    if !ret_flags.is_null() {
                                        *ret_flags =
                                            0 as libc::c_int as OM_uint32
                                    }
                                    if !time_rec.is_null() {
                                        *time_rec =
                                            0 as libc::c_int as OM_uint32
                                    }
                                    major_status =
                                        ((1 as libc::c_int) <<
                                             0 as libc::c_int +
                                                 0 as libc::c_int) as
                                            OM_uint32
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if cred_locked != 0 { k5_mutex_unlock(&mut (*kcred).lock); }
    if initialContextToken != 0 &&
           major_status &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
        iakerb_release_context(ctx);
        *context_handle = 0 as gss_ctx_id_t
    }
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "996:1"]
pub unsafe extern "C" fn iakerb_gss_unwrap(mut minor_status: *mut OM_uint32,
                                           mut context_handle: gss_ctx_id_t,
                                           mut input_message_buffer:
                                               gss_buffer_t,
                                           mut output_message_buffer:
                                               gss_buffer_t,
                                           mut conf_state: *mut libc::c_int,
                                           mut qop_state: *mut gss_qop_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_unwrap(minor_status, (*ctx).gssc, input_message_buffer,
                           output_message_buffer, conf_state, qop_state);
}
#[no_mangle]
#[c2rust::src_loc = "1011:1"]
pub unsafe extern "C" fn iakerb_gss_wrap(mut minor_status: *mut OM_uint32,
                                         mut context_handle: gss_ctx_id_t,
                                         mut conf_req_flag: libc::c_int,
                                         mut qop_req: gss_qop_t,
                                         mut input_message_buffer:
                                             gss_buffer_t,
                                         mut conf_state: *mut libc::c_int,
                                         mut output_message_buffer:
                                             gss_buffer_t) -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_wrap(minor_status, (*ctx).gssc, conf_req_flag, qop_req,
                         input_message_buffer, conf_state,
                         output_message_buffer);
}
#[no_mangle]
#[c2rust::src_loc = "1027:1"]
pub unsafe extern "C" fn iakerb_gss_process_context_token(mut minor_status:
                                                              *mut OM_uint32,
                                                          context_handle:
                                                              gss_ctx_id_t,
                                                          token_buffer:
                                                              gss_buffer_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_process_context_token(minor_status, (*ctx).gssc,
                                          token_buffer);
}
#[no_mangle]
#[c2rust::src_loc = "1041:1"]
pub unsafe extern "C" fn iakerb_gss_context_time(mut minor_status:
                                                     *mut OM_uint32,
                                                 mut context_handle:
                                                     gss_ctx_id_t,
                                                 mut time_rec: *mut OM_uint32)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_context_time(minor_status, (*ctx).gssc, time_rec);
}
#[no_mangle]
#[c2rust::src_loc = "1055:1"]
pub unsafe extern "C" fn iakerb_gss_export_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut interprocess_token:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut maj: OM_uint32 = 0;
    let mut ctx: iakerb_ctx_id_t = *context_handle as iakerb_ctx_id_t;
    /* We don't currently support exporting partially established contexts. */
    if (*ctx).established == 0 {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    maj =
        krb5_gss_export_sec_context(minor_status, &mut (*ctx).gssc,
                                    interprocess_token);
    if (*ctx).gssc.is_null() {
        iakerb_release_context(ctx);
        *context_handle = 0 as gss_ctx_id_t
    }
    return maj;
}
#[no_mangle]
#[c2rust::src_loc = "1076:1"]
pub unsafe extern "C" fn iakerb_gss_import_sec_context(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut interprocess_token:
                                                           gss_buffer_t,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t)
 -> OM_uint32 {
    let mut maj: OM_uint32 = 0;
    let mut tmpmin: OM_uint32 = 0;
    let mut code: krb5_error_code = 0;
    let mut gssc: gss_ctx_id_t = 0 as *mut gss_ctx_id_struct;
    let mut kctx: krb5_gss_ctx_id_t = 0 as *mut _krb5_gss_ctx_id_rec;
    let mut ctx: iakerb_ctx_id_t = 0 as *mut iakerb_ctx_id_rec;
    maj =
        krb5_gss_import_sec_context(minor_status, interprocess_token,
                                    &mut gssc);
    if maj != 0 as libc::c_int as libc::c_uint { return maj }
    kctx = gssc as krb5_gss_ctx_id_t;
    if (*kctx).established() == 0 {
        /* We don't currently support importing partially established
         * contexts. */
        krb5_gss_delete_sec_context(&mut tmpmin, &mut gssc,
                                    0 as gss_buffer_t);
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    code = iakerb_alloc_context(&mut ctx, (*kctx).initiate() as libc::c_int);
    if code != 0 as libc::c_int {
        krb5_gss_delete_sec_context(&mut tmpmin, &mut gssc,
                                    0 as gss_buffer_t);
        *minor_status = code as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    (*ctx).gssc = gssc;
    (*ctx).established = 1 as libc::c_int;
    *context_handle = ctx as gss_ctx_id_t;
    return 0 as libc::c_int as OM_uint32;
}
/* LEAN_CLIENT */
#[no_mangle]
#[c2rust::src_loc = "1113:1"]
pub unsafe extern "C" fn iakerb_gss_inquire_context(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut context_handle:
                                                        gss_ctx_id_t,
                                                    mut src_name:
                                                        *mut gss_name_t,
                                                    mut targ_name:
                                                        *mut gss_name_t,
                                                    mut lifetime_rec:
                                                        *mut OM_uint32,
                                                    mut mech_type:
                                                        *mut gss_OID,
                                                    mut ctx_flags:
                                                        *mut OM_uint32,
                                                    mut initiate:
                                                        *mut libc::c_int,
                                                    mut opened:
                                                        *mut libc::c_int)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if !src_name.is_null() { *src_name = 0 as gss_name_t }
    if !targ_name.is_null() { *targ_name = 0 as gss_name_t }
    if !lifetime_rec.is_null() {
        *lifetime_rec = 0 as libc::c_int as OM_uint32
    }
    if !mech_type.is_null() { *mech_type = gss_mech_iakerb }
    if !ctx_flags.is_null() { *ctx_flags = 0 as libc::c_int as OM_uint32 }
    if !initiate.is_null() { *initiate = (*ctx).initiate }
    if !opened.is_null() { *opened = (*ctx).established }
    if (*ctx).gssc.is_null() { return 0 as libc::c_int as OM_uint32 }
    ret =
        krb5_gss_inquire_context(minor_status, (*ctx).gssc, src_name,
                                 targ_name, lifetime_rec, mech_type,
                                 ctx_flags, initiate, opened);
    if (*ctx).established == 0 {
        /* Report IAKERB as the mech OID until the context is established. */
        if !mech_type.is_null() { *mech_type = gss_mech_iakerb }
        /* We don't support exporting partially-established contexts. */
        if !ctx_flags.is_null() {
            *ctx_flags &= !(256 as libc::c_int) as libc::c_uint
        }
    }
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "1158:1"]
pub unsafe extern "C" fn iakerb_gss_wrap_size_limit(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut context_handle:
                                                        gss_ctx_id_t,
                                                    mut conf_req_flag:
                                                        libc::c_int,
                                                    mut qop_req: gss_qop_t,
                                                    mut req_output_size:
                                                        OM_uint32,
                                                    mut max_input_size:
                                                        *mut OM_uint32)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_wrap_size_limit(minor_status, (*ctx).gssc, conf_req_flag,
                                    qop_req, req_output_size, max_input_size);
}
#[no_mangle]
#[c2rust::src_loc = "1173:1"]
pub unsafe extern "C" fn iakerb_gss_get_mic(mut minor_status: *mut OM_uint32,
                                            mut context_handle: gss_ctx_id_t,
                                            mut qop_req: gss_qop_t,
                                            mut message_buffer: gss_buffer_t,
                                            mut message_token: gss_buffer_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_get_mic(minor_status, (*ctx).gssc, qop_req,
                            message_buffer, message_token);
}
#[no_mangle]
#[c2rust::src_loc = "1187:1"]
pub unsafe extern "C" fn iakerb_gss_verify_mic(mut minor_status:
                                                   *mut OM_uint32,
                                               mut context_handle:
                                                   gss_ctx_id_t,
                                               mut msg_buffer: gss_buffer_t,
                                               mut token_buffer: gss_buffer_t,
                                               mut qop_state: *mut gss_qop_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_verify_mic(minor_status, (*ctx).gssc, msg_buffer,
                               token_buffer, qop_state);
}
#[no_mangle]
#[c2rust::src_loc = "1201:1"]
pub unsafe extern "C" fn iakerb_gss_inquire_sec_context_by_oid(mut minor_status:
                                                                   *mut OM_uint32,
                                                               context_handle:
                                                                   gss_ctx_id_t,
                                                               desired_object:
                                                                   gss_OID,
                                                               mut data_set:
                                                                   *mut gss_buffer_set_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_inquire_sec_context_by_oid(minor_status, (*ctx).gssc,
                                               desired_object, data_set);
}
#[no_mangle]
#[c2rust::src_loc = "1216:1"]
pub unsafe extern "C" fn iakerb_gss_set_sec_context_option(mut minor_status:
                                                               *mut OM_uint32,
                                                           mut context_handle:
                                                               *mut gss_ctx_id_t,
                                                           desired_object:
                                                               gss_OID,
                                                           value:
                                                               gss_buffer_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = *context_handle as iakerb_ctx_id_t;
    if ctx.is_null() || (*ctx).gssc.is_null() {
        return (16 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_set_sec_context_option(minor_status, &mut (*ctx).gssc,
                                           desired_object, value);
}
#[no_mangle]
#[c2rust::src_loc = "1231:1"]
pub unsafe extern "C" fn iakerb_gss_wrap_iov(mut minor_status: *mut OM_uint32,
                                             mut context_handle: gss_ctx_id_t,
                                             mut conf_req_flag: libc::c_int,
                                             mut qop_req: gss_qop_t,
                                             mut conf_state: *mut libc::c_int,
                                             mut iov:
                                                 *mut gss_iov_buffer_desc,
                                             mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_wrap_iov(minor_status, (*ctx).gssc, conf_req_flag,
                             qop_req, conf_state, iov, iov_count);
}
#[no_mangle]
#[c2rust::src_loc = "1245:1"]
pub unsafe extern "C" fn iakerb_gss_unwrap_iov(mut minor_status:
                                                   *mut OM_uint32,
                                               mut context_handle:
                                                   gss_ctx_id_t,
                                               mut conf_state:
                                                   *mut libc::c_int,
                                               mut qop_state: *mut gss_qop_t,
                                               mut iov:
                                                   *mut gss_iov_buffer_desc,
                                               mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_unwrap_iov(minor_status, (*ctx).gssc, conf_state,
                               qop_state, iov, iov_count);
}
#[no_mangle]
#[c2rust::src_loc = "1259:1"]
pub unsafe extern "C" fn iakerb_gss_wrap_iov_length(mut minor_status:
                                                        *mut OM_uint32,
                                                    mut context_handle:
                                                        gss_ctx_id_t,
                                                    mut conf_req_flag:
                                                        libc::c_int,
                                                    mut qop_req: gss_qop_t,
                                                    mut conf_state:
                                                        *mut libc::c_int,
                                                    mut iov:
                                                        *mut gss_iov_buffer_desc,
                                                    mut iov_count:
                                                        libc::c_int)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_wrap_iov_length(minor_status, (*ctx).gssc, conf_req_flag,
                                    qop_req, conf_state, iov, iov_count);
}
/* LEAN_CLIENT */
#[no_mangle]
#[c2rust::src_loc = "1274:1"]
pub unsafe extern "C" fn iakerb_gss_pseudo_random(mut minor_status:
                                                      *mut OM_uint32,
                                                  mut context_handle:
                                                      gss_ctx_id_t,
                                                  mut prf_key: libc::c_int,
                                                  prf_in: gss_buffer_t,
                                                  mut desired_output_len:
                                                      ssize_t,
                                                  mut prf_out: gss_buffer_t)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_pseudo_random(minor_status, (*ctx).gssc, prf_key, prf_in,
                                  desired_output_len, prf_out);
}
#[no_mangle]
#[c2rust::src_loc = "1288:1"]
pub unsafe extern "C" fn iakerb_gss_get_mic_iov(mut minor_status:
                                                    *mut OM_uint32,
                                                mut context_handle:
                                                    gss_ctx_id_t,
                                                mut qop_req: gss_qop_t,
                                                mut iov:
                                                    *mut gss_iov_buffer_desc,
                                                mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_get_mic_iov(minor_status, (*ctx).gssc, qop_req, iov,
                                iov_count);
}
#[no_mangle]
#[c2rust::src_loc = "1302:1"]
pub unsafe extern "C" fn iakerb_gss_verify_mic_iov(mut minor_status:
                                                       *mut OM_uint32,
                                                   mut context_handle:
                                                       gss_ctx_id_t,
                                                   mut qop_state:
                                                       *mut gss_qop_t,
                                                   mut iov:
                                                       *mut gss_iov_buffer_desc,
                                                   mut iov_count: libc::c_int)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_verify_mic_iov(minor_status, (*ctx).gssc, qop_state, iov,
                                   iov_count);
}
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
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
#[no_mangle]
#[c2rust::src_loc = "1316:1"]
pub unsafe extern "C" fn iakerb_gss_get_mic_iov_length(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut context_handle:
                                                           gss_ctx_id_t,
                                                       mut qop_req: gss_qop_t,
                                                       mut iov:
                                                           *mut gss_iov_buffer_desc,
                                                       mut iov_count:
                                                           libc::c_int)
 -> OM_uint32 {
    let mut ctx: iakerb_ctx_id_t = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    return krb5_gss_get_mic_iov_length(minor_status, (*ctx).gssc, qop_req,
                                       iov, iov_count);
}
