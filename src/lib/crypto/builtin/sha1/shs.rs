use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:2"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:2"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:2"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:2"]
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
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/sha1/shs.h:2"]
pub mod shs_h {
    #[c2rust::src_loc = "10:1"]
    pub type SHS_BYTE = krb5_octet;
    #[c2rust::src_loc = "11:1"]
    pub type SHS_LONG = krb5_ui_4;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "23:9"]
    pub struct SHS_INFO {
        pub digest: [SHS_LONG; 5],
        pub countLo: SHS_LONG,
        pub countHi: SHS_LONG,
        pub data: [SHS_LONG; 16],
    }
    use super::krb5_h::{krb5_octet, krb5_ui_4};
    /* _SHS_DEFINED */
}
#[c2rust::header_src = "/usr/include/string.h:2"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:2"]
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
pub use self::types_h::{__uint8_t, __uint32_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::k5_platform_h::{C2RustUnnamed, load_32_be};
pub use self::krb5_h::{krb5_octet, krb5_ui_4};
pub use self::shs_h::{SHS_BYTE, SHS_LONG, SHS_INFO};
use self::string_h::memcpy;
pub use self::byteswap_h::__bswap_32;
/* Note that it may be necessary to add parentheses to these macros if they
   are to be called with expressions as arguments */
/* 32-bit rotate left - kludged with shifts */
/* The initial expanding function.  The hash function is defined over an
   80-word expanded input array W, where the first 16 are copies of the input
   data, and the remaining 64 are defined by

   W[ i ] = W[ i - 16 ] ^ W[ i - 14 ] ^ W[ i - 8 ] ^ W[ i - 3 ]

   This implementation generates these values on the fly in a circular
   buffer - thanks to Colin Plumb, colin@nyx10.cs.du.edu for this
   optimization.

   The updated SHS changes the expanding function by adding a rotate of 1
   bit.  Thanks to Jim Gillogly, jim@rand.org, and an anonymous contributor
   for this information */
/* NEW_SHS */
/* The prototype SHS sub-round.  The fundamental sub-round is:

   a' = e + ROTL( 5, a ) + f( b, c, d ) + k + data;
   b' = a;
   c' = ROTL( 30, b );
   d' = c;
   e' = d;

   but this is implemented by unrolling the loop 5 times and renaming the
   variables ( e, a, b, c, d ) = ( a', b', c', d', e' ) each iteration.
   This code is then replicated 20 times for each of the 4 functions, using
   the next 20 values from the W[] array each time */
/* Initialize the SHS values */
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn shsInit(mut shsInfo: *mut SHS_INFO) {
    /* Set the h-vars to their initial values */
    (*shsInfo).digest[0 as libc::c_int as usize] =
        0x67452301 as libc::c_long as SHS_LONG;
    (*shsInfo).digest[1 as libc::c_int as usize] =
        0xefcdab89 as libc::c_long as SHS_LONG;
    (*shsInfo).digest[2 as libc::c_int as usize] =
        0x98badcfe as libc::c_long as SHS_LONG;
    (*shsInfo).digest[3 as libc::c_int as usize] =
        0x10325476 as libc::c_long as SHS_LONG;
    (*shsInfo).digest[4 as libc::c_int as usize] =
        0xc3d2e1f0 as libc::c_long as SHS_LONG;
    /* Initialise bit count */
    (*shsInfo).countHi = 0 as libc::c_int as SHS_LONG;
    (*shsInfo).countLo = (*shsInfo).countHi;
}
/* Perform the SHS transformation.  Note that this code, like MD5, seems to
   break some optimizing compilers due to the complexity of the expressions
   and the size of the basic block.  It may be necessary to split it into
   sections, e.g. based on the four subrounds

   Note that this corrupts the shsInfo->data area */
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn SHSTransform(mut digest: *mut SHS_LONG,
                                  mut data: *const SHS_LONG) {
    let mut A: SHS_LONG = 0; /* Local vars */
    let mut B: SHS_LONG = 0; /* Expanded data */
    let mut C: SHS_LONG = 0;
    let mut D: SHS_LONG = 0;
    let mut E: SHS_LONG = 0;
    let mut eData: [SHS_LONG; 16] = [0; 16];
    /* Set up first buffer and local data buffer */
    A = *digest.offset(0 as libc::c_int as isize);
    B = *digest.offset(1 as libc::c_int as isize);
    C = *digest.offset(2 as libc::c_int as isize);
    D = *digest.offset(3 as libc::c_int as isize);
    E = *digest.offset(4 as libc::c_int as isize);
    memcpy(eData.as_mut_ptr() as *mut libc::c_void,
           data as *const libc::c_void,
           ::std::mem::size_of::<[SHS_LONG; 16]>() as libc::c_ulong);
    /* Heavy mangling, in 4 sub-rounds of 20 interations each. */
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ B & (C ^ D)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[0 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ A & (B ^ C)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[1 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ E & (A ^ B)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[2 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ D & (E ^ A)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[3 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ C & (D ^ E)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[4 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ B & (C ^ D)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[5 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ A & (B ^ C)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[6 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ E & (A ^ B)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[7 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ D & (E ^ A)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[8 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ C & (D ^ E)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[9 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ B & (C ^ D)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[10 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ A & (B ^ C)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[11 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ E & (A ^ B)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[12 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ D & (E ^ A)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[13 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ C & (D ^ E)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[14 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ B & (C ^ D)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[15 as libc::c_int as usize] as libc::c_long)) as
            SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(16 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(16 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(16 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(16 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(16 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(16 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(16 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(16 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(16 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ A & (B ^ C)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[(16 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(17 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(17 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(17 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(17 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(17 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(17 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(17 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(17 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(17 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ E & (A ^ B)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[(17 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(18 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(18 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(18 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(18 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(18 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(18 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(18 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(18 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(18 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ D & (E ^ A)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[(18 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(19 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(19 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(19 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(19 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(19 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(19 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(19 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(19 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(19 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ C & (D ^ E)) as
                  libc::c_long + 0x5a827999 as libc::c_long +
                  eData[(19 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(20 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(20 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(20 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(20 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(20 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(20 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(20 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(20 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(20 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(20 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(21 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(21 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(21 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(21 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(21 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(21 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(21 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(21 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(21 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(21 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(22 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(22 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(22 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(22 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(22 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(22 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(22 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(22 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(22 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(22 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(23 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(23 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(23 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(23 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(23 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(23 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(23 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(23 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(23 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(23 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(24 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(24 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(24 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(24 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(24 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(24 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(24 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(24 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(24 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(24 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(25 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(25 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(25 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(25 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(25 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(25 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(25 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(25 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(25 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(25 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(26 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(26 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(26 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(26 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(26 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(26 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(26 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(26 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(26 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(26 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(27 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(27 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(27 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(27 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(27 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(27 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(27 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(27 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(27 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(27 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(28 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(28 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(28 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(28 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(28 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(28 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(28 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(28 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(28 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(28 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(29 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(29 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(29 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(29 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(29 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(29 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(29 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(29 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(29 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(29 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(30 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(30 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(30 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(30 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(30 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(30 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(30 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(30 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(30 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(30 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(31 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(31 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(31 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(31 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(31 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(31 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(31 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(31 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(31 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(31 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(32 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(32 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(32 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(32 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(32 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(32 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(32 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(32 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(32 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(32 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(33 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(33 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(33 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(33 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(33 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(33 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(33 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(33 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(33 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(33 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(34 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(34 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(34 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(34 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(34 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(34 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(34 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(34 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(34 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(34 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(35 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(35 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(35 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(35 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(35 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(35 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(35 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(35 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(35 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(35 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(36 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(36 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(36 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(36 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(36 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(36 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(36 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(36 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(36 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(36 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(37 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(37 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(37 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(37 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(37 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(37 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(37 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(37 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(37 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(37 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(38 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(38 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(38 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(38 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(38 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(38 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(38 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(38 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(38 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(38 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(39 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(39 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(39 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(39 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(39 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(39 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(39 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(39 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(39 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0x6ed9eba1 as libc::c_long +
                  eData[(39 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(40 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(40 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(40 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(40 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(40 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(40 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(40 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(40 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(40 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B & C | D & (B | C))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(40 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(41 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(41 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(41 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(41 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(41 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(41 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(41 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(41 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(41 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A & B | C & (A | B))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(41 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(42 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(42 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(42 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(42 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(42 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(42 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(42 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(42 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(42 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E & A | B & (E | A))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(42 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(43 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(43 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(43 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(43 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(43 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(43 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(43 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(43 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(43 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D & E | A & (D | E))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(43 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(44 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(44 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(44 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(44 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(44 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(44 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(44 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(44 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(44 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C & D | E & (C | D))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(44 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(45 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(45 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(45 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(45 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(45 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(45 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(45 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(45 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(45 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B & C | D & (B | C))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(45 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(46 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(46 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(46 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(46 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(46 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(46 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(46 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(46 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(46 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A & B | C & (A | B))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(46 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(47 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(47 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(47 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(47 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(47 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(47 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(47 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(47 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(47 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E & A | B & (E | A))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(47 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(48 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(48 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(48 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(48 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(48 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(48 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(48 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(48 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(48 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D & E | A & (D | E))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(48 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(49 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(49 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(49 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(49 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(49 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(49 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(49 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(49 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(49 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C & D | E & (C | D))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(49 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(50 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(50 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(50 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(50 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(50 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(50 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(50 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(50 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(50 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B & C | D & (B | C))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(50 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(51 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(51 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(51 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(51 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(51 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(51 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(51 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(51 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(51 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A & B | C & (A | B))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(51 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(52 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(52 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(52 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(52 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(52 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(52 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(52 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(52 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(52 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E & A | B & (E | A))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(52 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(53 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(53 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(53 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(53 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(53 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(53 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(53 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(53 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(53 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D & E | A & (D | E))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(53 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(54 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(54 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(54 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(54 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(54 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(54 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(54 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(54 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(54 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C & D | E & (C | D))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(54 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(55 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(55 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(55 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(55 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(55 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(55 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(55 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(55 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(55 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B & C | D & (B | C))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(55 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(56 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(56 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(56 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(56 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(56 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(56 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(56 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(56 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(56 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A & B | C & (A | B))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(56 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(57 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(57 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(57 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(57 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(57 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(57 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(57 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(57 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(57 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E & A | B & (E | A))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(57 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(58 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(58 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(58 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(58 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(58 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(58 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(58 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(58 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(58 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D & E | A & (D | E))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(58 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(59 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(59 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(59 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(59 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(59 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(59 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(59 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(59 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(59 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C & D | E & (C | D))
                  as libc::c_long + 0x8f1bbcdc as libc::c_long +
                  eData[(59 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(60 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(60 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(60 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(60 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(60 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(60 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(60 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(60 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(60 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(60 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(61 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(61 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(61 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(61 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(61 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(61 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(61 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(61 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(61 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(61 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(62 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(62 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(62 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(62 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(62 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(62 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(62 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(62 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(62 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(62 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(63 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(63 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(63 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(63 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(63 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(63 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(63 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(63 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(63 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(63 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(64 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(64 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(64 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(64 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(64 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(64 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(64 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(64 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(64 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(64 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(65 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(65 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(65 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(65 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(65 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(65 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(65 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(65 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(65 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(65 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(66 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(66 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(66 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(66 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(66 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(66 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(66 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(66 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(66 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(66 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(67 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(67 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(67 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(67 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(67 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(67 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(67 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(67 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(67 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(67 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(68 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(68 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(68 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(68 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(68 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(68 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(68 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(68 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(68 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(68 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(69 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(69 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(69 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(69 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(69 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(69 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(69 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(69 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(69 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(69 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(70 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(70 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(70 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(70 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(70 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(70 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(70 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(70 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(70 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(70 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(71 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(71 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(71 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(71 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(71 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(71 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(71 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(71 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(71 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(71 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(72 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(72 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(72 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(72 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(72 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(72 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(72 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(72 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(72 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(72 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(73 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(73 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(73 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(73 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(73 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(73 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(73 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(73 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(73 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(73 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(74 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(74 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(74 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(74 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(74 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(74 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(74 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(74 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(74 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(74 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(75 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(75 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(75 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(75 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(75 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(75 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(75 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(75 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(75 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    E =
        (E as libc::c_long +
             ((A << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   A >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(B ^ C ^ D) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(75 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    E &= 0xffffffff as libc::c_uint;
    B =
        B << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            B >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(76 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(76 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(76 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(76 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(76 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(76 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(76 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(76 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(76 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    D =
        (D as libc::c_long +
             ((E << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   E >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(A ^ B ^ C) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(76 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    D &= 0xffffffff as libc::c_uint;
    A =
        A << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            A >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(77 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(77 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(77 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(77 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(77 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(77 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(77 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(77 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(77 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    C =
        (C as libc::c_long +
             ((D << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   D >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(E ^ A ^ B) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(77 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    C &= 0xffffffff as libc::c_uint;
    E =
        E << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            E >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(78 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(78 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(78 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(78 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(78 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(78 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(78 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(78 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(78 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    B =
        (B as libc::c_long +
             ((C << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   C >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(D ^ E ^ A) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(78 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    B &= 0xffffffff as libc::c_uint;
    D =
        D << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            D >> 32 as libc::c_int - 30 as libc::c_int;
    eData[(79 as libc::c_int & 15 as libc::c_int) as usize] =
        (eData[(79 as libc::c_int & 15 as libc::c_int) as usize] ^
             eData[(79 as libc::c_int - 14 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(79 as libc::c_int - 8 as libc::c_int & 15 as libc::c_int)
                       as usize] ^
             eData[(79 as libc::c_int - 3 as libc::c_int & 15 as libc::c_int)
                       as usize]) << 1 as libc::c_int &
            0xffffffff as libc::c_uint |
            (eData[(79 as libc::c_int & 15 as libc::c_int) as usize] ^
                 eData[(79 as libc::c_int - 14 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(79 as libc::c_int - 8 as libc::c_int &
                            15 as libc::c_int) as usize] ^
                 eData[(79 as libc::c_int - 3 as libc::c_int &
                            15 as libc::c_int) as usize]) >>
                32 as libc::c_int - 1 as libc::c_int;
    A =
        (A as libc::c_long +
             ((B << 5 as libc::c_int & 0xffffffff as libc::c_uint |
                   B >>
                       32 as libc::c_int -
                           5 as libc::c_int).wrapping_add(C ^ D ^ E) as
                  libc::c_long + 0xca62c1d6 as libc::c_long +
                  eData[(79 as libc::c_int & 15 as libc::c_int) as usize] as
                      libc::c_long)) as SHS_LONG;
    A &= 0xffffffff as libc::c_uint;
    C =
        C << 30 as libc::c_int & 0xffffffff as libc::c_uint |
            C >> 32 as libc::c_int - 30 as libc::c_int;
    /* Build message digest */
    let ref mut fresh0 = *digest.offset(0 as libc::c_int as isize);
    *fresh0 =
        (*fresh0 as libc::c_uint).wrapping_add(A) as SHS_LONG as SHS_LONG;
    let ref mut fresh1 = *digest.offset(0 as libc::c_int as isize);
    *fresh1 &= 0xffffffff as libc::c_uint;
    let ref mut fresh2 = *digest.offset(1 as libc::c_int as isize);
    *fresh2 =
        (*fresh2 as libc::c_uint).wrapping_add(B) as SHS_LONG as SHS_LONG;
    let ref mut fresh3 = *digest.offset(1 as libc::c_int as isize);
    *fresh3 &= 0xffffffff as libc::c_uint;
    let ref mut fresh4 = *digest.offset(2 as libc::c_int as isize);
    *fresh4 =
        (*fresh4 as libc::c_uint).wrapping_add(C) as SHS_LONG as SHS_LONG;
    let ref mut fresh5 = *digest.offset(2 as libc::c_int as isize);
    *fresh5 &= 0xffffffff as libc::c_uint;
    let ref mut fresh6 = *digest.offset(3 as libc::c_int as isize);
    *fresh6 =
        (*fresh6 as libc::c_uint).wrapping_add(D) as SHS_LONG as SHS_LONG;
    let ref mut fresh7 = *digest.offset(3 as libc::c_int as isize);
    *fresh7 &= 0xffffffff as libc::c_uint;
    let ref mut fresh8 = *digest.offset(4 as libc::c_int as isize);
    *fresh8 =
        (*fresh8 as libc::c_uint).wrapping_add(E) as SHS_LONG as SHS_LONG;
    let ref mut fresh9 = *digest.offset(4 as libc::c_int as isize);
    *fresh9 &= 0xffffffff as libc::c_uint;
}
/* Update SHS for a block of data */
#[no_mangle]
#[c2rust::src_loc = "244:1"]
pub unsafe extern "C" fn shsUpdate(mut shsInfo: *mut SHS_INFO,
                                   mut buffer: *const SHS_BYTE,
                                   mut count: libc::c_uint) {
    let mut tmp: SHS_LONG = 0;
    let mut dataCount: libc::c_uint = 0;
    let mut canfill: libc::c_int = 0;
    let mut lp: *mut SHS_LONG = 0 as *mut SHS_LONG;
    /* Update bitcount */
    tmp = (*shsInfo).countLo; /* Carry from low to high */
    (*shsInfo).countLo = tmp.wrapping_add(count << 3 as libc::c_int);
    (*shsInfo).countLo &= 0xffffffff as libc::c_uint;
    if (*shsInfo).countLo < tmp {
        (*shsInfo).countHi = (*shsInfo).countHi.wrapping_add(1)
    }
    (*shsInfo).countHi =
        ((*shsInfo).countHi as
             libc::c_uint).wrapping_add(count >> 29 as libc::c_int) as
            SHS_LONG as SHS_LONG;
    /* Get count of bytes already in data */
    dataCount = tmp >> 3 as libc::c_int & 0x3f as libc::c_int as libc::c_uint;
    /* Handle any leading odd-sized chunks */
    if dataCount != 0 {
        lp =
            (*shsInfo).data.as_mut_ptr().offset(dataCount.wrapping_div(4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                                                    as isize);
        dataCount =
            (64 as libc::c_int as libc::c_uint).wrapping_sub(dataCount);
        canfill = (count >= dataCount) as libc::c_int;
        if dataCount.wrapping_rem(4 as libc::c_int as libc::c_uint) != 0 {
            /* Fill out a full 32 bit word first if needed -- this
               is not very efficient (computed shift amount),
               but it shouldn't happen often. */
            while dataCount.wrapping_rem(4 as libc::c_int as libc::c_uint) !=
                      0 && count > 0 as libc::c_int as libc::c_uint {
                let fresh10 = buffer;
                buffer = buffer.offset(1);
                dataCount = dataCount.wrapping_sub(1);
                *lp |=
                    (*fresh10 as SHS_LONG) <<
                        dataCount.wrapping_rem(4 as libc::c_int as
                                                   libc::c_uint).wrapping_mul(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint);
                count = count.wrapping_sub(1)
            }
            lp = lp.offset(1)
        }
        while lp <
                  (*shsInfo).data.as_mut_ptr().offset(16 as libc::c_int as
                                                          isize) {
            if count < 4 as libc::c_int as libc::c_uint {
                *lp = 0 as libc::c_int as SHS_LONG;
                let mut current_block_19: u64;
                match count.wrapping_rem(4 as libc::c_int as libc::c_uint) {
                    3 => {
                        *lp |=
                            (*buffer.offset(2 as libc::c_int as isize) as
                                 SHS_LONG) << 8 as libc::c_int;
                        current_block_19 = 4232962747835228946;
                    }
                    2 => { current_block_19 = 4232962747835228946; }
                    1 => { current_block_19 = 7102108004811975442; }
                    _ => { current_block_19 = 17478428563724192186; }
                }
                match current_block_19 {
                    4232962747835228946 => {
                        *lp |=
                            (*buffer.offset(1 as libc::c_int as isize) as
                                 SHS_LONG) << 16 as libc::c_int;
                        current_block_19 = 7102108004811975442;
                    }
                    _ => { }
                }
                match current_block_19 {
                    7102108004811975442 => {
                        *lp |=
                            (*buffer.offset(0 as libc::c_int as isize) as
                                 SHS_LONG) << 24 as libc::c_int
                    }
                    _ => { }
                }
                count = 0 as libc::c_int as libc::c_uint;
                break ;
                /* out of while loop */
            } else {
                let fresh11 = lp;
                lp = lp.offset(1);
                *fresh11 = load_32_be(buffer as *const libc::c_void);
                buffer = buffer.offset(4 as libc::c_int as isize);
                count = count.wrapping_sub(4 as libc::c_int as libc::c_uint)
            }
        }
        if canfill != 0 {
            SHSTransform((*shsInfo).digest.as_mut_ptr(),
                         (*shsInfo).data.as_mut_ptr());
        }
    }
    /* Process data in SHS_DATASIZE chunks */
    while count >= 64 as libc::c_int as libc::c_uint {
        lp = (*shsInfo).data.as_mut_ptr();
        while lp <
                  (*shsInfo).data.as_mut_ptr().offset(16 as libc::c_int as
                                                          isize) {
            let fresh12 = lp;
            lp = lp.offset(1);
            *fresh12 = load_32_be(buffer as *const libc::c_void);
            buffer = buffer.offset(4 as libc::c_int as isize)
        }
        SHSTransform((*shsInfo).digest.as_mut_ptr(),
                     (*shsInfo).data.as_mut_ptr());
        count = count.wrapping_sub(64 as libc::c_int as libc::c_uint)
    }
    if count > 0 as libc::c_int as libc::c_uint {
        lp = (*shsInfo).data.as_mut_ptr();
        while count > 4 as libc::c_int as libc::c_uint {
            let fresh13 = lp;
            lp = lp.offset(1);
            *fresh13 = load_32_be(buffer as *const libc::c_void);
            buffer = buffer.offset(4 as libc::c_int as isize);
            count = count.wrapping_sub(4 as libc::c_int as libc::c_uint)
        }
        *lp = 0 as libc::c_int as SHS_LONG;
        let mut current_block_50: u64;
        match count.wrapping_rem(4 as libc::c_int as libc::c_uint) {
            0 => {
                *lp |= *buffer.offset(3 as libc::c_int as isize) as SHS_LONG;
                current_block_50 = 13165257528546088254;
            }
            3 => { current_block_50 = 13165257528546088254; }
            2 => { current_block_50 = 8375352771447145478; }
            1 => { current_block_50 = 10597189263733886816; }
            _ => { current_block_50 = 10753070352654377903; }
        }
        match current_block_50 {
            13165257528546088254 => {
                *lp |=
                    (*buffer.offset(2 as libc::c_int as isize) as SHS_LONG) <<
                        8 as libc::c_int;
                current_block_50 = 8375352771447145478;
            }
            _ => { }
        }
        match current_block_50 {
            8375352771447145478 => {
                *lp |=
                    (*buffer.offset(1 as libc::c_int as isize) as SHS_LONG) <<
                        16 as libc::c_int;
                current_block_50 = 10597189263733886816;
            }
            _ => { }
        }
        match current_block_50 {
            10597189263733886816 => {
                *lp |=
                    (*buffer.offset(0 as libc::c_int as isize) as SHS_LONG) <<
                        24 as libc::c_int
            }
            _ => { }
        }
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* Some useful types */
/* Define the following to use the updated SHS implementation */
/* */
/* The SHS block size and message digest sizes, in bytes */
/* The structure for storing SHS info */
/* Message digest */
/* 64-bit bit count */
/* SHS data buffer */
/* Message digest functions (shs.c) */
/* Final wrapup - pad to SHS_DATASIZE-byte boundary with the bit pattern
   1 0* (64-bit count of bits processed, MSB-first) */
#[no_mangle]
#[c2rust::src_loc = "335:1"]
pub unsafe extern "C" fn shsFinal(mut shsInfo: *mut SHS_INFO) {
    let mut count: libc::c_int = 0;
    let mut lp: *mut SHS_LONG = 0 as *mut SHS_LONG;
    /* Compute number of bytes mod 64 */
    count = (*shsInfo).countLo as libc::c_int;
    count = count >> 3 as libc::c_int & 0x3f as libc::c_int;
    /* Set the first char of padding to 0x80.  This is safe since there is
       always at least one byte free */
    lp =
        (*shsInfo).data.as_mut_ptr().offset((count / 4 as libc::c_int) as
                                                isize);
    match count % 4 as libc::c_int {
        3 => {
            let fresh14 = lp;
            lp = lp.offset(1);
            *fresh14 |= 0x80 as libc::c_int as SHS_LONG
        }
        2 => {
            let fresh15 = lp;
            lp = lp.offset(1);
            *fresh15 |= (0x80 as libc::c_int as SHS_LONG) << 8 as libc::c_int
        }
        1 => {
            let fresh16 = lp;
            lp = lp.offset(1);
            *fresh16 |= (0x80 as libc::c_int as SHS_LONG) << 16 as libc::c_int
        }
        0 => {
            let fresh17 = lp;
            lp = lp.offset(1);
            *fresh17 = (0x80 as libc::c_int as SHS_LONG) << 24 as libc::c_int
        }
        _ => { }
    }
    /* at this point, lp can point *past* shsInfo->data.  If it points
       there, just Transform and reset.  If it points to the last
       element, set that to zero.  This pads out to 64 bytes if not
       enough room for length words */
    if lp == (*shsInfo).data.as_mut_ptr().offset(15 as libc::c_int as isize) {
        let fresh18 = lp;
        lp = lp.offset(1);
        *fresh18 = 0 as libc::c_int as SHS_LONG
    }
    if lp == (*shsInfo).data.as_mut_ptr().offset(16 as libc::c_int as isize) {
        SHSTransform((*shsInfo).digest.as_mut_ptr(),
                     (*shsInfo).data.as_mut_ptr());
        lp = (*shsInfo).data.as_mut_ptr()
    }
    /* Pad out to 56 bytes */
    while lp < (*shsInfo).data.as_mut_ptr().offset(14 as libc::c_int as isize)
          {
        let fresh19 = lp;
        lp = lp.offset(1);
        *fresh19 = 0 as libc::c_int as SHS_LONG
    }
    /* Append length in bits and transform */
    let fresh20 = lp;
    lp = lp.offset(1);
    *fresh20 = (*shsInfo).countHi;
    let fresh21 = lp;
    lp = lp.offset(1);
    *fresh21 = (*shsInfo).countLo;
    SHSTransform((*shsInfo).digest.as_mut_ptr(),
                 (*shsInfo).data.as_mut_ptr());
}
