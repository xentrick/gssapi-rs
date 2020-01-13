use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:100"]
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
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:100"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:100"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:100"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:100"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:100"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:100"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:100"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:100"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:100"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "561:5"]
    pub struct C2RustUnnamed {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "644:5"]
    pub struct C2RustUnnamed_1 {
        pub i: uint16_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed_2 {
        pub i: uint32_t,
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
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "639:1"]
    pub unsafe extern "C" fn store_16_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_1)).i = val as uint16_t;
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_2)).i = val;
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t};
    use super::byteswap_h::{__bswap_16, __bswap_32};
    use super::types_h::__uint16_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:100"]
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
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2140:16"]
    pub struct _krb5_ap_rep_enc_part {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub subkey: *mut krb5_keyblock,
        pub seq_number: krb5_ui_4,
    }
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    #[c2rust::src_loc = "2140:1"]
    pub type krb5_ap_rep_enc_part = _krb5_ap_rep_enc_part;
    /* *< Client time, seconds portion */
    /* *< Client time, microseconds portion */
    /* *< Subkey (optional) */
    /* *< Sequence number */
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::{_krb5_context, krb5_key_st, _krb5_kt};
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
        /* *
 * Retrieve a specified credentials from a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [in]  flags           Flags bit mask
 * @param [in]  mcreds          Credentials to match
 * @param [out] creds           Credentials matching the requested value
 *
 * This function searches a credential cache for credentials matching @a mcreds
 * and returns it if found.
 *
 * Valid values for @a flags are:
 *
 * @li #KRB5_TC_MATCH_TIMES        The requested lifetime must be at least as
 *                                 great as in @a mcreds .
 * @li #KRB5_TC_MATCH_IS_SKEY      The @a is_skey field much match exactly.
 * @li #KRB5_TC_MATCH_FLAGS        Flags set in @a mcreds must be set.
 * @li #KRB5_TC_MATCH_TIMES_EXACT  The requested lifetime must match exactly.
 * @li #KRB5_TC_MATCH_FLAGS_EXACT  Flags must match exactly.
 * @li #KRB5_TC_MATCH_AUTHDATA     The authorization data must match.
 * @li #KRB5_TC_MATCH_SRV_NAMEONLY Only the name portion of the principal
 *                                 name must match, not the realm.
 * @li #KRB5_TC_MATCH_2ND_TKT      The second tickets must match.
 * @li #KRB5_TC_MATCH_KTYPE        The encryption key types must match.
 * @li #KRB5_TC_SUPPORTED_KTYPES   Check all matching entries that have any
 *                                 supported encryption type and return the
 *                                 one with the encryption type listed earliest.
 *
 * Use krb5_free_cred_contents() to free @a creds when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2457:1"]
        pub fn krb5_cc_retrieve_cred(context: krb5_context,
                                     cache: krb5_ccache, flags: krb5_flags,
                                     mcreds: *mut krb5_creds,
                                     creds: *mut krb5_creds)
         -> krb5_error_code;
        /*
 * end "keytab.h"
 */
        /*
 * begin "func-proto.h"
 */
        /* *< Use secure context configuration */
        /* *< Use KDC configuration if available */
        /* *
 * Create a krb5 library context.
 *
 * @param [out] context         Library context
 *
 * The @a context must be released by calling krb5_free_context() when
 * it is no longer needed.
 *
 * @warning Any program or module that needs the Kerberos code to not trust the
 * environment must use krb5_init_secure_context(), or clean out the
 * environment.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "2922:1"]
        pub fn krb5_init_context(context: *mut krb5_context)
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
        /* *
 * Set default TGS encryption types in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] etypes           Encryption type(s) to set
 *
 * This function sets the default enctype list for TGS requests
 * made using @a context to @a etypes.
 *
 * @note This overrides the default list (from config file or built-in).
 *
 * @retval
 *  0    Success
 * @retval
 *  KRB5_PROG_ETYPE_NOSUPP Program lacks support for encryption type
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3010:1"]
        pub fn krb5_set_default_tgs_enctypes(context: krb5_context,
                                             etypes: *const krb5_enctype)
         -> krb5_error_code;
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
 * Create a @c KRB_AP_REQ message using supplied credentials.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     in_creds       Credentials for the service with valid ticket
 *                                and key
 * @param [out]    outbuf         @c AP-REQ message
 *
 * Valid @a ap_req_options are:
 * @li #AP_OPTS_USE_SESSION_KEY - Use the session key when creating the
 *                                request used for user to user
 *                                authentication.
 * @li #AP_OPTS_MUTUAL_REQUIRED - Request a mutual authentication packet from
 *                                the reciever.
 * @li #AP_OPTS_USE_SUBKEY      - Generate a subsession key from the current
 *                                session key obtained from the credentials.
 *
 * This function creates a KRB_AP_REQ message using supplied credentials @a
 * in_creds.  @a auth_context may point to an existing auth context or to NULL,
 * in which case a new one will be created.  If @a in_data is non-null, a
 * checksum of it will be included in the authenticator contained in the
 * KRB_AP_REQ message.  Use krb5_free_data_contents() to free @a outbuf when it
 * is no longer needed.
 *
 * On successful return, the authenticator is stored in @a auth_context with
 * the @a client and @a checksum fields nulled out.  (This is to prevent
 * pointer-sharing problems; the caller should not need these fields anyway,
 * since the caller supplied them.)
 *
 * @sa krb5_mk_req()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3201:1"]
        pub fn krb5_mk_req_extended(context: krb5_context,
                                    auth_context: *mut krb5_auth_context,
                                    ap_req_options: krb5_flags,
                                    in_data: *mut krb5_data,
                                    in_creds: *mut krb5_creds,
                                    outbuf: *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Format and encrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3240:1"]
        pub fn krb5_mk_rep_dce(context: krb5_context,
                               auth_context: krb5_auth_context,
                               outbuf: *mut krb5_data) -> krb5_error_code;
        /* *
 * Parse and decrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] repl            Decrypted reply message
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a repl with a pointer to allocated memory containing the fields
 * from the encrypted response.
 *
 * Use krb5_free_ap_rep_enc_part() to free @a repl when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3259:1"]
        pub fn krb5_rd_rep(context: krb5_context,
                           auth_context: krb5_auth_context,
                           inbuf: *const krb5_data,
                           repl: *mut *mut krb5_ap_rep_enc_part)
         -> krb5_error_code;
        /* *
 * Decode a @c KRB-ERROR message.
 *
 * @param [in]  context         Library context
 * @param [in]  enc_errbuf      Encoded error message
 * @param [out] dec_error       Decoded error message
 *
 * This function processes @c KRB-ERROR message @a enc_errbuf and returns
 * an allocated structure @a dec_error containing the error message.
 * Use krb5_free_error() to free @a dec_error when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3309:1"]
        pub fn krb5_rd_error(context: krb5_context,
                             enc_errbuf: *const krb5_data,
                             dec_error: *mut *mut krb5_error)
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
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4710:1"]
        pub fn krb5_free_keyblock(context: krb5_context,
                                  val: *mut krb5_keyblock);
        /* *
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4732:1"]
        pub fn krb5_free_ap_rep_enc_part(context: krb5_context,
                                         val: *mut krb5_ap_rep_enc_part);
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
 * Get a forwarded TGT and format a @c KRB-CRED message.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rhost            Remote host
 * @param [in] client           Client principal of TGT
 * @param [in] server           Principal of server to receive TGT
 * @param [in] cc               Credential cache handle (NULL to use default)
 * @param [in] forwardable      Whether TGT should be forwardable
 * @param [out] outbuf          KRB-CRED message
 *
 * Get a TGT for use at the remote host @a rhost and format it into a KRB-CRED
 * message.  If @a rhost is NULL and @a server is of type #KRB5_NT_SRV_HST,
 * the second component of @a server will be used.
 *
 * @retval
 *  0 Success
 * @retval
 *   ENOMEM Insufficient memory
 * @retval
 *   KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 *   KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 *   KRB5_CC_BADNAME Credential cache name or principal name malformed
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5588:1"]
        pub fn krb5_fwd_tgt_creds(context: krb5_context,
                                  auth_context: krb5_auth_context,
                                  rhost: *const libc::c_char,
                                  client: krb5_principal,
                                  server: krb5_principal, cc: krb5_ccache,
                                  forwardable: libc::c_int,
                                  outbuf: *mut krb5_data) -> krb5_error_code;
        /* *
 * Create and initialize an authentication context.
 *
 * @param [in]  context         Library context
 * @param [out] auth_context    Authentication context
 *
 * This function creates an authentication context to hold configuration and
 * state relevant to krb5 functions for authenticating principals and
 * protecting messages once authentication has occurred.
 *
 * By default, flags for the context are set to enable the use of the replay
 * cache (#KRB5_AUTH_CONTEXT_DO_TIME), but not sequence numbers.  Use
 * krb5_auth_con_setflags() to change the flags.
 *
 * The allocated @a auth_context must be freed with krb5_auth_con_free() when
 * it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5613:1"]
        pub fn krb5_auth_con_init(context: krb5_context,
                                  auth_context: *mut krb5_auth_context)
         -> krb5_error_code;
        /* *
 * Free a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context to be freed
 *
 * This function frees an auth context allocated by krb5_auth_con_init().
 *
 * @retval 0  (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "5626:1"]
        pub fn krb5_auth_con_free(context: krb5_context,
                                  auth_context: krb5_auth_context)
         -> krb5_error_code;
        /* *
 * Set a flags field in a krb5_auth_context structure.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] flags            Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "5644:1"]
        pub fn krb5_auth_con_setflags(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      flags: krb5_int32) -> krb5_error_code;
        /* *
 * Retrieve flags from a krb5_auth_context structure.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] flags           Flags bit mask
 *
 * Valid values for @a flags are:
 * @li #KRB5_AUTH_CONTEXT_DO_TIME Use timestamps
 * @li #KRB5_AUTH_CONTEXT_RET_TIME Save timestamps
 * @li #KRB5_AUTH_CONTEXT_DO_SEQUENCE Use sequence numbers
 * @li #KRB5_AUTH_CONTEXT_RET_SEQUENCE Save sequence numbers
 *
 * @retval 0 (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "5662:1"]
        pub fn krb5_auth_con_getflags(context: krb5_context,
                                      auth_context: krb5_auth_context,
                                      flags: *mut krb5_int32)
         -> krb5_error_code;
        /* *
 * Set a checksum callback in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] func             Checksum callback
 * @param [in] data             Callback argument
 *
 * Set a callback to obtain checksum data in krb5_mk_req().  The callback will
 * be invoked after the subkey and local sequence number are stored in @a
 * auth_context.
 *
 * @retval 0 (always)
 */
        #[no_mangle]
        #[c2rust::src_loc = "5680:1"]
        pub fn krb5_auth_con_set_checksum_func(context: krb5_context,
                                               auth_context:
                                                   krb5_auth_context,
                                               func:
                                                   krb5_mk_req_checksum_func,
                                               data: *mut libc::c_void)
         -> krb5_error_code;
        /* *
 * Set the session key in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] keyblock         User key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5765:1"]
        pub fn krb5_auth_con_setuseruserkey(context: krb5_context,
                                            auth_context: krb5_auth_context,
                                            keyblock: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Retrieve the send subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Send subkey
 *
 * This function creates a keyblock containing the send subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5815:1"]
        pub fn krb5_auth_con_getsendsubkey(ctx: krb5_context,
                                           ac: krb5_auth_context,
                                           keyblock: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Retrieve the send subkey from an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets @a key to the send subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5830:1"]
        pub fn krb5_auth_con_getsendsubkey_k(ctx: krb5_context,
                                             ac: krb5_auth_context,
                                             key: *mut krb5_key)
         -> krb5_error_code;
        /* *
 * Set the send subkey in an auth context.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Send subkey
 *
 * This function sets the send subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5894:1"]
        pub fn krb5_auth_con_setsendsubkey_k(ctx: krb5_context,
                                             ac: krb5_auth_context,
                                             key: krb5_key)
         -> krb5_error_code;
        /* *
 * Retrieve the local sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Local sequence number
 *
 * Retrieve the local sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "5955:1"]
        pub fn krb5_auth_con_getlocalseqnumber(context: krb5_context,
                                               auth_context:
                                                   krb5_auth_context,
                                               seqnumber: *mut krb5_int32)
         -> krb5_error_code;
        /* *
 * Set checksum type in an an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] cksumtype        Checksum type
 *
 * This function sets the checksum type in @a auth_context to be used by
 * krb5_mk_req() for the authenticator checksum.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6051:1"]
        pub fn krb5_auth_con_set_req_cksumtype(context: krb5_context,
                                               auth_context:
                                                   krb5_auth_context,
                                               cksumtype: krb5_cksumtype)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:100"]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1737:8"]
    pub struct ldap_seqof_key_data {
        pub mkvno: krb5_int32,
        pub kvno: krb5_ui_2,
        pub key_data: *mut _krb5_key_data,
        pub n_key_data: krb5_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1925:16"]
    pub struct _krb5int_access {
        pub auth_con_get_subkey_enctype: Option<unsafe extern "C" fn(_:
                                                                         krb5_context,
                                                                     _:
                                                                         krb5_auth_context,
                                                                     _:
                                                                         *mut krb5_enctype)
                                                    -> krb5_error_code>,
        pub mandatory_cksumtype: Option<unsafe extern "C" fn(_: krb5_context,
                                                             _: krb5_enctype,
                                                             _:
                                                                 *mut krb5_cksumtype)
                                            -> krb5_error_code>,
        pub ser_pack_int64: Option<unsafe extern "C" fn(_: int64_t,
                                                        _:
                                                            *mut *mut krb5_octet,
                                                        _: *mut size_t)
                                       -> krb5_error_code>,
        pub ser_unpack_int64: Option<unsafe extern "C" fn(_: *mut int64_t,
                                                          _:
                                                              *mut *mut krb5_octet,
                                                          _: *mut size_t)
                                         -> krb5_error_code>,
        pub asn1_ldap_encode_sequence_of_keys: Option<unsafe extern "C" fn(_:
                                                                               *const ldap_seqof_key_data,
                                                                           _:
                                                                               *mut *mut krb5_data)
                                                          -> krb5_error_code>,
        pub asn1_ldap_decode_sequence_of_keys: Option<unsafe extern "C" fn(_:
                                                                               *const krb5_data,
                                                                           _:
                                                                               *mut *mut ldap_seqof_key_data)
                                                          -> krb5_error_code>,
        pub encode_krb5_auth_pack: Option<unsafe extern "C" fn(_:
                                                                   *const krb5_auth_pack,
                                                               _:
                                                                   *mut *mut krb5_data)
                                              -> krb5_error_code>,
        pub encode_krb5_kdc_dh_key_info: Option<unsafe extern "C" fn(_:
                                                                         *const krb5_kdc_dh_key_info,
                                                                     _:
                                                                         *mut *mut krb5_data)
                                                    -> krb5_error_code>,
        pub encode_krb5_pa_pk_as_rep: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_pa_pk_as_rep,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code>,
        pub encode_krb5_pa_pk_as_req: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_pa_pk_as_req,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code>,
        pub encode_krb5_reply_key_pack: Option<unsafe extern "C" fn(_:
                                                                        *const krb5_reply_key_pack,
                                                                    _:
                                                                        *mut *mut krb5_data)
                                                   -> krb5_error_code>,
        pub encode_krb5_td_dh_parameters: Option<unsafe extern "C" fn(_:
                                                                          *const *mut krb5_algorithm_identifier,
                                                                      _:
                                                                          *mut *mut krb5_data)
                                                     -> krb5_error_code>,
        pub encode_krb5_td_trusted_certifiers: Option<unsafe extern "C" fn(_:
                                                                               *const *mut krb5_external_principal_identifier,
                                                                           _:
                                                                               *mut *mut krb5_data)
                                                          -> krb5_error_code>,
        pub decode_krb5_auth_pack: Option<unsafe extern "C" fn(_:
                                                                   *const krb5_data,
                                                               _:
                                                                   *mut *mut krb5_auth_pack)
                                              -> krb5_error_code>,
        pub decode_krb5_pa_pk_as_req: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_data,
                                                                  _:
                                                                      *mut *mut krb5_pa_pk_as_req)
                                                 -> krb5_error_code>,
        pub decode_krb5_pa_pk_as_rep: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_data,
                                                                  _:
                                                                      *mut *mut krb5_pa_pk_as_rep)
                                                 -> krb5_error_code>,
        pub decode_krb5_kdc_dh_key_info: Option<unsafe extern "C" fn(_:
                                                                         *const krb5_data,
                                                                     _:
                                                                         *mut *mut krb5_kdc_dh_key_info)
                                                    -> krb5_error_code>,
        pub decode_krb5_principal_name: Option<unsafe extern "C" fn(_:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut *mut krb5_principal_data)
                                                   -> krb5_error_code>,
        pub decode_krb5_reply_key_pack: Option<unsafe extern "C" fn(_:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut *mut krb5_reply_key_pack)
                                                   -> krb5_error_code>,
        pub decode_krb5_td_dh_parameters: Option<unsafe extern "C" fn(_:
                                                                          *const krb5_data,
                                                                      _:
                                                                          *mut *mut *mut krb5_algorithm_identifier)
                                                     -> krb5_error_code>,
        pub decode_krb5_td_trusted_certifiers: Option<unsafe extern "C" fn(_:
                                                                               *const krb5_data,
                                                                           _:
                                                                               *mut *mut *mut krb5_external_principal_identifier)
                                                          -> krb5_error_code>,
        pub encode_krb5_kdc_req_body: Option<unsafe extern "C" fn(_:
                                                                      *const krb5_kdc_req,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code>,
        pub free_kdc_req: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: *mut krb5_kdc_req)
                                     -> ()>,
        pub set_prompt_types: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *mut krb5_prompt_type)
                                         -> ()>,
    }
    #[c2rust::src_loc = "1925:1"]
    pub type krb5int_access = _krb5int_access;
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
    /* Return the delta between two timestamps (a - b) as a signed 32-bit value,
 * without relying on undefined behavior. */
    #[inline]
    #[c2rust::src_loc = "2346:1"]
    pub unsafe extern "C" fn ts_delta(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_deltat {
        return (a as uint32_t).wrapping_sub(b as uint32_t) as krb5_deltat;
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
    /* Return true if a comes after b. */
    #[inline]
    #[c2rust::src_loc = "2361:1"]
    pub unsafe extern "C" fn ts_after(mut a: krb5_timestamp,
                                      mut b: krb5_timestamp) -> krb5_boolean {
        return (a as uint32_t > b as uint32_t) as libc::c_int as krb5_boolean;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_keyblock, krb5_data, krb5_key, krb5_pointer,
                        krb5_error_code, krb5_context, krb5_keytab,
                        krb5_const_principal, krb5_kvno, krb5_keytab_entry,
                        krb5_kt_cursor, krb5_authdatatype, krb5_ui_2,
                        krb5_int16, krb5_auth_context, krb5_cksumtype,
                        krb5_octet, krb5_principal_data, krb5_kdc_req,
                        krb5_authdata, _krb5_auth_context, krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::authdata_plugin_h::{authdata_client_plugin_fini_proc,
                                   krb5plugin_authdata_client_ftable_v0,
                                   authdata_client_request_init_proc,
                                   authdata_client_request_fini_proc};
    use super::stdint_intn_h::int64_t;
    use super::stddef_h::size_t;
    use super::k5_int_pkinit_h::{krb5_auth_pack, krb5_kdc_dh_key_info,
                                 krb5_pa_pk_as_rep, krb5_pa_pk_as_req,
                                 krb5_reply_key_pack,
                                 krb5_algorithm_identifier,
                                 krb5_external_principal_identifier};
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
        #[c2rust::src_loc = "1735:8"]
        pub type _krb5_key_data;
        #[no_mangle]
        #[c2rust::src_loc = "619:1"]
        pub fn krb5int_init_context_kdc(_: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1015:1"]
        pub fn krb5_authdata_export_authdata(kcontext: krb5_context,
                                             context: krb5_authdata_context,
                                             usage: krb5_flags,
                                             pauthdata:
                                                 *mut *mut *mut krb5_authdata)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2016:1"]
        pub fn krb5int_accessor(_: *mut krb5int_access, _: krb5_int32)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2209:1"]
        pub fn krb5_auth_con_set_authdata_context(context: krb5_context,
                                                  auth_context:
                                                      krb5_auth_context,
                                                  ad_context:
                                                      krb5_authdata_context)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:100"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:100"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:100"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:100"]
pub mod k5_int_pkinit_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * pkinit structures
 */
    /* PKAuthenticator */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct _krb5_pk_authenticator {
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub paChecksum: krb5_checksum,
        pub freshnessToken: *mut krb5_data,
    }
    #[c2rust::src_loc = "40:1"]
    pub type krb5_pk_authenticator = _krb5_pk_authenticator;
    /* AlgorithmIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    /* Optional */
    /* SubjectPublicKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct _krb5_subject_pk_info {
        pub algorithm: krb5_algorithm_identifier,
        pub subjectPublicKey: krb5_data,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_subject_pk_info = _krb5_subject_pk_info;
    /* BIT STRING */
    /* * AuthPack from RFC 4556*/
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct _krb5_auth_pack {
        pub pkAuthenticator: krb5_pk_authenticator,
        pub clientPublicValue: *mut krb5_subject_pk_info,
        pub supportedCMSTypes: *mut *mut krb5_algorithm_identifier,
        pub clientDHNonce: krb5_data,
        pub supportedKDFs: *mut *mut krb5_data,
    }
    #[c2rust::src_loc = "61:1"]
    pub type krb5_auth_pack = _krb5_auth_pack;
    /* OIDs of KDFs; OPTIONAL */
    /* ExternalPrincipalIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    /* Optional */
    /* PA-PK-AS-REQ (rfc4556 -- PA TYPE 16) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:16"]
    pub struct _krb5_pa_pk_as_req {
        pub signedAuthPack: krb5_data,
        pub trustedCertifiers: *mut *mut krb5_external_principal_identifier,
        pub kdcPkId: krb5_data,
    }
    #[c2rust::src_loc = "77:1"]
    pub type krb5_pa_pk_as_req = _krb5_pa_pk_as_req;
    /* Optional */
    /* * Pkinit DHRepInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:16"]
    pub struct _krb5_dh_rep_info {
        pub dhSignedData: krb5_data,
        pub serverDHNonce: krb5_data,
        pub kdfID: *mut krb5_data,
    }
    #[c2rust::src_loc = "84:1"]
    pub type krb5_dh_rep_info = _krb5_dh_rep_info;
    /* OID of selected KDF OPTIONAL */
    /* KDCDHKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:16"]
    pub struct _krb5_kdc_dh_key_info {
        pub subjectPublicKey: krb5_data,
        pub nonce: krb5_int32,
        pub dhKeyExpiration: krb5_timestamp,
    }
    #[c2rust::src_loc = "91:1"]
    pub type krb5_kdc_dh_key_info = _krb5_kdc_dh_key_info;
    /* Optional */
    /* ReplyKeyPack */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct _krb5_reply_key_pack {
        pub replyKey: krb5_keyblock,
        pub asChecksum: krb5_checksum,
    }
    #[c2rust::src_loc = "98:1"]
    pub type krb5_reply_key_pack = _krb5_reply_key_pack;
    /* PA-PK-AS-REP (rfc4556 -- PA TYPE 17) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:16"]
    pub struct _krb5_pa_pk_as_rep {
        pub choice: krb5_pa_pk_as_rep_selection,
        pub u: krb5_pa_pk_as_rep_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:11"]
    pub union krb5_pa_pk_as_rep_choices {
        pub dh_Info: krb5_dh_rep_info,
        pub encKeyPack: krb5_data,
    }
    #[c2rust::src_loc = "105:5"]
    pub type krb5_pa_pk_as_rep_selection = libc::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const choice_pa_pk_as_rep_encKeyPack: krb5_pa_pk_as_rep_selection = 1;
    #[c2rust::src_loc = "107:9"]
    pub const choice_pa_pk_as_rep_dhInfo: krb5_pa_pk_as_rep_selection = 0;
    #[c2rust::src_loc = "106:9"]
    pub const choice_pa_pk_as_rep_UNKNOWN: krb5_pa_pk_as_rep_selection = -1;
    #[c2rust::src_loc = "104:1"]
    pub type krb5_pa_pk_as_rep = _krb5_pa_pk_as_rep;
    use super::krb5_h::{krb5_int32, krb5_timestamp, krb5_checksum, krb5_data,
                        krb5_keyblock};
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/authdata_plugin.h:100"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:101"]
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
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:101"]
pub mod gssapiP_generic_h {
    #[c2rust::src_loc = "129:1"]
    pub type g_seqnum_state = *mut g_seqnum_state_st;
    use super::gssapi_h::{gss_OID_desc, gss_int32, OM_uint32, gss_OID};
    use super::stdint_uintn_h::uint64_t;
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
        #[no_mangle]
        #[c2rust::src_loc = "177:1"]
        pub fn gssint_g_seqstate_init(state_out: *mut g_seqnum_state,
                                      seqnum: uint64_t,
                                      do_replay: libc::c_int,
                                      do_sequence: libc::c_int,
                                      wide: libc::c_int) -> libc::c_long;
        /* set */
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn generic_gss_copy_oid(_: *mut OM_uint32, _: *const gss_OID_desc,
                                    _: *mut gss_OID) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapiP_krb5.h:101"]
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
    pub type krb5_gss_cred_id_t = *mut _krb5_gss_cred_id_rec;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "197:16"]
    pub struct _krb5_gss_ctx_ext_rec {
        pub iakerb: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "198:5"]
    pub struct C2RustUnnamed_3 {
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
    pub type krb5_gss_ctx_id_rec = _krb5_gss_ctx_id_rec;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_gss_ctx_id_t = *mut _krb5_gss_ctx_id_rec;
    #[inline]
    #[c2rust::src_loc = "1255:1"]
    pub unsafe extern "C" fn data_to_gss(mut input_k5data: *mut krb5_data,
                                         mut output_buffer: gss_buffer_t)
     -> krb5_error_code {
        let mut code: krb5_error_code = 0 as libc::c_int;
        (*output_buffer).length = (*input_k5data).length as size_t;
        (*output_buffer).value = (*input_k5data).data as *mut libc::c_void;
        *input_k5data = empty_data();
        return code;
    }
    use super::krb5_h::{krb5_principal, krb5_keytab, krb5_rcache, krb5_ccache,
                        krb5_boolean, krb5_timestamp, krb5_enctype, krb5_data,
                        krb5_magic, krb5_key, krb5_ticket_times, krb5_flags,
                        krb5_context, krb5_auth_context, krb5_cksumtype,
                        krb5_authdata, krb5_checksum, krb5_error_code};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_int_h::{krb5_authdata_context, _krb5_context, krb5_key_st,
                          empty_data};
    use super::gssapi_h::{gss_cred_usage_t, OM_uint32, gss_OID_desc,
                          gss_cred_id_t, gss_channel_bindings_struct,
                          gss_channel_bindings_t, gss_cred_id_struct,
                          gss_name_struct, gss_name_t, gss_ctx_id_t,
                          gss_buffer_desc_struct, gss_buffer_t,
                          gss_OID_desc_struct, gss_OID};
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::gssapiP_generic_h::g_seqnum_state;
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "256:1"]
        pub fn kg_get_defcred(minor_status: *mut OM_uint32,
                              cred: *mut gss_cred_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn kg_checksum_channel_bindings(context: krb5_context,
                                            cb: gss_channel_bindings_t,
                                            cksum: *mut krb5_checksum)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "278:1"]
        pub fn kg_setup_keys(context: krb5_context,
                             ctx: *mut krb5_gss_ctx_id_rec, subkey: krb5_key,
                             cksumtype: *mut krb5_cksumtype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "376:1"]
        pub fn kg_sync_ccache_name(context: krb5_context,
                                   minor_status: *mut OM_uint32) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "499:1"]
        pub fn kg_cred_resolve(minor_status: *mut OM_uint32,
                               context: krb5_context,
                               cred_handle: gss_cred_id_t,
                               target_name: gss_name_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "551:1"]
        pub fn krb5_gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "645:1"]
        pub fn krb5_gss_delete_sec_context(_: *mut OM_uint32,
                                           _: *mut gss_ctx_id_t,
                                           _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "910:1"]
        pub fn krb5_gss_convert_static_mech_oid(oid: gss_OID) -> gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "944:1"]
        pub fn kg_compare_name(context: krb5_context, name1: krb5_gss_name_t,
                               name2: krb5_gss_name_t) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "1186:5"]
        pub fn krb5_gss_save_error_info(minor_code: OM_uint32,
                                        ctx: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "937:1"]
        pub fn kg_release_name(context: krb5_context,
                               name: *mut krb5_gss_name_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1238:1"]
        pub fn iakerb_make_finished(context: krb5_context, key: krb5_key,
                                    conv: *const krb5_data,
                                    finished: *mut *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "940:1"]
        pub fn kg_duplicate_name(context: krb5_context, src: krb5_gss_name_t,
                                 dst: *mut krb5_gss_name_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "1171:1"]
        pub fn gss_krb5int_initialize_library() -> OM_uint32;
    }
    /* _GSSAPIP_KRB5_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:100"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:100"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:100"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
#[c2rust::header_src = "/usr/include/assert.h:100"]
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
#[c2rust::header_src = "/usr/include/bits/byteswap.h:100"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
        return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
                    |
                    (__bsx as libc::c_int & 0xff as libc::c_int) <<
                        8 as libc::c_int) as __uint16_t;
    }
    use super::types_h::{__uint32_t, __uint16_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:101"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/krb5/gssapi_krb5.h:101"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        /* {iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-enterprise-name(6)}. */
        #[no_mangle]
        #[c2rust::src_loc = "81:33"]
        pub static gss_mech_krb5: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "82:33"]
        pub static gss_mech_krb5_old: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "83:33"]
        pub static gss_mech_krb5_wrong: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "84:33"]
        pub static gss_mech_iakerb: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __int64_t, __uint64_t, __off_t,
                        __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_lock,
                            k5_mutex_unlock, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
                              C2RustUnnamed_2, store_16_be, store_32_be,
                              store_16_le, store_32_le};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_cksumtype,
                       krb5_authdatatype, krb5_preauthtype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_pointer,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, _krb5_address, krb5_address,
                       krb5_post_recv_fn, krb5_context, krb5_pre_send_fn,
                       krb5_trace_callback, krb5_trace_info, _krb5_trace_info,
                       krb5_prompt_type, krb5_auth_context, _krb5_keyblock,
                       krb5_keyblock, krb5_key, _krb5_checksum, krb5_checksum,
                       _krb5_enc_data, krb5_enc_data, _krb5_ticket_times,
                       krb5_ticket_times, _krb5_authdata, krb5_authdata,
                       _krb5_transited, krb5_transited, _krb5_enc_tkt_part,
                       krb5_enc_tkt_part, _krb5_ticket, krb5_ticket,
                       _krb5_creds, krb5_creds, _krb5_pa_data, krb5_pa_data,
                       _krb5_kdc_req, krb5_kdc_req, _krb5_error, krb5_error,
                       _krb5_ap_req, krb5_ap_req, _krb5_ap_rep_enc_part,
                       krb5_ap_rep_enc_part, krb5_mk_req_checksum_func,
                       krb5_ccache, krb5_rcache, krb5_kt_cursor,
                       krb5_keytab_entry_st, krb5_keytab_entry, krb5_keytab,
                       _profile_t, _krb5_auth_context, _krb5_ccache,
                       krb5_rc_st, krb5_k_create_key, krb5_k_free_key,
                       krb5_cc_retrieve_cred, krb5_init_context,
                       krb5_free_context, krb5_set_default_tgs_enctypes,
                       krb5_get_credentials, krb5_mk_req_extended,
                       krb5_mk_rep_dce, krb5_rd_rep, krb5_rd_error,
                       krb5_principal_compare, krb5_free_authdata,
                       krb5_free_error, krb5_free_creds,
                       krb5_free_cred_contents, krb5_free_checksum_contents,
                       krb5_free_keyblock, krb5_free_ap_rep_enc_part,
                       krb5_free_data, krb5_free_data_contents,
                       krb5_timeofday, krb5_fwd_tgt_creds, krb5_auth_con_init,
                       krb5_auth_con_free, krb5_auth_con_setflags,
                       krb5_auth_con_getflags,
                       krb5_auth_con_set_checksum_func,
                       krb5_auth_con_setuseruserkey,
                       krb5_auth_con_getsendsubkey,
                       krb5_auth_con_getsendsubkey_k,
                       krb5_auth_con_setsendsubkey_k,
                       krb5_auth_con_getlocalseqnumber,
                       krb5_auth_con_set_req_cksumtype};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, krb5_key_st, derived_key, _krb5_kt,
                         _krb5_kt_ops, _krb5_authdata_context,
                         _krb5_authdata_context_module, krb5_authdata_context,
                         ldap_seqof_key_data, _krb5int_access, krb5int_access,
                         make_data, empty_data, ts_delta, ts_incr, ts_after,
                         plugin_mapping, _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, _krb5_key_data,
                         krb5int_init_context_kdc,
                         krb5_authdata_export_authdata, krb5int_accessor,
                         krb5_auth_con_set_authdata_context};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::k5_int_pkinit_h::{_krb5_pk_authenticator, krb5_pk_authenticator,
                                _krb5_algorithm_identifier,
                                krb5_algorithm_identifier,
                                _krb5_subject_pk_info, krb5_subject_pk_info,
                                _krb5_auth_pack, krb5_auth_pack,
                                _krb5_external_principal_identifier,
                                krb5_external_principal_identifier,
                                _krb5_pa_pk_as_req, krb5_pa_pk_as_req,
                                _krb5_dh_rep_info, krb5_dh_rep_info,
                                _krb5_kdc_dh_key_info, krb5_kdc_dh_key_info,
                                _krb5_reply_key_pack, krb5_reply_key_pack,
                                _krb5_pa_pk_as_rep, krb5_pa_pk_as_rep_choices,
                                krb5_pa_pk_as_rep_selection,
                                choice_pa_pk_as_rep_encKeyPack,
                                choice_pa_pk_as_rep_dhInfo,
                                choice_pa_pk_as_rep_UNKNOWN,
                                krb5_pa_pk_as_rep};
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
                         gss_OID_desc, gss_OID, gss_buffer_desc_struct,
                         gss_buffer_desc, gss_buffer_t,
                         gss_channel_bindings_struct, gss_channel_bindings_t,
                         gss_cred_usage_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct};
pub use self::gssapiP_generic_h::{g_seqnum_state, g_seqnum_state_st,
                                  gssint_g_token_size,
                                  gssint_g_make_token_header,
                                  gssint_g_verify_token_header,
                                  gssint_g_seqstate_init,
                                  generic_gss_copy_oid};
pub use self::gssapiP_krb5_h::{_krb5_gss_name_rec, krb5_gss_name_t,
                               _krb5_gss_cred_id_rec, krb5_gss_cred_id_t,
                               _krb5_gss_ctx_ext_rec, C2RustUnnamed_3,
                               krb5_gss_ctx_ext_rec, krb5_gss_ctx_ext_t,
                               _krb5_gss_ctx_id_rec, krb5_gss_ctx_id_rec,
                               krb5_gss_ctx_id_t, data_to_gss, kg_get_defcred,
                               kg_checksum_channel_bindings, kg_setup_keys,
                               kg_sync_ccache_name, kg_cred_resolve,
                               krb5_gss_release_cred,
                               krb5_gss_delete_sec_context,
                               krb5_gss_convert_static_mech_oid,
                               kg_compare_name, krb5_gss_save_error_info,
                               kg_release_name, iakerb_make_finished,
                               kg_duplicate_name,
                               gss_krb5int_initialize_library};
use self::stdio_h::{fprintf, stderr};
use self::stdlib_h::{free, malloc};
use self::string_h::{strerror, memcmp, memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::byteswap_h::{__bswap_32, __bswap_16};
pub use self::gssapi_alloc_h::gssalloc_malloc;
use self::gssapi_krb5_h::{gss_mech_krb5, gss_mech_krb5_old,
                          gss_mech_krb5_wrong, gss_mech_iakerb};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "239:8"]
pub struct gss_checksum_data {
    pub ctx: *mut krb5_gss_ctx_id_rec,
    pub cred: krb5_gss_cred_id_t,
    pub md5: krb5_checksum,
    pub checksum_data: krb5_data,
    pub exts: krb5_gss_ctx_ext_t,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2002, 2003, 2007, 2008 by the Massachusetts Institute of
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
/*
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * $Id$
 */
/* XXX This is for debugging only!!!  Should become a real bitfield
   at some point */
#[no_mangle]
#[c2rust::src_loc = "114:5"]
pub static mut krb5_gss_dbg_client_expcreds: libc::c_int = 0 as libc::c_int;
/*
 * Common code which fetches the correct krb5 credentials from the
 * ccache.
 */
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn get_credentials(mut context: krb5_context,
                                     mut cred: krb5_gss_cred_id_t,
                                     mut server: krb5_gss_name_t,
                                     mut now: krb5_timestamp,
                                     mut endtime: krb5_timestamp,
                                     mut out_creds: *mut *mut krb5_creds)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
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
    let mut evidence_creds: krb5_creds =
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
    let mut mcreds: krb5_creds =
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
    let mut result_creds: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut flags: krb5_flags = 0 as libc::c_int;
    let mut server_data: krb5_principal_data =
        krb5_principal_data{magic: 0,
                            realm:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},
                            data: 0 as *mut krb5_data,
                            length: 0,
                            type_0: 0,};
    *out_creds = 0 as *mut krb5_creds;
    memset(&mut in_creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    memset(&mut evidence_creds as *mut krb5_creds as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
    in_creds.server = 0 as krb5_principal;
    in_creds.client = in_creds.server;
    if !(*cred).name.is_null() {
    } else {
        __assert_fail(b"cred->name != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"init_sec_context.c\x00" as *const u8 as
                          *const libc::c_char,
                      141 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 34],
                                                &[libc::c_char; 34]>(b"krb5_error_code get_credentials()\x00")).as_ptr());
    }
    /* Remove assumed realm from host-based S4U2Proxy requests as they must
     * start in the client realm. */
    server_data = *(*server).princ;
    if !(*cred).impersonator.is_null() &&
           server_data.type_0 == 3 as libc::c_int {
        server_data.realm = empty_data()
    }
    in_creds.server = &mut server_data;
    in_creds.client = (*(*cred).name).princ;
    in_creds.times.endtime = endtime;
    in_creds.authdata = 0 as *mut *mut krb5_authdata;
    in_creds.keyblock.enctype = 0 as libc::c_int;
    /*
     * cred->name is immutable, so there is no need to acquire
     * cred->name->lock.
     */
    if !(*(*cred).name).ad_context.is_null() {
        code =
            krb5_authdata_export_authdata(context, (*(*cred).name).ad_context,
                                          0x2 as libc::c_int,
                                          &mut in_creds.authdata);
        if code != 0 as libc::c_int {
            current_block = 4109192512989772929;
        } else { current_block = 4956146061682418353; }
    } else { current_block = 4956146061682418353; }
    match current_block {
        4956146061682418353 => {
            /*
     * For IAKERB or constrained delegation, only check the cache in this step.
     * For IAKERB we will ask the server to make any necessary TGS requests;
     * for constrained delegation we will adjust in_creds and make an S4U2Proxy
     * request below if the cache lookup fails.
     */
            if !(*cred).impersonator.is_null() ||
                   (*cred).iakerb_mech() as libc::c_int != 0 {
                flags |= 2 as libc::c_int
            }
            code =
                krb5_get_credentials(context, flags, (*cred).ccache,
                                     &mut in_creds, &mut result_creds);
            /*
     * Try constrained delegation if we have proxy credentials, unless
     * we are trying to get a ticket to ourselves (in which case we could
     * just use the evidence ticket directly from cache).
     */
            if code as libc::c_long == -(1765328243 as libc::c_long) &&
                   !(*cred).impersonator.is_null() &&
                   (*cred).iakerb_mech() == 0 &&
                   krb5_principal_compare(context,
                                          (*cred).impersonator as
                                              krb5_const_principal,
                                          (*server).princ as
                                              krb5_const_principal) == 0 {
                memset(&mut mcreds as *mut krb5_creds as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<krb5_creds>() as libc::c_ulong);
                mcreds.magic = -(1760647408 as libc::c_long) as krb5_magic;
                mcreds.server = (*cred).impersonator;
                mcreds.client = (*(*cred).name).princ;
                code =
                    krb5_cc_retrieve_cred(context, (*cred).ccache,
                                          0x20 as libc::c_int, &mut mcreds,
                                          &mut evidence_creds);
                if code != 0 {
                    current_block = 4109192512989772929;
                } else {
                    in_creds.client = (*cred).impersonator;
                    in_creds.second_ticket = evidence_creds.ticket;
                    flags = 4 as libc::c_int | 64 as libc::c_int;
                    code =
                        krb5_get_credentials(context, flags, (*cred).ccache,
                                             &mut in_creds,
                                             &mut result_creds);
                    current_block = 5689316957504528238;
                }
            } else { current_block = 5689316957504528238; }
            match current_block {
                4109192512989772929 => { }
                _ => {
                    if !(code != 0) {
                        if flags & 64 as libc::c_int != 0 {
                            if krb5_principal_compare(context,
                                                      (*(*cred).name).princ as
                                                          krb5_const_principal,
                                                      (*result_creds).client
                                                          as
                                                          krb5_const_principal)
                                   == 0 {
                                /* server did not support constrained delegation */
                                code =
                                    -(1765328237 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 4109192512989772929;
                            } else { current_block = 2873832966593178012; }
                        } else { current_block = 2873832966593178012; }
                        match current_block {
                            4109192512989772929 => { }
                            _ =>
                            /*
     * Enforce a stricter limit (without timeskew forgiveness at the
     * boundaries) because accept_sec_context code is also similarly
     * non-forgiving.
     */
                            {
                                if krb5_gss_dbg_client_expcreds == 0 &&
                                       ts_after(now,
                                                (*result_creds).times.endtime)
                                           != 0 {
                                    code =
                                        -(1765328352 as libc::c_long) as
                                            krb5_error_code
                                } else {
                                    *out_creds = result_creds;
                                    result_creds = 0 as *mut krb5_creds
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    krb5_free_authdata(context, in_creds.authdata);
    krb5_free_cred_contents(context, &mut evidence_creds);
    krb5_free_creds(context, result_creds);
    return code;
}
#[c2rust::src_loc = "250:1"]
unsafe extern "C" fn make_gss_checksum(mut context: krb5_context,
                                       mut auth_context: krb5_auth_context,
                                       mut cksum_data: *mut libc::c_void,
                                       mut out: *mut *mut krb5_data)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut code: krb5_error_code = 0;
    let mut con_flags: krb5_int32 = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut data: *mut gss_checksum_data =
        cksum_data as *mut gss_checksum_data;
    let mut credmsg: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut junk: libc::c_uint = 0;
    let mut finished: *mut krb5_data = 0 as *mut krb5_data;
    let mut send_subkey: krb5_key = 0 as *mut krb5_key_st;
    (*data).checksum_data.data = 0 as *mut libc::c_char;
    credmsg.data = 0 as *mut libc::c_char;
    /* build the checksum field */
    if (*(*data).ctx).gss_flags & 1 as libc::c_int as libc::c_uint != 0 {
        /* first get KRB_CRED message, so we know its length */
        /* clear the time check flag that was set in krb5_auth_con_init() */
        krb5_auth_con_getflags(context, auth_context, &mut con_flags);
        krb5_auth_con_setflags(context, auth_context,
                               con_flags & !(0x1 as libc::c_int));
        if !(*(*data).cred).name.is_null() {
        } else {
            __assert_fail(b"data->cred->name != NULL\x00" as *const u8 as
                              *const libc::c_char,
                          b"init_sec_context.c\x00" as *const u8 as
                              *const libc::c_char,
                          275 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 89],
                                                    &[libc::c_char; 89]>(b"krb5_error_code make_gss_checksum(krb5_context, krb5_auth_context, void *, krb5_data **)\x00")).as_ptr());
        }
        /*
         * RFC 4121 4.1.1 specifies forwarded credentials must be encrypted in
         * the session key, but krb5_fwd_tgt_creds will use the send subkey if
         * it's set in the auth context.  Suppress the send subkey
         * temporarily.
         */
        krb5_auth_con_getsendsubkey_k(context, auth_context,
                                      &mut send_subkey);
        krb5_auth_con_setsendsubkey_k(context, auth_context, 0 as krb5_key);
        code =
            krb5_fwd_tgt_creds(context, auth_context,
                               0 as *const libc::c_char,
                               (*(*(*data).cred).name).princ,
                               (*(*(*data).ctx).there).princ,
                               (*(*data).cred).ccache, 1 as libc::c_int,
                               &mut credmsg);
        /* Turn KRB5_AUTH_CONTEXT_DO_TIME back on and reset the send subkey. */
        krb5_auth_con_setflags(context, auth_context, con_flags);
        krb5_auth_con_setsendsubkey_k(context, auth_context, send_subkey);
        krb5_k_free_key(context, send_subkey);
        if code != 0 {
            /* don't fail here; just don't accept/do the delegation
               request */
            (*(*data).ctx).gss_flags &=
                !(1 as libc::c_int | 32768 as libc::c_int) as libc::c_uint;
            (*data).checksum_data.length = 24 as libc::c_int as libc::c_uint;
            current_block = 13472856163611868459;
        } else if credmsg.length.wrapping_add(28 as libc::c_int as
                                                  libc::c_uint) >
                      65535 as libc::c_int as libc::c_uint {
            code = -(1765328323 as libc::c_long) as krb5_error_code;
            current_block = 11340719365295539846;
        } else {
            (*data).checksum_data.length =
                (28 as libc::c_int as
                     libc::c_uint).wrapping_add(credmsg.length);
            current_block = 13472856163611868459;
        }
    } else {
        (*data).checksum_data.length = 24 as libc::c_int as libc::c_uint;
        current_block = 13472856163611868459;
    }
    match current_block {
        13472856163611868459 => {
            junk = 0 as libc::c_int as libc::c_uint;
            if !(*data).exts.is_null() {
            } else {
                __assert_fail(b"data->exts != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"init_sec_context.c\x00" as *const u8 as
                                  *const libc::c_char,
                              328 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 89],
                                                        &[libc::c_char; 89]>(b"krb5_error_code make_gss_checksum(krb5_context, krb5_auth_context, void *, krb5_data **)\x00")).as_ptr());
            }
            if !(*(*data).exts).iakerb.conv.is_null() {
                let mut key: krb5_key = 0 as *mut krb5_key_st;
                code =
                    krb5_auth_con_getsendsubkey_k(context, auth_context,
                                                  &mut key);
                if code != 0 as libc::c_int {
                    current_block = 11340719365295539846;
                } else {
                    code =
                        iakerb_make_finished(context, key,
                                             (*(*data).exts).iakerb.conv,
                                             &mut finished);
                    if code != 0 as libc::c_int {
                        krb5_k_free_key(context, key);
                        current_block = 11340719365295539846;
                    } else {
                        krb5_k_free_key(context, key);
                        (*data).checksum_data.length =
                            (*data).checksum_data.length.wrapping_add((8 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint).wrapping_add((*finished).length));
                        current_block = 2873832966593178012;
                    }
                }
            } else { current_block = 2873832966593178012; }
            match current_block {
                11340719365295539846 => { }
                _ => {
                    (*data).checksum_data.length =
                        (*data).checksum_data.length.wrapping_add(junk);
                    /* now allocate a buffer to hold the checksum data and
       (maybe) KRB_CRED msg */
                    (*data).checksum_data.data =
                        malloc((*data).checksum_data.length as libc::c_ulong)
                            as *mut libc::c_char;
                    if (*data).checksum_data.data.is_null() {
                        code = 12 as libc::c_int
                    } else {
                        ptr =
                            (*data).checksum_data.data as *mut libc::c_uchar;
                        store_32_le((*data).md5.length,
                                    ptr as *mut libc::c_void);
                        ptr = ptr.offset(4 as libc::c_int as isize);
                        memcpy(ptr as *mut libc::c_void,
                               (*data).md5.contents as *const libc::c_void,
                               (*data).md5.length as libc::c_ulong);
                        ptr = ptr.offset((*data).md5.length as isize);
                        store_32_le((*(*data).ctx).gss_flags,
                                    ptr as *mut libc::c_void);
                        ptr = ptr.offset(4 as libc::c_int as isize);
                        if !credmsg.data.is_null() {
                            store_16_le(1 as libc::c_int as libc::c_uint,
                                        ptr as *mut libc::c_void);
                            ptr = ptr.offset(2 as libc::c_int as isize);
                            store_16_le(credmsg.length,
                                        ptr as *mut libc::c_void);
                            ptr = ptr.offset(2 as libc::c_int as isize);
                            memcpy(ptr as *mut libc::c_void,
                                   credmsg.data as *const libc::c_void,
                                   credmsg.length as libc::c_ulong);
                            ptr = ptr.offset(credmsg.length as isize)
                        }
                        if !(*(*data).exts).iakerb.conv.is_null() {
                            store_32_be(1 as libc::c_int as libc::c_uint,
                                        ptr as *mut libc::c_void);
                            ptr = ptr.offset(4 as libc::c_int as isize);
                            store_32_be((*finished).length,
                                        ptr as *mut libc::c_void);
                            ptr = ptr.offset(4 as libc::c_int as isize);
                            memcpy(ptr as *mut libc::c_void,
                                   (*finished).data as *const libc::c_void,
                                   (*finished).length as libc::c_ulong);
                            ptr = ptr.offset((*finished).length as isize)
                        }
                        if junk != 0 {
                            memset(ptr as *mut libc::c_void, 'i' as i32,
                                   junk as libc::c_ulong);
                        }
                        *out = &mut (*data).checksum_data;
                        code = 0 as libc::c_int
                    }
                }
            }
        }
        _ => { }
    }
    krb5_free_data_contents(context, &mut credmsg);
    krb5_free_data(context, finished);
    return code;
}
#[c2rust::src_loc = "385:1"]
unsafe extern "C" fn make_ap_req_v1(mut context: krb5_context,
                                    mut ctx: *mut krb5_gss_ctx_id_rec,
                                    mut cred: krb5_gss_cred_id_t,
                                    mut k_cred: *mut krb5_creds,
                                    mut ad_context: krb5_authdata_context,
                                    mut chan_bindings: gss_channel_bindings_t,
                                    mut mech_type: gss_OID,
                                    mut token: gss_buffer_t,
                                    mut exts: krb5_gss_ctx_ext_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut mk_req_flags: krb5_flags = 0 as libc::c_int;
    let mut code: krb5_error_code = 0;
    let mut cksum_struct: gss_checksum_data =
        gss_checksum_data{ctx: 0 as *mut krb5_gss_ctx_id_rec,
                          cred: 0 as *mut _krb5_gss_cred_id_rec,
                          md5:
                              krb5_checksum{magic: 0,
                                            checksum_type: 0,
                                            length: 0,
                                            contents: 0 as *mut krb5_octet,},
                          checksum_data:
                              krb5_data{magic: 0,
                                        length: 0,
                                        data: 0 as *mut libc::c_char,},
                          exts: 0 as *mut _krb5_gss_ctx_ext_rec,};
    let mut md5: krb5_checksum =
        krb5_checksum{magic: 0,
                      checksum_type: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut ap_req: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut t: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tlen: libc::c_uint = 0;
    ap_req.data = 0 as *mut libc::c_char;
    /* compute the hash of the channel bindings */
    code = kg_checksum_channel_bindings(context, chan_bindings, &mut md5);
    if code != 0 { return code }
    krb5_auth_con_set_req_cksumtype(context, (*ctx).auth_context,
                                    0x8003 as libc::c_int);
    cksum_struct.md5 = md5;
    cksum_struct.ctx = ctx;
    cksum_struct.cred = cred;
    cksum_struct.checksum_data.data = 0 as *mut libc::c_char;
    cksum_struct.exts = exts;
    krb5_auth_con_set_checksum_func(context, (*ctx).auth_context,
                                    Some(make_gss_checksum as
                                             unsafe extern "C" fn(_:
                                                                      krb5_context,
                                                                  _:
                                                                      krb5_auth_context,
                                                                  _:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *mut *mut krb5_data)
                                                 -> krb5_error_code),
                                    &mut cksum_struct as
                                        *mut gss_checksum_data as
                                        *mut libc::c_void);
    /* call mk_req.  subkey and ap_req need to be used or destroyed */
    mk_req_flags = 0x1 as libc::c_int;
    if (*ctx).gss_flags & 2 as libc::c_int as libc::c_uint != 0 {
        mk_req_flags |= 0x20000000 as libc::c_int | 0x2 as libc::c_int
    }
    krb5_auth_con_set_authdata_context(context, (*ctx).auth_context,
                                       ad_context);
    code =
        krb5_mk_req_extended(context, &mut (*ctx).auth_context, mk_req_flags,
                             0 as *mut krb5_data, k_cred, &mut ap_req);
    krb5_auth_con_set_authdata_context(context, (*ctx).auth_context,
                                       0 as krb5_authdata_context);
    krb5_free_checksum_contents(context, &mut cksum_struct.md5);
    krb5_free_data_contents(context, &mut cksum_struct.checksum_data);
    if !(code != 0) {
        /* store the interesting stuff from creds and authent */
        (*ctx).krb_times = (*k_cred).times;
        (*ctx).krb_flags = (*k_cred).ticket_flags;
        /* build up the token */
        if (*ctx).gss_flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
            /*
         * For DCE RPC, do not encapsulate the AP-REQ in the
         * typical GSS wrapping.
         */
            code = data_to_gss(&mut ap_req, token);
            if code != 0 {
                current_block = 941242863683976212;
            } else { current_block = 10692455896603418738; }
        } else {
            /* allocate space for the token */
            tlen =
                gssint_g_token_size(mech_type as *const gss_OID_desc,
                                    ap_req.length);
            t = gssalloc_malloc(tlen as size_t) as *mut libc::c_uchar;
            if t.is_null() {
                code = 12 as libc::c_int;
                current_block = 941242863683976212;
            } else {
                /* fill in the buffer */
                ptr = t;
                gssint_g_make_token_header(mech_type as *const gss_OID_desc,
                                           ap_req.length, &mut ptr,
                                           0x100 as libc::c_int);
                memcpy(ptr as *mut libc::c_void,
                       ap_req.data as *const libc::c_void,
                       ap_req.length as libc::c_ulong);
                ptr = ptr.offset(ap_req.length as isize);
                /* pass it back */
                (*token).length = tlen as size_t;
                (*token).value = t as *mut libc::c_void;
                current_block = 10692455896603418738;
            }
        }
        match current_block {
            941242863683976212 => { }
            _ => { code = 0 as libc::c_int }
        }
    }
    if !ap_req.data.is_null() {
        krb5_free_data_contents(context, &mut ap_req);
    }
    return code;
}
/*
 * new_connection
 *
 * Do the grunt work of setting up a new context.
 */
#[c2rust::src_loc = "491:1"]
unsafe extern "C" fn kg_new_connection(mut minor_status: *mut OM_uint32,
                                       mut cred: krb5_gss_cred_id_t,
                                       mut context_handle: *mut gss_ctx_id_t,
                                       mut target_name: gss_name_t,
                                       mut mech_type: gss_OID,
                                       mut req_flags: OM_uint32,
                                       mut time_req: OM_uint32,
                                       mut input_chan_bindings:
                                           gss_channel_bindings_t,
                                       mut input_token: gss_buffer_t,
                                       mut actual_mech_type: *mut gss_OID,
                                       mut output_token: gss_buffer_t,
                                       mut ret_flags: *mut OM_uint32,
                                       mut time_rec: *mut OM_uint32,
                                       mut context: krb5_context,
                                       mut exts: krb5_gss_ctx_ext_t)
 -> OM_uint32 {
    let mut current_block: u64;
    let mut major_status: OM_uint32 = 0;
    let mut code: krb5_error_code = 0;
    let mut k_cred: *mut krb5_creds = 0 as *mut krb5_creds;
    let mut ctx: *mut krb5_gss_ctx_id_rec = 0 as *mut krb5_gss_ctx_id_rec;
    let mut ctx_free: *mut krb5_gss_ctx_id_rec =
        0 as *mut krb5_gss_ctx_id_rec;
    let mut now: krb5_timestamp = 0;
    let mut token: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut keyblock: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    major_status = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    token.length = 0 as libc::c_int as size_t;
    token.value = 0 as *mut libc::c_void;
    /* make sure the cred is usable for init */
    if (*cred).usage != 1 as libc::c_int && (*cred).usage != 0 as libc::c_int
       {
        *minor_status = 0 as libc::c_int as OM_uint32;
        return (7 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* complain if the input token is non-null */
    if !input_token.is_null() &&
           (*input_token).length != 0 as libc::c_int as libc::c_ulong {
        *minor_status = 0 as libc::c_int as OM_uint32;
        return (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* create the ctx */
    ctx =
        malloc(::std::mem::size_of::<krb5_gss_ctx_id_rec>() as libc::c_ulong)
            as *mut krb5_gss_ctx_id_rec;
    if ctx.is_null() {
        *minor_status = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* fill in the ctx */
    memset(ctx as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<krb5_gss_ctx_id_rec>() as libc::c_ulong);
    (*ctx).magic = 39756040 as libc::c_long as krb5_magic;
    ctx_free = ctx;
    code = krb5_auth_con_init(context, &mut (*ctx).auth_context);
    if !(code != 0) {
        krb5_auth_con_setflags(context, (*ctx).auth_context,
                               0x4 as libc::c_int);
        /* limit the encryption types negotiated (if requested) */
        if !(*cred).req_enctypes.is_null() {
            code =
                krb5_set_default_tgs_enctypes(context, (*cred).req_enctypes);
            if code != 0 {
                current_block = 2643310281128592672;
            } else { current_block = 17478428563724192186; }
        } else { current_block = 17478428563724192186; }
        match current_block {
            2643310281128592672 => { }
            _ => {
                (*ctx).set_initiate(1 as libc::c_int as libc::c_uint);
                (*ctx).set_seed_init(0 as libc::c_int as libc::c_uint);
                (*ctx).seqstate = 0 as g_seqnum_state;
                /* enforce_ok_as_delegate causes GSS_C_DELEG_FLAG to be treated as
     * GSS_C_DELEG_POLICY_FLAG (so ok-as-delegate is always enforced). */
                if (*context).enforce_ok_as_delegate != 0 &&
                       req_flags & 1 as libc::c_int as libc::c_uint != 0 {
                    req_flags &= !(1 as libc::c_int) as libc::c_uint;
                    req_flags |= 32768 as libc::c_int as libc::c_uint
                }
                (*ctx).gss_flags =
                    req_flags &
                        (16 as libc::c_int | 32 as libc::c_int |
                             2 as libc::c_int | 4 as libc::c_int |
                             8 as libc::c_int | 1 as libc::c_int |
                             0x1000 as libc::c_int | 0x2000 as libc::c_int |
                             0x4000 as libc::c_int) as libc::c_uint;
                (*ctx).gss_flags |= 256 as libc::c_int as libc::c_uint;
                if (*cred).suppress_ci_flags() == 0 {
                    (*ctx).gss_flags |=
                        (16 as libc::c_int | 32 as libc::c_int) as
                            libc::c_uint
                }
                if req_flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
                    (*ctx).gss_flags |= 2 as libc::c_int as libc::c_uint
                }
                code = krb5_timeofday(context, &mut now);
                if !(code != 0) {
                    if time_req == 0 as libc::c_int as libc::c_uint ||
                           time_req ==
                               0xffffffff as libc::c_ulong as OM_uint32 {
                        (*ctx).krb_times.endtime = 0 as libc::c_int
                    } else {
                        (*ctx).krb_times.endtime =
                            ts_incr(now, time_req as krb5_deltat)
                    }
                    code =
                        kg_duplicate_name(context, (*cred).name,
                                          &mut (*ctx).here);
                    if !(code != 0) {
                        code =
                            kg_duplicate_name(context,
                                              target_name as krb5_gss_name_t,
                                              &mut (*ctx).there);
                        if !(code != 0) {
                            code =
                                get_credentials(context, cred, (*ctx).there,
                                                now, (*ctx).krb_times.endtime,
                                                &mut k_cred);
                            if !(code != 0) {
                                (*ctx).krb_times = (*k_cred).times;
                                /*
     * GSS_C_DELEG_POLICY_FLAG means to delegate only if the
     * ok-as-delegate ticket flag is set.
     */
                                if req_flags &
                                       32768 as libc::c_int as libc::c_uint !=
                                       0 &&
                                       (*k_cred).ticket_flags &
                                           0x40000 as libc::c_int != 0 {
                                    (*ctx).gss_flags |=
                                        (1 as libc::c_int |
                                             32768 as libc::c_int) as
                                            libc::c_uint
                                }
                                if generic_gss_copy_oid(minor_status,
                                                        mech_type as
                                                            *const gss_OID_desc,
                                                        &mut (*ctx).mech_used)
                                       != 0 as libc::c_int as libc::c_uint {
                                    code = *minor_status as krb5_error_code
                                } else {
                                    /*
     * Now try to make it static if at all possible....
     */
                                    (*ctx).mech_used =
                                        krb5_gss_convert_static_mech_oid((*ctx).mech_used);
                                    /* gsskrb5 v1 */
                                    let mut seq_temp: krb5_int32 = 0;
                                    code =
                                        make_ap_req_v1(context, ctx, cred,
                                                       k_cred,
                                                       (*(*ctx).here).ad_context,
                                                       input_chan_bindings,
                                                       mech_type, &mut token,
                                                       exts);
                                    if code != 0 {
                                        if code as libc::c_long ==
                                               -(1765328189 as libc::c_long)
                                               ||
                                               code as libc::c_long ==
                                                   -(1765328243 as
                                                         libc::c_long) ||
                                               code as libc::c_long ==
                                                   39756044 as libc::c_long {
                                            major_status =
                                                (7 as libc::c_ulong as
                                                     OM_uint32) <<
                                                    16 as libc::c_int
                                        }
                                        if code as libc::c_long ==
                                               -(1765328352 as libc::c_long) {
                                            major_status =
                                                (11 as libc::c_ulong as
                                                     OM_uint32) <<
                                                    16 as libc::c_int
                                        }
                                    } else {
                                        krb5_auth_con_getlocalseqnumber(context,
                                                                        (*ctx).auth_context,
                                                                        &mut seq_temp);
                                        (*ctx).seq_send =
                                            seq_temp as uint64_t;
                                        code =
                                            krb5_auth_con_getsendsubkey(context,
                                                                        (*ctx).auth_context,
                                                                        &mut keyblock);
                                        if !(code != 0 as libc::c_int) {
                                            code =
                                                krb5_k_create_key(context,
                                                                  keyblock,
                                                                  &mut (*ctx).subkey);
                                            krb5_free_keyblock(context,
                                                               keyblock);
                                            if !(code != 0 as libc::c_int) {
                                                (*ctx).enc = 0 as krb5_key;
                                                (*ctx).seq = 0 as krb5_key;
                                                (*ctx).set_have_acceptor_subkey(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint);
                                                code =
                                                    kg_setup_keys(context,
                                                                  ctx,
                                                                  (*ctx).subkey,
                                                                  &mut (*ctx).cksumtype);
                                                if !(code != 0 as libc::c_int)
                                                   {
                                                    if (*ctx).gss_flags &
                                                           2 as libc::c_int as
                                                               libc::c_uint ==
                                                           0 {
                                                        /* There will be no AP-REP, so set up sequence state now. */
                                                        (*ctx).seq_recv =
                                                            (*ctx).seq_send;
                                                        code =
                                                            gssint_g_seqstate_init(&mut (*ctx).seqstate,
                                                                                   (*ctx).seq_recv,
                                                                                   ((*ctx).gss_flags
                                                                                        &
                                                                                        4
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint
                                                                                        !=
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint)
                                                                                       as
                                                                                       libc::c_int,
                                                                                   ((*ctx).gss_flags
                                                                                        &
                                                                                        8
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint
                                                                                        !=
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint)
                                                                                       as
                                                                                       libc::c_int,
                                                                                   (*ctx).proto)
                                                                as
                                                                krb5_error_code;
                                                        if code !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            current_block =
                                                                2643310281128592672;
                                                        } else {
                                                            current_block =
                                                                3229571381435211107;
                                                        }
                                                    } else {
                                                        current_block =
                                                            3229571381435211107;
                                                    }
                                                    match current_block {
                                                        2643310281128592672 =>
                                                        {
                                                        }
                                                        _ =>
                                                        /* compute time_rec */
                                                        {
                                                            if !time_rec.is_null()
                                                               {
                                                                code =
                                                                    krb5_timeofday(context,
                                                                                   &mut now);
                                                                if code != 0 {
                                                                    current_block
                                                                        =
                                                                        2643310281128592672;
                                                                } else {
                                                                    *time_rec
                                                                        =
                                                                        ts_delta((*ctx).krb_times.endtime,
                                                                                 now)
                                                                            as
                                                                            OM_uint32;
                                                                    current_block
                                                                        =
                                                                        5793491756164225964;
                                                                }
                                                            } else {
                                                                current_block
                                                                    =
                                                                    5793491756164225964;
                                                            }
                                                            match current_block
                                                                {
                                                                2643310281128592672
                                                                => {
                                                                }
                                                                _ => {
                                                                    /* set the other returns */
                                                                    *output_token
                                                                        =
                                                                        token;
                                                                    if !ret_flags.is_null()
                                                                       {
                                                                        *ret_flags
                                                                            =
                                                                            (*ctx).gss_flags
                                                                    }
                                                                    if !actual_mech_type.is_null()
                                                                       {
                                                                        *actual_mech_type
                                                                            =
                                                                            mech_type
                                                                    }
                                                                    /* return successfully */
                                                                    *context_handle
                                                                        =
                                                                        ctx as
                                                                            gss_ctx_id_t;
                                                                    ctx_free =
                                                                        0 as
                                                                            *mut krb5_gss_ctx_id_rec;
                                                                    if (*ctx).gss_flags
                                                                           &
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                           !=
                                                                           0 {
                                                                        (*ctx).set_established(0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint);
                                                                        major_status
                                                                            =
                                                                            ((1
                                                                                  as
                                                                                  libc::c_int)
                                                                                 <<
                                                                                 0
                                                                                     as
                                                                                     libc::c_int
                                                                                     +
                                                                                     0
                                                                                         as
                                                                                         libc::c_int)
                                                                                as
                                                                                OM_uint32
                                                                    } else {
                                                                        (*ctx).gss_flags
                                                                            |=
                                                                            128
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint;
                                                                        (*ctx).set_established(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint);
                                                                        major_status
                                                                            =
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                                as
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
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_creds(context, k_cred);
    if !ctx_free.is_null() {
        if !(*ctx_free).auth_context.is_null() {
            krb5_auth_con_free(context, (*ctx_free).auth_context);
        }
        if !(*ctx_free).here.is_null() {
            kg_release_name(context, &mut (*ctx_free).here);
        }
        if !(*ctx_free).there.is_null() {
            kg_release_name(context, &mut (*ctx_free).there);
        }
        if !(*ctx_free).subkey.is_null() {
            krb5_k_free_key(context, (*ctx_free).subkey);
        }
        free(ctx_free as *mut libc::c_void);
    }
    *minor_status = code as OM_uint32;
    return major_status;
}
/*
 * mutual_auth
 *
 * Handle the reply from the acceptor, if we're doing mutual auth.
 */
#[c2rust::src_loc = "722:1"]
unsafe extern "C" fn mutual_auth(mut minor_status: *mut OM_uint32,
                                 mut context_handle: *mut gss_ctx_id_t,
                                 mut target_name: gss_name_t,
                                 mut mech_type: gss_OID,
                                 mut req_flags: OM_uint32,
                                 mut time_req: OM_uint32,
                                 mut input_chan_bindings:
                                     gss_channel_bindings_t,
                                 mut input_token: gss_buffer_t,
                                 mut actual_mech_type: *mut gss_OID,
                                 mut output_token: gss_buffer_t,
                                 mut ret_flags: *mut OM_uint32,
                                 mut time_rec: *mut OM_uint32,
                                 mut context: krb5_context) -> OM_uint32 {
    let mut current_block: u64;
    let mut major_status: OM_uint32 = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ap_rep: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut ap_rep_data: *mut krb5_ap_rep_enc_part =
        0 as *mut krb5_ap_rep_enc_part;
    let mut now: krb5_timestamp = 0;
    let mut ctx: *mut krb5_gss_ctx_id_rec = 0 as *mut krb5_gss_ctx_id_rec;
    let mut krb_error: *mut krb5_error = 0 as *mut krb5_error;
    let mut code: krb5_error_code = 0;
    let mut kaccess: krb5int_access =
        krb5int_access{auth_con_get_subkey_enctype: None,
                       mandatory_cksumtype: None,
                       ser_pack_int64: None,
                       ser_unpack_int64: None,
                       asn1_ldap_encode_sequence_of_keys: None,
                       asn1_ldap_decode_sequence_of_keys: None,
                       encode_krb5_auth_pack: None,
                       encode_krb5_kdc_dh_key_info: None,
                       encode_krb5_pa_pk_as_rep: None,
                       encode_krb5_pa_pk_as_req: None,
                       encode_krb5_reply_key_pack: None,
                       encode_krb5_td_dh_parameters: None,
                       encode_krb5_td_trusted_certifiers: None,
                       decode_krb5_auth_pack: None,
                       decode_krb5_pa_pk_as_req: None,
                       decode_krb5_pa_pk_as_rep: None,
                       decode_krb5_kdc_dh_key_info: None,
                       decode_krb5_principal_name: None,
                       decode_krb5_reply_key_pack: None,
                       decode_krb5_td_dh_parameters: None,
                       decode_krb5_td_trusted_certifiers: None,
                       encode_krb5_kdc_req_body: None,
                       free_kdc_req: None,
                       set_prompt_types: None,};
    major_status = (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
    code =
        krb5int_accessor(&mut kaccess,
                         ((::std::mem::size_of::<krb5int_access>() as
                               libc::c_ulong &
                               0xffff as libc::c_int as libc::c_ulong |
                               ((23 as libc::c_int) << 16 as libc::c_int) as
                                   libc::c_ulong) as krb5_int32 as
                              libc::c_uint & 0xffffffff as libc::c_uint) as
                             krb5_int32);
    if !(code != 0) {
        ctx = *context_handle as krb5_gss_ctx_id_t;
        /* make sure the context is non-established, and that certain
       arguments are unchanged */
        if (*ctx).established() as libc::c_int != 0 ||
               (*ctx).gss_flags & 2 as libc::c_int as libc::c_uint ==
                   0 as libc::c_int as libc::c_uint {
            code = 39756036 as libc::c_long as krb5_error_code
        } else if kg_compare_name(context, (*ctx).there,
                                  target_name as krb5_gss_name_t) == 0 {
            krb5_gss_delete_sec_context(minor_status, context_handle,
                                        0 as gss_buffer_t);
            code = 0 as libc::c_int;
            major_status =
                (2 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else if input_token.is_null() {
            krb5_gss_delete_sec_context(minor_status, context_handle,
                                        0 as gss_buffer_t);
            code = 0 as libc::c_int;
            major_status =
                (9 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        } else {
            ptr = (*input_token).value as *mut libc::c_uchar;
            if (*ctx).gss_flags & 0x1000 as libc::c_int as libc::c_uint != 0 {
                /* verify the token and leave the AP_REP message in ap_rep */
                /* Raw AP-REP */
                ap_rep.length = (*input_token).length as libc::c_uint;
                ap_rep.data = (*input_token).value as *mut libc::c_char;
                current_block = 7746103178988627676;
            } else if gssint_g_verify_token_header((*ctx).mech_used,
                                                   &mut ap_rep.length,
                                                   &mut ptr,
                                                   0x200 as libc::c_int,
                                                   (*input_token).length as
                                                       libc::c_uint,
                                                   1 as libc::c_int) != 0 {
                if gssint_g_verify_token_header((*ctx).mech_used as gss_OID as
                                                    *const gss_OID_desc,
                                                &mut ap_rep.length, &mut ptr,
                                                0x300 as libc::c_int,
                                                (*input_token).length as
                                                    libc::c_uint,
                                                1 as libc::c_int) ==
                       0 as libc::c_int {
                    /* Handle a KRB_ERROR message from the server */
                    sptr = ptr as *mut libc::c_char; /* PC compiler bug */
                    ap_rep.data = sptr; /* PC compiler bug */
                    sptr = sptr.offset(ap_rep.length as isize);
                    code =
                        krb5_rd_error(context, &mut ap_rep, &mut krb_error);
                    if !(code != 0) {
                        if (*krb_error).error != 0 {
                            code =
                                ((*krb_error).error as krb5_error_code as
                                     libc::c_long +
                                     -(1765328384 as libc::c_long)) as
                                    krb5_error_code
                        } else { code = 0 as libc::c_int }
                        krb5_free_error(context, krb_error);
                    }
                } else {
                    *minor_status = 0 as libc::c_int as OM_uint32;
                    return (9 as libc::c_ulong as OM_uint32) <<
                               16 as libc::c_int
                }
                current_block = 16296328345790888590;
            } else { current_block = 7746103178988627676; }
            match current_block {
                16296328345790888590 => { }
                _ => {
                    sptr = ptr as *mut libc::c_char;
                    ap_rep.data = sptr;
                    sptr = sptr.offset(ap_rep.length as isize);
                    /* decode the ap_rep */
                    code =
                        krb5_rd_rep(context, (*ctx).auth_context, &mut ap_rep,
                                    &mut ap_rep_data);
                    if code != 0 {
                        /*
         * XXX A hack for backwards compatibility.
         * To be removed in 1999 -- proven
         */
                        krb5_auth_con_setuseruserkey(context,
                                                     (*ctx).auth_context,
                                                     &mut (*(*ctx).subkey).keyblock);
                        if krb5_rd_rep(context, (*ctx).auth_context,
                                       &mut ap_rep, &mut ap_rep_data) != 0 {
                            current_block = 16296328345790888590;
                        } else { current_block = 6717214610478484138; }
                    } else { current_block = 6717214610478484138; }
                    match current_block {
                        16296328345790888590 => { }
                        _ => {
                            /* store away the sequence number */
                            (*ctx).seq_recv =
                                (*ap_rep_data).seq_number as uint64_t;
                            code =
                                gssint_g_seqstate_init(&mut (*ctx).seqstate,
                                                       (*ctx).seq_recv,
                                                       ((*ctx).gss_flags &
                                                            4 as libc::c_int
                                                                as
                                                                libc::c_uint
                                                            !=
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                                           as libc::c_int,
                                                       ((*ctx).gss_flags &
                                                            8 as libc::c_int
                                                                as
                                                                libc::c_uint
                                                            !=
                                                            0 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                                           as libc::c_int,
                                                       (*ctx).proto) as
                                    krb5_error_code;
                            if code != 0 {
                                krb5_free_ap_rep_enc_part(context,
                                                          ap_rep_data);
                            } else {
                                if !(*ap_rep_data).subkey.is_null() &&
                                       ((*ctx).proto == 1 as libc::c_int ||
                                            (*ctx).gss_flags &
                                                0x1000 as libc::c_int as
                                                    libc::c_uint != 0 ||
                                            (*(*ap_rep_data).subkey).enctype
                                                !=
                                                (*(*ctx).subkey).keyblock.enctype)
                                   {
                                    /* Keep acceptor's subkey.  */
                                    (*ctx).set_have_acceptor_subkey(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint);
                                    code =
                                        krb5_k_create_key(context,
                                                          (*ap_rep_data).subkey,
                                                          &mut (*ctx).acceptor_subkey);
                                    if code != 0 {
                                        krb5_free_ap_rep_enc_part(context,
                                                                  ap_rep_data);
                                        current_block = 16296328345790888590;
                                    } else {
                                        code =
                                            kg_setup_keys(context, ctx,
                                                          (*ctx).acceptor_subkey,
                                                          &mut (*ctx).acceptor_subkey_cksumtype);
                                        if code != 0 {
                                            krb5_free_ap_rep_enc_part(context,
                                                                      ap_rep_data);
                                            current_block =
                                                16296328345790888590;
                                        } else {
                                            current_block =
                                                168769493162332264;
                                        }
                                    }
                                } else { current_block = 168769493162332264; }
                                match current_block {
                                    16296328345790888590 => { }
                                    _ => {
                                        /* free the ap_rep_data */
                                        krb5_free_ap_rep_enc_part(context,
                                                                  ap_rep_data);
                                        if (*ctx).gss_flags &
                                               0x1000 as libc::c_int as
                                                   libc::c_uint != 0 {
                                            let mut outbuf: krb5_data =
                                                krb5_data{magic: 0,
                                                          length: 0,
                                                          data:
                                                              0 as
                                                                  *mut libc::c_char,};
                                            code =
                                                krb5_mk_rep_dce(context,
                                                                (*ctx).auth_context,
                                                                &mut outbuf);
                                            if code != 0 {
                                                current_block =
                                                    16296328345790888590;
                                            } else {
                                                code =
                                                    data_to_gss(&mut outbuf,
                                                                output_token);
                                                if code != 0 {
                                                    current_block =
                                                        16296328345790888590;
                                                } else {
                                                    current_block =
                                                        7494008139977416618;
                                                }
                                            }
                                        } else {
                                            current_block =
                                                7494008139977416618;
                                        }
                                        match current_block {
                                            16296328345790888590 => { }
                                            _ => {
                                                /* set established */
                                                (*ctx).set_established(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint);
                                                /* set returns */
                                                if !time_rec.is_null() {
                                                    code =
                                                        krb5_timeofday(context,
                                                                       &mut now);
                                                    if code != 0 {
                                                        current_block =
                                                            16296328345790888590;
                                                    } else {
                                                        *time_rec =
                                                            ts_delta((*ctx).krb_times.endtime,
                                                                     now) as
                                                                OM_uint32;
                                                        current_block =
                                                            7858101417678297991;
                                                    }
                                                } else {
                                                    current_block =
                                                        7858101417678297991;
                                                }
                                                match current_block {
                                                    16296328345790888590 => {
                                                    }
                                                    _ => {
                                                        if !ret_flags.is_null()
                                                           {
                                                            *ret_flags =
                                                                (*ctx).gss_flags
                                                        }
                                                        if !actual_mech_type.is_null()
                                                           {
                                                            *actual_mech_type
                                                                = mech_type
                                                        }
                                                        /* success */
                                                        *minor_status =
                                                            0 as libc::c_int
                                                                as OM_uint32;
                                                        return 0 as
                                                                   libc::c_int
                                                                   as
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
                }
            }
        }
    }
    krb5_gss_delete_sec_context(minor_status, context_handle,
                                0 as gss_buffer_t);
    *minor_status = code as OM_uint32;
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "909:1"]
pub unsafe extern "C" fn krb5_gss_init_sec_context_ext(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut claimant_cred_handle:
                                                           gss_cred_id_t,
                                                       mut context_handle:
                                                           *mut gss_ctx_id_t,
                                                       mut target_name:
                                                           gss_name_t,
                                                       mut mech_type: gss_OID,
                                                       mut req_flags:
                                                           OM_uint32,
                                                       mut time_req:
                                                           OM_uint32,
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
                                                           *mut OM_uint32,
                                                       mut exts:
                                                           krb5_gss_ctx_ext_t)
 -> OM_uint32 {
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut defcred: gss_cred_id_t = 0 as gss_cred_id_t;
    let mut cred: krb5_gss_cred_id_t = 0 as *mut _krb5_gss_cred_id_rec;
    let mut kerr: krb5_error_code = 0;
    let mut major_status: OM_uint32 = 0;
    let mut tmp_min_stat: OM_uint32 = 0;
    if (*context_handle).is_null() {
        kerr = krb5_gss_init_context(&mut context);
        if kerr != 0 {
            *minor_status = kerr as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        if kg_sync_ccache_name(context, minor_status) &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
            krb5_gss_save_error_info(*minor_status, context);
            krb5_free_context(context);
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else {
        context = (*(*context_handle as *mut krb5_gss_ctx_id_rec)).k5_context
    }
    /* set up return values so they can be "freed" successfully */
    major_status =
        (13 as libc::c_ulong as OM_uint32) <<
            16 as libc::c_int; /* Default major code */
    (*output_token).length = 0 as libc::c_int as size_t;
    (*output_token).value = 0 as *mut libc::c_void;
    if !actual_mech_type.is_null() { *actual_mech_type = 0 as gss_OID }
    /* verify the mech_type */
    if mech_type.is_null() ||
           (*mech_type).length == (*gss_mech_krb5).length &&
               memcmp((*mech_type).elements, (*gss_mech_krb5).elements,
                      (*mech_type).length as libc::c_ulong) ==
                   0 as libc::c_int {
        mech_type = gss_mech_krb5
    } else if (*mech_type).length == (*gss_mech_krb5_old).length &&
                  memcmp((*mech_type).elements, (*gss_mech_krb5_old).elements,
                         (*mech_type).length as libc::c_ulong) ==
                      0 as libc::c_int {
        mech_type = gss_mech_krb5_old
    } else if (*mech_type).length == (*gss_mech_krb5_wrong).length &&
                  memcmp((*mech_type).elements,
                         (*gss_mech_krb5_wrong).elements,
                         (*mech_type).length as libc::c_ulong) ==
                      0 as libc::c_int {
        mech_type = gss_mech_krb5_wrong
    } else if (*mech_type).length == (*gss_mech_iakerb).length &&
                  memcmp((*mech_type).elements, (*gss_mech_iakerb).elements,
                         (*mech_type).length as libc::c_ulong) ==
                      0 as libc::c_int {
        mech_type = gss_mech_iakerb
    } else {
        *minor_status = 0 as libc::c_int as OM_uint32;
        if (*context_handle).is_null() { krb5_free_context(context); }
        return (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* is this a new connection or not? */
    /*SUPPRESS 29*/
    if (*context_handle).is_null() {
        /* verify the credential, or use the default */
        /*SUPPRESS 29*/
        if claimant_cred_handle.is_null() {
            major_status = kg_get_defcred(minor_status, &mut defcred);
            if major_status != 0 &&
                   major_status &
                       ((0o377 as libc::c_ulong as OM_uint32) <<
                            24 as libc::c_int |
                            (0o377 as libc::c_ulong as OM_uint32) <<
                                16 as libc::c_int) != 0 {
                if (*context_handle).is_null() { krb5_free_context(context); }
                return major_status
            }
            claimant_cred_handle = defcred
        }
        major_status =
            kg_cred_resolve(minor_status, context, claimant_cred_handle,
                            target_name);
        if major_status &
               ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                    (0o377 as libc::c_ulong as OM_uint32) <<
                        16 as libc::c_int) != 0 {
            krb5_gss_save_error_info(*minor_status, context);
            krb5_gss_release_cred(&mut tmp_min_stat, &mut defcred);
            if (*context_handle).is_null() { krb5_free_context(context); }
            return major_status
        }
        cred = claimant_cred_handle as krb5_gss_cred_id_t;
        major_status =
            kg_new_connection(minor_status, cred, context_handle, target_name,
                              mech_type, req_flags, time_req,
                              input_chan_bindings, input_token,
                              actual_mech_type, output_token, ret_flags,
                              time_rec, context, exts);
        k5_mutex_unlock(&mut (*cred).lock);
        krb5_gss_release_cred(&mut tmp_min_stat, &mut defcred);
        if (*context_handle).is_null() {
            krb5_gss_save_error_info(*minor_status, context);
            krb5_free_context(context);
        } else {
            let ref mut fresh0 =
                (*(*context_handle as *mut krb5_gss_ctx_id_rec)).k5_context;
            *fresh0 = context
        }
    } else {
        /* mutual_auth doesn't care about the credentials */
        major_status =
            mutual_auth(minor_status, context_handle, target_name, mech_type,
                        req_flags, time_req, input_chan_bindings, input_token,
                        actual_mech_type, output_token, ret_flags, time_rec,
                        context)
        /* If context_handle is now NO_CONTEXT, mutual_auth called
           delete_sec_context, which would've zapped the krb5 context
           too.  */
    }
    return major_status;
}
#[no_mangle]
#[c2rust::src_loc = "1030:12"]
pub static mut kg_kdc_flag_mutex: k5_mutex_t =
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
#[c2rust::src_loc = "1031:12"]
static mut kdc_flag: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "1034:1"]
pub unsafe extern "C" fn krb5_gss_init_context(mut ctxp: *mut krb5_context)
 -> krb5_error_code {
    let mut err: krb5_error_code = 0;
    let mut is_kdc: libc::c_int = 0;
    err = gss_krb5int_initialize_library() as krb5_error_code;
    if err != 0 { return err }
    k5_mutex_lock(&mut kg_kdc_flag_mutex);
    is_kdc = kdc_flag;
    k5_mutex_unlock(&mut kg_kdc_flag_mutex);
    if is_kdc != 0 { return krb5int_init_context_kdc(ctxp) }
    return krb5_init_context(ctxp);
}
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
#[no_mangle]
#[c2rust::src_loc = "1058:1"]
pub unsafe extern "C" fn krb5int_gss_use_kdc_context(mut minor_status:
                                                         *mut OM_uint32,
                                                     desired_mech: gss_OID,
                                                     desired_object: gss_OID,
                                                     mut value: gss_buffer_t)
 -> OM_uint32 {
    let mut err: OM_uint32 = 0;
    *minor_status = 0 as libc::c_int as OM_uint32;
    err = gss_krb5int_initialize_library();
    if err != 0 { return err }
    k5_mutex_lock(&mut kg_kdc_flag_mutex);
    kdc_flag = 1 as libc::c_int;
    k5_mutex_unlock(&mut kg_kdc_flag_mutex);
    return 0 as libc::c_int as OM_uint32;
}
#[no_mangle]
#[c2rust::src_loc = "1078:1"]
pub unsafe extern "C" fn krb5_gss_init_sec_context(mut minor_status:
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
    let mut exts: krb5_gss_ctx_ext_rec =
        krb5_gss_ctx_ext_rec{iakerb:
                                 C2RustUnnamed_3{conv: 0 as *mut krb5_data,
                                                 verified: 0,},};
    memset(&mut exts as *mut krb5_gss_ctx_ext_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_gss_ctx_ext_rec>() as libc::c_ulong);
    return krb5_gss_init_sec_context_ext(minor_status, claimant_cred_handle,
                                         context_handle, target_name,
                                         mech_type, req_flags, time_req,
                                         input_chan_bindings, input_token,
                                         actual_mech_type, output_token,
                                         ret_flags, time_rec, &mut exts);
}
