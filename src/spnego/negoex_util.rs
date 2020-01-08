use ::libc;

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

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod k5_buf_h {

    #[inline]

    pub unsafe extern "C" fn k5_buf_add_uint16_le(
        mut buf: *mut crate::k5_buf_h::k5buf,
        mut val: crate::stdlib::uint16_t,
    ) {
        let mut p = crate::k5_buf_h::k5_buf_get_space(buf, 2usize);
        if !p.is_null() {
            store_16_le(val as u32, p);
        };
    }
    #[inline]

    pub unsafe extern "C" fn k5_buf_add_uint32_le(
        mut buf: *mut crate::k5_buf_h::k5buf,
        mut val: crate::stdlib::uint32_t,
    ) {
        let mut p = crate::k5_buf_h::k5_buf_get_space(buf, 4usize);
        if !p.is_null() {
            store_32_le(val, p);
        };
    }
    #[inline]

    pub unsafe extern "C" fn k5_buf_add_uint64_le(
        mut buf: *mut crate::k5_buf_h::k5buf,
        mut val: crate::stdlib::uint64_t,
    ) {
        let mut p = crate::k5_buf_h::k5_buf_get_space(buf, 8usize);
        if !p.is_null() {
            store_64_le(val, p);
        };
    }

    use crate::src::spnego::negoex_util::k5_platform_h::store_16_le;
    use crate::src::spnego::negoex_util::k5_platform_h::store_32_le;
    use crate::src::spnego::negoex_util::k5_platform_h::store_64_le;

    /* K5_BUF_H */
}

pub mod k5_platform_h {

    #[inline]

    pub unsafe extern "C" fn store_16_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_5)).i = val as crate::stdlib::uint16_t;
    }
    #[inline]

    pub unsafe extern "C" fn store_32_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = val;
    }
    #[inline]

    pub unsafe extern "C" fn store_64_le(
        mut val: crate::stdlib::uint64_t,
        mut vp: *mut libc::c_void,
    ) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_4)).i = val;
    }
    #[inline]

    pub unsafe extern "C" fn load_16_le(mut cvp: *const libc::c_void) -> u16 {
        let mut p = cvp as *const u8;
        return (*(p as *const crate::k5_platform_h::C2RustUnnamed_5)).i;
    }
    #[inline]

    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void) -> u32 {
        let mut p = cvp as *const u8;
        return (*(p as *const crate::k5_platform_h::C2RustUnnamed_6)).i;
    }
    #[inline]

    pub unsafe extern "C" fn load_64_le(mut cvp: *const libc::c_void) -> crate::stdlib::uint64_t {
        let mut p = cvp as *const u8;
        return (*(p as *const crate::k5_platform_h::C2RustUnnamed_4)).i;
    }

    /* K5_PLATFORM_H */
}

pub mod k5_input_h {

    #[inline]

    pub unsafe extern "C" fn k5_input_init(
        mut in_0: *mut crate::k5_input_h::k5input,
        mut ptr: *const libc::c_void,
        mut len: crate::stddef_h::size_t,
    ) {
        (*in_0).ptr = ptr as *const u8;
        (*in_0).len = len;
        (*in_0).status = 0i32;
    }
    /* Only set the status value of in if it hasn't already been set, so status
     * reflects the first thing to go wrong. */
    #[inline]

    pub unsafe extern "C" fn k5_input_set_status(
        mut in_0: *mut crate::k5_input_h::k5input,
        mut status: crate::stdlib::int32_t,
    ) {
        if (*in_0).status == 0 {
            (*in_0).status = status
        };
    }
    #[inline]

    pub unsafe extern "C" fn k5_input_get_bytes(
        mut in_0: *mut crate::k5_input_h::k5input,
        mut len: crate::stddef_h::size_t,
    ) -> *const u8 {
        if (*in_0).len < len {
            k5_input_set_status(in_0, 22i32);
        }
        if (*in_0).status != 0 {
            return 0 as *const u8;
        }
        (*in_0).len = ((*in_0).len).wrapping_sub(len);
        (*in_0).ptr = (*in_0).ptr.offset(len as isize);
        return (*in_0).ptr.offset(-(len as isize));
    }
    #[inline]

    pub unsafe extern "C" fn k5_input_get_uint16_le(
        mut in_0: *mut crate::k5_input_h::k5input,
    ) -> crate::stdlib::uint16_t {
        let mut ptr = k5_input_get_bytes(in_0, 2usize);
        return if ptr.is_null() {
            0i32
        } else {
            load_16_le(ptr as *const libc::c_void) as i32
        } as crate::stdlib::uint16_t;
    }
    #[inline]

    pub unsafe extern "C" fn k5_input_get_uint32_le(
        mut in_0: *mut crate::k5_input_h::k5input,
    ) -> crate::stdlib::uint32_t {
        let mut ptr = k5_input_get_bytes(in_0, 4usize);
        return if ptr.is_null() {
            0u32
        } else {
            load_32_le(ptr as *const libc::c_void)
        };
    }
    #[inline]

    pub unsafe extern "C" fn k5_input_get_uint64_le(
        mut in_0: *mut crate::k5_input_h::k5input,
    ) -> crate::stdlib::uint64_t {
        let mut ptr = k5_input_get_bytes(in_0, 8usize);
        return if ptr.is_null() {
            0usize
        } else {
            load_64_le(ptr as *const libc::c_void)
        };
    }

    use crate::src::spnego::negoex_util::k5_platform_h::load_16_le;
    use crate::src::spnego::negoex_util::k5_platform_h::load_32_le;
    use crate::src::spnego::negoex_util::k5_platform_h::load_64_le;

    /* K5_BUF_H */
}

/* _GSSAPIP_GENERIC_H_ */
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
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_trace_info;
pub use crate::krb5_h::_profile_t;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_c_random_make_octets;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_keyblock_contents;
pub use crate::krb5_h::krb5_init_context;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::profile_h::profile_t;
pub use crate::src::spnego::negoex_util::k5_int_h::make_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;

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
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_const_buffer_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;

pub use crate::k5_buf_h::k5_buf_add;
pub use crate::k5_buf_h::k5_buf_add_fmt;
pub use crate::k5_buf_h::k5_buf_add_len;
pub use crate::k5_buf_h::k5_buf_free;
pub use crate::k5_buf_h::k5_buf_get_space;
pub use crate::k5_buf_h::k5_buf_init_dynamic;
pub use crate::k5_buf_h::k5_buf_truncate;
pub use crate::k5_buf_h::k5buf;
pub use crate::k5_buf_h::k5buftype;
pub use crate::k5_buf_h::K5BUF_DYNAMIC;
pub use crate::k5_buf_h::K5BUF_DYNAMIC_ZAP;
pub use crate::k5_buf_h::K5BUF_ERROR;
pub use crate::k5_buf_h::K5BUF_FIXED;
pub use crate::k5_input_h::k5input;
pub use crate::k5_platform_h::C2RustUnnamed_4;
pub use crate::k5_platform_h::C2RustUnnamed_5;
pub use crate::k5_platform_h::C2RustUnnamed_6;
pub use crate::src::spnego::negoex_util::k5_buf_h::k5_buf_add_uint16_le;
pub use crate::src::spnego::negoex_util::k5_buf_h::k5_buf_add_uint32_le;
pub use crate::src::spnego::negoex_util::k5_buf_h::k5_buf_add_uint64_le;
pub use crate::src::spnego::negoex_util::k5_input_h::k5_input_get_bytes;
pub use crate::src::spnego::negoex_util::k5_input_h::k5_input_get_uint16_le;
pub use crate::src::spnego::negoex_util::k5_input_h::k5_input_get_uint32_le;
pub use crate::src::spnego::negoex_util::k5_input_h::k5_input_get_uint64_le;
pub use crate::src::spnego::negoex_util::k5_input_h::k5_input_init;
pub use crate::src::spnego::negoex_util::k5_input_h::k5_input_set_status;
pub use crate::src::spnego::negoex_util::k5_platform_h::load_16_le;
pub use crate::src::spnego::negoex_util::k5_platform_h::load_32_le;
pub use crate::src::spnego::negoex_util::k5_platform_h::load_64_le;
pub use crate::src::spnego::negoex_util::k5_platform_h::store_16_le;
pub use crate::src::spnego::negoex_util::k5_platform_h::store_32_le;
pub use crate::src::spnego::negoex_util::k5_platform_h::store_64_le;

#[no_mangle]

pub unsafe extern "C" fn negoex_random(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut data: *mut crate::stdlib::uint8_t,
    mut length: crate::stddef_h::size_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut d = make_data(data as *mut libc::c_void, length as u32);
    *minor =
        crate::krb5_h::krb5_c_random_make_octets((*ctx).kctx, &mut d) as crate::gssapi_h::OM_uint32;
    return if *minor != 0 { (13u32) << 16i32 } else { 0u32 };
}
/*
 * SPNEGO functions expect to find the active mech context in ctx->ctx_handle,
 * but the metadata exchange APIs force us to have one mech context per mech
 * entry.  To address this mismatch, move the active mech context (if we have
 * one) to ctx->ctx_handle at the end of NegoEx processing.
 */
#[no_mangle]

pub unsafe extern "C" fn negoex_prep_context_for_spnego(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) {
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    mech = (*ctx).negoex_mechs.tqh_first;
    if mech.is_null() || (*mech).mech_context.is_null() {
        return;
    }
    if (*ctx).ctx_handle.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"ctx->ctx_handle == GSS_C_NO_CONTEXT\x00" as *const u8 as *const i8,
            b"negoex_util.c\x00" as *const u8 as *const i8,
            64u32,
            (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                b"void negoex_prep_context_for_spnego(spnego_gss_ctx_id_t)\x00",
            ))
            .as_ptr(),
        );
    }
    (*ctx).ctx_handle = (*mech).mech_context;
    (*mech).mech_context = 0 as crate::gssapi_h::gss_ctx_id_t;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_prep_context_for_negoex(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::krb5_h::krb5_error_code = 0;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    if !(*ctx).kctx.is_null() {
        /* The context is already initialized for NegoEx.  Undo what
         * negoex_prep_for_spnego() did, if applicable. */
        if !(*ctx).ctx_handle.is_null() {
            mech = (*ctx).negoex_mechs.tqh_first;
            if !mech.is_null() && (*mech).mech_context.is_null() {
            } else {
                crate::stdlib::__assert_fail(b"mech != NULL && mech->mech_context == GSS_C_NO_CONTEXT\x00"
                                  as *const u8 as *const i8,
                              b"negoex_util.c\x00" as *const u8 as
                                  *const i8,
                              80u32,
                              (*::std::mem::transmute::<&[u8; 75],
                                                        &[i8; 75]>(b"OM_uint32 negoex_prep_context_for_negoex(OM_uint32 *, spnego_gss_ctx_id_t)\x00")).as_ptr());
            }
            (*mech).mech_context = (*ctx).ctx_handle;
            (*ctx).ctx_handle = 0 as crate::gssapi_h::gss_ctx_id_t
        }
        return 0u32;
    }
    /* Initialize the NegoEX context fields.  (negoex_mechs is already set up
     * by SPNEGO.) */
    ret = crate::krb5_h::krb5_init_context(&mut (*ctx).kctx);
    if ret != 0 {
        *minor = ret as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    crate::k5_buf_h::k5_buf_init_dynamic(&mut (*ctx).negoex_transcript);
    return 0u32;
}

unsafe extern "C" fn release_all_mechs(mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t) {
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut next = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() && {
        next = (*mech).links.tqe_next;
        (1i32) != 0
    } {
        release_auth_mech(mech);
        mech = next
    }
    (*ctx).negoex_mechs.tqh_first = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    (*ctx).negoex_mechs.tqh_last = &mut (*ctx).negoex_mechs.tqh_first;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_release_context(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
) {
    crate::k5_buf_h::k5_buf_free(&mut (*ctx).negoex_transcript);
    release_all_mechs(ctx);
    crate::krb5_h::krb5_free_context((*ctx).kctx);
    (*ctx).kctx = 0 as crate::krb5_h::krb5_context;
}

unsafe extern "C" fn typestr(mut type_0: crate::gssapiP_negoex_h::message_type) -> *const i8 {
    if type_0 == crate::gssapiP_negoex_h::INITIATOR_NEGO {
        return b"INITIATOR_NEGO\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::ACCEPTOR_NEGO {
        return b"ACCEPTOR_NEGO\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::INITIATOR_META_DATA {
        return b"INITIATOR_META_DATA\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::ACCEPTOR_META_DATA {
        return b"ACCEPTOR_META_DATA\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::CHALLENGE {
        return b"CHALLENGE\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::AP_REQUEST {
        return b"AP_REQUEST\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::VERIFY {
        return b"VERIFY\x00" as *const u8 as *const i8;
    } else if type_0 == crate::gssapiP_negoex_h::ALERT {
        return b"ALERT\x00" as *const u8 as *const i8;
    } else {
        return b"UNKNOWN\x00" as *const u8 as *const i8;
    };
}

unsafe extern "C" fn add_guid(
    mut buf: *mut crate::k5_buf_h::k5buf,
    mut guid: *const crate::stdlib::uint8_t,
) {
    let mut data1 = load_32_le(guid as *const libc::c_void);
    let mut data2 = load_16_le(guid.offset(4isize) as *const libc::c_void);
    let mut data3 = load_16_le(guid.offset(6isize) as *const libc::c_void);
    crate::k5_buf_h::k5_buf_add_fmt(
        buf,
        b"%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x\x00" as *const u8 as *const i8,
        data1,
        data2 as i32,
        data3 as i32,
        *guid.offset(8isize) as i32,
        *guid.offset(9isize) as i32,
        *guid.offset(10isize) as i32,
        *guid.offset(11isize) as i32,
        *guid.offset(12isize) as i32,
        *guid.offset(13isize) as i32,
        *guid.offset(14isize) as i32,
        *guid.offset(15isize) as i32,
    );
}

unsafe extern "C" fn guid_to_string(mut guid: *const crate::stdlib::uint8_t) -> *mut i8 {
    let mut buf = crate::k5_buf_h::k5buf {
        buftype: crate::k5_buf_h::K5BUF_ERROR,
        data: 0 as *mut libc::c_void,
        space: 0,
        len: 0,
    };
    crate::k5_buf_h::k5_buf_init_dynamic(&mut buf);
    add_guid(&mut buf, guid);
    return buf.data as *mut i8;
}
/* Check that the described vector lies within the message, and return a
 * pointer to its first element. */
#[inline]

unsafe extern "C" fn vector_base(
    mut offset: crate::stddef_h::size_t,
    mut count: crate::stddef_h::size_t,
    mut width: crate::stddef_h::size_t,
    mut msg_base: *const crate::stdlib::uint8_t,
    mut msg_len: crate::stddef_h::size_t,
) -> *const crate::stdlib::uint8_t {
    if offset > msg_len || count > msg_len.wrapping_sub(offset).wrapping_div(width) {
        return 0 as *const crate::stdlib::uint8_t;
    }
    return msg_base.offset(offset as isize);
}
/* Trace a received message.  Call after the context sequence number is
 * incremented. */

unsafe extern "C" fn trace_received_message(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut msg: *const crate::gssapiP_negoex_h::negoex_message,
) {
    let mut buf = crate::k5_buf_h::k5buf {
        buftype: crate::k5_buf_h::K5BUF_ERROR,
        data: 0 as *mut libc::c_void,
        space: 0,
        len: 0,
    };
    let mut i: crate::stdlib::uint16_t = 0;
    let mut info = 0 as *mut i8;
    if (*msg).type_0 == crate::gssapiP_negoex_h::INITIATOR_NEGO
        || (*msg).type_0 == crate::gssapiP_negoex_h::ACCEPTOR_NEGO
    {
        crate::k5_buf_h::k5_buf_init_dynamic(&mut buf);
        i = 0u16;
        while (i as i32) < (*msg).u.n.nschemes as i32 {
            add_guid(
                &mut buf,
                (*msg).u.n.schemes.offset((i as i32 * 16i32) as isize),
            );
            if (i as i32 + 1i32) < (*msg).u.n.nschemes as i32 {
                crate::k5_buf_h::k5_buf_add(&mut buf, b" \x00" as *const u8 as *const i8);
            }
            i = i.wrapping_add(1)
        }
        info = buf.data as *mut i8
    } else if (*msg).type_0 == crate::gssapiP_negoex_h::INITIATOR_META_DATA
        || (*msg).type_0 == crate::gssapiP_negoex_h::ACCEPTOR_META_DATA
        || (*msg).type_0 == crate::gssapiP_negoex_h::CHALLENGE
        || (*msg).type_0 == crate::gssapiP_negoex_h::AP_REQUEST
    {
        info = guid_to_string((*msg).u.e.scheme.as_ptr())
    } else if (*msg).type_0 == crate::gssapiP_negoex_h::VERIFY {
        info = guid_to_string((*msg).u.v.scheme.as_ptr())
    } else if (*msg).type_0 == crate::gssapiP_negoex_h::ALERT {
        info = guid_to_string((*msg).u.a.scheme.as_ptr())
    }
    if info.is_null() {
        return;
    }
    if (*(*ctx).kctx).trace_callback.is_some() {
        crate::k5_trace_h::krb5int_trace(
            (*ctx).kctx,
            b"NegoEx received [{int}]{str}: {str}\x00" as *const u8 as *const i8,
            (*ctx).negoex_seqnum as i32 - 1i32,
            typestr((*msg).type_0),
            info,
        );
    }
    crate::stdlib::free(info as *mut libc::c_void);
}
/* Trace an outgoing message with a GUID info string.  Call after the context
 * sequence number is incremented. */

unsafe extern "C" fn trace_outgoing_message(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
    mut guid: *const crate::stdlib::uint8_t,
) {
    let mut info = guid_to_string(guid); /* skip over ErrorCode */
    if info.is_null() {
        return;
    }
    if (*(*ctx).kctx).trace_callback.is_some() {
        crate::k5_trace_h::krb5int_trace(
            (*ctx).kctx,
            b"NegoEx sending [{int}]{str}: {str}\x00" as *const u8 as *const i8,
            (*ctx).negoex_seqnum as i32 - 1i32,
            typestr(type_0),
            info,
        );
    }
    crate::stdlib::free(info as *mut libc::c_void);
}

unsafe extern "C" fn parse_nego_message(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut in_0: *mut crate::k5_input_h::k5input,
    mut msg_base: *const crate::stdlib::uint8_t,
    mut msg_len: crate::stddef_h::size_t,
    mut msg: *mut crate::gssapiP_negoex_h::nego_message,
) -> crate::gssapi_h::OM_uint32 {
    let mut p = 0 as *const crate::stdlib::uint8_t;
    let mut protocol_version: crate::stdlib::uint64_t = 0;
    let mut extension_type: crate::stdlib::uint32_t = 0;
    let mut offset: crate::stddef_h::size_t = 0;
    let mut count: crate::stddef_h::size_t = 0;
    let mut i: crate::stddef_h::size_t = 0;
    p = k5_input_get_bytes(in_0, ::std::mem::size_of::<[crate::stdlib::uint8_t; 32]>());
    if !p.is_null() {
        crate::stdlib::memcpy(
            (*msg).random.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            ::std::mem::size_of::<[crate::stdlib::uint8_t; 32]>(),
        );
    }
    protocol_version = k5_input_get_uint64_le(in_0);
    if protocol_version != 0usize {
        *minor = 0x20000018u32;
        return (16u32) << 16i32;
    }
    offset = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    count = k5_input_get_uint16_le(in_0) as crate::stddef_h::size_t;
    (*msg).schemes = vector_base(offset, count, 16usize, msg_base, msg_len);
    (*msg).nschemes = count as crate::stdlib::uint16_t;
    if (*msg).schemes.is_null() {
        *minor = 0x20000008u32;
        return (9u32) << 16i32;
    }
    offset = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    count = k5_input_get_uint16_le(in_0) as crate::stddef_h::size_t;
    p = vector_base(offset, count, 12usize, msg_base, msg_len);
    i = 0usize;
    while i < count {
        extension_type =
            load_32_le(p.offset(i.wrapping_mul(12usize) as isize) as *const libc::c_void);
        if extension_type & 0x80000000u32 != 0 {
            *minor = 0x20000017u32;
            return (16u32) << 16i32;
        }
        i = i.wrapping_add(1)
    }
    return 0u32;
}

unsafe extern "C" fn parse_exchange_message(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut in_0: *mut crate::k5_input_h::k5input,
    mut msg_base: *const crate::stdlib::uint8_t,
    mut msg_len: crate::stddef_h::size_t,
    mut msg: *mut crate::gssapiP_negoex_h::exchange_message,
) -> crate::gssapi_h::OM_uint32 {
    let mut p = 0 as *const crate::stdlib::uint8_t;
    let mut offset: crate::stddef_h::size_t = 0;
    let mut len: crate::stddef_h::size_t = 0;
    p = k5_input_get_bytes(in_0, 16usize);
    if !p.is_null() {
        crate::stdlib::memcpy(
            (*msg).scheme.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            16usize,
        );
    }
    offset = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    len = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    p = vector_base(offset, len, 1usize, msg_base, msg_len);
    if p.is_null() {
        *minor = 0x20000008u32;
        return (9u32) << 16i32;
    }
    (*msg).token.value = p as *mut libc::c_void;
    (*msg).token.length = len;
    return 0u32;
}

unsafe extern "C" fn parse_verify_message(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut in_0: *mut crate::k5_input_h::k5input,
    mut msg_base: *const crate::stdlib::uint8_t,
    mut msg_len: crate::stddef_h::size_t,
    mut token_offset: crate::stddef_h::size_t,
    mut msg: *mut crate::gssapiP_negoex_h::verify_message,
) -> crate::gssapi_h::OM_uint32 {
    let mut p = 0 as *const crate::stdlib::uint8_t;
    let mut offset: crate::stddef_h::size_t = 0;
    let mut len: crate::stddef_h::size_t = 0;
    let mut hdrlen: crate::stdlib::uint32_t = 0;
    let mut cksum_scheme: crate::stdlib::uint32_t = 0;
    p = k5_input_get_bytes(in_0, 16usize);
    if !p.is_null() {
        crate::stdlib::memcpy(
            (*msg).scheme.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            16usize,
        );
    }
    hdrlen = k5_input_get_uint32_le(in_0);
    if hdrlen != 20u32 {
        *minor = 0x20000008u32;
        return (9u32) << 16i32;
    }
    cksum_scheme = k5_input_get_uint32_le(in_0);
    if cksum_scheme != 1u32 {
        *minor = 0x20000015u32;
        return (16u32) << 16i32;
    }
    (*msg).cksum_type = k5_input_get_uint32_le(in_0);
    offset = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    len = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    (*msg).cksum = vector_base(offset, len, 1usize, msg_base, msg_len);
    (*msg).cksum_len = len;
    if (*msg).cksum.is_null() {
        *minor = 0x20000008u32;
        return (9u32) << 16i32;
    }
    (*msg).offset_in_token = token_offset;
    return 0u32;
}

unsafe extern "C" fn parse_alert_message(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut in_0: *mut crate::k5_input_h::k5input,
    mut msg_base: *const crate::stdlib::uint8_t,
    mut msg_len: crate::stddef_h::size_t,
    mut msg: *mut crate::gssapiP_negoex_h::alert_message,
) -> crate::gssapi_h::OM_uint32 {
    let mut p = 0 as *const crate::stdlib::uint8_t;
    let mut atype: crate::stdlib::uint32_t = 0;
    let mut reason: crate::stdlib::uint32_t = 0;
    let mut alerts_offset: crate::stddef_h::size_t = 0;
    let mut nalerts: crate::stddef_h::size_t = 0;
    let mut value_offset: crate::stddef_h::size_t = 0;
    let mut value_len: crate::stddef_h::size_t = 0;
    let mut i: crate::stddef_h::size_t = 0;
    let mut alerts_in = crate::k5_input_h::k5input {
        ptr: 0 as *const u8,
        len: 0,
        status: 0,
    };
    let mut pulse_in = crate::k5_input_h::k5input {
        ptr: 0 as *const u8,
        len: 0,
        status: 0,
    };
    p = k5_input_get_bytes(in_0, 16usize);
    if !p.is_null() {
        crate::stdlib::memcpy(
            (*msg).scheme.as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            16usize,
        );
    }
    k5_input_get_uint32_le(in_0);
    alerts_offset = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    nalerts = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    p = vector_base(alerts_offset, nalerts, 12usize, msg_base, msg_len);
    if p.is_null() {
        *minor = 0x20000008u32;
        return (9u32) << 16i32;
    }
    /* Look for a VERIFY_NO_KEY pulse alert in the alerts vector. */
    (*msg).verify_no_key = 0i32; /* skip header length */
    k5_input_init(
        &mut alerts_in,
        p as *const libc::c_void,
        nalerts.wrapping_mul(12usize),
    );
    i = 0usize;
    while i < nalerts {
        atype = k5_input_get_uint32_le(&mut alerts_in);
        value_offset = k5_input_get_uint32_le(&mut alerts_in) as crate::stddef_h::size_t;
        value_len = k5_input_get_uint32_le(&mut alerts_in) as crate::stddef_h::size_t;
        p = vector_base(value_offset, value_len, 1usize, msg_base, msg_len);
        if p.is_null() {
            *minor = 0x20000008u32;
            return (9u32) << 16i32;
        }
        if atype == 1u32 && value_len >= 8usize {
            k5_input_init(&mut pulse_in, p as *const libc::c_void, value_len);
            k5_input_get_uint32_le(&mut pulse_in);
            reason = k5_input_get_uint32_le(&mut pulse_in);
            if reason == 1u32 {
                (*msg).verify_no_key = 1i32
            }
        }
        i = i.wrapping_add(1)
    }
    return 0u32;
}

unsafe extern "C" fn parse_message(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut in_0: *mut crate::k5_input_h::k5input,
    mut token_base: *const crate::stdlib::uint8_t,
    mut msg: *mut crate::gssapiP_negoex_h::negoex_message,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut msg_base = (*in_0).ptr;
    let mut conv_id = 0 as *const crate::stdlib::uint8_t;
    let mut token_remaining = (*in_0).len;
    let mut header_len: crate::stddef_h::size_t = 0;
    let mut msg_len: crate::stddef_h::size_t = 0;
    let mut signature: crate::stdlib::uint64_t = 0;
    let mut type_0: crate::stdlib::uint32_t = 0;
    let mut seqnum: crate::stdlib::uint32_t = 0;
    signature = k5_input_get_uint64_le(in_0);
    type_0 = k5_input_get_uint32_le(in_0);
    seqnum = k5_input_get_uint32_le(in_0);
    header_len = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    msg_len = k5_input_get_uint32_le(in_0) as crate::stddef_h::size_t;
    conv_id = k5_input_get_bytes(in_0, 16usize);
    if (*in_0).status != 0 || msg_len > token_remaining || header_len > msg_len {
        *minor = 0x20000008u32;
        return (9u32) << 16i32;
    }
    if signature as u64 != 0x535458454f47454eu64 {
        *minor = 0x20000006u32;
        return (9u32) << 16i32;
    }
    if seqnum != (*ctx).negoex_seqnum {
        *minor = 0x20000019u32;
        return (9u32) << 16i32;
    }
    if seqnum == 0u32 {
        crate::stdlib::memcpy(
            (*ctx).negoex_conv_id.as_mut_ptr() as *mut libc::c_void,
            conv_id as *const libc::c_void,
            16usize,
        );
    } else if !(crate::stdlib::memcmp(
        conv_id as *const libc::c_void,
        (*ctx).negoex_conv_id.as_mut_ptr() as *const libc::c_void,
        16usize,
    ) == 0i32)
    {
        *minor = 0x20000009u32;
        return (9u32) << 16i32;
    }
    /* Restrict the input region to the header. */
    (*in_0).len = header_len.wrapping_sub((*in_0).ptr.wrapping_offset_from(msg_base) as usize);
    (*msg).type_0 = type_0;
    if type_0 == crate::gssapiP_negoex_h::INITIATOR_NEGO
        || type_0 == crate::gssapiP_negoex_h::ACCEPTOR_NEGO
    {
        major = parse_nego_message(minor, in_0, msg_base, msg_len, &mut (*msg).u.n)
    } else if type_0 == crate::gssapiP_negoex_h::INITIATOR_META_DATA
        || type_0 == crate::gssapiP_negoex_h::ACCEPTOR_META_DATA
        || type_0 == crate::gssapiP_negoex_h::CHALLENGE
        || type_0 == crate::gssapiP_negoex_h::AP_REQUEST
    {
        major = parse_exchange_message(minor, in_0, msg_base, msg_len, &mut (*msg).u.e)
    } else if type_0 == crate::gssapiP_negoex_h::VERIFY {
        major = parse_verify_message(
            minor,
            in_0,
            msg_base,
            msg_len,
            msg_base.wrapping_offset_from(token_base) as crate::stddef_h::size_t,
            &mut (*msg).u.v,
        )
    } else if type_0 == crate::gssapiP_negoex_h::ALERT {
        major = parse_alert_message(minor, in_0, msg_base, msg_len, &mut (*msg).u.a)
    } else {
        *minor = 0x20000007u32;
        return (9u32) << 16i32;
    }
    if major != 0u32 {
        return major;
    }
    /* Reset the input buffer to the remainder of the token. */
    if (*in_0).status == 0 {
        k5_input_init(
            in_0,
            msg_base.offset(msg_len as isize) as *const libc::c_void,
            token_remaining.wrapping_sub(msg_len),
        );
    }
    (*ctx).negoex_seqnum = (*ctx).negoex_seqnum.wrapping_add(1);
    trace_received_message(ctx, msg);
    return 0u32;
}
/*
 * Parse token into an array of negoex_message structures.  All pointer fields
 * within the parsed messages are aliases into token, so the result can be
 * freed with free().  An unknown protocol version, a critical extension, or an
 * unknown checksum scheme will cause a parsing failure.  Increment the
 * sequence number in ctx for each message, and record and check the
 * conversation ID in ctx as appropriate.
 */
#[no_mangle]

pub unsafe extern "C" fn negoex_parse_token(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut token: crate::gssapi_h::gss_const_buffer_t,
    mut messages_out: *mut *mut crate::gssapiP_negoex_h::negoex_message,
    mut count_out: *mut crate::stddef_h::size_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut count = 0usize;
    let mut in_0 = crate::k5_input_h::k5input {
        ptr: 0 as *const u8,
        len: 0,
        status: 0,
    };
    let mut messages = 0 as *mut crate::gssapiP_negoex_h::negoex_message;
    let mut newptr = 0 as *mut crate::gssapiP_negoex_h::negoex_message;
    *messages_out = 0 as *mut crate::gssapiP_negoex_h::negoex_message;
    *count_out = 0usize;
    if !token.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"token != GSS_C_NO_BUFFER\x00" as *const u8 as
                          *const i8,
                      b"negoex_util.c\x00" as *const u8 as
                          *const i8,
                      464u32,
                      (*::std::mem::transmute::<&[u8; 119],
                                                &[i8; 119]>(b"OM_uint32 negoex_parse_token(OM_uint32 *, spnego_gss_ctx_id_t, gss_const_buffer_t, struct negoex_message **, size_t *)\x00")).as_ptr());
    }
    k5_input_init(&mut in_0, (*token).value, (*token).length);
    while in_0.status == 0i32 && in_0.len > 0usize {
        newptr = crate::stdlib::realloc(
            messages as *mut libc::c_void,
            count
                .wrapping_add(1usize)
                .wrapping_mul(::std::mem::size_of::<crate::gssapiP_negoex_h::negoex_message>()),
        ) as *mut crate::gssapiP_negoex_h::negoex_message;
        if newptr.is_null() {
            crate::stdlib::free(messages as *mut libc::c_void);
            *minor = 12u32;
            return (13u32) << 16i32;
        }
        messages = newptr;
        major = parse_message(
            minor,
            ctx,
            &mut in_0,
            (*token).value as *const crate::stdlib::uint8_t,
            &mut *messages.offset(count as isize),
        );
        if major != 0u32 {
            break;
        }
        count = count.wrapping_add(1)
    }
    if in_0.status != 0 {
        *minor = 0x20000008u32;
        major = (9u32) << 16i32
    }
    if major != 0u32 {
        crate::stdlib::free(messages as *mut libc::c_void);
        return major;
    }
    *messages_out = messages;
    *count_out = count;
    return 0u32;
}

unsafe extern "C" fn locate_message(
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
) -> *mut crate::gssapiP_negoex_h::negoex_message {
    let mut i: crate::stdlib::uint32_t = 0;
    i = 0u32;
    while (i as usize) < nmessages {
        if (*messages.offset(i as isize)).type_0 == type_0 {
            return &mut *messages.offset(i as isize)
                as *mut crate::gssapiP_negoex_h::negoex_message;
        }
        i = i.wrapping_add(1)
    }
    return 0 as *mut crate::gssapiP_negoex_h::negoex_message;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_locate_nego_message(
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
) -> *mut crate::gssapiP_negoex_h::nego_message {
    let mut msg = locate_message(messages, nmessages, type_0);
    return if msg.is_null() {
        0 as *mut crate::gssapiP_negoex_h::nego_message
    } else {
        &mut (*msg).u.n
    };
}
#[no_mangle]

pub unsafe extern "C" fn negoex_locate_exchange_message(
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
) -> *mut crate::gssapiP_negoex_h::exchange_message {
    let mut msg = locate_message(messages, nmessages, type_0);
    return if msg.is_null() {
        0 as *mut crate::gssapiP_negoex_h::exchange_message
    } else {
        &mut (*msg).u.e
    };
}
#[no_mangle]

pub unsafe extern "C" fn negoex_locate_verify_message(
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
) -> *mut crate::gssapiP_negoex_h::verify_message {
    let mut msg = locate_message(messages, nmessages, crate::gssapiP_negoex_h::VERIFY);
    return if msg.is_null() {
        0 as *mut crate::gssapiP_negoex_h::verify_message
    } else {
        &mut (*msg).u.v
    };
}
#[no_mangle]

pub unsafe extern "C" fn negoex_locate_alert_message(
    mut messages: *mut crate::gssapiP_negoex_h::negoex_message,
    mut nmessages: crate::stddef_h::size_t,
) -> *mut crate::gssapiP_negoex_h::alert_message {
    let mut msg = locate_message(messages, nmessages, crate::gssapiP_negoex_h::ALERT);
    return if msg.is_null() {
        0 as *mut crate::gssapiP_negoex_h::alert_message
    } else {
        &mut (*msg).u.a
    };
}
/*
 * Add the encoding of a MESSAGE_HEADER structure to buf, given the number of
 * bytes of the payload following the full header.  Increment the sequence
 * number in ctx.  Set *payload_start_out to the position of the payload within
 * the message.
 */

unsafe extern "C" fn put_message_header(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
    mut payload_len: crate::stdlib::uint32_t,
    mut payload_start_out: *mut crate::stdlib::uint32_t,
) {
    let mut header_len: crate::stddef_h::size_t = 0;
    if type_0 == crate::gssapiP_negoex_h::INITIATOR_NEGO
        || type_0 == crate::gssapiP_negoex_h::ACCEPTOR_NEGO
    {
        header_len = 96usize
    } else if type_0 == crate::gssapiP_negoex_h::INITIATOR_META_DATA
        || type_0 == crate::gssapiP_negoex_h::ACCEPTOR_META_DATA
        || type_0 == crate::gssapiP_negoex_h::CHALLENGE
        || type_0 == crate::gssapiP_negoex_h::AP_REQUEST
    {
        header_len = 64usize
    } else if type_0 == crate::gssapiP_negoex_h::VERIFY {
        header_len = 80usize
    } else if type_0 == crate::gssapiP_negoex_h::ALERT {
        header_len = 72usize
    } else {
        crate::stdlib::abort();
    }
    k5_buf_add_uint64_le(
        &mut (*ctx).negoex_transcript,
        0x535458454f47454eu64 as crate::stdlib::uint64_t,
    );
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, type_0);
    let fresh0 = (*ctx).negoex_seqnum;
    (*ctx).negoex_seqnum = (*ctx).negoex_seqnum.wrapping_add(1);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, fresh0);
    k5_buf_add_uint32_le(
        &mut (*ctx).negoex_transcript,
        header_len as crate::stdlib::uint32_t,
    );
    k5_buf_add_uint32_le(
        &mut (*ctx).negoex_transcript,
        header_len.wrapping_add(payload_len as usize) as crate::stdlib::uint32_t,
    );
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        (*ctx).negoex_conv_id.as_mut_ptr() as *const libc::c_void,
        16usize,
    );
    *payload_start_out = header_len as crate::stdlib::uint32_t;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_add_nego_message(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
    mut random: *mut crate::stdlib::uint8_t,
) {
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut payload_start: crate::stdlib::uint32_t = 0;
    let mut seqnum = (*ctx).negoex_seqnum;
    let mut nschemes: crate::stdlib::uint16_t = 0;
    let mut buf = crate::k5_buf_h::k5buf {
        buftype: crate::k5_buf_h::K5BUF_ERROR,
        data: 0 as *mut libc::c_void,
        space: 0,
        len: 0,
    };
    nschemes = 0u16;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() {
        nschemes = nschemes.wrapping_add(1);
        mech = (*mech).links.tqe_next
    }
    put_message_header(
        ctx,
        type_0,
        (nschemes as i32 * 16i32) as crate::stdlib::uint32_t,
        &mut payload_start,
    );
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        random as *const libc::c_void,
        32usize,
    );
    /* ProtocolVersion */
    k5_buf_add_uint64_le(&mut (*ctx).negoex_transcript, 0usize);
    /* AuthSchemes vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint16_le(&mut (*ctx).negoex_transcript, nschemes);
    /* Extensions vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint16_le(&mut (*ctx).negoex_transcript, 0u16);
    /* Four bytes of padding to reach a multiple of 8 bytes. */
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        b"\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_void,
        4usize,
    );
    /* Payload (auth schemes); also build guid string for tracing. */
    crate::k5_buf_h::k5_buf_init_dynamic(&mut buf);
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() {
        crate::k5_buf_h::k5_buf_add_len(
            &mut (*ctx).negoex_transcript,
            (*mech).scheme.as_mut_ptr() as *const libc::c_void,
            16usize,
        );
        add_guid(
            &mut buf,
            (*mech).scheme.as_mut_ptr() as *const crate::stdlib::uint8_t,
        );
        crate::k5_buf_h::k5_buf_add(&mut buf, b" \x00" as *const u8 as *const i8);
        mech = (*mech).links.tqe_next
    }
    if buf.len > 0usize {
        crate::k5_buf_h::k5_buf_truncate(&mut buf, buf.len.wrapping_sub(1usize));
        if (*(*ctx).kctx).trace_callback.is_some() {
            crate::k5_trace_h::krb5int_trace(
                (*ctx).kctx,
                b"NegoEx sending [{int}]{str}: {str}\x00" as *const u8 as *const i8,
                seqnum as i32,
                typestr(type_0),
                buf.data,
            );
        }
        crate::k5_buf_h::k5_buf_free(&mut buf);
    };
}
#[no_mangle]

pub unsafe extern "C" fn negoex_add_exchange_message(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut type_0: crate::gssapiP_negoex_h::message_type,
    mut scheme: *const crate::stdlib::uint8_t,
    mut token: crate::gssapi_h::gss_buffer_t,
) {
    let mut payload_start: crate::stdlib::uint32_t = 0;
    put_message_header(
        ctx,
        type_0,
        (*token).length as crate::stdlib::uint32_t,
        &mut payload_start,
    );
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        scheme as *const libc::c_void,
        16usize,
    );
    /* Exchange byte vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint32_le(
        &mut (*ctx).negoex_transcript,
        (*token).length as crate::stdlib::uint32_t,
    );
    /* Payload (token) */
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        (*token).value,
        (*token).length,
    );
    trace_outgoing_message(ctx, type_0, scheme);
}
#[no_mangle]

pub unsafe extern "C" fn negoex_add_verify_message(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut scheme: *const crate::stdlib::uint8_t,
    mut cksum_type: crate::stdlib::uint32_t,
    mut cksum: *const crate::stdlib::uint8_t,
    mut cksum_len: crate::stdlib::uint32_t,
) {
    let mut payload_start: crate::stdlib::uint32_t = 0;
    put_message_header(
        ctx,
        crate::gssapiP_negoex_h::VERIFY,
        cksum_len,
        &mut payload_start,
    );
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        scheme as *const libc::c_void,
        16usize,
    );
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 20u32);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 1u32);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, cksum_type);
    /* ChecksumValue vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, cksum_len);
    /* Four bytes of padding to reach a multiple of 8 bytes. */
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        b"\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_void,
        4usize,
    );
    /* Payload (checksum contents) */
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        cksum as *const libc::c_void,
        cksum_len as crate::stddef_h::size_t,
    );
    trace_outgoing_message(ctx, crate::gssapiP_negoex_h::VERIFY, scheme);
}
/* Add an ALERT_MESSAGE containing a single ALERT_TYPE_PULSE alert with the
 * reason ALERT_VERIFY_NO_KEY. */
#[no_mangle]

pub unsafe extern "C" fn negoex_add_verify_no_key_alert(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut scheme: *const crate::stdlib::uint8_t,
) {
    let mut payload_start: crate::stdlib::uint32_t = 0;
    put_message_header(
        ctx,
        crate::gssapiP_negoex_h::ALERT,
        (12i32 + 8i32) as crate::stdlib::uint32_t,
        &mut payload_start,
    );
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        scheme as *const libc::c_void,
        16usize,
    );
    /* ErrorCode */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 0u32);
    /* Alerts vector */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, payload_start);
    k5_buf_add_uint16_le(&mut (*ctx).negoex_transcript, 1u16);
    /* Six bytes of padding to reach a multiple of 8 bytes. */
    crate::k5_buf_h::k5_buf_add_len(
        &mut (*ctx).negoex_transcript,
        b"\x00\x00\x00\x00\x00\x00\x00" as *const u8 as *const libc::c_void,
        6usize,
    );
    /* Payload part 1: a single ALERT element */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 1u32);
    k5_buf_add_uint32_le(
        &mut (*ctx).negoex_transcript,
        payload_start.wrapping_add(12u32),
    );
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 8u32);
    /* Payload part 2: ALERT_PULSE */
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 8u32);
    k5_buf_add_uint32_le(&mut (*ctx).negoex_transcript, 1u32);
    trace_outgoing_message(ctx, crate::gssapiP_negoex_h::ALERT, scheme);
}
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

unsafe extern "C" fn release_auth_mech(mut mech: *mut crate::gssapiP_negoex_h::negoex_auth_mech) {
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    if mech.is_null() {
        return;
    }
    crate::src::mechglue::g_delete_sec_context::gss_delete_sec_context(
        &mut tmpmin,
        &mut (*mech).mech_context,
        0 as crate::gssapi_h::gss_buffer_t,
    );
    crate::src::generic::oid_ops::generic_gss_release_oid(&mut tmpmin, &mut (*mech).oid);
    crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmpmin, &mut (*mech).metadata);
    crate::krb5_h::krb5_free_keyblock_contents(0 as crate::krb5_h::krb5_context, &mut (*mech).key);
    crate::krb5_h::krb5_free_keyblock_contents(
        0 as crate::krb5_h::krb5_context,
        &mut (*mech).verify_key,
    );
    crate::stdlib::free(mech as *mut libc::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn negoex_delete_auth_mech(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut mech: *mut crate::gssapiP_negoex_h::negoex_auth_mech,
) {
    if !(*mech).links.tqe_next.is_null() {
        (*(*mech).links.tqe_next).links.tqe_prev = (*mech).links.tqe_prev
    } else {
        (*ctx).negoex_mechs.tqh_last = (*mech).links.tqe_prev
    }
    *(*mech).links.tqe_prev = (*mech).links.tqe_next;
    release_auth_mech(mech);
}
/* Remove all auth mech entries except for mech from ctx->mechs. */
#[no_mangle]

pub unsafe extern "C" fn negoex_select_auth_mech(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut mech: *mut crate::gssapiP_negoex_h::negoex_auth_mech,
) {
    if !mech.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"mech != NULL\x00" as *const u8 as *const i8,
            b"negoex_util.c\x00" as *const u8 as *const i8,
            721u32,
            (*::std::mem::transmute::<&[u8; 77], &[i8; 77]>(
                b"void negoex_select_auth_mech(spnego_gss_ctx_id_t, struct negoex_auth_mech *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*mech).links.tqe_next.is_null() {
        (*(*mech).links.tqe_next).links.tqe_prev = (*mech).links.tqe_prev
    } else {
        (*ctx).negoex_mechs.tqh_last = (*mech).links.tqe_prev
    }
    *(*mech).links.tqe_prev = (*mech).links.tqe_next;
    release_all_mechs(ctx);
    (*mech).links.tqe_next = (*ctx).negoex_mechs.tqh_first;
    if !(*mech).links.tqe_next.is_null() {
        (*(*ctx).negoex_mechs.tqh_first).links.tqe_prev = &mut (*mech).links.tqe_next
    } else {
        (*ctx).negoex_mechs.tqh_last = &mut (*mech).links.tqe_next
    }
    (*ctx).negoex_mechs.tqh_first = mech;
    (*mech).links.tqe_prev = &mut (*ctx).negoex_mechs.tqh_first;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_add_auth_mech(
    mut minor: *mut crate::gssapi_h::OM_uint32,
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut oid: crate::gssapi_h::gss_const_OID,
    mut scheme: *mut crate::stdlib::uint8_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut major: crate::gssapi_h::OM_uint32 = 0;
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    mech = crate::stdlib::calloc(
        1usize,
        ::std::mem::size_of::<crate::gssapiP_negoex_h::negoex_auth_mech>(),
    ) as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    if mech.is_null() {
        *minor = 12u32;
        return (13u32) << 16i32;
    }
    major = crate::src::generic::oid_ops::generic_gss_copy_oid(minor, oid, &mut (*mech).oid);
    if major != 0u32 {
        crate::stdlib::free(mech as *mut libc::c_void);
        return major;
    }
    crate::stdlib::memcpy(
        (*mech).scheme.as_mut_ptr() as *mut libc::c_void,
        scheme as *const libc::c_void,
        16usize,
    );
    (*mech).links.tqe_next = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    (*mech).links.tqe_prev = (*ctx).negoex_mechs.tqh_last;
    *(*ctx).negoex_mechs.tqh_last = mech;
    (*ctx).negoex_mechs.tqh_last = &mut (*mech).links.tqe_next;
    *minor = 0u32;
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn negoex_locate_auth_scheme(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut scheme: *const crate::stdlib::uint8_t,
) -> *mut crate::gssapiP_negoex_h::negoex_auth_mech {
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() {
        if crate::stdlib::memcmp(
            (*mech).scheme.as_mut_ptr() as *const libc::c_void,
            scheme as *const libc::c_void,
            16usize,
        ) == 0i32
        {
            return mech;
        }
        mech = (*mech).links.tqe_next
    }
    return 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
}
/* Prune ctx->mechs to the schemes present in schemes, and reorder them to
 * match its order. */
#[no_mangle]

pub unsafe extern "C" fn negoex_common_auth_schemes(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut schemes: *const crate::stdlib::uint8_t,
    mut nschemes: crate::stdlib::uint16_t,
) {
    let mut list = crate::gssapiP_spnego_h::negoex_mech_list {
        tqh_first: 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech,
        tqh_last: 0 as *mut *mut crate::gssapiP_negoex_h::negoex_auth_mech,
    };
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut i: crate::stdlib::uint16_t = 0;
    /* Construct a new list in the order of schemes. */
    list.tqh_first = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    list.tqh_last = &mut list.tqh_first;
    i = 0u16;
    while (i as i32) < nschemes as i32 {
        mech = negoex_locate_auth_scheme(ctx, schemes.offset((i as i32 * 16i32) as isize));
        if !mech.is_null() {
            if !(*mech).links.tqe_next.is_null() {
                (*(*mech).links.tqe_next).links.tqe_prev = (*mech).links.tqe_prev
            } else {
                (*ctx).negoex_mechs.tqh_last = (*mech).links.tqe_prev
            }
            *(*mech).links.tqe_prev = (*mech).links.tqe_next;
            (*mech).links.tqe_next = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
            (*mech).links.tqe_prev = list.tqh_last;
            *list.tqh_last = mech;
            list.tqh_last = &mut (*mech).links.tqe_next
        }
        i = i.wrapping_add(1)
    }
    /* Release any leftover entries and replace the context list. */
    release_all_mechs(ctx);
    if !list.tqh_first.is_null() {
        *(*ctx).negoex_mechs.tqh_last = list.tqh_first;
        (*list.tqh_first).links.tqe_prev = (*ctx).negoex_mechs.tqh_last;
        (*ctx).negoex_mechs.tqh_last = list.tqh_last;
        list.tqh_first = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
        list.tqh_last = &mut list.tqh_first
    };
}
/* negoex_util.c */
/* Prune ctx->mechs to the schemes present in schemes, but do not change
 * their order. */
#[no_mangle]

pub unsafe extern "C" fn negoex_restrict_auth_schemes(
    mut ctx: crate::gssapiP_spnego_h::spnego_gss_ctx_id_t,
    mut schemes: *const crate::stdlib::uint8_t,
    mut nschemes: crate::stdlib::uint16_t,
) {
    let mut mech = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut next = 0 as *mut crate::gssapiP_negoex_h::negoex_auth_mech;
    let mut i: crate::stdlib::uint16_t = 0;
    let mut found: i32 = 0;
    mech = (*ctx).negoex_mechs.tqh_first;
    while !mech.is_null() && {
        next = (*mech).links.tqe_next;
        (1i32) != 0
    } {
        found = 0i32;
        i = 0u16;
        while (i as i32) < nschemes as i32 && found == 0 {
            if crate::stdlib::memcmp(
                (*mech).scheme.as_mut_ptr() as *const libc::c_void,
                schemes.offset((i as i32 * 16i32) as isize) as *const libc::c_void,
                16usize,
            ) == 0i32
            {
                found = 1i32
            }
            i = i.wrapping_add(1)
        }
        if found == 0 {
            negoex_delete_auth_mech(ctx, mech);
        }
        mech = next
    }
}
