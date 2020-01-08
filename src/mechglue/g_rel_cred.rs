use ::libc;

/* _GSSAPIP_GENERIC_H_ */
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
pub use crate::mglueP_h::gss_union_cred_t;

pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;

/* time_rec */
/* #pragma ident	"@(#)g_rel_cred.c	1.14	04/02/23 SMI" */
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
/* Glue routine for gss_release_cred */
#[no_mangle]

pub unsafe extern "C" fn gss_release_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_status: crate::gssapi_h::OM_uint32 = 0;
    let mut j: i32 = 0;
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if cred_handle.is_null() {
        return (7u32) << 16i32 | (1u32) << 24i32;
    }
    /*
     * Loop through the union_cred struct, selecting the approprate
     * underlying mechanism routine and calling it. At the end,
     * release all of the storage taken by the union_cred struct.
     */
    union_cred = *cred_handle;
    if union_cred == 0 as crate::mglueP_h::gss_union_cred_t {
        return 0u32;
    }
    if !(!union_cred.is_null() && (*union_cred).loopback == union_cred) {
        return (7u32) << 16i32 | (1u32) << 24i32;
    }
    *cred_handle = 0 as crate::gssapi_h::gss_cred_id_t;
    status = 0u32;
    j = 0i32;
    while j < (*union_cred).count {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            &mut *(*union_cred).mechs_array.offset(j as isize)
                as *mut crate::gssapi_h::gss_OID_desc_struct
                as crate::gssapi_h::gss_const_OID,
        );
        if !(*(*union_cred).mechs_array.offset(j as isize))
            .elements
            .is_null()
        {
            crate::stdlib::free((*(*union_cred).mechs_array.offset(j as isize)).elements);
        }
        if !mech.is_null() {
            if (*mech).gss_release_cred.is_some() {
                temp_status = (*mech).gss_release_cred.expect("non-null function pointer")(
                    minor_status,
                    &mut *(*union_cred).cred_array.offset(j as isize),
                );
                if temp_status != 0u32 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                        *minor_status,
                        &mut (*mech).mech_type,
                    );
                    status = (7u32) << 16i32
                }
            } else {
                status = (16u32) << 16i32
            }
        } else {
            status = (10u32) << 16i32
        }
        j += 1
    }
    crate::stdlib::free((*union_cred).cred_array as *mut libc::c_void);
    crate::stdlib::free((*union_cred).mechs_array as *mut libc::c_void);
    crate::stdlib::free(union_cred as *mut libc::c_void);
    return status;
}
