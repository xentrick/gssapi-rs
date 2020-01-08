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

pub use crate::src::mechglue::g_initialize::gss_indicate_mechs;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_oid_ops::gss_add_oid_set_member;
pub use crate::src::mechglue::g_rel_oid_set::gss_release_oid_set;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/mechglue/g_mechattr.c */
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

unsafe extern "C" fn testMechAttr(
    mut attr: crate::gssapi_h::gss_const_OID,
    mut against: crate::gssapi_h::gss_const_OID_set,
) -> i32 {
    let mut present = 0i32;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    if crate::src::generic::oid_ops::generic_gss_test_oid_set_member(
        &mut minor,
        attr,
        against as crate::gssapi_h::gss_OID_set,
        &mut present,
    ) & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
        != 0
    {
        return 0i32;
    }
    return present;
}
/*
 * Return TRUE iff all the elements of desired and none of the elements
 * of except exist in available.
 */

unsafe extern "C" fn testMechAttrsOffered(
    mut desired: crate::gssapi_h::gss_const_OID_set,
    mut except: crate::gssapi_h::gss_const_OID_set,
    mut available: crate::gssapi_h::gss_const_OID_set,
) -> i32 {
    let mut i: crate::stddef_h::size_t = 0;
    if !desired.is_null() {
        i = 0usize;
        while i < (*desired).count {
            if testMechAttr(
                &mut *(*desired).elements.offset(i as isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct
                    as crate::gssapi_h::gss_const_OID,
                available,
            ) == 0
            {
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    }
    if !except.is_null() {
        i = 0usize;
        while i < (*except).count {
            if testMechAttr(
                &mut *(*except).elements.offset(i as isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct
                    as crate::gssapi_h::gss_const_OID,
                available,
            ) != 0
            {
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    }
    return 1i32;
}
/*
 * Return TRUE iff all the elements of critical exist in known.
 */

unsafe extern "C" fn testMechAttrsKnown(
    mut critical: crate::gssapi_h::gss_const_OID_set,
    mut known: crate::gssapi_h::gss_const_OID_set,
) -> i32 {
    let mut i: crate::stddef_h::size_t = 0;
    if !critical.is_null() {
        i = 0usize;
        while i < (*critical).count {
            if testMechAttr(
                &mut *(*critical).elements.offset(i as isize)
                    as *mut crate::gssapi_h::gss_OID_desc_struct
                    as crate::gssapi_h::gss_const_OID,
                known,
            ) == 0
            {
                return 0i32;
            }
            i = i.wrapping_add(1)
        }
    }
    return 1i32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_indicate_mechs_by_attrs(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut desired_mech_attrs: crate::gssapi_h::gss_const_OID_set,
    mut except_mech_attrs: crate::gssapi_h::gss_const_OID_set,
    mut critical_mech_attrs: crate::gssapi_h::gss_const_OID_set,
    mut mechs: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut allMechs = 0 as crate::gssapi_h::gss_OID_set;
    let mut i: crate::stddef_h::size_t = 0;
    if !minor.is_null() {
        *minor = 0u32
    }
    if !mechs.is_null() {
        *mechs = 0 as crate::gssapi_h::gss_OID_set
    }
    if minor.is_null() || mechs.is_null() {
        return (2u32) << 24i32;
    }
    status = crate::src::mechglue::g_initialize::gss_indicate_mechs(minor, &mut allMechs);
    if !(status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
        status = crate::src::generic::oid_ops::generic_gss_create_empty_oid_set(minor, mechs);
        if !(status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
            i = 0usize;
            loop {
                if !(i < (*allMechs).count) {
                    current_block = 10043043949733653460;
                    break;
                }
                let mut supportedAttrs = 0 as crate::gssapi_h::gss_OID_set;
                let mut knownAttrs = 0 as crate::gssapi_h::gss_OID_set;
                status = gss_inquire_attrs_for_mech(
                    minor,
                    &mut *(*allMechs).elements.offset(i as isize)
                        as *mut crate::gssapi_h::gss_OID_desc_struct
                        as crate::gssapi_h::gss_const_OID,
                    &mut supportedAttrs,
                    &mut knownAttrs,
                );
                if !(status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                    if testMechAttrsOffered(
                        desired_mech_attrs,
                        except_mech_attrs,
                        supportedAttrs as crate::gssapi_h::gss_const_OID_set,
                    ) != 0
                        && testMechAttrsKnown(
                            critical_mech_attrs,
                            knownAttrs as crate::gssapi_h::gss_const_OID_set,
                        ) != 0
                    {
                        status = crate::src::mechglue::g_oid_ops::gss_add_oid_set_member(
                            minor,
                            &mut *(*allMechs).elements.offset(i as isize),
                            mechs,
                        );
                        if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
                                &mut tmpMinor,
                                &mut supportedAttrs,
                            );
                            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
                                &mut tmpMinor,
                                &mut knownAttrs,
                            );
                            current_block = 11383350759426688886;
                            break;
                        }
                    }
                    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
                        &mut tmpMinor,
                        &mut supportedAttrs,
                    );
                    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(
                        &mut tmpMinor,
                        &mut knownAttrs,
                    );
                }
                i = i.wrapping_add(1)
            }
            match current_block {
                11383350759426688886 => {}
                _ => {
                    *minor = 0u32;
                    status = 0u32
                }
            }
        }
    }
    crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpMinor, &mut allMechs);
    return status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_inquire_attrs_for_mech(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut mech_oid: crate::gssapi_h::gss_const_OID,
    mut mech_attrs: *mut crate::gssapi_h::gss_OID_set,
    mut known_mech_attrs: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut selected_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    if !minor.is_null() {
        *minor = 0u32
    }
    if !mech_attrs.is_null() {
        *mech_attrs = 0 as crate::gssapi_h::gss_OID_set
    }
    if !known_mech_attrs.is_null() {
        *known_mech_attrs = 0 as crate::gssapi_h::gss_OID_set
    }
    if minor.is_null() {
        return (2u32) << 24i32;
    }
    status = crate::src::mechglue::g_initialize::gssint_select_mech_type(
        minor,
        mech_oid,
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
    /* If the mech does not implement RFC 5587, return success with an empty
     * mech_attrs and known_mech_attrs. */
    if (*mech).gss_inquire_attrs_for_mech.is_none() {
        return 0u32;
    }
    public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
        selected_mech as crate::gssapi_h::gss_const_OID,
    );
    status = (*mech)
        .gss_inquire_attrs_for_mech
        .expect("non-null function pointer")(
        minor,
        public_mech as crate::gssapi_h::gss_const_OID,
        mech_attrs,
        known_mech_attrs,
    );
    if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        *minor =
            crate::src::generic::util_errmap::gssint_mecherrmap_map(*minor, &mut (*mech).mech_type);
        return status;
    }
    if !known_mech_attrs.is_null() && (*known_mech_attrs).is_null() {
        status = crate::src::generic::oid_ops::generic_gss_copy_oid_set(
            minor,
            crate::src::generic::gssapi_generic::gss_ma_known_attrs
                as *const crate::gssapi_h::gss_OID_set_desc,
            known_mech_attrs,
        );
        if status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            crate::src::mechglue::g_rel_oid_set::gss_release_oid_set(&mut tmpMinor, mech_attrs);
            if !mech_attrs.is_null() {
                *mech_attrs = 0 as crate::gssapi_h::gss_OID_set
            }
        }
    }
    return 0u32;
}
/* minor_status */
/* desired_mech_attrs */
/* except_mech_attrs */
/* critical_mech_attrs */
/* mechs */
/* minor_status */
/* mech */
/* mech_attrs */
/* known_mech_attrs */
#[no_mangle]

pub unsafe extern "C" fn gss_display_mech_attr(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut mech_attr: crate::gssapi_h::gss_const_OID,
    mut name: crate::gssapi_h::gss_buffer_t,
    mut short_desc: crate::gssapi_h::gss_buffer_t,
    mut long_desc: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    return crate::src::generic::gssapi_generic::generic_gss_display_mech_attr(
        minor, mech_attr, name, short_desc, long_desc,
    );
}
