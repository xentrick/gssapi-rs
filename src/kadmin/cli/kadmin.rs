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
#[c2rust::header_src = "/usr/include/bits/types.h:34"]
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
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:34"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:34"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stdarg.h:34"]
pub mod stdarg_h {
    #[c2rust::src_loc = "14:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:34"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:34"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:34"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_tm.h:34"]
pub mod struct_tm_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "7:8"]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *const libc::c_char,
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:34"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = val;
    }
    use super::stdint_uintn_h::uint32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:34"]
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
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    /* *
 * Opaque identifier for a key.
 *
 * Use with the krb5_k APIs for better performance for repeated operations with
 * the same key and usage.  Key identifiers must not be used simultaneously
 * within multiple threads, as they may contain mutable internal state and are
 * not mutex-protected.
 */
    /* to call krb5_encrypt_size, you need
                                           this.  it was a pointer, but it
                                           doesn't have to be.  gross. */
    /* checksum type */
    /* *
 * Structure to describe a region of text to be encrypted or decrypted.
 *
 * The @a flags member describes the type of the iov.
 * The @a data member points to the memory that will be manipulated.
 * All iov APIs take a pointer to the first element of an array of krb5_crypto_iov's
 * along with the size of that array. Buffer contents are manipulated in-place;
 * data is overwritten. Callers must allocate the right number of krb5_crypto_iov
 * structures before calling into an iov API.
 */
    /* *< @ref KRB5_CRYPTO_TYPE type of the iov */
    /* per Kerberos v5 protocol spec */
    /* *< @deprecated no longer supported */
    /* *< @deprecated no longer supported */
    /* *< @deprecated no longer supported */
    /* *< @deprecated no longer supported */
    /* *< @deprecated DES-3 cbc with SHA1 */
    /* *< @deprecated DES-3 cbc mode raw */
    /* *< @deprecated no longer supported */
    /* PKINIT */
    /* *< DSA with SHA1, CMS signature */
    /* *< MD5 with RSA, CMS signature */
    /* *< SHA1 with RSA, CMS signature */
    /* *< RC2 cbc mode, CMS enveloped data */
    /* *< RSA encryption, CMS enveloped data */
    /* *< RSA w/OEAP encryption, CMS enveloped data */
    /* *< DES-3 cbc mode, CMS enveloped data */
    /* *< RFC 3962 */
    /* *< RFC 3962 */
    /* *< RFC 8009 */
    /* *< RFC 8009 */
    /* *< RFC 4757 */
    /* *< RFC 4757 */
    /* *< RFC 6803 */
    /* *< RFC 6803 */
    /* des-mac-k */
/* rsa-md4-des-k */
    /* *< RFC 3962. Used with
                                                ENCTYPE_AES128_CTS_HMAC_SHA1_96 */
    /* *< RFC 3962. Used with
                                                ENCTYPE_AES256_CTS_HMAC_SHA1_96 */
    /* *< RFC 8009 */
    /* *< RFC 8009 */
    /* *< RFC 6803 */
    /* *< RFC 6803 */
    /* Microsoft netlogon */
    /* *< RFC 4757 */
    /*
 * The following are entropy source designations. Whenever
 * krb5_C_random_add_entropy is called, one of these source ids is passed in.
 * This allows the library to better estimate bits of entropy in the sample and
 * to keep track of what sources of entropy have contributed enough entropy.
 * Sources marked internal MUST NOT be used by applications outside the
 * Kerberos library
 */
    /*calls to krb5_C_RANDOM_SEED (INTERNAL)*/
    /* /dev/random or equivalent (internal)*/
    /* From KDC or other trusted party*/
    /*
     * This source should be used carefully; data in this category
     * should be from a third party trusted to give random bits
     * For example keys issued by the KDC in the application server.
     */
    /* Timing of operations*/
    /*Protocol data possibly from attacker*/
    /*Do not use; maximum source ID*/
    /* round x up to nearest multiple of y */
    /* roundup */
    /* macro function definitions to help clean up code */
    /* *
 * Encrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Data to be encrypted
 * @param [out]    output       Encrypted data
 *
 * This function encrypts the data block @a input and stores the output into @a
 * output.  The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result (using krb5_c_encrypt_length() to determine the amount
 * of space needed).  @a output->length will be set to the actual length of the
 * ciphertext.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Decrypt data using a key (operates on keyblock).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Compute encrypted data length.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  inputlen        Length of the data to be encrypted
 * @param [out] length          Length of the encrypted data
 *
 * This function computes the length of the ciphertext produced by encrypting
 * @a inputlen bytes including padding, confounder, and checksum.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return cipher block size.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] blocksize       Block size for @a enctype
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return length of the specified key in bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] keybytes        Number of bytes required to make a key
 * @param [out] keylength       Length of final key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Initialize a new cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Key
 * @param [in]  usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [out] new_state       New cipher state
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a cipher state previously allocated by krb5_c_init_state().
 *
 * @param [in] context          Library context
 * @param [in] key              Key
 * @param [in] state            Cipher state to be freed
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Generate enctype-specific pseudo-random bytes.
 *
 * @param [in]  context         Library context
 * @param [in]  keyblock        Key
 * @param [in]  input           Input data
 * @param [out] output          Output data
 *
 * This function selects a pseudo-random function based on @a keyblock and
 * computes its value over @a input, placing the result into @a output.
 * The caller must preinitialize @a output and allocate space for the
 * result, using krb5_c_prf_length() to determine the required length.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the output length of pseudo-random functions for an encryption type.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] len             Length of PRF output
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Generate pseudo-random bytes using RFC 6113 PRF+.
 *
 * @param [in]  context         Library context
 * @param [in]  k               KDC contribution key
 * @param [in]  input           Input data
 * @param [out] output          Pseudo-random output buffer
 *
 * This function fills @a output with PRF+(k, input) as defined in RFC 6113
 * section 5.1.  The caller must preinitialize @a output and allocate the
 * desired amount of space.  The length of the pseudo-random output will match
 * the length of @a output.
 *
 * @note RFC 4402 defines a different PRF+ operation.  This function does not
 * implement that operation.
 *
 * @return 0 on success, @c E2BIG if output->length is too large for PRF+ to
 * generate, @c ENOMEM on allocation failure, or an error code from
 * krb5_c_prf()
 */
    /* *
 * Derive a key using some input data (via RFC 6113 PRF+).
 *
 * @param [in]  context         Library context
 * @param [in]  k               KDC contribution key
 * @param [in]  input           Input string
 * @param [in]  enctype         Output key enctype (or @c ENCTYPE_NULL)
 * @param [out] out             Derived keyblock
 *
 * This function uses PRF+ as defined in RFC 6113 to derive a key from another
 * key and an input string.  If @a enctype is @c ENCTYPE_NULL, the output key
 * will have the same enctype as the input key.
 */
    /* *
 * Compute the KRB-FX-CF2 combination of two keys and pepper strings.
 *
 * @param [in]  context         Library context
 * @param [in]  k1              KDC contribution key
 * @param [in]  pepper1         String "PKINIT"
 * @param [in]  k2              Reply key
 * @param [in]  pepper2         String "KeyExchange"
 * @param [out] out             Output key
 *
 * This function computes the KRB-FX-CF2 function over its inputs and places
 * the results in a newly allocated keyblock.  This function is simple in that
 * it assumes that @a pepper1 and @a pepper2 are C strings with no internal
 * nulls and that the enctype of the result will be the same as that of @a k1.
 * @a k1 and @a k2 may be of different enctypes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Generate an enctype-specific random encryption key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type of the generated key
 * @param [out] k5_random_key   An allocated and initialized keyblock
 *
 * Use krb5_free_keyblock_contents() to free @a k5_random_key when
 * no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Generate an enctype-specific key from random data.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  random_data     Random input data
 * @param [out] k5_random_key   Resulting key
 *
 * This function takes random input data @a random_data and produces a valid
 * key @a k5_random_key for a given @a enctype.
 *
 * @note It is assumed that @a k5_random_key has already been initialized and
 * @a k5_random_key->contents has been allocated with the correct length.
 *
 * @sa krb5_c_keylengths()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Add entropy to the pseudo-random number generator.
 *
 * @param [in] context          Library context
 * @param [in] randsource       Entropy source (see KRB5_RANDSOURCE types)
 * @param [in] data             Data
 *
 * Contribute entropy to the PRNG used by krb5 crypto operations.  This may or
 * may not affect the output of the next crypto operation requiring random
 * data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Collect entropy from the OS if possible.
 *
 * @param [in]  context         Library context
 * @param [in]  strong          Strongest available source of entropy
 * @param [out] success         1 if OS provides entropy, 0 otherwise
 *
 * If @a strong is non-zero, this function attempts to use the strongest
 * available source of entropy.  Setting this flag may cause the function to
 * block on some operating systems.  Good uses include seeding the PRNG for
 * kadmind and realm setup.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* *
 * Convert a string (such a password) to a key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [out] key             Generated key
 *
 * This function converts @a string to a @a key of encryption type @a enctype,
 * using the specified @a salt.  The newly created @a key must be released by
 * calling krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a string (such as a password) to a key with additional parameters.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [in]  params          Parameters
 * @param [out] key             Generated key
 *
 * This function is similar to krb5_c_string_to_key(), but also takes
 * parameters which may affect the algorithm in an enctype-dependent way.  The
 * newly created @a key must be released by calling
 * krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Compare two encryption types.
 *
 * @param [in]  context         Library context
 * @param [in]  e1              First encryption type
 * @param [in]  e2              Second encryption type
 * @param [out] similar         @c TRUE if types are similar, @c FALSE if not
 *
 * This function determines whether two encryption types use the same kind of
 * keys.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Return the length of checksums for a checksum type.
 *
 * @param [in]  context         Library context
 * @param [in]  cksumtype       Checksum type
 * @param [out] length          Checksum length
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return a list of keyed checksum types usable with an encryption type.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [out] count           Count of allowable checksum types
 * @param [out] cksumtypes      Array of allowable checksum types
 *
 * Use krb5_free_cksumtypes() to free @a cksumtypes when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* * @defgroup KRB5_KEYUSAGE KRB5_KEYUSAGE
 * @{
 */
    /* XXX need to register these */
    /* Defined in Integrating SAM Mechanisms with Kerberos draft */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REQUEST */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_S4U_X509_USER_REPLY */
    /* Defined in [MS-SFU] */
/* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_CHALLENGE_TRACKID */
    /* * Note conflict with @ref KRB5_KEYUSAGE_PA_SAM_RESPONSE */
    /* unused */
    /* *< See RFC 6560 section 4.2 */
    /* define in draft-ietf-krb-wg-preauth-framework*/
    /* Key usage values 512-1023 are reserved for uses internal to a Kerberos
 * implementation. */
    /* *< Used for encrypted FAST cookies */
    /* *< Used for freshness tokens */
    /* * @} */
    /* end of KRB5_KEYUSAGE group */
    /* *
 * Verify that a specified encryption type is a valid Kerberos encryption type.
 *
 * @param [in] ktype            Encryption type
 *
 * @return @c TRUE if @a ktype is valid, @c FALSE if not
 */
    /* *
 * Verify that specified checksum type is a valid Kerberos checksum type.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is valid, @c FALSE if not
 */
    /* *
 * Test whether a checksum type is collision-proof.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is collision-proof, @c FALSE if it is not
 * collision-proof or not a valid checksum type.
 */
    /* *
 * Test whether a checksum type is keyed.
 *
 * @param [in] ctype            Checksum type
 *
 * @return @c TRUE if @a ctype is a keyed checksum type, @c FALSE otherwise.
 */
    /* AEAD APIs */
/* * @defgroup KRB5_CRYPTO_TYPE KRB5_CRYPTO_TYPE
 * @{
 */
    /* *< [in] ignored */
    /* *< [out] header */
    /* *< [in, out] plaintext */
    /* *< [in] associated data */
    /* *< [out] padding */
    /* *< [out] checksum for encrypt */
    /* *< [out] checksum for MIC */
    /* *< [in] entire message without
                                           decomposing the structure into
                                           header, data and trailer buffers */
    /* * @} */
    /* end of KRB5_CRYPTO_TYPE group */
    /* *
 * Fill in a checksum element in IOV array (operates on keyblock)
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] data            IOV array
 * @param [in]     num_data        Size of @a data
 *
 * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
 * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
 * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
 *
 * @note This function is similar to krb5_k_make_checksum_iov(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_verify_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Validate a checksum element in IOV array (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     data            IOV array
 * @param [in]     num_data        Size of @a data
 * @param [out]    valid           Non-zero for success, zero for failure
 *
 * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
 * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
 * regions in the iov.
 *
 * @note This function is similar to krb5_k_verify_checksum_iov(), but operates
 * on keyblock @a key.
 *
 * @sa krb5_c_make_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Encrypt data in place supporting AEAD (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     keyblock        Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function encrypts the data block @a data and stores the output in-place.
 * The actual encryption key will be derived from @a keyblock and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_k_encrypt_iov(), but operates
 * on keyblock @a keyblock.
 *
 * @sa krb5_c_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Decrypt data in place supporting AEAD (operates on keyblock).
 *
 * @param [in]     context         Library context
 * @param [in]     keyblock        Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function decrypts the data block @a data and stores the output in-place.
 * The actual decryption key will be derived from @a keyblock and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_k_decrypt_iov(), but operates
 * on keyblock @a keyblock.
 *
 * @sa krb5_c_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return a length of a message field specific to the encryption type.
 *
 * @param [in]  context      Library context
 * @param [in]  enctype      Encryption type
 * @param [in]  type         Type field (See @ref KRB5_CRYPTO_TYPE types)
 * @param [out] size         Length of the @a type specific to @a enctype
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Fill in lengths for header, trailer and padding in a IOV array.
 *
 * @param [in]      context      Library context
 * @param [in]      enctype      Encryption type
 * @param [in,out]  data         IOV array
 * @param [in]      num_data     Size of @a data
 *
 * Padding is set to the actual padding required based on the provided
 * @a data buffers. Typically this API is used after setting up the data
 * buffers and #KRB5_CRYPTO_TYPE_SIGN_ONLY buffers, but before actually
 * allocating header, trailer and padding.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return a number of padding octets.
 *
 * @param [in]  context      Library context
 * @param [in]  enctype      Encryption type
 * @param [in]  data_length  Length of the plaintext to pad
 * @param [out] size         Number of padding octets
 *
 * This function returns the number of the padding octets required to pad
 * @a data_length octets of plaintext.
 *
 * @retval 0 Success; otherwise - KRB5_BAD_ENCTYPE
 */
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
    /* * Increment the reference count on a key. */
    /* * Decrement the reference count on a key and free it if it hits zero. */
    /* * Retrieve a copy of the keyblock from a krb5_key structure. */
    /* * Retrieve the enctype of a krb5_key structure. */
    /* *
 * Encrypt data using a key (operates on opaque key).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Data to be encrypted
 * @param [out]    output       Encrypted data
 *
 * This function encrypts the data block @a input and stores the output into @a
 * output.  The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result (using krb5_c_encrypt_length() to determine the amount
 * of space needed).  @a output->length will be set to the actual length of the
 * ciphertext.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Encrypt data in place supporting AEAD (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     key             Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function encrypts the data block @a data and stores the output in-place.
 * The actual encryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the encryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_encrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_c_encrypt_iov(), but operates
 * on opaque key @a key.
 *
 * @sa krb5_k_decrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Decrypt data using a key (operates on opaque key).
 *
 * @param [in]     context      Library context
 * @param [in]     key          Encryption key
 * @param [in]     usage        Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] cipher_state Cipher state; specify NULL if not needed
 * @param [in]     input        Encrypted data
 * @param [out]    output       Decrypted data
 *
 * This function decrypts the data block @a input and stores the output into @a
 * output. The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 *
 * @note The caller must initialize @a output and allocate at least enough
 * space for the result.  The usual practice is to allocate an output buffer as
 * long as the ciphertext, and let krb5_c_decrypt() trim @a output->length.
 * For some enctypes, the resulting @a output->length may include padding
 * bytes.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Decrypt data in place supporting AEAD (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     key             Encryption key
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     cipher_state    Cipher state; specify NULL if not needed
 * @param [in,out] data            IOV array. Modified in-place.
 * @param [in]     num_data        Size of @a data
 *
 * This function decrypts the data block @a data and stores the output in-place.
 * The actual decryption key will be derived from @a key and @a usage
 * if key derivation is specified for the encryption type.  If non-null, @a
 * cipher_state specifies the beginning state for the decryption operation, and
 * is updated with the state to be passed as input to the next operation.
 * The caller must allocate the right number of krb5_crypto_iov
 * structures before calling into this API.
 *
 * @note On return from a krb5_c_decrypt_iov() call, the @a data->length in the
 * iov structure are adjusted to reflect actual lengths of the ciphertext used.
 * For example, if the padding length is too large, the length will be reduced.
 * Lengths are never increased.
 *
 * @note This function is similar to krb5_c_decrypt_iov(), but operates
 * on opaque key @a key.
 *
 * @sa krb5_k_encrypt_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Fill in a checksum element in IOV array (operates on opaque key)
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in,out] data            IOV array
 * @param [in]     num_data        Size of @a data
 *
 * Create a checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element over
 * #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY chunks in @a data.
 * Only the #KRB5_CRYPTO_TYPE_CHECKSUM region is modified.
 *
 * @note This function is similar to krb5_c_make_checksum_iov(), but operates
 * on opaque @a key.
 *
 * @sa krb5_k_verify_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Validate a checksum element in IOV array (operates on opaque key).
 *
 * @param [in]     context         Library context
 * @param [in]     cksumtype       Checksum type (0 for mandatory type)
 * @param [in]     key             Encryption key for a keyed checksum
 * @param [in]     usage           Key usage (see @ref KRB5_KEYUSAGE types)
 * @param [in]     data            IOV array
 * @param [in]     num_data        Size of @a data
 * @param [out]    valid           Non-zero for success, zero for failure
 *
 * Confirm that the checksum in the #KRB5_CRYPTO_TYPE_CHECKSUM element is a
 * valid checksum of the #KRB5_CRYPTO_TYPE_DATA and #KRB5_CRYPTO_TYPE_SIGN_ONLY
 * regions in the iov.
 *
 * @note This function is similar to krb5_c_verify_checksum_iov(), but operates
 * on opaque @a key.
 *
 * @sa krb5_k_make_checksum_iov()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Generate enctype-specific pseudo-random bytes (operates on opaque key).
 *
 * @param [in]  context      Library context
 * @param [in]  key          Key
 * @param [in]  input        Input data
 * @param [out] output       Output data
 *
 * This function selects a pseudo-random function based on @a key and
 * computes its value over @a input, placing the result into @a output.
 * The caller must preinitialize @a output and allocate space for the
 * result.
 *
 * @note This function is similar to krb5_c_prf(), but operates
 * on opaque @a key.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /*
 * old cryptosystem routine prototypes.  These are now layered
 * on top of the functions above.
 */
/* * @deprecated Replaced by krb5_c_* API family.*/
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated See krb5_c_string_to_key() */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated Replaced by krb5_c_* API family. */
    /* * @deprecated See krb5_c_checksum_length() */
    /* * @deprecated See krb5_c_make_checksum() */
    /* * @deprecated See krb5_c_verify_checksum() */
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
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for auth data */
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * Structure for transited encoding */
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Encrypted part of ticket. */
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
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* the unencrypted version */
/* *
 * Ticket authenticator.
 *
 * The C representation of an unencrypted authenticator.
 */
    /* *< client name/realm */
    /* *< checksum, includes type, optional */
    /* *< client usec portion */
    /* *< client sec portion */
    /* *< true session key, optional */
    /* *< sequence #, optional */
    /* *< authoriazation data */
    /* * Ticket authentication data. */
    /* * Credentials structure including ticket, session key, and lifetime info. */
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
    /* * Last request entry */
    /* *< LR type */
    /* *< Timestamp */
    /* * Pre-authentication data */
    /* *< Preauthentication data type */
    /* *< Length of data */
    /* *< Data */
    /* Don't use this; use krb5_pa_data instead. */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
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
    /* cleartext part: */
    /* *< KRB5_AS_REP or KRB5_KDC_REP */
    /* *< Preauthentication data from KDC */
    /* *< Client principal and realm */
    /* *< Ticket */
    /* *< Encrypted part of reply */
    /* *< Unencrypted version, if available */
    /* * Error message structure */
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
    /* *< Requested options */
    /* *< Ticket */
    /* *< Encrypted authenticator */
    /* *
 * C representaton of AP-REP message.
 *
 * The server's response to a client's request for mutual authentication.
 */
    /* *< Ciphertext of ApRepEncPart */
    /* * Cleartext that is encrypted and put into @c _krb5_ap_rep.  */
    /* *< Client time, seconds portion */
    /* *< Client time, microseconds portion */
    /* *< Subkey (optional) */
    /* *< Sequence number */
    /* Unused */
    /* * Credentials information inserted into @c EncKrbCredPart. */
    /* *< Session key used to encrypt ticket */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Ticket flags */
    /* *< Auth, start, end, renew_till */
    /* *< Array of pointers to addrs (optional) */
    /* * Cleartext credentials information.  */
    /* *< Nonce (optional) */
    /* *< Generation time, seconds portion */
    /* *< Generation time, microseconds portion */
    /* *< Sender address (optional) */
    /* *< Recipient address (optional) */
    /* * Credentials data structure.*/
    /* *< Tickets */
    /* *< Encrypted part */
    /* *< Unencrypted version, if available */
    /* Unused, but here for API compatibility. */
    /* Unused, but here for API compatibility. */
    /* Unused, but here for API compatibility. */
    /* * Referred name, only realm is required */
    /* Unused, but here for API compatibility. */
    /* * TRUE if a PAC should be included in TGS-REP */
    /*
 * begin "safepriv.h"
 */
    /* * @defgroup KRB5_AUTH_CONTEXT KRB5_AUTH_CONTEXT
 * @{
 */
/* * Prevent replays with timestamps and replay cache. */
    /* * Save timestamps for application. */
    /* * Prevent replays with sequence numbers. */
    /* * Save sequence numbers for application. */
    /* * @} */
    /* end of KRB5_AUTH_CONTEXT group */
    /* *
 * Replay data.
 *
 * Sequence number and timestamp information output by krb5_rd_priv() and
 * krb5_rd_safe().
 */
    /* *< Timestamp, seconds portion */
    /* *< Timestamp, microseconds portion */
    /* *< Sequence number  */
    /* Flags for krb5_auth_con_genaddrs(). */
    /* * Generate the local network address. */
    /* * Generate the remote network address.  */
    /* * Generate the local network address and the local port. */
    /* * Generate the remote network address and the remote port. */
    /* * Type of function used as a callback to generate checksum data for mk_req */
    /*
 * end "safepriv.h"
 */
    /*
 * begin "ccache.h"
 */
    /* * Cursor for sequential lookup */
    /* * Cursor for iterating over all ccaches */
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
    /* *
 * Store credentials in a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] creds            Credentials to be stored in cache
 *
 * This function stores @a creds into @a cache.  If @a creds->server and the
 * server in the decoded ticket @a creds->ticket differ, the credentials will
 * be stored under both server principal names.
 *
 * @retval
 *  0  Success
 * @return Permission errors; storage failure errors; Kerberos error codes
 */
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
    /* *
 * Remove credentials from a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 * @param [in] flags            Bitwise-ORed search flags
 * @param [in] creds            Credentials to be matched
 *
 * @warning This function is not implemented for some cache types.
 *
 * This function accepts the same flag values as krb5_cc_retrieve_cred().
 *
 * @retval KRB5_CC_NOSUPP Not implemented for this cache type
 * @return No matches found; Data cannot be deleted; Kerberos error codes
 */
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
    /* *
 * Retrieve flags from a credential cache structure.
 *
 * @param [in]  context         Library context
 * @param [in]  cache           Credential cache handle
 * @param [out] flags           Flag bit mask
 *
 * @warning For memory credential cache always returns a flag mask of 0.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the type of a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * @return The type of a credential cache as an alias that must not be modified
 * or freed by the caller.
 */
    /* *
 * Move a credential cache.
 *
 * @param [in] context          Library context
 * @param [in] src              The credential cache to move the content from
 * @param [in] dst              The credential cache to move the content to
 *
 * This function reinitializes @a dst and populates it with the credentials and
 * default principal of @a src; then, if successful, destroys @a src.
 *
 * @retval
 * 0 Success; @a src is closed.
 * @return
 * Kerberos error codes; @a src is still allocated.
 */
    /* *
 * Prepare to iterate over the collection of known credential caches.
 *
 * @param [in]  context         Library context
 * @param [out] cursor          Cursor
 *
 * Get a new cache iteration @a cursor that will iterate over all known
 * credential caches independent of type.
 *
 * Use krb5_cccol_cursor_free() to release @a cursor when it is no longer
 * needed.
 *
 * @sa krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the next credential cache in the collection.
 *
 * @param [in]  context         Library context
 * @param [in]  cursor          Cursor
 * @param [out] ccache          Credential cache handle
 *
 * @note When all caches are iterated over and the end of the list is reached,
 * @a ccache is set to NULL.
 *
 * Use krb5_cc_close() to close @a ccache when it is no longer needed.
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_free()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a credential cache collection cursor.
 *
 * @param [in] context          Library context
 * @param [in] cursor           Cursor
 *
 * @sa krb5_cccol_cursor_new(), krb5_cccol_cursor_next()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /*
 * end "ccache.h"
 */
    /*
 * begin "rcache.h"
 */
    /*
 * end "rcache.h"
 */
    /*
 * begin "keytab.h"
 */
    /* XXX */
    /* *< Long enough for MAXPATHLEN + some extra */
    /* * A key table entry. */
    /* *< Principal of this key */
    /* *< Time entry written to keytable */
    /* *< Key version number */
    /* *< The secret key */
    /* *
 * Return the type of a key table.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @return The type of a key table as an alias that must not be modified or
 * freed by the caller.
 */
    /* *
 * Get a key table name.
 *
 * @param [in]  context         Library context
 * @param [in]  keytab          Key table handle
 * @param [out] name            Key table name
 * @param [in]  namelen         Maximum length to fill in name
 *
 * Fill @a name with the name of @a keytab including the type and delimiter.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_KT_NAME_TOOLONG  Key table name does not fit in @a namelen bytes
 *
 * @return
 * Kerberos error codes
 */
    /* *
 * Close a key table handle.
 *
 * @param [in] context          Library context
 * @param [in] keytab           Key table handle
 *
 * @retval 0
 */
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
    /* *
 * Create a krb5 library context using a specified profile.
 *
 * @param [in]  profile         Profile object (NULL to create default profile)
 * @param [in]  flags           Context initialization flags
 * @param [out] context         Library context
 *
 * Create a context structure, optionally using a specified profile and
 * initialization flags.  If @a profile is NULL, the default profile will be
 * created from config files.  If @a profile is non-null, a copy of it will be
 * made for the new context; the caller should still clean up its copy.  Valid
 * flag values are:
 *
 * @li #KRB5_INIT_CONTEXT_SECURE Ignore environment variables
 * @li #KRB5_INIT_CONTEXT_KDC    Use KDC configuration if creating profile
 */
    /* *
 * Free a krb5 library context.
 *
 * @param [in] context          Library context
 *
 * This function frees a @a context that was created by krb5_init_context()
 * or krb5_init_secure_context().
 */
    /* *
 * Copy a krb5_context structure.
 *
 * @param [in]  ctx             Library context
 * @param [out] nctx_out        New context structure
 *
 * The newly created context must be released by calling krb5_free_context()
 * when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
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
    /* *
 * Return a list of encryption types permitted for session keys.
 *
 * @param [in]  context         Library context
 * @param [out] ktypes          Zero-terminated list of encryption types
 *
 * This function returns the list of encryption types permitted for session
 * keys within @a context, as determined by configuration or by a previous call
 * to krb5_set_default_tgs_enctypes().
 *
 * Use krb5_free_enctypes() to free @a ktypes when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Test whether the Kerberos library was built with multithread support.
 *
 * @retval
 * TRUE if the library is threadsafe; FALSE otherwise
 */
    /* libkrb.spec */
    /* *
 * Decrypt a ticket using the specified key table.
 *
 * @param [in] context          Library context
 * @param [in] kt               Key table
 * @param [in] ticket           Ticket to be decrypted
 *
 * This function takes a @a ticket as input and decrypts it using
 * key data from @a kt.  The result is placed into @a ticket->enc_part2.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free an array of credential structures.
 *
 * @param [in] context          Library context
 * @param [in] tgts             Null-terminated array of credentials to free
 *
 * @note The last entry in the array @a tgts must be a NULL pointer.
 */
    /* * @defgroup KRB5_GC  KRB5_GC
 * @{
 */
    /* *< Want user-user ticket */
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
    /* * @deprecated Replaced by krb5_get_validated_creds. */
    /* * @deprecated Replaced by krb5_get_renewed_creds. */
    /* *
 * Create a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     service        Service name, or NULL to use @c "host"
 * @param [in]     hostname       Host name, or NULL to use local hostname
 * @param [in]     in_data        Application data to be checksummed in the
 *                                authenticator, or NULL
 * @param [in]     ccache         Credential cache used to obtain credentials
 *                                for the desired service.
 * @param [out]    outbuf         @c AP-REQ message
 *
 * This function is similar to krb5_mk_req_extended() except that it uses a
 * given @a hostname, @a service, and @a ccache to construct a service
 * principal name and obtain credentials.
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Format and encrypt a @c KRB_AP_REP message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] outbuf          @c AP-REP message
 *
 * This function fills in @a outbuf with an AP-REP message using information
 * from @a auth_context.
 *
 * If the flags in @a auth_context indicate that a sequence number should be
 * used (either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or
 * #KRB5_AUTH_CONTEXT_RET_SEQUENCE) and the local sequence number in @a
 * auth_context is 0, a new number will be generated with
 * krb5_generate_seq_number().
 *
 * Use krb5_free_data_contents() to free @a outbuf when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Parse and decrypt a @c KRB_AP_REP message for DCE RPC.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           AP-REP message
 * @param [out] nonce           Sequence number from the decrypted reply
 *
 * This function parses, decrypts and verifies a message from @a inbuf and
 * fills in @a nonce with a decrypted reply sequence number.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Process @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  inbuf           @c KRB-SAFE message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-SAFE message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-SAFE message, verifies its integrity, and
 * stores its data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-SAFE message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-SAFE message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-SAFE message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Process a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication structure
 * @param [in]  inbuf           @c KRB-PRIV message to be parsed
 * @param [out] userdata_out    Data parsed from @c KRB-PRIV message
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function parses a @c KRB-PRIV message, verifies its integrity, and
 * stores its unencrypted data into @a userdata_out.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If @a auth_context has a remote address set, the address will be used to
 * verify the sender address in the KRB-PRIV message.  If @a auth_context has a
 * local address set, it will be used to verify the receiver address in the
 * KRB-PRIV message if the message contains one.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag is set in @a auth_context, the
 * sequence number of the KRB-PRIV message is checked against the remote
 * sequence number field of @a auth_context.  Otherwise, the sequence number is
 * not used.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, then the
 * timestamp in the message is verified to be within the permitted clock skew
 * of the current time, and the message is checked against an in-memory replay
 * cache to detect reflections or replays.
 *
 * Use krb5_free_data_contents() to free @a userdata_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *< Error if realm is present */
    /* *< Error if realm is not present */
    /* *< Create single-component
                                                  enterprise principle */
    /* *< Ignore realm if present */
    /* *
 * Convert a string principal name to a krb5_principal with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [in]  flags           Flag
 * @param [out] principal_out   New principal
 *
 * Similar to krb5_parse_name(), this function converts a single-string
 * representation of a principal name to a krb5_principal structure.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_PARSE_NO_REALM - no realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_REQUIRE_REALM - realm must be present in @a name
 * @li #KRB5_PRINCIPAL_PARSE_ENTERPRISE - create single-component enterprise
 *                                        principal
 * @li #KRB5_PRINCIPAL_PARSE_IGNORE_REALM - ignore realm if present in @a name
 *
 * If @c KRB5_PRINCIPAL_PARSE_NO_REALM or @c KRB5_PRINCIPAL_PARSE_IGNORE_REALM
 * is specified in @a flags, the realm of the new principal will be empty.
 * Otherwise, the default realm for @a context will be used if @a name does not
 * specify a realm.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
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
    /* *
 * Convert krb5_principal structure to string and length.
 *
 * @param [in]     context      Library context
 * @param [in]     principal    Principal
 * @param [in,out] name         String representation of principal name
 * @param [in,out] size         Size of unparsed name
 *
 * This function is similar to krb5_unparse_name(), but allows the use of an
 * existing buffer for the result.  If size is not NULL, then @a name must
 * point to either NULL or an existing buffer of at least the size pointed to
 * by @a size.  The buffer will be allocated or resized if necessary, with the
 * new pointer stored into @a name.  Whether or not the buffer is resized, the
 * necessary space for the result, including null terminator, will be stored
 * into @a size.
 *
 * If size is NULL, this function behaves exactly as krb5_unparse_name().
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
    /* *< Omit realm if it is the local realm */
    /* *< Omit realm always */
    /* *< Don't escape special characters */
    /* *
 * Convert krb5_principal structure to a string with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            String representation of principal name
 *
 * Similar to krb5_unparse_name(), this function converts a krb5_principal
 * structure to a string representation.
 *
 * The following flags are valid:
 * @li #KRB5_PRINCIPAL_UNPARSE_SHORT - omit realm if it is the local realm
 * @li #KRB5_PRINCIPAL_UNPARSE_NO_REALM - omit realm
 * @li #KRB5_PRINCIPAL_UNPARSE_DISPLAY - do not quote special characters
 *
 * Use krb5_free_unparsed_name() to free @a name when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
    /* *
 * Convert krb5_principal structure to string format with flags.
 *
 * @param [in]  context         Library context
 * @param [in]  principal       Principal
 * @param [in]  flags           Flags
 * @param [out] name            Single string format of principal name
 * @param [out] size            Size of unparsed name buffer
 *
 * @sa krb5_unparse_name() krb5_unparse_name_flags() krb5_unparse_name_ext()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes. On failure @a name is set to NULL
 */
    /* *
 * Set the realm field of a principal
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] realm            Realm name
 *
 * Set the realm name part of @a principal to @a realm, overwriting the
 * previous realm.
 *
 * @retval
 * 0   Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Search a list of addresses for a specified address.
 *
 * @param [in] context          Library context
 * @param [in] addr             Address to search for
 * @param [in] addrlist         Address list to be searched (or NULL)
 *
 * @note If @a addrlist contains only a NetBIOS addresses, it will be treated
 *       as a null list.
 *
 * @return
 * TRUE if @a addr is listed in @a addrlist, or @c addrlist is NULL; FALSE
 * otherwise
 */
    /* *
 * Compare two Kerberos addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address to be compared
 * @param [in] addr2            Second address to be compared
 *
 * @return
 * TRUE if the addresses are the same, FALSE otherwise
 */
    /* *
 * Return an ordering of the specified addresses.
 *
 * @param [in] context          Library context
 * @param [in] addr1            First address
 * @param [in] addr2            Second address
 *
 * @retval
 *  0 The two addresses are the same
 * @retval
 *  \< 0 First address is less than second
 * @retval
 *  \> 0 First address is greater than second
 */
    /* *
 * Compare the realms of two principals.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * @retval
 * TRUE if the realm names are the same; FALSE otherwise
 */
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
    /* *
 * Compare two principals ignoring realm components.
 *
 * @param [in] context          Library context
 * @param [in] princ1           First principal
 * @param [in] princ2           Second principal
 *
 * Similar to krb5_principal_compare(), but do not compare the realm
 * components of the principals.
 *
 * @retval
 * TRUE if the principals are the same; FALSE otherwise
 */
    /* *< ignore realm component */
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
    /* *
 * Initialize an empty @c krb5_keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  length          Length of keyblock (or 0)
 * @param [out] out             New keyblock structure
 *
 * Initialize a new keyblock and allocate storage for the contents of the key.
 * It is legal to pass in a length of 0, in which case contents are left
 * unallocated.  Use krb5_free_keyblock() to free @a out when it is no longer
 * needed.
 *
 * @note If @a length is set to 0, contents are left unallocated.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Copy a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Keyblock to be copied
 * @param [out] to              Copy of keyblock @a from
 *
 * This function creates a new keyblock with the same contents as @a from.  Use
 * krb5_free_keyblock() to free @a to when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Copy a krb5_data object.
 *
 * @param [in]  context           Library context
 * @param [in]  indata            Data object to be copied
 * @param [out] outdata           Copy of @a indata
 *
 * This function creates a new krb5_data object with the contents of @a indata.
 * Use krb5_free_data() to free @a outdata when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Copy an array of addresses.
 *
 * @param [in]  context         Library context
 * @param [in]  inaddr          Array of addresses to be copied
 * @param [out] outaddr         Copy of array of addresses
 *
 * This function creates a new address array containing a copy of @a inaddr.
 * Use krb5_free_addresses() to free @a outaddr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Copy a krb5_ticket structure.
 *
 * @param [in]  context         Library context
 * @param [in]  from            Ticket to be copied
 * @param [out] pto             Copy of ticket
 *
 * This function creates a new krb5_ticket structure containing the contents of
 * @a from.  Use krb5_free_ticket() to free @a pto when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Copy an authorization data list.
 *
 * @param [in]  context         Library context
 * @param [in]  in_authdat      List of @a krb5_authdata structures
 * @param [out] out             New array of @a krb5_authdata structures
 *
 * This function creates a new authorization data list containing a copy of @a
 * in_authdat, which must be null-terminated.  Use krb5_free_authdata() to free
 * @a out when it is no longer needed.
 *
 * @note The last array entry in @a in_authdat must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Find authorization data elements.
 *
 * @param [in]  context         Library context
 * @param [in]  ticket_authdata Authorization data list from ticket
 * @param [in]  ap_req_authdata Authorization data list from AP request
 * @param [in]  ad_type         Authorization data type to find
 * @param [out] results         List of matching entries
 *
 * This function searches @a ticket_authdata and @a ap_req_authdata for
 * elements of type @a ad_type.  Either input list may be NULL, in which case
 * it will not be searched; otherwise, the input lists must be terminated by
 * NULL entries.  This function will search inside AD-IF-RELEVANT containers if
 * found in either list.  Use krb5_free_authdata() to free @a results when it
 * is no longer needed.
 *
 * @version New in 1.10
 */
    /* *
 * Merge two authorization data lists into a new list.
 *
 * @param [in]  context         Library context
 * @param [in]  inauthdat1      First list of @a krb5_authdata structures
 * @param [in]  inauthdat2      Second list of @a krb5_authdata structures
 * @param [out] outauthdat      Merged list of @a krb5_authdata structures
 *
 * Merge two authdata arrays, such as the array from a ticket
 * and authenticator.
 * Use krb5_free_authdata() to free @a outauthdat when it is no longer needed.
 *
 * @note The last array entry in @a inauthdat1 and @a inauthdat2
 * must be a NULL pointer.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Copy a krb5_authenticator structure.
 *
 * @param [in]  context         Library context
 * @param [in]  authfrom        krb5_authenticator structure to be copied
 * @param [out] authto          Copy of krb5_authenticator structure
 *
 * This function creates a new krb5_authenticator structure with the content of
 * @a authfrom.  Use krb5_free_authenticator() to free @a authto when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Copy a krb5_checksum structure.
 *
 * @param [in]  context         Library context
 * @param [in]  ckfrom          Checksum to be copied
 * @param [out] ckto            Copy of krb5_checksum structure
 *
 * This function creates a new krb5_checksum structure with the contents of @a
 * ckfrom.  Use krb5_free_checksum() to free @a ckto when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Build a principal name using null-terminated strings.
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal name
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ...             List of char * components, ending with NULL
 *
 * Call krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @note krb5_build_principal() and krb5_build_principal_alloc_va() perform the
 * same task.  krb5_build_principal() takes variadic arguments.
 * krb5_build_principal_alloc_va() takes a pre-computed @a varargs pointer.
 *
 * @code
 * Example of how to build principal H/S@R
 *     krb5_build_principal(context, &principal,
 *                          strlen("R"), "R", "H", "S", (char*)NULL);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_build_principal_alloc_va(). */
    /* *
 * Build a principal name, using a precomputed variable argument list
 *
 * @param [in]  context         Library context
 * @param [out] princ           Principal structure
 * @param [in]  rlen            Realm name length
 * @param [in]  realm           Realm name
 * @param [in]  ap              List of char * components, ending with NULL
 *
 * Similar to krb5_build_principal(), this function builds a principal name,
 * but its name components are specified as a va_list.
 *
 * Use krb5_free_principal() to deallocate @a princ when it is no longer
 * needed.
 *
 * @code
 * Function usage example:
 *   va_list ap;
 *   va_start(ap, realm);
 *   krb5_build_principal_alloc_va(context, princ, rlen, realm, ap);
 *   va_end(ap);
 * @endcode
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Convert a Kerberos V4 principal to a Kerberos V5 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  name            V4 name
 * @param [in]  instance        V4 instance
 * @param [in]  realm           Realm
 * @param [out] princ           V5 principal
 *
 * This function builds a @a princ from V4 specification based on given input
 * @a name.instance\@realm.
 *
 * Use krb5_free_principal() to free @a princ when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a Kerberos V5 principal to a Kerberos V4 principal.
 *
 * @param [in]  context         Library context
 * @param [in]  princ           V5 Principal
 * @param [out] name            V4 principal's name to be filled in
 * @param [out] inst            V4 principal's instance name to be filled in
 * @param [out] realm           Principal's realm name to be filled in
 *
 * This function separates a V5 principal @a princ into @a name, @a instance,
 * and @a realm.
 *
 * @retval
 *  0  Success
 * @retval
 *  KRB5_INVALID_PRINCIPAL   Invalid principal name
 * @retval
 *  KRB5_CONFIG_CANTOPEN     Can't open or find Kerberos configuration file
 * @return
 * Kerberos error codes
 */
    /* *
 *@deprecated
 */
    /* *
 * Convert a Kerberos V5 credentials to a Kerberos V4 credentials
 *
 * @note Not implemented
 *
 * @retval KRB524_KRB4_DISABLED (always)
 */
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
    /* *
 * Get the default key table name.
 *
 * @param [in]     context      Library context
 * @param [out]    name         Default key table name
 * @param [in]     name_size    Space available in @a name
 *
 * Fill @a name with the name of the default key table for @a context.
 *
 * @sa MAX_KEYTAB_NAME_LEN
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_CONFIG_NOTENUFSPACE Buffer is too short
 * @return
 * Kerberos error codes
 */
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
    /* * @deprecated Use krb5_free_keytab_entry_contents instead. */
    /* remove and add are functions, so that they can return NOWRITE
   if not a writable keytab */
    /* *
 * Remove an entry from a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to remove from key table
 *
 * @retval
 * 0 Success
 * @retval
 *  KRB5_KT_NOWRITE     Key table is not writable
 * @return
 * Kerberos error codes
 */
    /* *
 * Add a new entry to a key table.
 *
 * @param [in] context          Library context
 * @param [in] id               Key table handle
 * @param [in] entry            Entry to be added
 *
 * @retval
 * 0  Success
 * @retval
 *  ENOMEM    Insufficient memory
 * @retval
 *  KRB5_KT_NOWRITE  Key table is not writeable
 * @return
 * Kerberos error codes
 */
    /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Set the default credential cache name.
 *
 * @param [in] context          Library context
 * @param [in] name             Default credential cache name or NULL
 *
 * Set the default credential cache name to @a name for future operations using
 * @a context.  If @a name is NULL, clear any previous application-set default
 * name and forget any cached value of the default name for @a context.
 *
 * Calls to this function invalidate the result of any previous calls to
 * krb5_cc_default_name() using @a context.
 *
 * @retval
 *  0  Success
 * @retval
 *  KV5M_CONTEXT          Bad magic number for @c _krb5_context structure
 * @return
 * Kerberos error codes
 */
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
    /* *
 * Copy a credential cache.
 *
 * @param [in]  context         Library context
 * @param [in]  incc            Credential cache to be copied
 * @param [out] outcc           Copy of credential cache to be filled in
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get a configuration value from a credential cache.
 *
 * @param [in]     context      Library context
 * @param [in]     id           Credential cache handle
 * @param [in]     principal    Configuration for this principal;
 *                              if NULL, global for the whole cache
 * @param [in]     key          Name of config variable
 * @param [out]    data         Data to be fetched
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
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
    /* *
 * Make a credential cache the primary cache for its collection.
 *
 * @param [in] context          Library context
 * @param [in] cache            Credential cache handle
 *
 * If the type of @a cache supports it, set @a cache to be the primary
 * credential cache for the collection it belongs to.
 *
 * @retval
 * 0  Success, or the type of @a cache doesn't support switching
 * @return
 * Kerberos error codes
 */
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
    /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
    /* *
 * Free a krb5_authenticator structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Authenticator structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the data stored in array of addresses.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of addresses to be freed
 *
 * This function frees the contents of @a val and the array itself.
 *
 * @note The last entry in the array must be a NULL pointer.
 */
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
    /* *
 * Free a ticket.
 *
 * @param [in] context          Library context
 * @param [in] val              Ticket to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free an error allocated by krb5_read_error() or krb5_sendauth().
 *
 * @param [in] context          Library context
 * @param [in] val              Error data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to be freed.
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the contents of a krb5_creds structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Credential structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
    /* *
 * Free a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the contents of a krb5_checksum structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Checksum structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
    /* *
 * Free a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Keyblock to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
    /* *
 * Free a krb5_ap_rep_enc_part structure.
 *
 * @param [in] context          Library context
 * @param [in] val              AP-REP enc part to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
    /* Free a krb5_octet_data structure (should be unused). */
    /* *
 * Free the contents of a krb5_data structure and zero the data field.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to free contents of
 *
 * This function frees the contents of @a val, but not the structure itself.
 */
    /* *
 * Free a string representation of a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Name string to be freed
 */
    /* *
 * Free a string allocated by a krb5 function.
 *
 * @param [in] context          Library context
 * @param [in] val              String to be freed
 *
 * @version New in 1.10
 */
    /* *
 * Free an array of encryption types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of enctypes to be freed
 *
 * @version New in 1.12
 */
    /* *
 * Free an array of checksum types.
 *
 * @param [in] context          Library context
 * @param [in] val              Array of checksum types to be freed
 */
    /* From krb5/os, but needed by the outside world */
/* *
 * Retrieve the system time of day, in sec and ms, since the epoch.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         System timeofday, seconds portion
 * @param [out] microseconds    System timeofday, microseconds portion
 *
 * This function retrieves the system time of day with the context
 * specific time offset adjustment.
 *
 * @sa krb5_crypto_us_timeofday()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
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
    /* *
 * Check if a timestamp is within the allowed clock skew of the current time.
 *
 * @param [in]     context      Library context
 * @param [in]     date         Timestamp to check
 *
 * This function checks if @a date is close enough to the current time
 * according to the configured allowable clock skew.
 *
 * @version New in 1.10
 *
 * @retval 0 Success
 * @retval KRB5KRB_AP_ERR_SKEW @a date is not within allowable clock skew
 */
    /* *
 * Return all interface addresses for this host.
 *
 * @param [in]  context         Library context
 * @param [out] addr            Array of krb5_address pointers, ending with
 *                              NULL
 *
 * Use krb5_free_addresses() to free @a addr when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the default realm.
 *
 * @param [in]  context         Library context
 * @param [out] lrealm          Default realm name
 *
 * Retrieves the default realm to be used if no user-specified realm is
 * available.
 *
 * Use krb5_free_default_realm() to free @a lrealm when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Override the default realm for the specified context.
 *
 * @param [in]     context      Library context
 * @param [in]     lrealm       Realm name for the default realm
 *
 * If @a lrealm is NULL, clear the default realm setting.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Free a default realm string returned by krb5_get_default_realm().
 *
 * @param [in] context          Library context
 * @param [in] lrealm           Realm to be freed
 */
    /* *
 * Canonicalize a hostname, possibly using name service.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Input hostname
 * @param [out] canonhost_out   Canonicalized hostname
 *
 * This function canonicalizes orig_hostname, possibly using name service
 * lookups if configuration permits.  Use krb5_free_string() to free @a
 * canonhost_out when it is no longer needed.
 *
 * @version New in 1.15
 */
    /* *
 * Generate a full principal name from a service name.
 *
 * @param [in]  context         Library context
 * @param [in]  hostname        Host name, or NULL to use local host
 * @param [in]  sname           Service name, or NULL to use @c "host"
 * @param [in]  type            Principal type
 * @param [out] ret_princ       Generated principal
 *
 * This function converts a @a hostname and @a sname into @a krb5_principal
 * structure @a ret_princ.  The returned principal will be of the form @a
 * sname\/hostname\@REALM where REALM is determined by krb5_get_host_realm().
 * In some cases this may be the referral (empty) realm.
 *
 * The @a type can be one of the following:
 *
 * @li #KRB5_NT_SRV_HST canonicalizes the host name before looking up the
 * realm and generating the principal.
 *
 * @li #KRB5_NT_UNKNOWN accepts the hostname as given, and does not
 * canonicalize it.
 *
 * Use krb5_free_principal to free @a ret_princ when it is no longer needed.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
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
    /* *
 * Change a password for an existing Kerberos account.
 *
 * @param [in]  context             Library context
 * @param [in]  creds               Credentials for kadmin/changepw service
 * @param [in]  newpw               New password
 * @param [out] result_code         Numeric error code from server
 * @param [out] result_code_string  String equivalent to @a result_code
 * @param [out] result_string       Change password response from the KDC
 *
 * Change the password for the existing principal identified by @a creds.
 *
 * The possible values of the output @a result_code are:
 *
 * @li #KRB5_KPASSWD_SUCCESS   (0) - success
 * @li #KRB5_KPASSWD_MALFORMED (1) - Malformed request error
 * @li #KRB5_KPASSWD_HARDERROR (2) - Server error
 * @li #KRB5_KPASSWD_AUTHERROR (3) - Authentication error
 * @li #KRB5_KPASSWD_SOFTERROR (4) - Password change rejected
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for a principal using specified credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  creds                Credentials for kadmin/changepw service
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the credentials @a creds to set the password @a newpw for
 * the principal @a change_password_for.  It implements the set password
 * operation of RFC 3244, for interoperability with Microsoft Windows
 * implementations.
 *
 * @note If @a change_password_for is NULL, the change is performed on the
 * current principal. If @a change_password_for is non-null, the change is
 * performed on the principal name passed in @a change_password_for.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @sa krb5_set_password_using_ccache()
 *
 * @retval
 * 0  Success and result_code is set to #KRB5_KPASSWD_SUCCESS.
 * @return
 * Kerberos error codes.
 */
    /* *
 * Set a password for a principal using cached credentials.
 *
 * @param [in]  context              Library context
 * @param [in]  ccache               Credential cache
 * @param [in]  newpw                New password
 * @param [in]  change_password_for  Change the password for this principal
 * @param [out] result_code          Numeric error code from server
 * @param [out] result_code_string   String equivalent to @a result_code
 * @param [out] result_string        Data returned from the remote system
 *
 * This function uses the cached credentials from @a ccache to set the password
 * @a newpw for the principal @a change_password_for.  It implements RFC 3244
 * set password operation (interoperable with MS Windows implementations) using
 * the credential cache.
 *
 * The error code and strings are returned in @a result_code,
 * @a result_code_string and @a result_string.
 *
 * @note If @a change_password_for is set to NULL, the change is performed on
 * the default principal in @a ccache. If @a change_password_for is non null,
 * the change is performed on the specified principal.
 *
 * @sa krb5_set_password()
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Get a result message for changing or setting a password.
 *
 * @param [in]  context            Library context
 * @param [in]  server_string      Data returned from the remote system
 * @param [out] message_out        A message displayable to the user
 *
 * This function processes the @a server_string returned in the @a
 * result_string parameter of krb5_change_password(), krb5_set_password(), and
 * related functions, and returns a displayable string.  If @a server_string
 * contains Active Directory structured policy information, it will be
 * converted into human-readable text.
 *
 * Use krb5_free_string() to free @a message_out when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 *
 * @version New in 1.11
 */
    /* *
 * Retrieve configuration profile from the context.
 *
 * @param [in]  context         Library context
 * @param [out] profile         Pointer to data read from a configuration file
 *
 * This function creates a new @a profile object that reflects profile
 * in the supplied @a context.
 *
 * The @a profile object may be freed with profile_release() function.
 * See profile.h and profile API for more details.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_get_init_creds_password().*/
    /* * @deprecated Replaced by krb5_get_init_creds(). */
    /* * @deprecated Replaced by krb5_get_init_creds_keytab(). */
    /* KRB5_DEPRECATED */
    /* *
 * Parse and decrypt a @c KRB_AP_REQ message.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     inbuf          AP-REQ message to be parsed
 * @param [in]     server         Matching principal for server, or NULL to
 *                                allow any principal in keytab
 * @param [in]     keytab         Key table, or NULL to use the default
 * @param [out]    ap_req_options If non-null, the AP-REQ flags on output
 * @param [out]    ticket         If non-null, ticket from the AP-REQ message
 *
 * This function parses, decrypts and verifies a AP-REQ message from @a inbuf
 * and stores the authenticator in @a auth_context.
 *
 * If a keyblock was specified in @a auth_context using
 * krb5_auth_con_setuseruserkey(), that key is used to decrypt the ticket in
 * AP-REQ message and @a keytab is ignored.  In this case, @a server should be
 * specified as a complete principal name to allow for proper transited-path
 * checking and replay cache selection.
 *
 * Otherwise, the decryption key is obtained from @a keytab, or from the
 * default keytab if it is NULL.  In this case, @a server may be a complete
 * principal name, a matching principal (see krb5_sname_match()), or NULL to
 * match any principal name.  The keys tried against the encrypted part of the
 * ticket are determined as follows:
 *
 * - If @a server is a complete principal name, then its entry in @a keytab is
 *   tried.
 * - Otherwise, if @a keytab is iterable, then all entries in @a keytab which
 *   match @a server are tried.
 * - Otherwise, the server principal in the ticket must match @a server, and
 *   its entry in @a keytab is tried.
 *
 * The client specified in the decrypted authenticator must match the client
 * specified in the decrypted ticket.
 *
 * If the @a remote_addr field of @a auth_context is set, the request must come
 * from that address.
 *
 * If a replay cache handle is provided in the @a auth_context, the
 * authenticator and ticket are verified against it.  If no conflict is found,
 * the new authenticator is then stored in the replay cache of @a auth_context.
 *
 * Various other checks are performed on the decoded data, including
 * cross-realm policy, clockskew, and ticket validation times.
 *
 * On success the authenticator, subkey, and remote sequence number of the
 * request are stored in @a auth_context. If the #AP_OPTS_MUTUAL_REQUIRED
 * bit is set, the local sequence number is XORed with the remote sequence
 * number in the request.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a service key from a key table.
 *
 * @param [in]  context     Library context
 * @param [in]  keyprocarg  Name of a key table (NULL to use default name)
 * @param [in]  principal   Service principal
 * @param [in]  vno         Key version number (0 for highest available)
 * @param [in]  enctype     Encryption type (0 for any type)
 * @param [out] key         Service key from key table
 *
 * Open and search the specified key table for the entry identified by @a
 * principal, @a enctype, and @a vno. If no key is found, return an error code.
 *
 * The default key table is used, unless @a keyprocarg is non-null.
 * @a keyprocarg designates a specific key table.
 *
 * Use krb5_free_keyblock() to free @a key when it is no longer needed.
 *
 * @retval
 * 0 Success
 * @return Kerberos error code if not found or @a keyprocarg is invalid.
 */
    /* *
 * Format a @c KRB-SAFE message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data in the message
 * @param [out] der_out         Formatted @c KRB-SAFE buffer
 * @param [out] rdata_out       Replay data. Specify NULL if not needed
 *
 * This function creates an integrity protected @c KRB-SAFE message
 * using data supplied by the application.
 *
 * Fields in @a auth_context specify the checksum type, the keyblock that
 * can be used to seed the checksum, full addresses (host and port) for
 * the sender and receiver, and @ref KRB5_AUTH_CONTEXT flags.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-SAFE message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-SAFE message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-SAFE message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-SAFE message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Format a @c KRB-PRIV message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  userdata        User data for @c KRB-PRIV message
 * @param [out] der_out         Formatted @c KRB-PRIV message
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * This function is similar to krb5_mk_safe(), but the message is encrypted and
 * integrity-protected, not just integrity-protected.
 *
 * The local address in @a auth_context must be set, and is used to form the
 * sender address used in the KRB-PRIV message.  The remote address is
 * optional; if specified, it will be used to form the receiver address used in
 * the message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, a
 * timestamp is included in the KRB-PRIV message, and an entry for the message
 * is entered in an in-memory replay cache to detect if the message is
 * reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not set, no
 * replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, a timestamp is included in the KRB-PRIV message and is stored
 * in @a rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-PRIV message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Client function for @c sendauth protocol.
 *
 * @param [in]     context        Library context
 * @param [in,out] auth_context   Pre-existing or newly created auth context
 * @param [in]     fd             File descriptor that describes network socket
 * @param [in]     appl_version   Application protocol version to be matched
 *                                with the receiver's application version
 * @param [in]     client         Client principal
 * @param [in]     server         Server principal
 * @param [in]     ap_req_options @ref AP_OPTS options
 * @param [in]     in_data        Data to be sent to the server
 * @param [in]     in_creds       Input credentials, or NULL to use @a ccache
 * @param [in]     ccache         Credential cache
 * @param [out]    error          If non-null, contains KRB_ERROR message
 *                                returned from server
 * @param [out]    rep_result     If non-null and @a ap_req_options is
 *                                #AP_OPTS_MUTUAL_REQUIRED, contains the result
 *                                of mutual authentication exchange
 * @param [out]    out_creds      If non-null, the retrieved credentials
 *
 * This function performs the client side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Credentials may be specified in three ways:
 *
 * @li If @a in_creds is NULL, credentials are obtained with
 * krb5_get_credentials() using the principals @a client and @a server.  @a
 * server must be non-null; @a client may NULL to use the default principal of
 * @a ccache.
 *
 * @li If @a in_creds is non-null, but does not contain a ticket, credentials
 * for the exchange are obtained with krb5_get_credentials() using @a in_creds.
 * In this case, the values of @a client and @a server are unused.
 *
 * @li If @a in_creds is a complete credentials structure, it used directly.
 * In this case, the values of @a client, @a server, and @a ccache are unused.
 *
 * If the server is using a different application protocol than that specified
 * in @a appl_version, an error will be returned.
 *
 * Use krb5_free_creds() to free @a out_creds, krb5_free_ap_rep_enc_part() to
 * free @a rep_result, and krb5_free_error() to free @a error when they are no
 * longer needed.
 *
 * @sa krb5_recvauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Server function for @a sendauth protocol.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     appl_version Application protocol version to be matched
 *                              against the client's application version
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Key table containing service keys
 * @param [out]    ticket       Ticket (NULL if not needed)
 *
 * This function performs the server side of a sendauth/recvauth exchange by
 * sending and receiving messages over @a fd.
 *
 * Use krb5_free_ticket() to free @a ticket when it is no longer needed.
 *
 * @sa krb5_sendauth()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Server function for @a sendauth protocol with version parameter.
 *
 * @param [in]     context      Library context
 * @param [in,out] auth_context Pre-existing or newly created auth context
 * @param [in]     fd           File descriptor
 * @param [in]     server       Server principal (NULL for any in @a keytab)
 * @param [in]     flags        Additional specifications
 * @param [in]     keytab       Decryption key
 * @param [out]    ticket       Ticket (NULL if not needed)
 * @param [out]    version      sendauth protocol version (NULL if not needed)
 *
 * This function is similar to krb5_recvauth() with the additional output
 * information place into @a version.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Format a @c KRB-CRED message for an array of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Null-terminated array of credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache information (NULL if not needed)
 *
 * This function takes an array of credentials @a creds and formats
 * a @c KRB-CRED message @a der_out to pass to krb5_rd_cred().
 *
 * The local and remote addresses in @a auth_context are optional; if either is
 * specified, they are used to form the sender and receiver addresses in the
 * KRB-CRED message.
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.
 *
 * If the #KRB5_AUTH_CONTEXT_DO_TIME flag is set in @a auth_context, an entry
 * for the message is entered in an in-memory replay cache to detect if the
 * message is reflected by an attacker.  If #KRB5_AUTH_CONTEXT_DO_TIME is not
 * set, no replay cache is used.  If #KRB5_AUTH_CONTEXT_RET_TIME is set in @a
 * auth_context, the timestamp used for the KRB-CRED message is stored in @a
 * rdata_out.
 *
 * If either #KRB5_AUTH_CONTEXT_DO_SEQUENCE or #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the @a auth_context local sequence number is included in the
 * KRB-CRED message and then incremented.  If #KRB5_AUTH_CONTEXT_RET_SEQUENCE
 * is set, the sequence number used is stored in @a rdata_out.
 *
 * Use krb5_free_data_contents() to free @a der_out when it is no longer
 * needed.
 *
 * The message will be encrypted using the send subkey of @a auth_context if it
 * is present, or the session key otherwise.  If neither key is present, the
 * credentials will not be encrypted, and the message should only be sent over
 * a secure channel.  No replay cache entry is used in this case.
 *
 * @retval
 *  0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *   KRB5_RC_REQUIRED Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
    /* *
 * Format a @c KRB-CRED message for a single set of credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creds           Pointer to credentials
 * @param [out] der_out         Encoded credentials
 * @param [out] rdata_out       Replay cache data (NULL if not needed)
 *
 * This is a convenience function that calls krb5_mk_ncred() with a single set
 * of credentials.
 *
 * @retval
 * 0 Success
 * @retval
 *  ENOMEM Insufficient memory
 * @retval
 *  KRB5_RC_REQUIRED   Message replay detection requires @a rcache parameter
 * @return
 * Kerberos error codes
 */
    /* *
 * Read and validate a @c KRB-CRED message.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [in]  creddata        @c KRB-CRED message
 * @param [out] creds_out       Null-terminated array of forwarded credentials
 * @param [out] rdata_out       Replay data (NULL if not needed)
 *
 * @note The @a rdata_out argument is required if the
 * #KRB5_AUTH_CONTEXT_RET_TIME or #KRB5_AUTH_CONTEXT_RET_SEQUENCE flag is set
 * in @a auth_context.`
 *
 * @a creddata will be decrypted using the receiving subkey if it is present in
 * @a auth_context, or the session key if the receiving subkey is not present
 * or fails to decrypt the message.
 *
 * Use krb5_free_tgt_creds() to free @a creds_out when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
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
    /* *
 * Get the checksum callback from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] func            Checksum callback
 * @param [out] data            Callback argument
 *
 * @retval 0 (always)
 */
    /* *
 * Set the local and remote addresses in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_addr       Local address
 * @param [in] remote_addr      Remote address
 *
 * This function releases the storage assigned to the contents of the local and
 * remote addresses of @a auth_context and then sets them to @a local_addr and
 * @a remote_addr respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve address fields from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] local_addr      Local address (NULL if not needed)
 * @param [out] remote_addr     Remote address (NULL if not needed)
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set local and remote port fields in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] local_port       Local port
 * @param [in] remote_port      Remote port
 *
 * This function releases the storage assigned to the contents of the local and
 * remote ports of @a auth_context and then sets them to @a local_port and @a
 * remote_port respectively.
 *
 * @sa krb5_auth_con_genaddrs()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the session key in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] keyblock         User key
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the session key from an auth context as a keyblock.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] keyblock        Session key
 *
 * This function creates a keyblock containing the session key from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the session key from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] key             Session key
 *
 * This function sets @a key to the session key from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 (always)
 */
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
    /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] keyblock        Receiving subkey
 *
 * This function creates a keyblock containing the receiving subkey from @a
 * auth_context.  Use krb5_free_keyblock() to free @a keyblock when it is no
 * longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the receiving subkey from an auth context as a keyblock.
 *
 * @param [in]  ctx             Library context
 * @param [in]  ac              Authentication context
 * @param [out] key             Receiving subkey
 *
 * This function sets @a key to the receiving subkey from @a auth_context.  Use
 * krb5_k_free_key() to release @a key when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the send subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Send subkey
 *
 * This function sets the send subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
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
    /* *
 * Set the receiving subkey in an auth context with a keyblock.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] keyblock         Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to a copy of @a keyblock.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the receiving subkey in an auth context.
 *
 * @param [in] ctx              Library context
 * @param [in] ac               Authentication context
 * @param [in] key              Receiving subkey
 *
 * This function sets the receiving subkey in @a ac to @a key, incrementing its
 * reference count.
 *
 * @version New in 1.9
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* * @deprecated Replaced by krb5_auth_con_getsendsubkey(). */
    /* * @deprecated Replaced by krb5_auth_con_getrecvsubkey(). */
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
    /* *
 * Retrieve the remote sequence number from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] seqnumber       Remote sequence number
 *
 * Retrieve the remote sequence number from @a auth_context and return it in @a
 * seqnumber.  The #KRB5_AUTH_CONTEXT_DO_SEQUENCE flag must be set in @a
 * auth_context for this function to be useful.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Cause an auth context to use cipher state.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 *
 * Prepare @a auth_context to use cipher state when krb5_mk_priv() or
 * krb5_rd_priv() encrypt or decrypt data.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set the replay cache in an auth context.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] rcache           Replay cache haddle
 *
 * This function sets the replay cache in @a auth_context to @a rcache.  @a
 * rcache will be closed when @a auth_context is freed, so the caller should
 * relinguish that responsibility.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve the replay cache from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] rcache          Replay cache handle
 *
 * This function fetches the replay cache from @a auth_context.  The caller
 * should not close @a rcache.
 *
 * @retval 0 (always)
 */
    /* *
 * Retrieve the authenticator from an auth context.
 *
 * @param [in]  context         Library context
 * @param [in]  auth_context    Authentication context
 * @param [out] authenticator   Authenticator
 *
 * Use krb5_free_authenticator() to free @a authenticator when it is no longer
 * needed.
 *
 * @retval 0 Success. Otherwise - Kerberos error codes
 */
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
    /*
 * end "func-proto.h"
 */
    /*
 * begin stuff from libos.h
 */
    /* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
    /* *
 * Convert a principal name to a local name.
 *
 * @param [in]  context         Library context
 * @param [in]  aname           Principal name
 * @param [in]  lnsize_in       Space available in @a lname
 * @param [out] lname           Local name buffer to be filled in
 *
 * If @a aname does not correspond to any local account, KRB5_LNAME_NOTRANS is
 * returned.  If @a lnsize_in is too small for the local name,
 * KRB5_CONFIG_NOTENUFSPACE is returned.
 *
 * Local names, rather than principal names, can be used by programs that
 * translate to an environment-specific name (for example, a user account
 * name).
 *
 * @retval
 * 0  Success
 * @retval
 *  System errors
 * @return
 * Kerberos error codes
 */
    /* *
 * Get the Kerberos realm names for a host.
 *
 * @param [in]  context         Library context
 * @param [in]  host            Host name (or NULL)
 * @param [out] realmsp         Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names.
 * If there are no known realms for the host, a list containing the referral
 * (empty) realm is returned.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 *
 * @retval
 *  0   Success
 * @retval
 *  ENOMEM  Insufficient memory
 * @return
 * Kerberos error codes
 */
    /* *
 *
 * @param [in] context           Library context
 * @param [in] hdata             Host name (or NULL)
 * @param [out] realmsp          Null-terminated list of realm names
 *
 * Fill in @a realmsp with a pointer to a null-terminated list of realm names
 * obtained through heuristics or insecure resolution methods which have lower
 * priority than KDC referrals.
 *
 * If @a host is NULL, the local host's realms are determined.
 *
 * Use krb5_free_host_realm() to release @a realmsp when it is no longer
 * needed.
 */
    /* *
 * Free the memory allocated by krb5_get_host_realm().
 *
 * @param [in] context          Library context
 * @param [in] realmlist        List of realm names to be released
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Determine if a principal is authorized to log in as a local user.
 *
 * @param [in] context          Library context
 * @param [in] principal        Principal name
 * @param [in] luser            Local username
 *
 * Determine whether @a principal is authorized to log in as a local user @a
 * luser.
 *
 * @retval
 * TRUE Principal is authorized to log in as user; FALSE otherwise.
 */
    /* *
 * Generate auth context addresses from a connected socket.
 *
 * @param [in] context          Library context
 * @param [in] auth_context     Authentication context
 * @param [in] infd             Connected socket descriptor
 * @param [in] flags            Flags
 *
 * This function sets the local and/or remote addresses in @a auth_context
 * based on the local and remote endpoints of the socket @a infd.  The
 * following flags determine the operations performed:
 *
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_ADDR   Generate local address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_ADDR  Generate remote address.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_LOCAL_FULL_ADDR  Generate local address and port.
 * @li #KRB5_AUTH_CONTEXT_GENERATE_REMOTE_FULL_ADDR Generate remote address and port.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set time offset field in a krb5_context structure.
 *
 * @param [in] context          Library context
 * @param [in] seconds          Real time, seconds portion
 * @param [in] microseconds     Real time, microseconds portion
 *
 * This function sets the time offset in @a context to the difference between
 * the system time and the real time as determined by @a seconds and @a
 * microseconds.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return the time offsets from the os context.
 *
 * @param [in]  context         Library context
 * @param [out] seconds         Time offset, seconds portion
 * @param [out] microseconds    Time offset, microseconds portion
 *
 * This function returns the time offsets in @a context.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* str_conv.c */
/* *
 * Convert a string to an encryption type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] enctypep        Encryption type
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a salt type.
 *
 * @param [in]  string          String to convert to an encryption type
 * @param [out] salttypep       Salt type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a checksum type.
 *
 * @param [in]  string          String to be converted
 * @param [out] cksumtypep      Checksum type to be filled in
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a timestamp.
 *
 * @param [in]  string          String to be converted
 * @param [out] timestampp      Pointer to timestamp
 *
 * @retval 0  Success; otherwise - EINVAL
 */
    /* *
 * Convert a string to a delta time value.
 *
 * @param [in]  string          String to be converted
 * @param [out] deltatp         Delta time to be filled in
 *
 * @retval 0  Success; otherwise - KRB5_DELTAT_BADFORMAT
 */
    /* *
 * Convert an encryption type to a string.
 *
 * @param [in]  enctype         Encryption type
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert an encryption type to a name or alias.
 *
 * @param [in]  enctype         Encryption type
 * @param [in]  shortest        Flag
 * @param [out] buffer          Buffer to hold encryption type string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * If @a shortest is FALSE, this function returns the enctype's canonical name
 * (like "aes128-cts-hmac-sha1-96").  If @a shortest is TRUE, it return the
 * enctype's shortest alias (like "aes128-cts").
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a salt type to a string.
 *
 * @param [in]  salttype        Salttype to convert
 * @param [out] buffer          Buffer to receive the converted string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a checksum type to a string.
 *
 * @param [in]  cksumtype       Checksum type
 * @param [out] buffer          Buffer to hold converted checksum type
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a timestamp to a string.
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold converted timestamp
 * @param [in]  buflen          Storage available in @a buffer
 *
 * The string is returned in the locale's appropriate date and time
 * representation.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a timestamp to a string, with optional output padding
 *
 * @param [in]  timestamp       Timestamp to convert
 * @param [out] buffer          Buffer to hold the converted timestamp
 * @param [in]  buflen          Length of buffer
 * @param [in]  pad             Optional value to pad @a buffer if converted
 *                              timestamp does not fill it
 *
 * If @a pad is not NULL, @a buffer is padded out to @a buflen - 1 characters
 * with the value of *@a pad.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Convert a relative time value to a string.
 *
 * @param [in]  deltat          Relative time value to convert
 * @param [out] buffer          Buffer to hold time string
 * @param [in]  buflen          Storage available in @a buffer
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* The name of the Kerberos ticket granting service... and its size */
    /* flags for recvauth */
    /* initial ticket api functions */
    /* * Text for prompt used in prompter callback function.  */
    /* *< The prompt to show to the user */
    /* *< Boolean; informative prompt or hidden (e.g. PIN) */
    /* *< Must be allocated before call to  prompt routine */
    /* * Pointer to a prompter callback function. */
    /* *
 * Prompt user for password.
 *
 * @param [in] context          Library context
 * @param      data             Unused (callback argument)
 * @param [in] name             Name to output during prompt
 * @param [in] banner           Banner to output during prompt
 * @param [in] num_prompts      Number of prompts in @a prompts
 * @param [in] prompts          Array of prompts and replies
 *
 * This function is intended to be used as a prompter callback for
 * krb5_get_init_creds_password() or krb5_init_creds_init().
 *
 * Writes @a name and @a banner to stdout, each followed by a newline, then
 * writes each prompt field in the @a prompts array, followed by ": ", and sets
 * the reply field of the entry to a line of input read from stdin.  If the
 * hidden flag is set for a prompt, then terminal echoing is turned off when
 * input is read.
 *
 * @retval
 *  0   Success
 * @return
 * Kerberos error codes
 *
 */
    /* *
 * Long-term password responder question
 *
 * This question is asked when the long-term password is needed. It has no
 * challenge and the response is simply the password string.
 *
 * @version New in 1.11
 */
    /* *
 * OTP responder question
 *
 * The OTP responder question is asked when the KDC indicates that an OTP
 * value is required in order to complete the authentication.  The JSON format
 * of the challenge is:
 *
 *  @n {
 *  @n   "service": <string (optional)>,
 *  @n   "tokenInfo": [
 *  @n      {
 *  @n        "flags":     <number>,
 *  @n        "vendor":    <string (optional)>,
 *  @n        "challenge": <string (optional)>,
 *  @n        "length":    <number (optional)>,
 *  @n        "format":    <number (optional)>,
 *  @n        "tokenID":   <string (optional)>,
 *  @n        "algID":     <string (optional)>,
 *  @n      },
 *  @n      ...
 *  @n    ]
 *  @n  }
 *
 * The answer to the question MUST be JSON formatted:
 *
 * @n  {
 * @n    "tokeninfo": <number>,
 * @n    "value":     <string (optional)>,
 * @n    "pin":       <string (optional)>,
 * @n  }
 *
 * For more detail, please see RFC 6560.
 *
 * @version New in 1.11
 */
    /* *
 * These format constants identify the format of the token value.
 */
    /* *
 * This flag indicates that the token value MUST be collected.
 */
    /* *
 * This flag indicates that the PIN value MUST be collected.
 */
    /* *
 * This flag indicates that the token is now in re-synchronization mode with
 * the server.  The user is expected to reply with the next code displayed on
 * the token.
 */
    /* *
 * This flag indicates that the PIN MUST be returned as a separate item. This
 * flag only takes effect if KRB5_RESPONDER_OTP_FLAGS_COLLECT_PIN is set. If
 * this flag is not set, the responder may either concatenate PIN + token value
 * and store it as "value" in the answer or it may return them separately. If
 * they are returned separately, they will be concatenated internally.
 */
    /* *
 * PKINIT responder question
 *
 * The PKINIT responder question is asked when the client needs a password
 * that's being used to protect key information, and is formatted as a JSON
 * object.  A specific identity's flags value, if not zero, is the bitwise-OR
 * of one or more of the KRB5_RESPONDER_PKINIT_FLAGS_TOKEN_* flags defined
 * below, and possibly other flags to be added later.  Any resemblance to
 * similarly-named CKF_* values in the PKCS#11 API should not be depended on.
 *
 *  @n {
 *  @n     identity <string> : flags <number>,
 *  @n     ...
 *  @n }
 *
 * The answer to the question MUST be JSON formatted:
 *
 *  @n {
 *  @n     identity <string> : password <string>,
 *  @n     ...
 *  @n }
 *
 * @version New in 1.12
 */
    /* *
 * This flag indicates that an incorrect PIN was supplied at least once since
 * the last time the correct PIN was supplied.
 */
    /* *
 * This flag indicates that supplying an incorrect PIN will cause the token to
 * lock itself.
 */
    /* *
 * This flag indicates that the user PIN is locked, and you can't log in to the
 * token with it.
 */
    /* *
 * A container for a set of preauthentication questions and answers
 *
 * A responder context is supplied by the krb5 authentication system to a @ref
 * krb5_responder_fn callback.  It contains a list of questions and can receive
 * answers.  Questions contained in a responder context can be listed using
 * krb5_responder_list_questions(), retrieved using
 * krb5_responder_get_challenge(), or answered using
 * krb5_responder_set_answer().  The form of a question's challenge and
 * answer depend on the question name.
 *
 * @version New in 1.11
 */
    /* *
 * List the question names contained in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 *
 * Return a pointer to a null-terminated list of question names which are
 * present in @a rctx.  The pointer is an alias, valid only as long as the
 * lifetime of @a rctx, and should not be modified or freed by the caller.  A
 * question's challenge can be retrieved using krb5_responder_get_challenge()
 * and answered using krb5_responder_set_answer().
 *
 * @version New in 1.11
 */
    /* *
 * Retrieve the challenge data for a given question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 *
 * Return a pointer to a C string containing the challenge for @a question
 * within @a rctx, or NULL if the question is not present in @a rctx.  The
 * structure of the question depends on the question name, but will always be
 * printable UTF-8 text.  The returned pointer is an alias, valid only as long
 * as the lifetime of @a rctx, and should not be modified or freed by the
 * caller.
 *
 * @version New in 1.11
 */
    /* *
 * Answer a named question in the responder context.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] question         Question name
 * @param [in] answer           The string to set (MUST be printable UTF-8)
 *
 * This function supplies an answer to @a question within @a rctx.  The
 * appropriate form of the answer depends on the question name.
 *
 * @retval EINVAL @a question is not present within @a rctx
 *
 * @version New in 1.11
 */
    /* *
 * Responder function for an initial credential exchange.
 *
 * @param [in] ctx              Library context
 * @param [in] data             Callback data
 * @param [in] rctx             Responder context
 *
 * A responder function is like a prompter function, but is used for handling
 * questions and answers as potentially complex data types.  Client
 * preauthentication modules will insert a set of named "questions" into
 * the responder context.  Each question may optionally contain a challenge.
 * This challenge is printable UTF-8, but may be an encoded value.  The
 * precise encoding and contents of the challenge are specific to the question
 * asked.  When the responder is called, it should answer all the questions it
 * understands.  Like the challenge, the answer MUST be printable UTF-8, but
 * may contain structured/encoded data formatted to the expected answer format
 * of the question.
 *
 * If a required question is unanswered, the prompter may be called.
 */
    /* -1 when not specified. */
    /* -1 when not specified. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_OTP to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_OTP
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to interact with OTP tokens without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_otp_challenge_free() to
 * be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl             Challenge structure
 *
 * @version New in 1.11
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_OTP question.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] ti               The index of the tokeninfo selected
 * @param [in] value            The value to set, or NULL for none
 * @param [in] pin              The pin to set, or NULL for none
 *
 * @version New in 1.11
 */
    /* *
 * Free the value returned by krb5_responder_otp_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.11
 */
    /* 0 when not specified or not applicable. */
    /* *
 * Decode the KRB5_RESPONDER_QUESTION_PKINIT to a C struct.
 *
 * A convenience function which parses the KRB5_RESPONDER_QUESTION_PKINIT
 * question challenge data, making it available in native C.  The main feature
 * of this function is the ability to read the challenge without parsing
 * the JSON.
 *
 * The returned value must be passed to krb5_responder_pkinit_challenge_free()
 * to be freed.
 *
 * @param [in]  ctx             Library context
 * @param [in]  rctx            Responder context
 * @param [out] chl_out         Challenge structure
 *
 * @version New in 1.12
 */
    /* *
 * Answer the KRB5_RESPONDER_QUESTION_PKINIT question for one identity.
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] identity         The identity for which a PIN is being supplied
 * @param [in] pin              The provided PIN, or NULL for none
 *
 * @version New in 1.12
 */
    /* *
 * Free the value returned by krb5_responder_pkinit_get_challenge().
 *
 * @param [in] ctx              Library context
 * @param [in] rctx             Responder context
 * @param [in] chl              The challenge to free
 *
 * @version New in 1.12
 */
    /* * Store options for @c _krb5_get_init_creds */
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
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    use super::stddef_h::size_t;
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
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[no_mangle]
        #[c2rust::src_loc = "1049:1"]
        pub fn krb5_c_valid_enctype(ktype: krb5_enctype) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context_0: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2479:1"]
        pub fn krb5_cc_get_principal(context_0: krb5_context,
                                     cache: krb5_ccache,
                                     principal: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context_0: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context_0: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context_0: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context_0: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context_0: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context_0: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context_0: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context_0: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4961:1"]
        pub fn krb5_sname_to_principal(context_0: krb5_context,
                                       hostname: *const libc::c_char,
                                       sname: *const libc::c_char,
                                       type_0: krb5_int32,
                                       ret_princ: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6096:1"]
        pub fn krb5_read_password(context_0: krb5_context,
                                  prompt: *const libc::c_char,
                                  prompt2: *const libc::c_char,
                                  return_pwd: *mut libc::c_char,
                                  size_return: *mut libc::c_uint)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6310:1"]
        pub fn krb5_string_to_deltat(string: *mut libc::c_char,
                                     deltatp: *mut krb5_deltat)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6354:1"]
        pub fn krb5_salttype_to_string(salttype: krb5_int32,
                                       buffer: *mut libc::c_char,
                                       buflen: size_t) -> krb5_error_code;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:34"]
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
    #[inline]
    #[c2rust::src_loc = "2338:1"]
    pub unsafe extern "C" fn ts2tt(mut timestamp: krb5_timestamp) -> time_t {
        return timestamp as uint32_t as time_t;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_timestamp};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
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
        #[c2rust::src_loc = "2096:1"]
        pub fn krb5int_c_deprecated_enctype(_: krb5_enctype) -> krb5_boolean;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:34"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:34"]
pub mod profile_h {
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:34"]
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
    #[c2rust::src_loc = "27:1"]
    pub type et_old_error_hook_func
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: errcode_t,
                                    _: *const libc::c_char,
                                    _: *mut __va_list_tag) -> ()>;
    use super::internal::__va_list_tag;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
        /*@modifies internalState@*/
        /*
 * The display routine should be application specific.  A global hook,
 * may cause inappropriate display procedures to be called between
 * applications under non-Unix environments.
 */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn set_com_err_hook(_: et_old_error_hook_func)
         -> et_old_error_hook_func;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:35"]
pub mod kdb_h {
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "156:16"]
    pub struct krb5_string_attr_st {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
    }
    #[c2rust::src_loc = "156:1"]
    pub type krb5_string_attr = krb5_string_attr_st;
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_enctype,
                        krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:35"]
pub mod admin_h {
    #[c2rust::src_loc = "70:1"]
    pub type kadm5_policy_t = *mut libc::c_char;
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    /* kadm5_config_params */
    /*#define KADM5_CONFIG_ADMIN_KEYTAB       0x00000080*/
    /*
 * permission bits
 */
    /*
 * API versioning constants
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_rec = _kadm5_principal_ent_t;
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_t = *mut _kadm5_principal_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_rec = _kadm5_policy_ent_t;
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_t = *mut _kadm5_policy_ent_t;
    /*
 * Data structure returned by kadm5_get_config_params()
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_enctype,
                        krb5_int32, krb5_context, krb5_ui_4, _krb5_ccache,
                        krb5_ccache, krb5_principal_data, krb5_boolean,
                        krb5_keyblock, krb5_error_code};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_key_salt_tuple,
                       krb5_string_attr};
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "319:1"]
        pub fn kadm5_init_anonymous(context_0: krb5_context,
                                    client_name: *mut libc::c_char,
                                    service_name: *mut libc::c_char,
                                    params: *mut kadm5_config_params,
                                    struct_version: krb5_ui_4,
                                    api_version: krb5_ui_4,
                                    db_args: *mut *mut libc::c_char,
                                    server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "326:1"]
        pub fn kadm5_init_with_password(context_0: krb5_context,
                                        client_name: *mut libc::c_char,
                                        pass: *mut libc::c_char,
                                        service_name: *mut libc::c_char,
                                        params: *mut kadm5_config_params,
                                        struct_version: krb5_ui_4,
                                        api_version: krb5_ui_4,
                                        db_args: *mut *mut libc::c_char,
                                        server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn kadm5_init_with_skey(context_0: krb5_context,
                                    client_name: *mut libc::c_char,
                                    keytab: *mut libc::c_char,
                                    service_name: *mut libc::c_char,
                                    params: *mut kadm5_config_params,
                                    struct_version: krb5_ui_4,
                                    api_version: krb5_ui_4,
                                    db_args: *mut *mut libc::c_char,
                                    server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "344:1"]
        pub fn kadm5_init_with_creds(context_0: krb5_context,
                                     client_name: *mut libc::c_char,
                                     cc: krb5_ccache,
                                     service_name: *mut libc::c_char,
                                     params: *mut kadm5_config_params,
                                     struct_version: krb5_ui_4,
                                     api_version: krb5_ui_4,
                                     db_args: *mut *mut libc::c_char,
                                     server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn kadm5_lock(server_handle: *mut libc::c_void) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "354:1"]
        pub fn kadm5_unlock(server_handle: *mut libc::c_void) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "356:1"]
        pub fn kadm5_destroy(server_handle: *mut libc::c_void) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "357:1"]
        pub fn kadm5_create_principal(server_handle: *mut libc::c_void,
                                      ent: kadm5_principal_ent_t,
                                      mask: libc::c_long,
                                      pass: *mut libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn kadm5_create_principal_3(server_handle: *mut libc::c_void,
                                        ent: kadm5_principal_ent_t,
                                        mask: libc::c_long,
                                        n_ks_tuple: libc::c_int,
                                        ks_tuple: *mut krb5_key_salt_tuple,
                                        pass: *mut libc::c_char)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn kadm5_delete_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "368:1"]
        pub fn kadm5_modify_principal(server_handle: *mut libc::c_void,
                                      ent: kadm5_principal_ent_t,
                                      mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn kadm5_rename_principal(server_handle: *mut libc::c_void,
                                      _: krb5_principal, _: krb5_principal)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "373:1"]
        pub fn kadm5_get_principal(server_handle: *mut libc::c_void,
                                   principal: krb5_principal,
                                   ent: kadm5_principal_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn kadm5_chpass_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal,
                                      pass: *mut libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "380:1"]
        pub fn kadm5_chpass_principal_3(server_handle: *mut libc::c_void,
                                        principal: krb5_principal,
                                        keepold: krb5_boolean,
                                        n_ks_tuple: libc::c_int,
                                        ks_tuple: *mut krb5_key_salt_tuple,
                                        pass: *mut libc::c_char)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "386:1"]
        pub fn kadm5_randkey_principal(server_handle: *mut libc::c_void,
                                       principal: krb5_principal,
                                       keyblocks: *mut *mut krb5_keyblock,
                                       n_keys: *mut libc::c_int)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "390:1"]
        pub fn kadm5_randkey_principal_3(server_handle: *mut libc::c_void,
                                         principal: krb5_principal,
                                         keepold: krb5_boolean,
                                         n_ks_tuple: libc::c_int,
                                         ks_tuple: *mut krb5_key_salt_tuple,
                                         keyblocks: *mut *mut krb5_keyblock,
                                         n_keys: *mut libc::c_int)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "423:1"]
        pub fn kadm5_create_policy(server_handle: *mut libc::c_void,
                                   ent: kadm5_policy_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "426:1"]
        pub fn kadm5_delete_policy(server_handle: *mut libc::c_void,
                                   policy: kadm5_policy_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "428:1"]
        pub fn kadm5_modify_policy(server_handle: *mut libc::c_void,
                                   ent: kadm5_policy_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "431:1"]
        pub fn kadm5_get_policy(server_handle: *mut libc::c_void,
                                policy: kadm5_policy_t,
                                ent: kadm5_policy_ent_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "434:1"]
        pub fn kadm5_get_privs(server_handle: *mut libc::c_void,
                               privs: *mut libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn kadm5_free_principal_ent(server_handle: *mut libc::c_void,
                                        ent: kadm5_principal_ent_t)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "447:1"]
        pub fn kadm5_free_policy_ent(server_handle: *mut libc::c_void,
                                     ent: kadm5_policy_ent_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "450:1"]
        pub fn kadm5_get_principals(server_handle: *mut libc::c_void,
                                    exp: *mut libc::c_char,
                                    princs: *mut *mut *mut libc::c_char,
                                    count: *mut libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "454:1"]
        pub fn kadm5_get_policies(server_handle: *mut libc::c_void,
                                  exp: *mut libc::c_char,
                                  pols: *mut *mut *mut libc::c_char,
                                  count: *mut libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "462:1"]
        pub fn kadm5_free_name_list(server_handle: *mut libc::c_void,
                                    names: *mut *mut libc::c_char,
                                    count: libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn kadm5_init_krb5_context(_: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "467:1"]
        pub fn kadm5_init_iprop(server_handle: *mut libc::c_void,
                                db_args: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "475:1"]
        pub fn kadm5_purgekeys(server_handle: *mut libc::c_void,
                               principal: krb5_principal,
                               keepkvno: libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "479:1"]
        pub fn kadm5_get_strings(server_handle: *mut libc::c_void,
                                 principal: krb5_principal,
                                 strings_out: *mut *mut krb5_string_attr,
                                 count_out: *mut libc::c_int) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "484:1"]
        pub fn kadm5_set_string(server_handle: *mut libc::c_void,
                                principal: krb5_principal,
                                key: *const libc::c_char,
                                value: *const libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "489:1"]
        pub fn kadm5_free_strings(server_handle: *mut libc::c_void,
                                  strings: *mut krb5_string_attr,
                                  count: libc::c_int) -> kadm5_ret_t;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/pwd.h:44"]
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
        #[c2rust::src_loc = "110:1"]
        pub fn getpwuid(__uid: __uid_t) -> *mut passwd;
    }
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:34"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
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
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:34"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
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
        #[c2rust::src_loc = "347:12"]
        pub fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:34"]
pub mod unistd_h {
    use super::types_h::__uid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:34"]
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
#[c2rust::header_src = "/usr/include/time.h:34"]
pub mod time_h {
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::struct_tm_h::tm;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn strftime(__s: *mut libc::c_char, __maxsize: size_t,
                        __format: *const libc::c_char, __tp: *const tm)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn localtime(__timer: *const time_t) -> *mut tm;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:34"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:36"]
pub mod adm_proto_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_boolean, krb5_error_code,
                        krb5_flags, krb5_int32};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/adm_proto.h */
/*
 * Copyright 1995, 2007,2008,2009 by the Massachusetts Institute of Technology.
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
 * This is ugly, but avoids having to include k5-int or kdb.h for this.
 */
        /* KRB5_KDB5__ */
        /* Ditto for admin.h */
        /* KRB5_KDB5__ */
        /*
 * Function prototypes.
 */
        /* logger.c */
        #[no_mangle]
        #[c2rust::src_loc = "50:1"]
        pub fn krb5_klog_init(_: krb5_context, _: *mut libc::c_char,
                              _: *mut libc::c_char, _: krb5_boolean)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn krb5_klog_close(_: krb5_context);
        /* str_conv.c */
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn krb5_flagspec_to_mask(_: *const libc::c_char,
                                     _: *mut krb5_flags, _: *mut krb5_flags)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn krb5_flags_to_strings(_: krb5_int32,
                                     _: *mut *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "90:1"]
        pub fn krb5_string_to_keysalts(_: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: *const libc::c_char,
                                       _: krb5_boolean,
                                       _: *mut *mut krb5_key_salt_tuple,
                                       _: *mut krb5_int32) -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/cli/kadmin.h:47"]
pub mod kadmin_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "81:1"]
        pub fn get_date_rel(_: *mut libc::c_char, _: time_t) -> time_t;
    }
    /* __KADMIN_H__ */
    /* Yucky global variables */
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __uid_t, __gid_t, __off_t, __off64_t,
                        __time_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdarg_h::va_list;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::struct_tm_h::tm;
pub use self::k5_platform_h::{C2RustUnnamed, store_32_le, krb5int_strlcpy};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, krb5_ccache, _profile_t, _krb5_ccache,
                       krb5_c_valid_enctype, krb5_cc_close,
                       krb5_cc_get_principal, krb5_free_context,
                       krb5_parse_name, krb5_unparse_name, krb5_cc_resolve,
                       krb5_cc_default, krb5_free_principal, krb5_timeofday,
                       krb5_get_default_realm, krb5_sname_to_principal,
                       krb5_read_password, krb5_string_to_deltat,
                       krb5_enctype_to_name, krb5_salttype_to_string,
                       krb5_get_error_message, krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, ts2tt, plugin_mapping,
                         _kdb_log_context, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5int_c_deprecated_enctype};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, et_old_error_hook_func, com_err,
                          set_com_err_hook};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_string_attr_st, krb5_string_attr, krb5_key_data,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_policy_t, kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, kadm5_principal_ent_t,
                        _kadm5_policy_ent_t, kadm5_policy_ent_rec,
                        kadm5_policy_ent_t, _kadm5_config_params,
                        kadm5_config_params, kadm5_init_anonymous,
                        kadm5_init_with_password, kadm5_init_with_skey,
                        kadm5_init_with_creds, kadm5_lock, kadm5_unlock,
                        kadm5_destroy, kadm5_create_principal,
                        kadm5_create_principal_3, kadm5_delete_principal,
                        kadm5_modify_principal, kadm5_rename_principal,
                        kadm5_get_principal, kadm5_chpass_principal,
                        kadm5_chpass_principal_3, kadm5_randkey_principal,
                        kadm5_randkey_principal_3, kadm5_create_policy,
                        kadm5_delete_policy, kadm5_modify_policy,
                        kadm5_get_policy, kadm5_get_privs,
                        kadm5_free_principal_ent, kadm5_free_policy_ent,
                        kadm5_get_principals, kadm5_get_policies,
                        kadm5_free_name_list, kadm5_init_krb5_context,
                        kadm5_init_iprop, kadm5_purgekeys, kadm5_get_strings,
                        kadm5_set_string, kadm5_free_strings};
pub use self::pwd_h::{passwd, getpwuid};
use self::string_h::{memcpy, memset, strcmp, strchr, strlen};
use self::stdlib_h::{atoi, malloc, calloc, realloc, free, exit, getenv};
use self::stdio_h::{stdin, stderr, fprintf, printf, vfprintf, vprintf,
                    snprintf, asprintf, fgets};
use self::unistd_h::getuid;
use self::getopt_core_h::{optarg, optind, getopt};
use self::time_h::{time, strftime, localtime};
use self::libintl_h::dgettext;
use self::adm_proto_h::{krb5_klog_init, krb5_klog_close,
                        krb5_flagspec_to_mask, krb5_flags_to_strings,
                        krb5_string_to_keysalts};
use self::kadmin_h::get_date_rel;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1994, 2008 by the Massachusetts Institute of Technology.
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
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* Base functions for a kadmin command line interface using the OVSecure
 * library */
/* for "_" macro */
/* #include <sys/timeb.h> */
#[c2rust::src_loc = "49:21"]
static mut script_mode: krb5_boolean = 0 as libc::c_int as krb5_boolean;
#[no_mangle]
#[c2rust::src_loc = "50:5"]
pub static mut exit_status: libc::c_int = 0 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "51:7"]
pub static mut def_realm: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "52:7"]
pub static mut whoami: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "54:7"]
pub static mut handle: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
#[c2rust::src_loc = "55:14"]
pub static mut context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[no_mangle]
#[c2rust::src_loc = "56:7"]
pub static mut ccache_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
#[c2rust::src_loc = "58:5"]
pub static mut locked: libc::c_int = 0 as libc::c_int;
/* Like printf, but suppressed if script_mode is set. */
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn info(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if script_mode != 0 { return }
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
}
/* Like fprintf to stderr; also set exit_status if script_mode is set. */
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn error(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if script_mode != 0 { exit_status = 1 as libc::c_int }
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn usage() {
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"Usage: %s [-r realm] [-p principal] [-q query] [clnt|local args]\n              [command args...]\n\tclnt args: [-s admin_server[:port]] [[-c ccache]|[-k [-t keytab]]]|[-n]\n\tlocal args: [-x db_args]* [-d dbname] [-e \"enc:salt ...\"] [-m]where,\n\t[-x db_args]* - any number of database specific arguments.\n\t\t\tLook at each database documentation for supported arguments\n\x00"
                       as *const u8 as *const libc::c_char), whoami);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "117:1"]
unsafe extern "C" fn strdur(mut duration: time_t) -> *mut libc::c_char {
    static mut out: [libc::c_char; 50] = [0; 50];
    let mut neg: libc::c_int = 0;
    let mut days: libc::c_int = 0;
    let mut hours: libc::c_int = 0;
    let mut minutes: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    if duration < 0 as libc::c_int as libc::c_long {
        duration *= -(1 as libc::c_int) as libc::c_long;
        neg = 1 as libc::c_int
    } else { neg = 0 as libc::c_int }
    days =
        (duration / (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long)
            as libc::c_int;
    duration %= (24 as libc::c_int * 3600 as libc::c_int) as libc::c_long;
    hours = (duration / 3600 as libc::c_int as libc::c_long) as libc::c_int;
    duration %= 3600 as libc::c_int as libc::c_long;
    minutes = (duration / 60 as libc::c_int as libc::c_long) as libc::c_int;
    duration %= 60 as libc::c_int as libc::c_long;
    seconds = duration as libc::c_int;
    snprintf(out.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong,
             b"%s%d %s %02d:%02d:%02d\x00" as *const u8 as
                 *const libc::c_char,
             if neg != 0 {
                 b"-\x00" as *const u8 as *const libc::c_char
             } else { b"\x00" as *const u8 as *const libc::c_char }, days,
             if days == 1 as libc::c_int {
                 b"day\x00" as *const u8 as *const libc::c_char
             } else { b"days\x00" as *const u8 as *const libc::c_char },
             hours, minutes, seconds);
    return out.as_mut_ptr();
}
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn strdate(mut when: krb5_timestamp)
 -> *const libc::c_char {
    let mut tm: *mut tm = 0 as *mut tm;
    static mut out: [libc::c_char; 40] = [0; 40];
    let mut lcltim: time_t = ts2tt(when);
    tm = localtime(&mut lcltim);
    if tm.is_null() ||
           strftime(out.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 40]>() as
                        libc::c_ulong,
                    b"%a %b %d %H:%M:%S %Z %Y\x00" as *const u8 as
                        *const libc::c_char, tm) ==
               0 as libc::c_int as libc::c_ulong {
        krb5int_strlcpy(out.as_mut_ptr(),
                        b"(error)\x00" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 40]>() as
                            libc::c_ulong);
    }
    return out.as_mut_ptr();
}
/* Parse a date string using getdate.y.  On failure, output an error message
 * and return (time_t)-1. */
#[c2rust::src_loc = "157:1"]
unsafe extern "C" fn parse_date(mut str: *mut libc::c_char, mut now: time_t)
 -> time_t {
    let mut date: time_t = 0;
    date = get_date_rel(str, now);
    if date == -(1 as libc::c_int) as time_t {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"Invalid date specification \"%s\".\n\x00" as
                           *const u8 as *const libc::c_char), str);
    }
    return date;
}
/*
 * Parse a time interval.  Use krb5_string_to_deltat() if it works; otherwise
 * use getdate.y and subtract now, with sanity checks.  On failure, output an
 * error message and return (time_t)-1.
 */
#[c2rust::src_loc = "173:1"]
unsafe extern "C" fn parse_interval(mut str: *mut libc::c_char,
                                    mut now: time_t) -> time_t {
    let mut date: time_t = 0;
    let mut delta: krb5_deltat = 0;
    if krb5_string_to_deltat(str, &mut delta) == 0 as libc::c_int {
        return delta as time_t
    }
    date = parse_date(str, now);
    if date == -(1 as libc::c_int) as time_t { return date }
    /* Interpret an absolute time of 0 (e.g. "never") as an interval of 0. */
    if date == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as time_t
    }
    /* Don't return a negative interval if the date is in the past. */
    if date < now {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"Interval specification \"%s\" is in the past.\n\x00"
                           as *const u8 as *const libc::c_char), str);
        return -(1 as libc::c_int) as time_t
    }
    return date - now;
}
/* this is a wrapper to go around krb5_parse_principal so we can set
   the default realm up properly */
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn kadmin_parse_name(mut name: *mut libc::c_char,
                                       mut principal: *mut krb5_principal)
 -> krb5_error_code {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut result: libc::c_int = 0;
    /* assumes def_realm is initialized! */
    cp = strchr(name, '@' as i32);
    while !cp.is_null() {
        if cp.wrapping_offset_from(name) as libc::c_long != 0 &&
               *cp.offset(-(1 as libc::c_int as isize)) as libc::c_int !=
                   '\\' as i32 {
            break ;
        }
        cp = strchr(cp.offset(1 as libc::c_int as isize), '@' as i32)
    }
    if cp.is_null() {
        result =
            asprintf(&mut fullname as *mut *mut libc::c_char,
                     b"%s@%s\x00" as *const u8 as *const libc::c_char, name,
                     def_realm)
    } else {
        result =
            asprintf(&mut fullname as *mut *mut libc::c_char,
                     b"%s\x00" as *const u8 as *const libc::c_char, name)
    }
    if result < 0 as libc::c_int { return 12 as libc::c_int }
    retval = krb5_parse_name(context, fullname, principal);
    free(fullname as *mut libc::c_void);
    return retval;
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn extended_com_err_fn(mut myprog: *const libc::c_char,
                                         mut code: errcode_t,
                                         mut fmt: *const libc::c_char,
                                         mut args: ::std::ffi::VaList) {
    let mut emsg: *const libc::c_char = 0 as *const libc::c_char;
    if code != 0 {
        emsg = krb5_get_error_message(context, code as krb5_error_code);
        error(b"%s: %s \x00" as *const u8 as *const libc::c_char, myprog,
              emsg);
        krb5_free_error_message(context, emsg);
    } else { error(b"%s: \x00" as *const u8 as *const libc::c_char, myprog); }
    vfprintf(stderr, fmt, args.as_va_list());
    error(b"\n\x00" as *const u8 as *const libc::c_char);
}
/* Create a principal using the oldest appropriate kadm5 API. */
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn create_princ(mut princ: *mut kadm5_principal_ent_rec,
                                  mut mask: libc::c_long,
                                  mut n_ks: libc::c_int,
                                  mut ks: *mut krb5_key_salt_tuple,
                                  mut pass: *mut libc::c_char)
 -> krb5_error_code {
    if !ks.is_null() {
        return kadm5_create_principal_3(handle, princ, mask, n_ks, ks, pass)
                   as krb5_error_code
    } else {
        return kadm5_create_principal(handle, princ, mask, pass) as
                   krb5_error_code
    };
}
/* Randomize a principal's password using the appropriate kadm5 API. */
#[no_mangle]
#[c2rust::src_loc = "256:1"]
pub unsafe extern "C" fn randkey_princ(mut lhandle: *mut libc::c_void,
                                       mut princ: krb5_principal,
                                       mut keepold: krb5_boolean,
                                       mut n_ks: libc::c_int,
                                       mut ks: *mut krb5_key_salt_tuple,
                                       mut key: *mut *mut krb5_keyblock,
                                       mut n_keys: *mut libc::c_int)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    /* Try the newer API first, because the Solaris kadmind only creates DES
     * keys when the old API is used. */
    ret =
        kadm5_randkey_principal_3(lhandle, princ, keepold, n_ks, ks, key,
                                  n_keys) as krb5_error_code;
    /* Fall back to the old version if we get an error and aren't using any new
     * parameters. */
    if ret as libc::c_long == 43787528 as libc::c_long && keepold == 0 &&
           ks.is_null() {
        ret =
            kadm5_randkey_principal(lhandle, princ, key, n_keys) as
                krb5_error_code
    }
    return ret;
}
#[c2rust::src_loc = "276:1"]
unsafe extern "C" fn policy_exists(mut name: *const libc::c_char)
 -> krb5_boolean {
    let mut pol: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    if kadm5_get_policy(handle, name as *mut libc::c_char, &mut pol) !=
           0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as krb5_boolean
    }
    kadm5_free_policy_ent(handle, &mut pol);
    return 1 as libc::c_int as krb5_boolean;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/cli/kadmin.h */
/*
 * Copyright 2001 by the Massachusetts Institute of Technology.
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
 * Prototypes for kadmin functions called from SS library.
 */
/* It would be nice if ss produced a header file we could reference */
#[no_mangle]
#[c2rust::src_loc = "287:1"]
pub unsafe extern "C" fn kadmin_startup(mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char,
                                        mut request_out:
                                            *mut *mut libc::c_char,
                                        mut args_out:
                                            *mut *mut *mut libc::c_char) {
    extern "C" {
        #[link_name = "optarg"]
        pub static mut optarg_0: *mut libc::c_char;
    }
    let mut princstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keytab_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut query: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut password: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut luser: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut optchar: libc::c_int = 0;
    let mut freeprinc: libc::c_int = 0 as libc::c_int;
    let mut use_keytab: libc::c_int = 0 as libc::c_int;
    let mut use_anonymous: libc::c_int = 0 as libc::c_int;
    let mut pw: *mut passwd = 0 as *mut passwd;
    let mut retval: kadm5_ret_t = 0;
    let mut cc: krb5_ccache = 0 as *mut _krb5_ccache;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut params: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    let mut db_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut db_args_size: libc::c_int = 0 as libc::c_int;
    let mut db_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut svcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut realm: *mut libc::c_char = 0 as *mut libc::c_char;
    memset(&mut params as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    set_com_err_hook(Some(extended_com_err_fn as
                              unsafe extern "C" fn(_: *const libc::c_char,
                                                   _: errcode_t,
                                                   _: *const libc::c_char,
                                                   _: ::std::ffi::VaList)
                                  -> ()));
    retval = kadm5_init_krb5_context(&mut context) as kadm5_ret_t;
    if retval != 0 {
        com_err(whoami, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5 library\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    loop  {
        optchar =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"+x:r:p:knq:w:d:s:mc:t:e:ON\x00" as *const u8 as
                       *const libc::c_char);
        if !(optchar != -(1 as libc::c_int)) { break ; }
        match optchar {
            120 => {
                db_args_size += 1;
                db_args =
                    realloc(db_args as *mut libc::c_void,
                            (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((db_args_size +
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if db_args.is_null() {
                    error(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"%s: Cannot initialize. Not enough memory\n\x00"
                                       as *const u8 as *const libc::c_char),
                          whoami);
                    exit(1 as libc::c_int);
                }
                let ref mut fresh0 =
                    *db_args.offset((db_args_size - 1 as libc::c_int) as
                                        isize);
                *fresh0 = optarg;
                let ref mut fresh1 = *db_args.offset(db_args_size as isize);
                *fresh1 = 0 as *mut libc::c_char
            }
            114 => { def_realm = optarg }
            112 => { princstr = optarg }
            99 => { ccache_name = optarg }
            107 => { use_keytab += 1 }
            110 => { use_anonymous += 1 }
            116 => { keytab_name = optarg }
            119 => { password = optarg }
            113 => { query = optarg }
            100 => {
                /* db_name has to be passed as part of the db_args. */
                free(db_name as *mut libc::c_void);
                asprintf(&mut db_name as *mut *mut libc::c_char,
                         b"dbname=%s\x00" as *const u8 as *const libc::c_char,
                         optarg);
                db_args_size += 1;
                db_args =
                    realloc(db_args as *mut libc::c_void,
                            (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((db_args_size +
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if db_args.is_null() {
                    error(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"%s: Cannot initialize. Not enough memory\n\x00"
                                       as *const u8 as *const libc::c_char),
                          whoami);
                    exit(1 as libc::c_int);
                }
                let ref mut fresh2 =
                    *db_args.offset((db_args_size - 1 as libc::c_int) as
                                        isize);
                *fresh2 = db_name;
                let ref mut fresh3 = *db_args.offset(db_args_size as isize);
                *fresh3 = 0 as *mut libc::c_char
            }
            115 => {
                params.admin_server = optarg;
                params.mask |= 0x10000 as libc::c_int as libc::c_long
            }
            109 => {
                params.mkey_from_kbd = 1 as libc::c_int;
                params.mask |= 0x40000 as libc::c_int as libc::c_long
            }
            101 => {
                retval =
                    krb5_string_to_keysalts(optarg, 0 as *const libc::c_char,
                                            0 as *const libc::c_char,
                                            0 as libc::c_int as krb5_boolean,
                                            &mut params.keysalts,
                                            &mut params.num_keysalts) as
                        kadm5_ret_t;
                if retval != 0 {
                    com_err(whoami, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while parsing keysalts %s\x00" as
                                         *const u8 as *const libc::c_char),
                            optarg);
                    exit(1 as libc::c_int);
                }
                params.mask |= 0x8000 as libc::c_int as libc::c_long
            }
            79 => { params.mask |= 0x100000 as libc::c_int as libc::c_long }
            78 => { params.mask |= 0x400000 as libc::c_int as libc::c_long }
            _ => { usage(); }
        }
    }
    if !ccache_name.is_null() && use_keytab != 0 ||
           !keytab_name.is_null() && use_keytab == 0 ||
           !ccache_name.is_null() && use_anonymous != 0 ||
           use_anonymous != 0 && use_keytab != 0 {
        usage();
    }
    if !query.is_null() && !(*argv.offset(optind as isize)).is_null() {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"%s: -q is exclusive with command-line query\x00" as
                           *const u8 as *const libc::c_char), whoami);
        usage();
    }
    if !(*argv.offset(optind as isize)).is_null() {
        script_mode = 1 as libc::c_int as krb5_boolean
    }
    if def_realm.is_null() &&
           krb5_get_default_realm(context, &mut def_realm) != 0 {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"%s: unable to get default realm\n\x00" as *const u8
                           as *const libc::c_char), whoami);
        exit(1 as libc::c_int);
    }
    params.mask |= 0x1 as libc::c_int as libc::c_long;
    params.realm = def_realm;
    if params.mask & 0x100000 as libc::c_int as libc::c_long != 0 {
        svcname =
            b"kadmin/admin\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    } else { svcname = 0 as *mut libc::c_char }
    /*
     * Set cc to an open credentials cache, either specified by the -c
     * argument or the default.
     */
    if ccache_name.is_null() {
        retval = krb5_cc_default(context, &mut cc) as kadm5_ret_t;
        if retval != 0 {
            com_err(whoami, retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while opening default credentials cache\x00" as
                                 *const u8 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    } else {
        retval =
            krb5_cc_resolve(context, ccache_name, &mut cc) as kadm5_ret_t;
        if retval != 0 {
            com_err(whoami, retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while opening credentials cache %s\x00" as
                                 *const u8 as *const libc::c_char),
                    ccache_name);
            exit(1 as libc::c_int);
        }
    }
    /*
     * If no principal name is specified: If authenticating anonymously, use
     * the anonymouse principal for the local realm, else if a ccache was
     * specified and its primary principal name can be read, it is used, else
     * if a keytab was specified, the principal name is host/hostname,
     * otherwise append "/admin" to the primary name of the default ccache,
     * $USER, or pw_name.
     *
     * Gee, 100+ lines to figure out the client principal name.  This
     * should be compressed...
     */
    if princstr.is_null() {
        if use_anonymous != 0 {
            if asprintf(&mut princstr as *mut *mut libc::c_char,
                        b"%s/%s@%s\x00" as *const u8 as *const libc::c_char,
                        b"WELLKNOWN\x00" as *const u8 as *const libc::c_char,
                        b"ANONYMOUS\x00" as *const u8 as *const libc::c_char,
                        def_realm) < 0 as libc::c_int {
                error(dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"%s: out of memory\n\x00" as *const u8 as
                                   *const libc::c_char), whoami);
                exit(1 as libc::c_int);
            }
            freeprinc += 1
        } else if !ccache_name.is_null() &&
                      krb5_cc_get_principal(context, cc, &mut princ) == 0 {
            retval =
                krb5_unparse_name(context, princ as krb5_const_principal,
                                  &mut princstr) as kadm5_ret_t;
            if retval != 0 {
                com_err(whoami, retval,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while canonicalizing principal name\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
            krb5_free_principal(context, princ);
            freeprinc += 1
        } else if use_keytab != 0 as libc::c_int {
            retval =
                krb5_sname_to_principal(context, 0 as *const libc::c_char,
                                        b"host\x00" as *const u8 as
                                            *const libc::c_char,
                                        3 as libc::c_int, &mut princ) as
                    kadm5_ret_t;
            if retval != 0 {
                com_err(whoami, retval,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"creating host service principal\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
            retval =
                krb5_unparse_name(context, princ as krb5_const_principal,
                                  &mut princstr) as kadm5_ret_t;
            if retval != 0 {
                com_err(whoami, retval,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while canonicalizing principal name\x00" as
                                     *const u8 as *const libc::c_char));
                exit(1 as libc::c_int);
            }
            krb5_free_principal(context, princ);
            freeprinc += 1
        } else if krb5_cc_get_principal(context, cc, &mut princ) == 0 {
            if krb5_unparse_name(context, princ as krb5_const_principal,
                                 &mut canon) != 0 {
                error(dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"%s: unable to canonicalize principal\n\x00"
                                   as *const u8 as *const libc::c_char),
                      whoami);
                exit(1 as libc::c_int);
            }
            /* Strip out realm of principal if it's there. */
            realm = strchr(canon, '@' as i32);
            while !realm.is_null() {
                if realm > canon &&
                       *realm.offset(-(1 as libc::c_int as isize)) as
                           libc::c_int != '\\' as i32 {
                    break ;
                }
                realm =
                    strchr(realm.offset(1 as libc::c_int as isize),
                           '@' as i32)
            }
            if !realm.is_null() {
                let fresh4 = realm;
                realm = realm.offset(1);
                *fresh4 = '\u{0}' as i32 as libc::c_char
            }
            cp = strchr(canon, '/' as i32);
            while !cp.is_null() {
                if cp > canon &&
                       *cp.offset(-(1 as libc::c_int as isize)) as libc::c_int
                           != '\\' as i32 {
                    break ;
                }
                cp = strchr(cp.offset(1 as libc::c_int as isize), '/' as i32)
            }
            if !cp.is_null() { *cp = '\u{0}' as i32 as libc::c_char }
            if asprintf(&mut princstr as *mut *mut libc::c_char,
                        b"%s/admin%s%s\x00" as *const u8 as
                            *const libc::c_char, canon,
                        (if !realm.is_null() {
                             b"@\x00" as *const u8 as *const libc::c_char
                         } else {
                             b"\x00" as *const u8 as *const libc::c_char
                         }),
                        (if !realm.is_null() {
                             realm as *const libc::c_char
                         } else {
                             b"\x00" as *const u8 as *const libc::c_char
                         })) < 0 as libc::c_int {
                error(dgettext(b"mit-krb5\x00" as *const u8 as
                                   *const libc::c_char,
                               b"%s: out of memory\n\x00" as *const u8 as
                                   *const libc::c_char), whoami);
                exit(1 as libc::c_int);
            }
            free(canon as *mut libc::c_void);
            krb5_free_principal(context, princ);
            freeprinc += 1
        } else {
            luser = getenv(b"USER\x00" as *const u8 as *const libc::c_char);
            if !luser.is_null() {
                if asprintf(&mut princstr as *mut *mut libc::c_char,
                            b"%s/admin@%s\x00" as *const u8 as
                                *const libc::c_char, luser, def_realm) <
                       0 as libc::c_int {
                    error(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"%s: out of memory\n\x00" as *const u8 as
                                       *const libc::c_char), whoami);
                    exit(1 as libc::c_int);
                }
                freeprinc += 1
            } else {
                pw = getpwuid(getuid());
                if !pw.is_null() {
                    if asprintf(&mut princstr as *mut *mut libc::c_char,
                                b"%s/admin@%s\x00" as *const u8 as
                                    *const libc::c_char, (*pw).pw_name,
                                def_realm) < 0 as libc::c_int {
                        error(dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"%s: out of memory\n\x00" as *const u8
                                           as *const libc::c_char), whoami);
                        exit(1 as libc::c_int);
                    }
                    freeprinc += 1
                } else {
                    error(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"%s: unable to figure out a principal name\n\x00"
                                       as *const u8 as *const libc::c_char),
                          whoami);
                    exit(1 as libc::c_int);
                }
            }
        }
    }
    retval =
        krb5_klog_init(context,
                       b"admin_server\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char, whoami,
                       0 as libc::c_int as krb5_boolean) as kadm5_ret_t;
    if retval != 0 {
        com_err(whoami, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting up logging\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /*
     * Initialize the kadm5 connection.  If we were given a ccache,
     * use it.  Otherwise, use/prompt for the password.
     */
    if !ccache_name.is_null() {
        info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                      b"Authenticating as principal %s with existing credentials.\n\x00"
                          as *const u8 as *const libc::c_char), princstr);
        retval =
            kadm5_init_with_creds(context, princstr, cc, svcname, &mut params,
                                  (0x12345600 as libc::c_int |
                                       0x1 as libc::c_int) as krb5_ui_4,
                                  (0x12345700 as libc::c_int |
                                       0x4 as libc::c_int) as krb5_ui_4,
                                  db_args, &mut handle)
    } else if use_anonymous != 0 {
        info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                      b"Authenticating as principal %s with password; anonymous requested.\n\x00"
                          as *const u8 as *const libc::c_char), princstr);
        retval =
            kadm5_init_anonymous(context, princstr, svcname, &mut params,
                                 (0x12345600 as libc::c_int |
                                      0x1 as libc::c_int) as krb5_ui_4,
                                 (0x12345700 as libc::c_int |
                                      0x4 as libc::c_int) as krb5_ui_4,
                                 db_args, &mut handle)
    } else if use_keytab != 0 {
        if !keytab_name.is_null() {
            info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Authenticating as principal %s with keytab %s.\n\x00"
                              as *const u8 as *const libc::c_char), princstr,
                 keytab_name);
        } else {
            info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Authenticating as principal %s with default keytab.\n\x00"
                              as *const u8 as *const libc::c_char), princstr);
        }
        retval =
            kadm5_init_with_skey(context, princstr, keytab_name, svcname,
                                 &mut params,
                                 (0x12345600 as libc::c_int |
                                      0x1 as libc::c_int) as krb5_ui_4,
                                 (0x12345700 as libc::c_int |
                                      0x4 as libc::c_int) as krb5_ui_4,
                                 db_args, &mut handle)
    } else {
        info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                      b"Authenticating as principal %s with password.\n\x00"
                          as *const u8 as *const libc::c_char), princstr);
        retval =
            kadm5_init_with_password(context, princstr, password, svcname,
                                     &mut params,
                                     (0x12345600 as libc::c_int |
                                          0x1 as libc::c_int) as krb5_ui_4,
                                     (0x12345700 as libc::c_int |
                                          0x4 as libc::c_int) as krb5_ui_4,
                                     db_args, &mut handle)
    }
    if retval != 0 {
        com_err(whoami, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing %s interface\x00" as *const u8
                             as *const libc::c_char), whoami);
        if retval == 43787562 as libc::c_long ||
               retval == 43787563 as libc::c_long {
            usage();
        }
        exit(1 as libc::c_int);
    }
    if freeprinc != 0 { free(princstr as *mut libc::c_void); }
    free(params.keysalts as *mut libc::c_void);
    free(db_name as *mut libc::c_void);
    free(db_args as *mut libc::c_void);
    retval = krb5_cc_close(context, cc) as kadm5_ret_t;
    if retval != 0 {
        com_err(whoami, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while closing ccache %s\x00" as *const u8 as
                             *const libc::c_char), ccache_name);
        exit(1 as libc::c_int);
    }
    retval =
        kadm5_init_iprop(handle, 0 as *mut *mut libc::c_char) as kadm5_ret_t;
    if retval != 0 {
        com_err(whoami, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while mapping update log\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    *request_out = query;
    *args_out = argv.offset(optind as isize);
}
#[no_mangle]
#[c2rust::src_loc = "609:1"]
pub unsafe extern "C" fn quit() -> libc::c_int {
    let mut retval: kadm5_ret_t = 0;
    if locked != 0 {
        retval = kadm5_unlock(handle);
        if retval != 0 {
            com_err(b"quit\x00" as *const u8 as *const libc::c_char, retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while unlocking locked database\x00" as
                                 *const u8 as *const libc::c_char));
            return 1 as libc::c_int
        }
        locked = 0 as libc::c_int
    }
    kadm5_destroy(handle);
    if !ccache_name.is_null() && script_mode == 0 {
        fprintf(stderr,
                b"\n\x07\x07\x07%s\x00" as *const u8 as *const libc::c_char,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Administration credentials NOT DESTROYED.\n\x00" as
                             *const u8 as *const libc::c_char));
    }
    /* insert more random cleanup here */
    krb5_klog_close(context);
    krb5_free_context(context);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "635:1"]
pub unsafe extern "C" fn kadmin_lock(mut argc: libc::c_int,
                                     mut argv: *mut *mut libc::c_char) {
    let mut retval: kadm5_ret_t = 0;
    if locked != 0 { return }
    retval = kadm5_lock(handle);
    if retval != 0 {
        com_err(b"lock\x00" as *const u8 as *const libc::c_char, retval,
                b"\x00" as *const u8 as *const libc::c_char);
        return
    }
    locked = 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "650:1"]
pub unsafe extern "C" fn kadmin_unlock(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut retval: kadm5_ret_t = 0;
    if locked == 0 { return }
    retval = kadm5_unlock(handle);
    if retval != 0 {
        com_err(b"unlock\x00" as *const u8 as *const libc::c_char, retval,
                b"\x00" as *const u8 as *const libc::c_char);
        return
    }
    locked = 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "665:1"]
pub unsafe extern "C" fn kadmin_delprinc(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut retval: kadm5_ret_t = 0;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reply: [libc::c_char; 5] = [0; 5];
    if !(argc == 2 as libc::c_int ||
             argc == 3 as libc::c_int &&
                 strcmp(b"-force\x00" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize)) == 0) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: delete_principal [-force] principal\n\x00" as
                           *const u8 as *const libc::c_char));
        return
    }
    retval =
        kadmin_parse_name(*argv.offset((argc - 1 as libc::c_int) as isize),
                          &mut princ) as kadm5_ret_t;
    if retval != 0 {
        com_err(b"delete_principal\x00" as *const u8 as *const libc::c_char,
                retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal name\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut canon)
            as kadm5_ret_t;
    if retval != 0 {
        com_err(b"delete_principal\x00" as *const u8 as *const libc::c_char,
                retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        if argc == 2 as libc::c_int && script_mode == 0 {
            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"Are you sure you want to delete the principal \"%s\"? (yes/no): \x00"
                                as *const u8 as *const libc::c_char), canon);
            fgets(reply.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                      as libc::c_int, stdin);
            if strcmp(b"yes\n\x00" as *const u8 as *const libc::c_char,
                      reply.as_mut_ptr()) != 0 {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Principal \"%s\" not deleted\n\x00" as
                                     *const u8 as *const libc::c_char),
                        canon);
                current_block = 8321155641006276435;
            } else { current_block = 15904375183555213903; }
        } else { current_block = 15904375183555213903; }
        match current_block {
            8321155641006276435 => { }
            _ => {
                retval = kadm5_delete_principal(handle, princ);
                if retval != 0 {
                    com_err(b"delete_principal\x00" as *const u8 as
                                *const libc::c_char, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while deleting principal \"%s\"\x00" as
                                         *const u8 as *const libc::c_char),
                            canon);
                } else {
                    info(dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Principal \"%s\" deleted.\n\x00" as
                                      *const u8 as *const libc::c_char),
                         canon);
                    info(dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Make sure that you have removed this principal from all ACLs before reusing.\n\x00"
                                      as *const u8 as *const libc::c_char));
                }
            }
        }
    }
    krb5_free_principal(context, princ);
    free(canon as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "713:1"]
pub unsafe extern "C" fn kadmin_renameprinc(mut argc: libc::c_int,
                                            mut argv:
                                                *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut retval: kadm5_ret_t = 0;
    let mut oprinc: krb5_principal = 0 as krb5_principal;
    let mut nprinc: krb5_principal = 0 as krb5_principal;
    let mut ocanon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ncanon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reply: [libc::c_char; 5] = [0; 5];
    if !(argc == 3 as libc::c_int ||
             argc == 4 as libc::c_int &&
                 strcmp(b"-force\x00" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize)) == 0) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: rename_principal [-force] old_principal new_principal\n\x00"
                           as *const u8 as *const libc::c_char));
        return
    }
    retval =
        kadmin_parse_name(*argv.offset((argc - 2 as libc::c_int) as isize),
                          &mut oprinc) as kadm5_ret_t;
    if retval != 0 {
        com_err(b"rename_principal\x00" as *const u8 as *const libc::c_char,
                retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing old principal name\x00" as *const u8
                             as *const libc::c_char));
    } else {
        retval =
            kadmin_parse_name(*argv.offset((argc - 1 as libc::c_int) as
                                               isize), &mut nprinc) as
                kadm5_ret_t;
        if retval != 0 {
            com_err(b"rename_principal\x00" as *const u8 as
                        *const libc::c_char, retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while parsing new principal name\x00" as
                                 *const u8 as *const libc::c_char));
        } else {
            retval =
                krb5_unparse_name(context, oprinc as krb5_const_principal,
                                  &mut ocanon) as kadm5_ret_t;
            if retval != 0 {
                com_err(b"rename_principal\x00" as *const u8 as
                            *const libc::c_char, retval,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while canonicalizing old principal\x00" as
                                     *const u8 as *const libc::c_char));
            } else {
                retval =
                    krb5_unparse_name(context, nprinc as krb5_const_principal,
                                      &mut ncanon) as kadm5_ret_t;
                if retval != 0 {
                    com_err(b"rename_principal\x00" as *const u8 as
                                *const libc::c_char, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while canonicalizing new principal\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                } else {
                    if argc == 3 as libc::c_int && script_mode == 0 {
                        printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"Are you sure you want to rename the principal \"%s\" to \"%s\"? (yes/no): \x00"
                                            as *const u8 as
                                            *const libc::c_char), ocanon,
                               ncanon);
                        fgets(reply.as_mut_ptr(),
                              ::std::mem::size_of::<[libc::c_char; 5]>() as
                                  libc::c_ulong as libc::c_int, stdin);
                        if strcmp(b"yes\n\x00" as *const u8 as
                                      *const libc::c_char, reply.as_mut_ptr())
                               != 0 {
                            fprintf(stderr,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"Principal \"%s\" not renamed\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char),
                                    ocanon);
                            current_block = 16515345418741672010;
                        } else { current_block = 2719512138335094285; }
                    } else { current_block = 2719512138335094285; }
                    match current_block {
                        16515345418741672010 => { }
                        _ => {
                            retval =
                                kadm5_rename_principal(handle, oprinc,
                                                       nprinc);
                            if retval != 0 {
                                com_err(b"rename_principal\x00" as *const u8
                                            as *const libc::c_char, retval,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while renaming principal \"%s\" to \"%s\"\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        ocanon, ncanon);
                            } else {
                                info(dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"Principal \"%s\" renamed to \"%s\".\n\x00"
                                                  as *const u8 as
                                                  *const libc::c_char),
                                     ocanon, ncanon);
                                info(dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"Make sure that you have removed the old principal from all ACLs before reusing.\n\x00"
                                                  as *const u8 as
                                                  *const libc::c_char));
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_principal(context, nprinc);
    krb5_free_principal(context, oprinc);
    free(ncanon as *mut libc::c_void);
    free(ocanon as *mut libc::c_void);
}
#[c2rust::src_loc = "777:1"]
unsafe extern "C" fn cpw_usage(mut str: *const libc::c_char) {
    if !str.is_null() {
        error(b"%s\n\x00" as *const u8 as *const libc::c_char, str);
    }
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"usage: change_password [-randkey] [-keepold] [-e keysaltlist] [-pw password] principal\n\x00"
                       as *const u8 as *const libc::c_char));
}
#[no_mangle]
#[c2rust::src_loc = "786:1"]
pub unsafe extern "C" fn kadmin_cpw(mut argc: libc::c_int,
                                    mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut retval: kadm5_ret_t = 0;
    static mut newpw: [libc::c_char; 1024] = [0; 1024];
    static mut prompt1: [libc::c_char; 1024] = [0; 1024];
    static mut prompt2: [libc::c_char; 1024] = [0; 1024];
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pwarg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut randkey: libc::c_int = 0 as libc::c_int;
    let mut keepold: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut db_args: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut db_args_size: libc::c_int = 0 as libc::c_int;
    if argc < 1 as libc::c_int { cpw_usage(0 as *const libc::c_char); return }
    argv = argv.offset(1);
    argc -= 1;
    loop  {
        if !(argc > 0 as libc::c_int && **argv as libc::c_int == '-' as i32) {
            current_block = 14447253356787937536;
            break ;
        }
        if strcmp(b"-x\x00" as *const u8 as *const libc::c_char, *argv) == 0 {
            argc -= 1;
            if argc < 1 as libc::c_int {
                cpw_usage(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"change_password: missing db argument\x00"
                                       as *const u8 as *const libc::c_char));
                current_block = 17736612138064574542;
                break ;
            } else {
                db_args_size += 1;
                db_args =
                    realloc(db_args as *mut libc::c_void,
                            (::std::mem::size_of::<*mut libc::c_char>() as
                                 libc::c_ulong).wrapping_mul((db_args_size +
                                                                  1 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if db_args.is_null() {
                    error(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"change_password: Not enough memory\n\x00"
                                       as *const u8 as *const libc::c_char));
                    exit(1 as libc::c_int);
                }
                argv = argv.offset(1);
                let ref mut fresh5 =
                    *db_args.offset((db_args_size - 1 as libc::c_int) as
                                        isize);
                *fresh5 = *argv;
                let ref mut fresh6 = *db_args.offset(db_args_size as isize);
                *fresh6 = 0 as *mut libc::c_char
            }
        } else if strcmp(b"-pw\x00" as *const u8 as *const libc::c_char,
                         *argv) == 0 {
            argc -= 1;
            if argc < 1 as libc::c_int {
                cpw_usage(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"change_password: missing password arg\x00"
                                       as *const u8 as *const libc::c_char));
                current_block = 17736612138064574542;
                break ;
            } else { argv = argv.offset(1); pwarg = *argv }
        } else if strcmp(b"-randkey\x00" as *const u8 as *const libc::c_char,
                         *argv) == 0 {
            randkey += 1
        } else if strcmp(b"-keepold\x00" as *const u8 as *const libc::c_char,
                         *argv) == 0 {
            keepold = 1 as libc::c_int as krb5_boolean
        } else if strcmp(b"-e\x00" as *const u8 as *const libc::c_char, *argv)
                      == 0 {
            argc -= 1;
            if argc < 1 as libc::c_int {
                cpw_usage(dgettext(b"mit-krb5\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"change_password: missing keysaltlist arg\x00"
                                       as *const u8 as *const libc::c_char));
                current_block = 17736612138064574542;
                break ;
            } else {
                argv = argv.offset(1);
                retval =
                    krb5_string_to_keysalts(*argv, 0 as *const libc::c_char,
                                            0 as *const libc::c_char,
                                            0 as libc::c_int as krb5_boolean,
                                            &mut ks_tuple, &mut n_ks_tuple) as
                        kadm5_ret_t;
                if retval != 0 {
                    com_err(b"change_password\x00" as *const u8 as
                                *const libc::c_char, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while parsing keysalts %s\x00" as
                                         *const u8 as *const libc::c_char),
                            *argv);
                    current_block = 17736612138064574542;
                    break ;
                }
            }
        } else {
            com_err(b"change_password\x00" as *const u8 as
                        *const libc::c_char, 0 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"unrecognized option %s\x00" as *const u8 as
                                 *const libc::c_char), *argv);
            cpw_usage(0 as *const libc::c_char);
            current_block = 17736612138064574542;
            break ;
        }
        argc -= 1;
        argv = argv.offset(1)
    }
    match current_block {
        14447253356787937536 => {
            if argc != 1 as libc::c_int {
                if argc < 1 as libc::c_int {
                    com_err(b"change_password\x00" as *const u8 as
                                *const libc::c_char,
                            0 as libc::c_int as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"missing principal name\x00" as
                                         *const u8 as *const libc::c_char));
                } else {
                    com_err(b"change_password\x00" as *const u8 as
                                *const libc::c_char,
                            0 as libc::c_int as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"too many arguments\x00" as *const u8 as
                                         *const libc::c_char));
                }
                cpw_usage(0 as *const libc::c_char);
            } else {
                retval = kadmin_parse_name(*argv, &mut princ) as kadm5_ret_t;
                if retval != 0 {
                    com_err(b"change_password\x00" as *const u8 as
                                *const libc::c_char, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while parsing principal name\x00" as
                                         *const u8 as *const libc::c_char));
                } else {
                    retval =
                        krb5_unparse_name(context,
                                          princ as krb5_const_principal,
                                          &mut canon) as kadm5_ret_t;
                    if retval != 0 {
                        com_err(b"change_password\x00" as *const u8 as
                                    *const libc::c_char, retval,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while canonicalizing principal\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                    } else if !pwarg.is_null() {
                        if keepold != 0 || !ks_tuple.is_null() {
                            retval =
                                kadm5_chpass_principal_3(handle, princ,
                                                         keepold, n_ks_tuple,
                                                         ks_tuple, pwarg)
                        } else {
                            retval =
                                kadm5_chpass_principal(handle, princ, pwarg)
                        }
                        if retval != 0 {
                            com_err(b"change_password\x00" as *const u8 as
                                        *const libc::c_char, retval,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while changing password for \"%s\".\x00"
                                                 as *const u8 as
                                                 *const libc::c_char), canon);
                        } else {
                            info(dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Password for \"%s\" changed.\n\x00"
                                              as *const u8 as
                                              *const libc::c_char), canon);
                        }
                    } else if randkey != 0 {
                        retval =
                            randkey_princ(handle, princ, keepold, n_ks_tuple,
                                          ks_tuple,
                                          0 as *mut *mut krb5_keyblock,
                                          0 as *mut libc::c_int) as
                                kadm5_ret_t;
                        if retval != 0 {
                            com_err(b"change_password\x00" as *const u8 as
                                        *const libc::c_char, retval,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while randomizing key for \"%s\".\x00"
                                                 as *const u8 as
                                                 *const libc::c_char), canon);
                        } else {
                            info(dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Key for \"%s\" randomized.\n\x00"
                                              as *const u8 as
                                              *const libc::c_char), canon);
                        }
                    } else {
                        let mut i: libc::c_uint =
                            (::std::mem::size_of::<[libc::c_char; 1024]>() as
                                 libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_uint;
                        snprintf(prompt1.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 1024]>()
                                     as libc::c_ulong,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Enter password for principal \"%s\"\x00"
                                              as *const u8 as
                                              *const libc::c_char), canon);
                        snprintf(prompt2.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 1024]>()
                                     as libc::c_ulong,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"Re-enter password for principal \"%s\"\x00"
                                              as *const u8 as
                                              *const libc::c_char), canon);
                        retval =
                            krb5_read_password(context, prompt1.as_mut_ptr(),
                                               prompt2.as_mut_ptr(),
                                               newpw.as_mut_ptr(), &mut i) as
                                kadm5_ret_t;
                        if retval != 0 {
                            com_err(b"change_password\x00" as *const u8 as
                                        *const libc::c_char, retval,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while reading password for \"%s\".\x00"
                                                 as *const u8 as
                                                 *const libc::c_char), canon);
                        } else {
                            if keepold != 0 || !ks_tuple.is_null() {
                                retval =
                                    kadm5_chpass_principal_3(handle, princ,
                                                             keepold,
                                                             n_ks_tuple,
                                                             ks_tuple,
                                                             newpw.as_mut_ptr())
                            } else {
                                retval =
                                    kadm5_chpass_principal(handle, princ,
                                                           newpw.as_mut_ptr())
                            }
                            memset(newpw.as_mut_ptr() as *mut libc::c_void,
                                   0 as libc::c_int,
                                   ::std::mem::size_of::<[libc::c_char; 1024]>()
                                       as libc::c_ulong);
                            if retval != 0 {
                                com_err(b"change_password\x00" as *const u8 as
                                            *const libc::c_char, retval,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while changing password for \"%s\".\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        canon);
                            } else {
                                info(dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"Password for \"%s\" changed.\n\x00"
                                                  as *const u8 as
                                                  *const libc::c_char),
                                     canon);
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    free(canon as *mut libc::c_void);
    free(db_args as *mut libc::c_void);
    krb5_free_principal(context, princ);
    free(ks_tuple as *mut libc::c_void);
}
#[c2rust::src_loc = "926:1"]
unsafe extern "C" fn kadmin_free_tl_data(mut n_tl_datap: *mut krb5_int16,
                                         mut tl_datap:
                                             *mut *mut krb5_tl_data) {
    let mut tl_data: *mut krb5_tl_data = *tl_datap;
    let mut next: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut n_tl_data: libc::c_int = *n_tl_datap as libc::c_int;
    let mut i: libc::c_int = 0;
    *n_tl_datap = 0 as libc::c_int as krb5_int16;
    *tl_datap = 0 as *mut krb5_tl_data;
    i = 0 as libc::c_int;
    while !tl_data.is_null() && i < n_tl_data {
        next = (*tl_data).tl_data_next;
        free((*tl_data).tl_data_contents as *mut libc::c_void);
        free(tl_data as *mut libc::c_void);
        tl_data = next;
        i += 1
    };
}
/* Construct a tl_data element and add it to the tail of *tl_datap. */
#[c2rust::src_loc = "945:1"]
unsafe extern "C" fn add_tl_data(mut n_tl_datap: *mut krb5_int16,
                                 mut tl_datap: *mut *mut krb5_tl_data,
                                 mut tl_type: krb5_int16, mut len: krb5_ui_2,
                                 mut contents: *mut krb5_octet) {
    let mut tl_data: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut copy: *mut krb5_octet = 0 as *mut krb5_octet;
    copy = malloc(len as libc::c_ulong) as *mut krb5_octet;
    tl_data =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong) as
            *mut krb5_tl_data;
    if copy.is_null() || tl_data.is_null() {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"Not enough memory\n\x00" as *const u8 as
                           *const libc::c_char));
        exit(1 as libc::c_int);
    }
    memcpy(copy as *mut libc::c_void, contents as *const libc::c_void,
           len as libc::c_ulong);
    (*tl_data).tl_data_type = tl_type;
    (*tl_data).tl_data_length = len;
    (*tl_data).tl_data_contents = copy;
    (*tl_data).tl_data_next = 0 as *mut _krb5_tl_data;
    while !(*tl_datap).is_null() { tl_datap = &mut (**tl_datap).tl_data_next }
    *tl_datap = tl_data;
    *n_tl_datap += 1;
}
#[c2rust::src_loc = "970:1"]
unsafe extern "C" fn unlock_princ(mut princ: kadm5_principal_ent_t,
                                  mut mask: *mut libc::c_long,
                                  mut caller: *const libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut now: krb5_timestamp = 0;
    let mut timebuf: [krb5_octet; 4] = [0; 4];
    /* Zero out the failed auth count. */
    (*princ).fail_auth_count = 0 as libc::c_int as krb5_kvno;
    *mask |= 0x10000 as libc::c_int as libc::c_long;
    /* Record the timestamp of this unlock operation so that replica KDCs will
     * see it, since fail_auth_count is unreplicated. */
    retval = krb5_timeofday(context, &mut now);
    if retval != 0 {
        com_err(caller, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting time\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    store_32_le(now as libc::c_uint,
                timebuf.as_mut_ptr() as *mut libc::c_void);
    add_tl_data(&mut (*princ).n_tl_data, &mut (*princ).tl_data,
                0x700 as libc::c_int as krb5_int16,
                4 as libc::c_int as krb5_ui_2, timebuf.as_mut_ptr());
    *mask |= 0x40000 as libc::c_int as libc::c_long;
}
/*
 * Parse addprinc or modprinc arguments.  Some output fields may be
 * filled in on error.
 */
#[c2rust::src_loc = "998:1"]
unsafe extern "C" fn kadmin_parse_princ_args(mut argc: libc::c_int,
                                             mut argv: *mut *mut libc::c_char,
                                             mut oprinc:
                                                 kadm5_principal_ent_t,
                                             mut mask: *mut libc::c_long,
                                             mut pass: *mut *mut libc::c_char,
                                             mut randkey: *mut krb5_boolean,
                                             mut nokey: *mut krb5_boolean,
                                             mut ks_tuple:
                                                 *mut *mut krb5_key_salt_tuple,
                                             mut n_ks_tuple: *mut libc::c_int,
                                             mut caller: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut now: time_t = 0;
    let mut date: time_t = 0;
    let mut interval: time_t = 0;
    let mut retval: krb5_error_code = 0;
    *mask = 0 as libc::c_int as libc::c_long;
    *pass = 0 as *mut libc::c_char;
    *n_ks_tuple = 0 as libc::c_int;
    *ks_tuple = 0 as *mut krb5_key_salt_tuple;
    time(&mut now);
    *randkey = 0 as libc::c_int as krb5_boolean;
    *nokey = 0 as libc::c_int as krb5_boolean;
    i = 1 as libc::c_int;
    while i < argc - 1 as libc::c_int {
        if strcmp(b"-x\x00" as *const u8 as *const libc::c_char,
                  *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            add_tl_data(&mut (*oprinc).n_tl_data, &mut (*oprinc).tl_data,
                        0x7fff as libc::c_int as krb5_int16,
                        strlen(*argv.offset(i as
                                                isize)).wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                            as krb5_ui_2,
                        *argv.offset(i as isize) as *mut krb5_octet);
            *mask |= 0x40000 as libc::c_int as libc::c_long
        } else if strcmp(b"-expire\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            date = parse_date(*argv.offset(i as isize), now);
            if date == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*oprinc).princ_expire_time = date as krb5_timestamp;
            *mask |= 0x2 as libc::c_int as libc::c_long
        } else if strcmp(b"-pwexpire\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            date = parse_date(*argv.offset(i as isize), now);
            if date == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*oprinc).pw_expiration = date as krb5_timestamp;
            *mask |= 0x4 as libc::c_int as libc::c_long
        } else if strcmp(b"-maxlife\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            interval = parse_interval(*argv.offset(i as isize), now);
            if interval == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*oprinc).max_life = interval as krb5_deltat;
            *mask |= 0x20 as libc::c_int as libc::c_long
        } else if strcmp(b"-maxrenewlife\x00" as *const u8 as
                             *const libc::c_char, *argv.offset(i as isize)) ==
                      0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            interval = parse_interval(*argv.offset(i as isize), now);
            if interval == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*oprinc).max_renewable_life = interval as krb5_deltat;
            *mask |= 0x2000 as libc::c_int as libc::c_long
        } else if strcmp(b"-kvno\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            (*oprinc).kvno = atoi(*argv.offset(i as isize)) as krb5_kvno;
            *mask |= 0x100 as libc::c_int as libc::c_long
        } else if strcmp(b"-policy\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            (*oprinc).policy = *argv.offset(i as isize);
            *mask |= 0x800 as libc::c_int as libc::c_long
        } else if strcmp(b"-clearpolicy\x00" as *const u8 as
                             *const libc::c_char, *argv.offset(i as isize)) ==
                      0 {
            (*oprinc).policy = 0 as *mut libc::c_char;
            *mask |= 0x1000 as libc::c_int as libc::c_long
        } else if strcmp(b"-pw\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            *pass = *argv.offset(i as isize)
        } else if strcmp(b"-randkey\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            *randkey = 1 as libc::c_int as krb5_boolean
        } else if strcmp(b"-nokey\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            *nokey = 1 as libc::c_int as krb5_boolean
        } else if strcmp(b"-unlock\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            unlock_princ(oprinc, mask, caller);
        } else if strcmp(b"-e\x00" as *const u8 as *const libc::c_char,
                         *argv.offset(i as isize)) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            retval =
                krb5_string_to_keysalts(*argv.offset(i as isize),
                                        0 as *const libc::c_char,
                                        0 as *const libc::c_char,
                                        0 as libc::c_int as krb5_boolean,
                                        ks_tuple, n_ks_tuple);
            if retval != 0 {
                com_err(caller, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while parsing keysalts %s\x00" as *const u8
                                     as *const libc::c_char),
                        *argv.offset(i as isize));
                return -(1 as libc::c_int)
            }
        } else {
            retval =
                krb5_flagspec_to_mask(*argv.offset(i as isize),
                                      &mut (*oprinc).attributes,
                                      &mut (*oprinc).attributes);
            if retval != 0 {
                return -(1 as libc::c_int)
            } else { *mask |= 0x10 as libc::c_int as libc::c_long }
        }
        i += 1
    }
    if i != argc - 1 as libc::c_int { return -(1 as libc::c_int) }
    retval =
        kadmin_parse_name(*argv.offset(i as isize), &mut (*oprinc).principal);
    if retval != 0 {
        com_err(caller, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return -(1 as libc::c_int)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1132:1"]
unsafe extern "C" fn kadmin_addprinc_usage() {
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"usage: add_principal [options] principal\n\x00" as
                       *const u8 as *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\toptions are:\n\x00" as *const u8 as
                       *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\t\t[-randkey|-nokey] [-x db_princ_args]* [-expire expdate] [-pwexpire pwexpdate] [-maxlife maxtixlife]\n\t\t[-kvno kvno] [-policy policy] [-clearpolicy]\n\t\t[-pw password] [-maxrenewlife maxrenewlife]\n\t\t[-e keysaltlist]\n\t\t[{+|-}attribute]\n\x00"
                       as *const u8 as *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\tattributes are:\n\x00" as *const u8 as
                       *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\t\tallow_postdated allow_forwardable allow_tgs_req allow_renewable\n\t\tallow_proxiable allow_dup_skey allow_tix requires_preauth\n\t\trequires_hwauth needchange allow_svr password_changing_service\n\t\tok_as_delegate ok_to_auth_as_delegate no_auth_data_required\n\t\tlockdown_keys\n\nwhere,\n\t[-x db_princ_args]* - any number of database specific arguments.\n\t\t\tLook at each database documentation for supported arguments\n\x00"
                       as *const u8 as *const libc::c_char));
}
#[c2rust::src_loc = "1156:1"]
unsafe extern "C" fn kadmin_modprinc_usage() {
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"usage: modify_principal [options] principal\n\x00" as
                       *const u8 as *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\toptions are:\n\x00" as *const u8 as
                       *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\t\t[-x db_princ_args]* [-expire expdate] [-pwexpire pwexpdate] [-maxlife maxtixlife]\n\t\t[-kvno kvno] [-policy policy] [-clearpolicy]\n\t\t[-maxrenewlife maxrenewlife] [-unlock] [{+|-}attribute]\n\x00"
                       as *const u8 as *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\tattributes are:\n\x00" as *const u8 as
                       *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\t\tallow_postdated allow_forwardable allow_tgs_req allow_renewable\n\t\tallow_proxiable allow_dup_skey allow_tix requires_preauth\n\t\trequires_hwauth needchange allow_svr password_changing_service\n\t\tok_as_delegate ok_to_auth_as_delegate no_auth_data_required\n\t\tlockdown_keys\n\nwhere,\n\t[-x db_princ_args]* - any number of database specific arguments.\n\t\t\tLook at each database documentation for supported arguments\n\x00"
                       as *const u8 as *const libc::c_char));
}
/* Create a dummy password for old-style (pre-1.8) randkey creation. */
#[c2rust::src_loc = "1180:1"]
unsafe extern "C" fn prepare_dummy_password(mut buf: *mut libc::c_char,
                                            mut sz: size_t) {
    let mut i: size_t = 0;
    /* Must try to pass any password policy in place, and be valid UTF-8. */
    krb5int_strlcpy(buf, b"6F a[\x00" as *const u8 as *const libc::c_char,
                    sz);
    i = strlen(buf);
    while i < sz.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        *buf.offset(i as isize) =
            ('a' as i32 as
                 libc::c_ulong).wrapping_add(i.wrapping_rem(26 as libc::c_int
                                                                as
                                                                libc::c_ulong))
                as libc::c_char;
        i = i.wrapping_add(1)
    }
    *buf.offset(sz.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
        '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "1192:1"]
pub unsafe extern "C" fn kadmin_addprinc(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut princ: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut mask: libc::c_long = 0;
    let mut randkey: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut nokey: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut old_style_randkey: krb5_boolean =
        0 as libc::c_int as krb5_boolean;
    let mut n_ks_tuple: libc::c_int = 0;
    let mut ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: krb5_error_code = 0;
    let mut newpw: [libc::c_char; 1024] = [0; 1024];
    let mut dummybuf: [libc::c_char; 256] = [0; 256];
    static mut prompt1: [libc::c_char; 1024] = [0; 1024];
    static mut prompt2: [libc::c_char; 1024] = [0; 1024];
    /* Zero all fields in request structure */
    memset(&mut princ as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    princ.attributes = 0 as libc::c_int;
    if kadmin_parse_princ_args(argc, argv, &mut princ, &mut mask, &mut pass,
                               &mut randkey, &mut nokey, &mut ks_tuple,
                               &mut n_ks_tuple,
                               b"add_principal\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char)
           != 0 {
        kadmin_addprinc_usage();
    } else {
        retval =
            krb5_unparse_name(context,
                              princ.principal as krb5_const_principal,
                              &mut canon);
        if retval != 0 {
            com_err(b"add_principal\x00" as *const u8 as *const libc::c_char,
                    retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while canonicalizing principal\x00" as
                                 *const u8 as *const libc::c_char));
        } else {
            if mask & 0x800 as libc::c_int as libc::c_long != 0 {
                /* Warn if the specified policy does not exist. */
                if script_mode == 0 && policy_exists(princ.policy) == 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"WARNING: policy \"%s\" does not exist\n\x00"
                                         as *const u8 as *const libc::c_char),
                            princ.policy);
                }
            } else if mask & 0x1000 as libc::c_int as libc::c_long == 0 {
                /* If the policy "default" exists, assign it. */
                if policy_exists(b"default\x00" as *const u8 as
                                     *const libc::c_char) != 0 {
                    if script_mode == 0 {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"No policy specified for %s; assigning \"default\"\n\x00"
                                             as *const u8 as
                                             *const libc::c_char), canon);
                    }
                    princ.policy =
                        b"default\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char;
                    mask |= 0x800 as libc::c_int as libc::c_long
                } else if script_mode == 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"No policy specified for %s; defaulting to no policy\n\x00"
                                         as *const u8 as *const libc::c_char),
                            canon);
                }
            }
            /* Don't send KADM5_POLICY_CLR to the server. */
            mask &= !(0x1000 as libc::c_int) as libc::c_long;
            if nokey != 0 {
                pass = 0 as *mut libc::c_char;
                mask |= 0x20000 as libc::c_int as libc::c_long;
                current_block = 790185930182612747;
            } else if randkey != 0 {
                pass = 0 as *mut libc::c_char;
                current_block = 790185930182612747;
            } else if pass.is_null() {
                let mut sz: libc::c_uint =
                    (::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) as
                        libc::c_uint;
                snprintf(prompt1.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 1024]>() as
                             libc::c_ulong,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Enter password for principal \"%s\"\x00"
                                      as *const u8 as *const libc::c_char),
                         canon);
                snprintf(prompt2.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 1024]>() as
                             libc::c_ulong,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Re-enter password for principal \"%s\"\x00"
                                      as *const u8 as *const libc::c_char),
                         canon);
                retval =
                    krb5_read_password(context, prompt1.as_mut_ptr(),
                                       prompt2.as_mut_ptr(),
                                       newpw.as_mut_ptr(), &mut sz);
                if retval != 0 {
                    com_err(b"add_principal\x00" as *const u8 as
                                *const libc::c_char, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while reading password for \"%s\".\x00"
                                         as *const u8 as *const libc::c_char),
                            canon);
                    current_block = 13698151031428642715;
                } else {
                    pass = newpw.as_mut_ptr();
                    current_block = 790185930182612747;
                }
            } else { current_block = 790185930182612747; }
            match current_block {
                13698151031428642715 => { }
                _ => {
                    mask |= 0x1 as libc::c_int as libc::c_long;
                    retval =
                        create_princ(&mut princ, mask, n_ks_tuple, ks_tuple,
                                     pass);
                    if retval == 22 as libc::c_int && randkey != 0 {
                        /*
         * The server doesn't support randkey creation.  Create the principal
         * with a dummy password and disallow tickets.
         */
                        prepare_dummy_password(dummybuf.as_mut_ptr(),
                                               ::std::mem::size_of::<[libc::c_char; 256]>()
                                                   as libc::c_ulong);
                        princ.attributes |= 0x40 as libc::c_int;
                        mask |= 0x10 as libc::c_int as libc::c_long;
                        pass = dummybuf.as_mut_ptr();
                        retval =
                            create_princ(&mut princ, mask, n_ks_tuple,
                                         ks_tuple, pass);
                        old_style_randkey = 1 as libc::c_int as krb5_boolean
                    }
                    if retval as libc::c_long == 43787534 as libc::c_long &&
                           nokey != 0 {
                        error(dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"Admin server does not support -nokey while creating \"%s\"\n\x00"
                                           as *const u8 as
                                           *const libc::c_char), canon);
                    } else if retval != 0 {
                        com_err(b"add_principal\x00" as *const u8 as
                                    *const libc::c_char, retval as errcode_t,
                                b"while creating \"%s\".\x00" as *const u8 as
                                    *const libc::c_char, canon);
                    } else {
                        if old_style_randkey != 0 {
                            /* Randomize the password and re-enable tickets. */
                            retval =
                                randkey_princ(handle, princ.principal,
                                              0 as libc::c_int as
                                                  krb5_boolean, n_ks_tuple,
                                              ks_tuple,
                                              0 as *mut *mut krb5_keyblock,
                                              0 as
                                                  *mut libc::c_int); /* clear notix */
                            if retval != 0 {
                                com_err(b"add_principal\x00" as *const u8 as
                                            *const libc::c_char,
                                        retval as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while randomizing key for \"%s\".\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        canon);
                                current_block = 13698151031428642715;
                            } else {
                                princ.attributes &= !(0x40 as libc::c_int);
                                mask = 0x10 as libc::c_int as libc::c_long;
                                retval =
                                    kadm5_modify_principal(handle, &mut princ,
                                                           mask) as
                                        krb5_error_code;
                                if retval != 0 {
                                    com_err(b"add_principal\x00" as *const u8
                                                as *const libc::c_char,
                                            retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while clearing DISALLOW_ALL_TIX for \"%s\".\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            canon);
                                    current_block = 13698151031428642715;
                                } else {
                                    current_block = 15669289850109000831;
                                }
                            }
                        } else { current_block = 15669289850109000831; }
                        match current_block {
                            13698151031428642715 => { }
                            _ => {
                                info(b"Principal \"%s\" created.\n\x00" as
                                         *const u8 as *const libc::c_char,
                                     canon);
                            }
                        }
                    }
                }
            }
        }
    }
    krb5_free_principal(context, princ.principal);
    free(ks_tuple as *mut libc::c_void);
    free(canon as *mut libc::c_void);
    kadmin_free_tl_data(&mut princ.n_tl_data, &mut princ.tl_data);
}
#[no_mangle]
#[c2rust::src_loc = "1315:1"]
pub unsafe extern "C" fn kadmin_modprinc(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    let mut princ: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut oldprinc: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut kprinc: krb5_principal = 0 as krb5_principal;
    let mut mask: libc::c_long = 0;
    let mut retval: krb5_error_code = 0;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut randkey: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut nokey: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut n_ks_tuple: libc::c_int = 0 as libc::c_int;
    let mut ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    if argc < 2 as libc::c_int { kadmin_modprinc_usage(); return }
    memset(&mut oldprinc as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    memset(&mut princ as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    retval =
        kadmin_parse_name(*argv.offset((argc - 1 as libc::c_int) as isize),
                          &mut kprinc);
    if retval != 0 {
        com_err(b"modify_principal\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, kprinc as krb5_const_principal,
                          &mut canon);
    if retval != 0 {
        com_err(b"modify_principal\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        retval =
            kadm5_get_principal(handle, kprinc, &mut oldprinc,
                                0x41ffff as libc::c_int as libc::c_long) as
                krb5_error_code;
        if retval != 0 {
            com_err(b"modify_principal\x00" as *const u8 as
                        *const libc::c_char, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting \"%s\".\x00" as *const u8 as
                                 *const libc::c_char), canon);
        } else {
            princ.attributes = oldprinc.attributes;
            kadm5_free_principal_ent(handle, &mut oldprinc);
            retval =
                kadmin_parse_princ_args(argc, argv, &mut princ, &mut mask,
                                        &mut pass, &mut randkey, &mut nokey,
                                        &mut ks_tuple, &mut n_ks_tuple,
                                        b"modify_principal\x00" as *const u8
                                            as *const libc::c_char as
                                            *mut libc::c_char);
            if retval != 0 || !ks_tuple.is_null() || randkey != 0 ||
                   nokey != 0 || !pass.is_null() {
                kadmin_modprinc_usage();
            } else {
                if mask & 0x800 as libc::c_int as libc::c_long != 0 {
                    /* Warn if the specified policy does not exist. */
                    if script_mode == 0 && policy_exists(princ.policy) == 0 {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"WARNING: policy \"%s\" does not exist\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                princ.policy);
                    }
                }
                if mask != 0 {
                    /* Skip this if all we're doing is setting certhash. */
                    retval =
                        kadm5_modify_principal(handle, &mut princ, mask) as
                            krb5_error_code
                }
                if retval != 0 {
                    com_err(b"modify_principal\x00" as *const u8 as
                                *const libc::c_char, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while modifying \"%s\".\x00" as
                                         *const u8 as *const libc::c_char),
                            canon);
                } else {
                    info(dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"Principal \"%s\" modified.\n\x00" as
                                      *const u8 as *const libc::c_char),
                         canon);
                }
            }
        }
    }
    krb5_free_principal(context, kprinc);
    krb5_free_principal(context, princ.principal);
    kadmin_free_tl_data(&mut princ.n_tl_data, &mut princ.tl_data);
    free(canon as *mut libc::c_void);
    free(ks_tuple as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1388:1"]
pub unsafe extern "C" fn kadmin_getprinc(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    let mut dprinc: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut retval: krb5_error_code = 0;
    let mut polname: *const libc::c_char = 0 as *const libc::c_char;
    let mut noexist: *const libc::c_char = 0 as *const libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modprincstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut attrstrs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if !(argc == 2 as libc::c_int ||
             argc == 3 as libc::c_int &&
                 strcmp(b"-terse\x00" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize)) == 0) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: get_principal [-terse] principal\n\x00" as
                           *const u8 as *const libc::c_char));
        return
    }
    memset(&mut dprinc as *mut kadm5_principal_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_principal_ent_rec>() as libc::c_ulong);
    retval =
        kadmin_parse_name(*argv.offset((argc - 1 as libc::c_int) as isize),
                          &mut princ);
    if retval != 0 {
        com_err(b"get_principal\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut canon);
    if retval != 0 {
        com_err(b"get_principal\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        retval =
            kadm5_get_principal(handle, princ, &mut dprinc,
                                (0x41ffff as libc::c_int |
                                     0x20000 as libc::c_int) as libc::c_long)
                as krb5_error_code;
        if retval != 0 {
            com_err(b"get_principal\x00" as *const u8 as *const libc::c_char,
                    retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while retrieving \"%s\".\x00" as *const u8 as
                                 *const libc::c_char), canon);
        } else {
            retval =
                krb5_unparse_name(context,
                                  dprinc.principal as krb5_const_principal,
                                  &mut princstr);
            if retval != 0 {
                com_err(b"get_principal\x00" as *const u8 as
                            *const libc::c_char, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while unparsing principal\x00" as *const u8
                                     as *const libc::c_char));
            } else {
                retval =
                    krb5_unparse_name(context,
                                      dprinc.mod_name as krb5_const_principal,
                                      &mut modprincstr);
                if retval != 0 {
                    com_err(b"get_principal\x00" as *const u8 as
                                *const libc::c_char, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while unparsing principal\x00" as
                                         *const u8 as *const libc::c_char));
                } else if argc == 2 as libc::c_int {
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Principal: %s\n\x00" as *const u8 as
                                        *const libc::c_char), princstr);
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Expiration date: %s\n\x00" as *const u8
                                        as *const libc::c_char),
                           if dprinc.princ_expire_time != 0 {
                               strdate(dprinc.princ_expire_time)
                           } else {
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"[never]\x00" as *const u8 as
                                            *const libc::c_char) as
                                   *const libc::c_char
                           });
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Last password change: %s\n\x00" as
                                        *const u8 as *const libc::c_char),
                           if dprinc.last_pwd_change != 0 {
                               strdate(dprinc.last_pwd_change)
                           } else {
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"[never]\x00" as *const u8 as
                                            *const libc::c_char) as
                                   *const libc::c_char
                           });
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Password expiration date: %s\n\x00" as
                                        *const u8 as *const libc::c_char),
                           if dprinc.pw_expiration != 0 {
                               strdate(dprinc.pw_expiration)
                           } else {
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"[never]\x00" as *const u8 as
                                            *const libc::c_char) as
                                   *const libc::c_char
                           });
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Maximum ticket life: %s\n\x00" as
                                        *const u8 as *const libc::c_char),
                           strdur(dprinc.max_life as time_t));
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Maximum renewable life: %s\n\x00" as
                                        *const u8 as *const libc::c_char),
                           strdur(dprinc.max_renewable_life as time_t));
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Last modified: %s (%s)\n\x00" as
                                        *const u8 as *const libc::c_char),
                           strdate(dprinc.mod_date), modprincstr);
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Last successful authentication: %s\n\x00"
                                        as *const u8 as *const libc::c_char),
                           if dprinc.last_success != 0 {
                               strdate(dprinc.last_success)
                           } else {
                               dgettext(b"mit-krb5\x00" as *const u8 as
                                            *const libc::c_char,
                                        b"[never]\x00" as *const u8 as
                                            *const libc::c_char) as
                                   *const libc::c_char
                           });
                    printf(b"Last failed authentication: %s\n\x00" as
                               *const u8 as *const libc::c_char,
                           if dprinc.last_failed != 0 {
                               strdate(dprinc.last_failed)
                           } else {
                               b"[never]\x00" as *const u8 as
                                   *const libc::c_char
                           });
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Failed password attempts: %d\n\x00" as
                                        *const u8 as *const libc::c_char),
                           dprinc.fail_auth_count);
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Number of keys: %d\n\x00" as *const u8
                                        as *const libc::c_char),
                           dprinc.n_key_data as libc::c_int);
                    i = 0 as libc::c_int;
                    while i < dprinc.n_key_data as libc::c_int {
                        let mut key_data: *mut krb5_key_data =
                            &mut *dprinc.key_data.offset(i as isize) as
                                *mut krb5_key_data;
                        let mut enctype: [libc::c_char; 8192] = [0; 8192];
                        let mut salttype: [libc::c_char; 8192] = [0; 8192];
                        let mut deprecated: *mut libc::c_char =
                            b"\x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char;
                        if krb5_enctype_to_name((*key_data).key_data_type[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                    as krb5_enctype,
                                                0 as libc::c_int as
                                                    krb5_boolean,
                                                enctype.as_mut_ptr(),
                                                ::std::mem::size_of::<[libc::c_char; 8192]>()
                                                    as libc::c_ulong) != 0 {
                            snprintf(enctype.as_mut_ptr(),
                                     ::std::mem::size_of::<[libc::c_char; 8192]>()
                                         as libc::c_ulong,
                                     dgettext(b"mit-krb5\x00" as *const u8 as
                                                  *const libc::c_char,
                                              b"<Encryption type 0x%x>\x00" as
                                                  *const u8 as
                                                  *const libc::c_char),
                                     (*key_data).key_data_type[0 as
                                                                   libc::c_int
                                                                   as usize]
                                         as libc::c_int);
                        }
                        if krb5_c_valid_enctype((*key_data).key_data_type[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize]
                                                    as krb5_enctype) == 0 {
                            deprecated =
                                b"UNSUPPORTED:\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char
                        } else if krb5int_c_deprecated_enctype((*key_data).key_data_type[0
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize]
                                                                   as
                                                                   krb5_enctype)
                                      != 0 {
                            deprecated =
                                b"DEPRECATED:\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char
                        }
                        printf(b"Key: vno %d, %s%s\x00" as *const u8 as
                                   *const libc::c_char,
                               (*key_data).key_data_kvno as libc::c_int,
                               deprecated, enctype.as_mut_ptr());
                        if (*key_data).key_data_ver as libc::c_int >
                               1 as libc::c_int &&
                               (*key_data).key_data_type[1 as libc::c_int as
                                                             usize] as
                                   libc::c_int != 0 as libc::c_int {
                            if krb5_salttype_to_string((*key_data).key_data_type[1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     usize]
                                                           as krb5_int32,
                                                       salttype.as_mut_ptr(),
                                                       ::std::mem::size_of::<[libc::c_char; 8192]>()
                                                           as libc::c_ulong)
                                   != 0 {
                                snprintf(salttype.as_mut_ptr(),
                                         ::std::mem::size_of::<[libc::c_char; 8192]>()
                                             as libc::c_ulong,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"<Salt type 0x%x>\x00" as
                                                      *const u8 as
                                                      *const libc::c_char),
                                         (*key_data).key_data_type[1 as
                                                                       libc::c_int
                                                                       as
                                                                       usize]
                                             as libc::c_int);
                            }
                            printf(b":%s\x00" as *const u8 as
                                       *const libc::c_char,
                                   salttype.as_mut_ptr());
                        }
                        printf(b"\n\x00" as *const u8 as *const libc::c_char);
                        i += 1
                    }
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"MKey: vno %d\n\x00" as *const u8 as
                                        *const libc::c_char), dprinc.mkvno);
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Attributes:\x00" as *const u8 as
                                        *const libc::c_char));
                    retval =
                        krb5_flags_to_strings(dprinc.attributes,
                                              &mut attrstrs);
                    if retval != 0 {
                        com_err(b"get_principal\x00" as *const u8 as
                                    *const libc::c_char, retval as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while printing flags\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                        return
                    }
                    sp = attrstrs;
                    while !sp.is_null() && !(*sp).is_null() {
                        printf(b" %s\x00" as *const u8 as *const libc::c_char,
                               *sp);
                        free(*sp as *mut libc::c_void);
                        sp = sp.offset(1)
                    }
                    free(attrstrs as *mut libc::c_void);
                    printf(b"\n\x00" as *const u8 as *const libc::c_char);
                    polname =
                        if !dprinc.policy.is_null() {
                            dprinc.policy
                        } else {
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"[none]\x00" as *const u8 as
                                         *const libc::c_char)
                        };
                    noexist =
                        if !dprinc.policy.is_null() &&
                               policy_exists(dprinc.policy) == 0 {
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b" [does not exist]\x00" as *const u8 as
                                         *const libc::c_char) as
                                *const libc::c_char
                        } else {
                            b"\x00" as *const u8 as *const libc::c_char
                        };
                    printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Policy: %s%s\n\x00" as *const u8 as
                                        *const libc::c_char), polname,
                           noexist);
                } else {
                    printf(b"\"%s\"\t%d\t%d\t%d\t%d\t\"%s\"\t%d\t%d\t%d\t%d\t\"%s\"\t%d\t%d\t%d\t%d\t%d\x00"
                               as *const u8 as *const libc::c_char, princstr,
                           dprinc.princ_expire_time, dprinc.last_pwd_change,
                           dprinc.pw_expiration, dprinc.max_life, modprincstr,
                           dprinc.mod_date, dprinc.attributes, dprinc.kvno,
                           dprinc.mkvno,
                           if !dprinc.policy.is_null() {
                               dprinc.policy as *const libc::c_char
                           } else {
                               b"[none]\x00" as *const u8 as
                                   *const libc::c_char
                           }, dprinc.max_renewable_life, dprinc.last_success,
                           dprinc.last_failed, dprinc.fail_auth_count,
                           dprinc.n_key_data as libc::c_int);
                    i = 0 as libc::c_int;
                    while i < dprinc.n_key_data as libc::c_int {
                        printf(b"\t%d\t%d\t%d\t%d\x00" as *const u8 as
                                   *const libc::c_char,
                               (*dprinc.key_data.offset(i as
                                                            isize)).key_data_ver
                                   as libc::c_int,
                               (*dprinc.key_data.offset(i as
                                                            isize)).key_data_kvno
                                   as libc::c_int,
                               (*dprinc.key_data.offset(i as
                                                            isize)).key_data_type[0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      usize]
                                   as libc::c_int,
                               (*dprinc.key_data.offset(i as
                                                            isize)).key_data_type[1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      usize]
                                   as libc::c_int);
                        i += 1
                    }
                    printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
            }
        }
    }
    krb5_free_principal(context, princ);
    kadm5_free_principal_ent(handle, &mut dprinc);
    free(canon as *mut libc::c_void);
    free(princstr as *mut libc::c_void);
    free(modprincstr as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1524:1"]
pub unsafe extern "C" fn kadmin_getprincs(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut expr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    expr = 0 as *mut libc::c_char;
    if !(argc == 1 as libc::c_int ||
             argc == 2 as libc::c_int &&
                 {
                     expr = *argv.offset(1 as libc::c_int as isize);
                     !expr.is_null()
                 }) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: get_principals [expression]\n\x00" as
                           *const u8 as *const libc::c_char));
        return
    }
    retval =
        kadm5_get_principals(handle, expr, &mut names, &mut count) as
            krb5_error_code;
    if retval != 0 {
        com_err(b"get_principals\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retrieving list.\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    i = 0 as libc::c_int;
    while i < count {
        printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
               *names.offset(i as isize));
        i += 1
    }
    kadm5_free_name_list(handle, names, count);
}
#[c2rust::src_loc = "1546:1"]
unsafe extern "C" fn kadmin_parse_policy_args(mut argc: libc::c_int,
                                              mut argv:
                                                  *mut *mut libc::c_char,
                                              mut policy: kadm5_policy_ent_t,
                                              mut mask: *mut libc::c_long,
                                              mut caller: *mut libc::c_char)
 -> libc::c_int {
    let mut retval: krb5_error_code = 0;
    let mut i: libc::c_int = 0;
    let mut now: time_t = 0;
    let mut interval: time_t = 0;
    time(&mut now);
    *mask = 0 as libc::c_int as libc::c_long;
    i = 1 as libc::c_int;
    while i < argc - 1 as libc::c_int {
        if strcmp(*argv.offset(i as isize),
                  b"-maxlife\x00" as *const u8 as *const libc::c_char) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            interval = parse_interval(*argv.offset(i as isize), now);
            if interval == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*policy).pw_max_life = interval;
            *mask |= 0x4000 as libc::c_int as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"-minlife\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            interval = parse_interval(*argv.offset(i as isize), now);
            if interval == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*policy).pw_min_life = interval;
            *mask |= 0x8000 as libc::c_int as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"-minlength\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            (*policy).pw_min_length =
                atoi(*argv.offset(i as isize)) as libc::c_long;
            *mask |= 0x10000 as libc::c_int as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"-minclasses\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            (*policy).pw_min_classes =
                atoi(*argv.offset(i as isize)) as libc::c_long;
            *mask |= 0x20000 as libc::c_int as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"-history\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            (*policy).pw_history_num =
                atoi(*argv.offset(i as isize)) as libc::c_long;
            *mask |= 0x40000 as libc::c_int as libc::c_long
        } else if strlen(*argv.offset(i as isize)) ==
                      11 as libc::c_int as libc::c_ulong &&
                      strcmp(*argv.offset(i as isize),
                             b"-maxfailure\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            (*policy).pw_max_fail =
                atoi(*argv.offset(i as isize)) as krb5_kvno;
            *mask |= 0x100000 as libc::c_int as libc::c_long
        } else if strlen(*argv.offset(i as isize)) ==
                      21 as libc::c_int as libc::c_ulong &&
                      strcmp(*argv.offset(i as isize),
                             b"-failurecountinterval\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            interval = parse_interval(*argv.offset(i as isize), now);
            if interval == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*policy).pw_failcnt_interval = interval as krb5_deltat;
            *mask |= 0x200000 as libc::c_int as libc::c_long
        } else if strlen(*argv.offset(i as isize)) ==
                      16 as libc::c_int as libc::c_ulong &&
                      strcmp(*argv.offset(i as isize),
                             b"-lockoutduration\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            interval = parse_interval(*argv.offset(i as isize), now);
            if interval == -(1 as libc::c_int) as time_t {
                return -(1 as libc::c_int)
            }
            (*policy).pw_lockout_duration = interval as krb5_deltat;
            *mask |= 0x400000 as libc::c_int as libc::c_long
        } else if strcmp(*argv.offset(i as isize),
                         b"-allowedkeysalts\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            let mut ks_tuple: *mut krb5_key_salt_tuple =
                0 as *mut krb5_key_salt_tuple;
            let mut n_ks_tuple: libc::c_int = 0 as libc::c_int;
            i += 1;
            if i > argc - 2 as libc::c_int { return -(1 as libc::c_int) }
            if strcmp(*argv.offset(i as isize),
                      b"-\x00" as *const u8 as *const libc::c_char) != 0 {
                retval =
                    krb5_string_to_keysalts(*argv.offset(i as isize),
                                            b",\x00" as *const u8 as
                                                *const libc::c_char,
                                            0 as *const libc::c_char,
                                            0 as libc::c_int as krb5_boolean,
                                            &mut ks_tuple, &mut n_ks_tuple);
                if retval != 0 {
                    com_err(caller, retval as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while parsing keysalts %s\x00" as
                                         *const u8 as *const libc::c_char),
                            *argv.offset(i as isize));
                    return -(1 as libc::c_int)
                }
                free(ks_tuple as *mut libc::c_void);
                (*policy).allowed_keysalts = *argv.offset(i as isize)
            }
            *mask |= 0x4000000 as libc::c_int as libc::c_long
        } else { return -(1 as libc::c_int) }
        i += 1
    }
    if i != argc - 1 as libc::c_int {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"%s: parser lost count!\n\x00" as *const u8 as
                           *const libc::c_char), caller);
        return -(1 as libc::c_int)
    } else { return 0 as libc::c_int };
}
#[c2rust::src_loc = "1649:1"]
unsafe extern "C" fn kadmin_addmodpol_usage(mut func: *mut libc::c_char) {
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"usage; %s [options] policy\n\x00" as *const u8 as
                       *const libc::c_char), func);
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\toptions are:\n\x00" as *const u8 as
                       *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\t\t[-maxlife time] [-minlife time] [-minlength length]\n\t\t[-minclasses number] [-history number]\n\t\t[-maxfailure number] [-failurecountinterval time]\n\t\t[-allowedkeysalts keysalts]\n\x00"
                       as *const u8 as *const libc::c_char));
    error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                   b"\t\t[-lockoutduration time]\n\x00" as *const u8 as
                       *const libc::c_char));
}
#[no_mangle]
#[c2rust::src_loc = "1661:1"]
pub unsafe extern "C" fn kadmin_addpol(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut mask: libc::c_long = 0;
    let mut policy: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    memset(&mut policy as *mut kadm5_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_policy_ent_rec>() as libc::c_ulong);
    if kadmin_parse_policy_args(argc, argv, &mut policy, &mut mask,
                                b"add_policy\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
           != 0 {
        kadmin_addmodpol_usage(b"add_policy\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char);
        return
    }
    policy.policy = *argv.offset((argc - 1 as libc::c_int) as isize);
    mask |= 0x800 as libc::c_int as libc::c_long;
    retval =
        kadm5_create_policy(handle, &mut policy, mask) as krb5_error_code;
    if retval != 0 {
        com_err(b"add_policy\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating policy \"%s\".\x00" as *const u8 as
                             *const libc::c_char), policy.policy);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1682:1"]
pub unsafe extern "C" fn kadmin_modpol(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut mask: libc::c_long = 0;
    let mut policy: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    memset(&mut policy as *mut kadm5_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_policy_ent_rec>() as libc::c_ulong);
    if kadmin_parse_policy_args(argc, argv, &mut policy, &mut mask,
                                b"modify_policy\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char)
           != 0 {
        kadmin_addmodpol_usage(b"modify_policy\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char);
        return
    }
    policy.policy = *argv.offset((argc - 1 as libc::c_int) as isize);
    retval =
        kadm5_modify_policy(handle, &mut policy, mask) as krb5_error_code;
    if retval != 0 {
        com_err(b"modify_policy\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while modifying policy \"%s\".\x00" as *const u8 as
                             *const libc::c_char), policy.policy);
    };
}
#[no_mangle]
#[c2rust::src_loc = "1703:1"]
pub unsafe extern "C" fn kadmin_delpol(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut reply: [libc::c_char; 5] = [0; 5];
    if !(argc == 2 as libc::c_int ||
             argc == 3 as libc::c_int &&
                 strcmp(b"-force\x00" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize)) == 0) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: delete_policy [-force] policy\n\x00" as
                           *const u8 as *const libc::c_char));
        return
    }
    if argc == 2 as libc::c_int && script_mode == 0 {
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Are you sure you want to delete the policy \"%s\"? (yes/no): \x00"
                            as *const u8 as *const libc::c_char),
               *argv.offset(1 as libc::c_int as isize));
        fgets(reply.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as
                  libc::c_int, stdin);
        if strcmp(b"yes\n\x00" as *const u8 as *const libc::c_char,
                  reply.as_mut_ptr()) != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Policy \"%s\" not deleted.\n\x00" as *const u8
                                 as *const libc::c_char),
                    *argv.offset(1 as libc::c_int as isize));
            return
        }
    }
    retval =
        kadm5_delete_policy(handle,
                            *argv.offset((argc - 1 as libc::c_int) as isize))
            as krb5_error_code;
    if retval != 0 {
        com_err(b"delete_policy:\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while deleting policy \"%s\"\x00" as *const u8 as
                             *const libc::c_char),
                *argv.offset((argc - 1 as libc::c_int) as isize));
    };
}
#[no_mangle]
#[c2rust::src_loc = "1729:1"]
pub unsafe extern "C" fn kadmin_getpol(mut argc: libc::c_int,
                                       mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut policy: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    if !(argc == 2 as libc::c_int ||
             argc == 3 as libc::c_int &&
                 strcmp(b"-terse\x00" as *const u8 as *const libc::c_char,
                        *argv.offset(1 as libc::c_int as isize)) == 0) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: get_policy [-terse] policy\n\x00" as *const u8
                           as *const libc::c_char));
        return
    }
    retval =
        kadm5_get_policy(handle,
                         *argv.offset((argc - 1 as libc::c_int) as isize),
                         &mut policy) as krb5_error_code;
    if retval != 0 {
        com_err(b"get_policy\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retrieving policy \"%s\".\x00" as *const u8
                             as *const libc::c_char),
                *argv.offset((argc - 1 as libc::c_int) as isize));
        return
    }
    if argc == 2 as libc::c_int {
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Policy: %s\n\x00" as *const u8 as
                            *const libc::c_char), policy.policy);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Maximum password life: %s\n\x00" as *const u8 as
                            *const libc::c_char), strdur(policy.pw_max_life));
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Minimum password life: %s\n\x00" as *const u8 as
                            *const libc::c_char), strdur(policy.pw_min_life));
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Minimum password length: %ld\n\x00" as *const u8 as
                            *const libc::c_char), policy.pw_min_length);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Minimum number of password character classes: %ld\n\x00"
                            as *const u8 as *const libc::c_char),
               policy.pw_min_classes);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Number of old keys kept: %ld\n\x00" as *const u8 as
                            *const libc::c_char), policy.pw_history_num);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Maximum password failures before lockout: %lu\n\x00"
                            as *const u8 as *const libc::c_char),
               policy.pw_max_fail as libc::c_ulong);
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Password failure count reset interval: %s\n\x00" as
                            *const u8 as *const libc::c_char),
               strdur(policy.pw_failcnt_interval as time_t));
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Password lockout duration: %s\n\x00" as *const u8 as
                            *const libc::c_char),
               strdur(policy.pw_lockout_duration as time_t));
        if !policy.allowed_keysalts.is_null() {
            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"Allowed key/salt types: %s\n\x00" as *const u8
                                as *const libc::c_char),
                   policy.allowed_keysalts);
        }
    } else {
        /* Output 0 where we used to output policy_refcnt. */
        printf(b"\"%s\"\t%ld\t%ld\t%ld\t%ld\t%ld\t0\t%lu\t%ld\t%ld\t%s\n\x00"
                   as *const u8 as *const libc::c_char, policy.policy,
               policy.pw_max_life, policy.pw_min_life, policy.pw_min_length,
               policy.pw_min_classes, policy.pw_history_num,
               policy.pw_max_fail as libc::c_ulong,
               policy.pw_failcnt_interval as libc::c_long,
               policy.pw_lockout_duration as libc::c_long,
               if policy.allowed_keysalts.is_null() {
                   b"-\x00" as *const u8 as *const libc::c_char
               } else { policy.allowed_keysalts as *const libc::c_char });
    }
    kadm5_free_policy_ent(handle, &mut policy);
}
#[no_mangle]
#[c2rust::src_loc = "1775:1"]
pub unsafe extern "C" fn kadmin_getpols(mut argc: libc::c_int,
                                        mut argv: *mut *mut libc::c_char) {
    let mut retval: krb5_error_code = 0;
    let mut expr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    expr = 0 as *mut libc::c_char;
    if !(argc == 1 as libc::c_int ||
             argc == 2 as libc::c_int &&
                 {
                     expr = *argv.offset(1 as libc::c_int as isize);
                     !expr.is_null()
                 }) {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: get_policies [expression]\n\x00" as *const u8
                           as *const libc::c_char));
        return
    }
    retval =
        kadm5_get_policies(handle, expr, &mut names, &mut count) as
            krb5_error_code;
    if retval != 0 {
        com_err(b"get_policies\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retrieving list.\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    i = 0 as libc::c_int;
    while i < count {
        printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
               *names.offset(i as isize));
        i += 1
    }
    kadm5_free_name_list(handle, names, count);
}
#[no_mangle]
#[c2rust::src_loc = "1797:1"]
pub unsafe extern "C" fn kadmin_getprivs(mut argc: libc::c_int,
                                         mut argv: *mut *mut libc::c_char) {
    static mut privs: [*mut libc::c_char; 4] =
        [b"INQUIRE\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"ADD\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         b"MODIFY\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         b"DELETE\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char];
    let mut retval: krb5_error_code = 0;
    let mut i: size_t = 0;
    let mut plist: libc::c_long = 0;
    if argc != 1 as libc::c_int {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: get_privs\n\x00" as *const u8 as
                           *const libc::c_char));
        return
    }
    retval = kadm5_get_privs(handle, &mut plist) as krb5_error_code;
    if retval != 0 {
        com_err(b"get_privs\x00" as *const u8 as *const libc::c_char,
                retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while retrieving privileges\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"current privileges:\x00" as *const u8 as
                        *const libc::c_char));
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[*mut libc::c_char; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                   as libc::c_ulong) {
        if plist & ((1 as libc::c_int) << i) as libc::c_long != 0 {
            printf(b" %s\x00" as *const u8 as *const libc::c_char,
                   privs[i as usize]);
        }
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
#[c2rust::src_loc = "1822:1"]
pub unsafe extern "C" fn kadmin_purgekeys(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char) {
    let mut retval: kadm5_ret_t = 0;
    let mut keepkvno: libc::c_int = -(1 as libc::c_int);
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    if argc == 4 as libc::c_int &&
           strcmp(*argv.offset(1 as libc::c_int as isize),
                  b"-keepkvno\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
        keepkvno = atoi(*argv.offset(2 as libc::c_int as isize));
        pname = *argv.offset(3 as libc::c_int as isize)
    } else if argc == 3 as libc::c_int &&
                  strcmp(*argv.offset(1 as libc::c_int as isize),
                         b"-all\x00" as *const u8 as *const libc::c_char) ==
                      0 as libc::c_int {
        keepkvno = 2147483647 as libc::c_int;
        pname = *argv.offset(2 as libc::c_int as isize)
    } else if argc == 2 as libc::c_int {
        pname = *argv.offset(1 as libc::c_int as isize)
    }
    if pname.is_null() {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: purgekeys [-all|-keepkvno oldest_kvno_to_keep] principal\n\x00"
                           as *const u8 as *const libc::c_char));
        return
    }
    retval = kadmin_parse_name(pname, &mut princ) as kadm5_ret_t;
    if retval != 0 {
        com_err(b"purgekeys\x00" as *const u8 as *const libc::c_char, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut canon)
            as kadm5_ret_t;
    if retval != 0 {
        com_err(b"purgekeys\x00" as *const u8 as *const libc::c_char, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        retval = kadm5_purgekeys(handle, princ, keepkvno);
        if retval != 0 {
            com_err(b"purgekeys\x00" as *const u8 as *const libc::c_char,
                    retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while purging keys for principal \"%s\"\x00" as
                                 *const u8 as *const libc::c_char), canon);
        } else if keepkvno == 2147483647 as libc::c_int {
            info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"All keys for principal \"%s\" removed.\n\x00" as
                              *const u8 as *const libc::c_char), canon);
        } else {
            info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Old keys for principal \"%s\" purged.\n\x00" as
                              *const u8 as *const libc::c_char), canon);
        }
    }
    krb5_free_principal(context, princ);
    free(canon as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1874:1"]
pub unsafe extern "C" fn kadmin_getstrings(mut argc: libc::c_int,
                                           mut argv: *mut *mut libc::c_char) {
    let mut retval: kadm5_ret_t = 0;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: krb5_principal = 0 as krb5_principal;
    let mut strings: *mut krb5_string_attr = 0 as *mut krb5_string_attr;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if argc != 2 as libc::c_int {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: get_strings principal\n\x00" as *const u8 as
                           *const libc::c_char));
        return
    }
    pname = *argv.offset(1 as libc::c_int as isize);
    retval = kadmin_parse_name(pname, &mut princ) as kadm5_ret_t;
    if retval != 0 {
        com_err(b"get_strings\x00" as *const u8 as *const libc::c_char,
                retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut canon)
            as kadm5_ret_t;
    if retval != 0 {
        com_err(b"get_strings\x00" as *const u8 as *const libc::c_char,
                retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        retval = kadm5_get_strings(handle, princ, &mut strings, &mut count);
        if retval != 0 {
            com_err(b"get_strings\x00" as *const u8 as *const libc::c_char,
                    retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while getting attributes for principal \"%s\"\x00"
                                 as *const u8 as *const libc::c_char), canon);
        } else {
            if count == 0 as libc::c_int {
                printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"(No string attributes.)\n\x00" as *const u8
                                    as *const libc::c_char));
            }
            i = 0 as libc::c_int;
            while i < count {
                printf(b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                       (*strings.offset(i as isize)).key,
                       (*strings.offset(i as isize)).value);
                i += 1
            }
            kadm5_free_strings(handle, strings, count);
        }
    }
    krb5_free_principal(context, princ);
    free(canon as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1920:1"]
pub unsafe extern "C" fn kadmin_setstring(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char) {
    let mut retval: kadm5_ret_t = 0;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: krb5_principal = 0 as krb5_principal;
    if argc != 4 as libc::c_int {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: set_string principal key value\n\x00" as
                           *const u8 as *const libc::c_char));
        return
    }
    pname = *argv.offset(1 as libc::c_int as isize);
    key = *argv.offset(2 as libc::c_int as isize);
    value = *argv.offset(3 as libc::c_int as isize);
    retval = kadmin_parse_name(pname, &mut princ) as kadm5_ret_t;
    if retval != 0 {
        com_err(b"set_string\x00" as *const u8 as *const libc::c_char, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut canon)
            as kadm5_ret_t;
    if retval != 0 {
        com_err(b"set_string\x00" as *const u8 as *const libc::c_char, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        retval = kadm5_set_string(handle, princ, key, value);
        if retval != 0 {
            com_err(b"set_string\x00" as *const u8 as *const libc::c_char,
                    retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while setting attribute on principal \"%s\"\x00"
                                 as *const u8 as *const libc::c_char), canon);
        } else {
            info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Attribute set for principal \"%s\".\n\x00" as
                              *const u8 as *const libc::c_char), canon);
        }
    }
    krb5_free_principal(context, princ);
    free(canon as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "1961:1"]
pub unsafe extern "C" fn kadmin_delstring(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char) {
    let mut retval: kadm5_ret_t = 0;
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut canon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: krb5_principal = 0 as krb5_principal;
    if argc != 3 as libc::c_int {
        error(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                       b"usage: del_string principal key\n\x00" as *const u8
                           as *const libc::c_char));
        return
    }
    pname = *argv.offset(1 as libc::c_int as isize);
    key = *argv.offset(2 as libc::c_int as isize);
    retval = kadmin_parse_name(pname, &mut princ) as kadm5_ret_t;
    if retval != 0 {
        com_err(b"delstring\x00" as *const u8 as *const libc::c_char, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while parsing principal\x00" as *const u8 as
                             *const libc::c_char));
        return
    }
    retval =
        krb5_unparse_name(context, princ as krb5_const_principal, &mut canon)
            as kadm5_ret_t;
    if retval != 0 {
        com_err(b"del_string\x00" as *const u8 as *const libc::c_char, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal\x00" as *const u8 as
                             *const libc::c_char));
    } else {
        retval =
            kadm5_set_string(handle, princ, key, 0 as *const libc::c_char);
        if retval != 0 {
            com_err(b"del_string\x00" as *const u8 as *const libc::c_char,
                    retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while deleting attribute from principal \"%s\"\x00"
                                 as *const u8 as *const libc::c_char), canon);
        } else {
            info(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                          b"Attribute removed from principal \"%s\".\n\x00" as
                              *const u8 as *const libc::c_char), canon);
        }
    }
    krb5_free_principal(context, princ);
    free(canon as *mut libc::c_void);
}
