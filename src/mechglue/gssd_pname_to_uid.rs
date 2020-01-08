use ::libc;

/* _GSSAPIP_GENERIC_H_ */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__gid_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uid_t;
pub use crate::stdlib::__uint32_t;
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

pub use crate::src::mechglue::g_glue::gssint_import_internal_name;
pub use crate::src::mechglue::g_glue::gssint_release_internal_name;
pub use crate::src::mechglue::g_initialize::gssint_get_mechanism;
pub use crate::src::mechglue::g_initialize::gssint_get_public_oid;
pub use crate::src::mechglue::g_initialize::gssint_select_mech_type;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;

pub use crate::stdlib::getpwnam_r;

pub use crate::stdlib::passwd;
pub use crate::stdlib::ssize_t;
pub use crate::stdlib::uid_t;
/* #pragma ident	"@(#)gssd_pname_to_uid.c	1.18	04/02/23 SMI" */
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
 *  glue routines that test the mech id either passed in to
 *  gss_init_sec_contex() or gss_accept_sec_context() or within the glue
 *  routine supported version of the security context and then call
 *  the appropriate underlying mechanism library procedure.
 *
 */

unsafe extern "C" fn attr_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mech: crate::mglueP_h::gss_mechanism,
    mech_name: crate::gssapi_h::gss_name_t,
    mut localname: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major = (16u32) << 16i32;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut more = -(1i32);
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
    value.value = 0 as *mut libc::c_void;
    display_value.value = 0 as *mut libc::c_void;
    if (*mech).gss_get_name_attribute.is_none() {
        return (16u32) << 16i32;
    }
    major = (*mech)
        .gss_get_name_attribute
        .expect("non-null function pointer")(
        minor,
        mech_name,
        crate::src::generic::gssapi_generic::GSS_C_ATTR_LOCAL_LOGIN_USER,
        &mut authenticated,
        &mut complete,
        &mut value,
        &mut display_value,
        &mut more,
    );
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        *minor =
            crate::src::generic::util_errmap::gssint_mecherrmap_map(*minor, &mut (*mech).mech_type)
    } else if authenticated == 0 {
        major = (16u32) << 16i32
    } else {
        (*localname).value = value.value;
        (*localname).length = value.length;
        value.value = 0 as *mut libc::c_void
    }
    if !display_value.value.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpMinor, &mut display_value);
    }
    if !value.value.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpMinor, &mut value);
    }
    return major;
}
#[no_mangle]

pub unsafe extern "C" fn gss_localname(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    pname: crate::gssapi_h::gss_name_t,
    mut mech_type: crate::gssapi_h::gss_const_OID,
    mut localname: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpMinor: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::mglueP_h::gss_config;
    let mut unionName = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut mechName = 0 as crate::gssapi_h::gss_name_t;
    let mut mechNameP = 0 as *mut crate::mglueP_h::gss_name_struct;
    let mut selected_mech = 0 as crate::gssapi_h::gss_OID;
    let mut public_mech = 0 as *mut crate::gssapi_h::gss_OID_desc_struct;
    if !localname.is_null() {
        (*localname).length = 0usize;
        (*localname).value = 0 as *mut libc::c_void
    }
    if minor.is_null() {
        return (2u32) << 24i32;
    }
    *minor = 0u32;
    if pname.is_null() {
        return (1u32) << 24i32;
    }
    if localname.is_null() {
        return (2u32) << 24i32;
    }
    unionName = pname;
    if !mech_type.is_null() {
        major = crate::src::mechglue::g_initialize::gssint_select_mech_type(
            minor,
            mech_type,
            &mut selected_mech,
        );
        if major != 0u32 {
            return major;
        }
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            selected_mech as crate::gssapi_h::gss_const_OID,
        )
    } else {
        mech = crate::src::mechglue::g_initialize::gssint_get_mechanism(
            (*unionName).mech_type as crate::gssapi_h::gss_const_OID,
        )
    }
    if mech.is_null() {
        return (1u32) << 16i32;
    }
    /* may need to create a mechanism specific name */
    if (*unionName).mech_type.is_null()
        || !(*unionName).mech_type.is_null()
            && !((*(*unionName).mech_type).length == (*mech).mech_type.length
                && crate::stdlib::memcmp(
                    (*(*unionName).mech_type).elements,
                    (*mech).mech_type.elements,
                    (*(*unionName).mech_type).length as usize,
                ) == 0i32)
    {
        major = crate::src::mechglue::g_glue::gssint_import_internal_name(
            minor,
            &mut (*mech).mech_type,
            unionName,
            &mut mechName,
        );
        if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            return major;
        }
        mechNameP = mechName
    } else {
        mechNameP = (*unionName).mech_name
    }
    major = (16u32) << 16i32;
    if (*mech).gss_localname.is_some() {
        public_mech = crate::src::mechglue::g_initialize::gssint_get_public_oid(
            selected_mech as crate::gssapi_h::gss_const_OID,
        );
        major = (*mech).gss_localname.expect("non-null function pointer")(
            minor,
            mechNameP,
            public_mech as crate::gssapi_h::gss_const_OID,
            localname,
        );
        if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            *minor = crate::src::generic::util_errmap::gssint_mecherrmap_map(
                *minor,
                &mut (*mech).mech_type,
            )
        }
    }
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        major = attr_localname(minor, mech, mechNameP, localname)
    }
    if !mechName.is_null() {
        crate::src::mechglue::g_glue::gssint_release_internal_name(
            &mut tmpMinor,
            &mut (*mech).mech_type,
            &mut mechName,
        );
    }
    return major;
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
#[no_mangle]

pub unsafe extern "C" fn gss_pname_to_uid(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    name: crate::gssapi_h::gss_name_t,
    mech_type: crate::gssapi_h::gss_OID,
    mut uidOut: *mut crate::stdlib::uid_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major = (16u32) << 16i32;
    let mut tmpminor: crate::gssapi_h::OM_uint32 = 0;
    let mut localname = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut pwbuf: [i8; 8192] = [0; 8192];
    let mut localuser = 0 as *mut i8;
    let mut pwd = 0 as *mut crate::stdlib::passwd;
    let mut pw = crate::stdlib::passwd {
        pw_name: 0 as *mut i8,
        pw_passwd: 0 as *mut i8,
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: 0 as *mut i8,
        pw_dir: 0 as *mut i8,
        pw_shell: 0 as *mut i8,
    };
    let mut code = 0i32;
    localname.value = 0 as *mut libc::c_void;
    major = gss_localname(
        minor,
        name,
        mech_type as crate::gssapi_h::gss_const_OID,
        &mut localname,
    );
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) == 0 && !localname.value.is_null() {
        localuser = crate::stdlib::malloc(localname.length.wrapping_add(1usize)) as *mut i8;
        if localuser.is_null() {
            code = 12i32
        }
        if code == 0i32 {
            crate::stdlib::memcpy(
                localuser as *mut libc::c_void,
                localname.value,
                localname.length,
            );
            *localuser.offset(localname.length as isize) = '\u{0}' as i8;
            code = if crate::stdlib::getpwnam_r(
                localuser,
                &mut pw,
                pwbuf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 8192]>(),
                &mut pwd,
            ) == 0i32
            {
                if pwd.is_null() {
                    -(1i32)
                } else {
                    0i32
                }
            } else {
                -(1i32)
            }
        }
        if code == 0i32 && !pwd.is_null() {
            *uidOut = (*pwd).pw_uid
        } else {
            major = (13u32) << 16i32
        }
    }
    crate::stdlib::free(localuser as *mut libc::c_void);
    if !localname.value.is_null() {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpminor, &mut localname);
    }
    /*NO_PASSWORD*/
    return major;
}
/*_WIN32*/
