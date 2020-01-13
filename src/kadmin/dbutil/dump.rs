use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/include/bits/types.h:31"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
}
#[c2rust::header_src = "/usr/include/sys/types.h:31"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __ssize_t, __caddr_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:31"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:31"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:31"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:31"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:31"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:31"]
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
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
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
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3664:1"]
        pub fn krb5_principal_compare(context: krb5_context,
                                      princ1: krb5_const_principal,
                                      princ2: krb5_const_principal)
         -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:31"]
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
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
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
        #[c2rust::src_loc = "614:1"]
        pub fn krb5_lock_file(_: krb5_context, _: libc::c_int, _: libc::c_int)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:36"]
pub mod kdb_log_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident        "@(#)kdb_log.h  1.3     04/02/23 SMI" */
    /*
 * DB macros
 */
    /*
 * Current DB version #
 */
    /*
 * DB log states
 */
    /*
 * DB log constants
 */
    /*
 * Default ulog file attributes
 */
    /* in seconds */
    /*
 * Max size of update entry + update header
 * We make this large since resizing can be costly.
 */
    /* Default size of principal record */
    /* 256 MB log file */
    /*
 * Prototype declarations
 */
    /* Log header magic # */
    /* Kerberos database version no. */
    /* # of updates in log */
    /* Timestamp of first update */
    /* Timestamp of last update */
    /* First serial # in the update log */
    /* Last serial # in the update log */
    /* State of update log */
    /* Block size of each element */
    /* Update entry magic # */
    /* Serial # of entry */
    /* Timestamp of update */
    /* Is the entry committed or not */
    /* Size of update entry */
    /* Address of kdb_incr_update_t */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    #[c2rust::src_loc = "102:1"]
    pub type kdb_log_context = _kdb_log_context;
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t, kdb_last_t, update_status_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn ulog_init_header(context: krb5_context) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ulog_map(context: krb5_context, logname: *const libc::c_char,
                        entries: uint32_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn ulog_get_sno_status(context: krb5_context,
                                   last: *const kdb_last_t)
         -> update_status_t;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn ulog_get_last(context: krb5_context, last_out: *mut kdb_last_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "78:1"]
        pub fn ulog_set_last(context: krb5_context, last: *const kdb_last_t)
         -> krb5_error_code;
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:36"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    #[c2rust::src_loc = "166:1"]
    pub type update_status_t = libc::c_uint;
    #[c2rust::src_loc = "172:2"]
    pub const UPDATE_PERM_DENIED: update_status_t = 5;
    #[c2rust::src_loc = "171:2"]
    pub const UPDATE_NIL: update_status_t = 4;
    #[c2rust::src_loc = "170:2"]
    pub const UPDATE_BUSY: update_status_t = 3;
    #[c2rust::src_loc = "169:2"]
    pub const UPDATE_FULL_RESYNC_NEEDED: update_status_t = 2;
    #[c2rust::src_loc = "168:2"]
    pub const UPDATE_ERROR: update_status_t = 1;
    #[c2rust::src_loc = "167:2"]
    pub const UPDATE_OK: update_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct kdb_last_t {
        pub last_sno: kdb_sno_t,
        pub last_time: kdbe_time_t,
    }
    use super::stdint_uintn_h::uint32_t;
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:36"]
pub mod iprop_hdr_h {
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:31"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:31"]
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
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:32"]
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
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_rec = _osa_policy_ent_t;
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[c2rust::src_loc = "237:1"]
    pub type osa_adb_iter_policy_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: osa_policy_ent_t)
                   -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_magic, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_kvno, krb5_principal,
                        krb5_enctype, krb5_int32, krb5_context, krb5_pointer,
                        krb5_error_code, krb5_keyblock, krb5_principal_data,
                        krb5_boolean};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn krb5_db_iterate(kcontext: krb5_context,
                               match_entry: *mut libc::c_char,
                               func:
                                   Option<unsafe extern "C" fn(_:
                                                                   krb5_pointer,
                                                               _:
                                                                   *mut krb5_db_entry)
                                              -> libc::c_int>,
                               func_arg: krb5_pointer, iterflags: krb5_flags)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn krb5_db_put_principal(kcontext: krb5_context,
                                     entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "369:1"]
        pub fn krb5_db_unlock(kcontext: krb5_context) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "368:1"]
        pub fn krb5_db_lock(kcontext: krb5_context, lock_mode: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn krb5_db_promote(kcontext: krb5_context,
                               db_args: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "365:1"]
        pub fn krb5_db_destroy(kcontext: krb5_context,
                               db_args: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn krb5_db_create(kcontext: krb5_context,
                              db_args: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "358:1"]
        pub fn krb5_db_open(kcontext: krb5_context,
                            db_args: *mut *mut libc::c_char,
                            mode: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn krb5_dbe_encrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         dbkey: *const krb5_keyblock,
                                         keysalt: *const krb5_keysalt,
                                         keyver: libc::c_int,
                                         key_data: *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "404:1"]
        pub fn krb5_db_fetch_mkey(context: krb5_context,
                                  mname: krb5_principal, etype: krb5_enctype,
                                  fromkeyboard: krb5_boolean,
                                  twice: krb5_boolean,
                                  db_args: *mut libc::c_char,
                                  kvno: *mut krb5_kvno, salt: *mut krb5_data,
                                  key: *mut krb5_keyblock) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "413:1"]
        pub fn krb5_db_fetch_mkey_list(context: krb5_context,
                                       mname: krb5_principal,
                                       mkey: *const krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "472:1"]
        pub fn krb5_dbe_find_mkey(context: krb5_context,
                                  entry: *mut krb5_db_entry,
                                  mkey: *mut *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn krb5_dbe_update_mkvno(context: krb5_context,
                                     entry: *mut krb5_db_entry,
                                     mkvno: krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "446:1"]
        pub fn krb5_dbe_decrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         key_data: *const krb5_key_data,
                                         dbkey: *mut krb5_keyblock,
                                         keysalt: *mut krb5_keysalt)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "827:1"]
        pub fn krb5_db_iter_policy(kcontext: krb5_context,
                                   match_entry: *mut libc::c_char,
                                   func: osa_adb_iter_policy_func,
                                   data: *mut libc::c_void)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "823:1"]
        pub fn krb5_db_put_policy(kcontext: krb5_context,
                                  policy: osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "814:1"]
        pub fn krb5_db_create_policy(kcontext: krb5_context,
                                     policy: osa_policy_ent_t)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:32"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:32"]
pub mod xdr_h {
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
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
    extern "C" {
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:32"]
pub mod admin_h {
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
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:33"]
pub mod server_internal_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    /*
 * This header file is used internally by the Admin API server
 * libraries and Admin server.  IF YOU THINK YOU NEED TO USE THIS FILE
 * FOR ANYTHING, YOU'RE ALMOST CERTAINLY WRONG.
 */
    /*
 * This is the history key version for a newly created DB.  We use this value
 * for principals which have no password history yet to avoid having to look up
 * the history key.  Values other than 2 will cause compatibility issues with
 * pre-1.8 libkadm5 code; the older code will reject key changes when it sees
 * an unexpected value of admin_history_kvno.
 */
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:16"]
    pub struct _osa_pw_hist_t {
        pub n_key_data: libc::c_int,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "57:1"]
    pub type osa_pw_hist_ent = _osa_pw_hist_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:16"]
    pub struct _osa_princ_ent_t {
        pub version: libc::c_int,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub old_key_len: libc::c_uint,
        pub old_key_next: libc::c_uint,
        pub admin_history_kvno: krb5_kvno,
        pub old_keys: *mut osa_pw_hist_ent,
    }
    #[c2rust::src_loc = "62:1"]
    pub type osa_princ_ent_rec = _osa_princ_ent_t;
    #[c2rust::src_loc = "62:1"]
    pub type osa_princ_ent_t = *mut _osa_princ_ent_t;
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal, krb5_kvno,
                        krb5_error_code};
    use super::admin_h::kadm5_config_params;
    use super::kdb_h::{krb5_key_data, krb5_db_entry};
    use super::xdr_h::XDR;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn kdb_free_entry(handle: kadm5_server_handle_t,
                              kdb: *mut krb5_db_entry,
                              adb: *mut osa_princ_ent_rec) -> krb5_error_code;
        /*
 * Why is this (or something similar) not defined *anywhere* in krb5?
 */
        /*
 * all the various mask bits or'd together
 */
        #[no_mangle]
        #[c2rust::src_loc = "160:1"]
        pub fn xdr_osa_princ_ent_rec(xdrs: *mut XDR, objp: osa_princ_ent_t)
         -> libc::c_int;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src = "/usr/include/regex.h:38"]
pub mod regex_h {
    #[c2rust::src_loc = "478:1"]
    pub type regex_t = re_pattern_buffer;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "413:8"]
    pub struct re_pattern_buffer {
        pub buffer: *mut re_dfa_t,
        pub allocated: __re_long_size_t,
        pub used: __re_long_size_t,
        pub syntax: reg_syntax_t,
        pub fastmap: *mut libc::c_char,
        pub translate: *mut libc::c_uchar,
        pub re_nsub: size_t,
        #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
        #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits =
                   "1..=2")]
        #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits =
                   "3..=3")]
        #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
        #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
        #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
        #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits =
                   "7..=7")]
        pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }
    #[c2rust::src_loc = "72:1"]
    pub type reg_syntax_t = libc::c_ulong;
    #[c2rust::src_loc = "56:1"]
    pub type __re_long_size_t = libc::c_ulong;
    #[c2rust::src_loc = "350:3"]
    pub const _REG_NOMATCH: C2RustUnnamed = 1;
    #[c2rust::src_loc = "490:1"]
    pub type regoff_t = libc::c_int;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "517:9"]
    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }
    #[c2rust::src_loc = "346:9"]
    pub type C2RustUnnamed = libc::c_int;
    #[c2rust::src_loc = "370:3"]
    pub const _REG_ERPAREN: C2RustUnnamed = 16;
    #[c2rust::src_loc = "369:3"]
    pub const _REG_ESIZE: C2RustUnnamed = 15;
    #[c2rust::src_loc = "368:3"]
    pub const _REG_EEND: C2RustUnnamed = 14;
    #[c2rust::src_loc = "365:3"]
    pub const _REG_BADRPT: C2RustUnnamed = 13;
    #[c2rust::src_loc = "364:3"]
    pub const _REG_ESPACE: C2RustUnnamed = 12;
    #[c2rust::src_loc = "363:3"]
    pub const _REG_ERANGE: C2RustUnnamed = 11;
    #[c2rust::src_loc = "362:3"]
    pub const _REG_BADBR: C2RustUnnamed = 10;
    #[c2rust::src_loc = "361:3"]
    pub const _REG_EBRACE: C2RustUnnamed = 9;
    #[c2rust::src_loc = "360:3"]
    pub const _REG_EPAREN: C2RustUnnamed = 8;
    #[c2rust::src_loc = "359:3"]
    pub const _REG_EBRACK: C2RustUnnamed = 7;
    #[c2rust::src_loc = "358:3"]
    pub const _REG_ESUBREG: C2RustUnnamed = 6;
    #[c2rust::src_loc = "357:3"]
    pub const _REG_EESCAPE: C2RustUnnamed = 5;
    #[c2rust::src_loc = "356:3"]
    pub const _REG_ECTYPE: C2RustUnnamed = 4;
    #[c2rust::src_loc = "355:3"]
    pub const _REG_ECOLLATE: C2RustUnnamed = 3;
    #[c2rust::src_loc = "354:3"]
    pub const _REG_BADPAT: C2RustUnnamed = 2;
    #[c2rust::src_loc = "349:3"]
    pub const _REG_NOERROR: C2RustUnnamed = 0;
    #[c2rust::src_loc = "348:3"]
    pub const _REG_ENOSYS: C2RustUnnamed = -1;
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "417:10"]
        pub type re_dfa_t;
        #[no_mangle]
        #[c2rust::src_loc = "639:1"]
        pub fn regcomp(__preg: *mut regex_t, __pattern: *const libc::c_char,
                       __cflags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "643:1"]
        pub fn regexec(__preg: *const regex_t, __String: *const libc::c_char,
                       __nmatch: size_t, __pmatch: *mut regmatch_t,
                       __eflags: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "648:1"]
        pub fn regerror(__errcode: libc::c_int, __preg: *const regex_t,
                        __errbuf: *mut libc::c_char, __errbuf_size: size_t)
         -> size_t;
        #[no_mangle]
        #[c2rust::src_loc = "651:1"]
        pub fn regfree(__preg: *mut regex_t);
    }
}
#[c2rust::header_src = "/usr/include/assert.h:31"]
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
#[c2rust::header_src = "/usr/include/string.h:31"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:31"]
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
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "688:1"]
        pub fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:31"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "137:14"]
        pub static mut stdin: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "279:1"]
        pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "391:12"]
        pub fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "399:12"]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "485:1"]
        pub fn fgetc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "486:1"]
        pub fn getc(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "521:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "522:1"]
        pub fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:31"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:31"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:31"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:31"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/dbutil/kdb5_util.h:36"]
pub mod kdb5_util_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_kvno, krb5_keyblock,
                        krb5_error_code, krb5_enctype, krb5_deltat,
                        krb5_timestamp, krb5_flags, krb5_int32, krb5_boolean,
                        krb5_principal_data, krb5_principal, krb5_magic,
                        krb5_octet};
    use super::kdb_h::{krb5_db_entry, krb5_key_salt_tuple};
    use super::admin_h::kadm5_config_params;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "47:1"]
        pub fn add_db_arg(arg: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn usage();
        #[no_mangle]
        #[c2rust::src_loc = "97:1"]
        pub fn get_next_kvno(_: krb5_context, _: *mut krb5_db_entry)
         -> krb5_kvno;
        #[no_mangle]
        #[c2rust::src_loc = "94:1"]
        pub fn add_new_mkey(_: krb5_context, _: *mut krb5_db_entry,
                            _: *mut krb5_keyblock, _: krb5_kvno)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "41:12"]
        pub static mut valid_master_key: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "40:28"]
        pub static mut global_params: kadm5_config_params;
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut util_context: krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "38:12"]
        pub static mut exit_status: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "37:21"]
        pub static mut dbactive: krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "35:23"]
        pub static mut master_princ: krb5_principal;
        #[no_mangle]
        #[c2rust::src_loc = "34:22"]
        pub static mut master_keyblock: krb5_keyblock;
        #[no_mangle]
        #[c2rust::src_loc = "32:14"]
        pub static mut progname: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "43:15"]
        pub static mut db5util_db_args: *mut *mut libc::c_char;
    }
}
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t, __ssize_t, __caddr_t};
pub use self::sys_types_h::{u_int, ssize_t, caddr_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t, krb5_parse_name,
                       krb5_unparse_name, krb5_principal_compare,
                       krb5_free_keyblock_contents};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_lock_file};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog,
                          kdb_log_context, ulog_init_header, ulog_map,
                          ulog_get_sno_status, ulog_get_last, ulog_set_last};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t, update_status_t,
                        UPDATE_PERM_DENIED, UPDATE_NIL, UPDATE_BUSY,
                        UPDATE_FULL_RESYNC_NEEDED, UPDATE_ERROR, UPDATE_OK,
                        kdb_last_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data, _krb5_keysalt, krb5_keysalt,
                      _krb5_db_entry_new, krb5_db_entry, _osa_policy_ent_t,
                      osa_policy_ent_rec, osa_policy_ent_t,
                      osa_adb_iter_policy_func, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple, krb5_db_iterate,
                      krb5_db_put_principal, krb5_db_free_principal,
                      krb5_db_unlock, krb5_db_lock, krb5_db_promote,
                      krb5_db_destroy, krb5_db_create, krb5_db_open,
                      krb5_dbe_encrypt_key_data, krb5_db_fetch_mkey,
                      krb5_db_fetch_mkey_list, krb5_dbe_find_mkey,
                      krb5_dbe_update_mkvno, krb5_dbe_decrypt_key_data,
                      krb5_db_iter_policy, krb5_db_put_policy,
                      krb5_db_create_policy};
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, XDR, xdr_ops,
                      gssrpc_xdrmem_create};
pub use self::admin_h::{_kadm5_config_params, kadm5_config_params};
pub use self::server_internal_h::{pwqual_handle, kadm5_hook_handle,
                                  _kadm5_server_handle_t,
                                  kadm5_server_handle_t, _osa_pw_hist_t,
                                  osa_pw_hist_ent, _osa_princ_ent_t,
                                  osa_princ_ent_rec, osa_princ_ent_t,
                                  pwqual_handle_st, kadm5_hook_handle_st,
                                  kdb_free_entry, xdr_osa_princ_ent_rec};
pub use self::regex_h::{regex_t, re_pattern_buffer, reg_syntax_t,
                        __re_long_size_t, _REG_NOMATCH, regoff_t, regmatch_t,
                        C2RustUnnamed, _REG_ERPAREN, _REG_ESIZE, _REG_EEND,
                        _REG_BADRPT, _REG_ESPACE, _REG_ERANGE, _REG_BADBR,
                        _REG_EBRACE, _REG_EPAREN, _REG_EBRACK, _REG_ESUBREG,
                        _REG_EESCAPE, _REG_ECTYPE, _REG_ECOLLATE, _REG_BADPAT,
                        _REG_NOERROR, _REG_ENOSYS, re_dfa_t, regcomp, regexec,
                        regerror, regfree};
use self::assert_h::__assert_fail;
use self::string_h::{memset, strcmp, strncmp, strlen};
use self::stdlib_h::{atoi, malloc, calloc, free, exit, mkstemp};
use self::stdio_h::{stdin, stdout, stderr, rename, fclose, fopen, fdopen,
                    fprintf, printf, asprintf, fscanf, sscanf, fgetc, getc,
                    fputc, putc, fgets};
use self::fcntl_h::open;
use self::errno_h::__errno_location;
use self::unistd_h::{close, write, unlink};
use self::libintl_h::dgettext;
use self::kdb5_util_h::{add_db_arg, usage, get_next_kvno, add_new_mkey,
                        valid_master_key, global_params, util_context,
                        exit_status, dbactive, master_princ, master_keyblock,
                        progname, db5util_db_args};
extern "C" {
    /* External data */
    #[no_mangle]
    #[c2rust::src_loc = "91:23"]
    pub static mut master_entry: *mut krb5_db_entry;
}
#[c2rust::src_loc = "69:1"]
pub type dump_version = _dump_version;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "69:16"]
pub struct _dump_version {
    pub name: *mut libc::c_char,
    pub header: *mut libc::c_char,
    pub updateonly: libc::c_int,
    pub iprop: libc::c_int,
    pub ipropx: libc::c_int,
    pub dump_princ: dump_func,
    pub dump_policy: osa_adb_iter_policy_func,
    pub load_record: load_func,
}
#[c2rust::src_loc = "66:1"]
pub type load_func
    =
    Option<unsafe extern "C" fn(_: krb5_context, _: *const libc::c_char,
                                _: *mut FILE, _: krb5_boolean,
                                _: *mut libc::c_int) -> libc::c_int>;
/* Use compile(3) if no regcomp present. */
/* !HAVE_REGCOMP && HAVE_REGEXP_H */
#[c2rust::src_loc = "62:1"]
pub type dump_func
    =
    Option<unsafe extern "C" fn(_: krb5_context, _: *mut krb5_db_entry,
                                _: *const libc::c_char, _: *mut FILE,
                                _: krb5_boolean, _: krb5_boolean)
               -> krb5_error_code>;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "80:8"]
pub struct dump_args {
    pub ofile: *mut FILE,
    pub context: krb5_context,
    pub names: *mut *mut libc::c_char,
    pub nnames: libc::c_int,
    pub verbose: krb5_boolean,
    pub omit_nra: krb5_boolean,
    pub dump: *mut dump_version,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/dbutil/dump.c - Dump a KDC database */
/*
 * Copyright 1990,1991,2001,2006,2008,2009,2013 by the Massachusetts Institute
 * of Technology.  All Rights Reserved.
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
/* HAVE_REGEX_H */
/* Needed for master key conversion. */
#[c2rust::src_loc = "42:21"]
static mut mkey_convert: krb5_boolean = 0;
#[no_mangle]
#[c2rust::src_loc = "43:15"]
pub static mut new_master_keyblock: krb5_keyblock =
    krb5_keyblock{magic: 0,
                  enctype: 0,
                  length: 0,
                  contents: 0 as *const krb5_octet as *mut krb5_octet,};
#[no_mangle]
#[c2rust::src_loc = "44:11"]
pub static mut new_mkvno: krb5_kvno = 0;
/*
 * Re-encrypt the key_data with the new master key...
 */
#[no_mangle]
#[c2rust::src_loc = "96:1"]
pub unsafe extern "C" fn master_key_convert(mut context: krb5_context,
                                            mut db_entry: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 0;
    let mut v5plainkey: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *const krb5_octet as *mut krb5_octet,};
    let mut key_ptr: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut tmp_mkey: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut keysalt: krb5_keysalt =
        krb5_keysalt{type_0: 0,
                     data:
                         krb5_data{magic: 0,
                                   length: 0,
                                   data: 0 as *mut libc::c_char,},};
    let mut new_key_data: krb5_key_data =
        krb5_key_data{key_data_ver: 0,
                      key_data_kvno: 0,
                      key_data_type: [0; 2],
                      key_data_length: [0; 2],
                      key_data_contents: [0 as *mut krb5_octet; 2],};
    let mut key_data: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut is_mkey: krb5_boolean = 0;
    let mut kvno: krb5_kvno = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    is_mkey =
        krb5_principal_compare(context, master_princ as krb5_const_principal,
                               (*db_entry).princ as krb5_const_principal);
    if is_mkey != 0 {
        return add_new_mkey(context, db_entry, &mut new_master_keyblock,
                            new_mkvno)
    }
    i = 0 as libc::c_int;
    while i < (*db_entry).n_key_data as libc::c_int {
        key_data =
            &mut *(*db_entry).key_data.offset(i as isize) as
                *mut krb5_key_data;
        retval = krb5_dbe_find_mkey(context, db_entry, &mut tmp_mkey);
        if retval != 0 { return retval }
        retval =
            krb5_dbe_decrypt_key_data(context, tmp_mkey, key_data,
                                      &mut v5plainkey, &mut keysalt);
        if retval != 0 { return retval }
        memset(&mut new_key_data as *mut krb5_key_data as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<krb5_key_data>() as libc::c_ulong);
        key_ptr = &mut v5plainkey;
        kvno = (*key_data).key_data_kvno as krb5_kvno;
        retval =
            krb5_dbe_encrypt_key_data(context, &mut new_master_keyblock,
                                      key_ptr, &mut keysalt,
                                      kvno as libc::c_int, &mut new_key_data);
        if retval != 0 { return retval }
        krb5_free_keyblock_contents(context, &mut v5plainkey);
        j = 0 as libc::c_int;
        while j < (*key_data).key_data_ver as libc::c_int {
            if (*key_data).key_data_length[j as usize] != 0 {
                free((*key_data).key_data_contents[j as usize] as
                         *mut libc::c_void);
            }
            j += 1
        }
        *key_data = new_key_data;
        i += 1
    }
    if new_mkvno > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"new_mkvno > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"dump.c\x00" as *const u8 as *const libc::c_char,
                      141 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"krb5_error_code master_key_convert(krb5_context, krb5_db_entry *)\x00")).as_ptr());
    }
    return krb5_dbe_update_mkvno(context, db_entry, new_mkvno);
}
/* Create temp file for new dump to be named ofile. */
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn create_ofile(mut ofile: *mut libc::c_char,
                                  mut tmpname: *mut *mut libc::c_char)
 -> *mut FILE {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut f: *mut FILE = 0 as *mut FILE;
    *tmpname = 0 as *mut libc::c_char;
    if !(asprintf(tmpname,
                  b"%s-XXXXXX\x00" as *const u8 as *const libc::c_char, ofile)
             < 0 as libc::c_int) {
        fd = mkstemp(*tmpname);
        if !(fd == -(1 as libc::c_int)) {
            f = fdopen(fd, b"w+\x00" as *const u8 as *const libc::c_char);
            if !f.is_null() { return f }
        }
    }
    com_err(progname, *__errno_location() as errcode_t,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"while allocating temporary filename dump\x00" as
                         *const u8 as *const libc::c_char));
    if fd >= 0 as libc::c_int { unlink(*tmpname); }
    exit(1 as libc::c_int);
}
/* Rename new dump file into place. */
#[c2rust::src_loc = "172:1"]
unsafe extern "C" fn finish_ofile(mut ofile: *mut libc::c_char,
                                  mut tmpname: *mut *mut libc::c_char) {
    if rename(*tmpname, ofile) == -(1 as libc::c_int) {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while renaming dump file into place\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    free(*tmpname as *mut libc::c_void);
    *tmpname = 0 as *mut libc::c_char;
}
/* Create the .dump_ok file. */
#[c2rust::src_loc = "184:1"]
unsafe extern "C" fn prep_ok_file(mut context: krb5_context,
                                  mut file_name: *mut libc::c_char,
                                  mut fd_out: *mut libc::c_int)
 -> krb5_boolean {
    static mut ok: [libc::c_char; 9] =
        [46, 100, 117, 109, 112, 95, 111, 107, 0];
    let mut retval: krb5_error_code = 0;
    let mut file_ok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut success: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    *fd_out = -(1 as libc::c_int);
    if asprintf(&mut file_ok as *mut *mut libc::c_char,
                b"%s%s\x00" as *const u8 as *const libc::c_char, file_name,
                ok.as_mut_ptr()) < 0 as libc::c_int {
        com_err(progname, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while allocating dump_ok filename\x00" as *const u8
                             as *const libc::c_char));
    } else {
        fd =
            open(file_ok,
                 0o1 as libc::c_int | 0o100 as libc::c_int |
                     0o1000 as libc::c_int, 0o600 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while creating \'ok\' file, \'%s\'\x00" as
                                 *const u8 as *const libc::c_char), file_ok);
        } else {
            retval = krb5_lock_file(context, fd, 0x2 as libc::c_int);
            if retval != 0 {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while locking \'ok\' file, \'%s\'\x00" as
                                     *const u8 as *const libc::c_char),
                        file_ok);
            } else {
                *fd_out = fd;
                fd = -(1 as libc::c_int);
                success = 1 as libc::c_int as krb5_boolean
            }
        }
    }
    free(file_ok as *mut libc::c_void);
    if fd != -(1 as libc::c_int) { close(fd); }
    if success == 0 { exit_status += 1 }
    return success;
}
/*
 * Update the "ok" file.
 */
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn update_ok_file(mut context: krb5_context,
                                    mut fd: libc::c_int) {
    write(fd,
          b"\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
          1 as libc::c_int as size_t);
    krb5_lock_file(context, fd, 0x8 as libc::c_int);
    close(fd);
}
/* Return true if a principal name matches a regular expression or string. */
#[c2rust::src_loc = "236:1"]
unsafe extern "C" fn name_matches(mut name: *mut libc::c_char,
                                  mut args: *mut dump_args) -> libc::c_int {
    let mut reg: regex_t =
        regex_t{buffer: 0 as *mut re_dfa_t,
                allocated: 0,
                used: 0,
                syntax: 0,
                fastmap: 0 as *mut libc::c_char,
                translate: 0 as *mut libc::c_uchar,
                re_nsub: 0,
                can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor:
                    [0; 1],
                c2rust_padding: [0; 7],};
    let mut rmatch: regmatch_t = regmatch_t{rm_so: 0, rm_eo: 0,};
    let mut st: libc::c_int = 0;
    let mut errmsg: [libc::c_char; 8192] = [0; 8192];
    /* HAVE_RE_COMP */
    let mut i: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    /* Check each regular expression in args. */
    match_0 =
        if (*args).nnames != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
    i = 0 as libc::c_int;
    while i < (*args).nnames && match_0 == 0 {
        /* Compile the regular expression. */
        st =
            regcomp(&mut reg, *(*args).names.offset(i as isize),
                    1 as libc::c_int);
        if st != 0 {
            regerror(st, &mut reg, errmsg.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong);
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: regular expression error: %s\n\x00" as
                                 *const u8 as *const libc::c_char), progname,
                    errmsg.as_mut_ptr());
            break ;
        } else {
            /* HAVE_REGCOMP */
            /* See if we have a match. */
            st =
                regexec(&mut reg, name, 1 as libc::c_int as size_t,
                        &mut rmatch, 0 as libc::c_int);
            if st == 0 as libc::c_int {
                /* See if it matches the whole name. */
                if rmatch.rm_so == 0 as libc::c_int &&
                       rmatch.rm_eo as size_t == strlen(name) {
                    match_0 = 1 as libc::c_int
                }
            } else if st != _REG_NOMATCH as libc::c_int {
                regerror(st, &mut reg, errmsg.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong);
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"%s: regular expression match error: %s\n\x00"
                                     as *const u8 as *const libc::c_char),
                        progname, errmsg.as_mut_ptr());
                break ;
            }
            regfree(&mut reg);
            i += 1
        }
    }
    return match_0;
}
/* Output "-1" if len is 0; otherwise output len bytes of data in hex. */
#[c2rust::src_loc = "305:1"]
unsafe extern "C" fn dump_octets_or_minus1(mut fp: *mut FILE,
                                           mut data: *mut libc::c_uchar,
                                           mut len: size_t) {
    if len > 0 as libc::c_int as libc::c_ulong {
        while len > 0 as libc::c_int as libc::c_ulong {
            let fresh0 = data;
            data = data.offset(1);
            fprintf(fp, b"%02x\x00" as *const u8 as *const libc::c_char,
                    *fresh0 as libc::c_int);
            len = len.wrapping_sub(1)
        }
    } else { fprintf(fp, b"-1\x00" as *const u8 as *const libc::c_char); };
}
/*
 * Dump TL data; common to principals and policies.
 *
 * If filter_kadm then the KRB5_TL_KADM_DATA (where a principal's policy
 * name is stored) is filtered out.  This is for dump formats that don't
 * support policies.
 */
#[c2rust::src_loc = "323:1"]
unsafe extern "C" fn dump_tl_data(mut ofile: *mut FILE,
                                  mut tlp: *mut krb5_tl_data,
                                  mut filter_kadm: krb5_boolean) {
    while !tlp.is_null() {
        if !((*tlp).tl_data_type as libc::c_int == 0x3 as libc::c_int &&
                 filter_kadm != 0) {
            fprintf(ofile,
                    b"\t%d\t%d\t\x00" as *const u8 as *const libc::c_char,
                    (*tlp).tl_data_type as libc::c_int,
                    (*tlp).tl_data_length as libc::c_int);
            dump_octets_or_minus1(ofile, (*tlp).tl_data_contents,
                                  (*tlp).tl_data_length as size_t);
        }
        tlp = (*tlp).tl_data_next
    };
}
/* Dump a principal entry in krb5 beta 7 format.  Omit kadmin tl-data if kadm
 * is false. */
#[c2rust::src_loc = "338:1"]
unsafe extern "C" fn k5beta7_common(mut context: krb5_context,
                                    mut entry: *mut krb5_db_entry,
                                    mut name: *const libc::c_char,
                                    mut fp: *mut FILE,
                                    mut verbose: krb5_boolean,
                                    mut omit_nra: krb5_boolean,
                                    mut kadm: krb5_boolean)
 -> krb5_error_code {
    let mut tlp: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut kdata: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut counter: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /*
     * The dump format is as follows:
     *      len strlen(name) n_tl_data n_key_data e_length
     *      name
     *      attributes max_life max_renewable_life expiration
     *      pw_expiration last_success last_failed fail_auth_count
     *      n_tl_data*[type length <contents>]
     *      n_key_data*[ver kvno ver*(type length <contents>)]
     *      <e_data>
     * Fields which are not encapsulated by angle-brackets are to appear
     * verbatim.  A bracketed field's absence is indicated by a -1 in its
     * place.
     */
    /* Make sure that the tagged list is reasonably correct. */
    skip = 0 as libc::c_int;
    counter = skip;
    tlp = (*entry).tl_data;
    while !tlp.is_null() {
        /* Don't dump tl data types we know aren't understood by earlier
         * versions. */
        if (*tlp).tl_data_type as libc::c_int == 0x3 as libc::c_int &&
               kadm == 0 {
            skip += 1
        } else { counter += 1 }
        tlp = (*tlp).tl_data_next
    }
    if counter + skip != (*entry).n_tl_data as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: tagged data list inconsistency for %s (counted %d, stored %d)\n\x00"
                             as *const u8 as *const libc::c_char), progname,
                name, counter + skip, (*entry).n_tl_data as libc::c_int);
        return 22 as libc::c_int
    }
    /* Write out header. */
    fprintf(fp,
            b"princ\t%d\t%lu\t%d\t%d\t%d\t%s\t\x00" as *const u8 as
                *const libc::c_char, (*entry).len as libc::c_int,
            strlen(name), counter, (*entry).n_key_data as libc::c_int,
            (*entry).e_length as libc::c_int, name);
    fprintf(fp,
            b"%d\t%d\t%d\t%u\t%u\t%u\t%u\t%d\x00" as *const u8 as
                *const libc::c_char, (*entry).attributes, (*entry).max_life,
            (*entry).max_renewable_life, (*entry).expiration as libc::c_uint,
            (*entry).pw_expiration as libc::c_uint,
            if omit_nra != 0 {
                0 as libc::c_int
            } else { (*entry).last_success } as libc::c_uint,
            if omit_nra != 0 {
                0 as libc::c_int
            } else { (*entry).last_failed } as libc::c_uint,
            if omit_nra != 0 {
                0 as libc::c_int as libc::c_uint
            } else { (*entry).fail_auth_count });
    /* Write out tagged data. */
    dump_tl_data(fp, (*entry).tl_data,
                 (kadm == 0) as libc::c_int as krb5_boolean);
    fprintf(fp, b"\t\x00" as *const u8 as *const libc::c_char);
    /* Write out key data. */
    counter = 0 as libc::c_int;
    while counter < (*entry).n_key_data as libc::c_int {
        kdata =
            &mut *(*entry).key_data.offset(counter as isize) as
                *mut krb5_key_data;
        fprintf(fp, b"%d\t%d\t\x00" as *const u8 as *const libc::c_char,
                (*kdata).key_data_ver as libc::c_int,
                (*kdata).key_data_kvno as libc::c_int);
        i = 0 as libc::c_int;
        while i < (*kdata).key_data_ver as libc::c_int {
            fprintf(fp, b"%d\t%d\t\x00" as *const u8 as *const libc::c_char,
                    (*kdata).key_data_type[i as usize] as libc::c_int,
                    (*kdata).key_data_length[i as usize] as libc::c_int);
            dump_octets_or_minus1(fp, (*kdata).key_data_contents[i as usize],
                                  (*kdata).key_data_length[i as usize] as
                                      size_t);
            fprintf(fp, b"\t\x00" as *const u8 as *const libc::c_char);
            i += 1
        }
        counter += 1
    }
    /* Write out extra data. */
    dump_octets_or_minus1(fp, (*entry).e_data, (*entry).e_length as size_t);
    /* Write trailer. */
    fprintf(fp, b";\n\x00" as *const u8 as *const libc::c_char);
    if verbose != 0 {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                name);
    }
    return 0 as libc::c_int;
}
/* Output a dump record in krb5b7 format. */
#[c2rust::src_loc = "422:1"]
unsafe extern "C" fn dump_k5beta7_princ(mut context: krb5_context,
                                        mut entry: *mut krb5_db_entry,
                                        mut name: *const libc::c_char,
                                        mut fp: *mut FILE,
                                        mut verbose: krb5_boolean,
                                        mut omit_nra: krb5_boolean)
 -> krb5_error_code {
    return k5beta7_common(context, entry, name, fp, verbose, omit_nra,
                          0 as libc::c_int as krb5_boolean);
}
#[c2rust::src_loc = "430:1"]
unsafe extern "C" fn dump_k5beta7_princ_withpolicy(mut context: krb5_context,
                                                   mut entry:
                                                       *mut krb5_db_entry,
                                                   mut name:
                                                       *const libc::c_char,
                                                   mut fp: *mut FILE,
                                                   mut verbose: krb5_boolean,
                                                   mut omit_nra: krb5_boolean)
 -> krb5_error_code {
    return k5beta7_common(context, entry, name, fp, verbose, omit_nra,
                          1 as libc::c_int as krb5_boolean);
}
#[c2rust::src_loc = "438:1"]
unsafe extern "C" fn dump_k5beta7_policy(mut data: *mut libc::c_void,
                                         mut entry: osa_policy_ent_t) {
    let mut arg: *mut dump_args = data as *mut dump_args;
    fprintf((*arg).ofile,
            b"policy\t%s\t%d\t%d\t%d\t%d\t%d\t%d\n\x00" as *const u8 as
                *const libc::c_char, (*entry).name, (*entry).pw_min_life,
            (*entry).pw_max_life, (*entry).pw_min_length,
            (*entry).pw_min_classes, (*entry).pw_history_num,
            0 as libc::c_int);
}
#[c2rust::src_loc = "448:1"]
unsafe extern "C" fn dump_r1_8_policy(mut data: *mut libc::c_void,
                                      mut entry: osa_policy_ent_t) {
    let mut arg: *mut dump_args = data as *mut dump_args;
    fprintf((*arg).ofile,
            b"policy\t%s\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\n\x00" as
                *const u8 as *const libc::c_char, (*entry).name,
            (*entry).pw_min_life, (*entry).pw_max_life,
            (*entry).pw_min_length, (*entry).pw_min_classes,
            (*entry).pw_history_num, 0 as libc::c_int, (*entry).pw_max_fail,
            (*entry).pw_failcnt_interval, (*entry).pw_lockout_duration);
}
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn dump_r1_11_policy(mut data: *mut libc::c_void,
                                       mut entry: osa_policy_ent_t) {
    let mut arg: *mut dump_args = data as *mut dump_args;
    fprintf((*arg).ofile,
            b"policy\t%s\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%d\t%s\t%d\x00"
                as *const u8 as *const libc::c_char, (*entry).name,
            (*entry).pw_min_life, (*entry).pw_max_life,
            (*entry).pw_min_length, (*entry).pw_min_classes,
            (*entry).pw_history_num, 0 as libc::c_int, (*entry).pw_max_fail,
            (*entry).pw_failcnt_interval, (*entry).pw_lockout_duration,
            (*entry).attributes, (*entry).max_life,
            (*entry).max_renewable_life,
            if !(*entry).allowed_keysalts.is_null() {
                (*entry).allowed_keysalts as *const libc::c_char
            } else { b"-\x00" as *const u8 as *const libc::c_char },
            (*entry).n_tl_data as libc::c_int);
    dump_tl_data((*arg).ofile, (*entry).tl_data,
                 0 as libc::c_int as krb5_boolean);
    fprintf((*arg).ofile, b"\n\x00" as *const u8 as *const libc::c_char);
}
#[c2rust::src_loc = "478:1"]
unsafe extern "C" fn dump_iterator(mut ptr: *mut libc::c_void,
                                   mut entry: *mut krb5_db_entry)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut args: *mut dump_args = ptr as *mut dump_args;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    ret =
        krb5_unparse_name((*args).context,
                          (*entry).princ as krb5_const_principal, &mut name);
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unparsing principal name\x00" as *const u8 as
                             *const libc::c_char));
        return ret
    }
    /* Re-encode the keys in the new master key, if necessary. */
    if mkey_convert != 0 {
        ret = master_key_convert((*args).context, entry);
        if ret != 0 {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while converting %s to new master key\x00" as
                                 *const u8 as *const libc::c_char), name);
            current_block = 8375004491318202329;
        } else { current_block = 2868539653012386629; }
    } else { current_block = 2868539653012386629; }
    match current_block {
        2868539653012386629 =>
        /* Don't dump this entry if we have match strings and it doesn't match. */
        {
            if !((*args).nnames > 0 as libc::c_int &&
                     name_matches(name, args) == 0) {
                ret =
                    (*(*args).dump).dump_princ.expect("non-null function pointer")((*args).context,
                                                                                   entry,
                                                                                   name,
                                                                                   (*args).ofile,
                                                                                   (*args).verbose,
                                                                                   (*args).omit_nra)
            }
        }
        _ => { }
    }
    free(name as *mut libc::c_void);
    return ret;
}
#[inline]
#[c2rust::src_loc = "513:1"]
unsafe extern "C" fn load_err(mut fname: *const libc::c_char,
                              mut lineno: libc::c_int,
                              mut msg: *const libc::c_char) {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"%s(%d): %s\n\x00" as *const u8 as *const libc::c_char),
            fname, lineno, msg);
}
/* Read a string of bytes.  Increment *lp for each newline.  Return 0 on
 * success, 1 on failure. */
#[c2rust::src_loc = "521:1"]
unsafe extern "C" fn read_string(mut f: *mut FILE, mut buf: *mut libc::c_char,
                                 mut len: libc::c_int,
                                 mut lp: *mut libc::c_int) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        c = fgetc(f);
        if c < 0 as libc::c_int { return 1 as libc::c_int }
        if c == '\n' as i32 { *lp += 1 }
        *buf.offset(i as isize) = c as libc::c_char;
        i += 1
    }
    *buf.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
/* Read a string of two-character representations of bytes. */
#[c2rust::src_loc = "539:1"]
unsafe extern "C" fn read_octet_string(mut f: *mut FILE,
                                       mut buf: *mut libc::c_uchar,
                                       mut len: libc::c_int) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        if fscanf(f, b"%02x\x00" as *const u8 as *const libc::c_char,
                  &mut c as *mut libc::c_int) != 1 as libc::c_int {
            return 1 as libc::c_int
        }
        *buf.offset(i as isize) = c as libc::c_uchar;
        i += 1
    }
    return 0 as libc::c_int;
}
/* Read the end of a dumpfile record. */
#[c2rust::src_loc = "553:1"]
unsafe extern "C" fn read_record_end(mut f: *mut FILE,
                                     mut fn_0: *const libc::c_char,
                                     mut lineno: libc::c_int) {
    let mut ch: libc::c_int = 0;
    ch = fgetc(f);
    if ch != ';' as i32 || { ch = fgetc(f); (ch) != '\n' as i32 } {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s(%d): ignoring trash at end of line: \x00" as
                             *const u8 as *const libc::c_char), fn_0, lineno);
        while ch != '\n' as i32 { putc(ch, stderr); ch = fgetc(f) }
        putc(ch, stderr);
    };
}
/* Allocate and form a TL data list of a desired size. */
#[c2rust::src_loc = "570:1"]
unsafe extern "C" fn alloc_tl_data(mut n_tl_data: krb5_int16,
                                   mut tldp: *mut *mut krb5_tl_data)
 -> libc::c_int {
    let mut tlp: *mut *mut krb5_tl_data = tldp; /* caller cleans up */
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_tl_data as libc::c_int {
        *tlp =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong) as
                *mut krb5_tl_data;
        if (*tlp).is_null() { return 12 as libc::c_int }
        tlp = &mut (**tlp).tl_data_next;
        i += 1
    }
    return 0 as libc::c_int;
}
/* If len is zero, read the string "-1" from fp.  Otherwise allocate space and
 * read len octets.  Return 0 on success, 1 on failure. */
#[c2rust::src_loc = "588:1"]
unsafe extern "C" fn read_octets_or_minus1(mut fp: *mut FILE, mut len: size_t,
                                           mut out: *mut *mut libc::c_uchar)
 -> libc::c_int {
    let mut ival: libc::c_int = 0;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    *out = 0 as *mut libc::c_uchar;
    if len == 0 as libc::c_int as libc::c_ulong {
        return (fscanf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
                       &mut ival as *mut libc::c_int) != 1 as libc::c_int ||
                    ival != -(1 as libc::c_int)) as libc::c_int
    }
    buf = malloc(len) as *mut libc::c_uchar;
    if buf.is_null() { return 1 as libc::c_int }
    if read_octet_string(fp, buf, len as libc::c_int) != 0 {
        free(buf as *mut libc::c_void);
        return 1 as libc::c_int
    }
    *out = buf;
    return 0 as libc::c_int;
}
/* Read TL data for a principal or policy.  Print an error and return -1 on
 * failure. */
#[c2rust::src_loc = "610:1"]
unsafe extern "C" fn process_tl_data(mut fname: *const libc::c_char,
                                     mut filep: *mut FILE,
                                     mut lineno: libc::c_int,
                                     mut tl_data: *mut krb5_tl_data)
 -> libc::c_int {
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut nread: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut u1: libc::c_uint = 0;
    tl = tl_data;
    while !tl.is_null() {
        nread =
            fscanf(filep, b"%d\t%u\t\x00" as *const u8 as *const libc::c_char,
                   &mut i1 as *mut libc::c_int, &mut u1 as *mut libc::c_uint);
        if nread != 2 as libc::c_int {
            load_err(fname, lineno,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"cannot read tagged data type and length\x00"
                                  as *const u8 as *const libc::c_char));
            return 22 as libc::c_int
        }
        if i1 < -(32767 as libc::c_int) - 1 as libc::c_int ||
               i1 > 32767 as libc::c_int ||
               u1 > 65535 as libc::c_int as libc::c_uint {
            load_err(fname, lineno,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"data type or length overflowed\x00" as
                                  *const u8 as *const libc::c_char));
            return 22 as libc::c_int
        }
        (*tl).tl_data_type = i1 as krb5_int16;
        (*tl).tl_data_length = u1 as krb5_ui_2;
        if read_octets_or_minus1(filep, (*tl).tl_data_length as size_t,
                                 &mut (*tl).tl_data_contents) != 0 {
            load_err(fname, lineno,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"cannot read tagged data contents\x00" as
                                  *const u8 as *const libc::c_char));
            return 22 as libc::c_int
        }
        tl = (*tl).tl_data_next
    }
    return 0 as libc::c_int;
}
/* Read a beta 7 entry and add it to the database.  Return -1 for end of file,
 * 0 for success and 1 for failure. */
#[c2rust::src_loc = "643:1"]
unsafe extern "C" fn process_k5beta7_princ(mut context: krb5_context,
                                           mut fname: *const libc::c_char,
                                           mut filep: *mut FILE,
                                           mut verbose: krb5_boolean,
                                           mut linenop: *mut libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dbentry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut t3: libc::c_int = 0;
    let mut t4: libc::c_int = 0;
    let mut u1: libc::c_uint = 0;
    let mut u2: libc::c_uint = 0;
    let mut u3: libc::c_uint = 0;
    let mut u4: libc::c_uint = 0;
    let mut u5: libc::c_uint = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kp: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut kd: *mut krb5_key_data = 0 as *mut krb5_key_data;
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut ret: krb5_error_code = 0;
    dbentry =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_db_entry>() as libc::c_ulong) as
            *mut krb5_db_entry;
    if dbentry.is_null() { return 1 as libc::c_int }
    *linenop += 1;
    nread =
        fscanf(filep,
               b"%u\t%u\t%u\t%u\t%u\t\x00" as *const u8 as
                   *const libc::c_char, &mut u1 as *mut libc::c_uint,
               &mut u2 as *mut libc::c_uint, &mut u3 as *mut libc::c_uint,
               &mut u4 as *mut libc::c_uint, &mut u5 as *mut libc::c_uint);
    if nread == -(1 as libc::c_int) {
        retval = -(1 as libc::c_int)
    } else {
        if nread != 5 as libc::c_int {
            load_err(fname, *linenop,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"cannot match size tokens\x00" as *const u8 as
                                  *const libc::c_char));
            current_block = 6966502904230106405;
        } else {
            /* Get memory for flattened principal name */
            name =
                malloc(u2.wrapping_add(1 as libc::c_int as libc::c_uint) as
                           libc::c_ulong) as *mut libc::c_char;
            if name.is_null() {
                current_block = 6966502904230106405;
            } else if u3 > 65535 as libc::c_int as libc::c_uint {
                load_err(fname, *linenop,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"cannot allocate tl_data (too large)\x00"
                                      as *const u8 as *const libc::c_char));
                current_block = 6966502904230106405;
            } else if alloc_tl_data(u3 as krb5_int16, &mut (*dbentry).tl_data)
                          != 0 {
                current_block = 6966502904230106405;
            } else {
                (*dbentry).n_tl_data = u3 as krb5_int16;
                /* Get memory for and form tagged data linked list */
                /* Get memory for key list */
                if u4 != 0 &&
                       {
                           kp =
                               calloc(u4 as libc::c_ulong,
                                      ::std::mem::size_of::<krb5_key_data>()
                                          as libc::c_ulong) as
                                   *mut krb5_key_data;
                           kp.is_null()
                       } {
                    current_block = 6966502904230106405;
                } else {
                    (*dbentry).len = u1 as krb5_ui_2;
                    (*dbentry).n_key_data = u4 as krb5_int16;
                    (*dbentry).e_length = u5 as krb5_ui_2;
                    if !kp.is_null() {
                        (*dbentry).key_data = kp;
                        kp = 0 as *mut krb5_key_data
                    }
                    /* Read in and parse the principal name */
                    if read_string(filep, name, u2 as libc::c_int, linenop) !=
                           0 {
                        load_err(fname, *linenop,
                                 dgettext(b"mit-krb5\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"cannot read name string\x00" as
                                              *const u8 as
                                              *const libc::c_char));
                        current_block = 6966502904230106405;
                    } else {
                        ret =
                            krb5_parse_name(context, name,
                                            &mut (*dbentry).princ);
                        if ret != 0 {
                            com_err(progname, ret as errcode_t,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"while parsing name %s\x00" as
                                                 *const u8 as
                                                 *const libc::c_char), name);
                            current_block = 6966502904230106405;
                        } else {
                            /* Get the fixed principal attributes */
                            nread =
                                fscanf(filep,
                                       b"%d\t%d\t%d\t%u\t%u\t%d\t%d\t%d\t\x00"
                                           as *const u8 as
                                           *const libc::c_char,
                                       &mut t1 as *mut libc::c_int,
                                       &mut t2 as *mut libc::c_int,
                                       &mut t3 as *mut libc::c_int,
                                       &mut u1 as *mut libc::c_uint,
                                       &mut u2 as *mut libc::c_uint,
                                       &mut u3 as *mut libc::c_uint,
                                       &mut u4 as *mut libc::c_uint,
                                       &mut u5 as *mut libc::c_uint);
                            if nread != 8 as libc::c_int {
                                load_err(fname, *linenop,
                                         dgettext(b"mit-krb5\x00" as *const u8
                                                      as *const libc::c_char,
                                                  b"cannot read principal attributes\x00"
                                                      as *const u8 as
                                                      *const libc::c_char));
                                current_block = 6966502904230106405;
                            } else {
                                (*dbentry).attributes = t1;
                                (*dbentry).max_life = t2;
                                (*dbentry).max_renewable_life = t3;
                                (*dbentry).expiration = u1 as krb5_timestamp;
                                (*dbentry).pw_expiration =
                                    u2 as krb5_timestamp;
                                (*dbentry).last_success =
                                    u3 as krb5_timestamp;
                                (*dbentry).last_failed = u4 as krb5_timestamp;
                                (*dbentry).fail_auth_count = u5;
                                (*dbentry).mask =
                                    (0x200000 as libc::c_int |
                                         0x1 as libc::c_int |
                                         0x10 as libc::c_int |
                                         0x20 as libc::c_int |
                                         0x2000 as libc::c_int |
                                         0x2 as libc::c_int |
                                         0x4000 as libc::c_int |
                                         0x8000 as libc::c_int |
                                         0x10000 as libc::c_int) as krb5_ui_4;
                                /* Read tagged data. */
                                if (*dbentry).n_tl_data != 0 {
                                    if process_tl_data(fname, filep, *linenop,
                                                       (*dbentry).tl_data) !=
                                           0 {
                                        current_block = 6966502904230106405;
                                    } else {
                                        tl = (*dbentry).tl_data;
                                        while !tl.is_null() {
                                            /* test to set mask fields */
                                            if (*tl).tl_data_type as
                                                   libc::c_int ==
                                                   0x3 as libc::c_int {
                                                let mut xdrs: XDR =
                                                    XDR{x_op: XDR_ENCODE,
                                                        x_ops:
                                                            0 as *mut xdr_ops,
                                                        x_public:
                                                            0 as
                                                                *mut libc::c_char,
                                                        x_private:
                                                            0 as
                                                                *mut libc::c_void,
                                                        x_base:
                                                            0 as
                                                                *mut libc::c_char,
                                                        x_handy: 0,};
                                                let mut osa_princ_ent:
                                                        osa_princ_ent_rec =
                                                    osa_princ_ent_rec{version:
                                                                          0,
                                                                      policy:
                                                                          0 as
                                                                              *mut libc::c_char,
                                                                      aux_attributes:
                                                                          0,
                                                                      old_key_len:
                                                                          0,
                                                                      old_key_next:
                                                                          0,
                                                                      admin_history_kvno:
                                                                          0,
                                                                      old_keys:
                                                                          0 as
                                                                              *mut osa_pw_hist_ent,};
                                                /*
                 * Assuming aux_attributes will always be
                 * there
                 */
                                                (*dbentry).mask |=
                                                    0x400 as libc::c_int as
                                                        libc::c_uint;
                                                /* test for an actual policy reference */
                                                memset(&mut osa_princ_ent as
                                                           *mut osa_princ_ent_rec
                                                           as
                                                           *mut libc::c_void,
                                                       0 as libc::c_int,
                                                       ::std::mem::size_of::<osa_princ_ent_rec>()
                                                           as libc::c_ulong);
                                                gssrpc_xdrmem_create(&mut xdrs,
                                                                     (*tl).tl_data_contents
                                                                         as
                                                                         *mut libc::c_char,
                                                                     (*tl).tl_data_length
                                                                         as
                                                                         u_int,
                                                                     XDR_DECODE);
                                                if xdr_osa_princ_ent_rec(&mut xdrs,
                                                                         &mut osa_princ_ent)
                                                       != 0 {
                                                    if osa_princ_ent.aux_attributes
                                                           &
                                                           0x800 as
                                                               libc::c_int as
                                                               libc::c_long !=
                                                           0 &&
                                                           !osa_princ_ent.policy.is_null()
                                                       {
                                                        (*dbentry).mask |=
                                                            0x800 as
                                                                libc::c_int as
                                                                libc::c_uint
                                                    }
                                                    kdb_free_entry(0 as
                                                                       kadm5_server_handle_t,
                                                                   0 as
                                                                       *mut krb5_db_entry,
                                                                   &mut osa_princ_ent);
                                                }
                                                if (*xdrs.x_ops).x_destroy.is_some()
                                                   {
                                                    Some((*xdrs.x_ops).x_destroy.expect("non-null function pointer")).expect("non-null function pointer")(&mut xdrs);
                                                }
                                            }
                                            tl = (*tl).tl_data_next
                                        }
                                        (*dbentry).mask |=
                                            0x40000 as libc::c_int as
                                                libc::c_uint;
                                        current_block = 14001958660280927786;
                                    }
                                } else {
                                    current_block = 14001958660280927786;
                                }
                                match current_block {
                                    6966502904230106405 => { }
                                    _ =>
                                    /* Get the key data. */
                                    {
                                        i = 0 as libc::c_int;
                                        's_319:
                                            loop  {
                                                if !(i <
                                                         (*dbentry).n_key_data
                                                             as libc::c_int) {
                                                    current_block =
                                                        12027283704867122503;
                                                    break ;
                                                }
                                                kd =
                                                    &mut *(*dbentry).key_data.offset(i
                                                                                         as
                                                                                         isize)
                                                        as *mut krb5_key_data;
                                                nread =
                                                    fscanf(filep,
                                                           b"%d\t%d\t\x00" as
                                                               *const u8 as
                                                               *const libc::c_char,
                                                           &mut t1 as
                                                               *mut libc::c_int,
                                                           &mut t2 as
                                                               *mut libc::c_int);
                                                if nread != 2 as libc::c_int {
                                                    load_err(fname, *linenop,
                                                             dgettext(b"mit-krb5\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      b"cannot read key size and version\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char));
                                                    current_block =
                                                        6966502904230106405;
                                                    break ;
                                                } else if t1 >
                                                              2 as libc::c_int
                                                 {
                                                    load_err(fname, *linenop,
                                                             dgettext(b"mit-krb5\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      b"unsupported key_data_ver version\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char));
                                                    current_block =
                                                        6966502904230106405;
                                                    break ;
                                                } else {
                                                    (*kd).key_data_ver =
                                                        t1 as krb5_int16;
                                                    (*kd).key_data_kvno =
                                                        t2 as krb5_ui_2;
                                                    j = 0 as libc::c_int;
                                                    while j < t1 {
                                                        nread =
                                                            fscanf(filep,
                                                                   b"%d\t%d\t\x00"
                                                                       as
                                                                       *const u8
                                                                       as
                                                                       *const libc::c_char,
                                                                   &mut t3 as
                                                                       *mut libc::c_int,
                                                                   &mut t4 as
                                                                       *mut libc::c_int);
                                                        if nread !=
                                                               2 as
                                                                   libc::c_int
                                                               ||
                                                               t4 <
                                                                   0 as
                                                                       libc::c_int
                                                           {
                                                            load_err(fname,
                                                                     *linenop,
                                                                     dgettext(b"mit-krb5\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char,
                                                                              b"cannot read key type and length\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char));
                                                            current_block =
                                                                6966502904230106405;
                                                            break 's_319 ;
                                                        } else {
                                                            (*kd).key_data_type[j
                                                                                    as
                                                                                    usize]
                                                                =
                                                                t3 as
                                                                    krb5_int16;
                                                            (*kd).key_data_length[j
                                                                                      as
                                                                                      usize]
                                                                =
                                                                t4 as
                                                                    krb5_ui_2;
                                                            if read_octets_or_minus1(filep,
                                                                                     t4
                                                                                         as
                                                                                         size_t,
                                                                                     &mut *(*kd).key_data_contents.as_mut_ptr().offset(j
                                                                                                                                           as
                                                                                                                                           isize))
                                                                   != 0 {
                                                                load_err(fname,
                                                                         *linenop,
                                                                         dgettext(b"mit-krb5\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char,
                                                                                  b"cannot read key data\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char));
                                                                current_block
                                                                    =
                                                                    6966502904230106405;
                                                                break 's_319 ;
                                                            } else { j += 1 }
                                                        }
                                                    }
                                                    i += 1
                                                }
                                            }
                                        match current_block {
                                            6966502904230106405 => { }
                                            _ => {
                                                if (*dbentry).n_key_data != 0
                                                   {
                                                    (*dbentry).mask |=
                                                        0x20000 as libc::c_int
                                                            as libc::c_uint
                                                }
                                                /* Get the extra data */
                                                if read_octets_or_minus1(filep,
                                                                         (*dbentry).e_length
                                                                             as
                                                                             size_t,
                                                                         &mut (*dbentry).e_data)
                                                       != 0 {
                                                    load_err(fname, *linenop,
                                                             dgettext(b"mit-krb5\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char,
                                                                      b"cannot read extra data\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char));
                                                    current_block =
                                                        6966502904230106405;
                                                } else {
                                                    /* Finally, find the end of the record. */
                                                    read_record_end(filep,
                                                                    fname,
                                                                    *linenop);
                                                    ret =
                                                        krb5_db_put_principal(context,
                                                                              dbentry);
                                                    if ret != 0 {
                                                        com_err(progname,
                                                                ret as
                                                                    errcode_t,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"while storing %s\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char),
                                                                name);
                                                        current_block =
                                                            6966502904230106405;
                                                    } else {
                                                        if verbose != 0 {
                                                            fprintf(stderr,
                                                                    b"%s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    name);
                                                        }
                                                        retval =
                                                            0 as libc::c_int;
                                                        current_block =
                                                            3145223973222467422;
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
        match current_block {
            3145223973222467422 => { }
            _ => { retval = 1 as libc::c_int }
        }
    }
    free(kp as *mut libc::c_void);
    free(name as *mut libc::c_void);
    krb5_db_free_principal(context, dbentry);
    return retval;
}
#[c2rust::src_loc = "824:1"]
unsafe extern "C" fn process_k5beta7_policy(mut context: krb5_context,
                                            mut fname: *const libc::c_char,
                                            mut filep: *mut FILE,
                                            mut verbose: krb5_boolean,
                                            mut linenop: *mut libc::c_int)
 -> libc::c_int {
    let mut rec: osa_policy_ent_rec =
        osa_policy_ent_rec{version: 0,
                           name: 0 as *mut libc::c_char,
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
    let mut namebuf: [libc::c_char; 1024] = [0; 1024];
    let mut refcnt: libc::c_uint = 0;
    let mut nread: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    memset(&mut rec as *mut osa_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_policy_ent_rec>() as libc::c_ulong);
    *linenop += 1;
    rec.name = namebuf.as_mut_ptr();
    nread =
        fscanf(filep,
               b"%1023s\t%u\t%u\t%u\t%u\t%u\t%u\x00" as *const u8 as
                   *const libc::c_char, rec.name,
               &mut rec.pw_min_life as *mut krb5_ui_4,
               &mut rec.pw_max_life as *mut krb5_ui_4,
               &mut rec.pw_min_length as *mut krb5_ui_4,
               &mut rec.pw_min_classes as *mut krb5_ui_4,
               &mut rec.pw_history_num as *mut krb5_ui_4,
               &mut refcnt as *mut libc::c_uint);
    if nread == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if nread != 7 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"cannot parse policy (%d read)\n\x00" as *const u8
                             as *const libc::c_char), nread);
        return 1 as libc::c_int
    }
    ret = krb5_db_create_policy(context, &mut rec);
    if ret != 0 { ret = krb5_db_put_policy(context, &mut rec) }
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating policy\x00" as *const u8 as
                             *const libc::c_char));
        return 1 as libc::c_int
    }
    if verbose != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"created policy %s\n\x00" as *const u8 as
                             *const libc::c_char), rec.name);
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "861:1"]
unsafe extern "C" fn process_r1_8_policy(mut context: krb5_context,
                                         mut fname: *const libc::c_char,
                                         mut filep: *mut FILE,
                                         mut verbose: krb5_boolean,
                                         mut linenop: *mut libc::c_int)
 -> libc::c_int {
    let mut rec: osa_policy_ent_rec =
        osa_policy_ent_rec{version: 0,
                           name: 0 as *mut libc::c_char,
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
    let mut namebuf: [libc::c_char; 1024] = [0; 1024];
    let mut refcnt: libc::c_uint = 0;
    let mut nread: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    memset(&mut rec as *mut osa_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_policy_ent_rec>() as libc::c_ulong);
    *linenop += 1;
    rec.name = namebuf.as_mut_ptr();
    nread =
        fscanf(filep,
               b"%1023s\t%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\x00" as *const u8
                   as *const libc::c_char, rec.name,
               &mut rec.pw_min_life as *mut krb5_ui_4,
               &mut rec.pw_max_life as *mut krb5_ui_4,
               &mut rec.pw_min_length as *mut krb5_ui_4,
               &mut rec.pw_min_classes as *mut krb5_ui_4,
               &mut rec.pw_history_num as *mut krb5_ui_4,
               &mut refcnt as *mut libc::c_uint,
               &mut rec.pw_max_fail as *mut krb5_ui_4,
               &mut rec.pw_failcnt_interval as *mut krb5_ui_4,
               &mut rec.pw_lockout_duration as *mut krb5_ui_4);
    if nread == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if nread != 10 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"cannot parse policy (%d read)\n\x00" as *const u8
                             as *const libc::c_char), nread);
        return 1 as libc::c_int
    }
    ret = krb5_db_create_policy(context, &mut rec);
    if ret != 0 { ret = krb5_db_put_policy(context, &mut rec) }
    if ret != 0 {
        com_err(progname, ret as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating policy\x00" as *const u8 as
                             *const libc::c_char));
        return 1 as libc::c_int
    }
    if verbose != 0 {
        fprintf(stderr,
                b"created policy %s\n\x00" as *const u8 as
                    *const libc::c_char, rec.name);
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "900:1"]
unsafe extern "C" fn process_r1_11_policy(mut context: krb5_context,
                                          mut fname: *const libc::c_char,
                                          mut filep: *mut FILE,
                                          mut verbose: krb5_boolean,
                                          mut linenop: *mut libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut rec: osa_policy_ent_rec =
        osa_policy_ent_rec{version: 0,
                           name: 0 as *mut libc::c_char,
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
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut tl_next: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut namebuf: [libc::c_char; 1024] = [0; 1024];
    let mut keysaltbuf: [libc::c_char; 513] = [0; 513];
    let mut refcnt: libc::c_uint = 0;
    let mut nread: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    memset(&mut rec as *mut osa_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_policy_ent_rec>() as libc::c_ulong);
    *linenop += 1;
    rec.name = namebuf.as_mut_ptr();
    /*
     * Due to a historical error, iprop dumps use the same version before and
     * after the 1.11 policy extensions.  So we need to accept both 1.8-format
     * and 1.11-format policy entries.  Begin by reading the 1.8 fields.
     */
    nread =
        fscanf(filep,
               b"%1023s\t%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\t%u\x00" as *const u8
                   as *const libc::c_char, rec.name,
               &mut rec.pw_min_life as *mut krb5_ui_4,
               &mut rec.pw_max_life as *mut krb5_ui_4,
               &mut rec.pw_min_length as *mut krb5_ui_4,
               &mut rec.pw_min_classes as *mut krb5_ui_4,
               &mut rec.pw_history_num as *mut krb5_ui_4,
               &mut refcnt as *mut libc::c_uint,
               &mut rec.pw_max_fail as *mut krb5_ui_4,
               &mut rec.pw_failcnt_interval as *mut krb5_ui_4,
               &mut rec.pw_lockout_duration as *mut krb5_ui_4);
    if nread == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if nread != 10 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"cannot parse policy (%d read)\n\x00" as *const u8
                             as *const libc::c_char), nread);
        return 1 as libc::c_int
    }
    /* The next character should be a newline (1.8) or a tab (1.11). */
    c = getc(filep);
    if c == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if c != '\n' as i32 {
        /* Read the additional 1.11-format fields. */
        rec.allowed_keysalts = keysaltbuf.as_mut_ptr();
        nread =
            fscanf(filep,
                   b"%u\t%u\t%u\t%512s\t%hd\x00" as *const u8 as
                       *const libc::c_char,
                   &mut rec.attributes as *mut krb5_ui_4,
                   &mut rec.max_life as *mut krb5_ui_4,
                   &mut rec.max_renewable_life as *mut krb5_ui_4,
                   rec.allowed_keysalts,
                   &mut rec.n_tl_data as *mut krb5_int16);
        if nread == -(1 as libc::c_int) { return -(1 as libc::c_int) }
        if nread != 5 as libc::c_int {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"cannot parse policy (%d read)\n\x00" as
                                 *const u8 as *const libc::c_char), nread);
            return 1 as libc::c_int
        }
        if !rec.allowed_keysalts.is_null() &&
               strcmp(rec.allowed_keysalts,
                      b"-\x00" as *const u8 as *const libc::c_char) == 0 {
            rec.allowed_keysalts = 0 as *mut libc::c_char
        }
        /* Get TL data */
        ret = alloc_tl_data(rec.n_tl_data, &mut rec.tl_data);
        if ret != 0 {
            current_block = 5238898740514850902;
        } else {
            ret = process_tl_data(fname, filep, *linenop, rec.tl_data);
            if ret != 0 {
                current_block = 5238898740514850902;
            } else { current_block = 15925075030174552612; }
        }
    } else { current_block = 15925075030174552612; }
    match current_block {
        15925075030174552612 => {
            ret = krb5_db_create_policy(context, &mut rec);
            if ret != 0 { ret = krb5_db_put_policy(context, &mut rec) }
            if ret != 0 {
                com_err(progname, ret as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while creating policy\x00" as *const u8 as
                                     *const libc::c_char));
            } else if verbose != 0 {
                fprintf(stderr,
                        b"created policy %s\n\x00" as *const u8 as
                            *const libc::c_char, rec.name);
            }
        }
        _ => { }
    }
    tl = rec.tl_data;
    while !tl.is_null() {
        tl_next = (*tl).tl_data_next;
        free((*tl).tl_data_contents as *mut libc::c_void);
        free(tl as *mut libc::c_void);
        tl = tl_next
    }
    return if ret != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
/* Read a record which is tagged with "princ" or "policy", calling princfn
 * or policyfn as appropriate. */
#[c2rust::src_loc = "986:1"]
unsafe extern "C" fn process_tagged(mut context: krb5_context,
                                    mut fname: *const libc::c_char,
                                    mut filep: *mut FILE,
                                    mut verbose: krb5_boolean,
                                    mut linenop: *mut libc::c_int,
                                    mut princfn: load_func,
                                    mut policyfn: load_func) -> libc::c_int {
    let mut nread: libc::c_int = 0;
    let mut rectype: [libc::c_char; 100] = [0; 100];
    nread =
        fscanf(filep, b"%99s\t\x00" as *const u8 as *const libc::c_char,
               rectype.as_mut_ptr());
    if nread == -(1 as libc::c_int) { return -(1 as libc::c_int) }
    if nread != 1 as libc::c_int { return 1 as libc::c_int }
    if strcmp(rectype.as_mut_ptr(),
              b"princ\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return Some(princfn.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                     fname,
                                                                                                     filep,
                                                                                                     verbose,
                                                                                                     linenop)
    }
    if strcmp(rectype.as_mut_ptr(),
              b"policy\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return Some(policyfn.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                      fname,
                                                                                                      filep,
                                                                                                      verbose,
                                                                                                      linenop)
    }
    if strcmp(rectype.as_mut_ptr(),
              b"End\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        /* Only expected for OV format */
        return -(1 as libc::c_int)
    }
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"unknown record type \"%s\"\n\x00" as *const u8 as
                         *const libc::c_char), rectype.as_mut_ptr());
    return 1 as libc::c_int;
}
#[c2rust::src_loc = "1010:1"]
unsafe extern "C" fn process_k5beta7_record(mut context: krb5_context,
                                            mut fname: *const libc::c_char,
                                            mut filep: *mut FILE,
                                            mut verbose: krb5_boolean,
                                            mut linenop: *mut libc::c_int)
 -> libc::c_int {
    return process_tagged(context, fname, filep, verbose, linenop,
                          Some(process_k5beta7_princ as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut FILE,
                                                        _: krb5_boolean,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int),
                          Some(process_k5beta7_policy as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut FILE,
                                                        _: krb5_boolean,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int));
}
#[c2rust::src_loc = "1018:1"]
unsafe extern "C" fn process_r1_8_record(mut context: krb5_context,
                                         mut fname: *const libc::c_char,
                                         mut filep: *mut FILE,
                                         mut verbose: krb5_boolean,
                                         mut linenop: *mut libc::c_int)
 -> libc::c_int {
    return process_tagged(context, fname, filep, verbose, linenop,
                          Some(process_k5beta7_princ as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut FILE,
                                                        _: krb5_boolean,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int),
                          Some(process_r1_8_policy as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut FILE,
                                                        _: krb5_boolean,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int));
}
#[c2rust::src_loc = "1026:1"]
unsafe extern "C" fn process_r1_11_record(mut context: krb5_context,
                                          mut fname: *const libc::c_char,
                                          mut filep: *mut FILE,
                                          mut verbose: krb5_boolean,
                                          mut linenop: *mut libc::c_int)
 -> libc::c_int {
    return process_tagged(context, fname, filep, verbose, linenop,
                          Some(process_k5beta7_princ as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut FILE,
                                                        _: krb5_boolean,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int),
                          Some(process_r1_11_policy as
                                   unsafe extern "C" fn(_: krb5_context,
                                                        _:
                                                            *const libc::c_char,
                                                        _: *mut FILE,
                                                        _: krb5_boolean,
                                                        _: *mut libc::c_int)
                                       -> libc::c_int));
}
#[no_mangle]
#[c2rust::src_loc = "1034:14"]
pub static mut beta7_version: dump_version =
    unsafe {
        {
            let mut init =
                _dump_version{name:
                                  b"Kerberos version 5\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              header:
                                  b"kdb5_util load_dump version 4\n\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              updateonly: 0 as libc::c_int,
                              iprop: 0 as libc::c_int,
                              ipropx: 0 as libc::c_int,
                              dump_princ:
                                  Some(dump_k5beta7_princ as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *mut krb5_db_entry,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    krb5_boolean)
                                               -> krb5_error_code),
                              dump_policy:
                                  Some(dump_k5beta7_policy as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    osa_policy_ent_t)
                                               -> ()),
                              load_record:
                                  Some(process_k5beta7_record as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> libc::c_int),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "1044:14"]
pub static mut r1_3_version: dump_version =
    unsafe {
        {
            let mut init =
                _dump_version{name:
                                  b"Kerberos version 5 release 1.3\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              header:
                                  b"kdb5_util load_dump version 5\n\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              updateonly: 0 as libc::c_int,
                              iprop: 0 as libc::c_int,
                              ipropx: 0 as libc::c_int,
                              dump_princ:
                                  Some(dump_k5beta7_princ_withpolicy as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *mut krb5_db_entry,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    krb5_boolean)
                                               -> krb5_error_code),
                              dump_policy:
                                  Some(dump_k5beta7_policy as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    osa_policy_ent_t)
                                               -> ()),
                              load_record:
                                  Some(process_k5beta7_record as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> libc::c_int),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "1054:14"]
pub static mut r1_8_version: dump_version =
    unsafe {
        {
            let mut init =
                _dump_version{name:
                                  b"Kerberos version 5 release 1.8\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              header:
                                  b"kdb5_util load_dump version 6\n\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              updateonly: 0 as libc::c_int,
                              iprop: 0 as libc::c_int,
                              ipropx: 0 as libc::c_int,
                              dump_princ:
                                  Some(dump_k5beta7_princ_withpolicy as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *mut krb5_db_entry,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    krb5_boolean)
                                               -> krb5_error_code),
                              dump_policy:
                                  Some(dump_r1_8_policy as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    osa_policy_ent_t)
                                               -> ()),
                              load_record:
                                  Some(process_r1_8_record as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> libc::c_int),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "1064:14"]
pub static mut r1_11_version: dump_version =
    unsafe {
        {
            let mut init =
                _dump_version{name:
                                  b"Kerberos version 5 release 1.11\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              header:
                                  b"kdb5_util load_dump version 7\n\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              updateonly: 0 as libc::c_int,
                              iprop: 0 as libc::c_int,
                              ipropx: 0 as libc::c_int,
                              dump_princ:
                                  Some(dump_k5beta7_princ_withpolicy as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *mut krb5_db_entry,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    krb5_boolean)
                                               -> krb5_error_code),
                              dump_policy:
                                  Some(dump_r1_11_policy as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    osa_policy_ent_t)
                                               -> ()),
                              load_record:
                                  Some(process_r1_11_record as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> libc::c_int),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "1074:14"]
pub static mut iprop_version: dump_version =
    unsafe {
        {
            let mut init =
                _dump_version{name:
                                  b"Kerberos iprop version\x00" as *const u8
                                      as *const libc::c_char as
                                      *mut libc::c_char,
                              header:
                                  b"iprop\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              updateonly: 0 as libc::c_int,
                              iprop: 1 as libc::c_int,
                              ipropx: 0 as libc::c_int,
                              dump_princ:
                                  Some(dump_k5beta7_princ_withpolicy as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *mut krb5_db_entry,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    krb5_boolean)
                                               -> krb5_error_code),
                              dump_policy:
                                  Some(dump_k5beta7_policy as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    osa_policy_ent_t)
                                               -> ()),
                              load_record:
                                  Some(process_k5beta7_record as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> libc::c_int),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "1084:14"]
pub static mut ipropx_1_version: dump_version =
    unsafe {
        {
            let mut init =
                _dump_version{name:
                                  b"Kerberos iprop extensible version\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char,
                              header:
                                  b"ipropx\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                              updateonly: 0 as libc::c_int,
                              iprop: 1 as libc::c_int,
                              ipropx: 1 as libc::c_int,
                              dump_princ:
                                  Some(dump_k5beta7_princ_withpolicy as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *mut krb5_db_entry,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    krb5_boolean)
                                               -> krb5_error_code),
                              dump_policy:
                                  Some(dump_r1_11_policy as
                                           unsafe extern "C" fn(_:
                                                                    *mut libc::c_void,
                                                                _:
                                                                    osa_policy_ent_t)
                                               -> ()),
                              load_record:
                                  Some(process_r1_11_record as
                                           unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const libc::c_char,
                                                                _: *mut FILE,
                                                                _:
                                                                    krb5_boolean,
                                                                _:
                                                                    *mut libc::c_int)
                                               -> libc::c_int),};
            init
        }
    };
/* Read the dump header.  Return 1 on success, 0 if the file is not a
 * recognized iprop dump format. */
#[c2rust::src_loc = "1097:1"]
unsafe extern "C" fn parse_iprop_header(mut buf: *mut libc::c_char,
                                        mut dv: *mut *mut dump_version,
                                        mut last: *mut kdb_last_t)
 -> libc::c_int {
    let mut head: [libc::c_char; 128] = [0; 128];
    let mut nread: libc::c_int = 0;
    let mut u: [uint32_t; 4] = [0; 4];
    let mut up: *mut uint32_t =
        &mut *u.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut uint32_t;
    nread =
        sscanf(buf,
               b"%127s %u %u %u %u\x00" as *const u8 as *const libc::c_char,
               head.as_mut_ptr(),
               &mut *u.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut uint32_t,
               &mut *u.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut uint32_t,
               &mut *u.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut uint32_t,
               &mut *u.as_mut_ptr().offset(3 as libc::c_int as isize) as
                   *mut uint32_t);
    if nread < 1 as libc::c_int { return 0 as libc::c_int }
    if strcmp(head.as_mut_ptr(), ipropx_1_version.header) == 0 {
        if nread != 5 as libc::c_int { return 0 as libc::c_int }
        if u[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_uint {
            *dv = &mut iprop_version
        } else if u[0 as libc::c_int as usize] ==
                      1 as libc::c_int as libc::c_uint {
            *dv = &mut ipropx_1_version
        } else {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: Unknown iprop dump version %d\n\x00" as
                                 *const u8 as *const libc::c_char), progname,
                    u[0 as libc::c_int as usize]);
            return 0 as libc::c_int
        }
        up =
            &mut *u.as_mut_ptr().offset(1 as libc::c_int as isize) as
                *mut uint32_t
    } else if strcmp(head.as_mut_ptr(), iprop_version.header) == 0 {
        if nread != 4 as libc::c_int { return 0 as libc::c_int }
        *dv = &mut iprop_version
    } else {
        fprintf(stderr,
                b"Invalid iprop header\n\x00" as *const u8 as
                    *const libc::c_char);
        return 0 as libc::c_int
    }
    let fresh1 = up;
    up = up.offset(1);
    (*last).last_sno = *fresh1;
    let fresh2 = up;
    up = up.offset(1);
    (*last).last_time.seconds = *fresh2;
    let fresh3 = up;
    up = up.offset(1);
    (*last).last_time.useconds = *fresh3;
    return 1 as libc::c_int;
}
/* Return true if the serial number and timestamp in an existing dump file is
 * in the ulog. */
#[c2rust::src_loc = "1139:1"]
unsafe extern "C" fn current_dump_sno_in_ulog(mut context: krb5_context,
                                              mut ifile: *const libc::c_char)
 -> krb5_boolean {
    let mut status: update_status_t =
        UPDATE_OK; /* aliasing other errors to ENOENT here is OK */
    let mut junk: *mut dump_version = 0 as *mut dump_version;
    let mut last: kdb_last_t =
        kdb_last_t{last_sno: 0,
                   last_time: kdbe_time_t{seconds: 0, useconds: 0,},};
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(ifile, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() { return 0 as libc::c_int as krb5_boolean }
    r =
        fgets(buf.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                  as libc::c_int, f);
    fclose(f);
    if r.is_null() {
        return if *__errno_location() != 0 {
                   -(1 as libc::c_int)
               } else { 0 as libc::c_int } as krb5_boolean
    }
    if parse_iprop_header(buf.as_mut_ptr(), &mut junk, &mut last) == 0 {
        return 0 as libc::c_int as krb5_boolean
    }
    status = ulog_get_sno_status(context, &mut last);
    return (status as libc::c_uint == UPDATE_OK as libc::c_int as libc::c_uint
                ||
                status as libc::c_uint ==
                    UPDATE_NIL as libc::c_int as libc::c_uint) as libc::c_int
               as krb5_boolean;
}
/*
 * usage is:
 *      dump_db [-b7] [-r13] [-r18] [-verbose] [-mkey_convert]
 *              [-new_mkey_file mkey_file] [-rev] [-recurse]
 *              [filename [principals...]]
 */
#[no_mangle]
#[c2rust::src_loc = "1170:1"]
pub unsafe extern "C" fn dump_db(mut argc: libc::c_int,
                                 mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut args: dump_args =
        dump_args{ofile: 0 as *mut FILE,
                  context: 0 as *mut _krb5_context,
                  names: 0 as *mut *mut libc::c_char,
                  nnames: 0,
                  verbose: 0,
                  omit_nra: 0,
                  dump: 0 as *mut dump_version,};
    let mut ofile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpofile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_mkey_file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: krb5_error_code = 0;
    let mut retval: krb5_error_code = 0;
    let mut dump: *mut dump_version = 0 as *mut dump_version;
    let mut aindex: libc::c_int = 0;
    let mut ok_fd: libc::c_int = -(1 as libc::c_int);
    let mut dump_sno: libc::c_int = 0 as libc::c_int;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ipropx_version: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut kt_kvno: krb5_kvno = 0;
    let mut conditional: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut last: kdb_last_t =
        kdb_last_t{last_sno: 0,
                   last_time: kdbe_time_t{seconds: 0, useconds: 0,},};
    let mut iterflags: krb5_flags = 0 as libc::c_int;
    /* Parse the arguments. */
    dump = &mut r1_11_version;
    args.verbose = 0 as libc::c_int as krb5_boolean;
    args.omit_nra = 0 as libc::c_int as krb5_boolean;
    mkey_convert = 0 as libc::c_int as krb5_boolean;
    log_ctx = (*util_context).kdblog_context;
    /*
     * Parse the qualifiers.
     */
    aindex = 1 as libc::c_int;
    loop  {
        if !(aindex < argc) { current_block = 5891011138178424807; break ; }
        if strcmp(*argv.offset(aindex as isize),
                  b"-b7\x00" as *const u8 as *const libc::c_char) == 0 {
            dump = &mut beta7_version
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-ov\x00" as *const u8 as *const libc::c_char) == 0
         {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"OV dump format not supported\n\x00" as
                                 *const u8 as *const libc::c_char));
            current_block = 11625054877772784798;
            break ;
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-r13\x00" as *const u8 as *const libc::c_char) == 0
         {
            dump = &mut r1_3_version
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-r18\x00" as *const u8 as *const libc::c_char) == 0
         {
            dump = &mut r1_8_version
        } else if strncmp(*argv.offset(aindex as isize),
                          b"-i\x00" as *const u8 as *const libc::c_char,
                          2 as libc::c_int as libc::c_ulong) == 0 {
            if !log_ctx.is_null() && (*log_ctx).iproprole as libc::c_uint != 0
               {
                /* ipropx_version is the maximum version acceptable. */
                ipropx_version =
                    atoi((*argv.offset(aindex as
                                           isize)).offset(2 as libc::c_int as
                                                              isize)) as
                        libc::c_uint;
                dump =
                    if ipropx_version != 0 {
                        &mut ipropx_1_version
                    } else { &mut iprop_version };
                /*
                 * dump_sno is used to indicate if the serial number should be
                 * populated in the output file to be used later by iprop for
                 * updating the replica's update log when loading.
                 */
                dump_sno = 1 as libc::c_int;
                /* FLAG_OMIT_NRA is set to indicate that non-replicated
                 * attributes should be omitted. */
                args.omit_nra = 1 as libc::c_int as krb5_boolean
            } else {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Iprop not enabled\n\x00" as *const u8 as
                                     *const libc::c_char));
                current_block = 11625054877772784798;
                break ;
            }
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-c\x00" as *const u8 as *const libc::c_char) == 0 {
            conditional = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-verbose\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            args.verbose = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-mkey_convert\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            mkey_convert = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-new_mkey_file\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            aindex += 1;
            new_mkey_file = *argv.offset(aindex as isize);
            mkey_convert = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-rev\x00" as *const u8 as *const libc::c_char) == 0
         {
            iterflags |= 0x2 as libc::c_int
        } else {
            if !(strcmp(*argv.offset(aindex as isize),
                        b"-recurse\x00" as *const u8 as *const libc::c_char)
                     == 0) {
                current_block = 5891011138178424807;
                break ;
            }
            iterflags |= 0x4 as libc::c_int
        }
        aindex += 1
    }
    match current_block {
        5891011138178424807 => {
            args.names = 0 as *mut *mut libc::c_char;
            args.nnames = 0 as libc::c_int;
            if aindex < argc {
                ofile = *argv.offset(aindex as isize);
                aindex += 1;
                if aindex < argc {
                    args.names =
                        &mut *argv.offset(aindex as isize) as
                            *mut *mut libc::c_char;
                    args.nnames = argc - aindex
                }
            }
            /* If a conditional ipropx dump we check if the existing dump is
     * good enough. */
            if !ofile.is_null() && conditional != 0 {
                if (*dump).iprop == 0 {
                    com_err(progname, 0 as libc::c_int as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Conditional dump is an undocumented option for use only for iprop dumps\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    current_block = 11625054877772784798;
                } else {
                    if current_dump_sno_in_ulog(util_context, ofile) != 0 {
                        return
                    }
                    current_block = 307447392441238883;
                }
            } else { current_block = 307447392441238883; }
            match current_block {
                11625054877772784798 => { }
                _ =>
                /*
     * Make sure the database is open.  The policy database only has
     * to be opened if we try a dump that uses it.
     */
                {
                    if dbactive == 0 {
                        com_err(progname, 0 as libc::c_int as errcode_t,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"Database not currently opened!\x00"
                                             as *const u8 as
                                             *const libc::c_char));
                    } else {
                        /*
     * If we're doing a master key conversion, set up for it.
     */
                        if mkey_convert != 0 {
                            if valid_master_key == 0 {
                                /* TRUE here means read the keyboard, but only once */
                                retval =
                                    krb5_db_fetch_mkey(util_context,
                                                       master_princ,
                                                       master_keyblock.enctype,
                                                       1 as libc::c_int as
                                                           krb5_boolean,
                                                       0 as libc::c_int as
                                                           krb5_boolean,
                                                       0 as *mut libc::c_char,
                                                       0 as *mut krb5_kvno,
                                                       0 as *mut krb5_data,
                                                       &mut master_keyblock);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while reading master key\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit(1 as libc::c_int);
                                }
                                retval =
                                    krb5_db_fetch_mkey_list(util_context,
                                                            master_princ,
                                                            &mut master_keyblock);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while verifying master key\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit(1 as libc::c_int);
                                }
                            }
                            new_master_keyblock.enctype =
                                global_params.enctype;
                            if new_master_keyblock.enctype ==
                                   0x1ff as libc::c_int {
                                new_master_keyblock.enctype =
                                    0x12 as libc::c_int
                            }
                            if !new_mkey_file.is_null() {
                                if global_params.mask &
                                       0x20000000 as libc::c_int as
                                           libc::c_long != 0 {
                                    kt_kvno = global_params.kvno
                                } else {
                                    kt_kvno = 0 as libc::c_int as krb5_kvno
                                }
                                retval =
                                    krb5_db_fetch_mkey(util_context,
                                                       master_princ,
                                                       new_master_keyblock.enctype,
                                                       0 as libc::c_int as
                                                           krb5_boolean,
                                                       0 as libc::c_int as
                                                           krb5_boolean,
                                                       new_mkey_file,
                                                       &mut kt_kvno,
                                                       0 as *mut krb5_data,
                                                       &mut new_master_keyblock);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while reading new master key\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit(1 as libc::c_int);
                                }
                            } else {
                                printf(dgettext(b"mit-krb5\x00" as *const u8
                                                    as *const libc::c_char,
                                                b"Please enter new master key....\n\x00"
                                                    as *const u8 as
                                                    *const libc::c_char));
                                retval =
                                    krb5_db_fetch_mkey(util_context,
                                                       master_princ,
                                                       new_master_keyblock.enctype,
                                                       1 as libc::c_int as
                                                           krb5_boolean,
                                                       1 as libc::c_int as
                                                           krb5_boolean,
                                                       0 as *mut libc::c_char,
                                                       0 as *mut krb5_kvno,
                                                       0 as *mut krb5_data,
                                                       &mut new_master_keyblock);
                                if retval != 0 {
                                    com_err(progname, retval as errcode_t,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"while reading new master key\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    exit(1 as libc::c_int);
                                }
                            }
                            /* Get new master key vno that will be used to protect princs. */
                            new_mkvno =
                                get_next_kvno(util_context, master_entry)
                        }
                        ret = 0 as libc::c_int;
                        if !ofile.is_null() &&
                               strcmp(ofile,
                                      b"-\x00" as *const u8 as
                                          *const libc::c_char) != 0 {
                            /* Discourage accidental dumping to filenames beginning with '-'. */
                            if *ofile.offset(0 as libc::c_int as isize) as
                                   libc::c_int == '-' as i32 {
                                usage(); /* prep_ok_file() bumps exit_status */
                            }
                            if prep_ok_file(util_context, ofile, &mut ok_fd)
                                   == 0 {
                                return
                            }
                            f = create_ofile(ofile, &mut tmpofile);
                            if f.is_null() {
                                com_err(progname,
                                        *__errno_location() as errcode_t,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"while opening %s for writing\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        ofile);
                                current_block = 11625054877772784798;
                            } else { current_block = 16593409533420678784; }
                        } else {
                            f = stdout;
                            current_block = 16593409533420678784;
                        }
                        match current_block {
                            11625054877772784798 => { }
                            _ => {
                                args.ofile = f;
                                args.context = util_context;
                                args.dump = dump;
                                fprintf(args.ofile,
                                        b"%s\x00" as *const u8 as
                                            *const libc::c_char,
                                        (*dump).header);
                                if dump_sno != 0 {
                                    ret =
                                        ulog_get_last(util_context,
                                                      &mut last);
                                    if ret != 0 {
                                        com_err(progname, ret as errcode_t,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"while reading update log header\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                        current_block = 11625054877772784798;
                                    } else {
                                        if ipropx_version != 0 {
                                            fprintf(f,
                                                    b" %u\x00" as *const u8 as
                                                        *const libc::c_char,
                                                    1 as libc::c_int);
                                        }
                                        fprintf(f,
                                                b" %u\x00" as *const u8 as
                                                    *const libc::c_char,
                                                last.last_sno);
                                        fprintf(f,
                                                b" %u\x00" as *const u8 as
                                                    *const libc::c_char,
                                                last.last_time.seconds);
                                        fprintf(f,
                                                b" %u\x00" as *const u8 as
                                                    *const libc::c_char,
                                                last.last_time.useconds);
                                        current_block = 13014351284863956202;
                                    }
                                } else {
                                    current_block = 13014351284863956202;
                                }
                                match current_block {
                                    11625054877772784798 => { }
                                    _ => {
                                        if *(*dump).header.offset(strlen((*dump).header).wrapping_sub(1
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_ulong)
                                                                      as
                                                                      isize)
                                               as libc::c_int != '\n' as i32 {
                                            fputc('\n' as i32, args.ofile);
                                        }
                                        ret =
                                            krb5_db_iterate(util_context,
                                                            0 as
                                                                *mut libc::c_char,
                                                            Some(dump_iterator
                                                                     as
                                                                     unsafe extern "C" fn(_:
                                                                                              *mut libc::c_void,
                                                                                          _:
                                                                                              *mut krb5_db_entry)
                                                                         ->
                                                                             krb5_error_code),
                                                            &mut args as
                                                                *mut dump_args
                                                                as
                                                                krb5_pointer,
                                                            iterflags);
                                        if ret != 0 {
                                            com_err(progname,
                                                    ret as errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"performing %s dump\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    (*dump).name);
                                        } else {
                                            /* Don't dump policies if specific principal entries were requested. */
                                            if (*dump).dump_policy.is_some()
                                                   &&
                                                   args.nnames ==
                                                       0 as libc::c_int {
                                                ret =
                                                    krb5_db_iter_policy(util_context,
                                                                        b"*\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        (*dump).dump_policy,
                                                                        &mut args
                                                                            as
                                                                            *mut dump_args
                                                                            as
                                                                            *mut libc::c_void);
                                                if ret != 0 {
                                                    com_err(progname,
                                                            ret as errcode_t,
                                                            dgettext(b"mit-krb5\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char,
                                                                     b"performing %s dump\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char),
                                                            (*dump).name);
                                                    current_block =
                                                        11625054877772784798;
                                                } else {
                                                    current_block =
                                                        11702799181856929651;
                                                }
                                            } else {
                                                current_block =
                                                    11702799181856929651;
                                            }
                                            match current_block {
                                                11625054877772784798 => { }
                                                _ => {
                                                    if f != stdout {
                                                        fclose(f);
                                                        finish_ofile(ofile,
                                                                     &mut tmpofile);
                                                        update_ok_file(util_context,
                                                                       ok_fd);
                                                    }
                                                    return
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
        _ => { }
    }
    if !tmpofile.is_null() { unlink(tmpofile); }
    free(tmpofile as *mut libc::c_void);
    exit_status += 1;
}
/* Restore the database from any version dump file. */
#[c2rust::src_loc = "1397:1"]
unsafe extern "C" fn restore_dump(mut context: krb5_context,
                                  mut dumpfile: *mut libc::c_char,
                                  mut f: *mut FILE, mut verbose: krb5_boolean,
                                  mut dump: *mut dump_version)
 -> libc::c_int {
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut lineno: libc::c_int = 1 as libc::c_int;
    loop 
         /* Process the records. */
         {
        err =
            (*dump).load_record.expect("non-null function pointer")(context,
                                                                    dumpfile,
                                                                    f,
                                                                    verbose,
                                                                    &mut lineno);
        if !(err == 0) { break ; }
    }
    if err != -(1 as libc::c_int) {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s: error processing line %d of %s\n\x00" as
                             *const u8 as *const libc::c_char), progname,
                lineno, dumpfile);
        return err
    }
    return 0 as libc::c_int;
}
/*
 * Usage: load_db [-b7] [-r13] [-r18] [-verbose] [-update] [-hash] filename
 */
#[no_mangle]
#[c2rust::src_loc = "1417:1"]
pub unsafe extern "C" fn load_db(mut argc: libc::c_int,
                                 mut argv: *mut *mut libc::c_char) {
    let mut current_block: u64;
    let mut ret: krb5_error_code = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut dumpfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dbname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    let mut load: *mut dump_version = 0 as *mut dump_version;
    let mut aindex: libc::c_int = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut last: kdb_last_t =
        kdb_last_t{last_sno: 0,
                   last_time: kdbe_time_t{seconds: 0, useconds: 0,},};
    let mut db_locked: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut temp_db_created: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut verbose: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut update: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    let mut iprop_load: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    /* Parse the arguments. */
    dbname = global_params.dbname;
    exit_status = 0 as libc::c_int;
    log_ctx = (*util_context).kdblog_context;
    aindex = 1 as libc::c_int;
    loop  {
        if !(aindex < argc) { current_block = 4090602189656566074; break ; }
        if strcmp(*argv.offset(aindex as isize),
                  b"-b7\x00" as *const u8 as *const libc::c_char) == 0 {
            load = &mut beta7_version
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-ov\x00" as *const u8 as *const libc::c_char) == 0
         {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"OV dump format not supported\n\x00" as
                                 *const u8 as *const libc::c_char));
            current_block = 16487195769851017152;
            break ;
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-r13\x00" as *const u8 as *const libc::c_char) == 0
         {
            load = &mut r1_3_version
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-r18\x00" as *const u8 as *const libc::c_char) == 0
         {
            load = &mut r1_8_version
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-i\x00" as *const u8 as *const libc::c_char) == 0 {
            if !log_ctx.is_null() && (*log_ctx).iproprole as libc::c_uint != 0
               {
                load = &mut iprop_version;
                iprop_load = 1 as libc::c_int as krb5_boolean
            } else {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Iprop not enabled\n\x00" as *const u8 as
                                     *const libc::c_char));
                current_block = 16487195769851017152;
                break ;
            }
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-verbose\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            verbose = 1 as libc::c_int as krb5_boolean
        } else if strcmp(*argv.offset(aindex as isize),
                         b"-update\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            update = 1 as libc::c_int as krb5_boolean
        } else {
            if !(strcmp(*argv.offset(aindex as isize),
                        b"-hash\x00" as *const u8 as *const libc::c_char) ==
                     0) {
                current_block = 4090602189656566074;
                break ;
            }
            if add_db_arg(b"hash=true\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char) == 0 {
                com_err(progname, 12 as libc::c_int as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while parsing options\x00" as *const u8 as
                                     *const libc::c_char));
                current_block = 16487195769851017152;
                break ;
            }
        }
        aindex += 1
    }
    match current_block {
        4090602189656566074 => {
            if argc - aindex != 1 as libc::c_int { usage(); }
            dumpfile = *argv.offset(aindex as isize);
            /* Open the dumpfile. */
            if !dumpfile.is_null() {
                f =
                    fopen(dumpfile,
                          b"r\x00" as *const u8 as *const libc::c_char);
                if f.is_null() {
                    com_err(progname, *__errno_location() as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while opening %s\x00" as *const u8 as
                                         *const libc::c_char), dumpfile);
                    current_block = 16487195769851017152;
                } else { current_block = 11763295167351361500; }
            } else {
                f = stdin;
                dumpfile =
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"standard input\x00" as *const u8 as
                                 *const libc::c_char);
                current_block = 11763295167351361500;
            }
            match current_block {
                16487195769851017152 => { }
                _ =>
                /* Auto-detect dump version if we weren't told, or verify if we were. */
                {
                    if fgets(buf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 8192]>() as
                                 libc::c_ulong as libc::c_int, f).is_null() {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"%s: can\'t read dump header in %s\n\x00"
                                             as *const u8 as
                                             *const libc::c_char), progname,
                                dumpfile);
                        current_block = 16487195769851017152;
                    } else {
                        if !load.is_null() {
                            /* Only check what we know; some headers only contain a prefix.
         * NB: this should work for ipropx even though load is iprop */
                            if strncmp(buf.as_mut_ptr(), (*load).header,
                                       strlen((*load).header)) !=
                                   0 as libc::c_int {
                                fprintf(stderr,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"%s: dump header bad in %s\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char),
                                        progname, dumpfile);
                                current_block = 16487195769851017152;
                            } else { current_block = 16779030619667747692; }
                        } else if strcmp(buf.as_mut_ptr(),
                                         beta7_version.header) ==
                                      0 as libc::c_int {
                            load = &mut beta7_version;
                            current_block = 16779030619667747692;
                        } else if strcmp(buf.as_mut_ptr(),
                                         r1_3_version.header) ==
                                      0 as libc::c_int {
                            load = &mut r1_3_version;
                            current_block = 16779030619667747692;
                        } else if strcmp(buf.as_mut_ptr(),
                                         r1_8_version.header) ==
                                      0 as libc::c_int {
                            load = &mut r1_8_version;
                            current_block = 16779030619667747692;
                        } else if strcmp(buf.as_mut_ptr(),
                                         r1_11_version.header) ==
                                      0 as libc::c_int {
                            load = &mut r1_11_version;
                            current_block = 16779030619667747692;
                        } else {
                            fprintf(stderr,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"%s: dump header bad in %s\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char),
                                    progname, dumpfile);
                            current_block = 16487195769851017152;
                        }
                        match current_block {
                            16487195769851017152 => { }
                            _ => {
                                if global_params.iprop_enabled != 0 &&
                                       ulog_map(util_context,
                                                global_params.iprop_logfile,
                                                global_params.iprop_ulogsize)
                                           != 0 {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"Could not open iprop ulog\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char));
                                    current_block = 16487195769851017152;
                                } else if (*load).updateonly != 0 &&
                                              update == 0 {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"%s: dump version %s can only be loaded with the -update flag\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            progname, (*load).name);
                                    current_block = 16487195769851017152;
                                } else {
                                    /* If we are not in update mode, we create an alternate database and then
     * promote it to be the live db. */
                                    if update == 0 {
                                        if add_db_arg(b"temporary\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char)
                                               == 0 {
                                            com_err(progname,
                                                    12 as libc::c_int as
                                                        errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"computing parameters for database\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                            current_block =
                                                16487195769851017152;
                                        } else if iprop_load != 0 &&
                                                      add_db_arg(b"merge_nra\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char
                                                                     as
                                                                     *mut libc::c_char)
                                                          == 0 {
                                            com_err(progname,
                                                    12 as libc::c_int as
                                                        errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"computing parameters for database\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                            current_block =
                                                16487195769851017152;
                                        } else {
                                            ret =
                                                krb5_db_create(util_context,
                                                               db5util_db_args);
                                            if ret != 0 {
                                                com_err(progname,
                                                        ret as errcode_t,
                                                        dgettext(b"mit-krb5\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"while creating database\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char));
                                                current_block =
                                                    16487195769851017152;
                                            } else {
                                                temp_db_created =
                                                    1 as libc::c_int as
                                                        krb5_boolean;
                                                current_block =
                                                    10411727741569490626;
                                            }
                                        }
                                    } else {
                                        /* Initialize the database. */
                                        ret =
                                            krb5_db_open(util_context,
                                                         db5util_db_args,
                                                         0 as libc::c_int |
                                                             0x200 as
                                                                 libc::c_int);
                                        if ret != 0 {
                                            com_err(progname,
                                                    ret as errcode_t,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"while opening database\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                            current_block =
                                                16487195769851017152;
                                        } else {
                                            /* Make sure the db is left unusable if the update fails, if the db
         * supports locking. */
                                            ret =
                                                krb5_db_lock(util_context,
                                                             0x8 as
                                                                 libc::c_int);
                                            if ret == 0 as libc::c_int {
                                                db_locked =
                                                    1 as libc::c_int as
                                                        krb5_boolean;
                                                current_block =
                                                    10411727741569490626;
                                            } else if ret as libc::c_long !=
                                                          -(1765328134 as
                                                                libc::c_long)
                                             {
                                                com_err(progname,
                                                        ret as errcode_t,
                                                        dgettext(b"mit-krb5\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char,
                                                                 b"while permanently locking database\x00"
                                                                     as
                                                                     *const u8
                                                                     as
                                                                     *const libc::c_char));
                                                current_block =
                                                    16487195769851017152;
                                            } else {
                                                current_block =
                                                    10411727741569490626;
                                            }
                                        }
                                    }
                                    match current_block {
                                        16487195769851017152 => { }
                                        _ => {
                                            if !log_ctx.is_null() &&
                                                   (*log_ctx).iproprole as
                                                       libc::c_uint != 0 &&
                                                   update == 0 {
                                                /* Don't record updates we are making to the temporary DB.  We will
         * reinitialize or update the ulog header after promoting it. */
                                                (*log_ctx).iproprole =
                                                    IPROP_REPLICA;
                                                if iprop_load != 0 {
                                                    /* Parse the iprop header information. */
                                                    if parse_iprop_header(buf.as_mut_ptr(),
                                                                          &mut load,
                                                                          &mut last)
                                                           == 0 {
                                                        current_block =
                                                            16487195769851017152;
                                                    } else {
                                                        current_block =
                                                            2945622622075328793;
                                                    }
                                                } else {
                                                    current_block =
                                                        2945622622075328793;
                                                }
                                            } else {
                                                current_block =
                                                    2945622622075328793;
                                            }
                                            match current_block {
                                                16487195769851017152 => { }
                                                _ => {
                                                    if restore_dump(util_context,
                                                                    if !dumpfile.is_null()
                                                                       {
                                                                        dumpfile
                                                                    } else {
                                                                        dgettext(b"mit-krb5\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"standard input\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char)
                                                                    }, f,
                                                                    verbose,
                                                                    load) != 0
                                                       {
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"%s: %s restore failed\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char),
                                                                progname,
                                                                (*load).name);
                                                        current_block =
                                                            16487195769851017152;
                                                    } else if db_locked != 0
                                                                  &&
                                                                  {
                                                                      ret =
                                                                          krb5_db_unlock(util_context);
                                                                      (ret) !=
                                                                          0
                                                                  } {
                                                        com_err(progname,
                                                                ret as
                                                                    errcode_t,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"while unlocking database\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                        current_block =
                                                            16487195769851017152;
                                                    } else if update == 0 {
                                                        /* Initialize the ulog header before promoting so we can't leave behind
         * the pre-load ulog state if we are killed just after promoting. */
                                                        if !log_ctx.is_null()
                                                               &&
                                                               (*log_ctx).iproprole
                                                                   as
                                                                   libc::c_uint
                                                                   != 0 {
                                                            ret =
                                                                ulog_init_header(util_context);
                                                            if ret != 0 {
                                                                com_err(progname,
                                                                        ret as
                                                                            errcode_t,
                                                                        dgettext(b"mit-krb5\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char,
                                                                                 b"while reinitializing update log\x00"
                                                                                     as
                                                                                     *const u8
                                                                                     as
                                                                                     *const libc::c_char));
                                                                current_block
                                                                    =
                                                                    16487195769851017152;
                                                            } else {
                                                                current_block
                                                                    =
                                                                    9775647934248138666;
                                                            }
                                                        } else {
                                                            current_block =
                                                                9775647934248138666;
                                                        }
                                                        match current_block {
                                                            16487195769851017152
                                                            => {
                                                            }
                                                            _ => {
                                                                ret =
                                                                    krb5_db_promote(util_context,
                                                                                    db5util_db_args);
                                                                /* Ignore a not supported error since there is nothing to do about it
         * anyway. */
                                                                if ret !=
                                                                       0 as
                                                                           libc::c_int
                                                                       &&
                                                                       ret as
                                                                           libc::c_long
                                                                           !=
                                                                           -(1765328134
                                                                                 as
                                                                                 libc::c_long)
                                                                   {
                                                                    com_err(progname,
                                                                            ret
                                                                                as
                                                                                errcode_t,
                                                                            dgettext(b"mit-krb5\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char,
                                                                                     b"while making newly loaded database live\x00"
                                                                                         as
                                                                                         *const u8
                                                                                         as
                                                                                         *const libc::c_char));
                                                                    current_block
                                                                        =
                                                                        16487195769851017152;
                                                                } else if !log_ctx.is_null()
                                                                              &&
                                                                              (*log_ctx).iproprole
                                                                                  as
                                                                                  libc::c_uint
                                                                                  !=
                                                                                  0
                                                                 {
                                                                    /* Reinitialize the ulog header since we replaced the DB, and
             * record the iprop state if we received it. */
                                                                    ret =
                                                                        ulog_init_header(util_context);
                                                                    if ret !=
                                                                           0 {
                                                                        com_err(progname,
                                                                                ret
                                                                                    as
                                                                                    errcode_t,
                                                                                dgettext(b"mit-krb5\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char,
                                                                                         b"while reinitializing update log\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
                                                                        current_block
                                                                            =
                                                                            16487195769851017152;
                                                                    } else if iprop_load
                                                                                  !=
                                                                                  0
                                                                     {
                                                                        ret =
                                                                            ulog_set_last(util_context,
                                                                                          &mut last);
                                                                        if ret
                                                                               !=
                                                                               0
                                                                           {
                                                                            com_err(progname,
                                                                                    ret
                                                                                        as
                                                                                        errcode_t,
                                                                                    dgettext(b"mit-krb5\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char,
                                                                                             b"while writing update log header\x00"
                                                                                                 as
                                                                                                 *const u8
                                                                                                 as
                                                                                                 *const libc::c_char));
                                                                            current_block
                                                                                =
                                                                                16487195769851017152;
                                                                        } else {
                                                                            current_block
                                                                                =
                                                                                5702461520494192007;
                                                                        }
                                                                    } else {
                                                                        current_block
                                                                            =
                                                                            5702461520494192007;
                                                                    }
                                                                } else {
                                                                    current_block
                                                                        =
                                                                        5702461520494192007;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block =
                                                            5702461520494192007;
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
        _ => { }
    }
    match current_block {
        16487195769851017152 => { exit_status += 1 }
        _ => { }
    }
    /* If we created a temporary DB but didn't succeed, destroy it. */
    if exit_status != 0 && temp_db_created != 0 {
        ret = krb5_db_destroy(util_context, db5util_db_args);
        /* Ignore a not supported error since there is nothing to do about
         * it anyway. */
        if ret != 0 as libc::c_int &&
               ret as libc::c_long != -(1765328134 as libc::c_long) {
            com_err(progname, ret as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while deleting bad database %s\x00" as
                                 *const u8 as *const libc::c_char), dbname);
        }
    }
    if !f.is_null() && f != stdin { fclose(f); };
}
