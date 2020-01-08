use ::libc;

pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;

pub use crate::stdlib::uint32_t;
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
/* This code has knowledge of the min and max errors of each type
within the gssapi major status */
/* */

static mut calling_error_string: [*const i8; 4] = [
    0 as *const i8,
    b"A required input parameter could not be read\x00" as *const u8 as *const i8,
    b"A required input parameter could not be written\x00" as *const u8 as *const i8,
    b"A parameter was malformed\x00" as *const u8 as *const i8,
];

static mut calling_error: *const i8 = b"calling error\x00" as *const u8 as *const i8;
/* */

static mut routine_error_string: [*const i8; 17] = [
    0 as *const i8,
    b"An unsupported mechanism was requested\x00" as *const u8 as *const i8,
    b"An invalid name was supplied\x00" as *const u8 as *const i8,
    b"A supplied name was of an unsupported type\x00" as *const u8 as *const i8,
    b"Incorrect channel bindings were supplied\x00" as *const u8 as *const i8,
    b"An invalid status code was supplied\x00" as *const u8 as *const i8,
    b"A token had an invalid signature\x00" as *const u8 as *const i8,
    b"No credentials were supplied\x00" as *const u8 as *const i8,
    b"No context has been established\x00" as *const u8 as *const i8,
    b"A token was invalid\x00" as *const u8 as *const i8,
    b"A credential was invalid\x00" as *const u8 as *const i8,
    b"The referenced credentials have expired\x00" as *const u8 as *const i8,
    b"The context has expired\x00" as *const u8 as *const i8,
    b"Miscellaneous failure\x00" as *const u8 as *const i8,
    b"The quality-of-protection requested could not be provided\x00" as *const u8 as *const i8,
    b"The operation is forbidden by the local security policy\x00" as *const u8 as *const i8,
    b"The operation or option is not available\x00" as *const u8 as *const i8,
];

static mut routine_error: *const i8 = b"routine error\x00" as *const u8 as *const i8;
/* */
/* this becomes overly gross after about 4 strings */

static mut sinfo_string: [*const i8; 4] = [
    b"The routine must be called again to complete its function\x00" as *const u8 as *const i8,
    b"The token was a duplicate of an earlier token\x00" as *const u8 as *const i8,
    b"The token\'s validity period has expired\x00" as *const u8 as *const i8,
    b"A later token has already been processed\x00" as *const u8 as *const i8,
];

static mut sinfo_code: *const i8 = b"supplementary info code\x00" as *const u8 as *const i8;
/* */
/* */

static mut no_error: *const i8 = b"No error\x00" as *const u8 as *const i8;

static mut unknown_error: *const i8 = b"Unknown %s (field = %d)\x00" as *const u8 as *const i8;
/* */

unsafe extern "C" fn display_unknown(
    mut kind: *const i8,
    mut value: crate::gssapi_h::OM_uint32,
    mut buffer: crate::gssapi_h::gss_buffer_t,
) -> i32 {
    let mut str = 0 as *mut i8;
    if crate::stdlib::asprintf(
        &mut str as *mut *mut i8,
        crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as *const i8, unknown_error),
        kind,
        value,
    ) < 0i32
    {
        return 0i32;
    }
    (*buffer).length = crate::stdlib::strlen(str);
    (*buffer).value = str as *mut libc::c_void;
    return 1i32;
}
/* code should be set to the calling error field */

unsafe extern "C" fn display_calling(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut code: crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut str = 0 as *const i8;
    str = if (code & (0o377u32) << 24i32) < (1u32) << 24i32
        || code & (0o377u32) << 24i32 > (3u32) << 24i32
    {
        0 as *mut i8
    } else {
        crate::stdlib::dgettext(
            b"mit-krb5\x00" as *const u8 as *const i8,
            calling_error_string[(code >> 24i32 & 0o377u32) as usize],
        )
    };
    if !str.is_null() {
        if crate::src::generic::util_buffer::gssint_g_make_string_buffer(str, status_string) == 0 {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
    } else if display_unknown(
        crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as *const i8, calling_error),
        code >> 24i32 & 0o377u32,
        status_string,
    ) == 0
    {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    *minor_status = 0u32;
    return 0u32;
}
/* code should be set to the routine error field */

unsafe extern "C" fn display_routine(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut code: crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut str = 0 as *const i8;
    str = if (code & (0o377u32) << 16i32) < (1u32) << 16i32
        || code & (0o377u32) << 16i32 > (13u32) << 16i32
    {
        0 as *mut i8
    } else {
        crate::stdlib::dgettext(
            b"mit-krb5\x00" as *const u8 as *const i8,
            routine_error_string[(code >> 16i32 & 0o377u32) as usize],
        )
    };
    if !str.is_null() {
        if crate::src::generic::util_buffer::gssint_g_make_string_buffer(str, status_string) == 0 {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
    } else if display_unknown(
        crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as *const i8, routine_error),
        code >> 16i32 & 0o377u32,
        status_string,
    ) == 0
    {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    *minor_status = 0u32;
    return 0u32;
}
/* code should be set to the bit offset (log_2) of a supplementary info bit */

unsafe extern "C" fn display_bit(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut code: crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut str = 0 as *const i8;
    str = if (1i32) << code < (1i32) << 0i32 + 0i32 || (1i32) << code > (1i32) << 0i32 + 3i32 {
        0 as *const i8
    } else {
        sinfo_string[code as usize]
    };
    if !str.is_null() {
        if crate::src::generic::util_buffer::gssint_g_make_string_buffer(str, status_string) == 0 {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
    } else if display_unknown(
        crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as *const i8, sinfo_code),
        ((1i32) << code) as crate::gssapi_h::OM_uint32,
        status_string,
    ) == 0
    {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    *minor_status = 0u32;
    return 0u32;
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
/* */
/* return error messages, for routine errors, call error, and status,
   in that order.
   message_context == 0 : print the routine error
   message_context == 1 : print the calling error
   message_context > 2  : print supplementary info bit (message_context-2)
*/
#[no_mangle]

pub unsafe extern "C" fn gssint_g_display_major_status(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut status_value: crate::gssapi_h::OM_uint32,
    mut message_context: *mut crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    let mut bit: i32 = 0;
    /* ** deal with no error at all specially */
    if status_value == 0u32 {
        if crate::src::generic::util_buffer::gssint_g_make_string_buffer(no_error, status_string)
            == 0
        {
            *minor_status = 12u32;
            return (13u32) << 16i32;
        }
        *message_context = 0u32;
        *minor_status = 0u32;
        return 0u32;
    }
    /* ** do routine error */
    if *message_context == 0u32 {
        tmp = status_value & (0o377u32) << 16i32;
        if tmp != 0 {
            status_value = (status_value).wrapping_sub(tmp);
            ret = display_routine(minor_status, tmp, status_string);
            if ret != 0 {
                return ret;
            }
            *minor_status = 0u32;
            if status_value != 0 {
                *message_context = (*message_context).wrapping_add(1);
                return 0u32;
            } else {
                *message_context = 0u32;
                return 0u32;
            }
        } else {
            *message_context = (*message_context).wrapping_add(1)
        }
    } else {
        status_value = (status_value).wrapping_sub(status_value & (0o377u32) << 16i32)
    }
    /* ** do calling error */
    if *message_context == 1u32 {
        tmp = status_value & (0o377u32) << 24i32;
        if tmp != 0 {
            status_value = (status_value).wrapping_sub(tmp);
            ret = display_calling(minor_status, tmp, status_string);
            if ret != 0 {
                return ret;
            }
            *minor_status = 0u32;
            if status_value != 0 {
                *message_context = (*message_context).wrapping_add(1);
                return 0u32;
            } else {
                *message_context = 0u32;
                return 0u32;
            }
        } else {
            *message_context = (*message_context).wrapping_add(1)
        }
    } else {
        status_value = (status_value).wrapping_sub(status_value & (0o377u32) << 24i32)
    }
    /* ** do sinfo bits (*message_context == 2 + number of bits done) */
    tmp = status_value >> 0i32 & 0o177777u32;
    /* mask off the bits which have been done */
    if *message_context > 2u32 {
        tmp &= !((1i32) << (*message_context).wrapping_sub(3u32)
            ^ ((1i32) << (*message_context).wrapping_sub(3u32)) - 1i32) as u32;
        status_value &= !((1i32) << (*message_context).wrapping_sub(3u32)
            ^ ((1i32) << (*message_context).wrapping_sub(3u32)) - 1i32)
            as u32
    }
    if tmp == 0 {
        /* bogon input - there should be something left */
        *minor_status = -(2045022971 as isize) as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    /* compute the bit offset */
    /*SUPPRESS 570*/
    bit = 0i32;
    while (1u32) << bit != (tmp ^ tmp.wrapping_sub(1u32)).wrapping_add(1u32) >> 1i32 {
        bit += 1
    }
    /* print it */
    ret = display_bit(
        minor_status,
        bit as crate::gssapi_h::OM_uint32,
        status_string,
    );
    if ret != 0 {
        return ret;
    }
    /* compute the new status_value/message_context */
    status_value = (status_value).wrapping_sub((1u32) << bit);
    if status_value != 0 {
        *message_context = (bit + 3i32) as crate::gssapi_h::OM_uint32;
        return 0u32;
    } else {
        *message_context = 0u32;
        return 0u32;
    };
}
