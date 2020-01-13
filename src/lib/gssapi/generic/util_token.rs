use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:24"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:24"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:24"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:24"]
pub mod gssapi_h {
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "92:1"]
    pub type gss_int32 = int32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:24"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
    }
}
pub use self::types_h::{__int32_t, __uint32_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, gss_int32, OM_uint32,
                         gss_OID_desc_struct, gss_OID_desc};
use self::string_h::{memcpy, memcmp};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * $Id$
 */
/* XXXX this code currently makes the assumption that a mech oid will
   never be longer than 127 bytes.  This assumption is not inherent in
   the interfaces, so the code can be fixed if the OSI namespace
   balloons unexpectedly. */
/*
 * Each token looks like this:
 * 0x60                 tag for APPLICATION 0, SEQUENCE
 *                              (constructed, definite-length)
 * <length>             possible multiple bytes, need to parse/generate
 * 0x06                 tag for OBJECT IDENTIFIER
 * <moid_length>        compile-time constant string (assume 1 byte)
 * <moid_bytes>         compile-time constant string
 * <inner_bytes>        the ANY containing the application token
 * bytes 0,1 are the token type
 * bytes 2,n are the token data
 *
 * Note that the token type field is a feature of RFC 1964 mechanisms and
 * is not used by other GSSAPI mechanisms.  As such, a token type of -1
 * is interpreted to mean that no token type should be expected or
 * generated.
 *
 * For the purposes of this abstraction, the token "header" consists of
 * the sequence tag and length octets, the mech OID DER encoding, and the
 * first two inner bytes, which indicate the token type.  The token
 * "body" consists of everything else.
 */
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn der_length_size(mut length: libc::c_int)
 -> libc::c_uint {
    if length < (1 as libc::c_int) << 7 as libc::c_int {
        return 1 as libc::c_int as libc::c_uint
    } else if length < (1 as libc::c_int) << 8 as libc::c_int {
        return 2 as libc::c_int as libc::c_uint
    } else if length < (1 as libc::c_int) << 16 as libc::c_int {
        return 3 as libc::c_int as libc::c_uint
    } else if length < (1 as libc::c_int) << 24 as libc::c_int {
        return 4 as libc::c_int as libc::c_uint
    } else { return 5 as libc::c_int as libc::c_uint };
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn der_write_length(mut buf: *mut *mut libc::c_uchar,
                                      mut length: libc::c_int) {
    if length < (1 as libc::c_int) << 7 as libc::c_int {
        let fresh0 = *buf;
        *buf = (*buf).offset(1);
        *fresh0 = length as libc::c_uchar
    } else {
        let fresh1 = *buf;
        *buf = (*buf).offset(1);
        *fresh1 =
            der_length_size(length).wrapping_add(127 as libc::c_int as
                                                     libc::c_uint) as
                libc::c_uchar;
        if length >= (1 as libc::c_int) << 24 as libc::c_int {
            let fresh2 = *buf;
            *buf = (*buf).offset(1);
            *fresh2 = (length >> 24 as libc::c_int) as libc::c_uchar
        }
        if length >= (1 as libc::c_int) << 16 as libc::c_int {
            let fresh3 = *buf;
            *buf = (*buf).offset(1);
            *fresh3 =
                (length >> 16 as libc::c_int & 0xff as libc::c_int) as
                    libc::c_uchar
        }
        if length >= (1 as libc::c_int) << 8 as libc::c_int {
            let fresh4 = *buf;
            *buf = (*buf).offset(1);
            *fresh4 =
                (length >> 8 as libc::c_int & 0xff as libc::c_int) as
                    libc::c_uchar
        }
        let fresh5 = *buf;
        *buf = (*buf).offset(1);
        *fresh5 = (length & 0xff as libc::c_int) as libc::c_uchar
    };
}
/* returns decoded length, or < 0 on failure.  Advances buf and
   decrements bufsize */
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn der_read_length(mut buf: *mut *mut libc::c_uchar,
                                     mut bufsize: *mut libc::c_int)
 -> libc::c_int {
    let mut sf: libc::c_uchar = 0;
    let mut ret: libc::c_int = 0;
    if *bufsize < 1 as libc::c_int { return -(1 as libc::c_int) }
    let fresh6 = *buf;
    *buf = (*buf).offset(1);
    sf = *fresh6;
    *bufsize -= 1;
    if sf as libc::c_int & 0x80 as libc::c_int != 0 {
        sf = (sf as libc::c_int & 0x7f as libc::c_int) as libc::c_uchar;
        if sf as libc::c_int > *bufsize - 1 as libc::c_int {
            return -(1 as libc::c_int)
        }
        if sf as libc::c_ulong >
               ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
            return -(1 as libc::c_int)
        }
        ret = 0 as libc::c_int;
        while sf != 0 {
            let fresh7 = *buf;
            *buf = (*buf).offset(1);
            ret = (ret << 8 as libc::c_int) + *fresh7 as libc::c_int;
            *bufsize -= 1;
            sf = sf.wrapping_sub(1)
        }
    } else { ret = sf as libc::c_int }
    return ret;
}
/* returns the length of a token, given the mech oid and the body size */
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn gssint_g_token_size(mut mech: *const gss_OID_desc,
                                             mut body_size: libc::c_uint)
 -> libc::c_uint {
    /* set body_size to sequence contents size */
    body_size =
        body_size.wrapping_add((4 as libc::c_int as
                                    libc::c_uint).wrapping_add((*mech).length)); /* NEED overflow check */
    return (1 as libc::c_int as
                libc::c_uint).wrapping_add(der_length_size(body_size as
                                                               libc::c_int)).wrapping_add(body_size);
}
/* fills in a buffer with the token header.  The buffer is assumed to
   be the right size.  buf is advanced past the token header */
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn gssint_g_make_token_header(mut mech:
                                                        *const gss_OID_desc,
                                                    mut body_size:
                                                        libc::c_uint,
                                                    mut buf:
                                                        *mut *mut libc::c_uchar,
                                                    mut tok_type:
                                                        libc::c_int) {
    let fresh8 = *buf;
    *buf = (*buf).offset(1);
    *fresh8 = 0x60 as libc::c_int as libc::c_uchar;
    der_write_length(buf,
                     ((if tok_type == -(1 as libc::c_int) {
                           2 as libc::c_int
                       } else { 4 as libc::c_int }) as
                          libc::c_uint).wrapping_add((*mech).length).wrapping_add(body_size)
                         as libc::c_int);
    let fresh9 = *buf;
    *buf = (*buf).offset(1);
    *fresh9 = 0x6 as libc::c_int as libc::c_uchar;
    let fresh10 = *buf;
    *buf = (*buf).offset(1);
    *fresh10 = (*mech).length as libc::c_uchar;
    memcpy(*buf as *mut libc::c_void, (*mech).elements,
           (*mech).length as libc::c_ulong);
    *buf = (*buf).offset((*mech).length as isize);
    if tok_type != -(1 as libc::c_int) {
        let fresh11 = *buf;
        *buf = (*buf).offset(1);
        *fresh11 =
            (tok_type >> 8 as libc::c_int & 0xff as libc::c_int) as
                libc::c_uchar;
        let fresh12 = *buf;
        *buf = (*buf).offset(1);
        *fresh12 = (tok_type & 0xff as libc::c_int) as libc::c_uchar
    };
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * $Id$
 */
/* * helper macros **/
/* this code knows that an int on the wire is 32 bits.  The type of
   num should be at least this big, or the extra shifts may do weird
   things */
/* * malloc wrappers; these may actually do something later */
/* * helper functions **/
/* hide names from applications, especially glib applications */
/* flags for g_verify_token_header() */
/*
 * Given a buffer containing a token, reads and verifies the token,
 * leaving buf advanced past the token header, and setting body_size
 * to the number of remaining bytes.  Returns 0 on success,
 * G_BAD_TOK_HEADER for a variety of errors, and G_WRONG_MECH if the
 * mechanism in the token does not match the mech argument.  buf and
 * *body_size are left unmodified on error.
 */
#[no_mangle]
#[c2rust::src_loc = "170:1"]
pub unsafe extern "C" fn gssint_g_verify_token_header(mut mech:
                                                          *const gss_OID_desc,
                                                      mut body_size:
                                                          *mut libc::c_uint,
                                                      mut buf_in:
                                                          *mut *mut libc::c_uchar,
                                                      mut tok_type:
                                                          libc::c_int,
                                                      mut toksize_in:
                                                          libc::c_uint,
                                                      mut flags: libc::c_int)
 -> gss_int32 {
    let mut buf: *mut libc::c_uchar = *buf_in;
    let mut seqsize: libc::c_int = 0;
    let mut toid: gss_OID_desc =
        gss_OID_desc{length: 0, elements: 0 as *mut libc::c_void,};
    let mut toksize: libc::c_int = toksize_in as libc::c_int;
    toksize -= 1 as libc::c_int;
    if toksize < 0 as libc::c_int {
        return -(2045022964 as libc::c_long) as gss_int32
    }
    let fresh13 = buf;
    buf = buf.offset(1);
    if *fresh13 as libc::c_int != 0x60 as libc::c_int {
        if flags & 0x1 as libc::c_int != 0 {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        buf = buf.offset(-1);
        toksize += 1
    } else {
        seqsize = der_read_length(&mut buf, &mut toksize);
        if seqsize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        if seqsize != toksize {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        toksize -= 1 as libc::c_int;
        if toksize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        let fresh14 = buf;
        buf = buf.offset(1);
        if *fresh14 as libc::c_int != 0x6 as libc::c_int {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        toksize -= 1 as libc::c_int;
        if toksize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        let fresh15 = buf;
        buf = buf.offset(1);
        toid.length = *fresh15 as OM_uint32;
        toksize =
            (toksize as libc::c_uint).wrapping_sub(toid.length) as libc::c_int
                as libc::c_int;
        if toksize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        toid.elements = buf as *mut libc::c_void;
        buf = buf.offset(toid.length as isize);
        if !(toid.length == (*mech).length &&
                 memcmp(toid.elements, (*mech).elements,
                        toid.length as libc::c_ulong) == 0 as libc::c_int) {
            return -(2045022965 as libc::c_long) as gss_int32
        }
    }
    if tok_type != -(1 as libc::c_int) {
        toksize -= 2 as libc::c_int;
        if toksize < 0 as libc::c_int {
            return -(2045022964 as libc::c_long) as gss_int32
        }
        let fresh16 = buf;
        buf = buf.offset(1);
        if *fresh16 as libc::c_int !=
               tok_type >> 8 as libc::c_int & 0xff as libc::c_int ||
               {
                   let fresh17 = buf;
                   buf = buf.offset(1);
                   (*fresh17 as libc::c_int) != tok_type & 0xff as libc::c_int
               } {
            return -(2045022960 as libc::c_long) as gss_int32
        }
    }
    *buf_in = buf;
    *body_size = toksize as libc::c_uint;
    return 0 as libc::c_int;
}
