use ::libc;
use ::c2rust_asm_casts;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
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
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
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
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    use super::krb5_h::{krb5_keyblock, krb5_data, krb5_key};
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:27"]
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
    /* Return the total length of the to-be-signed or to-be-encrypted buffers in an
 * iov chain. */
    /*
 * Return the number of contiguous blocks available within the current input
 * IOV of the cursor c, so that the caller can do in-place encryption.
 * Do not call if c might be exhausted.
 */
    /* Return the current input pointer within the cursor c.  Do not call if c
 * might be exhausted. */
    /*
 * Advance the input and output pointers of c by nblocks blocks.  nblocks must
 * not be greater than the return value of iov_cursor_contig_blocks, and the
 * input and output positions must be identical.
 */
    #[inline]
    #[c2rust::src_loc = "637:1"]
    pub unsafe extern "C" fn iov_cursor_advance(mut c: *mut iov_cursor,
                                                mut nblocks: size_t) {
        (*c).in_pos =
            ((*c).in_pos as
                 libc::c_ulong).wrapping_add(nblocks.wrapping_mul((*c).block_size))
                as size_t as size_t;
        (*c).out_pos =
            ((*c).out_pos as
                 libc::c_ulong).wrapping_add(nblocks.wrapping_mul((*c).block_size))
                as size_t as size_t;
    }
    #[inline]
    #[c2rust::src_loc = "626:1"]
    pub unsafe extern "C" fn iov_cursor_ptr(mut c: *mut iov_cursor)
     -> *mut libc::c_uchar {
        return &mut *(*(*c).iov.offset((*c).in_iov as
                                           isize)).data.data.offset((*c).in_pos
                                                                        as
                                                                        isize)
                   as *mut libc::c_char as *mut libc::c_uchar;
    }
    #[inline]
    #[c2rust::src_loc = "618:1"]
    pub unsafe extern "C" fn iov_cursor_contig_blocks(mut c: *mut iov_cursor)
     -> size_t {
        return ((*(*c).iov.offset((*c).in_iov as isize)).data.length as
                    libc::c_ulong).wrapping_sub((*c).in_pos).wrapping_div((*c).block_size);
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
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_key, krb5_data, krb5_crypto_iov,
                        krb5_keyblock, krb5_keyusage, krb5_cksumtype,
                        krb5_flags, krb5_boolean, krb5_enctype,
                        krb5_cryptotype};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn krb5int_default_free_state(state: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "415:1"]
        pub fn k5_iov_cursor_init(cursor: *mut iov_cursor,
                                  iov: *const krb5_crypto_iov, count: size_t,
                                  block_size: size_t, signing: krb5_boolean);
        #[no_mangle]
        #[c2rust::src_loc = "418:1"]
        pub fn k5_iov_cursor_get(cursor: *mut iov_cursor,
                                 block: *mut libc::c_uchar) -> krb5_boolean;
        #[no_mangle]
        #[c2rust::src_loc = "421:1"]
        pub fn k5_iov_cursor_put(cursor: *mut iov_cursor,
                                 block: *mut libc::c_uchar);
    }
    /* CRYPTO_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/aes/aes.h:27"]
pub mod aes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub struct aes_ctx {
        pub k_sch: [uint32_t; 64],
        pub n_rnd: uint32_t,
        pub n_blk: uint32_t,
    }
    #[c2rust::src_loc = "58:1"]
    pub type aes_fret = uint16_t;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    extern "C" {
        /* for Kerberos 5 tree -- hide names!  */
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn krb5int_aes_dec_blk(in_blk: *const libc::c_uchar,
                                   out_blk: *mut libc::c_uchar,
                                   cx: *const aes_ctx) -> aes_fret;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn krb5int_aes_dec_key(in_key: *const libc::c_uchar,
                                   klen: libc::c_uint, cx: *mut aes_ctx)
         -> aes_fret;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn krb5int_aes_enc_blk(in_blk: *const libc::c_uchar,
                                   out_blk: *mut libc::c_uchar,
                                   cx: *const aes_ctx) -> aes_fret;
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn krb5int_aes_enc_key(in_key: *const libc::c_uchar,
                                   klen: libc::c_uint, cx: *mut aes_ctx)
         -> aes_fret;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
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
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
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
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/cpuid.h:47"]
pub mod cpuid_h {
    #[inline]
    #[c2rust::src_loc = "247:1"]
    pub unsafe extern "C" fn __get_cpuid_max(mut __leaf: libc::c_uint,
                                             mut __sig: *mut libc::c_uint)
     -> libc::c_int {
        let mut __eax: libc::c_uint = 0;
        let mut __ebx: libc::c_uint = 0;
        let mut __ecx: libc::c_uint = 0;
        let mut __edx: libc::c_uint = 0;
        let fresh0 = &mut __eax;
        let fresh1;
        let fresh2 = __leaf;
        asm!("  xchgq  %rbx,${1:q}\n  cpuid\n  xchgq  %rbx,${1:q}" : "={ax}"
             (fresh1), "=r" (__ebx), "={cx}" (__ecx), "={dx}" (__edx) : "0"
             (c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        if !__sig.is_null() { *__sig = __ebx }
        return __eax as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "277:1"]
    pub unsafe extern "C" fn __get_cpuid(mut __leaf: libc::c_uint,
                                         mut __eax: *mut libc::c_uint,
                                         mut __ebx: *mut libc::c_uint,
                                         mut __ecx: *mut libc::c_uint,
                                         mut __edx: *mut libc::c_uint)
     -> libc::c_int {
        let mut __max_leaf: libc::c_uint =
            __get_cpuid_max(__leaf & 0x80000000 as libc::c_uint,
                            0 as *mut libc::c_uint) as libc::c_uint;
        if __max_leaf == 0 as libc::c_int as libc::c_uint ||
               __max_leaf < __leaf {
            return 0 as libc::c_int
        }
        let fresh3 = &mut *__eax;
        let fresh4;
        let fresh5 = __leaf;
        asm!("  xchgq  %rbx,${1:q}\n  cpuid\n  xchgq  %rbx,${1:q}" : "={ax}"
             (fresh4), "=r" (*__ebx), "={cx}" (*__ecx), "={dx}" (*__edx) : "0"
             (c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5)) :);
        c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        return 1 as libc::c_int;
    }
    use c2rust_asm_casts::AsmCastTrait;
}
pub use self::types_h::{__uint8_t, __uint16_t, __int32_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_boolean, krb5_enctype,
                       krb5_cksumtype, krb5_keyusage, krb5_cryptotype,
                       krb5_flags, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, _krb5_keyblock, krb5_keyblock, krb5_key,
                       _krb5_crypto_iov, krb5_crypto_iov};
pub use self::k5_int_h::{krb5_key_st, derived_key, zapfree};
pub use self::crypto_int_h::{krb5_enc_provider, krb5_cksumtypes, verify_func,
                             checksum_func, krb5_hash_provider, krb5_keytypes,
                             prf_func, rand2key_func, str2key_func,
                             crypt_func, crypto_length_func, iov_cursor,
                             iov_cursor_advance, iov_cursor_ptr,
                             iov_cursor_contig_blocks, iov_total_length,
                             krb5int_default_free_state, k5_iov_cursor_init,
                             k5_iov_cursor_get, k5_iov_cursor_put};
pub use self::aes_h::{aes_ctx, aes_fret, krb5int_aes_dec_blk,
                      krb5int_aes_dec_key, krb5int_aes_enc_blk,
                      krb5int_aes_enc_key};
use self::stdlib_h::{malloc, free, abort};
pub use self::k5_platform_h::{store_32_n, load_32_n};
use self::string_h::{explicit_bzero, memset, memcpy};
use self::assert_h::__assert_fail;
pub use self::cpuid_h::{__get_cpuid_max, __get_cpuid};
extern "C" {
    #[no_mangle]
    #[c2rust::src_loc = "58:1"]
    pub fn k5_iEncExpandKey128(key: *mut libc::c_uchar,
                               expanded_key: *mut uint32_t);
    #[no_mangle]
    #[c2rust::src_loc = "59:1"]
    pub fn k5_iEncExpandKey256(key: *mut libc::c_uchar,
                               expanded_key: *mut uint32_t);
    #[no_mangle]
    #[c2rust::src_loc = "60:1"]
    pub fn k5_iDecExpandKey256(key: *mut libc::c_uchar,
                               expanded_key: *mut uint32_t);
    #[no_mangle]
    #[c2rust::src_loc = "61:1"]
    pub fn k5_iDecExpandKey128(key: *mut libc::c_uchar,
                               expanded_key: *mut uint32_t);
    #[no_mangle]
    #[c2rust::src_loc = "63:1"]
    pub fn k5_iEnc128_CBC(data: *mut aes_data);
    #[no_mangle]
    #[c2rust::src_loc = "64:1"]
    pub fn k5_iDec128_CBC(data: *mut aes_data);
    #[no_mangle]
    #[c2rust::src_loc = "65:1"]
    pub fn k5_iEnc256_CBC(data: *mut aes_data);
    #[no_mangle]
    #[c2rust::src_loc = "66:1"]
    pub fn k5_iDec256_CBC(data: *mut aes_data);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/enc_provider/aes.c */
/*
 * Copyright (C) 2003, 2007, 2008 by the Massachusetts Institute of Technology.
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
 * Private per-key data to cache after first generation.  We don't
 * want to mess with the imported AES implementation too much, so
 * we'll just use two copies of its context, one for encryption and
 * one for decryption, and use the #rounds field as a flag for whether
 * we've initialized each half.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "37:8"]
pub struct aes_key_info_cache {
    pub enc_ctx: aes_ctx,
    pub dec_ctx: aes_ctx,
    pub aesni: krb5_boolean,
}
/* Use AES-NI instructions (via assembly functions) when possible. */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "49:8"]
pub struct aes_data {
    pub in_block: *mut libc::c_uchar,
    pub out_block: *mut libc::c_uchar,
    pub expanded_key: *mut uint32_t,
    pub iv: *mut libc::c_uchar,
    pub num_blocks: size_t,
}
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn aesni_supported_by_cpu() -> krb5_boolean {
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    return (__get_cpuid(1 as libc::c_int as libc::c_uint, &mut a, &mut b,
                        &mut c, &mut d) != 0 &&
                c & ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_uint
                    != 0) as libc::c_int as krb5_boolean;
}
#[inline]
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn aesni_supported(mut key: krb5_key) -> krb5_boolean {
    return (*((*key).cache as *mut aes_key_info_cache)).aesni;
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn aesni_expand_enc_key(mut key: krb5_key) {
    let mut cache: *mut aes_key_info_cache =
        (*key).cache as *mut aes_key_info_cache;
    if (*key).keyblock.length == 16 as libc::c_int as libc::c_uint {
        k5_iEncExpandKey128((*key).keyblock.contents,
                            (*cache).enc_ctx.k_sch.as_mut_ptr());
    } else {
        k5_iEncExpandKey256((*key).keyblock.contents,
                            (*cache).enc_ctx.k_sch.as_mut_ptr());
    }
    (*cache).enc_ctx.n_rnd = 1 as libc::c_int as uint32_t;
}
#[c2rust::src_loc = "94:1"]
unsafe extern "C" fn aesni_expand_dec_key(mut key: krb5_key) {
    let mut cache: *mut aes_key_info_cache =
        (*key).cache as *mut aes_key_info_cache;
    if (*key).keyblock.length == 16 as libc::c_int as libc::c_uint {
        k5_iDecExpandKey128((*key).keyblock.contents,
                            (*cache).dec_ctx.k_sch.as_mut_ptr());
    } else {
        k5_iDecExpandKey256((*key).keyblock.contents,
                            (*cache).dec_ctx.k_sch.as_mut_ptr());
    }
    (*cache).dec_ctx.n_rnd = 1 as libc::c_int as uint32_t;
}
#[inline]
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn aesni_enc(mut key: krb5_key,
                               mut data: *mut libc::c_uchar,
                               mut nblocks: size_t,
                               mut iv: *mut libc::c_uchar) {
    let mut cache: *mut aes_key_info_cache =
        (*key).cache as *mut aes_key_info_cache;
    let mut d: aes_data =
        aes_data{in_block: 0 as *mut libc::c_uchar,
                 out_block: 0 as *mut libc::c_uchar,
                 expanded_key: 0 as *mut uint32_t,
                 iv: 0 as *mut libc::c_uchar,
                 num_blocks: 0,};
    d.in_block = data;
    d.out_block = data;
    d.expanded_key = (*cache).enc_ctx.k_sch.as_mut_ptr();
    d.iv = iv;
    d.num_blocks = nblocks;
    if (*key).keyblock.length == 16 as libc::c_int as libc::c_uint {
        k5_iEnc128_CBC(&mut d);
    } else { k5_iEnc256_CBC(&mut d); };
}
#[inline]
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn aesni_dec(mut key: krb5_key,
                               mut data: *mut libc::c_uchar,
                               mut nblocks: size_t,
                               mut iv: *mut libc::c_uchar) {
    let mut cache: *mut aes_key_info_cache =
        (*key).cache as *mut aes_key_info_cache;
    let mut d: aes_data =
        aes_data{in_block: 0 as *mut libc::c_uchar,
                 out_block: 0 as *mut libc::c_uchar,
                 expanded_key: 0 as *mut uint32_t,
                 iv: 0 as *mut libc::c_uchar,
                 num_blocks: 0,};
    d.in_block = data;
    d.out_block = data;
    d.expanded_key = (*cache).dec_ctx.k_sch.as_mut_ptr();
    d.iv = iv;
    d.num_blocks = nblocks;
    if (*key).keyblock.length == 16 as libc::c_int as libc::c_uint {
        k5_iDec128_CBC(&mut d);
    } else { k5_iDec256_CBC(&mut d); };
}
/* not AESNI */
/* out = out ^ in */
#[inline]
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn xorblock(mut in_0: *const libc::c_uchar,
                              mut out: *mut libc::c_uchar) {
    let mut q: size_t = 0;
    q = 0 as libc::c_int as size_t;
    while q < 16 as libc::c_int as libc::c_ulong {
        store_32_n(load_32_n(out.offset(q as isize) as *const libc::c_void) ^
                       load_32_n(in_0.offset(q as isize) as
                                     *const libc::c_void),
                   out.offset(q as isize) as *mut libc::c_void);
        q =
            (q as
                 libc::c_ulong).wrapping_add(4 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    };
}
#[inline]
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn init_key_cache(mut key: krb5_key) -> krb5_error_code {
    if !(*key).cache.is_null() { return 0 as libc::c_int }
    (*key).cache =
        malloc(::std::mem::size_of::<aes_key_info_cache>() as libc::c_ulong);
    if (*key).cache.is_null() { return 12 as libc::c_int }
    let ref mut fresh6 =
        (*((*key).cache as *mut aes_key_info_cache)).dec_ctx.n_rnd;
    *fresh6 = 0 as libc::c_int as uint32_t;
    (*((*key).cache as *mut aes_key_info_cache)).enc_ctx.n_rnd = *fresh6;
    (*((*key).cache as *mut aes_key_info_cache)).aesni =
        aesni_supported_by_cpu();
    return 0 as libc::c_int;
}
#[inline]
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn expand_enc_key(mut key: krb5_key) {
    if (*((*key).cache as *mut aes_key_info_cache)).enc_ctx.n_rnd != 0 {
        return
    }
    if aesni_supported(key) != 0 {
        aesni_expand_enc_key(key);
    } else if krb5int_aes_enc_key((*key).keyblock.contents as
                                      *const libc::c_uchar,
                                  (*key).keyblock.length,
                                  &mut (*((*key).cache as
                                              *mut aes_key_info_cache)).enc_ctx)
                  as libc::c_int != 1 as libc::c_int {
        abort();
    };
}
#[inline]
#[c2rust::src_loc = "186:1"]
unsafe extern "C" fn expand_dec_key(mut key: krb5_key) {
    if (*((*key).cache as *mut aes_key_info_cache)).dec_ctx.n_rnd != 0 {
        return
    }
    if aesni_supported(key) != 0 {
        aesni_expand_dec_key(key);
    } else if krb5int_aes_dec_key((*key).keyblock.contents as
                                      *const libc::c_uchar,
                                  (*key).keyblock.length,
                                  &mut (*((*key).cache as
                                              *mut aes_key_info_cache)).dec_ctx)
                  as libc::c_int != 1 as libc::c_int {
        abort();
    };
}
/* CBC encrypt nblocks blocks of data in place, using and updating iv. */
#[inline]
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn cbc_enc(mut key: krb5_key, mut data: *mut libc::c_uchar,
                             mut nblocks: size_t,
                             mut iv: *mut libc::c_uchar) {
    if aesni_supported(key) != 0 { aesni_enc(key, data, nblocks, iv); return }
    while nblocks > 0 as libc::c_int as libc::c_ulong {
        xorblock(iv, data);
        if krb5int_aes_enc_blk(data as *const libc::c_uchar, data,
                               &mut (*((*key).cache as
                                           *mut aes_key_info_cache)).enc_ctx
                                   as *mut aes_ctx as *const aes_ctx) as
               libc::c_int != 1 as libc::c_int {
            abort();
        }
        memcpy(iv as *mut libc::c_void, data as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
        nblocks = nblocks.wrapping_sub(1);
        data = data.offset(16 as libc::c_int as isize)
    };
}
/* CBC decrypt nblocks blocks of data in place, using and updating iv. */
#[inline]
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn cbc_dec(mut key: krb5_key, mut data: *mut libc::c_uchar,
                             mut nblocks: size_t,
                             mut iv: *mut libc::c_uchar) {
    let mut last_cipherblock: [libc::c_uchar; 16] = [0; 16];
    if aesni_supported(key) != 0 { aesni_dec(key, data, nblocks, iv); return }
    if nblocks > 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(b"nblocks > 0\x00" as *const u8 as *const libc::c_char,
                      b"aes.c\x00" as *const u8 as *const libc::c_char,
                      224 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 65],
                                                &[libc::c_char; 65]>(b"void cbc_dec(krb5_key, unsigned char *, size_t, unsigned char *)\x00")).as_ptr());
    }
    data =
        data.offset(nblocks.wrapping_sub(1 as libc::c_int as
                                             libc::c_ulong).wrapping_mul(16 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                        as isize);
    memcpy(last_cipherblock.as_mut_ptr() as *mut libc::c_void,
           data as *const libc::c_void, 16 as libc::c_int as libc::c_ulong);
    while nblocks > 0 as libc::c_int as libc::c_ulong {
        if krb5int_aes_dec_blk(data as *const libc::c_uchar, data,
                               &mut (*((*key).cache as
                                           *mut aes_key_info_cache)).dec_ctx
                                   as *mut aes_ctx as *const aes_ctx) as
               libc::c_int != 1 as libc::c_int {
            abort();
        }
        xorblock(if nblocks == 1 as libc::c_int as libc::c_ulong {
                     iv
                 } else { data.offset(-(16 as libc::c_int as isize)) }, data);
        nblocks = nblocks.wrapping_sub(1);
        data = data.offset(-(16 as libc::c_int as isize))
    }
    memcpy(iv as *mut libc::c_void,
           last_cipherblock.as_mut_ptr() as *const libc::c_void,
           16 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "235:1"]
pub unsafe extern "C" fn krb5int_aes_encrypt(mut key: krb5_key,
                                             mut ivec: *const krb5_data,
                                             mut data: *mut krb5_crypto_iov,
                                             mut num_data: size_t)
 -> krb5_error_code {
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut block: [libc::c_uchar; 16] = [0; 16];
    let mut blockN2: [libc::c_uchar; 16] = [0; 16];
    let mut blockN1: [libc::c_uchar; 16] = [0; 16];
    let mut input_length: size_t = 0;
    let mut nblocks: size_t = 0;
    let mut ncontig: size_t = 0;
    let mut cursor: iov_cursor =
        iov_cursor{iov: 0 as *const krb5_crypto_iov,
                   iov_count: 0,
                   block_size: 0,
                   signing: 0,
                   in_iov: 0,
                   in_pos: 0,
                   out_iov: 0,
                   out_pos: 0,};
    if init_key_cache(key) != 0 { return 12 as libc::c_int }
    expand_enc_key(key);
    k5_iov_cursor_init(&mut cursor, data, num_data,
                       16 as libc::c_int as size_t,
                       0 as libc::c_int as krb5_boolean);
    input_length =
        iov_total_length(data, num_data, 0 as libc::c_int as krb5_boolean);
    nblocks =
        input_length.wrapping_add(16 as libc::c_int as
                                      libc::c_ulong).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_div(16
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong);
    if nblocks == 1 as libc::c_int as libc::c_ulong {
        k5_iov_cursor_get(&mut cursor, block.as_mut_ptr());
        memset(iv.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               16 as libc::c_int as libc::c_ulong);
        cbc_enc(key, block.as_mut_ptr(), 1 as libc::c_int as size_t,
                iv.as_mut_ptr());
        k5_iov_cursor_put(&mut cursor, block.as_mut_ptr());
        return 0 as libc::c_int
    }
    if !ivec.is_null() {
        memcpy(iv.as_mut_ptr() as *mut libc::c_void,
               (*ivec).data as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    } else {
        memset(iv.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               16 as libc::c_int as libc::c_ulong);
    }
    while nblocks > 2 as libc::c_int as libc::c_ulong {
        ncontig = iov_cursor_contig_blocks(&mut cursor);
        if ncontig > 0 as libc::c_int as libc::c_ulong {
            /* Encrypt a series of contiguous blocks in place if we can, but
             * don't touch the last two blocks. */
            ncontig =
                if ncontig >
                       nblocks.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                   {
                    nblocks.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                } else { ncontig };
            cbc_enc(key, iov_cursor_ptr(&mut cursor), ncontig,
                    iv.as_mut_ptr());
            iov_cursor_advance(&mut cursor, ncontig);
            nblocks =
                (nblocks as libc::c_ulong).wrapping_sub(ncontig) as size_t as
                    size_t
        } else {
            k5_iov_cursor_get(&mut cursor, block.as_mut_ptr());
            cbc_enc(key, block.as_mut_ptr(), 1 as libc::c_int as size_t,
                    iv.as_mut_ptr());
            k5_iov_cursor_put(&mut cursor, block.as_mut_ptr());
            nblocks = nblocks.wrapping_sub(1)
        }
    }
    /* Encrypt the last two blocks and put them back in reverse order, possibly
     * truncating the encrypted second-to-last block. */
    k5_iov_cursor_get(&mut cursor, blockN2.as_mut_ptr());
    k5_iov_cursor_get(&mut cursor, blockN1.as_mut_ptr());
    cbc_enc(key, blockN2.as_mut_ptr(), 1 as libc::c_int as size_t,
            iv.as_mut_ptr());
    cbc_enc(key, blockN1.as_mut_ptr(), 1 as libc::c_int as size_t,
            iv.as_mut_ptr());
    k5_iov_cursor_put(&mut cursor, blockN1.as_mut_ptr());
    k5_iov_cursor_put(&mut cursor, blockN2.as_mut_ptr());
    if !ivec.is_null() {
        memcpy((*ivec).data as *mut libc::c_void,
               iv.as_mut_ptr() as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    }
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
/* The following are used by test programs and are just handler functions from
 * the AES and Camellia enc providers. */
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn krb5int_aes_decrypt(mut key: krb5_key,
                                             mut ivec: *const krb5_data,
                                             mut data: *mut krb5_crypto_iov,
                                             mut num_data: size_t)
 -> krb5_error_code {
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut dummy_iv: [libc::c_uchar; 16] = [0; 16];
    let mut block: [libc::c_uchar; 16] = [0; 16];
    let mut blockN2: [libc::c_uchar; 16] = [0; 16];
    let mut blockN1: [libc::c_uchar; 16] = [0; 16];
    let mut input_length: size_t = 0;
    let mut last_len: size_t = 0;
    let mut nblocks: size_t = 0;
    let mut ncontig: size_t = 0;
    let mut cursor: iov_cursor =
        iov_cursor{iov: 0 as *const krb5_crypto_iov,
                   iov_count: 0,
                   block_size: 0,
                   signing: 0,
                   in_iov: 0,
                   in_pos: 0,
                   out_iov: 0,
                   out_pos: 0,};
    if init_key_cache(key) != 0 { return 12 as libc::c_int }
    expand_dec_key(key);
    k5_iov_cursor_init(&mut cursor, data, num_data,
                       16 as libc::c_int as size_t,
                       0 as libc::c_int as krb5_boolean);
    input_length =
        iov_total_length(data, num_data, 0 as libc::c_int as krb5_boolean);
    nblocks =
        input_length.wrapping_add(16 as libc::c_int as
                                      libc::c_ulong).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong).wrapping_div(16
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong);
    last_len =
        input_length.wrapping_sub(nblocks.wrapping_sub(1 as libc::c_int as
                                                           libc::c_ulong).wrapping_mul(16
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong));
    if nblocks == 1 as libc::c_int as libc::c_ulong {
        k5_iov_cursor_get(&mut cursor, block.as_mut_ptr());
        memset(iv.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               16 as libc::c_int as libc::c_ulong);
        cbc_dec(key, block.as_mut_ptr(), 1 as libc::c_int as size_t,
                iv.as_mut_ptr());
        k5_iov_cursor_put(&mut cursor, block.as_mut_ptr());
        return 0 as libc::c_int
    }
    if !ivec.is_null() {
        memcpy(iv.as_mut_ptr() as *mut libc::c_void,
               (*ivec).data as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    } else {
        memset(iv.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               16 as libc::c_int as libc::c_ulong);
    }
    while nblocks > 2 as libc::c_int as libc::c_ulong {
        ncontig = iov_cursor_contig_blocks(&mut cursor);
        if ncontig > 0 as libc::c_int as libc::c_ulong {
            /* Decrypt a series of contiguous blocks in place if we can, but
             * don't touch the last two blocks. */
            ncontig =
                if ncontig >
                       nblocks.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                   {
                    nblocks.wrapping_sub(2 as libc::c_int as libc::c_ulong)
                } else { ncontig };
            cbc_dec(key, iov_cursor_ptr(&mut cursor), ncontig,
                    iv.as_mut_ptr());
            iov_cursor_advance(&mut cursor, ncontig);
            nblocks =
                (nblocks as libc::c_ulong).wrapping_sub(ncontig) as size_t as
                    size_t
        } else {
            k5_iov_cursor_get(&mut cursor, block.as_mut_ptr());
            cbc_dec(key, block.as_mut_ptr(), 1 as libc::c_int as size_t,
                    iv.as_mut_ptr());
            k5_iov_cursor_put(&mut cursor, block.as_mut_ptr());
            nblocks = nblocks.wrapping_sub(1)
        }
    }
    /* Get the last two ciphertext blocks.  Save the first as the new iv. */
    k5_iov_cursor_get(&mut cursor, blockN2.as_mut_ptr());
    k5_iov_cursor_get(&mut cursor, blockN1.as_mut_ptr());
    if !ivec.is_null() {
        memcpy((*ivec).data as *mut libc::c_void,
               blockN2.as_mut_ptr() as *const libc::c_void,
               16 as libc::c_int as libc::c_ulong);
    }
    /* Decrypt the second-to-last ciphertext block, using the final ciphertext
     * block as the CBC IV.  This produces the final plaintext block. */
    memcpy(dummy_iv.as_mut_ptr() as *mut libc::c_void,
           blockN1.as_mut_ptr() as *const libc::c_void,
           ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong);
    cbc_dec(key, blockN2.as_mut_ptr(), 1 as libc::c_int as size_t,
            dummy_iv.as_mut_ptr());
    /* Use the final bits of the decrypted plaintext to pad the last ciphertext
     * block, and decrypt it to produce the second-to-last plaintext block. */
    memcpy(blockN1.as_mut_ptr().offset(last_len as isize) as
               *mut libc::c_void,
           blockN2.as_mut_ptr().offset(last_len as isize) as
               *const libc::c_void,
           (16 as libc::c_int as libc::c_ulong).wrapping_sub(last_len));
    cbc_dec(key, blockN1.as_mut_ptr(), 1 as libc::c_int as size_t,
            iv.as_mut_ptr());
    /* Put the last two plaintext blocks back into the iovec. */
    k5_iov_cursor_put(&mut cursor, blockN1.as_mut_ptr());
    k5_iov_cursor_put(&mut cursor, blockN2.as_mut_ptr());
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn aes_init_state(mut key: *const krb5_keyblock,
                                    mut usage: krb5_keyusage,
                                    mut state: *mut krb5_data)
 -> krb5_error_code {
    (*state).length = 16 as libc::c_int as libc::c_uint;
    (*state).data =
        malloc(16 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if (*state).data.is_null() { return 12 as libc::c_int }
    memset((*state).data as *mut libc::c_void, 0 as libc::c_int,
           (*state).length as libc::c_ulong);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "380:1"]
unsafe extern "C" fn aes_key_cleanup(mut key: krb5_key) {
    zapfree((*key).cache,
            ::std::mem::size_of::<aes_key_info_cache>() as libc::c_ulong);
}
#[no_mangle]
#[c2rust::src_loc = "386:32"]
pub static mut krb5int_enc_aes128: krb5_enc_provider =
    unsafe {
        {
            let mut init =
                krb5_enc_provider{block_size: 16 as libc::c_int as size_t,
                                  keybytes: 16 as libc::c_int as size_t,
                                  keylength: 16 as libc::c_int as size_t,
                                  encrypt:
                                      Some(krb5int_aes_encrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  decrypt:
                                      Some(krb5int_aes_decrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  cbc_mac: None,
                                  init_state:
                                      Some(aes_init_state as
                                               unsafe extern "C" fn(_:
                                                                        *const krb5_keyblock,
                                                                    _:
                                                                        krb5_keyusage,
                                                                    _:
                                                                        *mut krb5_data)
                                                   -> krb5_error_code),
                                  free_state:
                                      Some(krb5int_default_free_state as
                                               unsafe extern "C" fn(_:
                                                                        *mut krb5_data)
                                                   -> ()),
                                  key_cleanup:
                                      Some(aes_key_cleanup as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key)
                                                   -> ()),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "397:32"]
pub static mut krb5int_enc_aes256: krb5_enc_provider =
    unsafe {
        {
            let mut init =
                krb5_enc_provider{block_size: 16 as libc::c_int as size_t,
                                  keybytes: 32 as libc::c_int as size_t,
                                  keylength: 32 as libc::c_int as size_t,
                                  encrypt:
                                      Some(krb5int_aes_encrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  decrypt:
                                      Some(krb5int_aes_decrypt as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key,
                                                                    _:
                                                                        *const krb5_data,
                                                                    _:
                                                                        *mut krb5_crypto_iov,
                                                                    _: size_t)
                                                   -> krb5_error_code),
                                  cbc_mac: None,
                                  init_state:
                                      Some(aes_init_state as
                                               unsafe extern "C" fn(_:
                                                                        *const krb5_keyblock,
                                                                    _:
                                                                        krb5_keyusage,
                                                                    _:
                                                                        *mut krb5_data)
                                                   -> krb5_error_code),
                                  free_state:
                                      Some(krb5int_default_free_state as
                                               unsafe extern "C" fn(_:
                                                                        *mut krb5_data)
                                                   -> ()),
                                  key_cleanup:
                                      Some(aes_key_cleanup as
                                               unsafe extern "C" fn(_:
                                                                        krb5_key)
                                                   -> ()),};
            init
        }
    };
