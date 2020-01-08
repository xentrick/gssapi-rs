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
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;

/* #pragma ident	"@(#)g_compare_name.c	1.16	04/02/23 SMI" */
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
 *  glue routine for gss_compare_name
 *
 */

unsafe extern "C" fn val_comp_name_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name1: crate::gssapi_h::gss_name_t,
    mut name2: crate::gssapi_h::gss_name_t,
    mut name_equal: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    /* Validate arguments. */
    if name1.is_null() || name2.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    if name_equal.is_null() {
        return (2u32) << 24i32;
    }
    return 0u32;
}
/* mech_set */
#[no_mangle]

pub unsafe extern "C" fn gss_compare_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name1: crate::gssapi_h::gss_name_t,
    mut name2: crate::gssapi_h::gss_name_t,
    mut name_equal: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name1 = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut union_name2 = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut mech = 0 as crate::mglueP_h::gss_mechanism;
    let mut internal_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    major_status = val_comp_name_args(minor_status, name1, name2, name_equal);
    if major_status != 0u32 {
        return major_status;
    }
    union_name1 = name1;
    union_name2 = name2;
    /*
     * Try our hardest to make union_name1 be the mechanism-specific
     * name.  (Of course we can't if both names aren't
     * mechanism-specific.)
     */
    if (*union_name1).mech_type.is_null() {
        union_name1 = name2;
        union_name2 = name1
    }
    /*
     * If union_name1 is mechanism specific, then fetch its mechanism
     * information.
     */
    if !(*union_name1).mech_type.is_null() {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            (*union_name1).mech_type as crate::gssapi_h::gss_const_OID,
        ); /* Default to *not* equal.... */
        if mech.is_null() {
            return (1u32) << 16i32;
        }
        if (*mech).gss_compare_name.is_none() {
            return (16u32) << 16i32;
        }
    }
    *name_equal = 0i32;
    /*
     * First case... both names are mechanism-specific
     */
    if !(*union_name1).mech_type.is_null() && !(*union_name2).mech_type.is_null() {
        if !((*(*union_name1).mech_type).length == (*(*union_name2).mech_type).length
            && crate::stdlib::memcmp(
                (*(*union_name1).mech_type).elements,
                (*(*union_name2).mech_type).elements,
                (*(*union_name1).mech_type).length as usize,
            ) == 0i32)
        {
            return 0u32;
        }
        if (*union_name1).mech_name.is_null() || (*union_name2).mech_name.is_null() {
            /* should never happen */
            return (2u32) << 16i32;
        }
        if mech.is_null() {
            return (1u32) << 16i32;
        }
        if (*mech).gss_compare_name.is_none() {
            return (16u32) << 16i32;
        }
        major_status = (*mech).gss_compare_name.expect("non-null function pointer")(
            minor_status,
            (*union_name1).mech_name,
            (*union_name2).mech_name,
            name_equal,
        );
        if major_status != 0u32 {
            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor_status,
                &mut (*mech).mech_type,
            )
        }
        return major_status;
    }
    /*
     * Second case... both names are NOT mechanism specific.
     *
     * All we do here is make sure the two name_types are equal and then
     * that the external_names are equal. Note the we do not take care
     * of the case where two different external names map to the same
     * internal name. We cannot determine this, since we as yet do not
     * know what mechanism to use for calling the underlying
     * gss_import_name().
     */
    if (*union_name1).mech_type.is_null() && (*union_name2).mech_type.is_null() {
        /*
         * Second case, first sub-case... one name has null
         * name_type, the other doesn't.
         *
         * Not knowing a mech_type we can't import the name with
         * null name_type so we can't compare.
         */
        if (*union_name1).name_type.is_null() && !(*union_name2).name_type.is_null()
            || !(*union_name1).name_type.is_null() && (*union_name2).name_type.is_null()
        {
            return 0u32;
        }
        /*
         * Second case, second sub-case... both names have
         * name_types, but they are different.
         */
        if !(*union_name1).name_type.is_null()
            && !(*union_name2).name_type.is_null()
            && !((*(*union_name1).name_type).length == (*(*union_name2).name_type).length
                && crate::stdlib::memcmp(
                    (*(*union_name1).name_type).elements,
                    (*(*union_name2).name_type).elements,
                    (*(*union_name1).name_type).length as usize,
                ) == 0i32)
        {
            return 0u32;
        }
        /*
         * Second case, third sub-case... both names have equal
         * name_types (and both have no mech_types) so we just
         * compare the external_names.
         */
        if (*(*union_name1).external_name).length != (*(*union_name2).external_name).length
            || crate::stdlib::memcmp(
                (*(*union_name1).external_name).value,
                (*(*union_name2).external_name).value,
                (*(*union_name1).external_name).length,
            ) != 0i32
        {
            return 0u32;
        }
        *name_equal = 1i32;
        return 0u32;
    }
    /*
     * Final case... one name is mechanism specific, the other isn't.
     *
     * We attempt to convert the general name to the mechanism type of
     * the mechanism-specific name, and then do the compare.  If we
     * can't import the general name, then we return that the name is
     * _NOT_ equal.
     */
    if !(*union_name2).mech_type.is_null() {
        /* We make union_name1 the mechanism specific name. */
        union_name1 = name2; /* return complete, but not equal */
        union_name2 = name1
    }
    major_status = crate::src::mechglue::g_glue::gssint_import_internal_name(
        minor_status,
        (*union_name1).mech_type,
        union_name2,
        &mut internal_name,
    );
    if major_status != 0u32 {
        return 0u32;
    }
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_compare_name.is_none() {
        return (16u32) << 16i32;
    }
    major_status = (*mech).gss_compare_name.expect("non-null function pointer")(
        minor_status,
        (*union_name1).mech_name,
        internal_name,
        name_equal,
    );
    if major_status != 0u32 {
        *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
            *minor_status,
            &mut (*mech).mech_type,
        )
    }
    crate::src::mechglue::g_glue::gssint_release_internal_name(
        &mut temp_minor,
        (*union_name1).mech_type,
        &mut internal_name,
    );
    return major_status;
}
