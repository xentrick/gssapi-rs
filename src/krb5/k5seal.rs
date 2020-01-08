use ::libc;

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn store_32_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = __bswap_32(val);
    }
    #[inline]

    pub unsafe extern "C" fn store_16_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_5)).i = val as crate::stdlib::uint16_t;
    }
    use crate::src::krb5::k5seal::byteswap_h::__bswap_32;

    /* K5_PLATFORM_H */
}

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_32(
        mut __bsx: crate::stdlib::__uint32_t,
    ) -> crate::stdlib::__uint32_t {
        return (__bsx & 0xff000000u32) >> 24i32
            | (__bsx & 0xff0000u32) >> 8i32
            | (__bsx & 0xff00u32) << 8i32
            | (__bsx & 0xffu32) << 24i32;
    }
}

pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]

    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        crate::stdlib::free(value);
    }
    #[inline]

    pub unsafe extern "C" fn gssalloc_malloc(
        mut size: crate::stddef_h::size_t,
    ) -> *mut libc::c_void {
        return crate::stdlib::malloc(size);
    }
}
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_platform_h::C2RustUnnamed_5;
pub use crate::k5_platform_h::C2RustUnnamed_6;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_checksum;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_enc_data;
pub use crate::krb5_h::_krb5_enc_tkt_part;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_ticket;
pub use crate::krb5_h::_krb5_ticket_times;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_krb5_transited;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_address;
pub use crate::krb5_h::krb5_addrtype;
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_c_checksum_length;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_pointer;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_free_keyblock;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_key_keyblock;
pub use crate::krb5_h::krb5_k_make_checksum;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_ui_4;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::k5seal::k5_platform_h::store_16_le;
pub use crate::src::krb5::k5seal::k5_platform_h::store_32_be;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;

pub use crate::authdata_plugin_h::authdata_client_copy_proc;
pub use crate::authdata_plugin_h::authdata_client_delete_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_export_authdata_proc;
pub use crate::authdata_plugin_h::authdata_client_export_internal_proc;
pub use crate::authdata_plugin_h::authdata_client_externalize_proc;
pub use crate::authdata_plugin_h::authdata_client_free_internal_proc;
pub use crate::authdata_plugin_h::authdata_client_get_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_get_attribute_types_proc;
pub use crate::authdata_plugin_h::authdata_client_import_authdata_proc;
pub use crate::authdata_plugin_h::authdata_client_internalize_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_fini_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_flags_proc;
pub use crate::authdata_plugin_h::authdata_client_plugin_init_proc;
pub use crate::authdata_plugin_h::authdata_client_request_fini_proc;
pub use crate::authdata_plugin_h::authdata_client_request_init_proc;
pub use crate::authdata_plugin_h::authdata_client_set_attribute_proc;
pub use crate::authdata_plugin_h::authdata_client_size_proc;
pub use crate::authdata_plugin_h::authdata_client_verify_proc;
pub use crate::authdata_plugin_h::krb5plugin_authdata_client_ftable_v0;
pub use crate::gssapiP_generic_h::g_seqnum_state;
pub use crate::gssapiP_generic_h::g_seqnum_state_st;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapiP_krb5_h::seal_alg;
pub use crate::gssapiP_krb5_h::sgn_alg;
pub use crate::gssapiP_krb5_h::SEAL_ALG_DES3KD;
pub use crate::gssapiP_krb5_h::SEAL_ALG_MICROSOFT_RC4;
pub use crate::gssapiP_krb5_h::SEAL_ALG_NONE;
pub use crate::gssapiP_krb5_h::SGN_ALG_HMAC_MD5;
pub use crate::gssapiP_krb5_h::SGN_ALG_HMAC_SHA1_DES3_KD;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_token::gssint_g_make_token_header;
pub use crate::src::generic::util_token::gssint_g_token_size;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::k5seal::byteswap_h::__bswap_32;
pub use crate::src::krb5::k5seal::gssapi_alloc_h::gssalloc_free;
pub use crate::src::krb5::k5seal::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::krb5::k5sealv3::gss_krb5int_make_seal_token_v3;
pub use crate::src::krb5::util_crypt::kg_arcfour_docrypt;
pub use crate::src::krb5::util_crypt::kg_confounder_size;
pub use crate::src::krb5::util_crypt::kg_encrypt;
pub use crate::src::krb5::util_crypt::kg_make_confounder;
pub use crate::src::krb5::util_seqnum::kg_make_seq_num;

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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */

unsafe extern "C" fn make_seal_token_v1(
    mut context: crate::krb5_h::krb5_context,
    mut enc: crate::krb5_h::krb5_key,
    mut seq: crate::krb5_h::krb5_key,
    mut seqnum: *mut crate::stdlib::uint64_t,
    mut direction: i32,
    mut text: crate::gssapi_h::gss_buffer_t,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut signalg: i32,
    mut cksum_size: crate::stddef_h::size_t,
    mut sealalg: i32,
    mut do_encrypt: i32,
    mut toktype: i32,
    mut oid: crate::gssapi_h::gss_OID,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut sumlen: crate::stddef_h::size_t = 0;
    let mut data_ptr = 0 as *mut i8;
    let mut plaind = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut md5cksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    /* msglen contains the message length
     * we are signing/encrypting.  tmsglen
     * contains the length of the message
     * we plan to write out to the token.
     * tlen is the length of the token
     * including header. */
    let mut conflen = 0u32;
    let mut tmsglen: u32 = 0;
    let mut tlen: u32 = 0;
    let mut msglen: u32 = 0;
    let mut t = 0 as *mut u8;
    let mut ptr = 0 as *mut u8;
    let mut plain = 0 as *mut u8;
    let mut pad: u8 = 0;
    let mut sign_usage = 23i32;
    if do_encrypt == 0 || toktype == 0x201i32 {
    } else {
        crate::stdlib::__assert_fail(b"(!do_encrypt) || (toktype == KG_TOK_SEAL_MSG)\x00" as
                          *const u8 as *const i8,
                      b"k5seal.c\x00" as *const u8 as *const i8,
                      87u32,
                      (*::std::mem::transmute::<&[u8; 151],
                                                &[i8; 151]>(b"krb5_error_code make_seal_token_v1(krb5_context, krb5_key, krb5_key, uint64_t *, int, gss_buffer_t, gss_buffer_t, int, size_t, int, int, int, gss_OID)\x00")).as_ptr());
    }
    /* create the token buffer */
    /* Do we need confounder? */
    if do_encrypt != 0 || toktype == 0x201i32 {
        conflen = crate::src::krb5::util_crypt::kg_confounder_size(context, (*enc).keyblock.enctype)
            as u32
    } else {
        conflen = 0u32
    }
    if toktype == 0x201i32 {
        match sealalg {
            16 => {
                msglen = (conflen as usize)
                    .wrapping_add((*text).length)
                    .wrapping_add(1usize) as u32;
                pad = 1u8
            }
            _ => {
                /* XXX knows that des block size is 8 */
                msglen = ((conflen as usize)
                    .wrapping_add((*text).length)
                    .wrapping_add(8usize)
                    & !(7i32) as usize) as u32;
                pad = (8usize).wrapping_sub((*text).length.wrapping_rem(8usize)) as u8
            }
        }
        tmsglen = msglen
    } else {
        tmsglen = 0u32;
        msglen = (*text).length as u32;
        pad = 0u8
    }
    tlen = crate::src::generic::util_token::gssint_g_token_size(
        oid as *const crate::gssapi_h::gss_OID_desc,
        (14usize)
            .wrapping_add(cksum_size)
            .wrapping_add(tmsglen as usize) as u32,
    );
    t = gssalloc_malloc(tlen as crate::stddef_h::size_t) as *mut u8;
    if t.is_null() {
        return 12i32;
    }
    /* ** fill in the token */
    ptr = t;
    crate::src::generic::util_token::gssint_g_make_token_header(
        oid as *const crate::gssapi_h::gss_OID_desc,
        (14usize)
            .wrapping_add(cksum_size)
            .wrapping_add(tmsglen as usize) as u32,
        &mut ptr,
        toktype,
    );
    /* 0..1 SIGN_ALG */
    store_16_le(
        signalg as u32,
        &mut *ptr.offset(0isize) as *mut u8 as *mut libc::c_void,
    );
    /* 2..3 SEAL_ALG or Filler */
    if toktype == 0x201i32 && do_encrypt != 0 {
        store_16_le(
            sealalg as u32,
            &mut *ptr.offset(2isize) as *mut u8 as *mut libc::c_void,
        );
    } else {
        /* No seal */
        *ptr.offset(2isize) = 0xffu8;
        *ptr.offset(3isize) = 0xffu8
    }
    /* 4..5 Filler */
    *ptr.offset(4isize) = 0xffu8;
    *ptr.offset(5isize) = 0xffu8;
    /* pad the plaintext, encrypt if needed, and stick it in the token */
    /* initialize the the checksum */
    match signalg {
        4 => md5cksum.checksum_type = 0xci32,
        17 => {
            md5cksum.checksum_type = -(138i32);
            if toktype != 0x201i32 {
                sign_usage = 15i32
            }
        }
        _ => {
            crate::stdlib::abort();
        }
    }
    code = crate::krb5_h::krb5_c_checksum_length(context, md5cksum.checksum_type, &mut sumlen);
    if code != 0 {
        gssalloc_free(t as *mut libc::c_void);
        return code;
    }
    md5cksum.length = sumlen as u32;
    plain = crate::stdlib::malloc(if msglen != 0 { msglen } else { 1u32 } as usize) as *mut u8;
    if plain.is_null() {
        gssalloc_free(t as *mut libc::c_void);
        return 12i32;
    }
    if conflen != 0 {
        code = crate::src::krb5::util_crypt::kg_make_confounder(
            context,
            (*enc).keyblock.enctype,
            plain,
        );
        if code != 0 {
            crate::stdlib::free(plain as *mut libc::c_void);
            gssalloc_free(t as *mut libc::c_void);
            return code;
        }
    }
    crate::stdlib::memcpy(
        plain.offset(conflen as isize) as *mut libc::c_void,
        (*text).value,
        (*text).length,
    );
    if pad != 0 {
        crate::stdlib::memset(
            plain
                .offset(conflen as isize)
                .offset((*text).length as isize) as *mut libc::c_void,
            pad as i32,
            pad as usize,
        );
    }
    /* compute the checksum */
    /* 8 = head of token body as specified by mech spec */
    data_ptr = crate::stdlib::malloc((8u32).wrapping_add(msglen) as usize) as *mut i8;
    if data_ptr.is_null() {
        crate::stdlib::free(plain as *mut libc::c_void);
        gssalloc_free(t as *mut libc::c_void);
        return 12i32;
    }
    crate::stdlib::memcpy(
        data_ptr as *mut libc::c_void,
        ptr.offset(-(2isize)) as *const libc::c_void,
        8usize,
    );
    crate::stdlib::memcpy(
        data_ptr.offset(8isize) as *mut libc::c_void,
        plain as *const libc::c_void,
        msglen as usize,
    );
    plaind.length = (8u32).wrapping_add(msglen);
    plaind.data = data_ptr;
    code = crate::krb5_h::krb5_k_make_checksum(
        context,
        md5cksum.checksum_type,
        seq,
        sign_usage,
        &mut plaind,
        &mut md5cksum,
    );
    crate::stdlib::free(data_ptr as *mut libc::c_void);
    if code != 0 {
        crate::stdlib::free(plain as *mut libc::c_void);
        gssalloc_free(t as *mut libc::c_void);
        return code;
    }
    match signalg {
        4 => {
            /*
             * Using key derivation, the call to krb5_c_make_checksum
             * already dealt with encrypting.
             */
            if md5cksum.length as usize != cksum_size {
                crate::stdlib::abort();
            }
            crate::stdlib::memcpy(
                ptr.offset(14isize) as *mut libc::c_void,
                md5cksum.contents as *const libc::c_void,
                md5cksum.length as usize,
            );
        }
        17 => {
            crate::stdlib::memcpy(
                ptr.offset(14isize) as *mut libc::c_void,
                md5cksum.contents as *const libc::c_void,
                cksum_size,
            );
        }
        _ => {}
    }
    crate::krb5_h::krb5_free_checksum_contents(context, &mut md5cksum);
    /* create the seq_num */
    code = crate::src::krb5::util_seqnum::kg_make_seq_num(
        context,
        seq,
        if direction != 0 { 0i32 } else { 0xffi32 },
        *seqnum as crate::krb5_h::krb5_ui_4,
        ptr.offset(14isize),
        ptr.offset(6isize),
    );
    if code != 0 {
        crate::stdlib::free(plain as *mut libc::c_void);
        gssalloc_free(t as *mut libc::c_void);
        return code;
    }
    if do_encrypt != 0 {
        match sealalg {
            16 => {
                let mut bigend_seqnum: [u8; 4] = [0; 4];
                let mut enc_key = 0 as *mut crate::krb5_h::krb5_keyblock;
                let mut i: i32 = 0;
                store_32_be(
                    *seqnum as u32,
                    bigend_seqnum.as_mut_ptr() as *mut libc::c_void,
                );
                code = crate::krb5_h::krb5_k_key_keyblock(context, enc, &mut enc_key);
                if code != 0 {
                    crate::stdlib::free(plain as *mut libc::c_void);
                    gssalloc_free(t as *mut libc::c_void);
                    return code;
                }
                if (*enc_key).length == 16u32 {
                } else {
                    crate::stdlib::__assert_fail(b"enc_key->length == 16\x00" as *const u8 as
                                      *const i8,
                                  b"k5seal.c\x00" as *const u8 as
                                      *const i8,
                                  240u32,
                                  (*::std::mem::transmute::<&[u8; 151],
                                                            &[i8; 151]>(b"krb5_error_code make_seal_token_v1(krb5_context, krb5_key, krb5_key, uint64_t *, int, gss_buffer_t, gss_buffer_t, int, size_t, int, int, int, gss_OID)\x00")).as_ptr());
                }
                i = 0i32;
                while i <= 15i32 {
                    let ref mut fresh0 = *((*enc_key).contents as *mut i8).offset(i as isize);
                    *fresh0 = (*fresh0 as i32 ^ 0xf0i32) as i8;
                    i += 1
                }
                code = crate::src::krb5::util_crypt::kg_arcfour_docrypt(
                    enc_key,
                    0i32,
                    bigend_seqnum.as_mut_ptr(),
                    4usize,
                    plain,
                    tmsglen as crate::stddef_h::size_t,
                    ptr.offset(14isize).offset(cksum_size as isize),
                );
                crate::krb5_h::krb5_free_keyblock(context, enc_key);
                if code != 0 {
                    crate::stdlib::free(plain as *mut libc::c_void);
                    gssalloc_free(t as *mut libc::c_void);
                    return code;
                }
            }
            _ => {
                code = crate::src::krb5::util_crypt::kg_encrypt(
                    context,
                    enc,
                    22i32,
                    0 as *mut libc::c_void,
                    plain as crate::krb5_h::krb5_const_pointer,
                    ptr.offset(cksum_size as isize).offset(14isize) as crate::krb5_h::krb5_pointer,
                    tmsglen,
                );
                if code != 0 {
                    crate::stdlib::free(plain as *mut libc::c_void);
                    gssalloc_free(t as *mut libc::c_void);
                    return code;
                }
            }
        }
    } else if tmsglen != 0 {
        crate::stdlib::memcpy(
            ptr.offset(14isize).offset(cksum_size as isize) as *mut libc::c_void,
            plain as *const libc::c_void,
            tmsglen as usize,
        );
    }
    crate::stdlib::free(plain as *mut libc::c_void);
    /* that's it.  return the token */
    *seqnum = (*seqnum).wrapping_add(1);
    *seqnum &= 0xffffffff as usize;
    (*token).length = tlen as crate::stddef_h::size_t;
    (*token).value = t as *mut libc::c_void;
    return 0i32;
}
/* if signonly is true, ignore conf_req, conf_state,
and do not encode the ENC_TYPE, MSG_LENGTH, or MSG_TEXT fields */
#[no_mangle]

pub unsafe extern "C" fn kg_seal(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    (*output_message_buffer).length = 0usize;
    (*output_message_buffer).value = 0 as *mut libc::c_void;
    /* Only default qop or matching established cryptosystem is allowed.

    There are NO EXTENSIONS to this set for AES and friends!  The
    new spec says "just use 0".  The old spec plus extensions would
    actually allow for certain non-zero values.  Fix this to handle
    them later.  */
    if qop_req != 0u32 {
        *minor_status = -(2045022968 as isize) as crate::gssapi_h::OM_uint32; /* XXX */
        return (14u32) << 16i32;
    }
    ctx = context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if (*ctx).terminated() as i32 != 0 || (*ctx).established() == 0 {
        *minor_status = 39756039u32;
        return (8u32) << 16i32;
    }
    context = (*ctx).k5_context;
    match (*ctx).proto {
        0 => {
            code = make_seal_token_v1(
                context,
                (*ctx).enc,
                (*ctx).seq,
                &mut (*ctx).seq_send,
                (*ctx).initiate() as i32,
                input_message_buffer,
                output_message_buffer,
                (*ctx).signalg,
                (*ctx).cksum_size,
                (*ctx).sealalg,
                conf_req_flag,
                toktype,
                (*ctx).mech_used,
            )
        }
        1 => {
            code = crate::src::krb5::k5sealv3::gss_krb5int_make_seal_token_v3(
                context,
                ctx,
                input_message_buffer as *const crate::gssapi_h::gss_buffer_desc,
                output_message_buffer,
                conf_req_flag,
                toktype,
            )
        }
        _ => code = -(2045022968 as isize) as crate::krb5_h::krb5_error_code,
    }
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
        return (13u32) << 16i32;
    }
    if !conf_state.is_null() {
        *conf_state = conf_req_flag
    }
    *minor_status = 0u32;
    return 0u32;
}
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_wrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    return kg_seal(
        minor_status,
        context_handle,
        conf_req_flag,
        qop_req,
        input_message_buffer,
        conf_state,
        output_message_buffer,
        0x201i32,
    );
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
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_get_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut message_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    return kg_seal(
        minor_status,
        context_handle,
        0i32,
        qop_req,
        message_buffer,
        0 as *mut i32,
        message_token,
        0x101i32,
    );
}
