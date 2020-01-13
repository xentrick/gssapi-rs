use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:37"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:37"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:37"]
pub mod krb5_h {
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/md5/rsa-md5.h:38"]
pub mod rsa_md5_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:9"]
    pub struct krb5_MD5_CTX {
        pub i: [krb5_ui_4; 2],
        pub buf: [krb5_ui_4; 4],
        pub in_0: [libc::c_uchar; 64],
        pub digest: [libc::c_uchar; 16],
    }
    use super::krb5_h::krb5_ui_4;
    /* KRB5_RSA_MD5__ */
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_le,
                              load_32_le};
pub use self::krb5_h::krb5_ui_4;
pub use self::rsa_md5_h::krb5_MD5_CTX;
#[c2rust::src_loc = "54:28"]
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
/* F, G, H and I are basic MD5 functions */
/* ROTATE_LEFT rotates x left n bits */
/* FF, GG, HH, and II transformations for rounds 1, 2, 3, and 4 */
/* Rotation is separate from addition to prevent recomputation */
/* The routine krb5int_MD5Init initializes the message-digest context
   mdContext. All fields are set to zero.
*/
#[no_mangle]
#[c2rust::src_loc = "108:1"]
pub unsafe extern "C" fn krb5int_MD5Init(mut mdContext: *mut krb5_MD5_CTX) {
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
/* The routine krb5int_MD5Update updates the message-digest context to
   account for the presence of each of the characters inBuf[0..inLen-1]
   in the message whose digest is being computed.
*/
#[no_mangle]
#[c2rust::src_loc = "125:1"]
pub unsafe extern "C" fn krb5int_MD5Update(mut mdContext: *mut krb5_MD5_CTX,
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
/* The routine krb5int_MD5Final terminates the message-digest computation and
   ends with the desired message digest in mdContext->digest[0...15].
*/
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn krb5int_MD5Final(mut mdContext: *mut krb5_MD5_CTX) {
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
    krb5int_MD5Update(mdContext, PADDING.as_ptr(), padLen);
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
/*
 * Copyright (C) 1990, RSA Data Security, Inc. All rights reserved.
 *
 * License to copy and use this software is granted provided that
 * it is identified as the "RSA Data Security, Inc. MD5 Message-
 * Digest Algorithm" in all material mentioning or referencing this
 * software or this function.
 *
 * License is also granted to make and use derivative works
 * provided that such works are identified as "derived from the RSA
 * Data Security, Inc. MD5 Message-Digest Algorithm" in all
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
***********************************************************************
** md5.c -- the source code for MD5 routines                         **
** RSA Data Security, Inc. MD5 Message-Digest Algorithm              **
** Created: 2/17/90 RLR                                              **
** Revised: 1/91 SRD,AJ,BSK,JT Reference C ver., 7/10 constant corr. **
***********************************************************************
*/
/*
 * Modified by John Carr, MIT, to use Kerberos 5 typedefs.
 */
/*
***********************************************************************
**  Message-digest routines:                                         **
**  To form the message digest for a message M                       **
**    (1) Initialize a context buffer mdContext using krb5int_MD5Init   **
**    (2) Call krb5int_MD5Update on mdContext and M                     **
**    (3) Call krb5int_MD5Final on mdContext                            **
**  The message digest is now in mdContext->digest[0...15]           **
***********************************************************************
*/
/* forward declaration */
/* Basic MD5 step. Transforms buf based on in.
 */
#[c2rust::src_loc = "190:1"]
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
                                                                                  isize)).wrapping_add(3614090360
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 1 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(3905402710
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 12 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 2 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(606105819
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 17 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 3 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(3
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(3250441966
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 22 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 4 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(4
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(4118548399
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 5 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(5
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(1200080426
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 12 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 6 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(6
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(2821735955
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 17 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 7 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(7
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(4249261313
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 22 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 8 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(8
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(1770035416
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 9 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(9
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(2336552879
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 12 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 10 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(10
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(4294925233
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 17 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 11 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(11
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(2304563134
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 22 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 12 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & c |
                                             !b &
                                                 d).wrapping_add(*in_0.offset(12
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(1804603682
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 7 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 13 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & b |
                                             !a &
                                                 c).wrapping_add(*in_0.offset(13
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(4254626195
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 12 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 14 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & a |
                                             !d &
                                                 b).wrapping_add(*in_0.offset(14
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(2792965006
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 17 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 15 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & d |
                                             !c &
                                                 a).wrapping_add(*in_0.offset(15
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize)).wrapping_add(1236535329
                                                                                                           as
                                                                                                           libc::c_ulong
                                                                                                           as
                                                                                                           krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 22 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 16 */
    /* Round 2 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(4129170786
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 17 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(6
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(3225465664
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 18 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(11
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(643717713
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 14 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 19 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(3921069994
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 20 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 20 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(5
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(3593408605
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 21 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(10
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(38016083
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 22 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(15
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(3634488961
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 14 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 23 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(4
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(3889429448
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 20 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 24 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(9
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(568446438
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 25 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(14
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(3275163606
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 26 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(3
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(4107603335
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 14 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 27 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(8
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(1163531501
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 20 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 28 */
    a =
        (a as
             libc::c_uint).wrapping_add((b & d |
                                             c &
                                                 !d).wrapping_add(*in_0.offset(13
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(2850285829
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 5 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 29 */
    d =
        (d as
             libc::c_uint).wrapping_add((a & c |
                                             b &
                                                 !c).wrapping_add(*in_0.offset(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(4243563512
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 9 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 30 */
    c =
        (c as
             libc::c_uint).wrapping_add((d & b |
                                             a &
                                                 !b).wrapping_add(*in_0.offset(7
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(1735328473
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 14 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 31 */
    b =
        (b as
             libc::c_uint).wrapping_add((c & a |
                                             d &
                                                 !a).wrapping_add(*in_0.offset(12
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)).wrapping_add(2368359562
                                                                                                            as
                                                                                                            libc::c_ulong
                                                                                                            as
                                                                                                            krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 20 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 32 */
    /* Round 3 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(5 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(4294588738
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 4 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 33 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(8 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(2272392833
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 34 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(11
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(1839030562
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 16 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 35 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(14
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(4259657740
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 23 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 36 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(1 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(2763975236
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 4 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 37 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(4 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(1272893353
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 38 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(7 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(4139469664
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 16 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 39 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(10
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(3200236656
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 23 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 40 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(13
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(681279174
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 4 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 41 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(3936430074
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 42 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(3 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(3572445317
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 16 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 43 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(6 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(76029189
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 23 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 44 */
    a =
        (a as
             libc::c_uint).wrapping_add((b ^ c ^
                                             d).wrapping_add(*in_0.offset(9 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(3654602809
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 4 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 45 */
    d =
        (d as
             libc::c_uint).wrapping_add((a ^ b ^
                                             c).wrapping_add(*in_0.offset(12
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(3873151461
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 11 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 46 */
    c =
        (c as
             libc::c_uint).wrapping_add((d ^ a ^
                                             b).wrapping_add(*in_0.offset(15
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(530742520
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 16 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 47 */
    b =
        (b as
             libc::c_uint).wrapping_add((c ^ d ^
                                             a).wrapping_add(*in_0.offset(2 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)).wrapping_add(3299628645
                                                                                                       as
                                                                                                       libc::c_ulong
                                                                                                       as
                                                                                                       krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 23 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 48 */
    /* Round 4 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(0
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(4096336452
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 6 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 49 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(7
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(1126891415
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 10 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 50 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(14
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(2878612391
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 51 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(5
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(4237533241
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 21 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 52 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(12
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(1700485571
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 6 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 53 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(3
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(2399980690
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 10 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 54 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(10
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(4293915773
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 55 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(2240044497
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 21 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 56 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(8
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(1873313359
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 6 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 57 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(15
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(4264355552
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 10 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 58 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(6
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(2734768916
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 59 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(13
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(1309151649
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 21 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 60 */
    a =
        (a as
             libc::c_uint).wrapping_add((c ^
                                             (b |
                                                  !d)).wrapping_add(*in_0.offset(4
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(4149444226
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    a =
        a << 6 as libc::c_int & 0xffffffff as libc::c_uint |
            a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as krb5_ui_4 as krb5_ui_4;
    a &= 0xffffffff as libc::c_uint;
    /* 61 */
    d =
        (d as
             libc::c_uint).wrapping_add((b ^
                                             (a |
                                                  !c)).wrapping_add(*in_0.offset(11
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(3174756917
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    d =
        d << 10 as libc::c_int & 0xffffffff as libc::c_uint |
            d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as krb5_ui_4 as krb5_ui_4;
    d &= 0xffffffff as libc::c_uint;
    /* 62 */
    c =
        (c as
             libc::c_uint).wrapping_add((a ^
                                             (d |
                                                  !b)).wrapping_add(*in_0.offset(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(718787259
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    c =
        c << 15 as libc::c_int & 0xffffffff as libc::c_uint |
            c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as krb5_ui_4 as krb5_ui_4;
    c &= 0xffffffff as libc::c_uint;
    /* 63 */
    b =
        (b as
             libc::c_uint).wrapping_add((d ^
                                             (c |
                                                  !a)).wrapping_add(*in_0.offset(9
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     isize)).wrapping_add(3951481745
                                                                                                              as
                                                                                                              libc::c_ulong
                                                                                                              as
                                                                                                              krb5_ui_4))
            as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    b =
        b << 21 as libc::c_int & 0xffffffff as libc::c_uint |
            b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as krb5_ui_4 as krb5_ui_4;
    b &= 0xffffffff as libc::c_uint;
    /* 64 */
    /* small? */
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
