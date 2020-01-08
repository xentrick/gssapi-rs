use ::libc;

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    /* not _WIN32 or DEBUG_GSSALLOC */
    #[inline]

    pub unsafe extern "C" fn gssalloc_strdup(mut str: *const i8) -> *mut i8 {
        let mut size = crate::stdlib::strlen(str).wrapping_add(1usize);
        let mut copy = gssalloc_malloc(size) as *mut i8;
        if !copy.is_null() {
            crate::stdlib::memcpy(copy as *mut libc::c_void, str as *const libc::c_void, size);
            *copy.offset(size.wrapping_sub(1usize) as isize) = '\u{0}' as i8
        }
        return copy;
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_ext_h::gss_any;
pub use crate::gssapi_ext_h::gss_any_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_ext_h::gss_const_key_value_set_t;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_key_value_element_desc;
pub use crate::gssapi_ext_h::gss_key_value_element_struct;
pub use crate::gssapi_ext_h::gss_key_value_set_desc;
pub use crate::gssapi_ext_h::gss_key_value_set_struct;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;

pub use crate::src::mechglue::g_dsp_status::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::mechglue::g_dsp_status::gssapi_alloc_h::gssalloc_strdup;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;

/* qop_state */
#[no_mangle]

pub unsafe extern "C" fn gss_display_status(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut status_value: crate::gssapi_h::OM_uint32,
    mut status_type: i32,
    mut req_mech_type: crate::gssapi_h::gss_OID,
    mut message_context: *mut crate::gssapi_h::OM_uint32,
    mut status_string: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut mech_type = req_mech_type;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut m_oid = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 0u32,
            elements: 0 as *mut libc::c_void,
        };
        init
    };
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !status_string.is_null() {
        (*status_string).length = 0usize;
        (*status_string).value = 0 as *mut libc::c_void
    }
    if minor_status.is_null() || message_context.is_null() || status_string.is_null() {
        return (2u32) << 24i32;
    }
    /* we handle major status codes, and the mechs do the minor */
    if status_type == 1i32 {
        return displayMajor(status_value, message_context, status_string);
    }
    /*
     * must be the minor status - let mechs do the work
     * select the appropriate underlying mechanism routine and
     * call it.
     */
    /* In this version, we only handle status codes that have been
    mapped to a flat numbering space.  Look up the value we got
    passed.  If it's not found, complain.  */
    if status_value == 0u32 {
        (*status_string).value =
            gssalloc_strdup(b"Unknown error\x00" as *const u8 as *const i8) as *mut libc::c_void;
        if (*status_string).value.is_null() {
            *minor_status = 12u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
            return (13u32) << 16i32;
        }
        (*status_string).length = crate::stdlib::strlen((*status_string).value as *const i8);
        *message_context = 0u32;
        *minor_status = 0u32;
        return 0u32;
    }
    let mut err: i32 = 0;
    let mut m_status = 0u32;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    err = crate::src::generic::util_errmap::gssint_mecherrmap_get(
        status_value,
        &mut m_oid,
        &mut m_status,
    );
    if err != 0 {
        *minor_status = err as crate::gssapi_h::OM_uint32;
        *minor_status =
            crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status);
        return (5u32) << 16i32;
    }
    if m_oid.length == 0u32 {
        /* Magic flag for com_err values.  */
        status = crate::src::generic::disp_com_err_status::gssint_g_display_com_err_status(
            minor_status,
            m_status,
            status_string,
        );
        if status != 0u32 {
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        }
        return status;
    }
    mech_type = &mut m_oid;
    status_value = m_status;
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_type as crate::gssapi_h::gss_const_OID,
    );
    if !mech.is_null() && (*mech).gss_display_status.is_some() {
        let mut r: crate::gssapi_h::OM_uint32 = 0;
        r = (*mech)
            .gss_display_status
            .expect("non-null function pointer")(
            minor_status,
            status_value,
            status_type,
            mech_type,
            message_context,
            status_string,
        );
        /* How's this for weird?  If we get an error returning the
        mechanism-specific error code, we save away the
        mechanism-specific error code describing the error.  */
        if r != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            )
        }
        return r;
    }
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    return (16u32) << 16i32;
}
/* #pragma ident	"@(#)g_dsp_status.c	1.17	04/02/23 SMI" */
/*
 * Copyright 1996 by Sun Microsystems, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of Sun Microsystems not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. Sun Microsystems makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * SUN MICROSYSTEMS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL SUN MICROSYSTEMS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 *  glue routine gss_display_status
 *
 */
/* local function */
/*
 * function to map the major error codes
 * it uses case statements so that the strings could be wrapped by gettext
 * msgCtxt is interpreted as:
 *	0 - first call
 *	1 - routine error
 *	>= 2 - the supplementary error code bit shifted by 1
 */

unsafe extern "C" fn displayMajor(
    mut status: crate::gssapi_h::OM_uint32,
    mut msgCtxt: *mut crate::gssapi_h::OM_uint32,
    mut outStr: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut oneVal: crate::gssapi_h::OM_uint32 = 0;
    let mut mask = 0x1u32;
    let mut currErr: crate::gssapi_h::OM_uint32 = 0;
    let mut errStr = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut haveErr = 0i32;
    /* take care of the success value first */
    if status == 0u32 {
        errStr = crate::stdlib::dgettext(
            b"mit-krb5\x00" as *const u8 as *const i8,
            b"The routine completed successfully\x00" as *const u8 as *const i8,
        )
    } else if *msgCtxt == 0u32 && {
        oneVal = status & (0o377u32) << 24i32;
        (oneVal) != 0
    } {
        match oneVal {
            16777216 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"A required input parameter could not be read\x00" as *const u8 as *const i8,
                )
            }
            33554432 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"A required output parameter could not be written\x00" as *const u8
                        as *const i8,
                )
            }
            50331648 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"A parameter was malformed\x00" as *const u8 as *const i8,
                )
            }
            _ => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"An invalid status code was supplied\x00" as *const u8 as *const i8,
                )
            }
        }
        /* we now need to determine new value of msgCtxt */
        if status & (0o377u32) << 16i32 != 0 {
            *msgCtxt = 1u32
        } else {
            oneVal = status & (0o177777u32) << 0i32;
            if oneVal != 0u32 {
                *msgCtxt = oneVal << 1i32
            } else {
                *msgCtxt = 0u32
            }
        }
    } else if (*msgCtxt == 0u32 || *msgCtxt == 1u32) && {
        oneVal = status & (0o377u32) << 16i32;
        (oneVal) != 0
    } {
        match oneVal {
            65536 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"An unsupported mechanism was requested\x00" as
                                 *const u8 as *const i8)
            }
            131072 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"An invalid name was supplied\x00" as *const u8
                                 as *const i8)
            }
            196608 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"A supplied name was of an unsupported type\x00"
                                 as *const u8 as *const i8)
            }
            262144 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"Incorrect channel bindings were supplied\x00"
                                 as *const u8 as *const i8)
            }
            393216 => {
                /* same as GSS_S_BAD_MIC: */
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"A token had an invalid Message Integrity Check (MIC)\x00"
                                 as *const u8 as *const i8)
            }
            458752 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"No credentials were supplied, or the credentials were unavailable or inaccessible\x00"
                                 as *const u8 as *const i8)
            }
            524288 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"No context has been established\x00" as
                                 *const u8 as *const i8)
            }
            589824 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"Invalid token was supplied\x00" as *const u8 as
                                 *const i8)
            }
            655360 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"Invalid credential was supplied\x00" as
                                 *const u8 as *const i8)
            }
            720896 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The referenced credential has expired\x00" as
                                 *const u8 as *const i8)
            }
            786432 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The referenced context has expired\x00" as
                                 *const u8 as *const i8)
            }
            851968 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"Unspecified GSS failure.  Minor code may provide more information\x00"
                                 as *const u8 as *const i8)
            }
            917504 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The quality-of-protection (QOP) requested could not be provided\x00"
                                 as *const u8 as *const i8)
            }
            983040 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The operation is forbidden by local security policy\x00"
                                 as *const u8 as *const i8)
            }
            1048576 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The operation or option is not available or unsupported\x00"
                                 as *const u8 as *const i8)
            }
            1114112 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The requested credential element already exists\x00"
                                 as *const u8 as *const i8)
            }
            1179648 => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"The provided name was not mechanism specific (MN)\x00"
                                 as *const u8 as *const i8)
            }
            327680 | _ => {
                errStr =
                    crate::stdlib::dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const i8,
                             b"An invalid status code was supplied\x00" as
                                 *const u8 as *const i8)
            }
        }
        /* we must determine if the caller should call us again */
        oneVal = status & (0o177777u32) << 0i32;
        if oneVal != 0u32 {
            *msgCtxt = oneVal << 1i32
        } else {
            *msgCtxt = 0u32
        }
    } else if (*msgCtxt == 0u32 || *msgCtxt >= 2u32) && {
        oneVal = status & (0o177777u32) << 0i32;
        (oneVal) != 0
    } {
        /*
         * if msgCtxt is not 0, then it should encode
         * the supplementary error code we should be printing
         */
        if *msgCtxt >= 2u32 {
            oneVal = *msgCtxt >> 1i32
        } else {
            oneVal = status & (0o177777u32) << 0i32
        }
        /* we display the errors LSB first */
        i = 0i32;
        while i < 16i32 {
            if oneVal & mask != 0 {
                haveErr = 1i32;
                break;
            } else {
                mask <<= 1i32;
                i += 1
            }
        }
        /* isolate the bit or if not found set to illegal value */
        if haveErr != 0 {
            currErr = oneVal & mask
        } else {
            currErr = ((1i32) << 17i32) as crate::gssapi_h::OM_uint32
        } /* illegal value */
        match currErr {
            1 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"The routine must be called again to complete its function\x00" as *const u8
                        as *const i8,
                )
            }
            2 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"The token was a duplicate of an earlier token\x00" as *const u8 as *const i8,
                )
            }
            4 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"The token\'s validity period has expired\x00" as *const u8 as *const i8,
                )
            }
            8 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"A later token has already been processed\x00" as *const u8 as *const i8,
                )
            }
            16 => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"An expected per-message token was not received\x00" as *const u8 as *const i8,
                )
            }
            _ => {
                errStr = crate::stdlib::dgettext(
                    b"mit-krb5\x00" as *const u8 as *const i8,
                    b"An invalid status code was supplied\x00" as *const u8 as *const i8,
                )
            }
        }
        /*
         * we must check if there is any other supplementary errors
         * if found, then turn off current bit, and store next value
         * in msgCtxt shifted by 1 bit
         */
        if haveErr == 0 {
            *msgCtxt = 0u32
        } else if oneVal & (0o177777u32) << 0i32 ^ mask != 0 {
            *msgCtxt = (oneVal & (0o177777u32) << 0i32 ^ mask) << 1i32
        } else {
            *msgCtxt = 0u32
        }
    }
    if errStr.is_null() {
        errStr = b"An invalid status code was supplied\x00" as *const u8 as *mut i8
    }
    /* now copy the status code and return to caller */
    (*outStr).length = crate::stdlib::strlen(errStr);
    (*outStr).value = gssalloc_strdup(errStr) as *mut libc::c_void;
    if (*outStr).value.is_null() {
        (*outStr).length = 0usize;
        return (13u32) << 16i32;
    }
    return 0u32;
}
/* displayMajor */
