use ::libc;

/* _GSSAPI_KRB5_H_ */

/* __cplusplus */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_kt;
pub use crate::k5_int_h::_krb5_kt_ops;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::krb5_h::_krb5_ccache;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::profile_h::profile_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;

pub use crate::gssapiP_krb5_h::krb5_gss_ccache_name_req;
pub use crate::gssapiP_krb5_h::krb5_gss_import_cred_req;
pub use crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;

pub use crate::src::mechglue::g_buffer_set::gss_release_buffer_set;
pub use crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context;
pub use crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid;
pub use crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke;
pub use crate::src::mechglue::g_set_cred_option::gss_set_cred_option;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * Copyright (c) 2006-2008, Novell, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *   * The copyright holder's name is not used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * $Id$
 */
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_get_tkt_flags(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut ticket_flags: *mut crate::krb5_h::krb5_flags,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x01\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    if ticket_flags.is_null() {
        return (2u32) << 24i32;
    }
    major_status = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        minor_status,
        context_handle,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut data_set,
    );
    if major_status != 0u32 {
        return major_status;
    }
    if data_set.is_null()
        || (*data_set).count != 1usize
        || (*(*data_set).elements.offset(0isize)).length
            != ::std::mem::size_of::<crate::krb5_h::krb5_flags>()
    {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    *ticket_flags =
        *((*(*data_set).elements.offset(0isize)).value as *mut crate::krb5_h::krb5_flags);
    crate::src::mechglue::g_buffer_set::gss_release_buffer_set(minor_status, &mut data_set);
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_copy_ccache(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut out_ccache: crate::krb5_h::krb5_ccache,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x02\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    if out_ccache.is_null() {
        return (2u32) << 24i32;
    }
    req_buffer.value = out_ccache as *mut libc::c_void;
    req_buffer.length = ::std::mem::size_of::<crate::krb5_h::krb5_ccache>();
    major_status = crate::src::mechglue::g_set_cred_option::gss_set_cred_option(
        minor_status,
        &mut cred_handle,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_import_cred(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut id: crate::krb5_h::krb5_ccache,
    mut keytab_principal: crate::krb5_h::krb5_principal,
    mut keytab: crate::krb5_h::krb5_keytab,
    mut cred: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\r\x00" as *const u8 as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req = crate::gssapiP_krb5_h::krb5_gss_import_cred_req {
        id: 0 as *mut crate::krb5_h::_krb5_ccache,
        keytab_principal: 0 as *mut crate::krb5_h::krb5_principal_data,
        keytab: 0 as *mut crate::k5_int_h::_krb5_kt,
    };
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    if cred.is_null() {
        return (2u32) << 24i32;
    }
    *cred = 0 as crate::gssapi_h::gss_cred_id_t;
    req.id = id;
    req.keytab_principal = keytab_principal;
    req.keytab = keytab;
    req_buffer.value =
        &mut req as *mut crate::gssapiP_krb5_h::krb5_gss_import_cred_req as *mut libc::c_void;
    req_buffer.length = ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_import_cred_req>();
    major_status = crate::src::mechglue::g_set_cred_option::gss_set_cred_option(
        minor_status,
        cred,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_export_lucid_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut version: crate::gssapi_h::OM_uint32,
    mut kctx: *mut *mut libc::c_void,
) -> crate::gssapi_h::OM_uint32 {
    let mut oid_buf: [u8; 17] = [0; 17];
    let mut req_oid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    if kctx.is_null() {
        return (2u32) << 24i32;
    }
    *kctx = 0 as *mut libc::c_void;
    req_oid.elements = oid_buf.as_mut_ptr() as *mut libc::c_void;
    req_oid.length = ::std::mem::size_of::<[u8; 17]>() as crate::gssapi_h::OM_uint32;
    major_status = crate::src::generic::oid_ops::generic_gss_oid_compose(
        minor_status,
        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x06\x00" as *const u8 as *const i8,
        11usize,
        version as i32,
        &mut req_oid,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major_status;
    }
    major_status = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        minor_status,
        *context_handle,
        &mut req_oid,
        &mut data_set,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major_status;
    }
    if data_set.is_null()
        || (*data_set).count != 1usize
        || (*(*data_set).elements.offset(0isize)).length
            != ::std::mem::size_of::<*mut libc::c_void>()
    {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    *kctx = *((*(*data_set).elements.offset(0isize)).value as *mut *mut libc::c_void);
    /* Clean up the context state (it is an error for
     * someone to attempt to use this context again)
     */
    crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
        minor_status,
        context_handle,
        0 as crate::gssapi_h::gss_buffer_t,
    );
    *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t;
    crate::src::generic::util_buffer_set::generic_gss_release_buffer_set(&mut minor, &mut data_set);
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_set_allowable_enctypes(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut num_ktypes: crate::gssapi_h::OM_uint32,
    mut ktypes: *mut crate::krb5_h::krb5_enctype,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x04\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req = crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req {
        num_ktypes: 0,
        ktypes: 0 as *mut crate::krb5_h::krb5_enctype,
    };
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    req.num_ktypes = num_ktypes;
    req.ktypes = ktypes;
    req_buffer.length =
        ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req>();
    req_buffer.value = &mut req as *mut crate::gssapiP_krb5_h::krb5_gss_set_allowable_enctypes_req
        as *mut libc::c_void;
    major_status = crate::src::mechglue::g_set_cred_option::gss_set_cred_option(
        minor_status,
        &mut cred,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_ccache_name(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut name: *const i8,
    mut out_name: *mut *const i8,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x03\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req = crate::gssapiP_krb5_h::krb5_gss_ccache_name_req {
        name: 0 as *const i8,
        out_name: 0 as *mut *const i8,
    };
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    req.name = name;
    req.out_name = out_name;
    req_buffer.length = ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ccache_name_req>();
    req_buffer.value =
        &mut req as *mut crate::gssapiP_krb5_h::krb5_gss_ccache_name_req as *mut libc::c_void;
    major_status = crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke(
        minor_status,
        crate::src::krb5::gssapi_krb5::gss_mech_krb5,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_free_lucid_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut kctx: *mut libc::c_void,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x07\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    req_buffer.length = ::std::mem::size_of::<*mut libc::c_void>();
    req_buffer.value = kctx;
    major_status = crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke(
        minor_status,
        crate::src::krb5::gssapi_krb5::gss_mech_krb5,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_register_acceptor_identity(
    mut keytab: *const i8,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\t\x00" as *const u8 as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    req_buffer.length = if keytab.is_null() {
        0usize
    } else {
        crate::stdlib::strlen(keytab)
    };
    req_buffer.value = keytab as *mut libc::c_void;
    major_status = crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke(
        &mut minor_status,
        crate::src::krb5::gssapi_krb5::gss_mech_krb5,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2008 by the Massachusetts Institute of Technology.
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
 *
 */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/* work around sunos braindamage */
/* The include of gssapi_krb5.h will dtrt with the above #defines in
 * effect.
 */
/* for debugging */
/* * constants **/
/* Incorrect krb5 mech OID emitted by MS. */
/* IAKERB variant */
/* * CFX flags **/
/* These are to be stored in little-endian order, i.e., des-mac is
stored as 02 00.  */
/* SGN_ALG_DES_MAC_MD5           = 0x0000, */
/* SGN_ALG_MD2_5                 = 0x0001, */
/* SGN_ALG_DES_MAC               = 0x0002, */
/* SGN_ALG_3                     = 0x0003, /\* not published *\/ */
/* microsoft w2k;  */
/* SEAL_ALG_DES             = 0x0000, */
/* SEAL_ALG_1               = 0x0001, /\* not published *\/ */
/* microsoft w2k;  */
/* for 3DES */
/* for draft-ietf-krb-wg-gssapi-cfx-01 */
/* GSS_KRB5_INTEG_C_QOP_MD5       = 0x0001, */
/* GSS_KRB5_INTEG_C_QOP_DES_MD5   = 0x0002, */
/* GSS_KRB5_INTEG_C_QOP_DES_MAC   = 0x0003, */
/* GSS_KRB5_CONF_C_QOP_DES        = 0x0100, */
/* * internal types **/
/* immutable */
/* immutable */
/* immutable */
/* protects ad_context only for now */
/* protect against simultaneous accesses */
/* name/type of credential */
/* keytab (accept) data */
/* ccache (init) data */
/* limit negotiated enctypes to this list */
/* nonzero if initiating, zero if accepting */
/* XXX tested but never actually set */
/* One of two potential keys to use with RFC 4121
 * packets; this key must always be set. */
/* RFC 1964 encryption key; seq xored with a constant
 * for DES, seq for other RFC 1964 enctypes  */
/* RFC 1964 sequencing key */
/* XXX these used to be signed.  the old spec is inspecific, and
the new spec specifies unsigned.  I don't believe that the change
affects the wire encoding. */
/* Protocol spec revision for sending packets
0 => RFC 1964 with 3DES and RC4 enhancements
1 => RFC 4121
No others defined so far.  It is always permitted to receive
tokens in RFC 4121 format.  If enc is non-null, receiving RFC
1964 tokens is permitted.*/
/* for "main" subkey */
/* CFX only */
/* did we get rcache from creds? */
/* LEAN_CLIENT */
/* * helper functions **/
/* Encrypt length bytes at ptr in place, with the given key and usage.  If
 * iv is not NULL, use it as the cipher state. */
/* AEAD */
/* for conf len */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* desired_name */
/* time_req */
/* desired_mechs */
/* cred_usage */
/* output_cred_handle */
/* actual_mechs */
/* time_rec */
/* minor_status */
/* cred_handle */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* minor_status */
/* claimant_cred_handle */
/* context_handle */
/* target_name */
/* mech_type */
/* req_flags */
/* time_req */
/* input_chan_bindings */
/* input_token */
/* actual_mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* exts */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/* minor_status */
/* context_handle */
/* verifier_cred_handle */
/* input_token_buffer */
/* input_chan_bindings */
/* src_name */
/* mech_type */
/* output_token */
/* ret_flags */
/* time_rec */
/* delegated_cred_handle */
/*exts */
/* LEAN_CLIENT */
/* minor_status */
/* context_handle */
/* desired_object */
/* data_set */
/* minor_status */
/* context_handle */
/* desired_object */
/* value */
/* minor_status */
/* context_handle */
/* token_buffer */
/* minor_status */
/* context_handle */
/* output_token */
/* minor_status */
/* context_handle */
/* time_rec */
/* minor_status */
/* status_value */
/* status_type */
/* mech_type */
/* message_context */
/* status_string */
/* minor_status */
/* mech_set */
/* minor_status */
/* name1 */
/* name2 */
/* name_equal */
/* minor_status */
/* input_name */
/* output_name_buffer */
/* output_name_type */
/* minor_status */
/* input_name_buffer */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* minor_status */
/* cred_handle */
/* name */
/* lifetime */
/* cred_usage */
/* mechanisms */
/* minor_status */
/* context_handle */
/* initiator_name */
/* acceptor_name */
/* lifetime_rec */
/* mech_type */
/* ret_flags */
/* locally_initiated */
/* open */
/* New V2 entry points */
/* minor_status */
/* context_handle */
/* qop_req */
/* message_buffer */
/* message_token */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* qop_req */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* message_buffer */
/* message_token */
/* qop_state */
/* minor_status */
/* context_handle */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* input_message_buffer */
/* conf_state */
/* output_message_buffer */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* conf_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* input_message_buffer */
/* output_message_buffer */
/* conf_state */
/* qop_state */
/* minor_status */
/* context_handle */
/* conf_state */
/* qop_state */
/* iov */
/* iov_count */
/* minor_status */
/* context_handle */
/* conf_req_flag */
/* qop_req */
/* req_output_size */
/* max_input_size */
/* minor_status */
/* input_name */
/* input_name_type */
/* output_name */
/* minor_status */
/* input_name */
/* desired_name_type */
/* output_name */
/* minor_status */
/* cred_handle */
/* mech_type */
/* name */
/* initiator_lifetime */
/* acceptor_lifetime */
/* cred_usage */
/* minor_status */
/* context_handle */
/* interprocess_token */
/* minor_status */
/* interprocess_token */
/* context_handle */
/* LEAN_CLIENT */
/* minor_status */
/* oid */
/* minor_status */
/* oid */
/* minor_status */
/* mechanism */
/* name_types */
/* minor_status */
/* input_name */
/* mech_type */
/* output_name */
/* minor_status */
/* input_name */
/* exported_name */
/* minor_status */
/* input_name */
/* dest_name */
/* minor_status */
/* cred */
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
/* cred_handle */
/* context */
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_use_kdc_context() -> crate::krb5_h::krb5_error_code {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x08\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    req_buffer.length = 0usize;
    req_buffer.value = 0 as *mut libc::c_void;
    major_status = crate::src::mechglue::g_mech_invoke::gssspi_mech_invoke(
        &mut minor_status,
        crate::src::krb5::gssapi_krb5::gss_mech_krb5,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    if major_status != 0u32 {
        if minor_status != 0u32 {
            ret = minor_status as crate::krb5_h::krb5_error_code
        } else {
            ret = -(1765328324 as isize) as crate::krb5_h::krb5_error_code
        }
    } else {
        ret = 0i32
    }
    return ret;
}
/*
 * This API should go away and be replaced with an accessor
 * into a gss_name_t.
 */
#[no_mangle]

pub unsafe extern "C" fn gsskrb5_extract_authz_data_from_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut ad_type: i32,
    mut ad_data: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut req_oid = crate::gssapi_h::gss_OID_desc {
        length: 0,
        elements: 0 as *mut libc::c_void,
    };
    let mut oid_buf: [u8; 17] = [0; 17];
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    if ad_data.is_null() {
        return (2u32) << 24i32;
    }
    req_oid.elements = oid_buf.as_mut_ptr() as *mut libc::c_void;
    req_oid.length = ::std::mem::size_of::<[u8; 17]>() as crate::gssapi_h::OM_uint32;
    major_status = crate::src::generic::oid_ops::generic_gss_oid_compose(
        minor_status,
        b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\n\x00" as *const u8 as *const i8,
        11usize,
        ad_type,
        &mut req_oid,
    );
    if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        return major_status;
    }
    major_status = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        minor_status,
        context_handle,
        &mut req_oid as *mut crate::gssapi_h::gss_OID_desc,
        &mut data_set,
    );
    if major_status != 0u32 {
        return major_status;
    }
    if data_set.is_null() || (*data_set).count != 1usize {
        return (13u32) << 16i32;
    }
    (*ad_data).length = (*(*data_set).elements.offset(0isize)).length;
    (*ad_data).value = (*(*data_set).elements.offset(0isize)).value;
    (*(*data_set).elements.offset(0isize)).length = 0usize;
    let ref mut fresh0 = (*(*data_set).elements.offset(0isize)).value;
    *fresh0 = 0 as *mut libc::c_void;
    (*data_set).count = 0usize;
    crate::src::mechglue::g_buffer_set::gss_release_buffer_set(minor_status, &mut data_set);
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5_set_cred_rcache(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut rcache: crate::krb5_h::krb5_rcache,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0b\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut req_buffer = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    req_buffer.length = ::std::mem::size_of::<crate::krb5_h::krb5_rcache>();
    req_buffer.value = rcache as *mut libc::c_void;
    major_status = crate::src::mechglue::g_set_cred_option::gss_set_cred_option(
        minor_status,
        &mut cred,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut req_buffer,
    );
    return major_status;
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/* C++ friendlyness */
/* __cplusplus */
/* Reserved static storage for GSS_oids.  See rfc 1964 for more details. */
/* 2.1.1. Kerberos Principal Name Form: */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5_name(1)}.  The recommended symbolic name for this type
 * is "GSS_KRB5_NT_PRINCIPAL_NAME". */
/* 2.1.2. Host-Based Service Name Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The previously recommended symbolic
 * name for this type is "GSS_KRB5_NT_HOSTBASED_SERVICE_NAME".  The
 * currently preferred symbolic name for this type is
 * "GSS_C_NT_HOSTBASED_SERVICE". */
/* 2.2.1. User Name Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) user_name(1)}.  The recommended symbolic name for this
 * type is "GSS_KRB5_NT_USER_NAME". */
/* 2.2.2. Machine UID Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) machine_uid_name(2)}.  The recommended symbolic name for
 * this type is "GSS_KRB5_NT_MACHINE_UID_NAME". */
/* 2.2.3. String UID Form */
/* This name form shall be represented by the Object Identifier {iso(1)
 * member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) string_uid_name(3)}.  The recommended symbolic name for
 * this type is "GSS_KRB5_NT_STRING_UID_NAME". */
/* Kerberos Enterprise Name Form (see RFC 6806 section 5): */
/* {iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-enterprise-name(6)}. */
/*
 * This OID can be used with gss_set_cred_option() to suppress the
 * confidentiality and integrity flags from being asserted in initial context
 * tokens.
 *
 * iso(1) member-body(2) Sweden(752) Stockholm University(43) Heimdal GSS-API
 * Extensions(13) no_ci_flags(29)
 */
/*
 * This OID can be used with gss_inquire_cred_by_oid(0 to retrieve the
 * impersonator name (if any).
 *
 * iso(1) member-body(2) United States(840) mit(113554) infosys(1) gssapi(2)
 * krb5(2) krb5-gssapi-ext(5) get-cred-impersonator(14)
 */
/* key encryption type */
/* length of key data */
/* actual key data */
/* signing algorthm */
/* seal/encrypt algorithm */
/* Context key
(Kerberos session key or subkey) */
/* 1 if there is an acceptor_subkey
present, 0 otherwise */
/* Context key
(Kerberos session key or subkey) */
/* acceptor-asserted subkey or
0's if no acceptor subkey */
/* Structure version number (1)
MUST be at beginning of struct! */
/* Are we the initiator? */
/* expiration time of context */
/* sender sequence number */
/* receive sequence number */
/* 0: rfc1964,
1: draft-ietf-krb-wg-gssapi-cfx-07 */
/*
 * if (protocol == 0) rfc1964_kd should be used
 * and cfx_kd contents are invalid and should be zero
 * if (protocol == 1) cfx_kd should be used
 * and rfc1964_kd contents are invalid and should be zero
 */
/*
 * Mask for determining the version of a lucid context structure.  Callers
 * should not require this.
 */
/* Structure version number */
/* Alias for Heimdal compat. */
/*
 * Copy krb5 creds from cred_handle into out_ccache, which must already be
 * initialized.  Use gss_store_cred_into() (new in krb5 1.11) instead, if
 * possible.
 */
/*
 * gss_krb5_set_allowable_enctypes
 *
 * This function may be called by a context initiator after calling
 * gss_acquire_cred(), but before calling gss_init_sec_context(),
 * to restrict the set of enctypes which will be negotiated during
 * context establishment to those in the provided array.
 *
 * 'cred' must be a valid credential handle obtained via
 * gss_acquire_cred().  It may not be GSS_C_NO_CREDENTIAL.
 * gss_acquire_cred() may have been called to get a handle to
 * the default credential.
 *
 * The purpose of this function is to limit the keys that may
 * be exported via gss_krb5_export_lucid_sec_context(); thus it
 * should limit the enctypes of all keys that will be needed
 * after the security context has been established.
 * (i.e. context establishment may use a session key with a
 * stronger enctype than in the provided array, however a
 * subkey must be established within the enctype limits
 * established by this function.)
 *
 */
/*
 * Returns a non-opaque (lucid) version of the internal context
 * information.
 *
 * Note that context_handle must not be used again by the caller
 * after this call.  The GSS implementation is free to release any
 * resources associated with the original context.  It is up to the
 * GSS implementation whether it returns pointers to existing data,
 * or copies of the data.  The caller should treat the returned
 * lucid context as read-only.
 *
 * The caller must call gss_krb5_free_lucid_context() to free
 * the context and allocated resources when it is finished with it.
 *
 * 'version' is an integer indicating the requested version of the lucid
 * context.  If the implementation does not understand the requested version,
 * it will return an error.
 *
 * For example:
 *      void *return_ctx;
 *      gss_krb5_lucid_context_v1_t *ctx;
 *      OM_uint32 min_stat, maj_stat;
 *      OM_uint32 vers;
 *      gss_ctx_id_t *ctx_handle;
 *
 *      maj_stat = gss_krb5_export_lucid_sec_context(&min_stat,
 *                      ctx_handle, 1, &return_ctx);
 *      // Verify success
 *      ctx = (gss_krb5_lucid_context_v1_t *) return_ctx;
 */
/*
 * Frees the allocated storage associated with an
 * exported struct gss_krb5_lucid_context.
 */
#[no_mangle]

pub unsafe extern "C" fn gsskrb5_extract_authtime_from_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut authtime: *mut crate::krb5_h::krb5_timestamp,
) -> crate::gssapi_h::OM_uint32 {
    static mut req_oid: crate::gssapi_h::gss_OID_desc = {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0c\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    };
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut data_set = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    if authtime.is_null() {
        return (2u32) << 24i32;
    }
    major_status = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        minor_status,
        context_handle,
        &req_oid as *const crate::gssapi_h::gss_OID_desc as crate::gssapi_h::gss_OID,
        &mut data_set,
    );
    if major_status != 0u32 {
        return major_status;
    }
    if data_set.is_null()
        || (*data_set).count != 1usize
        || (*(*data_set).elements.offset(0isize)).length
            != ::std::mem::size_of::<crate::krb5_h::krb5_timestamp>()
    {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    *authtime =
        *((*(*data_set).elements.offset(0isize)).value as *mut crate::krb5_h::krb5_timestamp);
    crate::src::mechglue::g_buffer_set::gss_release_buffer_set(minor_status, &mut data_set);
    *minor_status = 0u32;
    return 0u32;
}
