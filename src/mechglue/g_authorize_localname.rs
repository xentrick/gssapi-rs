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
pub use crate::mglueP_h::gss_union_name_t;
pub use crate::src::generic::gssapi_generic::GSS_C_ATTR_LOCAL_LOGIN_USER;
pub use crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME;

pub use crate::src::mechglue::g_canon_name::gss_canonicalize_name;
pub use crate::src::mechglue::g_compare_name::gss_compare_name;
pub use crate::src::mechglue::g_get_name_attr::gss_get_name_attribute;
pub use crate::src::mechglue::g_imp_name::gss_import_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_name::gss_release_name;

/*
 * Copyright (c) 2011, PADL Software Pty Ltd.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of PADL Software nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY PADL SOFTWARE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL PADL SOFTWARE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* #pragma ident	"@(#)g_userok.c	1.1	04/03/25 SMI" */

unsafe extern "C" fn mech_authorize_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    unionName: crate::mglueP_h::gss_union_name_t,
    unionUser: crate::mglueP_h::gss_union_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major = (16u32) << 16i32;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    if (*unionName).mech_type.is_null() {
        return (18u32) << 16i32;
    }
    mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
        (*unionName).mech_type as crate::gssapi_h::gss_const_OID,
    );
    if mech.is_null() {
        return (16u32) << 16i32;
    }
    if (*mech).gssspi_authorize_localname.is_some() {
        major = (*mech)
            .gssspi_authorize_localname
            .expect("non-null function pointer")(
            minor,
            (*unionName).mech_name,
            (*unionUser).external_name as crate::gssapi_h::gss_const_buffer_t,
            (*unionUser).name_type as crate::gssapi_h::gss_const_OID,
        );
        if major != 0u32 {
            *minor = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor,
                &mut (*mech).mech_type,
            )
        }
    }
    return major;
}
/*
 * Naming extensions based local login authorization.
 */

unsafe extern "C" fn attr_authorize_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    name: crate::gssapi_h::gss_name_t,
    unionUser: crate::mglueP_h::gss_union_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major = (16u32) << 16i32; /* attribute not present */
    let mut externalName = 0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
    let mut more = -(1i32);
    if !(*unionUser).name_type.is_null()
        && !((*(*unionUser).name_type).length
            == (*crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME).length
            && crate::stdlib::memcmp(
                (*(*unionUser).name_type).elements,
                (*crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME).elements,
                (*(*unionUser).name_type).length as usize,
            ) == 0i32)
    {
        return (3u32) << 16i32;
    }
    externalName = (*unionUser).external_name;
    if !externalName.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"externalName != GSS_C_NO_BUFFER\x00" as *const u8 as
                          *const i8,
                      b"g_authorize_localname.c\x00" as *const u8 as
                          *const i8,
                      92u32,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[i8; 90]>(b"OM_uint32 attr_authorize_localname(OM_uint32 *, const gss_name_t, const gss_union_name_t)\x00")).as_ptr());
    }
    while more != 0i32 && major != 0u32 {
        let mut tmpMajor: crate::gssapi_h::OM_uint32 = 0;
        let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
        let mut value = crate::gssapi_h::gss_buffer_desc {
            length: 0,
            value: 0 as *mut libc::c_void,
        };
        let mut display_value = crate::gssapi_h::gss_buffer_desc {
            length: 0,
            value: 0 as *mut libc::c_void,
        };
        let mut authenticated = 0i32;
        let mut complete = 0i32;
        tmpMajor = crate::src::mechglue::g_get_name_attr::gss_get_name_attribute(
            minor,
            name,
            crate::src::generic::gssapi_generic::GSS_C_ATTR_LOCAL_LOGIN_USER,
            &mut authenticated,
            &mut complete,
            &mut value,
            &mut display_value,
            &mut more,
        );
        if tmpMajor & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            major = tmpMajor;
            break;
        } else {
            if authenticated != 0
                && value.length == (*externalName).length
                && crate::stdlib::memcmp(value.value, (*externalName).value, (*externalName).length)
                    == 0i32
            {
                major = 0u32
            } else {
                major = (15u32) << 16i32
            }
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpMinor, &mut value);
            crate::src::mechglue::g_rel_buffer::gss_release_buffer(
                &mut tmpMinor,
                &mut display_value,
            );
        }
    }
    return major;
}
/*
 * Equality based local login authorization.
 */

unsafe extern "C" fn compare_names_authorize_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    unionName: crate::mglueP_h::gss_union_name_t,
    user: crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut canonName = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut match_0 = 0i32;
    status = crate::src::mechglue::g_canon_name::gss_canonicalize_name(
        minor,
        user,
        (*unionName).mech_type,
        &mut canonName,
    );
    if status != 0u32 {
        return status;
    }
    status = crate::src::mechglue::g_compare_name::gss_compare_name(
        minor,
        unionName,
        canonName,
        &mut match_0,
    );
    if status == 0u32 && match_0 == 0i32 {
        status = (15u32) << 16i32
    }
    crate::src::mechglue::g_rel_name::gss_release_name(&mut tmpMinor, &mut canonName);
    return status;
}
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
#[no_mangle]

pub unsafe extern "C" fn gss_authorize_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    name: crate::gssapi_h::gss_name_t,
    user: crate::gssapi_h::gss_name_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut unionName = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut unionUser = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut mechAvailable = 0i32;
    if minor.is_null() {
        return (2u32) << 24i32;
    }
    *minor = 0u32;
    if name.is_null() || user.is_null() {
        return (1u32) << 24i32;
    }
    unionName = name;
    unionUser = user;
    if !(*unionUser).mech_type.is_null() {
        return (2u32) << 16i32;
    }
    /* If mech returns yes, we return yes */
    major = mech_authorize_localname(minor, unionName, unionUser);
    if major == 0u32 {
        return 0u32;
    } else {
        if major != (16u32) << 16i32 {
            mechAvailable = 1i32
        }
    }
    /* If attribute exists, we evaluate attribute */
    major = attr_authorize_localname(minor, unionName, unionUser);
    if major == 0u32 || major == (15u32) << 16i32 {
        return major;
    }
    /* If mech did not implement SPI, compare the local name */
    if mechAvailable == 0i32 && !(*unionName).mech_type.is_null() {
        major = compare_names_authorize_localname(minor, unionName, unionUser)
    }
    return major;
}
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
#[no_mangle]

pub unsafe extern "C" fn gss_userok(name: crate::gssapi_h::gss_name_t, mut user: *const i8) -> i32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut userBuf = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut userName = 0 as *mut crate::mglueP_h::gss_name_struct;
    userBuf.value = user as *mut libc::c_void;
    userBuf.length = crate::stdlib::strlen(user);
    major = crate::src::mechglue::g_imp_name::gss_import_name(
        &mut minor,
        &mut userBuf,
        crate::src::generic::gssapi_generic::GSS_C_NT_USER_NAME,
        &mut userName,
    );
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return 0i32;
    }
    major = gss_authorize_localname(&mut minor, name, userName);
    crate::src::mechglue::g_rel_name::gss_release_name(&mut minor, &mut userName);
    return (major == 0u32) as i32;
}
