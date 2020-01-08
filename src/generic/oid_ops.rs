use ::libc;

pub mod gssapiP_generic_h {
    #[inline]

    pub unsafe extern "C" fn k5buf_to_gss(
        mut minor: *mut crate::gssapi_h::OM_uint32,
        mut input_k5buf: *mut crate::k5_buf_h::k5buf,
        mut output_buffer: crate::gssapi_h::gss_buffer_t,
    ) -> crate::gssapi_h::OM_uint32 {
        let mut status = 0u32;
        if crate::k5_buf_h::k5_buf_status(input_k5buf) != 0i32 {
            *minor = 12u32;
            return (13u32) << 16i32;
        }
        (*output_buffer).length = (*input_k5buf).len;
        (*output_buffer).value = (*input_k5buf).data;
        crate::stdlib::memset(
            input_k5buf as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::k5_buf_h::k5buf>(),
        );
        return status;
    }

    /* _GSSAPIP_GENERIC_H_ */
}

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        crate::stdlib::free(value);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_calloc(
        mut count: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::calloc(count, size);
    }
}

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;

pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::k5_buf_h::k5_buf_add;
pub use crate::k5_buf_h::k5_buf_add_fmt;
pub use crate::k5_buf_h::k5_buf_add_len;
pub use crate::k5_buf_h::k5_buf_init_dynamic;
pub use crate::k5_buf_h::k5_buf_status;
pub use crate::k5_buf_h::k5buf;
pub use crate::k5_buf_h::k5buftype;
pub use crate::k5_buf_h::K5BUF_DYNAMIC;
pub use crate::k5_buf_h::K5BUF_DYNAMIC_ZAP;
pub use crate::k5_buf_h::K5BUF_ERROR;
pub use crate::k5_buf_h::K5BUF_FIXED;

pub use crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS;

pub use crate::src::generic::gssapi_generic::GSS_C_NT_EXPORT_NAME;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_HOSTBASED_SERVICE;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_MACHINE_UID_NAME;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_STRING_UID_NAME;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME;
pub use crate::src::generic::oid_ops::gssapiP_generic_h::k5buf_to_gss;
pub use crate::src::generic::rel_oid_set::generic_gss_release_oid_set;
pub use crate::stdlib::C2RustUnnamed_7;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;

pub use crate::stdlib::__ctype_b_loc;

pub use crate::src::generic::oid_ops::gssapi_alloc_h::gssalloc_calloc;
pub use crate::src::generic::oid_ops::gssapi_alloc_h::gssalloc_free;
pub use crate::src::generic::oid_ops::gssapi_alloc_h::gssalloc_malloc;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/generic/oid_ops.c */
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
/* GSS-API V2 interfaces to manipulate OIDs */
/*
 * The functions for allocating and releasing individual OIDs use malloc and
 * free instead of the gssalloc wrappers, because the mechglue currently mixes
 * generic_gss_copy_oid() with hand-freeing of OIDs.  We do not need to free
 * free OIDs allocated by mechanisms, so this should not be a problem.
 */
#[no_mangle]

pub unsafe extern "C" fn generic_gss_release_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if oid.is_null() || (*oid).is_null() {
        return 0u32;
    }
    /*
     * The V2 API says the following!
     *
     * gss_release_oid[()] will recognize any of the GSSAPI's own OID values,
     * and will silently ignore attempts to free these OIDs; for other OIDs
     * it will call the C free() routine for both the OID data and the
     * descriptor.  This allows applications to freely mix their own heap-
     * allocated OID values with OIDs returned by GSS-API.
     */
    /*
     * We use the official OID definitions instead of the unofficial OID
     * definitions. But we continue to support the unofficial OID
     * gss_nt_service_name just in case if some gss applications use
     * the old OID.
     */
    if *oid != crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME
        && *oid != crate::src::generic::gssapi_generic::GSS_C_NT_MACHINE_UID_NAME
        && *oid != crate::src::generic::gssapi_generic::GSS_C_NT_STRING_UID_NAME
        && *oid != crate::src::generic::gssapi_generic::GSS_C_NT_HOSTBASED_SERVICE
        && *oid != crate::src::generic::gssapi_generic::GSS_C_NT_ANONYMOUS
        && *oid != crate::src::generic::gssapi_generic::GSS_C_NT_EXPORT_NAME
        && *oid != crate::src::generic::gssapi_generic::GSS_C_NT_COMPOSITE_EXPORT
        && *oid != crate::src::generic::gssapi_generic::gss_nt_service_name
    {
        crate::stdlib::free((**oid).elements);
        crate::stdlib::free(*oid as *mut libc::c_void);
    }
    *oid = 0 as crate::gssapi_h::gss_OID;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_copy_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    oid: *const crate::gssapi_h::gss_OID_desc,
    mut new_oid: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut p = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    *minor_status = 0u32;
    p = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>())
        as crate::gssapi_h::gss_OID;
    if p.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    (*p).length = (*oid).length;
    (*p).elements = crate::stdlib::malloc((*p).length as usize);
    if (*p).elements.is_null() {
        crate::stdlib::free(p as *mut libc::c_void);
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy((*p).elements, (*oid).elements, (*p).length as usize);
    *new_oid = p;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_create_empty_oid_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    *minor_status = 0u32;
    if oid_set.is_null() {
        return (2u32) << 24i32;
    }
    *oid_set = gssalloc_malloc(::std::mem::size_of::<crate::gssapi_h::gss_OID_set_desc>())
        as crate::gssapi_h::gss_OID_set;
    if !(*oid_set).is_null() {
        crate::stdlib::memset(
            *oid_set as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::gssapi_h::gss_OID_set_desc>(),
        );
        return 0u32;
    } else {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    };
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_add_oid_set_member(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    member_oid: *const crate::gssapi_h::gss_OID_desc,
    mut oid_set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut elist = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut lastel = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    *minor_status = 0u32;
    if member_oid.is_null() || (*member_oid).length == 0u32 || (*member_oid).elements.is_null() {
        return (1u32) << 24i32;
    }
    if oid_set.is_null() {
        return (2u32) << 24i32;
    }
    elist = (**oid_set).elements;
    /* Get an enlarged copy of the array */
    (**oid_set).elements = gssalloc_malloc(
        (**oid_set)
            .count
            .wrapping_add(1usize)
            .wrapping_mul(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>()),
    ) as crate::gssapi_h::gss_OID;
    if !(**oid_set).elements.is_null() {
        /* Copy in the old junk */
        if !elist.is_null() {
            crate::stdlib::memcpy(
                (**oid_set).elements as *mut libc::c_void,
                elist as *const libc::c_void,
                (**oid_set)
                    .count
                    .wrapping_mul(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>()),
            );
        }
        /* Duplicate the input element */
        lastel = &mut *(**oid_set).elements.offset((**oid_set).count as isize)
            as *mut crate::gssapi_h::gss_OID_desc_struct;
        (*lastel).elements = gssalloc_malloc((*member_oid).length as crate::stddef_h::size_t);
        if !(*lastel).elements.is_null() {
            /* Success - copy elements */
            crate::stdlib::memcpy(
                (*lastel).elements,
                (*member_oid).elements,
                (*member_oid).length as crate::stddef_h::size_t,
            );
            /* Set length */
            (*lastel).length = (*member_oid).length;
            /* Update count */
            (**oid_set).count = (**oid_set).count.wrapping_add(1);
            if !elist.is_null() {
                gssalloc_free(elist as *mut libc::c_void);
            }
            *minor_status = 0u32;
            return 0u32;
        } else {
            gssalloc_free((**oid_set).elements as *mut libc::c_void);
        }
    }
    /* Failure - restore old contents of list */
    (**oid_set).elements = elist;
    *minor_status = 12u32;
    return (13u32) << 16i32;
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_test_oid_set_member(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    member: *const crate::gssapi_h::gss_OID_desc,
    mut set: crate::gssapi_h::gss_OID_set,
    mut present: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut i: crate::gssapi_h::OM_uint32 = 0;
    let mut result: i32 = 0;
    *minor_status = 0u32;
    if member.is_null() || set.is_null() {
        return (1u32) << 24i32;
    }
    if present.is_null() {
        return (2u32) << 24i32;
    }
    result = 0i32;
    i = 0u32;
    while (i as usize) < (*set).count {
        if (*(*set).elements.offset(i as isize)).length == (*member).length
            && crate::stdlib::memcmp(
                (*(*set).elements.offset(i as isize)).elements,
                (*member).elements,
                (*member).length as crate::stddef_h::size_t,
            ) == 0
        {
            result = 1i32;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    *present = result;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_oid_to_str(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    oid: *const crate::gssapi_h::gss_OID_desc,
    mut oid_str: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut number: usize = 0;
    let mut n: usize = 0;
    let mut i: crate::gssapi_h::OM_uint32 = 0;
    let mut first: i32 = 0;
    let mut cp = 0 as *mut u8;
    let mut buf = crate::k5_buf_h::k5buf {
        buftype: crate::k5_buf_h::K5BUF_ERROR,
        data: 0 as *mut libc::c_void,
        space: 0,
        len: 0,
    };
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !oid_str.is_null() {
        (*oid_str).length = 0usize;
        (*oid_str).value = 0 as *mut libc::c_void
    }
    if oid.is_null() || (*oid).length == 0u32 || (*oid).elements.is_null() {
        return (1u32) << 24i32;
    }
    if oid_str.is_null() {
        return (2u32) << 24i32;
    }
    /* Decoded according to krb5/gssapi_krb5.c */
    cp = (*oid).elements as *mut u8;
    number = *cp.offset(0isize) as usize;
    crate::k5_buf_h::k5_buf_init_dynamic(&mut buf);
    crate::k5_buf_h::k5_buf_add(&mut buf, b"{ \x00" as *const u8 as *const i8);
    number = 0usize;
    cp = (*oid).elements as *mut u8;
    first = 1i32;
    i = 0u32;
    while i < (*oid).length {
        number = number << 7i32 | (*cp.offset(i as isize) as i32 & 0x7fi32) as usize;
        if *cp.offset(i as isize) as i32 & 0x80i32 == 0i32 {
            if first != 0 {
                n = if number < 40usize {
                    0i32
                } else if number < 80usize {
                    1i32
                } else {
                    2i32
                } as usize;
                crate::k5_buf_h::k5_buf_add_fmt(
                    &mut buf as *mut crate::k5_buf_h::k5buf,
                    b"%lu %lu \x00" as *const u8 as *const i8,
                    n,
                    number.wrapping_sub(n.wrapping_mul(40usize)),
                );
                first = 0i32
            } else {
                crate::k5_buf_h::k5_buf_add_fmt(
                    &mut buf as *mut crate::k5_buf_h::k5buf,
                    b"%lu \x00" as *const u8 as *const i8,
                    number,
                );
            }
            number = 0usize
        }
        i = i.wrapping_add(1)
    }
    crate::k5_buf_h::k5_buf_add_len(
        &mut buf,
        b"}\x00\x00" as *const u8 as *const libc::c_void,
        2usize,
    );
    return k5buf_to_gss(minor_status, &mut buf, oid_str);
}
/* Return the length of a DER OID subidentifier encoding. */

unsafe extern "C" fn arc_encoded_length(mut arc: usize) -> crate::stddef_h::size_t {
    let mut len = 1usize;
    arc >>= 7i32;
    while arc != 0 {
        len = len.wrapping_add(1);
        arc >>= 7i32
    }
    return len;
}
/* Encode a subidentifier into *bufp and advance it to the encoding's end. */

unsafe extern "C" fn arc_encode(mut arc: usize, mut bufp: *mut *mut u8) {
    let mut p = 0 as *mut u8;
    /* Advance to the end and encode backwards. */
    *bufp = (*bufp).offset(arc_encoded_length(arc) as isize);
    p = *bufp;
    p = p.offset(-1);
    *p = (arc & 0x7fusize) as u8;
    arc >>= 7i32;
    while arc != 0 {
        p = p.offset(-1);
        *p = (arc & 0x7fusize | 0x80usize) as u8;
        arc >>= 7i32
    }
}
/* Fetch an arc value from *bufp and advance past it and any following spaces
 * or periods.  Return 1 on success, 0 if *bufp is not at a valid arc value. */

unsafe extern "C" fn get_arc(
    mut bufp: *mut *const u8,
    mut end: *const u8,
    mut arc_out: *mut usize,
) -> i32 {
    let mut p = *bufp;
    let mut arc = 0usize;
    let mut newval: usize = 0;
    if p == end
        || *(*crate::stdlib::__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & crate::stdlib::_ISdigit as u16 as i32
            == 0
    {
        return 0i32;
    }
    while p < end
        && *(*crate::stdlib::__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & crate::stdlib::_ISdigit as u16 as i32
            != 0
    {
        newval = arc
            .wrapping_mul(10usize)
            .wrapping_add((*p as i32 - '0' as i32) as usize);
        if newval < arc {
            return 0i32;
        }
        arc = newval;
        p = p.offset(1)
    }
    while p < end
        && (*(*crate::stdlib::__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & crate::stdlib::_ISspace as u16 as i32
            != 0
            || *p as i32 == '.' as i32)
    {
        p = p.offset(1)
    }
    *bufp = p;
    *arc_out = arc;
    return 1i32;
}
/*
 * Convert a sequence of two or more decimal arc values into a DER-encoded OID.
 * The values may be separated by any combination of whitespace and period
 * characters, and may be optionally surrounded with braces.  Leading
 * whitespace and trailing garbage is allowed.  The first arc value must be 0,
 * 1, or 2, and the second value must be less than 40 if the first value is not
 * 2.
 */
#[no_mangle]

pub unsafe extern "C" fn generic_gss_str_to_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut oid_str: crate::gssapi_h::gss_buffer_t,
    mut oid_out: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut p = 0 as *const u8;
    let mut end = 0 as *const u8;
    let mut arc3_start = 0 as *const u8;
    let mut out = 0 as *mut u8;
    let mut arc: usize = 0;
    let mut arc1: usize = 0;
    let mut arc2: usize = 0;
    let mut nbytes: crate::stddef_h::size_t = 0;
    let mut brace = 0i32;
    let mut oid = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !oid_out.is_null() {
        *oid_out = 0 as crate::gssapi_h::gss_OID
    }
    if oid_str.is_null() || (*oid_str).value.is_null() || (*oid_str).length == 0usize {
        return (1u32) << 24i32;
    }
    if oid_out.is_null() {
        return (2u32) << 24i32;
    }
    /* Skip past initial spaces and, optionally, an open brace. */
    brace = 0i32;
    p = (*oid_str).value as *const u8;
    end = p.offset((*oid_str).length as isize);
    while p < end
        && *(*crate::stdlib::__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & crate::stdlib::_ISspace as u16 as i32
            != 0
    {
        p = p.offset(1)
    }
    if p < end && *p as i32 == '{' as i32 {
        brace = 1i32;
        p = p.offset(1)
    }
    while p < end
        && *(*crate::stdlib::__ctype_b_loc()).offset(*p as i32 as isize) as i32
            & crate::stdlib::_ISspace as u16 as i32
            != 0
    {
        p = p.offset(1)
    }
    /* Get the first two arc values, to be encoded as one subidentifier. */
    if get_arc(&mut p, end, &mut arc1) == 0 || get_arc(&mut p, end, &mut arc2) == 0 {
        return (13u32) << 16i32;
    }
    if arc1 > 2usize
        || arc1 < 2usize && arc2 > 39usize
        || arc2
            > (9223372036854775807 as usize)
                .wrapping_mul(2usize)
                .wrapping_add(1usize)
                .wrapping_sub(80usize)
    {
        return (13u32) << 16i32;
    }
    arc3_start = p;
    /* Compute the total length of the encoding while checking syntax. */
    nbytes = arc_encoded_length(arc1.wrapping_mul(40usize).wrapping_add(arc2));
    while get_arc(&mut p, end, &mut arc) != 0 {
        nbytes = (nbytes).wrapping_add(arc_encoded_length(arc))
    }
    if brace != 0 && (p == end || *p as i32 != '}' as i32) {
        return (13u32) << 16i32;
    }
    /* Allocate an oid structure. */
    oid = crate::stdlib::malloc(::std::mem::size_of::<crate::gssapi_h::gss_OID_desc_struct>())
        as crate::gssapi_h::gss_OID;
    if oid.is_null() {
        return (13u32) << 16i32;
    }
    (*oid).elements = crate::stdlib::malloc(nbytes);
    if (*oid).elements.is_null() {
        crate::stdlib::free(oid as *mut libc::c_void);
        return (13u32) << 16i32;
    }
    (*oid).length = nbytes as crate::gssapi_h::OM_uint32;
    out = (*oid).elements as *mut u8;
    arc_encode(arc1.wrapping_mul(40usize).wrapping_add(arc2), &mut out);
    p = arc3_start;
    while get_arc(&mut p, end, &mut arc) != 0 {
        arc_encode(arc, &mut out);
    }
    if out.offset(-(nbytes as isize)) == (*oid).elements as *mut u8 {
    } else {
        crate::stdlib::__assert_fail(
            b"out - nbytes == oid->elements\x00" as *const u8 as *const i8,
            b"oid_ops.c\x00" as *const u8 as *const i8,
            411u32,
            (*::std::mem::transmute::<&[u8; 71], &[i8; 71]>(
                b"OM_uint32 generic_gss_str_to_oid(OM_uint32 *, gss_buffer_t, gss_OID *)\x00",
            ))
            .as_ptr(),
        );
    }
    *oid_out = oid;
    return 0u32;
}
/* Compose an OID of a prefix and an integer suffix */
#[no_mangle]

pub unsafe extern "C" fn generic_gss_oid_compose(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut prefix: *const i8,
    mut prefix_len: crate::stddef_h::size_t,
    mut suffix: i32,
    mut oid: *mut crate::gssapi_h::gss_OID_desc,
) -> crate::gssapi_h::OM_uint32 {
    let mut osuffix: i32 = 0;
    let mut i: i32 = 0;
    let mut nbytes: crate::stddef_h::size_t = 0;
    let mut op = 0 as *mut u8;
    if oid.is_null() {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    if ((*oid).length as usize) < prefix_len {
        *minor_status = 34u32;
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy((*oid).elements, prefix as *const libc::c_void, prefix_len);
    nbytes = 0usize;
    osuffix = suffix;
    while suffix != 0 {
        nbytes = nbytes.wrapping_add(1);
        suffix >>= 7i32
    }
    suffix = osuffix;
    if ((*oid).length as usize) < prefix_len.wrapping_add(nbytes) {
        *minor_status = 34u32;
        return (13u32) << 16i32;
    }
    op = ((*oid).elements as *mut u8)
        .offset(prefix_len as isize)
        .offset(nbytes as isize);
    i = -(1i32);
    while suffix != 0 {
        *op.offset(i as isize) = (suffix as u8 as i32 & 0x7fi32) as u8;
        if i != -(1i32) {
            let ref mut fresh0 = *op.offset(i as isize);
            *fresh0 = (*fresh0 as i32 | 0x80i32) as u8
        }
        i -= 1;
        suffix >>= 7i32
    }
    (*oid).length = prefix_len.wrapping_add(nbytes) as crate::gssapi_h::OM_uint32;
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn generic_gss_oid_decompose(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut prefix: *const i8,
    mut prefix_len: crate::stddef_h::size_t,
    mut oid: *mut crate::gssapi_h::gss_OID_desc,
    mut suffix: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut i: crate::stddef_h::size_t = 0;
    let mut slen: crate::stddef_h::size_t = 0;
    let mut op = 0 as *mut u8;
    if ((*oid).length as usize) < prefix_len
        || crate::stdlib::memcmp((*oid).elements, prefix as *const libc::c_void, prefix_len) != 0i32
    {
        return (1u32) << 16i32;
    }
    op = ((*oid).elements as *mut u8).offset(prefix_len as isize);
    *suffix = 0i32;
    slen = ((*oid).length as usize).wrapping_sub(prefix_len);
    i = 0usize;
    while i < slen {
        *suffix = *suffix << 7i32 | *op.offset(i as isize) as i32 & 0x7fi32;
        if i.wrapping_add(1usize) != slen && *op.offset(i as isize) as i32 & 0x80i32 == 0i32 {
            *minor_status = 22u32;
            return (13u32) << 16i32;
        }
        i = i.wrapping_add(1)
    }
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
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* buffer */
/* minor_status */
/* set */
/* minor_status */
/* set */
/* minor_status */
/* oid */
/* new_oid */
/* minor_status */
/* oid_set */
/* minor_status */
/* member_oid */
/* oid_set */
/* minor_status */
/* member */
/* set */
/* present */
/* minor_status */
/* oid */
/* oid_str */
/* minor_status */
/* oid_str */
/* oid */
/* minor_status */
/* prefix */
/* prefix_len */
/* suffix */
/* oid */
/* minor_status */
/*prefix */
/* prefix_len */
/* oid */
/* suffix */
/*
 * Transfer contents of a k5buf to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*member_buffer*/
/*buffer_set*/
/*minor_status*/
/*buffer_set*/
#[no_mangle]

pub unsafe extern "C" fn generic_gss_copy_oid_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    oidset: *const crate::gssapi_h::gss_OID_set_desc,
    mut new_oidset: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut copy = 0 as *mut crate::gssapi_h::gss_OID_set_desc;
    let mut minor = 0u32;
    let mut major = 0u32;
    let mut i: crate::gssapi_h::OM_uint32 = 0;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !new_oidset.is_null() {
        *new_oidset = 0 as crate::gssapi_h::gss_OID_set
    }
    if oidset.is_null() {
        return (1u32) << 24i32;
    }
    if new_oidset.is_null() {
        return (2u32) << 24i32;
    }
    copy = gssalloc_calloc(
        1usize,
        ::std::mem::size_of::<crate::gssapi_h::gss_OID_set_desc>(),
    ) as *mut crate::gssapi_h::gss_OID_set_desc;
    if copy.is_null() {
        major = (13u32) << 16i32
    } else {
        (*copy).elements = gssalloc_calloc(
            (*oidset).count,
            ::std::mem::size_of::<crate::gssapi_h::gss_OID_desc_struct>(),
        ) as *mut crate::gssapi_h::gss_OID_desc;
        if (*copy).elements.is_null() {
            major = (13u32) << 16i32
        } else {
            (*copy).count = (*oidset).count;
            i = 0u32;
            loop {
                if !((i as usize) < (*copy).count) {
                    current_block = 5634871135123216486;
                    break;
                }
                let mut out: *mut crate::gssapi_h::gss_OID_desc =
                    &mut *(*copy).elements.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct;
                let mut in_0: *mut crate::gssapi_h::gss_OID_desc =
                    &mut *(*oidset).elements.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct;
                (*out).elements = gssalloc_malloc((*in_0).length as crate::stddef_h::size_t);
                if (*out).elements.is_null() {
                    major = (13u32) << 16i32;
                    current_block = 1186425362246550222;
                    break;
                } else {
                    crate::stdlib::memcpy(
                        (*out).elements,
                        (*in_0).elements,
                        (*in_0).length as usize,
                    );
                    (*out).length = (*in_0).length;
                    i = i.wrapping_add(1)
                }
            }
            match current_block {
                1186425362246550222 => {}
                _ => *new_oidset = copy,
            }
        }
    }
    if major != 0u32 {
        crate::src::generic::rel_oid_set::generic_gss_release_oid_set(&mut minor, &mut copy);
    }
    return major;
}
