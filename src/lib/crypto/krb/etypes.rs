use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:28"]
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
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    #[c2rust::src_loc = "182:1"]
    pub type krb5_keyusage = krb5_int32;
    #[c2rust::src_loc = "183:1"]
    pub type krb5_cryptotype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
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
    #[c2rust::src_loc = "415:16"]
    pub struct _krb5_crypto_iov {
        pub flags: krb5_cryptotype,
        pub data: krb5_data,
    }
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
    #[c2rust::src_loc = "415:1"]
    pub type krb5_crypto_iov = _krb5_crypto_iov;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::krb5_key_st;
    /* *< @ref KRB5_CRYPTO_TYPE type of the iov */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:28"]
pub mod k5_int_h {
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
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key};
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:28"]
pub mod crypto_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:8"]
    pub struct krb5_enc_provider {
        pub block_size: size_t,
        pub keybytes: size_t,
        pub keylength: size_t,
        pub encrypt: Option<unsafe extern "C" fn(_: krb5_key,
                                                 _: *const krb5_data,
                                                 _: *mut krb5_crypto_iov,
                                                 _: size_t)
                                -> krb5_error_code>,
        pub decrypt: Option<unsafe extern "C" fn(_: krb5_key,
                                                 _: *const krb5_data,
                                                 _: *mut krb5_crypto_iov,
                                                 _: size_t)
                                -> krb5_error_code>,
        pub cbc_mac: Option<unsafe extern "C" fn(_: krb5_key,
                                                 _: *const krb5_crypto_iov,
                                                 _: size_t,
                                                 _: *const krb5_data,
                                                 _: *mut krb5_data)
                                -> krb5_error_code>,
        pub init_state: Option<unsafe extern "C" fn(_: *const krb5_keyblock,
                                                    _: krb5_keyusage,
                                                    _: *mut krb5_data)
                                   -> krb5_error_code>,
        pub free_state: Option<unsafe extern "C" fn(_: *mut krb5_data) -> ()>,
        pub key_cleanup: Option<unsafe extern "C" fn(_: krb5_key) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "160:8"]
    pub struct krb5_cksumtypes {
        pub ctype: krb5_cksumtype,
        pub name: *mut libc::c_char,
        pub aliases: [*mut libc::c_char; 2],
        pub out_string: *mut libc::c_char,
        pub enc: *const krb5_enc_provider,
        pub hash: *const krb5_hash_provider,
        pub checksum: checksum_func,
        pub verify: verify_func,
        pub compute_size: libc::c_uint,
        pub output_size: libc::c_uint,
        pub flags: krb5_flags,
    }
    #[c2rust::src_loc = "153:1"]
    pub type verify_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_cksumtypes, _: krb5_key,
                                    _: krb5_keyusage,
                                    _: *const krb5_crypto_iov, _: size_t,
                                    _: *const krb5_data, _: *mut krb5_boolean)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "140:1"]
    pub type checksum_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_cksumtypes, _: krb5_key,
                                    _: krb5_keyusage,
                                    _: *const krb5_crypto_iov, _: size_t,
                                    _: *mut krb5_data) -> krb5_error_code>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:8"]
    pub struct krb5_hash_provider {
        pub hash_name: [libc::c_char; 8],
        pub hashsize: size_t,
        pub blocksize: size_t,
        pub hash: Option<unsafe extern "C" fn(_: *const krb5_crypto_iov,
                                              _: size_t, _: *mut krb5_data)
                             -> krb5_error_code>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct krb5_keytypes {
        pub etype: krb5_enctype,
        pub name: *mut libc::c_char,
        pub aliases: [*mut libc::c_char; 2],
        pub out_string: *mut libc::c_char,
        pub enc: *const krb5_enc_provider,
        pub hash: *const krb5_hash_provider,
        pub prf_length: size_t,
        pub crypto_length: crypto_length_func,
        pub encrypt: crypt_func,
        pub decrypt: crypt_func,
        pub str2key: str2key_func,
        pub rand2key: rand2key_func,
        pub prf: prf_func,
        pub required_ctype: krb5_cksumtype,
        pub flags: krb5_flags,
        pub ssf: libc::c_uint,
    }
    #[c2rust::src_loc = "94:1"]
    pub type prf_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes, _: krb5_key,
                                    _: *const krb5_data, _: *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "91:1"]
    pub type rand2key_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_data,
                                    _: *mut krb5_keyblock)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "85:1"]
    pub type str2key_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *const krb5_data,
                                    _: *mut krb5_keyblock)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "80:1"]
    pub type crypt_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes, _: krb5_key,
                                    _: krb5_keyusage, _: *const krb5_data,
                                    _: *mut krb5_crypto_iov, _: size_t)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "77:1"]
    pub type crypto_length_func
        =
        Option<unsafe extern "C" fn(_: *const krb5_keytypes,
                                    _: krb5_cryptotype) -> libc::c_uint>;
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_key, krb5_data, krb5_crypto_iov,
                        krb5_keyblock, krb5_keyusage, krb5_cksumtype,
                        krb5_flags, krb5_boolean, krb5_enctype,
                        krb5_cryptotype};
    use super::k5_int_h::krb5_key_st;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "296:1"]
        pub fn krb5int_aes2_prf(ktp: *const krb5_keytypes, key: krb5_key,
                                in_0: *const krb5_data, out: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "277:1"]
        pub fn k5_rand2key_direct(randombits: *const krb5_data,
                                  keyblock: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "270:1"]
        pub fn krb5int_aes2_string_to_key(enc: *const krb5_keytypes,
                                          string: *const krb5_data,
                                          salt: *const krb5_data,
                                          params: *const krb5_data,
                                          key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "239:1"]
        pub fn krb5int_etm_decrypt(ktp: *const krb5_keytypes, key: krb5_key,
                                   usage: krb5_keyusage,
                                   ivec: *const krb5_data,
                                   data: *mut krb5_crypto_iov,
                                   num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn krb5int_etm_encrypt(ktp: *const krb5_keytypes, key: krb5_key,
                                   usage: krb5_keyusage,
                                   ivec: *const krb5_data,
                                   data: *mut krb5_crypto_iov,
                                   num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "193:1"]
        pub fn krb5int_aes2_crypto_length(ktp: *const krb5_keytypes,
                                          type_0: krb5_cryptotype)
         -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "441:40"]
        pub static krb5int_hash_sha384: krb5_hash_provider;
        #[no_mangle]
        #[c2rust::src_loc = "431:39"]
        pub static krb5int_enc_aes256: krb5_enc_provider;
        #[no_mangle]
        #[c2rust::src_loc = "440:40"]
        pub static krb5int_hash_sha256: krb5_hash_provider;
        #[no_mangle]
        #[c2rust::src_loc = "430:39"]
        pub static krb5int_enc_aes128: krb5_enc_provider;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn krb5int_dk_cmac_prf(ktp: *const krb5_keytypes, key: krb5_key,
                                   in_0: *const krb5_data,
                                   out: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "265:1"]
        pub fn krb5int_camellia_string_to_key(enc: *const krb5_keytypes,
                                              string: *const krb5_data,
                                              salt: *const krb5_data,
                                              params: *const krb5_data,
                                              key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "234:1"]
        pub fn krb5int_dk_cmac_decrypt(ktp: *const krb5_keytypes,
                                       key: krb5_key, usage: krb5_keyusage,
                                       ivec: *const krb5_data,
                                       data: *mut krb5_crypto_iov,
                                       num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn krb5int_dk_cmac_encrypt(ktp: *const krb5_keytypes,
                                       key: krb5_key, usage: krb5_keyusage,
                                       ivec: *const krb5_data,
                                       data: *mut krb5_crypto_iov,
                                       num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "191:1"]
        pub fn krb5int_camellia_crypto_length(ktp: *const krb5_keytypes,
                                              type_0: krb5_cryptotype)
         -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "435:39"]
        pub static krb5int_enc_camellia256: krb5_enc_provider;
        #[no_mangle]
        #[c2rust::src_loc = "434:39"]
        pub static krb5int_enc_camellia128: krb5_enc_provider;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn krb5int_dk_prf(ktp: *const krb5_keytypes, key: krb5_key,
                              in_0: *const krb5_data, out: *mut krb5_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "260:1"]
        pub fn krb5int_aes_string_to_key(enc: *const krb5_keytypes,
                                         string: *const krb5_data,
                                         salt: *const krb5_data,
                                         params: *const krb5_data,
                                         key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "230:1"]
        pub fn krb5int_dk_decrypt(ktp: *const krb5_keytypes, key: krb5_key,
                                  usage: krb5_keyusage,
                                  ivec: *const krb5_data,
                                  data: *mut krb5_crypto_iov,
                                  num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "206:1"]
        pub fn krb5int_dk_encrypt(ktp: *const krb5_keytypes, key: krb5_key,
                                  usage: krb5_keyusage,
                                  ivec: *const krb5_data,
                                  data: *mut krb5_crypto_iov,
                                  num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "189:1"]
        pub fn krb5int_aes_crypto_length(ktp: *const krb5_keytypes,
                                         type_0: krb5_cryptotype)
         -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "439:40"]
        pub static krb5int_hash_sha1: krb5_hash_provider;
        #[no_mangle]
        #[c2rust::src_loc = "288:1"]
        pub fn krb5int_arcfour_prf(ktp: *const krb5_keytypes, key: krb5_key,
                                   in_0: *const krb5_data,
                                   out: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "250:1"]
        pub fn krb5int_arcfour_string_to_key(ktp: *const krb5_keytypes,
                                             string: *const krb5_data,
                                             salt: *const krb5_data,
                                             params: *const krb5_data,
                                             key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "225:1"]
        pub fn krb5int_arcfour_decrypt(ktp: *const krb5_keytypes,
                                       key: krb5_key, usage: krb5_keyusage,
                                       ivec: *const krb5_data,
                                       data: *mut krb5_crypto_iov,
                                       num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "201:1"]
        pub fn krb5int_arcfour_encrypt(ktp: *const krb5_keytypes,
                                       key: krb5_key, usage: krb5_keyusage,
                                       ivec: *const krb5_data,
                                       data: *mut krb5_crypto_iov,
                                       num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "185:1"]
        pub fn krb5int_arcfour_crypto_length(ktp: *const krb5_keytypes,
                                             type_0: krb5_cryptotype)
         -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "438:40"]
        pub static krb5int_hash_md5: krb5_hash_provider;
        #[no_mangle]
        #[c2rust::src_loc = "429:39"]
        pub static krb5int_enc_arcfour: krb5_enc_provider;
        #[no_mangle]
        #[c2rust::src_loc = "281:1"]
        pub fn k5_rand2key_des3(randombits: *const krb5_data,
                                keyblock: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "255:1"]
        pub fn krb5int_dk_string_to_key(enc: *const krb5_keytypes,
                                        string: *const krb5_data,
                                        salt: *const krb5_data,
                                        params: *const krb5_data,
                                        key: *mut krb5_keyblock)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "187:1"]
        pub fn krb5int_dk_crypto_length(ktp: *const krb5_keytypes,
                                        type_0: krb5_cryptotype)
         -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "428:39"]
        pub static krb5int_enc_des3: krb5_enc_provider;
        #[no_mangle]
        #[c2rust::src_loc = "221:1"]
        pub fn krb5int_raw_decrypt(ktp: *const krb5_keytypes, key: krb5_key,
                                   usage: krb5_keyusage,
                                   ivec: *const krb5_data,
                                   data: *mut krb5_crypto_iov,
                                   num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "197:1"]
        pub fn krb5int_raw_encrypt(ktp: *const krb5_keytypes, key: krb5_key,
                                   usage: krb5_keyusage,
                                   ivec: *const krb5_data,
                                   data: *mut krb5_crypto_iov,
                                   num_data: size_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "183:1"]
        pub fn krb5int_raw_crypto_length(ktp: *const krb5_keytypes,
                                         type_0: krb5_cryptotype)
         -> libc::c_uint;
    }
    /* CRYPTO_INT_H */
}
pub use self::types_h::{__uint8_t, __int32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_cksumtype, krb5_keyusage, krb5_cryptotype,
                       krb5_flags, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key};
pub use self::crypto_int_h::{krb5_enc_provider, krb5_cksumtypes, verify_func,
                             checksum_func, krb5_hash_provider, krb5_keytypes,
                             prf_func, rand2key_func, str2key_func,
                             crypt_func, crypto_length_func, krb5int_aes2_prf,
                             k5_rand2key_direct, krb5int_aes2_string_to_key,
                             krb5int_etm_decrypt, krb5int_etm_encrypt,
                             krb5int_aes2_crypto_length, krb5int_hash_sha384,
                             krb5int_enc_aes256, krb5int_hash_sha256,
                             krb5int_enc_aes128, krb5int_dk_cmac_prf,
                             krb5int_camellia_string_to_key,
                             krb5int_dk_cmac_decrypt, krb5int_dk_cmac_encrypt,
                             krb5int_camellia_crypto_length,
                             krb5int_enc_camellia256, krb5int_enc_camellia128,
                             krb5int_dk_prf, krb5int_aes_string_to_key,
                             krb5int_dk_decrypt, krb5int_dk_encrypt,
                             krb5int_aes_crypto_length, krb5int_hash_sha1,
                             krb5int_arcfour_prf,
                             krb5int_arcfour_string_to_key,
                             krb5int_arcfour_decrypt, krb5int_arcfour_encrypt,
                             krb5int_arcfour_crypto_length, krb5int_hash_md5,
                             krb5int_enc_arcfour, k5_rand2key_des3,
                             krb5int_dk_string_to_key,
                             krb5int_dk_crypto_length, krb5int_enc_des3,
                             krb5int_raw_decrypt, krb5int_raw_encrypt,
                             krb5int_raw_crypto_length};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
/* these will be linear searched.  if they ever get big, a binary
   search or hash table would be better, which means these would need
   to be sorted.  An array would be more efficient, but that assumes
   that the keytypes are all near each other.  I'd rather not make
   that assumption. */
/* Deprecations come from RFC 6649 and RFC 8249. */
#[no_mangle]
#[c2rust::src_loc = "37:28"]
pub static mut krb5int_enctypes_list: [krb5_keytypes; 10] =
    unsafe {
        [{
             let mut init =
                 krb5_keytypes{etype: 0x6 as libc::c_int,
                               name:
                                   b"des3-cbc-raw\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [0 as *const libc::c_char as
                                        *mut libc::c_char,
                                    0 as *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"Triple DES cbc mode raw\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_des3 as
                                       *const krb5_enc_provider,
                               hash: 0 as *const krb5_hash_provider,
                               prf_length: 16 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_raw_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_raw_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_raw_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_dk_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_des3 as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf: None,
                               required_ctype: 0 as libc::c_int,
                               flags:
                                   (1 as libc::c_int) << 0 as libc::c_int |
                                       (1 as libc::c_int) << 1 as libc::c_int,
                               ssf: 112 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x10 as libc::c_int,
                               name:
                                   b"des3-cbc-sha1\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"des3-hmac-sha1\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    b"des3-cbc-sha1-kd\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"Triple DES cbc mode with HMAC/sha1\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_des3 as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_sha1 as
                                       *const krb5_hash_provider,
                               prf_length: 16 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_dk_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_dk_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_dk_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_dk_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_des3 as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_dk_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0xc as libc::c_int,
                               flags: (1 as libc::c_int) << 1 as libc::c_int,
                               ssf: 112 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x17 as libc::c_int,
                               name:
                                   b"arcfour-hmac\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"rc4-hmac\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    b"arcfour-hmac-md5\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"ArcFour with HMAC/md5\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_arcfour as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_md5 as
                                       *const krb5_hash_provider,
                               prf_length: 20 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_arcfour_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_arcfour_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_arcfour_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_arcfour_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_arcfour_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: -(138 as libc::c_int),
                               flags: (1 as libc::c_int) << 1 as libc::c_int,
                               ssf: 64 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x18 as libc::c_int,
                               name:
                                   b"arcfour-hmac-exp\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"rc4-hmac-exp\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    b"arcfour-hmac-md5-exp\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"Exportable ArcFour with HMAC/md5\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_arcfour as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_md5 as
                                       *const krb5_hash_provider,
                               prf_length: 20 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_arcfour_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_arcfour_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_arcfour_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_arcfour_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_arcfour_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: -(138 as libc::c_int),
                               flags:
                                   (1 as libc::c_int) << 0 as libc::c_int |
                                       (1 as libc::c_int) << 1 as libc::c_int,
                               ssf: 40 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x11 as libc::c_int,
                               name:
                                   b"aes128-cts-hmac-sha1-96\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"aes128-cts\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    b"aes128-sha1\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"AES-128 CTS mode with 96-bit SHA-1 HMAC\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_aes128 as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_sha1 as
                                       *const krb5_hash_provider,
                               prf_length: 16 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_aes_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_dk_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_dk_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_aes_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_dk_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0xf as libc::c_int,
                               flags: 0 as libc::c_int,
                               ssf: 128 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x12 as libc::c_int,
                               name:
                                   b"aes256-cts-hmac-sha1-96\x00" as *const u8
                                       as *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"aes256-cts\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    b"aes256-sha1\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"AES-256 CTS mode with 96-bit SHA-1 HMAC\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_aes256 as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_sha1 as
                                       *const krb5_hash_provider,
                               prf_length: 16 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_aes_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_dk_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_dk_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_aes_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_dk_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0x10 as libc::c_int,
                               flags: 0 as libc::c_int,
                               ssf: 256 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x19 as libc::c_int,
                               name:
                                   b"camellia128-cts-cmac\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"camellia128-cts\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    0 as *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"Camellia-128 CTS mode with CMAC\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_camellia128 as
                                       *const krb5_enc_provider,
                               hash: 0 as *const krb5_hash_provider,
                               prf_length: 16 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_camellia_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_dk_cmac_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_dk_cmac_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_camellia_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_dk_cmac_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0x11 as libc::c_int,
                               flags: 0 as libc::c_int,
                               ssf: 128 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x1a as libc::c_int,
                               name:
                                   b"camellia256-cts-cmac\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"camellia256-cts\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    0 as *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"Camellia-256 CTS mode with CMAC\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_camellia256 as
                                       *const krb5_enc_provider,
                               hash: 0 as *const krb5_hash_provider,
                               prf_length: 16 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_camellia_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_dk_cmac_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_dk_cmac_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_camellia_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_dk_cmac_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0x12 as libc::c_int,
                               flags: 0 as libc::c_int,
                               ssf: 256 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x13 as libc::c_int,
                               name:
                                   b"aes128-cts-hmac-sha256-128\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"aes128-sha2\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    0 as *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"AES-128 CTS mode with 128-bit SHA-256 HMAC\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_aes128 as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_sha256 as
                                       *const krb5_hash_provider,
                               prf_length: 32 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_aes2_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_etm_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_etm_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_aes2_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_aes2_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0x13 as libc::c_int,
                               flags: 0 as libc::c_int,
                               ssf: 128 as libc::c_int as libc::c_uint,};
             init
         },
         {
             let mut init =
                 krb5_keytypes{etype: 0x14 as libc::c_int,
                               name:
                                   b"aes256-cts-hmac-sha384-192\x00" as
                                       *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               aliases:
                                   [b"aes256-sha2\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char,
                                    0 as *const libc::c_char as
                                        *mut libc::c_char],
                               out_string:
                                   b"AES-256 CTS mode with 192-bit SHA-384 HMAC\x00"
                                       as *const u8 as *const libc::c_char as
                                       *mut libc::c_char,
                               enc:
                                   &krb5int_enc_aes256 as
                                       *const krb5_enc_provider,
                               hash:
                                   &krb5int_hash_sha384 as
                                       *const krb5_hash_provider,
                               prf_length: 48 as libc::c_int as size_t,
                               crypto_length:
                                   Some(krb5int_aes2_crypto_length as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     krb5_cryptotype)
                                                -> libc::c_uint),
                               encrypt:
                                   Some(krb5int_etm_encrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               decrypt:
                                   Some(krb5int_etm_decrypt as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     krb5_keyusage,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_crypto_iov,
                                                                 _: size_t)
                                                -> krb5_error_code),
                               str2key:
                                   Some(krb5int_aes2_string_to_key as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               rand2key:
                                   Some(k5_rand2key_direct as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_keyblock)
                                                -> krb5_error_code),
                               prf:
                                   Some(krb5int_aes2_prf as
                                            unsafe extern "C" fn(_:
                                                                     *const krb5_keytypes,
                                                                 _: krb5_key,
                                                                 _:
                                                                     *const krb5_data,
                                                                 _:
                                                                     *mut krb5_data)
                                                -> krb5_error_code),
                               required_ctype: 0x14 as libc::c_int,
                               flags: 0 as libc::c_int,
                               ssf: 256 as libc::c_int as libc::c_uint,};
             init
         }]
    };
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "151:11"]
pub static mut krb5int_enctypes_length: libc::c_int = 0;
unsafe extern "C" fn run_static_initializers() {
    krb5int_enctypes_length =
        (::std::mem::size_of::<[krb5_keytypes; 10]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<krb5_keytypes>()
                                             as libc::c_ulong) as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
