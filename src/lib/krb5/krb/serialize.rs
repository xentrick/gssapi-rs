use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
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
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:27"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "589:5"]
    pub struct C2RustUnnamed_0 {
        pub i: uint64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_1 {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "634:12"]
    pub struct C2RustUnnamed_2 {
        pub i: uint64_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "582:1"]
    pub unsafe extern "C" fn store_64_be(mut val: uint64_t,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed_0)).i = __bswap_64(val);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_1)).i);
    }
    #[inline]
    #[c2rust::src_loc = "627:1"]
    pub unsafe extern "C" fn load_64_be(mut cvp: *const libc::c_void)
     -> uint64_t {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_64((*(p as *const C2RustUnnamed_2)).i);
    }
    use super::stdint_uintn_h::{uint32_t, uint64_t};
    use super::byteswap_h::{__bswap_32, __bswap_64};
    /* K5_PLATFORM_H */
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
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:27"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "69:15"]
    pub unsafe extern "C" fn __bswap_64(mut __bsx: __uint64_t) -> __uint64_t {
        return ((__bsx as libc::c_ulonglong &
                     0xff00000000000000 as libc::c_ulonglong) >>
                    56 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000000000 as libc::c_ulonglong) >>
                        40 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff0000000000 as libc::c_ulonglong) >>
                        24 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff00000000 as libc::c_ulonglong) >>
                        8 as libc::c_int |
                    (__bsx as libc::c_ulonglong &
                         0xff000000 as libc::c_ulonglong) << 8 as libc::c_int
                    |
                    (__bsx as libc::c_ulonglong &
                         0xff0000 as libc::c_ulonglong) << 24 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                        << 40 as libc::c_int |
                    (__bsx as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                        << 56 as libc::c_int) as __uint64_t;
    }
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::{__uint64_t, __uint32_t};
}
pub use self::types_h::{__uint8_t, __int32_t, __uint32_t, __int64_t,
                        __uint64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
                              C2RustUnnamed_2, store_32_be, store_64_be,
                              load_32_be, load_64_be};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_error_code};
use self::string_h::memcpy;
pub use self::byteswap_h::{__bswap_64, __bswap_32};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/krb5/krb/serialize.c - Base serialization routines */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
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
 * krb5_ser_pack_int32()        - Pack a 4-byte integer if space is available.
 *                                Update buffer pointer and remaining space.
 */
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn krb5_ser_pack_int32(mut iarg: krb5_int32,
                                             mut bufp: *mut *mut krb5_octet,
                                             mut remainp: *mut size_t)
 -> krb5_error_code {
    if *remainp >= ::std::mem::size_of::<krb5_int32>() as libc::c_ulong {
        store_32_be(iarg as libc::c_uint, *bufp as *mut libc::c_void);
        *bufp =
            (*bufp).offset(::std::mem::size_of::<krb5_int32>() as
                               libc::c_ulong as isize);
        *remainp =
            (*remainp as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<krb5_int32>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        return 0 as libc::c_int
    } else { return 12 as libc::c_int };
}
/*
 * krb5_ser_pack_int64()        - Pack an 8-byte integer if space is available.
 *                                Update buffer pointer and remaining space.
 */
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn krb5_ser_pack_int64(mut iarg: int64_t,
                                             mut bufp: *mut *mut krb5_octet,
                                             mut remainp: *mut size_t)
 -> krb5_error_code {
    if *remainp >= ::std::mem::size_of::<int64_t>() as libc::c_ulong {
        store_64_be(iarg as uint64_t,
                    *bufp as *mut libc::c_uchar as *mut libc::c_void);
        *bufp =
            (*bufp).offset(::std::mem::size_of::<int64_t>() as libc::c_ulong
                               as isize);
        *remainp =
            (*remainp as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<int64_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        return 0 as libc::c_int
    } else { return 12 as libc::c_int };
}
/*
 * krb5_ser_pack_bytes()        - Pack a string of bytes.
 */
#[no_mangle]
#[c2rust::src_loc = "66:1"]
pub unsafe extern "C" fn krb5_ser_pack_bytes(mut ostring: *mut krb5_octet,
                                             mut osize: size_t,
                                             mut bufp: *mut *mut krb5_octet,
                                             mut remainp: *mut size_t)
 -> krb5_error_code {
    if *remainp >= osize {
        memcpy(*bufp as *mut libc::c_void, ostring as *const libc::c_void,
               osize);
        *bufp = (*bufp).offset(osize as isize);
        *remainp =
            (*remainp as libc::c_ulong).wrapping_sub(osize) as size_t as
                size_t;
        return 0 as libc::c_int
    } else { return 12 as libc::c_int };
}
/*
 * krb5_ser_unpack_int32()      - Unpack a 4-byte integer if it's there.
 */
#[no_mangle]
#[c2rust::src_loc = "82:1"]
pub unsafe extern "C" fn krb5_ser_unpack_int32(mut intp: *mut krb5_int32,
                                               mut bufp: *mut *mut krb5_octet,
                                               mut remainp: *mut size_t)
 -> krb5_error_code {
    if *remainp >= ::std::mem::size_of::<krb5_int32>() as libc::c_ulong {
        *intp = load_32_be(*bufp as *const libc::c_void) as krb5_int32;
        *bufp =
            (*bufp).offset(::std::mem::size_of::<krb5_int32>() as
                               libc::c_ulong as isize);
        *remainp =
            (*remainp as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<krb5_int32>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        return 0 as libc::c_int
    } else { return 12 as libc::c_int };
}
/*
 * krb5_ser_unpack_int64()      - Unpack an 8-byte integer if it's there.
 */
#[no_mangle]
#[c2rust::src_loc = "98:1"]
pub unsafe extern "C" fn krb5_ser_unpack_int64(mut intp: *mut int64_t,
                                               mut bufp: *mut *mut krb5_octet,
                                               mut remainp: *mut size_t)
 -> krb5_error_code {
    if *remainp >= ::std::mem::size_of::<int64_t>() as libc::c_ulong {
        *intp =
            load_64_be(*bufp as *mut libc::c_uchar as *const libc::c_void) as
                int64_t;
        *bufp =
            (*bufp).offset(::std::mem::size_of::<int64_t>() as libc::c_ulong
                               as isize);
        *remainp =
            (*remainp as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<int64_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        return 0 as libc::c_int
    } else { return 12 as libc::c_int };
}
/*
 * krb5_ser_unpack_bytes()      - Unpack a byte string if it's there.
 */
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn krb5_ser_unpack_bytes(mut istring: *mut krb5_octet,
                                               mut isize: size_t,
                                               mut bufp: *mut *mut krb5_octet,
                                               mut remainp: *mut size_t)
 -> krb5_error_code {
    if *remainp >= isize {
        memcpy(istring as *mut libc::c_void, *bufp as *const libc::c_void,
               isize);
        *bufp = (*bufp).offset(isize as isize);
        *remainp =
            (*remainp as libc::c_ulong).wrapping_sub(isize) as size_t as
                size_t;
        return 0 as libc::c_int
    } else { return 12 as libc::c_int };
}
