use ::libc;

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void) -> u32 {
        let mut p = cvp as *const u8;
        return (*(p as *const crate::k5_platform_h::C2RustUnnamed_6)).i;
    }

    /* K5_PLATFORM_H */
}

pub mod k5_int_h {

    #[inline]

    pub unsafe extern "C" fn make_data(
        mut data: *mut libc::c_void,
        mut len: u32,
    ) -> crate::krb5_h::krb5_data {
        let mut d = crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        };
        d.magic = -(1760647422 as isize) as crate::krb5_h::krb5_magic;
        d.data = data as *mut i8;
        d.length = len;
        return d;
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]

    pub unsafe extern "C" fn k5calloc(
        mut nmemb: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
        ptr = crate::stdlib::calloc(
            if nmemb != 0 { nmemb } else { 1usize },
            if size != 0 { size } else { 1usize },
        );
        *code = if ptr.is_null() { 12i32 } else { 0i32 };
        return ptr;
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]

    pub unsafe extern "C" fn k5alloc(
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        return k5calloc(1usize, size, code);
    }
    /* Return a copy of the len bytes of memory at in; set *code to 0 or ENOMEM. */
    #[inline]

    pub unsafe extern "C" fn k5memdup(
        mut in_0: *const libc::c_void,
        mut len: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = k5alloc(len, code);
        if !ptr.is_null() && len > 0usize {
            crate::stdlib::memcpy(ptr, in_0, len);
        }
        return ptr;
    }

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod gssapi_alloc_h {
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;

pub use crate::gssapiP_spnego_h::negoex_mech_list;
pub use crate::gssapiP_spnego_h::spnego_ctx_st;
pub use crate::gssapiP_spnego_h::spnego_gss_ctx_id_t;
pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_context;
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
pub use crate::k5_platform_h::C2RustUnnamed_6;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::krb5_h::_krb5_checksum;
pub use crate::krb5_h::_krb5_crypto_iov;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_c_is_keyed_cksum;
pub use crate::krb5_h::krb5_c_make_checksum;
pub use crate::krb5_h::krb5_c_verify_checksum_iov;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_crypto_iov;
pub use crate::krb5_h::krb5_cryptotype;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_free_keyblock_contents;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::profile_h::profile_t;
pub use crate::src::spnego::negoex_ctx::k5_int_h::k5alloc;
pub use crate::src::spnego::negoex_ctx::k5_int_h::k5calloc;
pub use crate::src::spnego::negoex_ctx::k5_int_h::k5memdup;
pub use crate::src::spnego::negoex_ctx::k5_int_h::make_data;
pub use crate::src::spnego::negoex_ctx::k5_platform_h::load_32_le;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint8_t;

pub use crate::gssapiP_negoex_h::alert_message;
pub use crate::gssapiP_negoex_h::auth_scheme;
pub use crate::gssapiP_negoex_h::conversation_id;
pub use crate::gssapiP_negoex_h::exchange_message;
pub use crate::gssapiP_negoex_h::message_type;
pub use crate::gssapiP_negoex_h::nego_message;
pub use crate::gssapiP_negoex_h::negoex_auth_mech;
pub use crate::gssapiP_negoex_h::negoex_message;
pub use crate::gssapiP_negoex_h::verify_message;
pub use crate::gssapiP_negoex_h::C2RustUnnamed_1;
pub use crate::gssapiP_negoex_h::C2RustUnnamed_2;
pub use crate::gssapiP_negoex_h::ACCEPTOR_META_DATA;
pub use crate::gssapiP_negoex_h::ACCEPTOR_NEGO;
pub use crate::gssapiP_negoex_h::ALERT;
pub use crate::gssapiP_negoex_h::AP_REQUEST;
pub use crate::gssapiP_negoex_h::CHALLENGE;
pub use crate::gssapiP_negoex_h::INITIATOR_META_DATA;
pub use crate::gssapiP_negoex_h::INITIATOR_NEGO;
pub use crate::gssapiP_negoex_h::VERIFY;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
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
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::k5_buf_h::k5_buf_add_len;
pub use crate::k5_buf_h::k5buf;
pub use crate::k5_buf_h::k5buftype;
pub use crate::k5_buf_h::K5BUF_DYNAMIC;
pub use crate::k5_buf_h::K5BUF_DYNAMIC_ZAP;
pub use crate::k5_buf_h::K5BUF_ERROR;
pub use crate::k5_buf_h::K5BUF_FIXED;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::gssapi_generic::GSS_C_INQ_NEGOEX_KEY;
pub use crate::src::generic::gssapi_generic::GSS_C_INQ_NEGOEX_VERIFY_KEY;
pub use crate::src::mechglue::g_accept_sec_context::gss_accept_sec_context;
pub use crate::src::mechglue::g_buffer_set::gss_release_buffer_set;
pub use crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context;
pub use crate::src::mechglue::g_init_sec_context::gss_init_sec_context;
pub use crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid;
pub use crate::src::mechglue::g_negoex::gssspi_exchange_meta_data;
pub use crate::src::mechglue::g_negoex::gssspi_query_meta_data;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::src::mechglue::g_rel_cred::gss_release_cred;
pub use crate::src::mechglue::g_rel_name::gss_release_name;
pub use crate::src::spnego::negoex_ctx::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::spnego::negoex_util::negoex_add_exchange_message;
pub use crate::src::spnego::negoex_util::negoex_add_nego_message;
pub use crate::src::spnego::negoex_util::negoex_add_verify_message;
pub use crate::src::spnego::negoex_util::negoex_add_verify_no_key_alert;
pub use crate::src::spnego::negoex_util::negoex_common_auth_schemes;
pub use crate::src::spnego::negoex_util::negoex_delete_auth_mech;
pub use crate::src::spnego::negoex_util::negoex_locate_alert_message;
pub use crate::src::spnego::negoex_util::negoex_locate_auth_scheme;
pub use crate::src::spnego::negoex_util::negoex_locate_exchange_message;
pub use crate::src::spnego::negoex_util::negoex_locate_nego_message;
pub use crate::src::spnego::negoex_util::negoex_locate_verify_message;
pub use crate::src::spnego::negoex_util::negoex_parse_token;
pub use crate::src::spnego::negoex_util::negoex_prep_context_for_negoex;
pub use crate::src::spnego::negoex_util::negoex_prep_context_for_spnego;
pub use crate::src::spnego::negoex_util::negoex_random;
pub use crate::src::spnego::negoex_util::negoex_restrict_auth_schemes;
pub use crate::src::spnego::negoex_util::negoex_select_auth_mech;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2011-2018 PADL Software Pty Ltd.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*
 * The initial context token emitted by the initiator is a INITIATOR_NEGO
 * message followed by zero or more INITIATOR_META_DATA tokens, and zero
 * or one AP_REQUEST tokens.
 *
 * Upon receiving this, the acceptor computes the list of mutually supported
 * authentication mechanisms and performs the metadata exchange. The output
 * token is ACCEPTOR_NEGO followed by zero or more ACCEPTOR_META_DATA tokens,
 * and zero or one CHALLENGE tokens.
 *
 * Once the metadata exchange is complete and a mechanism is selected, the
 * selected mechanism's context token exchange continues with AP_REQUEST and
 * CHALLENGE messages.
 *
 * Once the context token exchange is complete, VERIFY messages are sent to
 * authenticate the entire exchange.
 */

unsafe extern "C" fn zero_and_release_buffer_set(
    mut pbuffers: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) {
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut buffers = *pbuffers;
    let mut i: crate::stdlib::uint32_t = 0;
    if !buffers.is_null() {
        i = 0u32;
        while (i as usize) < (*buffers).count {
            crate::stdlib::explicit_bzero(
                (*(*buffers).elements.offset(i as isize)).value,
                (*(*buffers).elements.offset(i as isize)).length,
            );
            i = i.wrapping_add(1)
        }
        crate::src::mechglue::g_buffer_set::gss_release_buffer_set(&mut tmpmin, &mut buffers);
    }
    *pbuffers = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
}

unsafe extern "C" fn buffer_set_to_key(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut buffers: crate::gssapi_ext_h::gss_buffer_set_t,
    mut key: *mut crate::krb5_h::krb5_keyblock,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    /* Returned keys must be in two buffers, with the key contents in the first
     * and the enctype as a 32-bit little-endian integer in the second. */
    if (*buffers).count != 2usize || (*(*buffers).elements.offset(1isize)).length != 4usize {
        *minor = 0x20000014u32;
        return (13u32) << 16i32;
    }
    crate::krb5_h::krb5_free_keyblock_contents(0 as crate::krb5_h::krb5_context, key);
    (*key).contents = k5memdup(
        (*(*buffers).elements.offset(0isize)).value,
        (*(*buffers).elements.offset(0isize)).length,
        &mut ret,
    ) as *mut crate::krb5_h::krb5_octet;
    if (*key).contents.is_null() {
        *minor = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    (*key).length = (*(*buffers).elements.offset(0isize)).length as u32;
    (*key).enctype =
        load_32_le((*(*buffers).elements.offset(1isize)).value) as crate::krb5_h::krb5_enctype;
    return 0u32;
}

unsafe extern "C" fn get_session_keys(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut mech: *mut crate::gssapiP_negoex_h::negoex_auth_mech,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut buffers = 0 as crate::gssapi_ext_h::gss_buffer_set_t;
    major = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        &mut tmpmin,
        (*mech).mech_context,
        crate::src::generic::gssapi_generic::GSS_C_INQ_NEGOEX_KEY,
        &mut buffers,
    );
    if major == 0u32 {
        major = buffer_set_to_key(minor, buffers, &mut (*mech).key);
        zero_and_release_buffer_set(&mut buffers);
        if major != 0u32 {
            return major;
        }
    }
    major = crate::src::mechglue::g_inq_context_oid::gss_inquire_sec_context_by_oid(
        &mut tmpmin,
        (*mech).mech_context,
        crate::src::generic::gssapi_generic::GSS_C_INQ_NEGOEX_VERIFY_KEY,
        &mut buffers,
    );
    if major == 0u32 {
        major = buffer_set_to_key(minor, buffers, &mut (*mech).verify_key);
        zero_and_release_buffer_set(&mut buffers);
        if major != 0u32 {
            return major;
        }
    }
    return 0u32;
}

unsafe extern "C" fn emit_initiator_nego(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut random: [crate::stdlib::uint8_t; 32] = [0; 32];
    major =
        crate::src::spnego::negoex_util::negoex_random(minor, ctx, random.as_mut_ptr(), 32usize);
    if major != 0u32 {
        return major;
    }
    crate::src::spnego::negoex_util::negoex_add_nego_message(
        ctx,
        crate::gssapiP_negoex_h::INITIATOR_NEGO,
        random.as_mut_ptr(),
    );
    return 0u32;
}

unsafe extern "C" fn process_initiator_nego(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::nego_message;
    if (*ctx).initiate == 0 && (*ctx).negoex_step == 1i32 {
    } else {
        crate::stdlib::__assert_fail(b"!ctx->initiate && ctx->negoex_step == 1\x00" as
                          *const u8 as *const i8,
                      b"negoex_ctx.c\x00" as *const u8 as *const i8,
                      146u32,
                      (*::std::mem::transmute::<&[u8; 100],
                                                &[i8; 100]>(b"OM_uint32 process_initiator_nego(OM_uint32 *, spnego_gss_ctx_id_t, struct negoex_message *, size_t)\x00")).as_ptr());
    }
    msg = crate::src::spnego::negoex_util::negoex_locate_nego_message(
        messages,
        nmessages,
        crate::gssapiP_negoex_h::INITIATOR_NEGO,
    );
    if msg.is_null() {
        *minor = 0x20000011u32;
        return (9u32) << 16i32;
    }
    crate::src::spnego::negoex_util::negoex_restrict_auth_schemes(
        ctx,
        (*msg).schemes,
        (*msg).nschemes,
    );
    return 0u32;
}

unsafe extern "C" fn emit_acceptor_nego(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut random: [crate::stdlib::uint8_t; 32] = [0; 32];
    major =
        crate::src::spnego::negoex_util::negoex_random(minor, ctx, random.as_mut_ptr(), 32usize);
    if major != 0u32 {
        return major;
    }
    crate::src::spnego::negoex_util::negoex_add_nego_message(
        ctx,
        crate::gssapiP_negoex_h::ACCEPTOR_NEGO,
        random.as_mut_ptr(),
    );
    return 0u32;
}

unsafe extern "C" fn process_acceptor_nego(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::nego_message;
    msg = crate::src::spnego::negoex_util::negoex_locate_nego_message(
        messages,
        nmessages,
        crate::gssapiP_negoex_h::ACCEPTOR_NEGO,
    );
    if msg.is_null() {
        *minor = 0x20000011u32;
        return (9u32) << 16i32;
    }
    /* Reorder and prune our mech list to match the acceptor's list (or a
     * subset of it). */
    crate::src::spnego::negoex_util::negoex_common_auth_schemes(
        ctx,
        (*msg).schemes,
        (*msg).nschemes,
    );
    return 0u32;
}

unsafe extern "C" fn query_meta_data(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut target: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
) {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut p = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut next = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    p = (*ctx).negoex_mechs.tqh_first;
    while !p.is_null() && {
        next = (*p).links.tqe_next;
        (1i32) != 0
    } {
        major = crate::src::mechglue::g_negoex::gssspi_query_meta_data(
            &mut minor,
            (*p).oid as crate::gssapi_h::gss_const_OID,
            cred,
            &mut (*p).mech_context,
            target,
            req_flags,
            &mut (*p).metadata,
        );
        /* GSS_Query_meta_data failure removes mechanism from list. */
        if major != 0u32 {
            crate::src::spnego::negoex_util::negoex_delete_auth_mech(ctx, p);
        }
        p = next
    }
}

unsafe extern "C" fn exchange_meta_data(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut target: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
) {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut minor: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut type_0 = crate::gssapiP_negoex_h::INITIATOR_NEGO;
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::exchange_message;
    let mut i: crate::stdlib::uint32_t = 0;
    type_0 = if (*ctx).initiate != 0 {
        crate::gssapiP_negoex_h::ACCEPTOR_META_DATA as i32
    } else {
        crate::gssapiP_negoex_h::INITIATOR_META_DATA as i32
    } as crate::gssapiP_negoex_h::message_type;
    i = 0u32;
    while (i as usize) < nmessages {
        if !((*messages.offset(i as isize)).type_0 != type_0) {
            msg = &mut (*messages.offset(i as isize)).u.e;
            mech = crate::src::spnego::negoex_util::negoex_locate_auth_scheme(
                ctx,
                (*msg).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
            );
            if !mech.is_null() {
                major = crate::src::mechglue::g_negoex::gssspi_exchange_meta_data(
                    &mut minor,
                    (*mech).oid as crate::gssapi_h::gss_const_OID,
                    cred,
                    &mut (*mech).mech_context,
                    target,
                    req_flags,
                    &mut (*msg).token as *mut crate::gssapi_h::gss_buffer_desc
                        as crate::gssapi_h::gss_const_buffer_t,
                );
                /* GSS_Exchange_meta_data failure removes mechanism from list. */
                if major != 0u32 {
                    crate::src::spnego::negoex_util::negoex_delete_auth_mech(ctx, mech);
                }
            }
        }
        i = i.wrapping_add(1)
    }
}
/*
 * In the initiator, if we are processing the acceptor's first reply, discard
 * the optimistic context if the acceptor ignored the optimistic token.  If the
 * acceptor continued the optimistic mech, discard all other mechs.
 */

unsafe extern "C" fn check_optimistic_result(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
) {
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    if (*ctx).initiate != 0 && (*ctx).negoex_step == 2i32 {
    } else {
        crate::stdlib::__assert_fail(b"ctx->initiate && ctx->negoex_step == 2\x00" as
                          *const u8 as *const i8,
                      b"negoex_ctx.c\x00" as *const u8 as *const i8,
                      250u32,
                      (*::std::mem::transmute::<&[u8; 83],
                                                &[i8; 83]>(b"void check_optimistic_result(spnego_gss_ctx_id_t, struct negoex_message *, size_t)\x00")).as_ptr());
    }
    /* Do nothing if we didn't make an optimistic context. */
    mech = (*ctx).negoex_mechs.tqh_first;
    if mech.is_null() || (*mech).mech_context.is_null() {
        return;
    }
    /* If the acceptor used the optimistic token, it will send an acceptor
     * token or a checksum (or both) in its first reply. */
    if !crate::src::spnego::negoex_util::negoex_locate_exchange_message(
        messages,
        nmessages,
        crate::gssapiP_negoex_h::CHALLENGE,
    )
    .is_null()
        || !crate::src::spnego::negoex_util::negoex_locate_verify_message(messages, nmessages)
            .is_null()
    {
        /* The acceptor continued the optimistic mech, and metadata exchange
         * didn't remove it.  Commit to this mechanism. */
        crate::src::spnego::negoex_util::negoex_select_auth_mech(ctx, mech);
    } else {
        /* The acceptor ignored the optimistic token.  Restart the mech. */
        crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
            &mut tmpmin,
            &mut (*mech).mech_context,
            0 as crate::gssapi_h::gss_buffer_t,
        );
        crate::krb5_h::krb5_free_keyblock_contents(
            0 as crate::krb5_h::krb5_context,
            &mut (*mech).key,
        );
        crate::krb5_h::krb5_free_keyblock_contents(
            0 as crate::krb5_h::krb5_context,
            &mut (*mech).verify_key,
        );
        (*mech).sent_checksum = 0i32;
        (*mech).complete = (*mech).sent_checksum
    };
}
/* Perform an initiator step of the underlying mechanism exchange. */

unsafe extern "C" fn mech_init(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut target: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut first_major = 0u32;
    let mut first_minor = 0u32;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut input_token = 0 as crate::gssapi_h::gss_buffer_t;
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::exchange_message;
    let mut first_mech: i32 = 0;
    (*output_token).value = 0 as *mut libc::c_void;
    (*output_token).length = 0usize;
    /* Allow disabling of optimistic token for testing. */
    if (*ctx).negoex_step == 1i32
        && !crate::stdlib::secure_getenv(
            b"NEGOEX_NO_OPTIMISTIC_TOKEN\x00" as *const u8 as *const i8,
        )
        .is_null()
    {
        return 0u32;
    }
    if (*ctx).negoex_mechs.tqh_first.is_null() {
        *minor = 0x20000013u32;
        return (13u32) << 16i32;
    }
    /*
     * Get the input token.  The challenge could be for the optimistic mech,
     * which we might have discarded in metadata exchange, so ignore the
     * challenge if it doesn't match the first auth mech.
     */
    mech = (*ctx).negoex_mechs.tqh_first;
    msg = crate::src::spnego::negoex_util::negoex_locate_exchange_message(
        messages,
        nmessages,
        crate::gssapiP_negoex_h::CHALLENGE,
    );
    if !msg.is_null()
        && crate::stdlib::memcmp(
            (*msg).scheme.as_mut_ptr() as *const libc::c_void,
            (*mech).scheme.as_mut_ptr() as *const libc::c_void,
            16usize,
        ) == 0i32
    {
        input_token = &mut (*msg).token
    }
    if (*mech).complete != 0 {
        return 0u32;
    }
    first_mech = 1i32;
    while !(*ctx).negoex_mechs.tqh_first.is_null() {
        mech = (*ctx).negoex_mechs.tqh_first;
        major = crate::src::mechglue::g_init_sec_context::gss_init_sec_context(
            minor,
            cred,
            &mut (*mech).mech_context,
            target,
            (*mech).oid,
            req_flags,
            time_req,
            0 as crate::gssapi_h::gss_channel_bindings_t,
            input_token,
            &mut (*ctx).actual_mech,
            output_token,
            &mut (*ctx).ctx_flags,
            time_rec,
        );
        if major == 0u32 {
            (*mech).complete = 1i32
        }
        if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) == 0 {
            return get_session_keys(minor, mech);
        }
        /* Remember the error we got from the first mech. */
        if first_mech != 0 {
            first_major = major;
            first_minor = *minor
        }
        /* If we still have multiple mechs to try, move on to the next one. */
        crate::src::spnego::negoex_util::negoex_delete_auth_mech(ctx, mech);
        first_mech = 0i32;
        input_token = 0 as crate::gssapi_h::gss_buffer_t
    }
    if (*ctx).negoex_mechs.tqh_first.is_null() {
        major = first_major;
        *minor = first_minor
    }
    return major;
}
/* Perform an acceptor step of the underlying mechanism exchange. */

unsafe extern "C" fn mech_accept(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::exchange_message;
    if (*ctx).initiate == 0 && !(*ctx).negoex_mechs.tqh_first.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"!ctx->initiate && !K5_TAILQ_EMPTY(&ctx->negoex_mechs)\x00"
                          as *const u8 as *const i8,
                      b"negoex_ctx.c\x00" as *const u8 as *const i8,
                      360u32,
                      (*::std::mem::transmute::<&[u8; 131],
                                                &[i8; 131]>(b"OM_uint32 mech_accept(OM_uint32 *, spnego_gss_ctx_id_t, gss_cred_id_t, struct negoex_message *, size_t, gss_buffer_t, OM_uint32 *)\x00")).as_ptr());
    }
    msg = crate::src::spnego::negoex_util::negoex_locate_exchange_message(
        messages,
        nmessages,
        crate::gssapiP_negoex_h::AP_REQUEST,
    );
    if msg.is_null() {
        /* No input token is okay on the first request or if the mech is
         * complete. */
        if (*ctx).negoex_step == 1i32 || (*(*ctx).negoex_mechs.tqh_first).complete != 0 {
            return 0u32;
        }
        *minor = 0x20000012u32;
        return (9u32) << 16i32;
    }
    if (*ctx).negoex_step == 1i32 {
        /* Ignore the optimistic token if it isn't for our most preferred
         * mech. */
        mech = (*ctx).negoex_mechs.tqh_first;
        if !(crate::stdlib::memcmp(
            (*msg).scheme.as_mut_ptr() as *const libc::c_void,
            (*mech).scheme.as_mut_ptr() as *const libc::c_void,
            16usize,
        ) == 0i32)
        {
            return 0u32;
        }
    } else {
        /* The initiator has selected a mech; discard other entries. */
        mech = crate::src::spnego::negoex_util::negoex_locate_auth_scheme(
            ctx,
            (*msg).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
        );
        if mech.is_null() {
            *minor = 0x20000013u32;
            return (13u32) << 16i32;
        }
        crate::src::spnego::negoex_util::negoex_select_auth_mech(ctx, mech);
    }
    if (*mech).complete != 0 {
        return 0u32;
    }
    if !(*ctx).internal_name.is_null() {
        crate::src::mechglue::g_rel_name::gss_release_name(&mut tmpmin, &mut (*ctx).internal_name);
    }
    if !(*ctx).deleg_cred.is_null() {
        crate::src::mechglue::g_rel_cred::gss_release_cred(&mut tmpmin, &mut (*ctx).deleg_cred);
    }
    major = crate::src::mechglue::g_accept_sec_context::gss_accept_sec_context(
        minor,
        &mut (*mech).mech_context,
        cred,
        &mut (*msg).token,
        0 as crate::gssapi_h::gss_channel_bindings_t,
        &mut (*ctx).internal_name,
        &mut (*ctx).actual_mech,
        output_token,
        &mut (*ctx).ctx_flags,
        time_rec,
        &mut (*ctx).deleg_cred,
    );
    if major == 0u32 {
        (*mech).complete = 1i32
    }
    if major & ((0o377u32) << 24i32 | (0o377u32) << 16i32) == 0 {
        major = get_session_keys(minor, mech)
    } else if (*ctx).negoex_step == 1i32 {
        /* This was an optimistic token; pretend this never happened. */
        major = 0u32;
        *minor = 0u32;
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, output_token);
        crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
            &mut tmpmin,
            &mut (*mech).mech_context,
            0 as crate::gssapi_h::gss_buffer_t,
        );
    }
    return major;
}

unsafe extern "C" fn verify_keyusage(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut make_checksum_0: i32,
) -> crate::krb5_h::krb5_keyusage {
    /* Of course, these are the wrong way around in the spec. */
    return if (*ctx).initiate ^ (make_checksum_0 == 0) as i32 != 0 {
        25i32
    } else {
        23i32
    };
}

unsafe extern "C" fn verify_checksum(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut send_alert_out: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut mech = (*ctx).negoex_mechs.tqh_first;
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::verify_message;
    let mut iov: [crate::krb5_h::krb5_crypto_iov; 3] = [crate::krb5_h::krb5_crypto_iov {
        flags: 0,
        data: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
    }; 3];
    let mut usage = verify_keyusage(ctx, 0i32);
    let mut valid: crate::krb5_h::krb5_boolean = 0;
    *send_alert_out = 0i32;
    if !mech.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"mech != NULL\x00" as *const u8 as *const i8,
                      b"negoex_ctx.c\x00" as *const u8 as *const i8,
                      440u32,
                      (*::std::mem::transmute::<&[u8; 114],
                                                &[i8; 114]>(b"OM_uint32 verify_checksum(OM_uint32 *, spnego_gss_ctx_id_t, struct negoex_message *, size_t, gss_buffer_t, int *)\x00")).as_ptr());
    }
    /* The other party may not be ready to send a verify token yet, or (in the
     * first initiator step) may send one for a mechanism we don't support. */
    msg = crate::src::spnego::negoex_util::negoex_locate_verify_message(messages, nmessages);
    if msg.is_null()
        || !(crate::stdlib::memcmp(
            (*msg).scheme.as_mut_ptr() as *const libc::c_void,
            (*mech).scheme.as_mut_ptr() as *const libc::c_void,
            16usize,
        ) == 0i32)
    {
        return 0u32;
    }
    /* A recoverable error may cause us to be unable to verify a token from the
     * other party.  In this case we should send an alert. */
    if (*mech).verify_key.enctype == 0i32 {
        *send_alert_out = 1i32;
        return 0u32;
    }
    /* Verify the checksum over the existing transcript and the portion of the
     * input token leading up to the verify message. */
    if !input_token.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"input_token != NULL\x00" as *const u8 as
                          *const i8,
                      b"negoex_ctx.c\x00" as *const u8 as *const i8,
                      457u32,
                      (*::std::mem::transmute::<&[u8; 114],
                                                &[i8; 114]>(b"OM_uint32 verify_checksum(OM_uint32 *, spnego_gss_ctx_id_t, struct negoex_message *, size_t, gss_buffer_t, int *)\x00")).as_ptr());
    }
    iov[0usize].flags = 2i32;
    iov[0usize].data = make_data(
        (*ctx).negoex_transcript.data,
        (*ctx).negoex_transcript.len as u32,
    );
    iov[1usize].flags = 2i32;
    iov[1usize].data = make_data((*input_token).value, (*msg).offset_in_token as u32);
    iov[2usize].flags = 6i32;
    iov[2usize].data = make_data((*msg).cksum as *mut libc::c_void, (*msg).cksum_len as u32);
    ret = crate::krb5_h::krb5_c_verify_checksum_iov(
        (*ctx).kctx,
        (*msg).cksum_type as crate::krb5_h::krb5_cksumtype,
        &mut (*mech).verify_key,
        usage,
        iov.as_mut_ptr(),
        3usize,
        &mut valid,
    );
    if ret != 0 {
        *minor = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    if valid == 0
        || crate::krb5_h::krb5_c_is_keyed_cksum((*msg).cksum_type as crate::krb5_h::krb5_cksumtype)
            == 0
    {
        *minor = 0x20000016u32;
        return (6u32) << 16i32;
    }
    (*mech).verified_checksum = 1i32;
    return 0u32;
}

unsafe extern "C" fn make_checksum(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut d = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut usage = verify_keyusage(ctx, 1i32);
    let mut cksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut mech = (*ctx).negoex_mechs.tqh_first;
    if !mech.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"mech != NULL\x00" as *const u8 as *const i8,
            b"negoex_ctx.c\x00" as *const u8 as *const i8,
            490u32,
            (*::std::mem::transmute::<&[u8; 58], &[i8; 58]>(
                b"OM_uint32 make_checksum(OM_uint32 *, spnego_gss_ctx_id_t)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*mech).key.enctype == 0i32 {
        if (*mech).complete != 0 {
            *minor = 0x20000014u32;
            return (16u32) << 16i32;
        } else {
            return 0u32;
        }
    }
    d = make_data(
        (*ctx).negoex_transcript.data,
        (*ctx).negoex_transcript.len as u32,
    );
    ret = crate::krb5_h::krb5_c_make_checksum(
        (*ctx).kctx,
        0i32,
        &mut (*mech).key,
        usage,
        &mut d,
        &mut cksum,
    );
    if ret != 0 {
        *minor = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    crate::src::spnego::negoex_util::negoex_add_verify_message(
        ctx,
        (*mech).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
        cksum.checksum_type as crate::stdlib::uint32_t,
        cksum.contents,
        cksum.length,
    );
    (*mech).sent_checksum = 1i32;
    crate::krb5_h::krb5_free_checksum_contents((*ctx).kctx, &mut cksum);
    return 0u32;
}
/* If the other side sent a VERIFY_NO_KEY pulse alert, clear the checksum state
 * on the mechanism so that we send another VERIFY message. */

unsafe extern "C" fn process_alerts(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stdlib::uint32_t,
) {
    let mut msg = 0 as *mut crate::gssapiP_negoex_h::alert_message;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    msg = crate::src::spnego::negoex_util::negoex_locate_alert_message(
        messages,
        nmessages as crate::stddef_h::size_t,
    );
    if !msg.is_null() && (*msg).verify_no_key != 0 {
        mech = crate::src::spnego::negoex_util::negoex_locate_auth_scheme(
            ctx,
            (*msg).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
        );
        if !mech.is_null() {
            (*mech).sent_checksum = 0i32;
            crate::krb5_h::krb5_free_keyblock_contents(
                0 as crate::krb5_h::krb5_context,
                &mut (*mech).key,
            );
            crate::krb5_h::krb5_free_keyblock_contents(
                0 as crate::krb5_h::krb5_context,
                &mut (*mech).verify_key,
            );
        }
    };
}

unsafe extern "C" fn make_output_token(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut mech_output_token: crate::gssapi_h::gss_buffer_t,
    mut send_alert: i32,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut type_0 = crate::gssapiP_negoex_h::INITIATOR_NEGO;
    let mut old_transcript_len = (*ctx).negoex_transcript.len;
    (*output_token).length = 0usize;
    (*output_token).value = 0 as *mut libc::c_void;
    /* If the mech is complete and we previously sent a checksum, we just
     * processed the last leg and don't need to send another token. */
    if (*mech_output_token).length == 0usize && (*(*ctx).negoex_mechs.tqh_first).sent_checksum != 0
    {
        return 0u32;
    }
    if (*ctx).negoex_step == 1i32 {
        if (*ctx).initiate != 0 {
            major = emit_initiator_nego(minor, ctx)
        } else {
            major = emit_acceptor_nego(minor, ctx)
        }
        if major != 0u32 {
            return major;
        }
        type_0 = if (*ctx).initiate != 0 {
            crate::gssapiP_negoex_h::INITIATOR_META_DATA as i32
        } else {
            crate::gssapiP_negoex_h::ACCEPTOR_META_DATA as i32
        } as crate::gssapiP_negoex_h::message_type;
        mech = (*ctx).negoex_mechs.tqh_first;
        while !mech.is_null() {
            if (*mech).metadata.length > 0usize {
                crate::src::spnego::negoex_util::negoex_add_exchange_message(
                    ctx,
                    type_0,
                    (*mech).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
                    &mut (*mech).metadata,
                );
            }
            mech = (*mech).links.tqe_next
        }
    }
    mech = (*ctx).negoex_mechs.tqh_first;
    if (*mech_output_token).length > 0usize {
        type_0 = if (*ctx).initiate != 0 {
            crate::gssapiP_negoex_h::AP_REQUEST as i32
        } else {
            crate::gssapiP_negoex_h::CHALLENGE as i32
        } as crate::gssapiP_negoex_h::message_type;
        crate::src::spnego::negoex_util::negoex_add_exchange_message(
            ctx,
            type_0,
            (*mech).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
            mech_output_token,
        );
    }
    if send_alert != 0 {
        crate::src::spnego::negoex_util::negoex_add_verify_no_key_alert(
            ctx,
            (*mech).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
        );
    }
    /* Try to add a VERIFY message if we haven't already done so. */
    if (*mech).sent_checksum == 0 {
        major = make_checksum(minor, ctx);
        if major != 0u32 {
            return major;
        }
    }
    if (*ctx).negoex_transcript.data.is_null() {
        *minor = 12u32;
        return (13u32) << 16i32;
    }
    /* Copy what we added to the transcript into the output token. */
    (*output_token).length = (*ctx)
        .negoex_transcript
        .len
        .wrapping_sub(old_transcript_len);
    (*output_token).value = gssalloc_malloc((*output_token).length);
    if (*output_token).value.is_null() {
        *minor = 12u32;
        return (13u32) << 16i32;
    }
    crate::stdlib::memcpy(
        (*output_token).value,
        ((*ctx).negoex_transcript.data as *mut crate::stdlib::uint8_t)
            .offset(old_transcript_len as isize) as *const libc::c_void,
        (*output_token).length,
    );
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_init(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mech_output_token = {
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: 0usize,
            value: 0 as *mut libc::c_void,
        };
        init
    };
    let mut messages = 0 as *mut crate::gssapiP_negoex_h::negoex_message;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut nmessages = 0usize;
    let mut send_alert = 0i32;
    if (*ctx).negoex_step == 0i32 && !input_token.is_null() && (*input_token).length != 0usize {
        return (9u32) << 16i32;
    }
    major = crate::src::spnego::negoex_util::negoex_prep_context_for_negoex(minor, ctx);
    if !(major != 0u32) {
        (*ctx).negoex_step += 1;
        if !input_token.is_null() && (*input_token).length > 0usize {
            major = crate::src::spnego::negoex_util::negoex_parse_token(
                minor,
                ctx,
                input_token as crate::gssapi_h::gss_const_buffer_t,
                &mut messages,
                &mut nmessages,
            );
            if major != 0u32 {
                current_block = 7071601115425651498;
            } else {
                current_block = 7746791466490516765;
            }
        } else {
            current_block = 7746791466490516765;
        }
        match current_block {
            7071601115425651498 => {}
            _ => {
                process_alerts(ctx, messages, nmessages as crate::stdlib::uint32_t);
                if (*ctx).negoex_step == 1i32 {
                    /* Choose a random conversation ID. */
                    major = crate::src::spnego::negoex_util::negoex_random(
                        minor,
                        ctx,
                        (*ctx).negoex_conv_id.as_mut_ptr(),
                        16usize,
                    );
                    if major != 0u32 {
                        current_block = 7071601115425651498;
                    } else {
                        /* Query each mech for its metadata (this may prune the mech list). */
                        query_meta_data(ctx, cred, target_name, req_flags);
                        current_block = 5783071609795492627;
                    }
                } else if (*ctx).negoex_step == 2i32 {
                    /* See if the mech processed the optimistic token. */
                    check_optimistic_result(ctx, messages, nmessages);
                    /* Pass the acceptor metadata to each mech to prune the list. */
                    exchange_meta_data(ctx, cred, target_name, req_flags, messages, nmessages);
                    /* Process the ACCEPTOR_NEGO message. */
                    major = process_acceptor_nego(minor, ctx, messages, nmessages);
                    if major != 0u32 {
                        current_block = 7071601115425651498;
                    } else {
                        current_block = 5783071609795492627;
                    }
                } else {
                    current_block = 5783071609795492627;
                }
                match current_block {
                    7071601115425651498 => {}
                    _ => {
                        /* Process the input token and/or produce an output token.  This may prune
                         * the mech list, but on success there will be at least one mech entry. */
                        major = mech_init(
                            minor,
                            ctx,
                            cred,
                            target_name,
                            req_flags,
                            time_req,
                            messages,
                            nmessages,
                            &mut mech_output_token,
                            time_rec,
                        );
                        if !(major != 0u32) {
                            if !(*ctx).negoex_mechs.tqh_first.is_null() {
                            } else {
                                crate::stdlib::__assert_fail(b"!K5_TAILQ_EMPTY(&ctx->negoex_mechs)\x00"
                                                  as *const u8 as
                                                  *const i8,
                                              b"negoex_ctx.c\x00" as *const u8
                                                  as *const i8,
                                              669u32,
                                              (*::std::mem::transmute::<&[u8; 146],
                                                                        &[i8; 146]>(b"OM_uint32 negoex_init(OM_uint32 *, spnego_gss_ctx_id_t, gss_cred_id_t, gss_name_t, OM_uint32, OM_uint32, gss_buffer_t, gss_buffer_t, OM_uint32 *)\x00")).as_ptr());
                            }
                            /* At this point in step 2 we have performed the metadata exchange and
                             * chosen a mech we can use, so discard any fallback mech entries. */
                            if (*ctx).negoex_step == 2i32 {
                                crate::src::spnego::negoex_util::negoex_select_auth_mech(
                                    ctx,
                                    (*ctx).negoex_mechs.tqh_first,
                                );
                            }
                            major = verify_checksum(
                                minor,
                                ctx,
                                messages,
                                nmessages,
                                input_token,
                                &mut send_alert,
                            );
                            if !(major != 0u32) {
                                if !input_token.is_null() {
                                    crate::k5_buf_h::k5_buf_add_len(
                                        &mut (*ctx).negoex_transcript,
                                        (*input_token).value,
                                        (*input_token).length,
                                    );
                                }
                                major = make_output_token(
                                    minor,
                                    ctx,
                                    &mut mech_output_token,
                                    send_alert,
                                    output_token,
                                );
                                if !(major != 0u32) {
                                    mech = (*ctx).negoex_mechs.tqh_first;
                                    major =
                                        if (*mech).complete != 0 && (*mech).verified_checksum != 0 {
                                            0i32
                                        } else {
                                            (1i32) << 0i32 + 0i32
                                        }
                                            as crate::gssapi_h::OM_uint32
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    crate::stdlib::free(messages as *mut libc::c_void);
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut mech_output_token);
    crate::src::spnego::negoex_util::negoex_prep_context_for_spnego(ctx);
    return major;
}
/* negoex_util.c */
/* negoex_ctx.c */
#[no_mangle]

pub unsafe extern "C" fn negoex_accept(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut cred: crate::gssapi_h::gss_cred_id_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut mech_output_token = {
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: 0usize,
            value: 0 as *mut libc::c_void,
        };
        init
    };
    let mut messages = 0 as *mut crate::gssapiP_negoex_h::negoex_message;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut nmessages: crate::stddef_h::size_t = 0;
    let mut send_alert = 0i32;
    if input_token.is_null() || (*input_token).length == 0usize {
        major = (9u32) << 16i32
    } else {
        major = crate::src::spnego::negoex_util::negoex_prep_context_for_negoex(minor, ctx);
        if !(major != 0u32) {
            (*ctx).negoex_step += 1;
            major = crate::src::spnego::negoex_util::negoex_parse_token(
                minor,
                ctx,
                input_token as crate::gssapi_h::gss_const_buffer_t,
                &mut messages,
                &mut nmessages,
            );
            if !(major != 0u32) {
                process_alerts(ctx, messages, nmessages as crate::stdlib::uint32_t);
                if (*ctx).negoex_step == 1i32 {
                    /* Read the INITIATOR_NEGO message to prune the candidate mech list. */
                    major = process_initiator_nego(minor, ctx, messages, nmessages);
                    if major != 0u32 {
                        current_block = 18014961938290295065;
                    } else {
                        /*
                         * Pass the initiator metadata to each mech to prune the list, and
                         * query each mech for its acceptor metadata (which may also prune the
                         * list).
                         */
                        exchange_meta_data(
                            ctx,
                            cred,
                            0 as crate::gssapi_h::gss_name_t,
                            0u32,
                            messages,
                            nmessages,
                        );
                        query_meta_data(ctx, cred, 0 as crate::gssapi_h::gss_name_t, 0u32);
                        if (*ctx).negoex_mechs.tqh_first.is_null() {
                            *minor = 0x20000013u32;
                            major = (13u32) << 16i32;
                            current_block = 18014961938290295065;
                        } else {
                            current_block = 4808432441040389987;
                        }
                    }
                } else {
                    current_block = 4808432441040389987;
                }
                match current_block {
                    18014961938290295065 => {}
                    _ => {
                        /*
                         * Process the input token and possibly produce an output token.  This may
                         * prune the list to a single mech.  Continue on error if an output token
                         * is generated, so that we send the token to the initiator.
                         */
                        major = mech_accept(
                            minor,
                            ctx,
                            cred,
                            messages,
                            nmessages,
                            &mut mech_output_token,
                            time_rec,
                        );
                        if !(major != 0u32 && mech_output_token.length == 0usize) {
                            if major == 0u32 {
                                major = verify_checksum(
                                    minor,
                                    ctx,
                                    messages,
                                    nmessages,
                                    input_token,
                                    &mut send_alert,
                                );
                                if major != 0u32 {
                                    current_block = 18014961938290295065;
                                } else {
                                    current_block = 18386322304582297246;
                                }
                            } else {
                                current_block = 18386322304582297246;
                            }
                            match current_block {
                                18014961938290295065 => {}
                                _ => {
                                    crate::k5_buf_h::k5_buf_add_len(
                                        &mut (*ctx).negoex_transcript,
                                        (*input_token).value,
                                        (*input_token).length,
                                    );
                                    major = make_output_token(
                                        minor,
                                        ctx,
                                        &mut mech_output_token,
                                        send_alert,
                                        output_token,
                                    );
                                    if !(major != 0u32) {
                                        mech = (*ctx).negoex_mechs.tqh_first;
                                        major = if (*mech).complete != 0
                                            && (*mech).verified_checksum != 0
                                        {
                                            0i32
                                        } else {
                                            (1i32) << 0i32 + 0i32
                                        }
                                            as crate::gssapi_h::OM_uint32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    crate::stdlib::free(messages as *mut libc::c_void);
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut mech_output_token);
    crate::src::spnego::negoex_util::negoex_prep_context_for_spnego(ctx);
    return major;
}
