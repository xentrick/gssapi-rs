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
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_OID_set;
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
pub use crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
pub use crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH;

pub use crate::src::mechglue::g_glue::gssint_get_mechanism_cred;
pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_make_public_oid_set;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_mechattr::gss_indicate_mechs_by_attrs;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;
pub use crate::src::mechglue::g_rel_name::gss_release_name;

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
 *  glue routine for gss_acquire_cred
 */

unsafe extern "C" fn val_acq_cred_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut _desired_name: crate::gssapi_h::gss_name_t,
    mut _time_req: crate::gssapi_h::OM_uint32,
    mut _desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut _cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
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
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_acquire_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return gss_acquire_cred_from(
        minor_status,
        desired_name,
        time_req,
        desired_mechs,
        cred_usage,
        0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
        output_cred_handle,
        actual_mechs,
        time_rec,
    );
}
#[no_mangle]

pub unsafe extern "C" fn gss_acquire_cred_from(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut desired_mechs: crate::gssapi_h::gss_OID_set,
    mut cred_usage: i32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major = (13u32) << 16i32;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut first_major = 0u32;
    let mut first_minor = 0u32;
    let mut initTimeOut = 0u32;
    let mut acceptTimeOut = 0u32;
    let mut outTime = 0xffffffffu32;
    let mut mechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut except_attrs = crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
    let mut attr_oids: [crate::gssapi_h::gss_OID_desc; 2] = [crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    }; 2];
    let mut i: u32 = 0;
    let mut creds = 0 as crate::mglueP_h::gss_union_cred_t;
    major = val_acq_cred_args(
        minor_status,
        desired_name,
        time_req,
        desired_mechs,
        cred_usage,
        cred_store,
        output_cred_handle,
        actual_mechs,
        time_rec,
    );
    if !(major != 0u32) {
        /*
         * if desired_mechs equals GSS_C_NULL_OID_SET, then try to
         * acquire credentials for all non-deprecated mechanisms.
         */
        if desired_mechs.is_null() {
            attr_oids[0usize] = *crate::src::generic::gssapi_generic::GSS_C_MA_DEPRECATED;
            attr_oids[1usize] = *crate::src::generic::gssapi_generic::GSS_C_MA_NOT_DFLT_MECH;
            except_attrs.count = 2usize;
            except_attrs.elements = attr_oids.as_mut_ptr();
            major = crate::src::mechglue::g_mechattr::gss_indicate_mechs_by_attrs(
                minor_status,
                0 as crate::gssapi_h::gss_const_OID_set,
                &mut except_attrs as *mut crate::gssapi_h::gss_OID_set_desc
                    as crate::gssapi_h::gss_const_OID_set,
                0 as crate::gssapi_h::gss_const_OID_set,
                &mut mechs,
            );
            if major != 0u32 {
                current_block = 13371929842352464343;
            } else {
                current_block = 12599329904712511516;
            }
        } else {
            mechs = desired_mechs;
            current_block = 12599329904712511516;
        }
        match current_block {
            13371929842352464343 => {}
            _ => {
                if (*mechs).count == 0usize {
                    major = (1u32) << 16i32
                } else {
                    /* allocate the output credential structure */
                    creds = crate::stdlib::calloc(
                        1usize,
                        ::std::mem::size_of::<crate::mglueP_h::gss_union_cred_desc>(),
                    ) as crate::mglueP_h::gss_union_cred_t;
                    if creds.is_null() {
                        major = (13u32) << 16i32;
                        *minor_status = 12u32
                    } else {
                        (*creds).count = 0i32;
                        (*creds).loopback = creds;
                        /* for each requested mech attempt to obtain a credential */
                        i = 0u32; /* for */
                        major = (16u32) << 16i32;
                        while (i as usize) < (*mechs).count {
                            major = gss_add_cred_from(
                                &mut tmpMinor,
                                creds,
                                desired_name,
                                &mut *(*mechs).elements.offset(i as isize),
                                cred_usage,
                                time_req,
                                time_req,
                                cred_store,
                                0 as *mut crate::gssapi_h::gss_cred_id_t,
                                0 as *mut crate::gssapi_h::gss_OID_set,
                                if !time_rec.is_null() {
                                    &mut initTimeOut
                                } else {
                                    0 as *mut crate::gssapi_h::OM_uint32
                                },
                                if !time_rec.is_null() {
                                    &mut acceptTimeOut
                                } else {
                                    0 as *mut crate::gssapi_h::OM_uint32
                                },
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
                            } else if first_major == 0u32 {
                                first_major = major;
                                first_minor = tmpMinor
                            }
                            i = i.wrapping_add(1)
                        }
                        /*
                         * time_rec is the lesser of the
                         * init/accept times
                         */
                        /* If we didn't get any creds, return the error status from the first mech
                         * (which is often the preferred one). */
                        if (*creds).count < 1i32 {
                            major = first_major;
                            *minor_status = first_minor
                        } else {
                            major = 0u32;
                            /*
                             * fill in output parameters
                             * setup the actual mechs output parameter
                             */
                            if !actual_mechs.is_null() {
                                major =
                                    crate::src::mechglue::g_initialize::gssint_make_public_oid_set(
                                        minor_status,
                                        (*creds).mechs_array,
                                        (*creds).count,
                                        actual_mechs,
                                    );
                                if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                                    current_block = 13371929842352464343;
                                } else {
                                    current_block = 9627623479216730126;
                                }
                            } else {
                                current_block = 9627623479216730126;
                            }
                            match current_block {
                                13371929842352464343 => {}
                                _ => {
                                    if !time_rec.is_null() {
                                        *time_rec = outTime
                                    }
                                    *output_cred_handle = creds
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        crate::src::mechglue::g_rel_cred::gss_release_cred(
            &mut tmpMinor,
            &mut creds as *mut crate::mglueP_h::gss_union_cred_t,
        );
    }
    if desired_mechs.is_null() {
        crate::src::generic::rel_oid_set::generic_gss_release_oid_set(&mut tmpMinor, &mut mechs);
    }
    return major;
}

unsafe extern "C" fn val_add_cred_args(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut _desired_name: crate::gssapi_h::gss_name_t,
    mut _desired_mech: crate::gssapi_h::gss_OID,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut _cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
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
    return 0u32;
}
/* Copy a mechanism credential (with the mechanism given by mech_oid) as
 * faithfully as possible. */

unsafe extern "C" fn copy_mech_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_in: crate::gssapi_h::gss_cred_id_t,
    mut mech_oid: crate::gssapi_h::gss_OID,
    mut cred_out: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut buf = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut life: crate::gssapi_h::OM_uint32 = 0;
    let mut usage: crate::gssapi_h::gss_cred_usage_t = 0;
    let mut oidset = crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        mech_oid as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    if (*mech).gss_export_cred.is_some() && (*mech).gss_import_cred.is_some() {
        status = (*mech).gss_export_cred.expect("non-null function pointer")(
            minor_status,
            cred_in,
            &mut buf,
        );
        if status != 0u32 {
            return status;
        }
        status = (*mech).gss_import_cred.expect("non-null function pointer")(
            minor_status,
            &mut buf,
            cred_out,
        );
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut buf);
    } else if (*mech).gss_inquire_cred.is_some() && (*mech).gss_acquire_cred.is_some() {
        status = (*mech).gss_inquire_cred.expect("non-null function pointer")(
            minor_status,
            cred_in,
            &mut name,
            &mut life,
            &mut usage,
            0 as *mut crate::gssapi_h::gss_OID_set,
        );
        if status != 0u32 {
            return status;
        }
        oidset.count = 1usize;
        oidset.elements = crate::src::mechglue::g_initialize::gssint_get_public_oid(
            mech_oid as crate::gssapi_h::gss_const_OID,
        );
        status = (*mech).gss_acquire_cred.expect("non-null function pointer")(
            minor_status,
            name,
            life,
            &mut oidset,
            usage,
            cred_out,
            0 as *mut crate::gssapi_h::gss_OID_set,
            0 as *mut crate::gssapi_h::OM_uint32,
        );
        crate::src::mechglue::g_rel_name::gss_release_name(&mut tmpmin, &mut name);
    } else {
        status = (16u32) << 16i32
    }
    return status;
}
/* Copy a union credential from cred_in to *cred_out. */

unsafe extern "C" fn copy_union_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_in: crate::gssapi_h::gss_cred_id_t,
    mut cred_out: *mut crate::mglueP_h::gss_union_cred_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut cred = cred_in;
    let mut ncred = 0 as crate::mglueP_h::gss_union_cred_t;
    let mut tmpcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut i: i32 = 0;
    ncred = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::mglueP_h::gss_cred_id_struct>(),
    ) as crate::mglueP_h::gss_union_cred_t;
    if ncred.is_null() {
        current_block = 16626164785316884790;
    } else {
        (*ncred).mechs_array = crate::stdlib::calloc(
            (*cred).count as usize,
            ::std::mem::size_of::<crate::gssapi_h::gss_OID_desc_struct>(),
        ) as crate::gssapi_h::gss_OID;
        (*ncred).cred_array = crate::stdlib::calloc(
            (*cred).count as usize,
            ::std::mem::size_of::<crate::gssapi_h::gss_cred_id_t>(),
        ) as *mut crate::gssapi_h::gss_cred_id_t;
        if (*ncred).mechs_array.is_null() || (*ncred).cred_array.is_null() {
            current_block = 16626164785316884790;
        } else {
            (*ncred).count = (*cred).count;
            i = 0i32;
            loop {
                if !(i < (*cred).count) {
                    current_block = 2370887241019905314;
                    break;
                }
                /* Copy this element's mechanism OID. */
                let ref mut fresh0 = (*(*ncred).mechs_array.offset(i as isize)).elements;
                *fresh0 = crate::stdlib::malloc(
                    (*(*cred).mechs_array.offset(i as isize)).length as usize,
                );
                if (*(*ncred).mechs_array.offset(i as isize))
                    .elements
                    .is_null()
                {
                    current_block = 16626164785316884790;
                    break;
                }
                crate::stdlib::memcpy(
                    (*(*ncred).mechs_array.offset(i as isize)).elements,
                    (*(*cred).mechs_array.offset(i as isize)).elements,
                    (*(*cred).mechs_array.offset(i as isize)).length as usize,
                );
                (*(*ncred).mechs_array.offset(i as isize)).length =
                    (*(*cred).mechs_array.offset(i as isize)).length;
                /* Copy this element's mechanism cred. */
                status = copy_mech_cred(
                    minor_status,
                    *(*cred).cred_array.offset(i as isize),
                    &mut *(*cred).mechs_array.offset(i as isize),
                    &mut *(*ncred).cred_array.offset(i as isize),
                );
                if status != 0u32 {
                    current_block = 9850252977296903552;
                    break;
                }
                i += 1
            }
            match current_block {
                9850252977296903552 => {}
                16626164785316884790 => {}
                _ => {
                    (*ncred).loopback = ncred;
                    *cred_out = ncred;
                    return 0u32;
                }
            }
        }
    }
    match current_block {
        16626164785316884790 => {
            status = (13u32) << 16i32;
            *minor_status = 12u32
        }
        _ => {}
    }
    tmpcred = ncred;
    crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut tmpcred);
    return status;
}
/* V2 KRB5_CALLCONV */
#[no_mangle]

pub unsafe extern "C" fn gss_add_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut desired_mech: crate::gssapi_h::gss_OID,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut initiator_time_req: crate::gssapi_h::OM_uint32,
    mut acceptor_time_req: crate::gssapi_h::OM_uint32,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut initiator_time_rec: *mut crate::gssapi_h::OM_uint32,
    mut acceptor_time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return gss_add_cred_from(
        minor_status,
        input_cred_handle,
        desired_name,
        desired_mech,
        cred_usage,
        initiator_time_req,
        acceptor_time_req,
        0 as crate::gssapi_ext_h::gss_const_key_value_set_t,
        output_cred_handle,
        actual_mechs,
        initiator_time_rec,
        acceptor_time_rec,
    );
}
/*
 * Copyright 2008 by the Massachusetts Institute of Technology.
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
/* __cplusplus */
/*
 * Solaris extensions
 */
/* *
 * Provides a platform-specific name for a GSSAPI name as interpreted by a
 * given mechanism.
 *
 * @param [out] minor      Minor status code
 * @param [in] name        The gss name resulting from accept_sec_context
 * @param [in] mech_type   The mechanism that will be asked to map @a name to a
 *                         local name
 * @param [out] localname  Caller-allocated buffer to be filled in with the
 *                         local name on success
 */
/* *
 * Determine whether a mechanism name is authorized to act as a username.
 *
 * @param [in] name      Mechanism name
 * @param [in] username  System username
 *
 * This is a simple wrapper around gss_authorize_localname().  It only supports
 * system usernames as local names, and cannot distinguish between lack of
 * authorization and other errors.
 *
 * @retval 1 @a name is authorized to act as @a username
 * @retval 0 @a name is not authorized or an error occurred
 */
/* *
 *  Determine whether a mechanism name is authorized to act as a local name.
 *
 * @param [out] minor  Minor status code
 * @param [in] name    Mechanism name
 * @param [in] user    Local name
 *
 * @a name is a mechanism name, typically the result of a completed
 * gss_accept_sec_context().  @a user is an internal name representing a local
 * name, such as a name imported by gss_import_name() with an @a
 * input_name_type of @c GSS_C_NT_USER_NAME.
 *
 * @return Return GSS_S_COMPLETE if @a name is authorized to act as @a user,
 * GSS_S_UNAUTHORIZED if not, or an appropriate GSS error code if an error
 * occurred.
 *
 * @sa gss_userok
 */
/* minor_status */
/* desired_name */
/* password */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* input_cred_handle */
/* desired_name */
/* desired_mech */
/* password */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/*
 * GGF extensions
 */
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*member_buffer*/
/*buffer_set*/
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*context_handle*/
/*desired_object*/
/*data_set*/
/*minor_status*/
/*cred_handle*/
/*desired_object*/
/*data_set*/
/*minor_status*/
/*cred_handle*/
/*desired_object*/
/*value*/
/*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
/* minor_status */
/* cred_handle */
/* token */
/* minor_status */
/* token */
/* cred_handle */
/*
 * Heimdal extension
 */
/*minor_status*/
/*cred*/
/*desired_object*/
/*value*/
/*
 * Call the given method on the given mechanism
 */
/*minor_status*/
/*desired_mech*/
/*desired_object*/
/*value*/
/*
 * AEAD extensions
 */
/*minor_status*/
/*context_handle*/
/*conf_req_flag*/
/*qop_req*/
/*input_assoc_buffer*/
/*input_payload_buffer*/
/*conf_state*/
/*output_message_buffer*/
/*minor_status*/
/*context_handle*/
/*input_message_buffer*/
/*input_assoc_buffer*/
/*output_payload_buffer*/
/*conf_state*/
/*qop_state*/
/*
 * SSPI extensions
 */
/*
 * Returns a buffer set with the first member containing the
 * session key for SSPI compatibility. The optional second
 * member contains an OID identifying the session key type.
 */
/* Packet data */
/* Mechanism header */
/* Mechanism specific parameters */
/* Mechanism trailer */
/* Padding */
/* Complete wrap token */
/* Sign only packet data */
/* MIC token destination */
/* indicates GSS should allocate */
/* indicates caller should free */
/*
 * Sign and optionally encrypt a sequence of buffers. The buffers
 * shall be ordered HEADER | DATA | PADDING | TRAILER. Suitable
 * space for the header, padding and trailer should be provided
 * by calling gss_wrap_iov_length(), or the ALLOCATE flag should
 * be set on those buffers.
 *
 * Encryption is in-place. SIGN_ONLY buffers are untouched. Only
 * a single PADDING buffer should be provided. The order of the
 * buffers in memory does not matter. Buffers in the IOV should
 * be arranged in the order above, and in the case of multiple
 * DATA buffers the sender and receiver should agree on the
 * order.
 *
 * With GSS_C_DCE_STYLE it is acceptable to not provide PADDING
 * and TRAILER, but the caller must guarantee the plaintext data
 * being encrypted is correctly padded, otherwise an error will
 * be returned.
 *
 * While applications that have knowledge of the underlying
 * cryptosystem may request a specific configuration of data
 * buffers, the only generally supported configurations are:
 *
 *  HEADER | DATA | PADDING | TRAILER
 *
 * which will emit GSS_Wrap() compatible tokens, and:
 *
 *  HEADER | SIGN_ONLY | DATA | PADDING | TRAILER
 *
 * for AEAD.
 *
 * The typical (special cased) usage for DCE is as follows:
 *
 *  SIGN_ONLY_1 | DATA | SIGN_ONLY_2 | HEADER
 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/*
 * Verify and optionally decrypt a sequence of buffers. To process
 * a GSS-API message without separate buffer, pass STREAM | DATA.
 * Upon return DATA will contain the decrypted or integrity
 * protected message. Only a single DATA buffer may be provided
 * with this usage. DATA by default will point into STREAM, but if
 * the ALLOCATE flag is set a copy will be returned.
 *
 * Otherwise, decryption is in-place. SIGN_ONLY buffers are
 * untouched.
 */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/*
 * Query HEADER, PADDING and TRAILER buffer lengths. DATA buffers
 * should be provided so the correct padding length can be determined.
 */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/*
 * Produce a GSSAPI MIC token for a sequence of buffers.  All SIGN_ONLY and
 * DATA buffers will be signed, in the order they appear.  One MIC_TOKEN buffer
 * must be included for the result.  Suitable space should be provided for the
 * MIC_TOKEN buffer by calling gss_get_mic_iov_length, or the ALLOCATE flag
 * should be set on that buffer.  If the ALLOCATE flag is used, use
 * gss_release_iov_buffer to free the allocated buffer within the iov list when
 * it is no longer needed.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/*
 * Query the MIC_TOKEN buffer length within the iov list.
 */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/*
 * Verify the MIC_TOKEN buffer within the iov list against the SIGN_ONLY and
 * DATA buffers in the order they appear.  Return values are the same as for
 * gss_verify_mic.
 */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/*
 * Release buffers that have the ALLOCATED flag set.
 */
/* minor_status */
/* iov */
/* iov_count */
/*
 * Protocol transition
 */
/* minor_status */
/* impersonator_cred_handle */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* input_cred_handle */
/* impersonator_cred_handle */
/* desired_name */
/* desired_mech */
/* cred_usage */
/* initiator_time_req */
/* acceptor_time_req */
/* output_cred_handle */
/* actual_mechs */
/* initiator_time_rec */
/* acceptor_time_rec */
/*
 * Naming extensions
 */
/* minor_status */
/* name */
/* display_as_name_type */
/* display_name */
/* minor_status */
/* name */
/* name_is_MN */
/* MN_mech */
/* attrs */
/* minor_status */
/* name */
/* attr */
/* authenticated */
/* complete */
/* value */
/* display_value */
/* more */
/* minor_status */
/* name */
/* complete */
/* attr */
/* value */
/* minor_status */
/* name */
/* attr */
/* minor_status */
/* name */
/* exp_composite_name */
/* minor_status */
/* name */
/* authenticated */
/* type_id */
/* output */
/* minor_status */
/* name */
/* type_id */
/* input */
/* draft-josefsson-gss-capsulate */
/* input_token */
/* token_oid */
/* output_token */
/* input_token */
/* token_oid */
/* output_token */
/* first_oid */
/* second_oid */
/* Credential store extensions */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* cred_store */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
#[no_mangle]

pub unsafe extern "C" fn gss_add_cred_from(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut input_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut desired_name: crate::gssapi_h::gss_name_t,
    mut desired_mech: crate::gssapi_h::gss_OID,
    mut cred_usage: crate::gssapi_h::gss_cred_usage_t,
    mut initiator_time_req: crate::gssapi_h::OM_uint32,
    mut acceptor_time_req: crate::gssapi_h::OM_uint32,
    mut cred_store: crate::gssapi_ext_h::gss_const_key_value_set_t,
    mut output_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
    mut actual_mechs: *mut crate::gssapi_h::gss_OID_set,
    mut initiator_time_rec: *mut crate::gssapi_h::OM_uint32,
    mut acceptor_time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut temp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut time_req: crate::gssapi_h::OM_uint32 = 0;
    let mut time_rec = 0u32;
    let mut time_recp = 0 as *mut crate::gssapi_h::OM_uint32;
    let mut union_name = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut union_cred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut internal_name = 0 as crate::gssapi_h::gss_name_t;
    let mut allocated_name = 0 as crate::gssapi_h::gss_name_t;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut cred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut tmpcred = 0 as *mut crate::mglueP_h::gss_cred_id_struct;
    let mut newptr = 0 as *mut libc::c_void;
    let mut oidbuf = 0 as *mut libc::c_void;
    let mut target_mechs = crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
    let mut selected_mech = 0 as crate::gssapi_h::gss_OID;
    status = val_add_cred_args(
        minor_status,
        input_cred_handle,
        desired_name,
        desired_mech,
        cred_usage,
        cred_store,
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
    } else {
        if (*mech).gss_acquire_cred.is_none() {
            return (16u32) << 16i32;
        }
    }
    if input_cred_handle.is_null() {
        /* Create a new credential handle. */
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
        (*union_cred).loopback = union_cred
    } else if output_cred_handle.is_null() {
        /* Add to the existing handle. */
        union_cred = input_cred_handle;
        if !crate::src::mechglue::g_glue::gssint_get_mechanism_cred(union_cred, selected_mech)
            .is_null()
        {
            return (17u32) << 16i32;
        }
    } else {
        /* Create a new credential handle with the mechanism credentials of the
         * input handle plus the acquired mechanism credential. */
        status = copy_union_cred(minor_status, input_cred_handle, &mut union_cred);
        if status != 0u32 {
            return status;
        }
    }
    /* We may need to create a mechanism specific name. */
    if !desired_name.is_null() {
        union_name = desired_name;
        if !(*union_name).mech_type.is_null()
            && ((*(*union_name).mech_type).length == (*selected_mech).length
                && crate::stdlib::memcmp(
                    (*(*union_name).mech_type).elements,
                    (*selected_mech).elements,
                    (*(*union_name).mech_type).length as usize,
                ) == 0i32)
        {
            internal_name = (*union_name).mech_name;
            current_block = 5529461102203738653;
        } else if crate::src::mechglue::g_glue::gssint_import_internal_name(
            minor_status,
            selected_mech,
            union_name,
            &mut allocated_name,
        ) != 0u32
        {
            status = (2u32) << 16i32;
            current_block = 1609792147738124217;
        } else {
            internal_name = allocated_name;
            current_block = 5529461102203738653;
        }
    } else {
        current_block = 5529461102203738653;
    }
    match current_block {
        5529461102203738653 => {
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
            target_mechs.count = 1usize;
            target_mechs.elements = crate::src::mechglue::g_initialize::gssint_get_public_oid(
                selected_mech as crate::gssapi_h::gss_const_OID,
            );
            if target_mechs.elements.is_null() {
                status = (13u32) << 16i32
            } else {
                if !initiator_time_rec.is_null() || !acceptor_time_rec.is_null() {
                    time_recp = &mut time_rec
                }
                if (*mech).gss_acquire_cred_from.is_some() {
                    status = (*mech)
                        .gss_acquire_cred_from
                        .expect("non-null function pointer")(
                        minor_status,
                        internal_name,
                        time_req,
                        &mut target_mechs,
                        cred_usage,
                        cred_store,
                        &mut cred,
                        0 as *mut crate::gssapi_h::gss_OID_set,
                        time_recp,
                    );
                    current_block = 5235537862154438448;
                } else if cred_store.is_null() {
                    status = (*mech).gss_acquire_cred.expect("non-null function pointer")(
                        minor_status,
                        internal_name,
                        time_req,
                        &mut target_mechs,
                        cred_usage,
                        &mut cred,
                        0 as *mut crate::gssapi_h::gss_OID_set,
                        time_recp,
                    );
                    current_block = 5235537862154438448;
                } else {
                    status = (16u32) << 16i32;
                    current_block = 1609792147738124217;
                }
                match current_block {
                    1609792147738124217 => {}
                    _ => {
                        if status != 0u32 {
                            *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                                *minor_status,
                                &mut (*mech).mech_type,
                            )
                        } else {
                            /* Extend the arrays in the union cred. */
                            newptr = crate::stdlib::realloc(
                                (*union_cred).mechs_array as *mut libc::c_void,
                                (((*union_cred).count + 1i32) as usize).wrapping_mul(
                                    ::std::mem::size_of::<crate::gssapi_h::gss_OID_desc>(),
                                ),
                            );
                            if newptr.is_null() {
                                status = (13u32) << 16i32
                            } else {
                                (*union_cred).mechs_array = newptr as crate::gssapi_h::gss_OID;
                                newptr = crate::stdlib::realloc(
                                    (*union_cred).cred_array as *mut libc::c_void,
                                    (((*union_cred).count + 1i32) as usize).wrapping_mul(
                                        ::std::mem::size_of::<crate::gssapi_h::gss_cred_id_t>(),
                                    ),
                                );
                                if newptr.is_null() {
                                    status = (13u32) << 16i32
                                } else {
                                    (*union_cred).cred_array =
                                        newptr as *mut crate::gssapi_h::gss_cred_id_t;
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
                                    oidbuf =
                                        crate::stdlib::malloc((*selected_mech).length as usize);
                                    if !oidbuf.is_null() {
                                        let ref mut fresh1 = (*(*union_cred)
                                            .mechs_array
                                            .offset((*union_cred).count as isize))
                                        .elements;
                                        *fresh1 = oidbuf;
                                        crate::stdlib::memcpy(
                                            (*(*union_cred)
                                                .mechs_array
                                                .offset((*union_cred).count as isize))
                                            .elements,
                                            (*selected_mech).elements,
                                            (*selected_mech).length as usize,
                                        );
                                        (*(*union_cred)
                                            .mechs_array
                                            .offset((*union_cred).count as isize))
                                        .length = (*selected_mech).length;
                                        if !actual_mechs.is_null() {
                                            status =
                                                crate::src::mechglue::g_initialize::gssint_make_public_oid_set(minor_status,
                                                                           (*union_cred).mechs_array,
                                                                           (*union_cred).count
                                                                               +
                                                                               1i32,
                                                                           actual_mechs);
                                            if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
                                                != 0
                                            {
                                                current_block = 1609792147738124217;
                                            } else {
                                                current_block = 14294131666767243020;
                                            }
                                        } else {
                                            current_block = 14294131666767243020;
                                        }
                                        match current_block {
                                            1609792147738124217 => {}
                                            _ => {
                                                let ref mut fresh2 = *(*union_cred)
                                                    .cred_array
                                                    .offset((*union_cred).count as isize);
                                                *fresh2 = cred;
                                                (*union_cred).count += 1;
                                                if !output_cred_handle.is_null() {
                                                    *output_cred_handle = union_cred
                                                }
                                                /* We're done with the internal name. Free it if we allocated it. */
                                                if !allocated_name.is_null() {
                                                    crate::src::mechglue::g_glue::gssint_release_internal_name(&mut temp_minor_status,
                                                                                 selected_mech,
                                                                                 &mut allocated_name);
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
        }
        _ => {}
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
    if !output_cred_handle.is_null() && !union_cred.is_null() {
        tmpcred = union_cred;
        crate::src::mechglue::g_rel_cred::gss_release_cred(&mut temp_minor_status, &mut tmpcred);
    }
    crate::stdlib::free(oidbuf);
    return status;
}
