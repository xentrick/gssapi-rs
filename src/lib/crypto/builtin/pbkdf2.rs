use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/ctype.h:27"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:28"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = __bswap_32(val);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
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
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key, krb5_magic};
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
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_key, krb5_data, krb5_crypto_iov,
                        krb5_keyblock, krb5_keyusage, krb5_enctype,
                        krb5_cksumtype, krb5_flags, krb5_cryptotype,
                        krb5_boolean};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "458:1"]
        pub fn krb5int_hmac_keyblock(hash: *const krb5_hash_provider,
                                     keyblock: *const krb5_keyblock,
                                     data: *const krb5_crypto_iov,
                                     num_data: size_t, output: *mut krb5_data)
         -> krb5_error_code;
    }
    /* CRYPTO_INT_H */
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
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:28"]
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
#[c2rust::header_src = "/usr/include/stdio.h:28"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::ctype_h::{C2RustUnnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank,
                        _ISgraph, _ISprint, _ISspace, _ISxdigit, _ISdigit,
                        _ISalpha, _ISlower, _ISupper, __ctype_b_loc};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed_0, store_32_be};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_cksumtype, krb5_keyusage, krb5_cryptotype,
                       krb5_flags, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key, make_data};
pub use self::crypto_int_h::{krb5_enc_provider, krb5_keytypes, prf_func,
                             rand2key_func, str2key_func, crypt_func,
                             crypto_length_func, krb5_hash_provider,
                             krb5_cksumtypes, verify_func, checksum_func,
                             krb5int_hmac_keyblock};
pub use self::byteswap_h::__bswap_32;
use self::stdlib_h::{abort, free, malloc};
use self::string_h::memcpy;
use self::assert_h::__assert_fail;
use self::stdio_h::printf;
#[c2rust::src_loc = "49:12"]
static mut debug_hmac: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn printd(mut descr: *const libc::c_char,
                            mut d: *mut krb5_data) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let r: libc::c_int = 16 as libc::c_int;
    printf(b"%s:\x00" as *const u8 as *const libc::c_char, descr);
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*d).length {
        printf(b"\n  %04x: \x00" as *const u8 as *const libc::c_char, i);
        j = i;
        while j < i.wrapping_add(r as libc::c_uint) && j < (*d).length {
            printf(b" %02x\x00" as *const u8 as *const libc::c_char,
                   0xff as libc::c_int &
                       *(*d).data.offset(j as isize) as libc::c_int);
            j = j.wrapping_add(1)
        }
        while j < i.wrapping_add(r as libc::c_uint) {
            printf(b"   \x00" as *const u8 as *const libc::c_char);
            j = j.wrapping_add(1)
        }
        printf(b"   \x00" as *const u8 as *const libc::c_char);
        j = i;
        while j < i.wrapping_add(r as libc::c_uint) && j < (*d).length {
            let mut c: libc::c_int =
                0xff as libc::c_int &
                    *(*d).data.offset(j as isize) as libc::c_int;
            printf(b"%c\x00" as *const u8 as *const libc::c_char,
                   if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                          _ISprint as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
                       c
                   } else { '.' as i32 });
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(r as libc::c_uint)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
 * Implements the hmac-sha1 PRF.  pass has been pre-hashed (if
 * necessary) and converted to a key already; salt has had the block
 * index appended to the original salt.
 *
 * NetBSD 8 declares an hmac() function in stdlib.h, so avoid that name.
 */
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn k5_hmac(mut hash: *const krb5_hash_provider,
                             mut pass: *mut krb5_keyblock,
                             mut salt: *mut krb5_data,
                             mut out: *mut krb5_data) -> krb5_error_code {
    let mut err: krb5_error_code = 0;
    let mut iov: krb5_crypto_iov =
        krb5_crypto_iov{flags: 0,
                        data:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},};
    if debug_hmac != 0 {
        printd(b" hmac input\x00" as *const u8 as *const libc::c_char, salt);
    }
    iov.flags = 2 as libc::c_int;
    iov.data = *salt;
    err =
        krb5int_hmac_keyblock(hash, pass, &mut iov,
                              1 as libc::c_int as size_t, out);
    if err == 0 as libc::c_int && debug_hmac != 0 {
        printd(b" hmac output\x00" as *const u8 as *const libc::c_char, out);
    }
    return err;
}
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn F(mut output: *mut libc::c_char,
                       mut u_tmp1: *mut libc::c_char,
                       mut u_tmp2: *mut libc::c_char,
                       mut hash: *const krb5_hash_provider, mut hlen: size_t,
                       mut pass: *mut krb5_keyblock,
                       mut salt: *const krb5_data, mut count: libc::c_ulong,
                       mut i: libc::c_int) -> krb5_error_code {
    let mut ibytes: [libc::c_uchar; 4] = [0; 4];
    let mut j: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut sdata: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut out: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut err: krb5_error_code = 0;
    /* Compute U_1.  */
    store_32_be(i as libc::c_uint, ibytes.as_mut_ptr() as *mut libc::c_void);
    memcpy(u_tmp2 as *mut libc::c_void, (*salt).data as *const libc::c_void,
           (*salt).length as libc::c_ulong);
    memcpy(u_tmp2.offset((*salt).length as isize) as *mut libc::c_void,
           ibytes.as_mut_ptr() as *const libc::c_void,
           4 as libc::c_int as libc::c_ulong);
    sdata =
        make_data(u_tmp2 as *mut libc::c_void,
                  (*salt).length.wrapping_add(4 as libc::c_int as
                                                  libc::c_uint));
    out = make_data(u_tmp1 as *mut libc::c_void, hlen as libc::c_uint);
    err = k5_hmac(hash, pass, &mut sdata, &mut out);
    if err != 0 { return err }
    memcpy(output as *mut libc::c_void, u_tmp1 as *const libc::c_void, hlen);
    /* Compute U_2, .. U_c.  */
    sdata.length = hlen as libc::c_uint;
    j = 2 as libc::c_int as libc::c_uint;
    while j as libc::c_ulong <= count {
        memcpy(u_tmp2 as *mut libc::c_void, u_tmp1 as *const libc::c_void,
               hlen);
        err = k5_hmac(hash, pass, &mut sdata, &mut out);
        if err != 0 { return err }
        /* And xor them together.  */
        k =
            0 as libc::c_int as
                libc::c_uint; /* XXX length shouldn't be hardcoded! */
        while (k as libc::c_ulong) < hlen {
            let ref mut fresh0 = *output.offset(k as isize);
            *fresh0 =
                (*fresh0 as libc::c_int ^
                     *u_tmp1.offset(k as isize) as libc::c_int) as
                    libc::c_char;
            k = k.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "137:1"]
unsafe extern "C" fn pbkdf2(mut hash: *const krb5_hash_provider,
                            mut pass: *mut krb5_keyblock,
                            mut salt: *const krb5_data,
                            mut count: libc::c_ulong,
                            mut output: *const krb5_data) -> krb5_error_code {
    let mut hlen: size_t = (*hash).hashsize;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut utmp1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut utmp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut utmp3: [libc::c_char; 128] = [0; 128];
    if (*output).length == 0 as libc::c_int as libc::c_uint ||
           hlen == 0 as libc::c_int as libc::c_ulong {
        abort();
    }
    /* Step 1 & 2.  */
    if ((*output).length as libc::c_ulong).wrapping_div(hlen) >
           0xffffffff as libc::c_uint as libc::c_ulong {
        abort();
    }
    /* Step 2.  */
    l =
        ((*output).length as
             libc::c_ulong).wrapping_add(hlen).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_div(hlen)
            as libc::c_int;
    utmp1 = malloc(hlen) as *mut libc::c_char;
    if utmp1.is_null() { return 12 as libc::c_int }
    utmp2 =
        malloc(((*salt).length.wrapping_add(4 as libc::c_int as libc::c_uint)
                    as libc::c_ulong).wrapping_add(hlen)) as
            *mut libc::c_char;
    if utmp2.is_null() {
        free(utmp1 as *mut libc::c_void);
        return 12 as libc::c_int
    }
    /* Step 3.  */
    i = 1 as libc::c_int;
    while i <= l {
        let mut err: krb5_error_code = 0;
        let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
        if i == l {
            out = utmp3.as_mut_ptr()
        } else {
            out =
                (*output).data.offset(((i - 1 as libc::c_int) as
                                           libc::c_ulong).wrapping_mul(hlen)
                                          as isize)
        }
        err = F(out, utmp1, utmp2, hash, hlen, pass, salt, count, i);
        if err != 0 {
            free(utmp1 as *mut libc::c_void);
            free(utmp2 as *mut libc::c_void);
            return err
        }
        if i == l {
            memcpy((*output).data.offset(((i - 1 as libc::c_int) as
                                              libc::c_ulong).wrapping_mul(hlen)
                                             as isize) as *mut libc::c_void,
                   utmp3.as_mut_ptr() as *const libc::c_void,
                   ((*output).length as
                        libc::c_ulong).wrapping_sub(((i - 1 as libc::c_int) as
                                                         libc::c_ulong).wrapping_mul(hlen)));
        }
        i += 1
    }
    free(utmp1 as *mut libc::c_void);
    free(utmp2 as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/krb/crypto_int.h - Master libk5crypto internal header */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
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
/* This header is the entry point for libk5crypto sources, and also documents
 * requirements for crypto modules and PRNG modules.  */
/* Enc providers and hash providers specify well-known ciphers and hashes to be
 * implemented by the crypto module. */
/* keybytes is the input size to make_key;
       keylength is the output size */
/* May be NULL if the cipher is not used for a cbc-mac checksum. */
/* May be NULL if there is no key-derived data cached.  */
/* ** RFC 3961 enctypes table ***/
/*
 * "Weak" means the enctype is believed to be vulnerable to practical attacks,
 * and will be disabled unless allow_weak_crypto is set to true.  "Deprecated"
 * means the enctype has been deprecated by the IETF, and affects display and
 * logging.
 */
/* ** RFC 3961 checksum types table ***/
/*
 * Compute a checksum over the header, data, padding, and sign-only fields of
 * the iov array data (of size num_data).  The output buffer will already be
 * allocated with ctp->compute_size bytes available; the handler just needs to
 * fill in the contents.  If ctp->enc is not NULL, the handler can assume that
 * key is a valid-length key of an enctype which uses that enc provider.
 */
/*
 * Verify a checksum over the header, data, padding, and sign-only fields of
 * the iov array data (of size num_data), and store the boolean result in
 * *valid.  The handler can assume that hash has length ctp->output_size.  If
 * ctp->enc is not NULL, the handler can assume that key a valid-length key of
 * an enctype which uses that enc provider.
 */
/* NULL means recompute checksum and compare */
/* Allocation size for checksum computation */
/* Possibly truncated output size */
/* ** Prototypes for enctype table functions ***/
/* Length */
/* Encrypt */
/* Decrypt */
/* String to key */
/* Random to key */
/* Pseudo-random function */
/* ** Prototypes for cksumtype handler functions ***/
/* ** Key derivation functions ***/
/* RFC 3961 section 5.1 */
/* NIST SP 800-108 with CMAC as PRF */
/* NIST SP 800-108 with HMAC as PRF */
/* ** Miscellaneous prototypes ***/
/* nfold algorithm from RFC 3961 */
/* Compute a CMAC checksum over data. */
/* Translate an RFC 3961 key usage to a Microsoft RC4 usage. */
/* Ensure library initialization has occurred. */
/* DES default state initialization handler (used by module enc providers). */
/* Default state cleanup handler (used by module enc providers). */
/* ** Input/output vector processing declarations **/
/* iov array we are iterating over */
/* size of iov array */
/* size of blocks we will be obtaining */
/* should we process SIGN_ONLY blocks */
/* read index into iov array */
/* read index into iov contents */
/* write index into iov array */
/* write index into iov contents */
/* ** Crypto module declarations ***/
/* Modules must implement the k5_sha256() function prototyped in k5-int.h. */
/* Modules must implement the following enc_providers and hash_providers: */
/* Modules must implement the following functions. */
/* Set the parity bits to the correct values in keybits. */
/* Return true if keybits is a weak or semi-weak DES key. */
/* Compute an HMAC using the provided hash function, key, and data, storing the
 * result into output (caller-allocated). */
/* As above, using a keyblock as the key input. */
/*
 * Compute the PBKDF2 (see RFC 2898) of password and salt, with the specified
 * count, using HMAC with the specified hash as the pseudo-random function,
 * storing the result into out (caller-allocated).
 */
#[no_mangle]
#[c2rust::src_loc = "188:1"]
pub unsafe extern "C" fn krb5int_pbkdf2_hmac(mut hash:
                                                 *const krb5_hash_provider,
                                             mut out: *const krb5_data,
                                             mut count: libc::c_ulong,
                                             mut pass: *const krb5_data,
                                             mut salt: *const krb5_data)
 -> krb5_error_code {
    let mut keyblock: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *mut krb5_octet,};
    let mut tmp: [libc::c_char; 128] = [0; 128];
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut iov: krb5_crypto_iov =
        krb5_crypto_iov{flags: 0,
                        data:
                            krb5_data{magic: 0,
                                      length: 0,
                                      data: 0 as *mut libc::c_char,},};
    let mut err: krb5_error_code = 0;
    if (*hash).hashsize <=
           ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {
    } else {
        __assert_fail(b"hash->hashsize <= sizeof(tmp)\x00" as *const u8 as
                          *const libc::c_char,
                      b"pbkdf2.c\x00" as *const u8 as *const libc::c_char,
                      199 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 143],
                                                &[libc::c_char; 143]>(b"krb5_error_code krb5int_pbkdf2_hmac(const struct krb5_hash_provider *, const krb5_data *, unsigned long, const krb5_data *, const krb5_data *)\x00")).as_ptr());
    }
    if (*pass).length as libc::c_ulong > (*hash).blocksize {
        d =
            make_data(tmp.as_mut_ptr() as *mut libc::c_void,
                      (*hash).hashsize as libc::c_uint);
        iov.flags = 2 as libc::c_int;
        iov.data = *pass;
        err =
            (*hash).hash.expect("non-null function pointer")(&mut iov,
                                                             1 as libc::c_int
                                                                 as size_t,
                                                             &mut d);
        if err != 0 { return err }
        keyblock.length = d.length;
        keyblock.contents = d.data as *mut krb5_octet
    } else {
        keyblock.length = (*pass).length;
        keyblock.contents = (*pass).data as *mut krb5_octet
    }
    keyblock.enctype = 0 as libc::c_int;
    err = pbkdf2(hash, &mut keyblock, salt, count, out);
    return err;
}
