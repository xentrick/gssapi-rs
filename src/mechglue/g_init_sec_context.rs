use ::libc;

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
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_glue::gssint_create_union_context;
pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;

/* #pragma ident	"@(#)g_init_sec_context.c	1.20	03/10/24 SMI" */
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
 *  glue routine for gss_init_sec_context
 */

unsafe extern "C" fn val_init_sec_ctx_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut _claimant_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut _req_mech_type: crate::gssapi_h::gss_OID,
    mut _req_flags: crate::gssapi_h::OM_uint32,
    mut _time_req: crate::gssapi_h::OM_uint32,
    mut _input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut _input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !actual_mech_type.is_null() {
        *actual_mech_type = 0 as crate::gssapi_h::gss_OID
    }
    if !output_token.is_null() {
        (*output_token).length = 0usize;
        (*output_token).value = 0 as *mut libc::c_void
    }
    if !ret_flags.is_null() {
        *ret_flags = 0u32
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() {
        return (2u32) << 24i32 | (8u32) << 16i32;
    }
    if target_name.is_null() {
        return (1u32) << 24i32 | (2u32) << 16i32;
    }
    if output_token.is_null() {
        return (2u32) << 24i32;
    }
    return 0u32;
}
/* cred_handle */
#[no_mangle]

pub unsafe extern "C" fn gss_init_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut claimant_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut req_mech_type: crate::gssapi_h::gss_OID,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut internal_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut union_ctx_id = 0 as *mut crate::mglueP_h::gss_union_ctx_id_struct;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut input_cred_handle = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    status = val_init_sec_ctx_args(
        minor_status,
        claimant_cred_handle,
        context_handle,
        target_name,
        req_mech_type,
        req_flags,
        time_req,
        input_chan_bindings,
        input_token,
        actual_mech_type,
        output_token,
        ret_flags,
        time_rec,
    );
    if status != 0u32 {
        return status;
    }
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        req_mech_type as crate::gssapi_h::gss_const_OID,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    union_name = target_name;
    /*
     * obtain the gss mechanism information for the requested
     * mechanism.  If mech_type is NULL, set it to the resultant
     * mechanism
     */
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_init_sec_context.is_none() {
        return (16u32) << 16i32;
    }
    /*
     * If target_name is mechanism_specific, then it must match the
     * mech_type that we're about to use.  Otherwise, do an import on
     * the external_name form of the target name.
     */
    if !(*union_name).mech_type.is_null()
        && ((*(*union_name).mech_type).length == (*selected_mech).length
            && crate::stdlib::memcmp(
                (*(*union_name).mech_type).elements,
                (*selected_mech).elements,
                (*(*union_name).mech_type).length as usize,
            ) == 0i32)
    {
        internal_name = (*union_name).mech_name
    } else {
        status = crate::src::mechglue::g_glue::gssint_import_internal_name(
            minor_status,
            selected_mech,
            union_name,
            &mut internal_name,
        );
        if status != 0u32 {
            return status;
        }
    }
    /*
     * if context_handle is GSS_C_NO_CONTEXT, allocate a union context
     * descriptor to hold the mech type information as well as the
     * underlying mechanism context handle. Otherwise, cast the
     * value of *context_handle to the union context variable.
     */
    if (*context_handle).is_null() {
        status = crate::src::mechglue::g_glue::gssint_create_union_context(
            minor_status,
            selected_mech as crate::gssapi_h::gss_const_OID,
            &mut union_ctx_id,
        );
        if status != 0u32 {
            current_block = 15764960595762745900;
        } else {
            current_block = 14763689060501151050;
        }
    } else {
        union_ctx_id = *context_handle as crate::mglueP_h::gss_union_ctx_id_t;
        if (*union_ctx_id).internal_ctx_id.is_null() {
            status = (8u32) << 16i32;
            current_block = 15764960595762745900;
        } else {
            current_block = 14763689060501151050;
        }
    }
    match current_block {
        14763689060501151050 => {
            /*
             * get the appropriate cred handle from the union cred struct.
             * defaults to GSS_C_NO_CREDENTIAL if there is no cred, which will
             * use the default credential.
             */
            union_cred = claimant_cred_handle;
            input_cred_handle =
                crate::src::mechglue::g_glue::gssint_get_mechanism_cred(union_cred, selected_mech);
            /*
             * now call the approprate underlying mechanism routine
             */
            status = (*mech)
                .gss_init_sec_context
                .expect("non-null function pointer")(
                minor_status,
                input_cred_handle,
                &mut (*union_ctx_id).internal_ctx_id,
                internal_name,
                crate::src::mechglue::g_initialize::gssint_get_public_oid(
                    selected_mech as crate::gssapi_h::gss_const_OID,
                ),
                req_flags,
                time_req,
                input_chan_bindings,
                input_token,
                actual_mech_type,
                output_token,
                ret_flags,
                time_rec,
            );
            if status != 0u32 && status != ((1i32) << 0i32 + 0i32) as u32 {
                /*
                 * RFC 2744 5.19 requires that we not create a context on a failed
                 * first call to init, and recommends that on a failed subsequent call
                 * we make the caller responsible for calling gss_delete_sec_context.
                 * Even if the mech deleted its context, keep the union context around
                 * for the caller to delete.
                 */
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                );
                if (*context_handle).is_null() {
                    crate::stdlib::free((*(*union_ctx_id).mech_type).elements);
                    crate::stdlib::free((*union_ctx_id).mech_type as *mut libc::c_void);
                    crate::stdlib::free(union_ctx_id as *mut libc::c_void);
                }
            } else if (*context_handle).is_null() {
                *context_handle = union_ctx_id as crate::gssapi_h::gss_ctx_id_t
            }
        }
        _ => {}
    }
    if (*union_name).mech_name.is_null() || (*union_name).mech_name != internal_name {
        crate::src::mechglue::g_glue::gssint_release_internal_name(
            &mut temp_minor_status,
            selected_mech,
            &mut internal_name,
        );
    }
    return status;
}
