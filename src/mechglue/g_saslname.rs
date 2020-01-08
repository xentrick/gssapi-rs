use ::libc;

/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::int32_t;
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
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::krb5_h::_krb5_crypto_iov;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_crypto_iov;
pub use crate::krb5_h::krb5_cryptotype;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_make_checksum_iov;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_magic;
pub use crate::mglueP_h::gss_config;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_mechanism;
pub use crate::mglueP_h::gss_name_struct;

pub use crate::src::mechglue::g_initialize::gss_indicate_mechs;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/mechglue/g_saslname.c */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
 * All rights reserved.
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

static mut basis_32: [i8; 33] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 50, 51, 52, 53, 54, 55, 0,
];

unsafe extern "C" fn oidToSaslName(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mech: crate::gssapi_h::gss_OID,
    mut sasl_name: *mut i8,
) -> crate::gssapi_h::OM_uint32 {
    let mut derBuf: [u8; 2] = [0; 2];
    let mut iov: [crate::krb5_h::krb5_crypto_iov; 3] = [crate::krb5_h::krb5_crypto_iov {
        flags: 0,
        data: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
    }; 3];
    let mut cksumBuf: [u8; 20] = [0; 20];
    let mut q = cksumBuf.as_mut_ptr();
    let mut p = sasl_name;
    if (*mech).length > 127u32 {
        *minor = 34u32;
        return (1u32) << 16i32;
    }
    derBuf[0usize] = 0x6u8;
    derBuf[1usize] = (*mech).length as u8;
    iov[0usize].flags = 3i32;
    iov[0usize].data.length = 2u32;
    iov[0usize].data.data = derBuf.as_mut_ptr() as *mut i8;
    iov[1usize].flags = 3i32;
    iov[1usize].data.length = (*mech).length;
    iov[1usize].data.data = (*mech).elements as *mut i8;
    iov[2usize].flags = 6i32;
    iov[2usize].data.length = ::std::mem::size_of::<[u8; 20]>() as u32;
    iov[2usize].data.data = cksumBuf.as_mut_ptr() as *mut i8;
    *minor = crate::krb5_h::krb5_k_make_checksum_iov(
        0 as crate::krb5_h::krb5_context,
        0x9i32,
        0 as crate::krb5_h::krb5_key,
        0i32,
        iov.as_mut_ptr(),
        3usize,
    ) as crate::gssapi_h::OM_uint32;
    if *minor != 0u32 {
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy(
        p as *mut libc::c_void,
        b"GS2-\x00" as *const u8 as *const libc::c_void,
        4usize,
    );
    p = p.offset(4isize);
    let fresh0 = p;
    p = p.offset(1);
    *fresh0 = basis_32[(*q.offset(0isize) as i32 >> 3i32) as usize];
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = basis_32
        [((*q.offset(0isize) as i32 & 7i32) << 2i32 | *q.offset(1isize) as i32 >> 6i32) as usize];
    let fresh2 = p;
    p = p.offset(1);
    *fresh2 = basis_32[((*q.offset(1isize) as i32 & 0x3fi32) >> 1i32) as usize];
    let fresh3 = p;
    p = p.offset(1);
    *fresh3 = basis_32
        [((*q.offset(1isize) as i32 & 1i32) << 4i32 | *q.offset(2isize) as i32 >> 4i32) as usize];
    let fresh4 = p;
    p = p.offset(1);
    *fresh4 = basis_32
        [((*q.offset(2isize) as i32 & 0xfi32) << 1i32 | *q.offset(3isize) as i32 >> 7i32) as usize];
    let fresh5 = p;
    p = p.offset(1);
    *fresh5 = basis_32[((*q.offset(3isize) as i32 & 0x7fi32) >> 2i32) as usize];
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = basis_32
        [((*q.offset(3isize) as i32 & 3i32) << 3i32 | *q.offset(4isize) as i32 >> 5i32) as usize];
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = basis_32[(*q.offset(4isize) as i32 & 0x1fi32) as usize];
    let fresh8 = p;
    p = p.offset(1);
    *fresh8 = basis_32[(*q.offset(5isize) as i32 >> 3i32) as usize];
    let fresh9 = p;
    p = p.offset(1);
    *fresh9 = basis_32
        [((*q.offset(5isize) as i32 & 7i32) << 2i32 | *q.offset(6isize) as i32 >> 6i32) as usize];
    let fresh10 = p;
    p = p.offset(1);
    *fresh10 = basis_32[((*q.offset(6isize) as i32 & 0x3fi32) >> 1i32) as usize];
    let fresh11 = p;
    p = p.offset(1);
    *fresh11 = '\u{0}' as i8;
    *minor = 0u32;
    return 0u32;
}

unsafe extern "C" fn oidToSaslNameAlloc(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mech: crate::gssapi_h::gss_OID,
    mut sasl_name: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    (*sasl_name).value = crate::stdlib::malloc(
        (::std::mem::size_of::<[i8; 16]>())
            .wrapping_sub(1usize)
            .wrapping_add(1usize),
    );
    if (*sasl_name).value.is_null() {
        *minor = 12u32;
        return (13u32) << 16i32;
    }
    (*sasl_name).length = (::std::mem::size_of::<[i8; 16]>()).wrapping_sub(1usize);
    status = oidToSaslName(minor, mech, (*sasl_name).value as *mut i8);
    if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpMinor, sasl_name);
        return status;
    }
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_saslname_for_mech(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    desired_mech: crate::gssapi_h::gss_OID,
    mut sasl_mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_description: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    *minor_status = 0u32;
    if !sasl_mech_name.is_null() {
        (*sasl_mech_name).length = 0usize;
        (*sasl_mech_name).value = 0 as *mut libc::c_void
    }
    if !mech_name.is_null() {
        (*mech_name).length = 0usize;
        (*mech_name).value = 0 as *mut libc::c_void
    }
    if !mech_description.is_null() {
        (*mech_description).length = 0usize;
        (*mech_description).value = 0 as *mut libc::c_void
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
        desired_mech as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (1u32) << 16i32;
    } else {
        if (*mech).gss_inquire_saslname_for_mech.is_none() {
            status = (16u32) << 16i32
        } else {
            public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
                selected_mech as crate::gssapi_h::gss_const_OID,
            );
            status = (*mech)
                .gss_inquire_saslname_for_mech
                .expect("non-null function pointer")(
                minor_status,
                public_mech,
                sasl_mech_name,
                mech_name,
                mech_description,
            );
            if status != 0u32 {
                *minor_status = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                    *minor_status,
                    &mut (*mech).mech_type,
                )
            }
        }
    }
    if status == (16u32) << 16i32 {
        if !sasl_mech_name.is_null() {
            status = oidToSaslNameAlloc(minor_status, desired_mech, sasl_mech_name)
        } else {
            status = 0u32
        }
    }
    return status;
}
/*
 * RFC 5801
 */
/* minor_status */
/* desired_mech */
/* sasl_mech_name */
/* mech_name */
/* mech_description */
/* We cannot interpose this function as mech_type is an output parameter. */
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_mech_for_saslname(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    sasl_mech_name: crate::gssapi_h::gss_buffer_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut mechSet = 0 as crate::gssapi_h::gss_OID_set;
    let mut i: crate::stddef_h::size_t = 0;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !mech_type.is_null() {
        *mech_type = 0 as crate::gssapi_h::gss_OID
    }
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    status = crate::src::mechglue::g_initialize::gss_indicate_mechs(minor_status, &mut mechSet);
    if status != 0u32 {
        return status;
    }
    i = 0usize;
    status = (1u32) << 16i32;
    while i < (*mechSet).count {
        let mut mech = 0 as *mut crate::mglueP_h::gss_config;
        let mut mappedName: [i8; 16] = [0; 16];
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            &mut *(*mechSet).elements.offset(i as isize)
                as *mut crate::gssapi_h::gss_OID_desc_struct
                as crate::gssapi_h::gss_const_OID,
        );
        if !mech.is_null() && (*mech).gss_inquire_mech_for_saslname.is_some() {
            status = (*mech)
                .gss_inquire_mech_for_saslname
                .expect("non-null function pointer")(
                minor_status, sasl_mech_name, mech_type
            );
            if status == 0u32 {
                break;
            }
        }
        if status == (1u32) << 16i32
            && (*sasl_mech_name).length == (::std::mem::size_of::<[i8; 16]>()).wrapping_sub(1usize)
            && oidToSaslName(
                &mut tmpMinor,
                &mut *(*mechSet).elements.offset(i as isize),
                mappedName.as_mut_ptr(),
            ) == 0u32
            && crate::stdlib::memcmp(
                (*sasl_mech_name).value,
                mappedName.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<[i8; 16]>()).wrapping_sub(1usize),
            ) == 0i32
        {
            if !mech_type.is_null() {
                *mech_type = &mut (*mech).mech_type
            }
            status = 0u32;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpMinor, &mut mechSet);
    return status;
}
