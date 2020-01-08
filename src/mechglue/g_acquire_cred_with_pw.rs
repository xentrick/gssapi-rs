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
pub use crate::gssapi_h::gss_OID_set_desc;
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
pub use crate::mglueP_h::gss_union_name_t;

pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_make_public_oid_set;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set;
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;

/* #pragma ident	"@(#)g_acquire_cred.c	1.22	04/02/23 SMI" */
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
 *  glue routine for gss_acquire_cred_with_password
 */

unsafe extern "C" fn val_acq_cred_pw_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    password: crate::gssapi_h::gss_buffer_t,
    mut _time_req: crate::gssapi_h::OM_uint32,
    _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_cred_handle.is_null() {
        *output_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    if !actual_mechs.is_null() {
        *actual_mechs = 0 as crate::gssapi_h::gss_OID_set
    }
    if !time_rec.is_null() {
        *time_rec = 0u32
    }
    /* Validate arguments. */
    if desired_name.is_null() {
        return (2u32) << 16i32;
    }
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if output_cred_handle.is_null() {
        return (2u32) << 24i32;
    }
    if cred_usage != 2i32 && cred_usage != 1i32 && cred_usage != 0i32 {
        if !minor_status.is_null() {
            *minor_status = 22u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13u32) << 16i32;
    }
    if password.is_null() || (*password).length == 0usize || (*password).value.is_null() {
        if !minor_status.is_null() {
            *minor_status = 22u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13u32) << 16i32;
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_acquire_cred_with_password(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_name: crate::gssapi_h::gss_name_t,
    password: crate::gssapi_h::gss_buffer_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major = (13u32) << 16i32;
    let mut initTimeOut: crate::gssapi_h::OM_uint32 = 0;
    let mut acceptTimeOut: crate::gssapi_h::OM_uint32 = 0;
    let mut outTime = 0xffffffffu32;
    let mut default_OID_set = crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
    let mut mechs = 0 as *mut crate::gssapi_h::gss_OID_set_desc_struct;
    let mut default_OID = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut i: u32 = 0;
    let mut creds = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    major = val_acq_cred_pw_args(
        minor_status,
        desired_name,
        password,
        time_req,
        desired_mechs,
        cred_usage,
        output_cred_handle,
        actual_mechs,
        time_rec,
    );
    if major != 0u32 {
        return major;
    }
    /* Initial value needed below. */
    major = (13u32) << 16i32;
    /*
     * if desired_mechs equals GSS_C_NULL_OID_SET, then pick an
     * appropriate default.  We use the first mechanism in the
     * mechansim list as the default. This set is created with
     * statics thus needs not be freed
     */
    if desired_mechs.is_null() {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            0 as crate::gssapi_h::gss_const_OID,
        );
        if mech.is_null() {
            return (1u32) << 16i32;
        }
        mechs = &mut default_OID_set;
        default_OID_set.count = 1usize;
        default_OID_set.elements = &mut default_OID;
        default_OID.length = (*mech).mech_type.length;
        default_OID.elements = (*mech).mech_type.elements
    } else {
        mechs = desired_mechs
    }
    if (*mechs).count == 0usize {
        return (1u32) << 16i32;
    }
    /* allocate the output credential structure */
    creds = crate::stdlib::malloc(::std::mem::size_of::<crate::mglueP_h::gss_union_cred_desc>())
        as crate::mglueP_h::gss_union_cred_t;
    if creds.is_null() {
        return (13u32) << 16i32;
    }
    /* initialize to 0s */
    crate::stdlib::memset(
        creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::mglueP_h::gss_union_cred_desc>(),
    );
    (*creds).loopback = creds;
    /* for each requested mech attempt to obtain a credential */
    i = 0u32; /* for */
    while (i as usize) < (*mechs).count {
        major = gss_add_cred_with_password(
            minor_status,
            creds,
            desired_name,
            &mut *(*mechs).elements.offset(i as isize),
            password,
            cred_usage,
            time_req,
            time_req,
            0 as *mut crate::gssapi_h::gss_cred_id_t,
            0 as *mut crate::gssapi_h::gss_OID_set,
            &mut initTimeOut,
            &mut acceptTimeOut,
        );
        if major == 0u32 {
            /* update the credential's time */
            if cred_usage == 2i32 {
                if outTime > acceptTimeOut {
                    outTime = acceptTimeOut
                }
            } else if cred_usage == 1i32 {
                if outTime > initTimeOut {
                    outTime = initTimeOut
                }
            } else if initTimeOut > acceptTimeOut {
                outTime = if outTime > acceptTimeOut {
                    acceptTimeOut
                } else {
                    outTime
                }
            } else {
                outTime = if outTime > initTimeOut {
                    initTimeOut
                } else {
                    outTime
                }
            }
        }
        i = i.wrapping_add(1)
    }
    /*
     * time_rec is the lesser of the
     * init/accept times
     */
    /* ensure that we have at least one credential element */
    if (*creds).count < 1i32 {
        crate::stdlib::free(creds as *mut libc::c_void);
        return major;
    }
    /*
     * fill in output parameters
     * setup the actual mechs output parameter
     */
    if !actual_mechs.is_null() {
        major = crate::src::mechglue::g_initialize::gssint_make_public_oid_set(
            minor_status,
            (*creds).mechs_array,
            (*creds).count,
            actual_mechs,
        );
        if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            crate::src::mechglue::g_rel_cred::gss_release_cred(
                minor_status,
                &mut creds as *mut crate::mglueP_h::gss_union_cred_t,
            );
            return major;
        }
    }
    if !time_rec.is_null() {
        *time_rec = outTime
    }
    (*creds).loopback = creds;
    *output_cred_handle = creds;
    return 0u32;
}

unsafe extern "C" fn val_add_cred_pw_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_name: crate::gssapi_h::gss_name_t,
    _desired_mech: crate::gssapi_h::gss_OID,
    password: crate::gssapi_h::gss_buffer_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut _initiator_time_req: crate::gssapi_h::OM_uint32,
    mut _acceptor_time_req: crate::gssapi_h::OM_uint32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut initiator_time_rec: *mut crate::gssapi_h::OM_uint32,
    mut acceptor_time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    /* Initialize outputs. */
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !output_cred_handle.is_null() {
        *output_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    if !actual_mechs.is_null() {
        *actual_mechs = 0 as crate::gssapi_h::gss_OID_set
    }
    if !acceptor_time_rec.is_null() {
        *acceptor_time_rec = 0u32
    }
    if !initiator_time_rec.is_null() {
        *initiator_time_rec = 0u32
    }
    /* Validate arguments. */
    if desired_name.is_null() {
        return (2u32) << 16i32;
    }
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    if input_cred_handle.is_null() && output_cred_handle.is_null() {
        return (2u32) << 24i32 | (7u32) << 16i32;
    }
    if cred_usage != 2i32 && cred_usage != 1i32 && cred_usage != 0i32 {
        if !minor_status.is_null() {
            *minor_status = 22u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13u32) << 16i32;
    }
    if password.is_null() || (*password).length == 0usize || (*password).value.is_null() {
        if !minor_status.is_null() {
            *minor_status = 22u32;
            *minor_status =
                crate::src::generic::util_errmap::gssint_mecherrmap_map_errcode(*minor_status)
        }
        return (13u32) << 16i32;
    }
    return 0u32;
}
/* V2 KRB5_CALLCONV */
#[no_mangle]

pub unsafe extern "C" fn gss_add_cred_with_password(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    desired_name: crate::gssapi_h::gss_name_t,
    desired_mech: crate::gssapi_h::gss_OID,
    password: crate::gssapi_h::gss_buffer_t,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut initiator_time_req: crate::gssapi_h::OM_uint32,
    mut acceptor_time_req: crate::gssapi_h::OM_uint32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut initiator_time_rec: *mut crate::gssapi_h::OM_uint32,
    mut acceptor_time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut time_req: crate::gssapi_h::OM_uint32 = 0;
    let mut time_rec: crate::gssapi_h::OM_uint32 = 0;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut new_union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut internal_name = 0 as crate::gssapi_h::gss_name_t;
    let mut allocated_name = 0 as crate::gssapi_h::gss_name_t;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut cred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut new_mechs_array = 0 as crate::gssapi_h::gss_OID;
    let mut new_cred_array = 0 as *mut crate::gssapi_h::gss_cred_id_t;
    let mut target_mechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut selected_mech = 0 as crate::gssapi_h::gss_OID;
    status = val_add_cred_pw_args(
        minor_status,
        input_cred_handle,
        desired_name,
        desired_mech,
        password,
        cred_usage,
        initiator_time_req,
        acceptor_time_req,
        output_cred_handle,
        actual_mechs,
        initiator_time_rec,
        acceptor_time_rec,
    );
    if status != 0u32 {
        return status;
    }
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor_status,
        desired_mech as crate::gssapi_h::gss_const_OID,
        &mut selected_mech,
    );
    if status != 0u32 {
        return status;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gssspi_acquire_cred_with_password.is_none() {
        return (16u32) << 16i32;
    }
    if input_cred_handle.is_null() {
        union_cred = crate::stdlib::malloc(::std::mem::size_of::<
            crate::mglueP_h::gss_union_cred_desc,
        >()) as crate::mglueP_h::gss_union_cred_t;
        if union_cred.is_null() {
            return (13u32) << 16i32;
        }
        crate::stdlib::memset(
            union_cred as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<crate::mglueP_h::gss_union_cred_desc>(),
        );
        /* for default credentials we will use GSS_C_NO_NAME */
        internal_name = 0 as crate::gssapi_h::gss_name_t
    } else {
        union_cred = input_cred_handle;
        if !crate::src::mechglue::g_glue::gssint_get_mechanism_cred(union_cred, selected_mech)
            .is_null()
        {
            return (17u32) << 16i32;
        }
    }
    /* may need to create a mechanism specific name */
    union_name = desired_name;
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
        if crate::src::mechglue::g_glue::gssint_import_internal_name(
            minor_status,
            selected_mech,
            union_name,
            &mut allocated_name,
        ) != 0u32
        {
            return (2u32) << 16i32;
        }
        internal_name = allocated_name
    }
    if cred_usage == 2i32 {
        time_req = acceptor_time_req
    } else if cred_usage == 1i32 {
        time_req = initiator_time_req
    } else if cred_usage == 0i32 {
        time_req = if acceptor_time_req > initiator_time_req {
            acceptor_time_req
        } else {
            initiator_time_req
        }
    } else {
        time_req = 0u32
    }
    status =
        crate::src::mechglue::g_oid_ops::gss_create_empty_oid_set(minor_status, &mut target_mechs);
    if !(status != 0u32) {
        status = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
            minor_status,
            crate::src::mechglue::g_initialize::gssint_get_public_oid(
                selected_mech as crate::gssapi_h::gss_const_OID,
            ),
            &mut target_mechs,
        );
        if !(status != 0u32) {
            status = (*mech)
                .gssspi_acquire_cred_with_password
                .expect("non-null function pointer")(
                minor_status,
                internal_name,
                password,
                time_req,
                target_mechs,
                cred_usage,
                &mut cred,
                0 as *mut crate::gssapi_h::gss_OID_set,
                &mut time_rec,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            } else {
                /* now add the new credential elements */
                new_mechs_array = crate::stdlib::malloc(
                    (::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>())
                        .wrapping_mul(((*union_cred).count + 1i32) as usize),
                ) as crate::gssapi_h::gss_OID;
                new_cred_array = crate::stdlib::malloc(
                    (::std::mem::size_of::<crate::gssapi_h::gss_cred_id_t>())
                        .wrapping_mul(((*union_cred).count + 1i32) as usize),
                ) as *mut crate::gssapi_h::gss_cred_id_t;
                if new_mechs_array.is_null() || new_cred_array.is_null() {
                    status = (13u32) << 16i32
                } else {
                    if !acceptor_time_rec.is_null() {
                        if cred_usage == 2i32 || cred_usage == 0i32 {
                            *acceptor_time_rec = time_rec
                        }
                    }
                    if !initiator_time_rec.is_null() {
                        if cred_usage == 1i32 || cred_usage == 0i32 {
                            *initiator_time_rec = time_rec
                        }
                    }
                    /*
                     * OK, expand the mechanism array and the credential array
                     */
                    crate::stdlib::memcpy(
                        new_mechs_array as *mut libc::c_void,
                        (*union_cred).mechs_array as *const libc::c_void,
                        (::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>())
                            .wrapping_mul((*union_cred).count as usize),
                    );
                    crate::stdlib::memcpy(
                        new_cred_array as *mut libc::c_void,
                        (*union_cred).cred_array as *const libc::c_void,
                        (::std::mem::size_of::<crate::gssapi_h::gss_cred_id_t>())
                            .wrapping_mul((*union_cred).count as usize),
                    );
                    let ref mut fresh0 = *new_cred_array.offset((*union_cred).count as isize);
                    *fresh0 = cred;
                    let ref mut fresh1 =
                        (*new_mechs_array.offset((*union_cred).count as isize)).elements;
                    *fresh1 = crate::stdlib::malloc((*selected_mech).length as usize);
                    if !(*fresh1).is_null() {
                        crate::stdlib::memcpy(
                            (*new_mechs_array.offset((*union_cred).count as isize)).elements,
                            (*selected_mech).elements,
                            (*selected_mech).length as usize,
                        );
                        (*new_mechs_array.offset((*union_cred).count as isize)).length =
                            (*selected_mech).length;
                        if !actual_mechs.is_null() {
                            status = crate::src::mechglue::g_initialize::gssint_make_public_oid_set(
                                minor_status,
                                new_mechs_array,
                                (*union_cred).count + 1i32,
                                actual_mechs,
                            );
                            if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                                crate::stdlib::free(
                                    (*new_mechs_array.offset((*union_cred).count as isize))
                                        .elements,
                                );
                                current_block = 5305876222738625659;
                            } else {
                                current_block = 12758904613967585247;
                            }
                        } else {
                            current_block = 12758904613967585247;
                        }
                        match current_block {
                            5305876222738625659 => {}
                            _ => {
                                if output_cred_handle.is_null() {
                                    crate::stdlib::free(
                                        (*union_cred).mechs_array as *mut libc::c_void,
                                    );
                                    crate::stdlib::free(
                                        (*union_cred).cred_array as *mut libc::c_void,
                                    );
                                    new_union_cred = union_cred;
                                    current_block = 11052029508375673978;
                                } else {
                                    new_union_cred = crate::stdlib::malloc(::std::mem::size_of::<
                                        crate::mglueP_h::gss_union_cred_desc,
                                    >(
                                    ))
                                        as crate::mglueP_h::gss_union_cred_t;
                                    if new_union_cred.is_null() {
                                        crate::stdlib::free(
                                            (*new_mechs_array.offset((*union_cred).count as isize))
                                                .elements,
                                        );
                                        current_block = 5305876222738625659;
                                    } else {
                                        *new_union_cred = *union_cred;
                                        *output_cred_handle = new_union_cred;
                                        current_block = 11052029508375673978;
                                    }
                                }
                                match current_block {
                                    5305876222738625659 => {}
                                    _ => {
                                        (*new_union_cred).mechs_array = new_mechs_array;
                                        (*new_union_cred).cred_array = new_cred_array;
                                        (*new_union_cred).count += 1;
                                        (*new_union_cred).loopback = new_union_cred;
                                        /* We're done with the internal name. Free it if we allocated it. */
                                        if !allocated_name.is_null() {
                                            crate::src::mechglue::g_glue::gssint_release_internal_name(&mut temp_minor_status,
                                                                         selected_mech,
                                                                         &mut allocated_name);
                                        }
                                        if !target_mechs.is_null() {
                                            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut temp_minor_status,
                                                                &mut target_mechs);
                                        }
                                        return 0u32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !new_mechs_array.is_null() {
        crate::stdlib::free(new_mechs_array as *mut libc::c_void);
    }
    if !new_cred_array.is_null() {
        crate::stdlib::free(new_cred_array as *mut libc::c_void);
    }
    if !cred.is_null() && (*mech).gss_release_cred.is_some() {
        (*mech).gss_release_cred.expect("non-null function pointer")(
            &mut temp_minor_status,
            &mut cred,
        );
    }
    if !allocated_name.is_null() {
        crate::src::mechglue::g_glue::gssint_release_internal_name(
            &mut temp_minor_status,
            selected_mech,
            &mut allocated_name,
        );
    }
    if !target_mechs.is_null() {
        crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
            &mut temp_minor_status,
            &mut target_mechs,
        );
    }
    if input_cred_handle.is_null() && !union_cred.is_null() {
        crate::stdlib::free(union_cred as *mut libc::c_void);
    }
    return status;
}
