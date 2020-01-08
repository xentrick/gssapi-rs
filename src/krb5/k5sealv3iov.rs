use ::libc;

pub mod k5_platform_h {

    /* Do it in macro form so we get the file/line of the invocation if
    the assertion fails.  */
    /* forward declaration for use in initializer */
    /* so ';' following macro use won't get error */
    /* This should be called in finalization only, so we shouldn't have
    multiple active threads mucking around in our library at this
    point.  So ignore the once_t object and just look at the flag.

    XXX Could we have problems with memory coherence between processors
    if we don't invoke mutex/once routines?  Probably not, the
    application code should already be coordinating things such that
    the library code is not in use by this point, and memory
    synchronization will be needed there.  */
    /* If we're using gcc, if the C++ support works, the compiler should
    build executables and shared libraries that support the use of
    static constructors and destructors.  The C compiler supports a
    function attribute that makes use of the same facility as C++.

    XXX How do we know if the C++ support actually works?  */
    /* Read and write integer values as (unaligned) octet strings in
    specific byte orders.  Add per-platform optimizations as
    needed.  */
    /* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
    it.  If both are defined, then BYTE_ORDER should be defined and
    match one of them.  Try those symbols, then try again with an
    underscore prefix.  */
    /* Optimize for GCC on platforms with known byte orders.

    GCC's packed structures can be written to with any alignment; the
    compiler will use byte operations, unaligned-word operations, or
    normal memory ops as appropriate for the architecture.

    This assumes the availability of uint##_t types, which should work
    on most of our platforms except Windows, where we're not using
    GCC.  */
    /* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
    with the indicated numbers of bits.

    Linux: byteswap.h, bswap_16 etc.
    Solaris 10: none
    macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
    NetBSD: sys/bswap.h, bswap16 etc.  */
    /* Note that on Windows at least this file can be included from C++
    source, so casts *from* void* are required.  */
    #[inline]

    pub unsafe extern "C" fn store_16_be(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_5)).i =
            __bswap_16(val as crate::stdlib::__uint16_t);
    }
    #[inline]

    pub unsafe extern "C" fn store_64_be(
        mut val: crate::stdlib::uint64_t,
        mut vp: *mut libc::c_void,
    ) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_4)).i = __bswap_64(val);
    }
    #[inline]

    pub unsafe extern "C" fn load_16_be(mut cvp: *const libc::c_void) -> u16 {
        let mut p = cvp as *const u8;
        return __bswap_16((*(p as *const crate::k5_platform_h::C2RustUnnamed_5)).i);
    }
    #[inline]

    pub unsafe extern "C" fn load_64_be(mut cvp: *const libc::c_void) -> crate::stdlib::uint64_t {
        let mut p = cvp as *const u8;
        return __bswap_64((*(p as *const crate::k5_platform_h::C2RustUnnamed_4)).i);
    }

    use crate::src::krb5::k5sealv3iov::byteswap_h::__bswap_16;
    use crate::src::krb5::k5sealv3iov::byteswap_h::__bswap_64;

    /* K5_PLATFORM_H */
}

pub mod byteswap_h {
    #[inline]

    pub unsafe extern "C" fn __bswap_64(
        mut __bsx: crate::stdlib::__uint64_t,
    ) -> crate::stdlib::__uint64_t {
        return ((__bsx as u64 & 0xff00000000000000u64) >> 56i32
            | (__bsx as u64 & 0xff000000000000u64) >> 40i32
            | (__bsx as u64 & 0xff0000000000u64) >> 24i32
            | (__bsx as u64 & 0xff00000000u64) >> 8i32
            | (__bsx as u64 & 0xff000000u64) << 8i32
            | (__bsx as u64 & 0xff0000u64) << 24i32
            | (__bsx as u64 & 0xff00u64) << 40i32
            | (__bsx as u64 & 0xffu64) << 56i32) as crate::stdlib::__uint64_t;
    }
    #[inline]

    pub unsafe extern "C" fn __bswap_16(
        mut __bsx: crate::stdlib::__uint16_t,
    ) -> crate::stdlib::__uint16_t {
        return (__bsx as i32 >> 8i32 & 0xffi32 | (__bsx as i32 & 0xffi32) << 8i32)
            as crate::stdlib::__uint16_t;
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
pub use crate::k5_platform_h::C2RustUnnamed_4;
pub use crate::k5_platform_h::C2RustUnnamed_5;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
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
pub use crate::krb5_h::krb5_c_block_size;
pub use crate::krb5_h::krb5_c_crypto_length;
pub use crate::krb5_h::krb5_c_padding_length;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_cryptotype;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_int32;
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
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::k5sealv3iov::k5_platform_h::load_16_be;
pub use crate::src::krb5::k5sealv3iov::k5_platform_h::load_64_be;
pub use crate::src::krb5::k5sealv3iov::k5_platform_h::store_16_be;
pub use crate::src::krb5::k5sealv3iov::k5_platform_h::store_64_be;
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
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
pub use crate::gssapi_ext_h::gss_iov_buffer_t;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_check;
pub use crate::src::krb5::k5sealv3iov::byteswap_h::__bswap_16;
pub use crate::src::krb5::k5sealv3iov::byteswap_h::__bswap_64;
pub use crate::src::krb5::util_cksum::kg_make_checksum_iov_v3;
pub use crate::src::krb5::util_cksum::kg_verify_checksum_iov_v3;
pub use crate::src::krb5::util_crypt::kg_allocate_iov;
pub use crate::src::krb5::util_crypt::kg_decrypt_iov;
pub use crate::src::krb5::util_crypt::kg_encrypt_iov;
pub use crate::src::krb5::util_crypt::kg_iov_msglen;
pub use crate::src::krb5::util_crypt::kg_locate_header_iov;
pub use crate::src::krb5::util_crypt::kg_locate_iov;
pub use crate::src::krb5::util_crypt::kg_release_iov;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/krb5/k5sealv3iov.c */
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
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_make_seal_token_v3_iov(
    mut context: crate::krb5_h::krb5_context,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut conf_req_flag: i32,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut toktype: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code = 0i32;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut padding = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut acceptor_flag: u8 = 0;
    let mut tok_id: u16 = 0;
    let mut outbuf = 0 as *mut u8;
    let mut tbuf = 0 as *mut u8;
    let mut key_usage: i32 = 0;
    let mut rrc = 0usize;
    let mut gss_headerlen: u32 = 0;
    let mut gss_trailerlen: u32 = 0;
    let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
    let mut cksumtype: crate::krb5_h::krb5_cksumtype = 0;
    let mut data_length: crate::stddef_h::size_t = 0;
    let mut assoc_data_length: crate::stddef_h::size_t = 0;
    if (*ctx).proto == 1i32 {
    } else {
        crate::stdlib::__assert_fail(b"ctx->proto == 1\x00" as *const u8 as
                          *const i8,
                      b"k5sealv3iov.c\x00" as *const u8 as
                          *const i8,
                      54u32,
                      (*::std::mem::transmute::<&[u8; 133],
                                                &[i8; 133]>(b"krb5_error_code gss_krb5int_make_seal_token_v3_iov(krb5_context, krb5_gss_ctx_id_rec *, int, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
    }
    acceptor_flag = if (*ctx).initiate() as i32 != 0 {
        0i32
    } else {
        0x1i32
    } as u8;
    key_usage = if toktype == 0x201i32 {
        if (*ctx).initiate() as i32 != 0 {
            24i32
        } else {
            22i32
        }
    } else if (*ctx).initiate() as i32 != 0 {
        25i32
    } else {
        23i32
    };
    if (*ctx).have_acceptor_subkey() != 0 {
        key = (*ctx).acceptor_subkey;
        cksumtype = (*ctx).acceptor_subkey_cksumtype
    } else {
        key = (*ctx).subkey;
        cksumtype = (*ctx).cksumtype
    }
    if !key.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"key != NULL\x00" as *const u8 as *const i8,
                      b"k5sealv3iov.c\x00" as *const u8 as
                          *const i8,
                      71u32,
                      (*::std::mem::transmute::<&[u8; 133],
                                                &[i8; 133]>(b"krb5_error_code gss_krb5int_make_seal_token_v3_iov(krb5_context, krb5_gss_ctx_id_rec *, int, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
    }
    if cksumtype != 0i32 {
    } else {
        crate::stdlib::__assert_fail(b"cksumtype != 0\x00" as *const u8 as
                          *const i8,
                      b"k5sealv3iov.c\x00" as *const u8 as
                          *const i8,
                      72u32,
                      (*::std::mem::transmute::<&[u8; 133],
                                                &[i8; 133]>(b"krb5_error_code gss_krb5int_make_seal_token_v3_iov(krb5_context, krb5_gss_ctx_id_rec *, int, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
    }
    crate::src::krb5::util_crypt::kg_iov_msglen(
        iov,
        iov_count,
        &mut data_length,
        &mut assoc_data_length,
    );
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if header.is_null() {
        return 22i32;
    }
    padding = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 9u32);
    if !padding.is_null() {
        (*padding).buffer.length = 0usize
    }
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    if toktype == 0x201i32 && conf_req_flag != 0 {
        let mut k5_headerlen: u32 = 0;
        let mut k5_trailerlen: u32 = 0;
        let mut k5_padlen: u32 = 0;
        let mut ec = 0usize;
        let mut conf_data_length = data_length.wrapping_sub(assoc_data_length);
        code = crate::krb5_h::krb5_c_crypto_length(
            context,
            (*key).keyblock.enctype,
            1i32,
            &mut k5_headerlen,
        );
        if code != 0i32 {
            current_block = 9040872671964311102;
        } else {
            code = crate::krb5_h::krb5_c_padding_length(
                context,
                (*key).keyblock.enctype,
                conf_data_length.wrapping_add(16usize),
                &mut k5_padlen,
            );
            if code != 0i32 {
                current_block = 9040872671964311102;
            } else {
                if k5_padlen == 0u32 && (*ctx).gss_flags & 0x1000u32 != 0 {
                    /* Windows rejects AEAD tokens with non-zero EC */
                    code =
                        crate::krb5_h::krb5_c_block_size(context, (*key).keyblock.enctype, &mut ec);
                    if code != 0i32 {
                        current_block = 9040872671964311102;
                    } else {
                        current_block = 10692455896603418738;
                    }
                } else {
                    ec = k5_padlen as crate::stddef_h::size_t;
                    current_block = 10692455896603418738;
                }
                match current_block {
                    9040872671964311102 => {}
                    _ => {
                        code = crate::krb5_h::krb5_c_crypto_length(
                            context,
                            (*key).keyblock.enctype,
                            5i32,
                            &mut k5_trailerlen,
                        );
                        if code != 0i32 {
                            current_block = 9040872671964311102;
                        } else {
                            gss_headerlen = (16u32).wrapping_add(k5_headerlen);
                            gss_trailerlen = ec
                                .wrapping_add(16usize)
                                .wrapping_add(k5_trailerlen as usize)
                                as u32;
                            if trailer.is_null() {
                                rrc = gss_trailerlen as crate::stddef_h::size_t;
                                /* Workaround for Windows bug where it rotates by EC + RRC */
                                if (*ctx).gss_flags & 0x1000u32 != 0 {
                                    rrc = (rrc).wrapping_sub(ec)
                                }
                                gss_headerlen = gss_headerlen.wrapping_add(gss_trailerlen)
                            }
                            if (*header).type_0 & 0x10000u32 != 0 {
                                code = crate::src::krb5::util_crypt::kg_allocate_iov(
                                    header,
                                    gss_headerlen as crate::stddef_h::size_t,
                                )
                            } else if (*header).buffer.length < gss_headerlen as usize {
                                code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
                            }
                            if code != 0i32 {
                                current_block = 9040872671964311102;
                            } else {
                                outbuf = (*header).buffer.value as *mut u8;
                                (*header).buffer.length = gss_headerlen as crate::stddef_h::size_t;
                                if !trailer.is_null() {
                                    if (*trailer).type_0 & 0x10000u32 != 0 {
                                        code = crate::src::krb5::util_crypt::kg_allocate_iov(
                                            trailer,
                                            gss_trailerlen as crate::stddef_h::size_t,
                                        )
                                    } else if (*trailer).buffer.length < gss_trailerlen as usize {
                                        code =
                                            -(1765328194 as isize) as crate::krb5_h::krb5_error_code
                                    }
                                    if code != 0i32 {
                                        current_block = 9040872671964311102;
                                    } else {
                                        (*trailer).buffer.length =
                                            gss_trailerlen as crate::stddef_h::size_t;
                                        current_block = 11441799814184323368;
                                    }
                                } else {
                                    current_block = 11441799814184323368;
                                }
                                match current_block {
                                    9040872671964311102 => {}
                                    _ => {
                                        /* TOK_ID */
                                        store_16_be(0x504u32, outbuf as *mut libc::c_void);
                                        /* flags */
                                        *outbuf.offset(2isize) = (acceptor_flag as i32
                                            | 0x2i32
                                            | (if (*ctx).have_acceptor_subkey() as i32 != 0 {
                                                0x4i32
                                            } else {
                                                0i32
                                            }))
                                            as u8;
                                        /* filler */
                                        *outbuf.offset(3isize) = 0xffu8;
                                        /* EC */
                                        store_16_be(
                                            ec as u32,
                                            outbuf.offset(4isize) as *mut libc::c_void,
                                        );
                                        /* RRC */
                                        store_16_be(
                                            0u32,
                                            outbuf.offset(6isize) as *mut libc::c_void,
                                        );
                                        store_64_be(
                                            (*ctx).seq_send,
                                            outbuf.offset(8isize) as *mut libc::c_void,
                                        );
                                        /* EC | copy of header to be encrypted, located in (possibly rotated) trailer */
                                        if trailer.is_null() {
                                            tbuf =
                                                ((*header).buffer.value as *mut u8).offset(16isize)
                                        } else {
                                            tbuf = (*trailer).buffer.value as *mut u8
                                        } /* Header */
                                        crate::stdlib::memset(
                                            tbuf as *mut libc::c_void,
                                            0xffi32,
                                            ec,
                                        );
                                        crate::stdlib::memcpy(
                                            tbuf.offset(ec as isize) as *mut libc::c_void,
                                            (*header).buffer.value,
                                            16usize,
                                        );
                                        code = crate::src::krb5::util_crypt::kg_encrypt_iov(
                                            context,
                                            (*ctx).proto,
                                            ((*ctx).gss_flags & 0x1000u32 != 0u32) as i32,
                                            ec,
                                            rrc,
                                            key,
                                            key_usage,
                                            0 as crate::krb5_h::krb5_pointer,
                                            iov,
                                            iov_count,
                                        );
                                        if code != 0i32 {
                                            current_block = 9040872671964311102;
                                        } else {
                                            /* RRC */
                                            store_16_be(
                                                rrc as u32,
                                                outbuf.offset(6isize) as *mut libc::c_void,
                                            );
                                            (*ctx).seq_send = (*ctx).seq_send.wrapping_add(1);
                                            current_block = 14652688882591975137;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        if toktype == 0x201i32 && conf_req_flag == 0 {
            tok_id = 0x504u16
        } else if toktype == 0x101i32 {
            tok_id = 0x404u16;
            trailer = 0 as crate::gssapi_ext_h::gss_iov_buffer_t
        } else if toktype == 0x102i32 {
            tok_id = 0x405u16
        } else {
            crate::stdlib::abort();
        }
        gss_headerlen = 16u32;
        code = crate::krb5_h::krb5_c_crypto_length(
            context,
            (*key).keyblock.enctype,
            6i32,
            &mut gss_trailerlen,
        );
        if code != 0i32 {
            current_block = 9040872671964311102;
        } else {
            if gss_trailerlen <= 0xffffu32 {
            } else {
                crate::stdlib::__assert_fail(b"gss_trailerlen <= 0xFFFF\x00" as *const u8 as
                                  *const i8,
                              b"k5sealv3iov.c\x00" as *const u8 as
                                  *const i8,
                              189u32,
                              (*::std::mem::transmute::<&[u8; 133],
                                                        &[i8; 133]>(b"krb5_error_code gss_krb5int_make_seal_token_v3_iov(krb5_context, krb5_gss_ctx_id_rec *, int, int *, gss_iov_buffer_desc *, int, int)\x00")).as_ptr());
            }
            if trailer.is_null() {
                rrc = gss_trailerlen as crate::stddef_h::size_t;
                gss_headerlen = gss_headerlen.wrapping_add(gss_trailerlen)
            }
            if (*header).type_0 & 0x10000u32 != 0 {
                code = crate::src::krb5::util_crypt::kg_allocate_iov(
                    header,
                    gss_headerlen as crate::stddef_h::size_t,
                )
            } else if (*header).buffer.length < gss_headerlen as usize {
                code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
            }
            if code != 0i32 {
                current_block = 9040872671964311102;
            } else {
                outbuf = (*header).buffer.value as *mut u8;
                (*header).buffer.length = gss_headerlen as crate::stddef_h::size_t;
                if !trailer.is_null() {
                    if (*trailer).type_0 & 0x10000u32 != 0 {
                        code = crate::src::krb5::util_crypt::kg_allocate_iov(
                            trailer,
                            gss_trailerlen as crate::stddef_h::size_t,
                        )
                    } else if (*trailer).buffer.length < gss_trailerlen as usize {
                        code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
                    }
                    if code != 0i32 {
                        current_block = 9040872671964311102;
                    } else {
                        (*trailer).buffer.length = gss_trailerlen as crate::stddef_h::size_t;
                        current_block = 2467484839200770573;
                    }
                } else {
                    current_block = 2467484839200770573;
                }
                match current_block {
                    9040872671964311102 => {}
                    _ => {
                        /* TOK_ID */
                        store_16_be(tok_id as u32, outbuf as *mut libc::c_void);
                        /* flags */
                        *outbuf.offset(2isize) = (acceptor_flag as i32
                            | (if (*ctx).have_acceptor_subkey() as i32 != 0 {
                                0x4i32
                            } else {
                                0i32
                            })) as u8;
                        /* filler */
                        *outbuf.offset(3isize) = 0xffu8;
                        if toktype == 0x201i32 {
                            /* Use 0 for checksum calculation, substitute
                             * checksum length later.
                             */
                            /* EC */
                            store_16_be(0u32, outbuf.offset(4isize) as *mut libc::c_void);
                            /* RRC */
                            store_16_be(0u32, outbuf.offset(6isize) as *mut libc::c_void);
                        } else {
                            /* MIC and DEL store 0xFF in EC and RRC */
                            store_16_be(0xffffu32, outbuf.offset(4isize) as *mut libc::c_void);
                            store_16_be(0xffffu32, outbuf.offset(6isize) as *mut libc::c_void);
                        }
                        store_64_be((*ctx).seq_send, outbuf.offset(8isize) as *mut libc::c_void);
                        code = crate::src::krb5::util_cksum::kg_make_checksum_iov_v3(
                            context, cksumtype, rrc, key, key_usage, iov, iov_count, toktype,
                        );
                        if code != 0i32 {
                            current_block = 9040872671964311102;
                        } else {
                            (*ctx).seq_send = (*ctx).seq_send.wrapping_add(1);
                            if toktype == 0x201i32 {
                                /* Fix up EC field */
                                store_16_be(
                                    gss_trailerlen,
                                    outbuf.offset(4isize) as *mut libc::c_void,
                                );
                                /* Fix up RRC field */
                                store_16_be(rrc as u32, outbuf.offset(6isize) as *mut libc::c_void);
                            }
                            current_block = 14652688882591975137;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        14652688882591975137 => {
            code = 0i32;
            if !conf_state.is_null() {
                *conf_state = conf_req_flag
            }
        }
        _ => {}
    }
    if code != 0i32 {
        crate::src::krb5::util_crypt::kg_release_iov(iov, iov_count);
    }
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn gss_krb5int_unseal_v3_iov(
    mut context: crate::krb5_h::krb5_context,
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut toktype: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut code: crate::gssapi_h::OM_uint32 = 0;
    let mut header = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut padding = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut trailer = 0 as *mut crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
    let mut acceptor_flag: u8 = 0;
    let mut ptr = 0 as *mut u8;
    let mut key_usage: i32 = 0;
    let mut rrc: crate::stddef_h::size_t = 0;
    let mut ec: crate::stddef_h::size_t = 0;
    let mut data_length: crate::stddef_h::size_t = 0;
    let mut assoc_data_length: crate::stddef_h::size_t = 0;
    let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
    let mut seqnum: crate::stdlib::uint64_t = 0;
    let mut valid: crate::krb5_h::krb5_boolean = 0;
    let mut cksumtype: crate::krb5_h::krb5_cksumtype = 0;
    let mut conf_flag = 0i32;
    if !qop_state.is_null() {
        *qop_state = 0u32
    }
    header = crate::src::krb5::util_crypt::kg_locate_header_iov(iov, iov_count, toktype);
    if !header.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"header != NULL\x00" as *const u8 as
                          *const i8,
                      b"k5sealv3iov.c\x00" as *const u8 as
                          *const i8,
                      302u32,
                      (*::std::mem::transmute::<&[u8; 139],
                                                &[i8; 139]>(b"OM_uint32 gss_krb5int_unseal_v3_iov(krb5_context, OM_uint32 *, krb5_gss_ctx_id_rec *, gss_iov_buffer_desc *, int, int *, gss_qop_t *, int)\x00")).as_ptr());
    }
    padding = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 9u32);
    if !padding.is_null() && (*padding).buffer.length != 0usize {
        return (9u32) << 16i32;
    }
    trailer = crate::src::krb5::util_crypt::kg_locate_iov(iov, iov_count, 7u32);
    acceptor_flag = if (*ctx).initiate() as i32 != 0 {
        0x1i32
    } else {
        0i32
    } as u8;
    key_usage = if toktype == 0x201i32 {
        if (*ctx).initiate() == 0 {
            24i32
        } else {
            22i32
        }
    } else if (*ctx).initiate() == 0 {
        25i32
    } else {
        23i32
    };
    crate::src::krb5::util_crypt::kg_iov_msglen(
        iov,
        iov_count,
        &mut data_length,
        &mut assoc_data_length,
    );
    ptr = (*header).buffer.value as *mut u8;
    if (*header).buffer.length < 16usize {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    if *ptr.offset(2isize) as i32 & 0x1i32 != acceptor_flag as i32 {
        *minor_status = -(2045022963 as isize) as crate::gssapi_h::OM_uint32;
        return (6u32) << 16i32;
    }
    if (*ctx).have_acceptor_subkey() as i32 != 0 && *ptr.offset(2isize) as i32 & 0x4i32 != 0 {
        key = (*ctx).acceptor_subkey;
        cksumtype = (*ctx).acceptor_subkey_cksumtype
    } else {
        key = (*ctx).subkey;
        cksumtype = (*ctx).cksumtype
    }
    if !key.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"key != NULL\x00" as *const u8 as *const i8,
                      b"k5sealv3iov.c\x00" as *const u8 as
                          *const i8,
                      340u32,
                      (*::std::mem::transmute::<&[u8; 139],
                                                &[i8; 139]>(b"OM_uint32 gss_krb5int_unseal_v3_iov(krb5_context, OM_uint32 *, krb5_gss_ctx_id_rec *, gss_iov_buffer_desc *, int, int *, gss_qop_t *, int)\x00")).as_ptr());
    }
    if toktype == 0x201i32 {
        let mut k5_trailerlen: u32 = 0;
        if load_16_be(ptr as *const libc::c_void) as i32 != 0x504i32 {
            current_block = 1507479929741853459;
        } else {
            conf_flag = (*ptr.offset(2isize) as i32 & 0x2i32 != 0i32) as i32;
            if *ptr.offset(3isize) as i32 != 0xffi32 {
                current_block = 1507479929741853459;
            } else {
                ec = load_16_be(ptr.offset(4isize) as *const libc::c_void)
                    as crate::stddef_h::size_t;
                rrc = load_16_be(ptr.offset(6isize) as *const libc::c_void)
                    as crate::stddef_h::size_t;
                seqnum = load_64_be(ptr.offset(8isize) as *const libc::c_void);
                code = crate::krb5_h::krb5_c_crypto_length(
                    context,
                    (*key).keyblock.enctype,
                    if conf_flag != 0 { 5i32 } else { 6i32 },
                    &mut k5_trailerlen,
                ) as crate::gssapi_h::OM_uint32;
                if code != 0u32 {
                    *minor_status = code;
                    return (13u32) << 16i32;
                }
                /* Deal with RRC */
                if trailer.is_null() {
                    let mut desired_rrc = k5_trailerlen as crate::stddef_h::size_t; /* E(Header) */
                    if conf_flag != 0 {
                        desired_rrc = (desired_rrc).wrapping_add(16usize);
                        if (*ctx).gss_flags & 0x1000u32 == 0u32 {
                            desired_rrc = (desired_rrc).wrapping_add(ec)
                        }
                    }
                    /* According to MS, we only need to deal with a fixed RRC for DCE */
                    if rrc != desired_rrc {
                        current_block = 1507479929741853459;
                    } else {
                        current_block = 2516253395664191498;
                    }
                } else if rrc != 0usize {
                    current_block = 1507479929741853459;
                } else {
                    current_block = 2516253395664191498;
                }
                match current_block {
                    1507479929741853459 => {}
                    _ => {
                        if conf_flag != 0 {
                            let mut althdr = 0 as *mut u8;
                            /* Decrypt */
                            code = crate::src::krb5::util_crypt::kg_decrypt_iov(
                                context,
                                (*ctx).proto,
                                ((*ctx).gss_flags & 0x1000u32 != 0u32) as i32,
                                ec,
                                rrc,
                                key,
                                key_usage,
                                0 as crate::krb5_h::krb5_pointer,
                                iov,
                                iov_count,
                            ) as crate::gssapi_h::OM_uint32;
                            if code != 0u32 {
                                *minor_status = code;
                                return (6u32) << 16i32;
                            }
                            /* Validate header integrity */
                            if trailer.is_null() {
                                althdr = ((*header).buffer.value as *mut u8)
                                    .offset(16isize)
                                    .offset(ec as isize)
                            } else {
                                althdr = ((*trailer).buffer.value as *mut u8).offset(ec as isize)
                            }
                            if load_16_be(althdr as *const libc::c_void) as i32 != 0x504i32
                                || *althdr.offset(2isize) as i32 != *ptr.offset(2isize) as i32
                                || *althdr.offset(3isize) as i32 != *ptr.offset(3isize) as i32
                                || crate::stdlib::memcmp(
                                    althdr.offset(8isize) as *const libc::c_void,
                                    ptr.offset(8isize) as *const libc::c_void,
                                    8usize,
                                ) != 0i32
                            {
                                *minor_status = 0u32;
                                return (6u32) << 16i32;
                            }
                            current_block = 777662472977924419;
                        } else if ec != k5_trailerlen as usize {
                            current_block = 1507479929741853459;
                        } else {
                            /* Verify checksum: note EC is checksum size here, not padding */
                            /* Zero EC, RRC before computing checksum */
                            store_16_be(0u32, ptr.offset(4isize) as *mut libc::c_void);
                            store_16_be(0u32, ptr.offset(6isize) as *mut libc::c_void);
                            code = crate::src::krb5::util_cksum::kg_verify_checksum_iov_v3(
                                context, cksumtype, rrc, key, key_usage, iov, iov_count, toktype,
                                &mut valid,
                            ) as crate::gssapi_h::OM_uint32;
                            if code != 0u32 || valid == 0u32 {
                                *minor_status = code;
                                return (6u32) << 16i32;
                            }
                            current_block = 777662472977924419;
                        }
                        match current_block {
                            1507479929741853459 => {}
                            _ => {
                                code = crate::src::generic::util_seqstate::gssint_g_seqstate_check(
                                    (*ctx).seqstate,
                                    seqnum,
                                );
                                current_block = 200744462051969938;
                            }
                        }
                    }
                }
            }
        }
    } else {
        if toktype == 0x101i32 {
            if load_16_be(ptr as *const libc::c_void) as i32 != 0x404i32 {
                current_block = 1507479929741853459;
            } else {
                current_block = 6141879417302102378;
            }
        } else if toktype == 0x102i32 {
            if load_16_be(ptr as *const libc::c_void) as i32 != 0x405i32 {
                current_block = 1507479929741853459;
            } else {
                current_block = 6141879417302102378;
            }
        } else {
            current_block = 1507479929741853459;
        }
        match current_block {
            1507479929741853459 => {}
            _ => {
                if *ptr.offset(3isize) as i32 != 0xffi32 {
                    current_block = 1507479929741853459;
                } else {
                    seqnum = load_64_be(ptr.offset(8isize) as *const libc::c_void);
                    /* For MIC tokens, the GSS header and checksum are in the same buffer.
                     * Fake up an RRC so that the checksum is expected in the header. */
                    rrc = if !trailer.is_null() {
                        0usize
                    } else {
                        (*header).buffer.length.wrapping_sub(16usize)
                    };
                    code = crate::src::krb5::util_cksum::kg_verify_checksum_iov_v3(
                        context, cksumtype, rrc, key, key_usage, iov, iov_count, toktype,
                        &mut valid,
                    ) as crate::gssapi_h::OM_uint32;
                    if code != 0u32 || valid == 0u32 {
                        *minor_status = code;
                        return (6u32) << 16i32;
                    }
                    code = crate::src::generic::util_seqstate::gssint_g_seqstate_check(
                        (*ctx).seqstate,
                        seqnum,
                    );
                    current_block = 200744462051969938;
                }
            }
        }
    }
    match current_block {
        1507479929741853459 =>
        /* Should have been rotated by kg_unseal_stream_iov() */
        {
            *minor_status = 0u32;
            return (9u32) << 16i32;
        }
        _ => {
            *minor_status = 0u32;
            if !conf_state.is_null() {
                *conf_state = conf_flag
            }
            return code;
        }
    };
}
