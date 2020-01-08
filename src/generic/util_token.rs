use ::libc;

pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_int32;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

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

unsafe extern "C" fn der_length_size(mut length: i32) -> u32 {
    if length < (1i32) << 7i32 {
        return 1u32;
    } else if length < (1i32) << 8i32 {
        return 2u32;
    } else if length < (1i32) << 16i32 {
        return 3u32;
    } else if length < (1i32) << 24i32 {
        return 4u32;
    } else {
        return 5u32;
    };
}

unsafe extern "C" fn der_write_length(mut buf: *mut *mut u8, mut length: i32) {
    if length < (1i32) << 7i32 {
        let fresh0 = *buf;
        *buf = (*buf).offset(1);
        *fresh0 = length as u8
    } else {
        let fresh1 = *buf;
        *buf = (*buf).offset(1);
        *fresh1 = der_length_size(length).wrapping_add(127u32) as u8;
        if length >= (1i32) << 24i32 {
            let fresh2 = *buf;
            *buf = (*buf).offset(1);
            *fresh2 = (length >> 24i32) as u8
        }
        if length >= (1i32) << 16i32 {
            let fresh3 = *buf;
            *buf = (*buf).offset(1);
            *fresh3 = (length >> 16i32 & 0xffi32) as u8
        }
        if length >= (1i32) << 8i32 {
            let fresh4 = *buf;
            *buf = (*buf).offset(1);
            *fresh4 = (length >> 8i32 & 0xffi32) as u8
        }
        let fresh5 = *buf;
        *buf = (*buf).offset(1);
        *fresh5 = (length & 0xffi32) as u8
    };
}
/* returns decoded length, or < 0 on failure.  Advances buf and
decrements bufsize */

unsafe extern "C" fn der_read_length(mut buf: *mut *mut u8, mut bufsize: *mut i32) -> i32 {
    let mut sf: u8 = 0;
    let mut ret: i32 = 0;
    if *bufsize < 1i32 {
        return -(1i32);
    }
    let fresh6 = *buf;
    *buf = (*buf).offset(1);
    sf = *fresh6;
    *bufsize -= 1;
    if sf as i32 & 0x80i32 != 0 {
        sf = (sf as i32 & 0x7fi32) as u8;
        if sf as i32 > *bufsize - 1i32 {
            return -(1i32);
        }
        if sf as usize > ::std::mem::size_of::<i32>() {
            return -(1i32);
        }
        ret = 0i32;
        while sf != 0 {
            let fresh7 = *buf;
            *buf = (*buf).offset(1);
            ret = (ret << 8i32) + *fresh7 as i32;
            *bufsize -= 1;
            sf = sf.wrapping_sub(1)
        }
    } else {
        ret = sf as i32
    }
    return ret;
}
/* returns the length of a token, given the mech oid and the body size */
#[no_mangle]

pub unsafe extern "C" fn gssint_g_token_size(
    mut mech: *const crate::gssapi_h::gss_OID_desc,
    mut body_size: u32,
) -> u32 {
    /* set body_size to sequence contents size */
    body_size = body_size.wrapping_add((4u32).wrapping_add((*mech).length)); /* NEED overflow check */
    return (1u32)
        .wrapping_add(der_length_size(body_size as i32))
        .wrapping_add(body_size);
}
/* fills in a buffer with the token header.  The buffer is assumed to
be the right size.  buf is advanced past the token header */
#[no_mangle]

pub unsafe extern "C" fn gssint_g_make_token_header(
    mut mech: *const crate::gssapi_h::gss_OID_desc,
    mut body_size: u32,
    mut buf: *mut *mut u8,
    mut tok_type: i32,
) {
    let fresh8 = *buf;
    *buf = (*buf).offset(1);
    *fresh8 = 0x60u8;
    der_write_length(
        buf,
        ((if tok_type == -(1i32) { 2i32 } else { 4i32 }) as u32)
            .wrapping_add((*mech).length)
            .wrapping_add(body_size) as i32,
    );
    let fresh9 = *buf;
    *buf = (*buf).offset(1);
    *fresh9 = 0x6u8;
    let fresh10 = *buf;
    *buf = (*buf).offset(1);
    *fresh10 = (*mech).length as u8;
    crate::stdlib::memcpy(
        *buf as *mut libc::c_void,
        (*mech).elements,
        (*mech).length as usize,
    );
    *buf = (*buf).offset((*mech).length as isize);
    if tok_type != -(1i32) {
        let fresh11 = *buf;
        *buf = (*buf).offset(1);
        *fresh11 = (tok_type >> 8i32 & 0xffi32) as u8;
        let fresh12 = *buf;
        *buf = (*buf).offset(1);
        *fresh12 = (tok_type & 0xffi32) as u8
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

pub unsafe extern "C" fn gssint_g_verify_token_header(
    mut mech: *const crate::gssapi_h::gss_OID_desc,
    mut body_size: *mut u32,
    mut buf_in: *mut *mut u8,
    mut tok_type: i32,
    mut toksize_in: u32,
    mut flags: i32,
) -> crate::gssapi_h::gss_int32 {
    let mut buf = *buf_in;
    let mut seqsize: i32 = 0;
    let mut toid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut toksize = toksize_in as i32;
    toksize -= 1i32;
    if toksize < 0i32 {
        return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
    }
    let fresh13 = buf;
    buf = buf.offset(1);
    if *fresh13 as i32 != 0x60i32 {
        if flags & 0x1i32 != 0 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        buf = buf.offset(-1);
        toksize += 1
    } else {
        seqsize = der_read_length(&mut buf, &mut toksize);
        if seqsize < 0i32 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        if seqsize != toksize {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        toksize -= 1i32;
        if toksize < 0i32 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        let fresh14 = buf;
        buf = buf.offset(1);
        if *fresh14 as i32 != 0x6i32 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        toksize -= 1i32;
        if toksize < 0i32 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        let fresh15 = buf;
        buf = buf.offset(1);
        toid.length = *fresh15 as crate::gssapi_h::OM_uint32;
        toksize = (toksize as u32).wrapping_sub(toid.length) as i32;
        if toksize < 0i32 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        toid.elements = buf as *mut libc::c_void;
        buf = buf.offset(toid.length as isize);
        if !(toid.length == (*mech).length
            && crate::stdlib::memcmp(toid.elements, (*mech).elements, toid.length as usize) == 0i32)
        {
            return -(2045022965 as isize) as crate::gssapi_h::gss_int32;
        }
    }
    if tok_type != -(1i32) {
        toksize -= 2i32;
        if toksize < 0i32 {
            return -(2045022964 as isize) as crate::gssapi_h::gss_int32;
        }
        let fresh16 = buf;
        buf = buf.offset(1);
        if *fresh16 as i32 != tok_type >> 8i32 & 0xffi32 || {
            let fresh17 = buf;
            buf = buf.offset(1);
            (*fresh17 as i32) != tok_type & 0xffi32
        } {
            return -(2045022960 as isize) as crate::gssapi_h::gss_int32;
        }
    }
    *buf_in = buf;
    *body_size = toksize as u32;
    return 0i32;
}
