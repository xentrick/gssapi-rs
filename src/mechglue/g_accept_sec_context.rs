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
pub use crate::mglueP_h::gss_union_cred_desc;
pub use crate::mglueP_h::gss_union_cred_t;
pub use crate::mglueP_h::gss_union_ctx_id_struct;
pub use crate::mglueP_h::gss_union_ctx_id_t;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH;

pub use crate::src::mechglue::g_glue::gssint_convert_name_to_union_name;
pub use crate::src::mechglue::g_glue::gssint_create_union_context;
pub use crate::src::mechglue::g_glue::gssint_get_mech_type;
pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;

/* #pragma ident	"@(#)g_accept_sec_context.c	1.19	04/02/23 SMI" */
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
 *  glue routine for gss_accept_sec_context
 */

unsafe extern "C" fn val_acc_sec_ctx_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut _verifier_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut input_token_buffer: crate::gssapi_h::gss_buffer_t,
    mut _input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut d_cred: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !src_name.is_null() {
        *src_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !mech_type.is_null() {
        *mech_type = 0 as crate::gssapi_h::gss_OID
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
    if !d_cred.is_null() {
        *d_cred = 0 as crate::gssapi_h::gss_cred_id_t
    }
    /* Validate arguments. */
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if context_handle.is_null() {
        return (2u32) << 24i32;
    }
    if input_token_buffer.is_null() {
        return (1u32) << 24i32;
    }
    if output_token.is_null() {
        return (2u32) << 24i32;
    }
    return 0u32;
}
/* Return true if mech should be accepted with no acceptor credential. */

unsafe extern "C" fn allow_mech_by_default(mut mech: crate::gssapi_h::gss_OID) -> i32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut attrs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut reject = 0i32;
    let mut p: i32 = 0;
    /* Whether we accept an interposer mech depends on whether we accept the
     * mech it interposes. */
    mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return 0i32;
    }
    status = crate::src::mechglue::g_mechattr::gss_inquire_attrs_for_mech(
        &mut minor,
        mech as crate::gssapi_h::gss_const_OID,
        &mut attrs,
        0 as *mut crate::gssapi_h::gss_OID_set,
    );
    if status != 0 {
        return 0i32;
    }
    /* If the mechanism doesn't support RFC 5587, don't exclude it. */
    if attrs.is_null() {
        return 1i32;
    }
    /* Check for each attribute which would cause us to exclude this mech from
     * the default credential. */
    if crate::src::generic::oid_ops::generic_gss_test_oid_set_member(
        &mut minor,
        crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED,
        attrs,
        &mut p,
    ) != 0u32
        || p != 0
    {
        reject = 1i32
    } else if crate::src::generic::oid_ops::generic_gss_test_oid_set_member(
        &mut minor,
        crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH,
        attrs,
        &mut p,
    ) != 0u32
        || p != 0
    {
        reject = 1i32
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut minor, &mut attrs);
    return (reject == 0) as i32;
}
/* time_rec */
#[no_mangle]

pub unsafe extern "C" fn gss_accept_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut verifier_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut input_token_buffer: crate::gssapi_h::gss_buffer_t,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut d_cred: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_ret_flags = 0u32;
    let mut union_ctx_id = 0 as crate::mglueP_h::gss_union_ctx_id_t;
    let mut input_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut tmp_d_cred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut internal_name = 0 as crate::gssapi_h::gss_name_t;
    let mut tmp_src_name = 0 as crate::gssapi_h::gss_name_t;
    let mut token_mech_type_desc = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut token_mech_type: crate::gssapi_h::gss_OID = &mut token_mech_type_desc;
    let mut actual_mech = 0 as crate::gssapi_h::gss_OID;
    let mut selected_mech = 0 as crate::gssapi_h::gss_OID;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as crate::mglueP_h::gss_mechanism;
    let mut uc = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut i: i32 = 0;
    status = val_acc_sec_ctx_args(
        minor_status,
        context_handle,
        verifier_cred_handle,
        input_token_buffer,
        input_chan_bindings,
        src_name,
        mech_type,
        output_token,
        ret_flags,
        time_rec,
        d_cred,
    );
    if status != 0u32 {
        return status;
    }
    /*
     * if context_handle is GSS_C_NO_CONTEXT, allocate a union context
     * descriptor to hold the mech type information as well as the
     * underlying mechanism context handle. Otherwise, cast the
     * value of *context_handle to the union context variable.
     */
    if (*context_handle).is_null() {
        if input_token_buffer.is_null() {
            return (1u32) << 24i32;
        }
        /* Get the token mech type */
        status =
            crate::src::mechglue::g_glue::gssint_get_mech_type(token_mech_type, input_token_buffer);
        if status != 0 {
            return status;
        }
        /*
         * An interposer calling back into the mechglue can't pass in a special
         * mech, so we have to recognize it using verifier_cred_handle.  Use
         * the mechanism for which we have matching creds, if available.
         */
        if !verifier_cred_handle.is_null() {
            uc = verifier_cred_handle;
            i = 0i32;
            while i < (*uc).count {
                public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
                    &mut *(*uc).mechs_array.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct
                        as crate::gssapi_h::gss_const_OID,
                );
                if !public_mech.is_null()
                    && ((*token_mech_type).length == (*public_mech).length
                        && crate::stdlib::memcmp(
                            (*token_mech_type).elements,
                            (*public_mech).elements,
                            (*token_mech_type).length as usize,
                        ) == 0i32)
                {
                    selected_mech = &mut *(*uc).mechs_array.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct;
                    break;
                } else {
                    i += 1
                }
            }
        }
        if selected_mech.is_null() {
            status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
                minor_status,
                token_mech_type as crate::gssapi_h::gss_const_OID,
                &mut selected_mech,
            );
            if status != 0 {
                return status;
            }
        }
    } else {
        union_ctx_id = *context_handle as crate::mglueP_h::gss_union_ctx_id_t;
        selected_mech = (*union_ctx_id).mech_type;
        if (*union_ctx_id).internal_ctx_id.is_null() {
            return (8u32) << 16i32;
        }
    }
    /* Now create a new context if we didn't get one. */
    if (*context_handle).is_null() {
        status = crate::src::mechglue::g_glue::gssint_create_union_context(
            minor_status,
            selected_mech as crate::gssapi_h::gss_const_OID,
            &mut union_ctx_id,
        );
        if status != 0u32 {
            return status;
        }
    }
    /*
     * get the appropriate cred handle from the union cred struct.
     */
    if !verifier_cred_handle.is_null() {
        input_cred_handle = crate::src::mechglue::g_glue::gssint_get_mechanism_cred(
            verifier_cred_handle,
            selected_mech,
        );
        if input_cred_handle.is_null() {
            /* verifier credential specified but no acceptor credential found */
            status = (7u32) << 16i32;
            current_block = 3461952770807793116;
        } else {
            current_block = 6174974146017752131;
        }
    } else if allow_mech_by_default(selected_mech) == 0 {
        status = (7u32) << 16i32;
        current_block = 3461952770807793116;
    } else {
        current_block = 6174974146017752131;
    }
    match current_block {
        6174974146017752131 => {
            /*
             * now select the approprate underlying mechanism routine and
             * call it.
             */
            mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
                selected_mech as crate::gssapi_h::gss_const_OID,
            );
            if !mech.is_null() && (*mech).gss_accept_sec_context.is_some() {
                status = (*mech)
                    .gss_accept_sec_context
                    .expect("non-null function pointer")(
                    minor_status,
                    &mut (*union_ctx_id).internal_ctx_id,
                    input_cred_handle,
                    input_token_buffer,
                    input_chan_bindings,
                    if !src_name.is_null() {
                        &mut internal_name
                    } else {
                        0 as *mut crate::gssapi_h::gss_name_t
                    },
                    &mut actual_mech,
                    output_token,
                    &mut temp_ret_flags,
                    time_rec,
                    if !d_cred.is_null() {
                        &mut tmp_d_cred
                    } else {
                        0 as *mut crate::gssapi_h::gss_cred_id_t
                    },
                );
                /* If there's more work to do, keep going... */
                if status == ((1i32) << 0i32 + 0i32) as u32 {
                    *context_handle = union_ctx_id as crate::gssapi_h::gss_ctx_id_t;
                    return ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32;
                }
                /* if the call failed, return with failure */
                if status != 0u32 {
                    *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                        *minor_status,
                        &mut (*mech).mech_type,
                    )
                } else {
                    /*
                     * if src_name is non-NULL,
                     * convert internal_name into a union name equivalent
                     * First call the mechanism specific display_name()
                     * then call gss_import_name() to create
                     * the union name struct cast to src_name
                     */
                    if !src_name.is_null() {
                        if !internal_name.is_null() {
                            /* consumes internal_name regardless of success */
                            temp_status =
                                crate::src::mechglue::g_glue::gssint_convert_name_to_union_name(
                                    &mut temp_minor_status,
                                    mech,
                                    internal_name,
                                    &mut tmp_src_name,
                                );
                            if temp_status != 0u32 {
                                status = temp_status;
                                *minor_status = temp_minor_status;
                                *minor_status =
                                    crate::src::generic::util_errmap::gssint_mecherrmap_map(
                                        *minor_status,
                                        &mut (*mech).mech_type,
                                    );
                                if (*output_token).length != 0 {
                                    crate::src::mechglue::g_rel_buffer::gss_release_buffer(
                                        &mut temp_minor_status,
                                        output_token,
                                    );
                                }
                                current_block = 3461952770807793116;
                            } else {
                                *src_name = tmp_src_name;
                                current_block = 12758904613967585247;
                            }
                        } else {
                            *src_name = 0 as crate::gssapi_h::gss_name_t;
                            current_block = 12758904613967585247;
                        }
                    } else {
                        current_block = 12758904613967585247;
                    }
                    match current_block {
                        3461952770807793116 => {}
                        _ =>
                        /* Ensure we're returning correct creds format */
                        {
                            if temp_ret_flags & 1u32 != 0 && !tmp_d_cred.is_null() {
                                public_mech =
                                    crate::src::mechglue::g_initialize::gssint_get_public_oid(
                                        selected_mech as crate::gssapi_h::gss_const_OID,
                                    );
                                if !actual_mech.is_null()
                                    && !public_mech.is_null()
                                    && !((*actual_mech).length >= (*public_mech).length
                                        && crate::stdlib::memcmp(
                                            (*actual_mech).elements,
                                            (*public_mech).elements,
                                            (*public_mech).length as usize,
                                        ) == 0i32)
                                {
                                    *d_cred = tmp_d_cred;
                                    current_block = 10261677128829721533;
                                /* unwrapped pseudo-mech */
                                } else {
                                    let mut d_u_cred = 0 as crate::mglueP_h::gss_union_cred_t;
                                    d_u_cred = crate::stdlib::malloc(::std::mem::size_of::<
                                        crate::mglueP_h::gss_union_cred_desc,
                                    >(
                                    ))
                                        as crate::mglueP_h::gss_union_cred_t;
                                    if d_u_cred.is_null() {
                                        status = (13u32) << 16i32;
                                        current_block = 3461952770807793116;
                                    } else {
                                        crate::stdlib::memset(
                                            d_u_cred as *mut libc::c_void,
                                            0i32,
                                            ::std::mem::size_of::<
                                                crate::mglueP_h::gss_union_cred_desc,
                                            >(),
                                        );
                                        (*d_u_cred).count = 1i32;
                                        status = crate::src::generic::oid_ops::generic_gss_copy_oid(
                                            &mut temp_minor_status,
                                            selected_mech as *const crate::gssapi_h::gss_OID_desc,
                                            &mut (*d_u_cred).mechs_array,
                                        );
                                        if status != 0u32 {
                                            crate::stdlib::free(d_u_cred as *mut libc::c_void);
                                            current_block = 3461952770807793116;
                                        } else {
                                            (*d_u_cred).cred_array =
                                                crate::stdlib::malloc(::std::mem::size_of::<
                                                    crate::gssapi_h::gss_cred_id_t,
                                                >(
                                                ))
                                                    as *mut crate::gssapi_h::gss_cred_id_t;
                                            if !(*d_u_cred).cred_array.is_null() {
                                                let ref mut fresh0 =
                                                    *(*d_u_cred).cred_array.offset(0isize);
                                                *fresh0 = tmp_d_cred;
                                                (*d_u_cred).loopback = d_u_cred;
                                                *d_cred = d_u_cred;
                                                current_block = 10261677128829721533;
                                            } else {
                                                crate::stdlib::free(d_u_cred as *mut libc::c_void);
                                                status = (13u32) << 16i32;
                                                current_block = 3461952770807793116;
                                            }
                                        }
                                    }
                                }
                            } else {
                                current_block = 10261677128829721533;
                            }
                            match current_block {
                                3461952770807793116 => {}
                                _ => {
                                    if !mech_type.is_null() {
                                        *mech_type =
                                            crate::src::mechglue::g_initialize::gssint_get_public_oid(actual_mech
                                                                      as
                                                                      crate::gssapi_h::gss_const_OID)
                                    }
                                    if !ret_flags.is_null() {
                                        *ret_flags = temp_ret_flags
                                    }
                                    *context_handle = union_ctx_id as crate::gssapi_h::gss_ctx_id_t;
                                    return 0u32;
                                }
                            }
                        }
                    }
                }
            } else {
                status = (1u32) << 16i32
            }
        }
        _ => {}
    }
    /*
     * RFC 2744 5.1 requires that we not create a context on a failed first
     * call to accept, and recommends that on a failed subsequent call we
     * make the caller responsible for calling gss_delete_sec_context.
     * Even if the mech deleted its context, keep the union context around
     * for the caller to delete.
     */
    if !union_ctx_id.is_null() && (*context_handle).is_null() {
        if !(*union_ctx_id).mech_type.is_null() {
            if !(*(*union_ctx_id).mech_type).elements.is_null() {
                crate::stdlib::free((*(*union_ctx_id).mech_type).elements);
            }
            crate::stdlib::free((*union_ctx_id).mech_type as *mut libc::c_void);
        }
        if !(*union_ctx_id).internal_ctx_id.is_null()
            && !mech.is_null()
            && (*mech).gss_delete_sec_context.is_some()
        {
            (*mech)
                .gss_delete_sec_context
                .expect("non-null function pointer")(
                &mut temp_minor_status,
                &mut (*union_ctx_id).internal_ctx_id,
                0 as crate::gssapi_h::gss_buffer_t,
            );
        }
        crate::stdlib::free(union_ctx_id as *mut libc::c_void);
    }
    if !src_name.is_null() {
        *src_name = 0 as crate::gssapi_h::gss_name_t
    }
    if !tmp_src_name.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(
            &mut temp_minor_status,
            tmp_src_name as crate::gssapi_h::gss_buffer_t,
        );
    }
    return status;
}
/* LEAN_CLIENT */
