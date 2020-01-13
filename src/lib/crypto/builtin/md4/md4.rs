use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:35"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:35"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "657:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "703:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "652:1"]
    pub unsafe extern "C" fn store_32_le(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = val;
    }
    #[inline]
    #[c2rust::src_loc = "698:1"]
    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed_0)).i;
    }
    use super::stdint_uintn_h::uint32_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:35"]
pub mod krb5_h {
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/md4/rsa-md4.h:36"]
pub mod rsa_md4_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:9"]
    pub struct krb5_MD4_CTX {
        pub i: [krb5_ui_4; 2],
        pub buf: [krb5_ui_4; 4],
        pub in_0: [libc::c_uchar; 64],
        pub digest: [libc::c_uchar; 16],
    }
    use super::krb5_h::krb5_ui_4;
    /* __KRB5_RSA_MD4_H__ */
    /*
**********************************************************************
** End of md4.h                                                     **
******************************* (cut) ********************************
*/
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_le,
                              load_32_le};
pub use self::krb5_h::krb5_ui_4;
pub use self::rsa_md4_h::krb5_MD4_CTX;
#[c2rust::src_loc = "41:28"]
static mut PADDING: [libc::c_uchar; 64] =
    [0x80 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar];
/* F, G and H are basic MD4 functions: selection, majority, parity */
/* ROTATE_LEFT rotates x left n bits */
/* FF, GG and HH are MD4 transformations for rounds 1, 2 and 3 */
/* Rotation is separate from addition to prevent recomputation */
#[no_mangle]
#[c2rust::src_loc = "75:1"]
pub unsafe extern "C" fn krb5int_MD4Init(mut mdContext: *mut krb5_MD4_CTX) {
    (*mdContext).i[1 as libc::c_int as usize] = 0 as libc::c_int as krb5_ui_4;
    (*mdContext).i[0 as libc::c_int as usize] =
        (*mdContext).i[1 as libc::c_int as usize];
    /* Load magic initialization constants.
     */
    (*mdContext).buf[0 as libc::c_int as usize] =
        0x67452301 as libc::c_ulong as krb5_ui_4;
    (*mdContext).buf[1 as libc::c_int as usize] =
        0xefcdab89 as libc::c_ulong as krb5_ui_4;
    (*mdContext).buf[2 as libc::c_int as usize] =
        0x98badcfe as libc::c_ulong as krb5_ui_4;
    (*mdContext).buf[3 as libc::c_int as usize] =
        0x10325476 as libc::c_ulong as krb5_ui_4;
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn krb5int_MD4Update(mut mdContext: *mut krb5_MD4_CTX,
                                           mut inBuf: *const libc::c_uchar,
                                           mut inLen: libc::c_uint) {
    let mut in_0: [krb5_ui_4; 16] = [0; 16];
    let mut mdi: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut ii: libc::c_uint = 0;
    /* compute number of bytes mod 64 */
    mdi =
        ((*mdContext).i[0 as libc::c_int as usize] >> 3 as libc::c_int &
             0x3f as libc::c_int as libc::c_uint) as libc::c_int;
    /* update number of bits */
    if (*mdContext).i[0 as libc::c_int as
                          usize].wrapping_add(inLen << 3 as libc::c_int) <
           (*mdContext).i[0 as libc::c_int as usize] {
        (*mdContext).i[1 as libc::c_int as usize] =
            (*mdContext).i[1 as libc::c_int as usize].wrapping_add(1)
    }
    (*mdContext).i[0 as libc::c_int as usize] =
        ((*mdContext).i[0 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(inLen << 3 as libc::c_int) as
            krb5_ui_4 as krb5_ui_4;
    (*mdContext).i[1 as libc::c_int as usize] =
        ((*mdContext).i[1 as libc::c_int as usize] as
             libc::c_uint).wrapping_add(inLen >> 29 as libc::c_int) as
            krb5_ui_4 as krb5_ui_4;
    loop  {
        let fresh0 = inLen;
        inLen = inLen.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        /* add new character to buffer, increment mdi */
        let fresh1 = inBuf;
        inBuf = inBuf.offset(1);
        let fresh2 = mdi;
        mdi = mdi + 1;
        (*mdContext).in_0[fresh2 as usize] = *fresh1;
        /* transform if necessary */
        if mdi == 0x40 as libc::c_int {
            i = 0 as libc::c_int as libc::c_uint;
            ii = 0 as libc::c_int as libc::c_uint;
            while i < 16 as libc::c_int as libc::c_uint {
                in_0[i as usize] =
                    load_32_le((*mdContext).in_0.as_mut_ptr().offset(ii as
                                                                         isize)
                                   as *const libc::c_void);
                i = i.wrapping_add(1);
                ii = ii.wrapping_add(4 as libc::c_int as libc::c_uint)
            }
            Transform((*mdContext).buf.as_mut_ptr(), in_0.as_mut_ptr());
            mdi = 0 as libc::c_int
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn krb5int_MD4Final(mut mdContext: *mut krb5_MD4_CTX) {
    let mut in_0: [krb5_ui_4; 16] = [0; 16];
    let mut mdi: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut ii: libc::c_uint = 0;
    let mut padLen: libc::c_uint = 0;
    /* save number of bits */
    in_0[14 as libc::c_int as usize] =
        (*mdContext).i[0 as libc::c_int as usize];
    in_0[15 as libc::c_int as usize] =
        (*mdContext).i[1 as libc::c_int as usize];
    /* compute number of bytes mod 64 */
    mdi =
        ((*mdContext).i[0 as libc::c_int as usize] >> 3 as libc::c_int &
             0x3f as libc::c_int as libc::c_uint) as libc::c_int;
    /* pad out to 56 mod 64 */
    padLen =
        if mdi < 56 as libc::c_int {
            (56 as libc::c_int) - mdi
        } else { (120 as libc::c_int) - mdi } as libc::c_uint;
    krb5int_MD4Update(mdContext, PADDING.as_ptr(), padLen);
    /* append length in bits and transform */
    i = 0 as libc::c_int as libc::c_uint;
    ii = 0 as libc::c_int as libc::c_uint;
    while i < 14 as libc::c_int as libc::c_uint {
        in_0[i as usize] =
            load_32_le((*mdContext).in_0.as_mut_ptr().offset(ii as isize) as
                           *const libc::c_void);
        i = i.wrapping_add(1);
        ii = ii.wrapping_add(4 as libc::c_int as libc::c_uint)
    }
    Transform((*mdContext).buf.as_mut_ptr(), in_0.as_mut_ptr());
    /* store buffer in digest */
    i = 0 as libc::c_int as libc::c_uint;
    ii = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        store_32_le((*mdContext).buf[i as usize],
                    (*mdContext).digest.as_mut_ptr().offset(ii as isize) as
                        *mut libc::c_void);
        i = i.wrapping_add(1);
        ii = ii.wrapping_add(4 as libc::c_int as libc::c_uint)
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/md4/md4.c */
/*
 * Copyright (C) 1990, RSA Data Security, Inc. All rights reserved.
 *
 * License to copy and use this software is granted provided that
 * it is identified as the "RSA Data Security, Inc. MD4 Message
 * Digest Algorithm" in all material mentioning or referencing this
 * software or this function.
 *
 * License is also granted to make and use derivative works
 * provided that such works are identified as "derived from the RSA
 * Data Security, Inc. MD4 Message Digest Algorithm" in all
 * material mentioning or referencing the derived work.
 *
 * RSA Data Security, Inc. makes no representations concerning
 * either the merchantability of this software or the suitability
 * of this software for any particular purpose.  It is provided "as
 * is" without express or implied warranty of any kind.
 *
 * These notices must be retained in any copies of any part of this
 * documentation and/or software.
 */
/*
**********************************************************************
** md4.c                                                            **
** RSA Data Security, Inc. MD4 Message Digest Algorithm             **
** Created: 2/17/90 RLR                                             **
** Revised: 1/91 SRD,AJ,BSK,JT Reference C Version                  **
**********************************************************************
*/
/* forward declaration */
/* Basic MD4 step. Transform buf based on in.
 */
#[c2rust::src_loc = "152:1"]
unsafe extern "C" fn Transform(mut buf: *mut krb5_ui_4,
                               mut in_0: *mut krb5_ui_4) {
    let mut a: krb5_ui_4 = *buf.offset(0 as libc::c_int as isize);
    let mut b: krb5_ui_4 = *buf.offset(1 as libc::c_int as isize);
    let mut c: krb5_ui_4 = *buf.offset(2 as libc::c_int as isize);
    let mut d: krb5_ui_4 = *buf.offset(3 as libc::c_int as isize);
    /* Round 1 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(0
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 7 as libc::c_int;
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(3
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 19 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 19 as libc::c_int;
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(4
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(5
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 7 as libc::c_int;
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(7
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 19 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 19 as libc::c_int;
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(9
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 7 as libc::c_int;
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(10
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(11
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 19 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 19 as libc::c_int;
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(12
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(13
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 7 as libc::c_int;
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(14
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(15
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 19 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 19 as libc::c_int;
    /* Round 2 */
    a =
        (a as
             libc::c_ulong).wrapping_add(((b & c | b & d |
                                               c &
                                                   d).wrapping_add(*in_0.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a & b | a & c |
                                               b &
                                                   c).wrapping_add(*in_0.offset(4
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 5 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d & a | d & b |
                                               a &
                                                   b).wrapping_add(*in_0.offset(8
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 9 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c & d | c & a |
                                               d &
                                                   a).wrapping_add(*in_0.offset(12
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 13 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 13 as libc::c_int;
    a =
        (a as
             libc::c_ulong).wrapping_add(((b & c | b & d |
                                               c &
                                                   d).wrapping_add(*in_0.offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a & b | a & c |
                                               b &
                                                   c).wrapping_add(*in_0.offset(5
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 5 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d & a | d & b |
                                               a &
                                                   b).wrapping_add(*in_0.offset(9
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 9 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c & d | c & a |
                                               d &
                                                   a).wrapping_add(*in_0.offset(13
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 13 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 13 as libc::c_int;
    a =
        (a as
             libc::c_ulong).wrapping_add(((b & c | b & d |
                                               c &
                                                   d).wrapping_add(*in_0.offset(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a & b | a & c |
                                               b &
                                                   c).wrapping_add(*in_0.offset(6
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 5 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d & a | d & b |
                                               a &
                                                   b).wrapping_add(*in_0.offset(10
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 9 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c & d | c & a |
                                               d &
                                                   a).wrapping_add(*in_0.offset(14
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 13 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 13 as libc::c_int;
    a =
        (a as
             libc::c_ulong).wrapping_add(((b & c | b & d |
                                               c &
                                                   d).wrapping_add(*in_0.offset(3
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a & b | a & c |
                                               b &
                                                   c).wrapping_add(*in_0.offset(7
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 5 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d & a | d & b |
                                               a &
                                                   b).wrapping_add(*in_0.offset(11
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 9 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c & d | c & a |
                                               d &
                                                   a).wrapping_add(*in_0.offset(15
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o13240474631
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 13 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 13 as libc::c_int;
    /* Round 3 */
    a =
        (a as
             libc::c_ulong).wrapping_add(((b ^ c ^
                                               d).wrapping_add(*in_0.offset(0
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a ^ b ^
                                               c).wrapping_add(*in_0.offset(8
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d ^ a ^
                                               b).wrapping_add(*in_0.offset(4
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c ^ d ^
                                               a).wrapping_add(*in_0.offset(12
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 15 as libc::c_int;
    a =
        (a as
             libc::c_ulong).wrapping_add(((b ^ c ^
                                               d).wrapping_add(*in_0.offset(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a ^ b ^
                                               c).wrapping_add(*in_0.offset(10
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d ^ a ^
                                               b).wrapping_add(*in_0.offset(6
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c ^ d ^
                                               a).wrapping_add(*in_0.offset(14
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 15 as libc::c_int;
    a =
        (a as
             libc::c_ulong).wrapping_add(((b ^ c ^
                                               d).wrapping_add(*in_0.offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a ^ b ^
                                               c).wrapping_add(*in_0.offset(9
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d ^ a ^
                                               b).wrapping_add(*in_0.offset(5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c ^ d ^
                                               a).wrapping_add(*in_0.offset(13
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 15 as libc::c_int;
    a =
        (a as
             libc::c_ulong).wrapping_add(((b ^ c ^
                                               d).wrapping_add(*in_0.offset(3
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 3 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 3 as libc::c_int;
    d =
        (d as
             libc::c_ulong).wrapping_add(((a ^ b ^
                                               c).wrapping_add(*in_0.offset(11
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    c =
        (c as
             libc::c_ulong).wrapping_add(((d ^ a ^
                                               b).wrapping_add(*in_0.offset(7
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 11 as libc::c_int;
    b =
        (b as
             libc::c_ulong).wrapping_add(((c ^ d ^
                                               a).wrapping_add(*in_0.offset(15
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))
                                              as
                                              libc::c_ulong).wrapping_add(0o15666365641
                                                                              as
                                                                              libc::c_ulong))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 15 as libc::c_int;
    let ref mut fresh3 = *buf.offset(0 as libc::c_int as isize);
    *fresh3 =
        (*fresh3 as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    let ref mut fresh4 = *buf.offset(1 as libc::c_int as isize);
    *fresh4 =
        (*fresh4 as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    let ref mut fresh5 = *buf.offset(2 as libc::c_int as isize);
    *fresh5 =
        (*fresh5 as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    let ref mut fresh6 = *buf.offset(3 as libc::c_int as isize);
    *fresh6 =
        (*fresh6 as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
}
/*
**********************************************************************
** End of md4.c                                                     **
******************************* (cut) ********************************
*/
