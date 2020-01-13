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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:27"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:27"]
pub mod crypto_int_h {
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
    #[c2rust::src_loc = "391:8"]
    pub struct iov_cursor {
        pub iov: *const krb5_crypto_iov,
        pub iov_count: size_t,
        pub block_size: size_t,
        pub signing: krb5_boolean,
        pub in_iov: size_t,
        pub in_pos: size_t,
        pub out_iov: size_t,
        pub out_pos: size_t,
    }
    #[inline]
    #[c2rust::src_loc = "581:1"]
    pub unsafe extern "C" fn encrypt_block(mut enc: *const krb5_enc_provider,
                                           mut key: krb5_key,
                                           mut block: *mut krb5_data)
     -> krb5_error_code {
        let mut iov: krb5_crypto_iov =
            krb5_crypto_iov{flags: 0,
                            data:
                                krb5_data{magic: 0,
                                          length: 0,
                                          data: 0 as *mut libc::c_char,},};
        if (*block).length as libc::c_ulong != (*enc).block_size ||
               (*enc).block_size == 1 as libc::c_int as libc::c_ulong {
            return 22 as libc::c_int
        }
        iov.flags = 2 as libc::c_int;
        iov.data = *block;
        if (*enc).cbc_mac.is_some() {
            return (*enc).cbc_mac.expect("non-null function pointer")(key,
                                                                      &mut iov,
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          size_t,
                                                                      0 as
                                                                          *const krb5_data,
                                                                      block)
        } else {
            return (*enc).encrypt.expect("non-null function pointer")(key,
                                                                      0 as
                                                                          *const krb5_data,
                                                                      &mut iov,
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          size_t)
        };
    }
    #[inline]
    #[c2rust::src_loc = "600:1"]
    pub unsafe extern "C" fn iov_total_length(mut data:
                                                  *const krb5_crypto_iov,
                                              mut num_data: size_t,
                                              mut signing: krb5_boolean)
     -> size_t {
        let mut i: size_t = 0;
        let mut total: size_t = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < num_data {
            if if signing != 0 {
                   ((*data.offset(i as isize)).flags == 1 as libc::c_int ||
                        ((*data.offset(i as isize)).flags == 2 as libc::c_int
                             ||
                             (*data.offset(i as isize)).flags ==
                                 4 as libc::c_int) ||
                        (*data.offset(i as isize)).flags == 3 as libc::c_int)
                       as libc::c_int
               } else {
                   ((*data.offset(i as isize)).flags == 1 as libc::c_int ||
                        ((*data.offset(i as isize)).flags == 2 as libc::c_int
                             ||
                             (*data.offset(i as isize)).flags ==
                                 4 as libc::c_int)) as libc::c_int
               } != 0 {
                total =
                    (total as
                         libc::c_ulong).wrapping_add((*data.offset(i as
                                                                       isize)).data.length
                                                         as libc::c_ulong) as
                        size_t as size_t
            }
            i = i.wrapping_add(1)
        }
        return total;
    }
    use super::krb5_h::{krb5_enctype, krb5_cksumtype, krb5_flags,
                        krb5_error_code, krb5_key, krb5_data, krb5_keyblock,
                        krb5_keyusage, krb5_crypto_iov, krb5_cryptotype,
                        krb5_boolean};
    use super::stddef_h::size_t;
    use super::k5_int_h::krb5_key_st;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn k5_iov_cursor_get(cursor: *mut iov_cursor,
                                 block: *mut libc::c_uchar) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn k5_iov_cursor_init(cursor: *mut iov_cursor,
                                  iov: *const krb5_crypto_iov, count: size_t,
                                  block_size: size_t, signing: krb5_boolean);
    }
    /* CRYPTO_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:27"]
pub mod k5_platform_h {
    #[inline]
    #[c2rust::src_loc = "732:1"]
    pub unsafe extern "C" fn store_32_n(mut val: libc::c_uint,
                                        mut vp: *mut libc::c_void) {
        let mut n: uint32_t = val;
        memcpy(vp, &mut n as *mut uint32_t as *const libc::c_void,
               4 as libc::c_int as libc::c_ulong);
    }
    #[inline]
    #[c2rust::src_loc = "751:1"]
    pub unsafe extern "C" fn load_32_n(mut p: *const libc::c_void)
     -> libc::c_uint {
        let mut n: uint32_t = 0;
        memcpy(&mut n as *mut uint32_t as *mut libc::c_void, p,
               4 as libc::c_int as libc::c_ulong);
        return n;
    }
    use super::stdint_uintn_h::uint32_t;
    use super::string_h::memcpy;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:27"]
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
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_cksumtype, krb5_keyusage, krb5_cryptotype,
                       krb5_flags, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key, make_data};
pub use self::crypto_int_h::{krb5_keytypes, prf_func, rand2key_func,
                             str2key_func, crypt_func, crypto_length_func,
                             krb5_hash_provider, krb5_enc_provider,
                             krb5_cksumtypes, verify_func, checksum_func,
                             iov_cursor, encrypt_block, iov_total_length,
                             k5_iov_cursor_get, k5_iov_cursor_init};
pub use self::k5_platform_h::{store_32_n, load_32_n};
use self::string_h::{memset, memcpy};
use self::assert_h::__assert_fail;
#[c2rust::src_loc = "31:22"]
static mut const_Rb: [libc::c_uchar; 16] =
    [0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0x87 as libc::c_int as libc::c_uchar];
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn xor_128(mut a: *mut libc::c_uchar,
                             mut b: *mut libc::c_uchar,
                             mut out: *mut libc::c_uchar) {
    let mut z: libc::c_int = 0;
    z = 0 as libc::c_int;
    while z < 16 as libc::c_int / 4 as libc::c_int {
        let mut aptr: *mut libc::c_uchar =
            &mut *a.offset((z * 4 as libc::c_int) as isize) as
                *mut libc::c_uchar;
        let mut bptr: *mut libc::c_uchar =
            &mut *b.offset((z * 4 as libc::c_int) as isize) as
                *mut libc::c_uchar;
        let mut outptr: *mut libc::c_uchar =
            &mut *out.offset((z * 4 as libc::c_int) as isize) as
                *mut libc::c_uchar;
        store_32_n(load_32_n(aptr as *const libc::c_void) ^
                       load_32_n(bptr as *const libc::c_void),
                   outptr as *mut libc::c_void);
        z += 1
    };
}
#[c2rust::src_loc = "50:1"]
unsafe extern "C" fn leftshift_onebit(mut input: *mut libc::c_uchar,
                                      mut output: *mut libc::c_uchar) {
    let mut i: libc::c_int = 0;
    let mut overflow: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    i = 16 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        *output.offset(i as isize) =
            ((*input.offset(i as isize) as libc::c_int) << 1 as libc::c_int)
                as libc::c_uchar;
        let ref mut fresh0 = *output.offset(i as isize);
        *fresh0 =
            (*fresh0 as libc::c_int | overflow as libc::c_int) as
                libc::c_uchar;
        overflow =
            if *input.offset(i as isize) as libc::c_int & 0x80 as libc::c_int
                   != 0 {
                1 as libc::c_int
            } else { 0 as libc::c_int } as libc::c_uchar;
        i -= 1
    };
}
/* Generate subkeys K1 and K2 as described in RFC 4493 figure 2.2. */
#[c2rust::src_loc = "64:1"]
unsafe extern "C" fn generate_subkey(mut enc: *const krb5_enc_provider,
                                     mut key: krb5_key,
                                     mut K1: *mut libc::c_uchar,
                                     mut K2: *mut libc::c_uchar)
 -> krb5_error_code {
    let mut L: [libc::c_uchar; 16] = [0; 16];
    let mut tmp: [libc::c_uchar; 16] = [0; 16];
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut ret: krb5_error_code = 0;
    /* L := encrypt(K, const_Zero) */
    memset(L.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong);
    d =
        make_data(L.as_mut_ptr() as *mut libc::c_void,
                  16 as libc::c_int as libc::c_uint);
    ret = encrypt_block(enc, key, &mut d);
    if ret != 0 as libc::c_int { return ret }
    /* K1 := (MSB(L) == 0) ? L << 1 : (L << 1) XOR const_Rb */
    if L[0 as libc::c_int as usize] as libc::c_int & 0x80 as libc::c_int ==
           0 as libc::c_int {
        leftshift_onebit(L.as_mut_ptr(), K1);
    } else {
        leftshift_onebit(L.as_mut_ptr(), tmp.as_mut_ptr());
        xor_128(tmp.as_mut_ptr(), const_Rb.as_mut_ptr(), K1);
    }
    /* K2 := (MSB(K1) == 0) ? K1 << 1 : (K1 << 1) XOR const_Rb */
    if *K1.offset(0 as libc::c_int as isize) as libc::c_int &
           0x80 as libc::c_int == 0 as libc::c_int {
        leftshift_onebit(K1, K2);
    } else {
        leftshift_onebit(K1, tmp.as_mut_ptr());
        xor_128(tmp.as_mut_ptr(), const_Rb.as_mut_ptr(), K2);
    }
    return 0 as libc::c_int;
}
/* Pad out lastb with a 1 bit followed by 0 bits, placing the result in pad. */
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn padding(mut lastb: *mut libc::c_uchar,
                             mut pad: *mut libc::c_uchar,
                             mut length: libc::c_int) {
    let mut j: libc::c_int = 0;
    /* original last block */
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        if j < length {
            *pad.offset(j as isize) = *lastb.offset(j as isize)
        } else if j == length {
            *pad.offset(j as isize) = 0x80 as libc::c_int as libc::c_uchar
        } else { *pad.offset(j as isize) = 0 as libc::c_int as libc::c_uchar }
        j += 1
    };
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
/*
 * Implementation of CMAC algorithm. When used with AES, this function
 * is compatible with RFC 4493 figure 2.3.
 */
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn krb5int_cmac_checksum(mut enc:
                                                   *const krb5_enc_provider,
                                               mut key: krb5_key,
                                               mut data:
                                                   *const krb5_crypto_iov,
                                               mut num_data: size_t,
                                               mut output: *mut krb5_data)
 -> krb5_error_code {
    let mut Y: [libc::c_uchar; 16] = [0; 16];
    let mut M_last: [libc::c_uchar; 16] = [0; 16];
    let mut padded: [libc::c_uchar; 16] = [0; 16];
    let mut K1: [libc::c_uchar; 16] = [0; 16];
    let mut K2: [libc::c_uchar; 16] = [0; 16];
    let mut input: [libc::c_uchar; 16] = [0; 16];
    let mut n: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut flag: libc::c_uint = 0;
    let mut ret: krb5_error_code = 0;
    let mut cursor: iov_cursor =
        iov_cursor{iov: 0 as *const krb5_crypto_iov,
                   iov_count: 0,
                   block_size: 0,
                   signing: 0,
                   in_iov: 0,
                   in_pos: 0,
                   out_iov: 0,
                   out_pos: 0,};
    let mut length: size_t = 0;
    let mut iov: [krb5_crypto_iov; 1] =
        [krb5_crypto_iov{flags: 0,
                         data:
                             krb5_data{magic: 0,
                                       length: 0,
                                       data: 0 as *mut libc::c_char,},}; 1];
    let mut d: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    if (*enc).cbc_mac.is_some() {
    } else {
        __assert_fail(b"enc->cbc_mac != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"cmac.c\x00" as *const u8 as *const libc::c_char,
                      138 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 128],
                                                &[libc::c_char; 128]>(b"krb5_error_code krb5int_cmac_checksum(const struct krb5_enc_provider *, krb5_key, const krb5_crypto_iov *, size_t, krb5_data *)\x00")).as_ptr());
    }
    if (*enc).block_size != 16 as libc::c_int as libc::c_ulong {
        return -(1765328194 as libc::c_long) as krb5_error_code
    }
    length =
        iov_total_length(data, num_data, 1 as libc::c_int as krb5_boolean);
    /* Step 1. */
    ret = generate_subkey(enc, key, K1.as_mut_ptr(), K2.as_mut_ptr());
    if ret != 0 as libc::c_int { return ret }
    /* Step 2. */
    n =
        length.wrapping_add(16 as libc::c_int as
                                libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_ulong).wrapping_div(16
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_ulong)
            as libc::c_uint;
    /* Step 3. */
    if n == 0 as libc::c_int as libc::c_uint {
        n = 1 as libc::c_int as libc::c_uint;
        flag = 0 as libc::c_int as libc::c_uint
    } else {
        flag =
            (length.wrapping_rem(16 as libc::c_int as libc::c_ulong) ==
                 0 as libc::c_int as libc::c_ulong) as libc::c_int as
                libc::c_uint
    }
    iov[0 as libc::c_int as usize].flags = 2 as libc::c_int;
    iov[0 as libc::c_int as usize].data =
        make_data(input.as_mut_ptr() as *mut libc::c_void,
                  16 as libc::c_int as libc::c_uint);
    /* Step 5 (we'll do step 4 in a bit). */
    memset(Y.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           16 as libc::c_int as libc::c_ulong);
    d =
        make_data(Y.as_mut_ptr() as *mut libc::c_void,
                  16 as libc::c_int as libc::c_uint);
    /* Step 6 (all but last block). */
    k5_iov_cursor_init(&mut cursor, data, num_data,
                       16 as libc::c_int as size_t,
                       1 as libc::c_int as krb5_boolean);
    i = 0 as libc::c_int as libc::c_uint;
    while i < n.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        k5_iov_cursor_get(&mut cursor, input.as_mut_ptr());
        ret =
            (*enc).cbc_mac.expect("non-null function pointer")(key,
                                                               iov.as_mut_ptr(),
                                                               1 as
                                                                   libc::c_int
                                                                   as size_t,
                                                               &mut d,
                                                               &mut d);
        if ret != 0 as libc::c_int { return ret }
        i = i.wrapping_add(1)
    }
    /* Step 4. */
    k5_iov_cursor_get(&mut cursor, input.as_mut_ptr());
    if flag != 0 {
        /* last block is complete block */
        xor_128(input.as_mut_ptr(), K1.as_mut_ptr(), M_last.as_mut_ptr());
    } else {
        padding(input.as_mut_ptr(), padded.as_mut_ptr(),
                length.wrapping_rem(16 as libc::c_int as libc::c_ulong) as
                    libc::c_int);
        xor_128(padded.as_mut_ptr(), K2.as_mut_ptr(), M_last.as_mut_ptr());
    }
    /* Step 6 (last block). */
    iov[0 as libc::c_int as usize].data =
        make_data(M_last.as_mut_ptr() as *mut libc::c_void,
                  16 as libc::c_int as libc::c_uint);
    ret =
        (*enc).cbc_mac.expect("non-null function pointer")(key,
                                                           iov.as_mut_ptr(),
                                                           1 as libc::c_int as
                                                               size_t, &mut d,
                                                           &mut d);
    if ret != 0 as libc::c_int { return ret }
    if (*output).length >= d.length {
    } else {
        __assert_fail(b"output->length >= d.length\x00" as *const u8 as
                          *const libc::c_char,
                      b"cmac.c\x00" as *const u8 as *const libc::c_char,
                      194 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 128],
                                                &[libc::c_char; 128]>(b"krb5_error_code krb5int_cmac_checksum(const struct krb5_enc_provider *, krb5_key, const krb5_crypto_iov *, size_t, krb5_data *)\x00")).as_ptr());
    }
    (*output).length = d.length;
    memcpy((*output).data as *mut libc::c_void, d.data as *const libc::c_void,
           d.length as libc::c_ulong);
    return 0 as libc::c_int;
}
