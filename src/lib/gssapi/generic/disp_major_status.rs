use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:24"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:24"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:24"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:24"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:24"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn gssint_g_make_string_buffer(str: *const libc::c_char,
                                           buffer: gss_buffer_t)
         -> libc::c_int;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/libintl.h:24"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_buffer_desc_struct,
                         gss_buffer_t};
use self::stdio_h::asprintf;
use self::string_h::strlen;
use self::gssapiP_generic_h::gssint_g_make_string_buffer;
use self::libintl_h::dgettext;
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
#[c2rust::src_loc = "41:27"]
static mut calling_error_string: [*const libc::c_char; 4] =
    [0 as *const libc::c_char,
     b"A required input parameter could not be read\x00" as *const u8 as
         *const libc::c_char,
     b"A required input parameter could not be written\x00" as *const u8 as
         *const libc::c_char,
     b"A parameter was malformed\x00" as *const u8 as *const libc::c_char];
#[c2rust::src_loc = "48:27"]
static mut calling_error: *const libc::c_char =
    b"calling error\x00" as *const u8 as *const libc::c_char;
/* */
#[c2rust::src_loc = "57:27"]
static mut routine_error_string: [*const libc::c_char; 17] =
    [0 as *const libc::c_char,
     b"An unsupported mechanism was requested\x00" as *const u8 as
         *const libc::c_char,
     b"An invalid name was supplied\x00" as *const u8 as *const libc::c_char,
     b"A supplied name was of an unsupported type\x00" as *const u8 as
         *const libc::c_char,
     b"Incorrect channel bindings were supplied\x00" as *const u8 as
         *const libc::c_char,
     b"An invalid status code was supplied\x00" as *const u8 as
         *const libc::c_char,
     b"A token had an invalid signature\x00" as *const u8 as
         *const libc::c_char,
     b"No credentials were supplied\x00" as *const u8 as *const libc::c_char,
     b"No context has been established\x00" as *const u8 as
         *const libc::c_char,
     b"A token was invalid\x00" as *const u8 as *const libc::c_char,
     b"A credential was invalid\x00" as *const u8 as *const libc::c_char,
     b"The referenced credentials have expired\x00" as *const u8 as
         *const libc::c_char,
     b"The context has expired\x00" as *const u8 as *const libc::c_char,
     b"Miscellaneous failure\x00" as *const u8 as *const libc::c_char,
     b"The quality-of-protection requested could not be provided\x00" as
         *const u8 as *const libc::c_char,
     b"The operation is forbidden by the local security policy\x00" as
         *const u8 as *const libc::c_char,
     b"The operation or option is not available\x00" as *const u8 as
         *const libc::c_char];
#[c2rust::src_loc = "77:27"]
static mut routine_error: *const libc::c_char =
    b"routine error\x00" as *const u8 as *const libc::c_char;
/* */
/* this becomes overly gross after about 4 strings */
#[c2rust::src_loc = "88:27"]
static mut sinfo_string: [*const libc::c_char; 4] =
    [b"The routine must be called again to complete its function\x00" as
         *const u8 as *const libc::c_char,
     b"The token was a duplicate of an earlier token\x00" as *const u8 as
         *const libc::c_char,
     b"The token\'s validity period has expired\x00" as *const u8 as
         *const libc::c_char,
     b"A later token has already been processed\x00" as *const u8 as
         *const libc::c_char];
#[c2rust::src_loc = "95:27"]
static mut sinfo_code: *const libc::c_char =
    b"supplementary info code\x00" as *const u8 as *const libc::c_char;
/* */
/* */
#[c2rust::src_loc = "106:27"]
static mut no_error: *const libc::c_char =
    b"No error\x00" as *const u8 as *const libc::c_char;
#[c2rust::src_loc = "107:27"]
static mut unknown_error: *const libc::c_char =
    b"Unknown %s (field = %d)\x00" as *const u8 as *const libc::c_char;
/* */
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn display_unknown(mut kind: *const libc::c_char,
                                     mut value: OM_uint32,
                                     mut buffer: gss_buffer_t)
 -> libc::c_int {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if asprintf(&mut str as *mut *mut libc::c_char,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         unknown_error), kind, value) < 0 as libc::c_int {
        return 0 as libc::c_int
    }
    (*buffer).length = strlen(str);
    (*buffer).value = str as *mut libc::c_void;
    return 1 as libc::c_int;
}
/* code should be set to the calling error field */
#[c2rust::src_loc = "127:1"]
unsafe extern "C" fn display_calling(mut minor_status: *mut OM_uint32,
                                     mut code: OM_uint32,
                                     mut status_string: gss_buffer_t)
 -> OM_uint32 {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    str =
        if (code & (0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int)
               < (1 as libc::c_ulong as OM_uint32) << 24 as libc::c_int ||
               code &
                   (0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int
                   > (3 as libc::c_ulong as OM_uint32) << 24 as libc::c_int {
            0 as *mut libc::c_char
        } else {
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     calling_error_string[(code >> 24 as libc::c_int &
                                               0o377 as libc::c_ulong as
                                                   OM_uint32) as usize])
        };
    if !str.is_null() {
        if gssint_g_make_string_buffer(str, status_string) == 0 {
            *minor_status = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if display_unknown(dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char,
                                       calling_error),
                              code >> 24 as libc::c_int &
                                  0o377 as libc::c_ulong as OM_uint32,
                              status_string) == 0 {
        *minor_status = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
/* code should be set to the routine error field */
#[c2rust::src_loc = "151:1"]
unsafe extern "C" fn display_routine(mut minor_status: *mut OM_uint32,
                                     mut code: OM_uint32,
                                     mut status_string: gss_buffer_t)
 -> OM_uint32 {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    str =
        if (code & (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int)
               < (1 as libc::c_ulong as OM_uint32) << 16 as libc::c_int ||
               code &
                   (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
                   > (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int {
            0 as *mut libc::c_char
        } else {
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     routine_error_string[(code >> 16 as libc::c_int &
                                               0o377 as libc::c_ulong as
                                                   OM_uint32) as usize])
        };
    if !str.is_null() {
        if gssint_g_make_string_buffer(str, status_string) == 0 {
            *minor_status = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if display_unknown(dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char,
                                       routine_error),
                              code >> 16 as libc::c_int &
                                  0o377 as libc::c_ulong as OM_uint32,
                              status_string) == 0 {
        *minor_status = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
}
/* code should be set to the bit offset (log_2) of a supplementary info bit */
#[c2rust::src_loc = "175:1"]
unsafe extern "C" fn display_bit(mut minor_status: *mut OM_uint32,
                                 mut code: OM_uint32,
                                 mut status_string: gss_buffer_t)
 -> OM_uint32 {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    str =
        if (1 as libc::c_int) << code <
               (1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int ||
               (1 as libc::c_int) << code >
                   (1 as libc::c_int) << 0 as libc::c_int + 3 as libc::c_int {
            0 as *const libc::c_char
        } else { sinfo_string[code as usize] };
    if !str.is_null() {
        if gssint_g_make_string_buffer(str, status_string) == 0 {
            *minor_status = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
    } else if display_unknown(dgettext(b"mit-krb5\x00" as *const u8 as
                                           *const libc::c_char, sinfo_code),
                              ((1 as libc::c_int) << code) as OM_uint32,
                              status_string) == 0 {
        *minor_status = 12 as libc::c_int as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    *minor_status = 0 as libc::c_int as OM_uint32;
    return 0 as libc::c_int as OM_uint32;
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
#[c2rust::src_loc = "205:1"]
pub unsafe extern "C" fn gssint_g_display_major_status(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut status_value:
                                                           OM_uint32,
                                                       mut message_context:
                                                           *mut OM_uint32,
                                                       mut status_string:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut ret: OM_uint32 = 0;
    let mut tmp: OM_uint32 = 0;
    let mut bit: libc::c_int = 0;
    /* ** deal with no error at all specially */
    if status_value == 0 as libc::c_int as libc::c_uint {
        if gssint_g_make_string_buffer(no_error, status_string) == 0 {
            *minor_status = 12 as libc::c_int as OM_uint32;
            return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
        }
        *message_context = 0 as libc::c_int as OM_uint32;
        *minor_status = 0 as libc::c_int as OM_uint32;
        return 0 as libc::c_int as OM_uint32
    }
    /* ** do routine error */
    if *message_context == 0 as libc::c_int as libc::c_uint {
        tmp =
            status_value &
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int;
        if tmp != 0 {
            status_value =
                (status_value as libc::c_uint).wrapping_sub(tmp) as OM_uint32
                    as OM_uint32;
            ret = display_routine(minor_status, tmp, status_string);
            if ret != 0 { return ret }
            *minor_status = 0 as libc::c_int as OM_uint32;
            if status_value != 0 {
                *message_context = (*message_context).wrapping_add(1);
                return 0 as libc::c_int as OM_uint32
            } else {
                *message_context = 0 as libc::c_int as OM_uint32;
                return 0 as libc::c_int as OM_uint32
            }
        } else { *message_context = (*message_context).wrapping_add(1) }
    } else {
        status_value =
            (status_value as
                 libc::c_uint).wrapping_sub(status_value &
                                                (0o377 as libc::c_ulong as
                                                     OM_uint32) <<
                                                    16 as libc::c_int) as
                OM_uint32 as OM_uint32
    }
    /* ** do calling error */
    if *message_context == 1 as libc::c_int as libc::c_uint {
        tmp =
            status_value &
                (0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int;
        if tmp != 0 {
            status_value =
                (status_value as libc::c_uint).wrapping_sub(tmp) as OM_uint32
                    as OM_uint32;
            ret = display_calling(minor_status, tmp, status_string);
            if ret != 0 { return ret }
            *minor_status = 0 as libc::c_int as OM_uint32;
            if status_value != 0 {
                *message_context = (*message_context).wrapping_add(1);
                return 0 as libc::c_int as OM_uint32
            } else {
                *message_context = 0 as libc::c_int as OM_uint32;
                return 0 as libc::c_int as OM_uint32
            }
        } else { *message_context = (*message_context).wrapping_add(1) }
    } else {
        status_value =
            (status_value as
                 libc::c_uint).wrapping_sub(status_value &
                                                (0o377 as libc::c_ulong as
                                                     OM_uint32) <<
                                                    24 as libc::c_int) as
                OM_uint32 as OM_uint32
    }
    /* ** do sinfo bits (*message_context == 2 + number of bits done) */
    tmp =
        status_value >> 0 as libc::c_int &
            0o177777 as libc::c_ulong as OM_uint32;
    /* mask off the bits which have been done */
    if *message_context > 2 as libc::c_int as libc::c_uint {
        tmp &=
            !((1 as libc::c_int) <<
                  (*message_context).wrapping_sub(3 as libc::c_int as
                                                      libc::c_uint) ^
                  ((1 as libc::c_int) <<
                       (*message_context).wrapping_sub(3 as libc::c_int as
                                                           libc::c_uint)) -
                      1 as libc::c_int) as libc::c_uint;
        status_value &=
            !((1 as libc::c_int) <<
                  (*message_context).wrapping_sub(3 as libc::c_int as
                                                      libc::c_uint) ^
                  ((1 as libc::c_int) <<
                       (*message_context).wrapping_sub(3 as libc::c_int as
                                                           libc::c_uint)) -
                      1 as libc::c_int) as libc::c_uint
    }
    if tmp == 0 {
        /* bogon input - there should be something left */
        *minor_status = -(2045022971 as libc::c_long) as OM_uint32;
        return (13 as libc::c_ulong as OM_uint32) << 16 as libc::c_int
    }
    /* compute the bit offset */
    /*SUPPRESS 570*/
    bit = 0 as libc::c_int;
    while (1 as libc::c_int as OM_uint32) << bit !=
              (tmp ^
                   tmp.wrapping_sub(1 as libc::c_int as
                                        libc::c_uint)).wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                  >> 1 as libc::c_int {
        bit += 1
    }
    /* print it */
    ret = display_bit(minor_status, bit as OM_uint32, status_string);
    if ret != 0 { return ret }
    /* compute the new status_value/message_context */
    status_value =
        (status_value as
             libc::c_uint).wrapping_sub((1 as libc::c_int as OM_uint32) <<
                                            bit) as OM_uint32 as OM_uint32;
    if status_value != 0 {
        *message_context = (bit + 3 as libc::c_int) as OM_uint32;
        return 0 as libc::c_int as OM_uint32
    } else {
        *message_context = 0 as libc::c_int as OM_uint32;
        return 0 as libc::c_int as OM_uint32
    };
}
