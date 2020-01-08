#[repr(C)]
#[derive(Copy, Clone)]
pub struct spnego_ctx_st {
    pub magic_num: crate::gssapi_h::OM_uint32,
    pub DER_mechTypes: crate::gssapi_h::gss_buffer_desc,
    pub mech_set: crate::gssapi_h::gss_OID_set,
    pub internal_mech: crate::gssapi_h::gss_OID,
    pub ctx_handle: crate::gssapi_h::gss_ctx_id_t,
    pub mic_reqd: i32,
    pub mic_sent: i32,
    pub mic_rcvd: i32,
    pub firstpass: i32,
    pub mech_complete: i32,
    pub nego_done: i32,
    pub initiate: i32,
    pub opened: i32,
    pub ctx_flags: crate::gssapi_h::OM_uint32,
    pub internal_name: crate::gssapi_h::gss_name_t,
    pub actual_mech: crate::gssapi_h::gss_OID,
    pub deleg_cred: crate::gssapi_h::gss_cred_id_t,
    pub negoex_step: i32,
    pub negoex_transcript: crate::k5_buf_h::k5buf,
    pub negoex_seqnum: crate::stdlib::uint32_t,
    pub negoex_conv_id: crate::gssapiP_negoex_h::conversation_id,
    pub negoex_mechs: crate::gssapiP_spnego_h::negoex_mech_list,
    pub kctx: crate::krb5_h::krb5_context,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct negoex_mech_list {
    pub tqh_first: *mut crate::gssapiP_negoex_h::negoex_auth_mech,
    pub tqh_last: *mut *mut crate::gssapiP_negoex_h::negoex_auth_mech,
}
/*
 * Copyright 2003 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */

/* #pragma ident	"@(#)gssapiP_spnego.h	1.3	03/09/18 SMI" */
pub type spnego_gss_ctx_id_t = *mut crate::gssapiP_spnego_h::spnego_ctx_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub mcred: crate::gssapi_h::gss_cred_id_t,
    pub neg_mechs: crate::gssapi_h::gss_OID_set,
    pub no_ask_integ: i32,
}
pub type send_token_flag = u32;
pub type spnego_token_t = *mut libc::c_void;
/* Structure for credential */
pub type spnego_gss_cred_id_t = *mut crate::gssapiP_spnego_h::C2RustUnnamed_3;
pub const ERROR_TOKEN_SEND: crate::gssapiP_spnego_h::send_token_flag = 4;
pub const CHECK_MIC: crate::gssapiP_spnego_h::send_token_flag = 3;
pub const CONT_TOKEN_SEND: crate::gssapiP_spnego_h::send_token_flag = 2;
pub const INIT_TOKEN_SEND: crate::gssapiP_spnego_h::send_token_flag = 1;
pub const NO_TOKEN_SEND: crate::gssapiP_spnego_h::send_token_flag = 0;
